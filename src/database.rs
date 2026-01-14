use chrono::Datelike;
use chrono::Local;
use sqlx::{Pool, Sqlite, sqlite::SqlitePool};
use std::env;

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

pub async fn get_all_by_month(pool: &SqlitePool) -> i64 {
    let month = Local::now().month();

    let result = sqlx::query!(
        "SELECT COUNT(*) as count
        FROM sex
        WHERE month = ?;",
        month,
    )
    .fetch_one(pool)
    .await
    .unwrap();
    result.count
}
pub async fn get_all(pool: &SqlitePool) -> i64 {
    let result = sqlx::query!(
        "SELECT COUNT(*) as count
        FROM sex;",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    result.count
}
pub async fn get_all_by_year(pool: &SqlitePool) -> i64 {
    let year = Local::now().year();

    let result = sqlx::query!(
        "SELECT COUNT(*) as count
        FROM sex
        WHERE year = ?;",
        year,
    )
    .fetch_one(pool)
    .await
    .unwrap();
    result.count
}

pub async fn get_all_by_certain_year(pool: &SqlitePool, year: i64) -> i64 {
    let result = sqlx::query!(
        "SELECT COUNT(*) as count
        FROM sex
        WHERE year = ?;",
        year,
    )
    .fetch_one(pool)
    .await
    .unwrap();
    result.count
}

pub async fn get_all_by_certain_month(pool: &SqlitePool, month: i64) -> i64 {
    let result = sqlx::query!(
        "SELECT COUNT(*) as count
        FROM sex
        WHERE month = ?;",
        month,
    )
    .fetch_one(pool)
    .await
    .unwrap();
    result.count
}
pub async fn add_row(pool: &SqlitePool) {
    let now = Local::now();
    let month = now.month();
    let year = now.year();
    sqlx::query!("INSERT INTO sex (month, year) VALUES (?, ?);", month, year)
        .execute(pool)
        .await
        .unwrap();
}
