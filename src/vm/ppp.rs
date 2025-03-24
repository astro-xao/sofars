///  P-vector addition.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  vector/matrix support function.
///
///  Given:
///  ```
///     a        double[3]      first p-vector
///     b        double[3]      second p-vector
///  ```
///
///  Returned:
///  ```
///     apb      double[3]      a + b
///  ```
///
///  Note:
///     It is permissible to re-use the same array for any of the
///     arguments.
pub fn ppp(a: &[f64; 3], b: [f64; 3]) -> [f64; 3]
{
   [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
