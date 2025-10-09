#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::run;

#[cfg(not(target_os = "linux"))]
mod fallback;

#[cfg(not(target_os = "linux"))]
pub use fallback::run;
