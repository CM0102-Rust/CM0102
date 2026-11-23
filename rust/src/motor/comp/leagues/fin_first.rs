use super::*;

/// Builds the Finnish First Division: discipline rule 10 (the Finnish league one) and four stages after the North
/// group; with no saved state, the rules, the North group table, the fixtures, the order and the stats; then the South
/// group stage and the predicted positions of both groups.
///
/// `FinFirst::FinFirst(year, record)`: `construct_league_with_stages` with vtable 0x96a400, discipline byte 0xa, four
/// extra stages (malloc of 0x10), `FinFirst::set_up_rules` (0x593b80) and `FinFirst::create_table` (0x593a70); the
/// boxes of fin_first.cpp lines 0x4a, 0x51, 0x58, 0x61, 0x68; at the end FinFirst::create_south_group_stage (0x594cd0)
/// and FinFirst::set_predicted_positions (0x594ff0), direct calls (thiscall, ret 0), instead of
/// League::set_predicted_positions.
/// Original: `FinFirst::FinFirst` (0x592eb0, fin_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_fin_first(this: usize, year: u32, record: usize) -> usize {
    construct_league_with_stages(League(this), year, record, &FINNISH_FIRST_DIVISION)
}

/// Comparer for `construct_fin_first`.
pub(crate) unsafe extern "thiscall" fn compare_construct_fin_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_FIN_FIRST);
    compare_league_with_stages_ctor("construct_fin_first", League(this), year, record, &FINNISH_FIRST_DIVISION, || o(this, year, record))
}

/// Sets up the rules of the Finnish First Division (Ykkönen): three legs, 3 points for a win; 2 go up, 1 plays the
/// relegation play-off and 2 go down; 7 substitutes and 3 substitutions.
///
/// `FinFirst::set_up_rules()`: the writes of `finnish_first_rules` and the main-stage round table (virtual
/// `comp_vtable::CREATE_SCHEDULE`) into `league::ROUND_DATES`. Returns 1.
/// Original: `FinFirst::set_up_rules` (0x593b80, fin_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_first_set_up_rules(this: usize) -> u32 {
    covered!("mwe_5 ykkonen rules");
    eng_prm_apply_league_rules(League(this), &finnish_first_rules())
}

/// Comparer for `fin_first_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FIN_FIRST_SET_UP_RULES);
    eng_prm_compare_league_rules("fin_first_set_up_rules", League(this), finnish_first_rules(), || o(this), || fin_first_set_up_rules(this))
}

/// Creates the Finnish First Division table of the league stage: the North group, 10 clubs (those of the division
/// whose regional group is the North one, in database order); no prize fund nor TV money.
///
/// `FinFirst::create_table()`: `.set_club_count()` = 10, the table (`exe::crt_malloc` of 0x24e; out of memory: the box
/// of fin_first.cpp (0x9c818c) line 0x1ad and 0), the clubs of `club_in_finnish_north_group` in database order;
/// returns 1 if it filled, otherwise 0. (The South group is a separate stage, FinFirst::create_south_group_stage.)
/// Original: `FinFirst::create_table` (0x593a70, fin_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_first_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0xa);
    if !allocate_first_division_table(this, 0x24e, 0x9c818c, 0x1ad) {
        return 0;
    }
    covered!("mwe_7 ykkonen north table");
    fill_first_division_rows(this, |c| club_in_finnish_north_group(c)) as u32 // mwe7: the North group clubs
}

/// Comparer for `fin_first_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FIN_FIRST_CREATE_TABLE);
    compare_first_division_table("fin_first_create_table", League(this), 0xa, |c| club_in_finnish_north_group(c), || o(this))
}

