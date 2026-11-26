use super::*;

/// Does club competition `id` count (for deciding a group/league early)? Returns 1 for three fixed lower divisions
/// (Spanish lower division, French CFA, French lower division); 0 if the competition record (107 bytes, +0x5d) is not
/// flagged; otherwise 1 only if its stage object is at stage 2 (+0x43), has rounds (+0x3e) and the first table row's
/// club (+0x57 or +0x60) is the stage's competition (+4).
pub(crate) unsafe extern "C" fn competition_counts(id: i32) -> u32 {
    let g = |a: usize| read_i32(a);
    if id == g(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) || id == g(KEY_CLUB_COMP_FRENCH_CFA) || id == g(KEY_CLUB_COMP_FRENCH_LOWER_DIVISION) {
        return 1;
    }
    if read_u32(read_usize(G_CLUB_COMP_TABLE).wrapping_add((id as usize).wrapping_mul(107)).wrapping_add(0x5d)) == 0 {
        return 0;
    }
    let c = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((id as usize).wrapping_mul(4))));
    if c.is_null() || (c.comp_type() as i8) != 2 || (c.club_count() as u16) == 0 {
        return 0;
    }
    let r = c.table();
    if r == 0 {
        return 0;
    }
    let first = Club(read_usize(r));
    let target = c.competition().addr() as i32;
    ((first.division().addr() as i32) == target || (first.reserve_division().addr() as i32) == target) as u32
}

/// qsort comparator: orders records by the signed byte at +0x5f, descending (second minus first; a null pointer counts
/// as 0).
pub(crate) unsafe extern "C" fn order_by_5f(a: *const usize, b: *const usize) -> i32 {
    let v = |p: usize| if p == 0 { 0 } else { read_i8(p + club::LAST_POSITION) as i32 };
    v(*b) - v(*a)
}

/// qsort comparator: sorts records in descending order of their signed 16-bit word at +0x80 (b minus a).
pub(crate) unsafe extern "C" fn order_by_80(a: *const usize, b: *const usize) -> i32 {
    read_i16(*b + club::REPUTATION) as i32 - read_i16(*a + club::REPUTATION) as i32
}

/// qsort comparator for club pointers: sorts by the club's word at +0x80 (reputation) in descending order, with null
/// entries placed last.
pub(crate) unsafe extern "C" fn order_by_80_desc(a: *const usize, b: *const usize) -> i32 {
    let (x, y) = (*a, *b);
    if x == 0 {
        return (y != 0) as i32;
    }
    if y == 0 {
        return -1;
    }
    read_i16(y + club::REPUTATION) as i32 - read_i16(x + club::REPUTATION) as i32
}

/// qsort comparator: orders records by their 16-bit word at +0x80 (the club reputation) in ascending order, null
/// entries last.
pub(crate) unsafe extern "C" fn order_by_80_asc(a: *const usize, b: *const usize) -> i32 {
    let (x, y) = (*a, *b);
    if x == 0 {
        return (y != 0) as i32;
    }
    if y == 0 {
        return -1;
    }
    read_i16(x + club::REPUTATION) as i32 - read_i16(y + club::REPUTATION) as i32
}

/// Returns the units digit of round code `n` (n % 10) when `n` is below 1000, else 0; used to read the round variant
/// from a fixture's round code.
pub(crate) unsafe extern "C" fn units_digit(n: i32) -> u8 {
    if n < 1000 {
        (n % 10) as u8
    } else {
        0
    }
}

/// Tells whether competition `comp` is a friendly: it is the one stored at +4 of the friendly manager `this`, or its id
/// (+0) is among the last 127 club competitions (count at 0xae2378). Returns 1/0.
pub(crate) unsafe extern "thiscall" fn is_friendly(this: usize, comp: usize) -> u32 {
    if comp == read_usize(this + 4) {
        return 1;
    }
    (read_i32(comp) >= read_i32(G_CLUB_COMP_COUNT).wrapping_sub(0x7f)) as u32
}

/// Returns the person's contract inside a contract list: `this` points to an array of 80-byte contract records, and the
/// index comes from the person's 79-byte staff-state record (table 0xae2d0c, indexed by person id). Returns 0 when
/// there is no person, the id is out of range (person count 0xae2364) or the index is negative.
pub(crate) unsafe extern "thiscall" fn contract_of(this: usize, person: usize) -> usize {
    if person == 0 {
        return 0;
    }
    let id = *(person as *const i32);
    if id >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    let table = *(G_STAFF_STATE_TABLE as *const usize);
    let i = *((table as isize + 79 * id as isize) as *const i32);
    if i < 0 {
        return 0;
    }
    (*(this as *const usize)).wrapping_add((80 * i) as usize)
}

/// Returns 1 if the club is a national team: national-team ids come after the clubs, so the id must be at least the
/// club count minus twice the nation count. A null club returns 0 and clears the global at 0xb67a34 (the original also
/// shows an error box, skipped here).
pub(crate) unsafe extern "C" fn is_national_team(club: usize) -> u32 {
    if club == 0 {
        write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
        return 0;
    }
    let limit = read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(G_NATION_COUNT).wrapping_mul(2));
    (read_i32(club) >= limit) as u32
}

/// Computes each club's strength key (`comp_util_get_club_strength_score`) and sorts clubs and keys descending by swapping, as `comp_util_sort_clubs_by_strength`.
pub(crate) unsafe fn sort_clubs_by_strength(v: *mut u32, k: *mut i32, n: i32) {
    for i in 0..n.max(0) as usize {
        let c = *v.add(i);
        *k.add(i) = if c != 0 { exe::comp_util_get_club_strength_score(c as usize) } else { 0 };
    }
    let mut i = 0usize;
    while (i as i32) < n - 1 {
        for j in i + 1..n as usize {
            if *k.add(i) < *k.add(j) { // cpt13: the stronger one moves up
                core::ptr::swap(k.add(i), k.add(j));
                core::ptr::swap(v.add(i), v.add(j));
            }
        }
        i += 1;
    }
}

/// Sorts an array of clubs by strength (`comp_util_get_club_strength_score`), strongest first.
///
/// `comp_util_sort_clubs_by_strength(clubs, n)`: sorts the array of `n` clubs by strength (`comp_util_get_club_strength_score`), highest first,
/// keeping the keys in a temporary array (`exe::crt_malloc` / `exe::crt_free`; the original fills it with 0x60 beforehand, all of which
/// is overwritten). Out of memory: the error box (line 0x292d) and nothing else.
/// Original: `comp_util_sort_clubs_by_strength` (0x4c08f0, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_sort_clubs_by_strength(v: usize, n: i32) {
    let k = exe::crt_malloc((n as u32).wrapping_mul(4));
    if k == 0 {
        return show_error_at(0x99ec20, 0x292d);
    }
    sort_clubs_by_strength(v as *mut u32, k as *mut i32, n);
    if n > 1 {
        covered!("cpt_13 clubs sorted");
    }
    exe::crt_free(k);
}

/// Finds or adds the club's record in the competition statistics and stores the position at
/// `comp_stats_club::FINAL_POSITION`.
///
/// `Comp_Stats::set_club_final_position(club, pos)`: finds (`exe::comp_stats_find_club_entry`) or adds
/// (`exe::comp_stats_add_club_entry`) the club's record in the competition statistics and stores the position (a byte)
/// at `comp_stats_club::FINAL_POSITION`; returns the record. Null club: the error box (line 0x680).
/// Original: `Comp_Stats::set_club_final_position` (0x4a2540, comp_stats.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_club_final_position(this: usize, club: usize, pos: u32) -> usize {
    if club == 0 {
        show_error_at(0x99ca94, 0x680);
        return 0; // simplification: the original returns the box’s eax; the caller does not read it
    }
    let mut e = exe::comp_stats_find_club_entry(this, club);
    if e == 0 {
        e = exe::comp_stats_add_club_entry(this, club);
    }
    covered!("cpt_13 position recorded");
    core::ptr::write_volatile((e + comp_stats_club::FINAL_POSITION) as *mut u8, pos as u8); // cpt13: the position
    e
}

/// The calls `cup_rank_teams` makes to `set_club_final_position` (Comp_Stats set_club_final_position: club, position), in order, with the vector
/// `v` already sorted.
///
/// From the second-to-last round (+0x3c − 2) backward ([+0xa3] + r × 0x68, the count of who goes out in +0x1a), until a
/// null club at the start of the round. In the first round seen, those who go out in it all get position 1 (without
/// checking for nulls); in each round, the following ones get the counter's position (which goes up with every slot,
/// null or not), skipping the nulls.
///
/// `gravar` = 0 only builds the list. Returns the (club, position) list.
pub(crate) unsafe fn set_final_positions_by_round(this: Cup, v: *const u32, write: usize) -> Vec<(usize, u8)> {
    let mut chamadas: Vec<(usize, u8)> = Vec::new();
    let mut s: i16 = 0; // the index into the vector (bp signed)
    let mut b: u8 = 1; // the position counter
    let mut a: u8 = 1; // the pushed position (only the low byte counts; the original pushes the dword with leftovers on top)
    let mut r = this.round_count().wrapping_sub(2) as i32;
    while r >= 0 {
        if *v.offset(s as isize) == 0 {
            break;
        }
        let count = |r: i32| read_i16(this.round_table() + r as usize * cup_round::SIZE + cup_round::TIES) as i32;
        if r == this.round_count() as i32 - 2 {
            let mut j = 0;
            while j < count(r) {
                let c = *v.offset(s as isize) as usize;
                s = s.wrapping_add(1);
                chamadas.push((c, a));
                if write != 0 {
                    exe::comp_stats_set_club_final_position(read_usize(this.addr() + 8), c, a as u32);
                }
                b = b.wrapping_add(1);
                j += 1;
            }
        }
        a = b; // cpt13: the round’s position
        let mut j = 0;
        while j < count(r) {
            let c = *v.offset(s as isize) as usize;
            if c != 0 {
                s = s.wrapping_add(1);
                chamadas.push((c, a));
                if write != 0 {
                    covered!("cpt_13 position by round");
                    exe::comp_stats_set_club_final_position(read_usize(this.addr() + 8), c, a as u32);
                }
            }
            b = b.wrapping_add(1);
            j += 1;
        }
        r -= 1;
    }
    chamadas
}

