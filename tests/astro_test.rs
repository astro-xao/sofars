use sofars::astro::{ab, apcg, apci, apci13, apco, apco13, apcs, atcc13, atci13, atciq, atco13, ld, ldsun, pvtob, IauAstrom};

#[test]
fn test_ab() {
    let pnat = [-0.76321968546737951, -0.60869453983060384, -0.21676408580639883];
    let v = [2.1044018893653786e-5, -8.9108923304429319e-5, -3.8633714797716569e-5];
    let s = 0.99980921395708788;
    let bm1 = 0.99999999506209258;

    let ppr = ab(&pnat, &v, s, bm1);

    assert!((ppr[0] - -0.7631631094219556269).abs() < 1e-12, "ab: ppr[0]");
    assert!((ppr[1] - -0.6087553082505590832).abs() < 1e-12, "ab: ppr[1]");
    assert!((ppr[2] - -0.2167926269368471279).abs() < 1e-12, "ab: ppr[2]");
}

#[test]
fn test_pvtob() {
    let elong = 2.0;
    let phi = 0.5;
    let hm = 3000.0;
    let xp = 1e-6;
    let yp = -0.5e-6;
    let sp = 1e-8;
    let theta = 5.0;
    let mut pv = [[0.0; 3]; 2];

    pvtob(elong, phi, hm, xp, yp, sp, theta, &mut pv);

    assert!((pv[0][0] - 4225081.367071159207).abs() < 1e-5, "pvtob: p(1)");
    assert!((pv[0][1] - 3681943.215856198144).abs() < 1e-5, "pvtob: p(2)");
    assert!((pv[0][2] - 3041149.399241260785).abs() < 1e-5, "pvtob: p(3)");
    assert!((pv[1][0] - -268.4915389365998787).abs() < 1e-9, "pvtob: v(1)");
    assert!((pv[1][1] - 308.0977983288903123).abs() < 1e-9, "pvtob: v(2)");
    assert!((pv[1][2] - 0.0).abs() < 1e-9, "pvtob: v(3)");
}

#[test]
fn test_apcg() {
    let date1 = 2456165.5;
    let date2 = 0.401182685;
    let ebpv = [
        [0.901310875, -0.417402664, -0.180982288],
        [0.00742727954, 0.0140507459, 0.00609045792],
    ];
    let ehp = [0.903358544, -0.415395237, -0.180084014];
    let mut astrom = IauAstrom::default();

    apcg(date1, date2, &ebpv, &ehp, &mut astrom);

    assert!((astrom.pmt - 12.65133794027378508).abs() < 1e-11, "apcg: pmt");
    assert!((astrom.eb[0] - 0.901310875).abs() < 1e-12, "apcg: eb(1)");
    assert!((astrom.eb[1] - -0.417402664).abs() < 1e-12, "apcg: eb(2)");
    assert!((astrom.eb[2] - -0.180982288).abs() < 1e-12, "apcg: eb(3)");
    assert!((astrom.eh[0] - 0.8940025429324143045).abs() < 1e-12, "apcg: eh(1)");
    assert!((astrom.eh[1] - -0.4110930268679817955).abs() < 1e-12, "apcg: eh(2)");
    assert!((astrom.eh[2] - -0.1782189004872870264).abs() < 1e-12, "apcg: eh(3)");
    assert!((astrom.em - 1.010465295811013146).abs() < 1e-12, "apcg: em");
    assert!((astrom.v[0] - 0.4289638913597693554e-4).abs() < 1e-16, "apcg: v(1)");
    assert!((astrom.v[1] - 0.8115034051581320575e-4).abs() < 1e-16, "apcg: v(2)");
    assert!((astrom.v[2] - 0.3517555136380563427e-4).abs() < 1e-16, "apcg: v(3)");
    assert!((astrom.bm1 - 0.9999999951686012981).abs() < 1e-12, "apcg: bm1");
    assert!((astrom.bpn[0][0] - 1.0).abs() < 1e-12, "apcg: bpn(1,1)");
    assert!((astrom.bpn[1][0] - 0.0).abs() < 1e-12, "apcg: bpn(2,1)");
    assert!((astrom.bpn[2][0] - 0.0).abs() < 1e-12, "apcg: bpn(3,1)");
    assert!((astrom.bpn[0][1] - 0.0).abs() < 1e-12, "apcg: bpn(1,2)");
    assert!((astrom.bpn[1][1] - 1.0).abs() < 1e-12, "apcg: bpn(2,2)");
    assert!((astrom.bpn[2][1] - 0.0).abs() < 1e-12, "apcg: bpn(3,2)");
    assert!((astrom.bpn[0][2] - 0.0).abs() < 1e-12, "apcg: bpn(1,3)");
    assert!((astrom.bpn[1][2] - 0.0).abs() < 1e-12, "apcg: bpn(2,3)");
    assert!((astrom.bpn[2][2] - 1.0).abs() < 1e-12, "apcg: bpn(3,3)");
}

