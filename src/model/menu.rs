use crate::common::date_format;
use crate::common::db::get_pool;
use crate::model::role_menu;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{Error, FromRow, MySql, QueryBuilder};
use tracing::info;

pub async fn list(app_id: u64) -> Result<Vec<Menu>, Error> {
    let ms: Vec<Menu> = sqlx::query_as("SELECT * FROM menu WHERE app_id = ? ORDER BY sort DESC")
        .bind(app_id)
        .fetch_all(get_pool().unwrap())
        .await?;
    Ok(ms)
}

pub async fn sou(menu: Menu) -> Result<u64, Error> {
    let row;
    if menu.id.is_none() {
        row = sqlx::query::<MySql>(
            "INSERT INTO menu (pid,app_id,name,path,component,icon,perms,type,sort,status) VALUES (?,?,?,?,?,?,?,?,?,?)",
        )
            .bind(menu.pid)
            .bind(menu.app_id)
            .bind(menu.name)
            .bind(menu.path)
            .bind(menu.component)
            .bind(menu.icon)
            .bind(menu.perms)
            .bind(menu.r#type)
            .bind(menu.sort)
            .bind(menu.status)
            .execute(get_pool().unwrap())
            .await?;
        info!("{} rows inserted", row.rows_affected());
    } else {
        let mut sql: QueryBuilder<MySql> = QueryBuilder::new("UPDATE menu SET pid=");
        sql.push_bind(menu.pid);
        sql.push(" ,name=").push_bind(menu.name);
        sql.push(" ,path=").push_bind(menu.path);
        sql.push(" ,component=").push_bind(menu.component);
        sql.push(" ,icon=").push_bind(menu.icon);
        sql.push(" ,perms=").push_bind(menu.perms);
        sql.push(" ,type=").push_bind(menu.r#type);
        sql.push(" ,sort=").push_bind(menu.sort);
        sql.push(" ,status=").push_bind(menu.status);
        sql.push(" WHERE id=").push_bind(menu.id);
        println!("sql:{}", sql.sql());
        row = sql.build().execute(get_pool().unwrap()).await?;
        info!("{} rows updated", row.rows_affected());
    }
    Ok(row.last_insert_id())
}

pub async fn del(ids: Vec<u64>) -> Result<(), Error> {
    let mut sql: QueryBuilder<MySql> = QueryBuilder::new("DELETE FROM menu WHERE id IN (");
    let mut ps = sql.separated(", ");
    for id in ids.iter() {
        ps.push_bind(id);
    }
    ps.push_unseparated(") ");
    let row = sql.build().execute(get_pool().unwrap()).await?;
    info!("{} rows deleted", row.rows_affected());
    role_menu::del_menu_ids(ids).await?;
    Ok(())
}

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct Menu {
    pub id: Option<u64>,
    pub pid: Option<u64>,
    pub app_id: Option<u64>,
    pub name: Option<String>,
    pub path: Option<String>,
    pub component: Option<String>,
    pub icon: Option<String>,
    pub perms: Option<String>,
    pub r#type: Option<i32>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
    #[serde(skip_deserializing)]
    #[serde(with = "date_format")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_deserializing)]
    #[serde(with = "date_format")]
    pub updated_at: Option<NaiveDateTime>,
    #[sqlx(skip)]
    pub children: Option<Vec<Menu>>,
}
