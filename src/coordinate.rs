use crate::utils::{linear_divisor, EARTH_RADIUS_KM};
use crate::DistanceUnit;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
/// ## Summary
/// Struct representing a latlon coordinate
pub struct Coordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl Coordinate {
    /// # Summary
    /// Construct a new Coordinate
    ///
    /// ## Example
    /// ```rust
    /// use geolocation_utils::Coordinate;
    ///
    /// let coordinate = Coordinate::new(34.8, -2.8);
    /// ```
    pub fn new(lat: f64, lon: f64) -> Self {
        Self {
            latitude: lat,
            longitude: lon,
        }
    }

    /// # Summary
    /// Checks if a coordinate is within the radius of another coordinate.
    ///
    /// ## Notes
    /// - Uses the Haversine formula
    /// - Implementation taken from https://www.geeksforgeeks.org/program-distance-two-points-earth/
    ///
    /// ## Example
    /// ```rust
    /// use geolocation_utils::{Coordinate, DistanceUnit};
    ///
    /// let coordinate = Coordinate::new(0.0, 0.0);
    /// let coordinate2 = Coordinate::new(1.0, 1.0);
    /// let radius: f64 = 150.0;
    /// let radius_unit = DistanceUnit::Miles;
    ///
    /// let is_in_radius = coordinate.in_radius(&coordinate2, radius, &radius_unit);
    /// assert_eq!(true, is_in_radius);
    /// ```
    pub fn in_radius(
        &self,
        other_coordinate: &Coordinate,
        radius: f64,
        distance_unit: &DistanceUnit,
    ) -> bool {
        // Formula from https://www.geeksforgeeks.org/program-distance-two-points-earth/
        let pi = std::f64::consts::PI;

        let lat1 = self.latitude * pi / 180.0;
        let lat2 = other_coordinate.latitude * pi / 180.0;
        let lon1 = self.longitude * pi / 180.0;
        let lon2 = other_coordinate.longitude * pi / 180.0;

        let d_lon = lon2 - lon1;
        let d_lat = lat2 - lat1;

        let a = (((d_lat / 2.0).sin()).powi(2))
            + (lat1.cos() * lat2.cos()) * ((d_lon / 2.0).sin()).powi(2);

        let c = 2.0 * (a.sqrt()).asin();

        let distance = c * EARTH_RADIUS_KM * linear_divisor(&DistanceUnit::Kilometers);
        let radius = radius * linear_divisor(distance_unit);

        return distance <= radius;
    }
}
