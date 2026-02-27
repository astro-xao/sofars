use crate::coords::ltecm;
use crate::vm::{anp, anpm, c2s, rxp, s2c};

/// Transformation from ICRS equatorial coordinates to ecliptic
/// coordinates (mean equinox and ecliptic of date), using a
/// long-term precession model.
///
/// Status:  support function.
///
/// Given:
///    epj     f64    Julian epoch (TT)
///    dr,dd   f64    ICRS right ascension and declination (radians)
///
/// Returned:
///    dl,db   f64    ecliptic longitude and latitude (radians)
///
/// Notes:
///
/// 1) No assumptions are made about whether the coordinates represent
///    starlight and embody astrometric effects such as parallax or
///    aberration.
///
/// 2) The transformation is approximately that from mean J2000.0 right
///    ascension and declination to ecliptic longitude and latitude
///    (mean equinox and ecliptic of date), with only frame bias (always
///    less than 25 mas) to disturb this classical picture.
///
/// 3) The Vondrak et al. (2011, 2012) 400 millennia precession model
///    is used.  It agrees with the IAU 2006 precession at J2000.0 and
///    stays within 100 microarcseconds during the 20th and 21st
///    centuries.  It is accurate to a few arcseconds throughout the
///    historical period, worsening to a few tenths of a degree at the
///    end of the +/- 200,000 year time span.
///
/// Called:
///    s2c       spherical coordinates to unit vector
///    ltecm     ICRS to ecliptic rotation matrix, long-term
///    rxp       product of r-matrix and p-vector
///    c2s       unit vector to spherical coordinates
///    anp       normalize angle into range 0 to 2pi
///    anpm      normalize angle into range +/- pi
pub fn lteqec(epj: f64, dr: f64, dd: f64) -> (f64, f64) {
    let mut v2 = [0.0; 3];

    /* Spherical to Cartesian. */
    let v1 = s2c(dr, dd);

    /* Rotation matrix, ICRS equatorial to ecliptic. */
    let rm = ltecm(epj);

    /* The transformation from ICRS to ecliptic. */
    rxp(&rm, &v1, &mut v2);

    /* Cartesian to spherical. */
    let (a, b) = c2s(&v2);

    /* Express in conventional ranges. */
    (anp(a), anpm(b))
}
