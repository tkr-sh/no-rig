use {
    crate::db::DB,
    axum::extract::Request,
    maud::{Markup, html},
    std::str::FromStr,
    uuid::Uuid,
    wini_macros::page,
};

struct MiniPoll {
    expected_number_of_users: i32,
    id: i32,
    title: String,
}

#[page]
pub async fn render(req: Request) -> Markup {
    let Some(uuid_as_str) = req.uri().path().split('/').next_back() else {
        return html!("Expected a UUID!");
    };

    let Ok(uuid) = Uuid::from_str(uuid_as_str) else {
        return html!("Not a valid uuid v4");
    };



    let Some(MiniPoll {
        expected_number_of_users,
        id,
        title,
    }): Option<MiniPoll> = sqlx::query_as!(
        MiniPoll,
        r#"
        select cardinality(allowed_usernames) as "expected_number_of_users!", id, title
        from polls
        where uuid = $1
        "#,
        uuid
    )
    .fetch_optional(&*DB)
    .await
    .expect("db")
    else {
        return html!("Doesn't exists!");
    };

    let number_of_users_that_voted: i64 = sqlx::query_scalar!(
        r#"
        select count(*) as "dum!"
        from polls_users pu
        where pu.poll_id = $1
        "#,
        id
    )
    .fetch_optional(&*DB)
    .await
    .expect("db")
    .unwrap_or_default();

    if number_of_users_that_voted == i64::from(expected_number_of_users) {
        struct Results {
            name: String,
            votes_array: Vec<String>,
            total_score: i64,
        }

        let results = sqlx::query_as!(
            Results,
            r#"
            select po.name, array_agg(concat(pu.name,': ', v.note)) as "votes_array!", sum(v.note) as "total_score!"
            from polls_options po
            inner join votes v on v.option_id = po.id
            inner join polls_users pu on v.user_poll_id = pu.id
            where po.poll_id = $1
            group by po.name
            order by sum(v.note) desc
            "#,
            id
        )
        .fetch_all(&*DB)
        .await
        .expect("db");

        html! {
            h1 {(title)}
            h2 {"The poll is over!!!"}
            p {"Here are the results"}
            ul {
                @for res in results {
                    li {
                        h2 { (res.name) ": " (res.total_score) "/" (number_of_users_that_voted * 5) }

                        p {
                            @for vote in res.votes_array {
                                (vote)
                                    "\n"
                            }
                        }
                    }
                }
            }
        }
    } else {
        struct VoteOption {
            id: i64,
            name: String,
        }

        let votes = sqlx::query_as!(
            VoteOption,
            r#"
            select id, name
            from polls_options 
            where poll_id = $1
            "#,
            id
        )
        .fetch_all(&*DB)
        .await
        .expect("db");

        html! {
            main {
                h1 {(title)}
                h2 {
                    "Rules"
                }
                p {
                    "You have a certain number of points that you can place. You must use "
                    i { "all" }
                    " your points! You must also vote for each option"
                }


                h2 { "Enter your name" }
                input #name {}

                h1 .invalid #score {
                    "0 / " (votes.len() * 3)
                }
                h4 {"points used"}
                h2 { "Vote for the different options" }
                ul {
                    @for vote in votes {
                        li .option {
                            (vote.name)
                            ul .score {
                                @for idx in 0..=5 {
                                    li class=(format!("opt-{}", vote.id)) onclick=(format!("set({},{idx})", vote.id)){(idx)}
                                }
                            }
                        }
                    }
                }
                p #err .hidden {
                }
                button #submit{
                    "Vote!"
                }
            }
        }
    }
}
