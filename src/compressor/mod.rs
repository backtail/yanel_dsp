mod ffi;

use embedded_audio_tools::FFCompressor;

#[repr(C)]
pub struct Compressor {
    comp: FFCompressor,
    sr: f32,
}

impl Compressor {
    pub fn init(sr: f32) -> Compressor {
        Compressor {
            comp: FFCompressor::new(0.2, 2.0, 1.0, sr),
            sr,
        }
    }

    pub fn tick(&mut self, sample: f32) -> f32 {
        self.comp.tick(sample)
    }

    pub fn set_attack(&mut self, val: f32) {
        self.comp.set_attack(val, self.sr);
    }

    pub fn set_release(&mut self, val: f32) {
        self.comp.set_release(val, self.sr);
    }

    pub fn set_attack_slope(&mut self, val: f32) {
        self.comp.set_attack_slope(val);
    }

    pub fn set_release_slope(&mut self, val: f32) {
        self.comp.set_release_slope(val);
    }

    pub fn set_ratio(&mut self, val: f32) {
        self.comp.set_ratio(val);
    }

    pub fn set_threshold(&mut self, val: f32) {
        self.comp.set_threshold(val);
    }
}
