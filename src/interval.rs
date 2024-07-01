use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interval {
    pub start: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
}

impl Interval {
    pub fn new(start: &str, end: &str) -> Self {
        Interval {
            start: DateTime::parse_from_rfc3339(start).unwrap(),
            end: DateTime::parse_from_rfc3339(end).unwrap(),
        }
    }

    pub fn contains(&self, interval: &Interval) -> bool {
        self.start <= interval.start && self.end >= interval.end
    }

    pub fn intersects(&self, interval: &Interval) -> bool {
        self.start < interval.end && self.end > interval.start
    }
}
