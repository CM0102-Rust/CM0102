// ---------- mwa_27: the League of Ireland's schedules (ire_prm.cpp, ire_first.cpp): the Premier and First Divisions' matchdays and the promotion play-off ----------

/// The League of Ireland's opening matchday: Saturday 15 August [inference: weekday 6, code 1] (both divisions share
/// the dates); (day, month from 0, years after the league's year, weekday, code, the match slots).
pub(crate) const IRISH_LEAGUE_OPENING: [FraThirdLeagueRound; 1] = [(15, 7, 0, 6, 1, &[(-1, -1, -1)])]; // mwa27: the opening matchday

/// The extra midweek matchday of a 36-round season: 17 August (weekday 1, code 2).
pub(crate) const IRISH_LEAGUE_EXTRA_AUGUST: [FraThirdLeagueRound; 1] = [(17, 7, 0, 1, 2, &[(-1, -1, -1)])];

/// The weekly matchdays from 22 August to 19 December (weekday 6, code 1).
pub(crate) const IRISH_LEAGUE_AUTUMN: [FraThirdLeagueRound; 17] = [
    (22, 7, 0, 6, 1, &[(-1, -1, -1)]), // 22 aug
    (5, 8, 0, 6, 1, &[(-1, -1, -1)]), // 5 sep
    (12, 8, 0, 6, 1, &[(-1, -1, -1)]), // 12 sep
    (19, 8, 0, 6, 1, &[(-1, -1, -1)]), // 19 sep
    (26, 8, 0, 6, 1, &[(-1, -1, -1)]), // 26 sep
    (3, 9, 0, 6, 1, &[(-1, -1, -1)]), // 3 oct
    (10, 9, 0, 6, 1, &[(-1, -1, -1)]), // 10 oct
    (17, 9, 0, 6, 1, &[(-1, -1, -1)]), // 17 oct
    (24, 9, 0, 6, 1, &[(-1, -1, -1)]), // 24 oct
    (31, 9, 0, 6, 1, &[(-1, -1, -1)]), // 31 oct
    (7, 10, 0, 6, 1, &[(-1, -1, -1)]), // 7 nov
    (14, 10, 0, 6, 1, &[(-1, -1, -1)]), // 14 nov
    (21, 10, 0, 6, 1, &[(-1, -1, -1)]), // 21 nov
    (28, 10, 0, 6, 1, &[(-1, -1, -1)]), // 28 nov
    (5, 11, 0, 6, 1, &[(-1, -1, -1)]), // 5 dec
    (12, 11, 0, 6, 1, &[(-1, -1, -1)]), // 12 dec
    (19, 11, 0, 6, 1, &[(-1, -1, -1)]), // 19 dec
];

/// The Christmas matchday when 25 or 24 December is the weekday-6 day the 26 December round would move to: 22 December
/// midweek (weekday 2, code 2).
pub(crate) const IRISH_LEAGUE_CHRISTMAS_MIDWEEK: [FraThirdLeagueRound; 1] = [(22, 11, 0, 2, 2, &[(-1, -1, -1)])];

/// The Christmas matchday otherwise: 26 December (weekday 6, code 1).
pub(crate) const IRISH_LEAGUE_CHRISTMAS: [FraThirdLeagueRound; 1] = [(26, 11, 0, 6, 1, &[(-1, -1, -1)])];

/// The January matchdays of the next year: 3, 16, 23 and 30 (weekday 6, code 1).
pub(crate) const IRISH_LEAGUE_JANUARY: [FraThirdLeagueRound; 4] = [
    (3, 0, 1, 6, 1, &[(-1, -1, -1)]),
    (16, 0, 1, 6, 1, &[(-1, -1, -1)]),
    (23, 0, 1, 6, 1, &[(-1, -1, -1)]),
    (30, 0, 1, 6, 1, &[(-1, -1, -1)]),
];

/// The extra midweek matchday of a 36-round season in the next year: 8 February (weekday 1, code 2).
pub(crate) const IRISH_LEAGUE_EXTRA_FEBRUARY: [FraThirdLeagueRound; 1] = [(8, 1, 1, 1, 2, &[(-1, -1, -1)])];

