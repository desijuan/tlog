use crate::jwt;
use axum::{
    Router,
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use tower_http::cors::{Any, CorsLayer};

mod api;
mod app;

pub fn router() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/", get(home))
        .nest("/api", api::router())
        .nest("/app", app::router())
        .layer(middleware::from_fn(log_request))
        .layer(cors)
}

async fn log_request(request: Request, next: Next) -> Response {
    tracing::info!(
        method = %request.method(),
        uri = %request.uri(),
        "incoming request"
    );
    next.run(request).await
}

#[allow(dead_code)]
async fn jwt_auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    if let Some(cookie_header) = req.headers().get("Cookie") {
        if let Ok(cookie_str) = cookie_header.to_str() {
            // Parse cookies: "jwt=abc123; other=xyz"
            let token = cookie_str
                .split(';')
                .map(|c| c.trim())
                .find_map(|c| c.strip_prefix("jwt="));

            if let Some(token) = token {
                match jwt::validate_jwt(token) {
                    Ok(claims) => {
                        let user_id: i64 =
                            claims.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;
                        req.extensions_mut().insert(user_id);
                        return Ok(next.run(req).await);
                    }
                    Err(_) => return Err(StatusCode::UNAUTHORIZED),
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

async fn home() -> &'static str {
    concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"))
}
