use std::time::Duration;

use crate::value_history::ValueHistory;

pub struct AudioFeature {
    pub avg: f64,
    pub min: f64,
    pub max: f64,
}

pub struct AudioFeatures {
    pub rms: AudioFeature,
    pub rms_hpf: AudioFeature,
    pub rms_lpf: AudioFeature,
    pub energy: AudioFeature,
    pub zcr: AudioFeature,
}

pub struct AudioFeaturesHistory {
    pub rms: ValueHistory,
    pub rms_lpf: ValueHistory,
    pub rms_hpf: ValueHistory,
    pub energy: ValueHistory,
    pub zcr: ValueHistory,
}

impl AudioFeaturesHistory {
    pub fn new() -> Self {
        Self {
            rms: ValueHistory::new(),
            rms_lpf: ValueHistory::new(),
            rms_hpf: ValueHistory::new(),
            energy: ValueHistory::new(),
            zcr: ValueHistory::new(),
        }
    }

    pub fn time_range(&self, since: Duration) -> AudioFeatures {
        let (rms_min, rms_max) = self.rms.min_max(since);
        let (rms_lpf_min, rms_lpf_max) = self.rms_lpf.min_max(since);
        let (rms_hpf_min, rms_hpf_max) = self.rms_hpf.min_max(since);
        let (energy_min, energy_max) = self.energy.min_max(since);
        let (zcr_min, zcr_max) = self.zcr.min_max(since);

        AudioFeatures {
            rms: AudioFeature {
                avg: self.rms.average(since),
                min: rms_min,
                max: rms_max,
            },
            rms_lpf: AudioFeature {
                avg: self.rms_lpf.average(since),
                min: rms_lpf_min,
                max: rms_lpf_max,
            },
            rms_hpf: AudioFeature {
                avg: self.rms_hpf.average(since),
                min: rms_hpf_min,
                max: rms_hpf_max,
            },
            energy: AudioFeature {
                avg: self.energy.average(since),
                min: energy_min,
                max: energy_max,
            },
            zcr: AudioFeature {
                avg: self.zcr.average(since),
                min: zcr_min,
                max: zcr_max,
            },
        }
    }

    pub fn delete_older_than(&mut self, delete_before: Duration) {
        self.rms.delete_older_than(delete_before);
        self.rms_lpf.delete_older_than(delete_before);
        self.rms_hpf.delete_older_than(delete_before);
        self.energy.delete_older_than(delete_before);
        self.zcr.delete_older_than(delete_before);
    }
}
