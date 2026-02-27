use crate::consts::{DAS2R, TURNAS};
use std::ops::Rem;

/// Fundamental argument, IERS Conventions (2003):
/// mean elongation of the Moon from the Sun.
///
/// Status:  canonical model.
///
/// Given:
///    t     f64    TDB, Julian centuries since J2000.0 (Note 1)
///
/// Returned (function value):
///          f64    D, radians (Note 2)
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
pub fn fad03(t: f64) -> f64 {
    /* Mean elongation of the Moon from the Sun (IERS Conventions 2003). */
    (1072260.703692 + t * (1602961601.2090 + t * (-6.3706 + t * (0.006593 + t * -0.00003169))))
        .rem(TURNAS)
        * DAS2R
}
