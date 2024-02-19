use std::time::Duration;

use colorsys::{Hsl, Rgb};

use crate::{audio::AudioFeatures, color::hsl, constants::NUM_LEDS, scene::Scene};

pub struct ScenePulseYellow {
    leds: [Rgb; NUM_LEDS as usize],
}

impl ScenePulseYellow {
    pub fn new() -> Self {
        Self {
            leds: core::array::from_fn(|_| Rgb::new(0.0, 0.0, 0.0, None)),
        }
    }
}

impl Scene for ScenePulseYellow {
    fn tick(
        &mut self,
        time_since_last_tick: Duration,
        total_time: Duration,
        audio_features: &AudioFeatures,
        osc_fader_values: &Vec<f32>,
    ) {
        let hue = audio_features.rms_hpf.avg;
        let lightness = audio_features.rms_lpf.avg;

        let rgb = hsl(hue, 1.0, lightness);

        for (i, led) in self.leds.iter_mut().enumerate() {
            *led = rgb.clone();
        }
    }

    fn get_rgb(&self) -> &[Rgb; NUM_LEDS as usize] {
        &self.leds
    }
}
