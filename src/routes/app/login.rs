use super::resources;
use axum::{
    Router, http,
    response::{Html, IntoResponse},
    routing::get,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_html))
        .route("/styles.css", get(get_css))
        .route("/script.js", get(get_js))
}

async fn get_html() -> Html<&'static str> {
    Html(resources::login_page_html())
}

async fn get_css() -> impl IntoResponse {
    (
        [(http::header::CONTENT_TYPE, "text/css")],
        resources::login_page_css(),
    )
}

async fn get_js() -> impl IntoResponse {
    (
        [(http::header::CONTENT_TYPE, "text/javascript")],
        resources::login_page_js(),
    )
}
