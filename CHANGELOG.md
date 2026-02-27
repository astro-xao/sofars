# Changelog

All notable changes to this project will be documented in this file.

## [unreleased]

## [0.6.1](https://github.com/astro-xao/sofars/compare/v0.6.0...v0.6.1) - 2026-02-27

### Added

- *(example)* add comprehensive astrometry transformation guide

### Other

- *(docs)* update READMEs with version metadata and formatting
- *(bench)* implement automated benchmarks for core astronomical modules

## [0.6.0](https://github.com/astro-xao/sofars/compare/v0.5.1...v0.6.0) - 2026-02-27

### Added

- *(projection)* implement gnomonic projection functions
- *(coords)* implement ecliptic, galactic, and geodetic transformations
- *(vm)* add pv-vector and rotation matrix utilities

### Other

- *(erst)* add era00 verification
- *(star)* migrate catalog conversions to modular structure
- *(fundargs)* migrate to modular directory structure
- standardize and professionalize README files in English and Chinese
- add missing documentation for pnp routines and cross-language links in READMEs
- correct project name in README files

## [0.5.1](https://github.com/astro-xao/sofars/compare/v0.5.0...v0.5.1) - 2026-02-26

### Other

- enhance documentation for various functions in pnp module

## [0.5.0](https://github.com/astro-xao/sofars/compare/v0.4.3...v0.5.0) - 2026-02-25

### Added

- Implement comprehensive IAU SOFA pnp routines (64 total)
- Add complete unit tests for all pnp routines against SOFA C canonical results
- Register and export CIP/CEO series routines (xys00a, xys00b, xys06a)

### Changed

- refactor(pnp): change pnp API to return arrays instead of using mutable reference parameters
- refactor(astro): update apci, apco, and pvtob to adapt to new pnp API
- test(erst/vm): refactor unit tests to use standardized accuracy verification helpers

### Fixed

- fix(fundargs): implement mean longitude of Neptune calculation in fane03

## [0.4.5](https://github.com/astro-xao/sofars/compare/v0.4.4...v0.4.5) - 2026-02-25

### Added

- implement moon98 and plan94 function
- add time scale transformations for TAI, UTC, TDB, TCG, and TT

### Other

- reorder import statements for consistency
- simplify match statements for time conversions

## [0.4.4](https://github.com/astro-xao/sofars/compare/v0.4.3...v0.4.4) - 2026-02-25

### Other

- update build status badge workflow in README files

## [0.4.3](https://github.com/astro-xao/sofars/compare/v0.4.2...v0.4.3) - 2026-02-25

### Added

- add release-plz workflow for automated releases

## [0.4.2](https://github.com/astro-xao/sofars/compare/v0.4.1...v0.4.2) - 2026-02-25

### Added

- implement multiple astro functions and sync changelog

## [0.4.1](https://github.com/astro-xao/sofars/compare/v0.4.0...v0.4.1) - 2026-02-24

### Added

- docs: update README files with build and version badges

## [0.4.0](https://github.com/astro-xao/sofars/compare/v0.3.3...v0.4.0) - 2026-02-24

### Added

- feat: add atoc13, atoi13, and pmsafe functions with tests
- feat: implement pmsafe and starpm functions for star motion
- feat: add atoc13, atoi13, and atoiq functions for astrometry
- feat: add pvu, sepp, and seps functions for vector operations
- feat: add validation functions for double and integer results
- feat: implement jdcalf function for Julian Date conversion
- feat: add epj2jd function for Julian Epoch to Julian Date conversion
- feat: implement epj function for Julian Epoch conversion
- feat: add epb2jd function for Besselian Epoch conversion

### Fixed

- fix: rename variable x to X for consistency
- fix: correct copyright holder in LICENSE file

### Other

- test: add unit tests for calendar conversion functions
- style: rename unused variable w to _w for clarity
- refactor: remove unused epoch conversion functions
- docs: update code comments for better clarity

## [0.3.3](https://github.com/astro-xao/sofars/compare/v0.3.2...v0.3.3) - 2025-03-27

### Added

- feat: fmt code

## [0.3.2]
(https://github.com/astro-xao/sofars/compare/v0.3.1...v0.3.2) - 2025-03-27

### Added

- add ut1tt function for transforming UT1 to TT

## [0.3.1](https://github.com/astro-xao/sofars/compare/v0.3.0...v0.3.1) - 2025-03-27

### Added

- add d2dtf function for formatting 2-part Julian Date with leap second handling

### Other

- add unit tests for d2dtf

## [0.3.0](https://github.com/astro-xao/sofars/compare/v0.2.0...v0.3.0) - 2025-03-25

### Added

- implement Greenwich apparent sidereal time functions for IAU 2000A, 2000B, and 1994
- implement Greenwich mean sidereal time functions for IAU 2000, 2006, and 1982
- add IAU 1994 equation of the equinoxes and update related functions
- add nutation function for IAU 1980 and update module exports

### Other

- add tests for Greenwich mean and apparent sidereal time functions
- enhance documentation for Greenwich apparent sidereal time functions in gst06 and gst06a
- improve documentation formatting in obl80.rs
- simplify variable declarations in pvtob function

## [0.2.0](https://github.com/astro-xao/sofars/compare/v0.1.7...v0.2.0) - 2025-03-24

### Added

- reorganize erst module and with some implementations
- add vm impl: ppp, ppsp, pv2s, s2pv
- remove horeq(module)
- add nut00b impl
- add pvstar, starpv
- add hd2ae, hd2pa
- add ae2hd

### Other

- update example

## [0.1.7](https://github.com/astro-xao/sofars/compare/v0.1.6...v0.1.7) - 2025-03-24

### Added

- add aticqn
- add aticq, atic13
- add atciqz
- add atciqn, ldn
- add pmp, ppp, ppsp
- add aper and aper13
- add apcs13
- add apcg13

### Other

- update ldn documentation
- update apcs13 documentation
- update apcs13 documentation
- update atciqn documentation

## [0.1.6](https://github.com/astro-xao/sofars/compare/v0.1.5...v0.1.6) - 2025-03-21

### Fixed

- docs format
- atccq docs format
- atcc13 docs format
- apcs docs format
- apco13 docs format
- apci13 docs format
- apci docs format
- apcg docs format

## [0.1.5](https://github.com/astro-xao/sofars/compare/v0.1.4...v0.1.5) - 2025-03-21

### Added

- update pnp tests
- add new impls
- add bp00
- add bi00

### Other

- add example
- update

## [0.1.4](https://github.com/astro-xao/sofars/compare/v0.1.3...v0.1.4) - 2025-03-21

### Added

- release 0.1.4
- update docs ref link to latest

## [0.1.3](https://github.com/astro-xao/sofars/compare/v0.1.2...v0.1.3) - 2025-03-20

### Added

- manually update CHANGELOG

## [0.1.2](https://github.com/astro-xao/sofars/compare/v0.1.1...v0.1.2) - 2025-03-20

### Added

- manually update CHANGELOG

## [0.1.1] - 2025-03-20

### 🚀 Features

- Add release CI
- Add CHANGELOG.md
- Update CHANGELOG.md
- Add test for ld, ldsun

## [0.1.0] - 2025-03-19

### 🚀 Features

- Initial release
<!-- generated by git-cliff -->