#[test]
fn test_apci() {
    let date1 = 2456165.5;
    let date2 = 0.401182685;
    let ebpv = [
        [0.901310875, -0.417402664, -0.180982288],
        [0.00742727954, 0.0140507459, 0.00609045792],
    ];
    let ehp = [0.903358544, -0.415395237, -0.180084014];
    let x = 0.0013122272;
    let y = -2.92808623e-5;
    let s = 3.05749468e-8;
    let mut astrom = IauAstrom::default();

    apci(date1, date2, &ebpv, &ehp, x, y, s, &mut astrom);

    assert!((astrom.pmt - 12.65133794027378508).abs() < 1e-11, "apci: pmt");
    assert!((astrom.eb[0] - 0.901310875).abs() < 1e-12, "apci: eb(1)");
    assert!((astrom.eb[1] - -0.417402664).abs() < 1e-12, "apci: eb(2)");
    assert!((astrom.eb[2] - -0.180982288).abs() < 1e-12, "apci: eb(3)");
    assert!((astrom.eh[0] - 0.8940025429324143045).abs() < 1e-12, "apci: eh(1)");
    assert!((astrom.eh[1] - -0.4110930268679817955).abs() < 1e-12, "apci: eh(2)");
    assert!((astrom.eh[2] - -0.1782189004872870264).abs() < 1e-12, "apci: eh(3)");
    assert!((astrom.em - 1.010465295811013146).abs() < 1e-12, "apci: em");
    assert!((astrom.v[0] - 0.4289638913597693554e-4).abs() < 1e-16, "apci: v(1)");
    assert!((astrom.v[1] - 0.8115034051581320575e-4).abs() < 1e-16, "apci: v(2)");
    assert!((astrom.v[2] - 0.3517555136380563427e-4).abs() < 1e-16, "apci: v(3)");
    assert!((astrom.bm1 - 0.9999999951686012981).abs() < 1e-12, "apci: bm1");
    assert!((astrom.bpn[0][0] - 0.9999991390295159156).abs() < 1e-12, "apci: bpn(1,1)");
    assert!((astrom.bpn[1][0] - 0.4978650072505016932e-7).abs() < 1e-12, "apci: bpn(2,1)");
    assert!((astrom.bpn[2][0] - 0.1312227200000000000e-2).abs() < 1e-12, "apci: bpn(3,1)");
    assert!((astrom.bpn[0][1] - -0.1136336653771609630e-7).abs() < 1e-12, "apci: bpn(1,2)");
    assert!((astrom.bpn[1][1] - 0.9999999995713154868).abs() < 1e-12, "apci: bpn(2,2)");
    assert!((astrom.bpn[2][1] - -0.2928086230000000000e-4).abs() < 1e-12, "apci: bpn(3,2)");
    assert!((astrom.bpn[0][2] - -0.1312227200895260194e-2).abs() < 1e-12, "apci: bpn(1,3)");
    assert!((astrom.bpn[1][2] - 0.2928082217872315680e-4).abs() < 1e-12, "apci: bpn(2,3)");
    assert!((astrom.bpn[2][2] - 0.9999991386008323373).abs() < 1e-12, "apci: bpn(3,3)");
}

