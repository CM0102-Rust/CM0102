// ---------- batch %%LEVA%%: the Copa del Rey and the Spanish Super Cup: the two constructors and the 83 Copa del Rey entrants (spa_cup.cpp, spa_super.cpp) ----------

/// Builds the cup's round table through its own virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table(−1,
/// &`cup::ROUND_COUNT`, &`cup::STAGE_LABEL`, null): push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this ->
/// thiscall, ret 0x10) and stores it in `cup::ROUND_TABLE`.
pub(crate) unsafe fn spa_cup_create_rounds(this: Cup) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    this.set_round_table(t);
}

/// Creates the cup's statistics at Comp +8 (operator new `exe::crt_operator_new` of 0x5ce, cdecl, and
/// Comp_Stats::Comp_Stats `exe::new_comp_stats`: push this; mov ecx, new -> thiscall, ret 4; the original's SEH frame
/// covers only that new; 0 without memory). Returns whether they were created.
pub(crate) unsafe fn spa_cup_create_comp_stats(this: Cup) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(this.addr() + 8, s); // Comp +8: the Comp_Stats object
    s != 0
}

/// The plan the comparer checks after the original constructor: the return value (this), the vtable, the competition
/// record (Comp +4) and `cup::YEAR`; on a new cup (`cup::STAGE_INDEX` = `stage_index::MAIN`), every fixed field the
/// constructor writes (offset, width, value) and the round table and statistics created. `None` = as planned.
pub(crate) unsafe fn spa_cup_check_constructor_plan(this: Cup, r: usize, year: u32, record: usize, vtable: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this.addr() || read_usize(this.addr()) != vtable || read_usize(this.addr() + 4) != record || read_u16(this.addr() + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this.addr()), read_usize(this.addr() + 4), read_u16(this.addr() + cup::YEAR)));
    }
    if this.stage_index() != stage_index::MAIN {
        return None;
    }
    for &(o, w, v) in fields {
        let x = match w {
            1 => read_u8(this.addr() + o) as u32,
            2 => read_u16(this.addr() + o) as u32,
            _ => read_u32(this.addr() + o),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, the constructor writes {:#x}", o, x, v));
        }
    }
    if (rounds && this.round_table() == 0) || read_usize(this.addr() + 8) == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", this.round_table(), read_usize(this.addr() + 8)));
    }
    None
}

/// The body of `construct_spa_cup` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_spa_cup_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x96f9c8); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_stage_count(0);
    this.set_club_sides(1);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(3);
    this.set_discipline_index(discipline_rule::SPAIN_CUP);
    this.set_run_by_stages(0);
    this.set_season_set_up(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_6 copa del rey new");
    this.set_team_count(0x53);
    let v = exe::crt_calloc(0x53, 6);
    this.set_team_table(v);
    if v == 0 {
        show_error_at(0xa82810, 0x52);
        return this.addr();
    }
    this.set_prize_fund(0xf4240);
    spa_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r == 0 {
        show_error_at(0xa82810, 0x60);
        return this.addr();
    }
    if !spa_cup_create_comp_stats(this) {
        show_error_at(0xa82810, 0x67);
        return this.addr();
    }
    this.addr()
}

/// The fixed fields `construct_spa_cup` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const COPA_DEL_REY_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 12] = [(cup::PRIZE_FUND, 4, 0xf4240), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::TEAM_COUNT, 2, 0x53), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SEASON_SET_UP, 4, 0x0), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x1b), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The constructor of the Copa del Rey: with a table of 83 entrants (6-byte records, zeroed); type 1 (clubs of one
/// nation), 83 teams, prize fund of 1,000,000, 7 substitutes on the bench and 3 substitutions, discipline rule 27; on a
/// new cup, in order: the rounds (the cup's own virtual `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`); the
/// tie tree (Cup::map_fixture_tree, `map_fixture_tree`, 0x518790, thiscall, ret 0; the error box of line 0x60 if it
/// returns 0); the statistics at Comp +8 (the error box of line 0x67 without them). Over the base Cup::Cup
/// (`exe::new_cup`, thiscall, ret 0) it writes the fixed fields in the original's order (the vtable 0x96f9c8, the
/// competition record at Comp +4, `cup::YEAR` and those of `COPA_DEL_REY_CONSTRUCTOR_FIELDS`); if the saved state loads
/// (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step
/// that fails shows its own error box (`show_error_at`) and returns this. Returns this.
/// Original: `SpaCup::SpaCup` (0x84cf50, spa_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_spa_cup(this: usize, year: u32, record: usize) -> usize {
    construct_spa_cup_body(Cup(this), year, record) // cqu6: the Copa del Rey
}

