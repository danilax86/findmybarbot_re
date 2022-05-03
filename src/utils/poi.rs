use geo::{Point};
use geo::haversine_distance::HaversineDistance;

pub struct Poi {
    pub name: String,
    pub location: Point<f64>,
    pub address: String,
    pub description: String,
}

pub fn calculate_distance(start: &Point<f64>, end: &Point<f64>) -> f64 {
    let meters = start.haversine_distance(end);
    let km = meters / 1000.0;

    km
}

impl Poi {
    fn calculate_distance_from_user(&self, user_location: &Point<f64>) -> f64 {
        let poi_location = &self.location;

        calculate_distance(&poi_location, &user_location)
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.parse().unwrap();
    }

    fn set_address(&mut self, address: &str) {
        self.address = address.parse().unwrap();
    }

    fn set_description(&mut self, description: &str) {
        self.description = description.parse().unwrap();
    }
}


#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use geo::point;
    use crate::Poi;
    use crate::utils::poi::calculate_distance;

    #[test]
    fn test_distance_calculator() {
        let start = point!(x: -74.006, y: 40.7128);
        let end = point!(x: -0.1278, y: 51.5074);

        let result = calculate_distance(&start, &end);

        println!("{}", result);

        assert_approx_eq!(result, 5570.0, 2f64);
    }

    #[test]
    fn test_set_name() {
        let mut a = Poi {
            name: "London".to_string(),
            location: point!(x: -74.006, y: 40.7128),

            address: "adad".to_string(),
            description: "Описание".to_string(),
        };

        a.set_name("Nelondon");

        assert_eq!(a.name, "Nelondon");
    }
}




