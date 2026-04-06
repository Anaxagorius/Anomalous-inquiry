
use axum::response::{Html, IntoResponse, Response};
use axum::http::StatusCode;
use tera::{Context, Tera};

pub fn render(tera: &Tera, template: &str, ctx: Context) -> Response {
    match tera.render(template, &ctx) {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Template render error ({}): {}", template, e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {e}"))
                .into_response()
        }
    }
}
