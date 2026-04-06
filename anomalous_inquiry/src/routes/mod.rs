
pub mod articles;
pub mod admin;
pub mod comments;
pub mod organizations;
pub mod rss;
pub mod tags;
pub mod timeline;

use axum::{
    Router,
    routing::{get, post},
};
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        // public
        .route("/", get(articles::home))
        .route("/articles", get(articles::list))
        .route("/articles/:slug", get(articles::detail))
        .route("/articles/:slug/pdf", get(articles::pdf_export))
        .route("/methodology", get(articles::methodology))
        .route("/tags", get(tags::index))
        .route("/tags/:tag", get(tags::by_tag))
        .route("/timeline", get(timeline::page))
        .route("/organizations", get(organizations::page))
        .route("/rss.xml", get(rss::feed))
        // comments
        .route("/articles/:slug/comments", post(comments::submit))
        // admin
        .route("/admin/login", get(admin::login_page).post(admin::login))
        .route("/admin/logout", post(admin::logout))
        .route("/admin", get(admin::dashboard))
        .route("/admin/comments/:id/approve", post(admin::approve))
        .route("/admin/comments/:id/reject", post(admin::reject))
        .with_state(state)
}