#[test]
fn test_apci13() {
    let date1 = 2456165.5;
    let date2 = 0.401182685;
    let mut astrom = IauAstrom::default();
    let mut eo = 0.0;

    apci13(date1, date2, &mut astrom, &mut eo);

    assert!((astrom.pmt - 12.65133794027378508).abs() < 1e-11, "apci13: pmt");
    assert!((astrom.eb[0] - 0.9013108747340644755).abs() < 1e-12, "apci13: eb(1)");
    assert!((astrom.eb[1] - -0.4174026640406119957).abs() < 1e-12, "apci13: eb(2)");
    assert!((astrom.eb[2] - -0.1809822877867817771).abs() < 1e-12, "apci13: eb(3)");
    assert!((astrom.eh[0] - 0.8940025429255499549).abs() < 1e-12, "apci13: eh(1)");
    assert!((astrom.eh[1] - -0.4110930268331896318).abs() < 1e-12, "apci13: eh(2)");
    assert!((astrom.eh[2] - -0.1782189006019749850).abs() < 1e-12, "apci13: eh(3)");
    assert!((astrom.em - 1.010465295964664178).abs() < 1e-12, "apci13: em");
    assert!((astrom.v[0] - 0.4289638912941341125e-4).abs() < 1e-16, "apci13: v(1)");
    assert!((astrom.v[1] - 0.8115034032405042132e-4).abs() < 1e-16, "apci13: v(2)");
    assert!((astrom.v[2] - 0.3517555135536470279e-4).abs() < 1e-16, "apci13: v(3)");
    assert!((astrom.bm1 - 0.9999999951686013142).abs() < 1e-12, "apci13: bm1");
    assert!((astrom.bpn[0][0] - 0.9999992060376761710).abs() < 1e-12, "apci13: bpn(1,1)");
    assert!((astrom.bpn[1][0] - 0.4124244860106037157e-7).abs() < 1e-12, "apci13: bpn(2,1)");
    assert!((astrom.bpn[2][0] - 0.1260128571051709670e-2).abs() < 1e-12, "apci13: bpn(3,1)");
    assert!((astrom.bpn[0][1] - -0.1282291987222130690e-7).abs() < 1e-12, "apci13: bpn(1,2)");
    assert!((astrom.bpn[1][1] - 0.9999999997456835325).abs() < 1e-12, "apci13: bpn(2,2)");
    assert!((astrom.bpn[2][1] - -0.2255288829420524935e-4).abs() < 1e-12, "apci13: bpn(3,2)");
    assert!((astrom.bpn[0][2] - -0.1260128571661374559e-2).abs() < 1e-12, "apci13: bpn(1,3)");
    assert!((astrom.bpn[1][2] - 0.2255285422953395494e-4).abs() < 1e-12, "apci13: bpn(2,3)");
    assert!((astrom.bpn[2][2] - 0.9999992057833604343).abs() < 1e-12, "apci13: bpn(3,3)");
    assert!((eo - -0.2900618712657375647e-2).abs() < 1e-12, "apci13: eo");
}

