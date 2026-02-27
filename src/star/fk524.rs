use crate::consts::DR2AS;
use crate::vm::{anp, pdp, pm, pmp, ppp, pv2s, s2pv, sxp};

/// Convert J2000.0 FK5 star catalog data to B1950.0 FK4.
///
/// Status:  support function.
///
/// Given: (all J2000.0, FK5)
///    r2000,d2000    f64   J2000.0 RA,Dec (rad)
///    dr2000,dd2000  f64   J2000.0 proper motions (rad/Jul.yr)
///    p2000          f64   parallax (arcsec)
///    v2000          f64   radial velocity (km/s, +ve = moving away)
///
/// Returned: (all B1950.0, FK4)
///    r1950,d1950    f64   B1950.0 RA,Dec (rad)
///    dr1950,dd1950  f64   B1950.0 proper motions (rad/trop.yr)
///    p1950          f64   parallax (arcsec)
///    v1950          f64   radial velocity (km/s, +ve = moving away)
///
/// Notes:
///
/// 1) The proper motions in RA are dRA/dt rather than cos(Dec)*dRA/dt,
///    and are per year rather than per century.
///
/// 2) The conversion is somewhat complicated, for several reasons:
///
///    . Change of standard epoch from J2000.0 to B1950.0.
///
///    . An intermediate transition date of 1984 January 1.0 TT.
///
///    . A change of precession model.
///
///    . Change of time unit for proper motion (Julian to tropical).
///
///    . FK4 positions include the E-terms of aberration, to simplify
///      the hand computation of annual aberration.  FK5 positions
///      assume a rigorous aberration computation based on the Earth's
///      barycentric velocity.
///
///    . The E-terms also affect proper motions, and in particular cause
///      objects at large distances to exhibit fictitious proper
///      motions.
///
///    The algorithm is based on Smith et al. (1989) and Yallop et al.
///    (1989), which presented a matrix method due to Standish (1982) as
///    developed by Aoki et al. (1983), using Kinoshita's development of
///    Andoyer's post-Newcomb precession.  The numerical constants from
///    Seidelmann (1992) are used canonically.
///
/// 4) In the FK4 catalog the proper motions of stars within 10 degrees
///    of the poles do not embody differential E-terms effects and
///    should, strictly speaking, be handled in a different manner from
///    stars outside these regions.  However, given the general lack of
///    homogeneity of the star data available for routine astrometry,
///    the difficulties of handling positions that may have been
///    determined from astrometric fields spanning the polar and non-
///    polar regions, the likelihood that the differential E-terms
///    effect was not taken into account when allowing for proper motion
///    in past astrometry, and the undesirability of a discontinuity in
///    the algorithm, the decision has been made in this SOFA algorithm
///    to include the effects of differential E-terms on the proper
///    motions for all stars, whether polar or not.  At epoch J2000.0,
///    and measuring "on the sky" rather than in terms of RA change, the
///    errors resulting from this simplification are less than
///    1 milliarcsecond in position and 1 milliarcsecond per century in
///    proper motion.
///
/// Called:
///    anp       normalize angle into range 0 to 2pi
///    pdp       scalar product of two p-vectors
///    pm        modulus of p-vector
///    pmp       p-vector minus p-vector
///    ppp       p-vector plus p-vector
///    pv2s      pv-vector to spherical coordinates
///    s2pv      spherical coordinates to pv-vector
///    sxp       multiply p-vector by scalar
///
/// References:
///
///    Aoki, S. et al., 1983, "Conversion matrix of epoch B1950.0
///    FK4-based positions of stars to epoch J2000.0 positions in
///    accordance with the new IAU resolutions".  Astron.Astrophys.
///    128, 263-267.
///
///    Seidelmann, P.K. (ed), 1992, "Explanatory Supplement to the
///    Astronomical Almanac", ISBN 0-935702-68-7.
///
///    Smith, C.A. et al., 1989, "The transformation of astrometric
///    catalog systems to the equinox J2000.0".  Astron.J. 97, 265.
///
///    Standish, E.M., 1982, "Conversion of positions and proper motions
///    from B1950.0 to the IAU system at J2000.0".  Astron.Astrophys.,
///    115, 1, 20-22.
///
///    Yallop, B.D. et al., 1989, "Transformation of mean star places
///    from FK4 B1950.0 to FK5 J2000.0 using matrices in 6-space".
///    Astron.J. 97, 274.
pub fn fk524(
    r2000: f64,
    d2000: f64,
    dr2000: f64,
    dd2000: f64,
    p2000: f64,
    v2000: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    /* Radians per year to arcsec per century */
    let pmf = 100.0 * DR2AS;

    /* Small number to avoid arithmetic problems */
    let tiny = 1e-30;

    /* Km per sec to au per tropical century */
    /* = 86400 * 36524.2198782 / 149597870.7 */
    let vf = 21.095;

    /* Constant pv-vector (cf. Seidelmann 3.591-2, vectors A and Adot) */
    let a = [
        [-1.62557e-6, -0.31919e-6, -0.13843e-6],
        [1.245e-3, -1.580e-3, -0.659e-3],
    ];

    /* 3x2 matrix of pv-vectors (cf. Seidelmann 3.592-1, matrix M^-1) */
    let em: [[[[f64; 3]; 2]; 3]; 2] = [
        [
            [
                [0.9999256795, 0.0111814828, 0.0048590039],
                [-0.00000242389840, -0.00000002710544, -0.00000001177742],
            ],
            [
                [-0.0111814828, 0.9999374849, -0.0000271771],
                [0.00000002710544, -0.00000242392702, 0.00000000006585],
            ],
            [
                [-0.0048590040, -0.0000271557, 0.9999881946],
                [0.00000001177742, 0.00000000006585, -0.00000242404995],
            ],
        ],
        [
            [
                [-0.000551, 0.238509, -0.435614],
                [0.99990432, 0.01118145, 0.00485852],
            ],
            [
                [-0.238560, -0.002667, 0.012254],
                [-0.01118145, 0.99991613, -0.00002717],
            ],
            [
                [0.435730, -0.008541, 0.002117],
                [-0.00485852, -0.00002716, 0.99996684],
            ],
        ],
    ];

    /* The FK5 data (units radians and arcsec per Julian century). */
    let ur = dr2000 * pmf;
    let ud = dd2000 * pmf;
    let mut px = p2000;
    let mut rv = v2000;

    /* Express as a pv-vector. */
    let pxvf = px * vf;
    let w = rv * pxvf;
    let r0 = s2pv(r2000, d2000, 1.0, ur, ud, w);

    /* Convert pv-vector to Bessel-Newcomb system (cf. Seidelmann 3.592-1). */
    let mut r1 = [[0.0; 3]; 2];
    for i in 0..2 {
        for j in 0..3 {
            let mut w_val = 0.0;
            for k in 0..2 {
                for l in 0..3 {
                    w_val += em[i][j][k][l] * r0[k][l];
                }
            }
            r1[i][j] = w_val;
        }
    }

    /* Apply E-terms (equivalent to Seidelmann 3.592-3, one iteration). */
    let mut pv_vec = [[0.0; 3]; 2];

    /* Direction. */
    let mut w_val = pm(r1[0]);
    let mut p1 = sxp(pdp(&r1[0], &a[0]), &r1[0]);
    let mut p2 = sxp(w_val, &a[0]);
    p1 = pmp(&p2, &p1);
    p1 = ppp(&r1[0], &p1);

    /* Recompute length. */
    w_val = pm(p1);

    /* Direction. */
    p1 = sxp(pdp(&r1[0], &a[0]), &r1[0]);
    p2 = sxp(w_val, &a[0]);
    p1 = pmp(&p2, &p1);
    pv_vec[0] = ppp(&r1[0], &p1);

    /* Derivative. */
    p1 = sxp(pdp(&r1[0], &a[1]), &pv_vec[0]);
    p2 = sxp(w_val, &a[1]);
    p1 = pmp(&p2, &p1);
    pv_vec[1] = ppp(&r1[1], &p1);

    /* Revert to catalog form. */
    let (r_res, d_res, w_res, ur_res, ud_res, rd_res) = pv2s(&pv_vec);
    if px > tiny {
        rv = rd_res / pxvf;
        px = px / w_res;
    }

    /* Return the results. */
    let r1950 = anp(r_res);
    let d1950 = d_res;
    let dr1950 = ur_res / pmf;
    let dd2000 = ud_res / pmf;

    (r1950, d1950, dr1950, dd2000, px, rv)
}
