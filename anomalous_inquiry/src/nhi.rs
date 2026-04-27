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
    NhiInsectoidsTemplate, NhiInsectoidsMantidsTemplate, NhiInsectoidsInsectoidsTemplate,
    NhiInsectoidsMantoidsTemplate, NhiInsectoidsItipuriansTemplate, NhiInsectoidsKlermersTemplate,
    NhiReptilianGeneralTemplate, NhiReptilianDraconianTemplate,
    NhiReptilianAlphaDraconianTemplate, NhiReptilianLacertianTemplate,
    NhiReptilianIguanoidTemplate, NhiReptilianHydraTemplate,
    NhiReptilianSerpentBeingsTemplate, NhiReptilianNagasTemplate,
    NhiReptilianDragonwormsTemplate,
    NhiHybridHumanGreyTemplate, NhiHybridGreyReptilianTemplate, NhiHybridAdamicEvadamicTemplate,
    NhiHybridElsElTemplate, NhiHybridZetaHumansTemplate, NhiHybridHubridsTemplate,
    NhiHybridSassaniTemplate,
    NhiEnergyTemplate, NhiEnergyEnergyzoa, NhiEnergyLightBeings, NhiEnergyPlasma,
    NhiEnergyOrbs, NhiEnergyShadowBeings, NhiEnergyAstral, NhiEnergyInterdimensional,
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
        .route("/hybrid/human-grey", get(hybrid_human_grey))
        .route("/hybrid/grey-reptilian", get(hybrid_grey_reptilian))
        .route("/hybrid/adamic-evadamic", get(hybrid_adamic_evadamic))
        .route("/hybrid/els-el", get(hybrid_els_el))
        .route("/hybrid/zeta-humans", get(hybrid_zeta_humans))
        .route("/hybrid/hu-brids", get(hybrid_hubrids))
        .route("/hybrid/sassani", get(hybrid_sassani))
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
        .route("/races/reptilians/general", get(reptilian_general))
        .route("/races/reptilians/draconians", get(reptilian_draconians))
        .route("/races/reptilians/alpha-draconians", get(reptilian_alpha_draconians))
        .route("/races/reptilians/lacertians", get(reptilian_lacertians))
        .route("/races/reptilians/iguanoids", get(reptilian_iguanoids))
        .route("/races/reptilians/hydra-reptilians", get(reptilian_hydra))
        .route("/races/reptilians/serpent-beings", get(reptilian_serpent_beings))
        .route("/races/reptilians/nagas", get(reptilian_nagas))
        .route("/races/reptilians/dragonworms", get(reptilian_dragonworms))
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
        .route("/insectoids", get(insectoids))
        .route("/insectoids/mantids", get(insectoids_mantids))
        .route("/insectoids/insectoids", get(insectoids_insectoids))
        .route("/insectoids/mantoids", get(insectoids_mantoids))
        .route("/insectoids/itipurians", get(insectoids_itipurians))
        .route("/insectoids/klermers", get(insectoids_klermers))
        .route("/energy", get(energy))
        .route("/energy/energyzoa", get(energy_energyzoa))
        .route("/energy/light-beings", get(energy_light_beings))
        .route("/energy/plasma-entities", get(energy_plasma))
        .route("/energy/orbs", get(energy_orbs))
        .route("/energy/shadow-beings", get(energy_shadow_beings))
        .route("/energy/astral-entities", get(energy_astral))
        .route("/energy/interdimensional-intelligences", get(energy_interdimensional))
}

