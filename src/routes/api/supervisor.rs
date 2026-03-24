use axum::{
    Router,
    routing::{get, patch, post},
};

pub fn router() -> Router {
    Router::new()
        .route("/workers", get(get_workers_list).post(create_worker))
        .route("/workers/{id}", get(get_worker_detail).patch(edit_worker))
        .route("/workers/{id}/sessions", get(get_worker_sessions))
        .route("/sessions", get(get_all_sessions))
        .route("/reports/hours", get(get_reports_hours))
        .route("/schedules", get(get_schedules_list).post(create_schedule))
        .route(
            "/schedules/{id}",
            get(get_schedule_detail).patch(edit_schedule),
        )
}

async fn get_workers_list() -> &'static str {
    "supervisor-workers-list"
}

async fn create_worker() -> &'static str {
    "supervisor-create-worker"
}

async fn get_worker_detail() -> &'static str {
    "supervisor-worker-detail"
}

async fn edit_worker() -> &'static str {
    "supervisor-edit-worker"
}

async fn get_worker_sessions() -> &'static str {
    "supervisor-worker-sessions"
}

async fn get_all_sessions() -> &'static str {
    "supervisor-all-sessions"
}

async fn get_reports_hours() -> &'static str {
    "supervisor-reports-hours"
}

async fn get_schedules_list() -> &'static str {
    "supervisor-schedules-list"
}

async fn create_schedule() -> &'static str {
    "supervisor-create-schedule"
}

async fn get_schedule_detail() -> &'static str {
    "supervisor-schedule-detail"
}

async fn edit_schedule() -> &'static str {
    "supervisor-edit-schedule"
}
