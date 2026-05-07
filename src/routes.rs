use actix_web::{Responder, get};

#[get("/")]
pub async fn hello() -> impl Responder {
    // HttpResponse::Ok().body("Hello world!")
    format!("Hello world!")
}

// create a route for a post reqwest that will send in a lat and long and number for crimes
// that will need to run the nearest neighbor function of the r tree and return results.
// but first I have to finish impling traits for the rtree objects so that they can determine distance.
