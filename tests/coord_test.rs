mod common;

#[cfg(test)]
mod tests {
    use sofars::coords::*;
    use super::common::vvd;

    #[test]
    fn test_ae2hd() {
        let a = 5.5;
        let e = 1.1;
        let p = 0.7;

        let [h, d] = ae2hd(a, e, p);

        vvd(h, 0.5933291115507309663, 1e-14, "iauAe2hd", "h");
        vvd(d, 0.9613934761647817620, 1e-14, "iauAe2hd", "d");
    }

    #[test]
    fn test_hd2ae() {
        let h = 1.1;
        let d = 1.2;
        let p = 0.3;

        let [a, e] = hd2ae(h, d, p);

        vvd(a, 5.916889243730066194, 1e-13, "iauHd2ae", "a");
        vvd(e, 0.4472186304990486228, 1e-14, "iauHd2ae", "e");
    }

    #[test]
    fn test_hd2pa() {
        let h = 1.1;
        let d = 1.2;
        let p = 0.3;

        let q = hd2pa(h, d, p);

        vvd(q, 1.906227428001995580, 1e-13, "iauHd2pa", "q");
    }

    #[test]
    fn test_eceq06() {
        let date1 = 2456165.5;
        let date2 = 0.401182685;
        let dl = 5.1;
        let db = -0.9;

        let (dr, dd) = eceq06(date1, date2, dl, db);

        vvd(dr, 5.533459733613627767, 1e-14, "iauEceq06", "dr");
        vvd(dd, -1.246542932554480576, 1e-14, "iauEceq06", "dd");
    }

    #[test]
    fn test_ecm06() {
        let date1 = 2456165.5;
        let date2 = 0.401182685;

        let rm = ecm06(date1, date2);

        vvd(rm[0][0], 0.9999952427708701137, 1e-14, "iauEcm06", "rm11");
        vvd(rm[0][1], -0.2829062057663042347e-2, 1e-14, "iauEcm06", "rm12");
        vvd(rm[0][2], -0.1229163741100017629e-2, 1e-14, "iauEcm06", "rm13");
        vvd(rm[1][0], 0.3084546876908653562e-2, 1e-14, "iauEcm06", "rm21");
        vvd(rm[1][1], 0.9174891871550392514, 1e-14, "iauEcm06", "rm22");
        vvd(rm[1][2], 0.3977487611849338124, 1e-14, "iauEcm06", "rm23");
        vvd(rm[2][0], 0.2488512951527405928e-5, 1e-14, "iauEcm06", "rm31");
        vvd(rm[2][1], -0.3977506604161195467, 1e-14, "iauEcm06", "rm32");
        vvd(rm[2][2], 0.9174935488232863071, 1e-14, "iauEcm06", "rm33");
    }

    #[test]
    fn test_eqec06() {
        let date1 = 1234.5;
        let date2 = 2440000.5;
        let dr = 1.234;
        let dd = 0.987;

        let (dl, db) = eqec06(date1, date2, dr, dd);

        vvd(dl, 1.342509918994654619, 1e-14, "iauEqec06", "dl");
        vvd(db, 0.5926215259704608132, 1e-14, "iauEqec06", "db");
    }

    #[test]
    fn test_lteceq() {
        let epj = 2500.0;
        let dl = 1.5;
        let db = 0.6;

        let (dr, dd) = lteceq(epj, dl, db);

        vvd(dr, 1.275156021861921167, 1e-14, "iauLteceq", "dr");
        vvd(dd, 0.9966573543519204791, 1e-14, "iauLteceq", "dd");
    }

    #[test]
    fn test_ltecm() {
        let epj = -3000.0;

        let rm = ltecm(epj);

        vvd(rm[0][0], 0.3564105644859788825, 1e-14, "iauLtecm", "rm11");
        vvd(rm[0][1], 0.8530575738617682284, 1e-14, "iauLtecm", "rm12");
        vvd(rm[0][2], 0.3811355207795060435, 1e-14, "iauLtecm", "rm13");
        vvd(rm[1][0], -0.9343283469640709942, 1e-14, "iauLtecm", "rm21");
        vvd(rm[1][1], 0.3247830597681745976, 1e-14, "iauLtecm", "rm22");
        vvd(rm[1][2], 0.1467872751535940865, 1e-14, "iauLtecm", "rm23");
        vvd(rm[2][0], 0.1431636191201167793e-2, 1e-14, "iauLtecm", "rm31");
        vvd(rm[2][1], -0.4084222566960599342, 1e-14, "iauLtecm", "rm32");
        vvd(rm[2][2], 0.9127919865189030899, 1e-14, "iauLtecm", "rm33");
    }

    #[test]
    fn test_lteqec() {
        let epj = -1500.0;
        let dr = 1.234;
        let dd = 0.987;

        let (dl, db) = lteqec(epj, dr, dd);

        vvd(dl, 0.5039483649047114859, 1e-14, "iauLteqec", "dl");
        vvd(db, 0.5848534459726224882, 1e-14, "iauLteqec", "db");
    }

