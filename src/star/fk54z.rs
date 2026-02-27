use crate::star::fk524;
use crate::vm::{anp, c2s, s2c};

/// Convert a J2000.0 FK5 star position to B1950.0 FK4, assuming zero
/// proper motion in FK5 and parallax.
///
/// Status:  support function.
///
/// Given:
///    r2000,d2000    f64   J2000.0 FK5 RA,Dec (rad)
///    bepoch         f64   Besselian epoch (e.g. 1950.0)
///
/// Returned:
///    r1950,d1950    f64   B1950.0 FK4 RA,Dec (rad) at epoch BEPOCH
///    dr1950,dd1950  f64   B1950.0 FK4 proper motions (rad/trop.yr)
///
/// Notes:
///
/// 1) In contrast to the fk524 function, here the FK5 proper
///    motions, the parallax and the radial velocity are presumed zero.
///
/// 2) This function converts a star position from the IAU 1976 FK5
///   (Fricke) system to the former FK4 (Bessel-Newcomb) system, for
///    cases such as distant radio sources where it is presumed there is
///    zero parallax and no proper motion.  Because of the E-terms of
///    aberration, such objects have (in general) non-zero proper motion
///    in FK4, and the present function returns those fictitious proper
///    motions.
///
/// 3) Conversion from J2000.0 FK5 to B1950.0 FK4 only is provided for.
///    Conversions involving other equinoxes would require additional
///    treatment for precession.
///
/// 4) The position returned by this function is in the B1950.0 FK4
///    reference system but at Besselian epoch bepoch.  For comparison
///    with catalogs the bepoch argument will frequently be 1950.0. (In
///    this context the distinction between Besselian and Julian epoch
///    is insignificant.)
///
/// 5) The RA component of the returned (fictitious) proper motion is
///    dRA/dt rather than cos(Dec)*dRA/dt.
///
/// Called:
///    anp       normalize angle into range 0 to 2pi
///    c2s       p-vector to spherical
///    fk524     FK4 to FK5
///    s2c       spherical to p-vector
pub fn fk54z(r2000: f64, d2000: f64, bepoch: f64) -> (f64, f64, f64, f64) {
    /* FK5 equinox J2000.0 to FK4 equinox B1950.0. */
    let (r, d, pr, pd, _px, _rv) = fk524(r2000, d2000, 0.0, 0.0, 0.0, 0.0);

    /* Spherical to Cartesian. */
    let mut p = s2c(r, d);

    /* Fictitious proper motion (radians per year). */
    let mut v = [0.0; 3];
    v[0] = -pr * p[1] - pd * r.cos() * d.sin();
    v[1] = pr * p[0] - pd * r.sin() * d.sin();
    v[2] = pd * d.cos();

    /* Apply the motion. */
    let w = bepoch - 1950.0;
    for i in 0..3 {
        p[i] += w * v[i];
    }

    /* Cartesian to spherical. */
    let (w_res, d1950) = c2s(&p);
    let r1950 = anp(w_res);

    /* Fictitious proper motion. */
    (r1950, d1950, pr, pd)
}
