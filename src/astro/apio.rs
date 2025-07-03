use super::{IauAstrom, pvtob};
use crate::consts::CMPS;
use crate::vm::{anpm, ir, rx, ry, rz};

///  Prepare for CIRS <−> observed, terrestrial, special
///
///  For a terrestrial observer, prepare star-independent astrometry
///  parameters for transformations between CIRS and observed
///  coordinates.  The caller supplies the Earth orientation information
///  and the refraction constants as well as the site coordinates.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  Given:
///  ```
///     sp     double      the TIO locator s' (radians, Note 1)
///     theta  double      Earth rotation angle (radians)
///     elong  double      longitude (radians, east +ve, Note 2)
///     phi    double      geodetic latitude (radians, Note 2)
///     hm     double      height above ellipsoid (m, geodetic Note 2)
///     xp,yp  double      polar motion coordinates (radians, Note 3)
///     refa   double      refraction constant A (radians, Note 4)
///     refb   double      refraction constant B (radians, Note 4)
///  ```
///  Returned:
///  ```
///     astrom iauASTROM*  star-independent astrometry parameters:
///      pmt    double       unchanged
///      eb     double[3]    unchanged
///      eh     double[3]    unchanged
///      em     double       unchanged
///      v      double[3]    unchanged
///      bm1    double       unchanged
///      bpn    double[3][3] unchanged
///      along  double       adjusted longitude (radians)
///      xpl    double       polar motion xp wrt local meridian (radians)
///      ypl    double       polar motion yp wrt local meridian (radians)
///      sphi   double       sine of geodetic latitude
///      cphi   double       cosine of geodetic latitude
///      diurab double       magnitude of diurnal aberration vector
///      eral   double       "local" Earth rotation angle (radians)
///      refa   double       refraction constant A (radians)
///      refb   double       refraction constant B (radians)
///  ```
///  Notes:
///
///  1) sp, the TIO locator s', is a tiny quantity needed only by the
///     most precise applications.  It can either be set to zero or
///     predicted using the SOFA function iauSp00.
///
///  2) The geographical coordinates are with respect to the WGS84
///     reference ellipsoid.  TAKE CARE WITH THE LONGITUDE SIGN:  the
///     longitude required by the present function is east-positive
///     (i.e. right-handed), in accordance with geographical convention.
///
///  3) The polar motion xp,yp can be obtained from IERS bulletins.  The
///     values are the coordinates (in radians) of the Celestial
///     Intermediate Pole with respect to the International Terrestrial
///     Reference System (see IERS Conventions 2003), measured along the
///     meridians 0 and 90 deg west respectively.  For many applications,
///     xp and yp can be set to zero.
///
///     Internally, the polar motion is stored in a form rotated onto the
///     local meridian.
///
///  4) The refraction constants refa and refb are for use in a
///     dZ = A*tan(Z)+B*tan^3(Z) model, where Z is the observed
///     (i.e. refracted) zenith distance and dZ is the amount of
///     refraction.
///
///  5) It is advisable to take great care with units, as even unlikely
///     values of the input parameters are accepted and processed in
///     accordance with the models used.
///
///  6) In cases where the caller does not wish to provide the Earth
///     rotation information and refraction constants, the function
///     iauApio13 can be used instead of the present function.  This
///     starts from UTC and weather readings etc. and computes suitable
///     values using other SOFA functions.
///
///  7) This is one of several functions that inserts into the astrom
///     structure star-independent parameters needed for the chain of
///     astrometric transformations ICRS <-> GCRS <-> CIRS <-> observed.
///
///     The various functions support different classes of observer and
///     portions of the transformation chain:
///  ```
///          functions         observer        transformation
///
///       iauApcg iauApcg13    geocentric      ICRS <-> GCRS
///       iauApci iauApci13    terrestrial     ICRS <-> CIRS
///       iauApco iauApco13    terrestrial     ICRS <-> observed
///       iauApcs iauApcs13    space           ICRS <-> GCRS
///       iauAper iauAper13    terrestrial     update Earth rotation
///       iauApio iauApio13    terrestrial     CIRS <-> observed
///  ```
///     Those with names ending in "13" use contemporary SOFA models to
///     compute the various ephemerides.  The others accept ephemerides
///     supplied by the caller.
///
///     The transformation from ICRS to GCRS covers space motion,
///     parallax, light deflection, and aberration.  From GCRS to CIRS
///     comprises frame bias and precession-nutation.  From CIRS to
///     observed takes account of Earth rotation, polar motion, diurnal
///     aberration and parallax (unless subsumed into the ICRS <-> GCRS
///     transformation), and atmospheric refraction.
///
///  8) The context structure astrom produced by this function is used by
///     iauAtioq and iauAtoiq.
///
///  Called:
///  ```
///     iauIr        initialize r-matrix to identity
///     iauRz        rotate around Z-axis
///     iauRy        rotate around Y-axis
///     iauRx        rotate around X-axis
///     iauAnpm      normalize angle into range +/- pi
///     iauPvtob     position/velocity of terrestrial station
///  ```
pub fn apio(
    sp: f64,
    theta: f64,
    elong: f64,
    phi: f64,
    hm: f64,
    xp: f64,
    yp: f64,
    refa: f64,
    refb: f64,
    astrom: &mut IauAstrom,
) {
    let r = &mut [[0.0; 3]; 3];
    let mut a;
    let mut b;
    let eral;
    let c;
    let pv = &mut [[0.0; 3]; 2];

    // Form the rotation matrix, CIRS to apparent [HA,Dec].
    ir(r);
    rz(theta + sp, r);
    ry(-xp, r);
    rx(-yp, r);
    rz(elong, r);

    // Solve for local Earth rotation angle.
    a = r[0][0];
    b = r[0][1];
    eral = if a != 0.0 || b != 0.0 {
        b.atan2(a)
    } else {
        0.0
    };
    astrom.eral = eral;

    // Solve for polar motion [X,Y] with respect to local meridian.
    a = r[0][0];
    c = r[0][2];
    astrom.xpl = c.atan2((a * a + b * b).sqrt());
    a = r[1][2];
    b = r[2][2];
    astrom.ypl = if a != 0.0 || b != 0.0 {
        -a.atan2(b)
    } else {
        0.0
    };

    // Adjusted longitude.
    astrom.along = anpm(eral - theta);

    // Functions of latitude.
    astrom.sphi = phi.sin();
    astrom.cphi = phi.cos();

    // Observer's geocentric position and velocity (m, m/s, CIRS).
    pvtob(elong, phi, hm, xp, yp, sp, theta, pv);

    // Magnitude of diurnal aberration vector.
    astrom.diurab = (pv[1][0] * pv[1][0] + pv[1][1] * pv[1][1]).sqrt() / CMPS;

    // Refraction constants.
    astrom.refa = refa;
    astrom.refb = refb;
}
