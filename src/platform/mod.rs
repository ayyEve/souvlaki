pub use self::platform::*;

#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod platform;

#[cfg(target_os = "macos")]
#[path = "macos/mod.rs"]
mod platform;

#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
#[path = "empty/mod.rs"]
mod platform;