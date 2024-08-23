use dotenv::dotenv;
use tokio::net::TcpListener;

mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = routes::app().await;

    let port = std::env::var("PORT").expect("PORT must be set.");
    let listener = TcpListener::bind("0.0.0.0:".to_owned() + port.as_str())
        .await
        .unwrap();

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    tracing::info!("listening on http://localhost:{}", port);

    axum::serve(listener, app).await.unwrap();
}
