use crate::{consts::{AULT, DAU, DAYSEC, DJM, DJY}, vm::{pdp, pn}};

///  Apply proper motion and parallax
/// 
///  Proper motion and parallax.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  Given:
///     rc,dc  double     ICRS RA,Dec at catalog epoch (radians)
///     pr     double     RA proper motion (radians/year, Note 1)
///     pd     double     Dec proper motion (radians/year)
///     px     double     parallax (arcsec)
///     rv     double     radial velocity (km/s, +ve if receding)
///     pmt    double     proper motion time interval (SSB, Julian years)
///     pob    double[3]  SSB to observer vector (au)
///
///  Returned:
///     pco    double[3]  coordinate direction (BCRS unit vector)
///
///  Notes:
///
///  1) The proper motion in RA is dRA/dt rather than cos(Dec)*dRA/dt.
///
///  2) The proper motion time interval is for when the starlight
///     reaches the solar system barycenter.
///
///  3) To avoid the need for iteration, the Roemer effect (i.e. the
///     small annual modulation of the proper motion coming from the
///     changing light time) is applied approximately, using the
///     direction of the star at the catalog epoch.
///
///  References:
///
///     1984 Astronomical Almanac, pp B39-B41.
///
///     Urban, S. & Seidelmann, P. K. (eds), Explanatory Supplement to
///     the Astronomical Almanac, 3rd ed., University Science Books
///     (2013), Section 7.2.
///
///  Called:
///     iauPdp       scalar product of two p-vectors
///     iauPn        decompose p-vector into modulus and direction
pub fn pmpx(rc: f64, dc: f64, pr: f64, pd: f64, px: f64, rv: f64, pmt: f64, 
                                                pob: [f64; 3],) -> [f64; 3] {
    /* Km/s to au/year */
    let vf = DAYSEC * DJM / DAU;

    /* Light time for 1 au, Julian years */
    let aulty = AULT / DAYSEC / DJY;

    let sr = rc.sin();
    let cr = rc.cos();
    let sd = dc.sin();
    let cd = dc.cos();

    let x = cr * cd;
    let y = sr * cd;
    let z = sd;
    let mut p = [x, y, z];

    /* Proper motion time interval (y) including Roemer effect. */
    let dt = pmt + pdp(&p, &pob) * aulty;

    /* Space motion (radians per year). */
    let pxr = px * crate::consts::DAS2R;
    let w = vf * rv * pxr;
    let pdz = pd * z;
    let pm = [
        -pr * y - pdz * cr + w * x,
        pr * x - pdz * sr + w * y,
        pd * cd + w * z,
    ];

    /* Coordinate direction of star (unit vector, BCRS). */
    for i in 0..3 {
        p[i] += dt * pm[i] - pxr * pob[i];
    }

    let (_, u) = pn(&p);

    u
}