/// Returns the club's side in the fixture: 1 home, 2 away, 3 neutral, from the fixture's stadium and the two clubs'
/// stadiums (national sides: by the stadium's nation).
///
/// `comp_util_get_fixture_venue_side(club, game)`. Without a venue in the fixture (`.stadium()`): by the home club
/// (`fixture::HOME_CLUB`), 3 if it has no stadium (`club::STADIUM`), otherwise 1 for the home club and 2 for the other.
/// National side (`exe::database_is_national_team`): 1 if the venue's nation (`stadium::CITY`, `city::NATION`) is the
/// club's (`.nation()`), 2 if it is the opponent's (`club::NATION`), otherwise 3. Club: if the venue is the club's
/// stadium and not the opponent's, 1; if it is both clubs' stadium, 1 for the home club and 2 for the other; if only
/// the opponent's, 2; if neither's, 3. The caller reads only `al`.
/// Original: `comp_util_get_fixture_venue_side` (0x4afa80, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_get_fixture_venue_side(club: usize, game: usize) -> u8 {
    let game = Fixture(game);
    let club = Club(club);
    let field = game.stadium() as u32;
    let home = read_ptr(game.addr() + fixture::HOME_CLUB);
    if field == 0 {
        if read_u32(home + club::STADIUM) == 0 {
            return 3;
        }
        return if club.addr() != home { 2 } else { 1 };
    }
    if exe::database_is_national_team(club) != 0 {
        let nation = read_ptr(field as usize + stadium::CITY);
        if nation != 0 && read_u32(nation + city::NATION) == (club.nation().addr() as u32) {
            return 1;
        }
        let home = read_ptr(game.addr() + fixture::HOME_CLUB);
        let other = if home == club.addr() { read_ptr(game.addr() + fixture::AWAY_CLUB) } else { home };
        if nation != 0 && read_u32(nation + city::NATION) == read_u32(other + club::NATION) {
            return 2;
        }
        return 3;
    }
    let home = read_ptr(game.addr() + fixture::HOME_CLUB);
    let other = if home == club.addr() { read_ptr(game.addr() + fixture::AWAY_CLUB) } else { home };
    let field = game.stadium() as u32;
    if field != (club.stadium().addr() as u32) {
        return if field != read_u32(other + club::STADIUM) { 3 } else { 2 };
    }
    if field == read_u32(other + club::STADIUM) {
        return if club.addr() != home { 2 } else { 1 }; // the same stadium for both
    }
    covered!("cpq_12 at home");
    1 // the club’s stadium
}

/// The qsort order of the nation coefficients: highest first.
///
/// `compare_nation_coefficients_desc(a, b)`: the qsort comparison (12 bytes per entry, the coefficient as a double at
/// +4) used by game_update_nation_coefficients 0x5d6187 (on the day of the month from table 0x9cd200): higher first
/// (−1); lower or unordered (NaN, the C0 flag of fcomp), 1; equal, 0. Hot (inside qsort): no counter. The original
/// sorts the array and frees it right afterwards, so the order never reaches the game (only the comparison run notices
/// it).
/// Original: `compare_nation_coefficients_desc` (0x4afba0, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_nation_coefficients_desc(a: usize, b: usize) -> i32 {
    let x = read_f64(a + 4);
    let y = read_f64(b + 4);
    if x > y {
        return -1; // o maior antes
    }
    if !(x >= y) {
        return 1;
    }
    0
}

/// Swaps two randomly drawn entries of a pointer array.
pub(crate) unsafe fn swap_random_pair(items: usize, n: i32) {
    let a = friendly_random(n) as i16 as isize;
    let b = friendly_random(n) as i16 as isize;
    let pa = (items as isize + 4 * a) as usize;
    let pb = (items as isize + 4 * b) as usize;
    let (va, vb) = (read_usize(pa), read_usize(pb));
    write_usize(pa, vb);
    write_usize(pb, va);
}

/// shuffles an array of n pointers with 3 × n swaps of drawn pairs
/// Original: `comp_util_shuffle_pointer_array` (0x4afb40, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_shuffle_pointer_array(items: usize, n: u32) {
    let n = n as u16 as i16 as i32;
    if 3 * n <= 0 {
        return;
    }
    covered!("cpm_3 shuffle pointers");
    let mut k = 0i16;
    loop {
        swap_random_pair(items, n);
        k = k.wrapping_add(1); // one more swap
        if k as i32 >= 3 * n {
            break;
        }
    }
}

/// Sorts the clubs by reputation (nulls last) and shuffles the first min(3 × picks, n).
///
/// `comp_util_sort_and_shuffle_top_clubs(clubs, n, picks)`: sorts the `n` clubs by reputation, highest first and nulls
/// at the end (qsort `exe::crt_qsort` with 0x4b07d0), and shuffles the first m = min(3 × picks, n) with m swaps of randomly
/// drawn pairs among them. `n` negative or smaller than `picks`: the "Error" box (the original); `picks` < 1: nothing.
/// Original: `comp_util_sort_and_shuffle_top_clubs` (0x4afbd0, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_sort_and_shuffle_top_clubs(clubs: usize, n: u32, escolhas: u32) {
    let (nn, e) = (n as u16 as i16, escolhas as u16 as i16);
    if nn < 0 || nn < e {
        return original_fn::<unsafe extern "C" fn(usize, u32, u32)>(IDX_COMP_UTIL_SORT_AND_SHUFFLE_TOP_CLUBS)(clubs, n, escolhas); // a caixa
    }
    if e < 1 {
        return;
    }
    exe::crt_qsort(clubs, nn as i32 as u32, 4, 0x4b07d0);
    let mut m = e.wrapping_mul(3);
    if m > nn {
        m = nn;
    }
    if m <= 0 {
        return;
    }
    covered!("cpm_3 shuffle the best");
    for _ in 0..m as i32 { // m trocas
        swap_random_pair(clubs, m as i32);
    }
}

/// A club's rating in the league stats (`comp_util_get_league_clash_type`): weighted average of 4 attributes, weights scaled by games under 25.
pub(crate) unsafe fn league_stats_rating(e: usize, b73: f64) -> f64 {
    let e = CompStatsClub(e);
    let (mut p, mut k73, mut k72) = (read_f64(rdata::F64_0_5), 1.0f64, 1.0f64);
    let games = read_u8(e.addr() + 4);
    if games < 0x19 {
        let v = games as f64;
        p = v * read_f64(rdata::F64_0_02);
        k73 = v * read_f64(rdata::F64_0_04);
        k72 = read_f64(rdata::F64_4) - v * read_f64(rdata::F64_0_1200480192);
    }
    let s = e.worst_position() as f64 * p + e.final_position() as f64 * k72 + b73 * k73 + e.best_position() as f64 * p;
    s / (p + p + k72 + k73)
}

