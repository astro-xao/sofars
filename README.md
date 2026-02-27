# Sofars

[![Build Status](https://github.com/astro-xao/sofars/actions/workflows/release-plz.yml/badge.svg)](https://github.com/astro-xao/sofars/actions)
[![Crates.io](https://img.shields.io/crates/v/sofars.svg)](https://crates.io/crates/sofars)
[![Downloads](https://img.shields.io/crates/d/sofars.svg)](https://crates.io/crates/sofars)
[![Documentation](https://docs.rs/sofars/badge.svg)](https://docs.rs/sofars)
[![Rust](https://img.shields.io/badge/rust-1.85.0%2B-blue.svg?maxAge=3600)](https://github.com/astro-xao/sofars)

[中文](README-zh.md)

`sofars` is a pure Rust implementation of the International Astronomical Union's (IAU) official [Standards of Fundamental Astronomy (SOFA)](http://iausofa.org). It provides a comprehensive, efficient, and industry-standard set of astronomical calculation tools for high-precision time scale transformations, coordinate system conversions, and fundamental astronomical modeling.

## Key Features

- **Standard Compliance**: Strictly follows IAU 2000/2006 models, ensuring numerical consistency with the original SOFA C library.
- **Pure Rust**: No C compiler required. Enjoy Rust's memory safety, concurrency, and performance.
- **Comprehensive Coverage**:
  - **Time Scales (`ts`)**: Precise transformations between UTC, TAI, TT, TDB, TCG, TCB, and more.
  - **Precession, Nutation & Polar Motion (`pnp`)**: Full implementation of IAU precession and nutation series.
  - **Fundamental Astrometry (`astro`)**: Core algorithms for apparent star positions, refraction, etc.
  - **Coordinate Systems (`coords`)**: Conversions between geocentric, heliocentric, and spherical systems.
  - **Vector & Matrix Operations (`vm`)**: 3D vector and rotation matrix utilities optimized for astronomy.
  - **Calendars (`cal`)**: Transformations between Julian Dates and Gregorian calendar dates.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
sofars = "0.6.0"
```

## Quick Start

The following example shows how to calculate the Earth Rotation Angle (ERA) for a given Julian Date:

```rust
use sofars::consts::DJ00;
use sofars::ts::era00;

fn main() {
    // 2000 January 1, 12:00 (TT)
    let tt1 = DJ00;
    let tt2 = 0.5;

    let era = era00(tt1, tt2);
    println!("Earth Rotation Angle: {} radians", era);
}
```

## Documentation

For detailed API documentation and function lists, please visit [docs.rs/sofars](https://docs.rs/sofars).

## License & SOFA Terms of Use

This project is licensed under the **MIT** License.

**Important Note**: Since the core algorithms are derived from the IAU SOFA source code, any use of this project must also comply with the SOFA license terms.
- Any published work or commercial product that includes results obtained using `sofars` should acknowledge the use of algorithms provided by the **IAU SOFA Board**.
- Please refer to the `LICENSE` file for full details.
