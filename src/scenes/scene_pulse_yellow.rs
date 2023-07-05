use std::time::Duration;

use colorsys::{Hsl, Rgb};

use crate::{audio::AudioFeatures, constants::NUM_LEDS, scene::Scene};

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
        _time_since_last_tick: Duration,
        total_time: Duration,
        audio_features: &AudioFeatures,
    ) {
        let hue = audio_features.zcr_avg * 0.00004;
        let lightness = audio_features.rms_avg * 0.4;

        let hsv = Hsl::new(hue * 360.0, 100.0, lightness * 100.0, None);
        let rgb_255 = Rgb::from(hsv);
        let rgb = Rgb::new(
            rgb_255.red() / 255.0,
            rgb_255.green() / 255.0,
            rgb_255.blue() / 255.0,
            None,
        );

        for (i, led) in self.leds.iter_mut().enumerate() {
            *led = rgb.clone();
        }
    }

    fn get_rgb(&self) -> &[Rgb; NUM_LEDS as usize] {
        &self.leds
    }
}
