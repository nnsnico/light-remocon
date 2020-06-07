use actix_web::{error::ResponseError, HttpResponse};
use failure::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Fail, Debug, Deserialize, Serialize)]
pub enum Error {
    #[fail(display = "Internal Server Error")]
    InternalServerError,

    #[fail(display = "NotFound: {}", _0)]
    NotFound(JsonValue),
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::InternalServerError => {
                HttpResponse::InternalServerError().json("InternalServerError")
            }
            Error::NotFound(ref error) => HttpResponse::NotFound().json(error),
        }
    }
}
