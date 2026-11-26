use super::*;

/// Closes a nation's month on its last day: when tomorrow (`today` + 1 day) falls in another month or year than the
/// one the nation's month ratings cover, it runs the nation's month awards (the virtual `on_month_change`, slot +0xc:
/// player, young player and manager of the month) and then empties the month ratings for tomorrow's month and year
/// (`MONTH_RATINGS::reset`, the ratings pointer read again after the awards). A nation without month ratings does
/// nothing. The base of every nation's `daily_update`, called at its end with the day being played.
/// Original: `NationAwards::daily_update` (0x768310, nation_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nation_awards_daily_update(this: usize, today: usize) {
    let Some((month, year)) = nation_awards_new_month(this, today) else {
        return;
    };
    covered!("sx04_1 month change");
    let on_month_change: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + nation_awards_obj::ON_MONTH_CHANGE_SLOT));
    on_month_change(this); // sx04_1: the month awards
    let reset: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_MONTH_RATINGS_RESET);
    reset(read_usize(this + nation_awards_obj::MONTH_RATINGS), month, year); // sx04_1: the new month's ratings
}

/// Comparer for `nation_awards_daily_update`.
///
/// The month awards and the reset cannot run twice: only the original runs. The copy's decision is worked out first
/// (pure: tomorrow's date and the ratings' month); then, on a month change, the ratings must be empty and cover
/// tomorrow's month and year; otherwise their month and year must be unchanged.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_daily_update(this: usize, today: usize) {
    let plan = nation_awards_new_month(this, today);
    let ratings = read_usize(this + nation_awards_obj::MONTH_RATINGS);
    let before = if ratings != 0 { (read_u8(ratings + month_ratings_fields::MONTH), read_i16(ratings + month_ratings_fields::YEAR)) } else { (0, 0) };
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_NATION_AWARDS_DAILY_UPDATE)(this, today);
    let ratings = read_usize(this + nation_awards_obj::MONTH_RATINGS);
    if ratings == 0 {
        SCOREBOARD.record(plan.is_none(), || format!("nation_awards_daily_update(today {:#x}): no month ratings, rust planned {:?}", read_u32(today), plan));
        return;
    }
    let after = (read_u8(ratings + month_ratings_fields::MONTH), read_i16(ratings + month_ratings_fields::YEAR));
    let ok = match plan {
        Some((m, y)) => after == (m as u8, y as u16 as i16) && read_u32(ratings + 4) == 0,
        None => after == before,
    };
    SCOREBOARD.record(ok, || format!("nation_awards_daily_update(today {:#x}): month/year before {:?} after {:?}, rust planned {:?}", read_u32(today), before, after, plan));
}

/// Gives a yearly player award from a competition's season statistics: the three best of the competition's player
/// table for the award's statistic (`comp_stats_get_sorted_player_table`, at most 100) who are players, play one of the
/// `positions` (16 bits; 0 = any), have started (appearances − substitute appearances) at least the share of the
/// league's games ((club count − 1) × legs) and meet the filters (−1 = none): `nation` the person's nation,
/// `club_nation` his club's nation, `not_nation` not the person's nation, `home_nation` both. The share starts at 0.66
/// (0.1 for the statistics 2, 4 and 5) and drops by 0.1 a pass while it stays above 0, the chosen kept. With an `age`
/// (not 0xff), those older are counted and the younger only qualify after three were counted (a count never reset).
/// With all three: the record (the year of `today`, each winner's names, staff id, the statistic for award 2 or 0, the
/// club) goes to `YEAR_AWARD::record_winners`. A missing competition or a cup: the nation_awards.cpp 737 box.
/// Original: `nation_awards_award_player_of_year` (0x768cf0, nation_awards.cpp, thiscall, ret 0x28).
pub(crate) unsafe extern "thiscall" fn nation_awards_award_player_of_year(_this: usize, comp_index: u32, today: usize, award: usize, age: u32,
    positions: u32, stat: u32, nation: i32, club_nation: i32, not_nation: i32, home_nation: i32) {
    nation_awards_award_player_of_year_with(comp_index, today, award, age, positions, stat, nation, club_nation, not_nation, home_nation,
        &mut |award, record| record_year_award(award, record));
}

/// Gives a league's yearly manager award: the league's clubs (its virtual +0xa0) go to
/// `Staff_Records::get_manager_of_year_candidates`, which sorts them by their manager's points and keeps those whose
/// manager qualifies; with exactly three, their managers (names, staff id, club id; the score left unwritten) go with
/// the year of `today` to `YEAR_AWARD::record_winners`. The club list is freed. Not a league: the nation_awards.cpp
/// 468 box (no competition) or 475 box (not a league). Defect of the original: the record's score dwords are never
/// written, so the history keeps 12 bytes of stack garbage per award.
/// Original: `nation_awards_award_manager_of_year` (0x768940, nation_awards.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn nation_awards_award_manager_of_year(_this: usize, today: usize, award: usize, comp_index: u32) {
    nation_awards_award_manager_of_year_with(today, award, comp_index, &mut |award, record| record_year_award(award, record));
}

