// ---------- rx09_2: the Spanish Second Division B group 1 table, turnover and next stage (spa_second_b.cpp) ----------

/// The ids of the four Second Division B groups, in group order.
pub(crate) const SEGUNDA_B_GROUPS: [usize; 4] = [KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B2, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B3,
    KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B4];

/// The first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`] clubs (the count read again at each step, as the original
/// does), in database order.
pub(crate) unsafe fn domestic_clubs() -> Vec<Club> {
    let mut v = Vec::new();
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2)) {
        v.push(Club(club));
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    v
}

/// The clubs of a Second Division B group, in database order: those with a division (`club::DIVISION`) and a reserve
/// division (`club::RESERVE_DIVISION`, here the group) whose ids are the Second Division B's
/// ([`KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B`]) and the group's (`group`, a key).
pub(crate) unsafe fn segunda_b_group_clubs(group: usize) -> Vec<Club> {
    domestic_clubs().into_iter().filter(|c| {
        let division = read_usize(c.addr() + club::DIVISION);
        let reserve = read_usize(c.addr() + club::RESERVE_DIVISION);
        division != 0 && reserve != 0 && read_usize(reserve) == read_usize(group) && read_usize(division) == read_usize(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B)
    }).collect()
}

/// In 2001 the database puts the Second Division B clubs in their group's competition: each club whose division's id
/// is one of the four groups (`SEGUNDA_B_GROUPS`) gets that group as its reserve division (`club::RESERVE_DIVISION`)
/// and this league's record as its division.
pub(crate) unsafe fn move_segunda_b_groups(l: League) {
    for c in domestic_clubs() {
        let division = read_usize(c.addr() + club::DIVISION);
        if division != 0 && SEGUNDA_B_GROUPS.iter().any(|&g| read_usize(division) == read_usize(g)) {
            write_usize(c.addr() + club::RESERVE_DIVISION, division);
            write_usize(c.addr() + club::DIVISION, l.competition().addr()); // rx09_2: the group club joins the Second Division B
        }
    }
}

/// Builds the table of group 1 of the Spanish Second Division B: `club_count` = 20; in 2001 (the word
/// `league::YEAR`) the clubs are first moved into the league (`move_segunda_b_groups`); the table (`exe::crt_malloc` of
/// 0x3b per club; out of memory: the box at line 0x1cf and 0) and a row (League::init_table_row
/// `exe::league_init_table_row`(this, k, club, 0, 0)) for each club of group 1 (`segunda_b_group_clubs`). A 21st club
/// shows the box at line 0x1e0, loses its division (`club::DIVISION` = 0) and ends the scan. Returns 1 when the table
/// is full, else 0.
/// Original: `SpaSecondB::load_group_clubs` (0x856cd0, spa_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_second_b_load_group_clubs(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(0x14);
    if l.year() == 0x7d1 {
        move_segunda_b_groups(l);
    }
    let t = exe::crt_malloc((l.club_count() as i32 * league_table_row::SIZE as i32) as u32);
    l.set_table(t);
    if t == 0 {
        show_error_at(SEGUNDA_B_SOURCE, 0x1cf);
        return 0;
    }
    covered!("rx09_2 segunda b group 1 table");
    let mut k: u8 = 0;
    for c in segunda_b_group_clubs(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1) {
        if (k as i8 as i16) >= l.club_count() {
            show_error_at(SEGUNDA_B_SOURCE, 0x1e0);
            write_usize(c.addr() + club::DIVISION, 0);
            break;
        }
        exe::league_init_table_row(l, k as u32, c, 0, 0); // rx09_2: the row of the k-th club
        k = k.wrapping_add(1);
    }
    ((k as i8 as i16) == l.club_count()) as u32
}

/// Comparer for `spa_second_b_load_group_clubs`: only the original runs (it allocates the table and moves the clubs in
/// 2001); afterwards no club is left in a group's competition, the club count is 20, and the rows and the result
/// match group 1's clubs as they then stand.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_load_group_clubs(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    let l = League(this);
    let plan: Vec<usize> = segunda_b_group_clubs(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1).iter().take(20).map(|c| c.addr()).collect();
    let left = domestic_clubs().iter().filter(|c| {
        let d = read_usize(c.addr() + club::DIVISION);
        d != 0 && SEGUNDA_B_GROUPS.iter().any(|&g| read_usize(d) == read_usize(g))
    }).count();
    let found: Vec<usize> = if l.table() != 0 { (0..plan.len()).map(|k| aus_nsl_table_row_club(l, k)).collect() } else { Vec::new() };
    let same = r == (plan.len() == 20) as u32 && l.club_count() == 20 && found == plan && (l.year() != 0x7d1 || left == 0);
    SCOREBOARD.record(same, || format!("spa_second_b_load_group_clubs({:#x}) original={} plan={:x?} table={:x?} left in groups {}", this, r, plan, found, left));
    r
}

