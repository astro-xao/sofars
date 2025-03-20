use super::{rxpv, tr};

pub fn trxpv(r: &[[f64; 3]; 3], pv: &[[f64; 3]; 2], trpv: &mut [[f64; 3]; 2]) {
    let tr_= &mut [[0.0; 3]; 3];

    /* Transpose of matrix r. */
    tr(r, tr_);

    /* Matrix tr * vector pv -> vector trpv. */
    rxpv(tr_, pv, trpv);
}