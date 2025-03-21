use super::{IauAstrom, ldsun, pmpx, ab};
use crate::vm::{rxp, c2s, anp};

///  Quick ICRS −> CIRS
/// 
///  Quick ICRS, epoch J2000.0, to CIRS transformation, given precomputed
///  star-independent astrometry parameters.
///
///  Use of this function is appropriate when efficiency is important and
///  where many star positions are to be transformed for one date.  The
///  star-independent parameters can be obtained by calling one of the
///  functions iauApci[13], iauApcg[13], iauApco[13] or iauApcs[13].
///
///  If the parallax and proper motions are zero the iauAtciqz function
///  can be used instead.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  Given:
///  ```
///     rc,dc  double     ICRS RA,Dec at J2000.0 (radians, Note 1)
///     pr     double     RA proper motion (radians/year, Note 2)
///     pd     double     Dec proper motion (radians/year)
///     px     double     parallax (arcsec)
///     rv     double     radial velocity (km/s, +ve if receding)
///     astrom iauASTROM* star-independent astrometry parameters:
///      pmt    double       PM time interval (SSB, Julian years)
///      eb     double[3]    SSB to observer (vector, au)
///      eh     double[3]    Sun to observer (unit vector)
///      em     double       distance from Sun to observer (au)
///      v      double[3]    barycentric observer velocity (vector, c)
///      bm1    double       sqrt(1-|v|^2): reciprocal of Lorenz factor
///      bpn    double[3][3] bias-precession-nutation matrix
///      along  double       longitude + s' (radians)
///      xpl    double       polar motion xp wrt local meridian (radians)
///      ypl    double       polar motion yp wrt local meridian (radians)
///      sphi   double       sine of geodetic latitude
///      cphi   double       cosine of geodetic latitude
///      diurab double       magnitude of diurnal aberration vector
///      eral   double       "local" Earth rotation angle (radians)
///      refa   double       refraction constant A (radians)
///      refb   double       refraction constant B (radians)
///  ```
///  Returned:
///  ```
///     ri,di   double    CIRS RA,Dec (radians)
///  ```
///  Notes:
///
///  1) Star data for an epoch other than J2000.0 (for example from the
///     Hipparcos catalog, which has an epoch of J1991.25) will require a
///     preliminary call to iauPmsafe before use.
///
///  2) The proper motion in RA is dRA/dt rather than cos(Dec)*dRA/dt.
///
///  Called:
///  ```
///     iauPmpx      proper motion and parallax
///     iauLdsun     light deflection by the Sun
///     iauAb        stellar aberration
///     iauRxp       product of r-matrix and pv-vector
///     iauC2s       p-vector to spherical
///  ```
pub fn atciq(rc: f64, dc: f64, pr: f64, pd: f64, px: f64, rv: f64, 
                            astrom: &mut IauAstrom,) -> (f64, f64) {
    let pco = pmpx(rc, dc, pr, pd, px, rv, astrom.pmt, astrom.eb);
    let pnat = ldsun(pco, astrom.eh, astrom.em);
    let mut ppr = ab(&pnat, &astrom.v, astrom.em, astrom.bm1);
    let pi = &mut [0.0; 3];
    rxp(&astrom.bpn, &mut ppr, pi);
    let (w, di) = c2s(pi);
    let ri = anp(w);
    (ri, di)
}