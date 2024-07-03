use std::fmt::{Debug, Display};

use super::request::Request;
use super::IntrastekErrors;
use rocket::http::{ContentType, Status};
use rocket::request;
use rocket::response::{self, Responder};
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Debug)]
pub struct Response<T, E> {
    pub code: u16,
    pub data: Result<Json<T>, Json<E>>,
}

impl<T, E> Response<T, E> {
    pub fn ok(code: u16, data: T) -> Self {
        Self {
            code,
            data: Ok(Json(data)),
        }
    }

    pub fn err(code: u16, data: E) -> Self {
        Self {
            code,
            data: Err(Json(data)),
        }
    }
}

impl<T, E> From<Request<T>> for Response<T, E> {
    fn from(request: Request<T>) -> Self {
        Response {
            code: 200,
            data: Ok(Json(request.data)),
        }
    }
}

impl<'r, 'o: 'r, T, E> Responder<'r, 'o> for Response<T, E>
where
    T: Serialize,
    E: Serialize,
{
    fn respond_to(self, request: &'r request::Request<'_>) -> response::Result<'o> {
        match self.data {
            Ok(data) => response::Response::build_from(data.respond_to(request).unwrap()),
            Err(data) => response::Response::build_from(data.respond_to(request).unwrap()),
        }
        .status(Status::from_code(self.code).unwrap())
        .header(ContentType::JSON)
        .ok()
    }
}

impl<T, E> From<Result<T, E>> for Response<T, String> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(data) => Response::ok(200, data),
            Err(_) => Response::err(500, String::from("Internal error")),
        }
    }
}

impl<T> From<IntrastekErrors<T>> for Response<T, String>
where
    T: Display,
{
    fn from(value: IntrastekErrors<T>) -> Self {
        match value {
            IntrastekErrors::NotFound(id) => Response::err(404, format!("{} not found", id)),
            IntrastekErrors::AlreadyExists(id) => {
                Response::err(409, format!("{} already exists", id))
            }
            IntrastekErrors::InternalError => Response::err(500, String::from("Internal error")),
        }
    }
}
