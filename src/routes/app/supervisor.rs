use axum::{Router, routing::get};

pub fn router() -> Router<()> {
    Router::new()
        .route("/home", get(get_home))
        .route("/workers", get(get_workers_list))
        .route("/workers/new", get(get_new_worker_form))
        .route("/workers/{id}", get(get_worker_details))
        .route("/workers/{id}/edit", get(get_worker_edit_form))
        .route("/sessions", get(get_sessions_list))
        .route("/reports", get(get_reports))
        .route("/schedules", get(get_schedules_list))
        .route("/schedules/new", get(get_new_schedule_form))
        .route("/schedules/{id}/edit", get(get_schedule_edit_form))
}

async fn get_home() -> &'static str {
    "supervisor home"
}

async fn get_workers_list() -> &'static str {
    "workers list"
}

async fn get_new_worker_form() -> &'static str {
    "new worker"
}

async fn get_worker_details() -> &'static str {
    "worker details"
}

async fn get_worker_edit_form() -> &'static str {
    "edit worker details"
}

async fn get_sessions_list() -> &'static str {
    "sessions list"
}

async fn get_reports() -> &'static str {
    "reports"
}

async fn get_schedules_list() -> &'static str {
    "schedules list"
}

async fn get_new_schedule_form() -> &'static str {
    "new schedule"
}

async fn get_schedule_edit_form() -> &'static str {
    "edit schedule"
}
