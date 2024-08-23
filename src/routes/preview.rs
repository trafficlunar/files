use askama::Template;
use axum::{extract::Path, response::Html};

#[derive(Template)]
#[template(path = "preview.html")]
struct PreviewTemplate<'a> {
    file: &'a str,
    page_title: &'a str,
}

pub async fn route(Path(filename): Path<String>) -> Html<String> {
    let page_title = match std::env::var("PAGE_TITLE") {
        Ok(page_title) => page_title,
        Err(_) => "files".to_string()
    };
    let template = PreviewTemplate {
        file: filename.as_str(),
        page_title: page_title.as_str(),
    };
    let rendered = template.render().unwrap();
    Html(rendered)
}
