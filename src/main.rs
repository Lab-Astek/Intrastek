#![allow(dead_code)]

use activity::{Activities, Activity};
use chrono::Local;
use planner::Planner;

mod activity;
mod planner;

fn main() {
    let mut planner = Planner::from_file("planner.json").unwrap_or(Planner::new());

    let activity = Activity::new(Local::now(), Activities::FollowUp, "Home");
    planner.add_activity(activity);

    planner.save_to_file("planner.json").unwrap();
}
