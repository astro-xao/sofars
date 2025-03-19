pub fn eors(rnpb: &[[f64; 3]; 3], s: f64) -> f64 {
    let x = rnpb[2][0];
    let ax = x / (1.0 + rnpb[2][2]);
    let xs = 1.0 - ax * x;
    let ys = -ax * rnpb[2][1];
    let zs = -x;
    let p = rnpb[0][0] * xs + rnpb[0][1] * ys + rnpb[0][2] * zs;
    let q = rnpb[1][0] * xs + rnpb[1][1] * ys + rnpb[1][2] * zs;
    let eo = if p != 0.0 || q != 0.0 {
        s - q.atan2(p)
    } else {
        s
    };

    eo
}
