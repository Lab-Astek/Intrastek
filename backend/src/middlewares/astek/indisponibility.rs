use std::sync::Arc;

use log::error;
use prisma_client_rust::and;
use uuid::Uuid;

use crate::{
    astek::indisponibility::Indisponibility,
    db::{
        astek::{self},
        indisponibility, PrismaClient,
    },
    helpers::{InternalError, IntrastekError, NotFound},
};

pub async fn add_indisponibility_to_astek(
    id: Uuid,
    indisponibility: &Indisponibility,
    db: &Arc<PrismaClient>,
) -> Result<i32, Box<dyn IntrastekError>> {
    match db
        .indisponibility()
        .create(
            astek::id::equals(id.to_string()),
            indisponibility.get_interval().start,
            indisponibility.get_interval().end,
            vec![],
        )
        .exec()
        .await
    {
        Ok(indisponibility) => Ok(indisponibility.id),
        Err(_) => {
            error!("Failed to add indisponibility to astek {id}");
            return Err(Box::new(InternalError));
        }
    }
}

pub async fn remove_indisponibility_from_astek(
    id: Uuid,
    indisponibility_id: i32,
    db: &Arc<PrismaClient>,
) -> Result<(), Box<dyn IntrastekError>> {
    match db
        .indisponibility()
        .delete_many(vec![and![
            indisponibility::id::equals(indisponibility_id),
            indisponibility::astek_id::equals(id.to_string()),
        ]])
        .exec()
        .await
    {
        Ok(_) => Ok(()),
        Err(_) => {
            error!("Failed to remove indisponibility {indisponibility_id} from astek {id}");
            return Err(Box::new(NotFound {
                data: indisponibility_id,
            }));
        }
    }
}
