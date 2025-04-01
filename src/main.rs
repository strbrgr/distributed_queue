use distributed_queue::startup::Server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Server::build().await?;
    app.run_until_stopped().await?;

    Ok(())
}