/// The Ykkönen turnover: archives the season, clears the table, the round dates and the stages (and their vector, which
/// is allocated again), moves to the next year and rebuilds the rules, the North group table, the fixtures and the
/// order; then the board expectations, the referee renewal, the South group stage and the predicted positions.
///
/// `FinFirst::start_new_season()`: `league::SEASON_CLOSED` = 0, `exe::league_free_table`(0), the round dates freed
/// and zeroed, `exe::comp_stats_archive_season_and_reset`(1) when there are statistics, the stages 0..=current (when
/// current ≥ 0) destroyed by their virtual 0 with 1; the current stage −1, the stage vector freed (`exe::crt_free`),
/// the year + 1 and a new vector of 4 × `.stage_count()` bytes (`exe::crt_malloc`; 0 when there are no stages). Then,
/// each with the box of fin_first.cpp (0x9c818c) when it returns 0 (lines 0x21b, 0x222, 0x228, 0x22f, and 0 is
/// returned): the rules (virtual `comp_vtable::SET_UP_RULES`), `FinFirst::create_table` (0x593a70),
/// `exe::league_create_fixtures` and `exe::league_sort_table`(0). Then League::set_club_season_expectations,
/// OfficialsManager::renew_officials(record) on `G_OFFICIALS_MANAGER`, FinFirst::create_south_group_stage (0x594cd0,
/// thiscall, ret 0) and the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose eax is
/// returned.
/// Original: `FinFirst::start_new_season` (0x593c20, fin_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
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
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    exe::crt_free(this.stages());
    this.set_year(this.year().wrapping_add(1)); // mwe9: the Ykkonen next year
    let n = this.stage_count() as i32;
    let v = if n > 0 { exe::crt_malloc((n as u32).wrapping_mul(4)) } else { 0 };
    this.set_stages(v);
    let file = 0x9c818c;
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, 0x21b);
        return 0;
    }
    if exe::finfirst_create_table(this) == 0 {
        show_error_at(file, 0x222);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, 0x228);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, 0x22f);
        return 0;
    }
    covered!("mwe_9 ykkonen turnover");
    exe::league_set_club_season_expectations(this);
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr());
    exe::finfirst_create_south_group_stage(this);
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS)
}

/// Comparer for `fin_first_start_new_season`: the year + 1, and the stage vector allocated again when there are
/// stages.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FIN_FIRST_START_NEW_SEASON);
    let r = compare_state_league_new_season("fin_first_start_new_season", League(this), || o(this));
    let l = League(this);
    let ok = (l.stages() != 0) == (l.stage_count() as i32 > 0);
    SCOREBOARD.record(ok, || format!("fin_first_start_new_season({:#x}) stages {} vector {:#x}", this, l.stage_count(), l.stages()));
    r
}

/// Moves the Ykkönen to its next stage: after the North group (and the South group stage 0) the play-off stages are
/// built at stage 1; stage 2 waits for the Veikkausliiga to finish and then the Premier Division play-off is built at
/// stage 3; at the last stage, the error box (line 0x2ce).
///
/// `FinFirst::advance_stage()`: at the last stage (`.current_stage()` ≥ `.stage_count()` − 1, signed), the box of
/// fin_first.cpp (0x9c818c) line 0x2ce and nothing else. At stage 2, only when the Veikkausliiga
/// (`KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION`) is complete (virtual `comp_vtable::CHECK_COMPLETE` with (0, 1)), after
/// its virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS`; otherwise nothing. Then the current stage goes up by one: on
/// reaching 1, FinFirst::create_play_off_stages (0x594080); on reaching 3, FinFirst::create_premier_play_off_stage
/// (0x594610) (direct calls, thiscall, ret 0).
/// Original: `FinFirst::advance_stage` (0x593f90, fin_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_first_advance_stage(this: usize) {
    let this = League(this);
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x9c818c, 0x2ce);
    }
    if stage == 2 {
        let p = comp_by_global(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION);
        let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(p) + comp_vtable::CHECK_COMPLETE));
        if complete(p, 0, 1) == 0 {
            return;
        }
        let update: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(p) + league_vtable::UPDATE_POSITIONS_AND_STATUS));
        update(p); // mwe11: the Veikkausliiga statuses brought up to date
    }
    let stage = stage.wrapping_add(1);
    this.set_current_stage(stage as usize);
    let next: usize = match stage {
        1 => 0x594080,
        3 => 0x594610,
        _ => return,
    };
    covered!("mwe_11 ykkonen next stage built");
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(next);
    f(this.addr());
}

