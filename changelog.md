# 0.2.1
- Added guard to the `Coordinate::new()` method which will prevent overflow of lat / long coordinate if not within the required 90 / 180 degree bounds
# 0.2.0
- Added a `get_distance_from` function to the `Coordinate` struct
# 0.1.5
- Added new function `in_radius` to the `Coordinate` struct
- Update `serde` dependency