// ---------- sbE_2: the Finnish Premier Division constructor and the Liga de Honra exchanged with the zones (fin_prm.cpp, por_prm.cpp) ----------

/// The Finnish Premier Division (Veikkausliiga) as a nation division: no bench written, the Finnish league discipline
/// rule, no stages; FinPrm::set_up_rules and FinPrm::create_table; fin_prm.cpp (0x9c8254) with the boxes of the rules,
/// the table, the fixtures, the order and the statistics.
pub(crate) const FIN_PRM_DIVISION: NationDivision = NationDivision {
    vtable: 0x96a4c0,
    bench: None,
    discipline: discipline_rule::FINLAND_LEAGUE,
    stages: 0,
    set_up_rules: 0x595c90,
    create_table: 0x595b70,
    source_file: 0x9c8254,
    box_lines: [0x59, 0x60, 0x67, 0x70, 0x77],
};

/// The fixed start of a nation division's constructor: League::League (`exe::new_league`, thiscall, ret 0), the
/// competition record (`league::COMPETITION`), the vtable, the year (`league::YEAR`, word), the discipline rule and the
/// class's own fields (`before_load`); then the saved state (League::load_from_temp_file
/// `exe::league_load_from_temp_file`(this, 1), thiscall, ret 4). Returns true when it loaded.
pub(crate) unsafe fn open_nation_division(this: League, year: u32, record: usize, p: &NationDivision, before_load: impl FnOnce()) -> bool {
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), p.vtable);
    this.set_year(year as u16 as i16);
    this.set_discipline_rule(p.discipline);
    before_load();
    exe::league_load_from_temp_file(this, 1) != 0
}

/// The new league of a nation division's constructor: the stage index 0xff, the current stage −1, the stage count and
/// the stage vector (`exe::crt_malloc` of 4 per stage, unchecked; 0 without stages); then the rules, the table (direct
/// calls, thiscall, ret 0), the fixtures (`exe::league_create_fixtures`), the order (`exe::league_sort_table`(this, 0))
/// and the statistics (operator new of 0x5ce, `exe::new_comp_stats`) into `league::COMP_STATS`; the first step that
/// fails shows the box of its line and gives false. The same steps as `construct_nation_division`, for the
/// constructors that do something else between the load and these steps or after them.
pub(crate) unsafe fn build_new_nation_division(this: League, p: &NationDivision) -> bool {
    let set_up_rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.set_up_rules);
    let create_table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.create_table);
    this.set_stage_index(stage_index::MAIN);
    this.set_current_stage(0xffff_ffff);
    this.set_stage_count(p.stages as usize);
    this.set_stages(if p.stages != 0 { exe::crt_malloc(p.stages * 4) } else { 0 });
    let step = |done: bool, k: usize| {
        if !done {
            show_error_at(p.source_file, p.box_lines[k]);
        }
        done
    };
    if !step(set_up_rules(this.addr()) != 0, 0) || !step(create_table(this.addr()) != 0, 1)
        || !step(exe::league_create_fixtures(this) != 0, 2) || !step(exe::league_sort_table(this, 0) != 0, 3) {
        return false;
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(p.source_file, p.box_lines[4]);
        return false;
    }
    true
}

/// Moves into the division of key `target` every club (index up to `injury_club_count_without_nations`, read again at
/// each step) whose division (`club::DIVISION`) has the id of one of the `groups` keys: its old division is kept as its
/// reserve division (`club::RESERVE_DIVISION`) and the target's competition record becomes its division.
pub(crate) unsafe fn move_group_clubs_to_division(groups: &[usize], target: usize) {
    let mut i = 0i32;
    while i < injury_club_count_without_nations() {
        let c = club_at(i as isize);
        let d = c.division();
        if !d.is_null() && groups.iter().any(|&g| read_u32(d.addr()) == read_u32(g)) {
            c.set_reserve_division(d);
            c.set_division(Competition(comp_record_of(target)));
        }
        i += 1;
    }
}

/// Rule: the Finnish Premier Division (Veikkausliiga) at career creation: the Finnish league discipline rule and, with
/// no saved state, the clubs of the Ykkönen's North and South groups gathered into the Ykkönen (their group kept as the
/// reserve division), then the rules, the table, the fixtures, the order, the statistics and the predicted positions.
///
/// `open_nation_division` with `FIN_PRM_DIVISION`; loaded: done. Otherwise `move_group_clubs_to_division` of
/// `KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_NORTH` and `_SOUTH` into `KEY_CLUB_COMP_FINNISH_FIRST_DIVISION`,
/// `build_new_nation_division` and, when it went through, League::set_predicted_positions
/// (`exe::league_set_predicted_positions`). Returns this.
/// Original: `FinPrm::FinPrm` (0x595170, fin_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_fin_prm(this: usize, year: u32, record: usize) -> usize {
    let l = League(this);
    if open_nation_division(l, year, record, &FIN_PRM_DIVISION, || {}) {
        return this;
    }
    covered!("sbE_2 new veikkausliiga");
    move_group_clubs_to_division(&[KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_NORTH, KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_SOUTH], KEY_CLUB_COMP_FINNISH_FIRST_DIVISION); // sbE_2: the Ykkonen groups gathered
    if build_new_nation_division(l, &FIN_PRM_DIVISION) {
        exe::league_set_predicted_positions(l);
    }
    this
}

