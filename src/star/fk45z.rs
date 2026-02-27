use crate::cal::{epb2jd, epj};
use crate::consts::DR2AS;
use crate::vm::{anp, c2s, pdp, pmp, ppsp, pvu, s2c};

/// Convert a B1950.0 FK4 star position to J2000.0 FK5, assuming zero
/// proper motion in the FK5 system.
///
/// Status:  support function.
///
/// This function converts a star's catalog data from the old FK4
/// (Bessel-Newcomb) system to the later IAU 1976 FK5 (Fricke) system,
/// in such a way that the FK5 proper motion is zero.  Because such a
/// star has, in general, a non-zero proper motion in the FK4 system,
/// the function requires the epoch at which the position in the FK4
/// system was determined.
///
/// Given:
///    r1950,d1950    f64   B1950.0 FK4 RA,Dec at epoch (rad)
///    bepoch         f64   Besselian epoch (e.g. 1979.3)
///
/// Returned:
///    r2000,d2000    f64   J2000.0 FK5 RA,Dec (rad)
///
/// Notes:
///
/// 1) The epoch bepoch is strictly speaking Besselian, but if a
///    Julian epoch is supplied the result will be affected only to a
///    negligible extent.
///
/// 2) The method is from Appendix 2 of Aoki et al. (1983), but using
///    the constants of Seidelmann (1992).  See the function fk425
///    for a general introduction to the FK4 to FK5 conversion.
///
/// 3) Conversion from equinox B1950.0 FK4 to equinox J2000.0 FK5 only
///    is provided for.  Conversions for different starting and/or
///    ending epochs would require additional treatment for precession,
///    proper motion and E-terms.
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
/// Called:
///    anp       normalize angle into range 0 to 2pi
///    c2s       p-vector to spherical
///    epb2jd    Besselian epoch to Julian date
///    epj       Julian date to Julian epoch
///    pdp       scalar product of two p-vectors
///    pmp       p-vector minus p-vector
///    ppsp      p-vector plus scaled p-vector
///    pvu       update a pv-vector
///    s2c       spherical to p-vector
pub fn fk45z(r1950: f64, d1950: f64, bepoch: f64) -> (f64, f64) {
    /* Radians per year to arcsec per century */
    let pmf = 100.0 * DR2AS;

    /* Vectors A and Adot (Seidelmann 3.591-2) */
    let a = [-1.62557e-6, -0.31919e-6, -0.13843e-6];
    let ad = [1.245e-3, -1.580e-3, -0.659e-3];

    /* 3x2 matrix of p-vectors (cf. Seidelmann 3.591-4, matrix M) */
    let em: [[[f64; 3]; 3]; 2] = [
        [
            [0.9999256782, -0.0111820611, -0.0048579477],
            [0.0111820610, 0.9999374784, -0.0000271765],
            [0.0048579479, -0.0000271474, 0.9999881997],
        ],
        [
            [-0.000551, -0.238565, 0.435739],
            [0.238514, -0.002667, -0.008541],
            [-0.435623, 0.012254, 0.002117],
        ],
    ];

    /* Spherical coordinates to p-vector. */
    let r0 = s2c(r1950, d1950);

    /* Adjust p-vector A to give zero proper motion in FK5. */
    let mut w = (bepoch - 1950.0) / pmf;
    let mut p = ppsp(&a, w, &ad);

    /* Remove E-terms. */
    let p_term = ppsp(&p, -pdp(&r0, &p), &r0);
    p = pmp(&r0, &p_term);

    /* Convert to Fricke system pv-vector (cf. Seidelmann 3.591-3). */
    let mut pv = [[0.0; 3]; 2];
    for i in 0..2 {
        for j in 0..3 {
            let mut w_val = 0.0;
            for k in 0..3 {
                w_val += em[i][j][k] * p[k];
            }
            pv[i][j] = w_val;
        }
    }

    /* Allow for fictitious proper motion. */
    let (djm0, djm) = epb2jd(bepoch);
    w = (epj(djm0, djm) - 2000.0) / pmf;
    let pv_res = pvu(w, &pv);

    /* Revert to spherical coordinates. */
    let (w_res, d2000) = c2s(&pv_res[0]);
    let r2000 = anp(w_res);

    (r2000, d2000)
}
