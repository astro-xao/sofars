use crate::vm::{ir, rx, rz};

///  Form rotation matrix given the Fukushima-Williams angles.
///
///  Given:
///     gamb     f64            F-W angle gamma_bar (radians)
///     phib     f64            F-W angle phi_bar (radians)
///     psi      f64            F-W angle psi (radians)
///     eps      f64            F-W angle epsilon (radians)
///
///  Returned:
///              [[f64; 3]; 3]  rotation matrix
///
///  Notes:
///
///  1) Naming the following points:
///
///           e = J2000.0 ecliptic pole,
///           p = GCRS pole,
///           E = ecliptic pole of date,
///     and   P = CIP,
///
///     the four Fukushima-Williams angles are as follows:
///
///        gamb = gamma = epE
///        phib = phi = pE
///        psi = psi = pEP
///        eps = epsilon = EP
///
///  2) The matrix representing the combined effects of frame bias,
///     precession and nutation is:
///
///        NxPxB = R_1(-eps).R_3(-psi).R_1(phib).R_3(gamb)
///
///  3) The present function can construct three different matrices,
///     depending on which angles are supplied as the arguments gamb,
///     phib, psi and eps:
///
///     o  To obtain the nutation x precession x frame bias matrix,
///        first generate the four precession angles known conventionally
///        as gamma_bar, phi_bar, psi_bar and epsilon_A, then generate
///        the nutation components Dpsi and Depsilon and add them to
///        psi_bar and epsilon_A, and finally call the present function
///        using those four angles as arguments.
///
///     o  To obtain the precession x frame bias matrix, generate the
///        four precession angles and call the present function.
///
///     o  To obtain the frame bias matrix, generate the four precession
///        angles for date J2000.0 and call the present function.
///
///     The nutation-only and precession-only matrices can if necessary
///     be obtained by combining these three appropriately.
///
///  References:
///
///     Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855
///
///     Hilton, J. et al., 2006, Celest.Mech.Dyn.Astron. 94, 351
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
