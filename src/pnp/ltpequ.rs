use crate::consts::{D2PI, DAS2R};

///  Long-term precession of the equator.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  Given:
///     epj     double         Julian epoch (TT)
///
///  Returned (function value):
///             [f64; 3]      equator pole unit vector
///
///  Notes:
///
///  1) The returned vector is with respect to the J2000.0 mean equator
///     and equinox.
///
///  2) The Vondrak et al. (2011, 2012) 400 millennia precession model
///     agrees with the IAU 2006 precession at J2000.0 and stays within
///     100 microarcseconds during the 20th and 21st centuries.  It is
///     accurate to a few arcseconds throughout the historical period,
///     worsening to a few tenths of a degree at the end of the
///     +/- 200,000 year time span.
///
///  References:
///
///    Vondrak, J., Capitaine, N. and Wallace, P., 2011, New precession
///    expressions, valid for long time intervals, Astron.Astrophys. 534,
///    A22
///
///    Vondrak, J., Capitaine, N. and Wallace, P., 2012, New precession
///    expressions, valid for long time intervals (Corrigendum),
///    Astron.Astrophys. 541, C1
pub fn ltpequ(epj: f64) -> [f64; 3] {
    /* Polynomial coefficients */
    const NPOL: usize = 4;
    const XYPOL: [[f64; NPOL]; 2] = [
        [5453.282155, 0.4252841, -0.00037173, -0.000000152],
        [-73750.930350, -0.7675452, -0.00018725, 0.000000231],
    ];

    /* Periodic coefficients */
    const XYPER: [[f64; 5]; 14] = [
        [256.75, -819.940624, 75004.344875, 81491.287984, 1558.515853],
        [708.15, -8444.676815, 624.033993, 787.163481, 7774.939698],
        [274.20, 2600.009459, 1251.136893, 1251.296102, -2219.534038],
        [
            241.45,
            2755.175630,
            -1102.212834,
            -1257.950837,
            -2523.969396,
        ],
        [2309.00, -167.659835, -2660.664980, -2966.799730, 247.850422],
        [492.20, 871.855056, 699.291817, 639.744522, -846.485643],
        [396.10, 44.769698, 153.167220, 131.600209, -1393.124055],
        [288.90, -512.313065, -950.865637, -445.040117, 368.526116],
        [231.10, -819.415595, 499.754645, 584.522874, 749.045012],
        [1610.00, -538.071099, -145.188210, -89.756563, 444.704518],
        [620.00, -189.793622, 558.116553, 524.429630, 235.934465],
        [157.87, -402.922932, -23.923029, -13.549067, 374.049623],
        [220.30, 179.516345, -165.405086, -210.157124, -171.330180],
        [1200.00, -9.814756, 9.344131, -44.919798, -22.899655],
    ];
    const NPER: usize = XYPER.len();

    /* Centuries since J2000. */
    let t = (epj - 2000.0) / 100.0;

    /* Initialize X and Y accumulators. */
    let mut x = 0.0;
    let mut y = 0.0;

    /* Periodic terms. */
    let w = D2PI * t;
    for i in 0..NPER {
        let a = w / XYPER[i][0];
        let (s, c) = a.sin_cos();
        x += c * XYPER[i][1] + s * XYPER[i][3];
        y += c * XYPER[i][2] + s * XYPER[i][4];
    }

    /* Polynomial terms. */
    let mut w_poly = 1.0;
    for i in 0..NPOL {
        x += XYPOL[0][i] * w_poly;
        y += XYPOL[1][i] * w_poly;
        w_poly *= t;
    }

    /* X and Y (direction cosines). */
    x *= DAS2R;
    y *= DAS2R;

    /* Form the equator pole vector. */
    let mut w_pole = 1.0 - x * x - y * y;
    w_pole = if w_pole < 0.0 { 0.0 } else { w_pole.sqrt() };

    [x, y, w_pole]
}
