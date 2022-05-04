use geo::haversine_distance::HaversineDistance;
use geo::Point;

pub fn calculate_distance(start: &Point<f64>, end: &Point<f64>) -> f64 {
    let meters = start.haversine_distance(end);
    let km = meters / 1000.0;

    km
}