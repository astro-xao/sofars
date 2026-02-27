use crate::consts::DPI;

/// Transform geocentric coordinates to geodetic for a reference
/// ellipsoid of specified form.
///
/// Status:  support function.
///
/// Given:
///    a       f64     equatorial radius (Notes 2,4)
///    f       f64     flattening (Note 3)
///    xyz     [f64; 3]  geocentric vector (Note 4)
///
/// Returned:
///    (elong, phi, height)   (f64, f64, f64)
///
/// Returned (function value):
///    Result<(f64, f64, f64), i32> status:  0 = OK
///                                         -1 = illegal f
///                                         -2 = illegal a
///
/// Notes:
///
/// 1) This function is based on the GCONV2H Fortran subroutine by
///    Toshio Fukushima (see reference).
///
/// 2) The equatorial radius, a, can be in any units, but meters is
///    the conventional choice.
///
/// 3) The flattening, f, is (for the Earth) a value around 0.00335,
///    i.e. around 1/298.
///
/// 4) The equatorial radius, a, and the geocentric vector, xyz,
///    must be given in the same units, and determine the units of
///    the returned height, height.
///
/// 5) The inverse transformation is performed in the function gd2gce.
///
/// 6) The transformation for a standard ellipsoid (such as WGS84) can
///    more conveniently be performed by calling gc2gd, which uses a
///    numerical code to identify the required A and F values.
///
/// Reference:
///
///    Fukushima, T., "Transformation from Cartesian to geodetic
///    coordinates accelerated by Halley's method", J.Geodesy (2006)
///    79: 689-693
pub fn gc2gde(a: f64, f: f64, xyz: [f64; 3]) -> Result<(f64, f64, f64), i32> {
    /* ------------- */
    /* Preliminaries */
    /* ------------- */

    /* Validate ellipsoid parameters. */
    if f < 0.0 || f >= 1.0 {
        return Err(-1);
    }
    if a <= 0.0 {
        return Err(-2);
    }

    /* Functions of ellipsoid parameters (with further validation of f). */
    let aeps2 = a * a * 1e-32;
    let e2 = (2.0 - f) * f;
    let e4t = e2 * e2 * 1.5;
    let ec2 = 1.0 - e2;
    if ec2 <= 0.0 {
        return Err(-1);
    }
    let ec = ec2.sqrt();
    let b = a * ec;

    /* Cartesian components. */
    let x = xyz[0];
    let y = xyz[1];
    let z = xyz[2];

    /* Distance from polar axis squared. */
    let p2 = x * x + y * y;

    /* Longitude. */
    let elong = if p2 > 0.0 { y.atan2(x) } else { 0.0 };

    /* Unsigned z-coordinate. */
    let absz = z.abs();

    let (mut phi, height);

    /* Proceed unless polar case. */
    if p2 > aeps2 {
        /* Distance from polar axis. */
        let p = p2.sqrt();

        /* Normalization. */
        let s0 = absz / a;
        let pn = p / a;
        let zc = ec * s0;

        /* Prepare Newton correction factors. */
        let c0 = ec * pn;
        let c02 = c0 * c0;
        let c03 = c02 * c0;
        let s02 = s0 * s0;
        let s03 = s02 * s0;
        let a02 = c02 + s02;
        let a0 = a02.sqrt();
        let a03 = a02 * a0;
        let d0 = zc * a03 + e2 * s03;
        let f0 = pn * a03 - e2 * c03;

        /* Prepare Halley correction factor. */
        let b0 = e4t * s02 * c02 * pn * (a0 - ec);
        let s1 = d0 * f0 - b0 * s0;
        let cc = ec * (f0 * f0 - b0 * c0);

        /* Evaluate latitude and height. */
        phi = s1.atan2(cc);
        let s12 = s1 * s1;
        let cc2 = cc * cc;
        height = (p * cc + absz * s1 - a * (ec2 * s12 + cc2).sqrt()) / (s12 + cc2).sqrt();
    } else {
        /* Exception: pole. */
        phi = DPI / 2.0;
        height = absz - b;
    }

    /* Restore sign of latitude. */
    if z < 0.0 {
        phi = -phi;
    }

    /* OK status. */
    Ok((elong, phi, height))
}
