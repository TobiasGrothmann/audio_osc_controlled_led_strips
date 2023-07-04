mod audio;
mod constants;
mod led;
mod scene;
mod value_history;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rs_ws281x::{ChannelBuilder, ControllerBuilder, StripType};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use crate::{audio::AudioFeaturesHistory, constants::NUM_LEDS, led::render_scene, scene::Scene};
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

    let audio_feature_history = Arc::new(Mutex::new(AudioFeaturesHistory::new()));
    let audio_feature_history_audio_thread = audio_feature_history.clone();

    let stream = device
        .build_input_stream(
            &config.into(),
            move |data, _: &_| {
                audio_in_callback::<f32, f32>(
                    data,
                    sample_rate as f64,
                    &audio_feature_history_audio_thread,
                );
            },
            move |err| {
                eprintln!("an error occurred on stream: {}", err);
            },
            None,
        )
        .expect("could not create stream");

    stream.play().expect("could not start stream");

    let mut controller = ControllerBuilder::new()
        .channel(
            0, // Channel Index
            ChannelBuilder::new()
                .pin(21)
                .count(NUM_LEDS as i32)
                .brightness(150)
                .strip_type(StripType::Ws2812)
                .build(),
        )
        .build()
        .expect("could not build controller");

    let mut scene = Scene::new();

    let start_time = Instant::now();
    let mut time_last_tick = start_time;

    loop {
        // compute time
        let now = Instant::now();
        let total_time = now - start_time;
        let time_since_last_tick = now - time_last_tick;

        // get audio values
        let audio_features = audio_feature_history
            .lock()
            .expect("could not lock audio feature history to get values")
            .average(Duration::from_secs_f32(1.0));

        let a = audio_feature_history
            .lock()
            .expect("could not lock audio feature history to delete older")
            .delete_older_than(Duration::from_secs_f32(1.0));

        // scene.tick(time_since_last_tick, total_time, audio_features);
        scene.tick(time_since_last_tick, total_time, &audio_features);
        render_scene(&mut controller, &scene);

        time_last_tick = now;

        thread::sleep(Duration::from_millis(50));
    }
}

fn audio_in_callback<T, U>(
    signal_arr: &[f32],
    sample_rate: f64,
    audio_feature_history: &Mutex<AudioFeaturesHistory>,
) {
    let signal: Vec<f64> = signal_arr
        .iter()
        .map(|sample_f32| *sample_f32 as f64)
        .collect();

    let rms = meyda::get_rms(&signal) as f32;
    let energy = meyda::get_energy(&signal) as f32;
    // println!("{}", rms);

    let mut lock = audio_feature_history
        .lock()
        .expect("could not lock audio feature history to add values");

    lock.rms.add(rms);
    lock.energy.add(energy);
}
