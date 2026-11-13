// ---------- batch %%LEVA%%: the Portuguese tables and the Liga de Honra turnover (por_prm.cpp, por_second.cpp) ----------

/// Creates the Portuguese Premier Division table: 18 clubs, those of the division in database order; the prize fund by
/// a share of 100,000 per place and 1,000,000 of TV money to every club.
///
/// `PorPrm::create_table()`: the body of `eng_prm_fill_league_table` (checked instruction by instruction):
/// `.set_club_count()` = 18, the table (`exe::crt_malloc` of 0x426; out of memory: the box of por_prm.cpp (0xa78724)
/// line 0x1a8 and 0); the clubs of `KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION` in database order
/// (`exe::league_init_table_row`) until full; not full: 0. Full: the prize fund (`exe::league_calc_prize_fund`, push
/// 100,000), `.set_prize_mode()` = 1, the TV money (`exe::league_pay_tv_rights`: push 0; push 1,000,000 -> thiscall,
/// ret 8, to every club) and 1.
/// Original: `PorPrm::create_table` (0x7cf800, por_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_prm_create_table(this: usize) -> u32 {
    covered!("mwe_8 portuguese premier table");
    eng_prm_fill_league_table(League(this), 0x12, 0xa78724, 0x1a8, KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION, 0x186a0, || {
        eng_prm_pay_tv_rights(this, 0xf4240, 0); // mwe8: the Portuguese Premier TV money
    })
}

/// Creates the Liga de Honra table: 18 clubs, those of the division in database order; the B team of a Premier
/// Division club cannot go up (status −4); the prize fund by a share of 50,000 per place; no TV money.
///
/// `PorSecond::create_table()`: the body of `spa_second_create_table` (checked instruction by instruction):
/// `.set_club_count()` = 18, the table (`exe::crt_malloc` of 0x426; out of memory: the box of por_second.cpp
/// (0xa7875c) line 0x16c and 0); in database order (`G_CLUB_TABLE`, a signed word counter up to `G_CLUB_COUNT`), the
/// clubs of `KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION` (`spa_lower_is_in_comp`), each into the next row
/// (`exe::league_init_table_row`: push 0; push 0; push club; push row, a stack dword with the row byte (the routine
/// reads the byte) -> thiscall, ret 0x10); a B team of a Premier Division club (`is_b_team_of_comp_club`:
/// `exe::club_get_linked_b_team`(club, &flag, 1), cdecl, linked and flag 0, the linked club in
/// `KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION`) gets `league_table_row::STATUS` = 0xfc
/// (`table_status::PROMOTION_INELIGIBLE`); until full. Not full: 0. Full: the prize fund
/// (`exe::league_calc_prize_fund`, push 50,000), `.set_prize_mode()` = 1 and 1.
/// Original: `PorSecond::create_table` (0x7d1520, por_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_second_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x12);
    let t = exe::crt_malloc(0x426);
    this.set_table(t);
    if t == 0 {
        show_error_at(0xa7875c, 0x16c);
        return 0;
    }
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let base = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = base + (i as u16 as usize) * club::SIZE;
        if spa_lower_is_in_comp(Club(c), KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION) {
            exe::league_init_table_row(this, k as u32, c, 0, 0);
            if is_b_team_of_comp_club(c, KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION) { // mwe8: a Premier Division B team
                covered!("mwe_8 b team in the liga de honra");
                write_u8(this.table() + (k as i8 as isize as usize).wrapping_mul(league_table_row::SIZE) + league_table_row::STATUS, 0xfc);
            }
            k = k.wrapping_add(1);
            if k as i8 as i16 == this.club_count() {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    if k as i8 as i16 != this.club_count() {
        return 0;
    }
    covered!("mwe_8 liga de honra table");
    let f = exe::league_calc_prize_fund(this, 0xc350); // mwe8: the Liga de Honra prize share
    this.set_prize_fund(f);
    this.set_prize_mode(1);
    1
}

/// The Liga de Honra turnover: archives the season in the statistics, clears the table, the round dates and the
/// stages, moves to the next year and rebuilds the rules, the table, the fixtures and the order; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// `PorSecond::start_new_season()`: `league::SEASON_CLOSED` = 0 and the body of `rebuild_season` (checked instruction
/// by instruction: `exe::league_free_table`(0), the round dates freed, `exe::comp_stats_archive_season_and_reset`(1),
/// the stages 0..=current destroyed by their virtual 0 with 1, the year + 1, the current stage −1, the rules by virtual
/// `comp_vtable::SET_UP_RULES`, `PorSecond::create_table` (0x7d1520), `exe::league_create_fixtures`,
/// `exe::league_sort_table`(0), each with the box of por_second.cpp (0xa7875c) at 0x1e4, 0x1eb, 0x1f1, 0x1f8, and the
/// predicted positions by virtual `comp_vtable::SET_PREDICTED_POSITIONS`); then `set_expectations_and_renew_officials`
/// (League::set_club_season_expectations and OfficialsManager::renew_officials(record), whose eax is returned).
/// After a box: 0.
/// Original: `PorSecond::start_new_season` (0x7d1760, por_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x7d1520, 0xa7875c, [0x1e4, 0x1eb, 0x1f1, 0x1f8], 1) { // mwe8: the Liga de Honra next year
        return 0;
    }
    covered!("mwe_8 liga de honra turnover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `por_prm_create_table`: only the original runs (it allocates and pays), against the plan of
/// `eng_prm_compare_league_table`.
pub(crate) unsafe extern "thiscall" fn compare_por_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    eng_prm_compare_league_table("por_prm_create_table", League(this), 0x12, KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION, 0x186a0, || o(this))
}

/// Comparer for `por_second_create_table`: only the original runs (it allocates); the plan (the division's clubs in
/// database order, up to 18, with the B team mark) checked against the rows, the count and the return value, and the
/// prize fund when full.
pub(crate) unsafe extern "thiscall" fn compare_por_second_create_table(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let base = read_usize(G_CLUB_TABLE);
    let mut plan: Vec<(usize, bool)> = Vec::new();
    for i in 0..(read_usize(G_CLUB_COUNT) as i32).max(0) as usize {
        let c = base + i * club::SIZE;
        if spa_lower_is_in_comp(Club(c), KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION) {
            plan.push((c, is_b_team_of_comp_club(c, KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION)));
            if plan.len() == 0x12 {
                break;
            }
        }
    }
    let r = o(this.addr());
    let t = this.table();
    let mut ok = r == (plan.len() == 0x12) as u32 && this.club_count() == 0x12 && t != 0;
    if ok {
        for (k, &(c, b)) in plan.iter().enumerate() {
            let l = LeagueTableRow(t + k * league_table_row::SIZE);
            ok = ok && read_usize(l.addr()) == c && (!b || l.status() == table_status::PROMOTION_INELIGIBLE);
        }
        if r == 1 {
            ok = ok && this.prize_mode() == 1 && this.prize_fund() == exe::league_calc_prize_fund(this, 0xc350);
        }
    }
    SCOREBOARD.record(ok, || format!("por_second_create_table({:#x}): returned {}, plan {:x?}", this.addr(), r, plan));
    r
}

/// Comparer for `por_second_start_new_season`: only the original runs (it frees and allocates); the year must go up
/// by one (`compare_state_league_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_por_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_state_league_new_season("por_second_start_new_season", League(this), || o(this))
}
