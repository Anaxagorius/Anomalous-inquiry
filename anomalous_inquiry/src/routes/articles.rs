
use axum::{
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use printpdf::{BuiltinFont, Mm, PdfDocument};
use serde::Deserialize;
use tera::Context;

use crate::{
    models::CitationStyle,
    render::render,
    state::AppState,
};

const PDF_LINE_CHAR_WIDTH: usize = 90;

pub async fn home(State(state): State<AppState>) -> Response {
    list(State(state)).await
}

pub async fn list(State(state): State<AppState>) -> Response {
    let mut ctx = Context::new();
    ctx.insert("articles", state.articles.as_ref());
    ctx.insert("page_title", "Anomalous Inquiry");
    render(&state.tera, "home.html", ctx)
}

pub async fn methodology(State(state): State<AppState>) -> Response {
    let mut ctx = Context::new();
    ctx.insert("page_title", "Methodology");
    render(&state.tera, "methodology.html", ctx)
}

#[derive(Deserialize)]
pub struct CitationQuery {
    pub citation: Option<String>,
}

pub async fn detail(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(q): Query<CitationQuery>,
) -> Response {
    let article = state.articles.iter().find(|a| a.slug == slug);
    let Some(article) = article else {
        return (StatusCode::NOT_FOUND, "Article not found").into_response();
    };

    let style = match q.citation.as_deref() {
        Some("chicago") => CitationStyle::Chicago,
        _ => CitationStyle::Apa,
    };
    let citation = article.format_citation(style);
    let citation_style = match style {
        CitationStyle::Apa => "apa",
        CitationStyle::Chicago => "chicago",
    };

    let approved_comments: Vec<_> = state
        .comments
        .lock()
        .unwrap()
        .iter()
        .filter(|c| c.article_slug == slug && c.approved)
        .cloned()
        .collect();

    let mut ctx = Context::new();
    ctx.insert("article", article);
    ctx.insert("citation", &citation);
    ctx.insert("citation_style", &citation_style);
    ctx.insert("comments", &approved_comments);
    ctx.insert("page_title", &article.title);
    render(&state.tera, "article.html", ctx)
}

pub async fn pdf_export(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Response {
    let article = state.articles.iter().find(|a| a.slug == slug);
    let Some(article) = article else {
        return (StatusCode::NOT_FOUND, "Article not found").into_response();
    };

    let (doc, page1, layer1) =
        PdfDocument::new(&article.title, Mm(210.0), Mm(297.0), "Layer 1");
    let layer = doc.get_page(page1).get_layer(layer1);

    let font = match doc.add_builtin_font(BuiltinFont::HelveticaBold) {
        Ok(f) => f,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("PDF font error: {e}"))
                .into_response()
        }
    };
    let body_font = match doc.add_builtin_font(BuiltinFont::Helvetica) {
        Ok(f) => f,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("PDF font error: {e}"))
                .into_response()
        }
    };

    // Title
    layer.use_text(&article.title, 20.0, Mm(20.0), Mm(270.0), &font);

    // Published date
    let date_str = format!(
        "Published: {}  |  Tags: {}",
        article.published.format("%Y-%m-%d"),
        article.tags.join(", ")
    );
    layer.use_text(&date_str, 10.0, Mm(20.0), Mm(260.0), &body_font);

    // Separator
    layer.use_text(
        "─────────────────────────────────────────────────────",
        9.0,
        Mm(20.0),
        Mm(255.0),
        &body_font,
    );

    // Body — plain text, line-wrapped at ~90 chars, 12pt, stepping down
    let mut y = Mm(248.0);
    let line_step = Mm(6.0);
    let margin_bottom = Mm(20.0);

    for raw_line in article.body_markdown.lines() {
        // Strip simple markdown markers for plain PDF text
        let text = raw_line
            .trim_start_matches('#')
            .trim_start_matches('*')
            .trim_end_matches('*')
            .trim();
        if text.is_empty() {
            y -= line_step;
            continue;
        }
        // Chunk into ~90-char display segments
        let mut remaining = text;
        while !remaining.is_empty() {
            let chunk_end = remaining.char_indices().nth(PDF_LINE_CHAR_WIDTH).map(|(i, _)| i).unwrap_or(remaining.len());
            let chunk = &remaining[..chunk_end];
            remaining = &remaining[chunk_end..];

            if y < margin_bottom {
                break;
            }
            layer.use_text(chunk, 11.0, Mm(20.0), y, &body_font);
            y -= line_step;
        }
    }

    // Citation footer
    let citation = article.format_citation(CitationStyle::Apa);
    layer.use_text(&citation, 8.0, Mm(20.0), Mm(15.0), &body_font);

    let bytes = match doc.save_to_bytes() {
        Ok(b) => b,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("PDF save error: {e}"))
                .into_response()
        }
    };

    let content_disposition = format!("attachment; filename=\"{slug}.pdf\"");
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/pdf".parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        content_disposition.parse().unwrap(),
    );
    (headers, bytes).into_response()
}