/// Writes a club's current, best and worst progress codes in its Comp_Stats entry, interleaving the two groups and
/// mapping the play-off results to the codes after the league places.
///
/// `FinFirst::set_club_progress(club, stage, current, best, worst)`: the entry of the club in the league's Comp_Stats
/// (`exe::comp_stats_find_club_entry`, thiscall, ret 4). Stage (a signed byte) below 1 (the groups): each value v
/// becomes 2v − 1 (a byte; the North and South places interleave). Stage 1: as they come. Stage 3: for a club whose
/// division (`club::DIVISION`, read without a null check) is the Veikkausliiga, the entry of the Veikkausliiga's
/// Comp_Stats (`G_COMP_OBJECT_TABLE` by `KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION`, its `league::COMP_STATS`) with 1 → 11
/// and 2 → 12; any other club, as they come in the Ykkönen entry. Stage 2: for an Ykkönen club
/// (`KEY_CLUB_COMP_FINNISH_FIRST_DIVISION`), 1 → 15 and 3 → 17; any other club, nothing is written. Other stages: as
/// they come. The bytes go to `comp_stats_club::CURRENT_POSITION`, `::BEST_POSITION` and `::WORST_POSITION` (the entry
/// is not checked for null).
/// Original: `FinFirst::set_club_progress` (0x593e10, fin_first.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn fin_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let this = League(this);
    let club = Club(club);
    let mut e = exe::comp_stats_find_club_entry(this.comp_stats(), club);
    let s = stage as u8 as i8;
    let (mut c, mut b, mut w) = (current as u8, best as u8, worst as u8);
    if s < 1 {
        covered!("mwe_11 ykkonen group progress");
        c = c.wrapping_shl(1).wrapping_sub(1); // mwe11: the group places interleaved
        b = b.wrapping_shl(1).wrapping_sub(1);
        w = w.wrapping_shl(1).wrapping_sub(1);
    } else if s == 3 {
        if read_usize(club.division().addr()) == read_usize(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION) {
            e = exe::comp_stats_find_club_entry(read_usize(comp_by_global(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION) + league::COMP_STATS), club);
            c = finnish_progress_code(3, c);
            b = finnish_progress_code(3, b);
            w = finnish_progress_code(3, w);
        }
    } else if s == 2 {
        if read_usize(club.division().addr()) != read_usize(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION) {
            return;
        }
        c = finnish_progress_code(2, c);
        b = finnish_progress_code(2, b);
        w = finnish_progress_code(2, w);
    }
    write_u8(e + comp_stats_club::BEST_POSITION, b);
    write_u8(e + comp_stats_club::CURRENT_POSITION, c);
    write_u8(e + comp_stats_club::WORST_POSITION, w);
}

/// Comparer for `fin_first_advance_stage`: only the original runs (it builds stages); the current stage stays (at the
/// last stage, or at stage 2 with the Veikkausliiga still playing) or goes up by one.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FIN_FIRST_ADVANCE_STAGE);
    let l = League(this);
    let before = l.current_stage() as i32;
    let last = before >= (l.stage_count() as i32).wrapping_sub(1);
    o(this);
    let after = l.current_stage() as i32;
    // A fase sobe 1 e, quando a nova e 1 ou 3, a rotina chama o construtor da fase seguinte (0x594080 / 0x594610),
    // que avanca de novo: de 0 o original para em 2 (visto no jogo real da 304). Entao a invariante verdadeira e "a
    // fase subiu ao menos 1", nao "subiu exatamente 1" (17/11/2026).
    let ok = if last {
        after == before
    } else if before == 2 {
        after == 2 || after >= 3
    } else {
        after >= before + 1 && after <= (l.stage_count() as i32).wrapping_sub(1)
    };
    SCOREBOARD.record(ok, || format!("fin_first_advance_stage({:#x}) before={} after={}", this, before, after));
}

