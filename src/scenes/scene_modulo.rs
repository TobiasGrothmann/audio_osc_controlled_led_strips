use std::time::{Duration, Instant};

use colorsys::Rgb;

use crate::{audio::AudioFeatures, color::hsl, constants::NUM_LEDS, scene::Scene};

pub struct SceneModulo {
    leds: [Rgb; NUM_LEDS as usize],
    time_s: f64,
}

impl SceneModulo {
    pub fn new() -> Self {
        Self {
            leds: core::array::from_fn(|_| Rgb::new(0.0, 0.0, 0.0, None)),
            time_s: 0.0,
        }
    }
}

impl Scene for SceneModulo {
    fn tick(
        &mut self,
        time_since_last_tick: Duration,
        total_time: Duration,
        audio_features: &AudioFeatures,
        osc_fader_values: &Vec<f32>,
    ) {
        let time_speed = (osc_fader_values[0] as f64 * 2.0).powf(2.0);
        self.time_s += time_since_last_tick.as_secs_f64() * time_speed;

        for (i, led) in self.leds.iter_mut().enumerate() {
            let modulo_factor = (i as f64 * 0.1 + self.time_s) % 1.0;
            let hue = osc_fader_values[5] as f64 + (i as f64 / 400.0) - modulo_factor * 0.1;
            let saturation = osc_fader_values[6] as f64;
            let lightness = osc_fader_values[7] as f64;

            let rgb = hsl(hue, saturation, lightness);
            *led = Rgb::new(
                modulo_factor * rgb.red(),
                modulo_factor * rgb.green(),
                modulo_factor * rgb.blue(),
                None,
            )
        }
    }

    fn get_rgb(&self) -> &[Rgb; NUM_LEDS as usize] {
        &self.leds
    }
}
