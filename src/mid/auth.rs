use crate::common::jwt;
use crate::common::res::R;
use crate::models::db::get_pool;
use crate::models::user::User;
use axum::extract::Request;
use axum::http::{header, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use serde_json::Value;
use sqlx::Error;

pub async fn auth(mut req: Request, next: Next) -> Result<impl IntoResponse, (StatusCode, String)> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });
    if token.is_none() {
        return Ok(R::<Value>::code(886, String::from("login user")).into_response());
    }
    let uid = jwt::get_uid(token.unwrap());

    let user: Result<User, Error> = sqlx::query_as("SELECT * FROM user WHERE id = ?")
        .bind(uid)
        .fetch_one(get_pool().unwrap())
        .await;
    if user.is_err() {
        return Ok(R::<Value>::code(886, String::from("login user")).into_response());
    }
    req.extensions_mut().insert(user.unwrap());
    Ok(next.run(req).await)
}
