use std::time::Duration;

use colorsys::Rgb;

use crate::{audio::AudioFeatures, constants::NUM_LEDS};

pub trait Scene {
    fn tick(
        &mut self,
        time_since_last_tick: Duration,
        total_time: Duration,
        audio_features: &AudioFeatures,
    );

    fn get_rgb(&self) -> &[Rgb; NUM_LEDS as usize];
}
