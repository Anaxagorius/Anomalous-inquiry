use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::state::AppState;
use crate::templates::{
    CryptozoologyTemplate, CryptoHominidTemplate, CryptoCanidTemplate, CryptoFelineTemplate,
    CryptoReptilianTemplate, CryptoAquaticTemplate, CryptoAvianTemplate, CryptoInsectoidTemplate,
    CryptoHybridTemplate, CryptoFossilSurvivorTemplate, CryptoRegionalTemplate,
    CryptoAerialTemplate, CryptoUnknownTemplate,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/hominid", get(hominid))
        .route("/canid", get(canid))
        .route("/feline", get(feline))
        .route("/reptilian", get(reptilian))
        .route("/aquatic", get(aquatic))
        .route("/avian", get(avian))
        .route("/insectoid", get(insectoid))
        .route("/hybrid", get(hybrid))
        .route("/fossil-survivor", get(fossil_survivor))
        .route("/regional", get(regional))
        .route("/aerial", get(aerial))
        .route("/unknown", get(unknown))
}

async fn hub()            -> impl IntoResponse { HtmlTemplate(CryptozoologyTemplate) }
async fn hominid()        -> impl IntoResponse { HtmlTemplate(CryptoHominidTemplate) }
async fn canid()          -> impl IntoResponse { HtmlTemplate(CryptoCanidTemplate) }
async fn feline()         -> impl IntoResponse { HtmlTemplate(CryptoFelineTemplate) }
async fn reptilian()      -> impl IntoResponse { HtmlTemplate(CryptoReptilianTemplate) }
async fn aquatic()        -> impl IntoResponse { HtmlTemplate(CryptoAquaticTemplate) }
async fn avian()          -> impl IntoResponse { HtmlTemplate(CryptoAvianTemplate) }
async fn insectoid()      -> impl IntoResponse { HtmlTemplate(CryptoInsectoidTemplate) }
async fn hybrid()         -> impl IntoResponse { HtmlTemplate(CryptoHybridTemplate) }
async fn fossil_survivor()-> impl IntoResponse { HtmlTemplate(CryptoFossilSurvivorTemplate) }
async fn regional()       -> impl IntoResponse { HtmlTemplate(CryptoRegionalTemplate) }
async fn aerial()         -> impl IntoResponse { HtmlTemplate(CryptoAerialTemplate) }
async fn unknown()        -> impl IntoResponse { HtmlTemplate(CryptoUnknownTemplate) }
