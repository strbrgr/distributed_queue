use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::service::{load_balancer, queue::SharedQueue};

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

    let mut shared_queue = SharedQueue::<Task>::new();
    shared_queue.push_back(task);

    load_balancer::pull_from_queue(&mut shared_queue);

    (
        StatusCode::ACCEPTED,
        Json(json!(
            {"id": task_id})),
    )
}
