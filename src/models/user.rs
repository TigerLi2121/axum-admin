use crate::common::req::Page;
use crate::models::db;
use crate::models::db::get_pool;
use serde::Serialize;
use sqlx::{Error, FromRow, MySql, QueryBuilder};
use tracing::info;

pub async fn page(page: Page) -> Result<(), Error> {
    let mut tis: Vec<User> = sqlx::query_as("SELECT * FROM user")
        .bind(page.offset.to_string())
        .bind(page.limit.to_string())
        .execute(get_pool().unwrap())
        .await?;
    Ok(())
}

pub async fn sou(user: User) -> Result<(), Error> {
    if user.id.is_none() {
        sqlx::query::<MySql>(
            "INSERT INTO user (username,password,email,mobile,status) VALUES ( $1,$2,$3,$4,$5 )",
        )
        .bind(user.username)
        .bind(user.password)
        .bind(user.email)
        .bind(user.mobile)
        .bind(user.status)
        .execute(get_pool().unwrap())
        .await?;
    } else {
        sqlx::query::<MySql>(
            "UPDATE user SET username=$2,password=$3,email=$4,mobile=$5,status=$6 WHERE id=$1",
        )
        .bind(user.id)
        .bind(user.username)
        .bind(user.password)
        .bind(user.email)
        .bind(user.mobile)
        .bind(user.status)
        .execute(get_pool().unwrap())
        .await?;
    }
    Ok(())
}

pub async fn del(ids: Vec<i32>) -> Result<(), Error> {
    let mut sql: QueryBuilder<MySql> = QueryBuilder::new("DELETE FROM user WHERE id IN (");
    let mut ps = sql.separated(", ");
    for id in ids.iter() {
        ps.push_bind(id);
    }
    ps.push_unseparated(") ");
    let row = sql.build().execute(get_pool().unwrap()).await?;
    info!("{} rows deleted", row.rows_affected());
    Ok(())
}

#[derive(Debug, FromRow, Serialize)]
pub struct User {
    id: Option<u64>,
    username: Option<String>,
    password: Option<String>,
    email: Option<String>,
    mobile: Option<String>,
    status: Option<i32>,
    created_at: Option<String>,
    updated_at: Option<String>,
}
