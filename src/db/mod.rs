use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Quote {
    #[serde(skip)]
    pub id: i32,
    pub show: String,
    pub character: String,
    pub text: String,
}

pub async fn get_quotes_from_random_show(
    limit: i64,
    short: bool,
    pool: &Pool<Postgres>,
) -> Result<Vec<Quote>> {
    let quotes = if short {
        sqlx::query_file_as!(Quote, "./src/db/get_quotes_short.sql", limit)
            .fetch_all(pool)
            .await?
    } else {
        sqlx::query_file_as!(Quote, "./src/db/get_quotes.sql", limit)
            .fetch_all(pool)
            .await?
    };

    Ok(quotes)
}

pub async fn get_quotes_from_show(
    show: &str,
    limit: i64,
    short: bool,
    pool: &Pool<Postgres>,
) -> Result<Vec<Quote>> {
    let result = if short {
        sqlx::query_file_as!(
            Quote,
            "./src/db/get_quotes_from_show_short.sql",
            show,
            limit
        )
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_file_as!(Quote, "./src/db/get_quotes_from_show.sql", show, limit)
            .fetch_all(pool)
            .await?
    };

    if result.is_empty() {
        bail!("No quotes found for show {}", show);
    }

    Ok(result)
}

pub async fn get_shows(pool: &Pool<Postgres>) -> Result<Vec<String>> {
    let shows = sqlx::query_file!("./src/db/get_shows.sql")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| r.show)
        .collect();

    Ok(shows)
}

pub async fn get_quotes_number(pool: &Pool<Postgres>) -> Result<i32> {
    let count = sqlx::query_file!("./src/db/get_quotes_number.sql")
        .fetch_one(pool)
        .await?
        .table_count
        .expect("Count");

    Ok(count)
}
