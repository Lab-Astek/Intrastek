use serde::{Deserialize, Serialize};

use super::request::Request;

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    pub data: T,
}

impl<T> From<Request<T>> for Response<T> {
    fn from(request: Request<T>) -> Self {
        Response { data: request.data }
    }
}
