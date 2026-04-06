use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::state::AppState;
use crate::templates::SurvivalTemplate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(hub))
}

async fn hub() -> impl IntoResponse { HtmlTemplate(SurvivalTemplate) }
