
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tera::Context;

use crate::{render::render, state::AppState};

pub async fn index(State(state): State<AppState>) -> Response {
    use std::collections::HashMap;
    let mut tag_counts: HashMap<&str, usize> = HashMap::new();
    for article in state.articles.iter() {
        for tag in &article.tags {
            *tag_counts.entry(tag.as_str()).or_insert(0) += 1;
        }
    }
    let mut tags: Vec<(&str, usize)> = tag_counts.into_iter().collect();
    tags.sort_by(|a, b| a.0.cmp(b.0));

    let mut ctx = Context::new();
    ctx.insert("tags", &tags);
    ctx.insert("page_title", "Tag Index");
    render(&state.tera, "tags.html", ctx)
}

pub async fn by_tag(
    State(state): State<AppState>,
    Path(tag): Path<String>,
) -> Response {
    let articles: Vec<_> = state
        .articles
        .iter()
        .filter(|a| a.tags.iter().any(|t| t == &tag))
        .collect();

    if articles.is_empty() {
        return (StatusCode::NOT_FOUND, format!("No articles tagged '{tag}'"))
            .into_response();
    }

    let mut ctx = Context::new();
    ctx.insert("tag", &tag);
    ctx.insert("articles", &articles);
    ctx.insert("page_title", &format!("Tag: {tag}"));
    render(&state.tera, "tag.html", ctx)
}
