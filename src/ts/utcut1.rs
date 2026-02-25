use crate::cal::jd2cal;
use crate::ts::{dat, taiut1, utctai};

///  Time scale transformation:  Coordinated Universal Time, UTC, to
///  Universal Time, UT1.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  canonical.
///
///  Given:
///     utc1,utc2  double   UTC as a 2-part quasi Julian Date (Note 1)
///     dut1       double   Delta UT1: UT1-UTC in seconds (Note 2)
///
///  Returned:
///     ut11,ut12  double   UT1 as a 2-part Julian Date (Note 3)
///
///  Returned (function value):
///                int      status: +1 = dubious year (Note 4)
///                                  0 = OK
///                                 -1 = unacceptable date
///
pub fn utcut1(utc1: f64, utc2: f64, dut1: f64) -> Result<(f64, f64), i32> {
    let (iy, im, id, _) = match jd2cal(utc1, utc2) {
        Ok((iy, im, id, fd)) => (iy, im, id, fd),
        Err(_) => return Err(-1),
    };

    let dat_val = match dat(iy, im, id, 0.0) {
        Ok(v) => v,
        Err(_) => return Err(-1),
    };

    let dta = dut1 - dat_val;

    let (tai1, tai2) = match utctai(utc1, utc2) {
        Ok(v) => v,
        Err(js) => return Err(js),
    };

    let (ut11, ut12) = match taiut1(tai1, tai2, dta) {
        Ok((ut11, ut12)) => (ut11, ut12),
        Err(_) => return Err(-1),
    };

    Ok((ut11, ut12))
}
