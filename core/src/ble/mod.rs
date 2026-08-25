pub mod framing;

#[cfg(not(target_os = "linux"))]
mod fallback;
#[cfg(target_os = "linux")]
mod linux;

#[cfg(not(target_os = "linux"))]
pub use fallback::run;
#[cfg(target_os = "linux")]
pub use linux::run;

pub const SERVICE_UUID: &str = "80cf2c4a-803f-4db5-9464-ffd9ec0548c9";
pub const REQUEST_CHARACTERISTIC_UUID: &str = "e03062a5-a493-4712-9972-7616363b8fd3";
pub const RESPONSE_CHARACTERISTIC_UUID: &str = "479f504d-373c-4469-9940-fe169b3c5c9f";
