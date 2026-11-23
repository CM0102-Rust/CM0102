use super::*;

/// The Scottish Cup at career creation: 50 entrants (eight non-league clubs, the Second and Third Divisions, the First
/// and the Premier), 7 rounds from November to May, a prize fund of 1,000,000, 5 substitutes named and 3 allowed, the
/// discipline rule of the Scottish cups (0x19); then the fixture tree, the statistics and the predicted positions
/// (`construct_british_cup`, the mould of mwb_1).
/// Original: `ScoFaCup::ScoFaCup` (0x7ed310, sco_fa_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_sco_fa_cup(this: usize, year: u32, record: usize) -> usize {
    construct_british_cup(Cup(this), year, record, &SCOTTISH_CUP, "mwb_2 new scottish cup")
}

/// Comparer for `construct_sco_fa_cup` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_sco_fa_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SCO_FA_CUP);
    compare_british_cup_ctor("construct_sco_fa_cup", Cup(this), year, record, &SCOTTISH_CUP, || o(this, year, record))
}

/// rule: the Scottish Cup has 7 rounds from November to May: 1st round drawn 8/11, played 5/12 (16 clubs), 2nd 6/12 and
/// 2/1 (20: 8 winners and 12 entering), 3rd 3/1 and 29/1 (32: 10 winners and the 22 of the Premier and the First), 4th
/// 30/1 and 19/2, quarter-finals 20/2 and 11/3, semi-finals 12/3 and 8/4 at a neutral ground, final drawn 9/4 and played
/// on 27/5 at the national stadium; replays up to the semi-finals; category 4 then 6; the round winners get 3,000, none,
/// 6,000, 10,000, 20,000, 70,000 and 150,000. Outside stage −1 it writes nothing and returns 0; out of memory, the box
/// of line 0xa7 and 0 (`eng_cc_cup_create_single_stage_round_table`).
/// Original: `ScoFaCup::create_round_table` (0x7ed5d0, sco_fa_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn sco_fa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &SCOTTISH_CUP_ROUNDS, 0xa7adac, 0xa7, "mwb_2 scottish cup rounds")
}

/// Comparer for `sco_fa_cup_create_round_table` (side by side, the written rounds).
pub(crate) unsafe extern "thiscall" fn compare_sco_fa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SCO_FA_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("sco_fa_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| sco_fa_cup_create_round_table(this, stage, r, e, s))
}

/// rule: the Scottish Cup has 50 entrants: eight non-league Scottish clubs (sorted by reputation, the eight picks
/// shuffled), the Second and Third Divisions (all shuffled together with them: they enter in the first two rounds), the
/// First Division (shuffled among themselves: they enter in the second and third rounds) and the Premier Division (the
/// third round); returns 1 with the list full, 0 when clubs are missing and −1 out of memory
/// Original: `ScoFaCup::collect_entrants` (0x7edaf0, sco_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_fa_cup_collect_entrants(this: usize) -> i32 {
    scottish_cup_entrants(Cup(this)).0
}

/// Comparer for `sco_fa_cup_collect_entrants`: side by side with the random generator's state (the copy's draws not
/// counted), the return, the count and the list.
pub(crate) unsafe extern "thiscall" fn compare_sco_fa_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SCO_FA_CUP_COLLECT_ENTRANTS);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend_from_slice(&[(this.addr() + cup::TEAM_COUNT, 2), (G_MOUSE_LEFT_BUTTON_DOWN, 4)]);
    eng_cc_cup_compare_entrant_list("sco_fa_cup_collect_entrants", this, &reg, || o(this.addr()), || crate::rng::without_counting(|| scottish_cup_entrants(this)),
        |_, n| n.clamp(0, 0x32) as usize * 6)
}

/// The Scottish Cup turnover: the same template, with the Scottish Cup entrants (`ScoFaCup::collect_entrants`) and the
/// error-box lines 0x20f, 0x216 and 0x21d of sco_fa_cup.cpp.
/// Original: `ScoFaCup::start_new_season` (0x7ede10, sco_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_fa_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x7edaf0, 0xa7adac, [0x20f, 0x216, 0x21d], 1, "mwb_4 scottish cup turnover") // mwb4: the next Scottish Cup
}

