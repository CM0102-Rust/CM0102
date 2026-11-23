use super::*;

/// Rule: the Polish League Cup takes 36 clubs in 2001 and 34 afterwards, all class 0: the Second Division clubs that
/// came up from below (or have no last division), then the other Second Division clubs, then the First Division clubs
/// placed below 7th last season, then those placed up to 7th, each in database order until the list is full; returns
/// 1 when it is full.
///
/// `team_count` = 36 when the cup's year (+0x40) is 2001, else 34; the list (`exe::crt_malloc` of 6 × team_count; out
/// of memory: the box of pol_lge_cup.cpp at line 0x168 and −1). Four passes over the clubs (a 16-bit index up to
/// [`G_CLUB_COUNT`]), each adding at the next entry while the count is below `team_count`: division id of
/// `KEY_CLUB_COMP_POLISH_SECOND_DIVISION` with no `club::LAST_DIVISION` or one whose id is neither the First nor the
/// Second Division's; the same division with a last division that is one of them; division id of
/// `KEY_CLUB_COMP_POLISH_FIRST_DIVISION` with `club::LAST_POSITION` (a signed byte) above 7; and at most 7. Class and
/// status 0. Returns count == `team_count`.
/// Original: `PolLgeCup::collect_entrants` (0x7cad30, pol_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_lge_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let mut k = 0i32;
    this.set_team_count(if read_i16(this.addr() + cup::YEAR) == 0x7d1 { 0x24 } else { 0x22 });
    let l = exe::crt_malloc((this.team_count() as i32 * 6) as u32);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0xa785d4, 0x168);
        return -1;
    }
    covered!("mwd_30 polish league cup entrants");
    for pass in 0..4 {
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i = 0i16;
        while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
            let d = read_usize(club + club::DIVISION);
            let fits = if pass < 2 {
                d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_POLISH_SECOND_DIVISION) && k < this.team_count() as i32 && {
                    let last = read_usize(club + club::LAST_DIVISION);
                    let upper = last != 0 && (read_usize(last) == read_usize(KEY_CLUB_COMP_POLISH_FIRST_DIVISION) || read_usize(last) == read_usize(KEY_CLUB_COMP_POLISH_SECOND_DIVISION));
                    if pass == 0 { !upper } else { upper }
                }
            } else {
                d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_POLISH_FIRST_DIVISION) && k < this.team_count() as i32 && {
                    let pos = read_i8(club + club::LAST_POSITION);
                    if pass == 2 { pos > 7 } else { pos < 8 } // mwd30: the First Division, the lower half first
                }
            };
            if fits {
                set_cup_entry(this, k as usize, club, 0);
                k += 1;
            }
            i = i.wrapping_add(1);
            club = club.wrapping_add(club::SIZE);
        }
    }
    (k == this.team_count() as i32) as i32
}

/// Comparer for `pol_lge_cup_collect_entrants` (side by side, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_pol_lge_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_POL_LGE_CUP_COLLECT_ENTRANTS);
    compare_entrants_with_seeds("pol_lge_cup_collect_entrants", Cup(this), || o(this), || pol_lge_cup_collect_entrants(this), |_| true)
}

/// Rule: the Polish League Cup at career creation: discipline rule 0x1f and, with no saved state, its entrants, a
/// 400,000 prize fund, the rounds (7 substitutes and 3 substitutions set after them), the fixture tree, the statistics
/// and the predicted positions (`construct_east_nation_cup`).
/// Original: `PolLgeCup::PolLgeCup` (0x7ca4e0, pol_lge_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_pol_lge_cup(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &POLISH_LEAGUE_CUP, "mwd_18 polish league cup new")
}

/// Comparer for `construct_pol_lge_cup`.
pub(crate) unsafe extern "thiscall" fn compare_construct_pol_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_POL_LGE_CUP);
    compare_east_nation_cup_ctor("construct_pol_lge_cup", Cup(this), year, record, &POLISH_LEAGUE_CUP, || o(this, year, record))
}

/// Rule: the Polish League Cup season turnover, as the Croatian Cup's, with its entrants (`PolLgeCup::collect_entrants`
/// 0x7cad30; lines 0x1f3, 0x1fa, 0x201 of pol_lge_cup.cpp).
/// Original: `PolLgeCup::start_new_season` (0x7cafe0, pol_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_lge_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x7cad30, 0xa785d4, [0x1f3, 0x1fa, 0x201], 1, "mwd_9 polish league cup turnover")
}

/// Comparer for `pol_lge_cup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_pol_lge_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POL_LGE_CUP_START_NEW_SEASON);
    compare_cup_new_season("pol_lge_cup_start_new_season", Cup(this), || o(this))
}

/// The Polish League Cup: discipline rule 0x1f, 400,000 prize fund, the substitutes written after the round table.
pub(crate) const POLISH_LEAGUE_CUP: EastNationCup = EastNationCup {
    vtable: 0x96e414,
    discipline: 0x1f,
    entrants: 0x7cad30,
    prize: Some(0x61a80),
    late_substitutes: true,
    stage_vector: false,
    rank: true,
    file: 0xa785d4,
    rows: [0x3b, 0x4a, 0x51],
};
