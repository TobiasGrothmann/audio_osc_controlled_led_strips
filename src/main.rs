use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("could not get default input device");
    let config = device
        .default_input_config()
        .expect("could not get device default config");

    println!(
        "Input device: {}",
        device.name().expect("could not get input device name")
    );

    let stream = device
        .build_input_stream(
            &config.into(),
            move |data, _: &_| {
                audio_in_callback::<f32, f32>(data);
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

fn audio_in_callback<T, U>(input: &[f32]) {
    println!("samples: {}", input.len());
}
