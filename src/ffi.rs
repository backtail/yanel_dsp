use crate::DSPUtility;

#[no_mangle]
extern "C" fn f32_samples_to_seconds(val: f32, sr: f32) -> f32 {
    val.samples_to_seconds(sr)
}

#[no_mangle]
extern "C" fn f32_samples_to_millis(val: f32, sr: f32) -> f32 {
    val.samples_to_millis(sr)
}

#[no_mangle]
extern "C" fn f32_seconds_to_samples(val: f32, sr: f32) -> f32 {
    val.seconds_to_samples(sr)
}

#[no_mangle]
extern "C" fn f32_millis_to_samples(val: f32, sr: f32) -> f32 {
    val.millis_to_samples(sr)
}
