mod ffi;

///////////////
// TODO: Calculate required buffer allocations for common sample rates!
///////////////
use crate::tools::memory_access::{from_slice_mut, null_mut};
use crate::tools::{AllPass, Comb};

/// cbindgen:ignore
const STEREO_SPREAD: usize = 23; // at 48 kHz

/// cbindgen:ignore
const FIXED_GAIN: f32 = 0.015;

/// cbindgen:ignore
const SCALE_WET: f32 = 3.0;

/// cbindgen:ignore
const SCALE_DAMPENING: f32 = 0.4;

/// cbindgen:ignore
const SCALE_ROOM: f32 = 0.28;

/// cbindgen:ignore
const OFFSET_ROOM: f32 = 0.7;

#[repr(C)]
pub struct FreeverbParams {
    width: f32,
    dampening: f32,
    room_size: f32,
    frozen: bool,
    mix: f32,
}

#[repr(C)]
pub struct Freeverb {
    combs_l: [Comb; 8],
    combs_r: [Comb; 8],
    allpasses_l: [AllPass; 4],
    allpasses_r: [AllPass; 4],

    params: FreeverbParams,

    wet_gain_l: f32,
    wet_gain_r: f32,
    input_gain: f32,
    dry: f32,
    wet: f32,
}

impl Freeverb {
    pub fn new(sr: usize, buffer: &mut [f32]) -> Self {
        // freeverb specific tuning of filters
        let mut tunings = [
            1116,                 // COMB_TUNING_L1
            1116 + STEREO_SPREAD, // COMB_TUNING_R1
            1188,                 // COMB_TUNING_L2
            1188 + STEREO_SPREAD, // COMB_TUNING_R2
            1277,                 // COMB_TUNING_L3
            1277 + STEREO_SPREAD, // COMB_TUNING_R3
            1356,                 // COMB_TUNING_L4
            1356 + STEREO_SPREAD, // COMB_TUNING_R4
            1422,                 // COMB_TUNING_L5
            1422 + STEREO_SPREAD, // COMB_TUNING_R5
            1491,                 // COMB_TUNING_L6
            1491 + STEREO_SPREAD, // COMB_TUNING_R6
            1557,                 // COMB_TUNING_L7
            1557 + STEREO_SPREAD, // COMB_TUNING_R7
            1617,                 // COMB_TUNING_L8
            1617 + STEREO_SPREAD, // COMB_TUNING_R8
            556,                  // ALLPASS_TUNING_L1
            556 + STEREO_SPREAD,  // ALLPASS_TUNING_R1
            441,                  // ALLPASS_TUNING_L2
            441 + STEREO_SPREAD,  // ALLPASS_TUNING_R2
            341,                  // ALLPASS_TUNING_L3
            341 + STEREO_SPREAD,  // ALLPASS_TUNING_R3
            225,                  // ALLPASS_TUNING_L4
            225 + STEREO_SPREAD,  // ALLPASS_TUNING_R4
        ];

        // adjust to sample rate
        tunings
            .iter_mut()
            .for_each(|tuning| adjust_length(tuning, sr));

        // only continue, if at least required memory allocation is passed
        assert!(
            buffer.len() >= tunings.iter().sum(),
            "Plaese provide enough mutable memory!"
        );

        // create the freeverb object
        let mut freeverb = Freeverb {
            // reserve memory for delay lines and initiate null pointers
            combs_l: [Comb::new(null_mut()); 8],
            combs_r: [Comb::new(null_mut()); 8],
            allpasses_l: [AllPass::new(null_mut()); 4],
            allpasses_r: [AllPass::new(null_mut()); 4],
            wet_gain_l: 0.0,
            wet_gain_r: 0.0,
            input_gain: 0.0,
            wet: 0.0,
            dry: 0.0,
            params: FreeverbParams {
                width: 0.0,
                dampening: 0.0,
                room_size: 0.0,
                frozen: false,
                mix: 0.0,
            },
        };

        // configure
        freeverb.align_buffers(buffer, tunings);
        freeverb.set_wet(1.0);
        freeverb.set_width(0.5);
        freeverb.set_dampening(0.5);
        freeverb.set_room_size(0.5);
        freeverb.set_frozen(false);

        freeverb
    }

