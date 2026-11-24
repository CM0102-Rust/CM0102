use super::*;

/// Raises the reputations of an award winner: for a player, the world, home and current reputations rise by a fraction
/// of the award's reputations (up to 10000 and at least them); for a non-player, they rise up to the award floors and
/// the board confidence gains 1000. `award_raise_winner_reputation(person, club, nation, world, player)`, called for
/// the player-of-the-month slot (`set_winner`) and for the yearly and national side awards. A player (`.player()`) in a
/// player award: world (`player_sheet::WORLD_REPUTATION`) + 5% (`rdata::F64_0_05`) of the award's world reputation, up
/// to 10000 and at least that value; home (`player_sheet::HOME_REPUTATION`): for the club of his own nation
/// (`.nation()` of the club = `.nation()` of the person), + 10% (`rdata::F64_0_1`) of the national reputation and at
/// least that value; otherwise + 2.5% (`rdata::F64_0_025`) of the sum and at least the average; current
/// (`player_sheet::CURRENT_REPUTATION`) + 10% of the national reputation, at least that value; puts the player in the
/// clubs' lists (`exe::shortlist_manager_add_player_to_club_shortlists`, with 1) if the list manager exists. A
/// non-player (`.non_player()`) in a non-player award: the three rise to the floors (`.world_reputation()` world,
/// `.home_reputation()` by the club's nation, `.current_reputation()` national) and, with a club, the board confidence
/// (+6 of the 73-byte row of `G_MANAGER_MANAGER`) + 1000, between 6500 and 10000. Only the low words of the reputations
/// count.
/// Original: `award_raise_winner_reputation` (0x412f00, award_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_raise_winner_reputation(person: usize, club: usize, nation: u32, world: u32, player: u32) {
    let person = Staff(person);
    let club = Club(club);
    let nation = nation as u16 as i16;
    let world = world as u16 as i16;
    let p = person.player();
    if !p.is_null() && player != 0 {
        covered!("cpw_6 player award");
        add_to_word(p.addr() + player_sheet::WORLD_REPUTATION, ftol(world as i32 as f64 * read_f64(rdata::F64_0_05))); // cpw6: the worldwide increase;
        if p.world_reputation() > 10000 {
            p.set_world_reputation(10000);
        }
        if p.world_reputation() <= world { // cpw6: o piso mundial
            p.set_world_reputation(world);
        }
        if !club.is_null() && club.nation() == person.nation() { // cpw6: the club of the player’s nation
            add_to_word(p.addr() + player_sheet::HOME_REPUTATION, ftol(nation as i32 as f64 * read_f64(rdata::F64_0_1)));
            clamp_word(p.addr() + player_sheet::HOME_REPUTATION, nation);
        } else {
            add_to_word(p.addr() + player_sheet::HOME_REPUTATION, ftol((nation as i32 + world as i32) as f64 * read_f64(rdata::F64_0_025)));
            if p.home_reputation() > 10000 {
                p.set_home_reputation(10000);
            }
            let floor = nation as i32 / 2 + world as i32 / 2;
            if (p.home_reputation() as i32) <= floor {
                p.set_home_reputation(floor as i16);
            }
        }
        add_to_word(p.addr() + player_sheet::CURRENT_REPUTATION, ftol(nation as i32 as f64 * read_f64(rdata::F64_0_1)));
        clamp_word(p.addr() + player_sheet::CURRENT_REPUTATION, nation);
        let lists = read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856);
        if lists != 0 && !MONTH_AWARD_DRY_RUN.load(Relaxed) {
            exe::shortlist_manager_add_player_to_club_shortlists(lists, person, 1);
        }
        return;
    }
    let np = person.non_player();
    if np.is_null() || player != 0 {
        return;
    }
    if np.world_reputation() <= world {
        np.set_world_reputation(world);
    }
    if !club.is_null() && club.nation() == person.nation() {
        if np.home_reputation() <= nation {
            np.set_home_reputation(nation);
        }
    } else {
        let floor = nation as i32 / 2 + world as i32 / 2;
        if (np.home_reputation() as i32) <= floor {
            np.set_home_reputation(floor as i16);
        }
    }
    if np.current_reputation() <= nation {
        np.set_current_reputation(nation);
    }
    if club.is_null() {
        return;
    }
    let row = read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add((read_i32(club.addr()) as usize).wrapping_mul(73)).wrapping_add(6);
    let v = read_i16(row).wrapping_add(1000);
    let v = if v < 1 {
        1
    } else if v > 10000 {
        10000
    } else {
        v
    };
    write_i16(row, if v < 6500 { 6500 } else { v });
}