/// Comparer for `fin_first_set_club_progress`: side by side on the 3 bytes of the entry the routine writes (the
/// Veikkausliiga's at stage 3 for a Veikkausliiga club); only the original without an entry.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let this = League(this);
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_FIN_FIRST_SET_CLUB_PROGRESS);
    let premier_club = club.division().addr() != 0 && read_usize(club.division().addr()) == read_usize(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION);
    let stats = if stage as u8 as i8 == 3 && premier_club { read_usize(comp_by_global(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION) + league::COMP_STATS) } else { this.comp_stats() };
    let e = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if e == 0 || club.division().addr() == 0 {
        return o(this.addr(), club.addr(), stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(e + comp_stats_club::CURRENT_POSITION, 3)], || o(this.addr(), club.addr(), stage, current, best, worst), || fin_first_set_club_progress(this.addr(), club.addr(), stage, current, best, worst));
    SCOREBOARD.record(same, || format!("fin_first_set_club_progress({:#x}, {:#x}, stage {}, {} {} {}) original={:x?} rust={:x?}", this.addr(), club.addr(), stage as u8 as i8, current as u8, best as u8, worst as u8, mo, mr));
}

/// The Ykkonen schedule: the two groups (stages −1 and 0) have the same 27 rounds (three legs of 10 clubs), from 29 April
/// to 6 October, with the stage code 0x41d for the North group and 0x41e for the South one; stage 1 is the promotion
/// play-off (two-legged semi-finals of four clubs from 5 October and a two-legged final from 12 October); stage 2 the
/// relegation play-off (two two-legged ties from 5 October); stage 3 the play-off against the Veikkausliiga (one
/// two-legged tie from 19 October).
///
/// `FinFirst::create_schedule(stage, &rounds, &extra, &league)`: stage below 1 (a signed byte: −1 or 0): `*league` = 1
/// (when given), 27 rounds, extra 0x41d (stage −1) or 0x41e (any other, i.e. 0), the table (`exe::crt_malloc` of
/// 0x6db; out of memory: the box of fin_first.cpp (0x9c818c) line 0xb5 and 0) filled from `YKKONEN_ROUNDS`
/// (`eng_first_set_league_rounds`). Stage 1: `*league` = 0, 2 rounds, extra 0x458, malloc 0xd0 (box line 0x11d), the
/// rounds `YKKONEN_PLAY_OFF_SEMI` and `YKKONEN_PLAY_OFF_FINAL`. Stage 2: `*league` = 0, 1 round, extra 0x452, malloc
/// 0x68 (box 0x158), `YKKONEN_RELEGATION_PLAY_OFF`. Stage 3: `*league` = 0, 1 round, extra 0, malloc 0x68 (box 0x180),
/// `YKKONEN_PREMIER_PLAY_OFF`. The cup rounds by `write_play_off_round` (the dates with the year sign-extended).
/// Any other stage: 0.
/// Original: `FinFirst::create_schedule` (0x5931c0, fin_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn fin_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    let s = stage as u8 as i8;
    if s < 1 {
        if !league.is_null() {
            *league = 1;
        }
        *rounds = 0x1b;
        *extra = if s as u8 == 0xff { 0x41d } else { 0x41e }; // mwe13: the North and South group codes
        let t = exe::crt_malloc(0x6db);
        if t == 0 {
            show_error_at(0x9c818c, 0xb5);
            return 0;
        }
        covered!("mwe_13 ykkonen group schedule");
        eng_first_set_league_rounds(t, (this.year() as u16) as u32, 0, &YKKONEN_ROUNDS);
        return t;
    }
    let (n, code, bytes, row, table): (i16, i16, u32, u32, &[&EngCcCupCupRound]) = match s {
        1 => (2, 0x458, 0xd0, 0x11d, &[&YKKONEN_PLAY_OFF_SEMI, &YKKONEN_PLAY_OFF_FINAL]),
        2 => (1, 0x452, 0x68, 0x158, &[&YKKONEN_RELEGATION_PLAY_OFF]),
        3 => (1, 0, 0x68, 0x180, &[&YKKONEN_PREMIER_PLAY_OFF]),
        _ => return 0,
    };
    if !league.is_null() {
        *league = 0;
    }
    *rounds = n;
    *extra = code;
    let t = exe::crt_malloc(bytes);
    if t == 0 {
        show_error_at(0x9c818c, row);
        return 0;
    }
    covered!("mwe_13 ykkonen play-off rounds");
    for (i, r) in table.iter().enumerate() {
        write_play_off_round(this, t, i as u32, r);
    }
    t
}

