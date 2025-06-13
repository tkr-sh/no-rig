use {
    crate::{db::DB, shared::wini::err::ServerError},
    axum::{extract::Path, Json},
    serde::Deserialize,
};

#[derive(Debug, Deserialize)]
struct Vote {
    with_username: String,
    vote_id: String,
    options: Vec<u8>,
}


pub async fn post(Path(vote): Path<String>, Json(body): Json<Vote>) -> Result<(), ServerError> {
    let allowed_usernames = sqlx::query_scalar!(
        r#"
        select allowed_user_names
        from polls
        where uuid = $1
        "#,
        body.vote_id
    )
    .fetch_one(&*DB)
    .await?;

    if !allowed_usernames.contains(body.with_username) {
        todo!()
    }

    sqlx::query!(
        r#"
        insert into vote ()
        values ()
        "#
    )
    .execute()
    .await?;

    Ok(())
}
