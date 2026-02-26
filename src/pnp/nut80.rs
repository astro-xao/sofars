use std::ops::Rem;

use crate::consts::{D2PI, DAS2R, DJ00, DJC};
use crate::vm::anpm;

struct Coeff {
    nl: i32,
    nlp: i32,
    nf: i32,
    nd: i32,
    nom: i32, /* coefficients of l,l',F,D,Om */
    sp: f64,
    spt: f64, /* longitude sine, 1 and t coefficients */
    ce: f64,
    cet: f64, /* obliquity cosine, 1 and t coefficients */
}

impl Coeff {
    const fn new(
        nl: i32,
        nlp: i32,
        nf: i32,
        nd: i32,
        nom: i32,
        sp: f64,
        spt: f64,
        ce: f64,
        cet: f64,
    ) -> Self {
        Self {
            nl,
            nlp,
            nf,
            nd,
            nom,
            sp,
            spt,
            ce,
            cet,
        }
    }
}

const X: [Coeff; 106] = [
    /* 1-10 */
    Coeff::new(0, 0, 0, 0, 1, -171996.0, -174.2, 92025.0, 8.9),
    Coeff::new(0, 0, 0, 0, 2, 2062.0, 0.2, -895.0, 0.5),
    Coeff::new(-2, 0, 2, 0, 1, 46.0, 0.0, -24.0, 0.0),
    Coeff::new(2, 0, -2, 0, 0, 11.0, 0.0, 0.0, 0.0),
    Coeff::new(-2, 0, 2, 0, 2, -3.0, 0.0, 1.0, 0.0),
    Coeff::new(1, -1, 0, -1, 0, -3.0, 0.0, 0.0, 0.0),
    Coeff::new(0, -2, 2, -2, 1, -2.0, 0.0, 1.0, 0.0),
    Coeff::new(2, 0, -2, 0, 1, 1.0, 0.0, 0.0, 0.0),
    Coeff::new(0, 0, 2, -2, 2, -13187.0, -1.6, 5736.0, -3.1),
    Coeff::new(0, 1, 0, 0, 0, 1426.0, -3.4, 54.0, -0.1),
    /* 11-20 */
    Coeff::new(0, 1, 2, -2, 2, -517.0, 1.2, 224.0, -0.6),
    Coeff::new(0, -1, 2, -2, 2, 217.0, -0.5, -95.0, 0.3),
    Coeff::new(0, 0, 2, -2, 1, 129.0, 0.1, -70.0, 0.0),
    Coeff::new(2, 0, 0, -2, 0, 48.0, 0.0, 1.0, 0.0),
    Coeff::new(0, 0, 2, -2, 0, -22.0, 0.0, 0.0, 0.0),
    Coeff::new(0, 2, 0, 0, 0, 17.0, -0.1, 0.0, 0.0),
    Coeff::new(0, 1, 0, 0, 1, -15.0, 0.0, 9.0, 0.0),
    Coeff::new(0, 2, 2, -2, 2, -16.0, 0.1, 7.0, 0.0),
    Coeff::new(0, -1, 0, 0, 1, -12.0, 0.0, 6.0, 0.0),
    Coeff::new(-2, 0, 0, 2, 1, -6.0, 0.0, 3.0, 0.0),
    /* 21-30 */
    Coeff::new(0, -1, 2, -2, 1, -5.0, 0.0, 3.0, 0.0),
    Coeff::new(2, 0, 0, -2, 1, 4.0, 0.0, -2.0, 0.0),
    Coeff::new(0, 1, 2, -2, 1, 4.0, 0.0, -2.0, 0.0),
    Coeff::new(1, 0, 0, -1, 0, -4.0, 0.0, 0.0, 0.0),
    Coeff::new(2, 1, 0, -2, 0, 1.0, 0.0, 0.0, 0.0),
    Coeff::new(0, 0, -2, 2, 1, 1.0, 0.0, 0.0, 0.0),
    Coeff::new(0, 1, -2, 2, 0, -1.0, 0.0, 0.0, 0.0),
    Coeff::new(0, 1, 0, 0, 2, 1.0, 0.0, 0.0, 0.0),
    Coeff::new(-1, 0, 0, 1, 1, 1.0, 0.0, 0.0, 0.0),
    Coeff::new(0, 1, 2, -2, 0, -1.0, 0.0, 0.0, 0.0),
    /* 31-40 */
    Coeff::new(0, 0, 2, 0, 2, -2274.0, -0.2, 977.0, -0.5),
    Coeff::new(1, 0, 0, 0, 0, 712.0, 0.1, -7.0, 0.0),
    Coeff::new(0, 0, 2, 0, 1, -386.0, -0.4, 200.0, 0.0),
    Coeff::new(1, 0, 2, 0, 2, -301.0, 0.0, 129.0, -0.1),
    Coeff::new(1, 0, 0, -2, 0, -158.0, 0.0, -1.0, 0.0),
    Coeff::new(-1, 0, 2, 0, 2, 123.0, 0.0, -53.0, 0.0),
    Coeff::new(0, 0, 0, 2, 0, 63.0, 0.0, -2.0, 0.0),
    Coeff::new(1, 0, 0, 0, 1, 63.0, 0.1, -33.0, 0.0),
    Coeff::new(-1, 0, 0, 0, 1, -58.0, -0.1, 32.0, 0.0),
    Coeff::new(-1, 0, 2, 2, 2, -59.0, 0.0, 26.0, 0.0),
    /* 41-50 */
    Coeff::new(1, 0, 2, 0, 1, -51.0, 0.0, 27.0, 0.0),
    Coeff::new(0, 0, 2, 2, 2, -38.0, 0.0, 16.0, 0.0),
    Coeff::new(2, 0, 0, 0, 0, 29.0, 0.0, -1.0, 0.0),
    Coeff::new(1, 0, 2, -2, 2, 29.0, 0.0, -12.0, 0.0),
    Coeff::new(2, 0, 2, 0, 2, -31.0, 0.0, 13.0, 0.0),
    Coeff::new(0, 0, 2, 0, 0, 26.0, 0.0, -1.0, 0.0),
    Coeff::new(-1, 0, 2, 0, 1, 21.0, 0.0, -10.0, 0.0),
    Coeff::new(-1, 0, 0, 2, 1, 16.0, 0.0, -8.0, 0.0),
    Coeff::new(1, 0, 0, -2, 1, -13.0, 0.0, 7.0, 0.0),
    Coeff::new(-1, 0, 2, 2, 1, -10.0, 0.0, 5.0, 0.0),
    /* 51-60 */
    Coeff::new(1, 1, 0, -2, 0, -7.0, 0.0, 0.0, 0.0),
    Coeff::new(0, 1, 2, 0, 2, 7.0, 0.0, -3.0, 0.0),
    Coeff::new(0, -1, 2, 0, 2, -7.0, 0.0, 3.0, 0.0),
    Coeff::new(1, 0, 2, 2, 2, -8.0, 0.0, 3.0, 0.0),
    Coeff::new(1, 0, 0, 2, 0, 6.0, 0.0, 0.0, 0.0),
    Coeff::new(2, 0, 2, -2, 2, 6.0, 0.0, -3.0, 0.0),
    Coeff::new(0, 0, 0, 2, 1, -6.0, 0.0, 3.0, 0.0),
    Coeff::new(0, 0, 2, 2, 1, -7.0, 0.0, 3.0, 0.0),
    Coeff::new(1, 0, 2, -2, 1, 6.0, 0.0, -3.0, 0.0),
    Coeff::new(0, 0, 0, -2, 1, -5.0, 0.0, 3.0, 0.0),
    /* 61-70 */
    Coeff::new(1, -1, 0, 0, 0, 5.0, 0.0, 0.0, 0.0),
    Coeff::new(2, 0, 2, 0, 1, -5.0, 0.0, 3.0, 0.0),
    Coeff::new(0, 1, 0, -2, 0, -4.0, 0.0, 0.0, 0.0),
    Coeff::new(1, 0, -2, 0, 0, 4.0, 0.0, 0.0, 0.0),
    Coeff::new(0, 0, 0, 1, 0, -4.0, 0.0, 0.0, 0.0),
    Coeff::new(1, 1, 0, 0, 0, -3.0, 0.0, 0.0, 0.0),
    Coeff::new(1, 0, 2, 0, 0, 3.0, 0.0, 0.0, 0.0),
    Coeff::new(1, -1, 2, 0, 2, -3.0, 0.0, 1.0, 0.0),
    Coeff::new(-1, -1, 2, 2, 2, -3.0, 0.0, 1.0, 0.0),
    Coeff::new(-2, 0, 0, 0, 1, -2.0, 0.0, 1.0, 0.0),
    /* 71-80 */
    Coeff::new(3, 0, 2, 0, 2, -3.0, 0.0, 1.0, 0.0),
    Coeff::new(0, -1, 2, 2, 2, -3.0, 0.0, 1.0, 0.0),
    Coeff::new(1, 1, 2, 0, 2, 2.0, 0.0, -1.0, 0.0),
    Coeff::new(-1, 0, 2, -2, 1, -2.0, 0.0, 1.0, 0.0),
    Coeff::new(2, 0, 0, 0, 1, 2.0, 0.0, -1.0, 0.0),
    Coeff::new(1, 0, 0, 0, 2, -2.0, 0.0, 1.0, 0.0),
    Coeff::new(3, 0, 0, 0, 0, 2.0, 0.0, 0.0, 0.0),
    Coeff::new(0, 0, 2, 1, 2, 2.0, 0.0, -1.0, 0.0),
    Coeff::new(-1, 0, 0, 0, 2, 1.0, 0.0, -1.0, 0.0),
    Coeff::new(1, 0, 0, -4, 0, -1.0, 0.0, 0.0, 0.0),
    /* 81-90 */
    Coeff::new(-2, 0, 2, 2, 2, 1.0, 0.0, -1.0, 0.0),
    Coeff::new(-1, 0, 2, 4, 2, -2.0, 0.0, 1.0, 0.0),
    Coeff::new(2, 0, 0, -4, 0, -1.0, 0.0, 0.0, 0.0),
    Coeff::new(1, 1, 2, -2, 2, 1.0, 0.0, -1.0, 0.0),
    Coeff::new(1, 0, 2, 2, 1, -1.0, 0.0, 1.0, 0.0),
    Coeff::new(-2, 0, 2, 4, 2, -1.0, 0.0, 1.0, 0.0),
    Coeff::new(-1, 0, 4, 0, 2, 1.0, 0.0, 0.0, 0.0),
    Coeff::new(1, -1, 0, -2, 0, 1.0, 0.0, 0.0, 0.0),
    Coeff::new(2, 0, 2, -2, 1, 1.0, 0.0, -1.0, 0.0),
    Coeff::new(2, 0, 2, 2, 2, -1.0, 0.0, 0.0, 0.0),
    /* 91-100 */
    Coeff::new(1, 0, 0, 2, 1, -1.0, 0.0, 0.0, 0.0),
    Coeff::new(0, 0, 4, -2, 2, 1.0, 0.0, 0.0, 0.0),
    Coeff::new(3, 0, 2, -2, 2, 1.0, 0.0, 0.0, 0.0),
    Coeff::new(1, 0, 2, -2, 0, -1.0, 0.0, 0.0, 0.0),
    Coeff::new(0, 1, 2, 0, 1, 1.0, 0.0, 0.0, 0.0),
    Coeff::new(-1, -1, 0, 2, 1, 1.0, 0.0, 0.0, 0.0),
    Coeff::new(0, 0, -2, 0, 1, -1.0, 0.0, 0.0, 0.0),
    Coeff::new(0, 0, 2, -1, 2, -1.0, 0.0, 0.0, 0.0),
    Coeff::new(0, 1, 0, 2, 0, -1.0, 0.0, 0.0, 0.0),
    Coeff::new(1, 0, -2, -2, 0, -1.0, 0.0, 0.0, 0.0),
    /* 101-106 */
    Coeff::new(0, -1, 2, 0, 1, -1.0, 0.0, 0.0, 0.0),
    Coeff::new(1, 1, 0, -2, 1, -1.0, 0.0, 0.0, 0.0),
    Coeff::new(1, 0, -2, 2, 0, -1.0, 0.0, 0.0, 0.0),
    Coeff::new(2, 0, 0, 2, 0, 1.0, 0.0, 0.0, 0.0),
    Coeff::new(0, 0, 2, 4, 2, -1.0, 0.0, 0.0, 0.0),
    Coeff::new(0, 1, 0, 1, 0, 1.0, 0.0, 0.0, 0.0),
];

