use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::state::AppState;
use crate::templates::{
    NhiTemplate, NhiEtTemplate, NhiUltraTerrestrialTemplate, NhiInterdimensionalTemplate,
    NhiPlasmaTemplate, NhiOrbsTemplate, NhiArtificialTemplate, NhiHybridTemplate,
    NhiAncientTemplate, NhiConsciousnessTemplate, NhiTricksterTemplate,
    NhiAquaticTemplate, NhiOtherTemplate, NhiDocumentationTemplate,
    NhiRacesTemplate,
    NhiRaceGreysTemplate, NhiRaceTallWhitesTemplate, NhiRaceNordicsTemplate,
    NhiRacePleiadiansTemplate,
    NhiRaceReptiliansTemplate, NhiRaceMantidsTemplate, NhiRaceAviansTemplate,
    NhiRaceAviansBlueTemplate, NhiRaceAviansHumanoidTemplate, NhiRaceAviansGarudaTemplate,
    NhiRaceMaitreTemplate, NhiRaceSiriansTemplate, NhiRaceArcturiansTemplate,
    NhiRaceAndromedansTemplate, NhiRaceLyransTemplate,
    NhiRaceAltairiansTemplate, NhiRaceProcyonsTemplate, NhiRaceVegansTemplate,
    NhiRaceTauCetiansTemplate,
    NhiRaceOrionGroupTemplate, NhiRaceAlphaCentauriansTemplate,
    NhiRaceEbensTemplate, NhiRaceUmmitesTemplate, NhiRaceShadowBeingsTemplate,
    NhiRaceAnunnakiTemplate, NhiRaceEgarotTemplate, NhiRaceSolipsiRaiTemplate,
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
        .route("/races", get(races))
        .route("/races/greys", get(race_greys))
        .route("/races/tall-whites", get(race_tall_whites))
        .route("/races/nordics", get(race_nordics))
        .route("/races/pleiadians", get(race_pleiadians))
        .route("/races/reptilians", get(race_reptilians))
        .route("/races/mantids", get(race_mantids))
        .route("/races/avians", get(race_avians))
        .route("/races/avians/blue-avians", get(race_avians_blue))
        .route("/races/avians/avian-humanoids", get(race_avians_humanoid))
        .route("/races/avians/garuda", get(race_avians_garuda))
        .route("/races/maitre", get(race_maitre))
        .route("/races/sirians", get(race_sirians))
        .route("/races/arcturians", get(race_arcturians))
        .route("/races/andromedans", get(race_andromedans))
        .route("/races/lyrans", get(race_lyrans))
        .route("/races/altairians", get(race_altairians))
        .route("/races/procyons", get(race_procyons))
        .route("/races/vegans", get(race_vegans))
        .route("/races/tau-cetians", get(race_tau_cetians))
        .route("/races/orion-group", get(race_orion_group))
        .route("/races/alpha-centaurians", get(race_alpha_centaurians))
        .route("/races/ebens", get(race_ebens))
        .route("/races/ummites", get(race_ummites))
        .route("/races/shadow-beings", get(race_shadow_beings))
        .route("/races/anunnaki", get(race_anunnaki))
        .route("/races/egarot", get(race_egarot))
        .route("/races/solipsi-rai", get(race_solipsi_rai))
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
async fn races()            -> impl IntoResponse { HtmlTemplate(NhiRacesTemplate) }
async fn race_greys()       -> impl IntoResponse { HtmlTemplate(NhiRaceGreysTemplate) }
async fn race_tall_whites() -> impl IntoResponse { HtmlTemplate(NhiRaceTallWhitesTemplate) }
async fn race_nordics()     -> impl IntoResponse { HtmlTemplate(NhiRaceNordicsTemplate) }
async fn race_pleiadians()  -> impl IntoResponse { HtmlTemplate(NhiRacePleiadiansTemplate) }
async fn race_reptilians()  -> impl IntoResponse { HtmlTemplate(NhiRaceReptiliansTemplate) }
async fn race_mantids()     -> impl IntoResponse { HtmlTemplate(NhiRaceMantidsTemplate) }
async fn race_avians()      -> impl IntoResponse { HtmlTemplate(NhiRaceAviansTemplate) }
async fn race_avians_blue() -> impl IntoResponse { HtmlTemplate(NhiRaceAviansBlueTemplate) }
async fn race_avians_humanoid() -> impl IntoResponse { HtmlTemplate(NhiRaceAviansHumanoidTemplate) }
async fn race_avians_garuda() -> impl IntoResponse { HtmlTemplate(NhiRaceAviansGarudaTemplate) }
async fn race_maitre()      -> impl IntoResponse { HtmlTemplate(NhiRaceMaitreTemplate) }
async fn race_sirians()     -> impl IntoResponse { HtmlTemplate(NhiRaceSiriansTemplate) }
async fn race_arcturians()  -> impl IntoResponse { HtmlTemplate(NhiRaceArcturiansTemplate) }
async fn race_andromedans() -> impl IntoResponse { HtmlTemplate(NhiRaceAndromedansTemplate) }
async fn race_lyrans()      -> impl IntoResponse { HtmlTemplate(NhiRaceLyransTemplate) }
async fn race_altairians()  -> impl IntoResponse { HtmlTemplate(NhiRaceAltairiansTemplate) }
async fn race_procyons()    -> impl IntoResponse { HtmlTemplate(NhiRaceProcyonsTemplate) }
async fn race_vegans()      -> impl IntoResponse { HtmlTemplate(NhiRaceVegansTemplate) }
async fn race_tau_cetians() -> impl IntoResponse { HtmlTemplate(NhiRaceTauCetiansTemplate) }
async fn race_orion_group() -> impl IntoResponse { HtmlTemplate(NhiRaceOrionGroupTemplate) }
async fn race_alpha_centaurians() -> impl IntoResponse { HtmlTemplate(NhiRaceAlphaCentauriansTemplate) }
async fn race_ebens()       -> impl IntoResponse { HtmlTemplate(NhiRaceEbensTemplate) }
async fn race_ummites()     -> impl IntoResponse { HtmlTemplate(NhiRaceUmmitesTemplate) }
async fn race_shadow_beings() -> impl IntoResponse { HtmlTemplate(NhiRaceShadowBeingsTemplate) }
async fn race_anunnaki()    -> impl IntoResponse { HtmlTemplate(NhiRaceAnunnakiTemplate) }
async fn race_egarot()      -> impl IntoResponse { HtmlTemplate(NhiRaceEgarotTemplate) }
async fn race_solipsi_rai() -> impl IntoResponse { HtmlTemplate(NhiRaceSolipsiRaiTemplate) }
