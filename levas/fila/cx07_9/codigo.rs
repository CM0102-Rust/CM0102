// ---------- cx07_9: the League base constructor and a fixed fixture set by the clubs' names (league.cpp) ----------

/// The League base vtable (League::League writes it, League::~League writes it back).
const LEAGUE_VTABLE: usize = 0x96d020;
/// The League object size (the bytes League::League and the display copy take, through `league::FIXED_FIXTURES`).
const LEAGUE_OBJECT_SIZE: usize = 0xee;

/// Builds the League base of every league, cup stage group and national-team tournament: the Comp base
/// (`exe::new_comp`, Comp::Comp, thiscall, through its detour), the dates `league::LAST_RESULT_DATE` and
/// `league::LAST_UPDATE_DATE` (Date::Date 0x54a110, thiscall), the League vtable, a league (`league::COMP_TYPE` 2) with
/// the match category 2 and the prize mode 2; no draw day (`league::NEXT_DRAW_DAY` −1), no games per club, required
/// capacity or seats (−1 each); the rule bits `league::LEAGUE_FLAGS` 3 (champion and promotion statuses); no stages,
/// table, custom compare, tie-break codes, fixed fixtures or tie-break matches; not stopped nor complete. Returns
/// `this`. The original's SEH frame covers only the unwinding of the base.
/// Original: `League::League` (0x682200, league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn new_league(this: usize) -> usize {
    covered!("cx07_9 league built");
    exe::new_comp(this);
    exe::date_date_54a110(this + league::LAST_RESULT_DATE);
    exe::date_date_54a110(this + league::LAST_UPDATE_DATE);
    write_u8(this + league::COMP_TYPE, 2);
    write_u8(this + league::MATCH_CATEGORY, 2);
    write_u8(this + league::PRIZE_MODE, 2);
    for field in [league::NEXT_DRAW_DAY, league::GAMES_PER_CLUB, league::REQUIRED_CAPACITY, league::REQUIRED_SEATS] {
        write_u16(this + field, 0xffff);
    }
    write_usize(this, LEAGUE_VTABLE);
    for field in [league::MATCH_RULES, league::ROUND_DRAW_RULES, league::NEXT_DRAW_YEAR_OFFSET] {
        write_u16(this + field, 0);
    }
    for field in [league::TABLE_COLUMN, league::STAGES, league::TABLE, league::TIEBREAK_MATCHES, league::FIXED_FIXTURES, league::COMPLETE,
        league::TIEBREAKS, league::CUSTOM_COMPARE] {
        write_usize(this + field, 0);
    }
    write_u8(this + league::STOPPED, 0);
    write_u16(this + league::LEAGUE_FLAGS, 3); // cx07_9: the default rule bits
    this
}

/// Comparer for `new_league`: side by side over the 0xee bytes of the object and the return.
pub(crate) unsafe extern "thiscall" fn compare_new_league(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, LEAGUE_OBJECT_SIZE)], || o(this), || new_league(this));
    SCOREBOARD.record(same && r == n, || format!("new_league({:#x}): original {:x?} rust {:x?}", this, mo, mr));
    r
}

/// A fixed fixture of a league (an entry of `league::FIXED_FIXTURES`, 0x10 bytes) from its date and the two clubs'
/// names: the date (`exe::date_set`(entry, day, month, year, −1), Date::set, thiscall, ret 0x10) at +0, the home club id
/// at +8 and `fixed_fixture::AWAY_CLUB_ID` −1; then the league's table rows in order, comparing each club's short name
/// (`club_get_english_short_name`, the English one when another language is loaded) with `home` and with `away` (byte
/// by byte, as strcmp); a match stores the club's id; the search stops once both are found. A name that matches no
/// club leaves −1.
/// Original: `fixed_fixture_set_by_club_names` (0x68aff0, league.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn fixed_fixture_set_by_club_names(this: usize, league: usize, day: u32, month: u32, year: u32, home: usize, away: usize) {
    exe::date_set(this, day, month, year, -1);
    let home_id = this + 8; // the home club id of the fixed fixture
    write_u32(this + fixed_fixture::AWAY_CLUB_ID, u32::MAX);
    write_u32(home_id, u32::MAX);
    let league = League(league);
    let mut i = 0i32;
    while i < league.club_count() as i32 {
        let club = read_usize(league.table() + i as usize * league_table_row::SIZE + league_table_row::CLUB);
        if guio_same_c_string(club_get_english_short_name(club), home) {
            write_u32(home_id, read_u32(club)); // cx07_9: the home club's id
        }
        if guio_same_c_string(club_get_english_short_name(club), away) {
            write_u32(this + fixed_fixture::AWAY_CLUB_ID, read_u32(club));
        }
        if read_i32(home_id) != -1 && read_i32(this + fixed_fixture::AWAY_CLUB_ID) != -1 {
            break;
        }
        i += 1;
    }
}

/// Comparer for `fixed_fixture_set_by_club_names`: side by side over the 0x10 bytes of the entry.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_fixed_fixture_set_by_club_names(this: usize, league: usize, day: u32, month: u32, year: u32, home: usize, away: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, usize)>(%%I1%%);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(this, 0x10)], || o(this, league, day, month, year, home, away),
        || fixed_fixture_set_by_club_names(this, league, day, month, year, home, away));
    SCOREBOARD.record(same, || format!("fixed_fixture_set_by_club_names({:#x}, {:?} v {:?}): original {:x?} rust {:x?}", this,
        String::from_utf8_lossy(&c_string_bytes(home)), String::from_utf8_lossy(&c_string_bytes(away)), mo, mr));
}
