// ---------- sbE_11: the Regionalliga steps of the German turnover (ger_first.cpp) ----------

/// The German clubs that may go up into the Regionalliga: the clubs (index up to `injury_club_count_without_nations`,
/// read again at each step) of `KEY_NATION_GERMANY` (`club::NATION`) with no division or in the lower division
/// (`KEY_CLUB_COMP_A_LOWER_DIVISION`), except those with a partner (`exe::club_get_linked_b_team`(club, &flag, 1), cdecl)
/// and the flag 0 whose division is missing, the Regionalliga's or the lower division's; at most 200.
unsafe fn german_lower_division_candidates() -> Vec<usize> {
    let mut v = Vec::new();
    let mut flag: u32 = 0;
    let mut i = 0i32;
    while i < injury_club_count_without_nations() {
        let c = club_at(i as isize);
        let nation = read_usize(c.addr() + club::NATION);
        let d = c.division().addr();
        if nation != 0 && read_u32(nation) == read_u32(KEY_NATION_GERMANY) && (d == 0 || read_u32(d) == read_u32(KEY_CLUB_COMP_A_LOWER_DIVISION)) {
            let partner = exe::club_get_linked_b_team(c, &mut flag, 1);
            let pd = if partner != 0 { read_usize(partner + club::DIVISION) } else { 0 };
            let kept_out = partner != 0 && flag == 0
                && (pd == 0 || read_u32(pd) == read_u32(KEY_CLUB_COMP_GERMAN_REGIONAL) || read_u32(pd) == read_u32(KEY_CLUB_COMP_A_LOWER_DIVISION));
            if !kept_out {
                v.push(c.addr());
                if v.len() == 200 {
                    break;
                }
            }
        }
        i += 1;
    }
    v
}

/// Rule: at the German turnover with the Regionalliga loaded, the relegated clubs of both its sections go down to the
/// lower division, and as many German lower division clubs (one fewer when the Regionalliga has 19 clubs), sorted by
/// strength with the best ones shuffled, go up into the Regionalliga; with too few candidates, the box and nothing goes
/// up.
///
/// `german_lower_division_candidates`; then for the Regionalliga (`KEY_CLUB_COMP_GERMAN_REGIONAL`) and its stage 0, each
/// row with `league_table_row::STATUS` 3 goes down (League::relegate_club_to_division
/// `exe::league_relegate_club_to_division`(this, club, the lower division's record, 1)), one more counted, and its
/// club's `club::RESERVE_DIVISION` = 0. A Regionalliga of 19 clubs counts one fewer. Fewer candidates than the count
/// (16-bit): the ger_first.cpp (0x9cd414) box at line 0x3d8 and return. Otherwise
/// `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, count) and the first candidates move up
/// (League::move_club_to_division(this, club, the Regionalliga's record, 1)) while the count lasts.
/// Original: `ger_first_exchange_regional_lower_division_clubs` (0x5db980, ger_first.cpp, thiscall: ecx = the
/// Bundesliga, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_first_exchange_regional_lower_division_clubs(this: usize) {
    let mut list = german_lower_division_candidates();
    let mut down: i16 = 0;
    for group in [-1i8, 0] {
        let regional = League(comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL));
        let g = if group == -1 { regional } else { League(read_usize(regional.stages() + 4 * group as usize)) };
        let mut k: i16 = 0;
        while k < g.club_count() {
            let row = LeagueTableRow(g.table() + k as usize * league_table_row::SIZE);
            if row.status() == 3 {
                exe::league_relegate_club_to_division(this, row.club() as usize, comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION), 1);
                down += 1;
                write_usize(row.club() as usize + club::RESERVE_DIVISION, 0); // sbE_11: out of its section
            }
            k += 1;
        }
    }
    if League(comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL)).club_count() == 0x13 {
        down -= 1;
    }
    let n = list.len() as i16;
    if n < down {
        return show_error_at(0x9cd414, 0x3d8);
    }
    covered!("sbE_11 german lower division up");
    exe::comp_util_sort_and_shuffle_top_clubs(list.as_mut_ptr() as usize, n as u32, down as u16 as u32);
    for &club in list.iter() {
        if down == 0 {
            break;
        }
        exe::league_move_club_to_division(this, club, comp_record_of(KEY_CLUB_COMP_GERMAN_REGIONAL), 1);
        down -= 1;
    }
}

