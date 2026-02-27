use crate::vm::ppp;

/// Add one pv-vector to another.
///
/// Status:  vector/matrix support function.
///
/// Given:
///    a        [[f64; 3]; 2]      first pv-vector
///    b        [[f64; 3]; 2]      second pv-vector
///
/// Returned:
///    apb      [[f64; 3]; 2]      a + b
pub fn pvppv(a: &[[f64; 3]; 2], b: &[[f64; 3]; 2]) -> [[f64; 3]; 2] {
    let mut apb = [[0.0; 3]; 2];
    apb[0] = ppp(&a[0], &b[0]);
    apb[1] = ppp(&a[1], &b[1]);
    apb
}
