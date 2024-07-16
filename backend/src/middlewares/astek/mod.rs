use std::sync::Arc;

use indisponibility::IndisponibilityInfos;
use log::error;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    db::{astek, PrismaClient},
    helpers::{InternalError, IntrastekError, NotFound},
};

use crate::db;

pub mod indisponibility;

#[derive(Debug, Clone, Serialize)]
pub struct AstekInfos {
    pub id: String,
    pub indisponibilities: Vec<IndisponibilityInfos>,
}

impl TryFrom<db::astek::Data> for AstekInfos {
    type Error = Box<dyn IntrastekError>;

    fn try_from(value: db::astek::Data) -> Result<Self, Self::Error> {
        let indispos: Vec<IndisponibilityInfos> =
            match value.indisponibility().map_err(|_| Box::new(InternalError)) {
                Ok(indispos) => indispos,
                Err(e) => return Err(e),
            }
            .iter()
            .map(|i| IndisponibilityInfos::try_from(i.clone()))
            .filter_map(Result::ok)
            .collect();
        Ok(AstekInfos {
            id: value.id,
            indisponibilities: indispos,
        })
    }
}

pub async fn create_astek(
    email: &String,
    db: &Arc<PrismaClient>,
) -> Result<AstekInfos, Box<dyn IntrastekError>> {
    let new_uuid = uuid::Uuid::new_v4();

    match db
        .astek()
        .create(new_uuid.to_string(), email.clone(), vec![])
        .exec()
        .await
    {
        Ok(new) => new.try_into(),
        Err(e) => {
            error!("Failed to create astek: {e}");
            Err(Box::new(InternalError))
        }
    }
}

pub async fn get_asteks(
    db: &Arc<PrismaClient>,
) -> Result<Vec<AstekInfos>, Box<dyn IntrastekError>> {
    match db.astek().find_many(vec![]).exec().await {
        Ok(asteks) => Ok(asteks
            .into_iter()
            .flat_map(|a| AstekInfos::try_from(a))
            .collect()),
        Err(_) => {
            error!("Failed to get asteks");
            Err(Box::new(InternalError))
        }
    }
}

pub async fn get_astek(
    db: &Arc<PrismaClient>,
    uuid: Uuid,
) -> Result<AstekInfos, Box<dyn IntrastekError>> {
    match db
        .astek()
        .find_unique(astek::id::equals(uuid.to_string()))
        .exec()
        .await
    {
        Ok(Some(astek)) => astek.try_into(),
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
