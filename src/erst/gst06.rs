use super::era00;
use crate::pnp::{bpn2xy, eors, s06};
use crate::vm::anp;

/// Greenwich apparent ST, IAU 2006, given NPB matrix
pub fn gst06(uta: f64, utb: f64, tta: f64, ttb: f64, rnpb: &[[f64; 3]; 3]) -> f64 {
    let (x, y) = bpn2xy(rnpb);

    // The CIO locator, s.
    let s = s06(tta, ttb, x, y);

    // Greenwich apparent sidereal time.
    let era = era00(uta, utb);
    let eors = eors(rnpb, s);
    let gst = anp(era - eors);

    gst
}
