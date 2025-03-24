use crate::astro::ld;
use crate::consts::{AULT, DAYSEC};
use crate::vm::{cp, pdp, pmp, pn, ppsp};

use super::IauLdBody;

pub fn ldn(n: i32, b: &[IauLdBody], ob: &[f64; 3], sc: &[f64; 3]) -> [f64; 3] {
    let mut sn = [0.0; 3];
    /* Light time for 1 au (days) */
    const CR: f64 = AULT / DAYSEC;

    // int i;
    // double  v[3], dt, ev[3], em, e[3];

    /* Star direction prior to deflection. */
    cp(&sc, &mut sn);

    /* Body by body. */
    for i in 0..n {
        /* Body to observer vector at epoch of observation (au). */
        let v = pmp(ob, &b[i as usize].pv[0]);

        /* Minus the time since the light passed the body (days). */
        let dt = pdp(&mut sn, &v) * CR;

        /* Neutralize if the star is "behind" the observer. */
        let dt = dt.min(0.0);

        /* Backtrack the body to the time the light was passing the body. */
        let ev = ppsp(&v, -dt, &b[i as usize].pv[1]);

        /* Body to observer vector as magnitude and direction. */
        let (em, e) = pn(&ev);

        /* Apply light deflection for this body. */
        sn = ld(b[i as usize].bm, sn, sn, e, em, b[i as usize].dl);
        /* Next body. */
    }

    sn
}
