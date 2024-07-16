use std::sync::Mutex;

use rocket::{get, post, routes, serde::json::Json, Build, Rocket, State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::{ActivityType, Module},
    helpers::{request::Request, response::Response, InternalError},
    interval::Interval,
    state::IntrastekState,
};

// pub fn load_activities(rocket: Rocket<Build>) -> Rocket<Build> {
//     rocket.mount(
//         "/activities",
//         routes![get_activities, add_activity, get_activity],
//     )
// }

// #[get("/")]
// async fn get_activities(state: &State<Mutex<IntrastekState>>) -> Response<Vec<Uuid>, String> {
//     get_state(state, |mutex| {
//         Ok(mutex
//             .planner
//             .activities
//             .clone()
//             .iter()
//             .map(|a| a.id)
//             .collect())
//     })
//     .into()
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityRequest {
    pub activity: ActivityType,
    pub interval: Interval,
    pub location: String,
    pub needed_asteks: u32,
    pub module: Option<Module>,
}

// #[post("/", data = "<activity>")]
// async fn add_activity(
//     activity: Json<Request<ActivityRequest>>,
//     state: &State<Mutex<IntrastekState>>,
// ) -> Response<Uuid, String> {
//     get_state_mut(state, |mutex| {
//         let act: Activity = activity.data.clone().into();
//         mutex.planner.add_activity(act.clone());
//         Ok(act.id)
//     })
//     .into()
// }

// #[get("/<id>")]
// async fn get_activity(
//     id: Uuid,
//     state: &State<Mutex<IntrastekState>>,
// ) -> Response<Activity, String> {
//     get_state(state, |mutex| {
//         if let Some(activity) = mutex.planner.activities.iter().find(|a| a.id == id) {
//             Ok(activity.clone())
//         } else {
//             Err(Box::new(InternalError))
//         }
//     })
//     .into()
// }
