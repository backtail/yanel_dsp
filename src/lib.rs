#![cfg_attr(feature = "static", no_std)] // Use `no_std` only for staticlib builds

use embedded_audio_tools as tools;

mod ffi;

mod freeverb;
mod multi_filter;
mod simple_delay;

pub use crate::freeverb::Freeverb;
pub use crate::multi_filter::MultiFilter;
pub use crate::simple_delay::SimpleDelay;

pub use tools::float::DSPUtility;

// Re-export the structs so cbindgen can see them
pub use tools::memory_access::{MemorySlice, Mutable, NonMutable};
pub use tools::stereo::PanningError;
pub use tools::DelayLine;

// Define concrete type aliases
pub type MutableMemorySlice = MemorySlice<Mutable>;
pub type NonMutableMemorySlice = MemorySlice<NonMutable>;

#[cfg(feature = "static")] // embedded targets
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
