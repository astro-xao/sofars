use super::d2tf;
use crate::consts::D2PI;

pub fn a2tf(ndp: i32, angle: f64) -> (char, [i32; 4]) {
    /* Scale then use days to h,m,s function. */
    d2tf(ndp, angle / D2PI)
}
