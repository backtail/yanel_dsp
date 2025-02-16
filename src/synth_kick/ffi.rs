use crate::SynthKick;

/// Initializes `SynthKick` struct
#[no_mangle]
extern "C" fn synth_kick_init(sr: f32) -> SynthKick {
    SynthKick::init(sr)
}

/// Triggers the kick
#[no_mangle]
unsafe extern "C" fn synth_kick_trigger(ptr: *mut SynthKick) {
    ptr.as_mut().unwrap_unchecked().trigger();
}

/// Returns next sample
#[no_mangle]
unsafe extern "C" fn synth_kick_tick(ptr: *mut SynthKick) -> f32 {
    ptr.as_mut().unwrap_unchecked().tick()
}

/// Only accepts values between 0.0 and 1.0, otherwise clamps
#[no_mangle]
unsafe extern "C" fn synth_kick_set_attack(ptr: *mut SynthKick, val: f32) {
    ptr.as_mut().unwrap_unchecked().set_attack(val);
}

/// Only accepts values between 0.0 and 1.0, otherwise clamps
#[no_mangle]
unsafe extern "C" fn synth_kick_set_decay(ptr: *mut SynthKick, val: f32) {
    ptr.as_mut().unwrap_unchecked().set_decay(val);
}

/// Only accepts values between 0.0 and 1.0, otherwise clamps
#[no_mangle]
unsafe extern "C" fn synth_kick_set_decay_pitch(ptr: *mut SynthKick, val: f32) {
    ptr.as_mut().unwrap_unchecked().set_decay_pitch(val);
}

/// Only accepts values between 0.0 and 1.0, otherwise clamps
#[no_mangle]
unsafe extern "C" fn synth_kick_set_pitch(ptr: *mut SynthKick, val: f32) {
    ptr.as_mut().unwrap_unchecked().set_pitch(val);
}

/// Only accepts values between -1.0 and 1.0, otherwise clamps
#[no_mangle]
unsafe extern "C" fn synth_kick_set_env_slope(ptr: *mut SynthKick, val: f32) {
    ptr.as_mut().unwrap_unchecked().set_env_slope(val);
}
