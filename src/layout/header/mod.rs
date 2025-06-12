use {
    crate::shared::wini::err::ServerResult,
    axum::{body::Body, http::response::Parts},
    hyper::StatusCode,
    maud::{html, Markup, PreEscaped},
    wini_macros::layout,
};

#[layout]
pub async fn render(s: &str) -> ServerResult<Markup> {
    Ok(html! {
        header {
            "Welcome to Wini!"
        }
        (PreEscaped(s))
    })
}