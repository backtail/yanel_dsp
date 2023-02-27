# Yanel DSP
The Yanel DSP (**y**et **an**other **e**ffects **l**ibrary) is
written and envisioned for embedded targets!

 `#![no_std]` compatibility guaranteed!

### Examples
Run examples of the effects on your local machine with:
```shell
cargo run --release --example={EXAMPLE} --features="gtk-app"
```

The `gtk` framework needs to be installed for this to work!

### Freeverb
A [reverb](https://github.com/irh/freeverb-rs/) inpsired by [Ian Hobsen](https://github.com/irh) of his [ADC18 talk](https://www.youtube.com/watch?v=Yom9E-67bdI) but statically allocated delay lines and `#![no_std]` compatible!