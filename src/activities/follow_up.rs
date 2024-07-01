use chrono::{DateTime, Local};

use crate::activity::Activity;

#[derive(Debug, Clone)]
pub struct FollowUp {
    time: DateTime<Local>,
    location: String,
}

impl Activity for FollowUp {
    fn get_time(&self) -> DateTime<Local> {
        self.time
    }

    fn get_location(&self) -> String {
        self.location.clone()
    }
}
