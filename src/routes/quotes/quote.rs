use crate::{db, state::AppState};
use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use std::cmp;

#[derive(Deserialize)]
struct ShowParam {
    show: Option<String>,
}

#[get("/quotes")]
async fn get_quote(show: web::Query<ShowParam>, state: web::Data<AppState>) -> impl Responder {
    let show = show.into_inner().show;

    let quotes = match show {
        Some(show) => db::get_quotes_from_show(&show, 1, &state.pool).await,
        None => db::get_quotes_from_random_show(1, &state.pool).await,
    };

    let quotes = match quotes {
        Ok(quotes) => quotes,
        Err(msg) => return HttpResponse::NotFound().body(msg.to_string()),
    };

    HttpResponse::Ok().json(&quotes[0])
}

#[get("/quotes/{number}")]
async fn get_multiple_quotes(
    show: web::Query<ShowParam>,
    number_of_quotes: web::Path<usize>,
    state: web::Data<AppState>,
) -> impl Responder {
    let number_of_quotes = cmp::min(number_of_quotes.into_inner(), 10) as i64;
    let show = show.into_inner().show;

    let quotes = match show {
        Some(show) => db::get_quotes_from_show(&show, number_of_quotes, &state.pool).await,
        None => db::get_quotes_from_random_show(number_of_quotes as usize, &state.pool).await,
    };

    let quotes = match quotes {
        Ok(quotes) => quotes,
        Err(msg) => return HttpResponse::NotFound().body(msg.to_string()),
    };

    HttpResponse::Ok().json(quotes)
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(get_quote);
    config.service(get_multiple_quotes);
}
