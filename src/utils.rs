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
        DistanceUnit::Meters => LINEAR_DISTANCE_IN_METERS,
    }
}

/// # Summary
/// Takes input and wraps it between - and + of `neg_pos_bound`
///
pub fn wrap_to_bounds(input: f64, neg_pos_bound: f64) -> f64 {
    let abs_neg_pos = neg_pos_bound.abs();
    let add_or_sub = neg_pos_bound * 2.0;
    let mut wrapped = input;

    while wrapped < -abs_neg_pos || wrapped > abs_neg_pos {
        if wrapped <= -abs_neg_pos {
            wrapped -= add_or_sub * -1.0;
        } else if wrapped > abs_neg_pos {
            wrapped += add_or_sub * -1.0;
        }
    }

    wrapped
}

#[allow(dead_code)]
fn wrap_to_bounds_wip(angle: f64, bounds: f64) -> f64 {
    (angle + bounds).rem_euclid(2.0 * bounds) - bounds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_to_bounds() {
        let output = wrap_to_bounds(181.0, 90.0);
        assert_eq!(1.0, output);

        let output = wrap_to_bounds(91.0, 90.0);
        assert_eq!(-89.0, output);

        let output = wrap_to_bounds(181.0, 180.0);
        assert_eq!(-179.0, output);

        let output = wrap_to_bounds(120.0, 90.0);
        assert_eq!(-60.0, output);

        let output = wrap_to_bounds(-91.0, 90.0);
        assert_eq!(89.0, output);

        let output = wrap_to_bounds(-90.0, 90.0);
        assert_eq!(-90.0, output);

        let output = wrap_to_bounds(179.0, 90.0);
        assert_eq!(-1.0, output);
    }
}
