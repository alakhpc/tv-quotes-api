use crate::{db, state::AppState};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct ShowsResponse {
    pub shows: Vec<String>,
}

#[get("/quotes/shows")]
async fn get_shows(state: web::Data<AppState>) -> impl Responder {
    let show_names = db::get_shows(&state.pool).await;

    let resp = match show_names {
        Ok(show_names) => json!({ "shows": show_names }),
        Err(msg) => return HttpResponse::InternalServerError().body(msg.to_string()),
    };

    HttpResponse::Ok().json(resp)
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(get_shows);
}
