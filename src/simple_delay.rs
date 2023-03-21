use core::ops::Neg;

use crate::tools::{
    memory_access::{from_slice_mut, null_mut},
    stereo::crossfade_correlated_unchecked,
    DelayLine,
};
use crate::SAMPLING_RATE;

const MIN_DELAY_SAMPLES: usize = 32;
const MAX_DELAY_SAMPLES: usize = 144_000;

pub struct SimpleDelay {
    buffer: [f32; MAX_DELAY_SAMPLES],

    delay_line: crate::tools::DelayLine,

    delay_samples: f32,
    feedback: f32,
    dry_gain: f32,
    wet_gain: f32,

    delay_time_changed: bool,
    last_delay_samples: f32,
    crossfade_counter: usize,
    crossfade_samples: usize,
}

impl SimpleDelay {
    pub fn init(sr: usize) -> SimpleDelay {
        assert_eq!(
            sr, SAMPLING_RATE,
            "This delay owns memory on which it the delay lines sit. To safe on memory usage, its size is calculated at compile time!"
        );

        SimpleDelay {
            buffer: [0.0_f32; MAX_DELAY_SAMPLES],

            delay_line: (DelayLine::new(null_mut())),

            delay_samples: 0.5 * MAX_DELAY_SAMPLES as f32,
            feedback: 0.5,
            dry_gain: 0.0,
            wet_gain: 1.0,

            delay_time_changed: false,
            last_delay_samples: 0.0,
            crossfade_counter: 0,
            crossfade_samples: (0.01 * SAMPLING_RATE as f32) as usize, // 10ms
        }
    }

    ///////////////////////////////////////////////////////////////////////////////
    /// Public Interface
    ///////////////////////////////////////////////////////////////////////////////

    /// Checks if static buffer placement in memory and the pointers of the delay lines align. If not, it aligns it.
    ///
    /// Happens normally only once, after the object has been created.
    pub fn check_buffer_alignment(&mut self) {
        let buffer_start = core::ptr::addr_of_mut!(self.buffer[..]);
        let pointer_start = self.delay_line.get_ptr_slice_mut();

        if buffer_start != pointer_start {
            self.delay_line = DelayLine::new(from_slice_mut(&mut self.buffer[..]));
        }
    }

    pub fn tick(&mut self, input: f32) -> f32 {
        let output = self.get_delayed_sample() * self.feedback;

        self.delay_line.write_and_advance(input + output);

        self.dry_gain * input + self.wet_gain * output
    }

    pub fn set_delay_in_secs(&mut self, delay: f32) {
        let new_delay = (delay * SAMPLING_RATE as f32)
            .clamp(MIN_DELAY_SAMPLES as f32, MAX_DELAY_SAMPLES as f32);

        if new_delay != self.delay_samples {
            self.last_delay_samples = self.delay_samples;
            self.delay_time_changed = true;
        }

        self.delay_samples = new_delay;
    }

    pub fn set_delay_in_ms(&mut self, delay: f32) {
        let new_delay = ((delay * SAMPLING_RATE as f32) / 1000.0)
            .clamp(MIN_DELAY_SAMPLES as f32, MAX_DELAY_SAMPLES as f32);

        if new_delay != self.delay_samples {
            self.last_delay_samples = self.delay_samples;
            self.delay_time_changed = true;
        }

        self.delay_samples = new_delay;
    }

    pub fn set_feedback(&mut self, feedback: f32) {
        self.feedback = feedback.clamp(0.0, 1.0);
    }

    pub fn set_dry(&mut self, dry_gain: f32) {
        self.dry_gain = dry_gain.clamp(0.0, 1.0);
    }

    pub fn set_wet(&mut self, wet_gain: f32) {
        self.wet_gain = wet_gain.clamp(0.0, 1.0);
    }

    pub fn set_crossfade_in_ms(&mut self, fade_time: f32) {
        self.crossfade_samples = (fade_time * 0.001 * SAMPLING_RATE as f32) as usize;
    }

    ///////////////////////////////////////////////////////////////////////////////
    /// Private Functions
    ///////////////////////////////////////////////////////////////////////////////

    fn get_delayed_sample(&mut self) -> f32 {
        // get delayed sample from newest delay time
        let new_delayed = self
            .delay_line
            .read_lerp_wrapped_at(self.delay_samples.neg());

        // crossfade between new and old delay time samples
        if self.delay_time_changed {
            if self.crossfade_counter < self.crossfade_samples {
                self.crossfade_counter += 1;

                return crossfade_correlated_unchecked(
                    self.get_normalized_bipolar_crossfade(),
                    (
                        self.delay_line
                            .read_lerp_wrapped_at(self.last_delay_samples.neg()),
                        new_delayed,
                    ),
                );
            } else {
                self.delay_time_changed = false;
                self.crossfade_counter = 0;

                return new_delayed;
            }
        } else {
            return new_delayed;
        }
    }

    #[inline(always)]
    fn get_normalized_bipolar_crossfade(&self) -> f32 {
        (self.crossfade_counter as f32 / self.crossfade_samples as f32) * 2.0 - 1.0
    }
}

///////////////////////////////////////////////////////////////////////////////
/// Unit Tests
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ticking_delay() {
        let feedback_gain = 0.5;
        let delay_time = 1.0; //ms
        let delay_samples = ((delay_time * SAMPLING_RATE as f32) / 1000.0) as usize;

        let mut delay = SimpleDelay::init(SAMPLING_RATE);
        delay.set_dry(1.0);
        delay.set_wet(1.0);
        delay.set_feedback(feedback_gain);
        delay.set_delay_in_ms(delay_time);
        delay.check_buffer_alignment();

        // pass by crossfade
        for _ in 0..delay.crossfade_samples + 1 {
            delay.tick(0.0);
        }

        assert_eq!(
            delay.tick(1.0),
            1.0,
            "first sample was not the input sample"
        );

        for i in 0..delay_samples - 1 {
            assert_eq!(delay.tick(0.0), 0.0, "index was not muted: {}", i);
        }

        assert_eq!(
            delay.tick(0.0),
            feedback_gain,
            "delayed sample was not the feedback"
        );
    }

    #[test]
    fn buffer_alignment() {
        use core::ptr::addr_of_mut;

        let mut delay = SimpleDelay::init(SAMPLING_RATE);

        // Alignment should fail, since pointers are initiated as null
        let buffer_start = addr_of_mut!(delay.buffer[..]);
        let pointer_start = delay.delay_line.get_ptr_slice_mut();

        assert_ne!(buffer_start, pointer_start);

        delay.check_buffer_alignment();

        // Update pointer and check alignment at start
        let pointer_start = delay.delay_line.get_ptr_slice_mut();

        assert_eq!(
            buffer_start, pointer_start,
            "buffer and pointer are not aligned"
        );
    }

    #[test]
    fn crossfade_bounds() {
        let mut delay = SimpleDelay::init(SAMPLING_RATE);
        delay.crossfade_counter = 0;

        assert_eq!(delay.get_normalized_bipolar_crossfade(), -1.0);

        for _ in 0..delay.crossfade_samples - 1 {
            delay.crossfade_counter += 1;
            let crossfade = delay.get_normalized_bipolar_crossfade();
            assert!(crossfade > -1.0 && crossfade < 1.0);
        }

        delay.crossfade_counter += 1;
        assert_eq!(delay.get_normalized_bipolar_crossfade(), 1.0);
    }
}
