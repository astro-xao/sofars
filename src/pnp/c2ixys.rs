use crate::vm::{ir, ry, rz};

///  Celestial−to−intermediate matrix, given X,Y, IAU 2000
pub fn c2ixys(x: f64, y: f64, s: f64) -> [[f64; 3]; 3] {
    let mut rc2i = [[0.0; 3]; 3];
    let r2 = x * x + y * y;
    let e = if r2 > 0.0 { y.atan2(x) } else { 0.0 };
    let d = (r2 / (1.0 - r2)).sqrt().atan();

    ir(&mut rc2i);
    rz(e, &mut rc2i);
    ry(d, &mut rc2i);
    rz(-(e + s), &mut rc2i);
    rc2i
}
