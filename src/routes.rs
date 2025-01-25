use actix_web::web;
use crate::handlers::users_handler::{create_user_handler, test_user};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user_handler);
    cfg.service(test_user);
       
}