/// Classifies a league match between two clubs: 1 = title clash, 3 = zone clash, 0 = none, from the weighted statistics
/// ratings (`exe::comp_stats_find_club_entry`) against a third of the clubs or 0.2 × clubs + `.promoted_count()` (+
/// `.promotion_playoff_count()`).
///
/// `comp_util_get_league_clash_type(a, b)`: both clubs must be in the same division (`.division()`, neither reserve nor
/// minor: `FN_COMP_IS_RESERVE_OR_MINOR_COMP`), its league (`G_COMP_OBJECT_TABLE[id]`) must have statistics
/// (`.comp_stats()`, `exe::comp_stats_find_club_entry` for both), no stages (`.current_stage()` = −1) and
/// `.comp_type()` = `comp_type::LEAGUE`, and the difference in `.current_position()` must be at most a quarter
/// (`rdata::F64_0_25`) of the clubs (`.club_count()`). With the ratings (`league_stats_rating`): 1 = title clash
/// (virtual `league_vtable::HAS_AUTOMATIC_PROMOTION` = 0, both below a third (`rdata::F64_0_3333333333`) of the clubs
/// and each not below the other's `.best_position()`); 3 = zone clash (`league_vtable::HAS_AUTOMATIC_PROMOTION` ≠ 0,
/// both up to 0.2 (`rdata::F64_0_2`) × clubs + `.promoted_count()`, or, with `.promotion_playoff_count()`, up to 0.2 ×
/// clubs + `.promoted_count()` + `.promotion_playoff_count()`, and the same cross-check of `.best_position()`);
/// otherwise 0. Returns only `al`. No SEH, no random draw; the virtual `league_vtable::HAS_AUTOMATIC_PROMOTION` is
/// called up to three times, as in the original.
/// Original: `comp_util_get_league_clash_type` (0x4b02e0, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_get_league_clash_type(a: usize, b: usize) -> u8 {
    let b = Club(b);
    let a = Club(a);
    let comp = a.division().addr();
    if comp == 0 || comp != b.division().addr() {
        return 0;
    }
    let smaller: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_COMP_IS_RESERVE_OR_MINOR_COMP);
    if smaller(comp) != 0 {
        return 0;
    }
    let league = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_u32(a.division().addr()) as usize)));
    if league.is_null() || league.comp_stats() == 0 {
        return 0;
    }
    let ea = CompStatsClub(exe::comp_stats_find_club_entry(league.comp_stats(), a));
    let eb = CompStatsClub(exe::comp_stats_find_club_entry(league.comp_stats(), b));
    if ea.is_null() || eb.is_null() || league.current_stage() as i32 != -1 || league.comp_type() != comp_type::LEAGUE {
        return 0;
    }
    let (a73, b73) = (ea.current_position() as i32, eb.current_position() as i32);
    let clubs = league.club_count() as i32;
    if (a73 - b73).abs() > ftol(clubs as f64 * read_f64(rdata::F64_0_25)) as i16 as i32 {
        return 0;
    }
    let (na, nb) = (league_stats_rating(ea.addr(), a73 as f64), league_stats_rating(eb.addr(), b73 as f64));
    let (a74, b74) = (ea.best_position() as f64, eb.best_position() as f64);
    let virtual98 = || {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(league.addr()) + league_vtable::HAS_AUTOMATIC_PROMOTION));
        f(league.addr())
    };
    // x87 comparisons: NaN counts as ”less” (C0) and ”less or equal” (C0|C3)
    let cruzadas = !(a74 > nb) && !(b74 > na);
    if virtual98() == 0 {
        let t = clubs as f64 * read_f64(rdata::F64_0_3333333333);
        if !(na >= t) && !(nb >= t) && cruzadas {
            return 1; // title race
        }
    }
    let be = league.promoted_count() as i8 as f64;
    if virtual98() != 0 {
        let t = clubs as f64 * read_f64(rdata::F64_0_2) + be;
        if !(na > t) && !(nb > t) && cruzadas {
            return 3;
        }
    }
    if virtual98() == 0 {
        return 0;
    }
    let bf = league.promotion_playoff_count() as i8;
    if bf == 0 {
        return 0;
    }
    let t = clubs as f64 * read_f64(rdata::F64_0_2) + be + bf as f64;
    if !(na > t) && !(nb > t) && cruzadas {
        3
    } else {
        0
    }
}

// simplification: ~4 million calls in the career: compares only the value, without regions (the routine only reads; 0x4a4850 may
// reorder the statistics, and the original already leaves it sorted for the copy).

/// Computes a club's strength score: money (`.cash()` / 400 or balance / 300) + 16 × reputation + the off-match ratings
/// (`exe::match_man_score_player_for_selection`) of the 15 picked (`FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS`) from the
/// squad; null club: −10,000,000.
///
/// `comp_util_get_club_strength_score(club)`: the money (without the finance table `G_FINANCE_TABLE`: `.cash()` / 400;
/// with it, the 64-bit balance / 300 via `exe::financetable_get_club_finance`), plus 16 × the reputation
/// (`.reputation()`), plus the off-match rating (`exe::match_man_score_player_for_selection`, 0x840, limit −1) of the
/// 15 picked players (`FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS`, codes 1, 0x804, 0x84, 3 × 0x204, 0x810, 0x90, 3 ×
/// 0x210, 4 × 0x40) among the players (`staff::PLAYER`) of the squad (`exe::database_collect_club_squad`). Null club:
/// −10,000,000. No SEH, no random draw of its own. The original pushes the number of players as a dword with only the
/// low byte written (the rest is stack leftover); `FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS` reads only the byte, and
/// the copy passes the byte zero-extended.
/// Original: `comp_util_get_club_strength_score` (0x4c0720, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_get_club_strength_score(club: usize) -> i32 {
    let club = Club(club);
    if club.is_null() {
        return -10_000_000;
    }
    let finances = read_usize(G_FINANCE_TABLE);
    let mut strength: i32 = 0;
    if finances == 0 {
        strength = club.cash() / 400;
    } else {
        let f = exe::financetable_get_club_finance(finances, club);
        if f != 0 {
            strength = (read_i64(f) / 300) as i32;
        }
    }
    strength = strength.wrapping_add((club.reputation() as i32) << 4); // the reputation
    let mut squad = [0usize; 50];
    let mut players = [0usize; 50];
    let mut escalados = [0usize; 15];
    let codes: [u16; 15] = [1, 0x804, 0x84, 0x204, 0x204, 0x204, 0x810, 0x90, 0x210, 0x210, 0x210, 0x40, 0x40, 0x40, 0x40];
    exe::database_collect_club_squad(club, squad.as_mut_ptr(), 0);
    let mut k: i8 = 0;
    for &p in squad.iter() {
        if p != 0 && read_usize(p + staff::PLAYER) != 0 {
            players[k as usize] = p;
            k = k.wrapping_add(1);
        }
    }
    let pick: AssignPositionsFn = core::mem::transmute(FN_MATCH_MAN_ASSIGN_PLAYERS_TO_POSITIONS);
    pick(players.as_ptr() as usize, k as u8 as u32, codes.as_ptr() as usize, 0xf, 0, escalados.as_mut_ptr() as usize, 0x840, club.addr(), 0, 0, 0, 0, 0, 0, 0);
    for (j, &p) in escalados.iter().enumerate() {
        if p != 0 {
            strength = strength.wrapping_add(exe::match_man_score_player_for_selection(p, codes[j] as u32, 0x840, club, -1, 0));
        }
    }
    strength
}

// simplification: 1,117 calls; only the value. The line-up and the ratings have their own comparers, and the fitness that the rating
// updates once (0x62ed70, with a random roll); the original already leaves it ready for the copy.

/// Sorts the competition history and returns the winner, runner-up or third (+0xa, +0xe, +0x12) from the competition's
/// first row; not found: 0.
///
/// `comp_util_get_latest_placed_club(comp, place)`: sorts the competition history (qsort `exe::crt_qsort` with `database_compare_comp_history_rows`),
/// searches (bsearch `exe::crt_bsearch`) for the competition with year −1 (any), walks back to its first row and returns the club
/// in `place` (0 winner +0xa, 1 runner-up +0xe, 2 third +0x12); not found: 0. Any other place: the original (the
/// comp_util.cpp error box). The key on the original's stack has only +4 and +8 written, and `database_compare_comp_history_rows` reads only those
/// two; the copy zeroes the rest.
/// Original: `comp_util_get_latest_placed_club` (0x4afd80, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_get_latest_placed_club(comp: usize, place: u32) -> usize {
    let place = place as u8;
    if place > 2 {
        return original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(IDX_COMP_UTIL_GET_LATEST_PLACED_CLUB)(comp, place as u32); // message box: original only
    }
    exe::crt_qsort(read_usize(G_CLUB_COMP_HISTORY_TABLE), read_i32(G_CLUB_COMP_HISTORY_COUNT) as u32, 0x1a, 0x540780);
    let mut key = [0u8; 0x1a];
    key[4..8].copy_from_slice(&comp.to_le_bytes());
    key[8..10].copy_from_slice(&0xffffu16.to_le_bytes()); // o ano: qualquer
    let mut p = exe::crt_bsearch((key.as_ptr()) as usize, read_usize(G_CLUB_COMP_HISTORY_TABLE), read_i32(G_CLUB_COMP_HISTORY_COUNT) as u32, 0x1a, 0x540780);
    if p == 0 {
        return 0;
    }
    let t = read_usize(G_CLUB_COMP_HISTORY_TABLE);
    while p > t {
        p -= 0x1a;
        if exe::database_compare_comp_history_rows(key.as_ptr(), p) != 0 {
            p += 0x1a;
            break;
        }
    }
    read_usize(p + [0xa, 0xe, 0x12][place as usize]) // champion, runner-up, third
}

