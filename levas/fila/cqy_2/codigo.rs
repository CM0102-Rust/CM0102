// ---------- batch %%LEVA%%: the MLS season fixtures: the first division's own fixtures, the other two division stages and the cross-division fixtures (usa_mls.cpp) ----------

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
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    let l = League(this);
    let ok = r == 1 && l.stage_id() == 0x413 && l.round_dates() != 0 && l.stages() != 0 && read_usize(l.stages()) != 0 && read_usize(l.stages() + 4) != 0;
    SCOREBOARD.record(ok, || format!("usa_mls_create_season_fixtures({:#x}): returned {}, stage id {:#x}, table {:#x}", this, r, l.stage_id(), l.round_dates()));
    r
}
