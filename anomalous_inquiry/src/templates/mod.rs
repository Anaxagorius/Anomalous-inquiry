use askama::Template;
use crate::state::{Organization, TimelineEvent, JournalEntry};

// Auth & admin
#[derive(Template)] #[template(path = "landing.html")] pub struct LandingTemplate;
#[derive(Template)] #[template(path = "app_landing.html")] pub struct AppLandingTemplate { pub is_admin: bool }
#[derive(Template)] #[template(path = "auth_login.html")] pub struct LoginTemplate { pub error: Option<String> }
#[derive(Template)] #[template(path = "auth_signup.html")] pub struct SignupTemplate { pub error: Option<String> }
#[derive(Template)] #[template(path = "admin_login.html")] pub struct AdminLoginTemplate { pub error: Option<String> }
#[derive(Template)] #[template(path = "admin_change_password.html")] pub struct AdminChangePasswordTemplate { pub error: Option<String>, pub forced: bool }
#[derive(Template)] #[template(path = "admin_dashboard.html")] pub struct AdminDashboardTemplate;

// Data pages
#[derive(Template)] #[template(path = "timeline.html")] pub struct TimelineTemplate { pub events: Vec<TimelineEvent> }
#[derive(Template)] #[template(path = "organizations.html")] pub struct OrganizationsTemplate { pub organizations: Vec<Organization> }

// Journal
#[derive(Template)] #[template(path = "journal_admin.html")] pub struct AdminJournalTemplate { pub entries: Vec<JournalEntry>, pub error: Option<String> }
#[derive(Template)] #[template(path = "journal_guest.html")] pub struct GuestJournalTemplate { pub entries: Vec<JournalEntry> }

// Topic hubs
#[derive(Template)] #[template(path = "parapsychology.html")] pub struct ParapsychologyTemplate;
#[derive(Template)] #[template(path = "uap.html")] pub struct UapTemplate;
#[derive(Template)] #[template(path = "survival.html")] pub struct SurvivalTemplate;
#[derive(Template)] #[template(path = "altered_states.html")] pub struct AlteredStatesTemplate;

// Parapsychology sub-pages
#[derive(Template)] #[template(path = "esp.html")] pub struct EspTemplate;
#[derive(Template)] #[template(path = "ganzfeld.html")] pub struct GanzfeldTemplate;
#[derive(Template)] #[template(path = "precognition.html")] pub struct PrecognitionTemplate;
#[derive(Template)] #[template(path = "psychokinesis.html")] pub struct PsychokinesisTemplate;
#[derive(Template)] #[template(path = "pear_lab.html")] pub struct PearLabTemplate;
#[derive(Template)] #[template(path = "remote_viewing.html")] pub struct RemoteViewingTemplate;
#[derive(Template)] #[template(path = "mediumship.html")] pub struct MediumshipTemplate;
#[derive(Template)] #[template(path = "reincarnation.html")] pub struct ReincarnationTemplate;
#[derive(Template)] #[template(path = "terminal_lucidity.html")] pub struct TerminalLucidityTemplate;
#[derive(Template)] #[template(path = "kozyrev_mirror.html")] pub struct KozyrevMirrorTemplate;
#[derive(Template)] #[template(path = "gateway_process.html")] pub struct GatewayProcessTemplate;

// UAP sub-pages
#[derive(Template)] #[template(path = "nimitz.html")] pub struct NimitzTemplate;
#[derive(Template)] #[template(path = "roswell.html")] pub struct RoswellTemplate;
#[derive(Template)] #[template(path = "ce_archive.html")] pub struct CeArchiveTemplate;
#[derive(Template)] #[template(path = "phoenix_lights.html")] pub struct PhoenixLightsTemplate;
#[derive(Template)] #[template(path = "rendlesham.html")] pub struct RendleshamTemplate;
#[derive(Template)] #[template(path = "belgian_wave.html")] pub struct BelgianWaveTemplate;
#[derive(Template)] #[template(path = "military_encounters.html")] pub struct MilitaryEncountersTemplate;
#[derive(Template)] #[template(path = "uap_soviet_chinese.html")] pub struct UapSovietChineseTemplate;

// Survival sub-pages
#[derive(Template)] #[template(path = "nde.html")] pub struct NdeTemplate;
#[derive(Template)] #[template(path = "sde.html")] pub struct SdeTemplate;
#[derive(Template)] #[template(path = "mind_brain.html")] pub struct MindBrainTemplate;