/// Picks a league's team of the season: the award's substitutes (+0x253) become those its club competition names (3
/// without one) and the selection is cleared; then, for shares of the league's games from 0.66 down by 0.1 while above
/// 0 (the games per club, or (club count − 1) × legs when not worked out), each player of the league's rating table
/// (statistic 0x11) who started more games than that share is offered to the award with his average rating (rating
/// sum / appearances), until the award reports the team full. The table is freed, the season dates set for the
/// current year and the team recorded (`TEAM_AWARD::record_winners`: history, reputations). A missing competition or a
/// cup: the nation_awards.cpp 1624 box. No rated player: nothing (the selection is kept).
/// Original: `NationAwards::award_team_of_season` (0x7698f0, nation_awards.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn nation_awards_award_team_of_season(_this: usize, comp_index: u32, award: usize) {
    nation_awards_award_team_of_season_with(comp_index, award, &mut |award| record_team_award(award));
}

/// Comparer for `nation_awards_award_player_of_year`: the copy runs as a dry run (no record), then the original; its
/// newest year award history record must be the planned one. The error box: only the original.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_award_player_of_year(this: usize, comp_index: u32, today: usize, award: usize, age: u32,
    positions: u32, stat: u32, nation: i32, club_nation: i32, not_nation: i32, home_nation: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, usize, u32, u32, u32, i32, i32, i32, i32)>(IDX_NATION_AWARDS_AWARD_PLAYER_OF_YEAR);
    let league = award_league(comp_index);
    if league.is_null() || league.comp_type() == 1 {
        return o(this, comp_index, today, award, age, positions, stat, nation, club_nation, not_nation, home_nation);
    }
    let mut planned = None;
    crate::rng::pause_trace(|| nation_awards_award_player_of_year_with(comp_index, today, award, age, positions, stat, nation, club_nation,
        not_nation, home_nation, &mut |_, r| planned = Some(r.to_vec())));
    let list = read_usize(read_usize(G_AWARD_MANAGER) + award_manager_consts::YEAR_AWARD_FILE);
    compare_award_record("nation_awards_award_player_of_year", list, award + YEAR_AWARD_HISTORY_FIRST, award + YEAR_AWARD_HISTORY_LAST, planned, &[],
        || o(this, comp_index, today, award, age, positions, stat, nation, club_nation, not_nation, home_nation));
}

/// Comparer for `nation_awards_award_manager_of_year`: as the player award's, without the score dwords the original
/// never writes. The error boxes: only the original.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_award_manager_of_year(this: usize, today: usize, award: usize, comp_index: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_NATION_AWARDS_AWARD_MANAGER_OF_YEAR);
    let league = award_league(comp_index);
    if league.is_null() || league.comp_type() != 2 {
        return o(this, today, award, comp_index);
    }
    let mut planned = None;
    crate::rng::pause_trace(|| nation_awards_award_manager_of_year_with(today, award, comp_index, &mut |_, r| planned = Some(r.to_vec())));
    let list = read_usize(read_usize(G_AWARD_MANAGER) + award_manager_consts::YEAR_AWARD_FILE);
    let garbage: Vec<(usize, usize)> = (0..3).map(|k| (2 + k * YEAR_RECORD_ENTRY + YEAR_RECORD_SCORE, 4)).collect();
    compare_award_record("nation_awards_award_manager_of_year", list, award + YEAR_AWARD_HISTORY_FIRST, award + YEAR_AWARD_HISTORY_LAST, planned, &garbage,
        || o(this, today, award, comp_index));
}

/// Comparer for `nation_awards_award_team_of_season`: the copy runs as a dry run on the award's selection (kept, then
/// put back), then the original; its newest team award history record must be the copy's selection. The error box:
/// only the original.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_award_team_of_season(this: usize, comp_index: u32, award: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize)>(IDX_NATION_AWARDS_AWARD_TEAM_OF_SEASON);
    let league = award_league(comp_index);
    if league.is_null() || league.comp_type() == 1 {
        return o(this, comp_index, award);
    }
    let selection = [(award + TEAM_AWARD_RECORD, TEAM_AWARD_SUBSTITUTES_OFFSET + 1 - TEAM_AWARD_RECORD)];
    let before = crate::comparison::MemorySnapshot::read(&selection);
    let mut planned = None;
    crate::rng::pause_trace(|| nation_awards_award_team_of_season_with(comp_index, award,
        &mut |a| planned = Some((0..TEAM_RECORD_SIZE).map(|i| read_u8(a + TEAM_AWARD_RECORD + i)).collect::<Vec<u8>>())));
    before.write_back();
    let list = read_usize(read_usize(G_AWARD_MANAGER) + award_manager_consts::TEAM_AWARD_FILE);
    compare_award_record("nation_awards_award_team_of_season", list, award + TEAM_AWARD_HISTORY_FIRST, award + TEAM_AWARD_HISTORY_LAST, planned, &[],
        || o(this, comp_index, award));
}

