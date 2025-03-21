use crate::vm::{ir, rx, rz};

///  Form the matrix of nutation.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  Given:
///     epsa        double         mean obliquity of date (Note 1)
///     dpsi,deps   double         nutation (Note 2)
///
///  Returned:
///     rmatn       double[3][3]   nutation matrix (Note 3)
///
///  Notes:
///
///
///  1) The supplied mean obliquity epsa, must be consistent with the
///     precession-nutation models from which dpsi and deps were obtained.
///
///  2) The caller is responsible for providing the nutation components;
///     they are in longitude and obliquity, in radians and are with
///     respect to the equinox and ecliptic of date.
///
///  3) The matrix operates in the sense V(true) = rmatn * V(mean),
///     where the p-vector V(true) is with respect to the true
///     equatorial triad of date and the p-vector V(mean) is with
///     respect to the mean equatorial triad of date.
///
///  Called:
///     iauIr        initialize r-matrix to identity
///     iauRx        rotate around X-axis
///     iauRz        rotate around Z-axis
///
///  Reference:
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992),
///     Section 3.222-3 (p114).
///
///  This revision:  2021 May 11
///
///  SOFA release 2023-10-11
///
///  Copyright (C) 2023 IAU SOFA Board.  See notes at end.
///
pub fn numat(epsa: f64, dpsi: f64, deps: f64, rmatn: &mut [[f64; 3]; 3]) {
    ir(rmatn);
    rx(epsa, rmatn);
    rz(-dpsi, rmatn);
    rx(-(epsa + deps), rmatn);
}
