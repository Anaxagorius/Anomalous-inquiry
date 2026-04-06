use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::state::AppState;
use crate::templates::{
    AlteredStatesTemplate, ObeTemplate, DreamTelepathyTemplate, LucidDreamingTemplate,
    HypnagogicTemplate, TranceTemplate, DissociationTemplate, VisionaryTemplate,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/obe", get(obe))
        .route("/dream-telepathy", get(dream_telepathy))
        .route("/lucid-dreaming", get(lucid_dreaming))
        .route("/hypnagogic", get(hypnagogic))
        .route("/trance", get(trance))
        .route("/dissociation", get(dissociation))
        .route("/visionary", get(visionary))
}

async fn hub() -> impl IntoResponse { HtmlTemplate(AlteredStatesTemplate) }
async fn obe() -> impl IntoResponse { HtmlTemplate(ObeTemplate) }
async fn dream_telepathy() -> impl IntoResponse { HtmlTemplate(DreamTelepathyTemplate) }
async fn lucid_dreaming() -> impl IntoResponse { HtmlTemplate(LucidDreamingTemplate) }
async fn hypnagogic() -> impl IntoResponse { HtmlTemplate(HypnagogicTemplate) }
async fn trance() -> impl IntoResponse { HtmlTemplate(TranceTemplate) }
async fn dissociation() -> impl IntoResponse { HtmlTemplate(DissociationTemplate) }
async fn visionary() -> impl IntoResponse { HtmlTemplate(VisionaryTemplate) }
