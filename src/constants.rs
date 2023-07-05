use biquad::{Hertz, ToHertz};

pub const PIN: i32 = 21;
pub const NUM_LEDS: i32 = 100;
pub const BRIGHTNESS: u8 = 100;

pub const FREQ_LPF: f64 = 55.0;
pub const FREQ_HPF: f64 = 10000.0;

pub const AUDIO_AVERAGE_SECONDS: f32 = 0.03;
