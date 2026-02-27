use crate::consts::{DJ00, DJY};
use crate::star::fk5hip;
use crate::vm::{anp, pv2s, rv2m, rxp, rxr, s2c, sxp, trxp, pxp};

/// Transform a Hipparcos star position into FK5 J2000.0, assuming
/// zero Hipparcos proper motion.
///
/// Status:  support function.
///
/// Given:
///    rh            f64    Hipparcos RA (radians)
///    dh            f64    Hipparcos Dec (radians)
///    date1,date2   f64    TDB date (Note 1)
///
/// Returned (all FK5, equinox J2000.0, date date1+date2):
///    r5            f64    RA (radians)
///    d5            f64    Dec (radians)
///    dr5           f64    RA proper motion (rad/year, Note 4)
///    dd5           f64    Dec proper motion (rad/year, Note 4)
///
/// Notes:
///
/// 1) The TT date date1+date2 is a Julian Date, apportioned in any
///    convenient way between the two arguments.  For example,
///    JD(TT)=2450123.7 could be expressed in any of these ways,
///    among others:
///
///           date1          date2
///
///        2450123.7           0.0       (JD method)
///        2451545.0       -1421.3       (J2000 method)
///        2400000.5       50123.2       (MJD method)
///        2450123.5           0.2       (date & time method)
///
///    The JD method is the most natural and convenient to use in
///    cases where the loss of several decimal digits of resolution
///    is acceptable.  The J2000 method is best matched to the way
///    the argument is handled internally and will deliver the
///    optimum resolution.  The MJD method and the date & time methods
///    are both good compromises between resolution and convenience.
///
/// 2) The proper motion in RA is dRA/dt rather than cos(Dec)*dRA/dt.
///
/// 3) The FK5 to Hipparcos transformation is modeled as a pure rotation
///    and spin;  zonal errors in the FK5 catalog are not taken into
///    account.
///
/// 4) It was the intention that Hipparcos should be a close
///    approximation to an inertial frame, so that distant objects have
///    zero proper motion;  such objects have (in general) non-zero
///    proper motion in FK5, and this function returns those fictitious
///    proper motions.
///
/// 5) The position returned by this function is in the FK5 J2000.0
///    reference system but at date date1+date2.
///
/// 6) See also fk52h, h2fk5, fk5hz.
///
/// Called:
///    s2c       spherical coordinates to unit vector
///    fk5hip    FK5 to Hipparcos rotation and spin
///    rxp       product of r-matrix and p-vector
///    sxp       multiply p-vector by scalar
///    rv2m      r-vector to r-matrix
///    rxr       product of two r-matrices
///    trxp      product of transpose of r-matrix and p-vector
///    pxp       vector product of two p-vectors
///    pv2s      pv-vector to spherical
///    anp       normalize angle into range 0 to 2pi
///
/// Reference:
///    F.Mignard & M.Froeschle, 2000, Astron.Astrophys. 354, 732-739.
pub fn hfk5z(rh: f64, dh: f64, date1: f64, date2: f64) -> (f64, f64, f64, f64) {
    let mut sh = [0.0; 3];
    let mut rst = [[0.0; 3]; 3];
    let mut r5ht = [[0.0; 3]; 3];
    let mut pv5e = [[0.0; 3]; 2];

    /* Time interval from fundamental epoch J2000.0 to given date (JY). */
    let t = ((date1 - DJ00) + date2) / DJY;

    /* Hipparcos barycentric position vector (normalized). */
    let ph = s2c(rh, dh);

    /* FK5 to Hipparcos orientation matrix and spin vector. */
    let (r5h, s5h) = fk5hip();

    /* Rotate the spin into the Hipparcos system. */
    rxp(&r5h, &s5h, &mut sh);

    /* Accumulated Hipparcos wrt FK5 spin over that interval. */
    let vst = sxp(t, &s5h);

    /* Express the accumulated spin as a rotation matrix. */
    rv2m(&vst, &mut rst);

    /* Rotation matrix:  accumulated spin, then FK5 to Hipparcos. */
    rxr(&r5h, &rst, &mut r5ht);

    /* De-orient & de-spin the Hipparcos position into FK5 J2000.0. */
    trxp(&r5ht, &ph, &mut pv5e[0]);

    /* Apply spin to the position giving a space motion. */
    let vv = pxp(&sh, &ph);

    /* De-orient & de-spin the Hipparcos space motion into FK5 J2000.0. */
    trxp(&r5ht, &vv, &mut pv5e[1]);

    /* FK5 position/velocity pv-vector to spherical. */
    let (w, d5, _, dr5, dd5, _) = pv2s(&pv5e);
    let r5 = anp(w);

    (r5, d5, dr5, dd5)
}
