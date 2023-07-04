use std::time::Duration;

use crate::value_history::ValueHistory;

pub struct AudioFeatures {
    pub rms: f64,
    pub energy: f64,
    pub zcr: f64,
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

    pub fn average(&self, since: Duration) -> AudioFeatures {
        AudioFeatures {
            rms: self.rms.average(since),
            energy: self.energy.average(since),
            zcr: self.zcr.average(since),
        }
    }

    pub fn delete_older_than(&mut self, delete_before: Duration) {
        self.rms.delete_older_than(delete_before);
        self.energy.delete_older_than(delete_before);
        self.zcr.delete_older_than(delete_before);
    }
}
