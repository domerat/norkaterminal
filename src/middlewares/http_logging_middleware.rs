use axum::{

    response::Response,
    middleware::{Next},
    extract::Request,
};

pub async fn http_logging_middleware(
    request: Request,
    next: Next,
) -> Response {
    tracing::info!("({}) -> {}", request.method(), request.uri());

    let response = next.run(request).await;

    // do something with `response`...

    response
}
