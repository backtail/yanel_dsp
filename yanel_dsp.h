#ifndef _YANEL_DSP_H_
#define _YANEL_DSP_H_

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>


/*
 Raw mutable pointer that implements the `Send` trait since it's only acting on stack memory

 Should always point at the beginning of your audio buffer in use
 */
typedef float *Mutable;

/*
 Struct
 Raw slice pointer that implements the `Send` trait since it **only** works **safely** on **statically allocated memory**.

## Example

``` use embedded_audio_tools::memory_access::*;

// Thread-safe non-mutable slice let buffer = [0.0_f32; 24]; let non_mut_slice = from_slice(&buffer[..]);

// Thread-safe mutable slice let mut buffer = [0.0_f32; 24]; let mut mut_slice = from_slice_mut(&mut buffer[..]);

// Null pointer and length of 0 let mut ptr_buffer = null_mut();

// Change associated buffer in runtime unsafe {     ptr_buffer.change_mut_slice_unchecked(buffer.as_mut_ptr(), buffer.len()); }

assert_eq!(ptr_buffer.as_slice(), mut_slice.as_slice()); ```


 */
typedef struct MemorySlice_Mutable {
    Mutable ptr;
    size_t length;
} MemorySlice_Mutable;

typedef struct DelayLine {
    struct MemorySlice_Mutable buffer;
    size_t index;
} DelayLine;

typedef struct SimpleDelay {
    struct DelayLine delay_line;
    float delay_samples;
    float feedback;
    float dry_gain;
    float wet_gain;
    bool delay_time_changed;
    float last_delay_samples;
    size_t crossfade_counter;
    size_t crossfade_samples;
} SimpleDelay;

/*
 Initializes `SimpleDelay` struct
 */
struct SimpleDelay simple_delay_init(void);

/*
 Initializes `SimpleDelay` delay buffer
 */
void simple_delay_set_buffer(struct SimpleDelay *ptr, float *buffer, size_t length);

/*
 Returns next sample
 */
float simple_delay_tick(struct SimpleDelay *ptr, float sample);

#endif  /* _YANEL_DSP_H_ */
