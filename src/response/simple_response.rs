use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleResp {
    pub message: &'static str,
}