/// Rebuilds the fixture from eighteen consecutive news slots and resolves its clubs, stadium and competition.
///
/// The `fixture` is rebuilt from 18 consecutive slots of the `news` item (`exe::newsitem_get_slot`) starting at
/// `*cursor` (which advances by 18): `fixture::HOME_CLUB_ID` and `fixture::AWAY_CLUB_ID` the clubs,
/// `fixture::HOME_RESULT_MARK`, `fixture::AWAY_RESULT_MARK`, +4 the stadium, +0 the competition,
/// `fixture::STAGE_INDEX`, `fixture::ROUND_PREFIX`, `fixture::ROUND_NAME`, `fixture::HOME_GOALS`/`fixture::AWAY_GOALS`,
/// `fixture::HOME_AGGREGATE`/`fixture::AWAY_AGGREGATE` and
/// `fixture::HOME_GOALS_EXTRA_TIME`/`fixture::AWAY_GOALS_EXTRA_TIME` the scores, `fixture::DAY`, `fixture::YEAR` and
/// +8. Then the pointers: clubs (`G_CLUB_TABLE`, 0x245 bytes) at `fixture::HOME_CLUB`/`fixture::AWAY_CLUB`, stadium
/// (`G_STADIUM_TABLE`, 0x4e) at `fixture::STADIUM` and competition (`G_CLUB_COMP_TABLE`, 0x6b) at
/// `fixture::COMPETITION`; id −1: 0.
/// Original: `comp_util_read_fixture_from_news_slots` (0x4b0010, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_read_fixture_from_news_slots(news: usize, fixture: usize, cursor: *mut u8) {
    // simplification: the original pushes the whole eax (remainder of the previous slot in the high bits); 0x785830 only reads the byte
    let mut v = *cursor;
    let mut read = || {
        let r = exe::newsitem_get_slot(news, v as u32);
        v = v.wrapping_add(1);
        r
    };
    let d = |o: usize, x: u32| write_u32(fixture + o, x);
    let w = |o: usize, x: u32| write_u16(fixture + o, x as u16);
    let b = |o: usize, x: u32| write_u8(fixture + o, x as u8);
    d(fixture::HOME_CLUB_ID, read());
    d(fixture::AWAY_CLUB_ID, read());
    b(fixture::HOME_RESULT_MARK, read());
    b(fixture::AWAY_RESULT_MARK, read());
    d(4, read());
    d(0, read());
    b(fixture::STAGE_INDEX, read());
    w(fixture::ROUND_PREFIX, read());
    w(fixture::ROUND_NAME, read());
    b(fixture::HOME_GOALS, read());
    b(fixture::AWAY_GOALS, read());
    b(fixture::HOME_AGGREGATE, read());
    b(fixture::AWAY_AGGREGATE, read());
    b(fixture::HOME_GOALS_EXTRA_TIME, read());
    b(fixture::AWAY_GOALS_EXTRA_TIME, read());
    w(fixture::DAY, read());
    w(fixture::YEAR, read());
    d(8, read());
    *cursor = v;
    let pointer = |o: usize, table: usize, size: u32| {
        let i = read_u32(fixture + o);
        if i == 0xffff_ffff { 0 } else { read_u32(table).wrapping_add(i.wrapping_mul(size)) }
    };
    d(fixture::HOME_CLUB, pointer(fixture::HOME_CLUB_ID, G_CLUB_TABLE, 0x245)); // home club
    d(fixture::AWAY_CLUB, pointer(fixture::AWAY_CLUB_ID, G_CLUB_TABLE, club::SIZE as u32));
    d(fixture::STADIUM, pointer(4, G_STADIUM_TABLE, 0x4e));
    d(fixture::COMPETITION, pointer(0, G_CLUB_COMP_TABLE, 0x6b));
}

/// Classifies the cup tie result for the club, from the final margin, the side and the reputation gap.
///
/// The category (1 to 7) of the tie result for the `club` in the `fixture`: the final margin (score
/// `.home_aggregate()`/`.away_aggregate()`, or `.home_goals_extra_time()`/`.away_goals_extra_time()`, or
/// `.home_goals()`/`.away_goals()`, the first one without 0xff) from the club's side (home = `.home_club()`) and the
/// clubs' reputations (`.reputation()` / 500). 1 thrashing, 2 and 3 wins against stronger sides, 4 ordinary, 5 and 7
/// defeats to weaker sides, 6 upset. Only `al` is faithful (the caller uses `al`).
/// Original: `comp_util_classify_tie_result_for_news` (0x4b0890, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_classify_tie_result_for_news(club: usize, fixture: usize) -> u8 {
    let fixture = Fixture(fixture);
    let home = fixture.home_club();
    let away = fixture.away_club();
    let h = (home.reputation() as i32 / 500) as i8 as i32;
    let a = (away.reputation() as i32 / 500) as i8 as i32;
    let (g1, g2) = if (fixture.home_aggregate() as u8) != 0xff {
        ((fixture.home_aggregate() as u8), (fixture.away_aggregate() as u8))
    } else if (fixture.home_goals_extra_time() as u8) != 0xff {
        ((fixture.home_goals_extra_time() as u8), (fixture.away_goals_extra_time() as u8))
    } else {
        ((fixture.home_goals() as u8), (fixture.away_goals() as u8))
    };
    let m = g1.wrapping_sub(g2) as i8;
    // from the club’s point of view: `eu` and `ele` the reputations, `vitoria` the margin in its favour
    let (eu, ele) = if club == home.addr() { (h, a) } else { (a, h) };
    let win = if club == home.addr() { m } else { m.wrapping_neg() };
    if win == 0 {
        if eu < ele - 16 || eu - 15 > ele {
            return 6;
        }
        if eu < ele - 11 {
            return 2;
        }
        if eu < ele - 3 {
            return 3;
        }
        if eu - 5 > ele {
            return 5;
        }
        return if eu - 11 <= ele { 4 } else { 7 };
    }
    // the upset: the home side tolerates 8 / 7, the away side 13 / 7 difference in a win by 1 or 2
    let (z1, z2) = if club == home.addr() { (8, 2) } else { (13, 7) };
    if win > 0 {
        if (win < 3 && eu < ele - z1) || (win > 2 && eu < ele - z2) {
            return 6;
        }
        if win > 4 {
            return 1;
        }
        if win < 2 {
            if eu < ele - 3 {
                return 2;
            }
            if eu < ele {
                return 3;
            }
        } else if win > 2 && eu < ele {
            return 2;
        }
        return 4;
    }
    let d = win.wrapping_neg();
    if (d < 3 && ele < eu - 7) || (d > 2 && ele < eu - 2) {
        return 6;
    }
    if d < 2 {
        if eu - 6 > ele {
            return 7;
        }
        if eu - 2 > ele {
            return 5;
        }
    }
    4 // no highlight
}

/// Writes the fixture's clubs, venue, competition, score and round into 18 consecutive news slots and advances the
/// cursor.
///
/// The community map gives the 2nd argument as unused, but it is the news item. Writes the fixture into 18 consecutive
/// data slots of the news item (`exe::newsitem_set_slot`), starting at slot `[cursor]`, and advances the cursor by 18.
/// Order: `fixture::HOME_CLUB_ID`, `fixture::AWAY_CLUB_ID`, `fixture::HOME_RESULT_MARK`, `fixture::AWAY_RESULT_MARK`,
/// +4, +0, `fixture::STAGE_INDEX`, `fixture::ROUND_PREFIX`, `fixture::ROUND_NAME`, `fixture::HOME_GOALS`,
/// `fixture::AWAY_GOALS`, `fixture::HOME_AGGREGATE`, `fixture::AWAY_AGGREGATE`, `fixture::HOME_GOALS_EXTRA_TIME`,
/// `fixture::AWAY_GOALS_EXTRA_TIME`, `fixture::DAY`, `fixture::YEAR`, +8 (bytes and the words
/// `fixture::ROUND_NAME`/`fixture::ROUND_PREFIX` signed; `fixture::YEAR` and `fixture::DAY` unsigned). Returns what the
/// last write returned. The original keeps the index of the 18th slot in the low byte of the `cursor` argument itself
/// (only the low part reaches `exe::newsitem_set_slot`).
/// Original: `comp_util_write_fixture_to_news_slots` (0x4afeb0, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_write_fixture_to_news_slots(game: usize, news: usize, cursor: *mut u8) -> u32 {
    let game = Fixture(game);
    let b = |o: usize| read_i8(game.addr() + o) as i32 as u32;
    let s = |o: usize| read_i16(game.addr() + o) as i32 as u32;
    let values = [
        (game.home_club_id() as u32), (game.away_club_id() as u32), b(fixture::HOME_RESULT_MARK), b(fixture::AWAY_RESULT_MARK), read_u32(game.addr() + 4), read_u32(game.addr() + 0), b(fixture::STAGE_INDEX), s(fixture::ROUND_PREFIX), s(fixture::ROUND_NAME),
        b(fixture::HOME_GOALS), b(fixture::AWAY_GOALS), b(fixture::HOME_AGGREGATE), b(fixture::AWAY_AGGREGATE), b(fixture::HOME_GOALS_EXTRA_TIME), b(fixture::AWAY_GOALS_EXTRA_TIME), (game.day() as u32), (game.year() as u32), read_u32(game.addr() + 8),
    ];
    let mut k = *cursor;
    let mut r = 0;
    for v in values {
        r = exe::newsitem_set_slot(news, k as i8, v);
        k = k.wrapping_add(1);
    }
    *cursor = k;
    r
}

/// Writes the stage and round names (with their genders, through template 0x9a7cfc), or just one of them; without an
/// output or under 200 bytes, the error box and 0.
///
/// `comp_util_format_stage_round_name(stage, round, out, size, style)`: without an output or with fewer than 200 bytes,
/// the error box (line 0x26a1) and 0. Clears the output; with both stage and round, "{stage} {round}"
/// (`FN_LANGLIB_SPRINTF` with template 0x9a7cfc, the 100-byte names from `exe::comp_util_get_stage_name` and the
/// genders from `exe::comp_util_get_stage_name_gender`); with only one of them, its name directly in the output
/// (`exe::comp_util_get_stage_name` with the size); with neither, just the zero. Returns 1 (0 on the error box).
/// Original: `comp_util_format_stage_round_name` (0x4bffc0, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_format_stage_round_name(stage: u32, round: u32, out: *mut u8, size: i32, style: u32) -> i32 {
    if out.is_null() || size < 0xc8 {
        return original_fn::<unsafe extern "C" fn(u32, u32, *mut u8, i32, u32) -> i32>(IDX_COMP_UTIL_FORMAT_STAGE_ROUND_NAME)(stage, round, out, size, style); // a caixa
    }
    // 0x4b4850: 4 pushes (style, id, size, output); add esp -> cdecl; 0x4ba8f0: push style; push id -> cdecl
    *out = 0;
    if stage != 0 && round != 0 {
        let (mut a, mut b) = ([0u8; 0x64], [0u8; 0x64]);
        exe::comp_util_get_stage_name(a.as_mut_ptr(), 0x64, stage, style);
        let ga = exe::comp_util_get_stage_name_gender(stage, style) as i8 as i32 as u32;
        exe::comp_util_get_stage_name(b.as_mut_ptr(), 0x64, round, style);
        let gb = exe::comp_util_get_stage_name_gender(round, style) as i8 as i32 as u32;
        // 0x66f4e0: 8 pushes; add esp, 0x20 -> cdecl (output, template, ga, ga, gb, gb, &a, &b)
        let text: unsafe extern "C" fn(*mut u8, usize, u32, u32, u32, u32, *const u8, *const u8) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
        text(out, 0x9a7cfc, ga, ga, gb, gb, a.as_ptr(), b.as_ptr());
    } else if stage != 0 {
        exe::comp_util_get_stage_name(out, size, stage, style);
    } else if round != 0 {
        exe::comp_util_get_stage_name(out, size, round, style);
    } else {
        return 0; // neither of the two: 0 (0x4c0136)
    }
    1
}

