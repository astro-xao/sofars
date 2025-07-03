use crate::consts::SRS;
use crate::vm::pdp;

///  Apply stellar aberration
///
///  Apply aberration to transform natural direction into proper
///  direction.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  Given:
///    pnat    double[3]   natural direction to the source (unit vector)
///    v       double[3]   observer barycentric velocity in units of c
///    s       double      distance between the Sun and the observer (au)
///    bm1     double      sqrt(1-|v|^2): reciprocal of Lorenz factor
///
///  Returned:
///    ppr     double[3]   proper direction to source (unit vector)
///
///  Notes:
///
///  1) The algorithm is based on Expr. (7.40) in the Explanatory
///     Supplement (Urban & Seidelmann 2013), but with the following
///     changes:
///
///     o  Rigorous rather than approximate normalization is applied.
///
///     o  The gravitational potential term from Expr. (7) in
///        Klioner (2003) is added, taking into account only the Sun's
///        contribution.  This has a maximum effect of about
///        0.4 microarcsecond.
///
///  2) In almost all cases, the maximum accuracy will be limited by the
///     supplied velocity.  For example, if the SOFA iauEpv00 function is
///     used, errors of up to 5 microarcseconds could occur.
///
///  References:
///
///     Urban, S. & Seidelmann, P. K. (eds), Explanatory Supplement to
///     the Astronomical Almanac, 3rd ed., University Science Books
///     (2013).
///
///     Klioner, Sergei A., "A practical relativistic model for micro-
///     arcsecond astrometry in space", Astr. J. 125, 1580-1597 (2003).
///
///  Called:
///     iauPdp       scalar product of two p-vectors
pub fn ab(pnat: &[f64; 3], v: &[f64; 3], s: f64, bm1: f64) -> [f64; 3] {
    let mut p = [0.0; 3];
    let mut r2 = 0.0;

    let pdv = pdp(pnat, v);
    let w1 = 1.0 + pdv / (1.0 + bm1);
    let w2 = SRS / s; // SRS constant value

    for i in 0..3 {
        let w = pnat[i] * bm1 + w1 * v[i] + w2 * (v[i] - pdv * pnat[i]);
        p[i] = w;
        r2 += w * w;
    }

    let r = r2.sqrt();
    let mut ppr = [0.0; 3];
    for i in 0..3 {
        ppr[i] = p[i] / r;
    }

    ppr
}
