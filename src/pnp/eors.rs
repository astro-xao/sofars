///  Equation of the origins, given the classical NPB matrix and the
///  quantity s.
///
///  Given:
///     rnpb  [[f64; 3]; 3]  classical nutation x precession x bias matrix
///     s     f64            the quantity s (the CIO locator) in radians
///
///  Returned (function value):
///           f64            the equation of the origins in radians
///
///  Notes:
///
///  1)  The equation of the origins is the distance between the true
///      equinox and the celestial intermediate origin and, equivalently,
///      the difference between Earth rotation angle and Greenwich
///      apparent sidereal time (ERA-GST).  It comprises the precession
///      (since J2000.0) in right ascension plus the equation of the
///      equinoxes (including the small correction terms).
///
///  2)  The algorithm is from Wallace & Capitaine (2006).
///
///  References:
///
///     Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855
///
///     Wallace, P. & Capitaine, N., 2006, Astron.Astrophys. 459, 981
pub fn eors(rnpb: &[[f64; 3]; 3], s: f64) -> f64 {
    let x = rnpb[2][0];
    let ax = x / (1.0 + rnpb[2][2]);
    let xs = 1.0 - ax * x;
    let ys = -ax * rnpb[2][1];
    let zs = -x;
    let p = rnpb[0][0] * xs + rnpb[0][1] * ys + rnpb[0][2] * zs;
    let q = rnpb[1][0] * xs + rnpb[1][1] * ys + rnpb[1][2] * zs;
    let eo = if p != 0.0 || q != 0.0 {
        s - q.atan2(p)
    } else {
        s
    };

    eo
}