#[test]
fn test_apcs() {
    let astrom = &mut IauAstrom::default();
    let date1 = 2456384.5;
    let date2 = 0.970031644;

    let pv = [
        [-1836024.09, 1056607.72, -5998795.26],
        [-77.0361767, -133.310856, 0.0971855934],
    ];

    let ebpv = [
        [-0.974170438, -0.211520082, -0.0917583024],
        [0.00364365824, -0.0154287319, -0.00668922024],
    ];

    let ehp = [-0.973458265, -0.209215307, -0.0906996477];

    apcs(date1, date2, &pv, &ebpv, &ehp, astrom);

    assert!((astrom.pmt - 13.25248468622587269).abs() < 1e-11, "apcs: pmt");
    assert!((astrom.eb[0] - -0.9741827110629881886).abs() < 1e-12, "apcs: eb(1)");
    assert!((astrom.eb[1] - -0.2115130190136415986).abs() < 1e-12, "apcs: eb(2)");
    assert!((astrom.eb[2] - -0.09179840186954412099).abs() < 1e-12, "apcs: eb(3)");
    assert!((astrom.eh[0] - -0.9736425571689454706).abs() < 1e-12, "apcs: eh(1)");
    assert!((astrom.eh[1] - -0.2092452125850435930).abs() < 1e-12, "apcs: eh(2)");
    assert!((astrom.eh[2] - -0.09075578152248299218).abs() < 1e-12, "apcs: eh(3)");
    assert!((astrom.em - 0.9998233241709796859).abs() < 1e-12, "apcs: em");
    assert!((astrom.v[0] - 0.2078704993282685510e-4).abs() < 1e-16, "apcs: v(1)");
    assert!((astrom.v[1] - -0.8955360106989405683e-4).abs() < 1e-16, "apcs: v(2)");
    assert!((astrom.v[2] - -0.3863338994289409097e-4).abs() < 1e-16, "apcs: v(3)");
    assert!((astrom.bm1 - 0.9999999950277561237).abs() < 1e-12, "apcs: bm1");
    assert!((astrom.bpn[0][0] - 1.0).abs() < 1e-12, "apcs: bpn(1,1)");
    assert!((astrom.bpn[1][0] - 0.0).abs() < 1e-12, "apcs: bpn(2,1)");
    assert!((astrom.bpn[2][0] - 0.0).abs() < 1e-12, "apcs: bpn(3,1)");
    assert!((astrom.bpn[0][1] - 0.0).abs() < 1e-12, "apcs: bpn(1,2)");
    assert!((astrom.bpn[1][1] - 1.0).abs() < 1e-12, "apcs: bpn(2,2)");
    assert!((astrom.bpn[2][1] - 0.0).abs() < 1e-12, "apcs: bpn(3,2)");
    assert!((astrom.bpn[0][2] - 0.0).abs() < 1e-12, "apcs: bpn(1,3)");
    assert!((astrom.bpn[1][2] - 0.0).abs() < 1e-12, "apcs: bpn(2,3)");
    assert!((astrom.bpn[2][2] - 1.0).abs() < 1e-12, "apcs: bpn(3,3)");
}

#[test]
fn test_atcc13() {
    let rc = 2.71;
    let dc = 0.174;
    let pr = 1e-5;
    let pd = 5e-6;
    let px = 0.1;
    let rv = 55.0;
    let date1 = 2456165.5;
    let date2 = 0.401182685;

    let (ra, da) = atcc13(rc, dc, pr, pd, px, rv, date1, date2);

    assert!((ra - 2.710126504531372384).abs() < 1e-12, "atcc13: ra");
    assert!((da - 0.1740632537628350152).abs() < 1e-12, "atcc13: da");
}

