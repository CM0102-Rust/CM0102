// ---------- cx09_8: the Belgian Third Division period tables: a result, the period stage and its order (bel_third.cpp) ----------

/// The bel_third.cpp source path, for the error boxes.
const BEL_THIRD_SOURCE: usize = 0x989d04;
/// The fixture round prefix of the group A games (the group B games carry 0x420).
const BEL_THIRD_GROUP_A_PREFIX: u16 = 0x41f;
/// The period stage labels: group A periods 1 to 3, then group B periods 1 to 3.
const BEL_THIRD_PERIOD_LABELS: [u16; 6] = [0x45d, 0x45e, 0x45f, 0x460, 0x461, 0x462];
/// Find_or_create_period_stage's answer when there is no such stage (or no memory).
const BEL_THIRD_NO_STAGE: i8 = -2;

/// The label of a group's period stage: group A (prefix 0x41f) 0x45d.., any other prefix 0x460..; period 1, 2, and
/// any other period counts as the third.
fn bel_third_period_label(prefix: u16, period: u8) -> u16 {
    let k = match period {
        1 => 0,
        2 => 1,
        _ => 2,
    };
    BEL_THIRD_PERIOD_LABELS[if prefix == BEL_THIRD_GROUP_A_PREFIX { k } else { 3 + k }]
}

/// The period of a league round: rounds below 10 the first, below 20 the second, the rest the third.
fn bel_third_round_period(round: i16) -> u8 {
    if round < 10 {
        1
    } else if round < 20 {
        2
    } else {
        3
    }
}

/// Adds a Belgian Third Division result: always to the league table (League::add_result `exe::league_add_result`,
/// called directly); a group game (`fixture::STAGE_INDEX` 0xff for group A, 0 for group B) also counts in the table of
/// its group's period: the period of its round (`bel_third_round_period`), the stage found or created by
/// BelThird::find_or_create_period_stage (0x422cf0, through its detour) with the fixture's round prefix, the fixture's
/// stage index set to it meanwhile, and the result added there through the stage's virtual `comp_vtable::ADD_RESULT`
/// with the statistics (`league::COMP_STATS`) set aside, so the period tables write no statistics. Returns 1.
/// Original: `BelThird::add_result` (0x422c50, bel_third.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn bel_third_add_result(this: usize, fixture: usize, arg: u32) -> u32 {
    let l = League(this);
    let f = Fixture(fixture);
    exe::league_add_result(this, fixture, arg);
    let stage = f.stage_index();
    if stage != 0xff && stage != 0 {
        return 1;
    }
    covered!("cx09_8 belgian third division period result");
    let period = bel_third_round_period(f.round_number());
    let find: unsafe extern "thiscall" fn(usize, u32, u32, u32) -> i8 = core::mem::transmute(0x422cf0usize);
    f.set_stage_index(find(this, period as u32, f.round_prefix() as u16 as u32, 1) as u8);
    let stats = l.comp_stats();
    l.set_comp_stats(0);
    let s = read_usize(l.stages() + 4 * (f.stage_index() as i8 as isize as usize));
    let add: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(s) + comp_vtable::ADD_RESULT));
    add(s, fixture, arg);
    l.set_comp_stats(stats);
    f.set_stage_index(stage);
    1
}

/// The index of the period stage labelled `label` among stages 1 to `league::CURRENT_STAGE`, if any.
unsafe fn bel_third_period_stage(l: League, label: u16) -> Option<i8> {
    let last = l.current_stage() as i32;
    let mut i: i8 = 1;
    while (i as i32) <= last {
        if League(read_usize(l.stages() + 4 * i as usize)).stage_id() as u16 == label {
            return Some(i);
        }
        i = i.wrapping_add(1);
    }
    None
}

