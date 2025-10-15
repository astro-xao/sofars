use super::cp;

/// Multiply a p-vector by an r-matrix.
pub fn rxp(r: &[[f64; 3]; 3], p: &[f64; 3], rp: &mut [f64; 3]) {
    let wrp = &mut [0.0; 3];

    // Matrix r * vector p.
    for j in 0..3 {
        let mut w = 0.0;
        for i in 0..3 {
            w += r[j][i] * p[i];
        }
        wrp[j] = w;
    }

    /* Return the result. */
    cp(wrp, rp);
}