#[test]
fn test_apco() {
    let astrom = &mut IauAstrom::default();
    let date1 = 2456384.5;
    let date2 = 0.970031644;
    let ebpv = [
        [-0.974170438, -0.211520082, -0.0917583024],
        [0.00364365824, -0.0154287319, -0.00668922024],
    ];
    let ehp = [-0.973458265, -0.209215307, -0.0906996477];
    let x = 0.0013122272;
    let y = -2.92808623e-5;
    let s = 3.05749468e-8;
    let theta = 3.14540971;
    let elong = -0.527800806;
    let phi = -1.2345856;
    let hm = 2738.0;
    let xp = 2.47230737e-7;
    let yp = 1.82640464e-6;
    let sp = -3.01974337e-11;
    let refa = 0.000201418779;
    let refb = -2.36140831e-7;

    apco(date1, date2, &ebpv, &ehp, x, y, s, theta, elong, phi, hm, xp, yp, sp, refa, refb, astrom);

    assert!((astrom.pmt - 13.25248468622587269).abs() < 1e-11, "apco: pmt");
    assert!((astrom.eb[0] - -0.9741827110630322720).abs() < 1e-12, "apco: eb(1)");
    assert!((astrom.eb[1] - -0.2115130190135344832).abs() < 1e-12, "apco: eb(2)");
    assert!((astrom.eb[2] - -0.09179840186949532298).abs() < 1e-12, "apco: eb(3)");
    assert!((astrom.eh[0] - -0.9736425571689739035).abs() < 1e-12, "apco: eh(1)");
    assert!((astrom.eh[1] - -0.2092452125849330936).abs() < 1e-12, "apco: eh(2)");
    assert!((astrom.eh[2] - -0.09075578152243272599).abs() < 1e-12, "apco: eh(3)");
    assert!((astrom.em - 0.9998233241709957653).abs() < 1e-12, "apco: em");
    assert!((astrom.v[0] - 0.2078704992916728762e-4).abs() < 1e-16, "apco: v(1)");
    assert!((astrom.v[1] - -0.8955360107151952319e-4).abs() < 1e-16, "apco: v(2)");
    assert!((astrom.v[2] - -0.3863338994288951082e-4).abs() < 1e-16, "apco: v(3)");
    assert!((astrom.bm1 - 0.9999999950277561236).abs() < 1e-12, "apco: bm1");
    assert!((astrom.bpn[0][0] - 0.9999991390295159156).abs() < 1e-12, "apco: bpn(1,1)");
    assert!((astrom.bpn[1][0] - 0.4978650072505016932e-7).abs() < 1e-12, "apco: bpn(2,1)");
    assert!((astrom.bpn[2][0] - 0.1312227200000000000e-2).abs() < 1e-12, "apco: bpn(3,1)");
    assert!((astrom.bpn[0][1] - -0.1136336653771609630e-7).abs() < 1e-12, "apco: bpn(1,2)");
    assert!((astrom.bpn[1][1] - 0.9999999995713154868).abs() < 1e-12, "apco: bpn(2,2)");
    assert!((astrom.bpn[2][1] - -0.2928086230000000000e-4).abs() < 1e-12, "apco: bpn(3,2)");
    assert!((astrom.bpn[0][2] - -0.1312227200895260194e-2).abs() < 1e-12, "apco: bpn(1,3)");
    assert!((astrom.bpn[1][2] - 0.2928082217872315680e-4).abs() < 1e-12, "apco: bpn(2,3)");
    assert!((astrom.bpn[2][2] - 0.9999991386008323373).abs() < 1e-12, "apco: bpn(3,3)");
    assert!((astrom.along - -0.5278008060295995734).abs() < 1e-12, "apco: along");
    assert!((astrom.xpl - 0.1133427418130752958e-5).abs() < 1e-17, "apco: xpl");
    assert!((astrom.ypl - 0.1453347595780646207e-5).abs() < 1e-17, "apco: ypl");
    assert!((astrom.sphi - -0.9440115679003211329).abs() < 1e-12, "apco: sphi");
    assert!((astrom.cphi - 0.3299123514971474711).abs() < 1e-12, "apco: cphi");
    assert!((astrom.diurab - 0.0).abs() < 1e-12, "apco: diurab");
    assert!((astrom.eral - 2.617608903970400427).abs() < 1e-12, "apco: eral");
    assert!((astrom.refa - 0.2014187790000000000e-3).abs() < 1e-15, "apco: refa");
    assert!((astrom.refb - -0.2361408310000000000e-6).abs() < 1e-18, "apco: refb");
}

