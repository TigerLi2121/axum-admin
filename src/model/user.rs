use crate::common::date_format;
use crate::common::db::get_pool;
use crate::common::req::Page;
use crate::common::res::RP;
use crate::model::user_role;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{Error, FromRow, MySql, QueryBuilder};
use tracing::info;

pub async fn get_user(app_id: u64, username: String) -> Result<User, Error> {
    let m = sqlx::query_as::<MySql, User>("SELECT * FROM user WHERE app_id = ? AND username = ? LIMIT 1")
        .bind(app_id)
        .bind(username)
        .fetch_one(get_pool().unwrap())
        .await?;
    Ok(m)
}

pub async fn page(page: Page) -> Result<RP<Vec<User>>, Error> {
    let count: (i32,) = sqlx::query_as("SELECT count(1) FROM user WHERE app_id=?")
        .bind(page.app_id)
        .fetch_one(get_pool().unwrap())
        .await?;
    let mut ms: Vec<User> = vec![];
    if count.0 > 0 {
        ms = sqlx::query_as("SELECT * FROM user WHERE app_id=? ORDER BY id DESC LIMIT ? OFFSET ?")
            .bind(page.app_id)
            .bind(page.limit.to_string())
            .bind(page.offset().to_string())
            .fetch_all(get_pool().unwrap())
            .await?;
        // 获取用户角色id
        let user_ids = ms.iter().map(|e| e.id.unwrap()).collect();
        let urs = user_role::get_role_ids(user_ids).await?;
        for m in ms.iter_mut() {
            m.role_ids = Some(urs
                .iter()
                .filter(|e| e.user_id == m.id)
                .map(|e| e.role_id.unwrap())
                .collect());
        }
    }


    Ok(RP::ok(count.0, ms))
}

pub async fn sou(user: User) -> Result<u64, Error> {
    let row;
    if user.id.is_none() {
        row = sqlx::query::<MySql>(
            "INSERT INTO user (app_id,username,password,email,mobile,status) VALUES (?,?,?,?,?,?)",
        )
            .bind(user.app_id)
            .bind(user.username)
            .bind(user.password)
            .bind(user.email)
            .bind(user.mobile)
            .bind(user.status)
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
    user_role::del_user_ids(ids).await?;
    Ok(())
}

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<u64>,
    pub app_id: Option<u64>,
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
    #[sqlx(skip)]
    pub role_ids: Option<Vec<u64>>,
}
