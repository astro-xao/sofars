mod common;

#[cfg(test)]
mod tests {
    use super::common::vvd;
    use sofars::fundargs;

    #[test]
    fn test_fad03() {
        vvd(
            fundargs::fad03(0.80),
            1.946709205396925672,
            1e-12,
            "fad03",
            "status",
        );
    }

    #[test]
    fn test_fae03() {
        vvd(
            fundargs::fae03(0.80),
            1.744713738913081846,
            1e-12,
            "fae03",
            "status",
        );
    }

    #[test]
    fn test_faf03() {
        vvd(
            fundargs::faf03(0.80),
            0.2597711366745499518,
            1e-12,
            "faf03",
            "status",
        );
    }

    #[test]
    fn test_faju03() {
        vvd(
            fundargs::faju03(0.80),
            5.275711665202481138,
            1e-12,
            "faju03",
            "status",
        );
    }

    #[test]
    fn test_fal03() {
        vvd(
            fundargs::fal03(0.80),
            5.132369751108684150,
            1e-12,
            "fal03",
            "status",
        );
    }

    #[test]
    fn test_falp03() {
        vvd(
            fundargs::falp03(0.80),
            6.226797973505507345,
            1e-12,
            "falp03",
            "status",
        );
    }

    #[test]
    fn test_fama03() {
        vvd(
            fundargs::fama03(0.80),
            3.275506840277781492,
            1e-12,
            "fama03",
            "status",
        );
    }

    #[test]
    fn test_fame03() {
        vvd(
            fundargs::fame03(0.80),
            5.417338184297289661,
            1e-12,
            "fame03",
            "status",
        );
    }

    #[test]
    fn test_fane03() {
        vvd(
            fundargs::fane03(0.80),
            2.079343830860413523,
            1e-12,
            "fane03",
            "status",
        );
    }

    #[test]
    fn test_faom03() {
        vvd(
            fundargs::faom03(0.80),
            -5.973618440951302183,
            1e-12,
            "faom03",
            "status",
        );
    }

    #[test]
    fn test_fapa03() {
        vvd(
            fundargs::fapa03(0.80),
            0.1950884762240000000e-1,
            1e-12,
            "fapa03",
            "status",
        );
    }

    #[test]
    fn test_fasa03() {
        vvd(
            fundargs::fasa03(0.80),
            5.371574539440827046,
            1e-12,
            "fasa03",
            "status",
        );
    }

    #[test]
    fn test_faur03() {
        vvd(
            fundargs::faur03(0.80),
            5.180636450180413523,
            1e-12,
            "faur03",
            "status",
        );
    }

    #[test]
    fn test_fave03() {
        vvd(
            fundargs::fave03(0.80),
            3.424900460533758000,
            1e-12,
            "fave03",
            "status",
        );
    }
}
