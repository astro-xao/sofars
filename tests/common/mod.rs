/// Validate a double result (equivalent to vvd in t_sofa_c.c)
#[track_caller]
pub fn vvd(val: f64, valok: f64, dval: f64, func: &str, test: &str) {
    let diff = (val - valok).abs();
    let tol = dval.abs();
    assert!(
        diff <= tol,
        "{} failed: {} want {:.20} got {:.20} (diff: {:.3e}, tol: {:.3e})",
        func,
        test,
        valok,
        val,
        diff,
        tol
    );
}

/// Validate an integer result (equivalent to viv in t_sofa_c.c)
#[track_caller]
#[allow(dead_code)]
pub fn viv(ival: i32, ivalok: i32, func: &str, test: &str) {
    assert_eq!(ival, ivalok, "{} failed: {}", func, test);
}
