use super::{IauAstrom, apcs, pvtob};
use crate::pnp::c2ixys;
use crate::vm::{anpm, cr, ir, rx, ry, rz, trxpv};

///  Prepare for ICRS <−> observed, terrestrial, special
///
///  For a terrestrial observer, prepare star-independent astrometry
///  parameters for transformations between ICRS and observed
///  coordinates.  The caller supplies the Earth ephemeris, the Earth
///  rotation information and the refraction constants as well as the
///  site coordinates.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  Given:
///  ```
///     date1  double       TDB as a 2-part...
///     date2  double       ...Julian Date (Note 1)
///     ebpv   double[2][3] Earth barycentric PV (au, au/day, Note 2)
///     ehp    double[3]    Earth heliocentric P (au, Note 2)
///     x,y    double       CIP X,Y (components of unit vector)
///     s      double       the CIO locator s (radians)
///     theta  double       Earth rotation angle (radians)
///     elong  double       longitude (radians, east +ve, Note 3)
///     phi    double       latitude (geodetic, radians, Note 3)
///     hm     double       height above ellipsoid (m, geodetic, Note 3)
///     xp,yp  double       polar motion coordinates (radians, Note 4)
///     sp     double       the TIO locator s' (radians, Note 4)
///     refa   double       refraction constant A (radians, Note 5)
///     refb   double       refraction constant B (radians, Note 5)
///  ```
///  Returned:
///  ```
///     astrom iauASTROM*   star-independent astrometry parameters:
///      pmt    double       PM time interval (SSB, Julian years)
///      eb     double[3]    SSB to observer (vector, au)
///      eh     double[3]    Sun to observer (unit vector)
///      em     double       distance from Sun to observer (au)
///      v      double[3]    barycentric observer velocity (vector, c)
///      bm1    double       sqrt(1-|v|^2): reciprocal of Lorenz factor
///      bpn    double[3][3] bias-precession-nutation matrix
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
///  1) The TDB date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TDB)=2450123.7 could be expressed in any of these ways, among
///     others:
///  ```
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///  ```
///     The JD method is the most natural and convenient to use in cases
///     where the loss of several decimal digits of resolution is
///     acceptable.  The J2000 method is best matched to the way the
///     argument is handled internally and will deliver the optimum
///     resolution.  The MJD method and the date & time methods are both
///     good compromises between resolution and convenience.  For most
///     applications of this function the choice will not be at all
///     critical.
///
///     TT can be used instead of TDB without any significant impact on
///     accuracy.
///
///  2) The vectors eb, eh, and all the astrom vectors, are with respect
///     to BCRS axes.
///
///  3) The geographical coordinates are with respect to the WGS84
///     reference ellipsoid.  TAKE CARE WITH THE LONGITUDE SIGN
///     CONVENTION:  the longitude required by the present function is
///     right-handed, i.e. east-positive, in accordance with geographical
///     convention.
///
///     The adjusted longitude stored in the astrom array takes into
///     account the TIO locator and polar motion.
///
///  4) xp and yp are the coordinates (in radians) of the Celestial
///     Intermediate Pole with respect to the International Terrestrial
///     Reference System (see IERS Conventions), measured along the
///     meridians 0 and 90 deg west respectively.  sp is the TIO locator
///     s', in radians, which positions the Terrestrial Intermediate
///     Origin on the equator.  For many applications, xp, yp and
///     (especially) sp can be set to zero.
///
///     Internally, the polar motion is stored in a form rotated onto the
///     local meridian.
///
///  5) The refraction constants refa and refb are for use in a
///     dZ = A*tan(Z)+B*tan^3(Z) model, where Z is the observed
///     (i.e. refracted) zenith distance and dZ is the amount of
///     refraction.
///
///  6) It is advisable to take great care with units, as even unlikely
///     values of the input parameters are accepted and processed in
///     accordance with the models used.
///
///  7) In cases where the caller does not wish to provide the Earth
///     Ephemeris, the Earth rotation information and refraction
///     constants, the function iauApco13 can be used instead of the
///     present function.  This starts from UTC and weather readings etc.
///     and computes suitable values using other SOFA functions.
///
///  8) This is one of several functions that inserts into the astrom
///     structure star-independent parameters needed for the chain of
///     astrometric transformations ICRS <-> GCRS <-> CIRS <-> observed.
///
///     The various functions support different classes of observer and
///     portions of the transformation chain:
///     ```
///          functions         observer        transformation
///
///       iauApcg iauApcg13    geocentric      ICRS <-> GCRS
///       iauApci iauApci13    terrestrial     ICRS <-> CIRS
///       iauApco iauApco13    terrestrial     ICRS <-> observed
///       iauApcs iauApcs13    space           ICRS <-> GCRS
///       iauAper iauAper13    terrestrial     update Earth rotation
///       iauApio iauApio13    terrestrial     CIRS <-> observed
///     ```
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
///  9) The context structure astrom produced by this function is used by
///     iauAtioq, iauAtoiq, iauAtciq* and iauAticq*.
///
///  Called:
///  ```
///     iauIr        initialize r-matrix to identity
///     iauRz        rotate around Z-axis
///     iauRy        rotate around Y-axis
///     iauRx        rotate around X-axis
///     iauAnpm      normalize angle into range +/- pi
///     iauC2ixys    celestial-to-intermediate matrix, given X,Y and s
///     iauPvtob     position/velocity of terrestrial station
///     iauTrxpv     product of transpose of r-matrix and pv-vector
///     iauApcs      astrometry parameters, ICRS-GCRS, space observer
///     iauCr        copy r-matrix
///  ```
pub fn apco(
    date1: f64,
    date2: f64,
    ebpv: &[[f64; 3]; 2],
    ehp: &[f64; 3],
    x: f64,
    y: f64,
    s: f64,
    theta: f64,
    elong: f64,
    phi: f64,
    hm: f64,
    xp: f64,
    yp: f64,
    sp: f64,
    refa: f64,
    refb: f64,
    astrom: &mut IauAstrom,
) {
    let r = &mut [[0.0; 3]; 3];
    let (mut a, mut b, eral, c);

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

    // Refraction constants.
    astrom.refa = refa;
    astrom.refb = refb;

    // Disable the (redundant) diurnal aberration step.
    astrom.diurab = 0.0;

    /* CIO based BPN matrix. */
    c2ixys(x, y, s, r);

    /* Observer's geocentric position and velocity (m, m/s, CIRS). */
    let pvc = &mut [[0.0; 3]; 2];
    pvtob(elong, phi, hm, xp, yp, sp, theta, pvc);

    /* Rotate into GCRS. */
    let pv = &mut [[0.0; 3]; 2];
    trxpv(r, pvc, pv);

    /* ICRS <-> GCRS parameters. */
    apcs(date1, date2, pv, &ebpv, &ehp, astrom);

    /* Store the CIO based BPN matrix. */
    cr(r, &mut astrom.bpn);
}
