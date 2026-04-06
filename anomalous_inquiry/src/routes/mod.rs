
use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/methodology", get(methodology))
        .route("/timeline", get(timeline))
        .route("/rss.xml", get(rss))
}

async fn home() -> &'static str {
    "Anomalous Inquiry — Exploring phenomena at the limits of science and understanding"
}

async fn methodology() -> &'static str {
    "Methodology: Neutral documentation of anomalous reports without validation claims."
}

async fn timeline() -> &'static str {
    "Timeline view placeholder (UAP / CE1–CE5)"
}

async fn rss() -> &'static str {
    "RSS feed placeholder"
}
