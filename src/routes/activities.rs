use std::sync::Mutex;

use rocket::{get, post, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    activity::{Activities, Activity},
    helpers::{request::Request, response::Response},
    interval::Interval,
    module::Module,
    state::IntrastekState,
};

#[get("/activities")]
pub async fn get_activities(
    state: &State<Mutex<IntrastekState>>,
) -> Response<Vec<Activity>, String> {
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

#[post("/activities", data = "<activity>")]
pub async fn add_activity(
    activity: Json<Request<ActivityRequest>>,
    state: &State<Mutex<IntrastekState>>,
) -> Response<Uuid, String> {
    match state.lock() {
        Ok(mut mutex) => {
            let act = Activity::from(activity.0.data);
            mutex.planner.add_activity(act.clone());
            Response::ok(200, act.id)
        }
        Err(_) => Response::err(500, String::from("Internal error")),
    }
}

#[get("/activities/<id>")]
pub async fn get_activity(
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
