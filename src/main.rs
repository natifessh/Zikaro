
use std::sync::{Arc, Mutex};

use actix_cors::Cors;

use body::BoxBody;
use dev::Service;
use env_logger::Env;
use http::header::ContentType;
use middleware::Logger;
use models::{Diary, User};
use handlers::Handlers::{create_user, delete_entry, get_all, index, user_login, write_new};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use actix_web::*;
use futures_util::{future::FutureExt};
use bcrypt::{DEFAULT_COST, hash, verify};
use web::delete;
mod handlers;
mod models;
mod routes;

//idk why i wrote this
impl Responder for Diary{
    type Body = BoxBody;
    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body=serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body)

        
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   std::env::set_var("RUST_LOG", "actix_web=info");
   env_logger::init_from_env(Env::default().default_filter_or("mehhh"));

    let conn = Arc::new(Mutex::new(Connection::open("personalDi.db").unwrap()));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:5500")
            .allowed_methods(vec!["GET", "POST","DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(conn.clone()))
            
            .wrap(cors)
            .wrap(Logger::default()) 
            .configure(routes::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}