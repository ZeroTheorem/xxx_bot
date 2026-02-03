use std::{env, sync::Arc};

use chrono::{Datelike, Local};
use redis::aio::MultiplexedConnection;
use serde::Serialize;
use sqlx::{PgPool, postgres::PgPoolOptions};

const TOTAL_KEY: &str = "total";
const TTL: i64 = 43200;

#[derive(Serialize)]
pub struct Record {
    pub id: i64,
    pub day: i64,
    pub month: i64,
    pub year: i64,
}

pub struct Database {
    pg_pool: PgPool,
    redis_conn: MultiplexedConnection,
}

impl Database {
    pub async fn new() -> Arc<Database> {
        let pg_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();
        let client = redis::Client::open(env::var("REDIS_ADRESS").unwrap()).unwrap();
        let redis_conn = client.get_multiplexed_async_connection().await.unwrap();
        return Arc::new(Database {
            pg_pool: pg_pool,
            redis_conn: redis_conn,
        });
    }
    pub async fn create_table_if_not_exists(&self) {
        sqlx::query!(
            "CREATE TABLE IF NOT EXISTS sex (
            id SERIAL,
            day INTEGER NOT NULL,
            month INTEGER NOT NULL,
            year INTEGER NOT NULL
        );",
        )
        .execute(&self.pg_pool)
        .await
        .unwrap();
    }
    async fn get_total_from_postgres(&self) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM sex;",)
            .fetch_one(&self.pg_pool)
            .await?;

        return Ok(result.count.unwrap_or(0));
    }
    async fn get_from_redis(&self, key: &str) -> Result<Option<i64>, redis::RedisError> {
        redis::cmd("GET")
            .arg(key)
            .query_async(&mut self.redis_conn.clone())
            .await?
    }

    async fn set_to_redis(&self, key: &str, val: i64) -> Result<(), redis::RedisError> {
        redis::cmd("SET")
            .arg(key)
            .arg(val)
            .arg("EX")
            .arg(TTL)
            .exec_async(&mut self.redis_conn.clone())
            .await?;
        Ok(())
    }

    async fn incr_redis_key(&self, key: &str) -> Result<(), redis::RedisError> {
        redis::cmd("INCR")
            .arg(key)
            .exec_async(&mut self.redis_conn.clone())
            .await?;
        Ok(())
    }
    async fn exists_redis_key(&self, key: &str) -> Result<bool, redis::RedisError> {
        let exists: bool = redis::cmd("EXISTS")
            .arg(key)
            .query_async(&mut self.redis_conn.clone())
            .await?;
        return Ok(exists);
    }
    async fn decr_redis_key(&self, key: &str) -> Result<(), redis::RedisError> {
        redis::cmd("DECR")
            .arg(key)
            .exec_async(&mut self.redis_conn.clone())
            .await?;
        Ok(())
    }
    pub async fn get_total(&self) -> Result<i64, sqlx::Error> {
        let redis_result = self.get_from_redis(TOTAL_KEY).await;
        match redis_result {
            Ok(Some(total)) => return Ok(total),
            Ok(None) => {
                let total = self.get_total_from_postgres().await?;
                if let Err(err) = self.set_to_redis(TOTAL_KEY, total).await {
                    tracing::error!("redis error -> {}", err)
                }
                return Ok(total);
            }
            Err(err) => {
                tracing::error!("redis error -> {}", err);
                let total = self.get_total_from_postgres().await?;
                return Ok(total);
            }
        }
    }
    pub async fn get_total_by_month(&self) -> Result<i64, sqlx::Error> {
        let month = Local::now().month() as i32;

        let result = sqlx::query!(
            "SELECT COUNT(*) as total
             FROM sex
             WHERE month = $1;",
            month,
        )
        .fetch_one(&self.pg_pool)
        .await?;
        Ok(result.total.unwrap_or(0))
    }
    pub async fn get_total_by_year(&self) -> Result<i64, sqlx::Error> {
        let year = Local::now().year();

        let result = sqlx::query!(
            "SELECT COUNT(*) as total
             FROM sex
             WHERE year = $1;",
            year,
        )
        .fetch_one(&self.pg_pool)
        .await?;
        Ok(result.total.unwrap_or(0))
    }
    pub async fn get_total_by_certain_month(&self, month: i32) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(
            "SELECT COUNT(*) as total
             FROM sex
             WHERE month = $1;",
            month,
        )
        .fetch_one(&self.pg_pool)
        .await?;
        Ok(result.total.unwrap_or(0))
    }
    pub async fn get_total_by_certain_year(&self, year: i32) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(
            "SELECT COUNT(*) as total
             FROM sex
             WHERE year = $1;",
            year,
        )
        .fetch_one(&self.pg_pool)
        .await?;
        Ok(result.total.unwrap_or(0))
    }
    pub async fn get_last_five_rows(&self) -> Result<Vec<Record>, sqlx::Error> {
        let result = sqlx::query_as!(
            Record,
            "SELECT *
             FROM sex
             ORDER BY id DESC
             LIMIT 5",
        )
        .fetch_all(&self.pg_pool)
        .await?;
        Ok(result)
    }
    pub async fn add_row(&self) -> Result<(), sqlx::Error> {
        let now = Local::now();
        let day = now.day() as i32;
        let month = now.month() as i32;
        let year = now.year();
        sqlx::query!(
            "INSERT INTO sex (day, month, year) VALUES ($1, $2, $3);",
            day,
            month,
            year
        )
        .execute(&self.pg_pool)
        .await?;
        match self.exists_redis_key(TOTAL_KEY).await {
            Ok(true) => {
                if let Err(err) = self.incr_redis_key(TOTAL_KEY).await {
                    tracing::error!("{}", err)
                }
            }
            Ok(false) => (),
            Err(err) => {
                tracing::error!("redis error -> {}", err)
            }
        }
        Ok(())
    }
    pub async fn delete_last_row(&self) -> Result<(), sqlx::Error> {
        let _ = sqlx::query!(
            "DELETE FROM sex
        WHERE id = (SELECT MAX(id) FROM sex)
        RETURNING id",
        )
        .fetch_one(&self.pg_pool)
        .await?;
        match self.exists_redis_key(TOTAL_KEY).await {
            Ok(true) => {
                if let Err(err) = self.decr_redis_key(TOTAL_KEY).await {
                    tracing::error!("redis error -> {}", err)
                }
            }
            Ok(false) => (),
            Err(err) => {
                tracing::error!("redis error -> {}", err)
            }
        }

        Ok(())
    }
}
