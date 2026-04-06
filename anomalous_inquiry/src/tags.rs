use axum::{routing::get, Router, extract::{State, Path}, response::IntoResponse, http::StatusCode};
use std::collections::BTreeMap;
use crate::state::AppState;
use crate::auth::HtmlTemplate;
use crate::templates::{TagsTemplate, TagTemplate};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(tags_index))
        .route("/:tag", get(by_tag))
}

async fn tags_index(State(state): State<AppState>) -> impl IntoResponse {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for article in state.articles.iter() {
        for tag in &article.tags {
            *counts.entry(tag.clone()).or_insert(0) += 1;
        }
    }
    let tags: Vec<(String, usize)> = counts.into_iter().collect();
    HtmlTemplate(TagsTemplate { tags })
}

async fn by_tag(State(state): State<AppState>, Path(tag): Path<String>) -> impl IntoResponse {
    let articles: Vec<_> = state.articles.iter()
        .filter(|a| a.tags.iter().any(|t| t == &tag))
        .cloned()
        .collect();
    if articles.is_empty() {
        return (StatusCode::NOT_FOUND, "Tag not found").into_response();
    }
    HtmlTemplate(TagTemplate { tag, articles }).into_response()
}
