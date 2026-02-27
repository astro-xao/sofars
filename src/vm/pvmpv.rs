use crate::vm::pmp;

/// Subtract one pv-vector from another.
///
/// Status:  vector/matrix support function.
///
/// Given:
///    a       [[f64; 3]; 2]      first pv-vector
///    b       [[f64; 3]; 2]      second pv-vector
///
/// Returned:
///    amb     [[f64; 3]; 2]      a - b
pub fn pvmpv(a: &[[f64; 3]; 2], b: &[[f64; 3]; 2]) -> [[f64; 3]; 2] {
    let mut amb = [[0.0; 3]; 2];
    amb[0] = pmp(&a[0], &b[0]);
    amb[1] = pmp(&a[1], &b[1]);
    amb
}
