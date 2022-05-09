mod config;
mod db;
mod routes;
mod state;

#[cfg(test)]
mod tests;

use actix_web::{App, HttpServer};
use actix_web_prom::PrometheusMetricsBuilder;
use config::Config;
use sqlx::PgPool;
use state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();

    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    let state = AppState::new(pool).await;

    sqlx::migrate!("./migrations")
        .run(&state.pool)
        .await
        .expect("Migrations failed");

    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .expect("Failed to create prometheus metrics");

    let addr = format!("{}:{}", config.host, config.port);
    println!("Server starting on {}", addr);

    HttpServer::new(move || {
        App::new()
            .configure(routes::init_routes)
            .app_data(state.clone())
            .wrap(prometheus.clone())
    })
    .bind(addr)?
    .run()
    .await
}