///  Nutation, IAU 1980 model.
///
///  Given:
///     date1,date2   f64       TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     (dpsi, deps)  (f64,f64) nutation in longitude and obliquity (radians)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The nutation components are with respect to the ecliptic of
///     date.
///
///  Reference:
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992),
///     Section 3.222 (p111).
pub fn nut80(date1: f64, date2: f64) -> (f64, f64) {
    /* Units of 0.1 milliarcsecond to radians */
    const U2R: f64 = DAS2R / 1e4;

    /* Number of terms in the series */
    const NT: usize = X.len();

    /* Interval between fundamental epoch J2000.0 and given date (JC). */
    let t = ((date1 - DJ00) + date2) / DJC;

    /* --------------------- */
    /* Fundamental arguments */
    /* --------------------- */

    /* Mean longitude of Moon minus mean longitude of Moon's perigee. */
    let el = anpm(
        (485866.733 + (715922.633 + (31.310 + 0.064 * t) * t) * t) * DAS2R
            + (1325.0 * t).rem(1.0) * D2PI,
    );

    /* Mean longitude of Sun minus mean longitude of Sun's perigee. */
    let elp = anpm(
        (1287099.804 + (1292581.224 + (-0.577 - 0.012 * t) * t) * t) * DAS2R
            + (99.0 * t).rem(1.0) * D2PI,
    );

    /* Mean longitude of Moon minus mean longitude of Moon's node. */
    let f = anpm(
        (335778.877 + (295263.137 + (-13.257 + 0.011 * t) * t) * t) * DAS2R
            + (1342.0 * t).rem(1.0) * D2PI,
    );

    /* Mean elongation of Moon from Sun. */
    let d = anpm(
        (1072261.307 + (1105601.328 + (-6.891 + 0.019 * t) * t) * t) * DAS2R
            + (1236.0 * t).rem(1.0) * D2PI,
    );

    /* Longitude of the mean ascending node of the lunar orbit on the */
    /* ecliptic, measured from the mean equinox of date. */
    let om = anpm(
        (450160.280 + (-482890.539 + (7.455 + 0.008 * t) * t) * t) * DAS2R
            + (-5.0 * t).rem(1.0) * D2PI,
    );

    /* --------------- */
    /* Nutation series */
    /* --------------- */
    /* Initialize nutation components. */
    let mut dp = 0.0;
    let mut de = 0.0;

    /* Sum the nutation terms, ending with the biggest. */
    let (mut arg, mut s, mut c);
    for j in (0..NT).rev() {
        /* Form argument for current term. */
        arg = X[j].nl as f64 * el
            + X[j].nlp as f64 * elp
            + X[j].nf as f64 * f
            + X[j].nd as f64 * d
            + X[j].nom as f64 * om;

        /* Accumulate current nutation term. */
        s = X[j].sp + X[j].spt * t;
        c = X[j].ce + X[j].cet * t;
        if s != 0.0 {
            dp += s * arg.sin();
        }
        if c != 0.0 {
            de += c * arg.cos();
        }
    }

    /* Convert results from 0.1 mas units to radians. */
    let dpsi = dp * U2R;
    let deps = de * U2R;

    (dpsi, deps)
}
