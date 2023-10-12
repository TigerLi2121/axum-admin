use axum::{
    body::{Body, Bytes},
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use time::Instant;
use tracing::info;

pub async fn print_request_response(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    let uri = parts.uri.to_string();
    let method = parts.method.to_string();
    let bytes = get_bytes(body).await?;
    let req_body = std::str::from_utf8(&bytes);
    info!("{} {} req:{}", method, uri, req_body.unwrap());
    let req = Request::from_parts(parts, Body::from(bytes));
    let start = Instant::now();
    let res = next.run(req).await;
    let duration = start.elapsed();
    let (parts, body) = res.into_parts();
    let bytes = get_bytes(body).await?;
    let res_body = std::str::from_utf8(&bytes);
    info!("{} {} {} res:{}", method, uri, duration, res_body.unwrap());
    let res = Response::from_parts(parts, Body::from(bytes));
    Ok(res)
}

async fn get_bytes<B>(body: B) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((StatusCode::BAD_REQUEST, format!("failed to body: {}", err)));
        }
    };
    Ok(bytes)
}