#[test]
fn test_apco13() {
    let utc1 = 2456384.5;
    let utc2 = 0.969254051;
    let dut1 = 0.1550675;
    let elong = -0.527800806;
    let phi = -1.2345856;
    let hm = 2738.0;
    let xp = 2.47230737e-7;
    let yp = 1.82640464e-6;
    let phpa = 731.0;
    let tc = 12.8;
    let rh = 0.59;
    let wl = 0.55;

    let mut astrom = IauAstrom::default();
    let mut eo = 0.0;

    let j = apco13(
        utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, wl, &mut astrom, &mut eo,
    ).unwrap();

    assert!((astrom.pmt - 13.25248468622475727).abs() < 1e-11, "apco13: pmt");
    assert!((astrom.eb[0] - -0.9741827107320875162).abs() < 1e-12, "apco13: eb(1)");
    assert!((astrom.eb[1] - -0.2115130190489716682).abs() < 1e-12, "apco13: eb(2)");
    assert!((astrom.eb[2] - -0.09179840189496755339).abs() < 1e-12, "apco13: eb(3)");
    assert!((astrom.eh[0] - -0.9736425572586935247).abs() < 1e-12, "apco13: eh(1)");
    assert!((astrom.eh[1] - -0.2092452121603336166).abs() < 1e-12, "apco13: eh(2)");
    assert!((astrom.eh[2] - -0.09075578153885665295).abs() < 1e-12, "apco13: eh(3)");
    assert!((astrom.em - 0.9998233240913898141).abs() < 1e-12, "apco13: em");
    assert!((astrom.v[0] - 0.2078704994520489246e-4).abs() < 1e-16, "apco13: v(1)");
    assert!((astrom.v[1] - -0.8955360133238868938e-4).abs() < 1e-16, "apco13: v(2)");
    assert!((astrom.v[2] - -0.3863338993055887398e-4).abs() < 1e-16, "apco13: v(3)");
    assert!((astrom.bm1 - 0.9999999950277561004).abs() < 1e-12, "apco13: bm1");
    assert!((astrom.bpn[0][0] - 0.9999991390295147999).abs() < 1e-12, "apco13: bpn(1,1)");
    assert!((astrom.bpn[1][0] - 0.4978650075315529277e-7).abs() < 1e-12, "apco13: bpn(2,1)");
    assert!((astrom.bpn[2][0] - 0.001312227200850293372).abs() < 1e-12, "apco13: bpn(3,1)");
    assert!((astrom.bpn[0][1] - -0.1136336652812486604e-7).abs() < 1e-12, "apco13: bpn(1,2)");
    assert!((astrom.bpn[1][1] - 0.9999999995713154865).abs() < 1e-12, "apco13: bpn(2,2)");
    assert!((astrom.bpn[2][1] - -0.2928086230975367296e-4).abs() < 1e-12, "apco13: bpn(3,2)");
    assert!((astrom.bpn[0][2] - -0.001312227201745553566).abs() < 1e-12, "apco13: bpn(1,3)");
    assert!((astrom.bpn[1][2] - 0.2928082218847679162e-4).abs() < 1e-12, "apco13: bpn(2,3)");
    assert!((astrom.bpn[2][2] - 0.9999991386008312212).abs() < 1e-12, "apco13: bpn(3,3)");
    assert!((astrom.along - -0.5278008060295995733).abs() < 1e-12, "apco13: along");
    assert!((astrom.xpl - 0.1133427418130752958e-5).abs() < 1e-17, "apco13: xpl");
    assert!((astrom.ypl - 0.1453347595780646207e-5).abs() < 1e-17, "apco13: ypl");
    assert!((astrom.sphi - -0.9440115679003211329).abs() < 1e-12, "apco13: sphi");
    assert!((astrom.cphi - 0.3299123514971474711).abs() < 1e-12, "apco13: cphi");
    assert!((astrom.diurab - 0.0).abs() < 1e-12, "apco13: diurab");
    assert!((astrom.eral - 2.617608909189664000).abs() < 1e-12, "apco13: eral");
    assert!((astrom.refa - 0.2014187785940396921e-3).abs() < 1e-15, "apco13: refa");
    assert!((astrom.refb - -0.2361408314943696227e-6).abs() < 1e-18, "apco13: refb");
    assert!((eo - -0.003020548354802412839).abs() < 1e-14, "apco13: eo");
    assert_eq!(j, 0, "apco13: j");
}

