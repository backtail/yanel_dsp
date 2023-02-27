use crate::tools::{mut_mem_slice::from_slice, MutMemSlice};
use crate::tools::{AllPass, Comb};
use crate::SAMPLING_RATE;

// ========================
// Compile Time Calculation
// ========================

const fn adjust_length(length: usize) -> usize {
    length * SAMPLING_RATE / 44100
}

const STEREO_SPREAD: usize = 23;
const TUNINGS: [usize; 24] = [
    adjust_length(1116),                 // COMB_TUNING_L1
    adjust_length(1116 + STEREO_SPREAD), // COMB_TUNING_R1
    adjust_length(1188),                 // COMB_TUNING_L2
    adjust_length(1188 + STEREO_SPREAD), // COMB_TUNING_R2
    adjust_length(1277),                 // COMB_TUNING_L3
    adjust_length(1277 + STEREO_SPREAD), // COMB_TUNING_R3
    adjust_length(1356),                 // COMB_TUNING_L4
    adjust_length(1356 + STEREO_SPREAD), // COMB_TUNING_R4
    adjust_length(1422),                 // COMB_TUNING_L5
    adjust_length(1422 + STEREO_SPREAD), // COMB_TUNING_R5
    adjust_length(1491),                 // COMB_TUNING_L6
    adjust_length(1491 + STEREO_SPREAD), // COMB_TUNING_R6
    adjust_length(1557),                 // COMB_TUNING_L7
    adjust_length(1557 + STEREO_SPREAD), // COMB_TUNING_R7
    adjust_length(1617),                 // COMB_TUNING_L8
    adjust_length(1617 + STEREO_SPREAD), // COMB_TUNING_R8
    adjust_length(556),                  // ALLPASS_TUNING_L1
    adjust_length(556 + STEREO_SPREAD),  // ALLPASS_TUNING_R1
    adjust_length(441),                  // ALLPASS_TUNING_L2
    adjust_length(441 + STEREO_SPREAD),  // ALLPASS_TUNING_R2
    adjust_length(341),                  // ALLPASS_TUNING_L3
    adjust_length(341 + STEREO_SPREAD),  // ALLPASS_TUNING_R3
    adjust_length(225),                  // ALLPASS_TUNING_L4
    adjust_length(225 + STEREO_SPREAD),  // ALLPASS_TUNING_R4
];

const MAX_BUFFER_SIZE: usize = {
    let mut sum = 0;
    let mut i = 0;

    while i != 24 {
        sum += TUNINGS[i];
        i += 1;
    }

    sum
};

const FIXED_GAIN: f32 = 0.015;

const SCALE_WET: f32 = 3.0;
const SCALE_DAMPENING: f32 = 0.4;

const SCALE_ROOM: f32 = 0.28;
const OFFSET_ROOM: f32 = 0.7;

pub struct Freeverb {
    delay_line_buffer: [f32; MAX_BUFFER_SIZE],
    combs: [(Comb, Comb); 8],
    allpasses: [(AllPass, AllPass); 4],
    wet_gains: (f32, f32),
    wet: f32,
    width: f32,
    dry: f32,
    input_gain: f32,
    dampening: f32,
    room_size: f32,
    frozen: bool,
}

impl Freeverb {
    pub fn new(sr: usize) -> Self {
        assert_eq!(
            sr, SAMPLING_RATE,
            "This reverb owns memory on which it the delay lines sit. To safe on memory usage, its size is calculated at compile time!"
        );

        let mut freeverb = Freeverb {
            // reserve memory for delay lines and initiate null pointers
            delay_line_buffer: [0.0_f32; MAX_BUFFER_SIZE],
            combs: [(
                Comb::new(MutMemSlice::null()),
                Comb::new(MutMemSlice::null()),
            ); 8],
            allpasses: [(
                AllPass::new(MutMemSlice::null()),
                AllPass::new(MutMemSlice::null()),
            ); 4],
            wet_gains: (0.0, 0.0),
            wet: 0.0,
            dry: 0.0,
            input_gain: 0.0,
            width: 0.0,
            dampening: 0.0,
            room_size: 0.0,
            frozen: false,
        };

        freeverb.set_wet(1.0);
        freeverb.set_width(0.5);
        freeverb.set_dampening(0.5);
        freeverb.set_room_size(0.5);
        freeverb.set_frozen(false);

        freeverb
    }

