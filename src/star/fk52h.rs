use crate::astro::{pvstar, starpv};
use crate::star::fk5hip;
use crate::vm::{ppp, pxp, rxp};

/// Transform FK5 (J2000.0) star data into the Hipparcos system.
///
/// Status:  support function.
///
/// Given (all FK5, equinox J2000.0, epoch J2000.0):
///    r5      f64    RA (radians)
///    d5      f64    Dec (radians)
///    dr5     f64    proper motion in RA (dRA/dt, rad/Jyear)
///    dd5     f64    proper motion in Dec (dDec/dt, rad/Jyear)
///    px5     f64    parallax (arcsec)
///    rv5     f64    radial velocity (km/s, positive = receding)
///
/// Returned (all Hipparcos, epoch J2000.0):
///    rh      f64    RA (radians)
///    dh      f64    Dec (radians)
///    drh     f64    proper motion in RA (dRA/dt, rad/Jyear)
///    ddh     f64    proper motion in Dec (dDec/dt, rad/Jyear)
///    pxh     f64    parallax (arcsec)
///    rvh     f64    radial velocity (km/s, positive = receding)
///
/// Notes:
///
/// 1) This function transforms FK5 star positions and proper motions
///    into the system of the Hipparcos catalog.
///
/// 2) The proper motions in RA are dRA/dt rather than
///    cos(Dec)*dRA/dt, and are per year rather than per century.
///
/// 3) The FK5 to Hipparcos transformation is modeled as a pure
///    rotation and spin;  zonal errors in the FK5 catalog are not
///    taken into account.
///
/// 4) See also h2fk5, fk5hz, hfk5z.
///
/// Called:
///    starpv    star catalog data to space motion pv-vector
///    fk5hip    FK5 to Hipparcos rotation and spin
///    rxp       product of r-matrix and p-vector
///    pxp       vector product of two p-vectors
///    ppp       p-vector plus p-vector
///    pvstar    space motion pv-vector to star catalog data
///
/// Reference:
///    F.Mignard & M.Froeschle, Astron.Astrophys., 354, 732-739 (2000).
pub fn fk52h(
    r5: f64,
    d5: f64,
    dr5: f64,
    dd5: f64,
    px5: f64,
    rv5: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let mut pvh = [[0.0; 3]; 2];

    /* FK5 barycentric position/velocity pv-vector (normalized). */
    let (pv5, _) = starpv(r5, d5, dr5, dd5, px5, rv5);

    /* FK5 to Hipparcos orientation matrix and spin vector. */
    let (r5h, mut s5h) = fk5hip();

    /* Make spin units per day instead of per year. */
    for val in s5h.iter_mut() {
        *val /= 365.25;
    }

    /* Orient the FK5 position into the Hipparcos system. */
    rxp(&r5h, &pv5[0], &mut pvh[0]);

    /* Apply spin to the position giving an extra space motion component. */
    let wxp = pxp(&pv5[0], &s5h);

    /* Add this component to the FK5 space motion. */
    let vv = ppp(&wxp, &pv5[1]);

    /* Orient the FK5 space motion into the Hipparcos system. */
    rxp(&r5h, &vv, &mut pvh[1]);

    /* Hipparcos pv-vector to spherical. */
    let res = pvstar(&pvh).unwrap();
    let rh = res[0];
    let dh = res[1];
    let drh = res[2];
    let ddh = res[3];
    let pxh = res[4];
    let rvh = res[5];

    (rh, dh, drh, ddh, pxh, rvh)
}
