use super::d2tf;
use crate::consts::D2PI;

/// Decompose radians into degrees, arcminutes, arcseconds, fraction.
pub fn a2af(ndp: i32, angle: f64) -> (char, [i32; 4]) {
    d2tf(ndp, angle * 15.0 / D2PI)
}
