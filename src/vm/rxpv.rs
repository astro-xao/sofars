use super::rxp;

pub fn rxpv(r: &[[f64; 3]; 3], pv: &[[f64; 3]; 2], rpv: &mut [[f64; 3]; 2]) {
    rxp(r, &pv[0], &mut rpv[0]);
    rxp(r, &pv[1], &mut rpv[1]);
}
