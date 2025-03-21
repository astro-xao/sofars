use crate::consts::{DAS2R, DJ00, DJC};
use crate::pnp::{bi00, pr00};
use crate::vm::{cr, ir, rx, rxr, ry, rz};

///  Frame bias and precession matrices, IAU 2000.
///
///  Frame bias and precession, IAU 2000.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  canonical model.
///
///  Given:
///     date1,date2  double         TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     rb           double[3][3]   frame bias matrix (Note 2)
///     rp           double[3][3]   precession matrix (Note 3)
///     rbp          double[3][3]   bias-precession matrix (Note 4)
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
///  3) The matrix rp transforms vectors from J2000.0 mean equator and
///     equinox to mean equator and equinox of date by applying
///     precession.
///
///  4) The matrix rbp transforms vectors from GCRS to mean equator and
///     equinox of date by applying frame bias then precession.  It is
///     the product rp x rb.
///
///  5) It is permissible to re-use the same array in the returned
///     arguments.  The arrays are filled in the order given.
///
///  Called:
///     iauBi00      frame bias components, IAU 2000
///     iauPr00      IAU 2000 precession adjustments
///     iauIr        initialize r-matrix to identity
///     iauRx        rotate around X-axis
///     iauRy        rotate around Y-axis
///     iauRz        rotate around Z-axis
///     iauCr        copy r-matrix
///     iauRxr       product of two r-matrices
///
///  Reference:
///     "Expressions for the Celestial Intermediate Pole and Celestial
///     Ephemeris Origin consistent with the IAU 2000A precession-
///     nutation model", Astron.Astrophys. 400, 1145-1154 (2003)
///
///     n.b. The celestial ephemeris origin (CEO) was renamed "celestial
///          intermediate origin" (CIO) by IAU 2006 Resolution 2.
///
///  This revision:  2021 May 11
///
///  SOFA release 2023-10-11
///
///  Copyright (C) 2023 IAU SOFA Board.  See notes at end.
///
pub fn bp00(date1: f64, date2: f64, rb: &mut [[f64; 3]; 3], rp: &mut [[f64; 3]; 3], rbp: &mut [[f64; 3]; 3]) {
    /* J2000.0 obliquity (Lieske et al. 1977) */
    const EPS0: f64 = 84381.448 * DAS2R;

    /* Interval between fundamental epoch J2000.0 and current date (JC). */
    let t = ((date1 - DJ00) + date2) / DJC;

    /* Frame bias. */
    let (dpsibi, depsbi, dra0) = bi00();

    /* Precession angles (Lieske et al. 1977) */
    let psia77 = (5038.7784 + (-1.07259 + (-0.001147) * t) * t) * t * DAS2R;
    let oma77  =       EPS0 + ((0.05127 + (-0.007726) * t) * t) * t * DAS2R;
    let chia   = (  10.5526 + (-2.38064 + (-0.001125) * t) * t) * t * DAS2R;

    /* Apply IAU 2000 precession corrections. */
    let (dpsipr, depspr) = pr00(date1, date2);
    let psia = psia77 + dpsipr;
    let oma  = oma77  + depspr;

    /* Frame bias matrix: GCRS to J2000.0. */
    let rbw = &mut [[0.0; 3]; 3];
    ir(rbw);
    rz(dra0, rbw);
    ry(dpsibi*EPS0.sin(), rbw);
    rx(-depsbi, rbw);
    cr(rbw, rb);

    /* Precession matrix: J2000.0 to mean of date. */
    ir(rp);
    rx(EPS0, rp);
    rz(-psia, rp);
    rx(-oma, rp);
    rz(chia, rp);

    /* Bias-precession matrix: GCRS to mean of date. */
    rxr(rp, rbw, rbp);
}