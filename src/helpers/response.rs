use super::request::Request;
use rocket::http::{ContentType, Status};
use rocket::request;
use rocket::response::{self, Responder};
use rocket::serde::json::Json;
use serde::Serialize;

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
