// Star-independent astrometry parameters
pub struct IauAstrom {
    pub pmt: f64,           // PM time interval (SSB, Julian years)
    pub eb: [f64; 3],       // SSB to observer (vector, au)
    pub eh: [f64; 3],       // Sun to observer (unit vector)
    pub em: f64,            // distance from Sun to observer (au)
    pub v: [f64; 3],        // barycentric observer velocity (vector, c)
    pub bm1: f64,           // sqrt(1-|v|^2): reciprocal of Lorenz factor
    pub bpn: [[f64; 3]; 3], // bias-precession-nutation matrix
    pub along: f64,         // longitude + s' + dERA(DUT) (radians)
    pub phi: f64,           // geodetic latitude (radians)
    pub xpl: f64,           // polar motion xp wrt local meridian (radians)
    pub ypl: f64,           // polar motion yp wrt local meridian (radians)
    pub sphi: f64,          // sine of geodetic latitude
    pub cphi: f64,          // cosine of geodetic latitude
    pub diurab: f64,        // magnitude of diurnal aberration vector
    pub eral: f64,          // "local" Earth rotation angle (radians)
    pub refa: f64,          // refraction constant A (radians)
    pub refb: f64,          // refraction constant B (radians)
}

impl Default for IauAstrom {
    fn default() -> Self {
        IauAstrom {
            pmt: 0.0,
            eb: [0.0; 3],
            eh: [0.0; 3],
            em: 0.0,
            v: [0.0; 3],
            bm1: 0.0,
            bpn: [[0.0; 3]; 3],
            along: 0.0,
            phi: 0.0,
            xpl: 0.0,
            ypl: 0.0,
            sphi: 0.0,
            cphi: 0.0,
            diurab: 0.0,
            eral: 0.0,
            refa: 0.0,
            refb: 0.0,
        }
    }
}

// Body parameters for light deflection
pub struct IauLdBody {
    pub bm: f64,           // mass of the body (solar masses)
    pub dl: f64,           // deflection limiter (radians^2/2)
    pub pv: [[f64; 3]; 2], // barycentric PV of the body (au, au/day)
}

mod ab;
pub use ab::*;

mod apcg;
pub use apcg::*;

mod apci;
pub use apci::*;

mod apci13;
pub use apci13::*;

mod apco;
pub use apco::*;

mod apco13;
pub use apco13::*;

mod atco13;
pub use atco13::*;

mod apcs;
pub use apcs::*;

mod atciq;
pub use atciq::*;

mod atioq;
pub use atioq::*;

mod pmpx;
pub use pmpx::*;

mod pvtob;
pub use pvtob::*;

mod refco;
pub use refco::*;

mod ld;
pub use ld::*;

mod ldsun;
pub use ldsun::*;