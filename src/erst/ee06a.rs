use crate::vm::anpm;
use super::{gst06a, gmst06};

/// Equation of the equinoxes, IAU 2006/2000A
pub fn ee06a(date1: f64, date2: f64) -> f64 {
    let _gst06a = gst06a(0.0, 0.0, date1, date2);
    let _gmst06 = gmst06(0.0, 0.0, date1, date2);

    // Equation of the equinoxes.
    let ee = anpm(_gst06a - _gmst06);

    ee
}