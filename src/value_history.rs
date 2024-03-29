use std::{
    time::{Duration, Instant},
    vec,
};

#[derive(Copy, Clone)]
pub struct Measurement {
    value: f64,
    time: Instant,
}

#[derive(Copy, Clone)]
pub struct ValueFeatures {
    average: f64,
    min: f64,
    max: f64,
    range: f64,
}

pub struct ValueHistory {
    history: Vec<Measurement>,
}

impl ValueHistory {
    pub fn new() -> Self {
        Self { history: vec![] }
    }

    pub fn add(&mut self, value: f64) {
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
            .filter(|value| now - value.time < delete_before)
            .collect();
    }

    pub fn average(&self, since: Duration) -> f64 {
        let now = Instant::now();

        let mut sum: f64 = 0.0;
        let mut num: usize = 0;
        for value in self.history.iter().rev() {
            if now - value.time > since {
                break;
            }
            num += 1;
            sum += value.value;
        }
        if num == 0 {
            match self.history.last() {
                Some(value) => return value.value,
                None => return 0.0,
            }
        }
        sum / num as f64
    }

    pub fn min_max(&self, since: Duration) -> (f64, f64) {
        let now = Instant::now();

        let mut min: f64 = 0.0;
        let mut max: f64 = 0.0;
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
