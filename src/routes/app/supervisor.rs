use axum::{Router, routing::get};

pub fn router() -> Router<()> {
    Router::new().route("/", get(get_html))
}

async fn get_html() -> &'static str {
    "TODO"
}