    #[test]
    fn test_g2icrs() {
        let dl = 5.5850536063818546461558;
        let db = -0.7853981633974483096157;

        let (dr, dd) = g2icrs(dl, db);

        vvd(dr, 5.9338074302227188048671, 1e-14, "iauG2icrs", "dr");
        vvd(dd, -1.1784870613579944551541, 1e-14, "iauG2icrs", "dd");
    }

    #[test]
    fn test_icrs2g() {
        let dr = 5.9338074302227188048671;
        let dd = -1.1784870613579944551541;

        let (dl, db) = icrs2g(dr, dd);

        vvd(dl, 5.5850536063818546461558, 1e-14, "iauIcrs2g", "dl");
        vvd(db, -0.7853981633974483096157, 1e-14, "iauIcrs2g", "db");
    }

    #[test]
    fn test_eform() {
        assert_eq!(eform(0), Err(-1));

        let (a1, f1) = eform(1).unwrap();
        vvd(a1, 6378137.0, 1e-10, "iauEform", "a1");
        vvd(f1, 0.3352810664747480720e-2, 1e-18, "iauEform", "f1");

        let (a2, f2) = eform(2).unwrap();
        vvd(a2, 6378137.0, 1e-10, "iauEform", "a2");
        vvd(f2, 0.3352810681182318935e-2, 1e-18, "iauEform", "f2");

        let (a3, f3) = eform(3).unwrap();
        vvd(a3, 6378135.0, 1e-10, "iauEform", "a3");
        vvd(f3, 0.3352779454167504862e-2, 1e-18, "iauEform", "f3");

        assert_eq!(eform(4), Err(-1));
    }

    #[test]
    fn test_gc2gd() {
        let xyz = [2e6, 3e6, 5.244e6];

        assert!(gc2gd(0, xyz).is_err());

        let (e1, p1, h1) = gc2gd(1, xyz).unwrap();
        vvd(e1, 0.9827937232473290680, 1e-14, "iauGc2gd", "e1");
        vvd(p1, 0.97160184819075459, 1e-14, "iauGc2gd", "p1");
        vvd(h1, 331.4172461426059892, 1e-8, "iauGc2gd", "h1");

        let (e2, p2, h2) = gc2gd(2, xyz).unwrap();
        vvd(e2, 0.9827937232473290680, 1e-14, "iauGc2gd", "e2");
        vvd(p2, 0.97160184820607853, 1e-14, "iauGc2gd", "p2");
        vvd(h2, 331.41731754844348, 1e-8, "iauGc2gd", "h2");

        let (e3, p3, h3) = gc2gd(3, xyz).unwrap();
        vvd(e3, 0.9827937232473290680, 1e-14, "iauGc2gd", "e3");
        vvd(p3, 0.9716018181101511937, 1e-14, "iauGc2gd", "p3");
        vvd(h3, 333.2770726130318123, 1e-8, "iauGc2gd", "h3");
    }

    #[test]
    fn test_gc2gde() {
        let a = 6378136.0;
        let f = 0.0033528;
        let xyz = [2e6, 3e6, 5.244e6];

        let (e, p, h) = gc2gde(a, f, xyz).unwrap();
        vvd(e, 0.9827937232473290680, 1e-14, "iauGc2gde", "e");
        vvd(p, 0.9716018377570411532, 1e-14, "iauGc2gde", "p");
        vvd(h, 332.36862495764397, 1e-8, "iauGc2gde", "h");
    }

    #[test]
    fn test_gd2gc() {
        let e = 3.1;
        let p = -0.5;
        let h = 2500.0;

        assert!(gd2gc(0, e, p, h).is_err());

        let xyz1 = gd2gc(1, e, p, h).unwrap();
        vvd(xyz1[0], -5599000.5577049947, 1e-7, "iauGd2gc", "1/1");
        vvd(xyz1[1], 233011.67223479203, 1e-7, "iauGd2gc", "2/1");
        vvd(xyz1[2], -3040909.4706983363, 1e-7, "iauGd2gc", "3/1");

        let xyz2 = gd2gc(2, e, p, h).unwrap();
        vvd(xyz2[0], -5599000.5577260984, 1e-7, "iauGd2gc", "1/2");
        vvd(xyz2[1], 233011.6722356702949, 1e-7, "iauGd2gc", "2/2");
        vvd(xyz2[2], -3040909.4706095476, 1e-7, "iauGd2gc", "3/2");

        let xyz3 = gd2gc(3, e, p, h).unwrap();
        vvd(xyz3[0], -5598998.7626301490, 1e-7, "iauGd2gc", "1/3");
        vvd(xyz3[1], 233011.5975297822211, 1e-7, "iauGd2gc", "2/3");
        vvd(xyz3[2], -3040908.6861467111, 1e-7, "iauGd2gc", "3/3");
    }

    #[test]
    fn test_gd2gce() {
        let a = 6378136.0;
        let f = 0.0033528;
        let e = 3.1;
        let p = -0.5;
        let h = 2500.0;

        let xyz = gd2gce(a, f, e, p, h).unwrap();
        vvd(xyz[0], -5598999.6665116328, 1e-7, "iauGd2gce", "1");
        vvd(xyz[1], 233011.6351463057189, 1e-7, "iauGd2gce", "2");
        vvd(xyz[2], -3040909.0517314132, 1e-7, "iauGd2gce", "3");
    }
}
