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
    length: u32,
    location: String,
}

impl Activity {
    pub fn new(time: DateTime<Local>, activity: Activities, location: &str, length: u32) -> Self {
        if (length < 1) || (length > 8) {
            panic!("Activity length must be between 1 and 8 hours");
        }
        Activity {
            time,
            activity,
            location: location.to_string(),
            length,
        }
    }
}