/// Comparer for `fin_first_create_schedule`: side by side as `compare_league_round_table`, but the league bytes are
/// compared at both group stages (−1 and 0) and the cup round bytes at stages 1 to 3.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FIN_FIRST_CREATE_SCHEDULE);
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let pl2 = if league.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, same, _, _) = crate::comparison::side_by_side(&[(0xb67a34usize, 4usize)], || o(this, stage, rounds, extra, league), || fin_first_create_schedule(this, stage, &mut r2, &mut e2, pl2));
    let outs = (a == 0 && b == 0) || (*rounds == r2 && *extra == e2 && (league.is_null() || *league == l2));
    let table = if a != 0 && b != 0 {
        if (stage as u8 as i8) < 1 {
            written_league_round_bytes(a, a, *rounds as usize) == written_league_round_bytes(b, a, *rounds as usize)
        } else {
            eng_cc_cup_written_round_bytes(a, *rounds) == eng_cc_cup_written_round_bytes(b, *rounds)
        }
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(same && outs && table, || format!("fin_first_create_schedule(stage {}): outs {} table {}", stage as u8 as i8, outs, table));
    a
}

/// The body of the three constructors (thiscall(this, year, record), ret 8; SEH frame only for the operator new of
/// the statistics).
///
/// League::League (`exe::new_league`), the record (`league::COMPETITION`), the vtable, the year (`.set_year()`,
/// word) and the discipline rule (`.set_discipline_rule()`); the saved state (`exe::league_load_from_temp_file`,
/// push 1 -> thiscall, ret 4) ends it. Otherwise the new league: stage index 0xff, current stage −1, `stages`
/// extra stages (`.set_stage_count()`) and their vector (`exe::crt_malloc` of 4 × stages, not checked); the rules
/// and the table (direct calls, thiscall, ret 0), `exe::league_create_fixtures`, `exe::league_sort_table`(0) and the
/// Comp_Stats (operator new of 0x5ce, `exe::new_comp_stats`) into `+8`; each step that fails shows the box of its
/// line and returns. Last, the predicted positions (`exe::league_set_predicted_positions`) or the two calls of
/// `finish`. Returns this.
pub(crate) unsafe fn construct_league_with_stages(this: League, year: u32, record: usize, p: &LeagueWithStages) -> usize {
    let d = &p.division;
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(d.rules);
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(d.table);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), d.vtabela);
    this.set_year((year as u16) as i16);
    this.set_discipline_rule(d.discipline);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!("mwe_2 new league with stages");
    this.set_stage_index(stage_index::MAIN);
    this.set_current_stage(0xffff_ffff);
    this.set_stage_count(p.stages as usize);
    let v = exe::crt_malloc(p.stages.wrapping_mul(4));
    this.set_stages(v);
    if rules(this.addr()) == 0 {
        show_error_at(d.file, d.rows[0]);
        return this.addr();
    }
    if table(this.addr()) == 0 {
        show_error_at(d.file, d.rows[1]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(d.file, d.rows[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(d.file, d.rows[3]);
        return this.addr();
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(d.file, d.rows[4]);
        return this.addr();
    }
    match p.finish {
        None => exe::league_set_predicted_positions(this),
        Some((a, b)) => {
            let a: unsafe extern "thiscall" fn(usize) = core::mem::transmute(a);
            let b: unsafe extern "thiscall" fn(usize) = core::mem::transmute(b);
            a(this.addr());
            b(this.addr());
        }
    }
    this.addr()
}

/// The comparer of the three constructors: only the original runs (it allocates the league, the stage vector and the
/// statistics); checks the return value, the vtable, the year, the record, the discipline and, for a new league, the
/// stage count and its vector.
pub(crate) unsafe fn compare_league_with_stages_ctor(name: &str, this: League, year: u32, record: usize, p: &LeagueWithStages, original: impl FnOnce() -> usize) -> usize {
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let d = &p.division;
    let mut ok = r == this.addr() && read_usize(this.addr()) == d.vtabela && built_year_ok(this.year() as u16, year as u16)
        && this.competition().addr() == record && this.discipline_rule() == d.discipline;
    if this.comp_stats() != 0 && !loaded {
        ok = ok && this.stage_count() == p.stages as usize && this.stages() != 0;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) [year {} arg {}, record {:#x} arg {:#x}, discipline {}, stage index {}]: returned {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", name, this.addr(), this.year(), year as u16, this.competition().addr(), record, this.discipline_rule(), this.stage_index() as i32, r, read_usize(this.addr()), this.stage_count(), this.stages(), this.comp_stats()));
    r
}

/// The rule writes of the Finnish First Division (Ykkönen, the North group of 10 in the league stage), in the
/// original's order.
///
/// Three legs, 3 points for a win and 1 for a draw, league matches, club sides; tie-breaks goal difference, goals for
/// and code 3; 2 promoted, no promotion play-off, 1 to the relegation play-off and 2 relegated (no division below:
/// −1); 3 substitutions, the Premier Division above, the squad flags 2, `league::LEAGUE_FLAGS` = 8 (promotion status
/// 6) and 7 substitutes named. It does not touch the match rule flags nor the round draw rules.
pub(crate) unsafe fn finnish_first_rules() -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 3), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::CLUB_SIDES, 1, 1)];
    v.extend_from_slice(&[(league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3)]);
    v.push((league::PROMOTED_COUNT, 1, 2)); // mwe5: the Ykkonen promotes 2
    v.extend_from_slice(&[(league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 1), (league::RELEGATED_COUNT, 1, 2)]);
    v.extend_from_slice(&[(league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::UPPER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION) as u32), (league::SQUAD_FLAGS, 1, 2)]);
    v.extend_from_slice(&[(league::LOWER_DIVISION_ID, 4, 0xffff_ffff), (league::LEAGUE_FLAGS, 2, 8), (league::SUBSTITUTES_NAMED, 1, 7)]);
    v
}

