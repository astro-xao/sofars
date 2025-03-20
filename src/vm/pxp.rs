pub fn pxp(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    let mut axb: [f64; 3] = [0.0; 3];

    axb[0] = a[1] * b[2] - a[2] * b[1];
    axb[1] = a[2] * b[0] - a[0] * b[2];
    axb[2] = a[0] * b[1] - a[1] * b[0];

    axb
}