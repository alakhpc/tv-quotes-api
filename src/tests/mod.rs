mod quotes;
mod shows;
mod stats;

use crate::{db::Quote, routes, state::AppState};
use actix_http::Request;
use actix_web::{
    dev::{Service, ServiceResponse},
    error::Error,
    test, App,
};
use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;
use sqlx::{Executor, PgPool, Pool, Postgres};
use std::sync::Mutex;

lazy_static! {
    static ref SEED_STR: &'static str = include_str!("./db/seed.json");
    pub static ref QUOTES: Vec<Quote> = serde_json::from_str(&SEED_STR).unwrap();
}

lazy_static! {
    static ref DATABASE_URL: String = std::env::var("TEST_DATABASE_URL").unwrap();
    static ref POOL: Mutex<Option<Pool<Postgres>>> = Mutex::new(None);
}

pub async fn get_service() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    dotenv().ok();

    let pool = {
        let mut pool = POOL.lock().unwrap();
        if pool.is_none() {
            *pool = Some(get_pool().await);
        }
        pool.clone().unwrap()
    };

    let state = AppState::new(pool).await;
    test::init_service(App::new().configure(routes::init_routes).app_data(state)).await
}

pub async fn test_get<T>(
    app: &impl Service<Request, Response = ServiceResponse, Error = Error>,
    route: &str,
) -> (u16, T)
where
    T: DeserializeOwned,
{
    let req = test::TestRequest::get().uri(route).to_request();

    let resp = test::call_service(&app, req).await;

    println!("{:?}", resp);

    let status = resp.status().as_u16();
    let json_body = test::read_body_json(resp).await;

    (status, json_body)
}

async fn get_pool() -> Pool<Postgres> {
    let pool = PgPool::connect(&DATABASE_URL)
        .await
        .expect("Failed to connect to database");

    pool.execute(include_str!("./db/reset.sql"))
        .await
        .expect("Database reset failed");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Migrations failed");

    let seed_json: JsonValue = serde_json::from_str(&SEED_STR).unwrap();

    sqlx::query(include_str!("./db/seed.sql"))
        .bind(seed_json)
        .execute(&pool)
        .await
        .expect("Seed failed");

    pool
}
