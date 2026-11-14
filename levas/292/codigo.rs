// ---------- batch 292: the Série C schedule (the dates and matches of each round of the group and knockout stages) (bra_nat_third.cpp) ----------

/// The Brazilian Série C, the first-round groups (stages −128 to 6, league): 14 rounds (5 Jul to 7 Sep), 910 bytes, 28 operations.
const SERIE_C_SCHEDULE_FIRST_ROUND_OPS: [AsiaClubChampScheduleOp; 28] = [
    Cqp11D(0, 5, 7, 0, 6, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0xc, 7, 0, 6, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0xf, 7, 0, 2, 2, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0x13, 7, 0, 6, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 0x1a, 7, 0, 6, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 0x1d, 7, 0, 2, 2, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 2, 8, 0, 6, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 5, 8, 0, 2, 2, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 9, 8, 0, 6, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 0x10, 8, 0, 6, 1, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 0x13, 8, 0, 2, 2, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0x17, 8, 0, 6, 1, 0), Cqp11L(0xb, 0, -1, -1, -1, 0),
    Cqp11D(0xc, 0x1e, 8, 0, 6, 1, 0), Cqp11L(0xc, 0, -1, -1, -1, 0), Cqp11D(0xd, 7, 9, 0, 6, 1, 0),
    Cqp11L(0xd, 0, -1, -1, -1, 0),
];

/// The Brazilian Série C, stage 7 (knockout): 1 round (draw 8 Sep, match 14 Sep), 104 bytes, 19 operations.
const SERIE_C_SCHEDULE_SECOND_ROUND_OPS: [AsiaClubChampScheduleOp; 19] = [
    Cqp11S(0, 8, 9, 0, 0), Cqp11J(0, 0xe, 9, 0, 6, 1, 0), Cqp11W(0x7, 0x14), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 5), Cqp11W(0x18, 0x10), Cqp11W(0x1a, 8), Cqp11W(0x1c, 0x10), Cqp11W(0xf, 1),
    Cqp11W(0x11, 0), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0),
    Cqp11X(0x60, 0), Cqp11X(0x64, 0),
];

/// The Brazilian Série C, the third-round groups (stages 8 and 9, league): 6 rounds (28 Sep to 2 Nov), 390 bytes, 12 operations.
const SERIE_C_SCHEDULE_THIRD_ROUND_OPS: [AsiaClubChampScheduleOp; 12] = [
    Cqp11D(0, 0x1c, 9, 0, 6, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 4, 0xa, 0, 6, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0xb, 0xa, 0, 6, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0x12, 0xa, 0, 6, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 0x19, 0xa, 0, 6, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 2, 0xb, 0, 6, 1, 0), Cqp11L(5, 0, -1, -1, -1, 0),
];

/// The Brazilian Série C, stage 10, the final: 1 round (draw 3 Nov, match 9 Nov), 104 bytes, 19 operations.
const SERIE_C_SCHEDULE_FINAL_OPS: [AsiaClubChampScheduleOp; 19] = [
    Cqp11S(0, 3, 0xb, 0, 0), Cqp11J(0, 9, 0xb, 0, 6, 1, 0), Cqp11W(0x7, 0x96), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0), Cqp11B(0x17, 5), Cqp11W(0x18, 2), Cqp11W(0x1a, 1), Cqp11W(0x1c, 2), Cqp11W(0xf, 1),
    Cqp11W(0x11, 1), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0),
    Cqp11X(0x60, 0), Cqp11X(0x64, 0),
];

/// The stages of the round table of the Brazilian Série C (virtual `comp_vtable::CREATE_SCHEDULE`); the stage is the
/// signed byte of the argument.
///
/// Stages −128 to 6 (the first-round groups, league): 14 rounds from 5 Jul to 7 Sep, extra = stage + 0x420 (the word of
/// the sign-extended byte); stage 7 (knockout): 1 round, draw 8 Sep and match 14 Sep, extra 0; stages 8 and 9 (the
/// third-round groups, league): 6 rounds from 28 Sep to 2 Nov, extra = stage + 0x476; stage 10 (the final): 1 round, draw
/// 3 Nov and match 9 Nov, extra 0; any stage above 10 has no table (None). is_league is written only through a
/// non-null pointer; the rounds and the extra always.
fn serie_c_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    let s = stage as u8 as i8;
    match s {
        -128..=6 => Some(ScheduleStage { league: 1, rounds: 14, extra: (s as i16).wrapping_add(0x420), extra_check: false, size: 0x38e,
            row: 0xbb, ops: &SERIE_C_SCHEDULE_FIRST_ROUND_OPS, name: "cqy_1 serie c first round" }),
        7 => Some(ScheduleStage { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x10d,
            ops: &SERIE_C_SCHEDULE_SECOND_ROUND_OPS, name: "cqy_1 serie c stage 7" }),
        8 | 9 => Some(ScheduleStage { league: 1, rounds: 6, extra: (s as i16).wrapping_add(0x476), extra_check: false, size: 0x186,
            row: 0x136, ops: &SERIE_C_SCHEDULE_THIRD_ROUND_OPS, name: "cqy_1 serie c third round" }),
        10 => Some(ScheduleStage { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x15d,
            ops: &SERIE_C_SCHEDULE_FINAL_OPS, name: "cqy_1 serie c final" }),
        _ => None,
    }
}

/// The schedule of the Brazilian Série C: the first-round groups (stages up to 6) play 14 rounds from 5 Jul to 7 Sep;
/// stage 7 is a one-round knockout on 14 Sep; the third-round groups (stages 8 and 9) play 6 rounds from 28 Sep to
/// 2 Nov; stage 10 is the one-round final on 9 Nov; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures, the group stages and the final (`bra_nat_third_create_final_stage`). For a stage with a
/// table (`serie_c_schedule_stage`) it writes the outputs (is_league when that pointer is not null, the rounds and the
/// extra: stage + 0x420 in the first-round groups, stage + 0x476 in the third-round ones, 0 in the knockouts),
/// allocates the round table (`exe::crt_malloc`, 0x41 bytes per league round, 0x68 per knockout round; out of memory,
/// the error box of bra_nat_third.cpp, lines 0xbb, 0x10d, 0x136 and 0x15d, and 0) and writes the date (day, month,
/// years after the season year `year()`, weekday and code) and the matches of each round, in the original's order
/// (`create_round_table`, the cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table,
/// or 0 without writing anything for a stage above 10.
/// Original: `BraNatThird::create_schedule` (0x42d620, bra_nat_third.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_nat_third_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), serie_c_schedule_stage(stage), rounds, extra, league, 0x98aba4) // cqy1: the Série C schedule
}

/// Comparer for `bra_nat_third_create_schedule`: side by side (`asia_club_champ_compare_round_table`), the outputs and
/// the bytes written to the two tables.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_third_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_NAT_THIRD_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_nat_third_create_schedule", stage, serie_c_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_nat_third_create_schedule(this, stage, r, e, l))
}

// ---------- batch 292: the MLS season fixtures: the first division's own fixtures, the other two division stages and the cross-division fixtures (usa_mls.cpp) ----------

/// The MLS division pairs of a cross-division match, as the original's three-character strings ("00_" 0xaa2094: the
/// league's own table and stage 0; "0_0" 0xaa2098: the league and stage 1; "_00" 0xaa209c: stages 0 and 1): which of
/// the three tables (`[league, stage 0, stage 1]`) take part.
const MLS_PAIR_FIRST_SECOND: [bool; 3] = [true, true, false];
const MLS_PAIR_FIRST_THIRD: [bool; 3] = [true, false, true];
const MLS_PAIR_SECOND_THIRD: [bool; 3] = [false, true, true];

/// The six matches of an ordinary cross-division round, in the original's order (the pointer array at esp+0xc0): two of
/// the first against the second division, two of the first against the third, two of the second against the third.
const MLS_ORDINARY_ROUND_PAIRS: [[bool; 3]; 6] = [MLS_PAIR_FIRST_SECOND, MLS_PAIR_FIRST_SECOND, MLS_PAIR_FIRST_THIRD,
    MLS_PAIR_FIRST_THIRD, MLS_PAIR_SECOND_THIRD, MLS_PAIR_SECOND_THIRD];

/// The four matches of a row round, by the count of row rounds so far mod 3 (the arrays at esp+0x90, +0xa0 and +0xb0):
/// row i of one division against row i of the other.
const MLS_ROW_ROUND_PAIRS: [[[bool; 3]; 4]; 3] = [
    [MLS_PAIR_FIRST_SECOND, MLS_PAIR_FIRST_SECOND, MLS_PAIR_SECOND_THIRD, MLS_PAIR_SECOND_THIRD],
    [MLS_PAIR_FIRST_THIRD, MLS_PAIR_FIRST_THIRD, MLS_PAIR_FIRST_SECOND, MLS_PAIR_FIRST_SECOND],
    [MLS_PAIR_SECOND_THIRD, MLS_PAIR_SECOND_THIRD, MLS_PAIR_FIRST_THIRD, MLS_PAIR_FIRST_THIRD],
];

/// Whether cross-division round k is a row round (rounds 2, 5, 8, 12, 15 and 18: each row against the same row of
/// another division).
fn is_mls_row_round(k: i16) -> bool {
    k == 2 || k == 5 || k == 8 || k == 12 || k == 15 || k == 18 // cqy2: the row rounds
}

/// Puts one cross-division match into the fixture record and the calendar.
///
/// `rows` are the row indices of the three tables (`[league, stage 0, stage 1]`, 0xff = that division does not play);
/// `tables` their table pointers (`league::TABLE`, rows of 0x3b bytes, +0 the club). The home side is the first present
/// club and the away side the next one; a null club pointer in the home place lets the next club take the home side,
/// as in the original. With `swap`, they change places. The ids (`fixture::HOME_CLUB_ID`, `fixture::AWAY_CLUB_ID`) are
/// the clubs' +0, or −1 with none. Then the stadium (Stadium::set_fixture_venue `exe::stadium_set_fixture_venue`: push
/// the league's `fixed_stadium_id()`; push &match; push the round's `league_round_date::VENUE_VALUE`; mov ecx,
/// [0xdd7ef4] -> thiscall, ret 0xc) and the calendar (FIXTURE_MANAGER::add_fixture `exe::fixture_manager_add_fixture`:
/// push 1; push &match; mov ecx, [0xae2a58] -> thiscall, ret 8).
unsafe fn add_mls_cross_division_fixture(this: League, r: Fixture, tables: [usize; 3], rows: [u8; 3], swap: bool, venue: usize) {
    let (mut home, mut away) = (0usize, 0usize);
    for d in 0..3 {
        if rows[d] == 0xff {
            continue;
        }
        let c = read_usize(tables[d] + (rows[d] as i8 as isize * 0x3b) as usize);
        if home != 0 {
            away = c;
        } else {
            home = c;
        }
    }
    if swap {
        core::mem::swap(&mut home, &mut away); // cqy2: home and away by the parity of the round
    }
    r.set_home_club(Club(home));
    r.set_away_club(Club(away));
    r.set_home_club_id(if home != 0 { read_usize(home) } else { usize::MAX });
    r.set_away_club_id(if away != 0 { read_usize(away) } else { usize::MAX });
    exe::stadium_set_fixture_venue(read_usize(G_STADIUM), read_usize(venue), r, this.fixed_stadium_id());
    exe::fixture_manager_add_fixture(read_usize(G_FIXTURE_MANAGER), r, 1);
}

/// The MLS season fixtures: the first division's own round robin, the other two divisions as stages, then the
/// cross-division matches (every club meets clubs of the other two divisions), and the round table of the league's own
/// stage again. Returns 1.
///
/// In order: the league's round table for stage −3 (virtual `comp_vtable::CREATE_SCHEDULE`: push 0; push
/// &`league::STAGE_ID`; push &`league::ROUND_COUNT`; push −3 -> thiscall, ret 0x10) into `round_dates()`, the fixtures
/// of the first division (League::create_fixtures `create_fixtures`, thiscall, ret 0) and that table freed (free
/// `exe::crt_free`, the pointer left as it is); the second and third division stages
/// (UsaMls::create_second_division_stage and UsaMls::create_third_division_stage, thiscall, ret 0); the two stages read
/// (`stages()`[0] and [1]); the cross-division table, stage −2 (the same call), into `round_dates()`.
///
/// The match record (0x4f bytes on the stack): +0 the competition id, `fixture::COMPETITION`, `fixture::SEASON_YEAR`
/// (`year()`), `fixture::ROUND_NAME` 0, `fixture::ROUND_PREFIX` (`stage_id()`), `fixture::CATEGORY`
/// (`match_category()`, then per round), `fixture::ROUND_DRAW_RULES` (`round_draw_rules()`), `fixture::TIE_RULES`
/// (`match_rules()`), `fixture::SQUAD_RULES` low nibble `substitutes_named()` and next nibble `substitutions_allowed()`
/// (bits 8..10 cleared), `fixture::STAGE_INDEX` (`stage_index()`), +0x43..+0x4c 0xff, `fixture::MATCH_STATS_SLOT` −1,
/// the result marks 0, `fixture::STAGE` 0xff, both side marks 1, and `fixture::NEXT_ROUND_DAY` /
/// `fixture::NEXT_ROUND_YEAR` the last date of the cross-division table (Date::Date `exe::new_date` of its day and its
/// year + `year()`).
///
/// For each round k of the table: `fixture::YEAR`, `fixture::DAY` and `fixture::TIME_SLICE` of entry k,
/// `fixture::CATEGORY` 2 (5 in the last three rounds), `fixture::ROUND_NUMBER` k. A row round (`is_mls_row_round`; the
/// t-th, t from 0) plays row i (0..3) of one division against row i of another (`MLS_ROW_ROUND_PAIRS`[t % 3]), home and
/// away swapped when t is odd. An ordinary round (the s-th, s from 0) plays six matches (`MLS_ORDINARY_ROUND_PAIRS`):
/// with m = s % 4 and the base row b = 0 (s < 4 or 8 ≤ s < 12) or 2, match j moves b on (b + 1 from j = 1; at j = 4,
/// the second division's last row + 1), all mod 4; the first table present takes row b, the second row b + m (between
/// the second and third divisions, from j = 4: b + 2 with m even, otherwise b + 1 at j = 5 and b + 3 at j = 4), mod 4;
/// home and away swapped when s is odd (`add_mls_cross_division_fixture`).
///
/// Finally the cross-division table freed and the league's table for stage −1 (the same call) into `round_dates()`,
/// and `stage_id()` = 0x413.
///
/// Simplification: the original's record keeps leftovers of its stack in the bytes it does not write (+4..+7, +0x18,
/// +0x2e, +0x2f, +0x39, +0x3c and the bits 11..15 of `fixture::SQUAD_RULES`, read and written back by the and/or); here
/// they are 0, a value the game produces (as in `create_fixtures`).
/// Original: `UsaMls::create_season_fixtures` (0x9086e0, usa_mls.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn usa_mls_create_season_fixtures(this: usize) -> u32 {
    let this = League(this);
    let schedule: unsafe extern "thiscall" fn(usize, i32, usize, usize, u32) -> usize = core::mem::transmute(vtable_method(this, comp_vtable::CREATE_SCHEDULE));
    let rounds_at = this.addr() + league::ROUND_COUNT;
    let stage_id_at = this.addr() + league::STAGE_ID;
    this.set_round_dates(schedule(this.addr(), -3, rounds_at, stage_id_at, 0));
    create_fixtures(this.addr());
    exe::crt_free(this.round_dates());
    create_second_division_stage(this.addr());
    create_third_division_stage(this.addr());
    let second = read_usize(this.stages());
    let third = read_usize(this.stages() + 4);
    this.set_round_dates(schedule(this.addr(), -2, rounds_at, stage_id_at, 0));
    let mut reg = [0u8; 0x60];
    let r = Fixture(reg.as_mut_ptr() as usize);
    let c = this.competition().addr();
    r.set_squad_rules(((this.substitutes_named() & 0xf) as u16 | (((this.substitutions_allowed() & 0xf) as u16) << 4)) & 0xf8ff);
    write_usize(r.addr(), read_usize(c));
    r.set_competition(Competition(c));
    r.set_round_name(0);
    r.set_season_year(this.year() as u16);
    r.set_round_prefix(this.stage_id());
    r.set_category(this.match_category());
    r.set_round_draw_rules(this.round_draw_rules() as u16);
    r.set_tie_rules(this.match_rules() as u16);
    r.set_stage_index(this.stage_index());
    for k in 0x43..=0x4cusize {
        write_u8(r.addr() + k, 0xff);
    }
    r.set_match_stats_slot(usize::MAX);
    r.set_home_result_mark(0);
    r.set_away_result_mark(0);
    r.set_stage(0xff);
    r.set_home_side_mark(1);
    r.set_away_side_mark(1);
    let last = this.round_dates().wrapping_add((this.round_count() as isize * 0x41) as usize);
    let mut d = [0u8; 8];
    // the original pushes the words with leftovers in the high bytes; Date::Date only reads the words
    exe::new_date(d.as_mut_ptr() as usize, read_u16(last - 0x41) as u32, read_i16(last - 0x3f).wrapping_add(this.year()) as u16 as u32);
    r.set_next_round_day(read_u16(d.as_ptr() as usize));
    r.set_next_round_year(read_u16(d.as_ptr() as usize + 2));
    let tables = [this.table(), read_usize(second + league::TABLE), read_usize(third + league::TABLE)];
    let (mut ordinary, mut row_rounds) = (0i16, 0i16);
    let mut second_last_row: u8 = 0; // esp+0x17: written (j = 1) before it is read (j = 4) in every ordinary round
    let mut k: i16 = 0;
    while k < this.round_count() {
        let e = this.round_dates() + k as usize * 0x41;
        r.set_year(read_i16(e + league_round_date::YEAR_OFFSET).wrapping_add(this.year()) as u16);
        r.set_day(read_u16(e));
        r.set_time_slice(read_u8(e + league_round_date::TIME_SLICE));
        r.set_category(if (k as i32) < this.round_count() as i32 - 3 { 2 } else { 5 });
        r.set_round_number(k);
        let venue = e + league_round_date::VENUE_VALUE;
        if is_mls_row_round(k) {
            covered!("cqy_2 mls row round");
            let pairs = &MLS_ROW_ROUND_PAIRS[(row_rounds % 3) as usize];
            let swap = row_rounds % 2 != 0;
            for (i, p) in pairs.iter().enumerate() {
                let rows = [if p[0] { i as u8 } else { 0xff }, if p[1] { i as u8 } else { 0xff }, if p[2] { i as u8 } else { 0xff }];
                add_mls_cross_division_fixture(this, r, tables, rows, swap, venue);
            }
            row_rounds += 1;
        } else {
            covered!("cqy_2 mls ordinary round");
            let m = (ordinary % 4) as i8;
            let mut b: i8 = if ordinary < 4 || (8..12).contains(&ordinary) { 0 } else { 2 };
            let swap = ordinary % 2 != 0;
            for (j, p) in MLS_ORDINARY_ROUND_PAIRS.iter().enumerate() {
                if j == 4 {
                    b = (second_last_row as i8).wrapping_add(1); // cqy2: the second against the third from its last row
                } else if j != 0 {
                    b = b.wrapping_add(1);
                }
                b %= 4;
                let mut rows = [0xffu8; 3];
                let mut next = b;
                if p[0] {
                    rows[0] = b as u8;
                    next = b.wrapping_add(m) % 4;
                }
                if p[1] {
                    rows[1] = next as u8;
                    let shift = if j > 3 {
                        if m % 2 == 0 { 2 } else if j != 4 { 1 } else { 3 }
                    } else {
                        m
                    };
                    next = b.wrapping_add(shift) % 4;
                    second_last_row = rows[1];
                }
                if p[2] {
                    rows[2] = next as u8;
                }
                add_mls_cross_division_fixture(this, r, tables, rows, swap, venue);
            }
            ordinary += 1;
        }
        k += 1;
    }
    exe::crt_free(this.round_dates());
    this.set_round_dates(schedule(this.addr(), -1, rounds_at, stage_id_at, 0));
    this.set_stage_id(0x413);
    1
}

/// Comparer for `usa_mls_create_season_fixtures`: only the original runs (it creates the stages and puts the matches
/// into the calendar: it cannot run twice); afterwards the plan: it returns 1, `stage_id()` = 0x413, and the round table
/// of stage −1 and the two division stages exist.
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_create_season_fixtures(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_USA_MLS_CREATE_SEASON_FIXTURES)(this);
    let l = League(this);
    let ok = r == 1 && l.stage_id() == 0x413 && l.round_dates() != 0 && l.stages() != 0 && read_usize(l.stages()) != 0 && read_usize(l.stages() + 4) != 0;
    SCOREBOARD.record(ok, || format!("usa_mls_create_season_fixtures({:#x}): returned {}, stage id {:#x}, table {:#x}", this, r, l.stage_id(), l.round_dates()));
    r
}

// ---------- batch 292: the names of the rounds, groups and stages, and the competition, stage and round label (comp_util.cpp) ----------

