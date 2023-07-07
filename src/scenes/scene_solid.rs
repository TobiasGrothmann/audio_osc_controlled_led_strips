use std::time::{Duration, Instant};

use colorsys::Rgb;

use crate::{audio::AudioFeatures, color::hsl, constants::NUM_LEDS, scene::Scene};

pub struct SceneSolid {
    leds: [Rgb; NUM_LEDS as usize],
}

impl SceneSolid {
    pub fn new() -> Self {
        Self {
            leds: core::array::from_fn(|_| Rgb::new(0.0, 0.0, 0.0, None)),
        }
    }
}

impl Scene for SceneSolid {
    fn tick(
        &mut self,
        time_since_last_tick: Duration,
        total_time: Duration,
        audio_features: &AudioFeatures,
        osc_fader_values: &Vec<f32>,
    ) {
        let hue = osc_fader_values[5] as f64;
        let saturation = osc_fader_values[6] as f64;
        let lightness = osc_fader_values[7] as f64;
        let rgb = hsl(hue, saturation, lightness);
        self.leds = core::array::from_fn(|_| rgb.clone());
    }

    fn get_rgb(&self) -> &[Rgb; NUM_LEDS as usize] {
        &self.leds
    }
}
