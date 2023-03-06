use embedded_audio_tools::filter::{Biquad, BiquadCoeffs, Butterworth};

pub struct MultiFilter {
    biquad: Biquad<Butterworth>,
    filter: u8,
    sr: f32,
    fc: f32,
    q: f32,
    gain: f32,
}

impl MultiFilter {
    pub fn init(sr: usize) -> MultiFilter {
        MultiFilter {
            biquad: Biquad::new(BiquadCoeffs::new()),
            filter: 0,
            sr: sr as f32,
            fc: 100.0,
            q: 1.0,
            gain: 1.0,
        }
    }

    pub fn next(&mut self, input: f32) -> f32 {
        self.biquad.process(input)
    }

    pub fn set_filter(&mut self, filter: u8) {
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

    pub fn set_all(&mut self, filter: u8, freq: f32, q: f32, gain: f32) {
        self.filter = filter;
        self.fc = freq;
        self.q = q;
        self.gain = gain;
        self.update_coeffs();
    }

    fn update_coeffs(&mut self) {
        match self.filter {
            0 => self.biquad.coeffs.lowpass(self.fc, self.q, self.sr),
            1 => self.biquad.coeffs.highpass(self.fc, self.q, self.sr),
            2 => self.biquad.coeffs.allpass(self.fc, self.q, self.sr),
            3 => self.biquad.coeffs.notch(self.fc, self.q, self.sr),
            4 => self.biquad.coeffs.bell(self.fc, self.q, self.gain, self.sr),
            5 => self
                .biquad
                .coeffs
                .low_shelf(self.fc, self.q, self.gain, self.sr),
            _ => {}
        }
    }
}