/// The full names of the round, group and stage ids (form 0 of `comp_util_get_stage_name`): id and the langlib format
/// string the original prints, sorted by id (the text in the comment).
const STAGE_NAMES_FULL: [(i16, usize); 322] = [
    (1, 0x9a7c3c), // Replay
    (2, 0x9a7c30), // Replay 2
    (3, 0x9a7c24), // First Leg
    (4, 0x9a7c18), // Second Leg
    (5, 0x9a7c10), // Decider
    (10, 0x9a7c04), // First Round
    (11, 0x9a7bf0), // First Round Replay
    (12, 0x9a7bd4), // First Round Second Replay
    (13, 0x9a7bbc), // First Round First Leg
    (14, 0x9a7ba4), // First Round Second Leg
    (20, 0x9a7b94), // Second Round
    (21, 0x9a7b80), // Second Round Replay
    (22, 0x9a7b64), // Second Round Second Replay
    (23, 0x9a7b4c), // Second Round First Leg
    (24, 0x9a7b34), // Second Round Second Leg
    (25, 0x9a7b1c), // Second Round Decider
    (30, 0x9a7b10), // Third Round
    (31, 0x9a7afc), // Third Round Replay
    (32, 0x9a7ae0), // Third Round Second Replay
    (33, 0x9a7ac8), // Third Round First Leg
    (34, 0x9a7ab0), // Third Round Second Leg
    (40, 0x9a7aa0), // Fourth Round
    (41, 0x9a7a8c), // Fourth Round Replay
    (42, 0x9a7a70), // Fourth Round Second Replay
    (43, 0x9a7a58), // Fourth Round First Leg
    (44, 0x9a7a40), // Fourth Round Second Leg
    (50, 0x9a7a34), // Fifth Round
    (51, 0x9a7a20), // Fifth Round Replay
    (52, 0x9a7a04), // Fifth Round Second Replay
    (53, 0x9a79ec), // Fifth Round First Leg
    (54, 0x9a79d4), // Fifth Round Second Leg
    (60, 0x9a79c8), // Sixth Round
    (61, 0x9a79b4), // Sixth Round Replay
    (62, 0x9a7998), // Sixth Round Second Replay
    (63, 0x9a7980), // Sixth Round First Leg
    (64, 0x9a7968), // Sixth Round Second Leg
    (70, 0x9a7958), // Seventh Round
    (71, 0x9a7940), // Seventh Round Replay
    (72, 0x9a7924), // Seventh Round Second Replay
    (73, 0x9a790c), // Seventh Round First Leg
    (74, 0x9a78f0), // Seventh Round Second Leg
    (80, 0x9a78e0), // Eighth Round
    (81, 0x9a78cc), // Eighth Round Replay
    (82, 0x9a78b0), // Eighth Round Second Replay
    (83, 0x9a7898), // Eighth Round First Leg
    (84, 0x9a7880), // Eighth Round Second Leg
    (90, 0x9a7874), // Ninth Round
    (91, 0x9a7860), // Ninth Round Replay
    (92, 0x9a7844), // Ninth Round Second Replay
    (93, 0x9a782c), // Ninth Round First Leg
    (94, 0x9a7814), // Ninth Round Second Leg
    (100, 0x9a7808), // Tenth Round
    (101, 0x9a77f4), // Tenth Round Replay
    (102, 0x9a77d8), // Tenth Round Second Replay
    (103, 0x9a77c0), // Tenth Round First Leg
    (104, 0x9a77a8), // Tenth Round Second Leg
    (110, 0x9a7798), // Eleventh Round
    (111, 0x9a7780), // Eleventh Round Replay
    (112, 0x9a7760), // Eleventh Round Second Replay
    (113, 0x9a7744), // Eleventh Round First Leg
    (114, 0x9a7728), // Eleventh Round Second Leg
    (120, 0x9a7718), // Quarter Final
    (121, 0x9a7700), // Quarter Final Replay
    (122, 0x9a76e4), // Quarter Final Second Replay
    (123, 0x9a76cc), // Quarter Final First Leg
    (124, 0x9a76b0), // Quarter Final Second Leg
    (125, 0x9a7698), // Quarter Final Decider
    (130, 0x9a768c), // Semi Final
    (131, 0x9a7678), // Semi Final Replay
    (132, 0x9a765c), // Semi Final Second Replay
    (133, 0x9a7644), // Semi Final First Leg
    (134, 0x9a762c), // Semi Final Second Leg
    (135, 0x9a7618), // Semi Final Decider
    (140, 0x9a7604), // Third Place Playoff
    (141, 0x9a75e8), // Third Place Playoff Replay
    (142, 0x9a75c4), // Third Place Playoff Second Replay
    (143, 0x9a75a4), // Third Place Playoff First Leg
    (144, 0x9a7584), // Third Place Playoff Second Leg
    (150, 0x9a757c), // Final
    (151, 0x9a756c), // Final Replay
    (152, 0x9a7558), // Final Second Replay
    (153, 0x9a7548), // Final First Leg
    (154, 0x9a7534), // Final Second Leg
    (155, 0x9a7524), // Final Decider
    (160, 0x9a751c), // Playoff
    (161, 0x9a750c), // Playoff Replay
    (162, 0x9a74f4), // Playoff Second Replay
    (163, 0x9a74e0), // Playoff First Leg
    (164, 0x9a74cc), // Playoff Second Leg
    (170, 0x9a74bc), // Playoff Final
    (171, 0x9a74a4), // Playoff Final Replay
    (172, 0x9a7488), // Playoff Final Second Replay
    (173, 0x9a7470), // Playoff Final First Leg
    (174, 0x9a7454), // Playoff Final Second Leg
    (180, 0x9a7440), // Playoff Semi Final
    (181, 0x9a7424), // Playoff Semi Final Replay
    (182, 0x9a7400), // Playoff Semi Final Second Replay
    (183, 0x9a73e0), // Playoff Semi Final First Leg
    (184, 0x9a73c0), // Playoff Semi Final Second Leg
    (190, 0x9a73b8), // Playout
    (191, 0x9a73a8), // Playout Replay
    (192, 0x9a7390), // Playout Second Replay
    (193, 0x9a737c), // Playout First Leg
    (194, 0x9a7368), // Playout Second Leg
    (200, 0x9a7350), // First Preliminary Round
    (201, 0x9a7330), // First Preliminary Round Replay
    (202, 0x9a7308), // First Preliminary Round Second Replay
    (203, 0x9a72e4), // First Preliminary Round First Leg
    (204, 0x9a72c0), // First Preliminary Round Second Leg
    (210, 0x9a72a4), // Second Preliminary Round
    (211, 0x9a7284), // Second Preliminary Round Replay
    (212, 0x9a725c), // Second Preliminary Round Second Replay
    (213, 0x9a7238), // Second Preliminary Round First Leg
    (214, 0x9a7214), // Second Preliminary Round Second Leg
    (220, 0x9a7200), // Qualifying Round
    (221, 0x9a71e8), // Qualifying Round Replay
    (222, 0x9a71c8), // Qualifying Round Second Replay
    (223, 0x9a71ac), // Qualifying Round First Leg
    (224, 0x9a7190), // Qualifying Round Second Leg
    (230, 0x9a717c), // Preliminary Round
    (231, 0x9a7160), // Preliminary Round Replay
    (232, 0x9a7140), // Preliminary Round Second Replay
    (233, 0x9a7124), // Preliminary Round First Leg
    (234, 0x9a7104), // Preliminary Round Second Leg
    (240, 0x9a70f4), // Promotion Final
    (243, 0x9a70d8), // Promotion Final First Leg
    (244, 0x9a70bc), // Promotion Final Second Leg
    (250, 0x9a70a4), // First Qualifying Phase
    (253, 0x9a7080), // First Qualifying Phase First Leg
    (254, 0x9a705c), // First Qualifying Phase Second Leg
    (260, 0x9a7044), // Second Qualifying Phase
    (263, 0x9a7020), // Second Qualifying Phase First Leg
    (264, 0x9a6ffc), // Second Qualifying Phase Second Leg
    (270, 0x9a6fe4), // Third Qualifying Phase
    (273, 0x9a6fc0), // Third Qualifying Phase First Leg
    (274, 0x9a6f9c), // Third Qualifying Phase Second Leg
    (280, 0x9a6f90), // Playoff 1
    (283, 0x9a6f7c), // Playoff 1 First Leg
    (284, 0x9a6f64), // Playoff 1 Second Leg
    (290, 0x9a6f58), // Playoff 2
    (293, 0x9a6f44), // Playoff 2 First Leg
    (294, 0x9a6f2c), // Playoff 2 Second Leg
    (300, 0x9a6f1c), // North Playoff
    (310, 0x9a7604), // Third Place Playoff
    (311, 0x9a75e8), // Third Place Playoff Replay
    (312, 0x9a75c4), // Third Place Playoff Second Replay
    (313, 0x9a75a4), // Third Place Playoff First Leg
    (314, 0x9a7584), // Third Place Playoff Second Leg
    (320, 0x9a6f08), // Elimination Final
    (323, 0x9a6eec), // Elimination Final First Leg
    (324, 0x9a6ecc), // Elimination Final Second Leg
    (330, 0x9a6eb8), // Minor Semi Final
    (333, 0x9a6e9c), // Minor Semi Final First Leg
    (334, 0x9a6e80), // Minor Semi Final Second Leg
    (340, 0x9a6e6c), // Major Semi Final
    (343, 0x9a6e50), // Major Semi Final First Leg
    (344, 0x9a6e34), // Major Semi Final Second Leg
    (350, 0x9a6e20), // Preliminary Final
    (353, 0x9a6e04), // Preliminary Final First Leg
    (354, 0x9a6de4), // Preliminary Final Second Leg
    (360, 0x9a6dd8), // Grand Final
    (363, 0x9a6dc0), // Grand Final First Leg
    (364, 0x9a6da8), // Grand Final Second Leg
    (370, 0x9a6d94), // Interzone Playoff
    (373, 0x9a6d78), // Interzone Playoff First Leg
    (374, 0x9a6d58), // Interzone Playoff Second Leg
    (1000, 0x9a6d50), // Group 1
    (1001, 0x9a6d48), // Group 2
    (1002, 0x9a6d40), // Group 3
    (1003, 0x9a6d38), // Group 4
    (1004, 0x9a6d30), // Group 5
    (1005, 0x9a6d28), // Group 6
    (1006, 0x9a6d20), // Group 7
    (1007, 0x9a6d18), // Group 8
    (1008, 0x9a6d10), // Group 9
    (1009, 0x9a6d04), // Group 10
    (1010, 0x9a6cf8), // Group 11
    (1011, 0x9a6cec), // Group 12
    (1012, 0x9a6ce0), // Group 13
    (1013, 0x9a6cd4), // Group 14
    (1014, 0x9a6cc8), // Group 15
    (1015, 0x9a6cbc), // Group 16
    (1016, 0x9a6cb0), // Group 17
    (1017, 0x9a6ca4), // Group 18
    (1018, 0x9a6c98), // Group 19
    (1019, 0x9a6c8c), // Group 20
    (1020, 0x9a6c78), // Second Placed Teams
    (1021, 0x9a6c64), // First Round Group 1
    (1022, 0x9a6c50), // First Round Group 2
    (1023, 0x9a6c3c), // First Round Group 3
    (1024, 0x9a6c28), // First Round Group 4
    (1025, 0x9a6c14), // First Round Group 5
    (1026, 0x9a6c00), // First Round Group 6
    (1027, 0x9a6bec), // First Round Group 7
    (1028, 0x9a6bd8), // First Round Group 8
    (1029, 0x9a6bc4), // First Round Group 9
    (1030, 0x9a6bac), // First Round Group 10
    (1031, 0x9a6b94), // Second Round Group 1
    (1032, 0x9a6b7c), // Second Round Group 2
    (1033, 0x9a6b64), // Second Round Group 3
    (1034, 0x9a6b4c), // Second Round Group 4
    (1035, 0x9a6b34), // Second Round Group 5
    (1036, 0x9a6b20), // Semi Final Group 1
    (1037, 0x9a6b0c), // Semi Final Group 2
    (1038, 0x9a6af8), // Semi Final Group 3
    (1039, 0x9a6ae8), // Carribean Zone
    (1040, 0x9a6ad0), // Central American Zone
    (1041, 0x9a6ab8), // Premier/First(South)
    (1042, 0x9a6aa0), // Premier/First(North)
    (1043, 0x9a6a8c), // Eastern Conference
    (1044, 0x9a6a78), // Western Conference
    (1045, 0x9a6a5c), // Eastern Conference Playoff
    (1046, 0x9a6a40), // Western Conference Playoff
    (1047, 0x9a6a38), // MLS Cup
    (1048, 0x9a6a28), // Playoff Group 1
    (1049, 0x9a6a18), // Playoff Group 2
    (1050, 0x9a6a08), // Autumn Season
    (1051, 0x9a69f8), // Spring Season
    (1052, 0x99b800), // Group Stage
    (1053, 0x9a69e4), // Northern Section
    (1054, 0x9a69d0), // Southern Section
    (1055, 0x9a69c8), // Group A
    (1056, 0x9a69c0), // Group B
    (1057, 0x9a69b8), // Group C
    (1058, 0x9a69b0), // Group D
    (1059, 0x9a69a8), // Group E
    (1060, 0x9a69a0), // Group F
    (1061, 0x9a6998), // Group G
    (1062, 0x9a6990), // Group H
    (1063, 0x9a6988), // Group I
    (1064, 0x9a6980), // Group J
    (1065, 0x9a6978), // Group K
    (1066, 0x9a6970), // Group L
    (1067, 0x9a6968), // Group M
    (1068, 0x9a6960), // Group N
    (1069, 0x9a6958), // Group O
    (1070, 0x9a6950), // Group P
    (1071, 0x9a6938), // Oceania/Asia Playoff
    (1072, 0x9a6924), // First(South)/Second
    (1073, 0x9a6910), // First(North)/Second
    (1074, 0x9a6904), // First Stage
    (1075, 0x9a68f4), // Second Stage
    (1076, 0x9a68e4), // Opening Stage
    (1077, 0x9a68d4), // Closing Stage
    (1078, 0x9a68c4), // Interior Zone
    (1079, 0x9a68b0), // Metropolitan Zone
    (1080, 0x9a68a4), // Promotion
    (1081, 0x9a6894), // First Period
    (1082, 0x9a6884), // Second Period
    (1083, 0x9a6874), // Third Period
    (1084, 0x9a6864), // Fourth Period
    (1085, 0x9a6854), // Phase 1 Group A
    (1086, 0x9a6844), // Phase 1 Group B
    (1087, 0x9a6834), // Phase 1 Group C
    (1088, 0x9a6824), // Phase 1 Group D
    (1089, 0x9a6814), // Phase 1 Group E
    (1090, 0x9a6804), // Phase 1 Group F
    (1091, 0x9a67f4), // Phase 1 Group G
    (1092, 0x9a67e4), // Phase 1 Group H
    (1093, 0x9a67d4), // Phase 2 Group A
    (1094, 0x9a67c4), // Phase 2 Group B
    (1095, 0x9a67b4), // Phase 2 Group C
    (1096, 0x9a67a4), // Phase 2 Group D
    (1097, 0x9a6790), // Phase 1 Group Stage
    (1098, 0x9a677c), // Phase 2 Group Stage
    (1099, 0x9a6768), // Championship Group
    (1100, 0x9a6754), // Relegation Group
    (1101, 0x9a6740), // Promotion Group A
    (1102, 0x9a672c), // Promotion Group B
    (1103, 0x9a6718), // Promotion Group C
    (1104, 0x9a6704), // Promotion Group D
    (1105, 0x9a66ec), // Promotion Group Stage
    (1106, 0x9a66d8), // Relegation Playoff
    (1107, 0x9a66d0), // North
    (1108, 0x9a66c4), // Northeast
    (1109, 0x9a66b4), // West/Southwest
    (1110, 0x9a66ac), // South
    (1111, 0x9a6684), // Central<%s - COMMENT - Central section>
    (1112, 0x9a6670), // Promotion Playoff
    (1113, 0x9a6660), // First Promotion
    (1114, 0x9a664c), // Second Promotion
    (1115, 0x9a663c), // Third Promotion
    (1116, 0x9a6628), // Fourth Promotion
    (1117, 0x9a6610), // Group A First Period
    (1118, 0x9a65f8), // Group A Second Period
    (1119, 0x9a65e0), // Group A Third Period
    (1120, 0x9a65c8), // Group B First Period
    (1121, 0x9a65b0), // Group B Second Period
    (1122, 0x9a6598), // Group B Third Period
    (1123, 0x9a6d50), // Group 1
    (1124, 0x9a6d48), // Group 2
    (1125, 0x9a6d40), // Group 3
    (1126, 0x9a6d38), // Group 4
    (1127, 0x9a6d30), // Group 5
    (1128, 0x9a6d28), // Group 6
    (1129, 0x9a6d20), // Group 7
    (1130, 0x9a6d18), // Group 8
    (1131, 0x9a5358), // Championship
    (1132, 0x9a66d0), // North
    (1133, 0x9a6588), // East Svealand
    (1134, 0x9a6578), // West Svealand
    (1135, 0x9a6568), // East Gotaland
    (1136, 0x9a6558), // West Gotaland
    (1137, 0x9a6548), // South Gotaland
    (1138, 0x9a6530), // Second Round Group 6
    (1139, 0x9a6510), // Central American Zone Group 1
    (1140, 0x9a64f0), // Central American Zone Group 2
    (1141, 0x9a64d8), // Classification Group 1
    (1142, 0x9a64c0), // Classification Group 2
    (1143, 0x9a64a8), // Classification Group 3
    (1144, 0x9a6490), // Classification Group 4
    (1145, 0x9a6478), // Classification Group 5
    (1146, 0x9a6460), // Classification Groups
    (1147, 0x9a644c), // Central Conference
    (1148, 0x9a6430), // Northern Relegation Section
    (1149, 0x9a6414), // Southern Relegation Section
    (1150, 0x9a6400), // Third Round Group A
    (1151, 0x9a63ec), // Third Round Group B
    (1152, 0x9a4094), // East
    (1153, 0x9a408c), // West
    (1154, 0x9a63d8), // East Quarter Final
    (1155, 0x9a63c4), // West Quarter Final
];

/// The short names of the round, group and stage ids (form 1 of `comp_util_get_stage_name`): id and the langlib format
/// string the original prints, sorted by id (the text in the comment).
const STAGE_NAMES_SHORT: [(i16, usize); 322] = [
    (1, 0x9a5314), // Rep<%s - COMMENT - replay>
    (2, 0x9a5314), // Rep<%s - COMMENT - replay>
    (3, 0x9a530c), // Leg 1
    (4, 0x9a5304), // Leg 2
    (5, 0x9a7c10), // Decider
    (10, 0x9a52fc), // 1st Rnd
    (11, 0x9a52ec), // 1st Rnd Replay
    (12, 0x9a52dc), // 1st Rnd Rep 2
    (13, 0x9a52cc), // 1st Rnd Leg 1
    (14, 0x9a52bc), // 1st Rnd Leg 2
    (20, 0x9a52b4), // 2nd Rnd
    (21, 0x9a52a4), // 2nd Rnd Replay
    (22, 0x9a5290), // 2nd Rnd Replay 2
    (23, 0x9a5280), // 2nd Rnd Leg 1
    (24, 0x9a5270), // 2nd Rnd Leg 2
    (25, 0x9a5260), // 2nd Rnd Decider
    (30, 0x9a5258), // 3rd Rnd
    (31, 0x9a5248), // 3rd Rnd Replay
    (32, 0x9a5234), // 3rd Rnd Replay 2
    (33, 0x9a5224), // 3rd Rnd Leg 1
    (34, 0x9a5214), // 3rd Rnd Leg 2
    (40, 0x9a520c), // 4th Rnd
    (41, 0x9a51fc), // 4th Rnd Replay
    (42, 0x9a51e8), // 4th Rnd Replay 2
    (43, 0x9a51d8), // 4th Rnd Leg 1
    (44, 0x9a51c8), // 4th Rnd Leg 2
    (50, 0x9a51a8), // 5th Rnd
    (51, 0x9a5198), // 5th Rnd Replay
    (52, 0x9a5184), // 5th Rnd Replay 2
    (53, 0x9a5174), // 5th Rnd Leg 1
    (54, 0x9a5164), // 5th Rnd Leg 2
    (60, 0x9a515c), // 6th Rnd
    (61, 0x9a514c), // 6th Rnd Replay
    (62, 0x9a5138), // 6th Rnd Replay 2
    (63, 0x9a512c), // 6th Rnd L1
    (64, 0x9a5120), // 6th Rnd L2
    (70, 0x9a5114), // 7th Round
    (71, 0x9a5104), // 7th Rnd Replay
    (72, 0x9a50f0), // 7th Rnd Replay 2
    (73, 0x9a50e0), // 7th Rnd Leg 1
    (74, 0x9a50d0), // 7th Rnd Leg 2
    (80, 0x9a50c8), // 8th Rnd
    (81, 0x9a50b8), // 8th Rnd Replay
    (82, 0x9a50a4), // 8th Rnd Replay 2
    (83, 0x9a5094), // 8th Rnd Leg 1
    (84, 0x9a5084), // 8th Rnd Leg 2
    (90, 0x9a507c), // 9th Rnd
    (91, 0x9a506c), // 9th Rnd Replay
    (92, 0x9a5058), // 9th Rnd Replay 2
    (93, 0x9a5048), // 9th Rnd Leg 1
    (94, 0x9a5038), // 9th Rnd Leg 2
    (100, 0x9a502c), // 10th Rnd
    (101, 0x9a501c), // 10th Rnd Replay
    (102, 0x9a5008), // 10th Rnd Replay 2
    (103, 0x9a4ff8), // 10th Rnd Leg 1
    (104, 0x9a4fe8), // 10th Rnd Leg 2
    (110, 0x9a4fdc), // 11th Rnd
    (111, 0x9a4fcc), // 11th Rnd Replay
    (112, 0x9a4fb8), // 11th Rnd Replay 2
    (113, 0x9a4fa8), // 11th Rnd Leg 1
    (114, 0x9a4f98), // 11th Rnd Leg 2
    (120, 0x9a4f8c), // Qtr Final
    (121, 0x9a4f78), // Qtr Final Replay
    (122, 0x9a4f64), // Qtr Final Replay 2
    (123, 0x9a4f54), // Qtr Final Leg 1
    (124, 0x9a4f44), // Qtr Final Leg 2
    (125, 0x9a4f30), // Qtr Final Decider
    (130, 0x9a768c), // Semi Final
    (131, 0x9a7678), // Semi Final Replay
    (132, 0x9a4f1c), // Semi Final Replay 2
    (133, 0x9a4f08), // Semi Final Leg 1
    (134, 0x9a4ef4), // Semi Final Leg 2
    (135, 0x9a7618), // Semi Final Decider
    (140, 0x9a4ee0), // 3rd Place Playoff
    (141, 0x9a4ecc), // 3rd Place Replay
    (142, 0x9a4eb8), // 3rd Place Replay 2
    (143, 0x9a4ea4), // 3rd Place Pl. Leg 1
    (144, 0x9a4e90), // 3rd Place Pl. Leg 2
    (150, 0x9a757c), // Final
    (151, 0x9a756c), // Final Replay
    (152, 0x9a4e80), // Final Replay 1
    (153, 0x9a4e74), // Final Leg 1
    (154, 0x9a4e68), // Final Leg 2
    (155, 0x9a7524), // Final Decider
    (160, 0x9a751c), // Playoff
    (161, 0x9a750c), // Playoff Replay
    (162, 0x9a4e54), // Playoff Replay 2
    (163, 0x9a4e44), // Playoff Leg 1
    (164, 0x9a4e34), // Playoff Leg 2
    (170, 0x9a4e28), // Pl. Final
    (171, 0x9a4e14), // Pl. Final Replay
    (172, 0x9a4e00), // Pl. Final Replay 2
    (173, 0x9a4df0), // Pl. Final Leg 1
    (174, 0x9a4de0), // Pl. Final Leg 2
    (180, 0x9a4dd0), // Pl. Semi Final
    (181, 0x9a4dbc), // Pl. Semi Final Rep
    (182, 0x9a4da4), // Pl. Semi Final Rep 2
    (183, 0x9a4d8c), // Pl. Semi Final Leg 1
    (184, 0x9a4d74), // Pl. Semi Final Leg 2
    (190, 0x9a73b8), // Playout
    (191, 0x9a73a8), // Playout Replay
    (192, 0x9a4d60), // Playout Replay 2
    (193, 0x9a4d50), // Playout Leg 1
    (194, 0x9a4d40), // Playout Leg 2
    (200, 0x9a4d30), // 1st Pre. Rnd
    (201, 0x9a4d1c), // 1st Pre. Rnd Replay
    (202, 0x9a4d04), // 1st Pre. Rnd Replay 2
    (203, 0x9a4cf0), // 1st Pre. Rnd Leg 1
    (204, 0x9a4cdc), // 1st Pre. Rnd Leg 2
    (210, 0x9a4ccc), // 2nd Pre. Rnd
    (211, 0x9a4cb8), // 2nd Pre. Rnd Replay
    (212, 0x9a4ca0), // 2nd Pre. Rnd Replay 2
    (213, 0x9a4c8c), // 2nd Pre. Rnd Leg 1
    (214, 0x9a4c78), // 2nd Pre. Rnd Leg 2
    (220, 0x9a4c6c), // Qual.Rnd
    (221, 0x9a4c5c), // Qual.Rnd Replay
    (222, 0x9a4c48), // Qual.Rnd Replay 2
    (223, 0x9a4c38), // Qual. Rnd Leg 1
    (224, 0x9a4c28), // Qual. Rnd Leg 2
    (230, 0x9a4c1c), // Prelim Rnd
    (231, 0x9a4c08), // Prelim Rnd Replay
    (232, 0x9a4bf4), // Prelim Rnd Replay 2
    (233, 0x9a4be0), // Prelim Rnd Leg 1
    (234, 0x9a4bcc), // Prelim Rnd Leg 2
    (240, 0x9a4bc0), // Pro. Final
    (243, 0x9a4bac), // Pro. Final Leg 1
    (244, 0x9a4b98), // Pro. Final Leg 2
    (250, 0x9a4b90), // Qual.1
    (253, 0x9a4b80), // Qual.1 Leg 1
    (254, 0x9a4b70), // Qual.1 Leg 2
    (260, 0x9a4b68), // Qual.2
    (263, 0x9a4b58), // Qual.2 Leg 1
    (264, 0x9a4b48), // Qual.2 Leg 2
    (270, 0x9a4b40), // Qual.3
    (273, 0x9a4b30), // Qual.3 Leg 1
    (274, 0x9a4b20), // Qual.3 Leg 2
    (280, 0x9a4af4), // Pl. 1<%s - COMMENT - Playoff 1 short name>
    (283, 0x9a4ac0), // Pl. 1 Leg 1<%s - COMMENT - Playoff 1 short name>
    (284, 0x9a4a8c), // Pl. 1 Leg 2<%s - COMMENT - Playoff 1 short name>
    (290, 0x9a4a60), // Pl. 2<%s - COMMENT - Playoff 2 short name>
    (293, 0x9a4a2c), // Pl. 2 Leg 1<%s - COMMENT - Playoff 2 short name>
    (294, 0x9a49f8), // Pl. 2 Leg 2<%s - COMMENT - Playoff 2 short name>
    (300, 0x9a49d0), // North Pl.<%s - COMMENT - North Playoff>
    (310, 0x9a4ee0), // 3rd Place Playoff
    (311, 0x9a4ecc), // 3rd Place Replay
    (312, 0x9a4eb8), // 3rd Place Replay 2
    (313, 0x9a4ea4), // 3rd Place Pl. Leg 1
    (314, 0x9a4e90), // 3rd Place Pl. Leg 2
    (320, 0x9a49c4), // Elim. Final
    (323, 0x9a49b0), // Elim. Final Leg 1
    (324, 0x9a499c), // Elim. Final Leg 2
    (330, 0x9a6eb8), // Minor Semi Final
    (333, 0x9a4984), // Minor Semi Final Leg 1
    (334, 0x9a496c), // Minor Semi Final Leg 2
    (340, 0x9a6e6c), // Major Semi Final
    (343, 0x9a4954), // Major Semi Final Leg 1
    (344, 0x9a493c), // Major Semi Final Leg 2
    (350, 0x9a492c), // Prelim. Final
    (353, 0x9a4918), // Prelim. Final Leg 1
    (354, 0x9a4904), // Prelim. Final Leg 2
    (360, 0x9a6dd8), // Grand Final
    (363, 0x9a48f0), // Grand Final Leg 1
    (364, 0x9a48dc), // Grand Final Leg 2
    (370, 0x9a48cc), // Interzone Pl.
    (373, 0x9a48b8), // Interzone Pl. Leg 1
    (374, 0x9a48a4), // Interzone Pl. Leg 2
    (1000, 0x9a489c), // Grp 1
    (1001, 0x9a4894), // Grp 2
    (1002, 0x9a488c), // Grp 3
    (1003, 0x9a4884), // Grp 4
    (1004, 0x9a487c), // Grp 5
    (1005, 0x9a4874), // Grp 6
    (1006, 0x9a486c), // Grp 7
    (1007, 0x9a4864), // Grp 8
    (1008, 0x9a485c), // Grp 9
    (1009, 0x9a4854), // Grp 10
    (1010, 0x9a484c), // Grp 11
    (1011, 0x9a4844), // Grp 12
    (1012, 0x9a483c), // Grp 13
    (1013, 0x9a4834), // Grp 14
    (1014, 0x9a482c), // Grp 15
    (1015, 0x9a4824), // Grp 16
    (1016, 0x9a481c), // Grp 17
    (1017, 0x9a4814), // Grp 18
    (1018, 0x9a480c), // Grp 19
    (1019, 0x9a4804), // Grp 20
    (1020, 0x9a6c78), // Second Placed Teams
    (1021, 0x9a47f4), // 1st Rnd Grp 1
    (1022, 0x9a47e4), // 1st Rnd Grp 2
    (1023, 0x9a47d4), // 1st Rnd Grp 3
    (1024, 0x9a47c4), // 1st Rnd Grp 4
    (1025, 0x9a47b4), // 1st Rnd Grp 5
    (1026, 0x9a47a4), // 1st Rnd Grp 6
    (1027, 0x9a4794), // 1st Rnd Grp 7
    (1028, 0x9a4784), // 1st Rnd Grp 8
    (1029, 0x9a4774), // 1st Rnd Grp 9
    (1030, 0x9a4764), // 1st Rnd Grp 10
    (1031, 0x9a4754), // 2nd Rnd Grp 1
    (1032, 0x9a4744), // 2nd Rnd Grp 2
    (1033, 0x9a4734), // 2nd Rnd Grp 3
    (1034, 0x9a4724), // 2nd Rnd Grp 4
    (1035, 0x9a4714), // 2nd Rnd Grp 5
    (1036, 0x9a4700), // Semi Final Grp 1
    (1037, 0x9a46ec), // Semi Final Grp 2
    (1038, 0x9a46d8), // Semi Final Grp 3
    (1039, 0x9a46cc), // Carr. Zone
    (1040, 0x9a46bc), // Cent.Amer.Zone
    (1041, 0x9a751c), // Playoff
    (1042, 0x9a751c), // Playoff
    (1043, 0x9a46b0), // East Conf.
    (1044, 0x9a46a4), // West Conf.
    (1045, 0x9a4690), // East Conf. Playoff
    (1046, 0x9a467c), // West Conf. Playoff
    (1047, 0x9a6a38), // MLS Cup
    (1048, 0x9a466c), // Playoff Grp 1
    (1049, 0x9a465c), // Playoff Grp 2
    (1050, 0x9a4654), // Autumn
    (1051, 0x9a464c), // Spring
    (1052, 0x99b800), // Group Stage
    (1053, 0x9a66d0), // North
    (1054, 0x9a66ac), // South
    (1055, 0x9a4644), // Grp A
    (1056, 0x9a463c), // Grp B
    (1057, 0x9a4634), // Grp C
    (1058, 0x9a462c), // Grp D
    (1059, 0x9a4624), // Grp E
    (1060, 0x9a461c), // Grp F
    (1061, 0x9a4614), // Grp G
    (1062, 0x9a460c), // Grp H
    (1063, 0x9a4604), // Grp I
    (1064, 0x9a45fc), // Grp J
    (1065, 0x9a45f4), // Grp K
    (1066, 0x9a45ec), // Grp L
    (1067, 0x9a45e4), // Grp M
    (1068, 0x9a45dc), // Grp N
    (1069, 0x9a45d4), // Grp O
    (1070, 0x9a45cc), // Grp P
    (1071, 0x9a751c), // Playoff
    (1072, 0x9a6924), // First(South)/Second
    (1073, 0x9a6910), // First(North)/Second
    (1074, 0x9a45c0), // 1st Stage
    (1075, 0x9a45b4), // 2nd Stage
    (1076, 0x9a45ac), // Open
    (1077, 0x9a45a4), // Close
    (1078, 0x9a459c), // Int.
    (1079, 0x9a4594), // Metro.
    (1080, 0x9a458c), // Pro.
    (1081, 0x9a4580), // Period 1
    (1082, 0x9a4574), // Period 2
    (1083, 0x9a4568), // Period 3
    (1084, 0x9a455c), // Period 4
    (1085, 0x9a4550), // Ph.1 Grp A
    (1086, 0x9a4544), // Ph.1 Grp B
    (1087, 0x9a4538), // Ph.1 Grp C
    (1088, 0x9a452c), // Ph.1 Grp D
    (1089, 0x9a4520), // Ph.1 Grp E
    (1090, 0x9a4514), // Ph.1 Grp F
    (1091, 0x9a4508), // Ph.1 Grp G
    (1092, 0x9a44fc), // Ph.1 Grp H
    (1093, 0x9a44f0), // Ph.2 Grp A
    (1094, 0x9a44e4), // Ph.2 Grp B
    (1095, 0x9a44d8), // Ph.2 Grp C
    (1096, 0x9a44cc), // Ph.2 Grp D
    (1097, 0x9a44c4), // Phase 1
    (1098, 0x9a44bc), // Phase 2
    (1099, 0x9a44b0), // Champ. Grp
    (1100, 0x9a44a4), // Releg. Grp
    (1101, 0x9a4498), // Pro.Grp A
    (1102, 0x9a448c), // Pro.Grp B
    (1103, 0x9a4480), // Pro.Grp C
    (1104, 0x9a4474), // Pro.Grp D
    (1105, 0x9a4460), // Pro. Group Stage
    (1106, 0x9a4434), // Rel. Pl<%s - COMMENT - Relegation playoff>
    (1107, 0x9a421c), // N<%s - COMMENT - North>
    (1108, 0x9a4414), // NE<%s - COMMENT - Northeast>
    (1109, 0x9a43f0), // W/SW<%s - COMMENT - West/Southwest>
    (1110, 0x9a43d8), // S<%s - COMMENT - South>
    (1111, 0x9a43a4), // Central<%s - COMMENT - Central section(short name)>
    (1112, 0x9a4394), // Pro. Playoff
    (1113, 0x9a4360), // Pro, 1<%s - COMMENT - First Promotion short name>
    (1114, 0x9a432c), // Pro, 2<%s - COMMENT - First Promotion short name>
    (1115, 0x9a42f8), // Pro, 3<%s - COMMENT - First Promotion short name>
    (1116, 0x9a42c4), // Pro, 4<%s - COMMENT - First Promotion short name>
    (1117, 0x9a42b4), // Grp A Period 1
    (1118, 0x9a42a4), // Grp A Period 2
    (1119, 0x9a4294), // Grp A Period 3
    (1120, 0x9a4284), // Grp B Period 1
    (1121, 0x9a4274), // Grp B Period 2
    (1122, 0x9a4264), // Grp B Period 3
    (1123, 0x9a489c), // Grp 1
    (1124, 0x9a4894), // Grp 2
    (1125, 0x9a488c), // Grp 3
    (1126, 0x9a4884), // Grp 4
    (1127, 0x9a487c), // Grp 5
    (1128, 0x9a4874), // Grp 6
    (1129, 0x9a486c), // Grp 7
    (1130, 0x9a4864), // Grp 8
    (1131, 0x9a4234), // Champ.<%s - COMMENT - Championship short name>
    (1132, 0x9a421c), // N<%s - COMMENT - North>
    (1133, 0x9a41f8), // E.S<%s - COMMENT - East Svealand>
    (1134, 0x9a41d4), // W.S<%s - COMMENT - West Svealand>
    (1135, 0x9a41b0), // E.G<%s - COMMENT - East Gotaland>
    (1136, 0x9a418c), // W.G<%s - COMMENT - West Gotaland>
    (1137, 0x9a4168), // S.G<%s - COMMENT - South Gotaland>
    (1138, 0x9a4158), // 2nd Rnd Grp 6
    (1139, 0x9a4140), // Cent.Amer.Zone Grp 1
    (1140, 0x9a4128), // Cent.Amer.Zone Grp 2
    (1141, 0x9a411c), // Class.Grp 1
    (1142, 0x9a4110), // Class.Grp 2
    (1143, 0x9a4104), // Class.Grp 3
    (1144, 0x9a40f8), // Class.Grp 4
    (1145, 0x9a40ec), // Class.Grp 5
    (1146, 0x9a40e0), // Class.Grps
    (1147, 0x9a40d4), // Cent. Conf.
    (1148, 0x9a40c8), // North Rel.
    (1149, 0x9a40bc), // South Rel.
    (1150, 0x9a40ac), // 3rd Rnd Grp A
    (1151, 0x9a409c), // 3rd Rnd Grp B
    (1152, 0x9a4094), // East
    (1153, 0x9a408c), // West
    (1154, 0x9a407c), // E. Qtr Final
    (1155, 0x9a406c), // W. Qtr Final
];

