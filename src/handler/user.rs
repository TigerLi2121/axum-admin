use crate::common::req::Page;
use crate::common::res::{R, RP};
use crate::models::user;
use crate::models::user::User;
use axum::extract::Query;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::Error;
use std::fmt::Debug;
use std::future::Future;
use tracing::info;

pub async fn login(login: Json<Login>) -> R<Value> {
    info!("{login:?}");
    R::ok()
    // anyhow::bail!("it failed!");
}

pub async fn page(page: Query<Page>) -> RP<Value> {
    user::page(page.0)
}

pub async fn sou(user: Json<User>) -> R<Value> {
    match user::sou(user.0).await {
        Ok(_) => R::ok(),
        Err(_) => R::err(),
    }
}

pub async fn del(ids: Json<Vec<u64>>) -> R<Value> {
    match user::del(ids.0).await {
        Ok(_) => R::ok(),
        Err(_) => R::err(),
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Login {
    username: String,
    password: String,
}
