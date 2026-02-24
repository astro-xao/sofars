use crate::vm::{s2c, sepp};

///  Angular separation between two sets of spherical coordinates.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  vector/matrix support function.
///
///  Given:
///     al     double       first longitude (radians)
///     ap     double       first latitude (radians)
///     bl     double       second longitude (radians)
///     bp     double       second latitude (radians)
///
///  Returned (function value):
///            double       angular separation (radians)
///
///  Called:
///     iauS2c       spherical coordinates to unit vector
///     iauSepp      angular separation between two p-vectors
///
pub fn seps(al: f64, ap: f64, bl: f64, bp: f64) -> f64 {
    /* Spherical to Cartesian. */
    let ac = s2c(al, ap);
    let bc = s2c(bl, bp);

    /* Angle between the vectors. */
    sepp(&ac, &bc)
}
