mod common;

#[cfg(test)]
mod tests {
    use sofars::projection::*;
    use sofars::vm::s2c;
    use super::common::{vvd, viv};

    #[test]
    fn test_tpors() {
        let xi = -0.03;
        let eta = 0.07;
        let ra = 1.3;
        let dec = 1.5;

        let (az1, bz1, az2, bz2, n) = tpors(xi, eta, ra, dec);

        vvd(az1, 1.736621577783208748, 1e-13, "iauTpors", "az1");
        vvd(bz1, 1.436736561844090323, 1e-13, "iauTpors", "bz1");
        vvd(az2, 4.004971075806584490, 1e-13, "iauTpors", "az2");
        vvd(bz2, 1.565084088476417917, 1e-13, "iauTpors", "bz2");
        viv(n, 2, "tpors", "n");
    }

    #[test]
    fn test_tporv() {
        let xi = -0.03;
        let eta = 0.07;
        let ra = 1.3;
        let dec = 1.5;
        let v = s2c(ra, dec);

        let (vz1, vz2, n) = tporv(xi, eta, v);

        vvd(vz1[0], -0.02206252822366888610, 1e-15, "iauTporv", "x1");
        vvd(vz1[1], 0.1318251060359645016, 1e-14, "iauTporv", "y1");
        vvd(vz1[2], 0.9910274397144543895, 1e-14, "iauTporv", "z1");

        vvd(vz2[0], -0.003712211763801968173, 1e-16, "iauTporv", "x2");
        vvd(vz2[1], -0.004341519956299836813, 1e-16, "iauTporv", "y2");
        vvd(vz2[2], 0.9999836852110587012, 1e-14, "iauTporv", "z2");
        viv(n, 2, "tporv", "n");
    }

    #[test]
    fn test_tpsts() {
        let xi = -0.03;
        let eta = 0.07;
        let raz = 2.3;
        let decz = 1.5;

        let (ra, dec) = tpsts(xi, eta, raz, decz);

        vvd(ra, 0.7596127167359629775, 1e-14, "iauTpsts", "ra");
        vvd(dec, 1.540864645109263028, 1e-13, "iauTpsts", "dec");
    }

    #[test]
    fn test_tpstv() {
        let xi = -0.03;
        let eta = 0.07;
        let raz = 2.3;
        let decz = 1.5;
        let vz = s2c(raz, decz);

        let v = tpstv(xi, eta, vz);

        vvd(v[0], 0.02170030454907376677, 1e-15, "iauTpstv", "x");
        vvd(v[1], 0.02060909590535367447, 1e-15, "iauTpstv", "y");
        vvd(v[2], 0.9995520806583523804, 1e-14, "iauTpstv", "z");
    }

    #[test]
    fn test_tpxes() {
        let ra = 1.3;
        let dec = 1.55;
        let raz = 2.3;
        let decz = 1.5;

        let (xi, eta, j) = tpxes(ra, dec, raz, decz);

        vvd(xi, -0.01753200983236980595, 1e-15, "iauTpxes", "xi");
        vvd(eta, 0.05962940005778712891, 1e-15, "iauTpxes", "eta");
        viv(j, 0, "tpxes", "j");
    }

    #[test]
    fn test_tpxev() {
        let ra = 1.3;
        let dec = 1.55;
        let raz = 2.3;
        let decz = 1.5;
        let v = s2c(ra, dec);
        let vz = s2c(raz, decz);

        let (xi, eta, j) = tpxev(v, vz);

        vvd(xi, -0.01753200983236980595, 1e-15, "iauTpxev", "xi");
        vvd(eta, 0.05962940005778712891, 1e-15, "iauTpxev", "eta");
        viv(j, 0, "tpxev", "j");
    }
}
