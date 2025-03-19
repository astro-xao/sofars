use std::ops::Rem;

use crate::{consts::{D2PI, DJ00}, vm::anp};

pub fn era00(dj1: f64, dj2: f64) -> f64 {
    let d1: f64;
    let d2: f64;

    // Days since fundamental epoch.
    if dj1 < dj2 {
        d1 = dj1;
        d2 = dj2;
    } else {
        d1 = dj2;
        d2 = dj1;
    }

    let t = d1 + (d2 - DJ00);

    // Fractional part of T (days).
    let f = d1.rem(1.0) + d2.rem(1.0);

    // Earth rotation angle at this UT1.
    let theta = anp(D2PI * (f + 0.7790572732640 + 0.00273781191135448 * t));

    theta
}
