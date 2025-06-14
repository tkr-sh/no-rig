use {
    maud::{Markup, html},
    wini_macros::page,
};

// #[init_cache]
#[page]
pub async fn render() -> Markup {
    println!("wtf");
    html! {
        button #hello {
            "Say hello!???????"
        }
    }
}