/// Returns the stage index of a group's period table (`bel_third_period_label` of the round prefix and the period),
/// searched among stages 1 to the current one; when missing, -2 unless `create`. To create it: a list of
/// `league::CLUB_COUNT` club pointers (malloc; none: the bel_third.cpp box of line 0x378 and -2) with the clubs of the
/// group's table in row order (group A: this league's table; group B: stage 0's), `league::CURRENT_STAGE` + 1, and a
/// League_Stage (`exe::crt_operator_new`(0xee); League_Stage::League_Stage 0x68b100: this league, this league's club
/// count, the list, 1 leg, the competition, no round table and 0 rounds, the win, draw and category bytes, the
/// tie-breaks, the places (0, 1, 0, 0), the year, the new current stage, the label, importance 0, 1, 0, rule bits 2, 10,
/// prize fund 0 and prize mode 2) into that slot (0 without memory); the list freed; stages 1 to the current one sorted
/// by group and period (qsort with BelThird's stage order 0x424380) and each one's `league::STAGE_INDEX` set to its
/// slot; then the search again (without creating).
/// Original: `BelThird::find_or_create_period_stage` (0x422cf0, bel_third.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn bel_third_find_or_create_period_stage(this: usize, period: u32, prefix: u32, create: u32) -> i8 {
    let l = League(this);
    let label = bel_third_period_label(prefix as u16, period as u8);
    if let Some(i) = bel_third_period_stage(l, label) {
        return i;
    }
    if create == 0 {
        return BEL_THIRD_NO_STAGE;
    }
    let clubs = exe::crt_malloc((l.club_count() as i32 as u32).wrapping_mul(4));
    if clubs == 0 {
        show_error_at(BEL_THIRD_SOURCE, 0x378);
        return BEL_THIRD_NO_STAGE;
    }
    covered!("cx09_8 belgian third division period stage");
    let group = if prefix as u16 == BEL_THIRD_GROUP_A_PREFIX { l } else { League(read_usize(l.stages())) };
    let mut k: i16 = 0;
    while k < group.club_count() {
        write_usize(clubs + 4 * k as usize, LeagueTableRow(group.table() + k as usize * league_table_row::SIZE).club() as usize);
        k += 1;
    }
    l.set_current_stage((l.current_stage() as i32 + 1) as usize);
    let places: [u8; 4] = [0, 1, 0, 0];
    let p = exe::crt_operator_new(0xee);
    let s = if p != 0 {
        let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        ctor(p, this, l.club_count() as u16 as u32, clubs, 1, l.competition().addr(), 0, 0, l.win_points() as u32, l.draw_points() as u32,
            l.match_category() as u32, this + league::TIEBREAKS, places.as_ptr() as usize, l.year() as u16 as u32, l.current_stage() as u8 as u32,
            label as u32, 0, 1, 0, 2, 0xa, 0, 2)
    } else {
        0
    };
    write_usize(l.stages() + 4 * l.current_stage(), s);
    exe::crt_free(clubs);
    exe::crt_qsort(l.stages() + 4, l.current_stage() as u32, 4, 0x424380usize);
    let mut i: i16 = 1;
    while (i as i32) <= l.current_stage() as i32 {
        League(read_usize(l.stages() + 4 * i as usize)).set_stage_index(i as u8);
        i += 1;
    }
    bel_third_find_or_create_period_stage(this, period, prefix, 0)
}

/// The group and period of a period stage by its label (`BEL_THIRD_PERIOD_LABELS`); any other stage gets the
/// `fallback` dword as its group and its low byte as its period (the original reads its own second argument there).
fn bel_third_stage_order_key(label: u16, fallback: u32) -> (u16, i8) {
    match BEL_THIRD_PERIOD_LABELS.iter().position(|&x| x == label) {
        Some(k) => (if k < 3 { 0x41f } else { 0x420 }, (k % 3 + 1) as i8),
        None => (fallback as u16, fallback as u8 as i8),
    }
}

/// The qsort order of the Belgian Third Division's period stages: by group (0x41f before 0x420), then by period; both
/// differences signed. A stage that is not a period stage takes the second argument's pointer value as its key
/// (a defect of the original, harmless: only period stages are sorted).
/// Original: `bel_third_compare_period_stages` (0x424380, bel_third.cpp, cdecl).
pub(crate) unsafe extern "C" fn bel_third_compare_period_stages(a: *const usize, b: *const usize) -> i32 {
    let (ga, pa) = bel_third_stage_order_key(League(*a).stage_id() as u16, b as u32);
    let (gb, pb) = bel_third_stage_order_key(League(*b).stage_id() as u16, b as u32);
    if ga != gb {
        return ga as i16 as i32 - gb as i16 as i32;
    }
    pa as i32 - pb as i32
}

/// Comparer for `bel_third_add_result`: only the original (it adds the result to the tables); afterwards it must return
/// 1 with the fixture's stage index and the statistics as they were.
pub(crate) unsafe extern "thiscall" fn compare_bel_third_add_result(this: usize, fixture: usize, arg: u32) -> u32 {
    let (l, f) = (League(this), Fixture(fixture));
    let (stage, stats) = (f.stage_index(), l.comp_stats());
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%)(this, fixture, arg);
    SCOREBOARD.record(r == 1 && f.stage_index() == stage && l.comp_stats() == stats, || format!("bel_third_add_result({:#x}, {:#x}): returned {}, stage {:#x} -> {:#x}, stats {:#x} -> {:#x}", this, fixture, r, stage, f.stage_index(), stats, l.comp_stats()));
    r
}

/// Comparer for `bel_third_find_or_create_period_stage`: only the original (it may create the stage); the answer must
/// be what the copy's search finds afterwards (with `create`, a stage must exist).
pub(crate) unsafe extern "thiscall" fn compare_bel_third_find_or_create_period_stage(this: usize, period: u32, prefix: u32, create: u32) -> i8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> i8>(%%I1%%)(this, period, prefix, create);
    let found = bel_third_period_stage(League(this), bel_third_period_label(prefix as u16, period as u8)).unwrap_or(BEL_THIRD_NO_STAGE);
    SCOREBOARD.record(r == found && (create == 0 || r != BEL_THIRD_NO_STAGE), || format!("bel_third_find_or_create_period_stage({:#x}, {}, {:#x}, {}): original {} copy {}", this, period as u8, prefix as u16, create, r, found));
    r
}

compare_order!(compare_bel_third_compare_period_stages, bel_third_compare_period_stages, %%I2%%);
