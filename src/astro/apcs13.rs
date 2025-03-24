use crate::eph::epv00;

use super::{apcs, IauAstrom};

pub fn apcs13(date1: f64, date2: f64, pv: &[[f64; 3]; 2], astrom: &mut IauAstrom) {
    /* Earth barycentric & heliocentric position/velocity (au, au/d). */
    let (ehpv, ebpv) = epv00(date1, date2).unwrap();

    /* Compute the star-independent astrometry parameters. */
    apcs(date1, date2, pv, &ebpv, &ehpv[0], astrom);
}