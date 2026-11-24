// ---------- sx04b_8: a nation's player of the year by the season statistics and the orders of its candidates (nation_awards.cpp) ----------

/// A candidate of the statistics award (10 bytes): the staff record (+0), the score (f32, +4) and the games (word, +8).
const STATS_CANDIDATE_SIZE: usize = 0xa;
const STATS_CANDIDATE_SCORE: usize = 0x4;
const STATS_CANDIDATE_GAMES: usize = 0x8;
/// The candidates' qsort orders (cdecl, at their original addresses): by score, lowest first (mode 3) or highest first.
const FN_STATS_SCORE_ASCENDING: usize = 0x76a480;
const FN_STATS_SCORE_DESCENDING: usize = 0x76a4b0;

/// A nation record from a filter's nation id (−1 or below: none, 0).
unsafe fn stats_filter_nation(id: i32) -> usize {
    if id > -1 { read_usize(G_NATION_TABLE) + id as usize * nation::SIZE } else { 0 }
}

/// A player's statistics score weight: (his club's reputation / 500 + its division's reputation × 4) / 5, both read as
/// signed words and each division by truncation, or 1 without a club or a division; then halved (truncated).
unsafe fn stats_reputation_weight(club: Club) -> i32 {
    let division = if club.is_null() { 0 } else { read_usize(club.addr() + club::DIVISION) };
    let weight = if division != 0 {
        (read_i16(club.addr() + club::REPUTATION) as i32 / 500 + read_i16(division + competition::REPUTATION) as i32 * 4) / 5
    } else {
        1
    };
    weight as i16 as i32 / 2
}

