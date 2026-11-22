// ---------- batch %%LEVA%%: the competition statistics: the carried runs, the match totals, the labels and the values (comp_screens.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\comp_stats.CPP" (the error box), the 35.0 of the qualification thresholds and
/// the qsort / bsearch comparator of the saved runs (by the first dword, `training_compare_entries_by_staff_id`).
const COMP_STATS_SOURCE_FILE: usize = 0x99ca94;
const COMP_STATS_F32_35: usize = 0x968a7c;
const COMP_STATS_BY_ID: usize = 0x8b4e00;
/// The Comp_Stats fields: the saved runs (array, capacity, count), the team entries (0x76 bytes each) and their count.
mod comp_stats_runs {
    pub(crate) const SAVED: usize = 0x0;
    pub(crate) const SAVED_CAPACITY: usize = 0x4;
    pub(crate) const SAVED_COUNT: usize = 0x8;
    pub(crate) const TEAMS: usize = 0xc;
    pub(crate) const TEAM_COUNT: usize = 0x14;
    pub(crate) const TEAM_SIZE: usize = 0x76;
    pub(crate) const SAVED_SIZE: usize = 0x1c;
}

/// The run fields of a team entry copied to a saved run (0x1c bytes: the club id, then twelve words).
const COMP_STATS_RUN_WORDS: [usize; 12] = [0x19, 0x1d, 0x1f, 0x2d, 0x31, 0x33, 0x41, 0x45, 0x47, 0x55, 0x59, 0x5b];

/// Saves the four runs of every team entry before the team table is freed: the old saved array is freed; with teams,
/// a new array of one 0x1c-byte record per team (`malloc`; failing, the error box of comp_stats.CPP line 0x5f7 and the
/// capacity cleared) takes each team's club id and the twelve words of its runs (`COMP_STATS_RUN_WORDS`: each run's
/// length and its start date), then is sorted by club id (`qsort`).
/// Original: `Comp_Stats::save_team_runs` (0x4a2090, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_stats_save_team_runs(this: usize) {
    use comp_stats_runs as r;
    if read_usize(this + r::SAVED) != 0 {
        exe::crt_free(read_usize(this + r::SAVED));
        write_usize(this + r::SAVED, 0);
        write_u32(this + r::SAVED_CAPACITY, 0);
        write_u32(this + r::SAVED_COUNT, 0);
    }
    let teams = read_i32(this + r::TEAM_COUNT);
    if teams == 0 {
        return;
    }
    write_i32(this + r::SAVED_CAPACITY, teams);
    write_u32(this + r::SAVED_COUNT, 0);
    let saved = exe::crt_malloc((teams as u32).wrapping_mul(r::SAVED_SIZE as u32));
    write_usize(this + r::SAVED, saved);
    if saved == 0 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0x5f7);
        write_u32(this + r::SAVED_CAPACITY, 0);
        return;
    }
    covered!("tel30_11 team runs saved");
    let mut k = 0;
    while k < read_i32(this + r::TEAM_COUNT) {
        let team = read_usize(this + r::TEAMS) + k as usize * r::TEAM_SIZE;
        let rec = read_usize(this + r::SAVED) + read_u32(this + r::SAVED_COUNT) as usize * r::SAVED_SIZE;
        write_u32(rec, read_u32(team));
        for (i, &at) in COMP_STATS_RUN_WORDS.iter().enumerate() {
            write_u16(rec + 4 + i * 2, read_u16(team + at));
        }
        write_u32(this + r::SAVED_COUNT, read_u32(this + r::SAVED_COUNT) + 1);
        k += 1;
    }
    write_u32(this + r::SAVED_CAPACITY, read_u32(this + r::SAVED_COUNT));
    exe::crt_qsort(read_usize(this + r::SAVED), read_u32(this + r::SAVED_COUNT), r::SAVED_SIZE as u32, COMP_STATS_BY_ID);
}

