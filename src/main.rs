#![allow(dead_code)]

use activity::{Activities, Activity};
use astek::Astek;
use env_logger::{Builder, Env};
use helpers::{request::Request, response::Response};
use log::{info, warn};
use module::Module;
use planner::Planner;
use rocket::{get, post, routes, serde::json::Json};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

mod activity;
mod astek;
mod helpers;
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

async fn create_test_asteks() -> Result<Vec<Arc<RwLock<Astek>>>, String> {
    let mut asteks = Vec::new();

    for _ in 0..20 {
        let astek = Arc::new(RwLock::new(
            Astek::new(Uuid::new_v4().to_string().as_str()).map_err(|e| e.to_string())?,
        ));

        asteks.push(astek);
    }

    Ok(asteks)
}

#[post("/register-astek", data = "<informations>")]
async fn index(informations: Json<Request<Uuid>>) -> Json<Response<Uuid>> {
    info!("{}", informations.0.data);
    Json(informations.0.into())
}

#[get("/")]
async fn echo() -> &'static str {
    "Hello, world!"
}

//2fdfd8fe-59c0-4a93-9f3b-e0f75110bb1b

#[rocket::main]
async fn main() -> Result<(), String> {
    let env = Env::new().filter("ASSIGN_LOG");
    Builder::from_env(env).init();

    let asteks = create_test_asteks().await?;
    // let mut planner = create_test_planner();
    // match planner.compute(&asteks) {
    //     Ok(_) => (),
    //     Err(e) => error!("{}", e),
    // }
    // println!("{}", planner);
    asteks.iter().try_for_each(|astek| {
        warn!("{}", astek.as_ref().read().map_err(|e| e.to_string())?);
        Ok::<(), String>(())
    })?;

    rocket::build()
        .mount("/", routes![index, echo])
        .launch()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