#[test]
fn test_atci13() {
    let rc = 2.71;
    let dc = 0.174;
    let pr = 1e-5;
    let pd = 5e-6;
    let px = 0.1;
    let rv = 55.0;
    let date1 = 2456165.5;
    let date2 = 0.401182685;

    let (ri, di, eo) = atci13(rc, dc, pr, pd, px, rv, date1, date2);

    assert!((ri - 2.710121572968696744).abs() < 1e-12, "atci13: ri");
    assert!((di - 0.1729371367219539137).abs() < 1e-12, "atci13: di");
    assert!((eo - -0.002900618712657375647).abs() < 1e-14, "atci13: eo");
}

#[test]
fn test_atciq() {
    let date1 = 2456165.5;
    let date2 = 0.401182685;

    let astrom = &mut IauAstrom::default();
    let eo = &mut 0.0;

    apci13(date1, date2, astrom, eo);

    let rc = 2.71;
    let dc = 0.174;
    let pr = 1e-5;
    let pd = 5e-6;
    let px = 0.1;
    let rv = 55.0;

    let (ri, di) = atciq(rc, dc, pr, pd, px, rv, astrom);

    assert!((ri - 2.710121572968696744).abs() < 1e-12, "atciq: ri");
    assert!((di - 0.1729371367219539137).abs() < 1e-12, "atciq: di");
}

#[test]
fn test_atco13() {
    let rc = 2.71;
    let dc = 0.174;
    let pr = 1e-5;
    let pd = 5e-6;
    let px = 0.1;
    let rv = 55.0;
    let utc1 = 2456384.5;
    let utc2 = 0.969254051;
    let dut1 = 0.1550675;
    let elong = -0.527800806;
    let phi = -1.2345856;
    let hm = 2738.0;
    let xp = 2.47230737e-7;
    let yp = 1.82640464e-6;
    let phpa = 731.0;
    let tc = 12.8;
    let rh = 0.59;
    let wl = 0.55;

    let j = atco13(
        rc, dc, pr, pd, px, rv, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, wl);
    
    match j {
        Ok((aob, zob, hob, dob, rob, eo)) => {
            assert!((aob - 0.9251774485485515207e-1).abs() < 1e-12, "atco13: aob");
            assert!((zob - 1.407661405256499357).abs() < 1e-12, "atco13: zob");
            assert!((hob - -0.9265154431529724692e-1).abs() < 1e-12, "atco13: hob");
            assert!((dob - 0.1716626560072526200).abs() < 1e-12, "atco13: dob");
            assert!((rob - 2.710260453504961012).abs() < 1e-12, "atco13: rob");
            assert!((eo - -0.003020548354802412839).abs() < 1e-14, "atco13: eo");
        }
        Err(j) => assert_eq!(j, 0, "atco13: j"),
    }
}

#[test]
fn test_ld() {
    let bm = 0.00028574;
    let p = [-0.763276255, -0.608633767, -0.216735543];
    let q = [-0.763276255, -0.608633767, -0.216735543];
    let e = [0.76700421, 0.605629598, 0.211937094];
    let em = 8.91276983;
    let dlim = 3e-10;
    let p1 = ld(bm, p, q, e, em, dlim);

    assert!((p1[0] - -0.7632762548968159627).abs() < 1e-12, "ld: p1[0]");
    assert!((p1[1] - -0.6086337670823762701).abs() < 1e-12, "ld: p1[1]");
    assert!((p1[2] - -0.2167355431320546947).abs() < 1e-12, "ld: p1[2]");
}

#[test]
fn test_ldsun() {
    let p = [-0.763276255, -0.608633767, -0.216735543];
    let e = [-0.973644023, -0.20925523, -0.0907169552];
    let em = 0.999809214;

    let p1 = ldsun(p, e, em);

    assert!((p1[0] - -0.7632762580731413169).abs() < 1e-12, "ldsun: p1[0]");
    assert!((p1[1] - -0.6086337635262647900).abs() < 1e-12, "ldsun: p1[1]");
    assert!((p1[2] - -0.2167355419322321302).abs() < 1e-12, "ldsun: p1[2]");
}