/// Returns the round's value between two limits: the average with a single round, otherwise (high − low) × round /
/// (rounds − 1); outside 0..100, the error box.
///
/// `comp_util_scale_by_round(low, high, round, rounds)` (signed bytes and words; the words arrive in registers with
/// garbage above them and only the word is read): with a single round, the average (low + high) / 2; otherwise (high −
/// low) × round / (rounds − 1). Outside 0..=100: the original (error box at line 0x256e, returns 0 or 100). Returns the
/// original's whole eax (the caller pushes the whole register to `add_qualified_for_round_record`/`add_knocked_out_record`): the byte in `al` over the sum
/// (single round) or the quotient.
/// Original: `comp_util_scale_by_round` (0x4bf850, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_scale_by_round(low: u32, high: u32, round: u32, rounds: u32) -> u32 {
    let (eax, v) = interpolate_by_round(low, high, round, rounds);
    if !(0..=100).contains(&v) {
        return original_fn::<unsafe extern "C" fn(u32, u32, u32, u32) -> u32>(IDX_COMP_UTIL_SCALE_BY_ROUND)(low, high, round, rounds);
    }
    if rounds as u16 != 1 {
        covered!("cpq_11 lineup by round");
    }
    eax
}

/// Stores competition, year and four values in the free row of the competition history (without one, grows by 20 rows
/// through `exe::database_grow_comp_history_table`); returns the row.
///
/// `comp_util_add_history_record(league, a, b, c, d)`: the free row of the competition history (without one, it grows
/// by 20 rows through `exe::database_grow_comp_history_table` and uses the first new one; out of memory, 0) receives
/// the competition (the league's +4), the year `G_CURRENT_DATE_YEAR` (+8), and the four values at
/// `comp_history_row::CHAMPION` (+0xa), +0xe, +0x12, +0x16. Returns the row. No SEH, no random draw, no locals.
/// Original: `comp_util_add_history_record` (0x4afce0, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_add_history_record(league: usize, a: u32, b: u32, c: u32, d: u32) -> usize {
    let mut r = free_comp_history_row();
    if r == 0 {
        covered!("competition history grow");
        if exe::database_grow_comp_history_table(0x14) == 0 {
            return 0;
        }
        let n = read_i32(G_CLUB_COMP_HISTORY_COUNT);
        r = read_usize(G_CLUB_COMP_HISTORY_TABLE).wrapping_add(26usize.wrapping_mul(n.wrapping_sub(20) as usize));
    }
    write_usize(r + 4, read_usize(league + 4));
    write_u16(r + 8, read_u16(G_CURRENT_DATE_YEAR)); // o ano
    for (k, v) in [a, b, c, d].into_iter().enumerate() {
        write_u32(r + comp_history_row::CHAMPION + 4 * k, v);
    }
    r
}

/// Returns the group (the containing stage) of a stage or round by id, from fixed ranges; −1 without a group.
///
/// `comp_util_get_parent_stage_id(id)`: 0x3e8..0x3fb, 0x41f..0x42e, 0x413, 0x414, 0x47b and 0x3fc -> 0x41c;
/// 0x473..0x474 -> 0x410; 0x475..0x479 -> 0x47a; 0x3fd..0x406 -> 0xa; 0x407..0x40b and 0x472 -> 0x14; 0x47e..0x47f ->
/// 0x1e; 0x40c..0x40e -> 0x82; 0x43d..0x444 -> 0x449; 0x445..0x448 -> 0x44a; 0x44d..0x450 -> 0x451; everything else −1
/// (in the original's test order).
/// Original: `comp_util_get_parent_stage_id` (0x4b01d0, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_get_parent_stage_id(id: i32) -> i32 {
    let em = |a: i32, b: i32| id >= a && id <= b;
    if em(0x3e8, 0x3fb) {
        return 0x41c;
    }
    if em(0x473, 0x474) {
        return 0x410;
    }
    if em(0x475, 0x479) {
        return 0x47a;
    }
    if em(0x3fd, 0x406) {
        covered!("cqg_6 group 0xa");
        return 0xa; // cqg6: the group of stages 0x3fd..0x406
    }
    if em(0x407, 0x40b) || id == 0x472 {
        return 0x14;
    }
    if em(0x47e, 0x47f) {
        return 0x1e;
    }
    if em(0x41f, 0x42e) {
        return 0x41c;
    }
    if em(0x40c, 0x40e) {
        return 0x82;
    }
    if id == 0x413 || id == 0x414 || id == 0x47b || id == 0x3fc {
        return 0x41c;
    }
    if em(0x43d, 0x444) {
        return 0x449;
    }
    if em(0x445, 0x448) {
        return 0x44a;
    }
    if em(0x44d, 0x450) {
        return 0x451;
    }
    -1
}