// Altered states sub-pages
#[derive(Template)] #[template(path = "obe.html")] pub struct ObeTemplate;
#[derive(Template)] #[template(path = "dream_telepathy.html")] pub struct DreamTelepathyTemplate;
#[derive(Template)] #[template(path = "lucid_dreaming.html")] pub struct LucidDreamingTemplate;
#[derive(Template)] #[template(path = "hypnagogic.html")] pub struct HypnagogicTemplate;
#[derive(Template)] #[template(path = "trance.html")] pub struct TranceTemplate;
#[derive(Template)] #[template(path = "dissociation.html")] pub struct DissociationTemplate;
#[derive(Template)] #[template(path = "visionary.html")] pub struct VisionaryTemplate;
#[derive(Template)] #[template(path = "trauma.html")] pub struct TraumaTemplate;
#[derive(Template)] #[template(path = "meditation.html")] pub struct MeditationTemplate;
#[derive(Template)] #[template(path = "kundalini.html")] pub struct KundaliniTemplate;
#[derive(Template)] #[template(path = "breathwork.html")] pub struct BreathworkTemplate;
#[derive(Template)] #[template(path = "possession.html")] pub struct PossessionTemplate;
#[derive(Template)] #[template(path = "altered_time.html")] pub struct AlteredTimeTemplate;
#[derive(Template)] #[template(path = "enhanced_senses.html")] pub struct EnhancedSensesTemplate;
#[derive(Template)] #[template(path = "high_affect.html")] pub struct HighAffectTemplate;
#[derive(Template)] #[template(path = "gnosis.html")] pub struct GnosisTemplate;
#[derive(Template)] #[template(path = "group_consciousness.html")] pub struct GroupConsciousnessTemplate;
#[derive(Template)] #[template(path = "psychosis_adjacent.html")] pub struct PsychosisAdjacentTemplate;
#[derive(Template)] #[template(path = "contact_states.html")] pub struct ContactStatesTemplate;
#[derive(Template)] #[template(path = "ontological.html")] pub struct OntologicalTemplate;
#[derive(Template)] #[template(path = "liminal.html")] pub struct LiminalTemplate;

// Cryptozoology hub + category pages
#[derive(Template)] #[template(path = "cryptozoology.html")] pub struct CryptozoologyTemplate;
#[derive(Template)] #[template(path = "crypto_hominid.html")] pub struct CryptoHominidTemplate;
#[derive(Template)] #[template(path = "crypto_canid.html")] pub struct CryptoCanidTemplate;
#[derive(Template)] #[template(path = "crypto_feline.html")] pub struct CryptoFelineTemplate;
#[derive(Template)] #[template(path = "crypto_reptilian.html")] pub struct CryptoReptilianTemplate;
#[derive(Template)] #[template(path = "crypto_aquatic.html")] pub struct CryptoAquaticTemplate;
#[derive(Template)] #[template(path = "crypto_avian.html")] pub struct CryptoAvianTemplate;
#[derive(Template)] #[template(path = "crypto_insectoid.html")] pub struct CryptoInsectoidTemplate;
#[derive(Template)] #[template(path = "crypto_hybrid.html")] pub struct CryptoHybridTemplate;
#[derive(Template)] #[template(path = "crypto_fossil_survivor.html")] pub struct CryptoFossilSurvivorTemplate;
#[derive(Template)] #[template(path = "crypto_regional.html")] pub struct CryptoRegionalTemplate;
#[derive(Template)] #[template(path = "crypto_aerial.html")] pub struct CryptoAerialTemplate;
#[derive(Template)] #[template(path = "crypto_unknown.html")] pub struct CryptoUnknownTemplate;

// NHI hub + category pages
#[derive(Template)] #[template(path = "nhi.html")] pub struct NhiTemplate;
#[derive(Template)] #[template(path = "nhi_et.html")] pub struct NhiEtTemplate;
#[derive(Template)] #[template(path = "nhi_ultra_terrestrial.html")] pub struct NhiUltraTerrestrialTemplate;
#[derive(Template)] #[template(path = "nhi_interdimensional.html")] pub struct NhiInterdimensionalTemplate;
#[derive(Template)] #[template(path = "nhi_plasma.html")] pub struct NhiPlasmaTemplate;
#[derive(Template)] #[template(path = "nhi_orbs.html")] pub struct NhiOrbsTemplate;
#[derive(Template)] #[template(path = "nhi_artificial.html")] pub struct NhiArtificialTemplate;
#[derive(Template)] #[template(path = "nhi_hybrid.html")] pub struct NhiHybridTemplate;
#[derive(Template)] #[template(path = "nhi_ancient.html")] pub struct NhiAncientTemplate;
#[derive(Template)] #[template(path = "nhi_consciousness.html")] pub struct NhiConsciousnessTemplate;
#[derive(Template)] #[template(path = "nhi_trickster.html")] pub struct NhiTricksterTemplate;
#[derive(Template)] #[template(path = "nhi_aquatic.html")] pub struct NhiAquaticTemplate;
#[derive(Template)] #[template(path = "nhi_other.html")] pub struct NhiOtherTemplate;
#[derive(Template)] #[template(path = "nhi_documentation.html")] pub struct NhiDocumentationTemplate;

