use crate::consts::{DJM0, DJM00};
use crate::pnp::{fw2m, pfw06, pmat06};
use crate::vm::{rxr, tr};

///  Frame bias and precession, IAU 2006.
///
///  Given:
///     date1,date2  f64          TT as a 2-part Julian Date (Note 1)
///
///  Returned (function value):
///     (rb, rp, rbp) ([[f64; 3]; 3], [[f64; 3]; 3], [[f64; 3]; 3])
///
///     rb           frame bias matrix (Note 2)
///     rp           precession matrix (Note 3)
///     rbp          bias-precession matrix (Note 4)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///             date1         date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix rb transforms vectors from GCRS to mean J2000.0 by
///     applying frame bias.
///
///  3) The matrix rp transforms vectors from mean J2000.0 to mean of
///     date by applying precession.
///
///  4) The matrix rbp transforms vectors from GCRS to mean of date by
///     applying frame bias then precession.  It is the product rp x rb.
///
///  Called:
///     iauPfw06     bias-precession F-W angles, IAU 2006
///     iauFw2m      F-W angles to r-matrix
///     iauPmat06    PB matrix, IAU 2006
///     iauTr        transpose r-matrix
///     iauRxr       product of two r-matrices
///     iauCr        copy r-matrix
///
///  References:
///
///     Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855
///
///     Wallace, P.T. & Capitaine, N., 2006, Astron.Astrophys. 459, 981
pub fn bp06(date1: f64, date2: f64) -> ([[f64; 3]; 3], [[f64; 3]; 3], [[f64; 3]; 3]) {
    let mut rbt = [[0.0; 3]; 3];
    let mut rp = [[0.0; 3]; 3];

    /* B matrix. */
    let (gamb, phib, psib, epsa) = pfw06(DJM0, DJM00);
    let rb = fw2m(gamb, phib, psib, epsa);

    /* PxB matrix (temporary). */
    let rbp = pmat06(date1, date2);

    /* P matrix. */
    tr(&rb, &mut rbt);
    rxr(&rbp, &rbt, &mut rp);

    (rb, rp, rbp)
}
