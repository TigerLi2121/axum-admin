use dotenv;
use once_cell::sync::OnceCell;
use sqlx::{mysql::MySqlPoolOptions, Error, MySql, MySqlPool, QueryBuilder};
use tracing::info;

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
async fn user_batch_del() -> anyhow::Result<()> {
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
