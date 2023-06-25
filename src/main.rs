mod scene;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

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

    let stream = device
        .build_input_stream(
            &config.into(),
            move |data, _: &_| {
                audio_in_callback::<f32, f32>(data, sample_rate as f64);
            },
            move |err| {
                eprintln!("an error occurred on stream: {}", err);
            },
            None,
        )
        .expect("could not create stream");

    stream.play().expect("couldn ot start stream");
    std::thread::sleep(std::time::Duration::from_secs(5));
}

fn audio_in_callback<T, U>(signal_arr: &[f32], _sample_rate: f64) {
    let _signal: Vec<f64> = signal_arr
        .iter()
        .map(|sample_f32| *sample_f32 as f64)
        .collect();

    // let rms = meyda::get_rms(&signal);
    // let energy = meyda::get_energy(&signal);
    // let zcr = meyda::get_zcr(&signal);
    // let power_spectrum = meyda::get_power_spectrum(&signal);
    // let spectral_centroid = meyda::get_spectral_centroid(&signal);
    // let spectral_flatness = meyda::get_spectral_flatness(&signal);
    // let spectral_kurtosis = meyda::get_spectral_kurtosis(&signal);
    // let spectral_rolloff = meyda::get_spectral_rolloff(&signal, sample_rate, Some(0.95));
    // let bark_loudness = meyda::get_bark_loudness(&signal, sample_rate);
}
