#![allow(dead_code)]

use activity::{Activities, Activity};
use astek::Astek;
use env_logger::{Builder, Env};
use helpers::{request::Request, response::Response};
use log::{info, warn};
use module::Module;
use planner::Planner;
use rocket::{get, post, routes, serde::json::Json, State};
use state::IntrastekState;
use std::sync::{Arc, Mutex, RwLock};
use uuid::Uuid;

mod activity;
mod astek;
mod helpers;
mod interval;
mod module;
mod planner;
mod state;

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

#[post("/register-astek", data = "<informations>")]
async fn register_asteks(
    informations: Json<Request<Uuid>>,
    state: &State<Mutex<IntrastekState>>,
) -> Json<Response<()>> {
    let astek = Arc::new(RwLock::new(Astek::new(informations.0.data).unwrap()));

    state.lock().unwrap().asteks.push(astek.clone());
    Json(Response { data: () })
}

#[get("/")]
async fn ping(_state: &State<Mutex<IntrastekState>>) -> &'static str {
    "pong"
}

#[rocket::main]
async fn main() -> Result<(), String> {
    let env = Env::new().filter("ASSIGN_LOG");
    Builder::from_env(env).init();

    rocket::build()
        .mount("/", routes![register_asteks, ping])
        .manage(Mutex::new(IntrastekState::default()))
        .launch()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
