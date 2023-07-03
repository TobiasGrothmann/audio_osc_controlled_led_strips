use rs_ws281x::Controller;

use crate::scene::Scene;

pub fn render_scene(controller: &mut Controller, scene: &Scene) {
    let scene_brgw = scene.get_brgw();

    for (i, led) in controller.leds_mut(0).iter_mut().enumerate() {
        *led = scene_brgw[i];
    }
    controller.render().unwrap();
}
