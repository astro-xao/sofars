use crate::vm::ppsp;

///  Update a pv-vector.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  vector/matrix support function.
///
///  Given:
///     dt       double           time interval
///     pv       double[2][3]     pv-vector
///
///  Returned:
///     upv      double[2][3]     p updated, v unchanged
///
///  Notes:
///
///  1) "Update" means "refer the position component of the vector
///     to a new date dt time units from the existing date".
///
///  2) The time units of dt must match those of the velocity.
///
pub fn pvu(dt: f64, pv: &[[f64; 3]; 2]) -> [[f64; 3]; 2] {
    let mut upv = [[0.0; 3]; 2];
    upv[0] = ppsp(&pv[0], dt, &pv[1]);
    upv[1] = pv[1];
    upv
}