// Paranormal hub + category pages
#[derive(Template)] #[template(path = "paranormal.html")] pub struct ParanormalTemplate;
#[derive(Template)] #[template(path = "paranormal_ghosts.html")] pub struct ParanormalGhostsTemplate;
#[derive(Template)] #[template(path = "paranormal_vampires.html")] pub struct ParanormalVampiresTemplate;
#[derive(Template)] #[template(path = "paranormal_werewolves.html")] pub struct ParanormalWerewolvesTemplate;
#[derive(Template)] #[template(path = "paranormal_zombies.html")] pub struct ParanormalZombiesTemplate;
#[derive(Template)] #[template(path = "paranormal_demons.html")] pub struct ParanormalDemonsTemplate;
#[derive(Template)] #[template(path = "paranormal_witchcraft.html")] pub struct ParanormalWitchcraftTemplate;
#[derive(Template)] #[template(path = "paranormal_haunted.html")] pub struct ParanormalHauntedTemplate;
#[derive(Template)] #[template(path = "paranormal_exorcism.html")] pub struct ParanormalExorcismTemplate;

// Conspiracy Theory hub + sub-pages
#[derive(Template)] #[template(path = "conspiracy.html")] pub struct ConspiracyTemplate;
#[derive(Template)] #[template(path = "conspiracy_phenomenon.html")] pub struct ConspiracyPhenomenonTemplate;
#[derive(Template)] #[template(path = "conspiracy_political.html")] pub struct ConspiracyPoliticalTemplate;
#[derive(Template)] #[template(path = "conspiracy_war.html")] pub struct ConspiracyWarTemplate;
#[derive(Template)] #[template(path = "conspiracy_religious.html")] pub struct ConspiracyReligiousTemplate;
#[derive(Template)] #[template(path = "conspiracy_cultural.html")] pub struct ConspiracyCulturalTemplate;
#[derive(Template)] #[template(path = "conspiracy_technological.html")] pub struct ConspiracyTechnologicalTemplate;
#[derive(Template)] #[template(path = "conspiracy_economic.html")] pub struct ConspiracyEconomicTemplate;
#[derive(Template)] #[template(path = "conspiracy_health.html")] pub struct ConspiracyHealthTemplate;

// NHI Hybrid sub-pages
#[derive(Template)] #[template(path = "nhi_hybrid_human_grey.html")] pub struct NhiHybridHumanGreyTemplate;
#[derive(Template)] #[template(path = "nhi_hybrid_grey_reptilian.html")] pub struct NhiHybridGreyReptilianTemplate;
#[derive(Template)] #[template(path = "nhi_hybrid_adamic_evadamic.html")] pub struct NhiHybridAdamicEvadamicTemplate;
#[derive(Template)] #[template(path = "nhi_hybrid_els_el.html")] pub struct NhiHybridElsElTemplate;
#[derive(Template)] #[template(path = "nhi_hybrid_zeta_humans.html")] pub struct NhiHybridZetaHumansTemplate;
#[derive(Template)] #[template(path = "nhi_hybrid_hubrids.html")] pub struct NhiHybridHubridsTemplate;
#[derive(Template)] #[template(path = "nhi_hybrid_sassani.html")] pub struct NhiHybridSassaniTemplate;

// NHI Aquatic / Amphibious sub-types
#[derive(Template)] #[template(path = "nhi_aquatic_nommo.html")] pub struct NhiAquaticNommoTemplate;
#[derive(Template)] #[template(path = "nhi_aquatic_amphibians.html")] pub struct NhiAquaticAmphibiansTemplate;
#[derive(Template)] #[template(path = "nhi_aquatic_cetaceans.html")] pub struct NhiAquaticCetaceansTemplate;
#[derive(Template)] #[template(path = "nhi_aquatic_hydra.html")] pub struct NhiAquaticHydraTemplate;

// NHI Insectoid / Arthropod section
#[derive(Template)] #[template(path = "nhi_insectoids.html")] pub struct NhiInsectoidsTemplate;
#[derive(Template)] #[template(path = "nhi_insectoids_mantids.html")] pub struct NhiInsectoidsMantidsTemplate;
#[derive(Template)] #[template(path = "nhi_insectoids_insectoids.html")] pub struct NhiInsectoidsInsectoidsTemplate;
#[derive(Template)] #[template(path = "nhi_insectoids_mantoids.html")] pub struct NhiInsectoidsMantoidsTemplate;
#[derive(Template)] #[template(path = "nhi_insectoids_itipurians.html")] pub struct NhiInsectoidsItipuriansTemplate;
#[derive(Template)] #[template(path = "nhi_insectoids_klermers.html")] pub struct NhiInsectoidsKlermersTemplate;

// NHI Species & Race Index
#[derive(Template)] #[template(path = "nhi_races.html")] pub struct NhiRacesTemplate;

