// ---------- batch %LEVA%: the schedules of the Northern Irish Premier and First Divisions (nir_prm.cpp, nir_first.cpp) ----------

/// The Northern Irish leagues' first twenty matchdays (rounds 0 to 19), from 12 August to 16 December: Saturdays
/// (weekday 5, code 1) and a few midweek rounds (weekday 2, code 2); no TV slots.
pub(crate) const NORTHERN_IRISH_LEAGUE_START: [FraThirdLeagueRound; 20] = [
    (12, 7, 0, 5, 1, &[(-1, -1, -1)]), // mwa20: the first Northern Irish matchday, 12 August
    (19, 7, 0, 5, 1, &[(-1, -1, -1)]), // 1: 19 aug
    (26, 7, 0, 5, 1, &[(-1, -1, -1)]), // 2: 26 aug
    (30, 7, 0, 2, 2, &[(-1, -1, -1)]), // 3: 30 aug
    (9, 8, 0, 5, 1, &[(-1, -1, -1)]), // 4: 9 sep
    (13, 8, 0, 2, 2, &[(-1, -1, -1)]), // 5: 13 sep
    (16, 8, 0, 5, 1, &[(-1, -1, -1)]), // 6: 16 sep
    (23, 8, 0, 5, 1, &[(-1, -1, -1)]), // 7: 23 sep
    (27, 8, 0, 2, 2, &[(-1, -1, -1)]), // 8: 27 sep
    (30, 8, 0, 5, 1, &[(-1, -1, -1)]), // 9: 30 sep
    (14, 9, 0, 5, 1, &[(-1, -1, -1)]), // 10: 14 oct
    (21, 9, 0, 5, 1, &[(-1, -1, -1)]), // 11: 21 oct
    (28, 9, 0, 5, 1, &[(-1, -1, -1)]), // 12: 28 oct
    (4, 10, 0, 5, 1, &[(-1, -1, -1)]), // 13: 4 nov
    (11, 10, 0, 5, 1, &[(-1, -1, -1)]), // 14: 11 nov
    (18, 10, 0, 5, 1, &[(-1, -1, -1)]), // 15: 18 nov
    (25, 10, 0, 5, 1, &[(-1, -1, -1)]), // 16: 25 nov
    (2, 11, 0, 5, 1, &[(-1, -1, -1)]), // 17: 2 dec
    (9, 11, 0, 5, 1, &[(-1, -1, -1)]), // 18: 9 dec
    (16, 11, 0, 5, 1, &[(-1, -1, -1)]), // 19: 16 dec
];

/// The Christmas pair (rounds 20 and 21) by how the 16 December round falls (k = its day − 1 December's + 1 − 13, so k
/// = 0 to 6 for 13 to 19 December after the move to the weekday): k 0 or 1, 23 and 30 December; k 2 or 3, 23 and 27
/// December (midweek); k 4 or 5, 20 (midweek) and 27 December (midweek); any other k, 20 (midweek) and 23 December.
pub(crate) const NORTHERN_IRISH_CHRISTMAS_0: [FraThirdLeagueRound; 2] = [(23, 11, 0, 5, 1, &[(-1, -1, -1)]), (30, 11, 0, 5, 1, &[(-1, -1, -1)])];
pub(crate) const NORTHERN_IRISH_CHRISTMAS_2: [FraThirdLeagueRound; 2] = [(23, 11, 0, 5, 1, &[(-1, -1, -1)]), (27, 11, 0, 2, 2, &[(-1, -1, -1)])];
pub(crate) const NORTHERN_IRISH_CHRISTMAS_4: [FraThirdLeagueRound; 2] = [(20, 11, 0, 2, 2, &[(-1, -1, -1)]), (27, 11, 0, 2, 2, &[(-1, -1, -1)])];
pub(crate) const NORTHERN_IRISH_CHRISTMAS_DEFAULT: [FraThirdLeagueRound; 2] = [(20, 11, 0, 2, 2, &[(-1, -1, -1)]), (23, 11, 0, 5, 1, &[(-1, -1, -1)])];

