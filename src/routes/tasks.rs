use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::task_handling::handler::handle_task;

/// with state
pub async fn create_task(Json(payload): Json<CreateTask>) -> impl IntoResponse {
    let task = Task {
        id: Uuid::new_v4(),
        message: payload.message,
    };

    let task_id = task.id;

    handle_task(Json(task)).await;

    (
        StatusCode::ACCEPTED,
        Json(json!(
            {"id": task_id})),
    )
}

#[derive(Deserialize)]
pub struct CreateTask {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct Task {
    id: Uuid,
    message: String,
}
