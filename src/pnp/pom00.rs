use crate::vm::{ir, rx, ry, rz};

pub fn pom00(xp: f64, yp: f64, sp: f64, rpom: &mut [[f64; 3]; 3]) {
    ir(rpom);
    rz(sp, rpom);
    ry(-xp, rpom);
    rx(-yp, rpom);
}
