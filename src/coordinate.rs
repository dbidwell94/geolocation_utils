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
}
