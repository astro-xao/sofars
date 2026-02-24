use crate::consts::{DJM0, DJM00};

/// Julian Epoch to Julian Date.
///
/// This function is part of the International Astronomical Union's
/// SOFA (Standards of Fundamental Astronomy) software collection.
///
/// Status: support function.
///
/// # Given:
/// * `epj`: Julian Epoch (e.g. 1996.8)
///
/// # Returned:
/// * `djm0`: MJD zero-point: always 2400000.5
/// * `djm`: Modified Julian Date
///
/// # Note:
/// The Julian Date is returned in two pieces, in the usual SOFA
/// manner, which is designed to preserve time resolution. The
/// Julian Date is available as a single number by adding djm0 and
/// djm.
///
/// # Reference:
/// Lieske, J.H., 1979, Astron.Astrophys. 73, 282.
pub fn epj2jd(epj: f64) -> (f64, f64) {
    (DJM0, DJM00 + (epj - 2000.0) * 365.25)
}
