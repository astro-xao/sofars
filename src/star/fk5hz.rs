use crate::consts::{DJ00, DJY};
use crate::star::fk5hip;
use crate::vm::{anp, c2s, rv2m, rxp, s2c, sxp, trxp};

/// Transform an FK5 (J2000.0) star position into the system of the
/// Hipparcos catalog, assuming zero Hipparcos proper motion.
///
/// Status:  support function.
///
/// Given:
///    r5           f64   FK5 RA (radians), equinox J2000.0, at date
///    d5           f64   FK5 Dec (radians), equinox J2000.0, at date
///    date1,date2  f64   TDB date (Notes 1,2)
///
/// Returned:
///    rh           f64   Hipparcos RA (radians)
///    dh           f64   Hipparcos Dec (radians)
///
/// Notes:
///
/// 1) This function converts a star position from the FK5 system to
///    the Hipparcos system, in such a way that the Hipparcos proper
///    motion is zero.  Because such a star has, in general, a non-zero
///    proper motion in the FK5 system, the function requires the date
///    at which the position in the FK5 system was determined.
///
/// 2) The TT date date1+date2 is a Julian Date, apportioned in any
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
/// 3) The FK5 to Hipparcos transformation is modeled as a pure
///    rotation and spin;  zonal errors in the FK5 catalog are not
///    taken into account.
///
/// 4) The position returned by this function is in the Hipparcos
///    reference system but at date date1+date2.
///
/// 5) See also fk52h, h2fk5, hfk5z.
///
/// Called:
///    s2c       spherical coordinates to unit vector
///    fk5hip    FK5 to Hipparcos rotation and spin
///    sxp       multiply p-vector by scalar
///    rv2m      r-vector to r-matrix
///    trxp      product of transpose of r-matrix and p-vector
///    rxp       product of r-matrix and p-vector
///    c2s       p-vector to spherical
///    anp       normalize angle into range 0 to 2pi
///
/// Reference:
///    F.Mignard & M.Froeschle, 2000, Astron.Astrophys. 354, 732-739.
pub fn fk5hz(r5: f64, d5: f64, date1: f64, date2: f64) -> (f64, f64) {
    let mut rst = [[0.0; 3]; 3];
    let mut p5 = [0.0; 3];
    let mut ph = [0.0; 3];

    /* Interval from given date to fundamental epoch J2000.0 (JY). */
    let t = -((date1 - DJ00) + date2) / DJY;

    /* FK5 barycentric position vector. */
    let p5e = s2c(r5, d5);

    /* FK5 to Hipparcos orientation matrix and spin vector. */
    let (r5h, s5h) = fk5hip();

    /* Accumulated Hipparcos wrt FK5 spin over that interval. */
    let vst = sxp(t, &s5h);

    /* Express the accumulated spin as a rotation matrix. */
    rv2m(&vst, &mut rst);

    /* Derotate the vector's FK5 axes back to date. */
    trxp(&rst, &p5e, &mut p5);

    /* Rotate the vector into the Hipparcos system. */
    rxp(&r5h, &p5, &mut ph);

    /* Hipparcos vector to spherical. */
    let (w, dh) = c2s(&ph);
    let rh = anp(w);

    (rh, dh)
}