    pub fn tick(&mut self, input: (f32, f32)) -> (f32, f32) {
        let input_mixed = (input.0 + input.1) * FIXED_GAIN * self.input_gain;

        let mut out = (0.0, 0.0);

        for combs in core::iter::zip(self.combs_l.iter_mut(), self.combs_r.iter_mut()) {
            out.0 += combs.0.tick(input_mixed);
            out.1 += combs.1.tick(input_mixed);
        }

        for allpasses in core::iter::zip(self.allpasses_l.iter_mut(), self.allpasses_r.iter_mut()) {
            out.0 = allpasses.0.tick(out.0);
            out.1 = allpasses.1.tick(out.1);
        }

        (
            out.0 * self.wet_gain_l + out.1 * self.wet_gain_r + input.0 * self.dry,
            out.1 * self.wet_gain_l + out.0 * self.wet_gain_r + input.1 * self.dry,
        )
    }

    fn align_buffers(&mut self, buffer: &mut [f32], tunings: [usize; 24]) {
        let mut offset = 0;
        // Give delay lines the approriate memory strips on buffer
        for (i, _) in tunings.iter().enumerate().step_by(2) {
            let stage = i / 2;
            if i < 16 {
                self.combs_l[stage]
                    .change_buffer(from_slice_mut(&mut buffer[offset..offset + tunings[i]]));
                offset += tunings[i];

                self.combs_r[stage]
                    .change_buffer(from_slice_mut(&mut buffer[offset..offset + tunings[i + 1]]));
                offset += tunings[i + 1];
            } else {
                self.allpasses_l[stage - 8]
                    .change_buffer(from_slice_mut(&mut buffer[offset..offset + tunings[i]]));
                offset += tunings[i];

                self.allpasses_r[stage - 8]
                    .change_buffer(from_slice_mut(&mut buffer[offset..offset + tunings[i + 1]]));
                offset += tunings[i + 1];
            }
        }
    }

    pub fn set_dampening(&mut self, value: f32) {
        self.params.dampening = value * SCALE_DAMPENING;
        self.update_combs();
    }

    pub fn set_freeze(&mut self, frozen: bool) {
        self.params.frozen = frozen;
        self.update_combs();
    }

    pub fn set_wet(&mut self, value: f32) {
        self.wet = value * SCALE_WET;
        self.update_wet_gains();
    }

    pub fn set_width(&mut self, value: f32) {
        self.params.width = value;
        self.update_wet_gains();
    }

    fn update_wet_gains(&mut self) {
        self.wet_gain_l = self.wet * (self.params.width / 2.0 + 0.5);
        self.wet_gain_l = self.wet * ((1.0 - self.params.width) / 2.0);
    }

    fn set_frozen(&mut self, frozen: bool) {
        self.params.frozen = frozen;
        self.input_gain = if frozen { 0.0 } else { 1.0 };
        self.update_combs();
    }

    pub fn set_room_size(&mut self, value: f32) {
        self.params.room_size = value * SCALE_ROOM + OFFSET_ROOM;
        self.update_combs();
    }

    fn update_combs(&mut self) {
        let (feedback, dampening) = if self.params.frozen {
            (1.0, 0.0)
        } else {
            (self.params.room_size, self.params.dampening)
        };

        for combs in core::iter::zip(self.combs_l.iter_mut(), self.combs_r.iter_mut()) {
            combs.0.set_feedback(feedback);
            combs.1.set_feedback(feedback);

            combs.0.set_dampening(dampening);
            combs.1.set_dampening(dampening);
        }
    }

    pub fn set_dry(&mut self, value: f32) {
        self.dry = value;
    }

    pub fn set_all(&mut self, new: &FreeverbParams) {
        self.params.dampening = new.dampening * SCALE_DAMPENING;
        self.params.room_size = new.room_size * SCALE_ROOM + OFFSET_ROOM;
        self.params.width = new.width;
        self.params.frozen = new.frozen;
        self.params.mix = new.mix;

        self.input_gain = if new.frozen { 0.0 } else { 1.0 };
        self.dry = 1.0 - new.mix;
        self.wet = new.mix * SCALE_WET;

        self.update_combs();
        self.update_wet_gains();
    }
}

fn adjust_length(length: &mut usize, sr: usize) {
    *length = *length * sr / 44100
}

#[cfg(test)]
mod tests {
    #[test]
    fn ticking_does_something() {
        let mut buffer = [0_f32; 48000];
        let mut freeverb = super::Freeverb::new(48000, buffer.as_mut_slice());
        assert_eq!(freeverb.tick((1.0, 1.0)), (0.0, 0.0));
        for _ in 0..(1640 * 4) {
            freeverb.tick((0.0, 0.0));
        }
        assert_ne!(freeverb.tick((0.0, 0.0)), (0.0, 0.0));
    }
}
