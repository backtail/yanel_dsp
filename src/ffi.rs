use crate::SimpleDelay;

/// Initializes `SimpleDelay` struct
#[no_mangle]
extern "C" fn simple_delay_init() -> SimpleDelay {
    SimpleDelay::init()
}

/// Initializes `SimpleDelay` delay buffer
#[no_mangle]
extern "C" fn simple_delay_set_buffer(ptr: *mut SimpleDelay, buffer: *mut f32, length: usize) {
    unsafe {
        ptr.as_mut()
            .unwrap_unchecked()
            .set_buffer(core::slice::from_raw_parts_mut(buffer, length));
    }
}

/// Returns next sample
#[no_mangle]
extern "C" fn simple_delay_tick(ptr: *mut SimpleDelay, sample: f32) -> f32 {
    unsafe { ptr.as_mut().unwrap_unchecked().tick(sample) }
}
