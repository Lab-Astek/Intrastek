use std::fmt::{self, Display, Formatter};

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::module::Module;

#[derive(Deserialize, Serialize, Debug, Clone, Hash, PartialEq, Eq)]
pub enum Activities {
    FollowUp,
    Bootstrap,
    Review,
    Keynote,
    Surveillance,
    Permanence,
}

impl Display for Activities {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Activities::FollowUp => write!(f, "Follow Up"),
            Activities::Bootstrap => write!(f, "Bootstrap"),
            Activities::Review => write!(f, "Review"),
            Activities::Keynote => write!(f, "Keynote"),
            Activities::Surveillance => write!(f, "Surveillance"),
            Activities::Permanence => write!(f, "Permanence"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Activity {
    pub start: DateTime<FixedOffset>,
    pub activity: Activities,
    pub end: DateTime<FixedOffset>,
    pub location: String,
    pub needed_asteks: u32,
    pub asteks_names: Vec<String>,
    pub module: Option<Module>,
}

impl Activity {
    pub fn new(
        start: &str,
        activity: Activities,
        location: &str,
        end: &str,
        needed_asteks: u32,
        module: Option<Module>,
    ) -> Self {
        Activity {
            start: DateTime::parse_from_rfc3339(start).unwrap(),
            activity,
            location: location.to_string(),
            end: DateTime::parse_from_rfc3339(end).unwrap(),
            needed_asteks,
            asteks_names: Vec::new(),
            module,
        }
    }

    pub fn add_astek(&mut self, astek: String) {
        self.asteks_names.push(astek);
    }
}

impl Display for Activity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(module) = &self.module {
            writeln!(f, "{} {}:", self.activity, module)?;
        } else {
            writeln!(f, "{}:", self.activity)?;
        }
        writeln!(f, "\tAsteks:")?;
        if self.asteks_names.is_empty() {
            writeln!(f, "\t\t- None")?;
        }
        self.asteks_names
            .iter()
            .try_for_each(|astek| writeln!(f, "\t\t- {}", astek))
    }
}
