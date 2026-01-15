use chrono::Datelike;
use chrono::Local;
use serde::Serialize;
use sqlx::{Pool, Sqlite, sqlite::SqlitePool};
use std::env;

#[derive(Serialize)]
pub struct Record {
    pub id: i64,
    pub month: i64,
    pub year: i64,
}

pub async fn get_pool() -> Pool<Sqlite> {
    SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap()
}

pub async fn create_table_if_not_exists(pool: &SqlitePool) {
    sqlx::query_as!(
        User,
        "CREATE TABLE IF NOT EXISTS sex (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            month INTEGER NOT NULL,
            year INTEGER NOT NULL
        );"
    )
    .execute(pool)
    .await
    .unwrap();
}

pub async fn get_all_by_month(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    let month = Local::now().month();

    let result = sqlx::query!(
        "SELECT COUNT(*) as count
        FROM sex
        WHERE month = ?;",
        month,
    )
    .fetch_one(pool)
    .await?;
    Ok(result.count)
}
pub async fn get_all(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT COUNT(*) as count
        FROM sex;",
    )
    .fetch_one(pool)
    .await?;
    Ok(result.count)
}

pub async fn delete_last_row(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM sex
        WHERE id = (SELECT MAX(id) FROM sex);",
    )
    .execute(pool)
    .await?;
    Ok(())
}
pub async fn get_all_by_year(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    let year = Local::now().year();

    let result = sqlx::query!(
        "SELECT COUNT(*) as count
        FROM sex
        WHERE year = ?;",
        year,
    )
    .fetch_one(pool)
    .await?;
    Ok(result.count)
}

pub async fn get_all_by_certain_year(pool: &SqlitePool, year: i64) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT COUNT(*) as count
        FROM sex
        WHERE year = ?;",
        year,
    )
    .fetch_one(pool)
    .await?;
    Ok(result.count)
}

pub async fn get_last_five_rows(pool: &SqlitePool) -> Result<Vec<Record>, sqlx::Error> {
    let result = sqlx::query_as!(
        Record,
        "SELECT *
        FROM sex
        ORDER BY id DESC
        LIMIT 5
        ",
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}

pub async fn get_all_by_certain_month(pool: &SqlitePool, month: i64) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT COUNT(*) as count
        FROM sex
        WHERE month = ?;",
        month,
    )
    .fetch_one(pool)
    .await?;
    Ok(result.count)
}
pub async fn add_row(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let now = Local::now();
    let month = now.month();
    let year = now.year();
    sqlx::query!("INSERT INTO sex (month, year) VALUES (?, ?);", month, year)
        .execute(pool)
        .await?;
    Ok(())
}
