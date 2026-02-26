use super::obl06;
use crate::consts::{DAS2R, DJ00, DJC};

/// Precession angles, IAU 2006, equinox based.
///
/// This function is part of the International Astronomical Union's
/// SOFA (Standards of Fundamental Astronomy) software collection.
///
/// Status:  canonical models.
///
/// Given:
///    date1,date2   f64   TT as a 2-part Julian Date (Note 1)
///
/// Returned (see Note 2):
///    eps0          f64   epsilon_0
///    psia          f64   psi_A
///    oma           f64   omega_A
///    bpa           f64   P_A
///    bqa           f64   Q_A
///    pia           f64   pi_A
///    bpia          f64   Pi_A
///    epsa          f64   obliquity epsilon_A
///    chia          f64   chi_A
///    za            f64   z_A
///    zetaa         f64   zeta_A
///    thetaa        f64   theta_A
///    pa            f64   p_A
///    gam           f64   F-W angle gamma_J2000
///    phi           f64   F-W angle phi_J2000
///    psi           f64   F-W angle psi_J2000
///
/// Notes:
///
/// 1) The TT date date1+date2 is a Julian Date, apportioned in any
///    convenient way between the two arguments.  For example,
///    JD(TT)=2450123.7 could be expressed in any of these ways,
///    among others:
///
/// ```text
///           date1          date2
///
///        2450123.7           0.0       (JD method)
///        2451545.0       -1421.3       (J2000 method)
///        2400000.5       50123.2       (MJD method)
///        2450123.5           0.2       (date & time method)
/// ```
///
///    The JD method is the most natural and convenient to use in
///    cases where the loss of several decimal digits of resolution
///    is acceptable.  The J2000 method is best matched to the way
///    the argument is handled internally and will deliver the
///    optimum resolution.  The MJD method and the date & time methods
///    are both good compromises between resolution and convenience.
///
/// 2) This function returns the set of equinox based angles for the
///    Capitaine et al. "P03" precession theory, adopted by the IAU in
///    2006.  The angles are set out in Table 1 of Hilton et al. (2006):
///
///    eps0   epsilon_0   obliquity at J2000.0
///    psia   psi_A       luni-solar precession
///    oma    omega_A     inclination of equator wrt J2000.0 ecliptic
///    bpa    P_A         ecliptic pole x, J2000.0 ecliptic triad
///    bqa    Q_A         ecliptic pole -y, J2000.0 ecliptic triad
///    pia    pi_A        angle between moving and J2000.0 ecliptics
///    bpia   Pi_A        longitude of ascending node of the ecliptic
///    epsa   epsilon_A   obliquity of the ecliptic
///    chia   chi_A       planetary precession
///    za     z_A         equatorial precession: -3rd 323 Euler angle
///    zetaa  zeta_A      equatorial precession: -1st 323 Euler angle
///    thetaa theta_A     equatorial precession: 2nd 323 Euler angle
///    pa     p_A         general precession (n.b. see below)
///    gam    gamma_J2000 J2000.0 RA difference of ecliptic poles
///    phi    phi_J2000   J2000.0 codeclination of ecliptic pole
///    psi    psi_J2000   longitude difference of equator poles, J2000.0
///
///    The returned values are all radians.
///
///    Note that the t^5 coefficient in the series for p_A from
///    Capitaine et al. (2003) is incorrectly signed in Hilton et al.
///    (2006).
///
/// 3) Hilton et al. (2006) Table 1 also contains angles that depend on
///    models distinct from the P03 precession theory itself, namely the
///    IAU 2000A frame bias and nutation.  The quoted polynomials are
///    used in other SOFA functions:
///
///    . iauXy06  contains the polynomial parts of the X and Y series.
///
///    . iauS06  contains the polynomial part of the s+XY/2 series.
///
///    . iauPfw06  implements the series for the Fukushima-Williams
///      angles that are with respect to the GCRS pole (i.e. the variants
///      that include frame bias).
///
/// 4) The IAU resolution stipulated that the choice of parameterization
///    was left to the user, and so an IAU compliant precession
///    implementation can be constructed using various combinations of
///    the angles returned by the present function.
///
/// 5) The parameterization used by SOFA is the version of the Fukushima-
///    Williams angles that refers directly to the GCRS pole.  These
///    angles may be calculated by calling the function iauPfw06.  SOFA
///    also supports the direct computation of the CIP GCRS X,Y by
///    series, available by calling iauXy06.
///
/// 6) The agreement between the different parameterizations is at the
///    1 microarcsecond level in the present era.
///
/// 7) When constructing a precession formulation that refers to the GCRS
///    pole rather than the dynamical pole, it may (depending on the
///    choice of angles) be necessary to introduce the frame bias
///    explicitly.
///
/// References:
///
///    Capitaine, N., Wallace, P.T. & Chapront, J., 2003,
///    Astron.Astrophys., 412, 567
///
///    Hilton, J. et al., 2006, Celest.Mech.Dyn.Astron. 94, 351
pub fn p06e(
    date1: f64,
    date2: f64,
) -> (
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
) {
    /* Interval between fundamental date J2000.0 and given date (JC). */
    let t = ((date1 - DJ00) + date2) / DJC;

    /* Obliquity at J2000.0. */
    let eps0 = 84381.406 * DAS2R;

    /* Luni-solar precession. */
    let psia = (5038.481507
        + (-1.0790069 + (-0.00114045 + (0.000132851 + (-0.0000000951) * t) * t) * t) * t)
        * t
        * DAS2R;

    /* Inclination of mean equator with respect to the J2000.0 ecliptic. */
    let oma = eps0
        + (-0.025754
            + (0.0512623 + (-0.00772503 + (-0.000000467 + (0.0000003337) * t) * t) * t) * t)
            * t
            * DAS2R;

    /* Ecliptic pole x, J2000.0 ecliptic triad. */
    let bpa = (4.199094
        + (0.1939873 + (-0.00022466 + (-0.000000912 + (0.0000000120) * t) * t) * t) * t)
        * t
        * DAS2R;

    /* Ecliptic pole -y, J2000.0 ecliptic triad. */
    let bqa = (-46.811015
        + (0.0510283 + (0.00052413 + (-0.000000646 + (-0.0000000172) * t) * t) * t) * t)
        * t
        * DAS2R;

    /* Angle between moving and J2000.0 ecliptics. */
    let pia = (46.998973
        + (-0.0334926 + (-0.00012559 + (0.000000113 + (-0.0000000022) * t) * t) * t) * t)
        * t
        * DAS2R;

    /* Longitude of ascending node of the moving ecliptic. */
    let bpia = (629546.7936
        + (-867.95758 + (0.157992 + (-0.0005371 + (-0.00004797 + (0.000000072) * t) * t) * t) * t)
            * t)
        * DAS2R;

    /* Mean obliquity of the ecliptic. */
    let epsa = obl06(date1, date2);

    /* Planetary precession. */
    let chia = (10.556403
        + (-2.3814292 + (-0.00121197 + (0.000170663 + (-0.0000000560) * t) * t) * t) * t)
        * t
        * DAS2R;

    /* Equatorial precession: minus the third of the 323 Euler angles. */
    let za = (-2.650545
        + (2306.077181
            + (1.0927348 + (0.01826837 + (-0.000028596 + (-0.0000002904) * t) * t) * t) * t)
            * t)
        * DAS2R;

    /* Equatorial precession: minus the first of the 323 Euler angles. */
    let zetaa = (2.650545
        + (2306.083227
            + (0.2988499 + (0.01801828 + (-0.000005971 + (-0.0000003173) * t) * t) * t) * t)
            * t)
        * DAS2R;

    /* Equatorial precession: second of the 323 Euler angles. */
    let thetaa = (2004.191903
        + (-0.4294934 + (-0.04182264 + (-0.000007089 + (-0.0000001274) * t) * t) * t) * t)
        * t
        * DAS2R;

    /* General precession. */
    let pa = (5028.796195
        + (1.1054348 + (0.00007964 + (-0.000023857 + (-0.0000000383) * t) * t) * t) * t)
        * t
        * DAS2R;

    /* Fukushima-Williams angles for precession. */
    let gam = (10.556403
        + (0.4932044 + (-0.00031238 + (-0.000002788 + (0.0000000260) * t) * t) * t) * t)
        * t
        * DAS2R;

    let phi = eps0
        + (-46.811015
            + (0.0511269 + (0.00053289 + (-0.000000440 + (-0.0000000176) * t) * t) * t) * t)
            * t
            * DAS2R;

    let psi = (5038.481507
        + (1.5584176 + (-0.00018522 + (-0.000026452 + (-0.0000000148) * t) * t) * t) * t)
        * t
        * DAS2R;

    (
        eps0, psia, oma, bpa, bqa, pia, bpia, epsa, chia, za, zetaa, thetaa, pa, gam, phi, psi,
    )
}
