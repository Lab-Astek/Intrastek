use uuid::Uuid;

pub mod request;
pub mod response;

#[derive(Debug, Clone, Copy, Default)]
pub enum IntrastekErrors<T = Uuid> {
    NotFound(T),
    AlreadyExists(T),
    #[default]
    InternalError,
}
