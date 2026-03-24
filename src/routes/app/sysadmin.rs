use axum::{Router, routing::get};

pub fn router() -> Router<()> {
    Router::new()
        .route("/home", get(get_home))
        .route("/users", get(get_users_list))
        .route("/users/new", get(get_new_user_form))
        .route("/users/{id}", get(get_user_details))
        .route("/users/{id}/edit", get(get_user_edit_form))
        .route("/supervisors", get(get_supervisors_list))
        .route("/supervisors/new", get(get_new_supervisor_form))
}

async fn get_home() -> &'static str {
    "sysadmin home"
}

async fn get_users_list() -> &'static str {
    "users list"
}

async fn get_new_user_form() -> &'static str {
    "new user"
}

async fn get_user_details() -> &'static str {
    "user details"
}

async fn get_user_edit_form() -> &'static str {
    "edit user details"
}

async fn get_supervisors_list() -> &'static str {
    "supervisors list"
}

async fn get_new_supervisor_form() -> &'static str {
    "new supervisor"
}
