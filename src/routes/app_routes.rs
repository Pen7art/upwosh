use std::sync::Arc;

use axum::{middleware, routing::get, Router};

use crate::{data::model::app_state::AppState, middleware::auth_middleware::auth_middleware};

pub fn app_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
        .with_state(app_state)
}
