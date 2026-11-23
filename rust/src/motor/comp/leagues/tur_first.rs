use super::*;

/// Rule: the Turkish First Division: 2 legs, 3 points a win and 1 a draw; tie-breaks head to head, goal difference,
/// goals for, wins; nobody goes up, 3 go down (`turkish_league_rules`).
/// Original: `TurFirst::set_up_rules` (0x8fdf90, tur_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_first_set_up_rules(this: usize) -> u32 {
    covered!("mwd_14 turkish first division rules");
    eng_prm_apply_league_rules(League(this), &tur_first_rules())
}

/// Comparer for `tur_first_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_tur_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TUR_FIRST_SET_UP_RULES);
    eng_prm_compare_league_rules("tur_first_set_up_rules", League(this), tur_first_rules(), || o(this), || tur_first_set_up_rules(this))
}

/// Rule: the Turkish turnover, led by the First Division: the season reopened (+0x4c = 0); promotion and relegation
/// (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`: push 1, `tur_first_process_promotion_relegation_chain`);
/// the ten best clubs of the lowest division up into the second division category B and its relegated clubs down
/// (`tur_first_promote_lower_clubs_to_third_tier`, ecx = this); the third tier reshuffle (0x8fe580, ecx = this, not
/// ported); the First Division rebuilt for the next year (`rebuild_turkish_league`, lines 0x213, 0x21a, 0x220, 0x227);
/// the division tier of the clubs of both second divisions capped (League::limit_club_division_tier(1, 3, −3, 1),
/// category A then B) and both turned over (virtual `comp_vtable::START_NEW_SEASON`); then the predicted positions, the
/// board expectations and the officials (`finish_croatian_turnover`).
/// Original: `TurFirst::start_new_season` (0x8fe040, tur_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this.addr(), 1);
    for f in [0x8fe3a0usize, 0x8fe580] {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
        f(this.addr());
    }
    covered!("mwd_24 turkish first division turnover");
    if !rebuild_turkish_league(this, 0x8fddc0, 0xaa1840, [0x213, 0x21a, 0x220, 0x227]) {
        return 0;
    }
    let (a, b) = (comp_by_global(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A), comp_by_global(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B));
    exe::league_limit_club_division_tier(a, 1, 3, -3, 1);
    exe::league_limit_club_division_tier(b, 1, 3, -3, 1);
    bra_nat_first_call_virtual_no_args(a, comp_vtable::START_NEW_SEASON);
    bra_nat_first_call_virtual_no_args(b, comp_vtable::START_NEW_SEASON);
    finish_croatian_turnover(this)
}

/// Rule: Turkish promotion and relegation: the final positions of the First Division and both second divisions are
/// recorded (virtual `league_vtable::RECORD_FINAL_POSITIONS`); then the marked clubs are swapped between the First
/// Division and the second division category A (in pairs), and between category A and category B and each of
/// category B's four groups (its stages 0..3; as a block).
///
/// League::promote_and_relegate (`exe::league_promote_and_relegate`(this, above, below, 1, pairs, −1, −1): 6 pushes ->
/// thiscall, ret 0x18): First Division / category A with pairs 1, category A / category B with 0, then category A /
/// each of category B's stages `[+0xc][0..3]` with 0. Returns the last call's eax; the argument is not read.
/// Original: `TurFirst::process_promotion_relegation_chain` (0x8fe250, tur_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn tur_first_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    let (a, b) = (comp_by_global(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A), comp_by_global(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B));
    bra_nat_first_call_virtual_no_args(this, league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(a, league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(b, league_vtable::RECORD_FINAL_POSITIONS);
    covered!("mwd_24 turkish promotion and relegation");
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION), a, 1, 1, -1, -1); // mwd24: the top flight and category A
    let mut r = exe::league_promote_and_relegate(this, a, b, 1, 0, -1, -1);
    for k in 0..4 { // mwd24: the four groups of category B
        r = exe::league_promote_and_relegate(this, a, read_usize(read_usize(b + league::STAGES) + 4 * k), 1, 0, -1, -1);
    }
    r
}

/// Comparer for `tur_first_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_tur_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TUR_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("tur_first_start_new_season", League(this), || o(this))
}

/// Comparer for `tur_first_process_promotion_relegation_chain`: only the original (it moves clubs, as the Brazilian
/// chain's comparer).
pub(crate) unsafe extern "thiscall" fn compare_tur_first_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_TUR_FIRST_PROCESS_PROMOTION_RELEGATION_CHAIN)(this, pairs)
}

