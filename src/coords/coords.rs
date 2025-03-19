use crate::consts::{GRS80, WGS72, WGS84};
use crate::vm::zp;
/// ecliptic to ICRS, IAU 2006
pub fn eceq06() {}

/// rotation matrix, ICRS to ecliptic, IAU 2006
pub fn ecm06() {}

/// ICRS to ecliptic, IAU 2006
pub fn eqec06() {}

/// ecliptic to ICRS, long term
pub fn lteceq() {}

/// rotation matrix, ICRS to ecliptic, long-term
pub fn ltecm() {}

/// ICRS to ecliptic, long term
pub fn lteqec() {}

/// transform IAU 1958 galactic coordinates to ICRS
pub fn g2icrs() {}

/// transform ICRS coordinates to IAU 1958 Galactic
pub fn icrs2g() {}

/// a,f for a nominated Earth reference ellipsoid
pub fn eform(n: i32) -> Result<(f64, f64), i32> {
    match n {
        WGS84 => Ok((6378137.0, 1.0 / 298.257223563)),
        GRS80 => Ok((6378137.0, 1.0 / 298.257222101)),
        WGS72 => Ok((6378135.0, 1.0 / 298.26)),
        _ => Err(-1),
    }
}

/// geocentric to geodetic for a nominated ellipsoid
pub fn gc2gd() {}

/// geocentric to geodetic given ellipsoid a,f
pub fn gc2gde() {}

/// geodetic to geocentric for a nominated ellipsoid
pub fn gd2gc(n: i32, elong: f64, phi: f64, height: f64, xyz: &mut [f64; 3]) -> Result<i32, i32> {
    match eform(n) {
        Ok((a, f)) => match gd2gce(a, f, elong, phi, height, xyz) {
            Ok(_) => Ok(0),
            Err(_) => {
                zp(xyz);
                Err(-2)
            }
        },
        Err(_) => return Err(-1),
    }
}

/// geodetic to geocentric given ellipsoid a,f
pub fn gd2gce(
    a: f64,
    f: f64,
    elong: f64,
    phi: f64,
    height: f64,
    xyz: &mut [f64; 3],
) -> Result<i32, i32> {
    let (sp, cp, mut w, d, ac, as_, r);

    /* Functions of geodetic latitude. */
    sp = phi.sin();
    cp = phi.cos();
    w = 1.0 - f;
    w = w * w;
    d = cp * cp + w * sp * sp;
    if d <= 0.0 {
        return Err(-1);
    }
    ac = a / d.sqrt();
    as_ = w * ac;

    /* Geocentric vector. */
    r = (ac + height) * cp;
    xyz[0] = r * elong.cos();
    xyz[1] = r * elong.sin();
    xyz[2] = (as_ + height) * sp;

    /* Success. */
    Ok(0)
}
