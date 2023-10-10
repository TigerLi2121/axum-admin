use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct R<T: Serialize> {
    pub code: u16,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> R<T>
where
    T: Serialize,
{
    pub fn new(code: u16, msg: String, data: Option<T>) -> Self {
        Self { code, msg, data }
    }
    pub fn ok() -> Self {
        Self::new(0, "ok".to_string(), None)
    }
    pub fn ok_data(data: T) -> Self {
        Self::new(0, "ok".to_string(), Some(data))
    }
    pub fn err(msg: String) -> Self {
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