/// Comparer for `sco_fa_cup_start_new_season`: only the original runs; the year went up by one.
pub(crate) unsafe extern "thiscall" fn compare_sco_fa_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_FA_CUP_START_NEW_SEASON);
    compare_cup_new_season("sco_fa_cup_start_new_season", Cup(this), || o(this))
}

/// The Scottish Cup entrants, in the original routine's order.
///
/// `cup::TEAM_COUNT` = 50 and the table (`exe::crt_malloc(0x12c)`; out of memory, the box of line 0x153 and −1).
///
/// A local list of up to 30 clubs: first the Scottish non-league clubs (nation Scotland, not a national side
/// `exe::database_is_national_team`, division the Scottish lower league `KEY_CLUB_COMP_A_LOWER_DIVISION`), in database
/// order while the list has fewer than 30 (the test comes before each club); sorted by reputation with 8 picks
/// shuffled to the top (`exe::comp_util_sort_and_shuffle_top_clubs(list, count, 8)`, cdecl, the count as a word). The
/// list goes back to 8 and the clubs of the Third and Second Divisions are added after them (while fewer than 30 and
/// `cup::TEAM_COUNT` > 0); the whole list is shuffled (`exe::comp_util_shuffle_pointer_array(list, count)`, cdecl) and
/// copied into the table (class and status 0).
///
/// Then the First Division clubs, in database order, while the table has room (stops when full), and 20 swaps among the
/// last ten entries: each swaps the clubs of entries (k − 1 − random(10)) and (k − 1 − random(10)), k the count so far
/// (bytes, `exe::utils_random`, two draws per swap, the first for the first entry). Finally the Premier Division clubs
/// while there is room. Returns (1 with the list full, else 0; the count).
pub(crate) unsafe fn scottish_cup_entrants(this: Cup) -> (i32, i32) {
    this.set_team_count(0x32);
    let l = exe::crt_malloc(0x12c);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0xa7adac, 0x153);
        return (-1, 0);
    }
    let count = || read_usize(G_CLUB_COUNT) as i32;
    let mut list = [0usize; 30];
    let mut m: i8 = 0;
    let mut i: i16 = 0;
    while (i as i32) < count() && m < 0x1e {
        let c = eng_cc_cup_club_by_index(i);
        let nation = read_usize(c + club::NATION);
        if nation != 0 && exe::database_is_national_team(c) == 0 && read_usize(nation) == read_usize(KEY_NATION_SCOTLAND)
            && club_division_id(c) == Some(read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION)) {
            list[m as usize] = c;
            m += 1;
        }
        i = i.wrapping_add(1);
    }
    exe::comp_util_sort_and_shuffle_top_clubs(list.as_mut_ptr() as usize, m as i16 as u16 as u32, 8); // mwb2: the eight non-league clubs
    let mut m: i8 = 8;
    let mut i: i16 = 0;
    while (i as i32) < count() && m < 0x1e {
        let c = eng_cc_cup_club_by_index(i);
        let d = club_division_id(c);
        if (d == Some(read_usize(KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION)) || d == Some(read_usize(KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION))) && this.team_count() > 0 {
            list[m as usize] = c;
            m += 1;
        }
        i = i.wrapping_add(1);
    }
    let shuffled = m as i16;
    exe::comp_util_shuffle_pointer_array(list.as_mut_ptr() as usize, shuffled as u16 as u32);
    let mut k: i32 = 0;
    for (j, &c) in list.iter().enumerate().take(shuffled.max(0) as usize) {
        put_cup_entrant(this, j as i32, c);
        k = j as i32 + 1;
    }
    let mut i: i16 = 0;
    while (i as i32) < count() {
        let c = eng_cc_cup_club_by_index(i);
        if club_division_id(c) == Some(read_usize(KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION)) && k < this.team_count() as i32 {
            put_cup_entrant(this, k, c);
            k += 1;
            if k == this.team_count() as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    for _ in 0..0x14 { // mwb2: the swaps among the last ten
        let a = (k as u8).wrapping_sub(exe::utils_random(10) as u8).wrapping_sub(1) as i8 as i32;
        let b = (k as u8).wrapping_sub(exe::utils_random(10) as u8).wrapping_sub(1) as i8 as i32;
        let t = this.team_table();
        let x = read_usize(t + a as usize * 6);
        write_usize(t + a as usize * 6, read_usize(t + b as usize * 6));
        write_usize(t + b as usize * 6, x);
    }
    let mut i: i16 = 0;
    while (i as i32) < count() {
        let c = eng_cc_cup_club_by_index(i);
        if club_division_id(c) == Some(read_usize(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION)) && k < this.team_count() as i32 {
            put_cup_entrant(this, k, c);
            k += 1;
            if k == this.team_count() as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    covered!("mwb_2 scottish cup entrants");
    ((k == this.team_count() as i32) as i32, k)
}

/// The id of the club's division (`club::DIVISION` → its competition record → the id); `None` without one.
pub(crate) unsafe fn club_division_id(c: usize) -> Option<usize> {
    let c = Club(c);
    let d = c.division().addr();
    if d == 0 { None } else { Some(read_usize(d)) }
}

/// Writes club `c` with class and status 0 into entry `k` of the cup's team table (6-byte entries).
pub(crate) unsafe fn put_cup_entrant(this: Cup, k: i32, c: usize) {
    let e = this.team_table() + k as usize * 6;
    write_usize(e, c);
    write_u8(e + 4, 0);
    write_u8(e + 5, 0);
}

/// The Scottish Cup: 5 named substitutes and 3 substitutions, discipline rule 0x19, a prize fund of 1,000,000.
pub(crate) const SCOTTISH_CUP: BritishCupSpec = BritishCupSpec {
    vtable: 0x96f414,
    bench: Some((5, 3)),
    discipline: 0x19,
    prize_fund: 0xf4240,
    entrants: 0x7edaf0,
    file: 0xa7adac,
    rows: [0x4d, 0x5c, 0x63],
};

/// The seven rounds of the Scottish Cup: the first drawn on 8 November and played on 5 December (16 teams, 8 ties: the
/// 16 lowest entrants), the second on 6/12 and 2/1 (20 teams: the 8 winners and 12 entering from the 16th place), the
/// third on 3/1 and 29/1 (32 teams: 10 winners and 22 entering from the 28th: the Premier and the First), the fourth on
/// 30/1 and 19/2, the quarter-finals on 20/2 and 11/3, the semi-finals on 12/3 and 8/4 at a neutral ground (venue 0x28)
/// and the final drawn on 9/4 and played on 27 May at the national stadium (venue 4); replays allowed (+0x20 = 1) up to
/// the semi-finals, tie-break code 3 on a replay, and in the final code 3 on the day; category 4 up to the fourth round,
/// 6 after; the prize to the winner of each round: 3,000, none, 6,000, 10,000, 20,000, 70,000 and 150,000.
pub(crate) const SCOTTISH_CUP_ROUNDS: [EngCcCupCupRound; 7] = [
    cql10r((8, 0xa, 0, 6), (5, 0xb, 0, 5, 1, 0), 0xa, (0, 0, 3), 4, (0x10, 8, 0x10, 0), (1, 1, 0), (0, 0xbb8, 0)),
    cql10r((6, 0xb, 0, 6), (2, 0, 1, 5, 1, 0), 0x14, (0, 0, 3), 4, (0x14, 0xa, 0xc, 0x10), (1, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((3, 0, 1, 6), (0x1d, 0, 1, 5, 1, 0), 0x1e, (0, 0, 3), 4, (0x20, 0x10, 0x16, 0x1c), (1, 1, 0), (0, 0x1770, 0)),
    cql10r((0x1e, 0, 1, 6), (0x13, 1, 1, 5, 1, 0), 0x28, (0, 0, 3), 4, (0x10, 8, 0, 0), (1, 1, 0), (0, 0x2710, 0)),
    cql10r((0x14, 1, 1, 6), (0xb, 2, 1, 5, 1, 0), 0x32, (0, 0, 3), 6, (8, 4, 0, 0), (1, 1, 0), (0, 0x4e20, 0)), // mwb2: the Scottish Cup quarter-finals
    cql10r((0xc, 2, 1, 6), (8, 3, 1, 5, 1, 0x28), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (1, 1, 0), (0, 0x11170, 0)),
    cql10r((9, 3, 1, 6), (0x1b, 4, 1, 5, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0x249f0, 0)),
];
