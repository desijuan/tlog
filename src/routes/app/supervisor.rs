use crate::db::DB;
use axum::{Router, routing::get};

pub fn router() -> Router<DB> {
    Router::new().route("/", get(get_html))
}

async fn get_html() -> &'static str {
    "TODO"
}
