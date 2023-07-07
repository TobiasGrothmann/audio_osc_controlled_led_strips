use std::time::{Duration, Instant};

use colorsys::Rgb;

use crate::{
    audio::{self, AudioFeatures},
    color::hsl,
    constants::NUM_LEDS,
    scene::Scene,
};

pub struct ScneeHighRunner {
    leds: [Rgb; NUM_LEDS as usize],
    last_switch: Instant,
    interval: Duration,
}

impl ScneeHighRunner {
    pub fn new() -> Self {
        Self {
            leds: core::array::from_fn(|_| Rgb::new(0.0, 0.0, 0.0, None)),
            last_switch: Instant::now(),
            interval: Duration::from_millis(10),
        }
    }
}

impl Scene for ScneeHighRunner {
    fn tick(
        &mut self,
        time_since_last_tick: Duration,
        total_time: Duration,
        audio_features: &AudioFeatures,
        osc_fader_values: &Vec<f32>,
    ) {
        if Instant::now() - self.last_switch > self.interval {
            self.last_switch = Instant::now();
        } else {
            return;
        }

        let hue = osc_fader_values[5] as f64 + total_time.as_secs_f64() * 0.001;
        let lightness = audio_features.rms_hpf.avg.powf(1.5);
        self.leds[0] = hsl(hue, 0.9, lightness);

        let leds_copy = self.leds.clone();

        for (i, led) in self.leds.iter_mut().enumerate() {
            if i == 0 {
                continue;
            }

            *led = leds_copy[i - 1].clone()
        }
    }

    fn get_rgb(&self) -> &[Rgb; NUM_LEDS as usize] {
        &self.leds
    }
}
