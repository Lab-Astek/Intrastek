use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Indisponibility {
    start: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Astek {
    name: String,
    indisponibilities: Vec<Indisponibility>,
}

impl Astek {
    pub fn new(name: &str) -> Self {
        Astek {
            name: name.to_string(),
            indisponibilities: Vec::new(),
        }
    }

    pub fn add_indisponibility(&mut self, start: &str, end: &str) {
        self.indisponibilities.push(Indisponibility {
            start: DateTime::parse_from_rfc3339(start).unwrap(),
            end: DateTime::parse_from_rfc3339(end).unwrap(),
        });
    }
}
