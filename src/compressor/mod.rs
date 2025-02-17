mod ffi;

use embedded_audio_tools::{float::lerp_unchecked, FFCompressor};

const MIN_ATTACK_TIME: f32 = 0.0001; // s
const MIN_RELEASE_TIME: f32 = 0.0005; // s
const MIN_SLOPE: f32 = 0.0;
const MIN_RATIO: f32 = 1.0;
const MIN_MAKEUP_GAIN: f32 = 1.0;

const MAX_ATTACK_TIME: f32 = 0.05; // s
const MAX_RELEASE_TIME: f32 = 0.3; // s
const MAX_SLOPE: f32 = 10.0;
const MAX_RATIO: f32 = 10.0;
const MAX_MAKEUP_GAIN: f32 = 16.0;

#[repr(C)]
pub struct Compressor {
    comp: FFCompressor,
    sr: f32,
}

impl Compressor {
    pub fn init(sr: f32) -> Compressor {
        Compressor {
            comp: FFCompressor::new(1.0, 1.0, 1.0),
            sr,
        }
    }

    pub fn tick(&mut self, sample: f32) -> f32 {
        self.comp.tick(sample)
    }

    pub fn set_attack(&mut self, val: f32) {
        self.comp.set_attack(
            lerp_unchecked(MIN_ATTACK_TIME, MAX_ATTACK_TIME, val.clamp(0.0, 1.0)),
            self.sr,
        );
    }

    pub fn set_release(&mut self, val: f32) {
        self.comp.set_release(
            lerp_unchecked(MIN_RELEASE_TIME, MAX_RELEASE_TIME, val.clamp(0.0, 1.0)),
            self.sr,
        );
    }

    pub fn set_attack_slope(&mut self, val: f32) {
        self.comp
            .set_attack_slope(lerp_unchecked(MIN_SLOPE, -MAX_SLOPE, val.clamp(0.0, 1.0)));
    }

    pub fn set_release_slope(&mut self, val: f32) {
        self.comp
            .set_release_slope(lerp_unchecked(MIN_SLOPE, MAX_SLOPE, val.clamp(0.0, 1.0)));
    }

    pub fn set_ratio(&mut self, val: f32) {
        self.comp
            .set_ratio(lerp_unchecked(MIN_RATIO, MAX_RATIO, val.clamp(0.0, 1.0)));
    }

    pub fn set_threshold(&mut self, val: f32) {
        self.comp.set_threshold(val.clamp(0.0, 1.0));
    }

    pub fn set_makeup_gain(&mut self, val: f32) {
        self.comp.set_makeup_gain(lerp_unchecked(
            MIN_MAKEUP_GAIN,
            MAX_MAKEUP_GAIN,
            val.clamp(0.0, 1.0),
        ));
    }
}