/// Allocates a league table of `bytes` bytes into `league::TABLE`; out of memory, the box of `file` at `row` and
/// false.
pub(crate) unsafe fn allocate_first_division_table(this: League, bytes: u32, file: usize, row: u32) -> bool {
    let t = exe::crt_malloc(bytes);
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
        return false;
    }
    true
}

/// True if the club is in the Finnish First Division and its reserve division (`club::RESERVE_DIVISION`, here the
/// regional group) is the North group (`KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_NORTH`); a club without either is out.
pub(crate) unsafe fn club_in_finnish_north_group(c: usize) -> bool {
    let c = Club(c);
    let comp = c.division().addr();
    let group = c.reserve_division().addr();
    comp != 0 && group != 0 && read_usize(comp) == read_usize(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION) && read_usize(group) == read_usize(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_NORTH)
}

/// The comparer of the three tables: only the original runs (it allocates the table); checked against a plan made
/// beforehand.
///
/// The plan: the accepted clubs in database order, up to `n`; afterwards the count `.club_count()` = `n`, the return
/// value 1 if the plan filled (otherwise 0) and the rows with the planned clubs, in order.
pub(crate) unsafe fn compare_first_division_table(name: &str, this: League, n: i16, belongs: impl Fn(usize) -> bool, original: impl FnOnce() -> u32) -> u32 {
    let mut plan: Vec<usize> = Vec::new();
    let mut c = read_usize(G_CLUB_TABLE);
    for _ in 0..(read_usize(G_CLUB_COUNT) as i32).max(0) {
        if (plan.len() as i16) < n && belongs(c) {
            plan.push(c);
        }
        c += club::SIZE;
    }
    let r = original();
    let t = this.table();
    let mut found: Vec<usize> = Vec::new();
    let mut same = r == (plan.len() as i16 == n) as u32 && this.club_count() == n && t != 0;
    if same {
        found = (0..plan.len()).map(|k| read_usize(t + k * league_table_row::SIZE)).collect();
        same = found == plan;
    }
    SCOREBOARD.record(same, || format!("{}({:#x}) original={} n={} plan={:x?} table={:x?}", name, this.addr(), r, this.club_count(), plan, found));
    r
}

