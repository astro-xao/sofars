use crate::fundargs::{fal03, falp03, faom03, fae03, fave03, fad03, faf03, fapa03};
use crate::consts::{DAS2R, DJC, DJ00};

///  Equation of the equinoxes complementary terms, IAU 2000
///
///  Equation of the equinoxes complementary terms, consistent with
///  IAU 2000 resolutions.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  canonical model.
///
///  Given:
///  ```
///     date1,date2  double   TT as a 2-part Julian Date (Note 1)
///  ```
///  Returned (function value):
///  ```
///                  double   complementary terms (Note 2)
///  ```
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///  ```
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///  ```
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The "complementary terms" are part of the equation of the
///     equinoxes (EE), classically the difference between apparent and
///     mean Sidereal Time:
///
///        GAST = GMST + EE
///
///     with:
///
///        EE = dpsi * cos(eps)
///
///     where dpsi is the nutation in longitude and eps is the obliquity
///     of date.  However, if the rotation of the Earth were constant in
///     an inertial frame the classical formulation would lead to
///     apparent irregularities in the UT1 timescale traceable to side-
///     effects of precession-nutation.  In order to eliminate these
///     effects from UT1, "complementary terms" were introduced in 1994
///     (IAU, 1994) and took effect from 1997 (Capitaine and Gontier,
///     1993):
///
///        GAST = GMST + CT + EE
///
///     By convention, the complementary terms are included as part of
///     the equation of the equinoxes rather than as part of the mean
///     Sidereal Time.  This slightly compromises the "geometrical"
///     interpretation of mean sidereal time but is otherwise
///     inconsequential.
///
///     The present function computes CT in the above expression,
///     compatible with IAU 2000 resolutions (Capitaine et al., 2002, and
///     IERS Conventions 2003).
///
///  Called:
///  ```
///     iauFal03     mean anomaly of the Moon
///     iauFalp03    mean anomaly of the Sun
///     iauFaf03     mean argument of the latitude of the Moon
///     iauFad03     mean elongation of the Moon from the Sun
///     iauFaom03    mean longitude of the Moon's ascending node
///     iauFave03    mean longitude of Venus
///     iauFae03     mean longitude of Earth
///     iauFapa03    general accumulated precession in longitude
///  ```
///  References:
///
///     Capitaine, N. & Gontier, A.-M., Astron.Astrophys., 275,
///     645-650 (1993)
///
///     Capitaine, N., Wallace, P.T. and McCarthy, D.D., "Expressions to
///     implement the IAU 2000 definition of UT1", Astron.Astrophys., 406,
///     1135-1149 (2003)
///
///     IAU Resolution C7, Recommendation 3 (1994)
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn eect00(date1: f64, date2: f64) -> f64 {
    /* Time since J2000.0, in Julian centuries */
    let t: f64;

    /* Miscellaneous */
    let mut a: f64;
    let mut s0: f64;
    let mut s1: f64;

    /* Fundamental arguments */
    let mut fa = [0.0; 8];

    /* Returned value. */
    let eect: f64;

    /* ----------------------------------------- */
    /* The series for the EE complementary terms */
    /* ----------------------------------------- */

    struct Term {
        nfa: [i32; 8], /* coefficients of l,l',F,D,Om,LVe,LE,pA */
        s: f64,        /* sine coefficient */
        c: f64,        /* cosine coefficient */
    }

    /* Terms of order t^0 */
    const E0: [Term; 33] = [
        /* 1-10 */
        Term { nfa: [0, 0, 0, 0, 1, 0, 0, 0], s: 2640.96e-6, c: -0.39e-6 },
        Term { nfa: [0, 0, 0, 0, 2, 0, 0, 0], s: 63.52e-6, c: -0.02e-6 },
        Term { nfa: [0, 0, 2, -2, 3, 0, 0, 0], s: 11.75e-6, c: 0.01e-6 },
        Term { nfa: [0, 0, 2, -2, 1, 0, 0, 0], s: 11.21e-6, c: 0.01e-6 },
        Term { nfa: [0, 0, 2, -2, 2, 0, 0, 0], s: -4.55e-6, c: 0.00e-6 },
        Term { nfa: [0, 0, 2, 0, 3, 0, 0, 0], s: 2.02e-6, c: 0.00e-6 },
        Term { nfa: [0, 0, 2, 0, 1, 0, 0, 0], s: 1.98e-6, c: 0.00e-6 },
        Term { nfa: [0, 0, 0, 0, 3, 0, 0, 0], s: -1.72e-6, c: 0.00e-6 },
        Term { nfa: [0, 1, 0, 0, 1, 0, 0, 0], s: -1.41e-6, c: -0.01e-6 },
        Term { nfa: [0, 1, 0, 0, -1, 0, 0, 0], s: -1.26e-6, c: -0.01e-6 },
        /* 11-20 */
        Term { nfa: [1, 0, 0, 0, -1, 0, 0, 0], s: -0.63e-6, c: 0.00e-6 },
        Term { nfa: [1, 0, 0, 0, 1, 0, 0, 0], s: -0.63e-6, c: 0.00e-6 },
        Term { nfa: [0, 1, 2, -2, 3, 0, 0, 0], s: 0.46e-6, c: 0.00e-6 },
        Term { nfa: [0, 1, 2, -2, 1, 0, 0, 0], s: 0.45e-6, c: 0.00e-6 },
        Term { nfa: [0, 0, 4, -4, 4, 0, 0, 0], s: 0.36e-6, c: 0.00e-6 },
        Term { nfa: [0, 0, 1, -1, 1, -8, 12, 0], s: -0.24e-6, c: -0.12e-6 },
        Term { nfa: [0, 0, 2, 0, 0, 0, 0, 0], s: 0.32e-6, c: 0.00e-6 },
        Term { nfa: [0, 0, 2, 0, 2, 0, 0, 0], s: 0.28e-6, c: 0.00e-6 },
        Term { nfa: [1, 0, 2, 0, 3, 0, 0, 0], s: 0.27e-6, c: 0.00e-6 },
        Term { nfa: [1, 0, 2, 0, 1, 0, 0, 0], s: 0.26e-6, c: 0.00e-6 },
        /* 21-30 */
        Term { nfa: [0, 0, 2, -2, 0, 0, 0, 0], s: -0.21e-6, c: 0.00e-6 },
        Term { nfa: [0, 1, -2, 2, -3, 0, 0, 0], s: 0.19e-6, c: 0.00e-6 },
        Term { nfa: [0, 1, -2, 2, -1, 0, 0, 0], s: 0.18e-6, c: 0.00e-6 },
        Term { nfa: [0, 0, 0, 0, 0, 8, -13, -1], s: -0.10e-6, c: 0.05e-6 },
        Term { nfa: [0, 0, 0, 2, 0, 0, 0, 0], s: 0.15e-6, c: 0.00e-6 },
        Term { nfa: [2, 0, -2, 0, -1, 0, 0, 0], s: -0.14e-6, c: 0.00e-6 },
        Term { nfa: [1, 0, 0, -2, 1, 0, 0, 0], s: 0.14e-6, c: 0.00e-6 },
        Term { nfa: [0, 1, 2, -2, 2, 0, 0, 0], s: -0.14e-6, c: 0.00e-6 },
        Term { nfa: [1, 0, 0, -2, -1, 0, 0, 0], s: 0.14e-6, c: 0.00e-6 },
        Term { nfa: [0, 0, 4, -2, 4, 0, 0, 0], s: 0.13e-6, c: 0.00e-6 },
        /* 31-33 */
        Term { nfa: [0, 0, 2, -2, 4, 0, 0, 0], s: -0.11e-6, c: 0.00e-6 },
        Term { nfa: [1, 0, -2, 0, -3, 0, 0, 0], s: 0.11e-6, c: 0.00e-6 },
        Term { nfa: [1, 0, -2, 0, -1, 0, 0, 0], s: 0.11e-6, c: 0.00e-6 },
    ];

    /* Terms of order t^1 */
    const E1: [Term; 1] = [
        Term { nfa: [0, 0, 0, 0, 1, 0, 0, 0], s: -0.87e-6, c: 0.00e-6 },
    ];

    /* Number of terms in the series */
    const NE0: usize = E0.len();
    const NE1: usize = E1.len();

    /* ------------------------------------------------------------------ */

    /* Interval between fundamental epoch J2000.0 and current date (JC). */
    t = ((date1 - DJ00) + date2) / DJC;

    /* Fundamental Arguments (from IERS Conventions 2003) */

    /* Mean anomaly of the Moon. */
    fa[0] = fal03(t);

    /* Mean anomaly of the Sun. */
    fa[1] = falp03(t);

    /* Mean longitude of the Moon minus that of the ascending node. */
    fa[2] = faf03(t);

    /* Mean elongation of the Moon from the Sun. */
    fa[3] = fad03(t);

    /* Mean longitude of the ascending node of the Moon. */
    fa[4] = faom03(t);

    /* Mean longitude of Venus. */
    fa[5] = fave03(t);

    /* Mean longitude of Earth. */
    fa[6] = fae03(t);

    /* General precession in longitude. */
    fa[7] = fapa03(t);

    /* Evaluate the EE complementary terms. */
    s0 = 0.0;
    s1 = 0.0;

    for i in (0..NE0).rev() {
        a = 0.0;
        for j in 0..8 {
            a += (E0[i].nfa[j] as f64) * fa[j];
        }
        s0 += E0[i].s * a.sin() + E0[i].c * a.cos();
    }

    for i in (0..NE1).rev() {
        a = 0.0;
        for j in 0..8 {
            a += (E1[i].nfa[j] as f64) * fa[j];
        }
        s1 += E1[i].s * a.sin() + E1[i].c * a.cos();
    }

    eect = (s0 + s1 * t) * DAS2R;

    eect
}