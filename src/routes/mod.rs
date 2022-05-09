use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

pub mod quotes;

#[get("/")]
async fn index() -> impl Responder {
    let resp = json!({
            "name": "TV Quotes API",
            "docs": "https://github.com/alakhpc/tv-quotes-api"
        }
    );

    HttpResponse::Ok().json(resp)
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(index);
    quotes::init_routes(config);
}
