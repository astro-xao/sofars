use crate::vm::{pdp, pm, pxp};

///  Angular separation between two p-vectors.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  vector/matrix support function.
///
///  Given:
///     a      double[3]    first p-vector (not necessarily unit length)
///     b      double[3]    second p-vector (not necessarily unit length)
///
///  Returned (function value):
///            double       angular separation (radians, always positive)
///
///  Notes:
///
///  1) If either vector is null, a zero result is returned.
///
///  2) The angular separation is most simply formulated in terms of
///     scalar product.  However, this gives poor accuracy for angles
///     near zero and pi.  The present algorithm uses both cross product
///     and dot product, to deliver full accuracy whatever the size of
///     the angle.
///
///  Called:
///     iauPxp       vector product of two p-vectors
///     iauPm        modulus of p-vector
///     iauPdp       scalar product of two p-vectors
///
pub fn sepp(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    /* Sine of angle between the vectors, multiplied by the two moduli. */
    let axb = pxp(a, b);
    let ss = pm(axb);

    /* Cosine of the angle, multiplied by the two moduli. */
    let cs = pdp(a, b);

    /* The angle. */
    if ss != 0.0 || cs != 0.0 {
        ss.atan2(cs)
    } else {
        0.0
    }
}
