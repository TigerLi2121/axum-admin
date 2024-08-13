use crate::common::req::Pagination;
use crate::common::res::R;
use axum::extract::Query;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;

pub async fn login(login: Json<Login>) -> R<Value> {
    info!("{:?}", login);
    R::ok()
    // anyhow::bail!("it failed!");
}

pub async fn page(pagination: Query<Pagination>) -> R<Value> {
    info!("{pagination:?}");
    R::ok()
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Login {
    username: String,
    password: String,
}
