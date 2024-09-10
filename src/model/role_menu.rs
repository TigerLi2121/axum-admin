use crate::common::date_format;
use crate::common::db::get_pool;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{Error, FromRow, MySql, QueryBuilder};
use tracing::info;

pub async fn get_menu_ids(role_ids: Vec<u64>) -> Result<Vec<RoleMenu>, Error> {
    let mut sql: QueryBuilder<MySql> = QueryBuilder::new("SELECT * FROM role_menu WHERE role_id IN (");
    let mut ps = sql.separated(", ");
    for id in role_ids.iter() {
        ps.push_bind(id);
    }
    ps.push_unseparated(") ");
    let rms: Vec<RoleMenu> = sql.build_query_as().fetch_all(get_pool().unwrap()).await?;
    Ok(rms)
}
pub async fn del_role_ids(role_ids: Vec<u64>) -> Result<(), Error> {
    let mut sql: QueryBuilder<MySql> = QueryBuilder::new("DELETE FROM role_menu WHERE role_id IN (");
    let mut ps = sql.separated(", ");
    for id in role_ids.iter() {
        ps.push_bind(id);
    }
    ps.push_unseparated(") ");
    let row = sql.build().execute(get_pool().unwrap()).await?;
    info!("{} rows deleted", row.rows_affected());
    Ok(())
}

pub async fn del_menu_ids(user_ids: Vec<u64>) -> Result<(), Error> {
    let mut sql: QueryBuilder<MySql> = QueryBuilder::new("DELETE FROM role_menu WHERE menu_id IN (");
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
pub struct RoleMenu {
    pub id: Option<u64>,
    pub role_id: Option<u64>,
    pub menu_id: Option<u64>,
    #[serde(skip_deserializing)]
    #[serde(with = "date_format")]
    pub created_at: Option<NaiveDateTime>,
}
