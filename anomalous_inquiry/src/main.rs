
mod models;
mod render;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    let state = state::AppState::load();

    let app = routes::router(state);

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .trim()
        .parse()
        .unwrap_or(8080);
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
    println!("Listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
