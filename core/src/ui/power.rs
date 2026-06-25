#[cfg(target_os = "linux")]
use std::process::{Child, Command, Stdio};

#[cfg(target_os = "macos")]
use std::ffi::CString;

pub struct SleepInhibitor {
    #[cfg(target_os = "linux")]
    child: Option<Child>,
    #[cfg(target_os = "macos")]
    assertion_id: Option<u32>,
}

impl SleepInhibitor {
    pub fn new() -> Self {
        Self::new_platform()
    }

    #[cfg(target_os = "linux")]
    fn new_platform() -> Self {
        let child = Command::new("systemd-inhibit")
            .args([
                "--what=sleep:idle",
                "--why=Bloop desktop UI is running",
                "--mode=block",
                "sleep",
                "infinity",
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| {
                log::warn!("Failed to inhibit system sleep with systemd-inhibit: {error}");
                error
            })
            .ok();

        Self { child }
    }

    #[cfg(target_os = "macos")]
    fn new_platform() -> Self {
        let Some(reason) = cf_string("Bloop desktop UI is running") else {
            log::warn!("Failed to create sleep inhibition reason");
            return Self { assertion_id: None };
        };

        let Some(assertion_type) = cf_string("PreventUserIdleDisplaySleep") else {
            unsafe {
                CFRelease(reason);
            }
            log::warn!("Failed to create sleep inhibition type");
            return Self { assertion_id: None };
        };

        let mut assertion_id = 0;
        let result = unsafe {
            IOPMAssertionCreateWithName(assertion_type, K_IOPM_ASSERTION_LEVEL_ON, reason, &mut assertion_id)
        };

        unsafe {
            CFRelease(assertion_type);
            CFRelease(reason);
        }

        if result == K_IORETURN_SUCCESS {
            Self {
                assertion_id: Some(assertion_id),
            }
        } else {
            log::warn!("Failed to inhibit system sleep: IOPMAssertionCreateWithName returned {result}");
            Self { assertion_id: None }
        }
    }

    #[cfg(target_os = "windows")]
    fn new_platform() -> Self {
        let result = unsafe { SetThreadExecutionState(ES_CONTINUOUS | ES_DISPLAY_REQUIRED | ES_SYSTEM_REQUIRED) };

        if result == 0 {
            log::warn!("Failed to inhibit system sleep: SetThreadExecutionState returned 0");
        }

        Self {}
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    fn new_platform() -> Self {
        Self {}
    }
}

impl Drop for SleepInhibitor {
    fn drop(&mut self) {
        #[cfg(target_os = "linux")]
        if let Some(mut child) = self.child.take() {
            if let Err(error) = child.kill() {
                log::warn!("Failed to release systemd sleep inhibition: {error}");
            }

            let _ = child.wait();
        }

        #[cfg(target_os = "macos")]
        if let Some(assertion_id) = self.assertion_id.take() {
            let result = unsafe { IOPMAssertionRelease(assertion_id) };

            if result != K_IORETURN_SUCCESS {
                log::warn!("Failed to release system sleep inhibition: IOPMAssertionRelease returned {result}");
            }
        }

        #[cfg(target_os = "windows")]
        unsafe {
            SetThreadExecutionState(ES_CONTINUOUS);
        }
    }
}

#[cfg(target_os = "macos")]
fn cf_string(value: &str) -> Option<CFStringRef> {
    let value = CString::new(value).ok()?;
    let string = unsafe { CFStringCreateWithCString(std::ptr::null(), value.as_ptr(), K_CF_STRING_ENCODING_UTF8) };

    if string.is_null() {
        None
    } else {
        Some(string)
    }
}

#[cfg(target_os = "macos")]
type CFStringRef = *const std::ffi::c_void;

#[cfg(target_os = "macos")]
const K_CF_STRING_ENCODING_UTF8: u32 = 0x0800_0100;

#[cfg(target_os = "macos")]
const K_IOPM_ASSERTION_LEVEL_ON: u32 = 255;

#[cfg(target_os = "macos")]
const K_IORETURN_SUCCESS: i32 = 0;

#[cfg(target_os = "macos")]
#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFStringCreateWithCString(
        alloc: *const std::ffi::c_void,
        c_str: *const std::os::raw::c_char,
        encoding: u32,
    ) -> CFStringRef;
    fn CFRelease(cf: *const std::ffi::c_void);
}

#[cfg(target_os = "macos")]
#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOPMAssertionCreateWithName(
        assertion_type: CFStringRef,
        assertion_level: u32,
        assertion_name: CFStringRef,
        assertion_id: *mut u32,
    ) -> i32;
    fn IOPMAssertionRelease(assertion_id: u32) -> i32;
}

#[cfg(target_os = "windows")]
const ES_CONTINUOUS: u32 = 0x8000_0000;

#[cfg(target_os = "windows")]
const ES_SYSTEM_REQUIRED: u32 = 0x0000_0001;

#[cfg(target_os = "windows")]
const ES_DISPLAY_REQUIRED: u32 = 0x0000_0002;

#[cfg(target_os = "windows")]
#[link(name = "kernel32")]
extern "system" {
    fn SetThreadExecutionState(es_flags: u32) -> u32;
}
