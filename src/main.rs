use axum::{routing::post, Router};

use distributed_queue::routes::tasks::{self, create_task, Task};

use std::{
    collections::VecDeque,
    sync::{LazyLock, Mutex},
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/tasks", post(create_task()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