/// The month and the year (with the leap-year word above it, as the original pushes it) of tomorrow, when tomorrow is
/// in another month than the one the nation's month ratings cover; `None` when it is the same month or the nation has
/// no month ratings.
pub(crate) unsafe fn nation_awards_new_month(this: usize, today: usize) -> Option<(u32, u32)> {
    let mut tomorrow = [0u8; 8];
    exe::date_add_days(today, tomorrow.as_mut_ptr() as usize, 1);
    let ratings = read_usize(this + nation_awards_obj::MONTH_RATINGS);
    if ratings == 0 {
        return None;
    }
    let month = exe::date_get_month(tomorrow.as_ptr() as usize);
    let year = read_u16(tomorrow.as_ptr() as usize + cm_date::YEAR);
    // the ratings' year is signed, tomorrow's is not
    if read_u8(ratings + month_ratings_fields::MONTH) == month && read_i16(ratings + month_ratings_fields::YEAR) as i32 == year as i32 {
        return None;
    }
    Some((month as u32, read_u32(tomorrow.as_ptr() as usize + cm_date::YEAR)))
}

/// The game's sink: `YEAR_AWARD::record_winners(record, 1)`.
pub(crate) unsafe fn record_year_award(award: usize, record: &[u8; YEAR_RECORD_SIZE]) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_YEAR_AWARD_RECORD_WINNERS);
    f(award, record.as_ptr() as usize, 1);
}

/// The game's sink: `TEAM_AWARD::record_winners()`.
pub(crate) unsafe fn record_team_award(award: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TEAM_AWARD_RECORD_WINNERS);
    f(award);
}

/// The league object of the game's competition table (`G_COMP_OBJECT_TABLE`) at `index`.
pub(crate) unsafe fn award_league(index: u32) -> League {
    League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(index as usize * 4)))
}

/// The yearly player award of a competition from its season statistics (see `nation_awards_award_player_of_year`),
/// with the winners record going to `sink`.
pub(crate) unsafe fn nation_awards_award_player_of_year_with(comp_index: u32, today: usize, award: usize, age: u32, positions: u32,
    stat: u32, nation: i32, club_nation: i32, not_nation: i32, home_nation: i32, sink: YearAwardSink) {
    let stat_full = stat;
    let stat = stat as u8;
    // the share of the season's games a candidate must have started, lowered by 0.1 a pass
    let mut share: f32 = if stat == 2 || (stat > 3 && stat <= 5) { f32::from_bits(0x3dcccccd) } else { f32::from_bits(0x3f28f5c3) }; // 0.1 or 0.66
    let league = award_league(comp_index);
    if league.is_null() || league.comp_type() == 1 {
        tel26_error_box(NATION_AWARDS_CPP, 0x2e1);
        return;
    }
    let games = award_league_games(league);
    let mut n = 0i32;
    let rows = exe::comp_stats_get_sorted_player_table(league.comp_stats(), stat_full, &mut n, 0x64, u32::MAX);
    if n == 0 {
        return;
    }
    let mut chosen = [Staff(0); 3];
    let mut scores = [0u32; 3];
    let mut found = 0usize;
    let mut older = 0i32; // over the age limit: counted across the passes, never reset
    while share as f64 > 0.0 {
        let least = ftol(games as i32 as f64 * share as f64) as i16 as i32;
        for k in 0..n.max(0) as usize {
            let row = rows + k * comp_stats_obj::PLAYER_ENTRY_SIZE;
            let person = staff_at(read_i32(row + comp_stats_player::STAFF_ID) as isize);
            if person.player().is_null() {
                continue;
            }
            if positions as u16 != 0 && exe::staff_get_natural_position_mask(person) as u16 & positions as u16 == 0 {
                continue;
            }
            if chosen[..found].contains(&person) {
                continue;
            }
            let starts = read_u8(row + ROW_APPEARANCES) as i32 - read_u8(row + ROW_SUB_APPEARANCES) as i32;
            if starts < least { // sx04_2: enough starts
                continue;
            }
            let own = person.nation();
            let club_of = person.club_job();
            let club_nation_id = || if club_of.is_null() || club_of.nation().is_null() { None } else { Some(read_i32(club_of.nation().addr())) };
            if nation != -1 && (own.is_null() || read_i32(own.addr()) != nation) {
                continue;
            }
            if club_nation != -1 && club_nation_id() != Some(club_nation) {
                continue;
            }
            if not_nation != -1 && (own.is_null() || read_i32(own.addr()) == not_nation) {
                continue;
            }
            if home_nation != -1 && (club_nation_id() != Some(home_nation) || own.is_null() || read_i32(own.addr()) != home_nation) {
                continue;
            }
            if age as u8 != 0xff {
                if (person.age() as i8) > age as u8 as i8 {
                    older += 1;
                    continue;
                }
                if older < 3 {
                    continue;
                }
            }
            covered!("sx04_2 player of the year candidate");
            chosen[found] = person;
            scores[found] = if stat == 2 { read_u8(row + ROW_STAT) as u32 } else { 0 };
            found += 1;
            if found == 3 {
                break;
            }
        }
        share = (share as f64 - read_f64(rdata::F64_0_1)) as f32;
        if found == 3 {
            break;
        }
    }
    exe::crt_free(rows);
    if found != 3 {
        return;
    }
    let mut record = [0u8; YEAR_RECORD_SIZE];
    record[..2].copy_from_slice(&read_u16(today + cm_date::YEAR).to_le_bytes());
    for k in 0..3 {
        let club = chosen[k].club_job();
        year_record_entry(&mut record, k, chosen[k], Some(scores[k]), if club.is_null() { u32::MAX } else { read_u32(club.addr()) });
    }
    sink(award, &record);
}

