use std::time::Duration;

use colorsys::Rgb;

use crate::{audio::AudioFeatures, constants::NUM_LEDS};

pub struct Scene {
    leds: [Rgb; NUM_LEDS],
}

impl Scene {
    pub fn new() -> Self {
        Self {
            leds: core::array::from_fn(|_| Rgb::new(0.0, 0.0, 0.0, None)),
        }
    }

    pub fn tick(&mut self, _time_since_last_tick: Duration, total_time: Duration) {
        for (i, led) in self.leds.iter_mut().enumerate() {
            // let mut red =
            // (total_time.as_secs_f32() as f64 * -8.0 + i as f64 * 0.2).sin() * 0.5 + 0.5;
            // let green = (total_time.as_secs_f32() as f64 * -6.1 + i as f64 * 0.1).sin() * 0.5 + 0.5;
            // let blue = (total_time.as_secs_f32() as f64 * -7.2 + i as f64 * 0.15).sin() * 0.5 + 0.5;
            let red = 0.1;
            let green = 0.0;
            let blue = 0.0;

            // red = red * (audio_features.rms * 2.0) as f64;

            *led = Rgb::new(red, green, blue, None)
        }
    }

    pub fn get_brgw(&self) -> [[u8; 4]; NUM_LEDS] {
        core::array::from_fn(|i| {
            [
                (self.leds[i].blue() * 255.0).clamp(0.0, 255.0) as u8,
                (self.leds[i].red() * 255.0).clamp(0.0, 255.0) as u8,
                (self.leds[i].green() * 255.0).clamp(0.0, 255.0) as u8,
                0,
            ]
        })
    }
}