    /// Checks if static buffer placement in memory and the pointers of the delaylines align. If not, it aligns it.
    ///
    /// Happens normally only once, after the object has been created.
    fn check_buffer_alignment(&mut self) {
        let buffer_start = core::ptr::addr_of_mut!(self.delay_line_buffer[0]);
        let pointer_start = self.combs[0].0.delay_line.buffer.ptr.0;

        if buffer_start != pointer_start {
            let mut offset = 0;
            // Give delay lines the approriate memory strips on static buffer
            for (i, _tuning) in TUNINGS.iter().enumerate().step_by(2) {
                let stage = i / 2;
                if i < 16 {
                    self.combs[stage].0.delay_line.buffer =
                        from_slice(&mut self.delay_line_buffer[offset..offset + TUNINGS[i]]);
                    offset += TUNINGS[i];

                    self.combs[stage].1.delay_line.buffer =
                        from_slice(&mut self.delay_line_buffer[offset..offset + TUNINGS[i + 1]]);
                    offset += TUNINGS[i + 1];
                } else {
                    self.allpasses[stage - 8].0.delay_line.buffer =
                        from_slice(&mut self.delay_line_buffer[offset..offset + TUNINGS[i]]);
                    offset += TUNINGS[i];

                    self.allpasses[stage - 8].1.delay_line.buffer =
                        from_slice(&mut self.delay_line_buffer[offset..offset + TUNINGS[i + 1]]);
                    offset += TUNINGS[i + 1];
                }
            }
        }
    }

    pub fn tick(&mut self, input: (f32, f32)) -> (f32, f32) {
        let input_mixed = (input.0 + input.1) * FIXED_GAIN * self.input_gain;

        let mut out = (0.0, 0.0);

        self.check_buffer_alignment();

        for combs in self.combs.iter_mut() {
            out.0 += combs.0.tick(input_mixed);
            out.1 += combs.1.tick(input_mixed);
        }

        for allpasses in self.allpasses.iter_mut() {
            out.0 = allpasses.0.tick(out.0);
            out.1 = allpasses.1.tick(out.1);
        }

        (
            out.0 * self.wet_gains.0 + out.1 * self.wet_gains.1 + input.0 * self.dry,
            out.1 * self.wet_gains.0 + out.0 * self.wet_gains.1 + input.1 * self.dry,
        )
    }

    pub fn set_dampening(&mut self, value: f32) {
        self.dampening = value * SCALE_DAMPENING;
        self.update_combs();
    }

    pub fn set_freeze(&mut self, frozen: bool) {
        self.frozen = frozen;
        self.update_combs();
    }

    pub fn set_wet(&mut self, value: f32) {
        self.wet = value * SCALE_WET;
        self.update_wet_gains();
    }

    pub fn set_width(&mut self, value: f32) {
        self.width = value;
        self.update_wet_gains();
    }

    fn update_wet_gains(&mut self) {
        self.wet_gains = (
            self.wet * (self.width / 2.0 + 0.5),
            self.wet * ((1.0 - self.width) / 2.0),
        )
    }

    fn set_frozen(&mut self, frozen: bool) {
        self.frozen = frozen;
        self.input_gain = if frozen { 0.0 } else { 1.0 };
        self.update_combs();
    }

    pub fn set_room_size(&mut self, value: f32) {
        self.room_size = value * SCALE_ROOM + OFFSET_ROOM;
        self.update_combs();
    }

    fn update_combs(&mut self) {
        let (feedback, dampening) = if self.frozen {
            (1.0, 0.0)
        } else {
            (self.room_size, self.dampening)
        };

        for combs in self.combs.iter_mut() {
            combs.0.set_feedback(feedback);
            combs.1.set_feedback(feedback);

            combs.0.set_dampening(dampening);
            combs.1.set_dampening(dampening);
        }
    }

    pub fn set_dry(&mut self, value: f32) {
        self.dry = value;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ticking_does_something() {
        let mut freeverb = super::Freeverb::new(super::SAMPLING_RATE);
        assert_eq!(freeverb.tick((1.0, 1.0)), (0.0, 0.0));
        for _ in 0..(1640 * 4) {
            freeverb.tick((0.0, 0.0));
        }
        assert_ne!(freeverb.tick((0.0, 0.0)), (0.0, 0.0));
    }
}
