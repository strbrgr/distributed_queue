use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use axum::{http::StatusCode, response::IntoResponse, routing::post, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

type Queue = Arc<Mutex<VecDeque<Task>>>;

#[derive(Deserialize)]
pub struct CreateTask {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct Task {
    id: Uuid,
    message: String,
}

#[derive(Clone)]
pub struct AppState {
    queue: Queue,
}

async fn create_task(
    state: Extension<AppState>,
    Json(payload): Json<CreateTask>,
) -> impl IntoResponse {
    let task = Task {
        id: Uuid::new_v4(),
        message: payload.message,
    };
    let queue = &state.queue;

    let task_id = task.id;
    queue.lock().unwrap().push_back(task);

    (
        StatusCode::ACCEPTED,
        Json(json!(
            {"id": task_id})),
    )
}

async fn process_queue(state: AppState) {
    println!("{:?}", state.queue)
    //
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    let state = AppState { queue };
    let worker_state = state.clone();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();

    let router = Router::new()
        .route("/tasks", post(create_task))
        .layer(Extension(state));

    tokio::spawn(async move {
        process_queue(worker_state).await;
    });

    axum::serve(listener, router).await;
    Ok(())
}
