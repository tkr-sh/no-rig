use {
    maud::{Markup, html},
    wini_macros::page,
};

#[page]
pub async fn render() -> Markup {
    html! {
        h1 {"Let's create a new poll!"}
        input placeholder = "Poll title" #title;

        h2 { "Options" }
        ul #options {}
        input #option {}
        button _="on click make a <li/>
            put #option's value into its textContent
            append it to #options
            then
            set #option's value to ''
            " {
            "+"
        }

        h2 { "Users authorized" }
        ul #users {}
        input #user {}
        button _="on click make a <li/>
            put #user's value into its textContent
            append it to #users
            then
            set #user's value to ''
            " {
            "+"
        }

        button #submit
        {
            "Create"
        }

        p #res {

        }
    }
}