/// The last eleven matchdays, from 13 February to 23 April of the next year (weekday 6, code 1; 7 March midweek).
pub(crate) const IRISH_LEAGUE_SPRING: [FraThirdLeagueRound; 11] = [
    (13, 1, 1, 6, 1, &[(-1, -1, -1)]), // 13 feb (+1)
    (20, 1, 1, 6, 1, &[(-1, -1, -1)]), // 20 feb (+1)
    (27, 1, 1, 6, 1, &[(-1, -1, -1)]), // 27 feb (+1)
    (7, 2, 1, 1, 2, &[(-1, -1, -1)]), // 7 mar (+1)
    (12, 2, 1, 6, 1, &[(-1, -1, -1)]), // 12 mar (+1)
    (19, 2, 1, 6, 1, &[(-1, -1, -1)]), // 19 mar (+1)
    (26, 2, 1, 6, 1, &[(-1, -1, -1)]), // 26 mar (+1)
    (2, 3, 1, 6, 1, &[(-1, -1, -1)]), // 2 apr (+1)
    (9, 3, 1, 6, 1, &[(-1, -1, -1)]), // 9 apr (+1)
    (16, 3, 1, 6, 1, &[(-1, -1, -1)]), // 16 apr (+1)
    (23, 3, 1, 6, 1, &[(-1, -1, -1)]), // 23 apr (+1)
];

/// The league schedule of the League of Ireland's divisions (the virtual `comp_vtable::CREATE_SCHEDULE` at stage −1;
/// IrePrm and IreFirst write the same rounds, read instruction by instruction with the emulator of the 36 and 34 round
/// paths and both Christmas cases).
///
/// n = 36 rounds unless `league::LEGS` is 3 (then 34); `out` (when given) = 1, `rounds` = n, `extra` = 0; the table
/// (`exe::crt_malloc` of n × 0x41; out of memory, the box of `rows.0` and 0). The rounds, each with match slot 0 empty
/// (`fra_third_write_league_rounds`): `IRISH_LEAGUE_OPENING`; with 36, `IRISH_LEAGUE_EXTRA_AUGUST`;
/// `IRISH_LEAGUE_AUTUMN`; the Christmas round: Date(25 December of the league's year, no weekday) against Date(26
/// December moved to weekday 6) (Date::Date `exe::date_date_549f70`, thiscall, ret 0x10; the two words compared), and
/// if they differ the same with 24 December: equal in either, `IRISH_LEAGUE_CHRISTMAS_MIDWEEK`, otherwise
/// `IRISH_LEAGUE_CHRISTMAS`; `IRISH_LEAGUE_JANUARY`; with 36, `IRISH_LEAGUE_EXTRA_FEBRUARY`; `IRISH_LEAGUE_SPRING`.
/// When the rounds written are not n (never: the original's own check), the box of `rows.1`. Returns the table.
pub(crate) unsafe fn irish_league_schedule(this: League, rounds: *mut i16, extra: *mut i16, out: *mut i32, file: usize, rows: (u32, u32), branch: &'static str) -> usize {
    let n: i16 = if this.legs() != 3 { 0x24 } else { 0x22 };
    if !out.is_null() {
        *out = 1;
    }
    *rounds = n;
    *extra = 0;
    let t = exe::crt_malloc((n as i32 * 0x41) as u32);
    if t == 0 {
        show_error_at(file, rows.0);
        return 0;
    }
    covered!(branch);
    let year = this.year() as u16 as u32;
    let mut r = 0i32;
    let mut put = |table: &[FraThirdLeagueRound]| {
        fra_third_write_league_rounds(t, year, r, table);
        r += table.len() as i32;
    };
    put(&IRISH_LEAGUE_OPENING);
    if n == 0x24 {
        put(&IRISH_LEAGUE_EXTRA_AUGUST);
    }
    put(&IRISH_LEAGUE_AUTUMN);
    let same = |day: u32| {
        let (mut a, mut b) = ([0u16; 4], [0u16; 4]);
        exe::date_date_549f70(&mut a, day, 0xb, year, 0xffff_ffff);
        exe::date_date_549f70(&mut b, 0x1a, 0xb, year, 6);
        a[1] == b[1] && a[0] == b[0]
    };
    let midweek = same(0x19) || same(0x18); // mwa27: the Christmas case
    put(if midweek { &IRISH_LEAGUE_CHRISTMAS_MIDWEEK } else { &IRISH_LEAGUE_CHRISTMAS });
    put(&IRISH_LEAGUE_JANUARY);
    if n == 0x24 {
        put(&IRISH_LEAGUE_EXTRA_FEBRUARY);
    }
    put(&IRISH_LEAGUE_SPRING);
    if r as i16 != n {
        show_error_at(file, rows.1);
    }
    t
}

