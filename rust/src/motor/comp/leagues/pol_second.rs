use super::*;

/// Rule: the Polish Second Division turnover (called by the First Division's): the season reopened, last season's
/// stages destroyed and the league rebuilt for the next year (`rebuild_season` with PolSecond::create_table at 0x7cbdf0
/// and the boxes of pol_second.cpp at lines 0x204, 0x20b, 0x211, 0x218; the predicted positions), then the board
/// expectations and the officials (`set_expectations_and_renew_officials`), whose eax is returned. After a box: 0.
/// Original: `PolSecond::start_new_season` (0x7cc000, pol_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x7cbdf0, 0xa7860c, [0x204, 0x20b, 0x211, 0x218], 1) {
        return 0;
    }
    covered!("mwd_29 polish second division turnover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `pol_second_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_pol_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POL_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("pol_second_start_new_season", League(this), || o(this))
}

/// Rule: the Polish Second Division: 2 up directly and 2 to the promotion play-off, 6 down in 2001 and 4 afterwards
/// (`pol_second_rules`).
/// Original: `PolSecond::set_up_rules` (0x7cbf40, pol_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_second_set_up_rules(this: usize) -> u32 {
    covered!("mwd_28 polish second division rules");
    eng_prm_apply_league_rules(League(this), &pol_second_rules(this))
}

/// Comparer for `pol_second_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_pol_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POL_SECOND_SET_UP_RULES);
    eng_prm_compare_league_rules("pol_second_set_up_rules", League(this), pol_second_rules(this), || o(this), || pol_second_set_up_rules(this))
}

/// Rule: the Polish Second Division has 20 clubs in 2001 and 18 afterwards, those of the division
/// (`KEY_CLUB_COMP_POLISH_SECOND_DIVISION`) in database order; when full, the prize fund (20,000 a share); no TV money.
///
/// `club_count` = 20, games per club not computed (`league::GAMES_PER_CLUB` = −1), `club_count` = 18 unless the year is
/// 2001; then `eng_prm_fill_league_table` (the box at line 0x199; League::calc_prize_fund push 20000, `prize_mode` 1)
/// with no TV payment.
/// Original: `PolSecond::create_table` (0x7cbdf0, pol_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_second_create_table(this: usize) -> u32 {
    let this = League(this);
    let l = this;
    l.set_club_count(0x14);
    this.set_games_per_club(0xffffu16 as i16);
    if l.year() != 0x7d1 {
        l.set_club_count(0x12);
    }
    let n = l.club_count();
    eng_prm_fill_league_table(l, n, 0xa7860c, 0x199, KEY_CLUB_COMP_POLISH_SECOND_DIVISION, 0x4e20, || { covered!("mwd_16 polish second division table"); }) // mwd16: the Polish prize share
}

/// Comparer for `pol_second_create_table` (only the original runs; the planned clubs and the prize fund checked).
pub(crate) unsafe extern "thiscall" fn compare_pol_second_create_table(this: usize) -> u32 {
    let n = if League(this).year() == 0x7d1 { 0x14 } else { 0x12 };
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POL_SECOND_CREATE_TABLE);
    eng_prm_compare_league_table("pol_second_create_table", League(this), n, KEY_CLUB_COMP_POLISH_SECOND_DIVISION, 0x4e20, || o(this))
}

/// The rules of the Polish Second Division: as the First Division's, with 2 up directly (+0xbe) and 2 to the
/// promotion play-off (+0xbf), 6 down in 2001 (20 clubs) and 4 afterwards (+0xc1); the First Division above, nothing
/// below; +0xd9 = 3.
pub(crate) unsafe fn pol_second_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let down = if League(this).year() == 0x7d1 { 6 } else { 4 }; // mwd28: six down in 2001
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 4), (0xc6, 1, 1), (0xc7, 1, 2),
        (0xbe, 1, 2), (0xbf, 1, 2), (0xc0, 1, 0), (0xc1, 1, down), (0x52, 1, 0), (0x1c, 4, read_usize(KEY_CLUB_COMP_POLISH_FIRST_DIVISION) as u32),
        (0x4a, 1, 3), (0xd9, 2, 3), (0x20, 4, 0xffff_ffff), (0x49, 1, 7)]
}
