// on startup read the csv historical data into memeory
mod crime;
mod routes;

use actix_web::{App, HttpServer, middleware::Logger, web};
use crime::CrimeRecord;
use csv::ReaderBuilder;
use log::info;
use rstar::RTree;
use std::sync::Mutex;

struct AppState {
    crimes: Mutex<RTree<CrimeRecord>>,
}

// try getting a few crime records into a r tree
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();

    // work on getting an r tree of crime records and see if you can look at it or something.
    info!("Importing crime data...");
    let mut rdr = ReaderBuilder::new()
        .from_path("data/crime_smaller.csv")
        .unwrap(); // later instead of from path I will get it from the download

    let records: Vec<CrimeRecord> = rdr.deserialize().flatten().collect();
    let tree = RTree::bulk_load(records);
    info!("Finished, size of rtree: {}", tree.size());

    // start a server with the r tree
    let crimes = web::Data::new(AppState {
        crimes: Mutex::new(tree),
    });

    info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(crimes.clone()) // later just referred to as data
            // data: web::Data<AppState>
            // let mut crimes = data.crimes.lock().unwrap()
            .service(routes::hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
