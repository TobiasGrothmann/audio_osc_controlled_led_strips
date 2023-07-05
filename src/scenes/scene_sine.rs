use std::time::Duration;

use colorsys::Rgb;

use crate::{audio::AudioFeatures, constants::NUM_LEDS, scene::Scene};

pub struct SceneSine {
    leds: [Rgb; NUM_LEDS as usize],
}

impl SceneSine {
    pub fn new() -> Self {
        Self {
            leds: core::array::from_fn(|_| Rgb::new(0.0, 0.0, 0.0, None)),
        }
    }
}

impl Scene for SceneSine {
    fn tick(
        &mut self,
        _time_since_last_tick: Duration,
        total_time: Duration,
        audio_features: &AudioFeatures,
        osc_fader_values: &Vec<f32>,
    ) {
        for (i, led) in self.leds.iter_mut().enumerate() {
            let mut red =
                (total_time.as_secs_f32() as f64 * -1.5 + i as f64 * 7.0).sin() * 0.5 + 0.5;
            let green = audio_features.zcr.avg * 0.0001;
            let mut blue =
                (total_time.as_secs_f32() as f64 * -0.2 + i as f64 * 12.0).sin() * 0.8 + 0.5;

            red = red * 0.3 * (audio_features.rms.avg * audio_features.zcr.avg * 0.01) as f64;
            blue = blue * 0.3 * (0.5 - audio_features.rms.avg as f64 * 0.5);

            *led = Rgb::new(red, green, blue, None)
        }
    }

    fn get_rgb(&self) -> &[Rgb; NUM_LEDS as usize] {
        &self.leds
    }
}
