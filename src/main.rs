#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

use activity::{Activities, Activity};
use astek::Astek;
use module::Module;
use planner::Planner;

mod activity;
mod astek;
mod interval;
mod module;
mod planner;

fn create_test_planner() -> Planner {
    let mut planner = Planner::new();

    for i in 1..=5 {
        let permanence = Activity::new(
            format!("2024-07-0{}T10:00:00.000000000+02:00", i).as_str(),
            Activities::Permanence,
            "Lab",
            format!("2024-07-0{}T18:00:00.000000000+02:00", i).as_str(),
            4,
            None,
        );

        planner.add_activity(permanence);
    }

    let fu_cpe = Activity::new(
        "2024-07-06T10:00:00.000000000+02:00",
        Activities::FollowUp,
        "Grace Hopper",
        "2024-07-06T18:00:00.000000000+02:00",
        2,
        Some(Module::Cpe),
    );

    planner.add_activity(fu_cpe);
    planner
}

fn create_test_asteks() -> Vec<Rc<RefCell<Astek>>> {
    let mut asteks = Vec::new();

    for i in 0..20 {
        let astek = Rc::new(RefCell::new(Astek::new(format!("Astek {}", i).as_str())));

        asteks.push(astek);
    }

    asteks
}

fn main() {
    let asteks = create_test_asteks();
    let mut planner = create_test_planner();
    let _ = planner.compute(&asteks);
    println!("{}", planner);
    // asteks
    //     .iter()
    //     .for_each(|astek| println!("{}", astek.borrow()));
}
