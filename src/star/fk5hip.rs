use crate::consts::DAS2R;
use crate::vm::rv2m;

/// FK5 to Hipparcos rotation and spin.
///
/// Status:  support function.
///
/// Returned:
///    r5h   [[f64; 3]; 3] r-matrix: FK5 rotation wrt Hipparcos (Note 2)
///    s5h   [f64; 3]      r-vector: FK5 spin wrt Hipparcos (Note 3)
///
/// Notes:
///
/// 1) This function models the FK5 to Hipparcos transformation as a
///    pure rotation and spin;  zonal errors in the FK5 catalog are not
///    taken into account.
///
/// 2) The r-matrix r5h operates in the sense:
///
///          P_Hipparcos = r5h x P_FK5
///
///    where P_FK5 is a p-vector in the FK5 frame, and P_Hipparcos is
///    the equivalent Hipparcos p-vector.
///
/// 3) The r-vector s5h represents the time derivative of the FK5 to
///    Hipparcos rotation.  The units are radians per year (Julian,
///    TDB).
///
/// Called:
///    rv2m      r-vector to r-matrix
///
/// Reference:
///    F.Mignard & M.Froeschle, Astron.Astrophys., 354, 732-739 (2000).
pub fn fk5hip() -> ([[f64; 3]; 3], [f64; 3]) {
    let mut r5h = [[0.0; 3]; 3];
    let mut s5h = [0.0; 3];

    /* FK5 wrt Hipparcos orientation and spin (radians, radians/year) */
    let epx = -19.9e-3 * DAS2R;
    let epy = -9.1e-3 * DAS2R;
    let epz = 22.9e-3 * DAS2R;

    let omx = -0.30e-3 * DAS2R;
    let omy = 0.60e-3 * DAS2R;
    let omz = 0.70e-3 * DAS2R;

    /* FK5 to Hipparcos orientation expressed as an r-vector. */
    let v = [epx, epy, epz];

    /* Re-express as an r-matrix. */
    rv2m(&v, &mut r5h);

    /* Hipparcos wrt FK5 spin expressed as an r-vector. */
    s5h[0] = omx;
    s5h[1] = omy;
    s5h[2] = omz;

    (r5h, s5h)
}
