use super::*;

/// Rule: the German League Cup (Ligapokal) takes 6 clubs: the Bundesliga's top six of last season minus the German Cup
/// holder, who enters as the sixth; entries 4 and 5 (the best-placed one after the holder, and the holder) are class 1.
///
/// `team_count` = 6. A local list of 6 slots, zeroed; over the clubs (up to [`G_CLUB_COUNT`] read once) while fewer
/// than 6 are counted: a club whose `club::LAST_DIVISION` is the Bundesliga's record (`KEY_CLUB_COMP_GERMAN_FIRST_DIVISION`)
/// and whose `club::LAST_POSITION` (a signed byte) is 1..6 goes to slot position − 1 (the count goes up even when a
/// slot is overwritten). Short of 6: the box of ger_lge_cup.cpp at line 0x11d, then over the clubs (the count read
/// again) while short of 6: a German club (nation id `KEY_NATION_GERMANY`) of the same last division with a last
/// position above 6 goes to the first empty slot (the count goes up even when none is empty). The list
/// (`exe::crt_calloc`(team_count, 6) into `cup::TEAM_TABLE`; out of memory: the box at line 0x13d and −1). Entry 5 =
/// the latest winner of the German Cup (`exe::comp_util_get_latest_placed_club`(record of `KEY_CLUB_COMP_GERMAN_CUP`,
/// 0)); none: the box at line 0x146 and 0. Then entries 4, 3, 2, 1, 0 each take the first slot that is set and is not
/// the holder (the slot is cleared); none: the entry stays empty. Class and status 0 for all, class 1 for entries 4
/// and 5. Returns 1.
/// Original: `GerLgeCup::collect_entrants` (0x5dc700, ger_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_lge_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let mut slots = [0usize; 6];
    this.set_team_count(6);
    let first = comp_record_of(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION);
    let mut n: i16 = 0;
    let count = read_usize(G_CLUB_COUNT) as i32;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < count && n < 6 {
        let pos = read_i8(club + club::LAST_POSITION);
        if read_usize(club + club::LAST_DIVISION) == first && pos < 7 && pos > 0 {
            slots[(pos - 1) as usize] = club;
            n += 1;
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if n != 6 {
        show_error_at(0x9cd450, 0x11d);
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i = 0i32;
        while i < read_usize(G_CLUB_COUNT) as i32 && n < 6 {
            let nation = read_usize(club + club::NATION);
            if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_GERMANY) && read_usize(club + club::LAST_DIVISION) == first && read_i8(club + club::LAST_POSITION) > 6 {
                if let Some(s) = slots.iter_mut().find(|s| **s == 0) {
                    *s = club;
                }
                n += 1;
            }
            i += 1;
            club = club.wrapping_add(club::SIZE);
        }
    }
    let l = exe::crt_calloc(this.team_count() as i32 as u32, 6);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0x9cd450, 0x13d);
        return -1;
    }
    write_usize(l + 0x1e, exe::comp_util_get_latest_placed_club(comp_record_of(KEY_CLUB_COMP_GERMAN_CUP), 0));
    if read_usize(this.team_table() + 0x1e) == 0 {
        show_error_at(0x9cd450, 0x146);
        return 0;
    }
    covered!("mwc_23 german league cup entrants");
    let mut e = 4i32;
    while e >= 0 {
        if let Some(s) = slots.iter_mut().find(|s| **s != 0 && **s != read_usize(this.team_table() + 0x1e)) {
            write_usize(this.team_table() + 6 * e as usize, *s);
            *s = 0;
        }
        e -= 1;
    }
    for k in 0..6usize {
        write_u8(this.team_table() + 6 * k + 4, 0);
        write_u8(this.team_table() + 6 * k + 5, 0);
        if k >= 4 { // mwc23: the holder and the best placed after him seeded
            write_u8(this.team_table() + 6 * k + 4, 1);
        }
    }
    1
}

