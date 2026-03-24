use axum::Router;

mod auth;
mod supervisor;
mod sysadmin;
mod worker;

pub fn router() -> Router {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/worker", worker::router())
        .nest("/supervisor", supervisor::router())
        .nest("/sysadmin", sysadmin::router())
}
