use std::sync::Arc;

use log::error;
use uuid::Uuid;

use crate::{
    db::{astek, PrismaClient},
    helpers::{InternalError, IntrastekError, NotFound},
};

use crate::db;

pub mod indisponibility;

pub async fn create_astek(
    email: &String,
    db: &Arc<PrismaClient>,
) -> Result<db::astek::Data, Box<dyn IntrastekError>> {
    let new_uuid = uuid::Uuid::new_v4();

    match db
        .astek()
        .create(new_uuid.to_string(), email.clone(), vec![])
        .exec()
        .await
    {
        Ok(new) => Ok(new),
        Err(e) => {
            error!("Failed to create astek: {e}");
            Err(Box::new(InternalError))
        }
    }
}

pub async fn get_asteks(
    db: &Arc<PrismaClient>,
) -> Result<Vec<db::astek::Data>, Box<dyn IntrastekError>> {
    match db.astek().find_many(vec![]).exec().await {
        Ok(asteks) => Ok(asteks),
        Err(_) => {
            error!("Failed to get asteks");
            Err(Box::new(InternalError))
        }
    }
}

pub async fn get_astek(
    db: &Arc<PrismaClient>,
    uuid: Uuid,
) -> Result<astek::Data, Box<dyn IntrastekError>> {
    match db
        .astek()
        .find_unique(astek::id::equals(uuid.to_string()))
        .exec()
        .await
    {
        Ok(Some(astek)) => Ok(astek),
        _ => {
            error!("Failed to get astek {uuid}");
            Err(Box::new(NotFound { data: uuid }))
        }
    }
}

pub async fn delete_astek(
    db: &Arc<PrismaClient>,
    uuid: Uuid,
) -> Result<(), Box<dyn IntrastekError>> {
    match db
        .astek()
        .delete(astek::id::equals(uuid.to_string()))
        .exec()
        .await
    {
        Ok(_) => Ok(()),
        _ => {
            error!("Failed to delete astek {uuid}");
            Err(Box::new(NotFound { data: uuid }))
        }
    }
}
