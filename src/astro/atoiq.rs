use crate::vm::{anp, c2s, s2c};

use super::IauAstrom;

///  Quick observed −> CIRS
///
///  Quick observed place to CIRS, given the star-independent astrometry
///  parameters.
///
///  Use of this function is appropriate when efficiency is important and
///  where many star positions are all to be transformed for one date.
///  The star-independent astrometry parameters can be obtained by
///  calling iauApio[13] or iauApco[13].
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  Given:
///  ```
///     type   char[]     type of coordinates: "R", "H" or "A" (Note 1)
///     ob1    double     observed Az, HA or RA (radians; Az is N=0,E=90)
///     ob2    double     observed ZD or Dec (radians)
///     astrom iauASTROM* star-independent astrometry parameters:
///      pmt    double       PM time interval (SSB, Julian years)
///      eb     double[3]    SSB to observer (vector, au)
///      eh     double[3]    Sun to observer (unit vector)
///      em     double       distance from Sun to observer (au)
///      v      double[3]    barycentric observer velocity (vector, c)
///      bm1    double       sqrt(1-|v|^2): reciprocal of Lorenz factor
///      bpn    double[3][3] bias-precession-nutation matrix
///      along  double       longitude + s' (radians)
///      xpl    double       polar motion xp wrt local meridian (radians)
///      ypl    double       polar motion yp wrt local meridian (radians)
///      sphi   double       sine of geodetic latitude
///      cphi   double       cosine of geodetic latitude
///      diurab double       magnitude of diurnal aberration vector
///      eral   double       "local" Earth rotation angle (radians)
///      refa   double       refraction constant A (radians)
///      refb   double       refraction constant B (radians)
///  ```
///  Returned:
///  ```
///     ri     double*    CIRS right ascension (CIO-based, radians)
///     di     double*    CIRS declination (radians)
///  ```
///  Notes:
///
///  1) "Observed" Az,ZD means the position that would be seen by a
///     perfect geodetically aligned theodolite.  This is related to
///     the observed HA,Dec via the standard rotation, using the geodetic
///     latitude (corrected for polar motion), while the observed HA and
///     (CIO-based) RA are related simply through the Earth rotation
///     angle and the site longitude.  "Observed" RA,Dec or HA,Dec thus
///     means the position that would be seen by a perfect equatorial
///     with its polar axis aligned to the Earth's axis of rotation.
///
///  2) Only the first character of the type argument is significant.
///     "R" or "r" indicates that ob1 and ob2 are the observed right
///     ascension (CIO-based) and declination;  "H" or "h" indicates that
///     they are hour angle (west +ve) and declination;  anything else
///     ("A" or "a" is recommended) indicates that ob1 and ob2 are
///     azimuth (north zero, east 90 deg) and zenith distance.  (Zenith
///     distance is used rather than altitude in order to reflect the
///     fact that no allowance is made for depression of the horizon.)
///
///  3) The accuracy of the result is limited by the corrections for
///     refraction, which use a simple A*tan(z) + B*tan^3(z) model.
///     Providing the meteorological parameters are known accurately and
///     there are no gross local effects, the predicted intermediate
///     coordinates should be within 0.05 arcsec (optical) or 1 arcsec
///     (radio) for a zenith distance of less than 70 degrees, better
///     than 30 arcsec (optical or radio) at 85 degrees and better than
///     20 arcmin (optical) or 25 arcmin (radio) at the horizon.
///
///     Without refraction, the complementary functions iauAtioq and
///     iauAtoiq are self-consistent to better than 1 microarcsecond all
///     over the celestial sphere.  With refraction included, consistency
///     falls off at high zenith distances, but is still better than
///     0.05 arcsec at 85 degrees.
///
///  4) It is advisable to take great care with units, as even unlikely
///     values of the input parameters are accepted and processed in
///     accordance with the models used.
///
///  Called:
///     iauS2c       spherical coordinates to unit vector
///     iauC2s       p-vector to spherical
///     iauAnp       normalize angle into range 0 to 2pi
///
pub fn atoiq(type_: &str, ob1: f64, ob2: f64, astrom: &IauAstrom) -> (f64, f64) {
    /* Minimum sin(alt) for refraction purposes */
    const SELMIN: f64 = 0.05;

    let c1: f64;
    let c2: f64;
    let mut ce: f64;
    let xaeo: f64;
    let yaeo: f64;
    let zaeo: f64;
    let mut v = [0.0; 3];

    /* Coordinate type. */
    let c_char = type_.chars().next().unwrap_or('A');

    /* Coordinates. */
    c1 = ob1;
    c2 = ob2;

    /* Sin, cos of latitude. */
    let sphi = astrom.sphi;
    let cphi = astrom.cphi;

    /* Standardize coordinate type. */
    let c = if c_char == 'r' || c_char == 'R' {
        'R'
    } else if c_char == 'h' || c_char == 'H' {
        'H'
    } else {
        'A'
    };

    /* If Az,ZD, convert to Cartesian (S=0,E=90). */
    if c == 'A' {
        ce = c2.sin();
        xaeo = -c1.cos() * ce;
        yaeo = c1.sin() * ce;
        zaeo = c2.cos();
    } else {
        let c1_mod = if c == 'R' { astrom.eral - c1 } else { c1 };

        /* To Cartesian -HA,Dec. */
        v = s2c(-c1_mod, c2);
        let xmhdo = v[0];
        let ymhdo = v[1];
        let zmhdo = v[2];

        /* To Cartesian Az,El (S=0,E=90). */
        xaeo = sphi * xmhdo - cphi * zmhdo;
        yaeo = ymhdo;
        zaeo = cphi * xmhdo + sphi * zmhdo;
    }

    /* Azimuth (S=0,E=90). */
    let az = if xaeo != 0.0 || yaeo != 0.0 {
        yaeo.atan2(xaeo)
    } else {
        0.0
    };

    /* Sine of observed ZD, and observed ZD. */
    let sz = (xaeo * xaeo + yaeo * yaeo).sqrt();
    let zdo = sz.atan2(zaeo);

    /*
     ** Refraction
     ** ----------
     */

    /* Fast algorithm using two constant model. */
    let refa = astrom.refa;
    let refb = astrom.refb;
    let tz = sz / if zaeo > SELMIN { zaeo } else { SELMIN };
    let dref = (refa + refb * tz * tz) * tz;
    let zdt = zdo + dref;

    /* To Cartesian Az,ZD. */
    ce = zdt.sin();
    let xaet = az.cos() * ce;
    let yaet = az.sin() * ce;
    let zaet = zdt.cos();

    /* Cartesian Az,ZD to Cartesian -HA,Dec. */
    let xmhda = sphi * xaet + cphi * zaet;
    let ymhda = yaet;
    let zmhda = -cphi * xaet + sphi * zaet;

    /* Diurnal aberration. */
    let f = 1.0 + astrom.diurab * ymhda;
    let xhd = f * xmhda;
    let yhd = f * (ymhda - astrom.diurab);
    let zhd = f * zmhda;

    /* Polar motion. */
    let sx = astrom.xpl.sin();
    let cx = astrom.xpl.cos();
    let sy = astrom.ypl.sin();
    let cy = astrom.ypl.cos();
    v[0] = cx * xhd + sx * sy * yhd - sx * cy * zhd;
    v[1] = cy * yhd + sy * zhd;
    v[2] = sx * xhd - cx * sy * yhd + cx * cy * zhd;

    /* To spherical -HA,Dec. */
    let (hma, di) = c2s(&v);

    /* Right ascension. */
    let ri = anp(astrom.eral + hma);

    (ri, di)
}
