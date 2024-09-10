use crate::common::date_format;
use crate::common::db::get_pool;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{Error, FromRow, MySql, QueryBuilder};
use tracing::info;

pub async fn get_role_ids(user_ids: Vec<u64>) -> Result<Vec<UserRole>, Error> {
    let mut sql: QueryBuilder<MySql> = QueryBuilder::new("SELECT * FROM user_role WHERE user_id IN (");
    let mut ps = sql.separated(", ");
    for id in user_ids.iter() {
        ps.push_bind(id);
    }
    ps.push_unseparated(") ");
    let urs: Vec<UserRole> = sql.build_query_as().fetch_all(get_pool().unwrap()).await?;
    Ok(urs)
}

pub async fn del_role_ids(role_ids: Vec<u64>) -> Result<(), Error> {
    let mut sql: QueryBuilder<MySql> = QueryBuilder::new("DELETE FROM user_role WHERE role_id IN (");
    let mut ps = sql.separated(", ");
    for id in role_ids.iter() {
        ps.push_bind(id);
    }
    ps.push_unseparated(") ");
    let row = sql.build().execute(get_pool().unwrap()).await?;
    info!("{} rows deleted", row.rows_affected());
    Ok(())
}

pub async fn del_user_ids(user_ids: Vec<u64>) -> Result<(), Error> {
    let mut sql: QueryBuilder<MySql> = QueryBuilder::new("DELETE FROM user_role WHERE user_id IN (");
    let mut ps = sql.separated(", ");
    for id in user_ids.iter() {
        ps.push_bind(id);
    }
    ps.push_unseparated(") ");
    let row = sql.build().execute(get_pool().unwrap()).await?;
    info!("{} rows deleted", row.rows_affected());
    Ok(())
}

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct UserRole {
    pub id: Option<u64>,
    pub user_id: Option<u64>,
    pub role_id: Option<u64>,
    #[serde(skip_deserializing)]
    #[serde(with = "date_format")]
    pub created_at: Option<NaiveDateTime>,
}
