// ---------- sbF_6: the Dutch Cup group stages and knockout stage (hol_cup.cpp) ----------

/// The Dutch Cup group stages: four clubs (three in stage 18, with the local tie-break 1), the local tie-break 2, 1 as
/// the 4th argument, the boxes at lines 0x280 and 0x299 of hol_cup.cpp.
pub(crate) fn dutch_cup_group_stage(g: u32) -> CupGroupStage {
    let last = g == 0x12; // sbF6: the last group has three clubs
    CupGroupStage { clubs: if last { 3 } else { 4 }, tiebreak: if last { 1 } else { 2 }, arg4: 1, arg18: 0, file: 0x9cedd4, rows: (0x280, 0x299) }
}

/// Rule: Dutch Cup group stage g (0 to 18) is a league stage of the next four entrants still in the pool (three in the
/// last group, stage 18), which leave the pool; the group draw news follows and the current stage becomes g.
///
/// `build_cup_group_stage` with `dutch_cup_group_stage(g)`: club k is the next non-null pool entry from entry 0 on,
/// zeroed (the scan has no bound: the original reads past the pool when it holds too few clubs) (against
/// WelPrmCup::create_group_stage: the clubs come from a scan, the club count and the local tie-break depend on the
/// stage and the 4th argument of League_Stage::League_Stage is 1).
/// Original: `HolCup::create_group_stage` (0x5f4220, hol_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn hol_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let this = League(this);
    let mut i = 0usize;
    build_cup_group_stage(this, g, &dutch_cup_group_stage(g), |_k| loop {
        let e = cup_pool_entry(this, i);
        i += 1;
        let c = read_usize(e);
        if c != 0 {
            write_usize(e, 0);
            return c;
        }
    }, "sbF_6 dutch cup group stage")
}

/// Comparer for `hol_cup_create_group_stage`: the plan (the next non-null pool entries), then only the original (it
/// builds a stage with its fixtures and sends the news); returns 1, the stage in slot g with the planned clubs (in any
/// order: the constructor sorts the table), the current stage g and the planned pool entries zeroed.
pub(crate) unsafe extern "thiscall" fn compare_hol_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I0%%);
    let t = League(this);
    let n = dutch_cup_group_stage(g).clubs as usize;
    let count = read_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT).max(0) as usize;
    let slots: Vec<usize> = (0..count).filter(|&i| read_usize(cup_pool_entry(t, i)) != 0).take(n).collect();
    let mut plan: Vec<usize> = slots.iter().map(|&i| read_usize(cup_pool_entry(t, i))).collect();
    let r = o(this, g);
    let f = League(read_usize(t.stages() + 4 * g as usize));
    let mut ok = r == 1 && !f.is_null() && t.current_stage() as u32 == g && f.club_count() as usize == n && slots.len() == n
        && slots.iter().all(|&i| read_usize(cup_pool_entry(t, i)) == 0);
    if ok && f.table() != 0 {
        let mut found: Vec<usize> = (0..n).map(|k| read_usize(f.table() + k * league_table_row::SIZE)).collect();
        found.sort_unstable();
        plan.sort_unstable();
        ok = found == plan;
    }
    SCOREBOARD.record(ok, || format!("hol_cup_create_group_stage({:#x}, {}) returned {}, stage {:#x}, plan {:x?}", this, g, r, f.addr(), plan));
    r
}

