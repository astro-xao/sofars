use crate::consts::{DAS2R, TURNAS};
use std::ops::Rem;

/// Fundamental argument, IERS Conventions (2003):
/// mean anomaly of the Moon.
///
/// Status:  canonical model.
///
/// Given:
///    t     f64    TDB, Julian centuries since J2000.0 (Note 1)
///
/// Returned (function value):
///          f64    l, radians (Note 2)
///
/// Notes:
///
/// 1) Though t is strictly TDB, it is usually more convenient to use
///    TT, which makes no significant difference.
///
/// 2) The expression used is as adopted in IERS Conventions (2003) and
///    is from Simon et al. (1994).
///
/// References:
///
///    McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///    IERS Technical Note No. 32, BKG (2004)
///
///    Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///    Francou, G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
pub fn fal03(t: f64) -> f64 {
    /* Mean anomaly of the Moon (IERS Conventions 2003). */
    (485868.249036 + t * (1717915923.2178 + t * (31.8792 + t * (0.051635 + t * -0.00024470))))
        .rem(TURNAS)
        * DAS2R
}
