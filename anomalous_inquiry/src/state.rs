
use std::sync::Arc;
use crate::models::Article;

#[derive(Clone)]
pub struct AppState {
    pub articles: Arc<Vec<Article>>,
}
