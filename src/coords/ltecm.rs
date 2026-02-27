use crate::consts::DAS2R;
use crate::pnp::{ltpecl, ltpequ};
use crate::vm::{pn, pxp};

/// ICRS equatorial to ecliptic rotation matrix, long-term.
///
/// Status:  support function.
///
/// Given:
///    epj     f64         Julian epoch (TT)
///
/// Returned:
///    rm      [[f64; 3]; 3]   ICRS to ecliptic rotation matrix
///
/// Notes:
///
/// 1) The matrix is in the sense
///
///       E_ep = rm x P_ICRS,
///
///    where P_ICRS is a vector with respect to ICRS right ascension
///    and declination axes and E_ep is the same vector with respect to
///    the (inertial) ecliptic and equinox of epoch epj.
///
/// 2) P_ICRS is a free vector, merely a direction, typically of unit
///    magnitude, and not bound to any particular spatial origin, such
///    as the Earth, Sun or SSB.  No assumptions are made about whether
///    it represents starlight and embodies astrometric effects such as
///    parallax or aberration.  The transformation is approximately that
///    between mean J2000.0 right ascension and declination and ecliptic
///    longitude and latitude, with only frame bias (always less than
///    25 mas) to disturb this classical picture.
///
/// 3) The Vondrak et al. (2011, 2012) 400 millennia precession model
///    agrees with the IAU 2006 precession at J2000.0 and stays within
///    100 microarcseconds during the 20th and 21st centuries.  It is
///    accurate to a few arcseconds throughout the historical period,
///    worsening to a few tenths of a degree at the end of the
///    +/- 200,000 year time span.
pub fn ltecm(epj: f64) -> [[f64; 3]; 3] {
    let mut rm = [[0.0; 3]; 3];

    /* Frame bias (IERS Conventions 2010, Eqs. 5.21 and 5.33) */
    let dx = -0.016617 * DAS2R;
    let de = -0.0068192 * DAS2R;
    let dr = -0.0146 * DAS2R;

    /* Equator pole. */
    let p = ltpequ(epj);

    /* Ecliptic pole (bottom row of equatorial to ecliptic matrix). */
    let z = ltpecl(epj);

    /* Equinox (top row of matrix). */
    let w = pxp(&p, &z);
    let (_, x) = pn(&w);

    /* Middle row of matrix. */
    let y = pxp(&z, &x);

    /* Combine with frame bias. */
    rm[0][0] = x[0] - x[1] * dr + x[2] * dx;
    rm[0][1] = x[0] * dr + x[1] + x[2] * de;
    rm[0][2] = -x[0] * dx - x[1] * de + x[2];
    rm[1][0] = y[0] - y[1] * dr + y[2] * dx;
    rm[1][1] = y[0] * dr + y[1] + y[2] * de;
    rm[1][2] = -y[0] * dx - y[1] * de + y[2];
    rm[2][0] = z[0] - z[1] * dr + z[2] * dx;
    rm[2][1] = z[0] * dr + z[1] + z[2] * de;
    rm[2][2] = -z[0] * dx - z[1] * de + z[2];

    rm
}
