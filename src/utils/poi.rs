use geo::{Point};
use crate::utils::calculate::calculate_distance;

pub struct Poi {
    pub name: String,
    pub location: Point<f64>,
    pub address: String,
    pub description: String,
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
    use geo::{coord, point};
    use crate::utils::poi::Poi;

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