/// Rule: the Turkish First Division has 18 clubs, those of the top flight (`KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION`) in
/// database order; when full, no prize fund (prize mode 2) and TV pays 1,000,000 to each club. In 2001 it also moves
/// the clubs of the five category B groups into category B itself, keeping the group in the club's reserve division.
///
/// `club_count` = 18; in 2001, for the first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`] clubs whose national division
/// (`club::DIVISION`) has the id of one of `TURKISH_CATEGORY_B_GROUPS`: `club::RESERVE_DIVISION` = that division and
/// `club::DIVISION` = the record of category B (`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B`). Then the table
/// (`fill_league_table_rows`: the box at line 0x1a4); full: +0x28 = 0, `prize_mode` = 2 and League::pay_tv_rights (push
/// 0; push 1000000) and 1; otherwise 0.
/// Original: `TurFirst::create_table` (0x8fddc0, tur_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_first_create_table(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(0x12);
    if l.year() == 0x7d1 {
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i = 0i32;
        while i < (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2)) {
            let d = read_usize(club + club::DIVISION);
            if d != 0 && TURKISH_CATEGORY_B_GROUPS.iter().any(|&g| read_usize(d) == read_usize(g)) {
                covered!("mwd_25 turkish category b groups merged");
                write_usize(club + club::RESERVE_DIVISION, d);
                write_usize(club + club::DIVISION, comp_record_of(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B)); // mwd25: into category B
            }
            i += 1;
            club = club.wrapping_add(club::SIZE);
        }
    }
    if !fill_league_table_rows(l, 0x12, 0xaa1840, 0x1a4, KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION) {
        return 0;
    }
    l.set_prize_fund(0);
    l.set_prize_mode(2);
    eng_prm_pay_tv_rights(this, 0xf4240, 0); // mwd25: the Turkish top flight TV money
    1
}

/// Rule: at the Turkish turnover, the ten best clubs of the lowest division (`KEY_CLUB_COMP_TURKISH_LOWER_DIVISION`,
/// sorted by strength with the best 30 shuffled) go up into the second division category B, and the clubs of status 3
/// in category B and in its four group stages go down to the lowest division, their reserve division cleared; with
/// fewer than 10 candidates nobody moves.
///
/// A list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; out of memory: the box at line 0x27c and return) with the clubs
/// that have a nation (`club::NATION` set, not compared) and whose division is the lowest one, in database order
/// (16-bit index). Fewer than 10: the list freed and return. Otherwise `exe::comp_util_sort_and_shuffle_top_clubs`(list,
/// n, 10), the first ten moved to category B (League::move_club_to_division(this, club, its record, 1)); then for
/// category B (g = −1) and its stages g = 0..3 (`[+0xc][g]`), each row of status 3 goes down
/// (League::relegate_club_to_division(this, club, the lowest division's record, 1)) and that club's
/// `club::RESERVE_DIVISION` = 0. The list is freed.
/// Original: `tur_first_promote_lower_clubs_to_third_tier` (0x8fe3a0, tur_first.cpp, thiscall: ecx = the First Division, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_first_promote_lower_clubs_to_third_tier(this: usize) {
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0xaa1840, 0x27c);
        return;
    }
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(club + club::NATION) != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_TURKISH_LOWER_DIVISION) {
            write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
            n = n.wrapping_add(1);
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if (n as u16 as i16) < 10 {
        exe::crt_free(list);
        return;
    }
    covered!("mwd_25 turkish lowest division up");
    exe::comp_util_sort_and_shuffle_top_clubs(list, n, 10); // mwd25: the best thirty shuffled
    for k in 0..10 {
        exe::league_move_club_to_division(this, read_usize(list + 4 * k), comp_record_of(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B), 1);
    }
    let b = League(comp_by_global(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B));
    for g in -1i8..4 {
        let stage = League(if g == -1 { b.addr() } else { read_usize(b.stages() + 4 * g as usize) });
        let mut i = 0i32;
        while i < stage.club_count() as i32 {
            let row = stage.table() + i as usize * league_table_row::SIZE;
            if read_u8(row + league_table_row::STATUS) == 3 {
                exe::league_relegate_club_to_division(this, read_usize(row), comp_record_of(KEY_CLUB_COMP_TURKISH_LOWER_DIVISION), 1);
                write_usize(read_usize(stage.table() + i as usize * league_table_row::SIZE) + club::RESERVE_DIVISION, 0);
            }
            i += 1;
        }
    }
    exe::crt_free(list);
}

/// Comparer for `tur_first_create_table`: only the original runs (it moves clubs in 2001); the planned top-flight clubs
/// (`compare_division_table`) and, when full, prize mode 2 with no fund.
pub(crate) unsafe extern "thiscall" fn compare_tur_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TUR_FIRST_CREATE_TABLE);
    let r = compare_division_table("tur_first_create_table", League(this), 0x12, KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION, || o(this));
    if r == 1 {
        let l = League(this);
        SCOREBOARD.record(l.prize_fund() == 0 && l.prize_mode() == 2, || format!("tur_first_create_table({:#x}) prize fund {} mode {}", this, l.prize_fund(), l.prize_mode()));
    }
    r
}

