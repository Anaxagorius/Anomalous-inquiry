use axum::{routing::get, Router, extract::State, response::IntoResponse, http::{StatusCode, header}};
use rss::{ChannelBuilder, ItemBuilder};
use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(feed))
}

async fn feed(State(state): State<AppState>) -> impl IntoResponse {
    let items: Vec<rss::Item> = state.articles.iter()
        .map(|a| {
            ItemBuilder::default()
                .title(Some(a.title.clone()))
                .link(Some(format!("https://anomalous-inquiry.onrender.com/articles/{}", a.slug)))
                .pub_date(Some(a.published.to_string()))
                .build()
        })
        .collect();
    let channel = ChannelBuilder::default()
        .title("Anomalous Inquiry".to_string())
        .link("https://anomalous-inquiry.onrender.com".to_string())
        .description("Documentary research into anomalous and exceptional experiences.".to_string())
        .items(items)
        .build();
    let xml = channel.to_string();
    (StatusCode::OK, [(header::CONTENT_TYPE, "application/rss+xml; charset=utf-8")], xml).into_response()
}
