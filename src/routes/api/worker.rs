use axum::{
    Router,
    routing::{get, patch, post},
};

pub fn router() -> Router {
    Router::new()
        .route("/me", get(get_me))
        .route("/sessions", get(get_sessions))
        .route("/sessions/active", get(get_active_session))
        .route("/sessions/clock-in", post(clock_in))
        .route("/sessions/clock-out", post(clock_out))
        .route("/sessions/{id}", patch(edit_session))
}

async fn get_me() -> &'static str {
    "worker-me"
}

async fn get_sessions() -> &'static str {
    "worker-sessions"
}

async fn get_active_session() -> &'static str {
    "worker-active-session"
}

async fn clock_in() -> &'static str {
    "worker-clock-in"
}

async fn clock_out() -> &'static str {
    "worker-clock-out"
}

async fn edit_session() -> &'static str {
    "worker-edit-session"
}
