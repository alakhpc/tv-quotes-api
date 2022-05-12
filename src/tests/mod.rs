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
use sqlx::PgPool;

lazy_static! {
    static ref SEED_STR: &'static str = include_str!("./db/seed.json");
    pub static ref QUOTES: Vec<Quote> = serde_json::from_str(&SEED_STR).unwrap();
}

lazy_static! {
    static ref DATABASE_URL: String = std::env::var("TEST_DATABASE_URL").unwrap();
}

pub async fn get_service() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    dotenv().ok();

    let pool = PgPool::connect(&DATABASE_URL)
        .await
        .expect("Failed to connect to database");

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
