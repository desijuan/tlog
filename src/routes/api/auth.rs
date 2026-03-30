use crate::db::DB;
use axum::{Router, routing::post};

pub fn router() -> Router<DB> {
    Router::new()
        .route("/login", post(auth_login))
        .route("/logout", post(auth_logout))
        .route("/change-password", post(auth_change_password))
}

async fn auth_login() -> &'static str {
    "auth-login"
}

async fn auth_logout() -> &'static str {
    "auth-logout"
}

async fn auth_change_password() -> &'static str {
    "auth-change-password"
}
