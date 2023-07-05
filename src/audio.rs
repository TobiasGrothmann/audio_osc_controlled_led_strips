use std::time::Duration;

use crate::value_history::ValueHistory;

pub struct AudioFeature {
    pub avg: f64,
    pub min: f64,
    pub max: f64,
}

pub struct AudioFeatures {
    pub rms: AudioFeature,
    pub energy: AudioFeature,
    pub zcr: AudioFeature,
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
            rms: AudioFeature {
                avg: self.rms.average(since),
                min: rms_min,
                max: rms_max,
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
        self.energy.delete_older_than(delete_before);
        self.zcr.delete_older_than(delete_before);
    }
}
