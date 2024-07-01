use chrono::{DateTime, Local};
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
    time: DateTime<Local>,
    activity: Activities,
    location: String,
}

impl Activity {
    pub fn new(time: DateTime<Local>, activity: Activities, location: &str) -> Self {
        Activity {
            time,
            activity,
            location: location.to_string(),
        }
    }
}
