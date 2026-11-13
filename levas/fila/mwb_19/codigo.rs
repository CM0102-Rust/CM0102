// ---------- mwb_19: the Welsh League Cup (wel_lge_cup.cpp): the honours, the staff records and the group stages; the moulds of the two Welsh cups ----------

/// rule: the Welsh League Cup's season honours are those of its knockout stage (the sixth stage slot): the call goes on
/// to that stage's own `record_season_honours` (virtual +0x30), whose eax it returns
/// Original: `WelLgeCup::record_season_honours` (0x926890, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_record_season_honours(this: usize) -> u32 {
    covered!("mwb_19 welsh league cup honours");
    let knockout = read_usize(League(this).stages() + 4 * 5); // mwb19: the League Cup knockout slot
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(knockout) + comp_vtable::RECORD_SEASON_HONOURS));
    f(knockout)
}

// simplification: the honours go into the history: they do not run twice and the routine writes nothing besides the
// knockout's own. Original only.

/// Comparer for `wel_lge_cup_record_season_honours` (original only).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_record_season_honours(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this)
}

/// Virtual +0x44 of the two Welsh group cups: the staff records by outcome and stage; returns 0.
///
/// In the groups (`stage`, signed byte, below `groups`): outcome 6, qualification (`european_cup_add_qualified_record`:
/// stage 0, round 0x78, weight 0x28); 0xfe, failure to qualify (`european_cup_add_failed_to_qualify_record`: 0x41c,
/// weight 0x28). In the knockout (stage `groups`, the stage slot `groups`): 0xff, knocked out with the match's round
/// prefix (+0x32), round name (+0x30) and the weight `scale_by_round(0x32, 0x64, round number, scale)`; 0, qualification
/// with the round prefix, the code of the next round of the knockout's round table and the same weight; 1 and 2, the
/// winner's and the runner-up's records (0x86a800, 0x86b0b0: the staff records, club, match, a6).
pub(crate) unsafe fn welsh_cup_staff_records(this: League, club: usize, outcome: u32, stage: u32, game: usize, a6: u32, groups: i8, scale: u32) -> u32 {
    let game = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = this.competition().addr();
    if f < groups {
        if d == 6 {
            european_cup_add_qualified_record(club, comp, 0, 0x78, 0x28); // mwb19: the group qualification weight
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x28);
        }
    } else if f == groups {
        match d {
            0xff => {
                let weight = scale_by_round(0x32, 0x64, game.round_number(), scale);
                // simplification: +0x32 and +0x30 go in registers with garbage in the high 16 bits; 0x86c000 only uses
                // the words
                european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
            }
            0 => {
                let r = game.round_number();
                let weight = scale_by_round(0x32, 0x64, r, scale);
                let knockout = Cup(read_usize(this.stages() + groups as usize * 4));
                let round = read_i16(knockout.round_table().wrapping_add((r as isize * (cup_round::SIZE as isize)) as usize) + cup_round::ROUND_CODE + 0x68) as u16 as u32;
                european_cup_add_qualified_record(club, comp, game.round_prefix() as u16 as u32, round, weight);
            }
            1 => african_nations_record_final_result(0x86a800, club, game.addr(), a6),
            2 => african_nations_record_final_result(0x86b0b0, club, game.addr(), a6),
            _ => {}
        }
    }
    0
}

/// rule: in the six Welsh League Cup groups, a club going through gets the qualification record (weight 40) and one
/// going out the failed-to-qualify record (weight 40); in the knockout (stage 5) a club going through gets the
/// qualification by round and one going out the knocked-out record (weight 50 to 100 over 5 rounds), and the winner
/// and the runner-up their final records
/// Original: `WelLgeCup::update_staff_records_for_status` (0x9271e0, wel_lge_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    covered!("mwb_19 welsh league cup staff records");
    welsh_cup_staff_records(League(this), club, outcome, stage, game, a6, 5, 5)
}

// simplification: the records go into the managers' history: they do not run twice and the routine writes nothing
// besides them. Original only; the plan checks the return of 0.

/// Comparer for `wel_lge_cup_update_staff_records_for_status` (original only, the return 0).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%)(this, club, outcome, stage, a4, game, a6);
    SCOREBOARD.record(r == 0, || format!("wel_lge_cup_update_staff_records_for_status({:#x}) returned {}", this, r));
    r
}

