
use axum::{extract::State, response::Response};
use tera::Context;

use crate::{render::render, state::AppState};

pub async fn page(State(state): State<AppState>) -> Response {
    let mut ctx = Context::new();
    ctx.insert("organizations", state.organizations.as_ref());
    ctx.insert("page_title", "Organizations & Institutions");
    render(&state.tera, "organizations.html", ctx)
}
