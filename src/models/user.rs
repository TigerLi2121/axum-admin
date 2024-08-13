use crate::common::req::Page;
use crate::common::res::RP;
use crate::models::db::get_pool;
use serde::{Serialize, Serializer};
use sqlx::types::chrono::{Local, NaiveDateTime};
use sqlx::{Error, FromRow, MySql, QueryBuilder};
use tracing::info;

pub async fn page(page: Page) -> Result<RP<Vec<User>>, Error> {
    let ms: Vec<User> = sqlx::query_as("SELECT * FROM user ORDER BY id DESC LIMIT ? OFFSET ?")
        .bind(page.limit.to_string())
        .bind(page.offset().to_string())
        .fetch_all(get_pool().unwrap())
        .await?;
    Ok(RP::ok(0, ms))
}

pub async fn sou(user: User) -> Result<(), Error> {
    let now = Local::now().naive_local();
    if user.id.is_none() {
        let row = sqlx::query::<MySql>(
            "INSERT INTO user (username,password,email,mobile,status,created_at,updated_at) VALUES (?,?,?,?,?,?,?)",
        )
            .bind(user.username)
            .bind(user.password)
            .bind(user.email)
            .bind(user.mobile)
            .bind(user.status)
            .bind(now)
            .bind(now)
            .execute(get_pool().unwrap())
            .await?;
        info!("{} rows inserted", row.rows_affected());
    } else {
        let row = sqlx::query::<MySql>(
            "UPDATE user SET username=?,password=?,email=?,mobile=?,status=?,updated_at=? WHERE id=?",
        )
            .bind(user.username)
            .bind(user.password)
            .bind(user.email)
            .bind(user.mobile)
            .bind(user.status)
            .bind(now)
            .bind(user.id)
            .execute(get_pool().unwrap())
            .await?;
        info!("{} rows updated", row.rows_affected());
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
    pub id: Option<u64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub status: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