/// Gives a new team entry back the runs saved for its club: the saved run of its club id (`bsearch`), when there is
/// one, sets each run's length (+0x19, +0x2d, +0x41, +0x55: the saved word, signed) and start date (+0x1d, +0x31,
/// +0x45, +0x59: `Date::Date(day, year)`) and puts the current date at +0x25, +0x39, +0x4d and +0x61; the saved count
/// goes down and the array is freed when it reaches 0.
/// Original: `Comp_Stats::restore_team_runs` (0x4a22f0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_restore_team_runs(this: usize, team: usize) {
    use comp_stats_runs as r;
    let saved = read_usize(this + r::SAVED);
    if saved == 0 {
        return;
    }
    let key = read_u32(team);
    let rec = exe::crt_bsearch(&key as *const u32 as usize, saved, read_u32(this + r::SAVED_COUNT), r::SAVED_SIZE as u32, COMP_STATS_BY_ID);
    if rec == 0 {
        return;
    }
    covered!("tel30_11 team runs restored");
    let (mut date, mut scratch) = ([0u32; 2], [0u32; 2]);
    for (k, at) in [0x19usize, 0x2d, 0x41, 0x55].into_iter().enumerate() {
        write_i32(team + at, read_i16(rec + 4 + k * 6) as i32);
        let made = exe::new_date(date.as_mut_ptr() as usize, read_u16(rec + 6 + k * 6) as u32, read_u16(rec + 8 + k * 6) as u32);
        exe::date_operator_3d(team + at + 4, scratch.as_mut_ptr() as usize, made);
        exe::date_operator_3d(team + at + 0xc, scratch.as_mut_ptr() as usize, G_CURRENT_DATE);
    }
    let left = read_i32(this + r::SAVED_COUNT) - 1;
    write_i32(this + r::SAVED_COUNT, left);
    if left == 0 {
        exe::crt_free(read_usize(this + r::SAVED));
        write_usize(this + r::SAVED, 0);
        write_u32(this + r::SAVED_CAPACITY, 0);
    }
}

/// Adds a played match's player figures to the competition's totals; returns 1, or 0 for a match already counted
/// (byte +0x4c) and for a null match (with the error box of comp_stats.CPP line 0x707).
///
/// The match count (+0x5ca) goes up. For each side (two 0x625-byte blocks) and each of its 20 player rows (0x49 bytes
/// from +0x190 of the match) with both ids set (dwords +0 and +4 not -1) and, from the 12th row on, the byte +0x32 not
/// -1: the unsigned bytes +0x3e..+0x43 go to the totals +0x56a..+0x57e, +0x25 / +0x26 / +0x21 to +0x582 / +0x586 /
/// +0x58a and to the side's own (+0x58e.. for the first side, +0x59a.. for the second), +0x27 / +0x28 / +0x47 / +0x2b /
/// +0x29 / +0x2a / +0x35 / +0x2c / +0x2d to +0x5a6..+0x5c6.
/// Original: `Comp_Stats::accumulate_match_player_totals` (0x4a25f0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_accumulate_match_player_totals(this: usize, fixture: usize) -> u32 {
    if fixture == 0 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0x707);
        return 0;
    }
    if read_u8(fixture + 0x4c) != 0 {
        return 0;
    }
    let add = |at: usize, v: u8| write_u32(this + at, read_u32(this + at).wrapping_add(v as u32));
    write_u32(this + 0x5ca, read_u32(this + 0x5ca).wrapping_add(1));
    for side in 0..2usize {
        for k in 0..0x14usize {
            let row = fixture + 0x190 + side * 0x625 + k * 0x49;
            if read_i32(row) == -1 || read_i32(row + 4) == -1 || (k >= 0xb && read_i8(row + 0x32) == -1) {
                continue;
            }
            for (i, at) in (0x3e..=0x43).enumerate() {
                add(0x56a + i * 4, read_u8(row + at));
            }
            add(0x582, read_u8(row + 0x25));
            add(0x586, read_u8(row + 0x26));
            add(0x58a, read_u8(row + 0x21));
            let own = if side == 0 { 0x58e } else { 0x59a };
            add(own, read_u8(row + 0x25));
            add(own + 4, read_u8(row + 0x26));
            add(own + 8, read_u8(row + 0x21));
            for (i, at) in [0x27usize, 0x28, 0x47, 0x2b, 0x29, 0x2a, 0x35, 0x2c, 0x2d].into_iter().enumerate() {
                add(0x5a6 + i * 4, read_u8(row + at));
            }
        }
    }
    1
}

