use askama::Template;
use askama_axum::IntoResponse;

use super::HtmlTemplate;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate;
    HtmlTemplate(template)
}
