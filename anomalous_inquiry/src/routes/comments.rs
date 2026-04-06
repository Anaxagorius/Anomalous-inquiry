
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect, Response},
    Form,
};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::{models::Comment, state::{AppState, save_comments}};

#[derive(Deserialize)]
pub struct CommentForm {
    pub content: String,
}

pub async fn submit(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Form(form): Form<CommentForm>,
) -> Response {
    let content = form.content.trim().to_string();
    if content.is_empty() {
        return Redirect::to(&format!("/articles/{slug}")).into_response();
    }

    let comment = Comment {
        id: Uuid::new_v4(),
        article_slug: slug.clone(),
        content,
        approved: false,
        submitted_at: Utc::now(),
    };

    {
        let mut comments = state.comments.lock().unwrap();
        comments.push(comment);
        save_comments(&comments);
    }

    // Best-effort email notification (skips silently if SMTP env vars absent)
    send_notification(&slug).await;

    Redirect::to(&format!("/articles/{slug}?submitted=1")).into_response()
}

async fn send_notification(article_slug: &str) {
    use lettre::{
        message::header::ContentType,
        transport::smtp::authentication::Credentials,
        AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    };

    let (host, user, pass) = match (
        std::env::var("SMTP_HOST").ok(),
        std::env::var("SMTP_USER").ok(),
        std::env::var("SMTP_PASS").ok(),
    ) {
        (Some(h), Some(u), Some(p)) => (h, u, p),
        _ => return, // SMTP not configured
    };

    let recipient = std::env::var("ADMIN_EMAIL")
        .unwrap_or_else(|_| "tbburchell@gmail.com".to_string());

    let email = match Message::builder()
        .from("noreply@anomalousinquiry.org".parse().unwrap())
        .to(recipient.parse().unwrap())
        .subject("New comment awaiting moderation")
        .header(ContentType::TEXT_PLAIN)
        .body(format!(
            "A new anonymous comment has been submitted on article: /articles/{}\n\nVisit /admin to review.",
            article_slug
        )) {
        Ok(e) => e,
        Err(_) => return,
    };

    let creds = Credentials::new(user, pass);
    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        match AsyncSmtpTransport::<Tokio1Executor>::relay(&host) {
            Ok(b) => b.credentials(creds).build(),
            Err(_) => return,
        };

    let _ = mailer.send(email).await;
}