/// The plural names of the round, group and stage ids (form 3 of `comp_util_get_stage_name`): id and the langlib format
/// string the original prints, sorted by id (the text in the comment).
const STAGE_NAMES_PLURAL: [(i16, usize); 321] = [
    (1, 0x9a7c3c), // Replay
    (2, 0x9a7c30), // Replay 2
    (3, 0x9a63b8), // First Legs
    (4, 0x9a63ac), // Second Legs
    (5, 0x9a7c10), // Decider
    (10, 0x9a6384), // First Round<%s - Plural, arg not used>
    (11, 0x9a6370), // First Round Replays
    (12, 0x9a6354), // First Round Second Replays
    (13, 0x9a633c), // First Round First Legs
    (14, 0x9a6324), // First Round Second Legs
    (20, 0x9a62fc), // Second Round<%s - Plural, arg not used>
    (21, 0x9a62e4), // Second Round Replays
    (22, 0x9a62c8), // Second Round Second Replays
    (23, 0x9a62b0), // Second Round First Legs
    (24, 0x9a6294), // Second Round Second Legs
    (25, 0x9a627c), // Second Round Deciders
    (30, 0x9a6254), // Third Round<%s - Plural, arg not used>
    (31, 0x9a6240), // Third Round Replays
    (32, 0x9a6224), // Third Round Second Replays
    (33, 0x9a620c), // Third Round First Legs
    (34, 0x9a61f4), // Third Round Second Legs
    (40, 0x9a61cc), // Fourth Round<%s - Plural, arg not used>
    (41, 0x9a61b4), // Fourth Round Replays
    (42, 0x9a6198), // Fourth Round Second Replays
    (43, 0x9a6180), // Fourth Round First Legs
    (44, 0x9a6164), // Fourth Round Second Legs
    (50, 0x9a613c), // Fifth Round<%s - Plural, arg not used>
    (51, 0x9a6128), // Fifth Round Replays
    (52, 0x9a610c), // Fifth Round Second Replays
    (53, 0x9a60f4), // Fifth Round First Legs
    (54, 0x9a60dc), // Fifth Round Second Legs
    (60, 0x9a60b4), // Sixth Round<%s - Plural, arg not used>
    (61, 0x9a60a0), // Sixth Round Replays
    (62, 0x9a6084), // Sixth Round Second Replays
    (63, 0x9a606c), // Sixth Round First Legs
    (64, 0x9a6054), // Sixth Round Second Legs
    (70, 0x9a6028), // Seventh Round<%s - Plural, arg not used>
    (71, 0x9a6010), // Seventh Round Replays
    (72, 0x9a5ff0), // Seventh Round Second Replays
    (73, 0x9a5fd4), // Seventh Round First Legs
    (74, 0x9a5fb8), // Seventh Round Second Legs
    (80, 0x9a5f90), // Eighth Round<%s - Plural, arg not used>
    (81, 0x9a5f78), // Eighth Round Replays
    (82, 0x9a5f5c), // Eighth Round Second Replays
    (83, 0x9a5f44), // Eighth Round First Legs
    (84, 0x9a5f28), // Eighth Round Second Legs
    (90, 0x9a5f00), // Ninth Round<%s - Plural, arg not used>
    (91, 0x9a5eec), // Ninth Round Replays
    (92, 0x9a5ed0), // Ninth Round Second Replays
    (93, 0x9a5eb8), // Ninth Round First Legs
    (94, 0x9a5ea0), // Ninth Round Second Legs
    (100, 0x9a5e78), // Tenth Round<%s - Plural, arg not used>
    (101, 0x9a5e64), // Tenth Round Replays
    (102, 0x9a5e48), // Tenth Round Second Replays
    (103, 0x9a5e30), // Tenth Round First Legs
    (104, 0x9a5e18), // Tenth Round Second Legs
    (110, 0x9a5dec), // Eleventh Round<%s - Plural, arg not used>
    (111, 0x9a5dd4), // Eleventh Round Replays
    (112, 0x9a5db4), // Eleventh Round Second Replays
    (113, 0x9a5d98), // Eleventh Round First Legs
    (114, 0x9a5d7c), // Eleventh Round Second Legs
    (120, 0x9a5d6c), // Quarter Finals
    (121, 0x9a5d54), // Quarter Final Replays
    (122, 0x9a5d34), // Quarter Final Second Replays
    (123, 0x9a5d18), // Quarter Final First Legs
    (124, 0x9a5cfc), // Quarter Final Second Legs
    (125, 0x9a5ce4), // Quarter Final Deciders
    (130, 0x9a5cd8), // Semi Finals
    (131, 0x9a5cc4), // Semi Final Replays
    (132, 0x9a5ca8), // Semi Final Second Replays
    (133, 0x9a5c90), // Semi Final First Legs
    (134, 0x9a5c78), // Semi Final Second Legs
    (135, 0x9a5c64), // Semi Final Deciders
    (140, 0x9a5c4c), // Third Place Playoffs
    (141, 0x9a5c30), // Third Place Playoff Replays
    (142, 0x9a5c0c), // Third Place Playoff Second Replays
    (143, 0x9a5bec), // Third Place Playoff First Legs
    (144, 0x9a5bcc), // Third Place Playoff Second Legs
    (150, 0x9a5bc4), // Finals
    (151, 0x9a5bb4), // Final Replays
    (152, 0x9a5b9c), // Final Second Replays
    (153, 0x9a5b88), // Final First Legs
    (154, 0x9a5b74), // Final Second Legs
    (155, 0x9a5b64), // Final Deciders
    (160, 0x9a5b58), // Playoffs
    (161, 0x9a5b48), // Playoff Replays
    (162, 0x9a5b30), // Playoff Second Replays
    (163, 0x9a5b1c), // Playoff First Legs
    (164, 0x9a5b08), // Playoff Second Legs
    (170, 0x9a5af8), // Playoff Finals
    (171, 0x9a5ae0), // Playoff Final Replays
    (172, 0x9a5ac0), // Playoff Final Second Replays
    (173, 0x9a5aa4), // Playoff Final First Legs
    (174, 0x9a5a88), // Playoff Final Second Legs
    (180, 0x9a5a74), // Playoff Semi Finals
    (181, 0x9a5a58), // Playoff Semi Final Replays
    (182, 0x9a5a34), // Playoff Semi Final Second Replays
    (183, 0x9a5a14), // Playoff Semi Final First Legs
    (184, 0x9a59f4), // Playoff Semi Final Second Legs
    (190, 0x9a59e8), // Playouts
    (191, 0x9a59d8), // Playout Replays
    (192, 0x9a59c0), // Playout Second Replays
    (193, 0x9a59ac), // Playout First Legs
    (194, 0x9a5998), // Playout Second Legs
    (200, 0x9a5964), // First Preliminary Round<%s - Plural, arg not used>
    (201, 0x9a5944), // First Preliminary Round Replays
    (202, 0x9a591c), // First Preliminary Round Second Replays
    (203, 0x9a58f8), // First Preliminary Round First Legs
    (204, 0x9a58d4), // First Preliminary Round Second Legs
    (210, 0x9a58a0), // Second Preliminary Round<%s - Plural, arg not used>
    (211, 0x9a587c), // Second Preliminary Round Replays
    (212, 0x9a5854), // Second Preliminary Round Second Replays
    (213, 0x9a5830), // Second Preliminary Round First Legs
    (214, 0x9a5808), // Second Preliminary Round Second Legs
    (220, 0x9a57dc), // Qualifying Round<%s - Plural, arg not used>
    (221, 0x9a57c0), // Qualifying Round Replays
    (222, 0x9a57a0), // Qualifying Round Second Replays
    (223, 0x9a5784), // Qualifying Round First Legs
    (224, 0x9a5764), // Qualifying Round Second Legs
    (230, 0x9a5734), // Preliminary Round<%s - Plural, arg not used>
    (231, 0x9a5718), // Preliminary Round Replays
    (232, 0x9a56f4), // Preliminary Round Second Replays
    (233, 0x9a56d4), // Preliminary Round First Legs
    (234, 0x9a56b4), // Preliminary Round Second Legs
    (240, 0x9a70f4), // Promotion Final
    (243, 0x9a70d8), // Promotion Final First Leg
    (244, 0x9a70bc), // Promotion Final Second Leg
    (250, 0x9a70a4), // First Qualifying Phase
    (253, 0x9a5690), // First Qualifying Phase First Legs
    (254, 0x9a566c), // First Qualifying Phase Second Legs
    (260, 0x9a7044), // Second Qualifying Phase
    (263, 0x9a5648), // Second Qualifying Phase First Legs
    (264, 0x9a5624), // Second Qualifying Phase Second Legs
    (270, 0x9a6fe4), // Third Qualifying Phase
    (273, 0x9a5600), // Third Qualifying Phase First Legs
    (274, 0x9a55dc), // Third Qualifying Phase Second Legs
    (280, 0x9a6f90), // Playoff 1
    (283, 0x9a55c4), // Playoff 1 First Legs
    (284, 0x9a55ac), // Playoff 1 Second Legs
    (290, 0x9a6f58), // Playoff 2
    (293, 0x9a5594), // Playoff 2 First Legs
    (294, 0x9a557c), // Playoff 2 Second Legs
    (300, 0x9a6f1c), // North Playoff
    (310, 0x9a5c4c), // Third Place Playoffs
    (311, 0x9a5c30), // Third Place Playoff Replays
    (312, 0x9a5c0c), // Third Place Playoff Second Replays
    (313, 0x9a5bec), // Third Place Playoff First Legs
    (314, 0x9a5bcc), // Third Place Playoff Second Legs
    (320, 0x9a5568), // Elimination Finals
    (323, 0x9a5548), // Elimination Final First Legs
    (324, 0x9a5528), // Elimination Final Second Legs
    (330, 0x9a5514), // Minor Semi Finals
    (333, 0x9a54f8), // Minor Semi Final First Legs
    (334, 0x9a54d8), // Minor Semi Final Second Legs
    (340, 0x9a54c4), // Major Semi Finals
    (343, 0x9a54a8), // Major Semi Final First Legs
    (344, 0x9a5488), // Major Semi Final Second Legs
    (350, 0x9a5474), // Preliminary Finals
    (353, 0x9a5454), // Preliminary Final First Legs
    (354, 0x9a5434), // Preliminary Final Second Legs
    (360, 0x9a5424), // Grand Finals
    (363, 0x9a540c), // Grand Final First Legs
    (364, 0x9a53f4), // Grand Final Second Legs
    (370, 0x9a53e0), // Interzone Playoffs
    (373, 0x9a53c0), // Interzone Playoffs First Leg
    (374, 0x9a53a0), // Interzone Playoffs Second Leg
    (1000, 0x9a6d50), // Group 1
    (1001, 0x9a6d48), // Group 2
    (1002, 0x9a6d40), // Group 3
    (1003, 0x9a6d38), // Group 4
    (1004, 0x9a6d30), // Group 5
    (1005, 0x9a6d28), // Group 6
    (1006, 0x9a6d20), // Group 7
    (1007, 0x9a6d18), // Group 8
    (1008, 0x9a6d10), // Group 9
    (1009, 0x9a6d04), // Group 10
    (1010, 0x9a6cf8), // Group 11
    (1011, 0x9a6cec), // Group 12
    (1012, 0x9a6ce0), // Group 13
    (1013, 0x9a6cd4), // Group 14
    (1014, 0x9a6cc8), // Group 15
    (1015, 0x9a6cbc), // Group 16
    (1016, 0x9a6cb0), // Group 17
    (1017, 0x9a6ca4), // Group 18
    (1018, 0x9a6c98), // Group 19
    (1019, 0x9a6c8c), // Group 20
    (1020, 0x9a6c78), // Second Placed Teams
    (1021, 0x9a6c64), // First Round Group 1
    (1022, 0x9a6c50), // First Round Group 2
    (1023, 0x9a6c3c), // First Round Group 3
    (1024, 0x9a6c28), // First Round Group 4
    (1025, 0x9a6c14), // First Round Group 5
    (1026, 0x9a6c00), // First Round Group 6
    (1027, 0x9a6bec), // First Round Group 7
    (1028, 0x9a6bd8), // First Round Group 8
    (1029, 0x9a6bc4), // First Round Group 9
    (1030, 0x9a6bac), // First Round Group 10
    (1031, 0x9a6b94), // Second Round Group 1
    (1032, 0x9a6b7c), // Second Round Group 2
    (1033, 0x9a6b64), // Second Round Group 3
    (1034, 0x9a6b4c), // Second Round Group 4
    (1035, 0x9a6b34), // Second Round Group 5
    (1036, 0x9a6b20), // Semi Final Group 1
    (1037, 0x9a6b0c), // Semi Final Group 2
    (1038, 0x9a6af8), // Semi Final Group 3
    (1039, 0x9a6ae8), // Carribean Zone
    (1040, 0x9a6ad0), // Central American Zone
    (1041, 0x9a6ab8), // Premier/First(South)
    (1042, 0x9a6aa0), // Premier/First(North)
    (1043, 0x9a6a8c), // Eastern Conference
    (1044, 0x9a6a78), // Western Conference
    (1045, 0x9a5384), // Eastern Conference Playoffs
    (1046, 0x9a5368), // Western Conference Playoffs
    (1047, 0x9a6a38), // MLS Cup
    (1048, 0x9a6a28), // Playoff Group 1
    (1049, 0x9a6a18), // Playoff Group 2
    (1050, 0x9a6a08), // Autumn Season
    (1051, 0x9a69f8), // Spring Season
    (1053, 0x9a69e4), // Northern Section
    (1054, 0x9a69d0), // Southern Section
    (1055, 0x9a69c8), // Group A
    (1056, 0x9a69c0), // Group B
    (1057, 0x9a69b8), // Group C
    (1058, 0x9a69b0), // Group D
    (1059, 0x9a69a8), // Group E
    (1060, 0x9a69a0), // Group F
    (1061, 0x9a6998), // Group G
    (1062, 0x9a6990), // Group H
    (1063, 0x9a6988), // Group I
    (1064, 0x9a6980), // Group J
    (1065, 0x9a6978), // Group K
    (1066, 0x9a6970), // Group L
    (1067, 0x9a6968), // Group M
    (1068, 0x9a6960), // Group N
    (1069, 0x9a6958), // Group O
    (1070, 0x9a6950), // Group P
    (1071, 0x9a6938), // Oceania/Asia Playoff
    (1072, 0x9a6924), // First(South)/Second
    (1073, 0x9a6910), // First(North)/Second
    (1074, 0x9a6904), // First Stage
    (1075, 0x9a68f4), // Second Stage
    (1076, 0x9a68e4), // Opening Stage
    (1077, 0x9a68d4), // Closing Stage
    (1078, 0x9a68c4), // Interior Zone
    (1079, 0x9a68b0), // Metropolitan Zone
    (1080, 0x9a68a4), // Promotion
    (1081, 0x9a6894), // First Period
    (1082, 0x9a6884), // Second Period
    (1083, 0x9a6874), // Third Period
    (1084, 0x9a6864), // Fourth Period
    (1085, 0x9a6854), // Phase 1 Group A
    (1086, 0x9a6844), // Phase 1 Group B
    (1087, 0x9a6834), // Phase 1 Group C
    (1088, 0x9a6824), // Phase 1 Group D
    (1089, 0x9a6814), // Phase 1 Group E
    (1090, 0x9a6804), // Phase 1 Group F
    (1091, 0x9a67f4), // Phase 1 Group G
    (1092, 0x9a67e4), // Phase 1 Group H
    (1093, 0x9a67d4), // Phase 2 Group A
    (1094, 0x9a67c4), // Phase 2 Group B
    (1095, 0x9a67b4), // Phase 2 Group C
    (1096, 0x9a67a4), // Phase 2 Group D
    (1097, 0x9a6790), // Phase 1 Group Stage
    (1098, 0x9a677c), // Phase 2 Group Stage
    (1099, 0x9a6768), // Championship Group
    (1100, 0x9a6754), // Relegation Group
    (1101, 0x9a6740), // Promotion Group A
    (1102, 0x9a672c), // Promotion Group B
    (1103, 0x9a6718), // Promotion Group C
    (1104, 0x9a6704), // Promotion Group D
    (1105, 0x9a66ec), // Promotion Group Stage
    (1106, 0x9a66d8), // Relegation Playoff
    (1107, 0x9a66d0), // North
    (1108, 0x9a66c4), // Northeast
    (1109, 0x9a66b4), // West/Southwest
    (1110, 0x9a66ac), // South
    (1111, 0x9a6684), // Central<%s - COMMENT - Central section>
    (1112, 0x9a6670), // Promotion Playoff
    (1113, 0x9a6660), // First Promotion
    (1114, 0x9a664c), // Second Promotion
    (1115, 0x9a663c), // Third Promotion
    (1116, 0x9a6628), // Fourth Promotion
    (1117, 0x9a6610), // Group A First Period
    (1118, 0x9a65f8), // Group A Second Period
    (1119, 0x9a65e0), // Group A Third Period
    (1120, 0x9a65c8), // Group B First Period
    (1121, 0x9a65b0), // Group B Second Period
    (1122, 0x9a6598), // Group B Third Period
    (1123, 0x9a6d50), // Group 1
    (1124, 0x9a6d48), // Group 2
    (1125, 0x9a6d40), // Group 3
    (1126, 0x9a6d38), // Group 4
    (1127, 0x9a6d30), // Group 5
    (1128, 0x9a6d28), // Group 6
    (1129, 0x9a6d20), // Group 7
    (1130, 0x9a6d18), // Group 8
    (1131, 0x9a5358), // Championship
    (1132, 0x9a66d0), // North
    (1133, 0x9a6588), // East Svealand
    (1134, 0x9a6578), // West Svealand
    (1135, 0x9a6568), // East Gotaland
    (1136, 0x9a6558), // West Gotaland
    (1137, 0x9a6548), // South Gotaland
    (1138, 0x9a6530), // Second Round Group 6
    (1139, 0x9a6510), // Central American Zone Group 1
    (1140, 0x9a64f0), // Central American Zone Group 2
    (1141, 0x9a64d8), // Classification Group 1
    (1142, 0x9a64c0), // Classification Group 2
    (1143, 0x9a64a8), // Classification Group 3
    (1144, 0x9a6490), // Classification Group 4
    (1145, 0x9a6478), // Classification Group 5
    (1146, 0x9a6460), // Classification Groups
    (1147, 0x9a644c), // Central Conference
    (1148, 0x9a6430), // Northern Relegation Section
    (1149, 0x9a6414), // Southern Relegation Section
    (1150, 0x9a6400), // Third Round Group A
    (1151, 0x9a63ec), // Third Round Group B
    (1152, 0x9a4094), // East
    (1153, 0x9a408c), // West
    (1154, 0x9a5344), // East Quarter Finals
    (1155, 0x9a5330), // West Quarter Finals
];

/// The comp_util.cpp source path of the error boxes.
const COMP_UTIL_SOURCE: usize = 0x99ec20;

/// Writes the localised name of a round, group or stage id into `buf` and returns `buf`.
///
/// `comp_util_get_stage_name(buf, size, id, form)` (cdecl; the id is the word of the third argument, the form the byte
/// of the fourth). First `buf` gets a copy of the string at 0xad9c64 (the game's empty text). With `size` < 30: the
/// error box of line 0x110f (`show_error_at`, comp_util.cpp) and nothing else. Form 0 (full name), 1 (short name) or 3
/// (plural name): id 0 prints the string at 0xad9c64 (sprintf `exe::crt_sprintf`, the string as the format); an id of
/// the form's table (`STAGE_NAMES_FULL`, `STAGE_NAMES_SHORT`, `STAGE_NAMES_PLURAL`: 322, 322 and 321 ids from 1 to
/// 1155, extracted by following the original's switches for every id and form) prints its text (langlib_sprintf
/// `FN_LANGLIB_SPRINTF`, the text as the format, no arguments: the "<%s - ...>" parts are translator comments the
/// langlib drops); any other id writes "Cannot find index %d" with the id into the global text buffer 0xde1f64 and
/// leaves `buf` as it is. Any other form: the error box of line 0x15e2.
/// Original: `comp_util_get_stage_name` (0x4b4850, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_get_stage_name(buf: usize, size: i32, id: u32, form: u32) -> usize {
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    copy_c_string(buf, 0xad9c64);
    if size < 0x1e {
        show_error_at(COMP_UTIL_SOURCE, 0x110f);
        return buf;
    }
    let table: &[(i16, usize)] = match form as u8 {
        0 => &STAGE_NAMES_FULL, // cqy3: the full names
        3 => &STAGE_NAMES_PLURAL,
        1 => &STAGE_NAMES_SHORT,
        _ => {
            show_error_at(COMP_UTIL_SOURCE, 0x15e2);
            return buf;
        }
    };
    let id = id as u16 as i16;
    if id == 0 {
        sprintf(buf, 0xad9c64);
        return buf;
    }
    match table.binary_search_by_key(&id, |e| e.0) {
        Ok(k) => {
            covered!("cqy_3 stage name");
            lang_sprintf(buf, table[k].1);
        }
        Err(_) => {
            covered!("cqy_3 stage name not found");
            sprintf(0xde1f64, 0x9a51b0, id as i32); // the global text buffer, "Cannot find index %d"
        }
    }
    buf
}

/// Comparer for `comp_util_get_stage_name`: side by side, the original into the caller's buffer and the copy into its
/// own buffer of the same size (both write the same "Cannot find index" text into the global buffer); the two texts
/// must be equal. With `size` < 30 or a form other than 0, 1 and 3 (the error box), only the original.
pub(crate) unsafe extern "C" fn compare_comp_util_get_stage_name(buf: usize, size: i32, id: u32, form: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, u32, u32) -> usize>(IDX_COMP_UTIL_GET_STAGE_NAME);
    if size < 0x1e || !matches!(form as u8, 0 | 1 | 3) {
        return o(buf, size, id, form);
    }
    let mut mine = vec![0u8; (size as usize).max(0x400)];
    comp_util_get_stage_name(mine.as_mut_ptr() as usize, size, id, form);
    let r = o(buf, size, id, form);
    let a = c_string_bytes(buf);
    let b = c_string_bytes(mine.as_ptr() as usize);
    SCOREBOARD.record(a == b, || format!("comp_util_get_stage_name(id {}, form {}): {:?} x {:?}", id as u16 as i16, form as u8,
        String::from_utf8_lossy(&a), String::from_utf8_lossy(&b)));
    r
}

/// Lower-cases a C string in place, one character at a time (tolower `0x9465fc`, cdecl, the signed character), with a
/// signed byte index as in the original.
unsafe fn lower_case_in_place(p: usize) {
    let tolower: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x9465fcusize);
    let mut k: i8 = 0;
    let mut c = read_u8(p) as i8;
    while c != 0 {
        write_u8(p.wrapping_add(k as isize as usize), tolower(c as i32) as u8);
        k = k.wrapping_add(1);
        c = read_u8(p.wrapping_add(k as isize as usize)) as i8;
    }
}

