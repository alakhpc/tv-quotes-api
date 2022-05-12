use dotenv::dotenv;
use serde_json::Value as JsonValue;
use sqlx::{Executor, PgPool};

#[actix_web::main]
async fn main() {
    dotenv().ok();

    static SEED_STR: &str = include_str!("../tests/db/seed.json");
    let database_url: String =
        std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL not set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    pool.execute(include_str!("../tests/db/reset.sql"))
        .await
        .expect("Database reset failed");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migrations failed");

    let seed_json: JsonValue = serde_json::from_str(SEED_STR).unwrap();

    sqlx::query(include_str!("../tests/db/seed.sql"))
        .bind(seed_json)
        .execute(&pool)
        .await
        .expect("Seed failed");
}
