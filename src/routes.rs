use actix_web::web;
use crate::handlers::{
        users_handler::{create_user_handler, test_user},
        stocks_handler::get_stocks
    };

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .service(create_user_handler)
            .service(test_user)
            .service(get_stocks)
    );
       
}