use crate::vm::{ir, ry, rz};

///  Celestial−to−intermediate matrix, given X,Y, IAU 2000
pub fn c2ixys(x: f64, y: f64, s: f64, rc2i: &mut [[f64; 3]; 3]) {
    let r2 = x * x + y * y;
    let e = if r2 > 0.0 { y.atan2(x) } else { 0.0 };
    let d = (r2 / (1.0 - r2)).sqrt().atan();

    ir(rc2i);
    rz(e, rc2i);
    ry(d, rc2i);
    rz(-(e + s), rc2i);
}
