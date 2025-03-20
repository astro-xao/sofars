//! Constants
#![allow(dead_code)]
/// Pi
pub const DPI: f64 = 3.141592653589793238462643;

/// 2Pi
pub const D2PI: f64 = 6.283185307179586476925287;

/// Radians to degrees
pub const DR2D: f64 = 57.29577951308232087679815;

/// Degrees to radians
pub const DD2R: f64 = 1.745329251994329576923691e-2;

/// Radians to arcseconds
pub const DR2AS: f64 = 206264.8062470963551564734;

/// Arcseconds to radians
pub const DAS2R: f64 = 4.848136811095359935899141e-6;

/// Seconds of time to radians
pub const DS2R: f64 = 7.272205216643039903848712e-5;

/// Arcseconds in a full circle
pub const TURNAS: f64 = 1296000.0;

/// Milliarcseconds to radians
pub const DMAS2R: f64 = DAS2R / 1e3;

/// Length of tropical year B1900 (days)
pub const DTY: f64 = 365.242198781;

/// Seconds per day
pub const DAYSEC: f64 = 86400.0;

/// Days per Julian year
pub const DJY: f64 = 365.25;

/// Days per Julian century
pub const DJC: f64 = 36525.0;

/// Days per Julian millennium
pub const DJM: f64 = 365250.0;

/// Reference epoch (J2000.0), Julian Date
pub const DJ00: f64 = 2451545.0;

/// Julian Date of Modified Julian Date zero
pub const DJM0: f64 = 2400000.5;

/// Reference epoch (J2000.0), Modified Julian Date
pub const DJM00: f64 = 51544.5;

/// 1977 Jan 1.0 as MJD
pub const DJM77: f64 = 43144.0;

/// TT minus TAI (s)
pub const TTMTAI: f64 = 32.184;

/// Astronomical unit (m, IAU 2012)
pub const DAU: f64 = 149597870.7e3;

/// Speed of light (m/s)
pub const CMPS: f64 = 299792458.0;

/// Light time for 1 au (s)
pub const AULT: f64 = DAU / CMPS;

/// Speed of light (au per day)
pub const DC: f64 = DAYSEC / AULT;

/// L_G = 1 - d(TT)/d(TCG)
pub const ELG: f64 = 6.969290134e-10;

/// L_B = 1 - d(TDB)/d(TCB), and TDB (s) at TAI 1977/1/1.0
pub const ELB: f64 = 1.550519768e-8;
pub const TDB0: f64 = -6.55e-5;

/// Schwarzschild radius of the Sun (au)
pub const SRS: f64 = 1.97412574336e-8;

/// Reference ellipsoids
pub const WGS84: i32 = 1;
pub const GRS80: i32 = 2;
pub const WGS72: i32 = 3;

/// Earliest year allowed (4800BC)
pub const IYMIN: i32 = -4799;

/// Month lengths in days
pub const MTAB: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// J2000.0-B1900.0 (2415019.81352) in days
pub const D1900: f64 = 36524.68648;
