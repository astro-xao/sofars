use crate::vm::{anp, c2s, rxp};

use super::{IauAstrom, IauLdBody, ab, ldn, pmpx};

pub fn atciqn(
    rc: f64,
    dc: f64,
    pr: f64,
    pd: f64,
    px: f64,
    rv: f64,
    astrom: &mut IauAstrom,
    n: i32,
    b: &[IauLdBody],
) -> (f64, f64) {

    /* Proper motion and parallax, giving BCRS coordinate direction. */
    let pco = pmpx(rc, dc, pr, pd, px, rv, astrom.pmt, astrom.eb);

    /* Light deflection, giving BCRS natural direction. */
    let pnat = ldn(n, b, &astrom.eb, &pco);

    /* Aberration, giving GCRS proper direction. */
    let ppr = ab(&pnat, &astrom.v, astrom.em, astrom.bm1);

    /* Bias-precession-nutation, giving CIRS proper direction. */
    let pi = &mut [0.0; 3];
    rxp(&astrom.bpn, &ppr, pi);

    /* CIRS RA,Dec. */
    let (w, di) = c2s(pi);
    let ri = anp(w);

    (ri, di)
}
