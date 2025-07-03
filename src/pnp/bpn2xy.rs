///  Extract CIP X,Y coordinates from NPB matrix
pub fn bpn2xy(rbpn: &[[f64; 3]; 3]) -> (f64, f64) {
    let x = rbpn[2][0];
    let y = rbpn[2][1];
    (x, y)
}
