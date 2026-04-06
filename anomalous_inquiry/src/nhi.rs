use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::state::AppState;
use crate::templates::{
    NhiTemplate, NhiEtTemplate, NhiUltraTerrestrialTemplate, NhiInterdimensionalTemplate,
    NhiPlasmaTemplate, NhiOrbsTemplate, NhiArtificialTemplate, NhiHybridTemplate,
    NhiAncientTemplate, NhiConsciousnessTemplate, NhiTricksterTemplate,
    NhiAquaticTemplate, NhiOtherTemplate, NhiDocumentationTemplate,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/extraterrestrial", get(et))
        .route("/ultra-terrestrial", get(ultra_terrestrial))
        .route("/interdimensional", get(interdimensional))
        .route("/plasma", get(plasma))
        .route("/orbs", get(orbs))
        .route("/artificial", get(artificial))
        .route("/hybrid", get(hybrid))
        .route("/ancient", get(ancient))
        .route("/consciousness", get(consciousness))
        .route("/trickster", get(trickster))
        .route("/aquatic", get(aquatic))
        .route("/other", get(other))
        .route("/documentation", get(documentation))
}

async fn hub()              -> impl IntoResponse { HtmlTemplate(NhiTemplate) }
async fn et()               -> impl IntoResponse { HtmlTemplate(NhiEtTemplate) }
async fn ultra_terrestrial()-> impl IntoResponse { HtmlTemplate(NhiUltraTerrestrialTemplate) }
async fn interdimensional() -> impl IntoResponse { HtmlTemplate(NhiInterdimensionalTemplate) }
async fn plasma()           -> impl IntoResponse { HtmlTemplate(NhiPlasmaTemplate) }
async fn orbs()             -> impl IntoResponse { HtmlTemplate(NhiOrbsTemplate) }
async fn artificial()       -> impl IntoResponse { HtmlTemplate(NhiArtificialTemplate) }
async fn hybrid()           -> impl IntoResponse { HtmlTemplate(NhiHybridTemplate) }
async fn ancient()          -> impl IntoResponse { HtmlTemplate(NhiAncientTemplate) }
async fn consciousness()    -> impl IntoResponse { HtmlTemplate(NhiConsciousnessTemplate) }
async fn trickster()        -> impl IntoResponse { HtmlTemplate(NhiTricksterTemplate) }
async fn aquatic()          -> impl IntoResponse { HtmlTemplate(NhiAquaticTemplate) }
async fn other()            -> impl IntoResponse { HtmlTemplate(NhiOtherTemplate) }
async fn documentation()    -> impl IntoResponse { HtmlTemplate(NhiDocumentationTemplate) }
