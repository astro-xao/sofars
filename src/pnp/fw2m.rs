use crate::vm::{ir, rx, rz};

pub fn fw2m(gamb: f64, phib: f64, psi: f64, eps: f64, r: &mut [[f64; 3]; 3]) {
    /* Construct the matrix. */
    ir(r);
    rz(gamb, r);
    rx(phib, r);
    rz(-psi, r);
    rx(-eps, r);
}
