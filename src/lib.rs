#![cfg_attr(feature = "static", no_std)] // Use `no_std` only for staticlib builds

use embedded_audio_tools as tools;

mod ffi;

mod freeverb;
mod multi_filter;
mod simple_delay;
mod synth_kick;

pub use crate::freeverb::Freeverb;
pub use crate::multi_filter::MultiFilter;
pub use crate::simple_delay::SimpleDelay;
pub use crate::synth_kick::SynthKick;

pub use tools::float::DSPUtility;

#[cfg(feature = "static")] // embedded targets
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
