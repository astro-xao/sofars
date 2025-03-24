use crate::astro::ld;
use crate::consts::{AULT, DAYSEC};
use crate::vm::{cp, pdp, pmp, pn, ppsp};

use super::IauLdBody;

///  Light deflection by multiple solar−system bodies
///
///  For a star, apply light deflection by multiple solar-system bodies,
///  as part of transforming coordinate direction into natural direction.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  Given:
///  ```
///     n    int           number of bodies (note 1)
///     b    iauLDBODY[n]  data for each of the n bodies (Notes 1,2):
///      bm   double         mass of the body (solar masses, Note 3)
///      dl   double         deflection limiter (Note 4)
///      pv   [2][3]         barycentric PV of the body (au, au/day)
///     ob   double[3]     barycentric position of the observer (au)
///     sc   double[3]     observer to star coord direction (unit vector)
///  ```
///  Returned:
///  ```
///     sn    double[3]      observer to deflected star (unit vector)
///  ```
///  1) The array b contains n entries, one for each body to be
///     considered.  If n = 0, no gravitational light deflection will be
///     applied, not even for the Sun.
///
///  2) The array b should include an entry for the Sun as well as for
///     any planet or other body to be taken into account.  The entries
///     should be in the order in which the light passes the body.
///
///  3) In the entry in the b array for body i, the mass parameter
///     b[i].bm can, as required, be adjusted in order to allow for such
///     effects as quadrupole field.
///
///  4) The deflection limiter parameter b[i].dl is phi^2/2, where phi is
///     the angular separation (in radians) between star and body at
///     which limiting is applied.  As phi shrinks below the chosen
///     threshold, the deflection is artificially reduced, reaching zero
///     for phi = 0.   Example values suitable for a terrestrial
///     observer, together with masses, are as follows:
///  ```
///        body i     b[i].bm        b[i].dl
///
///        Sun        1.0            6e-6
///        Jupiter    0.00095435     3e-9
///        Saturn     0.00028574     3e-10
///  ```
///  5) For cases where the starlight passes the body before reaching the
///     observer, the body is placed back along its barycentric track by
///     the light time from that point to the observer.  For cases where
///     the body is "behind" the observer no such shift is applied.  If
///     a different treatment is preferred, the user has the option of
///     instead using the iauLd function.  Similarly, iauLd can be used
///     for cases where the source is nearby, not a star.
///
///  6) The returned vector sn is not normalized, but the consequential
///     departure from unit magnitude is always negligible.
///
///  7) The arguments sc and sn can be the same array.
///
///  8) For efficiency, validation is omitted.  The supplied masses must
///     be greater than zero, the position and velocity vectors must be
///     right, and the deflection limiter greater than zero.
///
///  Reference:
///
///     Urban, S. & Seidelmann, P. K. (eds), Explanatory Supplement to
///     the Astronomical Almanac, 3rd ed., University Science Books
///     (2013), Section 7.2.4.
///
///  Called:
///  ```
///     iauCp        copy p-vector
///     iauPdp       scalar product of two p-vectors
///     iauPmp       p-vector minus p-vector
///     iauPpsp      p-vector plus scaled p-vector
///     iauPn        decompose p-vector into modulus and direction
///     iauLd        light deflection by a solar-system body
///  ```
pub fn ldn(n: i32, b: &[IauLdBody], ob: &[f64; 3], sc: &[f64; 3]) -> [f64; 3] {
    let mut sn = [0.0; 3];
    /* Light time for 1 au (days) */
    const CR: f64 = AULT / DAYSEC;

    // int i;
    // double  v[3], dt, ev[3], em, e[3];

    /* Star direction prior to deflection. */
    cp(&sc, &mut sn);

    /* Body by body. */
    for i in 0..n {
        /* Body to observer vector at epoch of observation (au). */
        let v = pmp(ob, &b[i as usize].pv[0]);

        /* Minus the time since the light passed the body (days). */
        let dt = pdp(&mut sn, &v) * CR;

        /* Neutralize if the star is "behind" the observer. */
        let dt = dt.min(0.0);

        /* Backtrack the body to the time the light was passing the body. */
        let ev = ppsp(&v, -dt, &b[i as usize].pv[1]);

        /* Body to observer vector as magnitude and direction. */
        let (em, e) = pn(&ev);

        /* Apply light deflection for this body. */
        sn = ld(b[i as usize].bm, sn, sn, e, em, b[i as usize].dl);
        /* Next body. */
    }

    sn
}
