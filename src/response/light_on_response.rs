use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LightOnResp {
    status: u16,
    message: Option<String>,
}
