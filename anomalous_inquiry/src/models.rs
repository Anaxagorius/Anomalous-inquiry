
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
    pub published: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub article_slug: String,
    pub content: String,
    pub approved: bool,
    pub submitted_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Suggestion {
    pub id: Uuid,
    pub topic: String,
    pub submitted_at: DateTime<Utc>,
}