/// Writes a label of competition, stage and round names into `buf` (`size` ≥ 0x67), using the long or the short
/// competition name by `form`; returns 1 (0 with the error box).
///
/// `comp_util_format_comp_stage_round_label(comp, stage, round, form, buf, size)` (cdecl; the stage and round ids are
/// the words of their arguments, the form the byte). With no `buf` or `size` < 0x67: the error box of line 0x25af and
/// 0. The German regional league (`KEY_CLUB_COMP_GERMAN_REGIONAL`) with stage 0x453 becomes the North division
/// (`KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_NORTH`, a 0x6b-byte record of `G_CLUB_COMP_TABLE`) and stage 0; then, with
/// stage 0x456, the South division and stage 0.
///
/// Form 1: the competition's long name (`club_comp::NAME`, +4) and its gender byte (+0x37); form 2: the short name
/// (+0x38) and its gender byte (+0x52); form 3: the same as form 2. The stage and the round names come from
/// `comp_util_get_stage_name` (100-byte buffers; form 0, or 1 in form 3) with their genders
/// (comp_util_get_stage_name_gender `exe::comp_util_get_stage_name_gender`; the round's with form 1 only in form 3). With
/// no competition the name is empty and its gender 0. Any other form leaves the three names unset (the original then
/// prints its stack leftovers; here they are empty) and the genders 0.
///
/// With a competition: in German (langlib_get_language `exe::langlib_get_language` = 8) and a stage or a round, the
/// competition name goes through the grammar modifier "s" (langlib_apply_grammar_modifier
/// `exe::langlib_apply_grammar_modifier`(name, 0xad9c64, "s", 100, 100), the result copied back) and the stage and round
/// names are lower-cased (`lower_case_in_place`). Then, by which ids are not 0: stage and round, the format 0x9a7cc0
/// (competition, stage, round, each with its gender twice); stage only, 0x9a7c94; round only, 0x9a7c68; neither, the
/// competition name copied. With no competition: stage and round, 0x9a7c44 (stage, round); stage only or round only,
/// that name copied; neither, `buf` left as it is. The formats are printed through langlib_sprintf
/// (`FN_LANGLIB_SPRINTF`). Returns 1.
/// Original: `comp_util_format_comp_stage_round_label` (0x4bfa30, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_format_comp_stage_round_label(comp: usize, stage: u32, round: u32, form: u32, buf: usize, size: i32) -> u32 {
    let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if buf == 0 || size < 0x67 {
        show_error_at(COMP_UTIL_SOURCE, 0x25af);
        return 0;
    }
    let (mut comp, mut stage) = (comp, stage);
    if comp != 0 {
        let regional = read_usize(KEY_CLUB_COMP_GERMAN_REGIONAL);
        let table = read_usize(G_CLUB_COMP_TABLE);
        if read_usize(comp) == regional && stage as u16 == 0x453 {
            comp = table + read_usize(KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_NORTH) * 0x6b; // cqy3: the North division
            stage = 0;
        }
        if comp != 0 && read_usize(comp) == regional && stage as u16 == 0x456 {
            comp = table + read_usize(KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_SOUTH) * 0x6b;
            stage = 0;
        }
    }
    let (mut name, mut stage_name, mut round_name) = ([0u8; 100], [0u8; 100], [0u8; 100]); // cleared: see the header
    let (pn, ps, pr) = (name.as_mut_ptr() as usize, stage_name.as_mut_ptr() as usize, round_name.as_mut_ptr() as usize);
    let (mut comp_gender, mut stage_gender, mut round_gender) = (0u8, 0u8, 0u8);
    let names = match form as u8 {
        1 => Some((4usize, 0x37usize, 0u32, 0u32)),
        2 => Some((0x38, 0x52, 0, 0)),
        3 => Some((0x38, 0x52, 1, 1)),
        _ => None,
    };
    if let Some((name_at, gender_at, stage_form, round_form)) = names {
        if comp != 0 {
            copy_c_string(pn, comp + name_at);
            comp_gender = read_u8(comp + gender_at);
        } // with no competition the name stays empty (the original writes its first byte 0)
        exe::comp_util_get_stage_name(ps as *mut u8, 100, stage, stage_form);
        stage_gender = exe::comp_util_get_stage_name_gender(stage, stage_form);
        exe::comp_util_get_stage_name(pr as *mut u8, 100, round, round_form);
        round_gender = exe::comp_util_get_stage_name_gender(round, round_form);
    }
    let (has_stage, has_round) = (stage as u16 != 0, round as u16 != 0);
    let (cg, sg, rg) = (comp_gender as i8 as i32, stage_gender as i8 as i32, round_gender as i8 as i32);
    if comp != 0 {
        if exe::langlib_get_language() == 8 && (has_stage || has_round) {
            covered!("cqy_3 label in german");
            let m = exe::langlib_apply_grammar_modifier(pn, 0xad9c64 as *const u8, 0x99c368 as *const u8, 100, 100);
            copy_c_string(pn, m);
            if has_stage {
                lower_case_in_place(ps);
            }
            if round as u16 != 0 {
                lower_case_in_place(pr);
            }
        }
        if has_stage && has_round {
            covered!("cqy_3 label competition stage round");
            lang_sprintf(buf, 0x9a7cc0, cg, cg, sg, sg, rg, rg, pn, ps, pr);
        } else if has_stage {
            lang_sprintf(buf, 0x9a7c94, cg, cg, sg, sg, pn, ps);
        } else if has_round {
            lang_sprintf(buf, 0x9a7c68, cg, cg, rg, rg, pn, pr);
        } else {
            copy_c_string(buf, pn);
        }
    } else if has_stage && has_round {
        covered!("cqy_3 label stage round");
        lang_sprintf(buf, 0x9a7c44, sg, sg, rg, rg, ps, pr);
    } else if has_stage {
        copy_c_string(buf, ps);
    } else if has_round {
        copy_c_string(buf, pr);
    }
    1
}

/// Comparer for `comp_util_format_comp_stage_round_label`: side by side, the original into the caller's buffer and the
/// copy into its own buffer of the same size that starts with the caller's bytes (the "neither" case leaves it as it
/// was; `copy_c_string` and `c_string_bytes` are the database and contract_manager helpers); the two texts must be equal, and the returns. With no buffer or `size` < 0x67 (the error box), only the
/// original.
pub(crate) unsafe extern "C" fn compare_comp_util_format_comp_stage_round_label(comp: usize, stage: u32, round: u32, form: u32, buf: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, usize, i32) -> u32>(IDX_COMP_UTIL_FORMAT_COMP_STAGE_ROUND_LABEL);
    if buf == 0 || size < 0x67 {
        return o(comp, stage, round, form, buf, size);
    }
    let mut mine = vec![0u8; (size as usize).max(0x400)];
    core::ptr::copy_nonoverlapping(buf as *const u8, mine.as_mut_ptr(), size as usize);
    let b = comp_util_format_comp_stage_round_label(comp, stage, round, form, mine.as_mut_ptr() as usize, size);
    let a = o(comp, stage, round, form, buf, size);
    let (ta, tb) = (c_string_bytes(buf), c_string_bytes(mine.as_ptr() as usize));
    SCOREBOARD.record(a == b && ta == tb, || format!("comp_util_format_comp_stage_round_label({:#x}, {}, {}, form {}): {} {:?} x {} {:?}", comp,
        stage as u16 as i16, round as u16 as i16, form as u8, a, String::from_utf8_lossy(&ta), b, String::from_utf8_lossy(&tb)));
    a
}

// ---------- batch 292: the fixture tree of a cup stage: the ties of each round, the links to the next round and the entrants in the first ties (cup_stage.cpp) ----------

/// The cup_stage.cpp source path of the error boxes.
const CUP_STAGE_SOURCE: usize = 0x9b5944;

/// Links tie p of round `r` to a free side of a tie of round `target` (the search starts at tie `s`): the home side
/// (`cup_tie::HOME_LINK_TIE` free: the slot keeps p and r, the tie keeps the slot and `cup_tie::FEEDER_SIDE` 1) or else
/// the away side (`cup_tie::AWAY_LINK_TIE`, `cup_tie::FEEDER_SIDE` 0), moving to the next tie (back to 0 after the
/// round's last) while both are taken. Returns false, with nothing written, when it comes back to `s`.
unsafe fn link_tie_to_round(this: Cup, r: i32, p: i32, target: i32, s: i32) -> bool {
    let start = s;
    let mut s = s;
    let c = cup_tie(this, r, p);
    loop {
        let v = cup_tie(this, target, s as i16 as i32);
        let side = if read_i16(v + cup_tie::HOME_LINK_TIE) == -1 {
            Some(true) // cqy4: the home side first
        } else if read_i16(v + cup_tie::AWAY_LINK_TIE) == -1 {
            Some(false)
        } else {
            None
        };
        if let Some(home) = side {
            write_u16(v + if home { cup_tie::HOME_LINK_TIE } else { cup_tie::AWAY_LINK_TIE }, p as u16);
            write_u16(v + if home { cup_tie::HOME_LINK_ROUND } else { cup_tie::AWAY_LINK_ROUND }, r as u16);
            write_u16(c + cup_tie::FEEDER_TIE, s as u16);
            write_u16(c + cup_tie::FEEDER_SIDE, home as u16);
            return true;
        }
        s = if (s as i16 as i32) < read_i16(comp_cup_round(this, target) + cup_round::TIES) as i32 - 1 { s + 1 } else { 0 };
        if s as i16 == start as i16 {
            return false;
        }
    }
}

/// Builds the fixture tree of a cup stage made with a tree (`Cup_Stage::Cup_Stage` with `arvore`; the Champions
/// League knockout stage after the groups): the ties of each round, each tie linked to a tie of the next round (by
/// pair, or drawn and then the first free side), and the entrants of each round in its ties by pair. Returns 1, or −1
/// with the error box.
///
/// `Cup_Stage::map_fixture_tree()` (thiscall, ret 0). The tie arrays (`exe::crt_malloc`: 4 per round into
/// `tie_tables()` and `cup_round::TIES` × 0x35 per round; out of memory, the box of line 0x201 or 0x20b of
/// cup_stage.cpp and −1: not the fatal box of Cup::map_fixture_tree); status +5 of each team (`team_table()`, 6 bytes)
/// cleared. Without `season_set_up()`: 1 here (`current_round()` and `next_draw_round()` untouched). Otherwise
/// `current_round()` = `next_draw_round()` = 0 and each tie is emptied (the words +0, +2, +6, +0xa, +0xc, +0xe, +0x10 and
/// +0x12 = −1, `cup_tie::FEEDER_SIDE` 0, +0x14..+0x31 = 0xff, `cup_tie::REPLAYS_PLAYED`, `cup_tie::NEXT_MATCH` and
/// `cup_tie::FIXTURE_STATE` 0; +4 untouched).
///
/// m = the rounds, less one when the last round's code (`cup_round::ROUND_CODE`) is 0x8c (third place). For each round
/// r below m − 1, each tie p in order (no shuffle, unlike Cup::map_fixture_tree): the slot in round r + 1 is p/2 when
/// that round's `cup_round::TIE_RULES` has 0x200, otherwise drawn (utils_random `exe::utils_random`(its ties)); from it,
/// `link_tie_to_round`, with `cup_tie::FEEDER_ROUND` = r + 1. With no free side in round r + 1: in the last two rounds
/// before m, the fatal box of line 0x294 (the game stops); otherwise a slot drawn in round r + 2 and the same search
/// (`cup_tie::FEEDER_ROUND` = r + 2; none free: the fatal box of line 0x2bf).
///
/// With no ties in round 0: `run_by_stages()` = 1 and 1. Otherwise, for each round r below m, each entrant q
/// (`cup_round::NEW_TEAMS`, from `cup_round::FIRST_NEW_TEAM`) goes to tie q/2: to the home team when it and
/// `cup_tie::HOME_LINK_TIE` are free, otherwise to the away team when it and `cup_tie::AWAY_LINK_TIE` are free; a taken
/// tie with both teams set and the cup's `venue_stadium_id()` set has its teams swapped when the away club's stadium
/// (`club::STADIUM`) is that venue, and the search moves to the next tie (back to 0 after the last; all taken: the box
/// of line 0x310 and −1).
///
/// Finally the number of the first tie of each round (`cup_round::FIRST_TIE_NUMBER`), from back to front: with a
/// third-place round, from the round before the final, starting at 5; otherwise from the final's round − 1, starting
/// at the last round's +0x18 byte + 1; each round adds its ties (byte). Returns 1.
/// Original: `Cup_Stage::map_fixture_tree` (0x5241e0, cup_stage.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cup_stage_map_fixture_tree(this: usize) -> i32 {
    let this = Cup(this);
    let t = exe::crt_malloc((this.round_count() as i32 as u32) << 2);
    this.set_tie_tables(t);
    if t == 0 {
        show_error_at(CUP_STAGE_SOURCE, 0x201);
        return -1;
    }
    let mut i = 0i32;
    while i < this.round_count() as i32 {
        let n = read_i16(comp_cup_round(this, i) + cup_round::TIES) as i32;
        write_usize(this.tie_tables() + i as usize * 4, exe::crt_malloc((n * 0x35) as u32));
        if read_usize(this.tie_tables() + i as usize * 4) == 0 {
            show_error_at(CUP_STAGE_SOURCE, 0x20b);
            return -1;
        }
        i += 1;
    }
    if this.team_table() != 0 {
        let mut k = 0i32;
        while k < this.team_count() as i32 {
            write_u8(this.team_table() + k as usize * 6 + 5, 0);
            k += 1;
        }
    }
    if this.season_set_up() == 0 {
        return 1;
    }
    this.set_current_round(0);
    this.set_next_draw_round(0);
    let mut i = 0i32;
    while i < this.round_count() as i32 {
        let mut j = 0i32;
        while j < read_i16(comp_cup_round(this, i) + cup_round::TIES) as i32 {
            let c = cup_tie(this, i, j);
            for k in [0usize, 2, 6, 0xa, 0xc, 0xe, 0x10, 0x12] {
                write_u16(c + k, 0xffff);
            }
            write_u16(c + cup_tie::FEEDER_SIDE, 0);
            for k in 0x14..=0x31usize {
                write_u8(c + k, 0xff);
            }
            write_u8(c + cup_tie::REPLAYS_PLAYED, 0);
            write_u8(c + cup_tie::NEXT_MATCH, 0);
            write_u8(c + cup_tie::FIXTURE_STATE, 0);
            j += 1;
        }
        i += 1;
    }
    let nr = this.round_count();
    let m = (if read_i16(comp_cup_round(this, nr as i32).wrapping_sub(0x68) + cup_round::ROUND_CODE) == 0x8c { nr.wrapping_sub(1) } else { nr }) as i32;
    let mut r = 0i32;
    while r < m - 1 {
        let mut p = 0i32;
        while p < read_i16(comp_cup_round(this, r) + cup_round::TIES) as i32 {
            let next = comp_cup_round(this, r + 1);
            let s = if read_i16(next + cup_round::TIE_RULES) as u16 & 0x200 != 0 {
                p / 2 // cqy4: the slot by the pair
            } else {
                exe::utils_random(read_i16(next + cup_round::TIES) as i32)
            };
            if link_tie_to_round(this, r, p, r + 1, s) {
                covered!("cqy_4 tie linked to the next round");
                write_u16(cup_tie(this, r, p) + cup_tie::FEEDER_ROUND, (r + 1) as u16);
            } else {
                if r >= m - 2 {
                    fatal_error_at(CUP_STAGE_SOURCE, 0x294);
                }
                let s2 = exe::utils_random(read_i16(comp_cup_round(this, r + 2) + cup_round::TIES) as i32);
                if !link_tie_to_round(this, r, p, r + 2, s2) {
                    fatal_error_at(CUP_STAGE_SOURCE, 0x2bf);
                }
                write_u16(cup_tie(this, r, p) + cup_tie::FEEDER_ROUND, (r + 2) as u16);
            }
            p += 1;
        }
        r += 1;
    }
    if read_i16(comp_cup_round(this, 0) + cup_round::TIES) == 0 {
        this.set_run_by_stages(1);
        return 1;
    }
    let mut r = 0i32;
    while r < m {
        let mut q = 0i32;
        while q < read_i16(comp_cup_round(this, r) + cup_round::NEW_TEAMS) as i32 {
            let start = q / 2; // cqy4: the entrant's tie by the pair
            let mut s = start;
            loop {
                let c = cup_tie(this, r, s as i16 as i32);
                let home = read_i16(c + cup_tie::HOME_TEAM);
                let entrant = (read_i16(comp_cup_round(this, r) + cup_round::FIRST_NEW_TEAM) as u16).wrapping_add(q as u16);
                if home == -1 && read_i16(c + cup_tie::HOME_LINK_TIE) == -1 {
                    covered!("cqy_4 entrant at home");
                    write_u16(c + cup_tie::HOME_TEAM, entrant);
                    break;
                }
                if read_i16(c + cup_tie::AWAY_TEAM) == -1 && read_i16(c + cup_tie::AWAY_LINK_TIE) == -1 {
                    write_u16(c + cup_tie::AWAY_TEAM, entrant);
                    break;
                }
                let away = read_i16(c + cup_tie::AWAY_TEAM);
                if this.venue_stadium_id() as u32 != u32::MAX && home != -1 && away != -1 {
                    let club = Club(read_usize(this.team_table() + (away as isize * 6) as usize));
                    if !club.is_null() {
                        let stadium = club.stadium().addr();
                        if stadium != 0 && read_usize(stadium) == this.venue_stadium_id() {
                            write_u16(c + cup_tie::HOME_TEAM, away as u16);
                            write_u16(c + cup_tie::AWAY_TEAM, home as u16);
                        }
                    }
                }
                s = if (s as i16 as i32) < read_i16(comp_cup_round(this, r) + cup_round::TIES) as i32 - 1 { s + 1 } else { 0 };
                if s as i16 == start as i16 {
                    show_error_at(CUP_STAGE_SOURCE, 0x310);
                    return -1;
                }
            }
            q += 1;
        }
        r += 1;
    }
    let nr = this.round_count();
    if nr > 0 && read_i16(comp_cup_round(this, nr as i32).wrapping_sub(0x68) + cup_round::ROUND_CODE) == 0x8c {
        let mut k = nr.wrapping_sub(3) as i32;
        let mut n: u8 = 5;
        while k >= 0 {
            write_u8(comp_cup_round(this, k) + cup_round::FIRST_TIE_NUMBER, n);
            n = n.wrapping_add(read_u8(comp_cup_round(this, k) + cup_round::TIES));
            k -= 1;
        }
    } else {
        let mut k = nr.wrapping_sub(2) as i32;
        if k >= 0 {
            let mut n = read_u8(comp_cup_round(this, nr as i32 - 1) + cup_round::TEAMS).wrapping_add(1);
            while k >= 0 {
                write_u8(comp_cup_round(this, k) + cup_round::FIRST_TIE_NUMBER, n);
                n = n.wrapping_add(read_u8(comp_cup_round(this, k) + cup_round::TIES));
                k -= 1;
            }
        }
    }
    1
}

/// The first broken part of a built stage tree, or None: each tie of the rounds before m − 1 must be the one its
/// feeder slot points back to (`cup_tie::FEEDER_ROUND` r + 1 or r + 2, the slot's link tie and link round), and each
/// entrant of each round below m must be the home or away team of exactly one tie of its round.
unsafe fn cup_stage_tree_mismatch(this: Cup) -> Option<String> {
    let nr = this.round_count();
    let m = (if read_i16(comp_cup_round(this, nr as i32).wrapping_sub(0x68) + cup_round::ROUND_CODE) == 0x8c { nr.wrapping_sub(1) } else { nr }) as i32;
    for rd in 0..(m - 1).max(0) {
        for p in 0..read_i16(comp_cup_round(this, rd) + cup_round::TIES) as i32 {
            let c = cup_tie(this, rd, p);
            let target = read_i16(c + cup_tie::FEEDER_ROUND) as i32;
            let home = read_i16(c + cup_tie::FEEDER_SIDE) == 1;
            let ok = (target == rd + 1 || target == rd + 2) && {
                let v = cup_tie(this, target, read_i16(c + cup_tie::FEEDER_TIE) as i32);
                read_i16(v + if home { cup_tie::HOME_LINK_TIE } else { cup_tie::AWAY_LINK_TIE }) as i32 == p
                    && read_i16(v + if home { cup_tie::HOME_LINK_ROUND } else { cup_tie::AWAY_LINK_ROUND }) as i32 == rd
            };
            if !ok {
                return Some(format!("tie {} of round {} without its slot", p, rd));
            }
        }
    }
    if read_i16(comp_cup_round(this, 0) + cup_round::TIES) == 0 {
        return None;
    }
    for rd in 0..m.max(0) {
        let first = read_i16(comp_cup_round(this, rd) + cup_round::FIRST_NEW_TEAM) as i32;
        for q in 0..read_i16(comp_cup_round(this, rd) + cup_round::NEW_TEAMS) as i32 {
            let mut seen = 0;
            for p in 0..read_i16(comp_cup_round(this, rd) + cup_round::TIES) as i32 {
                let c = cup_tie(this, rd, p);
                seen += (read_i16(c + cup_tie::HOME_TEAM) as i32 == first + q) as i32 + (read_i16(c + cup_tie::AWAY_TEAM) as i32 == first + q) as i32;
            }
            if seen != 1 {
                return Some(format!("entrant {} of round {} placed {} times", first + q, rd, seen));
            }
        }
    }
    None
}

/// Comparer for `cup_stage_map_fixture_tree`: only the original runs (it allocates and draws); afterwards, on a season
/// set up and a return of 1, the plan (`cup_stage_tree_mismatch`).
pub(crate) unsafe extern "thiscall" fn compare_cup_stage_map_fixture_tree(this: usize) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_CUP_STAGE_MAP_FIXTURE_TREE)(this);
    let cup = Cup(this);
    if r != 1 || cup.season_set_up() == 0 {
        return r;
    }
    let error = cup_stage_tree_mismatch(cup);
    SCOREBOARD.record(error.is_none(), || format!("cup_stage_map_fixture_tree({:#x}): {}", this, error.clone().unwrap_or_default()));
    r
}

// ---------- batch 292: the cup news lines: a club wins in or is defeated in a cup round, and a team qualifies or fails to qualify from a group (comp_util.cpp) ----------

/// The round name code of a fixture without its variant: `fixture::ROUND_NAME` less its units digit (0 single match,
/// 3 first leg, 4 second leg, 5 replay) when it is below 1000, as the original's idiv by 10 (a word).
fn round_name_without_variant(code: i16) -> i16 {
    if code < 1000 { code.wrapping_sub(code % 10) } else { code }
}

/// Writes the short news line "<club> win in <round>" or "<club> defeat in <round>" into Text `text`.
///
/// `comp_util_write_cup_win_or_defeat_news(club, fixture, result, text)` (cdecl). The label of the fixture's
/// competition (`fixture::COMPETITION`), its stage (`fixture::ROUND_PREFIX`) and round (`round_name_without_variant` of
/// `fixture::ROUND_NAME`) in form 2 (comp_util_format_comp_stage_round_label, a 500-byte buffer); the gender of the
/// label: of the round (comp_util_get_stage_name_gender, form 0) when it is not 0, else of the stage, else the
/// competition's short-name gender (+0x52), else 1. With `result` (byte) 2, 4 or −1 the "defeat in" line (format
/// 0x9a3dbc), otherwise the "win in" line (0x9a3e1c): the club's short name and gender (`club::SHORT_NAME`,
/// `club::SHORT_GENDER_NAME` of club index `club`) and the label, printed into the global text buffer 0xde1f64
/// (langlib_sprintf `FN_LANGLIB_SPRINTF`). Then Text::set_string `exe::text_set_string` with it, and two links
/// (Text::add_link `exe::text_add_link`): the club name (type 0x7d5, the club) and the label (type 0x7d0, the
/// competition).
/// Original: `comp_util_write_cup_win_or_defeat_news` (0x4b4420, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_write_cup_win_or_defeat_news(club: i32, fixture: usize, result: u32, text: usize) {
    let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let f = Fixture(fixture);
    let comp = read_usize(fixture + fixture::COMPETITION);
    let prefix = f.round_prefix();
    let round = round_name_without_variant(f.round_name());
    let mut label = [0u8; 500];
    exe::comp_util_format_comp_stage_round_label(comp, prefix as u16 as u32, round as u16 as u32, 2, label.as_mut_ptr(), 500);
    let gender = if round != 0 {
        exe::comp_util_get_stage_name_gender(round as u16 as u32, 0)
    } else if prefix != 0 {
        exe::comp_util_get_stage_name_gender(prefix as u16 as u32, 0)
    } else if comp != 0 {
        read_u8(comp + 0x52)
    } else {
        1
    };
    let c = club_at(club as isize).addr();
    let cg = read_u8(c + club::SHORT_GENDER_NAME) as i8 as i32;
    let g = gender as i8 as i32;
    let format = match result as u8 {
        2 | 4 | 0xff => 0x9a3dbc, // cqy5: "defeat in"
        _ => 0x9a3e1c,
    };
    covered!("cqy_5 cup win or defeat news");
    lang_sprintf(0xde1f64, format, cg, cg, g, g, c + club::SHORT_NAME, label.as_ptr() as usize);
    exe::text_set_string(text, 0xde1f64 as *const u8);
    exe::text_add_link(text, (c + club::SHORT_NAME) as *const u8, 0x7d5, c);
    exe::text_add_link(text, label.as_ptr(), 0x7d0, comp);
}

/// Comparer for `comp_util_write_cup_win_or_defeat_news`: the original into the real Text and the copy into a local
/// one (`compare_text_writer`).
pub(crate) unsafe extern "C" fn compare_comp_util_write_cup_win_or_defeat_news(club: i32, fixture: usize, result: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(i32, usize, u32, usize)>(IDX_COMP_UTIL_WRITE_CUP_WIN_OR_DEFEAT_NEWS);
    compare_text_writer(text, || o(club, fixture, result, text), |t| comp_util_write_cup_win_or_defeat_news(club, fixture, result, t),
        || format!("comp_util_write_cup_win_or_defeat_news(club {}, fixture {:#x}, result {})", club, fixture, result as u8 as i8));
}

/// Writes the news line of a team qualifying from, or failing to qualify from, a competition group into Text `text`.
///
/// `comp_util_write_group_qualify_news(club, stage, comp, result, with_stage, text)` (cdecl). Only `result` (byte) 6 or
/// 7 (qualified) and −2 (failed) write anything. With `with_stage`: the stage's full name (`exe::comp_util_get_stage_name`
/// form 0, 100 bytes) and gender, and the line (format 0x9a3ed4 qualified, 0x9a3fd0 failed) with the team's short name
/// and gender (club index `club`), the stage name and gender and the competition's long name and gender
/// (`club_comp::NAME`, +4; +0x37); then Text::set_string and three links: the team name (type 0x7d5, the club), the
/// competition name (0x7d0, the competition) and the stage name (0x7e5, with the data (stage & 0x7ff | competition id
/// << 11) << 11 when qualified, (sign-extended stage word | 0xfffff800) << 11 when failed). Without it: the line (format
/// 0x9a3e74 qualified, 0x9a3f68 failed) with the team and the competition's short name and gender (+0x38, +0x52) and two
/// links: the team (0x7d5) and the competition's short name (0x7d0). The line is printed into the global text buffer
/// 0xde1f64 (langlib_sprintf `FN_LANGLIB_SPRINTF`).
/// Original: `comp_util_write_group_qualify_news` (0x4b4590, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_write_group_qualify_news(club: i32, stage: u32, comp: usize, result: u32, with_stage: u32, text: usize) {
    let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let qualified = match result as u8 {
        6 | 7 => true, // cqy5: qualified
        0xfe => false,
        _ => return,
    };
    let c = club_at(club as isize).addr();
    let cg = read_u8(c + club::SHORT_GENDER_NAME) as i8 as i32;
    let team = c + club::SHORT_NAME;
    if with_stage != 0 {
        covered!("cqy_5 group news with the stage");
        let mut name = [0u8; 100];
        exe::comp_util_get_stage_name(name.as_mut_ptr(), 100, stage, 0);
        let sg = exe::comp_util_get_stage_name_gender(stage, 0) as i8 as i32;
        let kg = read_u8(comp + 0x37) as i8 as i32;
        lang_sprintf(0xde1f64, if qualified { 0x9a3ed4 } else { 0x9a3fd0 }, cg, cg, sg, sg, kg, kg, team, name.as_ptr() as usize, comp + 4);
        exe::text_set_string(text, 0xde1f64 as *const u8);
        exe::text_add_link(text, team as *const u8, 0x7d5, c);
        exe::text_add_link(text, (comp + 4) as *const u8, 0x7d0, comp);
        let data = if qualified {
            ((stage & 0x7ff) | (read_u32(comp) << 0xb)) << 0xb
        } else {
            ((stage as u16 as i16 as i32 as u32) | 0xffff_f800) << 0xb
        };
        exe::text_add_link(text, name.as_ptr(), 0x7e5, data as usize);
    } else {
        covered!("cqy_5 group news without the stage");
        let kg = read_u8(comp + 0x52) as i8 as i32;
        lang_sprintf(0xde1f64, if qualified { 0x9a3e74 } else { 0x9a3f68 }, cg, cg, kg, kg, team, comp + 0x38);
        exe::text_set_string(text, 0xde1f64 as *const u8);
        exe::text_add_link(text, team as *const u8, 0x7d5, c);
        exe::text_add_link(text, (comp + 0x38) as *const u8, 0x7d0, comp);
    }
}

/// Comparer for `comp_util_write_group_qualify_news`: the original into the real Text and the copy into a local one
/// (`compare_text_writer`).
pub(crate) unsafe extern "C" fn compare_comp_util_write_group_qualify_news(club: i32, stage: u32, comp: usize, result: u32, with_stage: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(i32, u32, usize, u32, u32, usize)>(IDX_COMP_UTIL_WRITE_GROUP_QUALIFY_NEWS);
    compare_text_writer(text, || o(club, stage, comp, result, with_stage, text), |t| comp_util_write_group_qualify_news(club, stage, comp, result, with_stage, t),
        || format!("comp_util_write_group_qualify_news(club {}, stage {}, comp {:#x}, result {}, {})", club, stage as u16 as i16, comp, result as u8 as i8, with_stage));
}

