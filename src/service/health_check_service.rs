use actix_web::{web::Query, HttpResponse};
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::Error;
use crate::response::simple_response::SimpleResp;

#[derive(Serialize)]
pub struct HealthCheck {
    message: &'static str,
}

pub async fn health_check() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(SimpleResp { message: "Ok" }))
}

#[derive(Serialize, Deserialize)]
pub struct EchoReq {
    #[serde(alias = "p")]
    param: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct EchoResp {
    param: String,
}

pub async fn echo(Query(req): Query<EchoReq>) -> Result<HttpResponse, Error> {
    match req.param {
        Some(v) => {
            if v != "" {
                Ok(HttpResponse::Ok().json(EchoResp { param: v }))
            } else {
                let err = Error::NotFound(json!({"msg": "parameter is not found"}));
                debug!("{}", err);
                Err(err)
            }
        }
        _ => Ok(HttpResponse::Ok().json(EchoResp {
            param: "Ok".to_string(),
        })),
    }
}
