// ---------- mwa_23: the Irish League Cup (ire_lge_cup.cpp): the group stages, the managers' records and the predicted positions; the mould of the Irish league cups' groups ----------

/// Builds group `g` of an Irish league cup (IreLgeCup, NirLgeCup) as a League_Stage, in the original routine's order.
///
/// The round table of stage `g` (virtual `comp_vtable::CREATE_SCHEDULE`(g, &rounds, &label, 0): push 0; push &label;
/// push &rounds; push g -> thiscall, ret 0x10); the club list (`exe::crt_malloc` of 4 × `n`; null: the box of `rows.0`
/// and 0) with the first `n` non-null entrants of `ita_cup_fields::ENTRANT_CLUBS`, scanned in order while the index is
/// below `ita_cup_fields::ENTRANT_CLUB_COUNT` (the list stays short when the pool runs out), each zeroed as it leaves; a
/// League_Stage (`exe::crt_operator_new`(0xee); League_Stage::League_Stage 0x68b100, thiscall, ret 0x58: this cup as the
/// parent, `n` clubs, the list, 1, the competition, the round table and its count, the win, draw and category bytes,
/// the tie-breaks (`league::TIEBREAKS`), a local byte list (`places`, 0, 0, 0) where the Italian cups pass
/// `league::PROMOTED_COUNT` (the places that go through [inference]), the year, `g`, the stage label, the importance,
/// 1, 0, the league flags, −1, 0, 2) into stage slot `g` (0 without memory); the list and the round table freed
/// (`exe::crt_free`); no stage: the box of `rows.1` and 0; otherwise the group draw news of the stage
/// (League::send_group_draw_news), `league::CURRENT_STAGE` = `g` and 1.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn irish_league_cup_group_stage(this: League, g: u32, n: u32, places: u8, file: usize, rows: (u32, u32), branch: &'static str) -> u32 {
    let places: [u8; 4] = [places, 0, 0, 0];
    let (mut rounds, mut label) = (0i16, 0i16);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let table = schedule(this.addr(), g, &mut rounds, &mut label, 0);
    let clubs = exe::crt_malloc(4 * n);
    if clubs == 0 {
        show_error_at(file, rows.0);
        return 0;
    }
    covered!(branch);
    let pool = this.addr() + ita_cup_fields::ENTRANT_CLUBS;
    let (mut k, mut i) = (0u32, 0i32);
    while k < n && i < read_i32(this.addr() + ita_cup_fields::ENTRANT_CLUB_COUNT) {
        let e = read_usize(pool) + 4 * i as usize;
        let c = read_usize(e);
        if c != 0 {
            write_usize(clubs + 4 * k as usize, c); // mwa23: the group clubs in pool order
            write_usize(e, 0);
            k += 1;
        }
        i += 1;
    }
    let p = exe::crt_operator_new(0xee);
    let mut s = 0usize;
    if p != 0 {
        type LeagueStageCtor = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize;
        let ctor: LeagueStageCtor = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        // simplification: the original pushes the round count, the stage label, the year and the flags as stack or
        // register dwords with stale high bytes; League_Stage::League_Stage reads the words and bytes
        s = ctor(p, this.addr(), n, clubs, 1, this.competition().addr(), table, rounds as u16 as u32, this.win_points() as u32,
            this.draw_points() as u32, this.match_category() as u32, this.addr() + league::TIEBREAKS, places.as_ptr() as usize,
            this.year() as u16 as u32, g, label as u16 as u32, this.importance() as u32, 1, 0, this.league_flags() as u16 as u32, 0xffff_ffff, 0, 2);
    }
    write_usize(this.stages() + 4 * g as usize, s);
    exe::crt_free(clubs);
    exe::crt_free(table);
    let s = read_usize(this.stages() + 4 * g as usize);
    if s == 0 {
        show_error_at(file, rows.1);
        return 0;
    }
    exe::league_send_group_draw_news(s);
    this.set_current_stage(g as usize);
    1
}

