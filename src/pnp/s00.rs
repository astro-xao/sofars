#![allow(non_upper_case_globals)]
use crate::consts::{DAS2R, DJ00, DJC};
use crate::fundargs::{fad03, fae03, faf03, fal03, falp03, faom03, fapa03, fave03};

struct TERM(
    [i32; 8], // nfa
    f64,
    f64, // s, c
);

const sp: [f64; 6] = [
    94.00e-6,
    3808.35e-6,
    -119.94e-6,
    -72574.09e-6,
    27.70e-6,
    15.61e-6,
];

/* Terms of order t^0 */
const s0: [TERM; 33] = [
    TERM([0, 0, 0, 0, 1, 0, 0, 0], -2640.73e-6, 0.39e-6),
    TERM([0, 0, 0, 0, 2, 0, 0, 0], -63.53e-6, 0.02e-6),
    TERM([0, 0, 2, -2, 3, 0, 0, 0], -11.75e-6, -0.01e-6),
    TERM([0, 0, 2, -2, 1, 0, 0, 0], -11.21e-6, -0.01e-6),
    TERM([0, 0, 2, -2, 2, 0, 0, 0], 4.57e-6, 0.00e-6),
    TERM([0, 0, 2, 0, 3, 0, 0, 0], -2.02e-6, 0.00e-6),
    TERM([0, 0, 2, 0, 1, 0, 0, 0], -1.98e-6, 0.00e-6),
    TERM([0, 0, 0, 0, 3, 0, 0, 0], 1.72e-6, 0.00e-6),
    TERM([0, 1, 0, 0, 1, 0, 0, 0], 1.41e-6, 0.01e-6),
    TERM([0, 1, 0, 0, -1, 0, 0, 0], 1.26e-6, 0.01e-6),
    TERM([1, 0, 0, 0, -1, 0, 0, 0], 0.63e-6, 0.00e-6),
    TERM([1, 0, 0, 0, 1, 0, 0, 0], 0.63e-6, 0.00e-6),
    TERM([0, 1, 2, -2, 3, 0, 0, 0], -0.46e-6, 0.00e-6),
    TERM([0, 1, 2, -2, 1, 0, 0, 0], -0.45e-6, 0.00e-6),
    TERM([0, 0, 4, -4, 4, 0, 0, 0], -0.36e-6, 0.00e-6),
    TERM([0, 0, 1, -1, 1, -8, 12, 0], 0.24e-6, 0.12e-6),
    TERM([0, 0, 2, 0, 0, 0, 0, 0], -0.32e-6, 0.00e-6),
    TERM([0, 0, 2, 0, 2, 0, 0, 0], -0.28e-6, 0.00e-6),
    TERM([1, 0, 2, 0, 3, 0, 0, 0], -0.27e-6, 0.00e-6),
    TERM([1, 0, 2, 0, 1, 0, 0, 0], -0.26e-6, 0.00e-6),
    TERM([0, 0, 2, -2, 0, 0, 0, 0], 0.21e-6, 0.00e-6),
    TERM([0, 1, -2, 2, -3, 0, 0, 0], -0.19e-6, 0.00e-6),
    TERM([0, 1, -2, 2, -1, 0, 0, 0], -0.18e-6, 0.00e-6),
    TERM([0, 0, 0, 0, 0, 8, -13, -1], 0.10e-6, -0.05e-6),
    TERM([0, 0, 0, 2, 0, 0, 0, 0], -0.15e-6, 0.00e-6),
    TERM([2, 0, -2, 0, -1, 0, 0, 0], 0.14e-6, 0.00e-6),
    TERM([0, 1, 2, -2, 2, 0, 0, 0], 0.14e-6, 0.00e-6),
    TERM([1, 0, 0, -2, 1, 0, 0, 0], -0.14e-6, 0.00e-6),
    TERM([1, 0, 0, -2, -1, 0, 0, 0], -0.14e-6, 0.00e-6),
    TERM([0, 0, 4, -2, 4, 0, 0, 0], -0.13e-6, 0.00e-6),
    TERM([0, 0, 2, -2, 4, 0, 0, 0], 0.11e-6, 0.00e-6),
    TERM([1, 0, -2, 0, -3, 0, 0, 0], -0.11e-6, 0.00e-6),
    TERM([1, 0, -2, 0, -1, 0, 0, 0], -0.11e-6, 0.00e-6),
];

/* Terms of order t^1 */
const s1: [TERM; 3] = [
    TERM([0, 0, 0, 0, 2, 0, 0, 0], -0.07e-6, 3.57e-6),
    TERM([0, 0, 0, 0, 1, 0, 0, 0], 1.71e-6, -0.03e-6),
    TERM([0, 0, 2, -2, 3, 0, 0, 0], 0.00e-6, 0.48e-6),
];

