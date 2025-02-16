mod ffi;

use embedded_audio_tools::{
    envelope::MultiStageEnvelope, float::lerp_unchecked, FunctionalOscillator, PhaseAccumulator,
    SoftPhaseAccumulator,
};

#[allow(unused_imports)]
use embedded_audio_tools::F32Ext;

/// cbindgen:ignore
const SYNTH_KICK_LOWEST_DRIVE: f32 = 1.0;

/// cbindgen:ignore
const SHORTEST_ATTACK: f32 = 0.001; // s
/// cbindgen:ignore
const LONGEST_ATTACK: f32 = 0.030; // s

/// cbindgen:ignore
const SHORTEST_DECAY: f32 = 0.100; // s
/// cbindgen:ignore
const LONGEST_DECAY: f32 = 5.0; // s

/// cbindgen:ignore
const LOWEST_PITCH: f32 = 25.0; // Hz
/// cbindgen:ignore
const HIGHEST_PITCH: f32 = 200.0; // Hz
/// cbindgen:ignore
const DEFAULT_PITCH: f32 = 40.0; // Hz
/// cbindgen:ignore
const DEFAULT_PITCH_RANGE: f32 = 1.5; // Hz

/// cbindgen:ignore
const FADE_OUT: f32 = 0.035; // s

#[derive(PartialEq)]
#[repr(C)]
enum KickState {
    Idle,
    Triggered,
    Retriggered,
}

#[repr(C)]
pub struct SynthKick {
    // Audio Tools
    pitch_env: MultiStageEnvelope<3>,
    volume_env: MultiStageEnvelope<3>,
    osc: FunctionalOscillator<SoftPhaseAccumulator>,

    // State
    sr: f32,
    current_sample: f32,
    state: KickState,
    global_pitch: f32,
    global_pitch_range: f32,
    retrigger_slope: f32,
    retrigger_fade_out_amp: f32,

    // Params
    overdrive: f32,
    od_param: f32,
}

impl SynthKick {
    pub fn init(sr: f32) -> SynthKick {
        let mut kick = SynthKick {
            pitch_env: MultiStageEnvelope::new(1.0),
            volume_env: MultiStageEnvelope::new(0.0),
            osc: FunctionalOscillator::new(SoftPhaseAccumulator::new(DEFAULT_PITCH, sr)),

            sr,
            current_sample: 0.0,
            state: KickState::Idle,
            global_pitch: DEFAULT_PITCH,
            global_pitch_range: DEFAULT_PITCH_RANGE,
            retrigger_slope: 1.0 / (FADE_OUT * sr),
            retrigger_fade_out_amp: 1.0,

            overdrive: SYNTH_KICK_LOWEST_DRIVE,
            od_param: 1.0,
        };

        kick.pitch_env.set_all(0, 0.020, 0.8, 0.0, sr);
        kick.pitch_env.set_all(1, 2.0, 0.0, 5.0, sr);

        kick.volume_env.set_all(0, 0.05, 1.0, 0.0, sr);
        kick.volume_env.set_all(1, 0.3, 0.0, 10.0, sr);

        kick
    }

    pub fn trigger(&mut self) {
        match self.state {
            KickState::Idle => {
                self.pitch_env.reset(1.0);
                self.pitch_env.trigger();

                self.volume_env.reset(0.0);
                self.volume_env.trigger();

                self.osc.set_phase_shift_unchecked(0.0); // reset phase

                self.state = KickState::Triggered;
            }

            KickState::Triggered => {
                self.retrigger_fade_out_amp = 1.0;

                self.state = KickState::Retriggered;
            }

            KickState::Retriggered => {} // already retriggered
        }
    }

    pub fn tick(&mut self) -> f32 {
        match self.state {
            KickState::Idle => self.current_sample = 0.0,

            KickState::Triggered => {
                if self.volume_env.get_stage() == -1 {
                    self.state = KickState::Idle;
                }

                // apply pitch envelope
                self.osc.set_freq_unchecked(
                    self.global_pitch
                        + self.global_pitch * self.global_pitch_range.exp() * self.pitch_env.tick(),
                );

                // apply volume curve
                self.current_sample = self.volume_env.tick() * self.osc.next();
            }

            KickState::Retriggered => {
                if (self.current_sample.is_sign_positive() && self.current_sample >= 0.0001)
                    || (self.current_sample.is_sign_negative() && self.current_sample <= -0.0001)
                {
                    self.retrigger_fade_out_amp -= self.retrigger_slope;
                    self.current_sample *= self.retrigger_fade_out_amp;
                } else {
                    self.state = KickState::Idle;

                    self.trigger();
                }
            }
        }

        return self.current_sample;
    }

    pub fn update_sr(&mut self, sr: f32) {
        self.sr = sr;
        self.osc.set_sr_unchecked(sr);
        self.retrigger_slope = 1.0 / (FADE_OUT * sr); // quick inaudible fade out
    }

    pub fn set_overdrive(&mut self, val: f32) {
        if val >= SYNTH_KICK_LOWEST_DRIVE {
            self.overdrive = val;
        } else {
            self.overdrive = SYNTH_KICK_LOWEST_DRIVE;
        }
    }

    pub fn set_overdrive_param(&mut self, val: f32) {
        self.od_param = val;
    }

    /// Only accepts values between 0.0 and 1.0, otherwise clamps
    pub fn set_attack(&mut self, val: f32) {
        self.volume_env.set_all(
            0,
            lerp_unchecked(SHORTEST_ATTACK, LONGEST_ATTACK, val.clamp(0.0, 1.0)),
            1.0,
            0.0,
            self.sr,
        );
    }

    /// Only accepts values between 0.0 and 1.0, otherwise clamps
    pub fn set_decay(&mut self, val: f32) {
        self.volume_env.set_all(
            1,
            lerp_unchecked(SHORTEST_DECAY, LONGEST_DECAY, val.clamp(0.0, 1.0)),
            0.0,
            10.0,
            self.sr,
        );
    }

    /// Only accepts values between 0.0 and 1.0, otherwise clamps
    pub fn set_decay_pitch(&mut self, _val: f32) {}

    /// Only accepts values between 0.0 and 1.0, otherwise clamps
    pub fn set_pitch(&mut self, val: f32) {
        self.global_pitch = lerp_unchecked(LOWEST_PITCH, HIGHEST_PITCH, val.clamp(0.0, 1.0));
    }

    /// Only accepts values between -1.0 and 1.0, otherwise clamps
    pub fn set_env_slope(&mut self, _val: f32) {}
}
