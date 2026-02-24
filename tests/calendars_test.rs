mod common;

#[cfg(test)]
mod tests {
    use super::common::vvd;
    use sofars::cal::*;

    #[test]
    fn test_cal2jd() {
        let jd = cal2jd(2003, 6, 1).unwrap();
        vvd(jd.0, 2400000.5, 0.0, "cal2jd", "djm0");
        vvd(jd.1, 52791.0, 0.0, "cal2jd", "djm");
    }

    #[test]
    fn test_epb() {
        let epb = epb(2415019.8135, 30103.18648);
        vvd(epb, 1982.418424159278580, 1e-12, "epb", "");
    }

    #[test]
    fn test_epb2jd() {
        let (djm0, djm) = epb2jd(1957.3);
        vvd(djm0, 2400000.5, 1e-9, "epb2jd", "djm0");
        vvd(djm, 35948.1915101513, 1e-9, "epb2jd", "djm");
    }

    #[test]
    fn test_epj() {
        let result = epj(2451545.0, -7392.5);
        vvd(result, 1979.760438056125941, 1e-12, "epj", "");
    }

    #[test]
    fn test_epj2jd() {
        let (djm0, djm) = epj2jd(1996.8);
        vvd(djm0, 2400000.5, 1e-9, "epj2jd", "djm0");
        vvd(djm, 50375.7, 1e-9, "epj2jd", "mjd");
    }

    #[test]
    fn test_jd2cal() {
        use super::common::viv;
        let (iy, im, id, fd) = jd2cal(2400000.5, 50123.9999).unwrap();
        viv(iy, 1996, "jd2cal", "y");
        viv(im, 2, "jd2cal", "m");
        viv(id, 10, "jd2cal", "d");
        vvd(fd, 0.9999, 1e-7, "jd2cal", "fd");
    }

    #[test]
    fn test_jdcalf() {
        use super::common::viv;
        let dj1 = 2400000.5;
        let dj2 = 50123.9999;
        let (iy, im, id, f, j) = jdcalf(4, dj1, dj2).unwrap();

        viv(iy, 1996, "jdcalf", "y");
        viv(im, 2, "jdcalf", "m");
        viv(id, 10, "jdcalf", "d");
        viv(f, 9999, "jdcalf", "f");
        viv(j, 0, "jdcalf", "j");
    }
}
