use crate::vm::{ir, rx, ry, rz};

pub fn pom00(xp: f64, yp: f64, sp: f64) -> [[f64; 3]; 3] {
    let mut rpom = [[0.0; 3]; 3];
    ir(&mut rpom);
    rz(sp, &mut rpom);
    ry(-xp, &mut rpom);
    rx(-yp, &mut rpom);
    rpom
}
