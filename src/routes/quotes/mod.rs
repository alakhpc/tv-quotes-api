pub mod quote;
pub mod shows;
pub mod stats;

use actix_web::web;

pub fn init_routes(config: &mut web::ServiceConfig) {
    shows::init_routes(config);
    stats::init_routes(config);
    quote::init_routes(config);
}