/// The names of the team statistics 1..=17 and of the player statistics 1..=17.
const COMP_STATS_TEAM_LABELS: [usize; 17] = [0x99cbc0, 0x98b988, 0x98e9f8, 0x99cbb4, 0x99cba8, 0x99cb94, 0x99cb80, 0x99cb68, 0x99cb50,
    0x98e9cc, 0x99cb44, 0x99cb38, 0x99cb24, 0x99cb14, 0x99cb04, 0x99caf4, 0x99cae8];
const COMP_STATS_PLAYER_LABELS: [usize; 17] = [0x99cc40, 0x98e9cc, 0x99cb44, 0x99cb38, 0x99c06c, 0x99cc34, 0x99cc24, 0x990eb8, 0x99cc18,
    0x99caf4, 0x99cae8, 0x99c04c, 0x99cbf8, 0x99cbe8, 0x99cbd8, 0x99cbc8, 0x99cc08];

/// A label routine's body: a null buffer or one shorter than `min` gives the error box at `short_line` and 0; the
/// buffer is emptied, a known code writes its name (langlib) and returns 1, an unknown one gives the box at
/// `bad_line` and 0.
unsafe fn comp_stats_label(code: i8, out: usize, size: i32, min: i32, labels: &[usize], short_line: u32, bad_line: u32) -> u32 {
    if out == 0 || size < min {
        tel26_error_box(COMP_STATS_SOURCE_FILE, short_line);
        return 0;
    }
    write_u8(out, 0);
    let k = (code as i32).wrapping_sub(1) as u32 as usize;
    if k >= labels.len() {
        tel26_error_box(COMP_STATS_SOURCE_FILE, bad_line);
        return 0;
    }
    let langlib: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    langlib(out, labels[k]);
    1
}

/// The name of team statistic `code` (1 "Games" .. 17 "Red Cards", `COMP_STATS_TEAM_LABELS`) into `out` (at least
/// 0x33 bytes; else the box of line 0x7b5); an unknown code gives the box of line 0x838 and 0.
/// Original: `comp_stats_get_team_stat_label` (0x4a2810, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_stats_get_team_stat_label(code: i8, out: usize, size: i32) -> u32 {
    comp_stats_label(code, out, size, 0x33, &COMP_STATS_TEAM_LABELS, 0x7b5, 0x838)
}

/// The name of player statistic `code` (1 "Appearances" .. 17 "Average Rating", `COMP_STATS_PLAYER_LABELS`) into
/// `out` (at least 0x33 bytes; else the box of line 0x84b); an unknown code gives the box of line 0x8ce and 0.
/// Original: `comp_stats_get_player_stat_label` (0x4a2b10, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_stats_get_player_stat_label(code: i8, out: usize, size: i32) -> u32 {
    comp_stats_label(code, out, size, 0x33, &COMP_STATS_PLAYER_LABELS, 0x84b, 0x8ce)
}

