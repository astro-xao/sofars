use crate::consts::{DJM0, DTY};

/// Besselian Epoch to Julian Date.
///
/// This function is part of the International Astronomical Union's
/// SOFA (Standards of Fundamental Astronomy) software collection.
///
/// Status: support function.
///
/// # Given:
/// * `epb`: Besselian Epoch (e.g. 1957.3)
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
pub fn epb2jd(epb: f64) -> (f64, f64) {
    (DJM0, 15019.81352 + (epb - 1900.0) * DTY)
}
