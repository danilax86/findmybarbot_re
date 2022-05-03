use geo::haversine_distance::HaversineDistance;
use geo::Point;

pub fn calculate_distance(start: &Point<f32>, end: &Point<f32>) -> f32 {
    let meters = start.haversine_distance(end);
    let km = meters / 1000.0;
    km
}

#[cfg(test)]
mod tests {
    use geo::{Point, point};
    use crate::utils::poi::calculate_distance;
    use assert_approx_eq::assert_approx_eq;
    use crate::utils::utils::calculate_distance;

    #[test]
    fn test_distance_calculator() {
        let start = point!(x: -74.006f32, y: 40.7128f32);
        let end = point!(x: -0.1278f32, y: 51.5074f32);

        let result = calculate_distance(&start, &end);

        println!("{}", result);

        assert_approx_eq!(result, 5570.0, 2f32);
    }
}