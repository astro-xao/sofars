/// Rotate an r-matrix about the y-axis.
pub fn ry(theta: f64, r: &mut [[f64; 3]; 3]) {
    let s = theta.sin();
    let c = theta.cos();

    let a00 = c * r[0][0] - s * r[2][0];
    let a01 = c * r[0][1] - s * r[2][1];
    let a02 = c * r[0][2] - s * r[2][2];
    let a20 = s * r[0][0] + c * r[2][0];
    let a21 = s * r[0][1] + c * r[2][1];
    let a22 = s * r[0][2] + c * r[2][2];

    r[0][0] = a00;
    r[0][1] = a01;
    r[0][2] = a02;
    r[2][0] = a20;
    r[2][1] = a21;
    r[2][2] = a22;
}
