use actix_web::{App, HttpServer, middleware::Logger, web};
use diesel::SqliteConnection;
use diesel::r2d2::ConnectionManager;
use anyhow::{Result, Context};
use dotenvy::dotenv;
use std::env;
use env_logger::Env;
use compra_venda_acoes::{init_routes, DbPool, SayHi};


pub fn establish_connection() -> DbPool {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    
    DbPool::builder()
        .build(manager)
        .expect("Pool created successfully")
}

#[actix_web::main]
async fn main() -> Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let pool = establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::new("%a %r %s"))
            .wrap(SayHi)
            .configure(init_routes)
    })
    .bind("0.0.0.0:8080")
    .context("Falha ao vincular à porta 8080")?
    .run()
    .await
    .context("Falha ao iniciar o servidor")?;

    Ok(())

}