/// The yearly manager award of a league (see `nation_awards_award_manager_of_year`), with the winners record going to
/// `sink`.
pub(crate) unsafe fn nation_awards_award_manager_of_year_with(today: usize, award: usize, comp_index: u32, sink: YearAwardSink) {
    let league = award_league(comp_index);
    if league.is_null() {
        tel26_error_box(NATION_AWARDS_CPP, 0x1d4);
        return;
    }
    if league.comp_type() != 2 {
        tel26_error_box(NATION_AWARDS_CPP, 0x1db);
        return;
    }
    // the league's clubs (virtual +0xa0: a block the caller frees, and its count)
    let (mut clubs, mut n) = (0usize, 0u32);
    let get_clubs: unsafe extern "thiscall" fn(usize, *mut usize, *mut u32) = core::mem::transmute(read_usize(read_usize(league.addr()) + 0xa0));
    get_clubs(league.addr(), &mut clubs, &mut n);
    let candidates: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_OF_YEAR_CANDIDATES);
    if candidates(read_usize(G_STAFF_RECORDS), clubs, n) != 3 { // sx04_2: three managers
        exe::crt_free(clubs);
        return;
    }
    covered!("sx04_2 manager of the year");
    let mut record = [0u8; YEAR_RECORD_SIZE];
    record[..2].copy_from_slice(&read_u16(today + cm_date::YEAR).to_le_bytes());
    for k in 0..3 {
        let club = Club(read_usize(clubs + k * 4));
        // defect of the original: the score dword is not written (whatever the stack held goes to the history)
        year_record_entry(&mut record, k, club.manager(), None, read_u32(club.addr()));
    }
    sink(award, &record);
    exe::crt_free(clubs);
}

/// The team of the season of a league (see `nation_awards_award_team_of_season`), with the final record going to
/// `sink`.
pub(crate) unsafe fn nation_awards_award_team_of_season_with(comp_index: u32, award: usize, sink: TeamAwardSink) {
    let mut share: f32 = f32::from_bits(0x3f28f5c3); // 0.66 of the league's games
    let league = award_league(comp_index);
    if league.is_null() || league.comp_type() == 1 {
        tel26_error_box(NATION_AWARDS_CPP, 0x658);
        return;
    }
    let games = if league.games_per_club() != -1 { league.games_per_club() } else { award_league_games(league) };
    let mut n = 0i32;
    let rows = exe::comp_stats_get_sorted_player_table(league.comp_stats(), 0x11, &mut n, -1i32 as u32, u32::MAX);
    if n == 0 {
        return;
    }
    // the substitutes the award names: those of its club competition, 3 without one
    let mut substitutes = 3u8;
    let table = read_usize(G_COMP_OBJECT_TABLE);
    let club_comp = read_usize(award + 9);
    if table != 0 && club_comp != 0 {
        let comp = League(read_usize(table + read_u32(club_comp) as usize * 4));
        if !comp.is_null() {
            substitutes = comp.substitutes_named();
        }
    }
    write_u8(award + TEAM_AWARD_SUBSTITUTES_OFFSET, substitutes);
    let clear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TEAM_AWARD_CLEAR_SELECTION);
    clear(award);
    let add: unsafe extern "thiscall" fn(usize, usize, usize, f32, u32) -> u32 = core::mem::transmute(FN_TEAM_AWARD_ADD_CANDIDATE);
    'passes: loop {
        let least = ftol(games as i32 as f32 as f64 * share as f64) as i16 as i32;
        for k in 0..n.max(0) as usize {
            let row = rows + k * comp_stats_obj::PLAYER_ENTRY_SIZE;
            let person = staff_at(read_i32(row + comp_stats_player::STAFF_ID) as isize);
            let appearances = read_u8(row + ROW_APPEARANCES) as i32;
            let starts = appearances - read_u8(row + ROW_SUB_APPEARANCES) as i32;
            if starts <= least { // sx04_2: more starts than the share
                continue;
            }
            let rating = (read_i16(row + ROW_RATING_SUM) as i32 as f64 / appearances as f64) as f32;
            covered!("sx04_2 team of the season candidate");
            if add(award, person.addr(), person.club_job().addr(), rating, 0) != 0 {
                break 'passes;
            }
        }
        share = (share as f64 - read_f64(rdata::F64_0_1)) as f32;
        if share as f64 <= 0.0 {
            break;
        }
    }
    exe::crt_free(rows);
    let dates: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_TEAM_AWARD_SET_SEASON_DATES);
    dates(award, read_u16(G_CURRENT_DATE + cm_date::YEAR) as u32);
    sink(award);
}