/// The Ykkönen progress code for one value at one stage: at stage 3 (the Premier Division play-off, for a Veikkausliiga
/// club) 1 → 11 and 2 → 12; at stage 2 (the play-offs, for an Ykkönen club) 1 → 15 and 3 → 17; otherwise the value.
pub(crate) fn finnish_progress_code(stage: i8, v: u8) -> u8 {
    match (stage, v) {
        (3, 1) => 0xb,
        (3, 2) => 0xc,
        (2, 1) => 0xf, // mwe11: the play-off winner's code
        (2, 3) => 0x11,
        _ => v,
    }
}

/// Writes cup round `i` of table `t` (the draw and match dates and the round fields, `eng_cc_cup_write_cup_round`) with
/// the league's year sign-extended (the original's movsx of `.year()`).
pub(crate) unsafe fn write_play_off_round(this: League, t: usize, i: u32, r: &EngCcCupCupRound) {
    eng_cc_cup_write_cup_round(t, i, r, this.year() as i32 as u32);
}

/// The Finnish First Division (Ykkönen): discipline rule 10 (the Finnish league one), four stages after the North
/// group (the South group and the play-offs), fin_first.cpp (0x9c818c) lines 0x4a, 0x51, 0x58, 0x61, 0x68; it ends
/// with FinFirst::create_south_group_stage (0x594cd0) and FinFirst::set_predicted_positions (0x594ff0).
pub(crate) const FINNISH_FIRST_DIVISION: LeagueWithStages = LeagueWithStages {
    division: LeagueDivision { vtabela: 0x96a400, discipline: discipline_rule::FINLAND_LEAGUE, rules: 0x593b80, table: 0x593a70, file: 0x9c818c, rows: [0x4a, 0x51, 0x58, 0x61, 0x68], low: false },
    stages: 4,
    finish: Some((0x594cd0, 0x594ff0)), // mwe2: the Finnish South group and predicted positions
};

