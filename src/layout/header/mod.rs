use {
    crate::shared::wini::err::ServerResult,
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