/// Passes a news text request to one award of a nation: the award `award` of the nation awards `nation` writes the text
/// of `item` into `text` (headline or body by `body`).
///
/// The award object is [[`award_manager_fields::NATION_AWARDS`][nation] + 4][award] (no checks); its virtual +0xc
/// (`write_news`(item, text, body): YEAR_AWARD::write_news, TEAM_AWARD::write_news..., thiscall, ret 0xc), whose eax it
/// returns.
/// Original: `AWARD_MANAGER::write_award_news` (0x4143f0, award_manager.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn write_award_news(this: usize, nation: i32, award: i32, item: usize, text: usize, body: u32) -> u32 {
    let awards = read_usize(read_usize(this + award_manager_fields::NATION_AWARDS).wrapping_add((nation as isize * 4) as usize));
    let object = read_usize(read_usize(awards + 4).wrapping_add((award as isize * 4) as usize));
    covered!("mwg_15 award news");
    let write: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(object) + 0xc));
    write(object, item, text, body) // mwg15: headline or body
}

/// Comparer for `write_award_news`: side by side into two Texts (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_award_news(this: usize, nation: i32, award: i32, item: usize, text: usize, body: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, usize, usize, u32) -> u32>(IDX_WRITE_AWARD_NEWS);
    let mut r = 0;
    compare_text_writer(text, || r = o(this, nation, award, item, text, body), |t| {
        write_award_news(this, nation, award, item, t, body);
    }, || format!("write_award_news(nation {}, award {}, body {})", nation, award, body));
    r
}

/// Appends a shortlist to the award manager's growable array and gives its index.
///
/// When the count (`SHORTLIST_COUNT`, signed word) is not below the capacity (`SHORTLIST_CAPACITY`), the capacity goes up
/// by 5 and the array (`SHORTLISTS`) is allocated (`exe::crt_malloc`, first time) or grown (`exe::crt_realloc`) to 4 ×
/// capacity bytes; out of memory, the array becomes null, the error box (line 0x39a) and −1 (in ax; the original's high
/// word is the box's). Otherwise the shortlist goes into slot `count`, the count goes up by one and the result is the new
/// count − 1 in ax (the original computes it from esi, whose high word is still `this`'s: kept).
/// Original: `AWARD_MANAGER::add_shortlist` (0x4147f0, award_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn add_shortlist(this: usize, shortlist: usize) -> u32 {
    use award_manager_fields as am;
    let capacity = read_u16(this + am::SHORTLIST_CAPACITY);
    if read_i16(this + am::SHORTLIST_COUNT) >= capacity as i16 {
        let capacity = capacity.wrapping_add(5);
        write_u16(this + am::SHORTLIST_CAPACITY, capacity);
        let bytes = ((capacity as i16 as i32) << 2) as u32;
        let array = read_usize(this + am::SHORTLISTS);
        let array = if array == 0 { exe::crt_malloc(bytes) } else { exe::crt_realloc(array, bytes) };
        write_usize(this + am::SHORTLISTS, array);
        if array == 0 {
            show_error_at(AWARD_MANAGER_CPP, 0x39a);
            return 0xffff;
        }
        covered!("mwg_15 shortlists grown");
    }
    let count = read_i16(this + am::SHORTLIST_COUNT);
    write_usize(read_usize(this + am::SHORTLISTS).wrapping_add((count as isize * 4) as usize), shortlist);
    let count = count.wrapping_add(1);
    write_i16(this + am::SHORTLIST_COUNT, count);
    ((this as u32 & 0xffff_0000) | count as u16 as u32).wrapping_sub(1)
}

/// Comparer for `add_shortlist`.
pub(crate) unsafe extern "thiscall" fn compare_add_shortlist(this: usize, shortlist: usize) -> u32 {
    use award_manager_fields as am;
    let before = read_i16(this + am::SHORTLIST_COUNT);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_ADD_SHORTLIST)(this, shortlist);
    let after = read_i16(this + am::SHORTLIST_COUNT);
    let ok = r as u16 == 0xffff || (after == before.wrapping_add(1) && r as u16 as i16 == before
        && read_usize(read_usize(this + am::SHORTLISTS) + before as usize * 4) == shortlist);
    SCOREBOARD.record(ok, || format!("add_shortlist({:#x}) count {} -> {}, returned {:#x}", shortlist, before, after, r));
    r
}

/// Appends a year award history record to the year award file list.
///
/// `AWARD_MANAGER::add_year_award_record(record, first, last)`: `exe::file_llist_add_node` on the first file list
/// (`YEAR_AWARD_FILE`) with (first, last, record), and returns what it returns.
/// Original: `AWARD_MANAGER::add_year_award_record` (0x4142c0, award/award_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn award_manager_add_year_award_record(this: usize, record: usize, first: usize, last: usize) -> usize {
    exe::file_llist_add_node(read_usize(this + award_manager_consts::YEAR_AWARD_FILE), first, last, record as *const u8)
}

