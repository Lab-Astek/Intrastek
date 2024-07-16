use rocket::{get, post, routes, serde::json::Json, Build, Rocket, State};
use uuid::Uuid;

use crate::{
    helpers::{request::Request, response::Response},
    middlewares::activity::{self, ActivityInfos},
    state::IntrastekState,
};

pub fn load_activities(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount(
        "/activities",
        routes![get_activities, add_activity, get_activity],
    )
}

/// Get all activities registered
#[get("/")]
async fn get_activities(state: &State<IntrastekState>) -> Response<Vec<ActivityInfos>, String> {
    activity::get_activities(&state.db).await.into()
}

/// Register a new activity
#[post("/", data = "<activity>")]
async fn add_activity(
    activity: Json<Request<ActivityInfos>>,
    state: &State<IntrastekState>,
) -> Response<Uuid, String> {
    activity::create_activity(activity.data.clone(), &state.db)
        .await
        .into()
}

/// Get a specific activity
#[get("/<id>")]
async fn get_activity(id: Uuid, state: &State<IntrastekState>) -> Response<ActivityInfos, String> {
    activity::get_activity(&state.db, id).await.into()
}
