use crate::vm::{pn, pxp};

use super::{ltpecl, ltpequ};

///  Long-term precession matrix.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  Given:
///     epj     double         Julian epoch (TT)
///
///  Returned (function value):
///             [[f64; 3]; 3]   precession matrix, J2000.0 to date
///
///  Notes:
///
///  1) The matrix is in the sense
///
///        P_date = rp x P_J2000,
///
///     where P_J2000 is a vector with respect to the J2000.0 mean
///     equator and equinox and P_date is the same vector with respect to
///     the mean equator and equinox of epoch epj.
///
///  2) The Vondrak et al. (2011, 2012) 400 millennia precession model
///     agrees with the IAU 2006 precession at J2000.0 and stays within
///     100 microarcseconds during the 20th and 21st centuries.  It is
///     accurate to a few arcseconds throughout the historical period,
///     worsening to a few tenths of a degree at the end of the
///     +/- 200,000 year time span.
///
///  Called:
///     iauLtpequ    equator pole, long term
///     iauLtpecl    ecliptic pole, long term
///     iauPxp       vector product
///     iauPn        normalize vector
///
///  References:
///
///    Vondrak, J., Capitaine, N. and Wallace, P., 2011, New precession
///    expressions, valid for long time intervals, Astron.Astrophys. 534,
///    A22
///
///    Vondrak, J., Capitaine, N. and Wallace, P., 2012, New precession
///    expressions, valid for long time intervals (Corrigendum),
///    Astron.Astrophys. 541, C1
pub fn ltp(epj: f64) -> [[f64; 3]; 3] {
    let mut rp = [[0.0; 3]; 3];

    /* Equator pole (bottom row of matrix). */
    let peqr = ltpequ(epj);

    /* Ecliptic pole. */
    let pecl = ltpecl(epj);

    /* Equinox (top row of matrix). */
    let v = pxp(&peqr, &pecl);
    let (_, eqx) = pn(&v);

    /* Middle row of matrix. */
    let v_mid = pxp(&peqr, &eqx);

    /* Assemble the matrix. */
    for i in 0..3 {
        rp[0][i] = eqx[i];
        rp[1][i] = v_mid[i];
        rp[2][i] = peqr[i];
    }

    rp
}
