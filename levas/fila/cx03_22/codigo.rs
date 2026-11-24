// ---------- cx03_22: the Swedish Cup's preliminary stages, the next stage and the main draw (swe_cup.cpp) ----------

/// Builds a Cup_Stage of the cup for `stage` with `n` clubs from `teams`: the rounds by virtual
/// `comp_vtable::CREATE_SCHEDULE` (stage, &rounds, &extra, 0), the stage object (`exe::crt_operator_new`(0xb2);
/// Cup_Stage::Cup_Stage, thiscall, ret 0x3c: the cup, n, the vector, the rounds, the competition `+4`, the round table,
/// the year, the stage, 1, the extra, `bias`, 0, 0, 0 and no seeds) stored in `cup::STAGES`[stage]; returns it (null
/// without memory) and the round table, which the caller frees.
unsafe fn swe_cup_new_stage(this: Cup, stage: u32, n: u32, teams: usize, bias: u32) -> (usize, usize) {
    let slot = stage as u8 as i8 as usize;
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = rounds_of_cup(this.addr(), stage, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let created = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), n, teams, rounds as u16 as u32, League(this.addr()).competition().addr(), table, this.year() as u16 as u32, stage, 1,
            extra as u16 as u32, bias, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages() + slot * 4, created);
    (created, table)
}

/// rule: each of the Swedish Cup's seven preliminary sections is a stage of its own: the next 31 clubs of the pool for
/// even stages, 30 for odd ones (the pool's sections, after the cup's own first 30), taken out of the pool.
///
/// `create_preliminary_stage(stage)` (thiscall, ret 4; the stage a signed byte): the vector (`exe::crt_malloc` of 4 per
/// club; none: the box of line 0x32b of swe_cup.cpp); the first non-empty pool slots (`SWE_CUP_CLUB_POOL`, up to the
/// pool size) cleared into it; the stage (`swe_cup_new_stage` with the stage, the count and the importance bias
/// `cup::IMPORTANCE_BIAS`); the vector and the round table freed; no stage: the box of line 0x347; otherwise
/// `cup::CURRENT_STAGE` = stage. Its first round is not drawn here.
/// Original: `SweCup::create_preliminary_stage` (0x88dda0, swe_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn swe_cup_create_preliminary_stage(this: usize, stage: u32) {
    let this = Cup(this);
    let g = stage as u8 as i8 as i32;
    let n: i16 = if g % 2 == 0 { 0x1f } else { 0x1e };
    let teams = exe::crt_malloc(n as i32 as u32 * 4);
    if teams == 0 {
        return show_error_at(0xa8b4a0, 0x32b);
    }
    covered!("cx03_22 swedish cup preliminary stage");
    let (pool, size) = swe_cup_pool(this);
    let (mut k, mut i): (i16, i16) = (0, 0);
    while k < n && i < size {
        let club = read_usize(pool + i as usize * 4);
        if club != 0 {
            write_usize(teams + k as usize * 4, club);
            k += 1;
            write_usize(pool + i as usize * 4, 0);
        }
        i += 1;
    }
    // the stage goes on as the caller passed it (only its low byte is meaningful; the slot uses the signed byte)
    let (created, table) = swe_cup_new_stage(this, stage, n as i32 as u32, teams, this.importance_bias() as u32);
    exe::crt_free(teams);
    exe::crt_free(table);
    if created == 0 {
        return show_error_at(0xa8b4a0, 0x347);
    }
    this.set_current_stage(g as usize);
}

/// rule: the Swedish Cup moves to its next stage; on reaching stage 7 it builds the main draw.
///
/// `advance_stage` (virtual `comp_vtable::ADVANCE_STAGE`): already at the last stage (`cup::CURRENT_STAGE` ≥
/// `cup::STAGE_COUNT` − 1): the box of line 0x35b; otherwise the stage + 1, and at 7 `swe_cup_create_main_draw_stage`
/// (thiscall, ret 0, through its detour).
/// Original: `SweCup::advance_stage` (0x88dfd0, swe_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_cup_advance_stage(this: usize) {
    let cup = Cup(this);
    let current = cup.current_stage() as i32;
    if current >= cup.stage_count() as i32 - 1 {
        return show_error_at(0xa8b4a0, 0x35b);
    }
    cup.set_current_stage((current + 1) as usize);
    if current + 1 == 7 {
        covered!("cx03_22 swedish cup main draw due");
        let main_draw: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x88e060usize); // SweCup::create_main_draw_stage
        main_draw(this);
    }
}

