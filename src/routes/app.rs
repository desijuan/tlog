use crate::db::DB;
use axum::Router;

mod change_password;
mod login;
mod resources;
mod supervisor;
mod sysadmin;
mod worker;

pub fn router() -> Router<DB> {
    Router::new()
        .nest("/login", login::router())
        .nest("/change-password", change_password::router())
        .nest("/worker", worker::router())
        .nest("/supervisor", supervisor::router())
        .nest("/sysadmin", sysadmin::router())
}
