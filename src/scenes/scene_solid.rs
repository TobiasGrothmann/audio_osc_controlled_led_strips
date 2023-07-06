use std::time::{Duration, Instant};

use colorsys::{Hsl, Rgb};

use crate::{audio::AudioFeatures, constants::NUM_LEDS, scene::Scene};

pub struct SceneSolid {
    leds: [Rgb; NUM_LEDS as usize],
    last_switch: Instant,
    interval: Duration,
    is_on: bool,
}

impl SceneSolid {
    pub fn new() -> Self {
        Self {
            leds: core::array::from_fn(|_| Rgb::new(0.0, 0.0, 0.0, None)),
            last_switch: Instant::now(),
            interval: Duration::from_millis(30),
            is_on: false,
        }
    }
}

impl Scene for SceneSolid {
    fn tick(
        &mut self,
        _time_since_last_tick: Duration,
        total_time: Duration,
        audio_features: &AudioFeatures,
        osc_fader_values: &Vec<f32>,
    ) {
        let hue = osc_fader_values[5] as f64;
        let saturation = osc_fader_values[6] as f64;
        let lightness = osc_fader_values[7] as f64;
        let hsl = Hsl::new(hue * 360.0, saturation * 100.0, lightness * 100.0, None);
        let rgb = Rgb::from(hsl);
        self.leds = core::array::from_fn(|_| {
            Rgb::new(
                rgb.red() / 255.0,
                rgb.green() / 255.0,
                rgb.blue() / 255.0,
                None,
            )
        });
    }

    fn get_rgb(&self) -> &[Rgb; NUM_LEDS as usize] {
        &self.leds
    }
}
