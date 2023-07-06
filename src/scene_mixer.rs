use colorsys::Rgb;

use crate::{
    constants::NUM_LEDS,
    scene::{self, Scene},
};

pub struct SceneMixer {
    scenes: Vec<Box<dyn Scene>>,
    pub weights: Vec<f32>,
    leds: [Rgb; NUM_LEDS as usize],
}

impl SceneMixer {
    pub fn new(scenes: Vec<Box<dyn Scene>>) -> Self {
        Self {
            weights: vec![0.0; scenes.len()],
            scenes: scenes,
            leds: core::array::from_fn(|_| Rgb::new(0.0, 0.0, 0.0, None)),
        }
    }
}

impl Scene for SceneMixer {
    fn tick(
        &mut self,
        time_since_last_tick: std::time::Duration,
        total_time: std::time::Duration,
        audio_features: &crate::audio::AudioFeatures,
        osc_fader_values: &Vec<f32>,
    ) {
        let mut mixed: [Rgb; NUM_LEDS as usize] =
            core::array::from_fn(|_| Rgb::new(0.0, 0.0, 0.0, None));
        for (weight, scene) in self.weights.iter().zip(self.scenes.iter_mut()) {
            if *weight < 0.001 {
                continue;
            }

            scene.tick(
                time_since_last_tick,
                total_time,
                audio_features,
                osc_fader_values,
            );

            let scene_rendered = scene.get_rgb();
            for (rgb_scene, rgb_mixed) in scene_rendered.iter().zip(mixed.iter_mut()) {
                *rgb_mixed += Rgb::new(
                    *weight as f64 * rgb_scene.red(),
                    *weight as f64 * rgb_scene.green(),
                    *weight as f64 * rgb_scene.blue(),
                    None,
                );
            }
            self.leds = mixed.clone();
        }
    }

    fn get_rgb(&self) -> &[Rgb; NUM_LEDS as usize] {
        &self.leds
    }
}
