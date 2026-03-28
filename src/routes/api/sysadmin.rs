use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router {
    Router::new()
        .route("/users", get(get_users_list).post(create_user))
        .route(
            "/users/{id}",
            get(get_user_detail).patch(edit_user).delete(delete_user),
        )
        .route("/users/{id}/reset-password", post(reset_password))
}

async fn get_users_list() -> &'static str {
    "sysadmin-users-list"
}

async fn create_user() -> &'static str {
    "sysadmin-create-user"
}

async fn get_user_detail() -> &'static str {
    "sysadmin-user-detail"
}

async fn edit_user() -> &'static str {
    "sysadmin-edit-user"
}

async fn delete_user() -> &'static str {
    "sysadmin-delete-user"
}

async fn reset_password() -> &'static str {
    "sysadmin-reset-password"
}
