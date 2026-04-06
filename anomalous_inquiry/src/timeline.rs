use axum::{routing::get, Router, extract::State, response::IntoResponse};
use crate::state::AppState;
use crate::auth::HtmlTemplate;
use crate::templates::TimelineTemplate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(timeline_page))
}

async fn timeline_page(State(state): State<AppState>) -> impl IntoResponse {
    let events = state.timeline.as_ref().clone();
    HtmlTemplate(TimelineTemplate { events })
}