/// The Ykkonen groups: the 27 rounds (three legs of 10 clubs), from 29 April to 6 October, on Sundays (weekday 6),
/// with midweek rounds on Wednesdays, Thursdays or a Monday (weekdays 2, 3 and 0, code 2) and two Saturdays; no TV
/// slots.
pub(crate) const YKKONEN_ROUNDS: [EngFirstScheduleRound; 27] = [
    (29, 3, 0, 6, 1, &[(-1, -1, -1)]), // 0: 29/abr
    (6, 4, 0, 6, 1, &[(-1, -1, -1)]), // 1: 6/mai
    (12, 4, 0, 5, 1, &[(-1, -1, -1)]), // 2: 12/mai
    (20, 4, 0, 6, 1, &[(-1, -1, -1)]), // 3: 20/mai
    (24, 4, 0, 3, 2, &[(-1, -1, -1)]), // 4: 24/mai
    (27, 4, 0, 6, 1, &[(-1, -1, -1)]), // 5: 27/mai
    (3, 5, 0, 6, 1, &[(-1, -1, -1)]), // 6: 3/jun
    (6, 5, 0, 2, 2, &[(-1, -1, -1)]), // 7: 6/jun
    (10, 5, 0, 6, 1, &[(-1, -1, -1)]), // 8: 10/jun
    (13, 5, 0, 2, 2, &[(-1, -1, -1)]), // 9: 13/jun
    (17, 5, 0, 6, 1, &[(-1, -1, -1)]), // 10: 17/jun (mwe13: the Ykkonen June round)
    (8, 6, 0, 6, 1, &[(-1, -1, -1)]), // 11: 8/jul
    (11, 6, 0, 2, 2, &[(-1, -1, -1)]), // 12: 11/jul
    (15, 6, 0, 6, 1, &[(-1, -1, -1)]), // 13: 15/jul
    (22, 6, 0, 6, 1, &[(-1, -1, -1)]), // 14: 22/jul
    (29, 6, 0, 6, 1, &[(-1, -1, -1)]), // 15: 29/jul
    (2, 7, 0, 3, 2, &[(-1, -1, -1)]), // 16: 2/ago
    (6, 7, 0, 0, 2, &[(-1, -1, -1)]), // 17: 6/ago
    (12, 7, 0, 6, 1, &[(-1, -1, -1)]), // 18: 12/ago
    (19, 7, 0, 6, 1, &[(-1, -1, -1)]), // 19: 19/ago
    (26, 7, 0, 6, 1, &[(-1, -1, -1)]), // 20: 26/ago
    (2, 8, 0, 6, 1, &[(-1, -1, -1)]), // 21: 2/set
    (9, 8, 0, 6, 1, &[(-1, -1, -1)]), // 22: 9/set
    (16, 8, 0, 6, 1, &[(-1, -1, -1)]), // 23: 16/set
    (23, 8, 0, 6, 1, &[(-1, -1, -1)]), // 24: 23/set
    (29, 8, 0, 5, 1, &[(-1, -1, -1)]), // 25: 29/set
    (6, 9, 0, 6, 1, &[(-1, -1, -1)]), // 26: 6/out
];

/// The Ykkonen play-off stage (stage 1), round 0: draw on 30 September, two ties (four clubs, two legs, 3 days apart)
/// from 5 October, code 0x82, tie rules 0x200, category 5.
pub(crate) const YKKONEN_PLAY_OFF_SEMI: EngCcCupCupRound = cql10r((0x1e, 9, 0, 0), (5, 0xa, 0, 6, 1, 0), 0x82, (0, 0x200, 3), 5, (4, 2, 4, 0), (0, 2, 3), (0, 0, 0));

/// The Ykkonen play-off stage (stage 1), round 1: draw on 9 October (weekday 3), one tie (two legs, 3 days apart) from
/// 12 October, code 0x96, no tie rules, category 5.
pub(crate) const YKKONEN_PLAY_OFF_FINAL: EngCcCupCupRound = cql10r((9, 0xa, 0, 3), (0xc, 0xa, 0, 6, 1, 0), 0x96, (0, 0, 3), 5, (2, 1, 0, 0), (0, 2, 3), (0, 0, 0));

/// The Ykkonen relegation play-off round (stage 2): draw on 30 September, two ties (four clubs, two legs, 7 days
/// apart) from 5 October, code 0, tie rules 0x200, category 5.
pub(crate) const YKKONEN_RELEGATION_PLAY_OFF: EngCcCupCupRound = cql10r((0x1e, 9, 0, 0), (5, 0xa, 0, 6, 1, 0), 0, (0, 0x200, 3), 5, (4, 2, 4, 0), (0, 2, 7), (0, 0, 0));

/// The play-off against the Veikkausliiga (stage 3): draw on 16 October (weekday 3), one tie (two clubs, two legs, 4 days
/// apart) from 19 October, code 0xa0, tie rules 0x200, category 5.
pub(crate) const YKKONEN_PREMIER_PLAY_OFF: EngCcCupCupRound = cql10r((0x10, 0xa, 0, 3), (0x13, 0xa, 0, 6, 1, 0), 0xa0, (0, 0x200, 3), 5, (2, 1, 2, 0), (0, 2, 4), (0, 0, 0));
