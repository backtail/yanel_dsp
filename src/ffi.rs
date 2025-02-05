use crate::SimpleDelay;

/// Initializes `SimpleDelay` struct
#[no_mangle]
extern "C" fn simple_delay_init() -> SimpleDelay {
    SimpleDelay::init()
}

/// Initializes `SimpleDelay` delay buffer
#[no_mangle]
unsafe extern "C" fn simple_delay_set_buffer(
    ptr: *mut SimpleDelay,
    buffer: *mut f32,
    length: usize,
) {
    ptr.as_mut()
        .unwrap_unchecked()
        .set_buffer(core::slice::from_raw_parts_mut(buffer, length));
}

/// Returns next sample
#[no_mangle]
unsafe extern "C" fn simple_delay_tick(ptr: *mut SimpleDelay, sample: f32) -> f32 {
    ptr.as_mut().unwrap_unchecked().tick(sample)
}

/// Sample rate depending calculations should be performed earlier!
#[no_mangle]
unsafe extern "C" fn simple_delay_set_delay_length(ptr: *mut SimpleDelay, len_in_samples: f32) {
    ptr.as_mut().unwrap_unchecked().set_delay(len_in_samples);
}

/// Feedback can reach max. 100%
#[no_mangle]
unsafe extern "C" fn simple_delay_set_feedback(ptr: *mut SimpleDelay, feedback: f32) {
    ptr.as_mut().unwrap_unchecked().set_feedback(feedback);
}

/// Dry/Wet mixing
#[no_mangle]
unsafe extern "C" fn simple_delay_set_mix(ptr: *mut SimpleDelay, mix: f32) {
    ptr.as_mut()
        .unwrap_unchecked()
        .set_dry(1.0 - mix.clamp(0.0, 1.0));
    ptr.as_mut().unwrap_unchecked().set_wet(mix);
}