/// Builds group `g` of a Welsh group cup as a League_Stage, in the original routine's order.
///
/// The round table of stage `g` (virtual +0x3c: the round count and the stage label into locals, no league flag); the
/// club list (`exe::crt_malloc` of 4 × `n`; null: the box of the first line and 0) with pool entries g + 1 + k × `stride`
/// (k = 0..n), each zeroed in the pool; a League_Stage (operator new of 0xee and League_Stage::League_Stage 0x68b100:
/// this cup as the parent, `n` clubs, the list, 2, the record, the round table and its count, the win, draw and
/// category bytes, the tie-breaks (+0xc5), a local tie-break list (`first_tiebreak`, 0, 0, 0), the year, `g`, the stage
/// label, the importance (+0x51), 1, 0, the league flags (+0xd9), −1, 0, 2) into stage slot `g` (0 without memory); the
/// list and the round table freed; no stage: the box of the second line and 0; otherwise the group draw news of the
/// stage (League::send_group_draw_news), the current stage = `g` and 1.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn welsh_cup_group_stage(this: League, g: u32, n: u32, stride: usize, first_tiebreak: u8, file: usize, rows: (u32, u32), ramo: &'static str) -> u32 {
    let t = this.addr();
    let tiebreaks: [u8; 4] = [first_tiebreak, 0, 0, 0];
    let (mut rounds, mut label) = (0i16, 0i16);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(t) + comp_vtable::CREATE_SCHEDULE));
    let table = schedule(t, g, &mut rounds, &mut label, 0);
    let clubs = exe::crt_malloc(4 * n);
    if clubs == 0 {
        show_error_at(file, rows.0);
        return 0;
    }
    covered!(ramo);
    for k in 0..n as usize {
        let e = read_usize(t + WELSH_CUP_POOL) + (g as usize + 1 + k * stride) * 4; // mwb19: the group clubs in pool order
        write_usize(clubs + 4 * k, read_usize(e));
        write_usize(e, 0);
    }
    let p = exe::crt_operator_new(0xee);
    let mut s = 0usize;
    if p != 0 {
        type LeagueStageCtor = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize;
        let ctor: LeagueStageCtor = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        // simplification: the original pushes the round count, the stage label and the flags as stack or register
        // dwords with stale high bytes; League_Stage::League_Stage reads the words and bytes
        s = ctor(p, t, n, clubs, 2, read_usize(t + league::COMPETITION), table, rounds as u16 as u32, read_u8(t + league::WIN_POINTS) as u32,
            read_u8(t + league::DRAW_POINTS) as u32, read_u8(t + league::MATCH_CATEGORY) as u32, t + league::TIEBREAKS, tiebreaks.as_ptr() as usize,
            this.year() as u16 as u32, g, label as u16 as u32, read_u8(t + league::IMPORTANCE) as u32, 1, 0, read_u16(t + league::LEAGUE_FLAGS) as u32, 0xffff_ffff, 0, 2);
    }
    write_usize(this.stages() + 4 * g as usize, s);
    exe::crt_free(clubs);
    exe::crt_free(table);
    let s = read_usize(this.stages() + 4 * g as usize);
    if s == 0 {
        show_error_at(file, rows.1);
        return 0;
    }
    let news: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    news(s);
    this.set_current_stage(g as usize);
    1
}

/// Comparer for the Welsh group stages: the pool entries of the group planned beforehand; only the original runs (it
/// builds a stage with its fixtures); then the return 1, the stage in its slot, the current stage and the pool entries
/// zeroed.
pub(crate) unsafe fn compare_welsh_cup_group_stage(name: &str, this: League, g: u32, n: u32, stride: usize, original: impl FnOnce() -> u32) -> u32 {
    let pool = read_usize(this.addr() + WELSH_CUP_POOL);
    let r = original();
    let ok = r == 1 && read_usize(this.stages() + 4 * g as usize) != 0 && this.current_stage() as u32 == g
        && (0..n as usize).all(|k| read_usize(pool + (g as usize + 1 + k * stride) * 4) == 0);
    SCOREBOARD.record(ok, || format!("{}({:#x}, group {}) returned {}, stage {:#x}, current {}", name, this.addr(), g, r, read_usize(this.stages() + 4 * g as usize), this.current_stage() as i32));
    r
}

/// rule: Welsh League Cup group `g` (0 to 4; the sixth group is the cup's own table): three clubs, pool entries g + 1,
/// g + 7 and g + 13 (the pool's column g + 1), in a league stage of their own with tie-break list (1, 0, 0, 0); then the
/// group draw news; out of memory the box of line 0x249, without the stage that of 0x25f (and 0)
/// Original: `WelLgeCup::create_group_stage` (0x926b00, wel_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_create_group_stage(this: usize, g: u32) -> u32 {
    welsh_cup_group_stage(League(this), g, 3, 6, 1, 0xad56f4, (0x249, 0x25f), "mwb_19 welsh league cup group")
}

/// Comparer for `wel_lge_cup_create_group_stage` (original only, the plan).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I2%%);
    compare_welsh_cup_group_stage("wel_lge_cup_create_group_stage", League(this), g, 3, 6, || o(this, g))
}
