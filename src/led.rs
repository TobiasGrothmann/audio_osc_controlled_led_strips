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

    // println!(
    //     "rgb: {}, {}, {}",
    //     scene_rgb[0].red(),
    //     scene_rgb[0].green(),
    //     scene_rgb[0].blue()
    // );
    // println!(
    //     "brg: {}, {}, {}",
    //     scene_brgw[0][0], scene_brgw[0][1], scene_brgw[0][2]
    // );
}

fn get_brgw(rgb: &[Rgb; NUM_LEDS as usize]) -> [[u8; 4]; NUM_LEDS as usize] {
    core::array::from_fn(|i| {
        [
            255.0_f64.powf((rgb[i].blue() - 0.001).clamp(0.0, 1.0)) as u8 - 1,
            255.0_f64.powf((rgb[i].red() - 0.001).clamp(0.0, 1.0)) as u8 - 1,
            255.0_f64.powf((rgb[i].green() - 0.001).clamp(0.0, 1.0)) as u8 - 1,
            0,
        ]
    })
}
