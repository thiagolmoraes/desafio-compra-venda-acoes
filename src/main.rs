use actix_web::{App, HttpServer};
use anyhow::{Result, Context};
use compra_venda_acoes::configure;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
          .configure(configure)
    })
    .bind("0.0.0.0:8080")
    .context("Falha ao vincular à porta 8080")?
    .run()
    .await
    .context("Falha ao iniciar o servidor")?;

    Ok(())

}