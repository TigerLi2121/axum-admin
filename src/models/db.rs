use dotenv;
use once_cell::sync::OnceCell;
use sqlx::{mysql::MySqlPoolOptions, Error, MySqlPool, Row};

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