/// Comparer for `ger_lge_cup_collect_entrants` (side by side, every entry compared).
///
/// The holder comes from `comp_util_get_latest_placed_club` (0x4afd80), which re-sorts the global competition history
/// table (`[0xae23d4]`, `[0xae237c]` records of 0x1a bytes) with the CRT's unstable qsort on every call: run twice, the
/// second call can see records of equal key in another order. So the copy runs on the history as the original saw it,
/// and the original's history is put back after it (310th batch's real game: `list false` on the six entries).
pub(crate) unsafe extern "thiscall" fn compare_ger_lge_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_GER_LGE_CUP_COLLECT_ENTRANTS);
    let hist = || (read_usize(0xae23d4), read_usize(0xae237c) * 0x1a);
    let snap = |(p, n): (usize, usize)| if p == 0 { Vec::new() } else { core::slice::from_raw_parts(p as *const u8, n).to_vec() };
    let put = |p: usize, v: &[u8]| if p != 0 && !v.is_empty() { core::ptr::copy_nonoverlapping(v.as_ptr(), p as *mut u8, v.len()) };
    let before = core::cell::RefCell::new(Vec::new());
    let after = core::cell::RefCell::new(Vec::new());
    let lists = core::cell::RefCell::new((Vec::new(), Vec::new()));
    let entries = |t: usize| (0..6).map(|k| if t == 0 { 0 } else { read_usize(t + 6 * k) }).collect::<Vec<_>>();
    let r = compare_entrants_with_seeds("ger_lge_cup_collect_entrants", Cup(this), || {
        *before.borrow_mut() = snap(hist());
        let r = o(this);
        *after.borrow_mut() = snap(hist());
        lists.borrow_mut().0 = entries(Cup(this).team_table());
        r
    }, || {
        put(hist().0, &before.borrow());
        let r = ger_lge_cup_collect_entrants(this);
        lists.borrow_mut().1 = entries(Cup(this).team_table());
        put(hist().0, &after.borrow());
        r
    }, |_| true);
    let l = lists.borrow();
    if l.0 != l.1 {
        linha(&format!("engine: ger_lge_cup_collect_entrants lists original {:x?} rust {:x?}", l.0, l.1));
    }
    r
}

/// At the season turn the German League Cup is rebuilt: last edition's tables freed, the history archived, the year +1,
/// the entrants (GerLgeCup::collect_entrants), the rounds, the tie tree and the predicted positions.
///
/// The knockout-cup turnover template (`usa_mls_all_stars_cup_start_new_season`), checked instruction by instruction
/// against `JapSuper::start_new_season`: only the entrants routine (`GerLgeCup::collect_entrants`, thiscall, ret 0),
/// the source file and the box lines (0x1ac, 0x1b3, 0x1ba) change.
/// Original: `GerLgeCup::start_new_season` (0x5dc9e0, ger_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_lge_cup_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x5dc700, 0x9cd450, [0x1ac, 0x1b3, 0x1ba], 1, true, "mwc_15 ger lge cup turnover") // mwc15: the German League Cup
}

/// Comparer for `ger_lge_cup_start_new_season`: only the original runs; the year goes up by one
/// (`compare_cup_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_ger_lge_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GER_LGE_CUP_START_NEW_SEASON);
    compare_cup_new_season("ger_lge_cup_start_new_season", Cup(this), || o(this))
}

/// The German League Cup is born with discipline rule 13; with no saved state, the entrants
/// (GerLgeCup::collect_entrants), the round table, the tie tree (a failure is a negative return), the statistics and
/// the predicted positions (`construct_nation_cup`).
/// Original: `GerLgeCup::GerLgeCup` (0x5dc0e0, ger_lge_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ger_lge_cup(this: usize, year: u32, record: usize) -> usize {
    construct_nation_cup(Cup(this), year, record, &GER_LGE_CUP_CUP_PARAMS)
}

/// Comparer for `construct_ger_lge_cup`: only the original runs (`compare_nation_cup_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_ger_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_GER_LGE_CUP);
    compare_nation_cup_ctor("construct_ger_lge_cup", Cup(this), year, record, &GER_LGE_CUP_CUP_PARAMS, || o(this, year, record))
}

pub(crate) const GER_LGE_CUP_CUP_PARAMS: NationCup = NationCup {
    vtable: 0x96b1b0,
    bench: None,
    discipline: discipline_rule::GERMANY_LEAGUE, // mwc17: the German League Cup discipline rule
    entrants: 0x5dc700,
    zero_fails: false,
    predicted: true,
    source_file: 0x9cd450,
    box_lines: [0x3f, 0x4d, 0x54],
};
