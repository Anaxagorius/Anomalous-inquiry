use axum::{
    routing::get,
    Form, Router,
    extract::State,
    response::IntoResponse,
    http::{StatusCode, header},
};
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use crate::state::{AppState, ADMIN_USERNAME};
use crate::auth::HtmlTemplate;
use crate::templates::{AdminLoginTemplate, AdminChangePasswordTemplate, AdminDashboardTemplate};

pub const ADMIN_COOKIE: &str = "ai_admin_session";

#[derive(Debug, Deserialize)]
pub struct AdminLoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordForm {
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminClaims {
    pub sub: String,
    pub exp: usize,
    pub must_change_password: bool,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", get(admin_login_page).post(admin_login_post))
        .route("/change-password", get(admin_change_password_page).post(admin_change_password_post))
        .route("/logout", get(admin_logout))
        .route("/", get(admin_dashboard))
}

pub async fn admin_login_page() -> impl IntoResponse {
    HtmlTemplate(AdminLoginTemplate { error: None })
}

pub async fn admin_login_post(
    State(state): State<AppState>,
    cookies: Cookies,
    Form(form): Form<AdminLoginForm>,
) -> impl IntoResponse {
    if form.username != ADMIN_USERNAME {
        return HtmlTemplate(AdminLoginTemplate { error: Some("Invalid credentials".into()) }).into_response();
    }
    let hash = state.admin_password_hash.read().await.clone();
    let parsed_hash = match PasswordHash::new(&hash) {
        Ok(h) => h,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    if Argon2::default().verify_password(form.password.as_bytes(), &parsed_hash).is_err() {
        return HtmlTemplate(AdminLoginTemplate { error: Some("Invalid credentials".into()) }).into_response();
    }
    let must_change = *state.admin_must_change_password.read().await;
    let token = make_admin_token(&state, must_change);
    cookies.add(
        Cookie::build((ADMIN_COOKIE, token))
            .path("/")
            .http_only(true)
            .secure(true)
            .build(),
    );
    if must_change {
        (StatusCode::FOUND, [(header::LOCATION, "/admin/change-password")]).into_response()
    } else {
        (StatusCode::FOUND, [(header::LOCATION, "/home")]).into_response()
    }
}

pub async fn admin_change_password_page(State(state): State<AppState>, cookies: Cookies) -> impl IntoResponse {
    if get_admin_claims(&state, &cookies).is_none() {
        return (StatusCode::FOUND, [(header::LOCATION, "/admin/login")]).into_response();
    }
    let forced = *state.admin_must_change_password.read().await;
    HtmlTemplate(AdminChangePasswordTemplate { error: None, forced }).into_response()
}

pub async fn admin_change_password_post(
    State(state): State<AppState>,
    cookies: Cookies,
    Form(form): Form<ChangePasswordForm>,
) -> impl IntoResponse {
    if get_admin_claims(&state, &cookies).is_none() {
        return (StatusCode::FOUND, [(header::LOCATION, "/admin/login")]).into_response();
    }
    let forced = *state.admin_must_change_password.read().await;
    if form.new_password != form.confirm_password {
        return HtmlTemplate(AdminChangePasswordTemplate { error: Some("Passwords do not match".into()), forced }).into_response();
    }
    if form.new_password.len() < 8 {
        return HtmlTemplate(AdminChangePasswordTemplate { error: Some("Password must be at least 8 characters".into()), forced }).into_response();
    }
    let salt = SaltString::generate(&mut rand::thread_rng());
    let new_hash = Argon2::default().hash_password(form.new_password.as_bytes(), &salt).unwrap().to_string();
    *state.admin_password_hash.write().await = new_hash;
    *state.admin_must_change_password.write().await = false;
    let token = make_admin_token(&state, false);
    cookies.add(
        Cookie::build((ADMIN_COOKIE, token))
            .path("/")
            .http_only(true)
            .secure(true)
            .build(),
    );
    (StatusCode::FOUND, [(header::LOCATION, "/home")]).into_response()
}

pub async fn admin_logout(cookies: Cookies) -> impl IntoResponse {
    let mut removal = Cookie::new(ADMIN_COOKIE, "");
    removal.set_path("/");
    removal.make_removal();
    cookies.add(removal);
    (StatusCode::FOUND, [(header::LOCATION, "/")]).into_response()
}

pub async fn admin_dashboard(State(state): State<AppState>, cookies: Cookies) -> impl IntoResponse {
    let claims = match get_admin_claims(&state, &cookies) {
        Some(c) if !c.must_change_password => c,
        Some(_) => return (StatusCode::FOUND, [(header::LOCATION, "/admin/change-password")]).into_response(),
        None => return (StatusCode::FOUND, [(header::LOCATION, "/admin/login")]).into_response(),
    };
    let _ = claims;
    HtmlTemplate(AdminDashboardTemplate).into_response()
}

pub fn get_admin_claims(state: &AppState, cookies: &Cookies) -> Option<AdminClaims> {
    let token = cookies.get(ADMIN_COOKIE)?.value().to_string();
    decode::<AdminClaims>(&token, &DecodingKey::from_secret(state.jwt_secret.as_bytes()), &Validation::default())
        .ok()
        .map(|d| d.claims)
}

pub fn is_admin(state: &AppState, cookies: &Cookies) -> bool {
    get_admin_claims(state, cookies)
        .map(|c| !c.must_change_password)
        .unwrap_or(false)
}

fn make_admin_token(state: &AppState, must_change_password: bool) -> String {
    let exp = chrono::Utc::now().timestamp() as usize + 60 * 60 * 24;
    let claims = AdminClaims { sub: "admin".to_string(), exp, must_change_password };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(state.jwt_secret.as_bytes()))
        .expect("JWT encoding failed")
}
