use std::f64::consts::TAU;

pub const EQUATORIAL_CIRCUMFERENCE_GRID: f64 = 1000.0;
pub const EARTH_R: f64 = 200.0;

#[must_use]
pub const fn arc_from_grid(grid: f64) -> f64 {
    grid * (TAU / EQUATORIAL_CIRCUMFERENCE_GRID)
}

#[must_use]
pub const fn grid_from_arc(arc: f64) -> f64 {
    arc * (EQUATORIAL_CIRCUMFERENCE_GRID / TAU)
}
