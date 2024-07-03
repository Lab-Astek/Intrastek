use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::{activity::Activities, interval::Interval};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum IndisponibilityType {
    #[default]
    Private,
    Activity(Activities),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Indisponibility {
    interval: Interval,
    #[serde(alias = "type")]
    tpe: IndisponibilityType,
}

impl Indisponibility {
    pub fn new(interval: Interval, tpe: IndisponibilityType) -> Self {
        Indisponibility { interval, tpe }
    }

    pub fn get_interval(&self) -> &Interval {
        &self.interval
    }
}

impl Display for IndisponibilityType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IndisponibilityType::Private => write!(f, "Private"),
            IndisponibilityType::Activity(act) => write!(f, "{}", act),
        }
    }
}

impl Display for Indisponibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} from {}", self.tpe, self.interval)
    }
}
