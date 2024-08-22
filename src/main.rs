use tokio::net::TcpListener;

mod routes;

#[tokio::main]
async fn main() {
    let app = routes::app().await;

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let local_addr = listener.local_addr().unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("Listening on {}", local_addr);
}