/// Checks an award routine against its dry run: `planned` is the record the copy would give `record_winners` (none:
/// it would record nothing). The original runs; then either the award's history offsets did not move, or the newest
/// record of the history `list` is the planned one, except the `ignored` byte ranges.
pub(crate) unsafe fn compare_award_record(name: &str, list: usize, first: usize, last: usize, planned: Option<Vec<u8>>, ignored: &[(usize, usize)], original: impl FnOnce()) {
    let before = (read_u32(first), read_u32(last));
    original();
    let after = (read_u32(first), read_u32(last));
    let (ok, got) = match &planned {
        None => (after == before, None),
        Some(p) => {
            let got = newest_history_record(list, first, last, p.len());
            let same = got.as_ref().map_or(false, |g| (0..p.len()).all(|i| ignored.iter().any(|&(o, l)| i >= o && i < o + l) || g[i] == p[i]));
            (after != before && same, got)
        }
    };
    SCOREBOARD.record(ok, || format!("{}: history offsets {:x?} -> {:x?}, rust planned {:02x?}, original recorded {:02x?}", name, before, after, planned, got));
}

/// The league games a club plays: (club count − 1) × legs, as a 16-bit product.
pub(crate) unsafe fn award_league_games(league: League) -> i16 {
    league.club_count().wrapping_sub(1).wrapping_mul(league.legs())
}

/// Writes entry `k` of a year award record: the person's three name ids and staff id, the club id; the score only
/// when given (the manager award leaves it as the stack had it).
pub(crate) unsafe fn year_record_entry(record: &mut [u8; YEAR_RECORD_SIZE], k: usize, person: Staff, score: Option<u32>, club_id: u32) {
    let e = 2 + k * YEAR_RECORD_ENTRY;
    let mut put = |off: usize, v: u32| record[e + off..e + off + 4].copy_from_slice(&v.to_le_bytes());
    put(0, award_name_id_of_name(person.first_name()));
    put(4, award_name_id_of_name(person.second_name()));
    put(8, award_name_id_of_name(person.common_name()));
    put(0xc, read_u32(person.addr()));
    if let Some(s) = score {
        put(YEAR_RECORD_SCORE, s);
    }
    put(0x14, club_id);
}

/// The newest record (`size` bytes) of a history file list between the offsets at `first` and `last`, if any.
pub(crate) unsafe fn newest_history_record(list: usize, first: usize, last: usize, size: usize) -> Option<Vec<u8>> {
    let nodes = exe::file_llist_get_nodes(list, read_u32(first), read_u32(last));
    if nodes == 0 {
        return None;
    }
    write_usize(nodes + llist::CURSOR, read_usize(nodes + llist::TAIL));
    let d = exe::llist_get_prev(nodes);
    let r = if d != 0 { Some((0..size).map(|i| read_u8(d + i)).collect()) } else { None };
    exe::llist_scalar_deleting_destructor(nodes, 1);
    r
}

/// A name record's id (`names::ID`), −1 without one.
pub(crate) unsafe fn award_name_id_of_name(name: usize) -> u32 {
    if name != 0 { read_u32(name + names::ID) } else { u32::MAX }
}

/// `MONTH_RATINGS::reset(month, year)` (0x767f60, thiscall, ret 8); the month and the year go as the original pushes
/// them, in whole dwords.
pub(crate) const FN_MONTH_RATINGS_RESET: usize = 0x767f60;

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\nation_awards.cpp" (the error boxes).
pub(crate) const NATION_AWARDS_CPP: usize = 0xa5b26c;

/// `YEAR_AWARD::record_winners(record, add_to_list)` (0x9302f0, thiscall, ret 8): the winners record goes to the
/// history, the three winners' reputations rise and their staff records get the award.
pub(crate) const FN_YEAR_AWARD_RECORD_WINNERS: usize = 0x9302f0;

/// `TEAM_AWARD::clear_selection()` (0x8afd90, thiscall).
pub(crate) const FN_TEAM_AWARD_CLEAR_SELECTION: usize = 0x8afd90;

/// `TEAM_AWARD::add_candidate(person, club, f32 rating, 0)` (0x8afa40, thiscall, ret 0x10): nonzero when the team is full.
pub(crate) const FN_TEAM_AWARD_ADD_CANDIDATE: usize = 0x8afa40;

/// `TEAM_AWARD::set_season_dates(year)` (0x8aff50, thiscall, ret 4).
pub(crate) const FN_TEAM_AWARD_SET_SEASON_DATES: usize = 0x8aff50;

/// `TEAM_AWARD::record_winners()` (0x8affb0, thiscall): the team goes to the history, reputations rise, the selection
/// is cleared.
pub(crate) const FN_TEAM_AWARD_RECORD_WINNERS: usize = 0x8affb0;

/// `Staff_Records::get_manager_of_year_candidates(clubs, n)` (0x86fab0, thiscall, ret 8): sorts the clubs, keeps up to
/// three at the start and returns how many (al).
pub(crate) const FN_STAFF_RECORDS_GET_MANAGER_OF_YEAR_CANDIDATES: usize = 0x86fab0;

