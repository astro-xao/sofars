use crate::pnp::{pr00, obl80, nut00a};
use super::ee00;

/// Equation of the equinoxes, IAU 2000A
pub fn ee00a(date1: f64, date2: f64) -> f64 {
    let (_, depspr) = pr00(date1, date2);

    // Mean obliquity, consistent with IAU 2000 precession-nutation.
    let epsa = obl80(date1, date2) + depspr;

    // Nutation in longitude.
    let (dpsi, _) = nut00a(date1, date2);

    // Equation of the equinoxes.
    let ee = ee00(date1, date2, epsa, dpsi);

    ee
}