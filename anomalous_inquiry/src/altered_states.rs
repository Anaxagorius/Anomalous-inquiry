use axum::{routing::get, Router, response::IntoResponse};
use crate::auth::HtmlTemplate;
use crate::state::AppState;
use crate::templates::{
    AlteredStatesTemplate, ObeTemplate, DreamTelepathyTemplate, LucidDreamingTemplate,
    HypnagogicTemplate, TranceTemplate, DissociationTemplate, VisionaryTemplate,
    TraumaTemplate, MeditationTemplate, KundaliniTemplate, BreathworkTemplate,
    PossessionTemplate, AlteredTimeTemplate, EnhancedSensesTemplate, HighAffectTemplate,
    GnosisTemplate, GroupConsciousnessTemplate, PsychosisAdjacentTemplate,
    ContactStatesTemplate, OntologicalTemplate, LiminalTemplate,
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
        .route("/trauma", get(trauma))
        .route("/meditation", get(meditation))
        .route("/kundalini", get(kundalini))
        .route("/breathwork", get(breathwork))
        .route("/possession", get(possession))
        .route("/altered-time", get(altered_time))
        .route("/enhanced-senses", get(enhanced_senses))
        .route("/high-affect", get(high_affect))
        .route("/gnosis", get(gnosis))
        .route("/group-consciousness", get(group_consciousness))
        .route("/psychosis-adjacent", get(psychosis_adjacent))
        .route("/contact-states", get(contact_states))
        .route("/ontological", get(ontological))
        .route("/liminal", get(liminal))
}

async fn hub() -> impl IntoResponse { HtmlTemplate(AlteredStatesTemplate) }
async fn obe() -> impl IntoResponse { HtmlTemplate(ObeTemplate) }
async fn dream_telepathy() -> impl IntoResponse { HtmlTemplate(DreamTelepathyTemplate) }
async fn lucid_dreaming() -> impl IntoResponse { HtmlTemplate(LucidDreamingTemplate) }
async fn hypnagogic() -> impl IntoResponse { HtmlTemplate(HypnagogicTemplate) }
async fn trance() -> impl IntoResponse { HtmlTemplate(TranceTemplate) }
async fn dissociation() -> impl IntoResponse { HtmlTemplate(DissociationTemplate) }
async fn visionary() -> impl IntoResponse { HtmlTemplate(VisionaryTemplate) }
async fn trauma() -> impl IntoResponse { HtmlTemplate(TraumaTemplate) }
async fn meditation() -> impl IntoResponse { HtmlTemplate(MeditationTemplate) }
async fn kundalini() -> impl IntoResponse { HtmlTemplate(KundaliniTemplate) }
async fn breathwork() -> impl IntoResponse { HtmlTemplate(BreathworkTemplate) }
async fn possession() -> impl IntoResponse { HtmlTemplate(PossessionTemplate) }
async fn altered_time() -> impl IntoResponse { HtmlTemplate(AlteredTimeTemplate) }
async fn enhanced_senses() -> impl IntoResponse { HtmlTemplate(EnhancedSensesTemplate) }
async fn high_affect() -> impl IntoResponse { HtmlTemplate(HighAffectTemplate) }
async fn gnosis() -> impl IntoResponse { HtmlTemplate(GnosisTemplate) }
async fn group_consciousness() -> impl IntoResponse { HtmlTemplate(GroupConsciousnessTemplate) }
async fn psychosis_adjacent() -> impl IntoResponse { HtmlTemplate(PsychosisAdjacentTemplate) }
async fn contact_states() -> impl IntoResponse { HtmlTemplate(ContactStatesTemplate) }
async fn ontological() -> impl IntoResponse { HtmlTemplate(OntologicalTemplate) }
async fn liminal() -> impl IntoResponse { HtmlTemplate(LiminalTemplate) }
