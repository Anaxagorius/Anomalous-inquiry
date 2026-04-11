use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::state::AppState;
use crate::templates::{
    ParanormalTemplate, ParanormalGhostsTemplate, ParanormalVampiresTemplate,
    ParanormalWerewolvesTemplate, ParanormalZombiesTemplate, ParanormalDemonsTemplate,
    ParanormalWitchcraftTemplate, ParanormalHauntedTemplate, ParanormalExorcismTemplate,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(hub))
        .route("/ghosts", get(ghosts))
        .route("/vampires", get(vampires))
        .route("/werewolves", get(werewolves))
        .route("/zombies", get(zombies))
        .route("/demons", get(demons))
        .route("/witchcraft", get(witchcraft))
        .route("/haunted", get(haunted))
        .route("/exorcism", get(exorcism))
}

async fn hub()        -> impl IntoResponse { HtmlTemplate(ParanormalTemplate) }
async fn ghosts()     -> impl IntoResponse { HtmlTemplate(ParanormalGhostsTemplate) }
async fn vampires()   -> impl IntoResponse { HtmlTemplate(ParanormalVampiresTemplate) }
async fn werewolves() -> impl IntoResponse { HtmlTemplate(ParanormalWerewolvesTemplate) }
async fn zombies()    -> impl IntoResponse { HtmlTemplate(ParanormalZombiesTemplate) }
async fn demons()     -> impl IntoResponse { HtmlTemplate(ParanormalDemonsTemplate) }
async fn witchcraft() -> impl IntoResponse { HtmlTemplate(ParanormalWitchcraftTemplate) }
async fn haunted()    -> impl IntoResponse { HtmlTemplate(ParanormalHauntedTemplate) }
async fn exorcism()   -> impl IntoResponse { HtmlTemplate(ParanormalExorcismTemplate) }
