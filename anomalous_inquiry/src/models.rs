
use serde::{Deserialize, Serialize};
use chrono::{Datelike, DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub body_markdown: String,
    pub body_html: String,
    pub tags: Vec<String>,
    pub published: NaiveDate,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: Uuid,
    pub article_slug: String,
    pub content: String,
    pub approved: bool,
    pub submitted_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Suggestion {
    pub id: Uuid,
    pub topic: String,
    pub submitted_at: DateTime<Utc>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CitationStyle {
    Apa,
    Chicago,
}

impl Article {
    pub fn format_citation(&self, style: CitationStyle) -> String {
        let year = self.published.format("%Y");
        // Use day() for cross-platform day-of-month without leading zero
        let month_name = self.published.format("%B");
        let day = self.published.day();
        let year_str = self.published.format("%Y");
        match style {
            CitationStyle::Apa => format!(
                "Anomalous Inquiry. ({year}). *{}*. https://anomalousinquiry.org/articles/{}",
                self.title, self.slug
            ),
            CitationStyle::Chicago => format!(
                "Anomalous Inquiry. \"{}.\" Anomalous Inquiry, {month_name} {day}, {year_str}. https://anomalousinquiry.org/articles/{}.",
                self.title, self.slug
            ),
        }
    }
}
