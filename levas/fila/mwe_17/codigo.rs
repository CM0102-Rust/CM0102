// ---------- batch %%LEVA%%: the Portuguese Super Cup: its teams, its round table and its turnover (por_super.cpp) ----------

/// The Portuguese Super Cup round outside 2000: draw on 5 July (weekday 5), one match on 12 August (weekday 5) at
/// venue 4 [inference: a neutral ground], tie rules 3, category 6, two clubs, a single leg (+0x20 = 1, one leg) with
/// the gap byte 0x7e, no prizes; +0x11 = 3 is written apart (`write_portuguese_super_cup_round`).
pub(crate) const PORTUGUESE_SUPER_CUP: EngCcCupCupRound = cql10r((5, 6, 0, 5), (0xc, 7, 0, 5, 1, 4), 0, (0, 3, 0), 6, (2, 1, 2, 0), (1, 1, 0x7e), (0, 0, 0)); // mwe17: the Portuguese Super Cup date

/// The Portuguese Super Cup round in 2000: the same, with venue 0 (the clubs' grounds), no tie rules and two legs.
pub(crate) const PORTUGUESE_SUPER_CUP_2000: EngCcCupCupRound = cql10r((5, 6, 0, 5), (0xc, 7, 0, 5, 1, 0), 0, (0, 0, 0), 6, (2, 1, 2, 0), (1, 2, 0x7e), (0, 0, 0));

/// Writes the Super Cup round 0 of table `t` for the cup's year: the round of `PORTUGUESE_SUPER_CUP_2000` in 2000
/// (`.year()` = 0x7d0), otherwise `PORTUGUESE_SUPER_CUP` (`eng_cc_cup_write_cup_round`, the year sign-extended), and the
/// word +0x11 = 3 [unnamed].
pub(crate) unsafe fn write_portuguese_super_cup_round(this: Cup, t: usize) {
    let r = if this.year() as u16 == 0x7d0 { &PORTUGUESE_SUPER_CUP_2000 } else { &PORTUGUESE_SUPER_CUP };
    eng_cc_cup_write_cup_round(t, 0, r, this.year() as i32 as u32);
    write_u16(t + 0x11, 3);
}

/// The Portuguese Super Cup teams: the Premier Division champion and the Portuguese Cup winner; if the same club won
/// both, the cup runner-up takes the second place; if a club is missing, the error box.
///
/// `PorSuper::select_teams()`: `.set_team_count()` = 2, the entrant list (`exe::crt_malloc` of 0xc into
/// `cup::TEAM_TABLE`; out of memory: the box of por_super.cpp (0xa787d4) line 0xca and −1). Entry 0 = the last
/// champion of the Premier Division (`latest_placed_club`(`KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION`, 0):
/// `exe::comp_util_get_latest_placed_club`, push place; push record -> cdecl); none: the box of line 0xd3 and 0. Entry 1
/// (+6) = the last Portuguese Cup winner (`KEY_CLUB_COMP_PORTUGUESE_CUP`, 0); none: the box of 0xdc and 0. If both are
/// the same club, entry 1 = the cup runner-up (place 1); none: the box of 0xe8 and 0. Class and status bytes 0 for
/// both; returns 1.
/// Original: `PorSuper::select_teams` (0x7d3260, por_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_super_select_teams(this: usize) -> i32 {
    portuguese_super_cup_select_teams(Cup(this)).0
}

/// The body of `por_super_select_teams`; also returns the number of clubs written (2, or 0 after a box).
pub(crate) unsafe fn portuguese_super_cup_select_teams(this: Cup) -> (i32, i32) {
    this.set_team_count(2);
    let l = exe::crt_malloc(0xc);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0xa787d4, 0xca);
        return (-1, 0);
    }
    let a = latest_placed_club(KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION, 0); // mwe17: the Portuguese champion
    write_usize(l, a);
    if a == 0 {
        show_error_at(0xa787d4, 0xd3);
        return (0, 0);
    }
    let b = latest_placed_club(KEY_CLUB_COMP_PORTUGUESE_CUP, 0);
    write_usize(l + 6, b);
    if b == 0 {
        show_error_at(0xa787d4, 0xdc);
        return (0, 0);
    }
    if a == b {
        covered!("mwe_17 portuguese double");
        let b = latest_placed_club(KEY_CLUB_COMP_PORTUGUESE_CUP, 1);
        write_usize(l + 6, b);
        if b == 0 {
            show_error_at(0xa787d4, 0xe8);
            return (0, 0);
        }
    }
    covered!("mwe_17 portuguese super cup clubs");
    write_u8(l + 4, 0);
    write_u8(l + 5, 0);
    write_u8(l + 0xa, 0);
    write_u8(l + 0xb, 0);
    (1, 2)
}

