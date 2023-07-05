mod audio;
mod constants;
mod led;
mod osc;
mod scene;
mod scenes;
mod value_history;

use biquad::{Biquad, Coefficients, DirectForm1, ToHertz, Type, Q_BUTTERWORTH_F32};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rs_ws281x::{ChannelBuilder, ControllerBuilder, StripType};
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use crate::{
    audio::AudioFeaturesHistory,
    constants::{AUDIO_AVERAGE_SECONDS, BRIGHTNESS, FREQ_HPF, FREQ_LPF, NUM_LEDS, PIN},
    led::render_scene,
    osc::osc_start_listen,
    scene::Scene,
    scenes::{
        scene_pulse_yellow::ScenePulseYellow, scene_sine::SceneSine, scene_strobo::SceneStrobo,
    },
};

fn main() {
    let (tx, rx) = mpsc::channel();
    osc_start_listen(tx);

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

    // create filters
    let coeffs_lpf = Coefficients::<f64>::from_params(
        Type::LowPass,
        (sample_rate as f64).hz(),
        FREQ_LPF.hz(),
        Q_BUTTERWORTH_F32 as f64,
    )
    .unwrap();
    let mut biquad_lpf = DirectForm1::<f64>::new(coeffs_lpf);

    let coeffs_hpf = Coefficients::<f64>::from_params(
        Type::HighPass,
        (sample_rate as f64).hz(),
        FREQ_HPF.hz(),
        Q_BUTTERWORTH_F32 as f64,
    )
    .unwrap();
    let mut biquad_hpf = DirectForm1::<f64>::new(coeffs_hpf);

    let stream = device
        .build_input_stream(
            &config.into(),
            move |data, _: &_| {
                audio_in_callback::<f32, f32>(
                    data,
                    sample_rate as f64,
                    &audio_feature_history_audio_thread,
                    &mut biquad_lpf,
                    &mut biquad_hpf,
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
                .pin(PIN)
                .count(NUM_LEDS)
                .brightness(BRIGHTNESS)
                .strip_type(StripType::Ws2812)
                .build(),
        )
        .build()
        .expect("could not build controller");

    let mut scene = SceneStrobo::new();

    let start_time = Instant::now();
    let mut time_last_tick = start_time;

    loop {
        // compute time
        let now = Instant::now();
        let total_time = now - start_time;
        let time_since_last_tick = now - time_last_tick;
        time_last_tick = now;

        // println!("frame dur millis: {}", time_since_last_tick.as_millis());

        // get audio values
        let audio_features = audio_feature_history
            .lock()
            .expect("could not lock audio feature history to get values")
            .time_range(Duration::from_secs_f32(AUDIO_AVERAGE_SECONDS));

        // delete old values
        audio_feature_history
            .lock()
            .expect("could not lock audio feature history to delete older")
            .delete_older_than(Duration::from_secs_f32(AUDIO_AVERAGE_SECONDS));

        // render
        scene.tick(time_since_last_tick, total_time, &audio_features);
        render_scene(&mut controller, &scene);

        thread::sleep(Duration::from_millis(15));
    }
}

fn audio_in_callback<T, U>(
    signal_arr: &[f32],
    sample_rate: f64,
    audio_feature_history: &Mutex<AudioFeaturesHistory>,
    biquad_lpf: &mut DirectForm1<f64>,
    biquad_hpf: &mut DirectForm1<f64>,
) {
    let signal: Vec<f64> = signal_arr
        .iter()
        .map(|sample_f32| *sample_f32 as f64)
        .collect();

    // println!("samples: {}", signal.len());

    let rms = meyda::get_rms(&signal);
    let energy = meyda::get_energy(&signal);
    let zcr = meyda::get_zcr(&signal);

    // lpf
    let mut signal_lpf = vec![0.0; signal_arr.len()];
    for (i, sample) in signal.iter().enumerate() {
        signal_lpf[i] = biquad_lpf.run(*sample);
    }
    let rms_lpf = meyda::get_rms(&signal_lpf);

    // hpf
    let mut signal_hpf = vec![0.0; signal_arr.len()];
    for (i, sample) in signal.iter().enumerate() {
        signal_hpf[i] = biquad_hpf.run(*sample);
    }
    let rms_hpf = meyda::get_rms(&signal_hpf);

    // add measurements
    let mut lock = audio_feature_history
        .lock()
        .expect("could not lock audio feature history to add values");

    lock.rms.add(rms);
    lock.energy.add(energy);
    lock.zcr.add(energy);

    lock.rms_lpf.add(rms_lpf);
    lock.rms_hpf.add(rms_hpf);
}
