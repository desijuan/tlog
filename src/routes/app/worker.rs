use axum::{Router, routing::get};

pub fn router() -> Router<()> {
    Router::new()
        .route("/home", get(get_worker_home))
        .route("/sessions", get(get_worker_sessions))
        .route("/sessions/{id}/edit", get(get_worker_sessions_edit))
}

async fn get_worker_home() -> &'static str {
    "worker-home"
}

async fn get_worker_sessions() -> &'static str {
    "worker-sessions"
}

async fn get_worker_sessions_edit() -> &'static str {
    "worker-sessions-edit"
}
