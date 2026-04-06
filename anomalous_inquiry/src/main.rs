
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

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    println!("Listening on http://0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}
