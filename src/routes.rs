use actix_web::web;
use crate::handlers::{self, *};
pub fn init_routes(cfg:&mut web::ServiceConfig) {
    cfg.service(handlers::Handlers::create_user);
    cfg.service(handlers::Handlers::delete_entry);
    cfg.service(handlers::Handlers::user_login);
    cfg.service(handlers::Handlers::get_all);
    cfg.service(handlers::Handlers::write_new);
    cfg.service(handlers::Handlers::index);
    
}