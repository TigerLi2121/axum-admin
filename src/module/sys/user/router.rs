use crate::common::db::get_pool;
use crate::common::jwt::get_token;
use crate::common::req::Page;
use crate::common::res::{R, RP};
use crate::module::sys::user::model;
use crate::module::sys::user::model::User;
use axum::extract::Query;
use axum::routing::get;
use axum::{Json, Router};
use chrono::Local;
use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Error;
use std::fmt::Debug;

pub fn router() -> Router {
    Router::new().route("/", get(page).post(sou).delete(del))
}

pub async fn login(login: Json<Login>) -> R<String> {
    let user: Result<User, Error> = sqlx::query_as("SELECT * FROM user WHERE username = ?")
        .bind(login.username.to_string())
        .fetch_one(get_pool().unwrap())
        .await;
    if user.is_err() {
        return R::err_msg("username not exist".to_string());
    }
    let user = user.unwrap();
    let pwd = format!("{:x}", Md5::digest(login.password.as_bytes()));
    if user.password.unwrap() != pwd {
        return R::err_msg("password error".to_string());
    }
    let token = get_token(user.id.unwrap());
    R::ok_data(token)
}

pub async fn page(page: Query<Page>) -> RP<Vec<User>> {
    model::page(page.0).await.unwrap()
}

pub async fn sou(mut user: Json<User>) -> R<Value> {
    let now = Some(Local::now().naive_local());
    user.updated_at = now;
    if user.id.is_none() {
        user.created_at = now;
    }
    if user.password.is_some() && user.password.clone().unwrap().is_empty() {
        user.password = Some(format!(
            "{:x}",
            Md5::digest(user.password.clone().unwrap().as_bytes())
        ));
    }
    model::sou(user.0).await.unwrap();
    R::ok()
}

pub async fn del(ids: Json<Vec<u64>>) -> R<Value> {
    model::del(ids.0).await.unwrap();
    R::ok()
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Login {
    username: String,
    password: String,
}
