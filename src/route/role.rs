use crate::common::req::Page;
use crate::common::res::{R, RP};
use crate::model::role;
use crate::model::role::Role;
use axum::extract::Query;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::Value;

pub fn router() -> Router {
    Router::new().route("/", get(page).post(sou).delete(del))
}

pub async fn page(page: Query<Page>) -> RP<Vec<Role>> {
    role::page(page.0).await.unwrap()
}

pub async fn sou(app: Json<Role>) -> R<Value> {
    role::sou(app.0).await.unwrap();
    R::ok()
}

pub async fn del(ids: Json<Vec<u64>>) -> R<Value> {
    role::del(ids.0).await.unwrap();
    R::ok()
}