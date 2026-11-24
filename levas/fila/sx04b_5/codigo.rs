// ---------- sx04b_5: a tournament's player of the year and team of the tournament (nation_awards.cpp) ----------

/// Whether a competition is international (`exe::key_nation_index_of_comp` of its record gives 0 or 1): its awards
/// credit each player's national team instead of his club.
unsafe fn tournament_is_international(comp: League) -> bool {
    let kind = exe::key_nation_index_of_comp(comp.competition());
    kind == 1 || kind == 0
}

/// The fewest starts a tournament award candidate needs: the share of 3 games, cut by `_ftol` to 16 bits.
unsafe fn tournament_least_starts(share: f32) -> i32 {
    ftol(share as f64 * read_f32(rdata::F32_3) as f64) as i16 as i32
}

/// A tournament's player of the year (see `nation_awards_award_tournament_player`), the record going to `sink`.
pub(crate) unsafe fn nation_awards_award_tournament_player_with(comp_index: u32, today: usize, award: usize, age: u32, positions: u32, stat: u32,
    sink: YearAwardSink) {
    let comp = award_league(comp_index);
    if comp.is_null() {
        return;
    }
    let mut share: f32 = f32::from_bits(0x3f28f5c3); // 0.66 of three games
    let international = tournament_is_international(comp);
    let mut n = 0i32;
    let rows = exe::comp_stats_get_sorted_player_table(comp.comp_stats(), stat, &mut n, 0x64, u32::MAX);
    if n == 0 {
        return;
    }
    let mut chosen = [Staff(0); 3];
    let mut scores = [None; 3];
    let mut found = 0usize;
    let mut older = 0i32; // over the age limit: counted across the passes, never reset
    while share as f64 > 0.0 {
        let least = tournament_least_starts(share);
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
            if (read_u8(row + ROW_APPEARANCES) as i32 - read_u8(row + ROW_SUB_APPEARANCES) as i32) < least { // sx04b_5: enough starts
                continue;
            }
            if age as u8 != 0xff {
                if read_i8(person.addr() + staff::AGE) > age as u8 as i8 {
                    older += 1;
                    continue;
                }
                if older < 3 {
                    continue;
                }
            }
            covered!("sx04b_5 tournament player candidate");
            chosen[found] = person;
            if stat as u8 == 2 {
                scores[found] = Some(read_u8(row + ROW_STAT) as u32);
            }
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
    for (k, &person) in chosen.iter().enumerate() {
        let club_id = if international {
            let nation = person.nation();
            if nation.is_null() { u32::MAX } else { read_u32(exe::database_get_national_team(nation)) }
        } else {
            let club = person.club_job();
            if club.is_null() { u32::MAX } else { read_u32(club.addr()) }
        };
        year_record_entry(&mut record, k, person, scores[k], club_id);
    }
    sink(award, &record);
}

/// Gives a tournament's player award from its statistics: the first three of the competition's player table for the
/// award's statistic (`comp_stats_get_sorted_player_table`, at most 100) who are players, play one of the `positions`
/// (16 bits; 0 = any) and started at least the share of three games (0.66, down by 0.1 a pass while above 0, the
/// chosen kept). With an `age` (not 0xff), those older are counted and the younger only qualify after three were
/// counted. With all three: the record (the year of `today`, each winner's names, staff id, the statistic for award
/// statistic 2, and his national team's id in an international competition, else his club's, or −1) goes to
/// `YEAR_AWARD::record_winners`. No competition: nothing. Defect of the original: for the other statistics the score
/// dwords are never written (the history keeps stack garbage).
/// Original: `nation_awards_award_tournament_player` (0x7690c0, nation_awards.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn nation_awards_award_tournament_player(_this: usize, comp_index: u32, today: usize, award: usize, age: u32,
    positions: u32, stat: u32) {
    nation_awards_award_tournament_player_with(comp_index, today, award, age, positions, stat, &mut |award, record| record_year_award(award, record));
}