/* Terms of order t^2 */
const s2: [TERM; 25] = [
    TERM([0, 0, 0, 0, 1, 0, 0, 0], 743.53e-6, -0.17e-6),
    TERM([0, 0, 2, -2, 2, 0, 0, 0], 56.91e-6, 0.06e-6),
    TERM([0, 0, 2, 0, 2, 0, 0, 0], 9.84e-6, -0.01e-6),
    TERM([0, 0, 0, 0, 2, 0, 0, 0], -8.85e-6, 0.01e-6),
    TERM([0, 1, 0, 0, 0, 0, 0, 0], -6.38e-6, -0.05e-6),
    TERM([1, 0, 0, 0, 0, 0, 0, 0], -3.07e-6, 0.00e-6),
    TERM([0, 1, 2, -2, 2, 0, 0, 0], 2.23e-6, 0.00e-6),
    TERM([0, 0, 2, 0, 1, 0, 0, 0], 1.67e-6, 0.00e-6),
    TERM([1, 0, 2, 0, 2, 0, 0, 0], 1.30e-6, 0.00e-6),
    TERM([0, 1, -2, 2, -2, 0, 0, 0], 0.93e-6, 0.00e-6),
    TERM([1, 0, 0, -2, 0, 0, 0, 0], 0.68e-6, 0.00e-6),
    TERM([0, 0, 2, -2, 1, 0, 0, 0], -0.55e-6, 0.00e-6),
    TERM([1, 0, -2, 0, -2, 0, 0, 0], 0.53e-6, 0.00e-6),
    TERM([0, 0, 0, 2, 0, 0, 0, 0], -0.27e-6, 0.00e-6),
    TERM([1, 0, 0, 0, 1, 0, 0, 0], -0.27e-6, 0.00e-6),
    TERM([1, 0, -2, -2, -2, 0, 0, 0], -0.26e-6, 0.00e-6),
    TERM([1, 0, 0, 0, -1, 0, 0, 0], -0.25e-6, 0.00e-6),
    TERM([1, 0, 2, 0, 1, 0, 0, 0], 0.22e-6, 0.00e-6),
    TERM([2, 0, 0, -2, 0, 0, 0, 0], -0.21e-6, 0.00e-6),
    TERM([2, 0, -2, 0, -1, 0, 0, 0], 0.20e-6, 0.00e-6),
    TERM([0, 0, 2, 2, 2, 0, 0, 0], 0.17e-6, 0.00e-6),
    TERM([2, 0, 2, 0, 2, 0, 0, 0], 0.13e-6, 0.00e-6),
    TERM([2, 0, 0, 0, 0, 0, 0, 0], -0.13e-6, 0.00e-6),
    TERM([1, 0, 2, -2, 2, 0, 0, 0], -0.12e-6, 0.00e-6),
    TERM([0, 0, 2, 0, 0, 0, 0, 0], -0.11e-6, 0.00e-6),
];

/* Terms of order t^3 */
const s3: [TERM; 4] = [
    TERM([0, 0, 0, 0, 1, 0, 0, 0], 0.30e-6, -23.51e-6),
    TERM([0, 0, 2, -2, 2, 0, 0, 0], -0.03e-6, -1.39e-6),
    TERM([0, 0, 2, 0, 2, 0, 0, 0], -0.01e-6, -0.24e-6),
    TERM([0, 0, 0, 0, 2, 0, 0, 0], 0.00e-6, 0.22e-6),
];

/* Terms of order t^4 */
const s4: [TERM; 1] = [
    /* 1-1 */
    TERM([0, 0, 0, 0, 1, 0, 0, 0], -0.26e-6, -0.01e-6),
];

pub fn s00(date1: f64, date2: f64, x: f64, y: f64) -> f64 {
    /* Interval between fundamental epoch J2000.0 and current date (JC). */
    let t = ((date1 - DJ00) + date2) / DJC;

    /* Fundamental Arguments (from IERS Conventions 2003) */

    let mut fa: [f64; 8] = [0.0; 8];
    /* Mean anomaly of the Moon. */
    fa[0] = fal03(t);

    /* Mean anomaly of the Sun. */
    fa[1] = falp03(t);

    /* Mean longitude of the Moon minus that of the ascending node. */
    fa[2] = faf03(t);

    /* Mean elongation of the Moon from the Sun. */
    fa[3] = fad03(t);

    /* Mean longitude of the ascending node of the Moon. */
    fa[4] = faom03(t);

    /* Mean longitude of Venus. */
    fa[5] = fave03(t);

    /* Mean longitude of Earth. */
    fa[6] = fae03(t);

    /* General precession in longitude. */
    fa[7] = fapa03(t);

    let [mut w0, mut w1, mut w2, mut w3, mut w4, w5] = sp;
    for i in (0..s0.len()).rev() {
        let nfa = s0[i].0;
        let [s, c] = [s0[i].1, s0[i].2];
        let a = fa
            .iter()
            .zip(nfa.iter())
            .fold(0.0, |acc, (&fa, &nfa)| acc + fa * nfa as f64);
        w0 += s * a.sin() + c * a.cos();
    }

    for i in (0..s1.len()).rev() {
        let nfa = s1[i].0;
        let [s, c] = [s1[i].1, s1[i].2];
        let a = fa
            .iter()
            .zip(nfa.iter())
            .fold(0.0, |acc, (&fa, &nfa)| acc + fa * nfa as f64);
        w1 += s * a.sin() + c * a.cos();
    }

    for i in (0..s2.len()).rev() {
        let nfa = s2[i].0;
        let [s, c] = [s2[i].1, s2[i].2];
        let a = fa
            .iter()
            .zip(nfa.iter())
            .fold(0.0, |acc, (&fa, &nfa)| acc + fa * nfa as f64);
        w2 += s * a.sin() + c * a.cos();
    }

    for i in (0..s3.len()).rev() {
        let nfa = s3[i].0;
        let [s, c] = [s3[i].1, s3[i].2];
        let a = fa
            .iter()
            .zip(nfa.iter())
            .fold(0.0, |acc, (&fa, &nfa)| acc + fa * nfa as f64);
        w3 += s * a.sin() + c * a.cos();
    }

    for i in (0..s4.len()).rev() {
        let nfa = s4[i].0;
        let [s, c] = [s4[i].1, s4[i].2];
        let a = fa
            .iter()
            .zip(nfa.iter())
            .fold(0.0, |acc, (&fa, &nfa)| acc + fa * nfa as f64);
        w4 += s * a.sin() + c * a.cos();
    }

    let s = (w0 + (w1 + (w2 + (w3 + (w4 + w5 * t) * t) * t) * t) * t) * DAS2R - x * y / 2.0;

    s
}
