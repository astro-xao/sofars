mod common;

#[cfg(test)]
mod tests {
    use super::common::{viv, vvd};
    use sofars::ts;

    #[test]
    fn test_d2dtf() {
        let (iy, im, id, ihmsf) = ts::d2dtf("UTC", 5, 2400000.5, 49533.99999).unwrap();

        viv(iy, 1994, "d2dtf", "y");
        viv(im, 6, "d2dtf", "mo");
        viv(id, 30, "d2dtf", "d");
        viv(ihmsf[0], 23, "d2dtf", "h");
        viv(ihmsf[1], 59, "d2dtf", "m");
        viv(ihmsf[2], 60, "d2dtf", "s");
        viv(ihmsf[3], 13599, "d2dtf", "f");
    }

    #[test]
    fn test_dat() {
        let mut deltat: f64;

        deltat = ts::dat(2003, 6, 1, 0.0).unwrap();
        vvd(deltat, 32.0, 1e-12, "dat", "d1");

        deltat = ts::dat(2008, 1, 17, 0.0).unwrap();
        vvd(deltat, 33.0, 1e-12, "dat", "d2");

        deltat = ts::dat(2017, 9, 1, 0.0).unwrap();
        vvd(deltat, 37.0, 1e-12, "dat", "d3");
    }

    #[test]
    fn test_ut1tt() {
        let (t1, t2) = ts::ut1tt(2453750.5, 0.892104561, 64.8499).unwrap();

        vvd(t1, 2453750.5, 1e-6, "ut1tt", "t1");
        vvd(t2, 0.8928551385462962963, 1e-12, "ut1tt", "t2");
    }

    #[test]
    fn test_dtdb() {
        let result = ts::dtdb(2448939.5, 0.123, 0.76543, 5.0123, 5525.242, 3190.0);
        vvd(result, -0.1280368005936998991e-2, 1e-15, "dtdb", "");
    }

    #[test]
    fn test_dtf2d() {
        let (u1, u2) = ts::dtf2d("UTC", 1994, 6, 30, 23, 59, 60.13599).unwrap();
        vvd(u1 + u2, 2449534.49999, 1e-6, "dtf2d", "u");
    }

    #[test]
    fn test_taitt() {
        let (t1, t2) = ts::taitt(2453750.5, 0.892482639).unwrap();
        vvd(t1, 2453750.5, 1e-6, "taitt", "t1");
        vvd(t2, 0.892855139, 1e-12, "taitt", "t2");
    }

    #[test]
    fn test_taiut1() {
        let (u1, u2) = ts::taiut1(2453750.5, 0.892482639, -32.6659).unwrap();
        vvd(u1, 2453750.5, 1e-6, "taiut1", "u1");
        vvd(u2, 0.8921045614537037037, 1e-12, "taiut1", "u2");
    }

    #[test]
    fn test_utctai() {
        let (u1, u2) = ts::utctai(2453750.5, 0.892100694).unwrap();
        vvd(u1, 2453750.5, 1e-6, "utctai", "u1");
        vvd(u2, 0.8924826384444444444, 1e-12, "utctai", "u2");
    }

    #[test]
    fn test_utcut1() {
        let (u1, u2) = ts::utcut1(2453750.5, 0.892100694, 0.3341).unwrap();
        vvd(u1, 2453750.5, 1e-6, "utcut1", "u1");
        vvd(u2, 0.8921045608981481481, 1e-12, "utcut1", "u2");
    }

    #[test]
    fn test_taiutc() {
        let (u1, u2) = ts::taiutc(2453750.5, 0.892482639).unwrap();
        vvd(u1, 2453750.5, 1e-6, "taiutc", "u1");
        vvd(u2, 0.8921006945555555556, 1e-12, "taiutc", "u2");
    }

    #[test]
    fn test_tcbtdb() {
        let (b1, b2) = ts::tcbtdb(2453750.5, 0.893019599).unwrap();
        vvd(b1, 2453750.5, 1e-6, "tcbtdb", "b1");
        vvd(b2, 0.8928551362746343397, 1e-12, "tcbtdb", "b2");
    }

    #[test]
    fn test_tcgtt() {
        let (t1, t2) = ts::tcgtt(2453750.5, 0.892862531).unwrap();
        vvd(t1, 2453750.5, 1e-6, "tcgtt", "t1");
        vvd(t2, 0.8928551387488816828, 1e-12, "tcgtt", "t2");
    }

    #[test]
    fn test_tdbtcb() {
        let (b1, b2) = ts::tdbtcb(2453750.5, 0.892855137).unwrap();
        vvd(b1, 2453750.5, 1e-6, "tdbtcb", "b1");
        vvd(b2, 0.8930195997253656716, 1e-12, "tdbtcb", "b2");
    }

    #[test]
    fn test_tdbtt() {
        let (t1, t2) = ts::tdbtt(2453750.5, 0.892855137, -0.000201).unwrap();
        vvd(t1, 2453750.5, 1e-6, "tdbtt", "t1");
        vvd(t2, 0.8928551393263888889, 1e-12, "tdbtt", "t2");
    }

    #[test]
    fn test_tttai() {
        let (a1, a2) = ts::tttai(2453750.5, 0.892482639).unwrap();
        vvd(a1, 2453750.5, 1e-6, "tttai", "a1");
        vvd(a2, 0.892110139, 1e-12, "tttai", "a2");
    }

    #[test]
    fn test_tttcg() {
        let (g1, g2) = ts::tttcg(2453750.5, 0.892482639).unwrap();
        vvd(g1, 2453750.5, 1e-6, "tttcg", "g1");
        vvd(g2, 0.8924900312508587113, 1e-12, "tttcg", "g2");
    }

    #[test]
    fn test_tttdb() {
        let (b1, b2) = ts::tttdb(2453750.5, 0.892855139, -0.000201).unwrap();
        vvd(b1, 2453750.5, 1e-6, "tttdb", "b1");
        vvd(b2, 0.8928551366736111111, 1e-12, "tttdb", "b2");
    }

    #[test]
    fn test_ttut1() {
        let (u1, u2) = ts::ttut1(2453750.5, 0.892855139, 64.8499).unwrap();
        vvd(u1, 2453750.5, 1e-6, "ttut1", "u1");
        vvd(u2, 0.8921045614537037037, 1e-12, "ttut1", "u2");
    }

    #[test]
    fn test_ut1tai() {
        let (a1, a2) = ts::ut1tai(2453750.5, 0.892104561, -32.6659).unwrap();
        vvd(a1, 2453750.5, 1e-6, "ut1tai", "a1");
        vvd(a2, 0.8924826385462962963, 1e-12, "ut1tai", "a2");
    }

    #[test]
    fn test_ut1utc() {
        let (u1, u2) = ts::ut1utc(2453750.5, 0.892104561, 0.3341).unwrap();
        vvd(u1, 2453750.5, 1e-6, "ut1utc", "u1");
        vvd(u2, 0.8921006941018518519, 1e-12, "ut1utc", "u2");
    }
}
