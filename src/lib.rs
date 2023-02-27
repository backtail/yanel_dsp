#![no_std]

use embedded_audio_tools as tools;

mod freeverb;

pub use crate::freeverb::Freeverb;

#[cfg(feature = "SR44k1")]
pub const SAMPLING_RATE: usize = 44_100;

#[cfg(feature = "SR48k")]
pub const SAMPLING_RATE: usize = 48_000;

#[cfg(feature = "SR88k2")]
pub const SAMPLING_RATE: usize = 88_200;

#[cfg(feature = "SR96k")]
pub const SAMPLING_RATE: usize = 96_000;

#[cfg(feature = "SR192k")]
pub const SAMPLING_RATE: usize = 192_000;
