/// Copy a p-vector.
pub fn cp(p: &[f64; 3], c: &mut [f64; 3]) {
    c[0] = p[0];
    c[1] = p[1];
    c[2] = p[2];
}