/// The turnover of a league split into regional groups (the Turkish category B mould): the season reopened, League::
/// free_table (`exe::league_free_table`(0)), the round dates freed and cleared; with statistics,
/// `exe::comp_stats_archive_season_and_reset`(1); with a current stage ≥ 0, the stages 0..=current (re-read on each
/// step) destroyed by their virtual 0 with 1; the year + 1 and the current stage −1. Then, each with the box of `file`
/// at its line (`lines`) and 0 returned when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table
/// routine (thiscall, ret 0), League::sort_table (`exe::league_sort_table`(0)) and League::create_fixtures. Then the
/// group stage routine (thiscall(k), ret 4) for k = 0..`groups`, the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`) and OfficialsManager::renew_officials on `G_OFFICIALS_MANAGER` with the
/// record, whose eax is returned.
pub(crate) unsafe fn grouped_league_new_season(this: League, table: usize, group_stage: usize, groups: u32, file: usize, lines: [u32; 4], branch: &'static str) -> u32 {
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    let dates = this.round_dates();
    if dates != 0 {
        exe::crt_free(dates);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    this.set_year(this.year().wrapping_add(1)); // rx09_2: the next season
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, lines[0]);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if table(this.addr()) == 0 {
        show_error_at(file, lines[1]);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, lines[2]);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, lines[3]);
        return 0;
    }
    covered!(branch);
    let group_stage: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(group_stage);
    for k in 0..groups {
        group_stage(this.addr(), k);
    }
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr()) as u32
}

/// Rule: the Spanish Second Division B turnover: last season's stages destroyed, the next year, the rules, group 1's
/// table, the order and the fixtures, then groups 2 to 4, the predicted positions and the officials
/// (`grouped_league_new_season` with SpaSecondB::load_group_clubs 0x856cd0 and SpaSecondB::create_group_stage 0x857210,
/// boxes at lines 0x23b, 0x242, 0x249 and 0x24f).
/// Original: `SpaSecondB::start_new_season` (0x856f90, spa_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_second_b_start_new_season(this: usize) -> u32 {
    grouped_league_new_season(League(this), 0x856cd0, 0x857210, 3, SEGUNDA_B_SOURCE, [0x23b, 0x242, 0x249, 0x24f], "rx09_2 segunda b turnover")
}

/// Comparer for `spa_second_b_start_new_season`: only the original runs (it frees and rebuilds the league); the year
/// must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("spa_second_b_start_new_season", League(this), || o(this))
}

/// Rule: the next stage of the Spanish Second Division B: past the last stage, the error box; otherwise the current
/// stage goes up by one; on reaching stage 3 (the groups are over) the four promotion play-off groups and the
/// relegation play-off are built, on reaching stage 8 the relegation play-off final.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box at line 0x26d and nothing
/// else. Otherwise the current stage + 1; at 3, SpaSecondB::create_promotion_playoff_groups (0x857570) and
/// SpaSecondB::create_relegation_playoff (0x857860); at 8, SpaSecondB::create_relegation_playoff_final (0x857a00) (all
/// thiscall, ret 0).
/// Original: `SpaSecondB::advance_stage` (0x857150, spa_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_second_b_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(SEGUNDA_B_SOURCE, 0x26d);
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    covered!("rx09_2 segunda b next stage");
    if stage == 3 {
        call_on_league(0x857570, l);
        call_on_league(0x857860, l);
    } else if stage == 8 {
        call_on_league(0x857a00, l);
    }
}

/// Comparer for `spa_second_b_advance_stage` (`compare_advance_stage_building`: stages 3 and 8 build).
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_advance_stage_building("spa_second_b_advance_stage", League(this), |s| s == 3 || s == 8, || o(this), || spa_second_b_advance_stage(this))
}
