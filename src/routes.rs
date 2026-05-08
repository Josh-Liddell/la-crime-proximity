use crate::crime::{AppState, CrimeRecord};
use actix_web::{HttpResponse, Responder, get, web};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Location {
    lat: f64,
    lon: f64,
    count: usize,
}

#[get("/proximity")]
async fn proximity(data: web::Data<AppState>, query: web::Query<Location>) -> impl Responder {
    let results: Vec<&CrimeRecord> = data
        .crimes
        .nearest_neighbor_iter(&[query.lat, query.lon])
        .take(query.count)
        .collect();

    HttpResponse::Ok().json(results)
}
