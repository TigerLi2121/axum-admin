use crate::common::err::AppError;
use crate::common::res::R;
use anyhow;
use async_trait::async_trait;
use axum::extract::rejection::{FormRejection, JsonRejection};
use axum::extract::{FromRequest, Request};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Form, Json};
use axum_extra::extract::WithRejection;
use axum_valid::Valid;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use thiserror::Error;
use tracing::info;
use validator::Validate;

pub async fn login(
    WithRejection(Json(login), _): WithRejection<Json<Login>, ApiError>,
) -> impl IntoResponse {
    info!("{:?}", login);
    // R::ok()
    // anyhow::bail!("it failed!");

    Json(dbg!(login))
}

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct Login {
    #[validate(length(min = 1, message = "username is null"))]
    username: String,
    #[validate(length(min = 1, message = "password is null"))]
    password: String,
}

// We derive `thiserror::Error`
#[derive(Debug, Error)]
pub enum ApiError {
    // The `#[from]` attribute generates `From<JsonRejection> for ApiError`
    // implementation. See `thiserror` docs for more information
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
}

// We implement `IntoResponse` so ApiError can be used as a response
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::JsonExtractorRejection(json_rejection) => {
                (json_rejection.status(), json_rejection.body_text())
            }
        };

        let payload = json!({
            "message": message,
            "origin": "with_rejection"
        });

        (status, Json(payload)).into_response()
    }
}
