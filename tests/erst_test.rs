mod common;

#[cfg(test)]
mod tests {
    use super::common::vvd;
    use sofars::erst;

    #[test]
    fn test_ee00() {
        let epsa = 0.4090789763356509900;
        let dpsi = -0.9630909107115582393e-5;

        let ee = erst::ee00(2400000.5, 53736.0, epsa, dpsi);

        vvd(ee, -0.8834193235367965479e-5, 1e-18, "ee00", "status");
    }

    #[test]
    fn test_ee00a() {
        let ee = erst::ee00a(2400000.5, 53736.0);

        vvd(ee, -0.8834192459222588227e-5, 1e-18, "ee00a", "status");
    }

    #[test]
    fn test_ee00b() {
        let ee = erst::ee00b(2400000.5, 53736.0);

        vvd(ee, -0.8835700060003032831e-5, 1e-18, "ee00b", "status");
    }

    #[test]
    fn test_ee06a() {
        let ee = erst::ee06a(2400000.5, 53736.0);

        vvd(ee, -0.8834195072043790156e-5, 1e-15, "ee06a", "status");
    }

    #[test]
    fn test_eect00() {
        let eect = erst::eect00(2400000.5, 53736.0);

        vvd(eect, 0.2046085004885125264e-8, 1e-20, "eect00", "status");
    }

    #[test]
    fn test_eqeq94() {
        let eqeq = erst::eqeq94(2400000.5, 41234.0);

        vvd(eqeq, 0.5357758254609256894e-4, 1e-17, "eqeq94", "status");
    }

    #[test]
    fn test_era00() {
        let era = erst::era00(2400000.5, 54388.0);

        vvd(era, 0.4022837240028158102, 1e-12, "era00", "status");
    }

    #[test]
    fn test_gmst00() {
        let theta = erst::gmst00(2400000.5, 53736.0, 2400000.5, 53736.0);

        vvd(theta, 1.754174972210740592, 1e-12, "gmst00", "status");
    }

    #[test]
    fn test_gmst06() {
        let theta = erst::gmst06(2400000.5, 53736.0, 2400000.5, 53736.0);

        vvd(theta, 1.754174971870091203, 1e-12, "gmst06", "status");
    }

    #[test]
    fn test_gmst82() {
        let theta = erst::gmst82(2400000.5, 53736.0);

        vvd(theta, 1.754174981860675096, 1e-12, "gmst82", "status");
    }

    #[test]
    fn test_gst00a() {
        let theta = erst::gst00a(2400000.5, 53736.0, 2400000.5, 53736.0);

        vvd(theta, 1.754166138018281369, 1e-12, "gst00a", "status");
    }

    #[test]
    fn test_gst00b() {
        let theta = erst::gst00b(2400000.5, 53736.0);

        vvd(theta, 1.754166136510680589, 1e-12, "gst00b", "status");
    }

    #[test]
    fn test_gst06() {
        let rnpb = [
            [
                0.9999989440476103608,
                -0.1332881761240011518e-2,
                -0.5790767434730085097e-3,
            ],
            [
                0.1332858254308954453e-2,
                0.9999991109044505944,
                -0.4097782710401555759e-4,
            ],
            [
                0.5791308472168153320e-3,
                0.4020595661593994396e-4,
                0.9999998314954572365,
            ],
        ];

        let theta = erst::gst06(2400000.5, 53736.0, 2400000.5, 53736.0, &rnpb);

        vvd(theta, 1.754166138018167568, 1e-12, "gst06", "status");
    }

    #[test]
    fn test_gst06a() {
        let theta = erst::gst06a(2400000.5, 53736.0, 2400000.5, 53736.0);

        vvd(theta, 1.754166137675019159, 1e-12, "gst06a", "status");
    }

    #[test]
    fn test_gst94() {
        let theta = erst::gst94(2400000.5, 53736.0);

        vvd(theta, 1.754166136020645203, 1e-12, "gst94", "status");
    }
}
