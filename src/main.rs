#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

use activity::{Activities, Activity};
use astek::Astek;
use env_logger::{Builder, Env};
use log::{error, info};
use module::Module;
use planner::Planner;
use uuid::Uuid;

mod activity;
mod astek;
mod interval;
mod module;
mod planner;

fn create_test_planner() -> Planner {
    let mut planner = Planner::new();

    let fu_cpe = Activity::new(
        "2024-07-02T10:00:00.000000000+02:00",
        Activities::FollowUp,
        "Grace Hopper",
        "2024-07-02T18:00:00.000000000+02:00",
        2,
        Some(Module::Cpe),
    );

    planner.add_activity(fu_cpe);
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

    planner
}

fn create_test_asteks() -> Result<Vec<Rc<RefCell<Astek>>>, String> {
    let mut asteks = Vec::new();

    for _ in 0..20 {
        let astek = Rc::new(RefCell::new(
            Astek::new(Uuid::new_v4().to_string().as_str()).map_err(|e| e.to_string())?,
        ));

        asteks.push(astek);
    }

    Ok(asteks)
}

fn main() -> Result<(), String> {
    let env = Env::new().filter("ASSIGN_LOG");
    Builder::from_env(env).init();

    let asteks = create_test_asteks()?;
    // let mut planner = create_test_planner();
    // match planner.compute(&asteks) {
    //     Ok(_) => (),
    //     Err(e) => error!("{}", e),
    // }
    // println!("{}", planner);
    asteks.iter().for_each(|astek| info!("{}", astek.borrow()));
    Ok(())
}
