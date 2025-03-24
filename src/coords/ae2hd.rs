///  (azimuth, altitude) to (hour angle, declination)
///
///  Horizon to equatorial coordinates:  transform azimuth and altitude
///  to hour angle and declination.
///
///  Given:
///  ```
///     az       double       azimuth
///     el       double       altitude (informally, elevation)
///     phi      double       site latitude
///  ```
///  Returned:
///  ```
///     ha       double       hour angle (local)
///     dec      double       declination
///  ```
///  Notes:
///
///  1)  All the arguments are angles in radians.
///
///  2)  The sign convention for azimuth is north zero, east +pi/2.
///
///  3)  HA is returned in the range +/-pi.  Declination is returned in
///      the range +/-pi/2.
///
///  4)  The latitude phi is pi/2 minus the angle between the Earth's
///      rotation axis and the adopted zenith.  In many applications it
///      will be sufficient to use the published geodetic latitude of the
///      site.  In very precise (sub-arcsecond) applications, phi can be
///      corrected for polar motion.
///
///  5)  The azimuth az must be with respect to the rotational north pole,
///      as opposed to the ITRS pole, and an azimuth with respect to north
///      on a map of the Earth's surface will need to be adjusted for
///      polar motion if sub-arcsecond accuracy is required.
///
///  6)  Should the user wish to work with respect to the astronomical
///      zenith rather than the geodetic zenith, phi will need to be
///      adjusted for deflection of the vertical (often tens of
///      arcseconds), and the zero point of ha will also be affected.
///
///  7)  The transformation is the same as Ve = Ry(phi-pi/2)*Rz(pi)*Vh,
///      where Ve and Vh are lefthanded unit vectors in the (ha,dec) and
///      (az,el) systems respectively and Rz and Ry are rotations about
///      first the z-axis and then the y-axis.  (n.b. Rz(pi) simply
///      reverses the signs of the x and y components.)  For efficiency,
///      the algorithm is written out rather than calling other utility
///      functions.  For applications that require even greater
///      efficiency, additional savings are possible if constant terms
///      such as functions of latitude are computed once and for all.
///
///  8)  Again for efficiency, no range checking of arguments is carried
///      out.
pub fn ae2hd(az: f64, el: f64, phi: f64) -> [f64; 2] {
    let sa = az.sin();
    let ca = az.cos();
    let se = el.sin();
    let ce = el.cos();
    let sp = phi.sin();
    let cp = phi.cos();

    // HA,Dec unit vector.
    let x = -ca * ce * sp + se * cp;
    let y = -sa * ce;
    let z = ca * ce * cp + se * sp;

    // To spherical.
    let r = (x * x + y * y).sqrt();
    let ha = if r != 0.0 { y.atan2(x) } else { 0.0 };
    let dec = z.atan2(r);

    [ha, dec]
}
