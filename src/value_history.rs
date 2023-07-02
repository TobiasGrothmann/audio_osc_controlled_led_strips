use std::{
    time::{Duration, Instant},
    vec,
};

#[derive(Copy, Clone)]
pub struct Measurement {
    value: f32,
    time: Instant,
}

#[derive(Copy, Clone)]
pub struct ValueFeatures {
    average: f32,
    min: f32,
    max: f32,
    range: f32,
}

pub struct ValueHistory {
    history: Vec<Measurement>,
}

impl ValueHistory {
    pub fn new() -> Self {
        Self { history: vec![] }
    }

    pub fn add(&mut self, value: f32) {
        self.history.push(Measurement {
            value: value,
            time: Instant::now(),
        });
    }

    pub fn delete_older_than(&mut self, delete_before: Duration) {
        let now = Instant::now();

        self.history = self
            .history
            .iter()
            .map(|v| *v)
            .filter(|value| value.time - now < delete_before)
            .collect();
    }

    pub fn average(&self, since: Duration) -> f32 {
        let now = Instant::now();

        let mut sum: f32 = 0.0;
        let mut num: usize = 0;
        for value in self.history.iter() {
            if now - value.time < since {
                num += 1;
                sum += value.value;
            }
        }
        if num == 0 {
            return 0.0;
        }
        sum / num as f32
    }

    pub fn min_max(&self, since: Duration) -> (f32, f32) {
        let now = Instant::now();

        let mut min: f32 = 0.0;
        let mut max: f32 = 0.0;
        for value in self.history.iter() {
            if now - value.time < since {
                max = max.max(value.value);
                min = min.min(value.value);
            }
        }
        (min, max)
    }

    pub fn get_features(&self, since: Duration) -> ValueFeatures {
        let (min, max) = self.min_max(since);
        ValueFeatures {
            average: self.average(since),
            min: min,
            max: max,
            range: max - min,
        }
    }
}
