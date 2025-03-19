use crate::consts::DAYSEC;
pub fn taiut1(tai1: f64, tai2: f64, dta: f64) -> Result<(f64, f64), i32> {
    let dtad = dta / DAYSEC;

    let (ut11, ut12) = if tai1.abs() > tai2.abs() {
        (tai1, tai2 + dtad)
    } else {
        (tai1 + dtad, tai2)
    };

    Ok((ut11, ut12))
}