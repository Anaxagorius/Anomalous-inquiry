
use axum::{
    extract::State,
    http::header,
    response::{IntoResponse, Response},
};
use chrono::NaiveDateTime;
use rss::{ChannelBuilder, ItemBuilder};

use crate::state::AppState;

pub async fn feed(State(state): State<AppState>) -> Response {
    let base_url = std::env::var("BASE_URL")
        .unwrap_or_else(|_| "https://anomalousinquiry.org".to_string());
    let items: Vec<_> = state
        .articles
        .iter()
        .map(|a| {
            let dt = NaiveDateTime::new(
                a.published,
                chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            );
            let pub_date = dt.format("%a, %d %b %Y 00:00:00 +0000").to_string();
            ItemBuilder::default()
                .title(a.title.clone())
                .link(format!("{}/articles/{}", base_url, a.slug))
                .description(format!("Tags: {}", a.tags.join(", ")))
                .pub_date(pub_date)
                .build()
        })
        .collect();

    let channel = ChannelBuilder::default()
        .title("Anomalous Inquiry")
        .link(base_url)
        .description("Neutral, documentary-style research into anomalous phenomena.")
        .items(items)
        .build();

    (
        [(header::CONTENT_TYPE, "application/rss+xml; charset=utf-8")],
        channel.to_string(),
    )
        .into_response()
}
