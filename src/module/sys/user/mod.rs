pub mod model;
pub mod router;

#[cfg(test)]
mod user_test {
    use crate::common::db::{get_pool, init_db_pool};
    use crate::common::req::Page;
    use crate::module::sys::user::model::User;
    use sqlx::types::chrono::Local;
    use sqlx::{Error, MySql, QueryBuilder};

    #[sqlx::test]
    async fn save() -> anyhow::Result<()> {
        init_db_pool().await?;
        let now = Option::from(Local::now().naive_local());
        let user = User {
            id: None,
            username: Option::from(String::from("bbb4")),
            password: Option::from(String::from("bbb")),
            email: Option::from(String::from("bbb")),
            mobile: Option::from(String::from("bbb")),
            status: Option::from(1),
            created_at: None,
            updated_at: None,
        };

        let row = sqlx::query::<MySql>(
            "INSERT INTO user (username,password,email,mobile,status,created_at,updated_at) VALUES ( ?,?,?,?,?,?,? )",
        )
            .bind(user.username)
            .bind(user.password)
            .bind(user.email)
            .bind(user.mobile)
            .bind(user.status)
            .bind(now)
            .bind(now)
            .execute(get_pool().unwrap())
            .await?;
        println!("{} rows inserted", row.rows_affected());
        Ok(())
    }

    #[sqlx::test]
    async fn update() -> anyhow::Result<()> {
        init_db_pool().await?;
        let now = Option::from(Local::now().naive_local());
        let user = User {
            id: Option::from(1),
            username: Option::from(String::from("admin")),
            password: Option::from(String::from("21232f297a57a5a743894a0e4a801fc3")),
            // password: Option::from("".to_string()),
            email: Option::from(String::from("a1")),
            mobile: Option::from(String::from("a1")),
            status: Option::from(1),
            created_at: None,
            updated_at: now,
        };

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
        println!("sql {}", sql.sql());
        let row = sql.build().execute(get_pool().unwrap()).await?;
        println!("{} rows updated", row.rows_affected());
        Ok(())
    }

    #[sqlx::test]
    async fn count() -> anyhow::Result<()> {
        init_db_pool().await?;
        let row: (i32,) = sqlx::query_as("SELECT count(1) FROM user")
            .fetch_one(get_pool().unwrap())
            .await?;
        println!("{}", row.0);
        Ok(())
    }

    #[sqlx::test]
    async fn query_one() {
        init_db_pool().await.unwrap();
        let user: Result<User, Error> = sqlx::query_as("SELECT * FROM user WHERE username = ?")
            .bind("admin")
            .fetch_one(get_pool().unwrap())
            .await;
        match user {
            Ok(u) => {
                println!("{:?}", u)
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    #[sqlx::test]
    async fn query() -> anyhow::Result<()> {
        init_db_pool().await?;
        let page = Page { page: 1, limit: 3 };
        let row: Vec<User> = sqlx::query_as("SELECT * FROM user ORDER BY id DESC LIMIT ? OFFSET ?")
            .bind(page.limit.to_string())
            .bind(page.offset().to_string())
            .fetch_all(get_pool().unwrap())
            .await?;
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
}
