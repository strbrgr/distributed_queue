use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::service::queue::Queue;

#[derive(Deserialize)]
pub struct CreateTask {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct Task {
    id: Uuid,
    message: String,
}

pub async fn create_task(Json(payload): Json<CreateTask>) -> impl IntoResponse {
    let task = Task {
        id: Uuid::new_v4(),
        message: payload.message,
    };

    let task_id = task.id;

    handle_task(task).await;

    (
        StatusCode::ACCEPTED,
        Json(json!(
            {"id": task_id})),
    )
}

// TODO: This needs to get a queue reference, but where needs the queue need to be created?
pub async fn handle_task(task: Task) {
    let q = Queue::new();
    let mut data = q.data.lock().unwrap();
    data.push_back(task);
    // TODO: Make sure it goes out of scope so that I don't have to drop
    drop(data);
}
