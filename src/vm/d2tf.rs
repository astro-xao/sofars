use crate::consts::DAYSEC;

/// Decompose days to hours, minutes, seconds, fraction.
pub fn d2tf(ndp: i32, days: f64) -> (char, [i32; 4]) {
    // let mut ihmsf = vec![0; 4];
    let mut nrs;
    let (mut a, mut rs, rm, rh, w, ah, am, a_s, af);

    // Handle sign.
    let sign = if days >= 0.0 { '+' } else { '-' };

    // Interval in seconds.
    a = DAYSEC * days.abs();

    // Pre-round if resolution coarser than 1s (then pretend ndp=1).
    if ndp < 0 {
        nrs = 1;
        for i in 1..=-ndp {
            nrs *= if i == 2 || i == 4 { 6 } else { 10 };
        }
        rs = nrs as f64;
        w = a / rs;
        a = rs * w.round();
    }

    // Express the unit of each field in resolution units.
    nrs = 1;
    for _ in 1..=ndp {
        nrs *= 10;
    }
    rs = nrs as f64;
    rm = rs * 60.0;
    rh = rm * 60.0;

    // Round the interval and express in resolution units.
    a = (rs * a).round();

    // Break into fields.
    ah = (a / rh).floor();
    a -= ah * rh;
    am = (a / rm).floor();
    a -= am * rm;
    a_s = (a / rs).floor();
    af = a - a_s * rs;

    // Return results.
    (sign, [ah as i32, am as i32, a_s as i32, af as i32])
}
