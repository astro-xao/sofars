use crate::consts::D2PI;
use std::ops::Rem;

pub fn anp(a: f64) -> f64 {
    let mut w = a.rem(D2PI);
    if w < 0.0 {
        w += D2PI;
    }
    w
}
