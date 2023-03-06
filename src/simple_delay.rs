use core::ops::Neg;

use crate::SAMPLING_RATE;
use embedded_audio_tools as tools;

use tools::{mut_mem_slice::from_slice, DelayLine, MutMemSlice};

const MIN_DELAY_SAMPLES: usize = 32;
const MAX_DELAY_SAMPLES: usize = 144_000;

pub struct SimpleDelay {
    static_buffer: [f32; MAX_DELAY_SAMPLES],
    delay_line: tools::DelayLine,
    delay_samples: f32,
    feedback: f32,
    dry_gain: f32,
    wet_gain: f32,
}

impl SimpleDelay {
    pub fn init(sr: usize) -> SimpleDelay {
        assert_eq!(
            sr, SAMPLING_RATE,
            "This delay owns memory on which it the delay lines sit. To safe on memory usage, its size is calculated at compile time!"
        );

        SimpleDelay {
            static_buffer: [0.0_f32; MAX_DELAY_SAMPLES],
            delay_line: (DelayLine::new(MutMemSlice::null())),
            delay_samples: 0.5 * MAX_DELAY_SAMPLES as f32,
            feedback: 0.5,
            dry_gain: 0.0,
            wet_gain: 1.0,
        }
    }

    /// Checks if static buffer placement in memory and the pointers of the delay lines align. If not, it aligns it.
    ///
    /// Happens normally only once, after the object has been created.
    fn check_buffer_alignment(&mut self) {
        let buffer_start = core::ptr::addr_of_mut!(self.static_buffer[0]);
        let pointer_start = self.delay_line.buffer.ptr.0;

        if buffer_start != pointer_start {
            self.delay_line = DelayLine::new(from_slice(&mut self.static_buffer[..]));
        }
    }

    pub fn tick(&mut self, input: f32) -> f32 {
        self.check_buffer_alignment();

        let delayed_sample = self
            .delay_line
            .read_lerp_wrapped_at(self.delay_samples.neg());

        let output = delayed_sample * self.feedback;

        self.delay_line.write_and_advance(input + output);

        self.dry_gain * input + self.wet_gain * output
    }

    pub fn set_delay_in_secs(&mut self, delay: f32) {
        self.delay_samples = (delay * SAMPLING_RATE as f32)
            .clamp(MIN_DELAY_SAMPLES as f32, MAX_DELAY_SAMPLES as f32);
    }

    pub fn set_delay_in_ms(&mut self, delay: f32) {
        self.delay_samples = ((delay * SAMPLING_RATE as f32) / 1000.0)
            .clamp(MIN_DELAY_SAMPLES as f32, MAX_DELAY_SAMPLES as f32);
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
}

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
        let buffer_start = addr_of_mut!(delay.static_buffer[0]);
        let pointer_start = delay.delay_line.buffer.ptr.0;

        assert_ne!(buffer_start, pointer_start);

        delay.check_buffer_alignment();

        // Update pointer and check alignment at start
        let pointer_start = delay.delay_line.buffer.ptr.0;

        assert_eq!(
            buffer_start, pointer_start,
            "buffer and pointer are not aligned"
        );
    }
}
