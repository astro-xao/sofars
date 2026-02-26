use crate::consts::{D2PI, DAS2R};

///  Long-term precession of the ecliptic.
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
///             [f64; 3]      ecliptic pole unit vector
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
pub fn ltpecl(epj: f64) -> [f64; 3] {
    /* Obliquity at J2000.0 (radians). */
    const EPS0: f64 = 84381.406 * DAS2R;

    /* Polynomial coefficients */
    const NPOL: usize = 4;
    const PQPOL: [[f64; NPOL]; 2] = [
        [5851.607687, -0.1189000, -0.00028913, 0.000000101],
        [-1600.886300, 1.1689818, -0.00000020, -0.000000437],
    ];

    /* Periodic coefficients */
    const PQPER: [[f64; 5]; 8] = [
        [708.15, -5486.751211, -684.661560, 667.666730, -5523.863691],
        [2309.00, -17.127623, 2446.283880, -2354.886252, -549.747450],
        [1620.00, -617.517403, 399.671049, -428.152441, -310.998056],
        [492.20, 413.442940, -356.652376, 376.202861, 421.535876],
        [1183.00, 78.614193, -186.387003, 184.778874, -36.776172],
        [622.00, -180.732815, -316.800070, 335.321713, -145.278396],
        [882.00, -87.676083, 198.296701, -185.138669, -34.744450],
        [547.00, 46.140315, 101.135679, -120.972830, 22.885731],
    ];
    const NPER: usize = PQPER.len();

    /* Centuries since J2000. */
    let t = (epj - 2000.0) / 100.0;

    /* Initialize P_A and Q_A accumulators. */
    let mut p = 0.0;
    let mut q = 0.0;

    /* Periodic terms. */
    let w = D2PI * t;
    for i in 0..NPER {
        let a = w / PQPER[i][0];
        let s = a.sin();
        let c = a.cos();
        p += c * PQPER[i][1] + s * PQPER[i][3];
        q += c * PQPER[i][2] + s * PQPER[i][4];
    }

    /* Polynomial terms. */
    let mut w_poly = 1.0;
    for i in 0..NPOL {
        p += PQPOL[0][i] * w_poly;
        q += PQPOL[1][i] * w_poly;
        w_poly *= t;
    }

    /* P_A and Q_A (radians). */
    p *= DAS2R;
    q *= DAS2R;

    /* Form the ecliptic pole vector. */
    let mut w_pole = 1.0 - p * p - q * q;
    w_pole = if w_pole < 0.0 { 0.0 } else { w_pole.sqrt() };
    let (s_eps0, c_eps0) = EPS0.sin_cos();

    [
        p,
        -q * c_eps0 - w_pole * s_eps0,
        -q * s_eps0 + w_pole * c_eps0,
    ]
}