// ---------- cqz_1: the rules of the Asian Cup, the Oceania Nations Cup and the Olympic football (asia_nations.cpp, oceania_nations.cpp, olympics.cpp) ----------

/// rule: the Asian Cup groups are single round robin, 3/1 points, tiebreak 1, 2, 3, two qualifiers per group, national
/// teams, category 10, 7 substitutes and 3 changes; the draw date is 5 of month 5 of the cup year on weekday 5,
/// `.next_draw_year_offset()` 0; then the stage −1 round table (virtual +0x3c).
///
/// Writes the rule bytes and words of `AFRICAN_NATIONS_RULE_FIELDS` (the same values as the African Nations Cup:
/// `+0x43` 2, `+0xc6` 2, `+0xbe` 2, win 3, `+0xc7` 3, `+0x4a` 3, `+0x3c` 1, `+0xc3` 1, category 0xa, `+0xc5` 1,
/// `+0xbf`..`+0xc1` 0, `+0x49` 7), Date::Date(5, 5, year, 5) (0x549f70: lea ecx, local; 4 pushes -> thiscall, ret 0x10;
/// the year pushed with garbage in the high 16 bits, read as a word) into `.next_draw_day()`, `.next_draw_year_offset()`
/// 0 and the round table of stage −1 (virtual +0x3c: push 0; push &+0x3a; push &+0xa9; push −1 -> thiscall, ret 0x10)
/// into `.round_dates()` (`finish_rules_setup`). Returns 1. The SEH frame only covers the local date (no destructor).
/// Original: `AsiaNations::set_up_rules` (0x40f4c0, asia_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_nations_set_up_rules(this: usize) -> u32 {
    covered!("cqz_1 asian cup rules");
    let win = 3; // cqz1: the win in the Asian Cup groups
    goldcup_write_rule_fields(this, &AFRICAN_NATIONS_RULE_FIELDS, win);
    let month = 5; // cqz1: the month of the Asian Cup draw date
    let word = competition_date_word(League(this), 5, month, 0, 5);
    finish_rules_setup(League(this), word, 0)
}

/// Comparer for `asia_nations_set_up_rules`: original only (virtual +0x3c allocates the round table), then the fields,
/// the date and the return value are checked (`check_set_up_rules`).
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ASIA_NATIONS_SET_UP_RULES);
    check_set_up_rules("asia_nations_set_up_rules", this, &AFRICAN_NATIONS_RULE_FIELDS, (5, 5, 0, 5), 0, || o(this))
}

/// rule: the Oceania Nations Cup groups follow the Asian Cup rules (single round robin, 3/1 points, tiebreak 1, 2, 3,
/// two qualifiers per group, category 10, 7 substitutes and 3 changes); the draw date is 5 of month 2 of the cup year on
/// weekday 5, `.next_draw_year_offset()` 0.
///
/// The same body as `asia_nations_set_up_rules` (the fields of `AFRICAN_NATIONS_RULE_FIELDS`; note `+0xbe` = 2, not the
/// 1 of the Oceania World Cup qualifying), with Date::Date(5, 2, year, 5). Returns 1.
/// Original: `OceaniaNations::set_up_rules` (0x799690, oceania_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_nations_set_up_rules(this: usize) -> u32 {
    covered!("cqz_1 oceania nations cup rules");
    let win = 3; // cqz1: the win in the Oceania Nations Cup groups
    goldcup_write_rule_fields(this, &AFRICAN_NATIONS_RULE_FIELDS, win);
    let month = 2; // cqz1: the month of the Oceania Nations Cup draw date
    let word = competition_date_word(League(this), 5, month, 0, 5);
    finish_rules_setup(League(this), word, 0)
}

/// Comparer for `oceania_nations_set_up_rules` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_OCEANIA_NATIONS_SET_UP_RULES);
    check_set_up_rules("oceania_nations_set_up_rules", this, &AFRICAN_NATIONS_RULE_FIELDS, (5, 2, 0, 5), 0, || o(this))
}

/// rule: the Olympic football groups follow the Asian Cup rules (single round robin, 3/1 points, tiebreak 1, 2, 3, two
/// qualifiers per group, category 10, 7 substitutes and 3 changes); the draw date is 20 of month 2 of the Olympic year
/// on weekday 5, `.next_draw_year_offset()` 0.
///
/// The same body as `asia_nations_set_up_rules`, with Date::Date(0x14, 2, year, 5). Returns 1.
/// Original: `Olympics::set_up_rules` (0x7a0230, olympics.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn olympics_set_up_rules(this: usize) -> u32 {
    covered!("cqz_1 olympics rules");
    let win = 3; // cqz1: the win in the Olympic groups
    goldcup_write_rule_fields(this, &AFRICAN_NATIONS_RULE_FIELDS, win);
    let day = 0x14; // cqz1: the day of the Olympic draw date
    let word = competition_date_word(League(this), day, 2, 0, 5);
    finish_rules_setup(League(this), word, 0)
}

/// Comparer for `olympics_set_up_rules` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_olympics_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_OLYMPICS_SET_UP_RULES);
    check_set_up_rules("olympics_set_up_rules", this, &AFRICAN_NATIONS_RULE_FIELDS, (0x14, 2, 0, 5), 0, || o(this))
}

// ---------- cqz_2: the rules of the European Championship, the Confederations Cup and the World Cup (euro_champ.cpp, fifa_confed.cpp, world_cup.cpp) ----------

/// Day of year of the European Championship seeding date (the first word of the date), read by its daily update.
pub(crate) const EURO_CHAMP_SEEDING_DAY: usize = 0x13e;
/// Year of the European Championship seeding date relative to the finals year (+0x40).
pub(crate) const EURO_CHAMP_SEEDING_YEAR_OFFSET: usize = 0x140;

/// The rule fields of the European Championship finals (offset, width, value), in the original's write order: national
/// teams, 2 qualifiers per group, single round robin, 3/1 points, category 10, tiebreak codes 4, 1 and 2, the stage id
/// 0x3e8 and 7 substitutes.
pub(crate) const EURO_CHAMP_RULE_FIELDS: [(usize, u8, u32); 15] = [(0x43, 1, 2), (0xc7, 1, 2), (0xbe, 1, 2), (0xc2, 1, 3), (0x4a, 1, 3), (0x3c, 2, 1), (0xc3, 1, 1),
    (0xc4, 1, 0xa), (0xc5, 1, 4), (0xc6, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x3a, 2, 0x3e8), (0x49, 1, 7)];

/// rule: the European Championship groups are single round robin, 3/1 points, tiebreak codes 4, 1 and 2, two qualifiers
/// per group, national teams, category 10, 7 substitutes and 3 changes; the seeding date is 10 January and the draw
/// date 10 February of the finals year (weekday 5).
///
/// Writes `EURO_CHAMP_RULE_FIELDS` (the win at `+0xc2`; the stage id `+0x3a` = 0x3e8 before the round table rewrites
/// it), Date::Date(0xa, 0, year, 5) (0x549f70: lea ecx, local; 4 pushes -> thiscall, ret 0x10; the year with garbage
/// in the high 16 bits, read as a word) into `EURO_CHAMP_SEEDING_DAY` and 0 into `EURO_CHAMP_SEEDING_YEAR_OFFSET`, then
/// Date::Date(0xa, 1, year, 5) into `.next_draw_day()`, `.next_draw_year_offset()` 0 and the stage −1 round table
/// (virtual +0x3c: push 0; push &+0x3a; push &+0xa9; push −1 -> thiscall, ret 0x10) into `.round_dates()`. Returns 1.
/// Original: `EuroChamp::set_up_rules` (0x57b630, euro_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_set_up_rules(this: usize) -> u32 {
    covered!("cqz_2 euro rules");
    let win = 3; // cqz2: the win in the European Championship groups
    goldcup_write_rule_fields(this, &EURO_CHAMP_RULE_FIELDS, win);
    let seeding = 0; // cqz2: the seeding month of the European Championship (January)
    let word = competition_date_word(League(this), 0xa, seeding, 0, 5);
    write_u16(this + EURO_CHAMP_SEEDING_DAY, word);
    write_u16(this + EURO_CHAMP_SEEDING_YEAR_OFFSET, 0);
    let word = competition_date_word(League(this), 0xa, 1, 0, 5);
    finish_rules_setup(League(this), word, 0)
}

/// Comparer for `euro_champ_set_up_rules`: original only (virtual +0x3c allocates the round table), then the fields, both
/// dates and the return value.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_EURO_CHAMP_SET_UP_RULES);
    let r = check_set_up_rules("euro_champ_set_up_rules", this, &EURO_CHAMP_RULE_FIELDS.iter().copied().filter(|f| f.0 != 0x3a).collect::<Vec<_>>(), (0xa, 1, 0, 5), 0, || o(this));
    let word = competition_date_word(League(this), 0xa, 0, 0, 5);
    let (day, offset) = (read_u16(this + EURO_CHAMP_SEEDING_DAY), read_u16(this + EURO_CHAMP_SEEDING_YEAR_OFFSET));
    SCOREBOARD.record(day == word && offset == 0, || format!("euro_champ_set_up_rules({:#x}): seeding date {:#x} expected {:#x}, offset {}", this, day, word, offset));
    r
}

/// The rule fields of the Confederations Cup (offset, width, value), in the original's write order: like the European
/// Championship without the stage id, and `.league_flags()` 0x28.
pub(crate) const FIFA_CONFED_RULE_FIELDS: [(usize, u8, u32); 15] = [(0xc2, 1, 3), (0x4a, 1, 3), (0x43, 1, 2), (0xc7, 1, 2), (0xbe, 1, 2), (0x3c, 2, 1), (0xc3, 1, 1),
    (0xc4, 1, 0xa), (0xc5, 1, 4), (0xc6, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7), (0xd9, 2, 0x28)];

/// rule: the Confederations Cup groups are single round robin, 3/1 points, tiebreak codes 4, 1 and 2, two qualifiers per
/// group, national teams, category 10, 7 substitutes and 3 changes; the draw date is 20 February of the cup year
/// (weekday 5).
///
/// Writes `FIFA_CONFED_RULE_FIELDS` (the win at `+0xc2`; `.league_flags()` = 0x28), then, unlike its sisters, the
/// stage −1 round table first (virtual +0x3c: push 0; push &+0x3a; push &+0xa9; push −1 -> thiscall, ret 0x10) into
/// `.round_dates()`, and after it Date::Date(0x14, 1, year, 5) (0x549f70) into `.next_draw_day()` with
/// `.next_draw_year_offset()` 0. Returns 1. No SEH frame.
/// Original: `FifaConfed::set_up_rules` (0x58bdb0, fifa_confed.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fifa_confed_set_up_rules(this: usize) -> u32 {
    let league = League(this);
    covered!("cqz_2 confederations cup rules");
    let win = 3; // cqz2: the win in the Confederations Cup groups
    goldcup_write_rule_fields(this, &FIFA_CONFED_RULE_FIELDS, win);
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let t = table(this, 0xffff_ffff, this + league::ROUND_COUNT, this + league::STAGE_ID, 0);
    league.set_round_dates(t);
    let day = 0x14; // cqz2: the day of the Confederations Cup draw date
    let word = competition_date_word(league, day, 1, 0, 5);
    league.set_next_draw_year_offset(0);
    league.set_next_draw_day(word as i16);
    1
}

/// Comparer for `fifa_confed_set_up_rules` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_fifa_confed_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FIFA_CONFED_SET_UP_RULES);
    check_set_up_rules("fifa_confed_set_up_rules", this, &FIFA_CONFED_RULE_FIELDS, (0x14, 1, 0, 5), 0, || o(this))
}

/// The rule fields of the World Cup finals (offset, width, value), in the original's write order: those of the
/// European Championship plus `+0xc8` 0.
pub(crate) const WORLD_CUP_RULE_FIELDS: [(usize, u8, u32); 16] = [(0x43, 1, 2), (0xc7, 1, 2), (0xbe, 1, 2), (0xc2, 1, 3), (0x4a, 1, 3), (0x3c, 2, 1), (0xc3, 1, 1),
    (0xc4, 1, 0xa), (0xc5, 1, 4), (0xc6, 1, 1), (0xc8, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x3a, 2, 0x3e8), (0x49, 1, 7)];

/// rule: the World Cup groups are single round robin, 3/1 points, tiebreak codes 4, 1 and 2, two qualifiers per group,
/// national teams, category 10, 7 substitutes and 3 changes; the seeding date is 15 December and the draw date 27
/// December of the year before the finals (month 0xb, year offset −1, weekday −1: the exact day).
///
/// Writes `WORLD_CUP_RULE_FIELDS` (the win at `+0xc2`), Date::Date(0xf, 0xb, year, −1) (0x549f70: lea ecx, local; 4
/// pushes -> thiscall, ret 0x10) into `world_cup_fields::SEEDING_DAY` with `SEEDING_YEAR_OFFSET` −1, then
/// Date::Date(0x1b, 0xb, year, −1) into `.next_draw_day()` with `.next_draw_year_offset()` −1 and the stage −1 round
/// table (virtual +0x3c) into `.round_dates()`. Both dates are computed in the finals year itself (the offset −1 is
/// applied by the readers). Returns 1.
/// Original: `WorldCup::set_up_rules` (0x92d200, world_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_cup_set_up_rules(this: usize) -> u32 {
    covered!("cqz_2 world cup rules");
    let win = 3; // cqz2: the win in the World Cup groups
    goldcup_write_rule_fields(this, &WORLD_CUP_RULE_FIELDS, win);
    let word = competition_date_word(League(this), 0xf, 0xb, 0, 0xffff_ffff);
    write_u16(this + world_cup_fields::SEEDING_DAY, word);
    write_u16(this + world_cup_fields::SEEDING_YEAR_OFFSET, 0xffff);
    let day = 0x1b; // cqz2: the day of the World Cup draw date
    let word = competition_date_word(League(this), day, 0xb, 0, 0xffff_ffff);
    finish_rules_setup(League(this), word, 0xffff)
}

/// Comparer for `world_cup_set_up_rules`: original only, then the fields (not `+0x3a`, which the round table at the end
/// rewrites with the stage id), both dates and the return value.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WORLD_CUP_SET_UP_RULES);
    let r = check_set_up_rules("world_cup_set_up_rules", this, &WORLD_CUP_RULE_FIELDS.iter().copied().filter(|f| f.0 != 0x3a).collect::<Vec<_>>(), (0x1b, 0xb, 0, 0xffff_ffff), 0xffff, || o(this));
    let word = competition_date_word(League(this), 0xf, 0xb, 0, 0xffff_ffff);
    let (day, offset) = (read_u16(this + world_cup_fields::SEEDING_DAY), read_u16(this + world_cup_fields::SEEDING_YEAR_OFFSET));
    SCOREBOARD.record(day == word && offset == 0xffff, || format!("world_cup_set_up_rules({:#x}): seeding date {:#x} expected {:#x}, offset {:#x}", this, day, word, offset));
    r
}

// ---------- cqz_3: the constructors of the Asian Cup, the Olympic football and the Confederations Cup (asia_nations.cpp, olympics.cpp, fifa_confed.cpp) ----------

/// The edition year of a tournament held every four years: from `start`, goes up one year at a time while the remainder
/// of (year − `base`) by 4 is not 0 (signed, as the original's `and 0x80000003` with its sign adjustment).
fn four_year_edition(start: i16, base: i32) -> i16 {
    let mut year = start;
    while (year as i32 - base) % 4 != 0 {
        year = year.wrapping_add(1);
    }
    year
}

/// The Asian Cup at career creation: the edition is the game's year moved to the next year of the four-year cycle from
/// 1996 (1996, 2000, 2004...: in a 2001 game, 2004); vtable 0x967614, 3 stages, the rules AsiaNations::set_up_rules
/// (0x40f4c0) and the error boxes of lines 0x53 and 0x5a of asia_nations.cpp (`build_nations_tournament`: League::League,
/// the record, `club_sides` 4, no upper or lower division, discipline rule 0, importance 0xf; the saved state if it
/// loads; otherwise `league_flags` 0x28, inactive, the main stage, the stage vector, the rules and the statistics).
/// Original: `AsiaNations::AsiaNations` (0x40e860, asia_nations.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_asia_nations(this: usize, year: u32, record: usize) -> usize {
    covered!("cqz_3 asian cup");
    let edition = four_year_edition(year as i16, 0x7cc); // cqz3: the Asian Cup of the four-year cycle from 1996
    build_nations_tournament(League(this), edition, record, 0x967614, 3, 0x40f4c0, 0x987ba0, (0x53, 0x5a))
}

/// Comparer for `construct_asia_nations`: only the original runs (it allocates the stages and the statistics), then the
/// plan (`compare_nations_tournament_ctor`: this, vtable, record, edition year and importance).
pub(crate) unsafe extern "thiscall" fn compare_construct_asia_nations(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ASIA_NATIONS);
    compare_nations_tournament_ctor("construct_asia_nations", League(this), record, 0x967614, four_year_edition(year as i16, 0x7cc), || o(this, year, record))
}

/// The Olympic football at career creation: the edition is the game's year moved to the next Olympic year (the
/// four-year cycle from 1996: in a 2001 game, 2004); vtable 0x96e1d0, 4 stages, the rules Olympics::set_up_rules
/// (0x7a0230) and the error boxes of lines 0x56 and 0x5d of olympics.cpp (`build_nations_tournament`, the same body as
/// the Asian Cup's).
/// Original: `Olympics::Olympics` (0x79f530, olympics.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_olympics(this: usize, year: u32, record: usize) -> usize {
    covered!("cqz_3 olympics");
    let edition = four_year_edition(year as i16, 0x7cc); // cqz3: the Olympic year
    build_nations_tournament(League(this), edition, record, 0x96e1d0, 4, 0x7a0230, 0xa6a360, (0x56, 0x5d))
}

/// Comparer for `construct_olympics` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_olympics(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_OLYMPICS);
    compare_nations_tournament_ctor("construct_olympics", League(this), record, 0x96e1d0, four_year_edition(year as i16, 0x7cc), || o(this, year, record))
}

/// The Confederations Cup edition: from `start`, one year more while the year is even or is 2001 (an odd year other
/// than 2001: in a 2001 game, 2003).
fn confed_cup_edition(start: i16) -> i16 {
    let mut year = start;
    while (year as i32) % 2 == 0 || year == 0x7d1 {
        year = year.wrapping_add(1);
    }
    year
}

/// The Confederations Cup at career creation.
///
/// League::League (`exe::new_league`, mov ecx -> thiscall, ret 0); vtable 0x96a270, the record (`league::COMPETITION`),
/// `club_sides` 4, no upper or lower division (−1), the edition (`confed_cup_edition` of the game's year), discipline
/// rule 0 and importance 0xf. If the saved state loads (`exe::league_load_from_temp_file(this, 1)`: push 1; mov ecx ->
/// thiscall, ret 4; nonzero), returns this.
///
/// Otherwise: inactive, the main stage (0xff), `current_stage` −1, 2 stages and the stage vector (`exe::crt_malloc(8)`:
/// push; add esp -> cdecl, unchecked); then, unlike its sisters, the statistics before the rules: operator new
/// (`exe::crt_operator_new(0x5ce)`) and Comp_Stats::Comp_Stats (`exe::new_comp_stats(new, this)`: push this; mov ecx,
/// new -> thiscall, ret 4) into `comp_stats`; null: the error box of line 0x53 of fifa_confed.cpp and no rules;
/// otherwise the rules FifaConfed::set_up_rules (0x58bdb0, thiscall, ret 0; its return value is not read). The rules set
/// `league_flags` (0x28). Returns this.
/// Original: `FifaConfed::FifaConfed` (0x58b310, fifa_confed.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_fifa_confed(this: usize, year: u32, record: usize) -> usize {
    let league = League(this);
    exe::new_league(league);
    write_u32(this, 0x96a270);
    write_u32(this + league::COMPETITION, record as u32);
    league.set_club_sides(4);
    write_u32(this + league::UPPER_DIVISION_ID, 0xffff_ffff);
    write_u32(this + league::LOWER_DIVISION_ID, 0xffff_ffff);
    let edition = confed_cup_edition(year as i16); // cqz3: the Confederations Cup in an odd year after 2001
    league.set_year(edition);
    league.set_discipline_rule(0);
    league.set_importance(0xf);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this;
    }
    covered!("cqz_3 confederations cup");
    write_u32(this + league::ACTIVE, 0);
    league.set_stage_index(stage_index::MAIN);
    write_u32(this + league::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this + league::STAGE_COUNT, 2);
    write_u32(this + league::STAGES, exe::crt_malloc(8) as u32);
    let p = exe::crt_operator_new(0x5ce);
    let e = if p != 0 { exe::new_comp_stats(p, league) } else { 0 };
    write_u32(this + league::COMP_STATS, e as u32);
    if e == 0 {
        show_error_at(0x9c7474, 0x53);
    } else {
        let set_up_rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x58bdb0usize);
        set_up_rules(this);
    }
    this
}

/// Comparer for `construct_fifa_confed` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_fifa_confed(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_FIFA_CONFED);
    compare_nations_tournament_ctor("construct_fifa_confed", League(this), record, 0x96a270, confed_cup_edition(year as i16), || o(this, year, record))
}

// ---------- cqz_4: the constructors of the Asian, Oceania and South American World Cup qualifying (wc_asia_league.cpp, wc_oceania_league.cpp, wc_south_american_league.cpp) ----------

/// Calls a routine of the qualifying section by its address (thiscall, ret 0; through the hook when it is in Rust).
unsafe fn call_qualifier_routine(routine: usize, this: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(routine);
    f(this)
}

/// The first year of a World Cup qualifying campaign: a year one past the four-year cycle from 2000 goes back one year
/// (2001 → 2000); then up one year at a time until the remainder of (year − 2000) by 4 is 0 (signed, as the
/// original's `and 0x80000003` with its sign adjustment).
fn qualifying_campaign_year(year: i16) -> i16 {
    let mut y = if (year as i32 - 0x7d0) % 4 == 1 { year.wrapping_sub(1) } else { year };
    while (y as i32 - 0x7d0) % 4 != 0 {
        y = y.wrapping_add(1);
    }
    y
}

/// The fields every qualifying section writes before its saved state loads: League::League (`exe::new_league`, mov ecx
/// -> thiscall, ret 0), the vtable, the record (`league::COMPETITION`), `club_sides` 4, the sections above and below
/// (the competition ids read from the keys `upper` and `lower`: the qualifying sections are chained), discipline rule 0
/// and the campaign year.
unsafe fn start_qualifying_section(this: League, vtable: usize, record: usize, upper: usize, lower: usize, year: i16) {
    exe::new_league(this);
    write_u32(this.addr(), vtable as u32);
    write_u32(this.addr() + league::COMPETITION, record as u32);
    this.set_club_sides(4);
    write_u32(this.addr() + league::UPPER_DIVISION_ID, read_u32(upper));
    write_u32(this.addr() + league::LOWER_DIVISION_ID, read_u32(lower));
    this.set_discipline_rule(0);
    this.set_year(year);
}

/// The statistics of a qualifying section: operator new (`exe::crt_operator_new(0x5ce)`: push; add esp -> cdecl) and
/// Comp_Stats::Comp_Stats (`exe::new_comp_stats(new, this)`: push this; mov ecx, new -> thiscall, ret 4) into
/// `comp_stats`; null: the error box of line `row` of source `file`. Returns whether they were created.
unsafe fn qualifying_section_stats(this: League, file: usize, row: u32) -> bool {
    let p = exe::crt_operator_new(0x5ce);
    let e = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    write_u32(this.addr() + league::COMP_STATS, e as u32);
    if e == 0 {
        show_error_at(file, row);
    }
    e != 0
}

/// Shared comparer of the three constructors: they allocate the stages, the statistics and the entrant lists and read
/// the save, so only the original runs; then the plan: it returns this, the vtable, the record, the edition year and
/// the chained sections above and below.
unsafe fn compare_qualifying_ctor(name: &str, this: League, record: usize, vtable: usize, year: i16, upper: usize, lower: usize, o: impl FnOnce() -> usize) -> usize {
    let r = o();
    let read = this.year();
    let (u, l) = (read_u32(this.addr() + league::UPPER_DIVISION_ID), read_u32(this.addr() + league::LOWER_DIVISION_ID));
    let ok = r == this.addr() && read_usize(this.addr()) == vtable && this.competition().addr() == record && read == year && u == read_u32(upper) && l == read_u32(lower);
    SCOREBOARD.record(ok, || format!("{}({:#x}): year {} expected {}, vtable {:#x}, sections {} {}", name, this.addr(), read, year, read_usize(this.addr()), u as i32, l as i32));
    r
}

/// The Asian World Cup qualifying at career creation.
///
/// `start_qualifying_section` (vtable 0x970b70; above, the CONCACAF section; below, the Oceania section), 7 substitutes
/// (`+0x49`), 3 substitutions (`+0x4a`), the dword `+0xf7` = 0, the campaign year (`qualifying_campaign_year`, never
/// before 2004) and importance 0xa. If the saved state loads (`exe::league_load_from_temp_file(this, 1)`), returns
/// this.
///
/// Otherwise: `league_flags` 0x28, inactive, the entrant list (WcAsiaLeague::build_entrant_list 0x913f60, thiscall, ret
/// 0; its return value is not read), the main stage, 13 stages (`current_stage` −1) and the stage vector
/// (`exe::crt_malloc(0x34)`, unchecked); the rules (WcAsiaLeague::set_up_rules 0x912eb0; 0: the error box of line 0x6f
/// and returns this) and the statistics (null: the box of line 0x77). Returns this.
/// Original: `WcAsiaLeague::WcAsiaLeague` (0x911bf0, wc_asia_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wc_asia_league(this: usize, year: u32, record: usize) -> usize {
    let league = League(this);
    let edition = qualifying_campaign_year(year as i16).max(0x7d4); // cqz4: the Asian campaign, from 2004
    start_qualifying_section(league, 0x970b70, record, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, edition);
    write_u8(this + league::SUBSTITUTES_NAMED, 7);
    write_u8(this + league::SUBSTITUTIONS_ALLOWED, 3);
    write_u32(this + 0xf7, 0);
    league.set_importance(0xa);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this;
    }
    covered!("cqz_4 asian qualifying");
    league.set_league_flags(0x28);
    write_u32(this + league::ACTIVE, 0);
    call_qualifier_routine(0x913f60, this);
    league.set_stage_index(stage_index::MAIN);
    write_u32(this + league::STAGE_COUNT, 0xd);
    write_u32(this + league::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this + league::STAGES, exe::crt_malloc(0x34) as u32);
    if call_qualifier_routine(0x912eb0, this) == 0 {
        show_error_at(0xad4680, 0x6f);
        return this;
    }
    qualifying_section_stats(league, 0xad4680, 0x77);
    this
}

/// Comparer for `construct_wc_asia_league` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_wc_asia_league(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WC_ASIA_LEAGUE);
    compare_qualifying_ctor("construct_wc_asia_league", League(this), record, 0x970b70, qualifying_campaign_year(year as i16).max(0x7d4),
        KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, || o(this, year, record))
}