/// Comparer for `construct_fin_prm`: only the original runs (`compare_nation_division_ctor`); afterwards, in a new
/// league, no club may still be in the Ykkönen's North or South group.
pub(crate) unsafe extern "thiscall" fn compare_construct_fin_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    let loaded = league_will_load_saved_state(record);
    let r = compare_nation_division_ctor("construct_fin_prm", League(this), year, record, &FIN_PRM_DIVISION, || o(this, year, record));
    if !loaded {
        let groups = [read_u32(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_NORTH), read_u32(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_SOUTH)];
        let left = (0..injury_club_count_without_nations()).map(|i| club_at(i as isize)).filter(|c| !c.division().is_null() && groups.contains(&read_u32(c.division().addr()))).count();
        SCOREBOARD.record(left == 0, || format!("construct_fin_prm({:#x}): {} clubs left in the Ykkonen groups", this, left));
    }
    r
}

/// Rule: at the Portuguese turnover without the Second Division B, a B team in the Liga de Honra whose senior club is
/// also there (and not in the Premier Division) is marked relegated with the override news; the Liga de Honra's
/// relegated clubs go down to the North zone, and as many Portuguese clubs of the three zones, sorted by strength with
/// the best ones shuffled, go up to the Liga de Honra (only when there are at least as many candidates).
///
/// The candidates: a list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; none: the box of por_prm.cpp (0xa78724) at line
/// 0x28d and return) with the clubs in database order whose division (`club::DIVISION`) is one of the zones
/// (`KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_NORTH`, `_CENTRAL`, `_SOUTH`) and whose nation (`club::NATION`) is
/// `KEY_NATION_PORTUGAL` (16-bit count). Then each row k of the Liga de Honra (`KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION`,
/// its club count read again at each step; League::get_table_row): with a partner (`exe::club_get_linked_b_team`(club,
/// &flag, 1), cdecl) and the flag 0, the partner in the Liga de Honra and the club itself not in the Premier Division
/// (`KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION`; its division read without a null check), the row's status becomes 3 and
/// the Liga de Honra sends the override news (Comp::send_promotion_relegation_override_news
/// `exe::comp_send_promotion_relegation_override_news`(Liga de Honra, this competition, club, partner, 1), thiscall, ret
/// 0x10); then a row with status 3 counts one more relegated club (a byte) and goes down (League::relegate_club_to_division
/// `exe::league_relegate_club_to_division`(this, club, the North zone's record, 1)). With at least as many candidates
/// as relegated clubs: `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, relegated) and the first `relegated`
/// candidates move up (League::move_club_to_division(this, club, the Liga de Honra's record, 1)). The list is freed.
/// Original: `PorPrm::exchange_second_division_with_zones` (0x7cfd30, por_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_prm_exchange_second_division_with_zones(this: usize) {
    let honra = comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION);
    let list = exe::crt_malloc((read_i32(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        return show_error_at(0xa78724, 0x28d);
    }
    let zones = [KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_NORTH, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_CENTRAL, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_SOUTH];
    let mut n: i16 = 0;
    let mut i = 0i32;
    while i < read_i32(G_CLUB_COUNT) {
        let c = club_at(i as isize);
        let (d, nation) = (c.division().addr(), read_usize(c.addr() + club::NATION));
        if d != 0 && nation != 0 && zones.iter().any(|&z| read_u32(d) == read_u32(z)) && read_u32(nation) == read_u32(KEY_NATION_PORTUGAL) {
            write_usize(list + 4 * n as usize, c.addr());
            n = n.wrapping_add(1);
        }
        i += 1;
    }
    let mut relegated: i8 = 0;
    let mut flag: u32 = 0;
    let mut k = 0i32;
    while k < League(honra).club_count() as i32 {
        let club = read_usize(exe::league_get_table_row(honra, k));
        let partner = exe::club_get_linked_b_team(club, &mut flag, 1);
        if partner != 0 && flag == 0 {
            let pd = read_usize(partner + club::DIVISION);
            if pd != 0 && read_u32(pd) == read_u32(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION)
                && read_u32(read_usize(club + club::DIVISION)) != read_u32(KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION) {
                covered!("sbE_2 portuguese b team down");
                write_u8(exe::league_get_table_row(honra, k) + league_table_row::STATUS, 3);
                exe::comp_send_promotion_relegation_override_news(honra, League(this).competition(), club, partner, 1);
            }
        }
        if read_u8(exe::league_get_table_row(honra, k) + league_table_row::STATUS) == 3 {
            relegated = relegated.wrapping_add(1);
            let down = read_usize(exe::league_get_table_row(honra, k));
            exe::league_relegate_club_to_division(this, down, comp_record_of(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_NORTH), 1); // sbE_2: down to the North zone
        }
        k += 1;
    }
    if n >= relegated as i16 {
        covered!("sbE_2 portuguese zones up");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n as i32 as u32, relegated as i16 as i32 as u32);
        for j in 0..(relegated as i32).max(0) as usize {
            exe::league_move_club_to_division(this, read_usize(list + 4 * j), comp_record_of(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION), 1);
        }
    }
    exe::crt_free(list);
}

/// Comparer for `por_prm_exchange_second_division_with_zones`: only the original runs (it draws and moves clubs); the
/// Liga de Honra's relegated rows must end in the North zone.
pub(crate) unsafe extern "thiscall" fn compare_por_prm_exchange_second_division_with_zones(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let honra = League(comp_by_global(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION));
    let wrong: Vec<usize> = (0..honra.club_count().max(0) as usize).map(|k| honra.table() + k * league_table_row::SIZE)
        .filter(|&row| read_u8(row + league_table_row::STATUS) == 3 && read_usize(read_usize(row) + club::DIVISION) != comp_record_of(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_NORTH))
        .map(|row| read_usize(row)).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("por_prm_exchange_second_division_with_zones({:#x}) relegated clubs not in the North zone: {:x?}", this, wrong));
}
