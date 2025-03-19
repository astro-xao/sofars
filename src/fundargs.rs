use std::ops::Rem;
use crate::consts::{DAS2R, TURNAS, D2PI};

/// mean elongation of the Moon from the Sun
pub fn fad03(t: f64) -> f64 {
    /* Mean elongation of the Moon from the Sun (IERS Conventions 2003). */
    (            1072260.703692 +
        t * ( 1602961601.2090 +
        t * (         -6.3706 +
        t * (          0.006593 +
        t *           -0.00003169 )))).rem(TURNAS) * DAS2R
}

/// mean longitude of Earth
pub fn fae03(t: f64) -> f64 {
    (1.753470314 + 628.3075849991 * t).rem(D2PI)
}

/// mean argument of the latitude of the Moon
pub fn faf03(t: f64) -> f64 {
    (335779.526232 +
                    t * ( 1739527262.8478 +
                    t * (       - 12.7512 +
                    t * (        - 0.001037 +
                    t * (          0.00000417 ) ) ) )).rem(TURNAS ) * DAS2R
}

/// mean longitude of Jupiter
pub fn faju03(t: f64) -> f64 {
    (0.599546497 + 52.9690962641 * t).rem(D2PI)
}

/// mean anomaly of the Moon
pub fn fal03(t: f64) -> f64 {
    /* Mean anomaly of the Moon (IERS Conventions 2003). */
    (             485868.249036 +
        t * ( 1717915923.2178 +
        t * (         31.8792 +
        t * (          0.051635 +
        t * (         -0.00024470 ))))).rem(TURNAS) * DAS2R
}

/// mean anomaly of the Sun
pub fn falp03(t: f64) -> f64 {
    /* Mean anomaly of the Sun (IERS Conventions 2003). */
    (          1287104.79305 +
        t * (129596581.0481 +
        t * (       -0.5532 +
        t * (        0.000136 +
        t * (       -0.00001149 ))))).rem(TURNAS) * DAS2R
}

/// mean longitude of Mars
pub fn fama03(t: f64) -> f64 {
    /* Mean longitude of Mars (IERS Conventions 2003). */
    (6.203480913 + 334.0612426700 * t).rem(D2PI)
}

/// mean longitude of Mercury
pub fn fame03(t: f64) -> f64 {
    /* Mean longitude of Mercury (IERS Conventions 2003). */
    (4.402608842 + 2608.7903141574 * t).rem(D2PI)
}

/// mean longitude of Neptune
pub fn fane03() {}

/// mean longitude of the Moon’s ascending node
pub fn faom03(t: f64) -> f64 {
    (           450160.398036 +
                    t * ( - 6962890.5431 +
                    t * (         7.4722 +
                    t * (         0.007702 +
                    t * (       - 0.00005939 ) )))).rem(TURNAS ) * DAS2R
}

/// general accumulated precession in longitude
pub fn fapa03(t: f64) -> f64 {
    /* General accumulated precession in longitude. */
    (0.024381750 + 0.00000538691 * t) * t
}

/// mean longitude of Saturn
pub fn fasa03(t: f64) -> f64 {
    /* Mean longitude of Saturn (IERS Conventions 2003). */
    (0.874016757 + 21.3299104960 * t).rem(D2PI)
}

/// mean longitude of Uranus
pub fn faur03(t: f64) -> f64 {
    /* Mean longitude of Uranus (IERS Conventions 2003). */
    (5.481293872 + 7.4781598567 * t).rem(D2PI)
}

/// mean longitude of Venus
pub fn fave03(t: f64) -> f64 {
    (3.176146697 + 1021.3285546211 * t).rem(D2PI)
}
