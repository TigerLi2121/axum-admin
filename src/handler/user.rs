use crate::common::res::R;
use axum::Json;
use axum_valid::Valid;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;
use validator::Validate;

pub async fn login(Valid(Json(login)): Valid<Json<Login>>) -> R<Value> {
    info!("{:?}", login);
    // R::ok()
    R::err_msg("登录失败".to_string())
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct Login {
    #[validate(length(min = 1, message = "username is null"))]
    username: String,
    #[validate(length(min = 1, message = "password is null"))]
    password: String,
}
