use axum::{
    routing::get,
    Form, Router,
    extract::State,
    response::{IntoResponse, Response},
    http::{StatusCode, header},
};
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use crate::state::{AppState, User};
use crate::templates::{LoginTemplate, SignupTemplate};

#[derive(Debug, Deserialize)]
pub struct AuthForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", get(login_page).post(login_post))
        .route("/signup", get(signup_page).post(signup_post))
        .route("/logout", get(logout))
}

pub async fn login_page() -> impl IntoResponse {
    HtmlTemplate(LoginTemplate { error: None })
}

pub async fn signup_page() -> impl IntoResponse {
    HtmlTemplate(SignupTemplate { error: None })
}

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: askama::Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => (StatusCode::OK, [(header::CONTENT_TYPE, "text/html; charset=utf-8")], html).into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

pub async fn signup_post(State(state): State<AppState>, Form(form): Form<AuthForm>) -> impl IntoResponse {
    let mut users = state.users.write().await;
    if users.iter().any(|u| u.email == form.email) {
        return HtmlTemplate(SignupTemplate { error: Some("Email already registered".into()) }).into_response();
    }
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(form.password.as_bytes(), &salt)
        .unwrap().to_string();
    let user = User::new(form.email.clone(), password_hash);
    users.push(user);
    HtmlTemplate(LoginTemplate { error: Some("Account created. Please log in.".into()) }).into_response()
}

pub async fn login_post(State(state): State<AppState>, cookies: Cookies, Form(form): Form<AuthForm>) -> impl IntoResponse {
    let users = state.users.read().await;
    let user = match users.iter().find(|u| u.email == form.email) {
        Some(u) => u,
        None => {
            return HtmlTemplate(LoginTemplate { error: Some("Invalid credentials".into()) }).into_response();
        }
    };
    let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
    if Argon2::default().verify_password(form.password.as_bytes(), &parsed_hash).is_err() {
        return HtmlTemplate(LoginTemplate { error: Some("Invalid credentials".into()) }).into_response();
    }
    let exp = chrono::Utc::now().timestamp() as usize + 60 * 60 * 24;
    let claims = Claims { sub: user.id.to_string(), exp };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(state.jwt_secret.as_bytes())).unwrap();
    cookies.add(
        Cookie::build(("ai_session", token))
            .path("/")
            .http_only(true)
            .secure(true)
            .build(),
    );
    (StatusCode::FOUND, [(header::LOCATION, "/home")]).into_response()
}

#[allow(dead_code)]
pub fn decode_token(state: &AppState, token: &str) -> Option<Claims> {
    decode::<Claims>(token, &DecodingKey::from_secret(state.jwt_secret.as_bytes()), &Validation::default())
        .ok()
        .map(|d| d.claims)
}

pub async fn logout(cookies: Cookies) -> impl IntoResponse {
    let mut removal = Cookie::new("ai_session", "");
    removal.set_path("/");
    removal.make_removal();
    cookies.add(removal);
    (StatusCode::FOUND, [(header::LOCATION, "/")]).into_response()
}
