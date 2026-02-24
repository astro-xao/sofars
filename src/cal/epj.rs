use crate::consts::{DJ00, DJY};

/// Julian Date to Julian Epoch.
///
/// This function is part of the International Astronomical Union's
/// SOFA (Standards of Fundamental Astronomy) software collection.
///
/// Status: support function.
///
/// # Given:
/// * `dj1`, `dj2`: Julian Date (Note 4)
///
/// # Returned:
/// * Julian Epoch
///
/// # Notes:
/// 1) Julian Epoch is a method of expressing a moment in time as a
///    year plus fraction.
///
/// 2) Julian Epoch J2000.0 is 2000 Jan 1.5, and the length of the year
///    is 365.25 days.
///
/// 3) For historical reasons, the time scale formally associated with
///    Julian Epoch is TDB (or TT, near enough). However, Julian Epoch
///    can be used more generally as a calendrical convention to
///    represent other time scales such as TAI and TCB. This is
///    analogous to Julian Date, which was originally defined
///    specifically as a way of representing Universal Times but is now
///    routinely used for any of the regular time scales.
///
/// 4) The Julian Date is supplied in two pieces, in the usual SOFA
///    manner, which is designed to preserve time resolution. The
///    Julian Date is available as a single number by adding dj1 and
///    dj2. The maximum resolution is achieved if dj1 is 2451545.0
///    (J2000.0).
///
/// # Reference:
/// Lieske, J.H., 1979, Astron.Astrophys. 73, 282.
pub fn epj(dj1: f64, dj2: f64) -> f64 {
    2000.0 + ((dj1 - DJ00) + dj2) / DJY
}
