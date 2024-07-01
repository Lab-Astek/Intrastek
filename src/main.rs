#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

use crate::astek::astek::Astek;
use activity::{Activities, Activity};
use module::Module;
use planner::Planner;

mod activity;
mod astek;
mod module;
mod planner;
mod interval;

fn main() {
    let mut planner = Planner::from_file("planner.json").unwrap_or(Planner::new());
    let mut asteks: Vec<Rc<RefCell<Astek>>> = Vec::new();

    let activity = Activity::new(
        "2024-07-01T10:59:31.130656344+02:00",
        Activities::FollowUp,
        "Home",
        "2024-07-01T12:59:31.130656344+02:00",
        1,
        Some(Module::Cpe),
    );
    let activity2 = Activity::new(
        "2024-07-01T10:00:31.130656344+02:00",
        Activities::Bootstrap,
        "Home",
        "2024-07-01T17:59:31.130656344+02:00",
        1,
        Some(Module::Psu),
    );
    planner.add_activity(activity);
    planner.add_activity(activity2);
    let astek = Rc::new(RefCell::new(Astek::new("Alice")));

    astek.as_ref().borrow_mut().add_indisponibility(
        "2024-07-01T10:59:31.130656344+02:00",
        "2024-07-01T12:59:31.130656344+02:00",
    );

    let astek2 = Rc::new(RefCell::new(Astek::new("Bob")));
    let astek3 = Rc::new(RefCell::new(Astek::new("Paul")));

    asteks.push(astek);
    asteks.push(astek2);
    asteks.push(astek3);

    let _ = planner.compute(&asteks);
    println!("{}", planner);
    asteks
        .iter()
        .for_each(|astek| println!("{}", astek.borrow()));
}