/// Comparer for `construct_spa_cup`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`spa_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_spa_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, year, record);
    let error = spa_cup_check_constructor_plan(Cup(this), r, year, record, 0x96f9c8, &COPA_DEL_REY_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_spa_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of `construct_spa_super` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_spa_super_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x96fd38); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    this.set_stage_count(0);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_club_sides(1);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(3);
    this.set_discipline_index(discipline_rule::SPAIN_LEAGUE);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_6 spanish super cup new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x858aa0usize); // SpaSuper::select_teams
    let r = select_teams(this.addr());
    if r < 0 {
        show_error_at(0xa82a58, 0x43);
        return this.addr();
    }
    spa_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r < 0 {
        show_error_at(0xa82a58, 0x51);
        return this.addr();
    }
    if !spa_cup_create_comp_stats(this) {
        show_error_at(0xa82a58, 0x58);
        return this.addr();
    }
    this.addr()
}

/// The fixed fields `construct_spa_super` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const SPANISH_SUPER_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 9] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x1a), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The constructor of the Spanish Super Cup: with no entrant table of its own; type 1 (clubs of one nation), no prize
/// fund (`cup::PRIZE_FUND` stays as the base left it), 7 substitutes on the bench and 3 substitutions, discipline rule
/// 26; on a new cup, in order: the two clubs (SpaSuper::select_teams, `spa_super_select_teams`, 0x858aa0, thiscall, ret
/// 0; the error box of line 0x43 if it returns a negative value); the rounds (the cup's own virtual
/// `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`); the tie tree (Cup::map_fixture_tree, `map_fixture_tree`,
/// 0x518790, thiscall, ret 0; the error box of line 0x51 if it returns a negative value); the statistics at Comp +8
/// (the error box of line 0x58 without them). Over the base Cup::Cup (`exe::new_cup`, thiscall, ret 0) it writes the
/// fixed fields in the original's order (the vtable 0x96fd38, the competition record at Comp +4, `cup::YEAR` and those
/// of `SPANISH_SUPER_CUP_CONSTRUCTOR_FIELDS`); if the saved state loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx,
/// this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step that fails shows its own error box
/// (`show_error_at`) and returns this. Returns this.
/// Original: `SpaSuper::SpaSuper` (0x8586b0, spa_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_spa_super(this: usize, year: u32, record: usize) -> usize {
    construct_spa_super_body(Cup(this), year, record) // cqu6: the Spanish Super Cup
}

