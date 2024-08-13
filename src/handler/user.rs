use crate::common::req::Page;
use crate::common::res::R;
use axum::extract::Query;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt::Debug;
use tracing::info;

pub async fn login(login: Json<Login>) -> R<Value> {
    info!("{login:?}");
    R::ok()
    // anyhow::bail!("it failed!");
}

pub async fn page(page: Query<Page>) -> R<Value> {
    info!("{page:?}");
    R::ok_data(json!(page.0))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Login {
    username: String,
    password: String,
}
