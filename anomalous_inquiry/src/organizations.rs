use axum::{routing::get, Router, extract::State, response::IntoResponse};
use crate::state::AppState;
use crate::auth::HtmlTemplate;
use crate::templates::OrganizationsTemplate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(organizations_page))
}

async fn organizations_page(State(state): State<AppState>) -> impl IntoResponse {
    let organizations = state.organizations.as_ref().clone();
    HtmlTemplate(OrganizationsTemplate { organizations })
}
