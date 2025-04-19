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
type Workers = Arc<Mutex<Vec<Worker>>>;

#[derive(Clone, Debug)]
enum WorkerState {
    Idle,
    Working,
}

#[derive(Clone, Debug)]
struct Worker {
    id: Uuid,
    state: WorkerState,
    handled_tasks: i32,
    task_id: Option<Uuid>,
}

impl Worker {
    pub fn new() -> Self {
        Worker {
            id: Uuid::new_v4(),
            state: WorkerState::Idle,
            handled_tasks: 0,
            task_id: None,
        }
    }

    pub async fn handle_task(&mut self, id: &Uuid) {
        println!(
            "Worker with id: {} is handling task with id: {}",
            self.id, id
        );
        time::sleep(time::Duration::from_secs(5)).await;
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
    workers: Workers,
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

fn get_idle_worker_idx(workers: &Workers) -> Option<usize> {
    let workers_guard = workers.lock().unwrap();
    if let Some((idx, _)) = workers_guard
        .iter()
        .enumerate()
        .find(|(_, w)| matches!(w.state, WorkerState::Idle))
    {
        Some(idx)
    } else {
        println!("All workers are busy crunching numbers");
        None
    }
}

async fn process_queue(state: Arc<AppState>) {
    loop {
        let mut queue = state.queue.lock().unwrap();
        let idle_worker_idx = get_idle_worker_idx(&state.workers);
        if let Some(idx) = idle_worker_idx {
            println!("Idle worker index is: {}", idx);
            if let Some(task) = queue.pop_back() {
                handle_load(state.workers.clone(), task, idx);
            }
        }
        drop(queue);

        thread::sleep(time::Duration::from_secs(1));
    }
}

fn handle_load(workers: Arc<Mutex<Vec<Worker>>>, task: Task, idx: usize) {
    let mut workers_guard = workers.lock().unwrap();
    if let Some(w) = workers_guard.get_mut(idx) {
        w.state = WorkerState::Working;
        w.task_id = Some(task.id);
        let mut worker = w.clone();

        drop(workers_guard);

        // Background processing
        tokio::spawn(async move {
            worker.handle_task(&task.id).await;
            let mut workers = workers.lock().unwrap();
            if let Some(w) = workers.get_mut(idx) {
                w.state = WorkerState::Idle;
                w.task_id = None;
                w.handled_tasks += 1;
            }
        });
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
