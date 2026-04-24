use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::state::AppState;
use crate::templates::{
    UapTemplate, NimitzTemplate, RoswellTemplate, CeArchiveTemplate,
    PhoenixLightsTemplate, RendleshamTemplate, BelgianWaveTemplate,
    MilitaryEncountersTemplate,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/nimitz", get(nimitz))
        .route("/roswell", get(roswell))
        .route("/ce-archive", get(ce_archive))
        .route("/phoenix-lights", get(phoenix_lights))
        .route("/rendlesham", get(rendlesham))
        .route("/belgian-wave", get(belgian_wave))
        .route("/military-encounters", get(military_encounters))
}

async fn hub() -> impl IntoResponse { HtmlTemplate(UapTemplate) }
async fn nimitz() -> impl IntoResponse { HtmlTemplate(NimitzTemplate) }
async fn roswell() -> impl IntoResponse { HtmlTemplate(RoswellTemplate) }
async fn ce_archive() -> impl IntoResponse { HtmlTemplate(CeArchiveTemplate) }
async fn phoenix_lights() -> impl IntoResponse { HtmlTemplate(PhoenixLightsTemplate) }
async fn rendlesham() -> impl IntoResponse { HtmlTemplate(RendleshamTemplate) }
async fn belgian_wave() -> impl IntoResponse { HtmlTemplate(BelgianWaveTemplate) }
async fn military_encounters() -> impl IntoResponse { HtmlTemplate(MilitaryEncountersTemplate) }
