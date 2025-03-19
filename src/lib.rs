//! # Sofa
//! This project is a basic astronomy calculation library implemented purely in Rust, based on the official [Standards of Fundamental Astronomy (SOFA)](http://iausofa.org) by the International Astronomical Union.
//! 
//! ## License
//! sofa-rs follows the MIT license.
//! 
//! In addition to the MIT license, any use of this module must also comply with the SOFA license and terms of use, which are detailed in the license file.
//! 
//! In particular (but not limited to), any published work or commercial product that includes results obtained using sofa-rs should acknowledge the use of algorithms provided by the SOFA ANSIC source code to obtain these results. 
pub mod astro;
pub mod cal;
pub mod consts;
pub mod coords;
pub mod eph;
pub mod erst;
pub mod fundargs;
pub mod gnomic;
pub mod horeq;
pub mod pnp;
pub mod star;
pub mod ts;
pub mod vm;