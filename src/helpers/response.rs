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

impl<T, E> From<Result<T, IntrastekErrors<E>>> for Response<T, String>
where
    E: Display + Copy,
{
    fn from(result: Result<T, IntrastekErrors<E>>) -> Self {
        match result {
            Ok(data) => Response::ok(200, data),
            Err(err) => Response::err(err.into(), err.into()),
        }
    }
}

impl<T, E> From<IntrastekErrors<E>> for Response<T, String>
where
    E: Display + Copy,
{
    fn from(value: IntrastekErrors<E>) -> Self {
        Response::err(value.into(), value.into())
    }
}

impl<E> From<Response<(), E>> for Response<&'static str, E> {
    fn from(value: Response<(), E>) -> Self {
        match value.data {
            Ok(_) => Response::ok(value.code, "Ok"),
            Err(data) => Response::err(value.code, data.into_inner()),
        }
    }
}
