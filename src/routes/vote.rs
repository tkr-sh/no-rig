use {
    crate::{
        db::DB,
        shared::{validate::validate_votes, wini::err::ServerError},
    },
    axum::{Json, extract::Path},
    serde::Deserialize,
    std::{collections::HashMap, str::FromStr},
    uuid::Uuid,
};

#[derive(Debug, Deserialize)]
pub(crate) struct Vote {
    with_username: String,
    options: HashMap<i64, usize>,
}


pub(crate) async fn post(
    Path(poll_id): Path<String>,
    Json(body): Json<Vote>,
) -> Result<(), ServerError> {
    struct Poll {
        allowed_usernames: Vec<String>,
        id: i32,
    }

    let poll_uuid = Uuid::from_str(&poll_id)
        .map_err(|_| ServerError::DebugedError(String::from("UUID is not valid")))?;
    let Some(Poll {
        allowed_usernames,
        id,
    }) = sqlx::query_as!(
        Poll,
        r#"
        select allowed_usernames as "allowed_usernames!", id
        from polls
        where uuid = $1
        "#,
        poll_uuid
    )
    .fetch_optional(&*DB)
    .await?
    else {
        return Err(ServerError::DebugedError(String::from(
            "This vote doesn't exists",
        )));
    };


    if !allowed_usernames.contains(&body.with_username) {
        return Err(ServerError::DebugedError(String::from(
            "No user exists with this name",
        )));
    }

    let already_exists = sqlx::query_scalar!(
        r#"
        select exists(
            select 1
            from polls_users pu
            where pu.name = $1 and pu.poll_id = $2
        ) as "does_exists!"
        "#,
        body.with_username,
        id,
    )
    .fetch_one(&*DB)
    .await?;

    if already_exists {
        return Err(ServerError::DebugedError(String::from(
            "User already voted!",
        )));
    }

    let nb_options = sqlx::query_scalar!(
        r#"
        select count(*)
        from polls_options
        where poll_id = $1
        "#,
        id
    )
    .fetch_one(&*DB)
    .await?;

    if let Err(err) = validate_votes(
        usize::try_from(nb_options.unwrap_or_default()).unwrap(),
        &body.options.values().copied().collect::<Vec<_>>(),
    ) {
        return Err(ServerError::DebugedError(err.to_string()));
    }

    let poll_user_id = sqlx::query_scalar!(
        r#"
        insert into polls_users(name, poll_id)
        values ($1, $2)
        returning id
        "#,
        body.with_username,
        id,
    )
    .fetch_one(&*DB)
    .await?;

    sqlx::query!(
        r#"
        insert into votes (option_id, user_poll_id, note)
        select *
        from unnest($1::int8[], $2::int8[], $3::int2[])
        "#,
        &body.options.keys().copied().collect::<Vec<i64>>(),
        &vec![poll_user_id; body.options.len()],
        &body
            .options
            .values()
            .map(|n| i16::try_from(*n).unwrap())
            .collect::<Vec<i16>>(),
    )
    .execute(&*DB)
    .await?;

    Ok(())
}
