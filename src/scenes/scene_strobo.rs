use std::time::{Duration, Instant};

use colorsys::{Hsl, Rgb};

use crate::{audio::AudioFeatures, constants::NUM_LEDS, scene::Scene};

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
        _time_since_last_tick: Duration,
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

        let mut brightness = 0.0;
        if self.is_on {
            brightness = audio_features.rms_hpf.avg;
        }

        let hsv = Hsl::new(hue % 360.0, 70.0, brightness * 100.0, None);
        let mut rgb = Rgb::from(hsv);
        rgb = Rgb::new(
            rgb.red() / 255.0,
            rgb.green() / 255.0,
            rgb.blue() / 255.0,
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
