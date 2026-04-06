
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    Form,
};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde::Deserialize;
use tera::Context;
use uuid::Uuid;

use crate::{
    render::render,
    state::{save_comments, AppState},
};

const SESSION_COOKIE: &str = "admin_session";

fn admin_password() -> String {
    std::env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "anomalous_admin".to_string())
}

fn cookie_token(jar: &CookieJar) -> Option<String> {
    jar.get(SESSION_COOKIE).map(|c| c.value().to_string())
}

fn is_authenticated(jar: &CookieJar, state: &AppState) -> bool {
    match cookie_token(jar) {
        Some(tok) => state.is_valid_session(&tok),
        None => false,
    }
}

// GET /admin/login
pub async fn login_page(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Response {
    if is_authenticated(&jar, &state) {
        return Redirect::to("/admin").into_response();
    }
    let mut ctx = Context::new();
    ctx.insert("page_title", "Admin Login");
    ctx.insert("error", &false);
    render(&state.tera, "admin/login.html", ctx)
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub password: String,
}

// POST /admin/login
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Form(form): Form<LoginForm>,
) -> Response {
    if form.password != admin_password() {
        let mut ctx = Context::new();
        ctx.insert("page_title", "Admin Login");
        ctx.insert("error", &true);
        return render(&state.tera, "admin/login.html", ctx);
    }
    let token = Uuid::new_v4().to_string();
    {
        let mut stored = state.session_token.lock().unwrap();
        *stored = Some(token.clone());
    }
    let cookie = Cookie::build((SESSION_COOKIE, token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();
    (jar.add(cookie), Redirect::to("/admin")).into_response()
}

// POST /admin/logout
pub async fn logout(State(state): State<AppState>, jar: CookieJar) -> Response {
    {
        let mut stored = state.session_token.lock().unwrap();
        *stored = None;
    }
    let removal = Cookie::build((SESSION_COOKIE, "")).path("/").build();
    (jar.remove(removal), Redirect::to("/admin/login")).into_response()
}

// GET /admin
pub async fn dashboard(State(state): State<AppState>, jar: CookieJar) -> Response {
    if !is_authenticated(&jar, &state) {
        return Redirect::to("/admin/login").into_response();
    }
    let comments = state.comments.lock().unwrap().clone();
    let pending: Vec<_> = comments.iter().filter(|c| !c.approved).collect();
    let approved: Vec<_> = comments.iter().filter(|c| c.approved).collect();

    let mut ctx = Context::new();
    ctx.insert("pending", &pending);
    ctx.insert("approved", &approved);
    ctx.insert("page_title", "Admin Dashboard");
    render(&state.tera, "admin/dashboard.html", ctx)
}

// POST /admin/comments/:id/approve
pub async fn approve(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<String>,
) -> Response {
    if !is_authenticated(&jar, &state) {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }
    {
        let mut comments = state.comments.lock().unwrap();
        if let Some(c) = comments.iter_mut().find(|c| c.id.to_string() == id) {
            c.approved = true;
        }
        save_comments(&comments);
    }
    Redirect::to("/admin").into_response()
}

// POST /admin/comments/:id/reject
pub async fn reject(
    State(state): State<AppState>,
    jar: CookieJar,
    Path(id): Path<String>,
) -> Response {
    if !is_authenticated(&jar, &state) {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }
    {
        let mut comments = state.comments.lock().unwrap();
        comments.retain(|c| c.id.to_string() != id);
        save_comments(&comments);
    }
    Redirect::to("/admin").into_response()
}
