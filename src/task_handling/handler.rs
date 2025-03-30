use axum::Json;

use crate::{queue::queue::Queue, routes::tasks::Task};

pub async fn handle_task(task: Json<Task>, q: &Queue) {}
