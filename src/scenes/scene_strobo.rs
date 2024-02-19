use std::time::{Duration, Instant};

use colorsys::Rgb;

use crate::{audio::AudioFeatures, color::hsl, constants::NUM_LEDS, scene::Scene};

pub struct SceneStrobo {
    leds: [Rgb; NUM_LEDS as usize],
    last_switch: Instant,
    interval: Duration,
    is_on: bool,
}

impl SceneStrobo {
    pub fn new() -> Self {
        Self {
            leds: core::array::from_fn(|_| Rgb::new(0.0, 0.0, 0.0, None)),
            last_switch: Instant::now(),
            interval: Duration::from_millis(30),
            is_on: false,
        }
    }
}

impl Scene for SceneStrobo {
    fn tick(
        &mut self,
        time_since_last_tick: Duration,
        total_time: Duration,
        audio_features: &AudioFeatures,
        osc_fader_values: &Vec<f32>,
    ) {
        let hue = total_time.as_secs_f64() * 10.0;

        self.interval = Duration::from_millis(30);

        if Instant::now() - self.last_switch > self.interval {
            self.is_on = !self.is_on;
            self.last_switch = Instant::now();
        }

        let mut lightness = 0.0;
        if self.is_on {
            lightness = audio_features.rms_hpf.avg * 1.8;
        }

        let rgb = hsl(hue, 0.7, lightness);

        for (i, led) in self.leds.iter_mut().enumerate() {
            *led = rgb.clone();
        }
    }

    fn get_rgb(&self) -> &[Rgb; NUM_LEDS as usize] {
        &self.leds
    }
}
