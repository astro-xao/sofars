use std::ops::Rem;
use crate::consts::{D2PI, DPI};

pub fn anpm(a: f64) -> f64 {
    let mut w = a.rem(D2PI);
    if w.abs() >= DPI {
        w -= D2PI;
    }
    w
}