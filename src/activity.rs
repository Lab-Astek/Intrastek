use chrono::{DateTime, Local};

pub enum Activities {
    FollowUp,
    Bootstrap,
    Review,
    Keynote,
    Surveillance
}

pub trait Activity {
    fn get_time(&self) -> DateTime<Local>;

    fn get_location(&self) -> String;
}