/// The Oceania World Cup qualifying at career creation.
///
/// `start_qualifying_section` (vtable 0x970d78; above, the Asian section; below, the South American section), the
/// campaign year (`qualifying_campaign_year`, no minimum), inactive and importance 0xa. If the saved state loads,
/// returns this.
///
/// Otherwise: `league_flags` 0x28, the entrant pool (WcOceaniaLeague::build_entrant_pool 0x91fa80, thiscall, ret 0;
/// return value not read), 3 stages, then the statistics (null: the error box of line 0x63 and returns this); the
/// predicted positions (WcOceaniaLeague::set_predicted_positions 0x91ff30, thiscall, ret 0), the main stage,
/// `current_stage` −1 and the stage vector (`exe::crt_malloc(4 × stage_count)` when the count is positive, else null);
/// the rules (WcOceaniaLeague::set_up_rules 0x91ecf0; 0: the box of line 0x75) and the group table
/// (WcOceaniaLeague::init_group_table 0x91ec00; 0: the box of line 0x7c). Returns this.
/// Original: `WcOceaniaLeague::WcOceaniaLeague` (0x91df90, wc_oceania_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wc_oceania_league(this: usize, year: u32, record: usize) -> usize {
    let league = League(this);
    let edition = qualifying_campaign_year(year as i16); // cqz4: the Oceania campaign
    start_qualifying_section(league, 0x970d78, record, KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT, edition);
    write_u32(this + league::ACTIVE, 0);
    let importance = 0xa; // cqz4: the importance of the Oceania qualifying matches
    league.set_importance(importance);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this;
    }
    covered!("cqz_4 oceania qualifying");
    league.set_league_flags(0x28);
    call_qualifier_routine(0x91fa80, this);
    write_u32(this + league::STAGE_COUNT, 3);
    if !qualifying_section_stats(league, 0xad4afc, 0x63) {
        return this;
    }
    call_qualifier_routine(0x91ff30, this);
    league.set_stage_index(stage_index::MAIN);
    write_u32(this + league::CURRENT_STAGE, 0xffff_ffff);
    let n = read_u32(this + league::STAGE_COUNT) as i32;
    write_u32(this + league::STAGES, if n > 0 { exe::crt_malloc((n as u32) << 2) as u32 } else { 0 });
    if call_qualifier_routine(0x91ecf0, this) == 0 {
        show_error_at(0xad4afc, 0x75);
        return this;
    }
    if call_qualifier_routine(0x91ec00, this) == 0 {
        show_error_at(0xad4afc, 0x7c);
    }
    this
}

/// Comparer for `construct_wc_oceania_league` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_wc_oceania_league(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WC_OCEANIA_LEAGUE);
    compare_qualifying_ctor("construct_wc_oceania_league", League(this), record, 0x970d78, qualifying_campaign_year(year as i16),
        KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT, || o(this, year, record))
}

/// The South American World Cup qualifying at career creation.
///
/// `start_qualifying_section` (vtable 0x970e2c; above, the Oceania section; below, the African section), inactive and
/// the campaign year (`qualifying_campaign_year`, never before 2004); the importance is not written (it stays as
/// League::League left it). If the saved state loads, returns this.
///
/// Otherwise: `league_flags` 0x24, the main stage, `current_stage` −1, no stages (count 0, null vector); the exempt
/// nations (WcSouthAmericanLeague::count_exempt_nations 0x920d10, thiscall, ret 0); the rules
/// (WcSouthAmericanLeague::set_up_rules 0x921040; 0: the error box of line 0x65 and returns this); the team slots
/// (WcSouthAmericanLeague::build_team_slots 0x920eb0; 0: the box of line 0x6c and returns this); the statistics (null:
/// the box of line 0x73 and returns this). Then, when the game's year (the argument, signed word) is not before the
/// edition, the real 2000-2001 campaign (WcSouthAmericanLeague::load_2002_campaign 0x9216d0, thiscall, ret 0). Returns
/// this.
/// Original: `WcSouthAmericanLeague::WcSouthAmericanLeague` (0x9202b0, wc_south_american_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wc_south_american_league(this: usize, year: u32, record: usize) -> usize {
    let league = League(this);
    let edition = qualifying_campaign_year(year as i16).max(0x7d4); // cqz4: the South American campaign, from 2004
    start_qualifying_section(league, 0x970e2c, record, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION, edition);
    write_u32(this + league::ACTIVE, 0);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this;
    }
    covered!("cqz_4 south american qualifying");
    league.set_league_flags(0x24);
    league.set_stage_index(stage_index::MAIN);
    write_u32(this + league::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this + league::STAGE_COUNT, 0);
    write_u32(this + league::STAGES, 0);
    call_qualifier_routine(0x920d10, this);
    if call_qualifier_routine(0x921040, this) == 0 {
        show_error_at(0xad4ce0, 0x65);
        return this;
    }
    if call_qualifier_routine(0x920eb0, this) == 0 {
        show_error_at(0xad4ce0, 0x6c);
        return this;
    }
    if !qualifying_section_stats(league, 0xad4ce0, 0x73) {
        return this;
    }
    if year as i16 >= league.year() {
        covered!("cqz_4 south american 2002 campaign");
        call_qualifier_routine(0x9216d0, this);
    }
    this
}

/// Comparer for `construct_wc_south_american_league` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_wc_south_american_league(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WC_SOUTH_AMERICAN_LEAGUE);
    compare_qualifying_ctor("construct_wc_south_american_league", League(this), record, 0x970e2c, qualifying_campaign_year(year as i16).max(0x7d4),
        KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION, || o(this, year, record))
}

// ---------- cqz_5: the daily update of the Asian Cup, the Olympic football and the Confederations Cup (asia_nations.cpp, olympics.cpp, fifa_confed.cpp) ----------

/// The shape of a national-team tournament's day: the routines of its draw day and the original's quirks.
struct NationsDayPlan {
    /// Without `full`, the league day runs at once (the Asian Cup); otherwise the date test comes first.
    light_day_first: bool,
    /// The draw only runs on a day update without `full` (the Olympics and the Confederations Cup).
    draw_needs_light_day: bool,
    /// The draw of the finalists (thiscall, ret 0; its return value is not read).
    draw: usize,
    /// The first group table (thiscall, ret 0; 0 = error).
    group_table: usize,
    /// The source file of the error boxes and the lines of the group table, fixtures and sort errors.
    file: usize,
    rows: [u32; 3],
    /// The group stage routine and how many it builds: `Some(n)`, push k; thiscall, ret 4 for k = 0..n; `None`, one
    /// call without arguments (thiscall, ret 0).
    group_stage: usize,
    group_stages: Option<u32>,
    /// The entrant count +0xf2 is zeroed as a dword (not a word).
    count_as_dword: bool,
}

/// The day of a national-team tournament with a draw, groups and knockout (the virtual `comp_vtable::UPDATE_FOR_DATE`;
/// the SEH frame only covers the local dates).
///
/// With `light_day_first` and no `full`, only the league day (`League::update_for_date`(this, today, 0): 2 pushes; mov
/// ecx -> thiscall, ret 8). Otherwise the draw date (`oceania_nations_draw_date`: 0x549ef0 on `.next_draw_day()` and
/// `.next_draw_year_offset()` + `.year()`): before it (today's year lower, unsigned; or the same year and today's day
/// lower, signed) returns 0 without the league day. On the draw day, inactive (`league::ACTIVE` 0) and, with
/// `draw_needs_light_day`, only without `full`: the draw, the predicted positions (the virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`, thiscall, ret 0), the first group table (0: the box of `rows[0]` and 0), the
/// fixtures (`exe::league_create_fixtures`; 0: the box of `rows[1]` and 0), the sorted table (`exe::league_sort_table`
/// (this, 0): push 0 -> thiscall, ret 4; 0: the box of `rows[2]` and 0), the other group stages, the entrant list
/// +0xee freed (`exe::crt_free`: push; add esp -> cdecl) and zeroed, its count +0xf2 zeroed and `league::ACTIVE` = 1.
/// Then the league day with `full`, whose return value (al; the callers read the byte) it passes on.
unsafe fn nations_tournament_day(this: League, today: usize, full: u32, plan: &NationsDayPlan) -> u32 {
    // the full eax is passed on (the exe:: wrapper reads only the byte)
    let league_day: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    if plan.light_day_first && full == 0 {
        return league_day(this.addr(), today, 0) as u32;
    }
    let (day, year) = oceania_nations_draw_date(this);
    let (today_day, today_year) = (read_u16(today), read_u16(today + 2));
    // simplification: on the returns before the draw and on the errors the original returns only al (0); the caller reads the byte
    if today_year < year || (today_year == year && (today_day as i16) < day as i16) {
        return 0;
    }
    if today_year == year && today_day == day && this.active() == 0 && (!plan.draw_needs_light_day || full == 0) {
        let draw: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(plan.draw);
        let group_table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(plan.group_table);
        draw(this.addr());
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this.addr());
        if group_table(this.addr()) == 0 {
            show_error_at(plan.file, plan.rows[0]);
            return 0;
        }
        if exe::league_create_fixtures(this) == 0 {
            show_error_at(plan.file, plan.rows[1]);
            return 0;
        }
        if exe::league_sort_table(this, 0) == 0 {
            show_error_at(plan.file, plan.rows[2]);
            return 0;
        }
        match plan.group_stages {
            Some(n) => {
                let stage: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(plan.group_stage);
                for k in 0..n {
                    stage(this.addr(), k);
                }
            }
            None => {
                let stage: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(plan.group_stage);
                stage(this.addr());
            }
        }
        exe::crt_free(read_usize(this.addr() + 0xee));
        write_u32(this.addr() + 0xee, 0);
        if plan.count_as_dword {
            write_u32(this.addr() + 0xf2, 0);
        } else {
            write_u16(this.addr() + 0xf2, 0);
        }
        this.set_active(1);
    }
    league_day(this.addr(), today, full) as u32
}

/// Shared comparer: the draw, the tables, the fixtures and the league day cannot run twice, so only the original runs;
/// on the draw day (the copy's condition) the plan: `league::ACTIVE` = 1, the list +0xee and its count zeroed, the
/// first group table and the first group stage created.
unsafe fn compare_nations_tournament_day(name: &str, this: League, today: usize, full: u32, plan: &NationsDayPlan, o: impl FnOnce() -> u32) -> u32 {
    let draws = !(plan.light_day_first && full == 0) && this.active() == 0 && (!plan.draw_needs_light_day || full == 0) && {
        let (day, year) = oceania_nations_draw_date(this);
        read_u16(today) == day && read_u16(today + 2) == year
    };
    let r = o();
    if draws {
        let stages = this.stages();
        let count = if plan.count_as_dword { read_u32(this.addr() + 0xf2) } else { read_u16(this.addr() + 0xf2) as u32 };
        let ok = this.active() == 1 && read_usize(this.addr() + 0xee) == 0 && count == 0 && this.table() != 0 && stages != 0 && read_usize(stages) != 0;
        SCOREBOARD.record(ok, || format!("{}({:#x}): the draw did not build the groups (+0x45 {})", name, this.addr(), this.active()));
    }
    r
}

/// The Asian Cup day: the draw of the finalists (AsiaNations::select_and_draw_teams 0x40fdf0), group A's table
/// (AsiaNations::create_group_table 0x40f3d0) and the stages of the two other groups (AsiaNations::create_group_stage
/// 0x40f8a0 with 0 and 1); the boxes of lines 0x3dd, 0x3e3 and 0x3ea of asia_nations.cpp; the count +0xf2 zeroed as a
/// word; without `full`, the league day at once.
const ASIA_NATIONS_DAY: NationsDayPlan = NationsDayPlan { light_day_first: true, draw_needs_light_day: false, draw: 0x40fdf0, group_table: 0x40f3d0,
    file: 0x987ba0, rows: [0x3dd, 0x3e3, 0x3ea], group_stage: 0x40f8a0, group_stages: Some(2), count_as_dword: false };

/// rule: on the draw day of the Asian Cup (5 June of the cup year: day 5 of month 5, months from 0) the finalists are drawn into three groups, group A
/// gets its table and fixtures and the other two groups their stages (`nations_tournament_day`)
/// Original: `AsiaNations::update_for_date` (0x410170, asia_nations.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn asia_nations_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    covered!("cqz_5 asian cup day");
    nations_tournament_day(League(this), today, full, &ASIA_NATIONS_DAY)
}

/// Comparer for `asia_nations_update_for_date` (original only, then the plan on the draw day).
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ASIA_NATIONS_UPDATE_FOR_DATE);
    compare_nations_tournament_day("asia_nations_update_for_date", League(this), today, full, &ASIA_NATIONS_DAY, || o(this, today, full))
}

/// The Olympic day: the date test even without `full`, and the draw only on a day without `full` (a quirk of the
/// original, as in the Confederations Cup); the draw of the finalists (Olympics::draw_finalists 0x7a09e0), group A's
/// table (Olympics::init_group_a_table 0x7a0140) and the stages of groups B to D (Olympics::create_group_stage 0x7a05d0
/// with 0, 1 and 2); the boxes of lines 0x41b, 0x421 and 0x428 of olympics.cpp; the count +0xf2 zeroed as a dword.
const OLYMPICS_DAY: NationsDayPlan = NationsDayPlan { light_day_first: false, draw_needs_light_day: true, draw: 0x7a09e0, group_table: 0x7a0140,
    file: 0xa6a360, rows: [0x41b, 0x421, 0x428], group_stage: 0x7a05d0, group_stages: Some(3), count_as_dword: true };

/// rule: on the draw day of the Olympic football (20 March of the Olympic year: day 0x14 of month 2, months from 0) the finalists are drawn into four
/// groups, group A gets its table and fixtures and groups B to D their stages; before the draw day nothing runs, not
/// even the league day (`nations_tournament_day`)
/// Original: `Olympics::update_for_date` (0x7a11c0, olympics.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn olympics_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    covered!("cqz_5 olympics day");
    nations_tournament_day(League(this), today, full, &OLYMPICS_DAY)
}

/// Comparer for `olympics_update_for_date` (original only, then the plan on the draw day).
pub(crate) unsafe extern "thiscall" fn compare_olympics_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_OLYMPICS_UPDATE_FOR_DATE);
    compare_nations_tournament_day("olympics_update_for_date", League(this), today, full, &OLYMPICS_DAY, || o(this, today, full))
}

/// The Confederations Cup day: like the Olympic one (the date test even without `full`, the draw only without it); the
/// selection of the nations (FifaConfed::select_teams 0x58c6b0), the first group's table (FifaConfed::create_group_table
/// 0x58bcb0) and the second group's stage (FifaConfed::create_group_stage 0x58c100, no argument); the boxes of lines
/// 0x32b, 0x332 and 0x339 of fifa_confed.cpp; the count +0xf2 zeroed as a dword.
const FIFA_CONFED_DAY: NationsDayPlan = NationsDayPlan { light_day_first: false, draw_needs_light_day: true, draw: 0x58c6b0, group_table: 0x58bcb0,
    file: 0x9c7474, rows: [0x32b, 0x332, 0x339], group_stage: 0x58c100, group_stages: None, count_as_dword: true };

/// rule: on the draw day of the Confederations Cup (20 February of the cup year: day 0x14 of month 1) the nations are selected and drawn
/// into two groups, the first gets its table and fixtures and the second its stage (`nations_tournament_day`)
/// Original: `FifaConfed::update_for_date` (0x58c4b0, fifa_confed.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn fifa_confed_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    covered!("cqz_5 confederations cup day");
    nations_tournament_day(League(this), today, full, &FIFA_CONFED_DAY)
}

/// Comparer for `fifa_confed_update_for_date` (original only, then the plan on the draw day).
pub(crate) unsafe extern "thiscall" fn compare_fifa_confed_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_FIFA_CONFED_UPDATE_FOR_DATE);
    compare_nations_tournament_day("fifa_confed_update_for_date", League(this), today, full, &FIFA_CONFED_DAY, || o(this, today, full))
}

// ---------- cqz_6: the round tables of the Asian Cup, the Oceania Nations Cup and the Olympic football (asia_nations.cpp, oceania_nations.cpp, olympics.cpp) ----------

/// cqz_6 asian cup round table, the operations of round table 1 (from the original, in order).
const ASIA_NATIONS_OPS_1: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 0xd, 9, 0, 4, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 9, 0, 0, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 9, 0, 3, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];
/// cqz_6 asian cup round table, the operations of round table 2 (from the original, in order).
const ASIA_NATIONS_OPS_2: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 9, 0, 4, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 9, 0, 0, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 9, 0, 3, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 3, 1, 3, 4), Cqp12L(0, 1, 3, 2, 3, 0x40), Cqp12L(1, 0, 6, 1, 3, 4), Cqp12L(1, 1, 6, 2, 3, 0x40),
    Cqp12L(2, 0, 2, 1, 3, 4), Cqp12L(2, 1, 2, 2, 3, 0x40),
];
/// cqz_6 asian cup round table, the operations of round table 3 (from the original, in order).
const ASIA_NATIONS_OPS_3: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 9, 0, 4, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 9, 0, 0, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 9, 0, 3, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 4, 1, 3, 4), Cqp12L(0, 1, 4, 2, 3, 0x40), Cqp12L(1, 0, 0, 1, 3, 4), Cqp12L(1, 1, 0, 2, 3, 0x40),
    Cqp12L(2, 0, 3, 1, 3, 4), Cqp12L(2, 1, 3, 2, 3, 0x40),
];
/// cqz_6 asian cup round table, the operations of round table 4 (from the original, in order).
const ASIA_NATIONS_OPS_4: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 9, 0, 4, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 9, 0, 0, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 9, 0, 3, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 5, 1, 3, 4), Cqp12L(0, 1, 5, 2, 3, 0x40), Cqp12L(1, 0, 1, 1, 3, 4), Cqp12L(1, 1, 1, 2, 3, 0x40),
    Cqp12L(2, 0, 4, 1, 3, 4), Cqp12L(2, 1, 4, 2, 3, 0x40),
];
/// cqz_6 asian cup round table, the operations of round table 5 (from the original, in order).
const ASIA_NATIONS_OPS_5: [GoldcupScheduleOp; 80] = [
    Cqp12S(0, 0x15, 9, 0, 5), Cqp12J(0, 0x17, 9, 0, 0, 1, 1), Cqp12E(0, 0, 0, 2, 3, 0x40), Cqp12E(0, 1, 0, 1, 3, 4),
    Cqp12E(0, 2, 1, 2, 3, 0x40), Cqp12E(0, 3, 1, 1, 3, 4), Cqp12E(0, 4, -1, -1, -1, 0), Cqp12W(0x7, 0x78),
    Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x303), Cqp12B(0x17, 0xa), Cqp12W(0x18, 0x8), Cqp12W(0x1a, 0x4),
    Cqp12W(0x1c, 0x8), Cqp12W(0xf, 0x0), Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x1), Cqp12B(0x22, 0x0),
    Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0), Cqp12X(0x64, 0x0), Cqp12S(1, 0x19, 9, 0, 2), Cqp12J(1, 0x1a, 9, 0, 3, 2, 1),
    Cqp12E(1, 0, 3, 2, 3, 0x40), Cqp12E(1, 1, 3, 1, 3, 4), Cqp12E(1, 2, -1, -1, -1, 0), Cqp12W(0x6f, 0x82),
    Cqp12W(0x71, 0x1), Cqp12W(0x73, 0x0), Cqp12W(0x75, 0x3), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 0x4), Cqp12W(0x82, 0x2),
    Cqp12W(0x84, 0x0), Cqp12W(0x77, 0x0), Cqp12W(0x86, 0x0), Cqp12B(0x88, 0x0), Cqp12B(0x89, 0x1), Cqp12B(0x8a, 0x0),
    Cqp12X(0xc4, 0x0), Cqp12X(0xc8, 0x0), Cqp12X(0xcc, 0x0), Cqp12S(2, 0x1b, 9, 0, 4), Cqp12J(2, 0x1d, 9, 0, 6, 2, 4),
    Cqp12W(0xd7, 0x96), Cqp12W(0xd9, 0x2), Cqp12W(0xdb, 0x0), Cqp12W(0xdd, 0x3), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 0x2),
    Cqp12W(0xea, 0x1), Cqp12W(0xec, 0x0), Cqp12W(0xdf, 0x0), Cqp12W(0xee, 0x0), Cqp12B(0xf0, 0x0), Cqp12B(0xf1, 0x1),
    Cqp12B(0xf2, 0x0), Cqp12X(0x12c, 0x0), Cqp12X(0x130, 0x0), Cqp12X(0x134, 0x0), Cqp12S(3, 0x1b, 9, 0, 4),
    Cqp12J(3, 0x1d, 9, 0, 6, 1, 0x40), Cqp12W(0x150, 0x2), Cqp12W(0x141, 0x3), Cqp12W(0x145, 0x3),
    Cqp12W(0x13f, 0x8c), Cqp12W(0x143, 0x0), Cqp12B(0x14f, 0xa), Cqp12W(0x152, 0x1), Cqp12W(0x154, 0x0),
    Cqp12W(0x147, 0x0), Cqp12W(0x156, 0x0), Cqp12B(0x158, 0x0), Cqp12B(0x159, 0x1), Cqp12B(0x15a, 0x0),
    Cqp12X(0x194, 0x0), Cqp12X(0x198, 0x0), Cqp12X(0x19c, 0x0),
];

/// The round table of stage `stage` (signed byte) of cqz_6 asian cup round table, or none.
fn asia_nations_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=-2 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: (f as i16).wrapping_add(0x420), extra_check: false, size: 0xc3, row: 0x99,
            ops: &ASIA_NATIONS_OPS_1, ops2: &[], name: "cqz_6 asian cup round table" }),
        -1 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x41f, extra_check: false, size: 0xc3, row: 0x99,
            ops: &ASIA_NATIONS_OPS_2, ops2: &[], name: "cqz_6 asian cup round table" }),
        0 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x420, extra_check: false, size: 0xc3, row: 0x99,
            ops: &ASIA_NATIONS_OPS_3, ops2: &[], name: "cqz_6 asian cup round table" }),
        1 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x421, extra_check: false, size: 0xc3, row: 0x99,
            ops: &ASIA_NATIONS_OPS_4, ops2: &[], name: "cqz_6 asian cup round table" }),
        2 => Some(GoldcupStagePlan { league: 0, rounds: 4, extra: 0, extra_check: false, size: 0x1a0, row: 0xd3,
            ops: &ASIA_NATIONS_OPS_5, ops2: &[], name: "cqz_6 asian cup round table" }),
        3..=127 => None,
    }
}

/// cqz_6 oceania nations cup round table, the operations of round table 1 (from the original, in order).
const OCEANIA_NATIONS_EVEN_OPS_1: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 0xa, 0xa, 0, 4, 1, 1), Cqp12L(0, 1, -1, -1, -1, 0), Cqp12D(1, 0xd, 0xa, 0, 0, 1, 1),
    Cqp12L(1, 1, -1, -1, -1, 0), Cqp12D(2, 0xf, 0xa, 0, 2, 1, 1), Cqp12L(2, 1, -1, -1, -1, 0),
];
/// cqz_6 oceania nations cup round table, the operations of round table 2 (from the original, in order).
const OCEANIA_NATIONS_EVEN_OPS_2: [GoldcupScheduleOp; 9] = [
    Cqp12D(0, 0xa, 0xa, 0, 4, 1, 1), Cqp12L(0, 1, -1, -1, -1, 0), Cqp12D(1, 0xd, 0xa, 0, 0, 1, 1),
    Cqp12L(1, 1, -1, -1, -1, 0), Cqp12D(2, 0xf, 0xa, 0, 2, 1, 1), Cqp12L(2, 1, -1, -1, -1, 0),
    Cqp12L(0, 0, 4, 1, 3, 4), Cqp12L(1, 0, 0, 1, 3, 0x40), Cqp12L(2, 0, 2, 1, 3, 4),
];
/// cqz_6 oceania nations cup round table, the operations of round table 3 (from the original, in order).
const OCEANIA_NATIONS_EVEN_OPS_3: [GoldcupScheduleOp; 9] = [
    Cqp12D(0, 0xa, 0xa, 0, 4, 1, 1), Cqp12L(0, 1, -1, -1, -1, 0), Cqp12D(1, 0xd, 0xa, 0, 0, 1, 1),
    Cqp12L(1, 1, -1, -1, -1, 0), Cqp12D(2, 0xf, 0xa, 0, 2, 1, 1), Cqp12L(2, 1, -1, -1, -1, 0),
    Cqp12L(0, 0, 4, 2, 3, 0x40), Cqp12L(1, 0, 0, 2, 3, 4), Cqp12L(2, 0, 2, 2, 3, 0x40),
];
/// cqz_6 oceania nations cup round table, the operations of round table 4 (from the original, in order).
const OCEANIA_NATIONS_EVEN_OPS_4: [GoldcupScheduleOp; 57] = [
    Cqp12S(0, 0x10, 0xa, 0, 3), Cqp12J(0, 0x11, 0xa, 0, 4, 1, 1), Cqp12E(0, 0, 4, 2, 3, 0x40),
    Cqp12E(0, 1, 4, 1, 3, 4), Cqp12E(0, 2, -1, -1, -1, 0), Cqp12W(0x7, 0x82), Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0),
    Cqp12W(0xd, 0x303), Cqp12B(0x17, 0xa), Cqp12W(0x18, 0x4), Cqp12W(0x1a, 0x2), Cqp12W(0x1c, 0x4), Cqp12W(0xf, 0x0),
    Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x1), Cqp12B(0x22, 0x0), Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0),
    Cqp12X(0x64, 0x0), Cqp12S(1, 0x12, 0xa, 0, 5), Cqp12J(1, 0x13, 0xa, 0, 6, 2, 4), Cqp12W(0x6f, 0x96),
    Cqp12W(0x71, 0x1), Cqp12W(0x73, 0x0), Cqp12W(0x75, 0x3), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 0x2), Cqp12W(0x82, 0x1),
    Cqp12W(0x84, 0x0), Cqp12W(0x77, 0x0), Cqp12W(0x86, 0x0), Cqp12B(0x88, 0x0), Cqp12B(0x89, 0x1), Cqp12B(0x8a, 0x0),
    Cqp12X(0xc4, 0x0), Cqp12X(0xc8, 0x0), Cqp12X(0xcc, 0x0), Cqp12S(2, 0x12, 0xa, 0, 5),
    Cqp12J(2, 0x13, 0xa, 0, 6, 1, 0x40), Cqp12W(0xd7, 0x8c), Cqp12W(0xd9, 0x2), Cqp12W(0xdb, 0x0), Cqp12W(0xdd, 0x3),
    Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 0x2), Cqp12W(0xea, 0x1), Cqp12W(0xec, 0x0), Cqp12W(0xdf, 0x0), Cqp12W(0xee, 0x0),
    Cqp12B(0xf0, 0x0), Cqp12B(0xf1, 0x1), Cqp12B(0xf2, 0x0), Cqp12X(0x12c, 0x0), Cqp12X(0x130, 0x0),
    Cqp12X(0x134, 0x0),
];

/// The round table of stage `stage` (signed byte) of cqz_6 oceania nations cup round table, or none.
fn oceania_nations_even_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=-2 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: (f as i16).wrapping_add(0x3e9), extra_check: false, size: 0xc3, row: 0xa0,
            ops: &OCEANIA_NATIONS_EVEN_OPS_1, ops2: &[], name: "cqz_6 oceania nations cup round table" }),
        -1 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3e8, extra_check: false, size: 0xc3, row: 0xa0,
            ops: &OCEANIA_NATIONS_EVEN_OPS_2, ops2: &[], name: "cqz_6 oceania nations cup round table" }),
        0 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3e9, extra_check: false, size: 0xc3, row: 0xa0,
            ops: &OCEANIA_NATIONS_EVEN_OPS_3, ops2: &[], name: "cqz_6 oceania nations cup round table" }),
        1 => Some(GoldcupStagePlan { league: 0, rounds: 3, extra: 0, extra_check: false, size: 0x138, row: 0xcc,
            ops: &OCEANIA_NATIONS_EVEN_OPS_4, ops2: &[], name: "cqz_6 oceania nations cup round table" }),
        2..=127 => None,
    }
}