/// A tournament's team (see `nation_awards_award_tournament_team`), the final record going to `sink`.
pub(crate) unsafe fn nation_awards_award_tournament_team_with(comp_index: u32, award: usize, sink: TeamAwardSink) {
    let comp = award_league(comp_index);
    if comp.is_null() {
        return;
    }
    let mut share: f32 = f32::from_bits(0x3f28f5c3); // 0.66 of three games
    let international = tournament_is_international(comp);
    let mut n = 0i32;
    let rows = exe::comp_stats_get_sorted_player_table(comp.comp_stats(), 0x11, &mut n, u32::MAX, u32::MAX);
    if n == 0 {
        return;
    }
    // the substitutes the award names: those of its club competition, 3 without one
    let mut substitutes = 3u8;
    let table = read_usize(G_COMP_OBJECT_TABLE);
    let club_comp = read_usize(award + award_base::CLUB_COMP);
    if table != 0 && club_comp != 0 {
        let named = League(read_usize(table + read_u32(club_comp) as usize * 4));
        if !named.is_null() {
            substitutes = named.substitutes_named();
        }
    }
    write_u8(award + team_award_obj::SUBSTITUTES, substitutes);
    let clear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TEAM_AWARD_CLEAR_SELECTION);
    clear(award);
    let add: unsafe extern "thiscall" fn(usize, usize, usize, f32, u32) -> u32 = core::mem::transmute(FN_TEAM_AWARD_ADD_CANDIDATE);
    'passes: loop {
        let least = tournament_least_starts(share);
        for k in 0..n.max(0) as usize {
            let row = rows + k * comp_stats_obj::PLAYER_ENTRY_SIZE;
            let person = staff_at(read_i32(row + comp_stats_player::STAFF_ID) as isize);
            let appearances = read_u8(row + ROW_APPEARANCES) as i32;
            if appearances - read_u8(row + ROW_SUB_APPEARANCES) as i32 <= least { // sx04b_5: more starts than the share
                continue;
            }
            let side = if international { person.national_job() } else { person.club_job() };
            let rating = (read_i16(row + ROW_RATING_SUM) as i32 as f64 / appearances as f64) as f32;
            covered!("sx04b_5 tournament team candidate");
            if add(award, person.addr(), side.addr(), rating, 0) != 0 {
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

/// Picks a tournament's team: the award's substitutes (`team_award_obj::SUBSTITUTES`) become those its club
/// competition names (3 without one) and the selection is cleared; then, for shares of three games from 0.66 down by
/// 0.1 while above 0, each player of the competition's rating table (statistic 0x11) who started more games than that
/// share is offered to the award (`TEAM_AWARD::add_candidate`) with his national team in an international
/// competition, else his club, and his average rating (rating sum / appearances), until the award reports the team
/// full. The table is freed, the season dates set for the current year and the team recorded
/// (`TEAM_AWARD::record_winners`: history, reputations). No competition or no rated player: nothing.
/// Original: `NationAwards::award_tournament_team` (0x769af0, nation_awards.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn nation_awards_award_tournament_team(_this: usize, comp_index: u32, award: usize) {
    nation_awards_award_tournament_team_with(comp_index, award, &mut |award| record_team_award(award));
}

/// Comparer for `nation_awards_award_tournament_player`: the copy runs as a dry run (no record), then the original; its
/// newest year award history record must be the planned one (the scores only for statistic 2: the others are never
/// written).
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_award_tournament_player(this: usize, comp_index: u32, today: usize, award: usize, age: u32,
    positions: u32, stat: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, usize, u32, u32, u32)>(%%I0%%);
    let mut planned = None;
    crate::rng::pause_trace(|| nation_awards_award_tournament_player_with(comp_index, today, award, age, positions, stat, &mut |_, r| planned = Some(r.to_vec())));
    let unwritten: Vec<(usize, usize)> = if stat as u8 == 2 { Vec::new() } else { (0..3).map(|k| (2 + k * YEAR_RECORD_ENTRY + YEAR_RECORD_SCORE, 4)).collect() };
    let history = read_usize(read_usize(G_AWARD_MANAGER) + award_manager_consts::YEAR_AWARD_FILE);
    compare_award_record("nation_awards_award_tournament_player", history, award + YEAR_AWARD_HISTORY_FIRST, award + YEAR_AWARD_HISTORY_LAST, planned,
        &unwritten, || o(this, comp_index, today, award, age, positions, stat));
}

/// Comparer for `nation_awards_award_tournament_team`: the copy runs as a dry run on the award's selection (kept, then
/// put back), then the original; its newest team award history record must be the copy's selection.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_award_tournament_team(this: usize, comp_index: u32, award: usize) {
    use team_award_obj as ta;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize)>(%%I1%%);
    let selection = [(award + ta::RECORD, ta::SUBSTITUTES + 1 - ta::RECORD)];
    let before = crate::comparison::MemorySnapshot::read(&selection);
    let mut planned = None;
    crate::rng::pause_trace(|| nation_awards_award_tournament_team_with(comp_index, award,
        &mut |a| planned = Some((0..ta::RECORD_SIZE as usize).map(|i| read_u8(a + ta::RECORD + i)).collect::<Vec<u8>>())));
    before.write_back();
    let history = read_usize(read_usize(G_AWARD_MANAGER) + award_manager_consts::TEAM_AWARD_FILE);
    compare_award_record("nation_awards_award_tournament_team", history, award + ta::HISTORY_FIRST, award + ta::HISTORY_LAST, planned, &[],
        || o(this, comp_index, award));
}