/// The player of the year by the season statistics (see `nation_awards_award_player_of_year_by_stats`), the record
/// going to `sink`.
pub(crate) unsafe fn nation_awards_award_by_stats_with(today: usize, award: usize, club_nation: i32, nation: i32, excluded: i32, home: i32, age: u32,
    positions: u32, mode: u32, sink: YearAwardSink) {
    let m = mode as u8;
    let mut share: f32 = if m == 5 || m == 2 { f32::from_bits(0x3c23d70a) } else { f32::from_bits(0x3f28f5c3) }; // 0.01 or 0.66
    let buf = exe::crt_malloc(read_i32(G_STAFF_COUNT).wrapping_mul(STATS_CANDIDATE_SIZE as i32) as u32);
    if buf == 0 {
        tel26_error_box(NATION_AWARDS_CPP, 0x4fc);
        return;
    }
    let (club_nation, nation, excluded, home) = (stats_filter_nation(club_nation), stats_filter_nation(nation), stats_filter_nation(excluded), stats_filter_nation(home));
    // the statistics category: 2 for mode 2, else 6 (the original pushes it in a dword whose high bytes are stack
    // leftover; get_stat reads the byte)
    let category: u32 = if m == 2 { 2 } else { 6 };
    let stats = read_usize(G_PLAYER_STATS);
    let mut count = 0usize;
    let mut i = 0i32;
    while i < read_i32(G_STAFF_COUNT) {
        let person = staff_at(i as isize);
        i += 1;
        let sheet = person.player();
        if sheet.is_null() {
            continue;
        }
        let club = person.club_job();
        let comps = read_usize(G_COMP_OBJECT_TABLE);
        if !club.is_null() && comps != 0 {
            let division = read_usize(club.addr() + club::DIVISION);
            if division == 0 || read_usize(comps + read_u32(division) as usize * 4) == 0 { // sx04b_8: a club in a running league
                continue;
            }
        }
        let own = read_usize(person.addr() + staff::NATION);
        let club_nation_of = || if club.is_null() { None } else { Some(read_usize(club.addr() + club::NATION)) };
        if club_nation != 0 && club_nation_of() != Some(club_nation) {
            continue;
        }
        if nation != 0 && own != nation {
            continue;
        }
        if excluded != 0 && own == excluded {
            continue;
        }
        if home != 0 && own != home && club_nation_of() != Some(home) {
            continue;
        }
        let c = buf + count * STATS_CANDIDATE_SIZE;
        write_usize(c, person.addr());
        let raw = exe::playerstats_get_stat(stats, sheet, category, mode, 6.5);
        write_f32(c + STATS_CANDIDATE_SCORE, raw as f32);
        if m == 2 && raw == 0.0 {
            continue;
        }
        let games = ftol(exe::playerstats_get_stat(stats, sheet, category, 1, 6.5)) as i16;
        write_i16(c + STATS_CANDIDATE_GAMES, games);
        if m == 3 {
            let per_game = if games > 0 { (read_f32(c + STATS_CANDIDATE_SCORE) as f64 / games as f64) as f32 } else { 0.0 };
            write_f32(c + STATS_CANDIDATE_SCORE, per_game);
        }
        if m != 2 {
            let weight = stats_reputation_weight(person.club_job());
            write_f32(c + STATS_CANDIDATE_SCORE, (weight as f64 * read_f32(c + STATS_CANDIDATE_SCORE) as f64) as f32); // sx04b_8: weighed by reputation
        }
        covered!("sx04b_8 statistics candidate");
        count += 1;
    }
    if count == 0 {
        // defect of the original: the block is not freed
        return;
    }
    exe::crt_qsort(buf, count as u32, STATS_CANDIDATE_SIZE as u32, if m == 3 { FN_STATS_SCORE_ASCENDING } else { FN_STATS_SCORE_DESCENDING });
    let mut chosen = [Staff(0); 3];
    let mut scores = [None; 3];
    let mut found = 0usize;
    let mut older = 0i32; // over the age limit: counted across the passes, never reset
    let positions = positions as u16;
    while share as f64 > 0.0 {
        let least = ftol(share as f64 * read_f32(rdata::F32_25) as f64);
        for k in 0..count {
            let c = buf + k * STATS_CANDIDATE_SIZE;
            let person = Staff(read_usize(c));
            if positions != 0 {
                let mask = exe::staff_get_natural_position_mask(person);
                // a centre back (4) who also plays full back (2) is not one
                if mask as u16 & positions == 0 || (positions == 4 && mask as u8 & 2 != 0) {
                    continue;
                }
            }
            if chosen[..found].contains(&person) {
                continue;
            }
            if (read_i16(c + STATS_CANDIDATE_GAMES) as i32) < least { // sx04b_8: enough games
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
            chosen[found] = person;
            if m == 2 {
                scores[found] = Some(ftol(read_f32(c + STATS_CANDIDATE_SCORE) as f64) as u32);
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
    exe::crt_free(buf);
    if found != 3 {
        return;
    }
    let mut record = [0u8; YEAR_RECORD_SIZE];
    record[..2].copy_from_slice(&read_u16(today + cm_date::YEAR).to_le_bytes());
    for (k, &person) in chosen.iter().enumerate() {
        let club = person.club_job();
        year_record_entry(&mut record, k, person, scores[k], if club.is_null() { u32::MAX } else { read_u32(club.addr()) });
    }
    sink(award, &record);
}

/// Gives a year award by the season statistics: every player (`player()`) whose club, if any, plays in a running
/// competition (`club::DIVISION` in `G_COMP_OBJECT_TABLE`) and who passes the nation filters (−1 = none:
/// `club_nation` his club's nation, `nation` his own, `excluded` not his own, `home` his own or his club's) gets a
/// score from `PlayerStats::get_stat` (category 2 for mode 2, else 6; the statistic `mode`; default 6.5) and his games
/// (the statistic 1, truncated); in mode 2 a score of 0 leaves him out; in mode 3 the score is per game (0 without
/// games); except in mode 2 it is multiplied by half the reputation weight ((club reputation / 500 + division
/// reputation × 4) / 5, or 1). They are sorted by score (lowest first in mode 3, else highest first); then, for shares
/// of 25 games from 0.66 (0.01 in modes 2 and 5) down by 0.1 while above 0, the first three who play one of the
/// `positions` (a centre back, 4, must not also play full back), have that many games and pass the `age` rule (those
/// older are counted; the younger only after three were counted) win. Their record (the year of `today`, names, staff
/// id, the truncated score in mode 2, the club id or −1) goes to `YEAR_AWARD::record_winners`. Out of memory: the
/// nation_awards.cpp 1276 box. Defects of the original: without any candidate the block is not freed; outside mode 2
/// the record's score dwords are never written (stack garbage in the history).
/// Original: `nation_awards_award_player_of_year_by_stats` (0x769380, nation_awards.cpp, thiscall, ret 0x24).
pub(crate) unsafe extern "thiscall" fn nation_awards_award_player_of_year_by_stats(_this: usize, today: usize, award: usize, club_nation: i32, nation: i32,
    excluded: i32, home: i32, age: u32, positions: u32, mode: u32) {
    nation_awards_award_by_stats_with(today, award, club_nation, nation, excluded, home, age, positions, mode, &mut |award, record| record_year_award(award, record));
}

/// Comparer for `nation_awards_award_player_of_year_by_stats`: the copy runs as a dry run (no record), then the
/// original; its newest year award history record must be the planned one (the scores only in mode 2).
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_award_player_of_year_by_stats(this: usize, today: usize, award: usize, club_nation: i32,
    nation: i32, excluded: i32, home: i32, age: u32, positions: u32, mode: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32, i32, i32, i32, u32, u32, u32)>(%%I0%%);
    let mut planned = None;
    crate::rng::pause_trace(|| nation_awards_award_by_stats_with(today, award, club_nation, nation, excluded, home, age, positions, mode,
        &mut |_, r| planned = Some(r.to_vec())));
    let unwritten: Vec<(usize, usize)> = if mode as u8 == 2 { Vec::new() } else { (0..3).map(|k| (2 + k * YEAR_RECORD_ENTRY + YEAR_RECORD_SCORE, 4)).collect() };
    let history = read_usize(read_usize(G_AWARD_MANAGER) + award_manager_consts::YEAR_AWARD_FILE);
    compare_award_record("nation_awards_award_player_of_year_by_stats", history, award + YEAR_AWARD_HISTORY_FIRST, award + YEAR_AWARD_HISTORY_LAST, planned,
        &unwritten, || o(this, today, award, club_nation, nation, excluded, home, age, positions, mode));
}

/// The x87 order of two f32 scores (`fcomp`): (a < b or unordered, a > b) as the original tests C0 and C0|C3.
fn stats_score_order(a: f32, b: f32) -> (bool, bool) {
    (a < b || a.is_nan() || b.is_nan(), a > b)
}

/// The qsort order of the statistics candidates by score, lowest first: −1 when `a`'s score is lower (or the two
/// cannot be ordered), 1 when higher, 0 when equal.
/// Original: `nation_awards_compare_score_ascending` (0x76a480, nation_awards.cpp, cdecl).
pub(crate) unsafe extern "C" fn nation_awards_compare_score_ascending(a: *const usize, b: *const usize) -> i32 {
    let (below, above) = stats_score_order(read_f32(a as usize + STATS_CANDIDATE_SCORE), read_f32(b as usize + STATS_CANDIDATE_SCORE));
    if below { // sx04b_8: the lower score first
        -1
    } else if above {
        1
    } else {
        0
    }
}

compare_order!(compare_nation_awards_compare_score_ascending, nation_awards_compare_score_ascending, %%I1%%);

/// The qsort order of the statistics candidates by score, highest first: −1 when `a`'s score is higher, 1 when lower
/// (or the two cannot be ordered), 0 when equal.
/// Original: `nation_awards_compare_score_descending` (0x76a4b0, nation_awards.cpp, cdecl).
pub(crate) unsafe extern "C" fn nation_awards_compare_score_descending(a: *const usize, b: *const usize) -> i32 {
    let (below, above) = stats_score_order(read_f32(a as usize + STATS_CANDIDATE_SCORE), read_f32(b as usize + STATS_CANDIDATE_SCORE));
    covered!("sx04b_8 scores ordered");
    if above { // sx04b_8: the higher score first
        -1
    } else if below {
        1
    } else {
        0
    }
}

compare_order!(compare_nation_awards_compare_score_descending, nation_awards_compare_score_descending, %%I2%%);
