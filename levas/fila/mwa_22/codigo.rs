// ---------- batch %%LEVA%%: the Irish Super Cup: the table, the daily update and the season turnover (ire_super_cup.cpp) ----------

/// The clubs of the Irish Super Cup table, in the original's order (the plan of the comparer too): three passes over
/// all `G_CLUB_COUNT` clubs taking the Irish clubs (`club::NATION` = `KEY_NATION_REPUBLIC_OF_IRELAND`) entered in the
/// Champions League, then the UEFA Cup, then the Intertoto Cup (`club::CONTINENTAL_COMP_ID`); then, while places
/// remain, the Premier Division clubs (`irish_cup_club_count` clubs, by division) in table order, without checking
/// whether they are already in (a defect of the original: with fewer than four clubs in Europe, a Premier club in
/// Europe enters twice). Up to `n` clubs.
pub(crate) unsafe fn irish_super_cup_clubs(n: i16) -> Vec<usize> {
    let mut v: Vec<usize> = Vec::new();
    for key in [KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, KEY_CLUB_COMP_UEFA_CUP, KEY_CLUB_COMP_INTER_TOTO_CUP] { // mwa22: the European entrants first
        let mut i = 0i16;
        while (i as i32) < read_i32(G_CLUB_COUNT) {
            let c = club_at(i as isize);
            let nation = read_usize(c.addr() + club::NATION);
            if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_REPUBLIC_OF_IRELAND) && c.continental_comp_id() == read_u32(key) && (v.len() as i16) < n {
                v.push(c.addr());
            }
            if v.len() as i16 == n {
                break;
            }
            i = i.wrapping_add(1);
        }
    }
    let mut i = 0i16;
    while (i as i32) < irish_cup_club_count() && (v.len() as i16) < n {
        let c = club_at(i as isize);
        if irish_club_division_id(c) == Some(read_usize(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION)) {
            v.push(c.addr());
        }
        i = i.wrapping_add(1);
    }
    v
}

/// The body of `ire_super_cup_create_table`: `league::CLUB_COUNT` = 4; the table (`exe::crt_malloc` of 4 × 0x3b; out of
/// memory, the error box at line 0xb5 and −1); the clubs of `irish_super_cup_clubs` in rows 0 on (League::init_table_row
/// `exe::league_init_table_row`(this, row, club, 0, 0), thiscall, ret 0x10). Returns whether the four rows were filled.
pub(crate) unsafe fn create_irish_super_cup_table(this: League) -> i32 {
    this.set_club_count(4);
    let t = exe::crt_malloc(0xec);
    this.set_table(t);
    if t == 0 {
        show_error_at(0x9ea0b8, 0xb5);
        return -1;
    }
    covered!("mwa_22 irish super cup table");
    let clubs = irish_super_cup_clubs(this.club_count());
    for (k, &c) in clubs.iter().enumerate() {
        exe::league_init_table_row(this, k as u8 as u32, c, 0, 0);
    }
    (clubs.len() as i16 == this.club_count()) as i32
}

/// Rule: the Irish Super Cup has four clubs: the Irish clubs in the Champions League, then those in the UEFA Cup, then
/// those in the Intertoto Cup, completed by the Premier Division clubs in database order.
/// Original: `IreSuperCup::create_table` (0x63b0c0, ire_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_super_cup_create_table(this: usize) -> i32 {
    create_irish_super_cup_table(League(this))
}

// simplification: init_table_row allocates each row's form and head-to-head: it cannot run twice. Only the original;
// the plan: the clubs of `irish_super_cup_clubs` in the rows and the return.
unsafe extern "thiscall" fn compare_ire_super_cup_create_table(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    let plan = irish_super_cup_clubs(4);
    let r = o(this);
    let l = League(this);
    let ok = l.table() != 0 && l.club_count() == 4 && r == (plan.len() == 4) as i32 && plan.iter().enumerate().all(|(k, &c)| read_usize(l.table() + k * 0x3b) == c);
    SCOREBOARD.record(ok, || format!("ire_super_cup_create_table({:#x}) returned {}, plan {:x?}", this, r, plan));
    r
}

