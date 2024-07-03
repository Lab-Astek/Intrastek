use std::fmt::Display;

pub mod request;
pub mod response;

pub trait IntrastekError {
    fn get_code(&self) -> u16;
    fn get_message(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct NotFound<T: Clone + Display> {
    pub data: T,
}

impl<T> IntrastekError for NotFound<T>
where
    T: Clone + Display,
{
    fn get_code(&self) -> u16 {
        404
    }

    fn get_message(&self) -> String {
        format!("{} not found", self.data)
    }
}

#[derive(Debug, Clone)]
pub struct AlreadyExists<T: Clone + Display> {
    pub data: T,
}

impl<T> IntrastekError for AlreadyExists<T>
where
    T: Clone + Display,
{
    fn get_code(&self) -> u16 {
        409
    }

    fn get_message(&self) -> String {
        format!("{} already exists", self.data)
    }
}

#[derive(Debug, Clone)]
pub struct InternalError;

impl IntrastekError for InternalError {
    fn get_code(&self) -> u16 {
        500
    }

    fn get_message(&self) -> String {
        "Internal Error".to_string()
    }
}
