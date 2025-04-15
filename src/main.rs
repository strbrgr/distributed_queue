use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread,
};

use axum::{http::StatusCode, response::IntoResponse, routing::post, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::time;
use uuid::Uuid;

type Queue = Arc<Mutex<VecDeque<Task>>>;

#[derive(Clone)]
enum WorkerState {
    Idle,
    Working,
}

#[derive(Clone)]
struct Worker {
    id: Uuid,
    state: WorkerState,
    healthy: bool,
    handled_tasks: i32,
}

impl Worker {
    pub fn new() -> Self {
        Worker {
            id: Uuid::new_v4(),
            state: WorkerState::Idle,
            healthy: true,
            handled_tasks: 0,
        }
    }

    pub fn handle_task(&mut self, task: Task) {}
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

#[derive(Clone)]
pub struct AppState {
    queue: Queue,
    workers: Vec<Worker>,
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
    loop {
        let mut queue = state.queue.lock().unwrap();
        if !queue.is_empty() {
            let task = queue.pop_back().unwrap();
            handle_load(&state.workers, task);
        }
        drop(queue);
        thread::sleep(time::Duration::from_secs(5));
    }
}

fn handle_load(workers: &Vec<Worker>, task: Task) {
    todo!() // Handle load balancing
}

fn init_workers() -> Vec<Worker> {
    let mut workers: Vec<Worker> = Vec::new();
    for _ in 0..4 {
        let worker = Worker::new();
        workers.push(worker);
    }
    workers
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    let workers = init_workers();

    let state = AppState { queue, workers };
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
