use std::time::{Duration, Instant};

use colorsys::Rgb;

use crate::{audio::AudioFeatures, color::hsl, constants::NUM_LEDS, scene::Scene};

pub struct ScenePoles {
    leds: [Rgb; NUM_LEDS as usize],
}

impl ScenePoles {
    pub fn new() -> Self {
        Self {
            leds: core::array::from_fn(|_| Rgb::new(0.0, 0.0, 0.0, None)),
        }
    }
}

impl Scene for ScenePoles {
    fn tick(
        &mut self,
        time_since_last_tick: Duration,
        total_time: Duration,
        audio_features: &AudioFeatures,
        osc_fader_values: &Vec<f32>,
    ) {
        let hue_1 = osc_fader_values[5] as f64;
        let hue_2 = (hue_1 + 0.18) % 1.0;
        let saturation = osc_fader_values[6] as f64;
        let lightness_1 = audio_features.rms_lpf.avg * 0.6;
        let lightness_2 = audio_features.rms_hpf.avg * 0.9;

        let rgb_1 = hsl(hue_1, saturation, lightness_1);
        let rgb_2 = hsl(hue_2, saturation, lightness_2);

        self.leds = core::array::from_fn(|i| {
            let factor = i as f64 / NUM_LEDS as f64;
            Rgb::new(
                rgb_1.red() * factor + rgb_2.red() * (1.0 - factor),
                rgb_1.green() * factor + rgb_2.green() * (1.0 - factor),
                rgb_1.blue() * factor + rgb_2.blue() * (1.0 - factor),
                None,
            )
        });
    }

    fn get_rgb(&self) -> &[Rgb; NUM_LEDS as usize] {
        &self.leds
    }
}
