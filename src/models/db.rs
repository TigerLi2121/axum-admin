use crate::common::req::Page;
use crate::models::user::User;
use dotenv;
use once_cell::sync::OnceCell;
use sqlx::types::chrono::Local;
use sqlx::{mysql::MySqlPoolOptions, Error, MySql, MySqlPool, QueryBuilder};
use time::OffsetDateTime;

static MYSQL_POOL: OnceCell<MySqlPool> = OnceCell::new();
//建立mysql连接
pub async fn init_db_pool() -> Result<(), Error> {
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    assert!(MYSQL_POOL.set(pool).is_ok());
    Ok(())
}

//获取数据库
pub fn get_pool() -> Option<&'static MySqlPool> {
    MYSQL_POOL.get()
}

#[sqlx::test]
async fn save() -> anyhow::Result<()> {
    init_db_pool().await?;
    let user = User {
        id: None,
        username: Option::from(String::from("bbb3")),
        password: Option::from(String::from("bbb")),
        email: Option::from(String::from("bbb")),
        mobile: Option::from(String::from("bbb")),
        status: Option::from(1),
        created_at: None,
        updated_at: None,
    };

    let row = sqlx::query::<MySql>(
        "INSERT INTO user (username,password,email,mobile,status) VALUES ( ?,?,?,?,? )",
    )
        .bind(user.username)
        .bind(user.password)
        .bind(user.email)
        .bind(user.mobile)
        .bind(user.status)
        .execute(get_pool().unwrap())
        .await?;
    println!("{} rows inserted", row.rows_affected());
    Ok(())
}

#[sqlx::test]
async fn update() -> anyhow::Result<()> {
    init_db_pool().await?;
    let user = User {
        id: Option::from(5),
        username: Option::from(String::from("bbb3")),
        password: Option::from(String::from("bbb")),
        email: Option::from(String::from("bbb")),
        mobile: Option::from(String::from("bbb")),
        status: Option::from(1),
        created_at: None,
        updated_at: Option::from(Local::now().naive_local()),
    };

    let row = sqlx::query::<MySql>(
        "UPDATE user SET username=?,password=?,email=?,mobile=?,status=?,updated_at=? WHERE id=?",
    )
        .bind(user.username)
        .bind(user.password)
        .bind(user.email)
        .bind(user.mobile)
        .bind(user.status)
        .bind(user.updated_at)
        .bind(user.id)
        .execute(get_pool().unwrap())
        .await?;
    println!("{} rows updated", row.rows_affected());
    Ok(())
}

#[sqlx::test]
async fn count() -> anyhow::Result<()> {
    init_db_pool().await?;
    let row = sqlx::query_as("SELECT count(1) FROM user")
        .fetch_one(get_pool().unwrap()).await?;
    println!("{:?}", row);
    Ok(())
}

#[sqlx::test]
async fn query() -> anyhow::Result<()> {
    init_db_pool().await?;
    let page = Page { page: 1, limit: 3 };
    let row: Vec<User> = sqlx::query_as("SELECT * FROM user ORDER BY id DESC LIMIT ? OFFSET ?")
        .bind(page.limit.to_string())
        .bind(page.offset().to_string())
        .fetch_all(get_pool().unwrap()).await?;
    println!("{:?}", row);
    Ok(())
}

#[sqlx::test]
async fn batch_del() -> anyhow::Result<()> {
    init_db_pool().await?;
    let ids = vec![3, 4];
    let mut sql: QueryBuilder<MySql> = QueryBuilder::new("DELETE FROM user WHERE id IN (");
    let mut ms = sql.separated(", ");
    for tn in ids.iter() {
        ms.push_bind(tn);
    }
    ms.push_unseparated(") ");
    let row = sql.build().execute(get_pool().unwrap()).await?;

    println!("{} rows deleted", row.rows_affected());
    Ok(())
}
