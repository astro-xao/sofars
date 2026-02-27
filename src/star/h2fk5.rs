use crate::astro::{pvstar, starpv};
use crate::star::fk5hip;
use crate::vm::{pmp, pxp, rxp, trxp};

/// Transform Hipparcos star data into the FK5 (J2000.0) system.
///
/// Status:  support function.
///
/// Given (all Hipparcos, epoch J2000.0):
///    rh      f64    RA (radians)
///    dh      f64    Dec (radians)
///    drh     f64    proper motion in RA (dRA/dt, rad/Jyear)
///    ddh     f64    proper motion in Dec (dDec/dt, rad/Jyear)
///    pxh     f64    parallax (arcsec)
///    rvh     f64    radial velocity (km/s, positive = receding)
///
/// Returned (all FK5, equinox J2000.0, epoch J2000.0):
///    r5      f64    RA (radians)
///    d5      f64    Dec (radians)
///    dr5     f64    proper motion in RA (dRA/dt, rad/Jyear)
///    dd5     f64    proper motion in Dec (dDec/dt, rad/Jyear)
///    px5     f64    parallax (arcsec)
///    rv5     f64    radial velocity (km/s, positive = receding)
///
/// Notes:
///
/// 1) This function transforms Hipparcos star positions and proper
///    motions into FK5 J2000.0.
///
/// 2) The proper motions in RA are dRA/dt rather than
///    cos(Dec)*dRA/dt, and are per year rather than per century.
///
/// 3) The FK5 to Hipparcos transformation is modeled as a pure
///    rotation and spin;  zonal errors in the FK5 catalog are not
///    taken into account.
///
/// 4) See also fk52h, fk5hz, hfk5z.
///
/// Called:
///    starpv    star catalog data to space motion pv-vector
///    fk5hip    FK5 to Hipparcos rotation and spin
///    rv2m      r-vector to r-matrix
///    rxp       product of r-matrix and p-vector
///    trxp      product of transpose of r-matrix and p-vector
///    pxp       vector product of two p-vectors
///    pmp       p-vector minus p-vector
///    pvstar    space motion pv-vector to star catalog data
///
/// Reference:
///    F.Mignard & M.Froeschle, Astron.Astrophys., 354, 732-739 (2000).
pub fn h2fk5(
    rh: f64,
    dh: f64,
    drh: f64,
    ddh: f64,
    pxh: f64,
    rvh: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let mut pv5 = [[0.0; 3]; 2];
    let mut sh = [0.0; 3];

    /* Hipparcos barycentric position/velocity pv-vector (normalized). */
    let (pvh, _) = starpv(rh, dh, drh, ddh, pxh, rvh);

    /* FK5 to Hipparcos orientation matrix and spin vector. */
    let (r5h, mut s5h) = fk5hip();

    /* Make spin units per day instead of per year. */
    for val in s5h.iter_mut() {
        *val /= 365.25;
    }

    /* Orient the spin into the Hipparcos system. */
    rxp(&r5h, &s5h, &mut sh);

    /* De-orient the Hipparcos position into the FK5 system. */
    trxp(&r5h, &pvh[0], &mut pv5[0]);

    /* Apply spin to the position giving an extra space motion component. */
    let wxp = pxp(&pvh[0], &sh);

    /* Subtract this component from the Hipparcos space motion. */
    let vv = pmp(&pvh[1], &wxp);

    /* De-orient the Hipparcos space motion into the FK5 system. */
    trxp(&r5h, &vv, &mut pv5[1]);

    /* FK5 pv-vector to spherical. */
    let res = pvstar(&pv5).unwrap();
    let r5 = res[0];
    let d5 = res[1];
    let dr5 = res[2];
    let dd5 = res[3];
    let px5 = res[4];
    let rv5 = res[5];

    (r5, d5, dr5, dd5, px5, rv5)
}