/// The Northern Irish Premier Division's last fourteen matchdays (rounds 22 to 35), from 6 January to 28 April of the
/// next year, on weekday 5 (one midweek round on 11 April).
pub(crate) const NORTHERN_IRISH_PREMIER_END: [FraThirdLeagueRound; 14] = [
    (6, 0, 1, 5, 1, &[(-1, -1, -1)]), // 22: 6 jan (+1)
    (13, 0, 1, 5, 1, &[(-1, -1, -1)]), // 23: 13 jan (+1)
    (17, 0, 1, 5, 1, &[(-1, -1, -1)]), // 24: 17 jan (+1)
    (27, 0, 1, 5, 1, &[(-1, -1, -1)]), // 25: 27 jan (+1)
    (3, 1, 1, 5, 1, &[(-1, -1, -1)]), // 26: 3 feb (+1)
    (10, 1, 1, 5, 1, &[(-1, -1, -1)]), // 27: 10 feb (+1)
    (24, 1, 1, 5, 1, &[(-1, -1, -1)]), // 28: 24 feb (+1)
    (3, 2, 1, 5, 1, &[(-1, -1, -1)]), // 29: 3 mar (+1)
    (17, 2, 1, 5, 1, &[(-1, -1, -1)]), // 30: 17 mar (+1)
    (31, 2, 1, 5, 1, &[(-1, -1, -1)]), // 31: 31 mar (+1)
    (11, 3, 1, 2, 2, &[(-1, -1, -1)]), // 32: 11 apr (+1)
    (14, 3, 1, 5, 1, &[(-1, -1, -1)]), // 33: 14 apr (+1)
    (21, 3, 1, 5, 1, &[(-1, -1, -1)]), // 34: 21 apr (+1)
    (28, 3, 1, 5, 1, &[(-1, -1, -1)]), // 35: 28 apr (+1)
];

/// The Northern Irish First Division's last fourteen matchdays: the Premier's dates moved to weekday 6 (the 11 April
/// midweek round the same).
pub(crate) const NORTHERN_IRISH_FIRST_END: [FraThirdLeagueRound; 14] = [
    (6, 0, 1, 6, 1, &[(-1, -1, -1)]), // 22: 6 jan (+1)
    (13, 0, 1, 6, 1, &[(-1, -1, -1)]), // 23: 13 jan (+1)
    (17, 0, 1, 6, 1, &[(-1, -1, -1)]), // 24: 17 jan (+1)
    (27, 0, 1, 6, 1, &[(-1, -1, -1)]), // 25: 27 jan (+1)
    (3, 1, 1, 6, 1, &[(-1, -1, -1)]), // 26: 3 feb (+1)
    (10, 1, 1, 6, 1, &[(-1, -1, -1)]), // 27: 10 feb (+1)
    (24, 1, 1, 6, 1, &[(-1, -1, -1)]), // 28: 24 feb (+1)
    (3, 2, 1, 6, 1, &[(-1, -1, -1)]), // 29: 3 mar (+1)
    (17, 2, 1, 6, 1, &[(-1, -1, -1)]), // 30: 17 mar (+1)
    (31, 2, 1, 6, 1, &[(-1, -1, -1)]), // 31: 31 mar (+1)
    (11, 3, 1, 2, 2, &[(-1, -1, -1)]), // 32: 11 apr (+1)
    (14, 3, 1, 6, 1, &[(-1, -1, -1)]), // 33: 14 apr (+1)
    (21, 3, 1, 6, 1, &[(-1, -1, -1)]), // 34: 21 apr (+1)
    (28, 3, 1, 6, 1, &[(-1, -1, -1)]), // 35: 28 apr (+1)
];

/// The 36-matchday schedule of a Northern Irish league (the league's virtual `comp_vtable::CREATE_SCHEDULE` at stage
/// −1): `out` (when given) = 1, `rounds` = 36, `extra` = 0; the table (`exe::crt_malloc` of 36 × 0x41; out of memory,
/// the error box at `row` and 0); the first twenty rounds (`fra_third_write_league_rounds`); 1 December of the league's
/// year (Date::Date `exe::date_date_549f70`(&d, 1, 11, year, −1), thiscall, ret 0x10) and the Christmas pair chosen by
/// k = (round 19's day − that day + 1, in 16 bits) − 13 (`NORTHERN_IRISH_CHRISTMAS_*`); then the last fourteen rounds
/// `end`. Returns the table.
pub(crate) unsafe fn northern_irish_league_schedule(this: League, rounds: *mut i16, extra: *mut i16, out: *mut i32, end: &[FraThirdLeagueRound], file: usize, row: u32, branch: &'static str) -> usize {
    if !out.is_null() {
        *out = 1;
    }
    *rounds = 0x24;
    *extra = 0;
    let t = exe::crt_malloc(0x924);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    covered!(branch);
    let year = this.year() as u16 as u32;
    fra_third_write_league_rounds(t, year, 0, &NORTHERN_IRISH_LEAGUE_START);
    let mut d = [0u16; 4];
    exe::date_date_549f70(&mut d, 1, 0xb, year, 0xffff_ffff);
    let k = (read_u16(t + 19 * 0x41).wrapping_sub(d[0]).wrapping_add(1) as i16 as i32).wrapping_sub(0xd); // mwa20: the Christmas case
    let christmas: &[FraThirdLeagueRound] = match k {
        0 | 1 => &NORTHERN_IRISH_CHRISTMAS_0,
        2 | 3 => &NORTHERN_IRISH_CHRISTMAS_2,
        4 | 5 => &NORTHERN_IRISH_CHRISTMAS_4,
        _ => &NORTHERN_IRISH_CHRISTMAS_DEFAULT,
    };
    fra_third_write_league_rounds(t, year, 20, christmas);
    fra_third_write_league_rounds(t, year, 22, end);
    t
}

