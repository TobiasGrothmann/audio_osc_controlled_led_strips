use std::time::Duration;

use colorsys::Rgb;

use crate::{audio::AudioFeatures, constants::NUM_LEDS, scene::Scene};

pub struct SceneTravelOut {
    leds: [Rgb; NUM_LEDS as usize],
}

impl SceneTravelOut {
    pub fn new() -> Self {
        Self {
            leds: core::array::from_fn(|_| Rgb::new(0.0, 0.0, 0.0, None)),
        }
    }
}

impl Scene for SceneTravelOut {
    fn tick(
        &mut self,
        _time_since_last_tick: Duration,
        total_time: Duration,
        audio_features: &AudioFeatures,
    ) {
        let brightness = (audio_features.rms_avg * 0.8).powf(2.0);

        self.leds[0] = Rgb::new(
            brightness,
            brightness * audio_features.energy_max * 0.0005,
            brightness * 0.2,
            None,
        );
        let leds_old = self.leds.clone();

        for (i, led) in self.leds.iter_mut().enumerate() {
            if i == 0 {
                continue;
            }
            *led = leds_old[i - 1].clone();
        }
    }

    fn get_rgb(&self) -> &[Rgb; NUM_LEDS as usize] {
        &self.leds
    }
}
