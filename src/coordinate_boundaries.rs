use crate::{Coordinate, DistanceUnit};
use crate::utils::{divisor};

pub struct CoordinateBoundaries {
    latitude: f64,
    longitude: f64,
    distance: f64,
    distance_unit: DistanceUnit,
    max_lon: f64,
    min_lon: f64,
    max_lat: f64,
    min_lat: f64,
}

impl CoordinateBoundaries {
    /// # Summary
    /// Create a new `CoordinateBoundaries` struct. Returns `None` if `Coordinate` is invalid
    ///
    /// # Example
    /// ```rust
    /// use geolocation_utils::{Coordinate, CoordinateBoundaries};
    ///
    /// let coords = Coordinate::new(0.0, 0.0);
    /// let bounds = CoordinateBoundaries::new(coords, 12.0, None);
    ///
    /// assert!(bounds.is_some());
    /// ```
    pub fn new(origin: Coordinate, distance: f64, unit: Option<DistanceUnit>) -> Option<Self> {
        if !Self::validate(&origin) {
            return None;
        }
        let unit = unit.unwrap_or(DistanceUnit::Miles);
        let (min_lat, max_lat, min_lon, max_lon) =
            Self::calculate(&unit, distance, origin.latitude, origin.longitude);

        let to_return = Self {
            distance,
            distance_unit: unit.clone(),
            latitude: origin.latitude,
            longitude: origin.longitude,
            max_lat,
            max_lon,
            min_lat,
            min_lon,
        };
        Some(to_return)
    }

    /// # Summary
    /// Get the max longitude for the coords bounds
    /// # Example
    /// ```rust
    /// use geolocation_utils::{Coordinate, CoordinateBoundaries};
    /// 
    /// let coord = Coordinate::new(0.0, 0.0);
    /// let bounds = CoordinateBoundaries::new(coord, 1.0, None).unwrap();
    /// 
    /// let max_lon = bounds.max_longitude();
    /// assert_eq!(max_lon, 0.014492753623188406);
    /// ```
    pub fn max_longitude(&self) -> f64 {
        self.max_lon
    }

    /// # Summary
    /// Get the min longitude for the coords bounds
    /// # Example
    /// ```rust
    /// use geolocation_utils::{Coordinate, CoordinateBoundaries};
    /// 
    /// let coord = Coordinate::new(0.0, 0.0);
    /// let bounds = CoordinateBoundaries::new(coord, 1.0, None).unwrap();
    /// 
    /// let min_lon = bounds.min_longitude();
    /// assert_eq!(min_lon, -0.014492753623188406);
    /// ```
    pub fn min_longitude(&self) -> f64 {
        self.min_lon
    }
    
    /// # Summary
    /// Get the max latitude for the coords bounds
    /// # Example
    /// ```rust
    /// use geolocation_utils::{Coordinate, CoordinateBoundaries};
    /// 
    /// let coord = Coordinate::new(0.0, 0.0);
    /// let bounds = CoordinateBoundaries::new(coord, 1.0, None).unwrap();
    /// 
    /// let max_lat = bounds.max_latitude();
    /// assert_eq!(max_lat, 0.014492753623188406);
    /// ```
    pub fn max_latitude(&self) -> f64 {
        self.max_lat
    }

    /// # Summary
    /// Get the min latitude for the coords bounds
    /// # Example
    /// ```rust
    /// use geolocation_utils::{Coordinate, CoordinateBoundaries};
    /// 
    /// let coord = Coordinate::new(0.0, 0.0);
    /// let bounds = CoordinateBoundaries::new(coord, 1.0, None).unwrap();
    /// 
    /// let min_lat = bounds.min_latitude();
    /// assert_eq!(min_lat, -0.014492753623188406);
    /// ```
    pub fn min_latitude(&self) -> f64 {
        self.min_lat
    }

    /// # Summary
    /// Sets the coordinates used to calculate bounds
    /// # Example
    /// ```rust
    /// use geolocation_utils::{Coordinate, CoordinateBoundaries};
    /// 
    /// let coords = Coordinate::new(0.0, 0.0);
    /// let mut bounds = CoordinateBoundaries::new(coords, 1.0, None).unwrap();
    /// let min_lat = bounds.min_latitude();
    /// 
    /// bounds.set_coords(Coordinate::new(1.0, 1.5));
    /// let new_min_lat = bounds.min_latitude();
    /// 
    /// assert_ne!(min_lat, new_min_lat);
    /// ```
    pub fn set_coords(&mut self, coords: Coordinate) {
        self.latitude = coords.latitude;
        self.longitude = coords.longitude;
        let (min_lat, max_lat, min_lon, max_lon) = Self::calculate(
            &self.distance_unit,
            self.distance,
            coords.latitude,
            coords.longitude,
        );
        self.min_lat = min_lat;
        self.max_lat = max_lat;
        self.min_lon = min_lon;
        self.max_lon = max_lon;
    }

    /// # Summary
    /// Sets the distance and optionally the distance unit to calculate bounds
    /// # Example
    /// ```rust
    /// use geolocation_utils::{Coordinate, CoordinateBoundaries};
    /// 
    /// let coords = Coordinate::new(0.0, 0.0);
    /// let mut bounds = CoordinateBoundaries::new(coords, 1.0, None).unwrap();
    /// let min_lat = bounds.min_latitude();
    /// 
    /// bounds.set_distance(12.0, None);
    /// let new_min_lat = bounds.min_latitude();
    /// 
    /// assert_ne!(min_lat, new_min_lat);
    /// ```
    pub fn set_distance(&mut self, distance: f64, unit: Option<DistanceUnit>) {
        self.distance = distance;
        self.distance_unit = unit.unwrap_or(DistanceUnit::Miles);
        let (min_lat, max_lat, min_lon, max_lon) = Self::calculate(
            &self.distance_unit,
            self.distance,
            self.latitude,
            self.longitude,
        );
        self.min_lat = min_lat;
        self.max_lat = max_lat;
        self.min_lon = min_lon;
        self.max_lon = max_lon;
    }

    /// # Summary
    /// Calculate min_lat, max_lat, min_lon, and max_lon bounds
    fn calculate(unit: &DistanceUnit, distance: f64, lat: f64, lon: f64) -> (f64, f64, f64, f64) {
        let divisor = divisor(unit);

        let latitude_conversion_factor = distance / divisor;

        let longitude_conversion_factor = distance / divisor / lat.to_radians().cos().abs();

        let min_latitude = lat - latitude_conversion_factor;
        let max_latitude = lat + latitude_conversion_factor;

        let min_longitude = lon - longitude_conversion_factor;
        let max_longitude = lon + longitude_conversion_factor;
        (min_latitude, max_latitude, min_longitude, max_longitude)
    }

    fn validate(coord: &Coordinate) -> bool {
        if coord.latitude < -90.0 || coord.latitude > 90.0 {
            return false;
        }
        if coord.longitude < -180.0 || coord.longitude > 180.0 {
            return false;
        }

        return true;
    }
}