/// Writes the news sentence of a club's cup tie result (won, through, lost) into the Text `text`, with the score,
/// the round, the venue and the links.
///
/// `comp_util_write_cup_tie_result_news(club_index, fixture, verdict, outcome, next_stage, text)` (cdecl). The club is
/// `club_at(club_index)`; at home (`club_index` = `.home_club_id()`) the opponent is `.away_club()`, the club's mark
/// `.home_result_mark()` and the opponent's `.away_result_mark()`, away the other way round; the settling mark is the
/// home mark, else the away mark. The goals: the aggregate (`.home_aggregate()` other than −1), else after extra time
/// (`.home_goals_extra_time()` other than −1), else at full time, the club's first; the score text "<goals>-<goals>"
/// (format 0x9a3d94, 16 bytes). Won: `outcome` (byte) other than 2, 4 and −1. The round: `.round_name()` without its
/// variant (`round_name_without_variant`); the final is round 0x96 with no `.round_prefix()`.
///
/// The round label (form 1, 500 bytes, `exe::comp_util_format_comp_stage_round_label`): in the World Cup Oceania
/// qualifying section with a club (home, then away) whose nation's continent is not Oceania, and the same for the Asian
/// section and Asia, the label of the FIFA World Cup row with round 0xa0 and its gender; otherwise the competition's,
/// stage and round label, gendered by the raw round name (`exe::comp_util_get_stage_name_gender`), else the stage,
/// else the competition's gender (`.gender_name()`), else 1. With a stadium (`.stadium()`), the venue: its name, or
/// "<name>, <city>" (0x98f6ac) in languages 0 and 1 and "<name> (<city>)" (0x9a3d8c) in the others
/// (`exe::langlib_get_language`; `FN_CRT_SPRINTF`).
///
/// By `next_stage` (word): 1 or above names the next stage (`exe::comp_util_get_stage_name`, 100 bytes, and its gender;
/// `TieNews::say_through_to_stage`); −2 "the next stage" (`TieNews::say_through_unnamed`); otherwise the round or the
/// trophy (`TieNews::say_round_result`). Each picks the sentence by the mark, then the verdict (`tie_verdict`), the
/// outcome, the final, a single match (`.home_aggregate()` −1) and the stadium (`TieNews::say`).
///
/// Then the text is set (`exe::text_set_string`) and linked: the club's and the opponent's short names (0x7d5); the
/// score text (0x7d3, the match statistics slot) when the statistics entry is the fixture's
/// (`exe::match_stats_fixture_entry_matches`: slot, home and away ids, day, year); with the competition current on the
/// fixture's date (`is_comp_current_on_date` of `G_FRIENDLY`) the label (2000, the competition), and the next stage's
/// name when there is one (2000). The next stage's buffer starts with the byte at 0xad9c64.
/// Original: `comp_util_write_cup_tie_result_news` (0x4b0b80, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_write_cup_tie_result_news(club_index: i32, fixture: usize, verdict: u32, outcome: u32, next_stage: u32, text: usize) {
    covered!("gra_24 cup tie news");
    let f = Fixture(fixture);
    let club = club_at(club_index as isize);
    let (home_mark, away_mark) = (f.home_result_mark(), f.away_result_mark());
    let mark = if home_mark != 0 { home_mark } else { away_mark };
    let at_home = club_index as usize == f.home_club_id();
    let (opponent, own_mark, opponent_mark) = if at_home { (f.away_club(), home_mark, away_mark) } else { (f.home_club(), away_mark, home_mark) };
    let (home_goals, away_goals) = if f.home_aggregate() != -1 {
        (f.home_aggregate(), f.away_aggregate())
    } else if f.home_goals_extra_time() != -1 {
        (f.home_goals_extra_time(), f.away_goals_extra_time())
    } else {
        (f.home_goals(), f.away_goals())
    };
    let (own_goals, opponent_goals) = if at_home { (home_goals as i32, away_goals as i32) } else { (away_goals as i32, home_goals as i32) };
    let mut news = TieNews {
        club,
        opponent,
        comp: f.competition(),
        stadium: Stadium(f.stadium()),
        own_goals,
        opponent_goals,
        label_gender: 0,
        next_gender: 0,
        won: !matches!(outcome as u8, 2 | 4 | 0xff),
        final_round: false,
        single_match: f.home_aggregate() == -1,
        mark,
        verdict: verdict as u8,
        label: [0; 500],
        next: [0; 100],
        venue: [0; 300],
        score: [0; 16],
    };
    news.next[0] = read_u8(0xad9c64);
    let fmt = news_text_format();
    fmt(news.score.as_mut_ptr() as usize, 0x9a3d94, own_goals, opponent_goals);
    let raw_round = f.round_name();
    let round = round_name_without_variant(raw_round);
    let prefix = f.round_prefix();
    news.final_round = round == ROUND_FINAL && prefix == 0;
    let comp = news.comp;
    let outside = |continent_key: usize| {
        let away_from = |c: Club| {
            let nation = c.nation();
            if nation.is_null() {
                return false;
            }
            let continent = nation.continent();
            continent != 0 && read_i32(continent) != read_i32(continent_key)
        };
        away_from(f.home_club()) || away_from(f.away_club())
    };
    let comp_id = f.comp_id();
    let world_cup_label = if comp_id == read_i32(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION) {
        outside(KEY_CONTINENT_OCEANIA)
    } else if comp_id == read_i32(KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION) {
        outside(KEY_CONTINENT_ASIA)
    } else {
        false
    };
    let label = news.label.as_mut_ptr();
    if world_cup_label {
        covered!("gra_24 world cup qualifying label");
        let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_i32(KEY_CLUB_COMP_FIFA_WORLD_CUP) as usize).wrapping_mul(0x6b));
        exe::comp_util_format_comp_stage_round_label(world_cup, 0, 0xa0, 1, label, 500);
        news.label_gender = exe::comp_util_get_stage_name_gender(0xa0, 0) as i8 as i32;
    } else {
        exe::comp_util_format_comp_stage_round_label(comp, prefix as u16 as u32, round as u16 as u32, 1, label, 500);
        let gender = if raw_round != 0 {
            exe::comp_util_get_stage_name_gender(raw_round as u16 as u32, 0)
        } else if prefix != 0 {
            exe::comp_util_get_stage_name_gender(prefix as u16 as u32, 0)
        } else if comp.is_null() {
            1
        } else {
            comp.gender_name()
        };
        news.label_gender = gender as i8 as i32;
    }
    let stadium = news.stadium;
    if !stadium.is_null() {
        let lang = exe::langlib_get_language();
        let format = if lang == 0 || exe::langlib_get_language() == 1 { 0x98f6ac } else { 0x9a3d8c }; // "%s, %s" / "%s (%s)"
        let city = stadium.city();
        if city == 0 {
            let name = stadium.name_addr();
            let mut k = 0;
            loop {
                let c = read_u8(name + k);
                news.venue[k] = c;
                if c == 0 {
                    break;
                }
                k += 1;
            }
        } else {
            covered!("gra_24 venue with the city");
            let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
            sprintf(news.venue.as_mut_ptr() as usize, format, stadium.name_addr(), city + 4); // the city's name
        }
    }
    let stage = next_stage as u16 as i16;
    if stage >= 1 {
        exe::comp_util_get_stage_name(news.next.as_mut_ptr(), 100, next_stage, 0);
        news.next_gender = exe::comp_util_get_stage_name_gender(next_stage, 0) as i8 as i32;
        news.say_through_to_stage(own_mark, opponent_mark);
    } else if stage == -2 {
        news.say_through_unnamed();
    } else {
        news.say_round_result(own_mark, opponent_mark);
    }
    exe::text_set_string(text, NEWS_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
    exe::text_add_link(text, (opponent.addr() + club::SHORT_NAME) as *const u8, 0x7d5, opponent); // gra24: the opponent's link
    let slot = f.match_stats_slot();
    if exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), slot as i32, f.home_club_id() as i32, f.away_club_id() as i32, f.day() as i16 as i32, f.year() as i16 as i32) != 0 {
        exe::text_add_link(text, news.score.as_ptr(), 0x7d3, slot);
    }
    let mut date = [0u8; 8];
    exe::new_date(date.as_mut_ptr() as usize, f.day() as u32, f.year() as u32);
    if is_comp_current_on_date(read_usize(G_FRIENDLY), comp.addr(), date.as_ptr() as usize) != 0 {
        exe::text_add_link(text, news.label.as_ptr(), 2000, comp);
        if news.next[0] != 0 {
            exe::text_add_link(text, news.next.as_ptr(), 2000, comp);
        }
    }
}

/// Comparer for `comp_util_write_cup_tie_result_news`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "C" fn compare_comp_util_write_cup_tie_result_news(club_index: i32, fixture: usize, verdict: u32, outcome: u32, next_stage: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(i32, usize, u32, u32, u32, usize)>(IDX_COMP_UTIL_WRITE_CUP_TIE_RESULT_NEWS);
    compare_text_writer(text, || o(club_index, fixture, verdict, outcome, next_stage, text),
        |t| comp_util_write_cup_tie_result_news(club_index, fixture, verdict, outcome, next_stage, t),
        || format!("comp_util_write_cup_tie_result_news(club {}, fixture {:#x}, verdict {}, outcome {}, next stage {})", club_index, fixture, verdict as u8, outcome as u8 as i8, next_stage as u16 as i16));
}

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
    let fixture = Fixture(fixture);
    let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let f = fixture;
    let comp = fixture.competition().addr();
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
    let c = Club(club_at(club as isize).addr());
    let cg = c.short_gender_name() as i8 as i32;
    let g = gender as i8 as i32;
    let format = match result as u8 {
        2 | 4 | 0xff => 0x9a3dbc, // cqy5: "defeat in"
        _ => 0x9a3e1c,
    };
    covered!("cqy_5 cup win or defeat news");
    lang_sprintf(0xde1f64, format, cg, cg, g, g, c.addr() + club::SHORT_NAME, label.as_ptr() as usize);
    exe::text_set_string(text, 0xde1f64 as *const u8);
    exe::text_add_link(text, (c.addr() + club::SHORT_NAME) as *const u8, 0x7d5, c);
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
    let c = Club(club_at(club as isize).addr());
    let cg = c.short_gender_name() as i8 as i32;
    let team = c.addr() + club::SHORT_NAME;
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

/// Copies the C string at `src` with its terminator to `dst`.
pub(crate) unsafe fn copy_c_string(dst: usize, src: usize) {
    let mut i = 0;
    loop {
        let b = read_u8(src + i);
        write_u8(dst + i, b);
        if b == 0 {
            break;
        }
        i += 1;
    }
}

/// Lower-cases a C string in place, one character at a time (tolower `0x9465fc`, cdecl, the signed character), with a
/// signed byte index as in the original.
pub(crate) unsafe fn lower_case_in_place(p: usize) {
    let mut k: i8 = 0;
    let mut c = read_u8(p) as i8;
    while c != 0 {
        write_u8(p.wrapping_add(k as isize as usize), exe::at_9465fc(c as i32) as u8);
        k = k.wrapping_add(1);
        c = read_u8(p.wrapping_add(k as isize as usize)) as i8;
    }
}

/// The round name code of a fixture without its variant: `fixture::ROUND_NAME` less its units digit (0 single match,
/// 3 first leg, 4 second leg, 5 replay) when it is below 1000, as the original's idiv by 10 (a word).
pub(crate) fn round_name_without_variant(code: i16) -> i16 {
    if code < 1000 { code.wrapping_sub(code % 10) } else { code }
}

/// The round name code of the final (`fixture::ROUND_NAME` without its variant, with no `fixture::ROUND_PREFIX`).
pub(crate) const ROUND_FINAL: i16 = 0x96;

/// The pieces of the cup tie news of one club (`comp_util_write_cup_tie_result_news`).
pub(crate) struct TieNews {
    pub(crate) club: Club,
    pub(crate) opponent: Club,
    pub(crate) comp: Competition,
    pub(crate) stadium: Stadium,
    pub(crate) own_goals: i32,
    pub(crate) opponent_goals: i32,
    pub(crate) label_gender: i32,
    pub(crate) next_gender: i32,
    pub(crate) won: bool,
    pub(crate) final_round: bool,
    pub(crate) single_match: bool,
    pub(crate) mark: i8,
    pub(crate) verdict: u8,
    pub(crate) label: [u8; 500],
    pub(crate) next: [u8; 100],
    pub(crate) venue: [u8; 300],
    pub(crate) score: [u8; 16],
}

