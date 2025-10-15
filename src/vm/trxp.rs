use super::{rxp, tr};

/// Multiply a p-vector by the transpose of an r-matrix.
pub fn trxp(r: &[[f64; 3]; 3], p: &[f64; 3], trp: &mut [f64; 3]) {
    let tr_ = &mut [[0.0; 3]; 3];
    tr(r, tr_);
    rxp(tr_, p, trp);
}