/// cqz_6 oceania nations cup round table (other years), the operations of round table 1 (from the original, in order).
const OCEANIA_NATIONS_OTHER_OPS_1: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 0x13, 5, 0, 0, 1, 1), Cqp12L(0, 1, -1, -1, -1, 0), Cqp12D(1, 0x15, 5, 0, 2, 1, 1),
    Cqp12L(1, 1, -1, -1, -1, 0), Cqp12D(2, 0x17, 5, 0, 4, 1, 1), Cqp12L(2, 1, -1, -1, -1, 0),
];
/// cqz_6 oceania nations cup round table (other years), the operations of round table 2 (from the original, in order).
const OCEANIA_NATIONS_OTHER_OPS_2: [GoldcupScheduleOp; 9] = [
    Cqp12D(0, 0x13, 5, 0, 0, 1, 1), Cqp12L(0, 1, -1, -1, -1, 0), Cqp12D(1, 0x15, 5, 0, 2, 1, 1),
    Cqp12L(1, 1, -1, -1, -1, 0), Cqp12D(2, 0x17, 5, 0, 4, 1, 1), Cqp12L(2, 1, -1, -1, -1, 0),
    Cqp12L(0, 0, 0, 1, 3, 4), Cqp12L(1, 0, 2, 1, 3, 0x40), Cqp12L(2, 0, 4, 1, 3, 4),
];
/// cqz_6 oceania nations cup round table (other years), the operations of round table 3 (from the original, in order).
const OCEANIA_NATIONS_OTHER_OPS_3: [GoldcupScheduleOp; 9] = [
    Cqp12D(0, 0x13, 5, 0, 0, 1, 1), Cqp12L(0, 1, -1, -1, -1, 0), Cqp12D(1, 0x15, 5, 0, 2, 1, 1),
    Cqp12L(1, 1, -1, -1, -1, 0), Cqp12D(2, 0x17, 5, 0, 4, 1, 1), Cqp12L(2, 1, -1, -1, -1, 0),
    Cqp12L(0, 0, 0, 2, 3, 0x40), Cqp12L(1, 0, 2, 2, 3, 4), Cqp12L(2, 0, 4, 2, 3, 0x40),
];
/// cqz_6 oceania nations cup round table (other years), the operations of round table 4 (from the original, in order).
const OCEANIA_NATIONS_OTHER_OPS_4: [GoldcupScheduleOp; 57] = [
    Cqp12S(0, 0x18, 5, 0, 5), Cqp12J(0, 0x19, 5, 0, 6, 1, 1), Cqp12E(0, 0, 6, 2, 3, 0x40), Cqp12E(0, 1, 6, 1, 3, 4),
    Cqp12E(0, 2, -1, -1, -1, 0), Cqp12W(0x7, 0x82), Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x303),
    Cqp12B(0x17, 0xa), Cqp12W(0x18, 0x4), Cqp12W(0x1a, 0x2), Cqp12W(0x1c, 0x4), Cqp12W(0xf, 0x0), Cqp12W(0x1e, 0x0),
    Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x1), Cqp12B(0x22, 0x0), Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0), Cqp12X(0x64, 0x0),
    Cqp12S(1, 0x1a, 5, 0, 0), Cqp12J(1, 0x1c, 5, 0, 2, 2, 4), Cqp12W(0x6f, 0x96), Cqp12W(0x71, 0x1),
    Cqp12W(0x73, 0x0), Cqp12W(0x75, 0x3), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 0x2), Cqp12W(0x82, 0x1), Cqp12W(0x84, 0x0),
    Cqp12W(0x77, 0x0), Cqp12W(0x86, 0x0), Cqp12B(0x88, 0x0), Cqp12B(0x89, 0x1), Cqp12B(0x8a, 0x0), Cqp12X(0xc4, 0x0),
    Cqp12X(0xc8, 0x0), Cqp12X(0xcc, 0x0), Cqp12S(2, 0x1a, 5, 0, 0), Cqp12J(2, 0x1c, 5, 0, 2, 1, 0x40),
    Cqp12W(0xd7, 0x8c), Cqp12W(0xd9, 0x2), Cqp12W(0xdb, 0x0), Cqp12W(0xdd, 0x3), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 0x2),
    Cqp12W(0xea, 0x1), Cqp12W(0xec, 0x0), Cqp12W(0xdf, 0x0), Cqp12W(0xee, 0x0), Cqp12B(0xf0, 0x0), Cqp12B(0xf1, 0x1),
    Cqp12B(0xf2, 0x0), Cqp12X(0x12c, 0x0), Cqp12X(0x130, 0x0), Cqp12X(0x134, 0x0),
];

/// The round table of stage `stage` (signed byte) of cqz_6 oceania nations cup round table (other years), or none.
fn oceania_nations_other_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=-2 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: (f as i16).wrapping_add(0x3e9), extra_check: false, size: 0xc3, row: 0x11f,
            ops: &OCEANIA_NATIONS_OTHER_OPS_1, ops2: &[], name: "cqz_6 oceania nations cup round table (other years)" }),
        -1 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3e8, extra_check: false, size: 0xc3, row: 0x11f,
            ops: &OCEANIA_NATIONS_OTHER_OPS_2, ops2: &[], name: "cqz_6 oceania nations cup round table (other years)" }),
        0 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3e9, extra_check: false, size: 0xc3, row: 0x11f,
            ops: &OCEANIA_NATIONS_OTHER_OPS_3, ops2: &[], name: "cqz_6 oceania nations cup round table (other years)" }),
        1 => Some(GoldcupStagePlan { league: 0, rounds: 3, extra: 0, extra_check: false, size: 0x138, row: 0x14b,
            ops: &OCEANIA_NATIONS_OTHER_OPS_4, ops2: &[], name: "cqz_6 oceania nations cup round table (other years)" }),
        2..=127 => None,
    }
}

/// cqz_6 olympics round table, the operations of round table 1 (from the original, in order).
const OLYMPICS_OPS_1: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 0xd, 8, 0, 2, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 8, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 8, 0, 1, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];
/// cqz_6 olympics round table, the operations of round table 2 (from the original, in order).
const OLYMPICS_OPS_2: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 8, 0, 2, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 8, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 8, 0, 1, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 2, 1, 3, 4), Cqp12L(0, 1, 2, 1, 3, 0x80), Cqp12L(1, 0, 5, 1, 3, 4), Cqp12L(1, 1, 5, 1, 3, 0x80),
    Cqp12L(2, 0, 1, 1, 3, 4), Cqp12L(2, 1, 1, 1, 3, 0x80),
];
/// cqz_6 olympics round table, the operations of round table 3 (from the original, in order).
const OLYMPICS_OPS_3: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 8, 0, 2, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 8, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 8, 0, 1, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 2, 1, 3, 0x40), Cqp12L(0, 1, 2, 1, 3, 0x100), Cqp12L(1, 0, 5, 1, 3, 0x40),
    Cqp12L(1, 1, 5, 1, 3, 0x100), Cqp12L(2, 0, 1, 1, 3, 0x40), Cqp12L(2, 1, 1, 1, 3, 0x100),
];
/// cqz_6 olympics round table, the operations of round table 4 (from the original, in order).
const OLYMPICS_OPS_4: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 8, 0, 2, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 8, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 8, 0, 1, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 3, 1, 3, 0x40), Cqp12L(0, 1, 3, 1, 3, 0x100), Cqp12L(1, 0, 6, 1, 3, 0x40),
    Cqp12L(1, 1, 6, 1, 3, 0x100), Cqp12L(2, 0, 2, 1, 3, 0x40), Cqp12L(2, 1, 2, 1, 3, 0x100),
];
/// cqz_6 olympics round table, the operations of round table 5 (from the original, in order).
const OLYMPICS_OPS_5: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 8, 0, 2, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 8, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 8, 0, 1, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 3, 1, 3, 4), Cqp12L(0, 1, 3, 1, 3, 0x80), Cqp12L(1, 0, 6, 1, 3, 4), Cqp12L(1, 1, 6, 1, 3, 0x80),
    Cqp12L(2, 0, 2, 1, 3, 4), Cqp12L(2, 1, 2, 1, 3, 0x80),
];
/// cqz_6 olympics round table, the operations of round table 6 (from the original, in order).
const OLYMPICS_OPS_6: [GoldcupScheduleOp; 80] = [
    Cqp12S(0, 0x15, 8, 0, 3), Cqp12J(0, 0x17, 8, 0, 5, 1, 0), Cqp12E(0, 0, 5, 2, 3, 0x100),
    Cqp12E(0, 1, 5, 2, 3, 0x80), Cqp12E(0, 2, 5, 2, 3, 0x40), Cqp12E(0, 3, 5, 2, 3, 4), Cqp12E(0, 4, -1, -1, -1, 0),
    Cqp12W(0x7, 0x78), Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x303), Cqp12B(0x17, 0xa), Cqp12W(0x18, 0x8),
    Cqp12W(0x1a, 0x4), Cqp12W(0x1c, 0x8), Cqp12W(0xf, 0x0), Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x1),
    Cqp12B(0x22, 0x0), Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0), Cqp12X(0x64, 0x0), Cqp12S(1, 0x18, 8, 0, 6),
    Cqp12J(1, 0x1a, 8, 0, 1, 1, 0), Cqp12E(1, 0, 2, 2, 3, 0x40), Cqp12E(1, 1, 2, 1, 3, 4),
    Cqp12E(1, 2, -1, -1, -1, 0), Cqp12W(0x6f, 0x82), Cqp12W(0x71, 0x1), Cqp12W(0x73, 0x0), Cqp12W(0x75, 0x303),
    Cqp12B(0x7f, 0xa), Cqp12W(0x80, 0x4), Cqp12W(0x82, 0x2), Cqp12W(0x84, 0x0), Cqp12W(0x77, 0x0), Cqp12W(0x86, 0x0),
    Cqp12B(0x88, 0x0), Cqp12B(0x89, 0x1), Cqp12B(0x8a, 0x0), Cqp12X(0xc4, 0x0), Cqp12X(0xc8, 0x0), Cqp12X(0xcc, 0x0),
    Cqp12S(2, 0x1c, 8, 0, 3), Cqp12J(2, 0x1e, 8, 0, 5, 1, 4), Cqp12W(0xd7, 0x96), Cqp12W(0xd9, 0x2),
    Cqp12W(0xdb, 0x0), Cqp12W(0xdd, 0x3), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 0x2), Cqp12W(0xea, 0x1), Cqp12W(0xec, 0x0),
    Cqp12W(0xdf, 0x0), Cqp12W(0xee, 0x0), Cqp12B(0xf0, 0x0), Cqp12B(0xf1, 0x1), Cqp12B(0xf2, 0x0), Cqp12X(0x12c, 0x0),
    Cqp12X(0x130, 0x0), Cqp12X(0x134, 0x0), Cqp12S(3, 0x1c, 8, 0, 3), Cqp12J(3, 0x1d, 8, 0, 4, 1, 0x40),
    Cqp12W(0x141, 0x3), Cqp12W(0x145, 0x3), Cqp12W(0x13f, 0x8c), Cqp12W(0x143, 0x0), Cqp12B(0x14f, 0xa),
    Cqp12W(0x150, 0x2), Cqp12W(0x152, 0x1), Cqp12W(0x154, 0x0), Cqp12W(0x147, 0x0), Cqp12W(0x156, 0x0),
    Cqp12B(0x158, 0x0), Cqp12B(0x159, 0x1), Cqp12B(0x15a, 0x0), Cqp12X(0x194, 0x0), Cqp12X(0x198, 0x0),
    Cqp12X(0x19c, 0x0),
];

/// The round table of stage `stage` (signed byte) of cqz_6 olympics round table, or none.
fn olympics_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=-2 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: (f as i16).wrapping_add(0x3e9), extra_check: false, size: 0xc3, row: 0x9c,
            ops: &OLYMPICS_OPS_1, ops2: &[], name: "cqz_6 olympics round table" }),
        -1 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3e8, extra_check: false, size: 0xc3, row: 0x9c,
            ops: &OLYMPICS_OPS_2, ops2: &[], name: "cqz_6 olympics round table" }),
        0 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3e9, extra_check: false, size: 0xc3, row: 0x9c,
            ops: &OLYMPICS_OPS_3, ops2: &[], name: "cqz_6 olympics round table" }),
        1 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3ea, extra_check: false, size: 0xc3, row: 0x9c,
            ops: &OLYMPICS_OPS_4, ops2: &[], name: "cqz_6 olympics round table" }),
        2 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3eb, extra_check: false, size: 0xc3, row: 0x9c,
            ops: &OLYMPICS_OPS_5, ops2: &[], name: "cqz_6 olympics round table" }),
        3 => Some(GoldcupStagePlan { league: 0, rounds: 4, extra: 0, extra_check: false, size: 0x1a0, row: 0xde,
            ops: &OLYMPICS_OPS_6, ops2: &[], name: "cqz_6 olympics round table" }),
        4..=127 => None,
    }
}

/// rule: the Asian Cup has three groups (stages −1 to 1, ids 0x41f to 0x421) of three rounds on 13, 16 and 19 October,
/// each with the group's fixed matches; the knockout (stage 2) in single matches: quarter-finals on 23/10 (draw 21/10),
/// semi-finals on 26/10, third place and final on 29/10; stages from 3 on have no table (0).
///
/// Built from the original with the round-table machine of the Copa América (`GoldcupScheduleOp`, `build_phase_table`):
/// each operation is a call of the original (league_round_set_date 0x68a160, league_round_set_match_slot 0x68a1c0,
/// cup_set_round_draw_date 0x521e60, cup_set_round_match_date 0x521eb0, cup_set_round_match_day 0x521f10: pushes; add esp
/// -> cdecl) or a byte/word/dword written into the table, in the original's order (months from 0 = January); the outputs
/// (`rounds`, `extra` always written, `league` only when not null), the size of the table and the error-box line when
/// out of memory are per stage (signed byte), every stage −128..127 checked against the original by emulation.
/// Original: `AsiaNations::create_schedule` (0x40eb00, asia_nations.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn asia_nations_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), asia_nations_phase(stage), rounds, extra, league, 0x987ba0)
}

/// Comparer for `asia_nations_create_schedule`: side by side (`compare_phase_table`: the outputs and the bytes written to the two tables).
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ASIA_NATIONS_CREATE_SCHEDULE);
    compare_phase_table("asia_nations_create_schedule", stage, asia_nations_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| asia_nations_create_schedule(this, stage, r, e, l))
}

/// The round table of stage `stage` of the Oceania Nations Cup: the editions of years with remainder 2 by 4 (2002,
/// 2006...) have their own dates (November), the others the June ones.
unsafe fn oceania_nations_phase(this: League, stage: u32) -> Option<GoldcupStagePlan> {
    if (this.year() as i32) % 4 == 2 { // cqz6: the Oceania Nations Cup of 2002, 2006...
        oceania_nations_even_phase(stage)
    } else {
        oceania_nations_other_phase(stage)
    }
}

/// rule: the Oceania Nations Cup has two groups (stages −1 and 0, ids 0x3e8 and 0x3e9) of three rounds and the
/// knockout (stage 1: semi-finals, third place and final). In the years with remainder 2 by 4 (2002...) the groups play
/// on 10, 13 and 15 November and the knockout on 17/11 and 19/11; in the other years, on 19, 21 and 23 June and 25/6
/// and 28/6; the error-box lines differ between the two halves of the original routine. Stages from 2 on have no table.
///
/// Built from the original with the round-table machine of the Copa América (`GoldcupScheduleOp`, `build_phase_table`):
/// each operation is a call of the original (league_round_set_date 0x68a160, league_round_set_match_slot 0x68a1c0,
/// cup_set_round_draw_date 0x521e60, cup_set_round_match_date 0x521eb0, cup_set_round_match_day 0x521f10: pushes; add esp
/// -> cdecl) or a byte/word/dword written into the table, in the original's order (months from 0 = January); the outputs
/// (`rounds`, `extra` always written, `league` only when not null), the size of the table and the error-box line when
/// out of memory are per stage (signed byte), every stage −128..127 checked against the original by emulation.
/// Original: `OceaniaNations::create_schedule` (0x798b60, oceania_nations.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn oceania_nations_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), oceania_nations_phase(League(this), stage), rounds, extra, league, 0xa6a0ec)
}

/// Comparer for `oceania_nations_create_schedule`: side by side (`compare_phase_table`).
pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_OCEANIA_NATIONS_CREATE_SCHEDULE);
    compare_phase_table("oceania_nations_create_schedule", stage, oceania_nations_phase(League(this), stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| oceania_nations_create_schedule(this, stage, r, e, l))
}

/// rule: the Olympic football has four groups (stages −1 to 2, ids 0x3e8 to 0x3eb) of three rounds on 13, 16 and 19
/// September; the knockout (stage 3) in single matches: quarter-finals on 23/9, semi-finals on 26/9, third place on
/// 29/9 and final on 30/9; stages from 4 on have no table.
///
/// Built from the original with the round-table machine of the Copa América (`GoldcupScheduleOp`, `build_phase_table`):
/// each operation is a call of the original (league_round_set_date 0x68a160, league_round_set_match_slot 0x68a1c0,
/// cup_set_round_draw_date 0x521e60, cup_set_round_match_date 0x521eb0, cup_set_round_match_day 0x521f10: pushes; add esp
/// -> cdecl) or a byte/word/dword written into the table, in the original's order (months from 0 = January); the outputs
/// (`rounds`, `extra` always written, `league` only when not null), the size of the table and the error-box line when
/// out of memory are per stage (signed byte), every stage −128..127 checked against the original by emulation.
/// Original: `Olympics::create_schedule` (0x79f7d0, olympics.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn olympics_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), olympics_phase(stage), rounds, extra, league, 0xa6a360)
}

/// Comparer for `olympics_create_schedule`: side by side (`compare_phase_table`: the outputs and the bytes written to the two tables).
pub(crate) unsafe extern "thiscall" fn compare_olympics_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_OLYMPICS_CREATE_SCHEDULE);
    compare_phase_table("olympics_create_schedule", stage, olympics_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| olympics_create_schedule(this, stage, r, e, l))
}

// ---------- cqz_7: the round tables of the European Championship and of the Asian and Oceania World Cup qualifying (euro_champ.cpp, wc_asia_league.cpp, wc_oceania_league.cpp) ----------

/// cqz_7 euro round table, the operations of round table 1 (from the original, in order).
const EURO_CHAMP_OPS_1: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 9, 5, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xf, 5, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 5, 0, 2, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];
/// cqz_7 euro round table, the operations of round table 2 (from the original, in order).
const EURO_CHAMP_OPS_2: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 9, 5, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xf, 5, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 5, 0, 2, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 5, 2, 3, 4), Cqp12L(0, 1, 0, 2, 3, 0x80), Cqp12L(1, 0, 3, 2, 3, 4), Cqp12L(1, 1, 5, 1, 3, 0x80),
    Cqp12L(2, 0, 1, 1, 3, 4), Cqp12L(2, 1, 1, 1, 3, 0x80),
];
/// cqz_7 euro round table, the operations of round table 3 (from the original, in order).
const EURO_CHAMP_OPS_3: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 9, 5, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xf, 5, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 5, 0, 2, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 6, 1, 3, 0x40), Cqp12L(0, 1, 0, 1, 3, 0x1000), Cqp12L(1, 0, 3, 1, 3, 0x40),
    Cqp12L(1, 1, 5, 2, 3, 0x1000), Cqp12L(2, 0, 1, 2, 3, 0x1000), Cqp12L(2, 1, 1, 2, 3, 0x40),
];
/// cqz_7 euro round table, the operations of round table 4 (from the original, in order).
const EURO_CHAMP_OPS_4: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 9, 5, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xf, 5, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 5, 0, 2, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 6, 2, 3, 0x100), Cqp12L(0, 1, 1, 1, 3, 0x400), Cqp12L(1, 0, 4, 1, 3, 0x100),
    Cqp12L(1, 1, 6, 1, 3, 0x400), Cqp12L(2, 0, 2, 2, 3, 0x400), Cqp12L(2, 1, 2, 2, 3, 0x100),
];
/// cqz_7 euro round table, the operations of round table 5 (from the original, in order).
const EURO_CHAMP_OPS_5: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 9, 5, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xf, 5, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 5, 0, 2, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 6, 1, 3, 0x200), Cqp12L(0, 1, 1, 2, 3, 0x800), Cqp12L(1, 0, 4, 2, 3, 0x200),
    Cqp12L(1, 1, 6, 2, 3, 0x800), Cqp12L(2, 0, 2, 1, 3, 0x200), Cqp12L(2, 1, 2, 1, 3, 0x800),
];
/// cqz_7 euro round table, the operations of round table 6 (from the original, in order).
const EURO_CHAMP_OPS_6: [GoldcupScheduleOp; 62] = [
    Cqp12S(0, 0x14, 5, 0, 3), Cqp12J(0, 0x16, 5, 0, 5, 1, 1), Cqp12E(0, 0, 5, 1, 3, 0x40), Cqp12E(0, 1, 5, 2, 3, 4),
    Cqp12E(0, 2, 6, 2, 3, 0x80), Cqp12E(0, 3, 6, 1, 3, 0x100), Cqp12E(0, 4, -1, -1, -1, 0), Cqp12W(0x7, 0x78),
    Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x383), Cqp12B(0x17, 0xa), Cqp12W(0x18, 0x8), Cqp12W(0x1a, 0x4),
    Cqp12W(0x1c, 0x8), Cqp12W(0xf, 0x0), Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x1), Cqp12B(0x22, 0x0),
    Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0), Cqp12X(0x64, 0x0), Cqp12S(1, 0x18, 5, 0, 0), Cqp12J(1, 0x1a, 5, 0, 2, 1, 1),
    Cqp12E(1, 0, 2, 2, 3, 0x40), Cqp12E(1, 1, 2, 1, 3, 0x80), Cqp12E(1, 2, -1, -1, -1, 0), Cqp12W(0x6f, 0x82),
    Cqp12W(0x71, 0x1), Cqp12W(0x73, 0x0), Cqp12W(0x75, 0x383), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 0x4),
    Cqp12W(0x82, 0x2), Cqp12W(0x84, 0x0), Cqp12W(0x77, 0x0), Cqp12W(0x86, 0x0), Cqp12B(0x88, 0x0), Cqp12B(0x89, 0x1),
    Cqp12B(0x8a, 0x0), Cqp12X(0xc4, 0x0), Cqp12X(0xc8, 0x0), Cqp12X(0xcc, 0x0), Cqp12S(2, 0x1b, 5, 0, 3),
    Cqp12J(2, 0x1e, 5, 0, 6, 1, 4), Cqp12W(0xd7, 0x96), Cqp12W(0xd9, 0x2), Cqp12W(0xdb, 0x0), Cqp12W(0xdd, 0x283),
    Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 0x2), Cqp12W(0xea, 0x1), Cqp12W(0xec, 0x0), Cqp12W(0xdf, 0x0), Cqp12W(0xee, 0x0),
    Cqp12B(0xf0, 0x0), Cqp12B(0xf1, 0x1), Cqp12B(0xf2, 0x0), Cqp12X(0x12c, 0x0), Cqp12X(0x130, 0x0),
    Cqp12X(0x134, 0x0),
];

/// The round table of stage `stage` (signed byte) of cqz_7 euro round table, or none.
fn euro_champ_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=-2 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: (f as i16).wrapping_add(0x3e9), extra_check: false, size: 0xc3, row: 0xb4,
            ops: &EURO_CHAMP_OPS_1, ops2: &[], name: "cqz_7 euro round table" }),
        -1 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3e8, extra_check: false, size: 0xc3, row: 0xb4,
            ops: &EURO_CHAMP_OPS_2, ops2: &[], name: "cqz_7 euro round table" }),
        0 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3e9, extra_check: false, size: 0xc3, row: 0xb4,
            ops: &EURO_CHAMP_OPS_3, ops2: &[], name: "cqz_7 euro round table" }),
        1 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3ea, extra_check: false, size: 0xc3, row: 0xb4,
            ops: &EURO_CHAMP_OPS_4, ops2: &[], name: "cqz_7 euro round table" }),
        2 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3eb, extra_check: false, size: 0xc3, row: 0xb4,
            ops: &EURO_CHAMP_OPS_5, ops2: &[], name: "cqz_7 euro round table" }),
        3 => Some(GoldcupStagePlan { league: 0, rounds: 3, extra: 0, extra_check: false, size: 0x138, row: 0xf6,
            ops: &EURO_CHAMP_OPS_6, ops2: &[], name: "cqz_7 euro round table" }),
        4..=127 => None,
    }
}

/// cqz_7 asian qualifying round table, the operations of round table 1 (from the original, in order).
const WC_ASIA_LEAGUE_OPS_1: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0x1e, 3, 1, 0, 2, 0), Cqp12L(0, 0, -1, -1, -1, 0), Cqp12D(1, 4, 4, 1, 4, 2, 0),
    Cqp12L(1, 0, -1, -1, -1, 0), Cqp12D(2, 7, 4, 1, 0, 2, 0), Cqp12L(2, 0, -1, -1, -1, 0),
    Cqp12D(3, 0xb, 4, 1, 4, 2, 0), Cqp12L(3, 0, -1, -1, -1, 0), Cqp12D(4, 0x12, 4, 1, 4, 2, 0),
    Cqp12L(4, 0, -1, -1, -1, 0), Cqp12D(5, 0x19, 4, 1, 4, 2, 0), Cqp12L(5, 0, -1, -1, -1, 0),
];
/// cqz_7 asian qualifying round table, the operations of round table 2 (from the original, in order).
const WC_ASIA_LEAGUE_OPS_2: [GoldcupScheduleOp; 20] = [
    Cqp12D(0, 0x11, 7, 1, 4, 2, 0), Cqp12L(0, 0, -1, -1, -1, 0), Cqp12D(1, 0x18, 7, 1, 4, 2, 0),
    Cqp12L(1, 0, -1, -1, -1, 0), Cqp12D(2, 0x1f, 7, 1, 4, 2, 0), Cqp12L(2, 0, -1, -1, -1, 0),
    Cqp12D(3, 7, 8, 1, 4, 2, 0), Cqp12L(3, 0, -1, -1, -1, 0), Cqp12D(4, 0xe, 8, 1, 4, 2, 0),
    Cqp12L(4, 0, -1, -1, -1, 0), Cqp12D(5, 0x15, 8, 1, 4, 2, 0), Cqp12L(5, 0, -1, -1, -1, 0),
    Cqp12D(6, 0x1c, 8, 1, 4, 2, 0), Cqp12L(6, 0, -1, -1, -1, 0), Cqp12D(7, 5, 9, 1, 4, 2, 0),
    Cqp12L(7, 0, -1, -1, -1, 0), Cqp12D(8, 0xc, 9, 1, 4, 2, 0), Cqp12L(8, 0, -1, -1, -1, 0),
    Cqp12D(9, 0x13, 9, 1, 4, 2, 0), Cqp12L(9, 0, -1, -1, -1, 0),
];
/// cqz_7 asian qualifying round table, the operations of round table 3 (from the original, in order).
const WC_ASIA_LEAGUE_OPS_3: [GoldcupScheduleOp; 18] = [
    Cqp12S(0, 0x14, 9, 1, 5), Cqp12J(0, 0x1a, 9, 1, 4, 2, 0), Cqp12W(0x18, 0x2), Cqp12W(0x1c, 0x2), Cqp12W(0x7, 0x96),
    Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x3), Cqp12B(0x17, 0xa), Cqp12W(0x1a, 0x1), Cqp12W(0xf, 0x0),
    Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x1), Cqp12B(0x22, 0x0), Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0),
    Cqp12X(0x64, 0x0),
];
/// cqz_7 asian qualifying round table, the operations of round table 4 (from the original, in order).
const WC_ASIA_LEAGUE_OPS_4: [GoldcupScheduleOp; 18] = [
    Cqp12S(0, 0x1b, 9, 1, 5), Cqp12J(0, 0xb, 0xa, 1, 5, 1, 0), Cqp12W(0x18, 0x2), Cqp12W(0x1c, 0x2),
    Cqp12B(0x21, 0x2), Cqp12W(0x7, 0x0), Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x0), Cqp12B(0x17, 0xa),
    Cqp12W(0x1a, 0x1), Cqp12W(0xf, 0x3), Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x22, 0x4), Cqp12X(0x5c, 0x0),
    Cqp12X(0x60, 0x0), Cqp12X(0x64, 0x0),
];