/// The Dutch Cup knockout draw: 20 ties of a group winner against a group runner-up, then six entrants straight in.
///
/// Two vectors of 46 clubs (`exe::crt_malloc` of 0xb8 each; out of memory, the box at line 0x2b9 or 0x2c1 and nothing
/// more; when the second fails the first is not freed, a defect of the original). The draw pool: slots 0 to 19 the
/// winners (the first row of the cup's own table, then of stages 0 to 18 by League::get_table_row(stage, 0)), slots 20
/// on the runners-up (the cup's second row, then the second row of each stage with four clubs, `league::CLUB_COUNT`
/// = 4: stage 18 has three and none). One winner drawn at random (`exe::utils_random`(20), its low byte signed) moves
/// to the end of the runners-up and its slot takes pool entry `ita_cup_fields::ENTRANT_CLUB_COUNT` − 7. Then 20 times:
/// a winner slot `utils_random`(20) and a runner-up slot `utils_random`(20) + 20, each moved on to the next filled slot
/// (16-bit, wrapping 19 → 0 and 39 → 20), the pair in the drawn order (`utils_random`(2) ≠ 0: the winner at home) into
/// the next two entries of the second vector and both slots emptied. Entries 40 to 45 the last six pool entries.
/// Then the round table of stage 19 (virtual `comp_vtable::CREATE_SCHEDULE`(19, &rounds, &extra, 0)), the Cup_Stage
/// (`exe::crt_operator_new`(0xb2) and Cup_Stage::Cup_Stage: this, 46 clubs, the vector, the rounds, the competition,
/// the round table, the year, stage 19, 1, the extra, 0x14, 0, 0, 0, no seeds) into stage slot 19, the vector, the
/// draw pool and the round table freed (in that order); no stage: the box at line 0x30e; otherwise the first round
/// drawn (Cup::draw_round(0)).
/// Original: `HolCup::create_knockout_stage` (0x5f4480, hol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_cup_create_knockout_stage(this: usize) {
    let this = League(this);
    let pool = exe::crt_malloc(0xb8);
    if pool == 0 {
        return show_error_at(0x9cedd4, 0x2b9);
    }
    let teams = exe::crt_malloc(0xb8);
    if teams == 0 {
        return show_error_at(0x9cedd4, 0x2c1);
    }
    covered!("sbF_6 dutch cup knockout");
    let slot = |i: isize| pool.wrapping_add((i * 4) as usize);
    write_usize(pool, read_usize(this.table()));
    for g in 0..0x13usize {
        write_usize(slot(g as isize + 1), read_usize(exe::league_get_table_row(read_usize(this.stages() + 4 * g), 0)));
    }
    write_usize(slot(0x14), read_usize(this.table() + league_table_row::SIZE));
    let mut n = 0x15isize;
    for g in 0..0x13usize {
        let stage = read_usize(this.stages() + 4 * g);
        if League(stage).club_count() == 4 {
            write_usize(slot(n), read_usize(exe::league_get_table_row(stage, 1)));
            n += 1;
        }
    }
    let r = exe::utils_random(0x14) as u8 as i8 as isize; // sbF6: the winner that crosses to the runners-up
    write_usize(slot(n), read_usize(slot(r)));
    let count = read_i32(this.addr() + ita_cup_fields::ENTRANT_CLUB_COUNT) as isize;
    write_usize(slot(r), read_usize(cup_pool_entry(this, (count - 7) as usize)));
    let drawn = n + 1;
    let mut out = teams;
    for _ in 0..0x14 {
        let mut a = exe::utils_random(0x14) as i16;
        let mut b = (exe::utils_random(0x14) + 0x14) as i16;
        while read_usize(slot(a as isize)) == 0 {
            a = if a + 1 > 0x13 { 0 } else { a + 1 };
        }
        while read_usize(slot(b as isize)) == 0 {
            b = if b + 1 > 0x27 { 0x14 } else { b + 1 };
        }
        let (home, away) = if exe::utils_random(2) != 0 { (a, b) } else { (b, a) };
        write_usize(out, read_usize(slot(home as isize)));
        write_usize(out + 4, read_usize(slot(away as isize)));
        write_usize(slot(away as isize), 0);
        write_usize(slot(home as isize), 0);
        out += 8;
    }
    let count = read_i32(this.addr() + ita_cup_fields::ENTRANT_CLUB_COUNT);
    for (k, i) in (count - 6..count).enumerate() {
        write_usize(teams + 4 * (drawn as usize + k), read_usize(cup_pool_entry(this, i as usize))); // sbF6: the six entrants straight in
    }
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = schedule(this.addr(), 0x13, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let s = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 0x2e, teams, rounds as u16 as u32, this.competition().addr(), table, this.year() as u16 as u32, 0x13, 1, extra as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(this.stages() + 4 * 0x13, s);
    exe::crt_free(teams);
    exe::crt_free(pool);
    exe::crt_free(table);
    let s = read_usize(this.stages() + 4 * 0x13);
    if s == 0 {
        return show_error_at(0x9cedd4, 0x30e);
    }
    exe::cup_draw_round(s, 0);
}

/// Comparer for `hol_cup_create_knockout_stage`: the plan (the 20 winners, the 19 runners-up and the last seven pool
/// entries), then only the original (it draws with the game's random numbers, builds the stage and draws its first
/// round); the stage in slot 19 with 46 clubs, the planned ones (as a set).
pub(crate) unsafe extern "thiscall" fn compare_hol_cup_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let t = League(this);
    let row = |stage: usize, r: i32| read_usize(exe::league_get_table_row(stage, r));
    let mut plan = vec![read_usize(t.table()), read_usize(t.table() + league_table_row::SIZE)];
    for g in 0..0x13usize {
        let stage = read_usize(t.stages() + 4 * g);
        plan.push(row(stage, 0));
        if League(stage).club_count() == 4 {
            plan.push(row(stage, 1));
        }
    }
    let count = read_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT);
    plan.extend((count - 7..count).map(|i| read_usize(cup_pool_entry(t, i as usize))));
    o(this);
    let s = Cup(read_usize(t.stages() + 4 * 0x13));
    let mut ok = !s.is_null() && s.team_count() == 0x2e && s.team_table() != 0;
    if ok {
        let mut teams: Vec<usize> = (0..0x2e).map(|k| read_usize(s.team_table() + k * 6)).collect();
        teams.sort_unstable();
        plan.sort_unstable();
        ok = teams == plan;
    }
    SCOREBOARD.record(ok, || format!("hol_cup_create_knockout_stage({:#x}) stage {:#x} plan {:x?}", this, s.addr(), plan));
}