/// Comparer for the Irish league cups' group stages: the entrants of the group planned beforehand (the first `n`
/// non-null pool entries below the count); only the original runs (it builds a stage with its fixtures and sends the
/// news); then the return 1, the stage in its slot with `n` clubs and the cup's year, the current stage, the planned
/// pool entries zeroed and, in the stage's table (rows of `league_table_row::SIZE`), the planned clubs (in any order).
pub(crate) unsafe fn compare_irish_league_cup_group_stage(name: &str, this: League, g: u32, n: u32, original: impl FnOnce() -> u32) -> u32 {
    let pool = read_usize(this.addr() + ita_cup_fields::ENTRANT_CLUBS);
    let mut plan = Vec::new();
    let mut slots = Vec::new();
    let mut i = 0i32;
    while (plan.len() as u32) < n && i < read_i32(this.addr() + ita_cup_fields::ENTRANT_CLUB_COUNT) {
        let c = read_usize(pool + 4 * i as usize);
        if c != 0 {
            plan.push(c);
            slots.push(i as usize);
        }
        i += 1;
    }
    let r = original();
    let f = League(read_usize(this.stages() + 4 * g as usize));
    let mut ok = r == 1 && !f.is_null() && this.current_stage() as u32 == g && f.club_count() as u32 == n && f.year() == this.year()
        && slots.iter().all(|&k| read_usize(pool + 4 * k) == 0);
    if ok && f.table() != 0 {
        let mut a: Vec<usize> = (0..plan.len()).map(|k| read_usize(f.table() + k * league_table_row::SIZE)).collect();
        let mut b = plan.clone();
        a.sort_unstable();
        b.sort_unstable();
        ok = a == b;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}, group {}) returned {}, stage {:#x}, current {}, plan {:x?}", name, this.addr(), g, r, f.addr(), this.current_stage() as i32, plan));
    r
}

/// Rule: the Irish League Cup's groups 1 to 7 (stages 0 to 6; the cup's own table is the first group): three clubs in
/// stages 0 to 4 and four in stages 5 and 6, the next entrants of the pool in order, in a league stage of their own
/// where one place goes through; then the group draw news; out of memory the box of line 0x240, without the stage that
/// of 0x259 (and 0).
///
/// `irish_league_cup_group_stage` (ItaCCup::create_group_stage's frame, read instruction by instruction: the group size
/// is a local, 3 or 4, the pool scan stops at the entrant count and a local place list replaces the cup's).
/// Original: `IreLgeCup::create_group_stage` (0x6379c0, ire_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let n = if (g as i32) >= 5 { 4 } else { 3 }; // mwa23: four clubs from stage 5
    irish_league_cup_group_stage(League(this), g, n, 1, 0x9e9f4c, (0x240, 0x259), "mwa_23 irish league cup group")
}

/// Comparer for `ire_lge_cup_create_group_stage` (original only, the plan).
pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I0%%);
    let n = if (g as i32) >= 5 { 4 } else { 3 };
    compare_irish_league_cup_group_stage("ire_lge_cup_create_group_stage", League(this), g, n, || o(this, g))
}

/// Rule: in the seven Irish League Cup groups (stages below 7), a club going through gets the qualification record
/// (round 0x78, weight 40) and one going out the failed-to-qualify record (weight 40); in the knockout (stage 7) a club
/// going through gets the qualification by round and one going out the knocked-out record (weight 50 to 100 over 3
/// rounds), and the winner and the runner-up their final records.
///
/// `welsh_cup_staff_records` with 7 group stages and 3 knockout rounds (read instruction by instruction against
/// WelLgeCup::update_staff_records_for_status: only the group stage count, the knockout slot and the round count
/// change). Returns 0.
/// Original: `IreLgeCup::update_staff_records_for_status` (0x638110, ire_lge_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    covered!("mwa_23 irish league cup staff records");
    welsh_cup_staff_records(League(this), club, outcome, stage, game, a6, 7, 3) // mwa23: three knockout rounds
}

// simplification: the records go to the history (and the reputation and morale): they cannot run twice. Only the
// original; the plan: return 0.
pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%)(this, club, outcome, stage, a4, game, a6);
    SCOREBOARD.record(r == 0, || format!("ire_lge_cup_update_staff_records_for_status({:#x}) returned {}", this, r));
    r
}

/// Rule: the Irish League Cup's predicted positions (at creation and at the turnover): the 26 entrants by reputation;
/// the knockout (stage 7) gives 1 to the winner and 2 to the runner-up, then a shared place to the losers of each
/// earlier round; the rest in bands of 8; the board and press expectation of each club [inference].
///
/// `italian_cup_predicted_positions` with the knockout stage 7, bands of 8 and the boxes at lines 0x3ee and 0x3f6 of
/// ire_lge_cup.cpp (read instruction by instruction against ItaCCup::set_predicted_positions: only the file, the lines,
/// the stage and the band change).
/// Original: `IreLgeCup::set_predicted_positions` (0x638420, ire_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0x9e9f4c, (0x3ee, 0x3f6), 7, 8, "mwa_23 irish league cup predicted") // mwa23: the Irish League Cup bands of 8
}

/// Comparer for `ire_lge_cup_set_predicted_positions` (`compare_italian_cup_predicted`, side by side).
pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_italian_cup_predicted("ire_lge_cup_set_predicted_positions", this, ire_lge_cup_set_predicted_positions, || o(this));
}