/// Comparer for `tur_first_promote_lower_clubs_to_third_tier`: only the original runs (it draws); with at least 10
/// candidates, the clubs of status 3 in category B and its four groups must end in the lowest division with no reserve
/// division.
pub(crate) unsafe extern "thiscall" fn compare_tur_first_promote_lower_clubs_to_third_tier(this: usize) {
    let candidates = (0..read_usize(G_CLUB_COUNT)).filter(|&i| {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let d = read_usize(c + club::DIVISION);
        d != 0 && read_usize(c + club::NATION) != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_TURKISH_LOWER_DIVISION)
    }).count();
    let mut planned: Vec<usize> = Vec::new();
    let b = League(comp_by_global(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B));
    if candidates >= 10 {
        for g in -1i32..4 {
            let s = League(if g == -1 { b.addr() } else { read_usize(b.stages() + 4 * g as usize) });
            if s.addr() != 0 && s.table() != 0 {
                for i in 0..s.club_count() as usize {
                    let row = s.table() + i * league_table_row::SIZE;
                    if read_u8(row + league_table_row::STATUS) == 3 {
                        planned.push(read_usize(row));
                    }
                }
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TUR_FIRST_PROMOTE_LOWER_CLUBS_TO_THIRD_TIER)(this);
    let lower = comp_record_of(KEY_CLUB_COMP_TURKISH_LOWER_DIVISION);
    let wrong: Vec<usize> = planned.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != lower || read_usize(c + club::RESERVE_DIVISION) != 0).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("tur_first_promote_lower_clubs_to_third_tier({:#x}): relegated clubs not in the lowest division {:x?}", this, wrong));
}

/// Rule: the Turkish First Division at career creation: discipline rule 0x25 and the division constructor template
/// (`construct_division_league`).
/// Original: `TurFirst::TurFirst` (0x8fcf60, tur_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_tur_first(this: usize, year: u32, record: usize) -> usize {
    covered!("mwd_12 turkish first division");
    construct_division_league(League(this), year, record, &TURKISH_FIRST_DIVISION)
}

/// Comparer for `construct_tur_first` (only the original runs; `compare_division_league_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_tur_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_TUR_FIRST);
    compare_division_league_ctor("construct_tur_first", League(this), year, record, &TURKISH_FIRST_DIVISION, || o(this, year, record))
}

/// The rules of the two Turkish divisions: 2 legs, 3 points a win and 1 a draw; tie-breaks 4, 1, 2, 3 (head to head,
/// goal difference, goals for, wins); `up` go up directly (+0xbe), none to play-offs, 3 go down (+0xc1); the given
/// leagues above and below; +0x52 = 2; 7 substitutes, 3 substitutions; no rule bits set.
pub(crate) unsafe fn turkish_league_rules(up: u32, above: u32, below: u32) -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 4), (0xc6, 1, 1), (0xc7, 1, 2),
        (0xc8, 1, 3), (0xbe, 1, up), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 3), (0x1c, 4, above), (0x4a, 1, 3), (0x52, 1, 2), (0x20, 4, below), (0x49, 1, 7)]
}

/// The Turkish First Division: nobody goes up, nothing above, the division of id [0x9cf8d0] below.
pub(crate) unsafe fn tur_first_rules() -> Vec<(usize, u8, u32)> {
    turkish_league_rules(0, 0xffff_ffff, read_usize(KEY_CLUB_COMP_TURKISH_UNMATCHED_A) as u32)
}

/// Fills a league table with the clubs of the competition of id [`g`] in database order until it is full, as
/// `eng_prm_fill_league_table` but without the prize fund: `club_count` = n, the table (out of memory: the box at
/// `row` and false), League::init_table_row(this, k, club, 0, 0) for each club; true when full.
pub(crate) unsafe fn fill_league_table_rows(this: League, n: i16, file: usize, row: u32, g: usize) -> bool {
    this.set_club_count(n);
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x3b));
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
        return false;
    }
    let mut k: u8 = 0;
    let mut i: i32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let comp = read_usize(club + club::DIVISION);
        if comp != 0 && read_usize(comp) == read_usize(g) {
            exe::league_init_table_row(this, k as u32, club, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == this.club_count() {
                break;
            }
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    k as i8 as i16 == this.club_count()
}

/// The id global of the division below the Turkish top flight (0x9cf8d0, unnamed in the community map:
/// `g_key_club_comp_turkish_unmatched_a`, docs/chaves_uso.csv).
pub(crate) const KEY_CLUB_COMP_TURKISH_UNMATCHED_A: usize = 0x9cf8d0;

/// The five groups of the Turkish second division category B in the database (`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G1`
/// to `_G5`).
pub(crate) const TURKISH_CATEGORY_B_GROUPS: [usize; 5] = [KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G1, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G2,
    KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G3, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G4, KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G5];

/// The Turkish First Division (the top flight): discipline rule 0x25 (the Turkish one).
pub(crate) const TURKISH_FIRST_DIVISION: LeagueDivision = LeagueDivision {
    vtabela: 0x97052c,
    discipline: 0x25, // mwd12: the Turkish discipline rule
    rules: 0x8fdf90,
    table: 0x8fddc0,
    file: 0xaa1840,
    rows: [0x48, 0x4f, 0x56, 0x5f, 0x66],
    low: false,
};
