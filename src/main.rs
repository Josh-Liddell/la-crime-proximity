mod crime;
mod routes;

use actix_web::{App, HttpServer, middleware::Logger};
use anyhow::Result;
use log::info;

#[actix_web::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let crimes = crime::load_crime_data()?;

    info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(crimes.clone())
            .service(routes::proximity)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
