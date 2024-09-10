use crate::common::date_format;
use crate::common::db::get_pool;
use crate::common::req::Page;
use crate::common::res::RP;
use crate::model::{role_menu, user_role};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{Error, FromRow, MySql, QueryBuilder};
use tracing::info;

pub async fn page(page: Page) -> Result<RP<Vec<Role>>, Error> {
    let count: (i32,) = sqlx::query_as("SELECT count(1) FROM role WHERE app_id = ?")
        .bind(page.app_id)
        .fetch_one(get_pool().unwrap())
        .await?;
    let mut ms: Vec<Role> = vec![];
    if count.0 > 0 {
        ms = sqlx::query_as("SELECT * FROM role WHERE app_id = ? ORDER BY id DESC LIMIT ? OFFSET ?")
            .bind(page.app_id)
            .bind(page.limit.to_string())
            .bind(page.offset().to_string())
            .fetch_all(get_pool().unwrap())
            .await?;
        let ids = ms.iter().map(|e| e.id.unwrap()).collect();
        let rms = role_menu::get_menu_ids(ids).await?;
        for m in ms.iter_mut() {
            m.menu_ids = Some(rms.iter()
                .filter(|e| e.role_id == m.id)
                .map(|e| e.menu_id.unwrap())
                .collect());
        }
    }
    Ok(RP::ok(count.0, ms))
}

pub async fn sou(role: Role) -> Result<u64, Error> {
    let row;
    if role.id.is_none() {
        row = sqlx::query::<MySql>(
            "INSERT INTO role (app_id,name) VALUES (?,?)",
        )
            .bind(role.app_id)
            .bind(role.name)
            .execute(get_pool().unwrap())
            .await?;
        info!("{} rows inserted", row.rows_affected());
    } else {
        let mut sql: QueryBuilder<MySql> = QueryBuilder::new("UPDATE role SET app_id=");
        sql.push_bind(role.app_id);
        sql.push(" ,name=").push_bind(role.name);
        sql.push(" WHERE id=").push_bind(role.id);
        println!("sql:{}", sql.sql());
        row = sql.build().execute(get_pool().unwrap()).await?;
        info!("{} rows updated", row.rows_affected());
    }
    Ok(row.last_insert_id())
}

pub async fn del(ids: Vec<u64>) -> Result<(), Error> {
    let mut sql: QueryBuilder<MySql> = QueryBuilder::new("DELETE FROM role WHERE id IN (");
    let mut ps = sql.separated(", ");
    for id in ids.iter() {
        ps.push_bind(id);
    }
    ps.push_unseparated(") ");
    let row = sql.build().execute(get_pool().unwrap()).await?;
    info!("{} rows deleted", row.rows_affected());
    user_role::del_role_ids(ids.clone()).await?;
    role_menu::del_role_ids(ids).await?;
    Ok(())
}

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct Role {
    pub id: Option<u64>,
    pub app_id: Option<u64>,
    pub name: Option<String>,
    #[serde(skip_deserializing)]
    #[serde(with = "date_format")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_deserializing)]
    #[serde(with = "date_format")]
    pub updated_at: Option<NaiveDateTime>,
    #[sqlx(skip)]
    pub menu_ids: Option<Vec<u64>>,
}
