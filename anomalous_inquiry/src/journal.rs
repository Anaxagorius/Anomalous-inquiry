use axum::{
    routing::{get, post},
    Router,
    extract::{State, Path},
    response::IntoResponse,
    Form,
    http::{StatusCode, header},
};
use serde::Deserialize;
use tower_cookies::Cookies;
use uuid::Uuid;
use crate::state::{AppState, JournalEntry, save_journal};
use crate::admin;
use crate::auth::HtmlTemplate;
use crate::templates::{AdminJournalTemplate, GuestJournalTemplate};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(journal_home))
        .route("/reflections", get(guest_reflections))
        .route("/new", post(create_entry))
        .route("/delete/:id", post(delete_entry))
        .route("/toggle/:id", post(toggle_visibility))
}

async fn journal_home(State(state): State<AppState>, cookies: Cookies) -> impl IntoResponse {
    let claims = match admin::get_admin_claims(&state, &cookies) {
        Some(c) if !c.must_change_password => c,
        Some(_) => return (StatusCode::FOUND, [(header::LOCATION, "/admin/change-password")]).into_response(),
        None => return (StatusCode::FOUND, [(header::LOCATION, "/admin/login")]).into_response(),
    };
    let _ = claims;
    let entries: Vec<JournalEntry> = state.journal_entries.read().await.iter().cloned().rev().collect();
    HtmlTemplate(AdminJournalTemplate { entries, error: None }).into_response()
}

async fn guest_reflections(State(state): State<AppState>) -> impl IntoResponse {
    let entries: Vec<JournalEntry> = state.journal_entries.read().await
        .iter()
        .filter(|e| e.visible_to_guests)
        .cloned()
        .rev()
        .collect();
    HtmlTemplate(GuestJournalTemplate { entries })
}

#[derive(Deserialize)]
struct NewEntryForm {
    title: Option<String>,
    body: String,
}

async fn create_entry(State(state): State<AppState>, cookies: Cookies, Form(form): Form<NewEntryForm>) -> impl IntoResponse {
    if !admin::is_admin(&state, &cookies) {
        return (StatusCode::FOUND, [(header::LOCATION, "/admin/login")]).into_response();
    }
    if form.body.trim().is_empty() {
        return (StatusCode::FOUND, [(header::LOCATION, "/journal")]).into_response();
    }
    let now = chrono::Utc::now().format("%B %d, %Y · %H:%M UTC").to_string();
    let entry = JournalEntry {
        id: Uuid::new_v4(),
        title: form.title.unwrap_or_default().trim().to_string(),
        body: form.body.trim().to_string(),
        visible_to_guests: false,
        created_at: now,
    };
    state.journal_entries.write().await.push(entry);
    let snapshot = state.journal_entries.read().await.clone();
    save_journal(&snapshot).await;
    (StatusCode::FOUND, [(header::LOCATION, "/journal")]).into_response()
}

async fn delete_entry(State(state): State<AppState>, cookies: Cookies, Path(entry_id): Path<Uuid>) -> impl IntoResponse {
    if !admin::is_admin(&state, &cookies) {
        return (StatusCode::FOUND, [(header::LOCATION, "/admin/login")]).into_response();
    }
    let mut entries = state.journal_entries.write().await;
    entries.retain(|e| e.id != entry_id);
    let snapshot = entries.clone();
    drop(entries);
    save_journal(&snapshot).await;
    (StatusCode::FOUND, [(header::LOCATION, "/journal")]).into_response()
}

async fn toggle_visibility(State(state): State<AppState>, cookies: Cookies, Path(entry_id): Path<Uuid>) -> impl IntoResponse {
    if !admin::is_admin(&state, &cookies) {
        return (StatusCode::FOUND, [(header::LOCATION, "/admin/login")]).into_response();
    }
    let mut entries = state.journal_entries.write().await;
    if let Some(entry) = entries.iter_mut().find(|e| e.id == entry_id) {
        entry.visible_to_guests = !entry.visible_to_guests;
    }
    let snapshot = entries.clone();
    drop(entries);
    save_journal(&snapshot).await;
    (StatusCode::FOUND, [(header::LOCATION, "/journal")]).into_response()
}