/// Rule: the League of Ireland Premier Division plays 36 matchdays (34 when the clubs meet three times), from 15
/// August to 23 April of the next year, almost all on Saturdays [inference: weekday 6], with two midweek rounds in a
/// 36-round season (17 August and 8 February) and one on 7 March; the Christmas round is on 26 December, or midweek on
/// 22 December when Christmas or Christmas Eve is the Saturday; only the league stage.
///
/// Only stage −1 (the byte; otherwise 0 with nothing written): `irish_league_schedule` with the boxes at lines 0xad and
/// 0x156 of ire_prm.cpp.
/// Original: `IrePrm::create_schedule` (0x639e90, ire_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ire_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    irish_league_schedule(League(this), rounds, extra, out, 0x9ea080, (0xad, 0x156), "mwa_27 irish premier schedule")
}

/// Comparer for `ire_prm_create_schedule` (side by side: `compare_c1_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_ire_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_c1_schedule("ire_prm_create_schedule", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, l| ire_prm_create_schedule(this, stage, r, e, l))
}

/// The Irish promotion play-off: one round of two clubs over two legs two days apart [the gap byte the original
/// writes], drawn on 24 April and first played on 27 April of the next year (weekday 3, code 1); code 0xa0, tie rule 4
/// and two-leg rule 7; category 5; no prize.
pub(crate) const IRISH_PLAY_OFF: EngCcCupCupRound = cql10r((0x18, 3, 1, 0), (0x1b, 3, 1, 3, 1, 0), 0xa0, (0, 4, 7), 5, (2, 1, 2, 0), (0, 2, 2), (0, 0, 0)); // mwa27: the play-off on 27 April

/// Rule: the League of Ireland First Division plays on the Premier Division's dates (36 matchdays, or 34 when the
/// clubs meet three times); then the promotion play-off against the Premier Division's play-off club, two legs from 27
/// April.
///
/// Stage −1: `irish_league_schedule` with the boxes at lines 0xb5 and 0x15e of ire_first.cpp. Stage 0: `out` (when
/// given) = 0, `rounds` = 1, `extra` = 0, the table (`exe::crt_malloc` of 0x68; out of memory, the box at line 0x171 and
/// 0) with `IRISH_PLAY_OFF` (`eng_cc_cup_write_cup_round`; the original writes the same fields in another order).
/// Other stages: 0.
/// Original: `IreFirst::create_schedule` (0x634f40, ire_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ire_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let this = League(this);
    match stage as u8 {
        0xff => irish_league_schedule(this, rounds, extra, out, 0x9e9ed0, (0xb5, 0x15e), "mwa_27 irish first schedule"),
        0 => {
            if !out.is_null() {
                *out = 0;
            }
            *rounds = 1;
            *extra = 0;
            let t = exe::crt_malloc(0x68);
            if t == 0 {
                show_error_at(0x9e9ed0, 0x171);
                return 0;
            }
            covered!("mwa_27 irish play-off round");
            eng_cc_cup_write_cup_round(t, 0, &IRISH_PLAY_OFF, this.year() as i32 as u32);
            t
        }
        _ => 0,
    }
}

/// Comparer for `ire_first_create_schedule` (side by side: `compare_c1_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_ire_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_c1_schedule("ire_first_create_schedule", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, l| ire_first_create_schedule(this, stage, r, e, l))
}
