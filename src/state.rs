use actix_web::web;
use sqlx::{Pool, Postgres};

pub struct AppState {
    pub pool: Pool<Postgres>,
}

impl AppState {
    pub async fn new(pool: Pool<Postgres>) -> web::Data<Self> {
        let state = AppState { pool };

        web::Data::new(state)
    }
}
