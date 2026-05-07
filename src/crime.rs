use rstar::{AABB, RTreeObject};
use serde::Deserialize;

// needs to impl r tree object or something
#[derive(Deserialize, Debug)]
pub struct CrimeRecord {
    #[serde(rename = "LAT")]
    pub lat: f64,
    #[serde(rename = "LON")]
    pub lon: f64,
}

// have to work on understanding the below
// --------------------------------------------

// maybe need to add more, not sure
impl RTreeObject for CrimeRecord {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_point([self.lat, self.lon])
    }
}

// impl PointDistance for CrimeRecord
// {
//     fn distance_2(&self, point: &[f32; 2]) -> f32
//     {
//         let d_x = self.origin[0] - point[0];
//         let d_y = self.origin[1] - point[1];
//         let distance_to_origin = (d_x * d_x + d_y * d_y).sqrt();
//         let distance_to_ring = distance_to_origin - self.radius;
//         let distance_to_circle = f32::max(0.0, distance_to_ring);
//         // We must return the squared distance!
//         distance_to_circle * distance_to_circle
//     }

// }
