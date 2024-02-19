use colorsys::{Hsl, Rgb};

pub fn hsl(hue: f64, saturation: f64, lightness: f64) -> Rgb {
    let hsl = Hsl::new(
        (hue * 360.0) % 360.0,
        saturation * 100.0,
        lightness * 100.0,
        None,
    );
    let rgb_255 = Rgb::from(hsl);
    Rgb::new(
        rgb_255.red() / 255.0,
        rgb_255.green() / 255.0,
        rgb_255.blue() / 255.0,
        None,
    )
}
