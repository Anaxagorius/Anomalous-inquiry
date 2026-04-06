
mod models;
mod render;
mod routes;
mod state;

use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let state = state::AppState::load();

    let app = routes::router(state)
        .nest_service("/static", ServeDir::new("static"));

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap();
    println!("Listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
