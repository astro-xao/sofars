use super::ld;

///  Light deflection by the Sun
/// 
///  Deflection of starlight by the Sun.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  Given:
///  ```
///     p      double[3]  direction from observer to star (unit vector)
///     e      double[3]  direction from Sun to observer (unit vector)
///     em     double     distance from Sun to observer (au)
///  ```
///  Returned:
///  ```
///     p1     double[3]  observer to deflected star (unit vector)
///  ```
///
///  Notes:
///
///  1) The source is presumed to be sufficiently distant that its
///     directions seen from the Sun and the observer are essentially
///     the same.
///
///  2) The deflection is restrained when the angle between the star and
///     the center of the Sun is less than a threshold value, falling to
///     zero deflection for zero separation.  The chosen threshold value
///     is within the solar limb for all solar-system applications, and
///     is about 5 arcminutes for the case of a terrestrial observer.
///
///  3) The arguments p and p1 can be the same array.
///
///  Called:
///  ```
///     iauLd        light deflection by a solar-system body
///  ```
pub fn ldsun(p: [f64; 3], e: [f64; 3], em: f64) -> [f64; 3] {
    /* Deflection limiter (smaller for distant observers). */
    let mut em2 = em * em;
    if em2 < 1.0 {
        em2 = 1.0;
    }
    let dlim = 1e-6 / if em2 > 1.0 { em2 } else { 1.0 };

    /* Apply the deflection. */
    ld(1.0, p, p, e, em, dlim)
}