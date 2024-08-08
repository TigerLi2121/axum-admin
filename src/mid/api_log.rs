use axum::{
    body::{to_bytes, Body},
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::time::Instant;
use tracing::info;

pub async fn print_request_response(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let uri = parts.uri.to_string();
    let method = parts.method.to_string();
    let bytes = to_bytes(body, usize::MAX).await.unwrap();
    let req_bytes = &bytes.clone();
    let req_val = std::str::from_utf8(req_bytes).unwrap();
    info!("{} {} req:{}", method, uri, req_val);
    let req = Request::from_parts(parts, Body::from(bytes));
    let start = Instant::now();
    let res = next.run(req).await;
    let duration = start.elapsed();
    let (parts, body) = res.into_parts();
    let bytes = to_bytes(body, usize::MAX).await.unwrap();
    let res_body = std::str::from_utf8(&bytes);
    info!(
        "{} {} {:?} req:{} res:{}",
        method,
        uri,
        duration,
        req_val,
        res_body.unwrap()
    );
    let res = Response::from_parts(parts, Body::from(bytes));
    Ok(res)
}