impl TieNews {
    /// Writes the sentence `format` into `NEWS_TEXT_BUFFER`. The arguments: the club's gender twice, the middle's
    /// gender twice (when there is a middle), the opponent's gender twice, the stadium's gender twice (with `venue`);
    /// then the club's short name, the middle's text, the goals, the opponent's short name and the venue text (with
    /// `venue`). The goals with the opponent first also rewrite the score text (format 0x9a3d94) that way; a trophy
    /// middle (not the forfeit one) copies the competition's name into the label.
    pub(crate) unsafe fn say(&mut self, format: usize, middle: TieMiddle, score: TieScore, venue: bool) {
        let gender = |g: u8| g as i8 as i32 as usize;
        let comp = self.comp;
        let middle_part = match middle {
            TieMiddle::Round => Some((self.label_gender as usize, self.label.as_ptr() as usize)),
            TieMiddle::Trophy | TieMiddle::TrophyKeepLabel => Some((gender(comp.gender_name()), comp.name_addr())),
            TieMiddle::TrophyRoundGender => Some((self.label_gender as usize, comp.name_addr())),
            TieMiddle::NextStage => Some((self.next_gender as usize, self.next.as_ptr() as usize)),
            TieMiddle::Nothing => None,
        };
        let mut a = [0usize; 14];
        let mut n = 0;
        let mut put = |v: usize| {
            a[n] = v;
            n += 1;
        };
        let club_gender = gender(self.club.short_gender_name());
        put(club_gender);
        put(club_gender);
        if let Some((g, _)) = middle_part {
            put(g);
            put(g);
        }
        let opponent_gender = gender(self.opponent.short_gender_name());
        put(opponent_gender);
        put(opponent_gender);
        if venue {
            let stadium_gender = gender(self.stadium.gender_name());
            put(stadium_gender);
            put(stadium_gender);
        }
        put(self.club.addr() + club::SHORT_NAME);
        if let Some((_, t)) = middle_part {
            put(t);
        }
        match score {
            TieScore::OwnFirst => {
                put(self.own_goals as usize);
                put(self.opponent_goals as usize);
            }
            TieScore::OpponentFirst => {
                put(self.opponent_goals as usize);
                put(self.own_goals as usize);
            }
            TieScore::Nothing => {}
        }
        put(self.opponent.addr() + club::SHORT_NAME);
        if venue {
            put(self.venue.as_ptr() as usize);
        }
        let fmt = news_text_format();
        fmt(NEWS_TEXT_BUFFER, format, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13]);
        if score == TieScore::OpponentFirst {
            fmt(self.score.as_mut_ptr() as usize, 0x9a3d94, self.opponent_goals, self.own_goals); // "<goals>-<goals>"
        }
        if matches!(middle, TieMiddle::Trophy | TieMiddle::TrophyRoundGender) {
            let name = comp.name_addr();
            let mut k = 0;
            loop {
                let c = read_u8(name + k);
                self.label[k] = c;
                if c == 0 {
                    break;
                }
                k += 1;
            }
        }
    }

    /// `say` with the venue when the fixture has a stadium (`with_venue`), else without it (`without`).
    pub(crate) unsafe fn say_at(&mut self, with_venue: usize, without: usize, middle: TieMiddle, score: TieScore) {
        if !self.stadium.is_null() {
            self.say(with_venue, middle, score, true)
        } else {
            self.say(without, middle, score, false)
        }
    }

    /// The forfeit lost: "have lost in the <round> by being the lower seeded team" (0x99ecec when the opponent's mark
    /// is below the club's, else 0x99ec54).
    pub(crate) unsafe fn say_forfeit_lost(&mut self, own_mark: i8, opponent_mark: i8) {
        covered!("gra_24 forfeit lost");
        let format = if opponent_mark < own_mark { 0x99ecec } else { 0x99ec54 };
        self.say(format, TieMiddle::Round, TieScore::Nothing, false)
    }

    /// The defeats on a settling mark, shared by the three kinds of sentence: away goals, the lower seed, extra time
    /// (the opponent's goals first), penalties, the shootout and the golden goal, with and without the venue.
    pub(crate) unsafe fn say_mark_defeat(&mut self) {
        use TieMiddle::Round;
        use TieScore::{OpponentFirst, OwnFirst};
        match self.mark {
            tie_mark::AWAY_GOALS => self.say_at(0x9a0cb4, 0x9a2294, Round, OwnFirst),
            tie_mark::HIGHER_SEED => self.say_at(0x9a0a60, 0x9a2070, Round, OwnFirst),
            tie_mark::EXTRA_TIME => self.say_at(0x9a3a0c, 0x9a3964, Round, OpponentFirst),
            tie_mark::PENALTIES => self.say_at(0x9a3754, 0x9a1d40, Round, OwnFirst),
            tie_mark::SHOOTOUT => self.say_at(0x9a3544, 0x9a1b5c, Round, OwnFirst),
            _ => self.say_at(0x99fea4, 0x9a1954, Round, OwnFirst), // the golden goal
        }
    }

    /// An ordinary defeat by the verdict: disappointing (0x9a2b50 with the venue in a single match, else 0x9a2ab0),
    /// very disappointing (0x9a29f0 / 0x9a294c), any other verdict (single match 0x9a2630 / 0x9a25a0, two legs
    /// 0x9a26dc); the opponent's goals first. A shock defeat is written by the caller; in a round with no stage after
    /// it the other verdicts have their own two-legs sentence with the venue (`say_round_result`).
    pub(crate) unsafe fn say_verdict_defeat(&mut self) {
        use TieMiddle::Round;
        use TieScore::OpponentFirst;
        let with_venue = self.single_match && !self.stadium.is_null();
        match self.verdict {
            tie_verdict::DISAPPOINTING => self.say(if with_venue { 0x9a2b50 } else { 0x9a2ab0 }, Round, OpponentFirst, with_venue),
            tie_verdict::VERY_DISAPPOINTING => self.say(if with_venue { 0x9a29f0 } else { 0x9a294c }, Round, OpponentFirst, with_venue),
            _ if self.single_match => self.say_at(0x9a2630, 0x9a25a0, Round, OpponentFirst),
            _ => self.say(0x9a26dc, Round, OpponentFirst, false),
        }
    }

    /// The tie decided in a round with no stage after it named: "have won in the <round>" or, in the final, "have won
    /// the <competition>", or the defeat (block of `comp_util_write_cup_tie_result_news` for a next stage below 1 other
    /// than −2).
    pub(crate) unsafe fn say_round_result(&mut self, own_mark: i8, opponent_mark: i8) {
        use TieMiddle::{Round, Trophy, TrophyKeepLabel, TrophyRoundGender};
        use TieScore::{OpponentFirst, OwnFirst};
        covered!("gra_24 round result");
        if own_mark < 0 || opponent_mark < 0 {
            if !self.won {
                return self.say_forfeit_lost(own_mark, opponent_mark);
            }
            covered!("gra_24 forfeit won");
            let club_first = own_mark < opponent_mark;
            if self.final_round {
                return self.say(if club_first { 0x99eef8 } else { 0x99ee60 }, TrophyKeepLabel, TieScore::Nothing, false);
            }
            return self.say(if club_first { 0x99edec } else { 0x99ed54 }, Round, TieScore::Nothing, false);
        }
        // The wins on a settling mark: (in a round with/without the venue, in the final with/without the venue).
        let mark_win = match self.mark {
            tie_mark::AWAY_GOALS => Some((0x9a0e20, 0x9a0d78, 0x9a0f8c, 0x9a0ee4)),
            tie_mark::EXTRA_TIME => Some((0x9a0840, 0x9a079c, 0x9a09a0, 0x9a08fc)),
            tie_mark::PENALTIES => Some((0x9a058c, 0x9a04f0, 0x9a06e4, 0x9a0644)),
            tie_mark::SHOOTOUT => Some((0x9a02e0, 0x9a0244, 0x9a0438, 0x9a0398)),
            tie_mark::GOLDEN_GOAL => Some((0x9a0010, 0x99ff68, 0x9a0180, 0x9a00d4)),
            _ => None,
        };
        if self.mark == tie_mark::HIGHER_SEED {
            if self.won {
                return self.say_at(0x9a0be4, 0x9a0b30, Round, OwnFirst); // the higher seed wins the round, even a final
            }
            return self.say_mark_defeat();
        }
        if let Some((round, round_plain, trophy, trophy_plain)) = mark_win {
            if !self.won {
                return self.say_mark_defeat();
            }
            if !self.final_round {
                return self.say_at(round, round_plain, Round, OwnFirst);
            }
            covered!("gra_24 trophy won on a mark");
            return self.say_at(trophy, trophy_plain, Trophy, OwnFirst);
        }
        match self.verdict {
            tie_verdict::SHOCK if !self.won => {
                if self.single_match {
                    self.say_at(0x9a1638, 0x9a15a0, Round, OpponentFirst)
                } else {
                    self.say(0x9a16ec, Round, OpponentFirst, false)
                }
            }
            // A shock win, an excellent and a fine result (these two whatever the outcome): (single match in a round
            // with/without the venue, in the final with/without the venue; two legs in a round, in the final).
            tie_verdict::SHOCK | tie_verdict::EXCELLENT | tie_verdict::FINE => {
                let (round, round_plain, trophy, trophy_plain, legs, legs_trophy) = match self.verdict {
                    tie_verdict::SHOCK => (0x99fb6c, 0x99fad8, 0x99fcb4, 0x99fc1c, 0x99fd64, 0x99fe04),
                    tie_verdict::EXCELLENT => (0x99f788, 0x99f6f0, 0x99f8d8, 0x99f83c, 0x99f990, 0x99fa34),
                    _ => (0x99f3bc, 0x9a2cb4, 0x99f500, 0x99f46c, 0x99f5b0, 0x99f650),
                };
                match (self.single_match, self.final_round) {
                    (true, false) => self.say_at(round, round_plain, Round, OwnFirst),
                    (true, true) => self.say_at(trophy, trophy_plain, Trophy, OwnFirst),
                    (false, false) => self.say(legs, Round, OwnFirst, false),
                    (false, true) => self.say(legs_trophy, Trophy, OwnFirst, false),
                }
            }
            tie_verdict::DISAPPOINTING | tie_verdict::VERY_DISAPPOINTING => self.say_verdict_defeat(),
            _ if self.won => match (self.single_match, self.final_round) {
                (true, false) => self.say_at(0x99f0b0, 0x99f020, Round, OwnFirst), // gra24: the plain win in a round
                (true, true) => self.say_at(0x99f1e4, 0x99f158, TrophyRoundGender, OwnFirst),
                (false, false) => self.say(0x99f28c, Round, OwnFirst, false),
                (false, true) => self.say(0x99f324, Trophy, OwnFirst, false),
            },
            _ if self.single_match => self.say_at(0x9a2630, 0x9a25a0, Round, OpponentFirst),
            _ => self.say_at(0x99ef6c, 0x9a26dc, Round, OpponentFirst), // only here two legs have a venue variant
        }
    }

    /// The tie that sends the club "through to the next stage" without naming it, or the defeat (next stage −2).
    pub(crate) unsafe fn say_through_unnamed(&mut self) {
        use TieMiddle::{Nothing, Round};
        use TieScore::{OpponentFirst, OwnFirst};
        covered!("gra_24 through to the next stage");
        // The wins on a settling mark (with/without the venue).
        let mark_win = match self.mark {
            tie_mark::AWAY_GOALS => Some((0x9a23dc, 0x9a233c)),
            tie_mark::HIGHER_SEED => Some((0x9a21d0, 0x9a2124)),
            tie_mark::EXTRA_TIME => Some((0x9a1fbc, 0x9a1f24)),
            tie_mark::PENALTIES => Some((0x9a1e74, 0x9a1de0)),
            tie_mark::SHOOTOUT => Some((0x9a1c90, 0x9a1bfc)),
            tie_mark::GOLDEN_GOAL => Some((0x9a1aa0, 0x9a1a00)),
            _ => None,
        };
        if let Some((with_venue, plain)) = mark_win {
            if self.won {
                return self.say_at(with_venue, plain, Nothing, OwnFirst);
            }
            return self.say_mark_defeat();
        }
        match self.verdict {
            tie_verdict::SHOCK if !self.won => {
                if self.single_match {
                    self.say_at(0x9a1638, 0x9a15a0, Round, OpponentFirst)
                } else {
                    self.say(0x9a16ec, Round, OpponentFirst, false)
                }
            }
            // A shock win, an excellent and a fine result: (single match with/without the venue, two legs).
            tie_verdict::SHOCK | tie_verdict::EXCELLENT | tie_verdict::FINE => {
                let (with_venue, plain, legs) = match self.verdict {
                    tie_verdict::SHOCK => (0x9a1818, 0x9a178c, 0x9a18c0),
                    tie_verdict::EXCELLENT => (0x9a1458, 0x9a13c8, 0x9a1504),
                    _ => (0x9a1290, 0x9a1204, 0x9a1334),
                };
                if self.single_match {
                    self.say_at(with_venue, plain, Nothing, OwnFirst)
                } else {
                    self.say(legs, Nothing, OwnFirst, false)
                }
            }
            tie_verdict::DISAPPOINTING | tie_verdict::VERY_DISAPPOINTING => self.say_verdict_defeat(),
            _ if self.won => {
                if self.single_match {
                    self.say_at(0x9a10d4, 0x9a1050, Nothing, OwnFirst)
                } else {
                    self.say(0x9a1174, Nothing, OwnFirst, false)
                }
            }
            _ => self.say_verdict_defeat(),
        }
    }

    /// The tie that sends the club through to the named next stage, or the defeat (next stage 1 or above).
    pub(crate) unsafe fn say_through_to_stage(&mut self, own_mark: i8, opponent_mark: i8) {
        use TieMiddle::{NextStage, Round};
        use TieScore::{OpponentFirst, OwnFirst};
        covered!("gra_24 through to a named stage");
        if own_mark < 0 || opponent_mark < 0 {
            if !self.won {
                return self.say_forfeit_lost(own_mark, opponent_mark);
            }
            covered!("gra_24 forfeit through");
            let format = if own_mark < opponent_mark { 0x9a252c } else { 0x9a2494 };
            return self.say(format, NextStage, TieScore::Nothing, false);
        }
        match self.mark {
            // Away goals and the higher seed: no venue variant; the defeat is the plain sentence.
            tie_mark::AWAY_GOALS => {
                if self.won {
                    self.say(0x9a3ce4, NextStage, OwnFirst, false)
                } else {
                    self.say(0x9a2294, Round, OwnFirst, false)
                }
            }
            tie_mark::HIGHER_SEED => {
                if self.won {
                    self.say(0x9a3c30, NextStage, OwnFirst, false)
                } else {
                    self.say(0x9a2070, Round, OwnFirst, false)
                }
            }
            tie_mark::EXTRA_TIME | tie_mark::PENALTIES | tie_mark::SHOOTOUT | tie_mark::GOLDEN_GOAL => {
                if !self.won {
                    return self.say_mark_defeat();
                }
                let (with_venue, plain) = match self.mark {
                    tie_mark::EXTRA_TIME => (0x9a3b70, 0x9a3acc),
                    tie_mark::PENALTIES => (0x9a38ac, 0x9a380c),
                    tie_mark::SHOOTOUT => (0x9a369c, 0x9a35fc),
                    _ => (0x9a3480, 0x9a33d4),
                };
                self.say_at(with_venue, plain, NextStage, OwnFirst)
            }
            _ => match self.verdict {
                tie_verdict::SHOCK if self.won => {
                    if self.single_match {
                        self.say_at(0x9a3284, 0x9a31ec, NextStage, OwnFirst)
                    } else {
                        self.say(0x9a3334, NextStage, OwnFirst, false)
                    }
                }
                tie_verdict::SHOCK => {
                    if self.single_match {
                        self.say_at(0x9a308c, 0x9a2fe0, Round, OpponentFirst)
                    } else {
                        self.say(0x9a3148, Round, OpponentFirst, false)
                    }
                }
                // An excellent and a fine result, whatever the outcome: a single match is "have won in the <round>",
                // two legs "are through to <stage>".
                tie_verdict::EXCELLENT => {
                    if self.single_match {
                        self.say_at(0x9a2e84, 0x9a2de8, Round, OwnFirst)
                    } else {
                        self.say(0x9a2f38, NextStage, OwnFirst, false)
                    }
                }
                tie_verdict::FINE => {
                    if self.single_match {
                        self.say_at(0x99f3bc, 0x9a2cb4, Round, OwnFirst)
                    } else {
                        self.say(0x9a2d48, NextStage, OwnFirst, false)
                    }
                }
                tie_verdict::DISAPPOINTING if !self.single_match => self.say(0x9a2c0c, Round, OpponentFirst, false),
                tie_verdict::DISAPPOINTING | tie_verdict::VERY_DISAPPOINTING => self.say_verdict_defeat(),
                _ if self.won => {
                    if self.single_match {
                        self.say_at(0x9a2808, 0x9a2778, NextStage, OwnFirst)
                    } else {
                        self.say(0x9a28b4, NextStage, OwnFirst, false)
                    }
                }
                _ => self.say_verdict_defeat(),
            },
        }
    }
}