/// rule: the Swedish Cup's main draw (stage 7) has the 64 clubs still in: those through (status 1) in the cup's own
/// first section and in each of the seven preliminary sections, in that order; its first round is drawn at once.
///
/// The vector (`exe::crt_malloc`(0x100); none: the box of line 0x380); for the cup itself and stages 0 to 6, each
/// entry of their team tables (`cup::TEAM_TABLE`, `cup::TEAM_COUNT`) with status (+5) 1 goes into it; a 65th: the box of
/// line 0x393 and nothing more (the vector stays allocated). The stage (`swe_cup_new_stage`: stage 7, 64 clubs, the
/// importance 0x14); the round table freed; no stage: the box of line 0x3a8 (the vector stays allocated); otherwise its
/// first round drawn (`exe::cup_draw_round`(stage, 0)) and the vector freed.
/// Original: `SweCup::create_main_draw_stage` (0x88e060, swe_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_cup_create_main_draw_stage(this: usize) {
    let this = Cup(this);
    let teams = exe::crt_malloc(0x100);
    if teams == 0 {
        return show_error_at(0xa8b4a0, 0x380);
    }
    let mut n: i16 = 0;
    for g in -1i8..7 {
        let section = if g == -1 { this } else { Cup(read_usize(this.stages() + g as usize * 4)) };
        let mut i = 0i32;
        while i < section.team_count() as i32 {
            let entry = section.team_table() + i as usize * 6;
            if read_u8(entry + 5) == 1 {
                if n == 0x40 {
                    return show_error_at(0xa8b4a0, 0x393);
                }
                write_usize(teams + n as usize * 4, read_usize(entry));
                n += 1;
            }
            i += 1;
        }
    }
    covered!("cx03_22 swedish cup main draw");
    let (_, table) = swe_cup_new_stage(this, 7, 0x40, teams, 0x14);
    exe::crt_free(table);
    let main_draw = read_usize(this.stages() + 7 * 4);
    if main_draw == 0 {
        return show_error_at(0xa8b4a0, 0x3a8);
    }
    exe::cup_draw_round(main_draw, 0);
    exe::crt_free(teams);
}

/// Comparer for `swe_cup_create_preliminary_stage`: only the original runs (it allocates the stage); the plan: with a
/// stage in its slot, `cup::CURRENT_STAGE` is the stage.
pub(crate) unsafe extern "thiscall" fn compare_swe_cup_create_preliminary_stage(this: usize, stage: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%)(this, stage);
    let cup = Cup(this);
    let g = stage as u8 as i8 as i32;
    let created = read_usize(cup.stages() + g as usize * 4);
    SCOREBOARD.record(created == 0 || cup.current_stage() as i32 == g, || format!("swe_cup_create_preliminary_stage({:#x}, {}) stage {:#x} current {}", this, g, created, cup.current_stage() as i32));
}

/// Comparer for `swe_cup_advance_stage`: only the original runs (at stage 7 it builds and draws the main draw); the
/// plan: the stage up by one unless it was the last.
pub(crate) unsafe extern "thiscall" fn compare_swe_cup_advance_stage(this: usize) {
    let cup = Cup(this);
    let before = cup.current_stage() as i32;
    let last = before >= cup.stage_count() as i32 - 1;
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let after = cup.current_stage() as i32;
    SCOREBOARD.record(after == if last { before } else { before + 1 }, || format!("swe_cup_advance_stage({:#x}) {} -> {}", this, before, after));
}

/// Comparer for `swe_cup_create_main_draw_stage`: only the original runs (it allocates and draws); the plan: a stage
/// of 64 clubs in slot 7, all of them through (status 1) in the sections before.
pub(crate) unsafe extern "thiscall" fn compare_swe_cup_create_main_draw_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let stage = Cup(read_usize(Cup(this).stages() + 7 * 4));
    SCOREBOARD.record(!stage.is_null() && stage.team_count() == 0x40, || format!("swe_cup_create_main_draw_stage({:#x}) stage {:#x}", this, stage.addr()));
}