/// The value of team statistic `code` of a team entry, as the x87 float the original leaves in st0.
///
/// 1 the games (byte +4); 2 the average attendance (+6 / byte +5, 0.0 with no game); 3 / 4 / 5 the sums of the five
/// signed form bytes from +0xa / +0xf / +0x14; 6..9 the runs (+0x19, +0x2d, +0x41, +0x55); 10..12 the bytes +0x69..+0x6b;
/// 13..19 the bytes +0x6d..+0x73. A null entry (line 0x901) or an unknown code (line 0x9a1) gives the error box and 0.0.
/// Original: `comp_stats_get_team_entry_value` (0x4a2ec0, comp_screens.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn comp_stats_get_team_entry_value(entry: usize, code: i8) -> f64 {
    let zero = read_f32(rdata::F32_0) as f64;
    if entry == 0 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0x901);
        return zero;
    }
    let form = |at: usize| (0..5).map(|k| read_i8(entry + at + k) as i32).sum::<i32>() as f64;
    match code {
        1 => read_u8(entry + 4) as f64,
        2 => {
            let games = read_u8(entry + 5);
            if games == 0 { zero } else { read_i32(entry + 6) as f64 / games as f64 }
        }
        3 => form(0xa),
        4 => form(0xf),
        5 => form(0x14),
        6 => read_i32(entry + 0x19) as f64,
        7 => read_i32(entry + 0x2d) as f64,
        8 => read_i32(entry + 0x41) as f64,
        9 => read_i32(entry + 0x55) as f64,
        10..=12 => read_u8(entry + 0x69 + (code as usize - 10)) as f64,
        13..=19 => read_u8(entry + 0x6d + (code as usize - 13)) as f64,
        _ => {
            tel26_error_box(COMP_STATS_SOURCE_FILE, 0x9a1);
            zero
        }
    }
}

/// The games of the player entry's club in this competition (`find_club_entry`, then team statistic 1), 0.0 without
/// an entry; a null club (the club table + id × 0x245 at 0) gives the error box of line 0x8e2.
unsafe fn comp_stats_club_games(this: usize, entry: usize) -> f64 {
    let club = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(entry + 8).wrapping_mul(club::SIZE));
    if club == 0 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0x8e2);
        return read_f32(rdata::F32_0) as f64;
    }
    let team = exe::comp_stats_find_club_entry(this, club);
    if team == 0 {
        return read_f32(rdata::F32_0) as f64;
    }
    comp_stats_get_team_entry_value(team, 1)
}

/// The value of player statistic `code` of a player entry, as the x87 float the original leaves in st0.
///
/// 1..12 the bytes +0xc, +0xe, +0xf, +0x11..+0x19. The rates need the player's minutes (+0x29) to reach 35 × the
/// club's games (`comp_stats_club_games`): 13 pass completion (+0x1f / +0x1d × 100, with more than 20 passes and, up
/// to 500, more than a quarter of the minutes); 14 / 15 tackles / dribbles per game (+0x21 / +0x23 over the
/// appearances +0xc, with more than 0x95 minutes); 16 shots on target (+0x27 / +0x25 × 100, with more than 24 shots,
/// or more than 4 at under 46 minutes each, or more than 10 with more than 0x21c minutes); 17 the average rating
/// (+0x1a / the appearances, with appearances (statistic 1) of at least half the club's games, the games kept as a
/// float, and at least one); 18 +0xf / +0xc (with appearances). A rate short of its condition is 0.0; a null entry
/// (line 0x9d2) or an unknown code (line 0xa7f) gives the error box and 0.0.
/// Original: `Comp_Stats::get_player_entry_value` (0x4a3250, comp_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_player_entry_value(this: usize, entry: usize, code: i8) -> f64 {
    let zero = read_f32(rdata::F32_0) as f64;
    if entry == 0 {
        tel26_error_box(COMP_STATS_SOURCE_FILE, 0x9d2);
        return zero;
    }
    let k35 = read_f32(COMP_STATS_F32_35) as f64;
    let hundred = read_f32(rdata::F32_100) as f64;
    let apps = read_u8(entry + 0xc);
    match code {
        1 => apps as f64,
        2 => read_u8(entry + 0xe) as f64,
        3 => read_u8(entry + 0xf) as f64,
        4..=12 => read_u8(entry + 0x11 + (code as usize - 4)) as f64,
        13 => {
            let minutes = read_i32(entry + 0x29);
            if comp_stats_club_games(this, entry) * k35 > minutes as f64 {
                return zero;
            }
            let passes = read_i16(entry + 0x1d);
            if passes <= 0x14 || (passes <= 0x1f4 && passes as i32 <= minutes / 4) {
                return zero;
            }
            read_i16(entry + 0x1f) as f64 / passes as f64 * hundred
        }
        14 | 15 => {
            let minutes = read_i32(entry + 0x29);
            if comp_stats_club_games(this, entry) * k35 > minutes as f64 || minutes <= 0x95 {
                return zero;
            }
            read_i16(entry + if code == 14 { 0x21 } else { 0x23 }) as f64 / apps as f64
        }
        16 => {
            let minutes = read_i32(entry + 0x29);
            if comp_stats_club_games(this, entry) * k35 > minutes as f64 {
                return zero;
            }
            let shots = read_i16(entry + 0x25);
            let enough = shots > 0x18 || (shots > 4 && minutes / shots as i32 <= 0x2d) || (minutes > 0x21c && shots > 10);
            if !enough {
                return zero;
            }
            read_i16(entry + 0x27) as f64 / shots as f64 * hundred
        }
        17 => {
            let games = comp_stats_club_games(this, entry) as f32 as f64;
            let played = comp_stats_get_player_entry_value(this, entry, 1);
            if games * read_f64(rdata::F64_0_5) > played || apps == 0 {
                return zero;
            }
            read_i16(entry + 0x1a) as f64 / apps as f64
        }
        18 => {
            if apps == 0 {
                return zero;
            }
            read_u8(entry + 0xf) as f64 / apps as f64
        }
        _ => {
            tel26_error_box(COMP_STATS_SOURCE_FILE, 0xa7f);
            zero
        }
    }
}

