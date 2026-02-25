use crate::consts::{DAU, DD2R, DJ00, DJC};
use crate::pnp::pfw06;
use crate::vm::{ir, rx, rxpv, rz, s2pv};

///  Approximate geocentric position and velocity of the Moon.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  n.b. Not IAU-endorsed and without canonical status.
///
///  Given:
///     date1  double         TT date part A (Notes 1,4)
///     date2  double         TT date part B (Notes 1,4)
///
///  Returned:
///     pv     [[f64; 3]; 2]  Moon p,v, GCRS (au, au/d, Note 5)
///
pub fn moon98(date1: f64, date2: f64) -> [[f64; 3]; 2] {
    /* Moon's mean longitude (wrt mean equinox and ecliptic of date) */
    const ELP0: f64 = 218.31665436; /* Simon et al. (1994). */
    const ELP1: f64 = 481267.88123421;
    const ELP2: f64 = -0.0015786;
    const ELP3: f64 = 1.0 / 538841.0;
    const ELP4: f64 = -1.0 / 65194000.0;

    /* Moon's mean elongation */
    const D0: f64 = 297.8501921;
    const D1: f64 = 445267.1114034;
    const D2: f64 = -0.0018819;
    const D3: f64 = 1.0 / 545868.0;
    const D4: f64 = 1.0 / 113065000.0;

    /* Sun's mean anomaly */
    const EM0: f64 = 357.5291092;
    const EM1: f64 = 35999.0502909;
    const EM2: f64 = -0.0001536;
    const EM3: f64 = 1.0 / 24490000.0;
    const EM4: f64 = 0.0;

    /* Moon's mean anomaly */
    const EMP0: f64 = 134.9633964;
    const EMP1: f64 = 477198.8675055;
    const EMP2: f64 = 0.0087414;
    const EMP3: f64 = 1.0 / 69699.0;
    const EMP4: f64 = -1.0 / 14712000.0;

    /* Mean distance of the Moon from its ascending node */
    const F0: f64 = 93.2720950;
    const F1: f64 = 483202.0175233;
    const F2: f64 = -0.0036539;
    const F3: f64 = 1.0 / 3526000.0;
    const F4: f64 = 1.0 / 863310000.0;

    /* Meeus A_1, due to Venus (deg) */
    const A10: f64 = 119.75;
    const A11: f64 = 131.849;

    /* Meeus A_2, due to Jupiter (deg) */
    const A20: f64 = 53.09;
    const A21: f64 = 479264.290;

    /* Meeus A_3, due to sidereal motion of the Moon in longitude (deg) */
    const A30: f64 = 313.45;
    const A31: f64 = 481266.484;

    /* Coefficients for Meeus additive terms (deg) */
    const AL1: f64 = 0.003958;
    const AL2: f64 = 0.001962;
    const AL3: f64 = 0.000318;
    const AB1: f64 = -0.002235;
    const AB2: f64 = 0.000382;
    const AB3: f64 = 0.000175;
    const AB4: f64 = 0.000175;
    const AB5: f64 = 0.000127;
    const AB6: f64 = -0.000115;

    /* Fixed term in distance (m) */
    const R0: f64 = 385000560.0;

    /* Coefficients for (dimensionless) E factor */
    const E1: f64 = -0.002516;
    const E2: f64 = -0.0000074;

    struct TermLR {
        nd: i32,
        nem: i32,
        nemp: i32,
        nf: i32,
        coefl: f64,
        coefr: f64,
    }

    const TLR: [TermLR; 60] = [
        TermLR { nd: 0, nem: 0, nemp: 1, nf: 0, coefl: 6.288774, coefr: -20905355.0 },
        TermLR { nd: 2, nem: 0, nemp: -1, nf: 0, coefl: 1.274027, coefr: -3699111.0 },
        TermLR { nd: 2, nem: 0, nemp: 0, nf: 0, coefl: 0.658314, coefr: -2955968.0 },
        TermLR { nd: 0, nem: 0, nemp: 2, nf: 0, coefl: 0.213618, coefr: -569925.0 },
        TermLR { nd: 0, nem: 1, nemp: 0, nf: 0, coefl: -0.185116, coefr: 48888.0 },
        TermLR { nd: 0, nem: 0, nemp: 0, nf: 2, coefl: -0.114332, coefr: -3149.0 },
        TermLR { nd: 2, nem: 0, nemp: -2, nf: 0, coefl: 0.058793, coefr: 246158.0 },
        TermLR { nd: 2, nem: -1, nemp: -1, nf: 0, coefl: 0.057066, coefr: -152138.0 },
        TermLR { nd: 2, nem: 0, nemp: 1, nf: 0, coefl: 0.053322, coefr: -170733.0 },
        TermLR { nd: 2, nem: -1, nemp: 0, nf: 0, coefl: 0.045758, coefr: -204586.0 },
        TermLR { nd: 0, nem: 1, nemp: -1, nf: 0, coefl: -0.040923, coefr: -129620.0 },
        TermLR { nd: 1, nem: 0, nemp: 0, nf: 0, coefl: -0.034720, coefr: 108743.0 },
        TermLR { nd: 0, nem: 1, nemp: 1, nf: 0, coefl: -0.030383, coefr: 104755.0 },
        TermLR { nd: 2, nem: 0, nemp: 0, nf: -2, coefl: 0.015327, coefr: 10321.0 },
        TermLR { nd: 0, nem: 0, nemp: 1, nf: 2, coefl: -0.012528, coefr: 0.0 },
        TermLR { nd: 0, nem: 0, nemp: 1, nf: -2, coefl: 0.010980, coefr: 79661.0 },
        TermLR { nd: 4, nem: 0, nemp: -1, nf: 0, coefl: 0.010675, coefr: -34782.0 },
        TermLR { nd: 0, nem: 0, nemp: 3, nf: 0, coefl: 0.010034, coefr: -23210.0 },
        TermLR { nd: 4, nem: 0, nemp: -2, nf: 0, coefl: 0.008548, coefr: -21636.0 },
        TermLR { nd: 2, nem: 1, nemp: -1, nf: 0, coefl: -0.007888, coefr: 24208.0 },
        TermLR { nd: 2, nem: 1, nemp: 0, nf: 0, coefl: -0.006766, coefr: 30824.0 },
        TermLR { nd: 1, nem: 0, nemp: -1, nf: 0, coefl: -0.005163, coefr: -8379.0 },
        TermLR { nd: 1, nem: 1, nemp: 0, nf: 0, coefl: 0.004987, coefr: -16675.0 },
        TermLR { nd: 2, nem: -1, nemp: 1, nf: 0, coefl: 0.004036, coefr: -12831.0 },
        TermLR { nd: 2, nem: 0, nemp: 2, nf: 0, coefl: 0.003994, coefr: -10445.0 },
        TermLR { nd: 4, nem: 0, nemp: 0, nf: 0, coefl: 0.003861, coefr: -11650.0 },
        TermLR { nd: 2, nem: 0, nemp: -3, nf: 0, coefl: 0.003665, coefr: 14403.0 },
        TermLR { nd: 0, nem: 1, nemp: -2, nf: 0, coefl: -0.002689, coefr: -7003.0 },
        TermLR { nd: 2, nem: 0, nemp: -1, nf: 2, coefl: -0.002602, coefr: 0.0 },
        TermLR { nd: 2, nem: -1, nemp: -2, nf: 0, coefl: 0.002390, coefr: 10056.0 },
        TermLR { nd: 1, nem: 0, nemp: 1, nf: 0, coefl: -0.002348, coefr: 6322.0 },
        TermLR { nd: 2, nem: -2, nemp: 0, nf: 0, coefl: 0.002236, coefr: -9884.0 },
        TermLR { nd: 0, nem: 1, nemp: 2, nf: 0, coefl: -0.002120, coefr: 5751.0 },
        TermLR { nd: 0, nem: 2, nemp: 0, nf: 0, coefl: -0.002069, coefr: 0.0 },
        TermLR { nd: 2, nem: -2, nemp: -1, nf: 0, coefl: 0.002048, coefr: -4950.0 },
        TermLR { nd: 2, nem: 0, nemp: 1, nf: -2, coefl: -0.001773, coefr: 4130.0 },
        TermLR { nd: 2, nem: 0, nemp: 0, nf: 2, coefl: -0.001595, coefr: 0.0 },
        TermLR { nd: 4, nem: -1, nemp: -1, nf: 0, coefl: 0.001215, coefr: -3958.0 },
        TermLR { nd: 0, nem: 0, nemp: 2, nf: 2, coefl: -0.001110, coefr: 0.0 },
        TermLR { nd: 3, nem: 0, nemp: -1, nf: 0, coefl: -0.000892, coefr: 3258.0 },
        TermLR { nd: 2, nem: 1, nemp: 1, nf: 0, coefl: -0.000810, coefr: 2616.0 },
        TermLR { nd: 4, nem: -1, nemp: -2, nf: 0, coefl: 0.000759, coefr: -1897.0 },
        TermLR { nd: 0, nem: 2, nemp: -1, nf: 0, coefl: -0.000713, coefr: -2117.0 },
        TermLR { nd: 2, nem: 2, nemp: -1, nf: 0, coefl: -0.000700, coefr: 2354.0 },
        TermLR { nd: 2, nem: 1, nemp: -2, nf: 0, coefl: 0.000691, coefr: 0.0 },
        TermLR { nd: 2, nem: -1, nemp: 0, nf: -2, coefl: 0.000596, coefr: 0.0 },
        TermLR { nd: 4, nem: 0, nemp: 1, nf: 0, coefl: 0.000549, coefr: -1423.0 },
        TermLR { nd: 0, nem: 0, nemp: 4, nf: 0, coefl: 0.000537, coefr: -1117.0 },
        TermLR { nd: 4, nem: -1, nemp: 0, nf: 0, coefl: 0.000520, coefr: -1571.0 },
        TermLR { nd: 1, nem: 0, nemp: -2, nf: 0, coefl: -0.000487, coefr: -1739.0 },
        TermLR { nd: 2, nem: 1, nemp: 0, nf: -2, coefl: -0.000399, coefr: 0.0 },
        TermLR { nd: 0, nem: 0, nemp: 2, nf: -2, coefl: -0.000381, coefr: -4421.0 },
        TermLR { nd: 1, nem: 1, nemp: 1, nf: 0, coefl: 0.000351, coefr: 0.0 },
        TermLR { nd: 3, nem: 0, nemp: -2, nf: 0, coefl: -0.000340, coefr: 0.0 },
        TermLR { nd: 4, nem: 0, nemp: -3, nf: 0, coefl: 0.000330, coefr: 0.0 },
        TermLR { nd: 2, nem: -1, nemp: 2, nf: 0, coefl: 0.000327, coefr: 0.0 },
        TermLR { nd: 0, nem: 2, nemp: 1, nf: 0, coefl: -0.000323, coefr: 1165.0 },
        TermLR { nd: 1, nem: 1, nemp: -1, nf: 0, coefl: 0.000299, coefr: 0.0 },
        TermLR { nd: 2, nem: 0, nemp: 3, nf: 0, coefl: 0.000294, coefr: 0.0 },
        TermLR { nd: 2, nem: 0, nemp: -1, nf: -2, coefl: 0.000000, coefr: 8752.0 },
    ];

    struct TermB {
        nd: i32,
        nem: i32,
        nemp: i32,
        nf: i32,
        coefb: f64,
    }

    const TB: [TermB; 60] = [
        TermB { nd: 0, nem: 0, nemp: 0, nf: 1, coefb: 5.128122 },
        TermB { nd: 0, nem: 0, nemp: 1, nf: 1, coefb: 0.280602 },
        TermB { nd: 0, nem: 0, nemp: 1, nf: -1, coefb: 0.277693 },
        TermB { nd: 2, nem: 0, nemp: 0, nf: -1, coefb: 0.173237 },
        TermB { nd: 2, nem: 0, nemp: -1, nf: 1, coefb: 0.055413 },
        TermB { nd: 2, nem: 0, nemp: -1, nf: -1, coefb: 0.046271 },
        TermB { nd: 2, nem: 0, nemp: 0, nf: 1, coefb: 0.032573 },
        TermB { nd: 0, nem: 0, nemp: 2, nf: 1, coefb: 0.017198 },
        TermB { nd: 2, nem: 0, nemp: 1, nf: -1, coefb: 0.009266 },
        TermB { nd: 0, nem: 0, nemp: 2, nf: -1, coefb: 0.008822 },
        TermB { nd: 2, nem: -1, nemp: 0, nf: -1, coefb: 0.008216 },
        TermB { nd: 2, nem: 0, nemp: -2, nf: -1, coefb: 0.004324 },
        TermB { nd: 2, nem: 0, nemp: 1, nf: 1, coefb: 0.004200 },
        TermB { nd: 2, nem: 1, nemp: 0, nf: -1, coefb: -0.003359 },
        TermB { nd: 2, nem: -1, nemp: -1, nf: 1, coefb: 0.002463 },
        TermB { nd: 2, nem: -1, nemp: 0, nf: 1, coefb: 0.002211 },
        TermB { nd: 2, nem: -1, nemp: -1, nf: -1, coefb: 0.002065 },
        TermB { nd: 0, nem: 1, nemp: -1, nf: -1, coefb: -0.001870 },
        TermB { nd: 4, nem: 0, nemp: -1, nf: -1, coefb: 0.001828 },
        TermB { nd: 0, nem: 1, nemp: 0, nf: 1, coefb: -0.001794 },
        TermB { nd: 0, nem: 0, nemp: 0, nf: 3, coefb: -0.001749 },
        TermB { nd: 0, nem: 1, nemp: -1, nf: 1, coefb: -0.001565 },
        TermB { nd: 1, nem: 0, nemp: 0, nf: 1, coefb: -0.001491 },
        TermB { nd: 0, nem: 1, nemp: 1, nf: 1, coefb: -0.001475 },
        TermB { nd: 0, nem: 1, nemp: 1, nf: -1, coefb: -0.001410 },
        TermB { nd: 0, nem: 1, nemp: 0, nf: -1, coefb: -0.001344 },
        TermB { nd: 1, nem: 0, nemp: 0, nf: -1, coefb: -0.001335 },
        TermB { nd: 0, nem: 0, nemp: 3, nf: 1, coefb: 0.001107 },
        TermB { nd: 4, nem: 0, nemp: 0, nf: -1, coefb: 0.001021 },
        TermB { nd: 4, nem: 0, nemp: -1, nf: 1, coefb: 0.000833 },
        TermB { nd: 0, nem: 0, nemp: 1, nf: -3, coefb: 0.000777 },
        TermB { nd: 4, nem: 0, nemp: -2, nf: 1, coefb: 0.000671 },
        TermB { nd: 2, nem: 0, nemp: 0, nf: -3, coefb: 0.000607 },
        TermB { nd: 2, nem: 0, nemp: 2, nf: -1, coefb: 0.000596 },
        TermB { nd: 2, nem: -1, nemp: 1, nf: -1, coefb: 0.000491 },
        TermB { nd: 2, nem: 0, nemp: -2, nf: 1, coefb: -0.000451 },
        TermB { nd: 0, nem: 0, nemp: 3, nf: -1, coefb: 0.000439 },
        TermB { nd: 2, nem: 0, nemp: 2, nf: 1, coefb: 0.000422 },
        TermB { nd: 2, nem: 0, nemp: -3, nf: -1, coefb: 0.000421 },
        TermB { nd: 2, nem: 1, nemp: -1, nf: 1, coefb: -0.000366 },
        TermB { nd: 2, nem: 1, nemp: 0, nf: 1, coefb: -0.000351 },
        TermB { nd: 4, nem: 0, nemp: 0, nf: 1, coefb: 0.000331 },
        TermB { nd: 2, nem: -1, nemp: 1, nf: 1, coefb: 0.000315 },
        TermB { nd: 2, nem: -2, nemp: 0, nf: -1, coefb: 0.000302 },
        TermB { nd: 0, nem: 0, nemp: 1, nf: 3, coefb: -0.000283 },
        TermB { nd: 2, nem: 1, nemp: 1, nf: -1, coefb: -0.000229 },
        TermB { nd: 1, nem: 1, nemp: 0, nf: -1, coefb: 0.000223 },
        TermB { nd: 1, nem: 1, nemp: 0, nf: 1, coefb: 0.000223 },
        TermB { nd: 0, nem: 1, nemp: -2, nf: -1, coefb: -0.000220 },
        TermB { nd: 2, nem: 1, nemp: -1, nf: -1, coefb: -0.000220 },
        TermB { nd: 1, nem: 0, nemp: 1, nf: 1, coefb: -0.000185 },
        TermB { nd: 2, nem: -1, nemp: -2, nf: -1, coefb: 0.000181 },
        TermB { nd: 0, nem: 1, nemp: 2, nf: 1, coefb: -0.000177 },
        TermB { nd: 4, nem: 0, nemp: -2, nf: -1, coefb: 0.000176 },
        TermB { nd: 4, nem: -1, nemp: -1, nf: -1, coefb: 0.000166 },
        TermB { nd: 1, nem: 0, nemp: 1, nf: -1, coefb: -0.000164 },
        TermB { nd: 4, nem: 0, nemp: 1, nf: -1, coefb: 0.000132 },
        TermB { nd: 1, nem: 0, nemp: -1, nf: -1, coefb: -0.000119 },
        TermB { nd: 4, nem: -1, nemp: 0, nf: -1, coefb: 0.000115 },
        TermB { nd: 2, nem: -2, nemp: 0, nf: 1, coefb: 0.000107 },
    ];

    let mut rm = [[0.0; 3]; 3];

    /* Time since J2000.0, Julian centuries. */
    let t = ((date1 - DJ00) + date2) / DJC;

    /* Fundamental arguments (Simon et al. 1994). */

    /* Moon's mean longitude. */
    let elp = DD2R * (ELP0 + (ELP1 + (ELP2 + (ELP3 + ELP4 * t) * t) * t) * t).rem_euclid(360.0);
    let delp = DD2R * (ELP1 + (ELP2 * 2.0 + (ELP3 * 3.0 + ELP4 * 4.0 * t) * t) * t);

    /* Moon's mean elongation. */
    let d = DD2R * (D0 + (D1 + (D2 + (D3 + D4 * t) * t) * t) * t).rem_euclid(360.0);
    let dd = DD2R * (D1 + (D2 * 2.0 + (D3 * 3.0 + D4 * 4.0 * t) * t) * t);

    /* Sun's mean anomaly. */
    let em = DD2R * (EM0 + (EM1 + (EM2 + (EM3 + EM4 * t) * t) * t) * t).rem_euclid(360.0);
    let dem = DD2R * (EM1 + (EM2 * 2.0 + (EM3 * 3.0 + EM4 * 4.0 * t) * t) * t);

    /* Moon's mean anomaly. */
    let emp = DD2R * (EMP0 + (EMP1 + (EMP2 + (EMP3 + EMP4 * t) * t) * t) * t).rem_euclid(360.0);
    let demp = DD2R * (EMP1 + (EMP2 * 2.0 + (EMP3 * 3.0 + EMP4 * 4.0 * t) * t) * t);

    /* Mean distance of the Moon from its ascending node. */
    let f = DD2R * (F0 + (F1 + (F2 + (F3 + F4 * t) * t) * t) * t).rem_euclid(360.0);
    let df = DD2R * (F1 + (F2 * 2.0 + (F3 * 3.0 + F4 * 4.0 * t) * t) * t);

    /* Meeus further arguments. */
    let a1 = DD2R * (A10 + A11 * t);
    let da1 = DD2R * A11;
    let a2 = DD2R * (A20 + A21 * t);
    let da2 = DD2R * A21;
    let a3 = DD2R * (A30 + A31 * t);
    let da3 = DD2R * A31;

    /* E-factor, and square. */
    let e = 1.0 + (E1 + E2 * t) * t;
    let de = E1 + 2.0 * E2 * t;
    let esq = e * e;
    let desq = 2.0 * e * de;

    /* Use the Meeus additive terms (deg) to start off the summations. */
    let elpmf = elp - f;
    let delpmf = delp - df;
    let mut vel = AL1 * a1.sin() + AL2 * elpmf.sin() + AL3 * a2.sin();
    let mut vdel = AL1 * a1.cos() * da1 + AL2 * elpmf.cos() * delpmf + AL3 * a2.cos() * da2;

    let mut vr = 0.0;
    let mut vdr = 0.0;

    let a1mf = a1 - f;
    let da1mf = da1 - df;
    let a1pf = a1 + f;
    let da1pf = da1 + df;
    let dlpmp = elp - emp;
    let slpmp = elp + emp;
    let mut vb = AB1 * elp.sin()
        + AB2 * a3.sin()
        + AB3 * a1mf.sin()
        + AB4 * a1pf.sin()
        + AB5 * dlpmp.sin()
        + AB6 * slpmp.sin();
    let mut vdb = AB1 * elp.cos() * delp
        + AB2 * a3.cos() * da3
        + AB3 * a1mf.cos() * da1mf
        + AB4 * a1pf.cos() * da1pf
        + AB5 * dlpmp.cos() * (delp - demp)
        + AB6 * slpmp.cos() * (delp + demp);

    /* ----------------- */
    /* Series expansions */
    /* ----------------- */

    /* Longitude and distance plus derivatives. */
    for n in (0..TLR.len()).rev() {
        let dn = TLR[n].nd as f64;
        let i_val = TLR[n].nem;
        let emn = i_val as f64;
        let empn = TLR[n].nemp as f64;
        let fn_val = TLR[n].nf as f64;
        let en;
        let den;
        match i_val.abs() {
            1 => {
                en = e;
                den = de;
            }
            2 => {
                en = esq;
                den = desq;
            }
            _ => {
                en = 1.0;
                den = 0.0;
            }
        }
        let arg = dn * d + emn * em + empn * emp + fn_val * f;
        let darg = dn * dd + emn * dem + empn * demp + fn_val * df;
        let mut farg = arg.sin();
        let mut v = farg * en;
        let mut dv = arg.cos() * darg * en + farg * den;
        let mut coeff = TLR[n].coefl;
        vel += coeff * v;
        vdel += coeff * dv;
        farg = arg.cos();
        v = farg * en;
        dv = -arg.sin() * darg * en + farg * den;
        coeff = TLR[n].coefr;
        vr += coeff * v;
        vdr += coeff * dv;
    }
    let el = elp + DD2R * vel;
    let del = (delp + DD2R * vdel) / DJC;
    let r = (vr + R0) / DAU;
    let dr = vdr / DAU / DJC;

    /* Latitude plus derivative. */
    for n in (0..TB.len()).rev() {
        let dn = TB[n].nd as f64;
        let i_val = TB[n].nem;
        let emn = i_val as f64;
        let empn = TB[n].nemp as f64;
        let fn_val = TB[n].nf as f64;
        let en;
        let den;
        match i_val.abs() {
            1 => {
                en = e;
                den = de;
            }
            2 => {
                en = esq;
                den = desq;
            }
            _ => {
                en = 1.0;
                den = 0.0;
            }
        }
        let arg = dn * d + emn * em + empn * emp + fn_val * f;
        let darg = dn * dd + emn * dem + empn * demp + fn_val * df;
        let farg = arg.sin();
        let v = farg * en;
        let dv = arg.cos() * darg * en + farg * den;
        let coeff = TB[n].coefb;
        vb += coeff * v;
        vdb += coeff * dv;
    }
    let b = vb * DD2R;
    let db = vdb * DD2R / DJC;

    /* ------------------------------ */
    /* Transformation into final form */
    /* ------------------------------ */

    /* Longitude, latitude to x, y, z (au). */
    let pv = s2pv(el, b, r, del, db, dr);

    /* IAU 2006 Fukushima-Williams bias+precession angles. */
    let (gamb, phib, psib, _) = pfw06(date1, date2);

    /* Mean ecliptic coordinates to GCRS rotation matrix. */
    ir(&mut rm);
    rz(psib, &mut rm);
    rx(-phib, &mut rm);
    rz(-gamb, &mut rm);

    /* Rotate the Moon position and velocity into GCRS (Note 6). */
    let mut res = [[0.0; 3]; 2];
    rxpv(&rm, &pv, &mut res);

    res
}
