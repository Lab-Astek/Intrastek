use std::fmt::Display;

use uuid::Uuid;

pub mod request;
pub mod response;

#[derive(Debug, Clone, Copy, Default)]
pub enum IntrastekErrors<T: Copy = Uuid> {
    NotFound(T),
    AlreadyExists(T),
    #[default]
    InternalError,
}

impl<T> From<IntrastekErrors<T>> for String
where
    T: Display,
    T: Copy,
{
    fn from(value: IntrastekErrors<T>) -> Self {
        match value {
            IntrastekErrors::NotFound(id) => format!("{} not found", id),
            IntrastekErrors::AlreadyExists(id) => format!("{} already exists", id),
            IntrastekErrors::InternalError => "Internal error".to_string(),
        }
    }
}

impl<T> From<IntrastekErrors<T>> for u16
where
    T: Copy,
{
    fn from(value: IntrastekErrors<T>) -> Self {
        match value {
            IntrastekErrors::NotFound(_) => 404,
            IntrastekErrors::AlreadyExists(_) => 409,
            IntrastekErrors::InternalError => 500,
        }
    }
}
