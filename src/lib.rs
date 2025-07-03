//! # Sofa
//! This project is a basic astronomy calculation library implemented purely in Rust, based on the official [Standards of Fundamental Astronomy (SOFA)](http://iausofa.org) by the International Astronomical Union.
//!
//! Refer to the [documentation](https://docs.rs/sofars/) for detail.
//! ## License
//! sofa-rs follows the MIT license.
//!
//! In addition to the MIT license, any use of this module must also comply with the SOFA license and terms of use, which are detailed in the license file.
//!
//! In particular (but not limited to), any published work or commercial product that includes results obtained using sofa-rs should acknowledge the use of algorithms provided by the SOFA ANSIC source code to obtain these results.
//! ## Example
//! ```
//! use sofars::astro::*;
//! use sofars::consts::*;
//! use sofars::eph::epv00;
//! use sofars::pnp::bpn2xy;
//! use sofars::pnp::pnm00a;
//! use sofars::pnp::s06;
//! use sofars::ts;
//! use sofars::vm;
//!
//! use std::thread;
//!
//! fn reprd(s: &str, ra: f64, dc: f64) {
//!     let mut pm: char;
//!     let mut i: [i32; 4] = [0; 4];
//!
//!     print!("{:25}", s);
//!     (pm, i) = vm::a2tf(7, ra);
//!     if pm == '+' {
//!         pm = ' ';
//!     }
//!     print!("{}{:02} {:02} {:02}.{:07} ", pm, i[0], i[1], i[2], i[3]);
//!     (pm, i) = vm::a2af(6, dc);
//!     println!("{}{:02} {:02} {:02}.{:06}", pm, i[0], i[1], i[2], i[3]);
//! }
//!
//! fn example() {
//!
//!     /* Site longitude, latitude (radians) and height above the geoid (m). */
//!     let elong = vm::af2a('-', 5, 41, 54.2).unwrap();
//!     let phi = vm::af2a('-', 15, 57, 42.8).unwrap();
//!     let hm = 625.0;
//!
//!     /* Ambient pressure (HPa), temperature (C) and rel. humidity (frac). */
//!     let phpa = 952.0;
//!     let tc = 18.5;
//!     let rh = 0.83;
//!
//!     /* Effective color (microns). */
//!     let wl = 0.55;
//!
//!     /* UTC date */
//!     let (utc1, utc2) = match ts::dtf2d("UTC", 2013, 4, 2, 23, 15, 43.55) {
//!         Ok(t) => t,
//!         Err(_) => return (),
//!     };
//!
//!     /* TT date */
//!     let (tai1, tai2) = match ts::utctai(utc1, utc2) {
//!         Ok(t) => t,
//!         Err(_) => return (),
//!     };
//!     let (tt1, tt2) = match ts::taitt(tai1, tai2) {
//!         Ok(t) => t,
//!         Err(_) => return (),
//!     };
//!
//!     /* EOPs: polar motion in radians, UT1-UTC in seconds. */
//!     let xp = 50.995e-3 * DAS2R;
//!     let yp = 376.723e-3 * DAS2R;
//!     let dut1 = 155.0675e-3;
//!
//!     /* Corrections to IAU 2000A CIP (radians). */
//!     let dx = 0.269e-3 * DAS2R;
//!     let dy = -0.274e-3 * DAS2R;
//!
//!     /* Star ICRS RA,Dec (radians). */
//!     let rc = match vm::tf2a(' ', 14, 34, 16.81183) {
//!         Ok(rc) => rc,
//!         Err(_) => return (),
//!     };
//!     let dc = match vm::af2a('-', 12, 31, 10.3965) {
//!         Ok(dc) => dc,
//!         Err(_) => return (),
//!     };
//!     reprd("ICRS, epoch J2000.0:", rc, dc);
//!     /* Annual proper motion: RA/Dec derivatives, epoch J2000.0. */
//!     let pr = (-354.45e-3 * DAS2R).atan2(dc.cos());
//!     let pd = 595.35e-3 * DAS2R;
//!   
//!     /* Parallax (arcsec) and recession speed (km/s). */
//!     let px = 164.99e-3;
//!     let rv = 0.0;
//!  
//!
//!     /* ICRS catalog to astrometric place... */
//!     let (rca, dca) = atcc13(rc, dc, pr, pd, px, rv, tt1, tt2);
//!     reprd("catalog -> astrometric:", rca, dca);
//!
//!     /* ...then to CIRS (geocentric observer) */
//!     let (ri, di, eo) = atci13(rca, dca, 0.0, 0.0, 0.0, 0.0, tt1, tt2);
//!     reprd("astrometric -> CIRS:", ri, di);
//!
//!     /* ICRS catalog directly to CIRS (geocentric observer). */
//!     let (ri, di, eo) = atci13(rc, dc, pr, pd, px, rv, tt1, tt2);
//!     reprd("catalog -> CIRS:", ri, di);
//!
//!     /* Apparent place */
//!     let ra = vm::anp(ri - eo);
//!     let da = di;
//!     reprd("geocentric apparent:", ra, da);
//!
//!     /* CIRS to topocentric. */
//!     let (aot, zot, hot, dot, rot) = atio13(ri, di, utc1, utc2, dut1, elong, phi, hm, xp, yp, 0.0, 0.0, 0.0, 0.0).unwrap();
//!     reprd("CIRS -> topocentric:", rot, dot);
//!
//!     /* CIRS to observed. */
//!     let (aob, zob, hob, dob, rob)= atio13(ri, di, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, wl).unwrap();
//!     reprd("CIRS -> observed:", rob, dob);
//!
//!     /* ICRS to observed. */
//!     let (aob, zob, hob, dob, rob, eo) = atco13(rc, dc, pr, pd, px, rv, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, wl).unwrap();
//!     reprd("ICRS -> observed:", rob, dob);
//!
//!     /* ICRS to CIRS using some user-supplied parameters. */
//!     /* SOFA heliocentric Earth ephemeris. */
//!     let (pvh, pvb) = &mut epv00(tt1, tt2).unwrap();
//!
//!     /* JPL DE405 barycentric Earth ephemeris. */
//!     pvb[0][0] = -0.9741704366519668;
//!     pvb[0][1] = -0.2115201000882231;
//!     pvb[0][2] = -0.0917583114068277;
//!     pvb[1][0] = 0.0036436589347388;
//!     pvb[1][1] = -0.0154287318503146;
//!     pvb[1][2] = -0.0066892203821059;
//!
//!     /* IAU 2000 CIP */
//!     let r = &mut [[0.0; 3]; 3];
//!     pnm00a(tt1, tt2, r);
//!     let (mut x, mut y) = bpn2xy(r);
//!
//!     /* Apply IERS corrections */
//!     x += dx;
//!     y += dy;
//!
//!     /* SOFA CIO locator. */
//!     let s = s06(tt1, tt2, x, y);
//!
//!     let astrom = &mut IauAstrom::default();
//!     /* Populate the context. */
//!     apci(tt1, tt2, &pvb, &pvh[0], x, y, s, astrom);
//!
//!     /* Carry out the transformation and report the results. */
//!     let (ri, di) = atciq(rc, dc, pr, pd, px, rv, astrom);
//!     reprd("ICRS -> CIRS (JPL, IERS):", ri, di);
//!     
//!     /* The same but with Saturn then Jupiter then Sun light deflection. */
//!     let mut b = [
//!         IauLdBody::new(0.00028574, 3e-10, [
//!             [-7.8101442680818964, -5.6095668114887358, -1.9807981923749924],
//!             [0.0030723248971152, -0.0040699547707598, -0.0018133584165345]
//!         ]),
//!         IauLdBody::new(0.00095435, 3e-9, [
//!             [0.7380987962351833, 4.6365869247538951, 1.9693136030111202],
//!             [-0.0075581692172088, 0.0012691372216750, 0.0007279990012801]
//!         ]),
//!         IauLdBody::new(1.0, 6e-6, [
//!             [-0.0007121743770509, -0.0023047830339257, -0.0010586596574639],
//!             [0.0000062923521264, -0.0000003308883872, -0.0000002964866231]
//!         ])
//!     ];
//!
//!     let (ri, di) = atciqn(rc, dc, pr, pd, px, rv, astrom, 3, &b);
//!     reprd("ICRS -> CIRS (+ planets):", ri, di);
//!
//!     /* CIRS to ICRS (astrometric). */
//!     let (rca, dca) = aticqn(ri, di, astrom, 3, &b);
//!     reprd("CIRS -> astrometric:", rca, dca);
//! }
//!
//! // Output:
//! //
//! // ICRS, epoch J2000.0:      14 34 16.8118300 -12 31 10.396500
//! // catalog -> astrometric:   14 34 16.4960283 -12 31 02.523786
//! // astrometric -> CIRS:      14 34 20.2370587 -12 34 36.381654
//! // catalog -> CIRS:          14 34 20.2370587 -12 34 36.381654
//! // geocentric apparent:      14 35 01.7725802 -12 34 36.381654
//! // CIRS -> topocentric:      14 34 20.2570287 -12 34 36.141207
//! // CIRS -> observed:         14 34 16.9649101 -12 34 44.643091
//! // ICRS -> observed:         14 34 16.9649106 -12 34 44.643094
//! // ICRS -> CIRS (JPL, IERS): 14 34 20.2370639 -12 34 36.381756
//! // ICRS -> CIRS (+ planets): 14 34 20.2370658 -12 34 36.381784
//! // CIRS -> astrometric:      14 34 16.4960283 -12 31 02.523786
//! //
//! fn main() {
//!     let stack_size = 2 * 1024 * 1024;
//!     let name = String::from("mycal thread");
//!
//!     let builder = thread::Builder::new().stack_size(stack_size).name(name);
//!
//!     let handler = builder.spawn(|| {
//!         example();
//!     }).unwrap();
//!
//!     handler.join().unwrap();
//! }
//! ```
//!
pub mod astro;
pub mod cal;
pub mod consts;
pub mod coords;
pub mod eph;
pub mod erst;
pub mod fundargs;
pub mod gnomic;
pub mod pnp;
pub mod star;
pub mod ts;
pub mod vm;
