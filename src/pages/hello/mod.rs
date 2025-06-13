use {
    maud::{Markup, html},
    wini_macros::{init_cache, page},
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
