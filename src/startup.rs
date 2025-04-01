use axum::routing::post;
use axum::Router;
use tokio::net::TcpListener;

use crate::http::routes::tasks::create_task;

pub struct Server {
    router: Router,
    listener: TcpListener,
}

impl Server {
    pub async fn build() -> Result<Self, std::io::Error> {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
            .await
            .unwrap();

        Ok(Server {
            router: router(),
            listener,
        })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        axum::serve(self.listener, self.router).await
    }
}

pub fn router() -> Router {
    Router::new().route("/tasks", post(create_task))
}