/// The Portuguese Super Cup round table: one round, a single match on 12 August at a neutral ground (in 2000, two legs
/// at the clubs' grounds), drawn on 5 July.
///
/// `PorSuper::create_round_table(stage, &rounds, &extra, &out)`: at stage −1 (0xff), `*out` = 0 (when given), 1 round,
/// extra 0, the table (`exe::crt_malloc` of 0x68; out of memory: the box of por_super.cpp (0xa787d4) line 0x95 and 0)
/// with round 0 by `write_portuguese_super_cup_round` (Cup round draw and match dates, cdecl, and the fields); any other
/// stage: 0.
/// Original: `PorSuper::create_round_table` (0x7d30f0, por_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn por_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    if !out.is_null() {
        *out = 0;
    }
    *rounds = 1;
    *extra = 0;
    let t = exe::crt_malloc(0x68);
    if t == 0 {
        show_error_at(0xa787d4, 0x95);
        return 0;
    }
    covered!("mwe_17 portuguese super cup round");
    write_portuguese_super_cup_round(Cup(this), t);
    t
}

/// The Portuguese Super Cup turnover: last season's team list, rounds and ties freed, the season archived, the year + 1;
/// then the two teams (`PorSuper::select_teams`), the round table and the tie tree; unlike the other cups, no predicted
/// positions (the tie tree's eax is returned).
///
/// `PorSuper::start_new_season()`: +0x4c = 0; `cup::TEAM_TABLE` and the round list freed and zeroed; the tie arrays
/// (one per round) and their vector freed and zeroed; `exe::comp_stats_archive_season_and_reset`(1); the year + 1; the
/// current round and the stage flag 0; `PorSuper::select_teams` (0x7d3260, direct), the rounds (virtual
/// `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`), each with the box of
/// por_super.cpp (0xa787d4) at 0x12d, 0x134, 0x13b when it returns a negative value (and 0 returned); the tie tree's
/// value is returned.
/// Original: `PorSuper::start_new_season` (0x7d3490, por_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_super_start_new_season(this: usize) -> u32 {
    let this = Cup(this);
    this.set_season_closed(0);
    for o in [cup::TEAM_TABLE, 0xa3usize] {
        let l = read_usize(this.addr() + o);
        if l != 0 {
            exe::crt_free(l);
            write_u32(this.addr() + o, 0);
        }
    }
    if this.tie_tables() != 0 {
        let mut i = 0i32;
        while i < this.round_count() as i32 {
            let p = read_usize(this.tie_tables() + i as usize * 4);
            if p != 0 {
                exe::crt_free(p);
                write_u32(this.tie_tables() + i as usize * 4, 0);
            }
            i += 1;
        }
        exe::crt_free(this.tie_tables());
        this.set_tie_tables(0);
    }
    let e = read_usize(this.addr() + 8);
    if e != 0 {
        exe::comp_stats_archive_season_and_reset(e, 1);
    }
    this.set_year(this.year().wrapping_add(1)); // mwe17: the Super Cup next year
    covered!("mwe_17 portuguese super cup turnover");
    this.set_current_round(0);
    this.set_run_by_stages(0);
    let teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x7d3260usize);
    if teams(this.addr()) < 0 {
        show_error_at(0xa787d4, 0x12d);
        return 0;
    }
    if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(0xa787d4, 0x134);
        return 0;
    }
    let r = call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE);
    if r < 0 {
        show_error_at(0xa787d4, 0x13b);
        return 0;
    }
    r as u32
}

/// Comparer for `por_super_select_teams`: side by side (`eng_cc_cup_compare_entrant_list`: the count, the mouse flag
/// written by the boxes and the 12 bytes of the two entries when it returns 1; the copy's list freed).
pub(crate) unsafe extern "thiscall" fn compare_por_super_select_teams(this: usize) -> i32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    eng_cc_cup_compare_entrant_list("por_super_select_teams", this, &[(this.addr() + cup::TEAM_COUNT, 2), (G_MOUSE_LEFT_BUTTON_DOWN, 4)], || o(this.addr()), || portuguese_super_cup_select_teams(this),
        |r, _| if r == 1 { 12 } else { 0 })
}

/// Comparer for `por_super_create_round_table`: side by side (`compare_cup_round_table`: the outputs and the written
/// round bytes equal; the copy's table freed).
pub(crate) unsafe extern "thiscall" fn compare_por_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_cup_round_table("por_super_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| por_super_create_round_table(this, stage, r, e, s))
}

/// Comparer for `por_super_start_new_season`: only the original runs (it frees and allocates); the year + 1
/// (`compare_cup_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_por_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("por_super_start_new_season", Cup(this), || o(this))
}
