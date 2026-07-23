use std::ffi::c_void;
#[cfg(target_os = "android")]
use std::sync::Once;

use log::error;
use tokio::sync::{broadcast, mpsc};

use crate::{
    api::{
        client::{create_client_responses, handle_client_configuration, ClientConfigurationHandle},
        wire::{decode_request, encode_response},
    },
    bloop::{Request, Response},
    core::run_core,
    logger::{set_up_logger, LogOptions},
    AppConfig,
};

struct BloopContext {
    _core_thread: std::thread::JoinHandle<()>,
    _request_tx: mpsc::Sender<Request>,
    configuration: ClientConfigurationHandle,
    direct_response_tx: mpsc::Sender<Response>,
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
    let (configuration, mut responses) = create_client_responses(response_rx);
    let (direct_response_tx, mut direct_response_rx) = mpsc::channel(16);

    let app_config = AppConfig::default();
    let core_thread = run_core(request_rx, request_tx.clone(), response_tx, app_config);

    let runtime = tokio::runtime::Runtime::new().unwrap();

    let response_callback_context = BloopResponseCallbackContext::new(response_callback_context);

    let response_task = runtime.spawn(async move {
        loop {
            let response = tokio::select! {
                response = responses.recv() => match response {
                    Ok(response) => response,
                    Err(broadcast::error::RecvError::Lagged(count)) => Response::default().with_error(
                        &format!("Response stream skipped {count} updates; request ALL to resynchronise")
                    ),
                    Err(broadcast::error::RecvError::Closed) => break,
                },
                response = direct_response_rx.recv() => match response {
                    Some(response) => response,
                    None => break,
                },
            };

            let response_bytes = match encode_response(&response) {
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
        configuration,
        direct_response_tx,
        _response_task: response_task,
        _runtime: runtime,
    });

    Box::into_raw(ctx)
}

#[no_mangle]
extern "C" fn bloop_add_request(context: *mut BloopContext, request: *const u8, size: usize) -> BloopErrorCode {
    let ctx = unsafe { &*context };
    let request_bytes = unsafe { std::slice::from_raw_parts(request, size) };
    let request = match decode_request(request_bytes) {
        Ok(request) => request,
        Err(e) => {
            error!("Error converting bytes to request: {e}");
            return BloopErrorCode::InvalidRequest;
        }
    };

    ctx._runtime.block_on(async {
        if let Some(response) = handle_client_configuration(&request, &ctx.configuration) {
            if let Err(e) = ctx.direct_response_tx.send(response).await {
                error!("Error sending client configuration response: {e}");
                return BloopErrorCode::ErrorPostingRequest;
            }
            return BloopErrorCode::Success;
        }

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

#[cfg(target_os = "android")]
static ANDROID_CONTEXT_INIT: Once = std::sync::Once::new();

#[cfg(target_os = "android")]
#[no_mangle]
extern "C" fn bloop_set_android_context(vm: *mut c_void, context: *mut c_void) {
    if vm.is_null() || context.is_null() {
        error!("Ignoring Android context initialization with null JNI pointers");
        return;
    }

    ANDROID_CONTEXT_INIT.call_once(|| {
        // CPAL's Android backend requires JVM + Context to be initialized before use.
        unsafe {
            ndk_context::initialize_android_context(vm, context);
        }

        // rustls-platform-verifier needs JVM + Context to call Android's TrustManager
        // for certificate verification.  Without this, the first outgoing TLS connection
        // panics and kills the core thread.
        unsafe {
            match jni::JavaVM::from_raw(vm.cast()) {
                Ok(jvm) => match jvm.attach_current_thread() {
                    Ok(mut env) => {
                        let ctx = jni::objects::JObject::from_raw(context.cast());
                        if let Err(e) = rustls_platform_verifier::android::init_hosted(&mut env, ctx) {
                            error!("Failed to initialize TLS verifier: {e:?}");
                        }
                    }
                    Err(e) => error!("Failed to attach JVM thread for TLS init: {e}"),
                },
                Err(e) => error!("Failed to get JVM for TLS init: {e}"),
            }
        }
    });
}