pub(crate) const YEAR_RECORD_ENTRY: usize = 0x18;

/// The entry's score dword (+0x10 of an entry): the statistic for the stat-2 awards, 0 for the others.
pub(crate) const YEAR_RECORD_SCORE: usize = 0x10;

/// A year award's offsets of its records in the year award history file (`YEAR_AWARD +0x23 first, +0x27 last`).
pub(crate) const YEAR_AWARD_HISTORY_FIRST: usize = 0x23;

pub(crate) const YEAR_AWARD_HISTORY_LAST: usize = 0x27;

/// A team award's selection: the 0x205 bytes from +0x4e (twenty 0x19-byte candidates, the season dates +0x242/+0x24a,
/// the count +0x252) the history file keeps, then the number of substitutes (+0x253), then its history offsets.
pub(crate) const TEAM_AWARD_RECORD: usize = 0x4e;

pub(crate) const TEAM_RECORD_SIZE: usize = 0x205;

pub(crate) const TEAM_AWARD_SUBSTITUTES_OFFSET: usize = 0x253;

pub(crate) const TEAM_AWARD_HISTORY_FIRST: usize = 0x254;

pub(crate) const TEAM_AWARD_HISTORY_LAST: usize = 0x258;

/// A row of the competition's sorted player table (`comp_stats_get_sorted_player_table`, 0x2d bytes): appearances
/// (+0xc), of them as a substitute (+0xd), the statistic of the stat-2 awards (+0xe) and the rating sum (i16, +0x1a).
pub(crate) const ROW_APPEARANCES: usize = 0xc;

pub(crate) const ROW_SUB_APPEARANCES: usize = 0xd;

pub(crate) const ROW_STAT: usize = 0xe;

pub(crate) const ROW_RATING_SUM: usize = 0x1a;

/// What the award routines do with their winners: record them in the game, or (a comparer's dry run) only keep them.
pub(crate) type YearAwardSink<'a> = &'a mut dyn FnMut(usize, &[u8; YEAR_RECORD_SIZE]);

pub(crate) type TeamAwardSink<'a> = &'a mut dyn FnMut(usize);

/// The year award winners record (`YEAR_AWARD::record_winners`): the year (word) and three 0x18-byte entries (first,
/// second and common name ids, staff id, a score, club id); the history file keeps it whole.
pub(crate) const YEAR_RECORD_SIZE: usize = 0x4a;

/// Builds the base of every nation's awards object: the base vtable, no award table (`nation_awards_fields::TABLE`),
/// no awards (`COUNT`, i16) and no month ratings (`MONTH_RATINGS`). Called first by each nation's constructor, when the game is
/// created or loaded. Returns `this`.
/// Original: `NationAwards::NationAwards` (0x768290, nation_awards.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn construct_nation_awards(this: usize) -> usize {
    covered!("sx04b_2 nation awards built");
    write_u32(this, NATION_AWARDS_BASE_VTABLE); // sx04b_2: the base vtable
    write_u32(this + nation_awards_fields::TABLE, 0);
    write_u16(this + nation_awards_fields::COUNT, 0);
    write_u32(this + nation_awards_fields::MONTH_RATINGS, 0); // sx04b_2: no month ratings yet
    this
}

/// Comparer for `construct_nation_awards`: side by side over the 0xe bytes of the base.
pub(crate) unsafe extern "thiscall" fn compare_construct_nation_awards(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_CONSTRUCT_NATION_AWARDS);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0xe)], || o(this), || construct_nation_awards(this));
    SCOREBOARD.record(equal && r == n, || format!("construct_nation_awards original={:x?} rust={:x?}", mo, mr));
    r
}

/// Passes a played match to the nation's month ratings (`MONTH_RATINGS::record_match_ratings` with the same fixture
/// and record), when the nation has month ratings (`nation_awards_fields::MONTH_RATINGS`). The leagues' virtual
/// `record_match_ratings` (England's and Finland's at 0x5a96b0) call it for each league match.
/// Original: `NationAwards::record_match_ratings` (0x7683a0, nation_awards.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn nation_awards_record_match_ratings(this: usize, fixture: usize, record: usize) {
    let ratings = read_usize(this + nation_awards_fields::MONTH_RATINGS);
    if ratings == 0 {
        return;
    }
    covered!("sx04b_3 match to the month ratings");
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MONTH_RATINGS_RECORD_MATCH_RATINGS);
    f(ratings, fixture, record); // sx04b_3: the month ratings
}

/// Comparer for `nation_awards_record_match_ratings`: the ratings take each match once, so only the original runs (the
/// month ratings routine it calls has its own comparer).
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_record_match_ratings(this: usize, fixture: usize, record: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_NATION_AWARDS_RECORD_MATCH_RATINGS)(this, fixture, record)
}

