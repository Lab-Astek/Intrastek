use chrono::{DateTime, Local};

pub enum Activities {
    FollowUp,
    Bootstrap,
    Review,
    Keynote,
    Surveillance,
}

pub struct Activity {
    time: DateTime<Local>,
    activity: Activities,
    location: String,
}
