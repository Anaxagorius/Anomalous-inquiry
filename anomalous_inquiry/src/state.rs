use std::{
    fs,
    path::Path,
    sync::Arc,
};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use tracing::info;

pub const ADMIN_USERNAME: &str = "admin";
const ADMIN_INITIAL_PASSWORD: &str = "anomalous2024!";

pub fn journal_data_path() -> String {
    std::env::var("JOURNAL_DATA_FILE").unwrap_or_else(|_| "data/journal_entries.json".to_string())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Organization {
    pub name: String,
    pub category: String,
    pub description: String,
    pub website: Option<String>,
    pub founded: Option<String>,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TimelineEvent {
    pub date: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub sources: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub visible_to_guests: bool,
    pub created_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

impl User {
    pub fn new(email: String, password_hash: String) -> Self {
        User { id: Uuid::new_v4(), email, password_hash }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub organizations: Arc<Vec<Organization>>,
    pub timeline: Arc<Vec<TimelineEvent>>,
    pub journal_entries: Arc<RwLock<Vec<JournalEntry>>>,
    pub users: Arc<RwLock<Vec<User>>>,
    pub jwt_secret: Arc<String>,
    pub admin_password_hash: Arc<RwLock<String>>,
    pub admin_must_change_password: Arc<RwLock<bool>>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let organizations = load_organizations();
        let timeline = load_timeline();
        let journal_entries = load_journal_entries().await;

        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::default();
        let admin_hash = argon2
            .hash_password(ADMIN_INITIAL_PASSWORD.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash admin password: {e}"))?
            .to_string();

        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
            tracing::warn!("JWT_SECRET env var not set — using insecure default.");
            "change_me_super_secret".to_string()
        });

        info!("Loaded {} organizations, {} timeline events", organizations.len(), timeline.len());

        Ok(AppState {
            organizations: Arc::new(organizations),
            timeline: Arc::new(timeline),
            journal_entries: Arc::new(RwLock::new(journal_entries)),
            users: Arc::new(RwLock::new(Vec::new())),
            jwt_secret: Arc::new(jwt_secret),
            admin_password_hash: Arc::new(RwLock::new(admin_hash)),
            admin_must_change_password: Arc::new(RwLock::new(true)),
        })
    }
}

fn load_organizations() -> Vec<Organization> {
    let path = Path::new("content/organizations.json");
    if !path.exists() { return Vec::new(); }
    fs::read_to_string(path).ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn load_timeline() -> Vec<TimelineEvent> {
    let path = Path::new("content/timeline.json");
    if !path.exists() { return Vec::new(); }
    fs::read_to_string(path).ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

async fn load_journal_entries() -> Vec<JournalEntry> {
    let path = journal_data_path();
    match tokio::fs::read_to_string(&path).await {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

pub async fn save_journal(entries: &[JournalEntry]) {
    let path = journal_data_path();
    if let Some(parent) = std::path::Path::new(&path).parent() {
        let _ = tokio::fs::create_dir_all(parent).await;
    }
    if let Ok(json) = serde_json::to_string(entries) {
        let _ = tokio::fs::write(&path, json).await;
    }
}
