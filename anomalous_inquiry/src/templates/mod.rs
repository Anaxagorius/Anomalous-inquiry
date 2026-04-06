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
#[derive(Template)] #[template(path = "psychokinesis.html")] pub struct PsychokinesiTemplate;
#[derive(Template)] #[template(path = "pear_lab.html")] pub struct PearLabTemplate;
#[derive(Template)] #[template(path = "remote_viewing.html")] pub struct RemoteViewingTemplate;
#[derive(Template)] #[template(path = "mediumship.html")] pub struct MediumshipTemplate;
#[derive(Template)] #[template(path = "reincarnation.html")] pub struct ReincarnationTemplate;
#[derive(Template)] #[template(path = "terminal_lucidity.html")] pub struct TerminalLucidityTemplate;

// UAP sub-pages
#[derive(Template)] #[template(path = "nimitz.html")] pub struct NimitzTemplate;
#[derive(Template)] #[template(path = "roswell.html")] pub struct RoswellTemplate;
#[derive(Template)] #[template(path = "ce_archive.html")] pub struct CeArchiveTemplate;
#[derive(Template)] #[template(path = "phoenix_lights.html")] pub struct PhoenixLightsTemplate;
#[derive(Template)] #[template(path = "rendlesham.html")] pub struct RendleshamTemplate;
#[derive(Template)] #[template(path = "belgian_wave.html")] pub struct BelgianWaveTemplate;

// Survival sub-pages
#[derive(Template)] #[template(path = "nde.html")] pub struct NdeTemplate;
#[derive(Template)] #[template(path = "mind_brain.html")] pub struct MindBrainTemplate;

// Altered states sub-pages
#[derive(Template)] #[template(path = "obe.html")] pub struct ObeTemplate;
#[derive(Template)] #[template(path = "dream_telepathy.html")] pub struct DreamTelepathyTemplate;
#[derive(Template)] #[template(path = "lucid_dreaming.html")] pub struct LucidDreamingTemplate;
#[derive(Template)] #[template(path = "hypnagogic.html")] pub struct HypnagogicTemplate;
#[derive(Template)] #[template(path = "trance.html")] pub struct TranceTemplate;
#[derive(Template)] #[template(path = "dissociation.html")] pub struct DissociationTemplate;
#[derive(Template)] #[template(path = "visionary.html")] pub struct VisionaryTemplate;