/// The name of record period `code` (1 "All Time", 2 "Current") into `out` (at least 0x32 bytes; else the box of line
/// 0xdf0); another code gives the box of line 0xe0a and 0.
/// Original: `comp_stats_get_record_period_label` (0x4a5010, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_stats_get_record_period_label(code: i8, out: usize, size: i32) -> u32 {
    comp_stats_label(code, out, size, 0x32, &[0x99ccd0, 0x99ccdc], 0xdf0, 0xe0a)
}

/// Whether record type `kind` is shown for the competition (+0x502, its first entry at +4): never for a friendly
/// competition (`Friendly::is_friendly_competition`); type 2 only for a league division (`comp_util_is_league_division`
/// of the first entry's id); type 8 not for a competition of kind 4 (+0x42); the rest always.
/// Original: `Comp_Stats::is_record_type_shown` (0x4a54c0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_is_record_type_shown(this: usize, kind: i8) -> u32 {
    let comp = read_usize(this + 0x502);
    if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), read_usize(comp + 4)) != 0 {
        return 0;
    }
    match kind {
        2 => (exe::comp_util_is_league_division(read_i32(read_usize(read_usize(this + 0x502) + 4))) != 0) as u32,
        8 if read_u8(read_usize(this + 0x502) + 0x42) == 4 => 0,
        _ => 1,
    }
}

/// Comparer for `comp_stats_save_team_runs`: the original only (it allocates the array), then the plan: one saved run
/// per team, sorted by club id.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_save_team_runs(this: usize) {
    use comp_stats_runs as r;
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let (saved, count) = (read_usize(this + r::SAVED), read_i32(this + r::SAVED_COUNT));
    if saved == 0 {
        return;
    }
    let ids: Vec<i32> = (0..count.max(0) as usize).map(|k| read_i32(saved + k * r::SAVED_SIZE)).collect();
    let ok = count == read_i32(this + r::TEAM_COUNT) && ids.windows(2).all(|w| w[0] <= w[1]);
    SCOREBOARD.record(ok, || format!("Comp_Stats::save_team_runs({:#x}): {} saved for {} teams, ids {:?}", this, count, read_i32(this + r::TEAM_COUNT), ids));
}

/// Comparer for `comp_stats_restore_team_runs`: the original only (it may free the array), then the plan: the four run
/// lengths taken from the saved run of the club.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_restore_team_runs(this: usize, team: usize) {
    use comp_stats_runs as r;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    let saved = read_usize(this + r::SAVED);
    let key = read_u32(team);
    let rec = if saved == 0 { 0 } else {
        exe::crt_bsearch(&key as *const u32 as usize, saved, read_u32(this + r::SAVED_COUNT), r::SAVED_SIZE as u32, COMP_STATS_BY_ID)
    };
    let want: Option<[i32; 4]> = if rec == 0 { None } else { Some([0usize, 1, 2, 3].map(|k| read_i16(rec + 4 + k * 6) as i32)) };
    o(this, team);
    if let Some(want) = want {
        let got = [0x19usize, 0x2d, 0x41, 0x55].map(|at| read_i32(team + at));
        SCOREBOARD.record(got == want, || format!("Comp_Stats::restore_team_runs: runs original={:?} rust={:?}", got, want));
    }
}

