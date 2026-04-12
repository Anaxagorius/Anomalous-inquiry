use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::state::AppState;
use crate::templates::{SurvivalTemplate, NdeTemplate, SdeTemplate, MindBrainTemplate};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/nde", get(nde))
        .route("/sde", get(sde))
        .route("/mind-brain", get(mind_brain))
}

async fn hub() -> impl IntoResponse { HtmlTemplate(SurvivalTemplate) }
async fn nde() -> impl IntoResponse { HtmlTemplate(NdeTemplate) }
async fn sde() -> impl IntoResponse { HtmlTemplate(SdeTemplate) }
async fn mind_brain() -> impl IntoResponse { HtmlTemplate(MindBrainTemplate) }
