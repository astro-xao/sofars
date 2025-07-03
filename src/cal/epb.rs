use crate::consts::{D1900, DJ00, DTY};

/// Julian Date to Besselian Epoch
pub fn epb(dj1: f64, dj2: f64) -> f64 {
    1900.0 + ((dj1 - DJ00) + (dj2 + D1900)) / DTY
}
