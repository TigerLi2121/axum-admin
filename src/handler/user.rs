use crate::utils::resp::R;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;

pub async fn login(Json(login): Json<Login>) -> R<Value> {
    info!("{:?}", login);
    // R::ok()
    R::err("登录失败".to_string())
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Login {
    username: String,
    password: String,
}