/// Comparer for `award_manager_add_year_award_record`.
///
/// The record is written to the history file: it cannot run twice. Only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_award_manager_add_year_award_record(this: usize, record: usize, first: usize, last: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize>(IDX_AWARD_MANAGER_ADD_YEAR_AWARD_RECORD)(this, record, first, last)
}

/// Appends a team award history record to the team award file list.
///
/// `AWARD_MANAGER::add_team_award_record(record, first, last)`: `exe::file_llist_add_node` on the second file list
/// (`TEAM_AWARD_FILE`) with (first, last, record), and returns what it returns.
/// Original: `AWARD_MANAGER::add_team_award_record` (0x414300, award/award_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn award_manager_add_team_award_record(this: usize, record: usize, first: usize, last: usize) -> usize {
    exe::file_llist_add_node(read_usize(this + award_manager_consts::TEAM_AWARD_FILE), first, last, record as *const u8)
}

/// Comparer for `award_manager_add_team_award_record`.
///
/// The record is written to the history file: it cannot run twice. Only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_award_manager_add_team_award_record(this: usize, record: usize, first: usize, last: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize>(IDX_AWARD_MANAGER_ADD_TEAM_AWARD_RECORD)(this, record, first, last)
}

/// The nation award object of a nation: the first of the 30 slots (`NATION_AWARDS`) holding one of kind 1 for that
/// nation, or 0.
///
/// `AWARD_MANAGER::find_nation_awards(nation)`. Without a nation, the game's error box (line 0x2b3) and 0.
/// Original: `AWARD_MANAGER::find_nation_awards` (0x414340, award/award_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn find_nation_awards(this: usize, nation: usize) -> usize {
    use award_manager_consts as am;
    if nation == 0 {
        show_error_at(am::AWARD_MANAGER_CPP, 0x2b3);
        return 0;
    }
    covered!("mwg_6 nation awards");
    let table = read_usize(this + am::NATION_AWARDS);
    (0..am::NATION_AWARD_SLOTS)
        .map(|i| read_usize(table + i * 4))
        .find(|&e| e != 0 && read_u8(e + am::AWARDS_KIND) == 1 && read_usize(e + am::AWARDS_NATION) == nation) // mwg6: the nation's awards
        .unwrap_or(0)
}

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

pub(crate) unsafe extern "thiscall" fn compare_award_manager_process_fixture(this: usize, fixture: usize, record: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_AWARD_MANAGER_PROCESS_FIXTURE)(this, fixture, record)
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

pub(crate) unsafe extern "thiscall" fn compare_award_manager_process_date(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_AWARD_MANAGER_PROCESS_DATE)(this, today)
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, i32, u32, u32, i32, u32, u32) -> usize>(IDX_CONSTRUCT_AWARD);
    let (r, _, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0x1d)], || o(this, a1, a2, staff_comp, national_reputation, world_reputation, club_comp, unused, for_players), || construct_award(this, a1, a2, staff_comp, national_reputation, world_reputation, club_comp, unused, for_players));
    SCOREBOARD.record(equal, || format!("construct_award(staff comp {}, club comp {}) original={:x?} rust={:x?}", staff_comp, club_comp, mo, mr));
    r
}

/// YEAR_RATINGS::record_match_ratings (0x9315d0, thiscall(fixture, record, filter), ret 0xc).
pub(crate) const FN_YEAR_RATINGS_RECORD_MATCH_RATINGS: usize = 0x9315d0;

/// YEAR_RATINGS::reset_if_year (0x931aa0, thiscall(today), ret 4).
pub(crate) const FN_YEAR_RATINGS_RESET_IF_YEAR: usize = 0x931aa0;

/// The award manager's two year ratings (+0x10: every player, +0xc: the selected nations) and the filters (cdecl
/// `filter(person, fixture)`) each one is fed with.
pub(crate) const AWARD_MANAGER_WORLD_YEAR_RATINGS: usize = 0x10;

pub(crate) const AWARD_MANAGER_SELECTED_YEAR_RATINGS: usize = 0xc;

/// award_rating_filter_reputation_over_2000: a player whose current reputation is above 2000.
pub(crate) const AWARD_FILTER_REPUTATION_OVER_2000: usize = 0x414b20;

/// award_rating_filter_selected_nations: Denmark, Belgium, France, Holland or Sweden (the player's or the fixture's).
pub(crate) const AWARD_FILTER_SELECTED_NATIONS: usize = 0x414b40;
