use sofars::astro::*;
use sofars::consts::*;
use sofars::eph::epv00;
use sofars::pnp::{bpn2xy, pnm00a, s06};
use sofars::ts;
use sofars::vm;

fn reprd(s: &str, ra: f64, dc: f64) {
    let mut pm: char;
    let mut i: [i32; 4];

    print!("{:25}", s);
    (pm, i) = vm::a2tf(7, ra);
    if pm == '+' {
        pm = ' ';
    }
    print!("{}{:02} {:02} {:02}.{:07} ", pm, i[0], i[1], i[2], i[3]);
    (pm, i) = vm::a2af(6, dc);
    println!("{}{:02} {:02} {:02}.{:06}", pm, i[0], i[1], i[2], i[3]);
}

fn main() {
    // Site longitude, latitude (radians) and height above the geoid (m).
    let elong = vm::af2a('-', 5, 41, 54.2).unwrap();
    let phi = vm::af2a('-', 15, 57, 42.8).unwrap();
    let hm = 625.0;

    // Ambient pressure (HPa), temperature (C) and rel. humidity (frac).
    let phpa = 952.0;
    let tc = 18.5;
    let rh = 0.83;

    // Effective color (microns).
    let wl = 0.55;

    // UTC date
    let (utc1, utc2) = ts::dtf2d("UTC", 2013, 4, 2, 23, 15, 43.55).unwrap();

    // TT date
    let (tai1, tai2) = ts::utctai(utc1, utc2).unwrap();
    let (tt1, tt2) = ts::taitt(tai1, tai2).unwrap();

    // EOPs: polar motion in radians, UT1-UTC in seconds.
    let xp = 50.995e-3 * DAS2R;
    let yp = 376.723e-3 * DAS2R;
    let dut1 = 155.0675e-3;

    // Corrections to IAU 2000A CIP (radians).
    let dx = 0.269e-3 * DAS2R;
    let dy = -0.274e-3 * DAS2R;

    // Star ICRS RA,Dec (radians).
    let rc = vm::tf2a(' ', 14, 34, 16.81183).unwrap();
    let dc = vm::af2a('-', 12, 31, 10.3965).unwrap();

    // Annual proper motion: RA/Dec derivatives, epoch J2000.0.
    let pr = (-354.45e-3 * DAS2R).atan2(dc.cos());
    let pd = 595.35e-3 * DAS2R;

    // Parallax (arcsec) and recession speed (km/s).
    let px = 164.99e-3;
    let rv = 0.0;

    println!("--- Sofars Astrometry Comprehensive Example ---");
    reprd("ICRS, epoch J2000.0:", rc, dc);

    // 1. ICRS catalog entry to CIRS via astrometric place
    let (rca, dca) = atcc13(rc, dc, pr, pd, px, rv, tt1, tt2);
    reprd("Catalog -> Astrometric:", rca, dca);
    let (ri_ind, di_ind, _eo_ind) = atci13(rca, dca, 0.0, 0.0, 0.0, 0.0, tt1, tt2);
    reprd("Astrometric -> CIRS:", ri_ind, di_ind);

    // 1. ICRS catalog directly to CIRS
    let (ri, di, eo) = atci13(rc, dc, pr, pd, px, rv, tt1, tt2);
    reprd("Catalog -> CIRS (Direct):", ri, di);

    // 2. Geocentric apparent place via the equation of the origins
    let ra_app = vm::anp(ri - eo);
    let da_app = di;
    reprd("Geocentric Apparent:", ra_app, da_app);

    // 3. CIRS to topocentric (zero pressure)
    let (_aot, _zot, _hot, dot_topo, rot_topo) = atio13(
        ri, di, utc1, utc2, dut1, elong, phi, hm, xp, yp, 0.0, 0.0, 0.0, 0.0,
    )
    .unwrap();
    reprd("CIRS -> Topocentric:", rot_topo, dot_topo);

    // 4. CIRS to observed
    let (_aob, _zob, _hob, dob, rob) = atio13(
        ri, di, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, wl,
    )
    .unwrap();
    reprd("CIRS -> Observed:", rob, dob);

    // 5. ICRS to observed in a single call
    let (_aob2, _zob2, _hob2, dob2, rob2, _eo2) = atco13(
        rc, dc, pr, pd, px, rv, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, wl,
    )
    .unwrap();
    reprd("ICRS -> Observed (Single):", rob2, dob2);

    // 6. ICRS to CIRS using simulated JPL DE405 for Earth ephemeris
    let (pvh, mut pvb) = epv00(tt1, tt2).unwrap();
    // Simulate JPL DE405 values as per lib.rs example
    pvb[0][0] = -0.9741704366519668;
    pvb[0][1] = -0.2115201000882231;
    pvb[0][2] = -0.0917583114068277;
    pvb[1][0] = 0.0036436589347388;
    pvb[1][1] = -0.0154287318503146;
    pvb[1][2] = -0.0066892203821059;

    let r_mat = pnm00a(tt1, tt2);
    let (mut x, mut y) = bpn2xy(&r_mat);
    x += dx;
    y += dy;
    let s_cio = s06(tt1, tt2, x, y);

    let mut astrom = IauAstrom::default();
    apci(tt1, tt2, &pvb, &pvh[0], x, y, s_cio, &mut astrom);
    let (ri_jpl, di_jpl) = atciq(rc, dc, pr, pd, px, rv, &mut astrom);
    reprd("ICRS -> CIRS (JPL, IERS):", ri_jpl, di_jpl);

    // 7. Same but including light deflection by Jupiter and Saturn
    let bodies = [
        IauLdBody::new(
            0.00028574,
            3e-10,
            [
                // Saturn
                [
                    -7.8101442680818964,
                    -5.6095668114887358,
                    -1.9807981923749924,
                ],
                [0.0030723248971152, -0.0040699547707598, -0.0018133584165345],
            ],
        ),
        IauLdBody::new(
            0.00095435,
            3e-9,
            [
                // Jupiter
                [0.7380987962351833, 4.6365869247538951, 1.9693136030111202],
                [-0.0075581692172088, 0.0012691372216750, 0.0007279990012801],
            ],
        ),
        IauLdBody::new(
            1.0,
            6e-6,
            [
                // Sun
                [
                    -0.0007121743770509,
                    -0.0023047830339257,
                    -0.0010586596574639,
                ],
                [0.0000062923521264, -0.0000003308883872, -0.0000002964866231],
            ],
        ),
    ];
    let (ri_planets, di_planets) = atciqn(rc, dc, pr, pd, px, rv, &mut astrom, 3, &bodies);
    reprd("ICRS -> CIRS (+ Planets):", ri_planets, di_planets);

    // 8. The reverse, to check agreement with Astrometric Place
    let (rca_rev, dca_rev) = aticqn(ri_planets, di_planets, &mut astrom, 3, &bodies);
    reprd("CIRS -> Astrometric (Rev):", rca_rev, dca_rev);
}
