use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::state::AppState;
use crate::templates::{
    ConspiracyTemplate, ConspiracyPhenomenonTemplate, ConspiracyPoliticalTemplate,
    ConspiracyWarTemplate, ConspiracyReligiousTemplate, ConspiracyCulturalTemplate,
    ConspiracyTechnologicalTemplate, ConspiracyEconomicTemplate, ConspiracyHealthTemplate,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/phenomenon", get(phenomenon))
        .route("/political", get(political))
        .route("/war", get(war))
        .route("/religious", get(religious))
        .route("/cultural", get(cultural))
        .route("/technological", get(technological))
        .route("/economic", get(economic))
        .route("/health", get(health))
}

async fn hub()          -> impl IntoResponse { HtmlTemplate(ConspiracyTemplate) }
async fn phenomenon()   -> impl IntoResponse { HtmlTemplate(ConspiracyPhenomenonTemplate) }
async fn political()    -> impl IntoResponse { HtmlTemplate(ConspiracyPoliticalTemplate) }
async fn war()          -> impl IntoResponse { HtmlTemplate(ConspiracyWarTemplate) }
async fn religious()    -> impl IntoResponse { HtmlTemplate(ConspiracyReligiousTemplate) }
async fn cultural()     -> impl IntoResponse { HtmlTemplate(ConspiracyCulturalTemplate) }
async fn technological() -> impl IntoResponse { HtmlTemplate(ConspiracyTechnologicalTemplate) }
async fn economic()     -> impl IntoResponse { HtmlTemplate(ConspiracyEconomicTemplate) }
async fn health()       -> impl IntoResponse { HtmlTemplate(ConspiracyHealthTemplate) }
