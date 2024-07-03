use std::sync::Mutex;

use rocket::{get, post, routes, serde::json::Json, Build, Rocket, State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    activity::{Activities, Activity},
    helpers::{request::Request, response::Response},
    interval::Interval,
    module::Module,
    state::IntrastekState,
};

pub fn load_activities(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount(
        "/activities",
        routes![get_activities, add_activity, get_activity],
    )
}

#[get("/")]
async fn get_activities(state: &State<Mutex<IntrastekState>>) -> Response<Vec<Activity>, String> {
    match state.lock() {
        Ok(mutex) => Response::ok(200, mutex.planner.activities.clone()),
        Err(_) => Response::err(500, String::from("Internal error")),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityRequest {
    pub activity: Activities,
    pub interval: Interval,
    pub location: String,
    pub needed_asteks: u32,
    pub module: Option<Module>,
}

#[post("/", data = "<activity>")]
async fn add_activity(
    activity: Json<Request<ActivityRequest>>,
    state: &State<Mutex<IntrastekState>>,
) -> Response<Uuid, String> {
    match state.lock() {
        Ok(mut mutex) => {
            let act: Activity = activity.0.data.into();
            mutex.planner.add_activity(act.clone());
            Response::ok(200, act.id)
        }
        Err(_) => Response::err(500, String::from("Internal error")),
    }
}

#[get("/<id>")]
async fn get_activity(
    id: Uuid,
    state: &State<Mutex<IntrastekState>>,
) -> Response<Activity, String> {
    match state.lock() {
        Ok(mutex) => {
            if let Some(activity) = mutex.planner.activities.iter().find(|a| a.id == id) {
                Response::ok(200, activity.clone())
            } else {
                Response::err(404, String::from("Not found"))
            }
        }
        Err(_) => Response::err(500, String::from("Internal error")),
    }
}
