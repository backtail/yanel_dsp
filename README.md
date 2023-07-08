# Yanel DSP
The Yanel DSP (**y**et **an**other **e**ffects **l**ibrary) is
written and envisioned for embedded targets!

 `#![no_std]` compatibility guaranteed!

### Freeverb
A [reverb](https://github.com/irh/freeverb-rs/) inspired by [Ian Hobsen](https://github.com/irh) of his [ADC18 talk](https://www.youtube.com/watch?v=Yom9E-67bdI) but `#![no_std]` compatible!

### Simple Delay
Fractional delay without popping (while changing delay time).

### Multi Filter
A variable state filter with
- Lowpass
- Highpass
- Allpass
- Notch
- Bell Curve
- Low Shelf

and variable Q/Gain!

## Examples
There are a few implementations as VST3 plugins with the [nih-plug](https://github.com/robbert-vdh/nih-plug) crate:
- Multi Filter (no GUI)
- Freeverb
- Simple Delay