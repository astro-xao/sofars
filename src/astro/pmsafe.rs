use crate::vm::seps;

use super::starpm;

///  Star proper motion:  update star catalog data for space motion, with
///  special handling to handle the zero parallax case.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  Given:
///     ra1    double     right ascension (radians), before
///     dec1   double     declination (radians), before
///     pmr1   double     RA proper motion (radians/year), before
///     pmd1   double     Dec proper motion (radians/year), before
///     px1    double     parallax (arcseconds), before
///     rv1    double     radial velocity (km/s, +ve = receding), before
///     ep1a   double     "before" epoch, part A (Note 1)
///     ep1b   double     "before" epoch, part B (Note 1)
///     ep2a   double     "after" epoch, part A (Note 1)
///     ep2b   double     "after" epoch, part B (Note 1)
///
///  Returned:
///     (ra2, dec2, pmr2, pmd2, px2, rv2)
///
///  Returned (function value):
///            int        status:
///                          -1 = system error (should not occur)
///                           0 = no warnings or errors
///                           1 = distance overridden (Note 6)
///                           2 = excessive velocity (Note 7)
///                           4 = solution didn't converge (Note 8)
///                        else = binary logical OR of the above warnings
///
///  Notes:
///
///  1) The starting and ending TDB dates ep1a+ep1b and ep2a+ep2b are
///     Julian Dates, apportioned in any convenient way between the two
///     parts (A and B).  For example, JD(TDB)=2450123.7 could be
///     expressed in any of these ways, among others:
///
///            epNa            epNb
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in cases
///     where the loss of several decimal digits of resolution is
///     acceptable.  The J2000 method is best matched to the way the
///     argument is handled internally and will deliver the optimum
///     resolution.  The MJD method and the date & time methods are both
///     good compromises between resolution and convenience.
///
///  2) In accordance with normal star-catalog conventions, the object's
///     right ascension and declination are freed from the effects of
///     secular aberration.  The frame, which is aligned to the catalog
///     equator and equinox, is Lorentzian and centered on the SSB.
///
///     The proper motions are the rate of change of the right ascension
///     and declination at the catalog epoch and are in radians per TDB
///     Julian year.
///
///     The parallax and radial velocity are in the same frame.
///
///  3) Care is needed with units.  The star coordinates are in radians
///     and the proper motions in radians per Julian year, but the
///     parallax is in arcseconds.
///
///  4) The RA proper motion is in terms of coordinate angle, not true
///     angle.  If the catalog uses arcseconds for both RA and Dec proper
///     motions, the RA proper motion will need to be divided by cos(Dec)
///     before use.
///
///  5) Straight-line motion at constant speed, in the inertial frame,
///     is assumed.
///
///  6) An extremely small (or zero or negative) parallax is overridden
///     to ensure that the object is at a finite but very large distance,
///     but not so large that the proper motion is equivalent to a large
///     but safe speed (about 0.1c using the chosen constant).  A warning
///     status of 1 is added to the status if this action has been taken.
///
///  7) If the space velocity is a significant fraction of c (see the
///     constant VMAX in the function iauStarpv), it is arbitrarily set
///     to zero.  When this action occurs, 2 is added to the status.
///
///  8) The relativistic adjustment carried out in the iauStarpv function
///     involves an iterative calculation.  If the process fails to
///     converge within a set number of iterations, 4 is added to the
///     status.
///
///  Called:
///     iauSeps      angle between two points
///     iauStarpm    update star catalog data for space motion
///
pub fn pmsafe(
    ra1: f64,
    dec1: f64,
    pmr1: f64,
    pmd1: f64,
    px1: f64,
    rv1: f64,
    ep1a: f64,
    ep1b: f64,
    ep2a: f64,
    ep2b: f64,
) -> Result<([f64; 6], i32), i32> {
    /* Minimum allowed parallax (arcsec) */
    const PXMIN: f64 = 5e-7;

    /* Factor giving maximum allowed transverse speed of about 1% c */
    const F: f64 = 326.0;

    /* Proper motion in one year (radians). */
    let mut pm = seps(ra1, dec1, ra1 + pmr1, dec1 + pmd1);

    /* Override the parallax to reduce the chances of a warning status. */
    let mut jpx = 0;
    let mut px1a = px1;
    pm *= F;
    if px1a < pm {
        jpx = 1;
        px1a = pm;
    }
    if px1a < PXMIN {
        jpx = 1;
        px1a = PXMIN;
    }

    /* Carry out the transformation using the modified parallax. */
    match starpm(ra1, dec1, pmr1, pmd1, px1a, rv1, ep1a, ep1b, ep2a, ep2b) {
        Ok((res, mut j)) => {
            /* Revise and return the status. */
            if (j % 2) == 0 {
                j += jpx;
            }
            Ok((res, j))
        }
        Err(e) => Err(e),
    }
}
