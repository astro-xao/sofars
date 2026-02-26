use crate::vm::{ir, rx, rz};

pub fn fw2m(gamb: f64, phib: f64, psi: f64, eps: f64) -> [[f64; 3]; 3] {
    let mut r = [[0.0; 3]; 3];
    /* Construct the matrix. */
    ir(&mut r);
    rz(gamb, &mut r);
    rx(phib, &mut r);
    rz(-psi, &mut r);
    rx(-eps, &mut r);
    r
}
