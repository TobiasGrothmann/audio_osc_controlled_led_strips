use colorsys::Rgb;
use rs_ws281x::Controller;

use crate::{constants::NUM_LEDS, scene::Scene};

pub fn render_scene(controller: &mut Controller, scene: &impl Scene) {
    let scene_rgb = scene.get_rgb();
    let scene_brgw = get_brgw(scene_rgb);

    for (i, led) in controller.leds_mut(0).iter_mut().enumerate() {
        *led = scene_brgw[i];
    }
    controller.render().expect("could not render on controller");
}

fn get_brgw(rgb: &[Rgb; NUM_LEDS as usize]) -> [[u8; 4]; NUM_LEDS as usize] {
    core::array::from_fn(|i| {
        [
            (rgb[i].blue() * 255.0).clamp(0.0, 255.0) as u8,
            (rgb[i].red() * 255.0).clamp(0.0, 255.0) as u8,
            (rgb[i].green() * 255.0).clamp(0.0, 255.0) as u8,
            0,
        ]
    })
}
