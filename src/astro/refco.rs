///  Refraction constants
/// 
///  Determine the constants A and B in the atmospheric refraction model
///  dZ = A tan Z + B tan^3 Z.
///
///  Z is the "observed" zenith distance (i.e. affected by refraction)
///  and dZ is what to add to Z to give the "topocentric" (i.e. in vacuo)
///  zenith distance.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  Given:
///  ```
///    phpa   double    pressure at the observer (hPa = millibar)
///    tc     double    ambient temperature at the observer (deg C)
///    rh     double    relative humidity at the observer (range 0-1)
///    wl     double    wavelength (micrometers)
///  ```
///  Returned:
///  ```
///    refa   double*   tan Z coefficient (radians)
///    refb   double*   tan^3 Z coefficient (radians)
///  ```
///  Notes:
///
///  1) The model balances speed and accuracy to give good results in
///     applications where performance at low altitudes is not paramount.
///     Performance is maintained across a range of conditions, and
///     applies to both optical/IR and radio.
///
///  2) The model omits the effects of (i) height above sea level (apart
///     from the reduced pressure itself), (ii) latitude (i.e. the
///     flattening of the Earth), (iii) variations in tropospheric lapse
///     rate and (iv) dispersive effects in the radio.
///
///     The model was tested using the following range of conditions:
///
///       lapse rates 0.0055, 0.0065, 0.0075 deg/meter
///       latitudes 0, 25, 50, 75 degrees
///       heights 0, 2500, 5000 meters ASL
///       pressures mean for height -10% to +5% in steps of 5%
///       temperatures -10 deg to +20 deg with respect to 280 deg at SL
///       relative humidity 0, 0.5, 1
///       wavelengths 0.4, 0.6, ... 2 micron, + radio
///       zenith distances 15, 45, 75 degrees
///
///     The accuracy with respect to raytracing through a model
///     atmosphere was as follows:
///  ```
///                            worst         RMS
///
///       optical/IR           62 mas       8 mas
///       radio               319 mas      49 mas
///  ```
///     For this particular set of conditions:
///  ```
///       lapse rate 0.0065 K/meter
///       latitude 50 degrees
///       sea level
///       pressure 1005 mb
///       temperature 280.15 K
///       humidity 80%
///       wavelength 5740 Angstroms
///  ```
///     the results were as follows:
///  ```
///       ZD       raytrace     iauRefco   Saastamoinen
///
///       10         10.27        10.27        10.27
///       20         21.19        21.20        21.19
///       30         33.61        33.61        33.60
///       40         48.82        48.83        48.81
///       45         58.16        58.18        58.16
///       50         69.28        69.30        69.27
///       55         82.97        82.99        82.95
///       60        100.51       100.54       100.50
///       65        124.23       124.26       124.20
///       70        158.63       158.68       158.61
///       72        177.32       177.37       177.31
///       74        200.35       200.38       200.32
///       76        229.45       229.43       229.42
///       78        267.44       267.29       267.41
///       80        319.13       318.55       319.10
///
///      deg        arcsec       arcsec       arcsec
///  ```
///     The values for Saastamoinen's formula (which includes terms
///     up to tan^5) are taken from Hohenkerk and Sinclair (1985).
///
///  3) A wl value in the range 0-100 selects the optical/IR case and is
///     wavelength in micrometers.  Any value outside this range selects
///     the radio case.
///
///  4) Outlandish input parameters are silently limited to
///     mathematically safe values.  Zero pressure is permissible, and
///     causes zeroes to be returned.
///
///  5) The algorithm draws on several sources, as follows:
///
///     a) The formula for the saturation vapour pressure of water as
///        a function of temperature and temperature is taken from
///        Equations (A4.5-A4.7) of Gill (1982).
///
///     b) The formula for the water vapour pressure, given the
///        saturation pressure and the relative humidity, is from
///        Crane (1976), Equation (2.5.5).
///
///     c) The refractivity of air is a function of temperature,
///        total pressure, water-vapour pressure and, in the case
///        of optical/IR, wavelength.  The formulae for the two cases are
///        developed from Hohenkerk & Sinclair (1985) and Rueger (2002).
///        The IAG (1999) optical refractivity for dry air is used.
///
///     d) The formula for beta, the ratio of the scale height of the
///        atmosphere to the geocentric distance of the observer, is
///        an adaption of Equation (9) from Stone (1996).  The
///        adaptations, arrived at empirically, consist of (i) a small
///        adjustment to the coefficient and (ii) a humidity term for the
///        radio case only.
///
///     e) The formulae for the refraction constants as a function of
///        n-1 and beta are from Green (1987), Equation (4.31).
///
///  References:
///
///     Crane, R.K., Meeks, M.L. (ed), "Refraction Effects in the Neutral
///     Atmosphere", Methods of Experimental Physics: Astrophysics 12B,
///     Academic Press, 1976.
///
///     Gill, Adrian E., "Atmosphere-Ocean Dynamics", Academic Press,
///     1982.
///
///     Green, R.M., "Spherical Astronomy", Cambridge University Press,
///     1987.
///
///     Hohenkerk, C.Y., & Sinclair, A.T., NAO Technical Note No. 63,
///     1985.
///
///     IAG Resolutions adopted at the XXIIth General Assembly in
///     Birmingham, 1999, Resolution 3.
///
///     Rueger, J.M., "Refractive Index Formulae for Electronic Distance
///     Measurement with Radio and Millimetre Waves", in Unisurv Report
///     S-68, School of Surveying and Spatial Information Systems,
///     University of New South Wales, Sydney, Australia, 2002.
///
///     Stone, Ronald C., P.A.S.P. 108, 1051-1058, 1996.
pub fn refco(phpa: f64, tc: f64, rh: f64, wl: f64) -> (f64, f64) {
    let optic: bool;
    let p: f64;
    let t: f64;
    let r: f64;
    let w: f64;
    let ps: f64;
    let pw: f64;
    let tk: f64;
    let wlsq: f64;
    let gamma: f64;
    let mut beta: f64;

    // Decide whether optical/IR or radio case: switch at 100 microns.
    optic = wl <= 100.0;

    // Restrict parameters to safe values.
    t = tc.max(-150.0).min(200.0);
    p = phpa.max(0.0).min(10000.0);
    r = rh.max(0.0).min(1.0);
    w = wl.max(0.1).min(1e6);

    // Water vapour pressure at the observer.
    if p > 0.0 {
        ps = 10.0_f64.powf((0.7859 + 0.03477 * t) / (1.0 + 0.00412 * t))
            * (1.0 + p * (4.5e-6 + 6e-10 * t * t));
        pw = r * ps / (1.0 - (1.0 - r) * ps / p);
    } else {
        pw = 0.0;
    }

    // Refractive index minus 1 at the observer.
    tk = t + 273.15;
    if optic {
        wlsq = w * w;
        gamma = ((77.53484e-6 + (4.39108e-7 + 3.666e-9 / wlsq) / wlsq) * p - 11.2684e-6 * pw) / tk;
    } else {
        gamma = (77.6890e-6 * p - (6.3938e-6 - 0.375463 / tk) * pw) / tk;
    }

    // Formula for beta from Stone, with empirical adjustments.
    beta = 4.4474e-6 * tk;
    if !optic {
        beta -= 0.0074 * pw * beta;
    }

    // Refraction constants from Green.
    let refa = gamma * (1.0 - beta);
    let refb = -gamma * (beta - gamma / 2.0);

    (refa, refb)
}
