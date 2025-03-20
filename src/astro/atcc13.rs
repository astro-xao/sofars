use super::{apci13, atccq, IauAstrom};

///  catalog -> astrometric.
///
///  Transform a star's ICRS catalog entry (epoch J2000.0) into ICRS
///  astrometric place.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  Given:
///     rc     double   ICRS right ascension at J2000.0 (radians, Note 1)
///     dc     double   ICRS declination at J2000.0 (radians, Note 1)
///     pr     double   RA proper motion (radians/year, Note 2)
///     pd     double   Dec proper motion (radians/year)
///     px     double   parallax (arcsec)
///     rv     double   radial velocity (km/s, +ve if receding)
///     date1  double   TDB as a 2-part...
///     date2  double   ...Julian Date (Note 3)
///
///  Returned:
///     ra,da  double*  ICRS astrometric RA,Dec (radians)
///
///  Notes:
///
///  1) Star data for an epoch other than J2000.0 (for example from the
///     Hipparcos catalog, which has an epoch of J1991.25) will require a
///     preliminary call to iauPmsafe before use.
///
///  2) The proper motion in RA is dRA/dt rather than cos(Dec)*dRA/dt.
///
///  3) The TDB date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TDB)=2450123.7 could be expressed in any of these ways, among
///     others:
///
///            date1          date2
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
///     good compromises between resolution and convenience.  For most
///     applications of this function the choice will not be at all
///     critical.
///
///     TT can be used instead of TDB without any significant impact on
///     accuracy.
///
///  Called:
///     iauApci13    astrometry parameters, ICRS-CIRS, 2013
///     iauAtccq     quick catalog ICRS to astrometric
pub fn atcc13(
    rc: f64, dc: f64,
    pr: f64, pd: f64, px: f64, rv: f64,
    date1: f64, date2: f64
) -> (f64, f64) {
    /* Star-independent astrometry parameters */
    let astrom = &mut IauAstrom::default();
    let w = &mut 0.0;

    /* The transformation parameters. */
    apci13(date1, date2, astrom, w);

    /* Catalog ICRS (epoch J2000.0) to astrometric. */
    let (ra, da) = atccq(rc, dc, pr, pd, px, rv, astrom);

    (ra, da)
}