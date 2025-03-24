use crate::pnp::pnm06a;
use super::gst06;

/// Greenwich apparent sidereal time, IAU 2006/2000A
pub fn gst06a(uta: f64, utb: f64, tta: f64, ttb: f64) -> f64 {

    // Classical nutation x precession x bias matrix, IAU 2000A.
    let rnpb = pnm06a(tta, ttb);

    // Greenwich apparent sidereal time.
    gst06(uta, utb, tta, ttb, &rnpb)
}