/// Comparer for `comp_stats_accumulate_match_player_totals`: side by side over the totals (+0x56a..+0x5ce) and the
/// answer; a null match (the error box) the original only.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_accumulate_match_player_totals(this: usize, fixture: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    if fixture == 0 {
        return o(this, fixture);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this + 0x56a, 0x64)], || o(this, fixture),
        || comp_stats_accumulate_match_player_totals(this, fixture));
    SCOREBOARD.record(same && r == n, || format!("Comp_Stats::accumulate_match_player_totals: original={} rust={} differences {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// A label routine side by side over its buffer and its answer; the error cases (the box) the original only.
unsafe fn comp_stats_compare_label(name: &str, code: i8, out: usize, size: i32, min: i32, known: i8,
    original: unsafe extern "C" fn(i8, usize, i32) -> u32, copy: unsafe extern "C" fn(i8, usize, i32) -> u32) -> u32 {
    if out == 0 || size < min || code < 1 || code > known {
        return original(code, out, size);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(out, min as usize)], || original(code, out, size), || copy(code, out, size));
    SCOREBOARD.record(same && r == n, || format!("{}({}): differences {:?}", name, code, first_differences(&mo, &mr)));
    r
}

/// Comparer for `comp_stats_get_team_stat_label`.
pub(crate) unsafe extern "C" fn compare_comp_stats_get_team_stat_label(code: i8, out: usize, size: i32) -> u32 {
    comp_stats_compare_label("comp_stats_get_team_stat_label", code, out, size, 0x33, 17, original_fn(%%I3%%), comp_stats_get_team_stat_label)
}

/// Comparer for `comp_stats_get_player_stat_label`.
pub(crate) unsafe extern "C" fn compare_comp_stats_get_player_stat_label(code: i8, out: usize, size: i32) -> u32 {
    comp_stats_compare_label("comp_stats_get_player_stat_label", code, out, size, 0x33, 17, original_fn(%%I4%%), comp_stats_get_player_stat_label)
}

/// Comparer for `comp_stats_get_team_entry_value`: the value side by side (its bits); the error cases the original only.
pub(crate) unsafe extern "stdcall" fn compare_comp_stats_get_team_entry_value(entry: usize, code: i8) -> f64 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, i8) -> f64>(%%I5%%);
    let r = o(entry, code);
    if entry != 0 && (1..=19).contains(&code) {
        let n = comp_stats_get_team_entry_value(entry, code);
        SCOREBOARD.record(r.to_bits() == n.to_bits(), || format!("comp_stats_get_team_entry_value({}): original={} rust={}", code, r, n));
    }
    r
}

/// Comparer for `comp_stats_get_player_entry_value`: the value side by side (its bits); the error cases the original only.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_player_entry_value(this: usize, entry: usize, code: i8) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8) -> f64>(%%I6%%);
    let r = o(this, entry, code);
    if entry != 0 && (1..=18).contains(&code) {
        let n = comp_stats_get_player_entry_value(this, entry, code);
        SCOREBOARD.record(r.to_bits() == n.to_bits(), || format!("Comp_Stats::get_player_entry_value({}): original={} rust={}", code, r, n));
    }
    r
}

/// Comparer for `comp_stats_get_record_period_label`.
pub(crate) unsafe extern "C" fn compare_comp_stats_get_record_period_label(code: i8, out: usize, size: i32) -> u32 {
    comp_stats_compare_label("comp_stats_get_record_period_label", code, out, size, 0x32, 2, original_fn(%%I7%%), comp_stats_get_record_period_label)
}

compare_value!(compare_comp_stats_is_record_type_shown, comp_stats_is_record_type_shown, %%I8%%, (kind: i8) -> u32);
