use actix_web::web;
use crate::handler_hello;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(handler_hello);
}