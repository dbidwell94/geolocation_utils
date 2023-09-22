use crate::utils::{linear_divisor, EARTH_RADIUS_KM, wrap_to_bounds};
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
    /// Construct a new Coordinate. Automatically prevents overflow of lat / long coordinates
    ///
    /// ## Example
    /// ```rust
    /// use geolocation_utils::Coordinate;
    ///
    /// let coordinate = Coordinate::new(34.8, -2.8);
    /// assert_eq!(34.8, coordinate.latitude);
    /// assert_eq!(-2.8, coordinate.longitude);
    /// 
    /// // Overflowing coordinate
    /// let coordinate = Coordinate::new(91.6275, -181.875);
    /// assert_eq!(-88.3725, coordinate.latitude);
    /// assert_eq!(178.125, coordinate.longitude);
    /// ```
    pub fn new(lat: f64, lon: f64) -> Self {
        Self {
            latitude: wrap_to_bounds(lat, 90.0),
            longitude: wrap_to_bounds(lon, 180.0),
        }
    }

    /// # Summary
    /// Checks if a coordinate is within the radius of another coordinate.
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
        let distance = self.get_distance_from(other_coordinate, distance_unit);
        let radius = radius * linear_divisor(distance_unit);

        return distance <= radius;
    }

    /// # Summary
    /// Gets the distance between 2 coordinates
    ///
    /// ## Notes
    /// - Uses the Haversine formula
    /// - Implementation taken from https://www.geeksforgeeks.org/program-distance-two-points-earth/
    ///
    /// ## Example
    /// ```rust
    /// use geolocation_utils::{Coordinate, DistanceUnit};
    /// let coordinate1 = Coordinate::new(1.0, 1.0);
    /// let coordinate2 = Coordinate::new(0.0, 0.0);
    /// 
    /// let distance = coordinate1.get_distance_from(&coordinate2, &DistanceUnit::Kilometers);
    /// 
    /// // Rounding because output number is 157.24938127194397
    /// let rounded_distance = (distance * 100.0).round() / 100.0;
    /// 
    /// assert_eq!(157.25, rounded_distance);
    /// ```
    pub fn get_distance_from(&self, other: &Coordinate, unit: &DistanceUnit) -> f64 {
        // Formula from https://www.geeksforgeeks.org/program-distance-two-points-earth/
        let pi = std::f64::consts::PI;

        let lat1 = self.latitude * pi / 180.0;
        let lat2 = other.latitude * pi / 180.0;
        let lon1 = self.longitude * pi / 180.0;
        let lon2 = other.longitude * pi / 180.0;

        let d_lon = lon2 - lon1;
        let d_lat = lat2 - lat1;

        let a = (((d_lat / 2.0).sin()).powi(2))
            + (lat1.cos() * lat2.cos()) * ((d_lon / 2.0).sin()).powi(2);

        let c = 2.0 * (a.sqrt()).asin();

        let distance_meters = (c * EARTH_RADIUS_KM) * linear_divisor(&DistanceUnit::Kilometers);
        return distance_meters / linear_divisor(unit);
    }
}
