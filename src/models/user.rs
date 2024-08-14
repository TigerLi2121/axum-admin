use crate::common::date_format;
use crate::common::req::Page;
use crate::common::res::RP;
use crate::models::db::get_pool;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::types::chrono::{DateTime, Local, NaiveDateTime, Utc};
use sqlx::{Error, FromRow, MySql, QueryBuilder, Row};
use tracing::info;

pub async fn page(page: Page) -> Result<RP<Vec<User>>, Error> {
    let count: (i32,) = sqlx::query_as("SELECT count(1) FROM user")
        .fetch_one(get_pool().unwrap())
        .await?;
    let mut ms: Vec<User> = vec![];
    if count.0 > 0 {
        ms = sqlx::query_as("SELECT * FROM user ORDER BY id DESC LIMIT ? OFFSET ?")
            .bind(page.limit.to_string())
            .bind(page.offset().to_string())
            .fetch_all(get_pool().unwrap())
            .await?;
    }
    Ok(RP::ok(count.0, ms))
}

pub async fn sou(user: User) -> Result<u64, Error> {
    let now = Local::now().naive_local();
    let row;
    if user.id.is_none() {
        row = sqlx::query::<MySql>(
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
        row = sqlx::query::<MySql>(
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
    Ok(row.last_insert_id())
}

pub async fn del(ids: Vec<u64>) -> Result<(), Error> {
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

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Option<u64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub status: Option<i32>,
    #[serde(with = "date_format")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(with = "date_format")]
    pub updated_at: Option<NaiveDateTime>,
}
