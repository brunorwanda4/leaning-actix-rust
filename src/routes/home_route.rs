use actix_web::web;

use super::handler::home_handler::{home_handler, home_test};

pub fn config(config: &mut web::ServiceConfig) {
    config.service(web::scope("/home").service(home_handler).service(home_test));
}