async fn hub()              -> impl IntoResponse { HtmlTemplate(NhiTemplate) }
async fn et()               -> impl IntoResponse { HtmlTemplate(NhiEtTemplate) }
async fn ultra_terrestrial()-> impl IntoResponse { HtmlTemplate(NhiUltraTerrestrialTemplate) }
async fn interdimensional() -> impl IntoResponse { HtmlTemplate(NhiInterdimensionalTemplate) }
async fn plasma()           -> impl IntoResponse { HtmlTemplate(NhiPlasmaTemplate) }
async fn orbs()             -> impl IntoResponse { HtmlTemplate(NhiOrbsTemplate) }
async fn artificial()       -> impl IntoResponse { HtmlTemplate(NhiArtificialTemplate) }
async fn hybrid()           -> impl IntoResponse { HtmlTemplate(NhiHybridTemplate) }
async fn hybrid_human_grey()        -> impl IntoResponse { HtmlTemplate(NhiHybridHumanGreyTemplate) }
async fn hybrid_grey_reptilian()    -> impl IntoResponse { HtmlTemplate(NhiHybridGreyReptilianTemplate) }
async fn hybrid_adamic_evadamic()   -> impl IntoResponse { HtmlTemplate(NhiHybridAdamicEvadamicTemplate) }
async fn hybrid_els_el()            -> impl IntoResponse { HtmlTemplate(NhiHybridElsElTemplate) }
async fn hybrid_zeta_humans()       -> impl IntoResponse { HtmlTemplate(NhiHybridZetaHumansTemplate) }
async fn hybrid_hubrids()           -> impl IntoResponse { HtmlTemplate(NhiHybridHubridsTemplate) }
async fn hybrid_sassani()           -> impl IntoResponse { HtmlTemplate(NhiHybridSassaniTemplate) }
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
async fn insectoids()             -> impl IntoResponse { HtmlTemplate(NhiInsectoidsTemplate) }
async fn insectoids_mantids()     -> impl IntoResponse { HtmlTemplate(NhiInsectoidsMantidsTemplate) }
async fn insectoids_insectoids()  -> impl IntoResponse { HtmlTemplate(NhiInsectoidsInsectoidsTemplate) }
async fn insectoids_mantoids()    -> impl IntoResponse { HtmlTemplate(NhiInsectoidsMantoidsTemplate) }
async fn insectoids_itipurians()  -> impl IntoResponse { HtmlTemplate(NhiInsectoidsItipuriansTemplate) }
async fn insectoids_klermers()    -> impl IntoResponse { HtmlTemplate(NhiInsectoidsKlermersTemplate) }

async fn energy()                         -> impl IntoResponse { HtmlTemplate(NhiEnergyTemplate) }
async fn energy_energyzoa()               -> impl IntoResponse { HtmlTemplate(NhiEnergyEnergyzoa) }
async fn energy_light_beings()            -> impl IntoResponse { HtmlTemplate(NhiEnergyLightBeings) }
async fn energy_plasma()                  -> impl IntoResponse { HtmlTemplate(NhiEnergyPlasma) }
async fn energy_orbs()                    -> impl IntoResponse { HtmlTemplate(NhiEnergyOrbs) }
async fn energy_shadow_beings()           -> impl IntoResponse { HtmlTemplate(NhiEnergyShadowBeings) }
async fn energy_astral()                  -> impl IntoResponse { HtmlTemplate(NhiEnergyAstral) }
async fn energy_interdimensional()        -> impl IntoResponse { HtmlTemplate(NhiEnergyInterdimensional) }

async fn reptilian_general()          -> impl IntoResponse { HtmlTemplate(NhiReptilianGeneralTemplate) }
async fn reptilian_draconians()       -> impl IntoResponse { HtmlTemplate(NhiReptilianDraconianTemplate) }
async fn reptilian_alpha_draconians() -> impl IntoResponse { HtmlTemplate(NhiReptilianAlphaDraconianTemplate) }
async fn reptilian_lacertians()       -> impl IntoResponse { HtmlTemplate(NhiReptilianLacertianTemplate) }
async fn reptilian_iguanoids()        -> impl IntoResponse { HtmlTemplate(NhiReptilianIguanoidTemplate) }
async fn reptilian_hydra()            -> impl IntoResponse { HtmlTemplate(NhiReptilianHydraTemplate) }
async fn reptilian_serpent_beings()   -> impl IntoResponse { HtmlTemplate(NhiReptilianSerpentBeingsTemplate) }
async fn reptilian_nagas()            -> impl IntoResponse { HtmlTemplate(NhiReptilianNagasTemplate) }
async fn reptilian_dragonworms()      -> impl IntoResponse { HtmlTemplate(NhiReptilianDragonwormsTemplate) }
