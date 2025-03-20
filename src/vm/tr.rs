use super::cr;

pub fn tr(r: &[[f64; 3]; 3], rt: &mut [[f64; 3]; 3]) {
    let wm  = &mut [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            wm[i][j] = r[j][i];
        }
    }
    cr(wm, rt);
}