// Reptilian sub-group pages
#[derive(Template)] #[template(path = "nhi_reptilian_general.html")] pub struct NhiReptilianGeneralTemplate;
#[derive(Template)] #[template(path = "nhi_reptilian_draconians.html")] pub struct NhiReptilianDraconianTemplate;
#[derive(Template)] #[template(path = "nhi_reptilian_alpha_draconians.html")] pub struct NhiReptilianAlphaDraconianTemplate;
#[derive(Template)] #[template(path = "nhi_reptilian_lacertians.html")] pub struct NhiReptilianLacertianTemplate;
#[derive(Template)] #[template(path = "nhi_reptilian_iguanoids.html")] pub struct NhiReptilianIguanoidTemplate;
#[derive(Template)] #[template(path = "nhi_reptilian_hydra.html")] pub struct NhiReptilianHydraTemplate;
#[derive(Template)] #[template(path = "nhi_reptilian_serpent_beings.html")] pub struct NhiReptilianSerpentBeingsTemplate;
#[derive(Template)] #[template(path = "nhi_reptilian_nagas.html")] pub struct NhiReptilianNagasTemplate;
#[derive(Template)] #[template(path = "nhi_reptilian_dragonworms.html")] pub struct NhiReptilianDragonwormsTemplate;

#[derive(Template)] #[template(path = "nhi_race_greys.html")] pub struct NhiRaceGreysTemplate;
#[derive(Template)] #[template(path = "nhi_race_tall_whites.html")] pub struct NhiRaceTallWhitesTemplate;
#[derive(Template)] #[template(path = "nhi_race_nordics.html")] pub struct NhiRaceNordicsTemplate;
#[derive(Template)] #[template(path = "nhi_race_pleiadians.html")] pub struct NhiRacePleiadiansTemplate;
#[derive(Template)] #[template(path = "nhi_race_reptilians.html")] pub struct NhiRaceReptiliansTemplate;
#[derive(Template)] #[template(path = "nhi_race_mantids.html")] pub struct NhiRaceMantidsTemplate;
#[derive(Template)] #[template(path = "nhi_race_avians.html")] pub struct NhiRaceAviansTemplate;
#[derive(Template)] #[template(path = "nhi_race_avians_blue.html")] pub struct NhiRaceAviansBlueTemplate;
#[derive(Template)] #[template(path = "nhi_race_avians_humanoid.html")] pub struct NhiRaceAviansHumanoidTemplate;
#[derive(Template)] #[template(path = "nhi_race_avians_garuda.html")] pub struct NhiRaceAviansGarudaTemplate;
#[derive(Template)] #[template(path = "nhi_race_maitre.html")] pub struct NhiRaceMaitreTemplate;
#[derive(Template)] #[template(path = "nhi_race_sirians.html")] pub struct NhiRaceSiriansTemplate;
#[derive(Template)] #[template(path = "nhi_race_arcturians.html")] pub struct NhiRaceArcturiansTemplate;
#[derive(Template)] #[template(path = "nhi_race_andromedans.html")] pub struct NhiRaceAndromedansTemplate;
#[derive(Template)] #[template(path = "nhi_race_lyrans.html")] pub struct NhiRaceLyransTemplate;
#[derive(Template)] #[template(path = "nhi_race_altairians.html")] pub struct NhiRaceAltairiansTemplate;
#[derive(Template)] #[template(path = "nhi_race_procyons.html")] pub struct NhiRaceProcyonsTemplate;
#[derive(Template)] #[template(path = "nhi_race_vegans.html")] pub struct NhiRaceVegansTemplate;
#[derive(Template)] #[template(path = "nhi_race_tau_cetians.html")] pub struct NhiRaceTauCetiansTemplate;
#[derive(Template)] #[template(path = "nhi_race_orion_group.html")] pub struct NhiRaceOrionGroupTemplate;
#[derive(Template)] #[template(path = "nhi_race_alpha_centaurians.html")] pub struct NhiRaceAlphaCentauriansTemplate;
#[derive(Template)] #[template(path = "nhi_race_ebens.html")] pub struct NhiRaceEbensTemplate;
#[derive(Template)] #[template(path = "nhi_race_ummites.html")] pub struct NhiRaceUmmitesTemplate;
#[derive(Template)] #[template(path = "nhi_race_shadow_beings.html")] pub struct NhiRaceShadowBeingsTemplate;
#[derive(Template)] #[template(path = "nhi_race_anunnaki.html")] pub struct NhiRaceAnunnakiTemplate;
#[derive(Template)] #[template(path = "nhi_race_egarot.html")] pub struct NhiRaceEgarotTemplate;
#[derive(Template)] #[template(path = "nhi_race_solipsi_rai.html")] pub struct NhiRaceSolipsiRaiTemplate;
