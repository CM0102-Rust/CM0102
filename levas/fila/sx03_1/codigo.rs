// ---------- sx03_1: the award manager's daily and per-match passes and the award base constructor (award_manager.cpp) ----------

/// YEAR_RATINGS::record_match_ratings (0x9315d0, thiscall(fixture, record, filter), ret 0xc).
const FN_YEAR_RATINGS_RECORD_MATCH_RATINGS: usize = 0x9315d0;
/// YEAR_RATINGS::reset_if_year (0x931aa0, thiscall(today), ret 4).
const FN_YEAR_RATINGS_RESET_IF_YEAR: usize = 0x931aa0;
/// The award manager's two year ratings (+0x10: every player, +0xc: the selected nations) and the filters (cdecl
/// `filter(person, fixture)`) each one is fed with.
const AWARD_MANAGER_WORLD_YEAR_RATINGS: usize = 0x10;
const AWARD_MANAGER_SELECTED_YEAR_RATINGS: usize = 0xc;
/// award_rating_filter_reputation_over_2000: a player whose current reputation is above 2000.
const AWARD_FILTER_REPUTATION_OVER_2000: usize = 0x414b20;
/// award_rating_filter_selected_nations: Denmark, Belgium, France, Holland or Sweden (the player's or the fixture's).
const AWARD_FILTER_SELECTED_NATIONS: usize = 0x414b40;

/// Feeds a played match to the award objects: the nation awards of the match's competition nation and both year
/// ratings.
///
/// `AWARD_MANAGER::process_fixture(fixture, record)`, from the end of a match (match_day.rs, with `G_AWARD_MANAGER`). A
/// friendly (`fixture::CATEGORY` 1) counts for nothing. The nation is the competition's (`competition::NATION`; none
/// without a fixture, a competition or a nation); the first of the 30 nation award objects
/// (`award_manager_consts::NATION_AWARDS`) whose nation (`award_manager_consts::AWARDS_NATION`) is that one gets the
/// match through its virtual +8 (`record_match_ratings(fixture, record)`). With no nation, the first object with no
/// nation takes it (the international awards). Then the world year ratings (+0x10, the players above 2000 of current
/// reputation) and the selected nations' year ratings (+0xc), each when it exists.
/// Original: `AWARD_MANAGER::process_fixture` (0x4134c0, award_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn award_manager_process_fixture(this: usize, fixture: usize, record: usize) {
    use award_manager_consts as am;
    let game = Fixture(fixture);
    if game.category() == 1 {
        return;
    }
    let mut nation = 0;
    if fixture != 0 && !game.competition().is_null() && !game.competition().nation().is_null() {
        nation = game.competition().nation().addr();
    }
    let table = read_usize(this + am::NATION_AWARDS);
    if let Some(awards) = (0..am::NATION_AWARD_SLOTS).map(|i| read_usize(table + i * 4)).find(|&e| e != 0 && read_usize(e + am::AWARDS_NATION) == nation) {
        covered!("sx03_1 nation awards take the match");
        // virtual +8: record_match_ratings(fixture, record), thiscall, ret 8
        let record_match: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(read_usize(read_usize(awards) + 8));
        record_match(awards, fixture, record); // sx03_1: the nation's awards
    }
    let year_ratings: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_YEAR_RATINGS_RECORD_MATCH_RATINGS);
    let world = read_usize(this + AWARD_MANAGER_WORLD_YEAR_RATINGS);
    if world != 0 {
        covered!("sx03_1 world year ratings");
        year_ratings(world, fixture, record, AWARD_FILTER_REPUTATION_OVER_2000); // sx03_1: the world year ratings
    }
    let selected = read_usize(this + AWARD_MANAGER_SELECTED_YEAR_RATINGS);
    if selected != 0 {
        year_ratings(selected, fixture, record, AWARD_FILTER_SELECTED_NATIONS);
    }
}

// simplification: the nation awards and the year ratings add entries (they may allocate) and give the month awards:
// the routine cannot run twice. Original only; the copy is proven by the career.
pub(crate) unsafe extern "thiscall" fn compare_award_manager_process_fixture(this: usize, fixture: usize, record: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%)(this, fixture, record)
}

