use crate::common::date_format;
use crate::common::db::get_pool;
use crate::common::req::Page;
use crate::common::res::RP;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{Error, FromRow, MySql, QueryBuilder};
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
            .bind(user.created_at)
            .bind(user.updated_at)
            .execute(get_pool().unwrap())
            .await?;
        info!("{} rows inserted", row.rows_affected());
    } else {
        let mut sql: QueryBuilder<MySql> = QueryBuilder::new("UPDATE user SET username=");
        sql.push_bind(user.username);
        if user.password.is_some() && !user.password.clone().unwrap().is_empty() {
            sql.push(",password=").push_bind(user.password);
        }
        sql.push(",email=").push_bind(user.email);
        sql.push(",mobile=").push_bind(user.mobile);
        sql.push(",status=").push_bind(user.status);
        sql.push(",updated_at=").push_bind(user.updated_at);
        sql.push(" WHERE id=").push_bind(user.id);
        println!("sql:{}", sql.sql());
        row = sql.build().execute(get_pool().unwrap()).await?;
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

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<u64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub status: Option<i32>,
    #[serde(skip_deserializing)]
    #[serde(with = "date_format")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_deserializing)]
    #[serde(with = "date_format")]
    pub updated_at: Option<NaiveDateTime>,
}
