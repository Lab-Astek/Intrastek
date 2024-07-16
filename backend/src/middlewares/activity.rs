use std::sync::Arc;

use log::error;
use uuid::Uuid;

use crate::{
    db::{activity, ActivityType, PrismaClient},
    helpers::{InternalError, IntrastekError},
    routes::activities::ActivityRequest,
};

pub async fn create_activity(
    activity: ActivityRequest,
    db: &Arc<PrismaClient>,
) -> Result<(), Box<dyn IntrastekError>> {
    let new_uuid = Uuid::new_v4();

    match db
        .activity()
        .create(
            new_uuid.to_string(),
            ActivityType::Bootstrap,
            activity.interval.start,
            activity.interval.end,
            activity.location.clone(),
            activity.needed_asteks as i32,
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
    Ok(())
}
