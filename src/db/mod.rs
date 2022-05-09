use futures::future::join_all;
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
    limit: usize,
    pool: &Pool<Postgres>,
) -> Result<Vec<Quote>, sqlx::Error> {
    let mut quotes = Vec::with_capacity(limit);
    for _ in 0..limit {
        quotes.push(_get_quote_from_random_show(pool));
    }

    join_all(quotes).await.into_iter().collect()
}

async fn _get_quote_from_random_show(pool: &Pool<Postgres>) -> Result<Quote, sqlx::Error> {
    let id = sqlx::query_file!("./src/db/get_random_id.sql")
        .fetch_one(pool)
        .await?
        .id;

    let id = match id {
        Some(id) => id,
        None => return Err(sqlx::Error::RowNotFound),
    } as i32;

    sqlx::query_file_as!(Quote, "./src/db/get_quote_by_id.sql", id)
        .fetch_one(pool)
        .await
}

pub async fn get_quotes_from_show(
    show: &str,
    limit: i64,
    pool: &Pool<Postgres>,
) -> Result<Vec<Quote>, sqlx::Error> {
    let result = sqlx::query_file_as!(Quote, "./src/db/get_quote_from_show.sql", show, limit)
        .fetch_all(pool)
        .await?;

    if result.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    Ok(result)
}

pub async fn get_shows(pool: &Pool<Postgres>) -> Result<Vec<String>, sqlx::Error> {
    let shows = sqlx::query_file!("./src/db/get_shows.sql")
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| r.show)
        .collect();

    Ok(shows)
}

pub async fn get_quotes_number(pool: &Pool<Postgres>) -> Result<i32, sqlx::Error> {
    let count = sqlx::query_file!("./src/db/get_quotes_number.sql")
        .fetch_one(pool)
        .await?
        .table_count
        .expect("Count");

    Ok(count)
}