/// The full names of the round, group and stage ids (form 0 of `comp_util_get_stage_name`): id and the langlib format
/// string the original prints, sorted by id (the text in the comment).
pub(crate) const STAGE_NAMES_FULL: [(i16, usize); 322] = [
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
pub(crate) const STAGE_NAMES_SHORT: [(i16, usize); 322] = [
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
pub(crate) const STAGE_NAMES_PLURAL: [(i16, usize); 321] = [
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
pub(crate) const COMP_UTIL_SOURCE: usize = 0x99ec20;

/// What a cup tie sentence names after the club (with its gender pair before the club's).
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum TieMiddle {
    /// The round label and its gender.
    Round,
    /// The competition's name and gender ("have won the <competition>"); the label then becomes the competition's
    /// name, for its link.
    Trophy,
    /// The competition's name with the round label's gender; the label then becomes the competition's name.
    TrophyRoundGender,
    /// The competition's name and gender, the label kept (a final settled by a forfeit).
    TrophyKeepLabel,
    /// The next stage's name and gender.
    NextStage,
    /// Nothing ("are through to the next stage").
    Nothing,
}

/// The goals in a cup tie sentence.
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum TieScore {
    /// The club's goals, then the opponent's.
    OwnFirst,
    /// The opponent's goals, then the club's; the score link's text is rewritten the same way.
    OpponentFirst,
    /// No goals (a forfeit).
    Nothing,
}

/// Writes a date name into one of the shared name buffers and returns the buffer: the langlib template (its extra
/// argument is the empty string, which only the templates with a translator's comment read), or the empty string when
/// there is none.
pub(crate) unsafe fn write_date_name(buffer: usize, template: Option<usize>) -> usize {
    match template {
        Some(t) => story_lang!(buffer, t, EMPTY_STRING),
        None => copy_c_string(buffer, EMPTY_STRING),
    }
    buffer
}

/// qsort order of two club pointers by their last season: a club without a last division goes after one with it (two
/// without are equal); between two with one, the division of higher reputation first, then the higher competition id,
/// then the better (lower) last position, by the differences of those values.
/// Original: `compare_clubs_by_last_division` (0x4c0a40, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_clubs_by_last_division(a: *const usize, b: *const usize) -> i32 {
    let (ca, cb) = (Club(read_usize(a as usize)), Club(read_usize(b as usize)));
    let (da, db) = (ca.last_division(), cb.last_division());
    match (da.is_null(), db.is_null()) {
        (true, true) => 0,
        (true, false) => 1,
        (false, true) => -1,
        (false, false) => {
            let by_reputation = db.reputation() as i32 - da.reputation() as i32;
            if by_reputation != 0 {
                return by_reputation;
            }
            let by_id = read_u32(db.addr()).wrapping_sub(read_u32(da.addr())) as i32;
            if by_id != 0 {
                return by_id;
            }
            covered!("cx01_9 same last division");
            ca.last_position() as i8 as i32 - cb.last_position() as i8 as i32
        }
    }
}
