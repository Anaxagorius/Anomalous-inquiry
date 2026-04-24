use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::state::AppState;
use crate::templates::{
    ParapsychologyTemplate, EspTemplate, GanzfeldTemplate, PrecognitionTemplate,
    PsychokinesisTemplate, PearLabTemplate, RemoteViewingTemplate, MediumshipTemplate,
    ReincarnationTemplate, TerminalLucidityTemplate, KozyrevMirrorTemplate, GatewayProcessTemplate,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/esp", get(esp))
        .route("/ganzfeld", get(ganzfeld))
        .route("/precognition", get(precognition))
        .route("/psychokinesis", get(psychokinesis))
        .route("/pear-lab", get(pear_lab))
        .route("/remote-viewing", get(remote_viewing))
        .route("/mediumship", get(mediumship))
        .route("/reincarnation", get(reincarnation))
        .route("/terminal-lucidity", get(terminal_lucidity))
        .route("/kozyrev-mirror", get(kozyrev_mirror))
        .route("/gateway-process", get(gateway_process))
}

async fn hub() -> impl IntoResponse { HtmlTemplate(ParapsychologyTemplate) }
async fn esp() -> impl IntoResponse { HtmlTemplate(EspTemplate) }
async fn ganzfeld() -> impl IntoResponse { HtmlTemplate(GanzfeldTemplate) }
async fn precognition() -> impl IntoResponse { HtmlTemplate(PrecognitionTemplate) }
async fn psychokinesis() -> impl IntoResponse { HtmlTemplate(PsychokinesisTemplate) }
async fn pear_lab() -> impl IntoResponse { HtmlTemplate(PearLabTemplate) }
async fn remote_viewing() -> impl IntoResponse { HtmlTemplate(RemoteViewingTemplate) }
async fn mediumship() -> impl IntoResponse { HtmlTemplate(MediumshipTemplate) }
async fn reincarnation() -> impl IntoResponse { HtmlTemplate(ReincarnationTemplate) }
async fn terminal_lucidity() -> impl IntoResponse { HtmlTemplate(TerminalLucidityTemplate) }
async fn kozyrev_mirror() -> impl IntoResponse { HtmlTemplate(KozyrevMirrorTemplate) }
async fn gateway_process() -> impl IntoResponse { HtmlTemplate(GatewayProcessTemplate) }
