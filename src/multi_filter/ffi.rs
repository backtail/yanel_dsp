use crate::multi_filter::ButterworthType;
use crate::MultiFilter;

/// Initializes `MultiFilter` struct.
#[no_mangle]
unsafe extern "C" fn multifilter_init(sr: f32) -> MultiFilter {
    MultiFilter::init(sr as usize)
}

/// Returns next stereo samples. Raw pointer `stereo_samples` assumes to have exactly two elements!
#[no_mangle]
unsafe extern "C" fn multifilter_tick(ptr: *mut MultiFilter, sample: f32) -> f32 {
    ptr.as_mut().unwrap_unchecked().next(sample)
}

/// Sample rate depending calculations should be performed earlier!
#[no_mangle]
unsafe extern "C" fn multifilter_set_all_params(
    ptr: *mut MultiFilter,
    filter: ButterworthType,
    freq: f32,
    q: f32,
    gain: f32,
) {
    ptr.as_mut()
        .unwrap_unchecked()
        .set_all(filter, freq, q, gain);
}
