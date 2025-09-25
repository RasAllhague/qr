use askama::Template;
use poem::{handler, web::Html};

#[derive(Debug, Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    current: &'a str,
    year: i32,
}

#[handler]
pub fn index_ui() -> Html<String> {
    let index = IndexTemplate {
        year: 2025,
        current: "index",
    }
    .render()
    .unwrap();
    Html(index)
}

#[derive(Debug, Template)]
#[template(path = "create.html")]
struct CreateTemplate<'a> {
    current: &'a str,
    year: i32,
}

#[handler]
pub fn new_ui() -> Html<String> {
    let index = CreateTemplate {
        year: 2025,
        current: "new",
    }
    .render()
    .unwrap();
    Html(index)
}

#[derive(Debug, Template)]
#[template(path = "edit.html")]
struct EditTemplate<'a> {
    current: &'a str,
    year: i32,
}


#[handler]
pub fn edit_ui() -> Html<String> {
    let edit = EditTemplate {
        year: 2025,
        current: "edit",
    }
    .render()
    .unwrap();
    Html(edit)
}

#[derive(Debug, Template)]
#[template(path = "delete.html")]
struct DeleteTemplate<'a> {
    current: &'a str,
    year: i32,
}

#[handler]
pub fn delete_ui() -> Html<String> {
    let delete = DeleteTemplate {
        year: 2025,
        current: "delete",
    }
    .render()
    .unwrap();
    Html(delete)
}

#[derive(Debug, Template)]
#[template(path = "privacy.html")]
struct PrivacyTemplate<'a> {
    current: &'a str,
    year: i32,
}

#[handler]
pub fn privacy_ui() -> Html<String> {
    let delete = PrivacyTemplate {
        year: 2025,
        current: "privacy",
    }
    .render()
    .unwrap();
    Html(delete)
}

#[derive(Debug, Template)]
#[template(path = "legal_notice.html")]
struct LegalNoticeTemplate<'a> {
    current: &'a str,
    year: i32,
}

#[handler]
pub fn legal_notice_ui() -> Html<String> {
    let delete = LegalNoticeTemplate {
        year: 2025,
        current: "impressum",
    }
    .render()
    .unwrap();
    Html(delete)
}