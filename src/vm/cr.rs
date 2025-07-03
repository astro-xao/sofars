use super::cp;

pub fn cr(r: &[[f64; 3]; 3], c: &mut [[f64; 3]; 3]) {
    cp(&r[0], &mut c[0]);
    cp(&r[1], &mut c[1]);
    cp(&r[2], &mut c[2]);
}