/// Gives a year award from a year ratings table: the best players of `ratings` (`YEAR_RATINGS::collect_best_players`
/// with the seven filters: nation, club nation, excluded nation, home nation, division, mode, maximum age), as many as
/// the award's shortlist (`AWARD_SHORTLIST`) holds (its capacity) or 3 without one, into a block (`exe::crt_malloc`). Fewer found:
/// nothing. With a shortlist, it is cleared and each candidate is added (`AWARD_SHORTLIST::add_entry`: the person,
/// his club, the score of `ratings_shortlist_score`), then announced with `today`, the nation's id
/// (`nation_awards_fields::NATION_ID`) and the award's index (`award_base::INDEX`). Without one, the three winners (names, staff id, club id or −1) go with the year of `today` to
/// `YEAR_AWARD::record_winners`. The block is freed. Out of memory: the nation_awards.cpp 171 box; no ratings: the 178
/// box. Defects of the original: with no ratings the block is not freed; the record's score dwords are never written
/// (the history keeps stack garbage).
/// Original: `NationAwards::award_player_of_year_from_ratings` (0x7683c0, nation_awards.cpp, thiscall, ret 0x28).
pub(crate) unsafe extern "thiscall" fn nation_awards_award_player_of_year_from_ratings(this: usize, today: usize, award: usize, ratings: usize,
    nation: u32, club_nation: u32, excluded: u32, home: u32, division: u32, mode: u32, max_age: u32) {
    nation_awards_award_from_ratings_with(this, today, award, ratings, [nation, club_nation, excluded, home, division, mode, max_age], true,
        &mut |award, record| record_year_award(award, record));
}

/// Comparer for `nation_awards_award_player_of_year_from_ratings`. Errors (no ratings): only the original. With a
/// shortlist: the copy fills it as a dry run (no announcement), its contents are kept and the shortlist put back; the
/// original runs and must leave the same count, votes and entries (in any order: the announcement sorts them by
/// name). Without one: the copy runs as a dry run (no record), then the original; its newest year award history
/// record must be the planned one, without the score dwords the original never writes.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_award_player_of_year_from_ratings(this: usize, today: usize, award: usize, ratings: usize,
    nation: u32, club_nation: u32, excluded: u32, home: u32, division: u32, mode: u32, max_age: u32) {
    use award_shortlist_fields as asl;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32, u32, u32, u32, u32)>(IDX_NATION_AWARDS_AWARD_PLAYER_OF_YEAR_FROM_RATINGS);
    let original = || o(this, today, award, ratings, nation, club_nation, excluded, home, division, mode, max_age);
    let filters = [nation, club_nation, excluded, home, division, mode, max_age];
    if ratings == 0 {
        return original();
    }
    let shortlist = read_usize(award + AWARD_SHORTLIST);
    if shortlist != 0 {
        let regions = [(shortlist, 0x28), (read_usize(shortlist + asl::ENTRIES), read_i16(shortlist + asl::CAPACITY).max(0) as usize * asl::ENTRY_SIZE)];
        let before = crate::comparison::MemorySnapshot::read(&regions);
        crate::rng::pause_trace(|| nation_awards_award_from_ratings_with(this, today, award, ratings, filters, false, &mut |_, _| {}));
        let planned = shortlist_contents(shortlist);
        before.write_back();
        original();
        let got = shortlist_contents(shortlist);
        SCOREBOARD.record(got == planned, || format!("nation_awards_award_player_of_year_from_ratings(award {:#x}) shortlist original {:02x?} rust {:02x?}", award, got, planned));
        return;
    }
    let mut planned = None;
    crate::rng::pause_trace(|| nation_awards_award_from_ratings_with(this, today, award, ratings, filters, false, &mut |_, r| planned = Some(r.to_vec())));
    let list = read_usize(read_usize(G_AWARD_MANAGER) + award_manager_consts::YEAR_AWARD_FILE);
    let garbage: Vec<(usize, usize)> = (0..3).map(|k| (2 + k * YEAR_RECORD_ENTRY + YEAR_RECORD_SCORE, 4)).collect();
    compare_award_record("nation_awards_award_player_of_year_from_ratings", list, award + YEAR_AWARD_HISTORY_FIRST, award + YEAR_AWARD_HISTORY_LAST, planned, &garbage, original);
}

/// A year ratings entry's score for a shortlist, by the collecting `mode`: the mode's weighted rating sum (0, 1 or 2)
/// or, for any other mode, 2 × goals + assists, over the matches, as the x87 rounds it to an f32.
pub(crate) unsafe fn ratings_shortlist_score(entry: YearEntry, mode: u8) -> f32 {
    let total = match mode {
        0..=2 => entry.rating_sum(mode as usize),
        _ => entry.assists() + entry.goals() * 2,
    };
    (total as f64 / entry.matches() as i32 as f64) as f32
}

