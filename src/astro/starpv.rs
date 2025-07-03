use crate::{
    consts::{DAU, DAYSEC, DC, DJY, DR2AS},
    vm::{pdp, pm, pmp, pn, ppp, s2pv, sxp, zp},
};

///  Convert star catalog coordinates to position+velocity vector.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  Given (Note 1):
///     ra     double        right ascension (radians)
///     dec    double        declination (radians)
///     pmr    double        RA proper motion (radians/year)
///     pmd    double        Dec proper motion (radians/year)
///     px     double        parallax (arcseconds)
///     rv     double        radial velocity (km/s, positive = receding)
///
///  Returned (Note 2):
///     pv     double[2][3]  pv-vector (au, au/day)
///
///  Returned (function value):
///            int           status:
///                              0 = no warnings
///                              1 = distance overridden (Note 6)
///                              2 = excessive speed (Note 7)
///                              4 = solution didn't converge (Note 8)
///                           else = binary logical OR of the above
///
///  Notes:
///
///  1) The star data accepted by this function are "observables" for an
///     imaginary observer at the solar-system barycenter.  Proper motion
///     and radial velocity are, strictly, in terms of barycentric
///     coordinate time, TCB.  For most practical applications, it is
///     permissible to neglect the distinction between TCB and ordinary
///     "proper" time on Earth (TT/TAI).  The result will, as a rule, be
///     limited by the intrinsic accuracy of the proper-motion and
///     radial-velocity data;  moreover, the pv-vector is likely to be
///     merely an intermediate result, so that a change of time unit
///     would cancel out overall.
///
///     In accordance with normal star-catalog conventions, the object's
///     right ascension and declination are freed from the effects of
///     secular aberration.  The frame, which is aligned to the catalog
///     equator and equinox, is Lorentzian and centered on the SSB.
///
///  2) The resulting position and velocity pv-vector is with respect to
///     the same frame and, like the catalog coordinates, is freed from
///     the effects of secular aberration.  Should the "coordinate
///     direction", where the object was located at the catalog epoch, be
///     required, it may be obtained by calculating the magnitude of the
///     position vector pv[0][0-2] dividing by the speed of light in
///     au/day to give the light-time, and then multiplying the space
///     velocity pv[1][0-2] by this light-time and adding the result to
///     pv[0][0-2].
///
///     Summarizing, the pv-vector returned is for most stars almost
///     identical to the result of applying the standard geometrical
///     "space motion" transformation.  The differences, which are the
///     subject of the Stumpff paper referenced below, are:
///
///     (i) In stars with significant radial velocity and proper motion,
///     the constantly changing light-time distorts the apparent proper
///     motion.  Note that this is a classical, not a relativistic,
///     effect.
///
///     (ii) The transformation complies with special relativity.
///
///  3) Care is needed with units.  The star coordinates are in radians
///     and the proper motions in radians per Julian year, but the
///     parallax is in arcseconds; the radial velocity is in km/s, but
///     the pv-vector result is in au and au/day.
///
///  4) The RA proper motion is in terms of coordinate angle, not true
///     angle.  If the catalog uses arcseconds for both RA and Dec proper
///     motions, the RA proper motion will need to be divided by cos(Dec)
///     before use.
///
///  5) Straight-line motion at constant speed, in the inertial frame,
///     is assumed.
///
///  6) An extremely small (or zero or negative) parallax is interpreted
///     to mean that the object is on the "celestial sphere", the radius
///     of which is an arbitrary (large) value (see the constant PXMIN).
///     When the distance is overridden in this way, the status,
///     initially zero, has 1 added to it.
///
///  7) If the space velocity is a significant fraction of c (see the
///     constant VMAX), it is arbitrarily set to zero.  When this action
///     occurs, 2 is added to the status.
///
///  8) The relativistic adjustment involves an iterative calculation.
///     If the process fails to converge within a set number (IMAX) of
///     iterations, 4 is added to the status.
///
///  9) The inverse transformation is performed by the function
///     iauPvstar.
///
///  Called:
///     iauS2pv      spherical coordinates to pv-vector
///     iauPm        modulus of p-vector
///     iauZp        zero p-vector
///     iauPn        decompose p-vector into modulus and direction
///     iauPdp       scalar product of two p-vectors
///     iauSxp       multiply p-vector by scalar
///     iauPmp       p-vector minus p-vector
///     iauPpp       p-vector plus p-vector
///
///  Reference:
///
///     Stumpff, P., 1985, Astron.Astrophys. 144, 232-240.
///
///  This revision:  2023 May 4
///
///  SOFA release 2023-10-11
///
///  Copyright (C) 2023 IAU SOFA Board.  See notes at end.
///
pub fn starpv(
    ra: f64,
    dec: f64,
    pmr: f64,
    pmd: f64,
    px: f64,
    rv: f64,
) -> Result<[[f64; 3]; 2], i32> {
    /* Smallest allowed parallax */
    const PXMIN: f64 = 1e-7;

    /* Largest allowed speed (fraction of c) */
    const VMAX: f64 = 0.5;

    /* Maximum number of iterations for relativistic solution */
    const IMAX: i32 = 100;

    let mut i: i32 = 0;
    let mut iwarn: i32;
    let mut w: f64;
    let (r, rd, rad, decd, v, vsr, vst): (f64, f64, f64, f64, f64, f64, f64);
    let (betst, betsr, mut bett, mut betr, mut dd, mut ddel): (f64, f64, f64, f64, f64, f64);

    let (mut d, mut del, mut odd, mut oddel, mut od, mut odel) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    // Distance (au).
    if px >= PXMIN {
        w = px;
        iwarn = 0;
    } else {
        w = PXMIN;
        iwarn = 1;
    }
    r = DR2AS / w;

    // Radial speed (au/day).
    rd = DAYSEC * rv * 1e3 / DAU;

    // Proper motion (radian/day).
    rad = pmr / DJY;
    decd = pmd / DJY;

    // To pv-vector (au, au/day).
    let mut pv = s2pv(ra, dec, r, rad, decd, rd);

    // If excessive velocity, arbitrarily set it to zero.
    v = pm(pv[1]);
    if v / DC > VMAX {
        zp(&mut pv[1]);
        iwarn += 2;
    }

    // Isolate the radial component of the velocity (au/day).
    let (_, pu) = pn(&pv[0]);
    vsr = pdp(&pu, &pv[1]);
    let usr = sxp(vsr, &pu);

    // Isolate the transverse component of the velocity (au/day).
    let ust = pmp(&pv[1], &usr);
    vst = pm(ust);

    // Special-relativity dimensionless parameters.
    betsr = vsr / DC;
    betst = vst / DC;

    // Determine the observed-to-inertial correction terms.
    bett = betst;
    betr = betsr;
    while i < IMAX {
        d = 1.0 + betr;
        w = betr * betr + bett * bett;
        del = -w / ((1.0 - w).sqrt() + 1.0);
        betr = d * betsr + del;
        bett = d * betst;
        if i > 0 {
            dd = (d - od).abs();
            ddel = (del - odel).abs();
            if i > 1 && dd >= odd && ddel >= oddel {
                break;
            }
            odd = dd;
            oddel = ddel;
        }
        od = d;
        odel = del;
        i += 1;
    }
    if i >= IMAX {
        iwarn += 4;
    }

    // Scale observed tangential velocity vector into inertial (au/d).
    let ut = sxp(d, &ust);

    // Compute inertial radial velocity vector (au/d).
    let ur = sxp(DC * (d * betsr + del), &pu);

    // Combine the two to obtain the inertial space velocity vector.
    pv[1] = ppp(&ur, &ut);

    if iwarn == 0 { Ok(pv) } else { Err(iwarn) }
}
