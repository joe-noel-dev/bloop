use std::ffi::c_void;

use log::error;
use tokio::sync::{broadcast, mpsc};

use crate::{
    api::{Request, Response},
    core::run_core,
    logger::{set_up_logger, LogOptions},
};

use anyhow::{Context, Result};

struct BloopContext {
    _core_thread: std::thread::JoinHandle<()>,
    _request_tx: mpsc::Sender<Request>,
    _response_task: tokio::task::JoinHandle<()>,
    _runtime: tokio::runtime::Runtime,
}

#[repr(C)]
enum BloopErrorCode {
    Success,
    InvalidRequest,
    ErrorPostingRequest,
}

type BloopResponseCallback = extern "C" fn(*mut c_void, *const u8, usize);

#[no_mangle]
extern "C" fn bloop_init(
    response_callback: BloopResponseCallback,
    response_callback_context: *mut c_void,
) -> *mut BloopContext {
    let log_options = LogOptions::default().log_to_console(true);
    set_up_logger(log_options);

    let (request_tx, request_rx) = mpsc::channel(128);
    let (response_tx, response_rx) = broadcast::channel(128);

    let core_thread = run_core(request_rx, request_tx.clone(), response_tx);

    let runtime = tokio::runtime::Runtime::new().unwrap();

    let response_callback_context = BloopResponseCallbackContext::new(response_callback_context);

    let response_task = runtime.spawn(async move {
        let mut response_rx = response_rx.resubscribe();
        while let Ok(response) = response_rx.recv().await {
            let response_bytes = match convert_response_to_bytes(&response) {
                Ok(bytes) => bytes,
                Err(e) => {
                    error!("Error converting response to bytes: {e}");
                    continue;
                }
            };

            response_callback(
                response_callback_context.as_ptr(),
                response_bytes.as_ptr(),
                response_bytes.len(),
            );
        }
    });

    let ctx = Box::new(BloopContext {
        _core_thread: core_thread,
        _request_tx: request_tx,
        _response_task: response_task,
        _runtime: runtime,
    });

    Box::into_raw(ctx)
}

#[no_mangle]
extern "C" fn bloop_add_request(context: *mut BloopContext, request: *const u8, size: usize) -> BloopErrorCode {
    let ctx = unsafe { &*context };
    let request_bytes = unsafe { std::slice::from_raw_parts(request, size) };
    let request = match convert_bytes_to_request(request_bytes) {
        Ok(request) => request,
        Err(e) => {
            error!("Error converting bytes to request: {e}");
            return BloopErrorCode::InvalidRequest;
        }
    };

    ctx._runtime.block_on(async {
        if let Err(e) = ctx._request_tx.send(request).await {
            error!("Error sending request: {e}");
            return BloopErrorCode::ErrorPostingRequest;
        }

        BloopErrorCode::Success
    })
}

#[no_mangle]
extern "C" fn bloop_shutdown(ctx: *mut BloopContext) {
    unsafe {
        let context = Box::from_raw(ctx);
        drop(context);
    }
}

fn convert_response_to_bytes(response: &Response) -> Result<Vec<u8>> {
    let document = bson::to_document(response).context("Serializing response")?;
    let mut data: Vec<u8> = vec![];
    document.to_writer(&mut data)?;
    Ok(data)
}

fn convert_bytes_to_request(message: &[u8]) -> Result<Request> {
    let document = bson::Document::from_reader(message).context("Deserializing request")?;
    let request = bson::from_document(document).context("Deserializing request")?;
    Ok(request)
}

#[derive(Clone, Copy)]
struct BloopResponseCallbackContext {
    context: *mut c_void,
}

impl BloopResponseCallbackContext {
    fn new(context: *mut c_void) -> Self {
        Self { context }
    }

    fn as_ptr(&self) -> *mut c_void {
        self.context
    }
}

unsafe impl Send for BloopResponseCallbackContext {}
unsafe impl Sync for BloopResponseCallbackContext {}
