use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Activities {
    FollowUp,
    Bootstrap,
    Review,
    Keynote,
    Surveillance,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Activity {
    pub start: DateTime<FixedOffset>,
    pub activity: Activities,
    pub end: DateTime<FixedOffset>,
    pub location: String,
    pub needed_asteks: u32,
    pub asteks_names: Vec<String>,
}

impl Activity {
    pub fn new(
        start: &str,
        activity: Activities,
        location: &str,
        end: &str,
        needed_asteks: u32,
    ) -> Self {
        Activity {
            start: DateTime::parse_from_rfc3339(start).unwrap(),
            activity,
            location: location.to_string(),
            end: DateTime::parse_from_rfc3339(end).unwrap(),
            needed_asteks,
            asteks_names: Vec::new(),
        }
    }
}
