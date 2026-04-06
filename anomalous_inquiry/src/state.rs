use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDate;
use pulldown_cmark::{html, Options, Parser};
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use tracing::info;

pub const ADMIN_USERNAME: &str = "admin";
const ADMIN_INITIAL_PASSWORD: &str = "anomalous2024!";

pub fn journal_data_path() -> String {
    std::env::var("JOURNAL_DATA_FILE").unwrap_or_else(|_| "data/journal_entries.json".to_string())
}

pub fn comments_data_path() -> String {
    std::env::var("DATA_DIR")
        .map(|d| format!("{d}/comments.json"))
        .unwrap_or_else(|_| "data/comments.json".to_string())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub body_html: String,
    pub tags: Vec<String>,
    pub published: NaiveDate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub article_slug: String,
    pub content: String,
    pub approved: bool,
    pub submitted_at: String,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub article_slug: Option<String>,
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
    pub articles: Arc<Vec<Article>>,
    pub organizations: Arc<Vec<Organization>>,
    pub timeline: Arc<Vec<TimelineEvent>>,
    pub comments: Arc<RwLock<Vec<Comment>>>,
    pub journal_entries: Arc<RwLock<Vec<JournalEntry>>>,
    pub users: Arc<RwLock<Vec<User>>>,
    pub jwt_secret: Arc<String>,
    pub admin_password_hash: Arc<RwLock<String>>,
    pub admin_must_change_password: Arc<RwLock<bool>>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let articles = load_articles();
        let organizations = load_organizations();
        let timeline = load_timeline();
        let comments = load_comments();
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

        info!("Loaded {} articles, {} organizations, {} timeline events", articles.len(), organizations.len(), timeline.len());

        Ok(AppState {
            articles: Arc::new(articles),
            organizations: Arc::new(organizations),
            timeline: Arc::new(timeline),
            comments: Arc::new(RwLock::new(comments)),
            journal_entries: Arc::new(RwLock::new(journal_entries)),
            users: Arc::new(RwLock::new(Vec::new())),
            jwt_secret: Arc::new(jwt_secret),
            admin_password_hash: Arc::new(RwLock::new(admin_hash)),
            admin_must_change_password: Arc::new(RwLock::new(true)),
        })
    }
}

pub fn markdown_to_html(md: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(md, opts);
    let mut out = String::new();
    html::push_html(&mut out, parser);
    out
}

fn parse_frontmatter(content: &str) -> Option<(HashMap<String, String>, &str)> {
    let content = content.trim_start();
    if !content.starts_with("---") {
        return None;
    }
    let rest = &content[3..];
    let end = rest.find("\n---")?;
    let fm_str = &rest[..end];
    let body = rest[end + 4..].trim_start();
    let mut map = HashMap::new();
    for line in fm_str.lines() {
        if let Some(pos) = line.find(':') {
            let key = line[..pos].trim().to_string();
            let value = line[pos + 1..].trim().to_string();
            map.insert(key, value);
        }
    }
    Some((map, body))
}

fn load_articles() -> Vec<Article> {
    let dir = Path::new("content/articles");
    if !dir.exists() {
        return Vec::new();
    }
    let mut articles = Vec::new();
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let (fm, body) = match parse_frontmatter(&content) {
            Some(r) => r,
            None => continue,
        };
        let slug = path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown").to_string();
        let title = fm.get("title").cloned().unwrap_or_else(|| slug.clone());
        let tags: Vec<String> = fm.get("tags")
            .map(|t| t.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
            .unwrap_or_default();
        let published = fm.get("published")
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        let body_html = markdown_to_html(body);
        articles.push(Article { slug, title, body_html, tags, published });
    }
    articles.sort_by(|a, b| b.published.cmp(&a.published));
    articles
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

fn load_comments() -> Vec<Comment> {
    let path = PathBuf::from(comments_data_path());
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

pub async fn save_comments(comments: &[Comment]) {
    let path = PathBuf::from(comments_data_path());
    if let Some(parent) = path.parent() {
        let _ = tokio::fs::create_dir_all(parent).await;
    }
    if let Ok(json) = serde_json::to_string_pretty(comments) {
        let _ = tokio::fs::write(&path, json).await;
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
