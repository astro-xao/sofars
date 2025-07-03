use super::{ppp, sxp};

pub fn ppsp(a: &[f64; 3], s: f64, b: &[f64; 3]) -> [f64; 3] {
    /* s*b. */
    let sb = sxp(s, b);

    /* a + s*b. */
    ppp(a, &sb)
}
