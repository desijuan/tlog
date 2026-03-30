use sqlx::{
    Row,
    sqlite::{
        Sqlite, SqliteConnectOptions, SqlitePool, SqlitePoolOptions, SqliteQueryResult, SqliteRow,
    },
};
use std::str::FromStr;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[derive(Clone)]
pub struct DB {
    pool: SqlitePool,
}

impl DB {
    pub async fn new() -> Self {
        let options: SqliteConnectOptions = SqliteConnectOptions::from_str("sqlite://db.sqlite")
            .unwrap()
            .create_if_missing(true)
            .foreign_keys(true);
        let pool: SqlitePool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .expect("Failed to open connection");
        let schema: &'static str = include_str!("db/schema.sql");

        sqlx::raw_sql(schema)
            .execute(&pool)
            .await
            .expect("Failed to initialize schema");

        DB { pool }
    }

    pub async fn add_user(
        &self,
        name: &str,
        pwd_hash: &str,
        email: &str,
    ) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            "INSERT INTO users (name, pwd_hash, email, created_at) VALUES (?, ?, ?, unixepoch())",
        )
        .bind(name)
        .bind(pwd_hash)
        .bind(email)
        .execute(&self.pool)
        .await
    }

    pub async fn get_user_info(&self, id: i64) -> Result<User, sqlx::Error> {
        sqlx::query_as::<Sqlite, User>("SELECT id, name, email FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_user_id_pwd_hash(&self, name: &str) -> Result<(i64, String), sqlx::Error> {
        let row: SqliteRow = sqlx::query("SELECT id, pwd_hash FROM users WHERE name = ?")
            .bind(name)
            .fetch_one(&self.pool)
            .await?;

        Ok((row.get("id"), row.get("pwd_hash")))
    }
}
