use geo::haversine_distance::HaversineDistance;
use geo::Point;

pub fn calculate_distance(start: &Point<f64>, end: &Point<f64>) -> f64 {
    let meters = start.haversine_distance(end);
    let km = meters / 1000.0;
    km
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use geo::point;
    use crate::utils::calculate::calculate_distance;

    #[test]
    fn test_distance_calculator() {
        let start = point!(x: -74.006, y: 40.7128);
        let end = point!(x: -0.1278, y: 51.5074);

        let result = calculate_distance(&start, &end);

        println!("{}", result);

        assert_approx_eq!(result, 5570.0, 2f64);
    }
}