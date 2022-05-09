use crate::{db, state::AppState};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct StatsResponse {
    pub total: usize,
}

#[get("/quotes/stats")]
async fn get_stats(state: web::Data<AppState>) -> impl Responder {
    let total_quotes = db::get_quotes_number(&state.pool).await;

    let total_quotes = match total_quotes {
        Ok(total_quotes) => total_quotes,
        Err(msg) => return HttpResponse::InternalServerError().body(msg.to_string()),
    };

    let resp = json!({ "total": total_quotes });
    HttpResponse::Ok().json(resp)
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(get_stats);
}
