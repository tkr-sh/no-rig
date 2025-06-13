use {
    crate::{db::DB, shared::wini::err::ServerError},
    axum::Json,
    serde::Deserialize,
    uuid::Uuid,
};

#[derive(Debug, Deserialize)]
pub(crate) struct CreatePoll {
    title: String,
    options: Vec<String>,
    allowed_usernames: Vec<String>,
}

#[axum::debug_handler]
pub(crate) async fn post(Json(body): Json<CreatePoll>) -> Result<String, ServerError> {
    let uuid = Uuid::new_v4();

    let id = sqlx::query_scalar!(
        r#"
        insert into polls (title, uuid, allowed_usernames)
        values ($1, $2, $3)
        returning id
        "#,
        body.title,
        uuid,
        &body.allowed_usernames,
    )
    .fetch_one(&*DB)
    .await?;

    sqlx::query!(
        r#"
        insert into polls_options (name, poll_id)
        select *
        from unnest($1::text[], $2::int4[])
        "#,
        &body.options,
        &vec![id; body.options.len()],
    )
    .fetch_one(&*DB)
    .await?;


    Ok(uuid.to_string())
}
