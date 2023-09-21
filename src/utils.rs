use crate::DistanceUnit;

const LATITUDE_DISTANCE_IN_MILES: f64 = 69.0;
const LINEAR_DISTANCE_IN_MILES: f64 = 1609.0;
const LATITUDE_DISTANCE_IN_NAUTICAL_MILES: f64 = 60.0;
const LINEAR_DISTANCE_IN_NAUTICAL_MILES: f64 = 1852.0;
const LATITUDE_DISTANCE_IN_KILOMETERS: f64 = 111.045;
const LINEAR_DISTANCE_IN_KILOMETERS: f64 = 1000.0;
const LATITUDE_DISTANCE_IN_METERS: f64 = 111045.0;
const LINEAR_DISTANCE_IN_METERS: f64 = 1.0;
pub const EARTH_RADIUS_KM: f64 = 6371.0;

pub fn divisor(unit: &DistanceUnit) -> f64 {
    match unit {
        DistanceUnit::Miles => LATITUDE_DISTANCE_IN_MILES,
        DistanceUnit::NauticalMiles => LATITUDE_DISTANCE_IN_NAUTICAL_MILES,
        DistanceUnit::Kilometers => LATITUDE_DISTANCE_IN_KILOMETERS,
        DistanceUnit::Meters => LATITUDE_DISTANCE_IN_METERS,
    }
}

pub fn linear_divisor(unit: &DistanceUnit) -> f64 {
    match unit {
        DistanceUnit::Miles => LINEAR_DISTANCE_IN_MILES,
        DistanceUnit::NauticalMiles => LINEAR_DISTANCE_IN_NAUTICAL_MILES,
        DistanceUnit::Kilometers => LINEAR_DISTANCE_IN_KILOMETERS,
        DistanceUnit::Meters => LINEAR_DISTANCE_IN_METERS
    }
}