/// The player of the year from a year ratings table (see `nation_awards_award_player_of_year_from_ratings`), the
/// winners record going to `sink` and the shortlist announced only when `announce`.
pub(crate) unsafe fn nation_awards_award_from_ratings_with(this: usize, today: usize, award: usize, ratings: usize, filters: [u32; 7], announce: bool, sink: YearAwardSink) {
    let shortlist = read_usize(award + AWARD_SHORTLIST);
    // how many: the shortlist's capacity (the word over a dword whose high half is 0), or 3
    let wanted: u32 = if shortlist != 0 { read_u16(shortlist + award_shortlist_fields::CAPACITY) as u32 } else { 3 };
    let n = wanted as u16 as i16;
    let buf = exe::crt_malloc((n as i32 * 4) as u32);
    if buf == 0 {
        tel26_error_box(NATION_AWARDS_CPP, 0xab);
        return;
    }
    if ratings == 0 {
        // defect of the original: the block is not freed
        tel26_error_box(NATION_AWARDS_CPP, 0xb2);
        return;
    }
    covered!("sx04b_3 player of the year from ratings");
    let collect: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, u32, u32, u32) -> u16 = core::mem::transmute(FN_YEAR_RATINGS_COLLECT_BEST_PLAYERS);
    let [nation, club_nation, excluded, home, division, mode, max_age] = filters;
    let found = collect(ratings, buf, wanted, nation, club_nation, excluded, home, division, mode, max_age) as i16;
    if found < n { // sx04b_3: enough candidates
        exe::crt_free(buf);
        return;
    }
    let entry = |k: usize| YearEntry(read_usize(buf + 4 * k));
    if shortlist != 0 {
        covered!("sx04b_3 shortlist filled");
        let clear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_AWARD_SHORTLIST_CLEAR);
        clear(shortlist);
        let add: unsafe extern "thiscall" fn(usize, usize, usize, f32) -> u32 = core::mem::transmute(FN_AWARD_SHORTLIST_ADD_ENTRY);
        for k in 0..n.max(0) as usize {
            let person = entry(k).person();
            add(shortlist, person.addr(), person.club_job().addr(), ratings_shortlist_score(entry(k), mode as u8));
        }
        if announce {
            let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_AWARD_SHORTLIST_ANNOUNCE);
            f(shortlist, today, read_u32(this + nation_awards_fields::NATION_ID), read_u32(award + award_base::INDEX));
        }
    } else {
        covered!("sx04b_3 winners recorded");
        // without a shortlist n is 3: one entry per winner (the original would put any further one in the third)
        let people = [0, 1, 2].map(|k| entry(k).person());
        sink(award, &year_record_of(today, people));
    }
    exe::crt_free(buf);
}

/// A shortlist's count, votes and entries (sorted, since the announcement sorts them by name).
pub(crate) unsafe fn shortlist_contents(shortlist: usize) -> (Vec<u8>, Vec<Vec<u8>>) {
    use award_shortlist_fields as asl;
    let head = (asl::COUNT..0x28).map(|i| read_u8(shortlist + i)).collect();
    let mut entries: Vec<Vec<u8>> = (0..read_i16(shortlist + asl::COUNT).max(0) as usize)
        .map(|k| (0..asl::ENTRY_SIZE).map(|i| read_u8(read_usize(shortlist + asl::ENTRIES) + k * asl::ENTRY_SIZE + i)).collect())
        .collect();
    entries.sort();
    (head, entries)
}

/// Writes the three winners `people` into a year award record for the year of `today`: each one's name ids, staff id
/// and club id (−1 without a club); the scores are left as they are (the original never writes them).
pub(crate) unsafe fn year_record_of(today: usize, people: [Staff; 3]) -> [u8; YEAR_RECORD_SIZE] {
    let mut record = [0u8; YEAR_RECORD_SIZE];
    record[..2].copy_from_slice(&read_u16(today + cm_date::YEAR).to_le_bytes());
    for (k, person) in people.into_iter().enumerate() {
        let club = person.club_job();
        let club_id = if club.is_null() { u32::MAX } else { read_u32(club.addr()) };
        year_record_entry(&mut record, k, person, None, club_id);
    }
    record
}

/// `NationAwards`' base vtable.
pub(crate) const NATION_AWARDS_BASE_VTABLE: u32 = 0x96da10;

/// `MONTH_RATINGS::record_match_ratings(fixture, record)` (0x767ee0, month_ratings.cpp, thiscall, ret 8).
pub(crate) const FN_MONTH_RATINGS_RECORD_MATCH_RATINGS: usize = 0x767ee0;

/// `YEAR_RATINGS::collect_best_players(out, max, nation, club nation, excluded nation, home nation, division, mode,
/// max age)` (0x931ae0, year_ratings.cpp, thiscall, ret 0x24): how many were collected (ax).
pub(crate) const FN_YEAR_RATINGS_COLLECT_BEST_PLAYERS: usize = 0x931ae0;

/// `AWARD_SHORTLIST::clear()` (0x418320, thiscall), `add_entry(person, club, f32 score)` (0x4178a0, thiscall, ret
/// 0xc) and `announce(date, nation id, award index)` (0x417a20, thiscall, ret 0xc).
pub(crate) const FN_AWARD_SHORTLIST_CLEAR: usize = 0x418320;

pub(crate) const FN_AWARD_SHORTLIST_ADD_ENTRY: usize = 0x4178a0;

pub(crate) const FN_AWARD_SHORTLIST_ANNOUNCE: usize = 0x417a20;
