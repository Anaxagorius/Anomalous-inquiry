mod state;
mod auth;
mod admin;
mod templates;
mod journal;
mod organizations;
mod timeline;
mod parapsychology;
mod uap;
mod survival;
mod altered_states;
mod nhi;

use axum::{Router, routing::get, extract::State, response::IntoResponse, http::{StatusCode, header}};
use tower_cookies::{CookieManagerLayer, Cookies};
use tracing_subscriber::EnvFilter;
use state::AppState;
use auth::HtmlTemplate;
use templates::{LandingTemplate, AppLandingTemplate};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let state = AppState::new().await?;

    let app = Router::new()
        .route("/", get(landing))
        .route("/home", get(app_landing))
        .route("/favicon.svg", get(favicon))
        .nest("/admin", admin::routes())
        .nest("/auth", auth::routes())
        .nest("/journal", journal::routes())
        .nest("/organizations", organizations::routes())
        .nest("/timeline", timeline::routes())
        .nest("/parapsychology", parapsychology::routes())
        .nest("/uap", uap::routes())
        .nest("/survival", survival::routes())
        .nest("/altered-states", altered_states::routes())
        .nest("/nhi", nhi::routes())
        .layer(CookieManagerLayer::new())
        .with_state(state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(8080);
    let addr = format!("0.0.0.0:{port}");
    println!("🔭 Anomalous Inquiry running at http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn favicon() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "image/svg+xml")],
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 32 32"><text y="28" font-size="28">🔭</text></svg>"#,
    )
}

async fn landing() -> HtmlTemplate<LandingTemplate> {
    HtmlTemplate(LandingTemplate)
}

async fn app_landing(State(state): State<AppState>, cookies: Cookies) -> impl IntoResponse {
    let admin_claims = admin::get_admin_claims(&state, &cookies);
    if let Some(ref claims) = admin_claims {
        if claims.must_change_password {
            return (StatusCode::FOUND, [(header::LOCATION, "/admin/change-password")]).into_response();
        }
    }
    let is_admin = admin_claims.is_some();
    HtmlTemplate(AppLandingTemplate { is_admin }).into_response()
}
