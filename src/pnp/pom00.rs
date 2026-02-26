use crate::vm::{ir, rx, ry, rz};

///  Form the matrix of polar motion for a given date, IAU 2000.
///
///  Given:
///     xp,yp    f64            coordinates of the pole (radians, Note 1)
///     sp       f64            the TIO locator s' (radians, Note 2)
///
///  Returned:
///              [[f64; 3]; 3]  polar-motion matrix (Note 3)
///
///  Notes:
///
///  1) The arguments xp and yp are the coordinates (in radians) of the
///     Celestial Intermediate Pole with respect to the International
///     Terrestrial Reference System (see IERS Conventions 2003),
///     measured along the meridians 0 and 90 deg west respectively.
///
///  2) The argument sp is the TIO locator s', in radians, which
///     positions the Terrestrial Intermediate Origin on the equator.  It
///     is obtained from polar motion observations by numerical
///     integration, and so is in essence unpredictable.  However, it is
///     dominated by a secular drift of about 47 microarcseconds per
///     century, and so can be taken into account by using s' = -47*t,
///     where t is centuries since J2000.0.  The function sp00
///     implements this approximation.
///
///  3) The matrix operates in the sense V(TRS) = rpom * V(CIP), meaning
///     that it is the final rotation when computing the pointing
///     direction to a celestial source.
///
///  Reference:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn pom00(xp: f64, yp: f64, sp: f64) -> [[f64; 3]; 3] {
    let mut rpom = [[0.0; 3]; 3];
    ir(&mut rpom);
    rz(sp, &mut rpom);
    ry(-xp, &mut rpom);
    rx(-yp, &mut rpom);
    rpom
}
