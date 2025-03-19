pub fn jd2cal(dj1: f64, dj2: f64) -> Result<(i32, i32, i32, f64), i32> {
    /* Minimum and maximum allowed JD */
    const DJMIN: f64 = -68569.5;
    const DJMAX: f64 = 1e9;

    let mut jd: i64;
    let mut l: i64;
    let n: i64;
    let k: i64;
    let dj: f64;
    let f1: f64;
    let f2: f64;
    let mut d: f64;
    let mut s: f64;
    let mut cs: f64;
    let v: [f64; 2];
    let mut x: f64;
    let mut t: f64;
    let mut f: f64;

    /* Verify date is acceptable. */
    dj = dj1 + dj2;
    if dj < DJMIN || dj > DJMAX {
        return Err(-1);
    }

    /* Separate day and fraction (where -0.5 <= fraction < 0.5). */
    d = dj1.round();
    f1 = dj1 - d;
    jd = d as i64;
    d = dj2.round();
    f2 = dj2 - d;
    jd += d as i64;

    /* Compute f1+f2+0.5 using compensated summation (Klein 2006). */
    s = 0.5;
    cs = 0.0;
    v = [f1, f2];
    for i in 0..2 {
        x = v[i];
        t = s + x;
        cs += if s.abs() >= x.abs() { (s - t) + x } else { (x - t) + s };
        s = t;
        if s >= 1.0 {
            jd += 1;
            s -= 1.0;
        }
    }
    f = s + cs;
    cs = f - s;

    /* Deal with negative f. */
    if f < 0.0 {
        /* Compensated summation: assume that |s| <= 1.0. */
        f = s + 1.0;
        cs += (1.0 - f) + s;
        s = f;
        f = s + cs;
        cs = f - s;
        jd -= 1;
    }

    /* Deal with f that is 1.0 or more (when rounded to double). */
    if (f - 1.0) >= -f64::EPSILON / 4.0 {
        /* Compensated summation: assume that |s| <= 1.0. */
        t = s - 1.0;
        cs += (s - t) - 1.0;
        s = t;
        f = s + cs;
        if -f64::EPSILON / 2.0 < f {
            jd += 1;
            f = f.max(0.0);
        }
    }

    /* Express day in Gregorian calendar. */
    l = jd + 68569;
    n = (4 * l) / 146097;
    l -= (146097 * n + 3) / 4;
    let i = (4000 * (l + 1)) / 1461001;
    l -= (1461 * i) / 4 - 31;
    k = (80 * l) / 2447;
    let id = (l - (2447 * k) / 80) as i32;
    l = k / 11;
    let im = (k + 2 - 12 * l) as i32;
    let iy = (100 * (n - 49) + i + l) as i32;
    let fd = f;

    /* Success. */
    Ok((iy, im, id, fd))
}