use crate::consts::D2PI;
use std::ops::Rem;

/// Fundamental argument, IERS Conventions (2003):
/// mean longitude of Mercury.
///
/// Status:  canonical model.
///
/// Given:
///    t     f64    TDB, Julian centuries since J2000.0 (Note 1)
///
/// Returned (function value):
///          f64    mean longitude of Mercury, radians (Note 2)
///
/// Notes:
///
/// 1) Though t is strictly TDB, it is usually more convenient to use
///    TT, which makes no significant difference.
///
/// 2) The expression used is as adopted in IERS Conventions (2003) and
///    comes from Souchay et al. (1999) after Simon et al. (1994).
///
/// References:
///
///    McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///    IERS Technical Note No. 32, BKG (2004)
///
///    Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///    Francou, G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
///    Souchay, J., Loysel, B., Kinoshita, H., Folgueira, M. 1999,
///    Astron.Astrophys.Supp.Ser. 135, 111
pub fn fame03(t: f64) -> f64 {
    /* Mean longitude of Mercury (IERS Conventions 2003). */
    (4.402608842 + 2608.7903141574 * t).rem(D2PI)
}