/// Whether the Irish Super Cup is due today: not yet active (`league::ACTIVE` = 0), the UEFA seeding already in the
/// cup's year (the word `uefa_seeding::SEASON_YEAR` of [`G_UEFA_SEEDING`], zero-extended, equal to `league::YEAR`) and
/// the day's flag set.
pub(crate) unsafe fn irish_super_cup_due(this: League, flag: u32) -> bool {
    read_u32(this.addr() + league::ACTIVE) == 0 && read_u16(read_usize(G_UEFA_SEEDING) + uefa_seeding::SEASON_YEAR) as i32 == this.year() as i32 && flag != 0
}

/// Rule: once the European places of the Super Cup's year are known (the UEFA seeding has moved to that year), the Irish
/// Super Cup takes its four clubs, draws its fixtures and order and its predicted positions, once; then the day goes on
/// as in any league.
///
/// The league's daily update (thiscall(date, flag), ret 8): when due (`irish_super_cup_due`), the table
/// (`ire_super_cup_create_table` 0x63b0c0; 0: the error box at line 0x101), the fixtures (`exe::league_create_fixtures`;
/// 0: the box at line 0x106) and the order (`exe::league_sort_table`(this, 0); 0: the box at line 0x10c), each box
/// going on to the next step; the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`) and `league::ACTIVE` = 1.
/// Then always League::update_for_date (`exe::league_update_for_date`(this, date, flag), thiscall, ret 8), whose return
/// it passes on.
/// Original: `IreSuperCup::update_for_date` (0x63b300, ire_super_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn ire_super_cup_update_for_date(this: usize, date: usize, flag: u32) -> u8 {
    let l = League(this);
    if irish_super_cup_due(l, flag) { // mwa22: the Super Cup is set up once the seeding reaches its year
        covered!("mwa_22 irish super cup set up");
        let table: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63b0c0usize);
        if table(this) == 0 {
            show_error_at(0x9ea0b8, 0x101);
        }
        if exe::league_create_fixtures(l) == 0 {
            show_error_at(0x9ea0b8, 0x106);
        }
        if exe::league_sort_table(l, 0usize) == 0 {
            show_error_at(0x9ea0b8, 0x10c);
        }
        bra_nat_first_call_virtual_no_args(this, comp_vtable::SET_PREDICTED_POSITIONS);
        write_u32(this + league::ACTIVE, 1);
    }
    exe::league_update_for_date(l, date, flag)
}

// simplification: the set-up allocates and draws, and the base update plays the day: it cannot run twice. Only the
// original; the plan: when due, the cup becomes active with its table; otherwise `league::ACTIVE` stays.
unsafe extern "thiscall" fn compare_ire_super_cup_update_for_date(this: usize, date: usize, flag: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(%%I1%%);
    let l = League(this);
    let due = irish_super_cup_due(l, flag);
    let before = read_u32(this + league::ACTIVE);
    let r = o(this, date, flag);
    let after = read_u32(this + league::ACTIVE);
    let ok = if due { after == 1 && l.table() != 0 } else { after == before };
    SCOREBOARD.record(ok, || format!("ire_super_cup_update_for_date({:#x}) due {} active {} -> {}", this, due, before, after));
    r
}

/// Rule: at the season turn the Irish Super Cup is cleared: the table and the round table freed, the statistics
/// archived, the stages destroyed, the year moved on and the rules (with the new dates) made again; it waits inactive
/// for its new clubs.
///
/// `league::SEASON_CLOSED` = 0; the table freed (`exe::league_free_table`(this, 0)); the round table freed and zeroed;
/// the season archived (when there are statistics); with the current stage ≥ 0, the stages 0 to the current one
/// destroyed (their virtual 0 with 1); the year up by `year`; `league::CURRENT_STAGE` = −1; the rules
/// (`comp_vtable::SET_UP_RULES`; 0: the error box at line 0x15d and return); `league::ACTIVE` = 0.
pub(crate) unsafe fn start_irish_super_cup_season(this: League, year: i16) {
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
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
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    this.set_year(this.year().wrapping_add(year));
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        return show_error_at(0x9ea0b8, 0x15d);
    }
    covered!("mwa_22 irish super cup turnover");
    write_u32(this.addr() + league::ACTIVE, 0);
}

/// Original: `IreSuperCup::start_new_season` (0x63b550, ire_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_super_cup_start_new_season(this: usize) {
    start_irish_super_cup_season(League(this), 1) // mwa22: the Irish Super Cup turnover
}

unsafe extern "thiscall" fn compare_ire_super_cup_start_new_season(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_state_league_new_season("ire_super_cup_start_new_season", League(this), || {
        o(this);
        0
    });
}
