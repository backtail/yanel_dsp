mod ffi;

use embedded_audio_tools::filter::{
    butterworth::ButterworthType, Biquad, BiquadCoeffs, Butterworth,
};

#[repr(C)]
pub struct MultiFilter {
    biquad: Biquad<Butterworth>,
    filter: ButterworthType,
    sr: f32,
    fc: f32,
    q: f32,
    gain: f32,
}

impl MultiFilter {
    pub fn init(sr: usize) -> MultiFilter {
        MultiFilter {
            biquad: Biquad::new(BiquadCoeffs::new()),
            filter: ButterworthType::Lowpass,
            sr: sr as f32,
            fc: 100.0,
            q: 1.0,
            gain: 1.0,
        }
    }

    pub fn next(&mut self, input: f32) -> f32 {
        self.biquad.process(input)
    }

    pub fn set_filter(&mut self, filter: ButterworthType) {
        self.filter = filter;
        self.update_coeffs();
    }

    pub fn set_cutoff(&mut self, freq: f32) {
        self.fc = freq;
        self.update_coeffs();
    }

    pub fn set_q(&mut self, q: f32) {
        self.q = q;
        self.update_coeffs();
    }

    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
        self.update_coeffs();
    }

    pub fn set_all(&mut self, filter: ButterworthType, freq: f32, q: f32, gain: f32) {
        self.filter = filter;
        self.fc = freq;
        self.q = q;
        self.gain = gain;
        self.update_coeffs();
    }

    fn update_coeffs(&mut self) {
        match self.filter {
            ButterworthType::Lowpass => self.biquad.coeffs.lowpass(self.fc, self.q, self.sr),
            ButterworthType::Highpass => self.biquad.coeffs.highpass(self.fc, self.q, self.sr),
            ButterworthType::Allpass => self.biquad.coeffs.allpass(self.fc, self.q, self.sr),
            ButterworthType::Notch => self.biquad.coeffs.notch(self.fc, self.q, self.sr),
            ButterworthType::Bell => self.biquad.coeffs.bell(self.fc, self.q, self.gain, self.sr),
            ButterworthType::LowShelf => self
                .biquad
                .coeffs
                .low_shelf(self.fc, self.q, self.gain, self.sr),
        }
    }
}
