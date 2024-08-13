use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct RP<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub count: i32,
    pub data: T,
}

impl<T: Serialize> RP<T>
{
    pub fn new(code: i32, msg: String, count: i32, data: T) -> Self {
        Self { code, msg, count, data }
    }
    pub fn ok(count: i32, data: T) -> Self {
        Self::new(0, "ok".to_string(), count, data)
    }
}

impl<T> IntoResponse for RP<Vec<T>>
where
    Vec<T>: Serialize,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}


#[derive(Serialize)]
pub struct R<T: Serialize> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> R<T>
where
    T: Serialize,
{
    pub fn new(code: i32, msg: String, data: Option<T>) -> Self {
        Self { code, msg, data }
    }
    pub fn ok() -> Self {
        Self::new(0, "ok".to_string(), None)
    }
    pub fn ok_data(data: T) -> Self {
        Self::new(0, "ok".to_string(), Some(data))
    }
    pub fn err() -> Self {
        Self::new(500, "server error".to_string(), None)
    }
    pub fn err_msg(msg: String) -> Self {
        Self::new(500, msg, None)
    }
}

impl<T> IntoResponse for R<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
