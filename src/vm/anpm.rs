use crate::consts::{D2PI, DPI};
use std::ops::Rem;

/// Normalize angle into the range -pi <= a < +pi.
#[inline]
pub fn anpm(a: f64) -> f64 {
    let mut w = a.rem(D2PI);
    if w.abs() >= DPI {
        // SOFA: w -= dsign(D2PI, a)
        w -= if a < 0.0 { -D2PI } else { D2PI };
    }
    w
}
