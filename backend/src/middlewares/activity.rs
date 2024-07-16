use std::sync::Arc;

use chrono::{DateTime, FixedOffset};
use log::error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    db::{self, activity, ActivityType, Module, PrismaClient},
    helpers::{InternalError, IntrastekError},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityInfos {
    pub r#type: ActivityType,
    pub start: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
    pub name: String,
    pub needed_asteks: i32,
    pub module: Option<Module>,
}

impl From<db::activity::Data> for ActivityInfos {
    fn from(value: db::activity::Data) -> Self {
        ActivityInfos {
            r#type: value.r#type,
            start: value.start,
            end: value.end,
            name: value.name,
            needed_asteks: value.needed_asteks,
            module: value.module,
        }
    }
}

pub async fn create_activity(
    activity: ActivityInfos,
    db: &Arc<PrismaClient>,
) -> Result<Uuid, Box<dyn IntrastekError>> {
    let new_uuid = Uuid::new_v4();

    match db
        .activity()
        .create(
            new_uuid.to_string(),
            activity.r#type,
            activity.start,
            activity.end,
            activity.name.clone(),
            activity.needed_asteks,
            vec![],
        )
        .exec()
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => {
            error!("Failed to create activity");
            return Err(Box::new(InternalError));
        }
    }?;

    db.activity()
        .update(
            activity::id::equals(new_uuid.to_string()),
            vec![activity::module::set(activity.module)],
        )
        .exec()
        .await
        .unwrap();
    Ok(new_uuid)
}

pub async fn get_activities(
    db: &Arc<PrismaClient>,
) -> Result<Vec<ActivityInfos>, Box<dyn IntrastekError>> {
    match db.activity().find_many(vec![]).exec().await {
        Ok(activities) => Ok(activities.into_iter().map(|a| a.into()).collect()),
        Err(_) => {
            error!("Failed to get activities");
            Err(Box::new(InternalError))
        }
    }
}

pub async fn get_activity(
    db: &Arc<PrismaClient>,
    id: Uuid,
) -> Result<ActivityInfos, Box<dyn IntrastekError>> {
    match db
        .activity()
        .find_unique(activity::id::equals(id.to_string()))
        .exec()
        .await
    {
        Ok(Some(activity)) => Ok(activity.into()),
        _ => {
            error!("Failed to get activity {id}");
            Err(Box::new(InternalError))
        }
    }
}