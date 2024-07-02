use std::sync::Mutex;

use rocket::{get, State};

use crate::{activity::Activity, helpers::response::Response, state::IntrastekState};

#[get("/activities")]
pub async fn get_activities(
    state: &State<Mutex<IntrastekState>>,
) -> Response<Vec<Activity>, String> {
    match state.lock() {
        Ok(mutex) => Response::ok(200, mutex.planner.activities.clone()),
        Err(_) => Response::err(500, String::from("Internal error")),
    }
}
