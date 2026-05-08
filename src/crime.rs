use actix_web::web;
use anyhow::Result;
use csv::ReaderBuilder;
use log::{info, warn};
use rstar::{AABB, PointDistance, RTree, RTreeObject};
use serde::{Deserialize, Serialize};

// needs to impl r tree object or something
#[derive(Deserialize, Serialize, Debug)]
pub struct CrimeRecord {
    #[serde(alias = "DATE OCC")]
    pub date: String,
    #[serde(alias = "Premis Desc")]
    pub premis: Option<String>,
    #[serde(alias = "Weapon Desc")]
    pub weapon: Option<String>,
    #[serde(alias = "Crm Cd Desc")]
    pub description: String,
    #[serde(alias = "LAT")]
    pub lat: f64,
    #[serde(alias = "LON")]
    pub lon: f64,
}

impl RTreeObject for CrimeRecord {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.lat, self.lon])
    }
}

// its not taking into account the sphere of the earth
impl PointDistance for CrimeRecord {
    fn distance_2(&self, point: &[f64; 2]) -> f64 {
        let dx = self.lat - point[0];
        let dy = self.lon - point[1];
        (dx * dx) + (dy * dy)
    }
}

pub struct AppState {
    pub crimes: RTree<CrimeRecord>,
}

pub fn load_crime_data() -> Result<web::Data<AppState>> {
    info!("Loading crime data...");
    let mut rdr = ReaderBuilder::new().from_path("data/Crime_Data_from_2020_to_2024.csv")?;

    let records: Vec<CrimeRecord> = rdr
        .deserialize()
        .filter_map(|res| match res {
            Ok(record) => Some(record),
            Err(e) => {
                warn!("Skipping invalid row: {}", e);
                None
            }
        })
        .collect();

    let tree = RTree::bulk_load(records);
    info!("Finished loading: {} rows", tree.size());

    Ok(web::Data::new(AppState { crimes: tree }))
}
