use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sqlx::Row;

use super::models::User;

pub async fn user_create(
    State(pool): State<sqlx::PgPool>,
    Json(payload): Json<User>,
) -> Result<(), impl IntoResponse> {
    let result = sqlx::query(
        "insert into users(username,password,firstname,lastname) values($1,$2,$3,$4)
    ",
    )
    .bind(payload.username)
    .bind(payload.password)
    .bind(payload.first_name)
    .bind(payload.last_name)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Oops something went wrong"),
        )),
    }
}

pub async fn user_read(
    State(pool): State<sqlx::PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<User>, impl IntoResponse> {
    let result = sqlx::query("select * from users where id=$1")
        .bind(id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(data) => {
            let user = User {
                username: data.get("username"),
                password: data.get("password"),
                first_name: data.get("firstname"),
                last_name: data.get("lastname"),
            };
            Ok(Json(user))
        }

        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Oops something went wrong"),
        )),
    }
}

pub async fn users_read(
    State(pool): State<sqlx::PgPool>,
) -> Result<Json<Vec<User>>, impl IntoResponse> {
    let result: Result<Vec<sqlx::postgres::PgRow>, sqlx::Error> =
        sqlx::query("select * from users").fetch_all(&pool).await;
    match result {
        Ok(data) => {
            let users = data
                .iter()
                .map(|f| User {
                    username: f.get("username"),
                    password: f.get("password"),
                    first_name: f.get("firstname"),
                    last_name: f.get("lastname"),
                })
                .collect();
            Ok(Json(users))
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Oops something went wrong"),
        )),
    }
}

pub async fn user_update(
    State(pool): State<sqlx::PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<User>,
) -> Result<(), impl IntoResponse> {
    let mut set_fields: Vec<(String, String)> = vec![];
    if let Some(username) = payload.username {
        set_fields.push((String::from("username"), username));
    }
    if let Some(password) = payload.password {
        set_fields.push((String::from("password"), password));
    }
    if let Some(first_name) = payload.first_name {
        set_fields.push((String::from("firstname"), first_name));
    }
    if let Some(last_name) = payload.last_name {
        set_fields.push((String::from("lastname"), last_name));
    }

    let field_names: Vec<String> = set_fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let (a, _) = f;
            format!("{}=${}", a, i + 1)
        })
        .collect();

    let last_sql_binding_index = set_fields.len() + 1;

    let sql_query = format!(
        "update users set {} where id=${}",
        field_names.join(","),
        last_sql_binding_index
    );

    let mut sql_builder = sqlx::query(&sql_query);

    for (_, value) in set_fields.iter() {
        sql_builder = sql_builder.bind(value);
    }

    let result = sql_builder.bind(id).execute(&pool).await;

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e))),
    }
}

pub async fn user_delete(
    State(pool): State<sqlx::PgPool>,
    Path(id): Path<i32>,
) -> Result<(), impl IntoResponse> {
    let result = sqlx::query("delete from users where id=$1")
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(()),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Oops something went wrong"),
        )),
    }
}