/// The round table of stage `stage` (signed byte) of cqz_7 asian qualifying round table, or none.
fn wc_asia_league_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=8 => Some(GoldcupStagePlan { league: 1, rounds: 6, extra: (f as i16).wrapping_add(0x3fe), extra_check: false, size: 0x186, row: 0xc0,
            ops: &WC_ASIA_LEAGUE_OPS_1, ops2: &[], name: "cqz_7 asian qualifying round table" }),
        9..=10 => Some(GoldcupStagePlan { league: 1, rounds: 10, extra: (f as i16).wrapping_add(0x3fe), extra_check: false, size: 0x28a, row: 0xea,
            ops: &WC_ASIA_LEAGUE_OPS_2, ops2: &[], name: "cqz_7 asian qualifying round table" }),
        11 => Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0x1e, extra_check: false, size: 0x68, row: 0x120,
            ops: &WC_ASIA_LEAGUE_OPS_3, ops2: &[], name: "cqz_7 asian qualifying round table" }),
        12 => Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0xa0, extra_check: false, size: 0x68, row: 0x14b,
            ops: &WC_ASIA_LEAGUE_OPS_4, ops2: &[], name: "cqz_7 asian qualifying round table" }),
        13..=127 => None,
    }
}

/// cqz_7 oceania qualifying round table, the operations of round table 1 (from the original, in order).
const WC_OCEANIA_LEAGUE_OPS_1: [GoldcupScheduleOp; 20] = [
    Cqp12D(0, 7, 3, 1, 5, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 9, 3, 1, 0, 2, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0xb, 3, 1, 2, 2, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12D(3, 0xe, 3, 1, 5, 1, 1), Cqp12L(3, 2, -1, -1, -1, 0), Cqp12D(4, 0x10, 3, 1, 0, 2, 1),
    Cqp12L(4, 2, -1, -1, -1, 0), Cqp12L(0, 0, 6, 1, 3, 0x100), Cqp12L(0, 1, 6, 2, 3, 0x100),
    Cqp12L(1, 0, 2, 1, 3, 0x100), Cqp12L(1, 1, 2, 2, 3, 0x100), Cqp12L(2, 0, 5, 1, 3, 0x100),
    Cqp12L(2, 1, 5, 2, 3, 0x100), Cqp12L(3, 0, 0, 1, 3, 0x100), Cqp12L(3, 1, 0, 2, 3, 0x100),
    Cqp12L(4, 0, 2, 1, 3, 0x100), Cqp12L(4, 1, 2, 2, 3, 0x100),
];
/// cqz_7 oceania qualifying round table, the operations of round table 2 (from the original, in order).
const WC_OCEANIA_LEAGUE_OPS_2: [GoldcupScheduleOp; 20] = [
    Cqp12D(0, 7, 3, 1, 5, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 9, 3, 1, 0, 2, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0xb, 3, 1, 2, 2, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12D(3, 0xe, 3, 1, 5, 1, 1), Cqp12L(3, 2, -1, -1, -1, 0), Cqp12D(4, 0x10, 3, 1, 0, 2, 1),
    Cqp12L(4, 2, -1, -1, -1, 0), Cqp12L(0, 0, 5, 1, 3, 0x80), Cqp12L(0, 1, 5, 2, 3, 0x80),
    Cqp12L(1, 0, 1, 1, 3, 0x80), Cqp12L(1, 1, 1, 0, 3, 0x80), Cqp12L(2, 0, 4, 1, 3, 0x80),
    Cqp12L(2, 1, 4, 2, 3, 0x80), Cqp12L(3, 0, 6, 1, 3, 0x80), Cqp12L(3, 1, 6, 2, 3, 0x80),
    Cqp12L(4, 0, 1, 1, 3, 0x80), Cqp12L(4, 1, 1, 2, 3, 0x80),
];
/// cqz_7 oceania qualifying round table, the operations of round table 3 (from the original, in order).
const WC_OCEANIA_LEAGUE_OPS_3: [GoldcupScheduleOp; 18] = [
    Cqp12S(0, 0x1c, 3, 1, 3), Cqp12J(0, 0x14, 5, 1, 2, 2, 0), Cqp12W(0x18, 0x2), Cqp12W(0x1c, 0x2), Cqp12W(0x7, 0x96),
    Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x0), Cqp12B(0x17, 0xa), Cqp12W(0x1a, 0x1), Cqp12W(0xf, 0x3),
    Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x2), Cqp12B(0x22, 0x4), Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0),
    Cqp12X(0x64, 0x0),
];
/// cqz_7 oceania qualifying round table, the operations of round table 4 (from the original, in order).
const WC_OCEANIA_LEAGUE_OPS_4: [GoldcupScheduleOp; 18] = [
    Cqp12S(0, 0x17, 0xa, 1, 3), Cqp12J(0, 2, 0xb, 1, 5, 1, 0), Cqp12W(0x18, 0x2), Cqp12W(0x1c, 0x2), Cqp12W(0x7, 0x0),
    Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x0), Cqp12B(0x17, 0xa), Cqp12W(0x1a, 0x1), Cqp12W(0xf, 0x3),
    Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x2), Cqp12B(0x22, 0x7), Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0),
    Cqp12X(0x64, 0x0),
];

/// The round table of stage `stage` (signed byte) of cqz_7 oceania qualifying round table, or none.
fn wc_oceania_league_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=-2 => Some(GoldcupStagePlan { league: 1, rounds: 5, extra: 0x3e9, extra_check: false, size: 0x145, row: 0xc1,
            ops: &WC_OCEANIA_LEAGUE_OPS_1, ops2: &[], name: "cqz_7 oceania qualifying round table" }),
        -1 => Some(GoldcupStagePlan { league: 1, rounds: 5, extra: 0x3e8, extra_check: false, size: 0x145, row: 0xc1,
            ops: &WC_OCEANIA_LEAGUE_OPS_2, ops2: &[], name: "cqz_7 oceania qualifying round table" }),
        0 => Some(GoldcupStagePlan { league: 1, rounds: 5, extra: 0x3e9, extra_check: false, size: 0x145, row: 0xc1,
            ops: &WC_OCEANIA_LEAGUE_OPS_1, ops2: &[], name: "cqz_7 oceania qualifying round table" }),
        1 => Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x10b,
            ops: &WC_OCEANIA_LEAGUE_OPS_3, ops2: &[], name: "cqz_7 oceania qualifying round table" }),
        2 => Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0xa0, extra_check: false, size: 0x68, row: 0x134,
            ops: &WC_OCEANIA_LEAGUE_OPS_4, ops2: &[], name: "cqz_7 oceania qualifying round table" }),
        3..=127 => None,
    }
}

/// rule: the European Championship has four groups (stages −1 to 2, ids 0x3e8 to 0x3eb) of three rounds on 9, 15 and
/// 19 June; the knockout (stage 3) in single matches: quarter-finals on 22/6, semi-finals on 26/6 and final on 30/6
/// (no third place); stages from 4 on have no table.
///
/// Built from the original with the round-table machine of the Copa América (`GoldcupScheduleOp`, `build_phase_table`):
/// each operation is a call of the original (league_round_set_date 0x68a160, league_round_set_match_slot 0x68a1c0,
/// cup_set_round_draw_date 0x521e60, cup_set_round_match_date 0x521eb0, cup_set_round_match_day 0x521f10: pushes; add esp
/// -> cdecl) or a byte/word/dword written into the table, in the original's order (months from 0 = January); the outputs
/// (`rounds`, `extra` always written, `league` only when not null), the size of the table and the error-box line when
/// out of memory are per stage (signed byte), every stage −128..127 checked against the original by emulation.
/// Original: `EuroChamp::create_schedule` (0x57ab90, euro_champ.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn euro_champ_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), euro_champ_phase(stage), rounds, extra, league, 0x9c4638)
}

/// Comparer for `euro_champ_create_schedule`: side by side (`compare_phase_table`: the outputs and the bytes written to the two tables).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_EURO_CHAMP_CREATE_SCHEDULE);
    compare_phase_table("euro_champ_create_schedule", stage, euro_champ_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| euro_champ_create_schedule(this, stage, r, e, l))
}

/// rule: the Asian World Cup qualifying has first-round groups (stages up to 8, ids 0x3fe + stage) of six rounds
/// between 30 April and 25 May, second-round groups (stages 9 and 10) of ten rounds between 17 August and 19 October,
/// the two-legged tie of the group runners-up (stage 11: 26 October) and the inter-continental play-off (stage 12:
/// draw 27/10, 11 November); stages from 13 on have no table.
///
/// Built from the original with the round-table machine of the Copa América (`GoldcupScheduleOp`, `build_phase_table`):
/// each operation is a call of the original (league_round_set_date 0x68a160, league_round_set_match_slot 0x68a1c0,
/// cup_set_round_draw_date 0x521e60, cup_set_round_match_date 0x521eb0, cup_set_round_match_day 0x521f10: pushes; add esp
/// -> cdecl) or a byte/word/dword written into the table, in the original's order (months from 0 = January); the outputs
/// (`rounds`, `extra` always written, `league` only when not null), the size of the table and the error-box line when
/// out of memory are per stage (signed byte), every stage −128..127 checked against the original by emulation.
/// Original: `WcAsiaLeague::create_schedule` (0x911f00, wc_asia_league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), wc_asia_league_phase(stage), rounds, extra, league, 0xad4680)
}

/// Comparer for `wc_asia_league_create_schedule`: side by side (`compare_phase_table`: the outputs and the bytes written to the two tables).
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_WC_ASIA_LEAGUE_CREATE_SCHEDULE);
    compare_phase_table("wc_asia_league_create_schedule", stage, wc_asia_league_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| wc_asia_league_create_schedule(this, stage, r, e, l))
}

/// rule: the Oceania World Cup qualifying has two groups (stages −1 and 0, ids 0x3e8 and 0x3e9) of five rounds on 7,
/// 9, 11, 14 and 16 April; the tie of the group winners (stage 1: 20 June) and the inter-continental play-off (stage
/// 2: 2 December); stages from 3 on have no table.
///
/// Built from the original with the round-table machine of the Copa América (`GoldcupScheduleOp`, `build_phase_table`):
/// each operation is a call of the original (league_round_set_date 0x68a160, league_round_set_match_slot 0x68a1c0,
/// cup_set_round_draw_date 0x521e60, cup_set_round_match_date 0x521eb0, cup_set_round_match_day 0x521f10: pushes; add esp
/// -> cdecl) or a byte/word/dword written into the table, in the original's order (months from 0 = January); the outputs
/// (`rounds`, `extra` always written, `league` only when not null), the size of the table and the error-box line when
/// out of memory are per stage (signed byte), every stage −128..127 checked against the original by emulation.
/// Original: `WcOceaniaLeague::create_schedule` (0x91e2e0, wc_oceania_league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), wc_oceania_league_phase(stage), rounds, extra, league, 0xad4afc)
}

/// Comparer for `wc_oceania_league_create_schedule`: side by side (`compare_phase_table`: the outputs and the bytes written to the two tables).
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_WC_OCEANIA_LEAGUE_CREATE_SCHEDULE);
    compare_phase_table("wc_oceania_league_create_schedule", stage, wc_oceania_league_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| wc_oceania_league_create_schedule(this, stage, r, e, l))
}

// ---------- cqz_8: the South American World Cup qualifying: the round table, the exempt nations and the team slots (wc_south_american_league.cpp) ----------

/// cqz_8 south american qualifying round table, the operations of round table 1 (from the original, in order).
const WC_SOUTH_AMERICAN_TEN_OPS_1: [GoldcupScheduleOp; 36] = [
    Cqp12D(0, 0x1d, 2, 0, 2, 2, 0), Cqp12L(0, 0, -1, -1, -1, 0), Cqp12D(1, 0x1a, 3, 0, 2, 2, 0),
    Cqp12L(1, 0, -1, -1, -1, 0), Cqp12D(2, 4, 5, 0, 6, 1, 0), Cqp12L(2, 0, -1, -1, -1, 0),
    Cqp12D(3, 0x1c, 5, 0, 2, 2, 0), Cqp12L(3, 0, -1, -1, -1, 0), Cqp12D(4, 0x13, 6, 0, 5, 1, 0),
    Cqp12L(4, 0, -1, -1, -1, 0), Cqp12D(5, 0x1a, 6, 0, 5, 1, 0), Cqp12L(5, 0, -1, -1, -1, 0),
    Cqp12D(6, 0x10, 7, 0, 2, 2, 0), Cqp12L(6, 0, -1, -1, -1, 0), Cqp12D(7, 6, 8, 0, 2, 2, 0),
    Cqp12L(7, 0, -1, -1, -1, 0), Cqp12D(8, 7, 9, 0, 5, 1, 0), Cqp12L(8, 0, -1, -1, -1, 0),
    Cqp12D(9, 0xf, 0xa, 0, 2, 2, 0), Cqp12L(9, 0, -1, -1, -1, 0), Cqp12D(0xa, 0x1d, 2, 1, 2, 2, 0),
    Cqp12L(0xa, 0, -1, -1, -1, 0), Cqp12D(0xb, 0x1a, 3, 1, 2, 2, 0), Cqp12L(0xb, 0, -1, -1, -1, 0),
    Cqp12D(0xc, 4, 5, 1, 5, 1, 0), Cqp12L(0xc, 0, -1, -1, -1, 0), Cqp12D(0xd, 0x10, 7, 1, 2, 2, 0),
    Cqp12L(0xd, 0, -1, -1, -1, 0), Cqp12D(0xe, 6, 8, 1, 2, 2, 0), Cqp12L(0xe, 0, -1, -1, -1, 0),
    Cqp12D(0xf, 7, 9, 1, 5, 1, 0), Cqp12L(0xf, 0, -1, -1, -1, 0), Cqp12D(0x10, 8, 0xa, 1, 2, 2, 0),
    Cqp12L(0x10, 0, -1, -1, -1, 0), Cqp12D(0x11, 0xf, 0xa, 1, 2, 2, 0), Cqp12L(0x11, 0, -1, -1, -1, 0),
];

/// The round table of stage `stage` (signed byte) of cqz_8 south american qualifying round table, or none.
fn wc_south_american_ten_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=-2 => None,
        -1 => Some(GoldcupStagePlan { league: 1, rounds: 18, extra: 0, extra_check: false, size: 0x492, row: 0xb5,
            ops: &WC_SOUTH_AMERICAN_TEN_OPS_1, ops2: &[], name: "cqz_8 south american qualifying round table" }),
        0..=127 => None,
    }
}

/// cqz_8 south american qualifying round table (eight nations), the operations of round table 1 (from the original, in order).
const WC_SOUTH_AMERICAN_EIGHT_OPS_1: [GoldcupScheduleOp; 28] = [
    Cqp12D(0, 0x1d, 2, 0, 2, 2, 0), Cqp12L(0, 0, -1, -1, -1, 0), Cqp12D(1, 4, 5, 0, 6, 1, 0),
    Cqp12L(1, 0, -1, -1, -1, 0), Cqp12D(2, 0x1c, 5, 0, 2, 2, 0), Cqp12L(2, 0, -1, -1, -1, 0),
    Cqp12D(3, 0x13, 6, 0, 5, 1, 0), Cqp12L(3, 0, -1, -1, -1, 0), Cqp12D(4, 0x10, 7, 0, 2, 2, 0),
    Cqp12L(4, 0, -1, -1, -1, 0), Cqp12D(5, 6, 8, 0, 2, 2, 0), Cqp12L(5, 0, -1, -1, -1, 0),
    Cqp12D(6, 7, 9, 0, 5, 1, 0), Cqp12L(6, 0, -1, -1, -1, 0), Cqp12D(7, 0x1d, 2, 1, 2, 2, 0),
    Cqp12L(7, 0, -1, -1, -1, 0), Cqp12D(8, 0x1a, 3, 1, 2, 2, 0), Cqp12L(8, 0, -1, -1, -1, 0),
    Cqp12D(9, 0x10, 7, 1, 2, 2, 0), Cqp12L(9, 0, -1, -1, -1, 0), Cqp12D(0xa, 6, 8, 1, 2, 2, 0),
    Cqp12L(0xa, 0, -1, -1, -1, 0), Cqp12D(0xb, 7, 9, 1, 5, 1, 0), Cqp12L(0xb, 0, -1, -1, -1, 0),
    Cqp12D(0xc, 8, 0xa, 1, 2, 2, 0), Cqp12L(0xc, 0, -1, -1, -1, 0), Cqp12D(0xd, 0xf, 0xa, 1, 2, 2, 0),
    Cqp12L(0xd, 0, -1, -1, -1, 0),
];

/// The round table of stage `stage` (signed byte) of cqz_8 south american qualifying round table (eight nations), or none.
fn wc_south_american_eight_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=-2 => None,
        -1 => Some(GoldcupStagePlan { league: 1, rounds: 14, extra: 0, extra_check: false, size: 0x38e, row: 0xb5,
            ops: &WC_SOUTH_AMERICAN_EIGHT_OPS_1, ops2: &[], name: "cqz_8 south american qualifying round table (eight nations)" }),
        0..=127 => None,
    }
}

/// The round table of stage `stage` of the South American qualifying: the double round robin of ten nations (18 rounds)
/// or, with two or more exempt (`wc_south_american_league_fields::EXEMPT_COUNT`, signed byte), of eight (14 rounds).
unsafe fn wc_south_american_phase(this: League, stage: u32) -> Option<GoldcupStagePlan> {
    if (read_u8(this.addr() + wc_south_american_league_fields::EXEMPT_COUNT) as i8) >= 2 { // cqz8: eight nations with two exempt
        wc_south_american_eight_phase(stage)
    } else {
        wc_south_american_ten_phase(stage)
    }
}

/// rule: the South American qualifying is one double round robin (stage −1, id 0, a league): with ten nations, 18
/// rounds from 29 March of the campaign year to 15 November of the next (two windows a year: March to November); with
/// two or more nations exempt (the World Cup hosts and holder), eight nations and 14 rounds; other stages have no table.
///
/// Built from the original with the round-table machine of the Copa América (`GoldcupScheduleOp`,
/// `build_phase_table`): each operation is a call of the original (league_round_set_date 0x68a160,
/// league_round_set_match_slot 0x68a1c0: pushes; add esp -> cdecl) in the original's order (months from 0); the outputs
/// (`rounds`, the stage id 0 always written, `league` 1 only when not null), the size (0x41 per round) and the error-box
/// line when out of memory (0xb5); every stage −128..127 and both exempt cases checked against the original by
/// emulation.
/// Original: `WcSouthAmericanLeague::create_schedule` (0x920590, wc_south_american_league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), wc_south_american_phase(League(this), stage), rounds, extra, league, 0xad4ce0)
}

/// Comparer for `wc_south_american_league_create_schedule`: side by side (`compare_phase_table`).
pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_WC_SOUTH_AMERICAN_LEAGUE_CREATE_SCHEDULE);
    compare_phase_table("wc_south_american_league_create_schedule", stage, wc_south_american_phase(League(this), stage), rounds, extra, league,
        || o(this, stage, rounds, extra, league), |r, e, l| wc_south_american_league_create_schedule(this, stage, r, e, l))
}

/// Whether the nation with id `id` (`G_NATION_TABLE` + `nation::SIZE` × id) has a continent whose id is the one of the
/// key `continent`.
unsafe fn nation_id_on_continent(id: usize, continent: usize) -> bool {
    let c = Nation(read_usize(G_NATION_TABLE).wrapping_add(id.wrapping_mul(nation::SIZE))).continent();
    c != 0 && read_usize(c) == read_usize(continent)
}

/// The World Cup the qualifying leads to: the hosts of the World Cup two years after the campaign year (HostCountry::
/// find_hosts on [`G_HOST_COUNTRY`]: 5 pushes; mov ecx -> thiscall, ret 0x14; the year pushed with garbage in the high
/// 16 bits, read as a word), into `hosts` (−1 when none), and the number found (signed byte).
unsafe fn south_american_world_cup_hosts(this: League, hosts: &mut [i32; 2]) -> i8 {
    let (a, b) = hosts.split_at_mut(1);
    exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP), (this.year() as u16).wrapping_add(2) as u32, &mut a[0], &mut b[0], 1) as i8
}

/// The last World Cup winner (`exe::comp_util_get_latest_placed_club`(World Cup competition record, 0): 2 pushes; add
/// esp, 8 -> cdecl), or 0.
unsafe fn last_world_cup_winner() -> usize {
    let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b));
    exe::comp_util_get_latest_placed_club(world_cup, 0)
}

/// rule: the South American nations exempt from qualifying are the hosts of the next World Cup and the last World Cup
/// winner (when not a host); the same count is kept for Oceania (the Oceania section reads it)
///
/// `wc_south_american_league_fields::EXEMPT_COUNT` (+0xee) and `OCEANIA_EXEMPT_COUNT` (+0xef) = 0. Without
/// [`G_HOST_COUNTRY`]: the error box of line 0x1ab and no hosts. Otherwise the hosts of the World Cup two years later
/// (`south_american_world_cup_hosts`); for each of them (k signed byte, below the count) up to the first −1: a
/// South American nation (`KEY_CONTINENT_SOUTH_AMERICA`) adds one to +0xee, else an Oceania one
/// (`KEY_CONTINENT_OCEANIA`) to +0xef (a nation without continent, nothing). Then the last World Cup winner
/// (`last_world_cup_winner`), if any: its nation (`club::NATION`), with a continent, South American and neither host
/// adds one to +0xee; then (the nation read again) with a continent, from Oceania and neither host, one to +0xef. Both
/// host slots start at −1 and stay so without [`G_HOST_COUNTRY`].
/// Original: `WcSouthAmericanLeague::count_exempt_nations` (0x920d10, wc_south_american_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_count_exempt_nations(this: usize) {
    let league = League(this);
    let south = this + wc_south_american_league_fields::EXEMPT_COUNT;
    let oceania = this + wc_south_american_league_fields::OCEANIA_EXEMPT_COUNT;
    covered!("cqz_8 exempt nations");
    write_u8(south, 0); // cqz8: no South American nation exempt yet
    write_u8(oceania, 0);
    let mut hosts = [-1i32; 2];
    if read_usize(G_HOST_COUNTRY) == 0 {
        show_error_at(0xad4ce0, 0x1ab);
    } else {
        let n = south_american_world_cup_hosts(league, &mut hosts);
        let mut k = 0i8;
        // simplification: the original reads the host slot k of its two locals for any k below the count; find_hosts gives at most two
        while k < n && (k as usize) < hosts.len() {
            let id = hosts[k as usize];
            if id == -1 {
                break;
            }
            if nation_id_on_continent(id as usize, KEY_CONTINENT_SOUTH_AMERICA) {
                covered!("cqz_8 south american host");
                write_u8(south, read_u8(south).wrapping_add(1)); // cqz8: a South American host is exempt
            } else if nation_id_on_continent(id as usize, KEY_CONTINENT_OCEANIA) {
                write_u8(oceania, read_u8(oceania).wrapping_add(1));
            }
            k += 1;
        }
    }
    let winner = last_world_cup_winner();
    if winner == 0 {
        return;
    }
    let (h0, h1) = (hosts[0], hosts[1]);
    for (continent, count) in [(KEY_CONTINENT_SOUTH_AMERICA, south), (KEY_CONTINENT_OCEANIA, oceania)] {
        let nation = Club(winner).nation();
        if nation.is_null() {
            return;
        }
        let c = nation.continent();
        if c == 0 {
            return;
        }
        let id = read_usize(nation.addr()) as i32;
        if read_usize(c) == read_usize(continent) && id != h0 && id != h1 {
            covered!("cqz_8 world cup holder exempt");
            write_u8(count, read_u8(count).wrapping_add(1));
        }
    }
}

/// Comparer for `wc_south_american_league_count_exempt_nations`: side by side (it only reads and writes the two counts;
/// the error box needs a missing host table), the two counts equal.
pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_count_exempt_nations(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_SOUTH_AMERICAN_LEAGUE_COUNT_EXEMPT_NATIONS);
    let south = this + wc_south_american_league_fields::EXEMPT_COUNT;
    let (a, b) = (read_u8(south), read_u8(south + 1));
    wc_south_american_league_count_exempt_nations(this);
    let mine = (read_u8(south), read_u8(south + 1));
    write_u8(south, a);
    write_u8(south + 1, b);
    o(this);
    let theirs = (read_u8(south), read_u8(south + 1));
    SCOREBOARD.record(mine == theirs, || format!("wc_south_american_league_count_exempt_nations({:#x}): {:?} expected {:?}", this, mine, theirs));
}

/// rule: the South American qualifying table has ten places minus the exempt nations, filled with the South American
/// national teams in database order, leaving out the hosts of the next World Cup and the last winner
///
/// The hosts (`south_american_world_cup_hosts`; [`G_HOST_COUNTRY`] is not checked here; its count is not read: both
/// slots start at −1) and the last World Cup winner (`last_world_cup_winner`). `league::CLUB_COUNT` = 10 − exempt
/// (`EXEMPT_COUNT`, signed byte) and the table (`exe::crt_malloc` of 0x3b × count: push; add esp -> cdecl) into
/// `league::TABLE`; out of memory, the error box of line 0x204 and 0. Over the nations (`G_NATION_COUNT`, i word
/// signed), each South American one (`KEY_CONTINENT_SOUTH_AMERICA`) while the rows (b, signed byte) are fewer than the
/// count: its national team (`exe::database_get_national_team`: push; add esp, 4 -> cdecl), when the nation is neither
/// host and the team is not the winner, goes into row b (`exe::league_init_table_row`(this, b, team, 0, 0): 4 pushes;
/// mov ecx -> thiscall, ret 0x10; the row index pushed with garbage in the high bytes, read as a byte). Returns 1 when
/// the rows filled the count, else 0.
/// Original: `WcSouthAmericanLeague::build_team_slots` (0x920eb0, wc_south_american_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_build_team_slots(this: usize) -> u32 {
    let league = League(this);
    let mut hosts = [-1i32; 2];
    south_american_world_cup_hosts(league, &mut hosts);
    let winner = last_world_cup_winner();
    let places = 10 - read_u8(this + wc_south_american_league_fields::EXEMPT_COUNT) as i8 as i16; // cqz8: ten places minus the exempt
    league.set_club_count(places);
    let table = exe::crt_malloc((places as i32 as u32).wrapping_mul(0x3b));
    league.set_table(table);
    if table == 0 {
        show_error_at(0xad4ce0, 0x204);
        return 0;
    }
    let mut b = 0i8;
    let mut i = 0i16;
    while (i as i32) < read_usize(G_NATION_COUNT) as i32 {
        let nation = Nation(read_usize(G_NATION_TABLE).wrapping_add((i as isize as usize).wrapping_mul(nation::SIZE)));
        let c = nation.continent();
        if c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_SOUTH_AMERICA) && (b as i16) < league.club_count() {
            let team = exe::database_get_national_team(nation);
            let id = read_usize(nation.addr()) as i32;
            if id != hosts[0] && id != hosts[1] && team != winner {
                covered!("cqz_8 south american slot");
                exe::league_init_table_row(league, b as u8 as u32, team, 0, 0);
                b = b.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    ((b as i16) == league.club_count()) as u32
}

// simplification: the rows allocate the position history and the table is the league's own: it cannot run twice. Only the original;
// the plan: the return value is 1 exactly when the table has `club_count` rows, and each row holds a South American
// national team.
pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_build_team_slots(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_SOUTH_AMERICAN_LEAGUE_BUILD_TEAM_SLOTS);
    let r = o(this);
    let league = League(this);
    let expected = 10 - read_u8(this + wc_south_american_league_fields::EXEMPT_COUNT) as i8 as i16;
    let mut ok = league.club_count() == expected && (r == 0 || r == 1);
    if r == 1 && league.table() != 0 {
        for k in 0..expected.max(0) as usize {
            let team = read_usize(league.table() + k * 0x3b);
            let nation = if team != 0 { Club(team).nation() } else { Nation(0) };
            let c = if nation.is_null() { 0 } else { nation.continent() };
            ok &= c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_SOUTH_AMERICA);
        }
    }
    SCOREBOARD.record(ok, || format!("wc_south_american_league_build_team_slots({:#x}) returned {} with {} places", this, r, league.club_count()));
    r
}