/// Rule: the Northern Irish Premier Division plays 36 matchdays (ten clubs meeting four times), from 12 August to 28
/// April of the next year, mostly on Saturdays with a few midweek rounds; the two Christmas rounds depend on the
/// weekday of the 16 December round; only the league stage (no play-off).
///
/// Only stage −1 (the byte; otherwise 0 with nothing written): `northern_irish_league_schedule` with
/// `NORTHERN_IRISH_PREMIER_END` and the error box at line 0xa0 of nir_prm.cpp.
/// Original: `NirPrm::create_schedule` (0x78e330, nir_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn nir_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    northern_irish_league_schedule(League(this), rounds, extra, out, &NORTHERN_IRISH_PREMIER_END, 0xa696d0, 0xa0, "mwa_20 northern irish premier schedule")
}

unsafe extern "thiscall" fn compare_nir_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_c1_schedule("nir_prm_create_schedule", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, l| nir_prm_create_schedule(this, stage, r, e, l))
}

/// The Northern Irish promotion play-off: one round of two clubs over two legs seven days apart, drawn on 29 April and
/// first played on 5 May of the next year (weekday 3); code 0xa0, tie rule 3 [inference: away goals]; category 5; no
/// prize.
pub(crate) const NORTHERN_IRISH_PLAY_OFF: EngCcCupCupRound = cql10r((0x1d, 3, 1, 0), (5, 4, 1, 3, 1, 0), 0xa0, (0, 0, 3), 5, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)); // mwa20: the play-off on 5 May

/// Rule: the Northern Irish First Division plays 36 matchdays (ten clubs meeting four times), on the Premier's dates up
/// to Christmas and on Sundays after it [inference: weekday 6]; then the promotion play-off against the Premier's second
/// last, two legs seven days apart from 5 May.
///
/// Stage −1: `northern_irish_league_schedule` with `NORTHERN_IRISH_FIRST_END` and the error box at line 0xae of
/// nir_first.cpp. Stage 0: `out` (when given) = 0, `rounds` = 1, `extra` = 0, the table (`exe::crt_malloc` of 0x68; out of
/// memory, the box at line 0x163 and 0) with `NORTHERN_IRISH_PLAY_OFF` (`eng_cc_cup_write_cup_round`; the original writes
/// the same fields in another order). Other stages: 0.
/// Original: `NirFirst::create_schedule` (0x78b310, nir_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn nir_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let this = League(this);
    match stage as u8 {
        0xff => northern_irish_league_schedule(this, rounds, extra, out, &NORTHERN_IRISH_FIRST_END, 0xa6965c, 0xae, "mwa_20 northern irish first schedule"),
        0 => {
            if !out.is_null() {
                *out = 0;
            }
            *rounds = 1;
            *extra = 0;
            let t = exe::crt_malloc(0x68);
            if t == 0 {
                show_error_at(0xa6965c, 0x163);
                return 0;
            }
            covered!("mwa_20 northern irish play-off round");
            eng_cc_cup_write_cup_round(t, 0, &NORTHERN_IRISH_PLAY_OFF, this.year() as i32 as u32);
            t
        }
        _ => 0,
    }
}

unsafe extern "thiscall" fn compare_nir_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_c1_schedule("nir_first_create_schedule", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, l| nir_first_create_schedule(this, stage, r, e, l))
}
