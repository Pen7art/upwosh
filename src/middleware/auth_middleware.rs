use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use reqwest::{header::AUTHORIZATION, StatusCode};
use tracing::info;

use crate::data::model::app_state::AppState;

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Response {
    let failed_auth_response = Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .body(Body::new("Unauthorized".to_string()))
        .unwrap();

    info!("Headers: {:?}", headers);

    if headers.contains_key(AUTHORIZATION) {
        let access_token = headers[AUTHORIZATION].to_str();

        if access_token.is_err() {
            return failed_auth_response;
        }

        if access_token.unwrap() != state.access_token {
            return failed_auth_response;
        }
        let response = next.run(request).await;
        return response;
    }

    return failed_auth_response;
}
