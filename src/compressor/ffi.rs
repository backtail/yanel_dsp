use crate::Compressor;

/// Initializes `Compressor` struct
#[no_mangle]
extern "C" fn compressor_init(sr: f32) -> Compressor {
    Compressor::init(sr)
}

/// Returns next sample
#[no_mangle]
unsafe extern "C" fn compressor_tick(ptr: *mut Compressor, sample: f32) -> f32 {
    ptr.as_mut().unwrap_unchecked().tick(sample + f32::EPSILON)
}

#[no_mangle]
unsafe extern "C" fn compressor_set_attack(ptr: *mut Compressor, val: f32) {
    ptr.as_mut()
        .unwrap_unchecked()
        .set_attack(val / 50.0 + 0.01);
}

#[no_mangle]
unsafe extern "C" fn compressor_set_release(ptr: *mut Compressor, val: f32) {
    ptr.as_mut()
        .unwrap_unchecked()
        .set_release(val / 5.0 + 10.0);
}

#[no_mangle]
unsafe extern "C" fn compressor_set_attack_slope(ptr: *mut Compressor, val: f32) {
    ptr.as_mut().unwrap_unchecked().set_attack_slope(val * 10.0);
}

#[no_mangle]
unsafe extern "C" fn compressor_set_release_slope(ptr: *mut Compressor, val: f32) {
    ptr.as_mut()
        .unwrap_unchecked()
        .set_release_slope(val * 10.0);
}

#[no_mangle]
unsafe extern "C" fn compressor_set_ratio(ptr: *mut Compressor, val: f32) {
    ptr.as_mut().unwrap_unchecked().set_ratio(val * 100.0 + 1.0);
}

#[no_mangle]
unsafe extern "C" fn compressor_set_threshold(ptr: *mut Compressor, val: f32) {
    ptr.as_mut().unwrap_unchecked().set_threshold(val * 0.5);
}
