use super::cr;

pub fn rxr(a: &[[f64; 3]; 3], b: &[[f64; 3]; 3], atb: &mut [[f64; 3]; 3]) {
    let wm = &mut [[0.0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            wm[i][j] = 0.0;
            for k in 0..3 {
                wm[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    cr(wm, atb)
}
