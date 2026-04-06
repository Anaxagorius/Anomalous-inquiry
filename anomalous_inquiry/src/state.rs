
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use pulldown_cmark::{html, Options, Parser};
use tera::Tera;

use crate::models::{Article, Comment, Organization, TimelineEvent};

#[derive(Clone)]
pub struct AppState {
    pub articles: Arc<Vec<Article>>,
    pub comments: Arc<Mutex<Vec<Comment>>>,
    pub timeline: Arc<Vec<TimelineEvent>>,
    pub organizations: Arc<Vec<Organization>>,
    pub tera: Arc<Tera>,
    pub session_token: Arc<Mutex<Option<String>>>,
}

impl AppState {
    pub fn load() -> Self {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![
            ("base.html",          include_str!("../templates/base.html")),
            ("home.html",          include_str!("../templates/home.html")),
            ("article.html",       include_str!("../templates/article.html")),
            ("tag.html",           include_str!("../templates/tag.html")),
            ("tags.html",          include_str!("../templates/tags.html")),
            ("timeline.html",      include_str!("../templates/timeline.html")),
            ("organizations.html", include_str!("../templates/organizations.html")),
            ("methodology.html",   include_str!("../templates/methodology.html")),
            ("admin/login.html",     include_str!("../templates/admin/login.html")),
            ("admin/dashboard.html", include_str!("../templates/admin/dashboard.html")),
        ]).expect("Failed to load Tera templates");
        let articles = load_articles();
        let comments = load_comments();
        let timeline = load_timeline();
        let organizations = load_organizations();
        AppState {
            articles: Arc::new(articles),
            comments: Arc::new(Mutex::new(comments)),
            timeline: Arc::new(timeline),
            organizations: Arc::new(organizations),
            tera: Arc::new(tera),
            session_token: Arc::new(Mutex::new(None)),
        }
    }

    pub fn is_valid_session(&self, token: &str) -> bool {
        let stored = self.session_token.lock().unwrap();
        matches!(stored.as_deref(), Some(t) if t == token)
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
    use chrono::NaiveDate;
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
        let slug = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        let title = fm.get("title").cloned().unwrap_or_else(|| slug.clone());
        let tags: Vec<String> = fm
            .get("tags")
            .map(|t| t.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
            .unwrap_or_default();
        let published = fm
            .get("published")
            .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
        let body_html = markdown_to_html(body);
        articles.push(Article {
            slug,
            title,
            body_markdown: body.to_string(),
            body_html,
            tags,
            published,
        });
    }
    articles.sort_by(|a, b| b.published.cmp(&a.published));
    articles
}

fn data_dir() -> PathBuf {
    std::env::var("DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("data"))
}

fn load_comments() -> Vec<Comment> {
    let path = data_dir().join("comments.json");
    if !path.exists() {
        return Vec::new();
    }
    fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn load_timeline() -> Vec<TimelineEvent> {
    let path = Path::new("content/timeline.json");
    if !path.exists() {
        return Vec::new();
    }
    fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn load_organizations() -> Vec<Organization> {
    let path = Path::new("content/organizations.json");
    if !path.exists() {
        return Vec::new();
    }
    fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_comments(comments: &[Comment]) {
    let path = data_dir().join("comments.json");
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(comments) {
        let _ = fs::write(path, json);
    }
}
