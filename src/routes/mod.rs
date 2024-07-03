use std::sync::Mutex;

use activities::load_activities;
use asteks::load_asteks;
use rocket::{get, put, routes, Build, Rocket, State};

use crate::state::IntrastekState;

pub mod activities;
mod asteks;

pub fn init_router(mut rocket: Rocket<Build>) -> Rocket<Build> {
    rocket = rocket.mount("/", routes![ping]);
    rocket = load_activities(rocket);
    load_asteks(rocket)
}

#[get("/")]
pub async fn ping(_state: &State<Mutex<IntrastekState>>) -> &'static str {
    "pong"
}

#[put("/compute")]
pub async fn compute(_state: &State<Mutex<IntrastekState>>) -> &'static str {
    // get_state_mut(state, |mutex| {
    //     mutex.planner.compute(mutex.asteks)
    // });
    "Ok"
}