/// Comparer for `ger_first_exchange_regional_lower_division_clubs`: only the original runs (it draws and moves clubs);
/// the Regionalliga's relegated rows must end in the lower division.
pub(crate) unsafe extern "thiscall" fn compare_ger_first_exchange_regional_lower_division_clubs(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let regional = League(comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL));
    let wrong: Vec<usize> = (0..regional.club_count().max(0) as usize).map(|k| regional.table() + k * league_table_row::SIZE)
        .filter(|&row| read_u8(row + league_table_row::STATUS) == 3 && read_usize(read_usize(row) + club::DIVISION) != comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION))
        .map(|row| read_usize(row)).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("ger_first_exchange_regional_lower_division_clubs({:#x}) relegated clubs not in the lower division: {:x?}", this, wrong));
}

/// Rule: a Regionalliga B team whose senior club goes down from the 2. Bundesliga is marked relegated too (a B team may
/// not play in the same division as its senior club), with the override news.
///
/// For each row k of the 2. Bundesliga (`KEY_CLUB_COMP_GERMAN_SECOND_DIVISION`, 16-bit index, the count read again):
/// with a partner (`exe::club_get_linked_b_team`(club, &flag, 1), cdecl) and the flag not 0 whose division is the
/// Regionalliga, its row in the section of its `club::RESERVE_DIVISION` (read without a null check): the Regionalliga's
/// table for `KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_NORTH`, its stage 0 for `_SOUTH` (League::find_club_row(partner,
/// 0)); for any other section the row of the previous turn is kept (at first the stage 0 object itself: a defect of the
/// original). No row: the ger_first.cpp box at line 0x4d2. When the senior club's row has `league_table_row::STATUS` 3
/// and the partner's row not, the partner's row gets 3 and the Regionalliga sends the override news
/// (Comp::send_promotion_relegation_override_news `exe::comp_send_promotion_relegation_override_news`(the Regionalliga;
/// the 2. Bundesliga's competition, the partner, the club, 1)). The object it is called on is not read.
/// Original: `ger_first_relegate_b_teams_of_relegated_clubs` (0x5dbf50, ger_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_first_relegate_b_teams_of_relegated_clubs(_this: usize) {
    let second = League(comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION));
    let regional = League(comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL));
    let south = read_usize(regional.stages());
    let mut found = south;
    let mut flag: u32 = 0;
    let mut k: i16 = 0;
    while k < second.club_count() {
        let row = second.table() + k as usize * league_table_row::SIZE;
        let club = read_usize(row);
        let partner = exe::club_get_linked_b_team(club, &mut flag, 1);
        let pd = if partner != 0 && flag != 0 { read_usize(partner + club::DIVISION) } else { 0 };
        if pd != 0 && read_u32(pd) == read_u32(KEY_CLUB_COMP_GERMAN_REGIONAL) {
            let section = read_u32(read_usize(partner + club::RESERVE_DIVISION));
            if section == read_u32(KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_NORTH) {
                found = exe::league_find_club_row(regional, partner, 0);
            } else if section == read_u32(KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_SOUTH) {
                found = exe::league_find_club_row(south, partner, 0);
            }
            if found == 0 {
                show_error_at(0x9cd414, 0x4d2);
            } else if read_u8(row + league_table_row::STATUS) == 3 && read_u8(found + league_table_row::STATUS) != 3 {
                covered!("sbE_11 german b team relegated");
                write_u8(found + league_table_row::STATUS, 3); // sbE_11: the B team goes down with its senior club
                exe::comp_send_promotion_relegation_override_news(regional, second.competition(), partner, club, 1);
            }
        }
        k += 1;
    }
}

/// Comparer for `ger_first_relegate_b_teams_of_relegated_clubs`: only the original runs (it marks rows and sends news).
pub(crate) unsafe extern "thiscall" fn compare_ger_first_relegate_b_teams_of_relegated_clubs(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}
