use crate::consts::{DAS2R, DJC, DJ00};
use crate::vm::anp;
use super::era00;

/// Greenwich mean sidereal time, IAU 2006
pub fn gmst06(uta: f64, utb: f64, tta: f64, ttb: f64) -> f64 {
    let t = ((tta - DJ00) + ttb) / DJC;

    let gmst = anp(
        era00(uta, utb) +
                (    0.014506     +
                (  4612.156534    +
                (     1.3915817   +
                (    -0.00000044  +
                (    -0.000029956 +
                (    -0.0000000368 )
        * t) * t) * t) * t) * t) * DAS2R
    );

    gmst
}