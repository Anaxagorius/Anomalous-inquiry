use axum::{
    routing::{get, post},
    Router,
    extract::{State, Path},
    response::IntoResponse,
    Form,
    http::{StatusCode, header},
};
use serde::Deserialize;
use uuid::Uuid;
use crate::state::{AppState, Comment, save_comments};
use crate::auth::HtmlTemplate;
use crate::templates::{ArticleListTemplate, ArticleTemplate};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(article_list))
        .route("/:slug", get(article_detail))
        .route("/:slug/comments", post(submit_comment))
}

async fn article_list(State(state): State<AppState>) -> impl IntoResponse {
    let articles = state.articles.as_ref().clone();
    HtmlTemplate(ArticleListTemplate { articles })
}

async fn article_detail(State(state): State<AppState>, Path(slug): Path<String>) -> impl IntoResponse {
    match state.articles.iter().find(|a| a.slug == slug) {
        Some(article) => {
            let comments: Vec<Comment> = state.comments.read().await
                .iter()
                .filter(|c| c.article_slug == slug && c.approved)
                .cloned()
                .collect();
            HtmlTemplate(ArticleTemplate { article: article.clone(), comments }).into_response()
        }
        None => (StatusCode::NOT_FOUND, "Article not found").into_response(),
    }
}

#[derive(Deserialize)]
struct CommentForm {
    content: String,
}

async fn submit_comment(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Form(form): Form<CommentForm>,
) -> impl IntoResponse {
    if form.content.trim().is_empty() {
        return (StatusCode::FOUND, [(header::LOCATION, format!("/articles/{slug}"))]).into_response();
    }
    let now = chrono::Utc::now().format("%B %d, %Y · %H:%M UTC").to_string();
    let comment = Comment {
        id: Uuid::new_v4(),
        article_slug: slug.clone(),
        content: form.content.trim().to_string(),
        approved: false,
        submitted_at: now,
    };
    state.comments.write().await.push(comment);
    let snapshot = state.comments.read().await.clone();
    save_comments(&snapshot).await;
    (StatusCode::FOUND, [(header::LOCATION, format!("/articles/{slug}"))]).into_response()
}
