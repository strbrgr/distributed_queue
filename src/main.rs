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

#[derive(Clone, Debug, PartialEq)]
enum WorkerState {
    Idle,
    Working,
}

#[derive(Clone, Debug)]
struct Worker {
    id: Uuid,
    state: WorkerState,
    healthy: bool,
    handled_tasks: i32,
    task_id: Option<Uuid>,
}

impl Worker {
    pub fn new() -> Self {
        Worker {
            id: Uuid::new_v4(),
            state: WorkerState::Idle,
            healthy: true,
            handled_tasks: 0,
            task_id: None,
        }
    }

    pub fn handle_task(&mut self, task: Task) {
        println!("{:?}", task);
        self.handled_tasks += 1;
        self.state = WorkerState::Working;
    }
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
    workers: Arc<Mutex<Vec<Worker>>>,
}

async fn create_task(
    state: Extension<Arc<AppState>>,
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

async fn process_queue(state: Arc<AppState>) {
    loop {
        let mut queue = state.queue.lock().unwrap();
        if let Some(task) = queue.pop_back() {
            drop(queue); // early release
            let mut workers = state.workers.lock().unwrap();
            handle_load(&mut workers, task);
        } else {
            drop(queue);
        }
        thread::sleep(time::Duration::from_secs(5));
    }
}

fn handle_load(workers: &mut Vec<Worker>, task: Task) {
    let worker = workers
        .iter_mut()
        .find(|w| matches!(w.state, WorkerState::Idle));

    if let Some(w) = worker {
        w.handle_task(task);
    }
}

fn init_workers() -> Arc<Mutex<Vec<Worker>>> {
    let mut workers: Vec<Worker> = Vec::new();
    for _ in 0..4 {
        let worker = Worker::new();
        workers.push(worker);
    }
    Arc::new(Mutex::new(workers))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    let workers = init_workers();

    let state = Arc::new(AppState { queue, workers });
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
