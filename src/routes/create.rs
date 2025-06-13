use {
    crate::{db::DB, shared::wini::err::ServerError},
    axum::{extract::Path, Json},
    serde::Deserialize,
    uuid::Uuid,
};

#[derive(Debug, Deserialize)]
struct CreatePoll {
    title: String,
    options: Vec<String>,
    allowed_usernames: Vec<String>,
}

#[axum::debug_handler]
pub async fn post(
    Path(vote): Path<String>,
    Json(body): Json<CreatePoll>,
) -> Result<(), ServerError> {
    let uuid = Uuid::new_v4();

    let poll_id: i32 = sqlx::query_scalar!(
        r#"
        insert into poll (title, uuid, allowd_usernames)
        values ($1, $2, $3)
        returning id
        "#,
        body.title,
        uuid,
        body.allowed_usernames,
    )
    .fetch_one(&*DB)
    .await?;

    Ok(())
}
