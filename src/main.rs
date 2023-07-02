mod scene;
mod value_history;

use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use value_history::ValueHistory;

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("could not get default input device");
    let config = device
        .default_input_config()
        .expect("could not get device default config");

    let sample_rate = config.sample_rate().0;
    println!("sample rate: {}", sample_rate);

    println!(
        "Input device: {}",
        device.name().expect("could not get input device name")
    );

    let mut rms_history = ValueHistory::new();

    let stream = device
        .build_input_stream(
            &config.into(),
            move |data, _: &_| {
                audio_in_callback::<f32, f32>(data, sample_rate as f64, &mut rms_history);
            },
            move |err| {
                eprintln!("an error occurred on stream: {}", err);
            },
            None,
        )
        .expect("could not create stream");

    stream.play().expect("couldn ot start stream");
    std::thread::sleep(std::time::Duration::from_secs(5));

    let mut controller = ControllerBuilder::new()
        .freq(800_000)
        .dma(10)
        .channel(
            0, // Channel Index
            ChannelBuilder::new()
                .pin(10) // GPIO 10 = SPI0 MOSI
                .count(64) // Number of LEDs
                .strip_type(StripType::Ws2812)
                .brightness(20) // default: 255
                .build(),
        )
        .build()
        .unwrap();

    let leds = controller.leds_mut(0);

    for led in leds {
        *led = [0, 0, 255, 0];
    }

    controller.render().unwrap();
}

fn audio_in_callback<T, U>(signal_arr: &[f32], _sample_rate: f64, rms_history: &mut ValueHistory) {
    let signal: Vec<f64> = signal_arr
        .iter()
        .map(|sample_f32| *sample_f32 as f64)
        .collect();

    let rms = meyda::get_rms(&signal);
    rms_history.add(rms as f32);

    println!("{}", rms_history.average(Duration::from_secs_f32(5.0)));
    rms_history.delete_older_than(Duration::from_secs(10));

    // let energy = meyda::get_energy(&signal);
    // let zcr = meyda::get_zcr(&signal);
    // let power_spectrum = meyda::get_power_spectrum(&signal);
    // let spectral_centroid = meyda::get_spectral_centroid(&signal);
    // let spectral_flatness = meyda::get_spectral_flatness(&signal);
    // let spectral_kurtosis = meyda::get_spectral_kurtosis(&signal);
    // let spectral_rolloff = meyda::get_spectral_rolloff(&signal, sample_rate, Some(0.95));
    // let bark_loudness = meyda::get_bark_loudness(&signal, sample_rate);
}
