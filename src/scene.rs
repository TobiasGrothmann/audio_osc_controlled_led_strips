use array2d::Array2D;
use colorsys::Rgb;

const NUM_LEDS: usize = 100;
const NUM_STRIPS: usize = 4;

pub struct Scene {
    leds: Array2D<Rgb>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            leds: Array2D::filled_with(Rgb::new(0.0, 0.0, 0.0, None), NUM_STRIPS, NUM_LEDS),
        }
    }
}