/// Comparer for `construct_spa_super`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`spa_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_spa_super(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%)(this, year, record);
    let error = spa_cup_check_constructor_plan(Cup(this), r, year, record, 0x96fd38, &SPANISH_SUPER_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_spa_super({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// Whether club `c` is left out of the Copa del Rey by its link to a B team (club_get_linked_b_team
/// `exe::club_get_linked_b_team`: push 1; push &senior; push club; add esp, 0xc -> cdecl): it is when there is a link
/// and the output says it is not the senior side (0).
pub(crate) unsafe fn spa_cup_is_linked_b_team(c: Club) -> bool {
    let mut senior = 0u32;
    let r = exe::club_get_linked_b_team(c, &mut senior, 1);
    r != 0 && senior == 0
}

/// Whether the club's division (`club::LAST_DIVISION`, the competition record; +0 the id) is the one of key `key`.
pub(crate) unsafe fn spa_cup_in_division(c: Club, key: usize) -> bool {
    let l = c.last_division();
    !l.is_null() && read_usize(l.addr()) == read_usize(key)
}

/// Whether the club, with a nation (`club::NATION`) and a division (`club::LAST_DIVISION`), is in the lower Spanish
/// divisions (`KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B` and the four groups `KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1`
/// to `_B4`: the Segunda B and the Tercera [inference]).
pub(crate) unsafe fn spa_cup_is_lower_division_club(c: Club) -> bool {
    if c.nation().is_null() || c.last_division().is_null() {
        return false;
    }
    [KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B2,
        KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B3, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B4].iter().any(|&g| spa_cup_in_division(c, g))
}

/// Writes club `c` into slot `k` of the entrant table (`cup::TEAM_TABLE`, 6-byte `cup_team_entry` records: the club,
/// the seed level and the status zeroed).
pub(crate) unsafe fn spa_cup_put_entrant(this: Cup, k: i32, c: usize) {
    let t = this.team_table() + k as usize * 6;
    write_usize(t + cup_team_entry::CLUB, c);
    write_u8(t + cup_team_entry::SEED_LEVEL, 0);
    write_u8(t + cup_team_entry::STATUS, 0);
}

/// Copies the `n` clubs of `list` into the slots from `from` on.
pub(crate) unsafe fn spa_cup_put_block(this: Cup, from: i32, list: &[usize], n: i32) {
    for j in 0..n.max(0) {
        spa_cup_put_entrant(this, from + j, list[j as usize]);
    }
}

/// The body of `spa_cup_collect_entrants` (the comparer runs it on its own copy of the table).
pub(crate) unsafe fn spa_cup_collect_entrants_body(this: Cup) -> i32 {
    let base = read_usize(G_CLUB_TABLE);
    let limit = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let mut continental = [0usize; 20];
    let mut first = [0usize; 20];
    let mut second = [0usize; 22];
    let (mut nc, mut nf, mut ns) = (0i16, 0i16, 0i16);
    let mut i = 0i32;
    while i < limit() {
        let c = Club(base + i as usize * club::SIZE);
        let nation = c.nation();
        if !nation.is_null() && read_usize(nation.addr()) == read_usize(KEY_NATION_SPAIN) && !spa_cup_is_linked_b_team(c) {
            let cont = c.continental_comp_id() as i32;
            if cont != -1 && cont != read_usize(KEY_CLUB_COMP_INTER_TOTO_CUP) as i32 && nc < 0x14 && nc < 7 {
                continental[nc as usize] = c.addr();
                nc += 1;
            } else if !c.last_division().is_null() {
                if spa_cup_in_division(c, KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) && nf < 0x14 {
                    first[nf as usize] = c.addr();
                    nf += 1;
                } else if spa_cup_in_division(c, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION) && ns < 0x16 {
                    covered!("cqu_6 second division");
                    second[ns as usize] = c.addr();
                    ns += 1;
                }
            }
        }
        i += 1;
    }
    let missing = 0x53i32 - ns as i32 - nf as i32 - nc as i32;
    this.set_team_count(0x53);
    let t = exe::crt_calloc(0x53, 6);
    this.set_team_table(t);
    if t == 0 {
        show_error_at(0xa82810, 0x1e0);
        return -1;
    }
    let missing_w = missing as i16 as i32;
    let mut k = 0i32;
    let nation = Competition(read_usize(this.addr() + 4)).nation();
    if nation.league_selected() & 4 != 0 {
        let list = exe::crt_malloc(0x2d0);
        if list == 0 {
            show_error_at(0xa82810, 0x1ee);
            return 0;
        }
        let mut n = 0i32;
        let mut i = 0i32;
        while i < limit() {
            if n >= 0x78 {
                break;
            }
            let c = Club(base + i as usize * club::SIZE);
            if spa_cup_is_lower_division_club(c) && !spa_cup_is_linked_b_team(c) && c.continental_comp_id() as i32 == -1 {
                write_usize(list + n as usize * 6, c.addr());
                write_u8(list + n as usize * 6 + 4, c.last_position()); // the sort key
                n += 1;
            }
            i += 1;
        }
        exe::crt_qsort(list, n as u32, 6, 0x92f660usize);
        let mut j = 0i32;
        while j < missing_w && j < n {
            k += 1;
            spa_cup_put_entrant(this, missing_w - 1 - j, read_usize(list + j as usize * 6));
            j += 1;
        }
        exe::crt_free(list);
    } else {
        let mut i = 0i32;
        while i < limit() {
            let c = Club(base + i as usize * club::SIZE);
            if spa_cup_is_lower_division_club(c) && !spa_cup_is_linked_b_team(c) && c.continental_comp_id() as i32 == -1 {
                k += 1;
                spa_cup_put_entrant(this, missing_w - k, c.addr());
                if k >= missing_w {
                    break;
                }
            }
            i += 1;
        }
    }
    if missing_w - k > 0 && limit() > 0 {
        covered!("cqu_6 fill with clubs without division");
        let mut i = 0i32;
        while i < limit() {
            let c = Club(base + i as usize * club::SIZE);
            let nation = c.nation();
            if !nation.is_null() && read_usize(nation.addr()) == read_usize(KEY_NATION_SPAIN) && !spa_cup_is_linked_b_team(c) && c.continental_comp_id() as i32 == -1
                && (c.last_division().is_null() || spa_cup_in_division(c, KEY_CLUB_COMP_A_LOWER_DIVISION) || spa_cup_in_division(c, KEY_CLUB_COMP_SPANISH_LOWER_DIVISION)) {
                k += 1;
                spa_cup_put_entrant(this, missing_w - k, c.addr());
                if k >= missing_w {
                    break;
                }
            }
            i += 1;
        }
    }
    let (ns, nf, nc) = (ns as i32, nf as i32, nc as i32);
    spa_cup_put_block(this, missing_w, &second, ns); spa_cup_put_block(this, missing_w + ns.max(0), &first, nf); // cqu6: the Segunda and then the Primera
    spa_cup_put_block(this, missing_w + ns.max(0) + nf.max(0), &continental, nc);
    1
}

/// The 83 Copa del Rey entrants (called by the Copa del Rey day routine, 0x84d740, once up to 2002-06-01). In club
/// table order (`G_CLUB_TABLE`, the first `G_CLUB_COUNT` − 2 × `G_NATION_COUNT`, the limit re-read on every pass), the
/// Spanish clubs (`club::NATION` = `KEY_NATION_SPAIN`) that are not a linked B team (`spa_cup_is_linked_b_team`) go to
/// three groups: those in a continental competition (`club::CONTINENTAL_COMP_ID` ≠ −1 and ≠
/// `KEY_CLUB_COMP_INTER_TOTO_CUP`), up to 7; otherwise those of the Primera (`club::LAST_DIVISION` =
/// `KEY_CLUB_COMP_SPANISH_FIRST_DIVISION`, up to 20) and of the Segunda (`KEY_CLUB_COMP_SPANISH_SECOND_DIVISION`, up to
/// 22). Missing = 83 − the three groups; `cup::TEAM_COUNT` = 83 and the table (calloc `exe::crt_calloc` of 83 × 6:
/// push 6; push 0x53; add esp, 8 -> cdecl) in `cup::TEAM_TABLE` (without memory: the error box of line 0x1e0 of
/// spa_cup.cpp and −1). Slots 0 to missing − 1 go to the lower clubs (`spa_cup_is_lower_division_club`, not a B team,
/// `club::CONTINENTAL_COMP_ID` = −1), from the last to the first: if the cup's nation (the competition record at Comp
/// +4, `competition::NATION`) has bit 4 in `nation::LEAGUE_SELECTED`, up to 120 of them in a list (malloc
/// `exe::crt_malloc` of 0x2d0; without memory: the error box of line 0x1ee and 0) with `club::LAST_POSITION` as the
/// key, sorted (qsort `exe::crt_qsort` with 0x92f660: push 0x92f660; push 6; push n; push list; add esp, 0x10 ->
/// cdecl; the list freed with `exe::crt_free`), and the first of them; otherwise in table order. If some are still
/// missing, the Spanish clubs with no division or in the generic divisions `KEY_CLUB_COMP_A_LOWER_DIVISION` or
/// `KEY_CLUB_COMP_SPANISH_LOWER_DIVISION` (not a B team, `club::CONTINENTAL_COMP_ID` = −1). After the lower slots come
/// the Segunda, the Primera and the continental clubs, in that order (the cup rounds make the top ones enter later).
/// Returns 1.
/// Original: `SpaCup::collect_entrants` (0x84d910, spa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_cup_collect_entrants(this: usize) -> i32 {
    spa_cup_collect_entrants_body(Cup(this))
}

/// Comparer for `spa_cup_collect_entrants`: side by side from the same state (`cup::TEAM_COUNT`; it does not draw): the
/// original leaves its table in `cup::TEAM_TABLE`, the copy makes its own, compared over the 83 slots (6 bytes each)
/// and freed; the return value too.
pub(crate) unsafe extern "thiscall" fn compare_spa_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%);
    let before = this.team_table();
    let table_original = core::cell::Cell::new(0usize);
    let table_rust = core::cell::Cell::new(0usize);
    let (a, b, same, _, _) = crate::comparison::side_by_side(&[(this.addr() + cup::TEAM_COUNT, 2usize)], || {
        let r = o(this.addr());
        table_original.set(this.team_table());
        this.set_team_table(before);
        r
    }, || {
        let r = spa_cup_collect_entrants_body(this);
        table_rust.set(this.team_table());
        this.set_team_table(table_original.get());
        r
    });
    let (lo, lr) = (table_original.get(), table_rust.get());
    let table = lo == 0 || lr == 0 || core::slice::from_raw_parts(lo as *const u8, 0x53 * 6) == core::slice::from_raw_parts(lr as *const u8, 0x53 * 6);
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(same && a == b && table, || format!("spa_cup_collect_entrants({:#x}) returned {} x {} memory {} table {}", this.addr(), a, b, same, table));
    a
}
