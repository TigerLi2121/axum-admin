use crate::common::date_format;
use crate::common::db::get_pool;
use crate::common::req::Page;
use crate::common::res::RP;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{Error, FromRow, MySql, QueryBuilder};
use tracing::info;

pub async fn page(page: Page) -> Result<RP<Vec<App>>, Error> {
    let count: (i32,) = sqlx::query_as("SELECT count(1) FROM app")
        .fetch_one(get_pool().unwrap())
        .await?;
    let mut ms: Vec<App> = vec![];
    if count.0 > 0 {
        ms = sqlx::query_as("SELECT * FROM app ORDER BY id DESC LIMIT ? OFFSET ?")
            .bind(page.limit.to_string())
            .bind(page.offset().to_string())
            .fetch_all(get_pool().unwrap())
            .await?;
    }
    Ok(RP::ok(count.0, ms))
}

pub async fn sou(app: App) -> Result<u64, Error> {
    let row;
    if app.id.is_none() {
        row = sqlx::query::<MySql>(
            "INSERT INTO app (name) VALUES (?)",
        )
            .bind(app.name)
            .execute(get_pool().unwrap())
            .await?;
        info!("{} rows inserted", row.rows_affected());
    } else {
        let mut sql: QueryBuilder<MySql> = QueryBuilder::new("UPDATE app SET name=");
        sql.push_bind(app.name);
        sql.push(" WHERE id=").push_bind(app.id);
        println!("sql:{}", sql.sql());
        row = sql.build().execute(get_pool().unwrap()).await?;
        info!("{} rows updated", row.rows_affected());
    }
    Ok(row.last_insert_id())
}

pub async fn del(ids: Vec<u64>) -> Result<(), Error> {
    let mut sql: QueryBuilder<MySql> = QueryBuilder::new("DELETE FROM app WHERE id IN (");
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
pub struct App {
    pub id: Option<u64>,
    pub name: Option<String>,
    #[serde(skip_deserializing)]
    #[serde(with = "date_format")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_deserializing)]
    #[serde(with = "date_format")]
    pub updated_at: Option<NaiveDateTime>,
}