/// Runs the day for the award objects: each nation award object's daily slot, the past award block and both year
/// ratings.
///
/// `AWARD_MANAGER::process_date(today)`, once a day. Each of the 30 nation award objects
/// (`award_manager_consts::NATION_AWARDS`) that exists gets its virtual +4 (`daily_update(today)`), in slot order.
/// When the past award flag (+0xdc) is set, the 0x30 dwords of the past award state (+0x1c) and the flag are cleared.
/// Then the world year ratings (+0x10) and the selected nations' year ratings (+0xc) are reset when the year is theirs
/// (`YEAR_RATINGS::reset_if_year(today)`), each when it exists.
/// Original: `AWARD_MANAGER::process_date` (0x413540, award_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn award_manager_process_date(this: usize, today: usize) {
    use award_manager_consts as am;
    let table = read_usize(this + am::NATION_AWARDS);
    for i in 0..am::NATION_AWARD_SLOTS {
        let awards = read_usize(table + i * 4);
        if awards != 0 {
            covered!("sx03_1 daily slot");
            // virtual +4: daily_update(today), thiscall, ret 4
            let daily: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_usize(read_usize(awards) + 4));
            daily(awards, today); // sx03_1: each nation's awards
        }
    }
    // +0xdc: the past award state block (+0x1c, 0x30 dwords) holds something
    if read_u32(this + 0xdc) != 0 {
        covered!("sx03_1 past award block cleared");
        for k in 0..0x30usize {
            write_u32(this + 0x1c + k * 4, 0);
        }
        write_u32(this + 0xdc, 0);
    }
    let reset: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_YEAR_RATINGS_RESET_IF_YEAR);
    for ratings in [read_usize(this + AWARD_MANAGER_WORLD_YEAR_RATINGS), read_usize(this + AWARD_MANAGER_SELECTED_YEAR_RATINGS)] {
        if ratings != 0 {
            reset(ratings, today);
        }
    }
}

// simplification: the daily slots give the awards (news, history records, reputations): the routine cannot run twice.
// Original only; the copy is proven by the career.
pub(crate) unsafe extern "thiscall" fn compare_award_manager_process_date(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, today)
}

/// Builds the base part of an award: its vtable, competitions, the two values the award class passes on, its
/// reputations and whether it is for players.
///
/// `AWARD::AWARD(a1, a2, staff_comp, national_reputation, world_reputation, club_comp, unused, for_players)`, from the
/// month, year and team award constructors. The vtable is the base one (0x967864); +5 is the staff competition record
/// `staff_comp` of `G_STAFF_COMP_TABLE` (101-byte records, no check for −1); +9 is the club competition record
/// `club_comp` of `G_CLUB_COMP_TABLE` (107-byte records), or 0 for −1; +0xd and +0x11 take `a1` and
/// `a2`; the reputations go to `award_fields::NATIONAL_REPUTATION` and `award_fields::WORLD_REPUTATION` (their low
/// words) and the player flag to `award_fields::FOR_PLAYERS`. The seventh argument is not read. Returns the object.
/// Original: `AWARD::AWARD` (0x412e40, award_manager.cpp, thiscall, ret 0x20).
pub(crate) unsafe extern "thiscall" fn construct_award(this: usize, a1: u32, a2: u32, staff_comp: i32, national_reputation: u32, world_reputation: u32, club_comp: i32, _unused: u32, for_players: u32) -> usize {
    covered!("sx03_1 award built");
    write_u32(this, 0x967864);
    write_usize(this + 5, read_usize(G_STAFF_COMP_TABLE).wrapping_add((staff_comp as usize).wrapping_mul(101)));
    if club_comp == -1 {
        write_usize(this + 9, 0);
    } else {
        write_usize(this + 9, read_usize(G_CLUB_COMP_TABLE).wrapping_add((club_comp as usize).wrapping_mul(107)));
    }
    write_u32(this + 0xd, a1);
    write_u32(this + 0x11, a2);
    write_u16(this + award_fields::NATIONAL_REPUTATION, national_reputation as u16); // sx03_1: the national reputation
    write_u16(this + award_fields::WORLD_REPUTATION, world_reputation as u16);
    write_u32(this + award_fields::FOR_PLAYERS, for_players);
    this
}

/// Comparer for `construct_award`: side by side over the 0x1d bytes of the base part.
pub(crate) unsafe extern "thiscall" fn compare_construct_award(this: usize, a1: u32, a2: u32, staff_comp: i32, national_reputation: u32, world_reputation: u32, club_comp: i32, unused: u32, for_players: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, i32, u32, u32, i32, u32, u32) -> usize>(%%I2%%);
    let (r, _, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0x1d)], || o(this, a1, a2, staff_comp, national_reputation, world_reputation, club_comp, unused, for_players), || construct_award(this, a1, a2, staff_comp, national_reputation, world_reputation, club_comp, unused, for_players));
    SCOREBOARD.record(equal, || format!("construct_award(staff comp {}, club comp {}) original={:x?} rust={:x?}", staff_comp, club_comp, mo, mr));
    r
}
