use std::fmt::{self, Display, Formatter};

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::activity::Activity;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Indisponibility {
    pub start: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Astek {
    pub name: String,
    indisponibilities: Vec<Indisponibility>,
    assignations: Vec<Activity>,
}

impl Astek {
    pub fn new(name: &str) -> Self {
        Astek {
            name: name.to_string(),
            indisponibilities: Vec::new(),
            assignations: Vec::new(),
        }
    }

    pub fn add_indisponibility(&mut self, start: &str, end: &str) {
        self.indisponibilities.push(Indisponibility {
            start: DateTime::parse_from_rfc3339(start).unwrap(),
            end: DateTime::parse_from_rfc3339(end).unwrap(),
        });
    }

    pub fn is_available(&self, start: DateTime<FixedOffset>, end: DateTime<FixedOffset>) -> bool {
        self.indisponibilities
            .iter()
            .all(|indisponibility| start < indisponibility.start || end > indisponibility.end)
    }

    pub fn assign(&mut self, activity: Activity) {
        self.add_indisponibility(&activity.start.to_rfc3339(), &activity.end.to_rfc3339());
        self.assignations.push(activity);
    }
}

impl Display for Astek {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:", self.name)?;
        writeln!(f, "Indisponibilities:")?;
        self.indisponibilities.iter().try_for_each(|indisponibility| {
            writeln!(
                f,
                "\t- {} to {}",
                indisponibility.start, indisponibility.end
            )
        })?;
        writeln!(f, "Assignations:")?;
        self.assignations
            .iter()
            .try_for_each(|activity| writeln!(f, "\t- {}", activity))
    }
}
