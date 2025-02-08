use crate::Freeverb;

use super::FreeverbParams;

/// Initializes `Freeverb` struct. `buffer` needs to be `length >= 25450` for `sr = 48000`. Otherwise will panic!
#[no_mangle]
unsafe extern "C" fn freeverb_init(sr: f32, buffer: *mut f32, length: usize) -> Freeverb {
    Freeverb::new(sr as usize, core::slice::from_raw_parts_mut(buffer, length))
}

/// Returns next stereo samples. Raw pointer `stereo_samples` assumes to have exactly two elements!
#[no_mangle]
unsafe extern "C" fn freeverb_tick(ptr: *mut Freeverb, stereo_samples: *mut f32) {
    let samples = core::slice::from_raw_parts_mut(stereo_samples, 2);

    (samples[0], samples[1]) = ptr
        .as_mut()
        .unwrap_unchecked()
        .tick((samples[0], samples[1]));
}

/// Sample rate depending calculations should be performed earlier!
#[no_mangle]
unsafe extern "C" fn freeverb_set_all_params(ptr: *mut Freeverb, params: *mut FreeverbParams) {
    ptr.as_mut()
        .unwrap_unchecked()
        .set_all(params.as_ref().unwrap_unchecked());
}
