use std::time::Duration;

use crate::value_history::ValueHistory;

pub struct AudioFeatures {
    pub rms_avg: f64,
    pub rms_min: f64,
    pub rms_max: f64,
    pub energy_avg: f64,
    pub energy_min: f64,
    pub energy_max: f64,
    pub zcr_avg: f64,
    pub zcr_min: f64,
    pub zcr_max: f64,
}

pub struct AudioFeaturesHistory {
    pub rms: ValueHistory,
    pub energy: ValueHistory,
    pub zcr: ValueHistory,
}

impl AudioFeaturesHistory {
    pub fn new() -> Self {
        Self {
            rms: ValueHistory::new(),
            energy: ValueHistory::new(),
            zcr: ValueHistory::new(),
        }
    }

    pub fn time_range(&self, since: Duration) -> AudioFeatures {
        let (rms_min, rms_max) = self.rms.min_max(since);
        let (energy_min, energy_max) = self.energy.min_max(since);
        let (zcr_min, zcr_max) = self.zcr.min_max(since);

        AudioFeatures {
            rms_avg: self.rms.average(since),
            rms_min: rms_min,
            rms_max: rms_max,
            energy_avg: self.energy.average(since),
            energy_min: energy_min,
            energy_max: energy_max,
            zcr_avg: self.zcr.average(since),
            zcr_min: zcr_min,
            zcr_max: zcr_max,
        }
    }

    pub fn delete_older_than(&mut self, delete_before: Duration) {
        self.rms.delete_older_than(delete_before);
        self.energy.delete_older_than(delete_before);
        self.zcr.delete_older_than(delete_before);
    }
}
