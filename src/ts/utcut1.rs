use crate::cal::jd2cal;
use crate::ts::{dat, taiut1, utctai};

pub fn utcut1(utc1: f64, utc2: f64, dut1: f64) -> Result<(f64, f64), i32> {
    let (iy, im, id, _w) = match jd2cal(utc1, utc2) {
        Ok((iy, im, id, fd)) => (iy, im, id, fd),
        Err(_) => return Err(-1),
    };

    let dat = match dat(iy, im, id, 0.0) {
        Ok(dat) => dat,
        Err(_) => return Err(-1),
    };

    let dta = dut1 - dat;

    let (tai1, tai2) = match utctai(utc1, utc2) {
        Ok((tai1, tai2)) => (tai1, tai2),
        Err(js) => return Err(js),
    };

    let (ut11, ut12) = match taiut1(tai1, tai2, dta) {
        Ok((ut11, ut12)) => (ut11, ut12),
        Err(_) => return Err(-1),
    };

    Ok((ut11, ut12))
}
