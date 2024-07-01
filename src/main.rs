#![allow(dead_code)]

use activity::{Activities, Activity};
use astek::Astek;
use planner::Planner;

mod activity;
mod astek;
mod planner;

fn main() {
    let mut planner = Planner::from_file("planner.json").unwrap_or(Planner::new());
    let mut asteks: Vec<Astek> = Vec::new();

    let activity = Activity::new(
        "2024-07-01T10:59:31.130656344+02:00",
        Activities::FollowUp,
        "Home",
        "2024-07-01T12:59:31.130656344+02:00",
        2,
    );
    planner.add_activity(activity);
    let mut astek = Astek::new("Alice");
    astek.add_indisponibility(
        "2024-07-01T10:59:31.130656344+02:00",
        "2024-07-01T12:59:31.130656344+02:00",
    );

    let astek2 = Astek::new("Bob");
    let astek3 = Astek::new("Paul");

    asteks.push(astek);
    asteks.push(astek2);
    asteks.push(astek3);

    planner.compute(asteks);
}
