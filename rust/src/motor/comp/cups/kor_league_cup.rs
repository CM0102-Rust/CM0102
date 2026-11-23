use super::*;

/// Rule: the Korean League Cup at career creation: the year and the database competition, 5 substitutes on the bench
/// and 3 substitutions, discipline rule 42, importance 12; with nothing saved, flags 8, two stages (group B and the
/// knockout), the ten clubs, the statistics, the predicted positions, the rules, group A's table (the cup is itself the
/// league of group A), its fixtures and order, and group B's stage.
///
/// The ItaCCup/IreLgeCup body (`construct_irish_league_cup`), checked instruction by instruction: over League::League
/// (`exe::new_league`, thiscall, ret 0) the record, the vtable 0x96ce84, the year, the bench, the discipline rule and
/// importance 0xc; the saved state (`exe::league_load_from_temp_file`(this, 1); nonzero: returns this). Otherwise
/// `league::LEAGUE_FLAGS` = 8 (0x28 in the Irish and Italian cups), the main stage, current stage −1, two stages and
/// their array (`exe::crt_malloc` of 8, unchecked); KorLeagueCup::select_teams (0x66cf20, unchecked); the statistics
/// (`ita_cups_create_comp_stats`; the box at line 0x53); KorLeagueCup::set_predicted_positions (0x66d3a0, unchecked);
/// KorLeagueCup::set_up_rules (0x66c870; 0: line 0x5d); KorLeagueCup::create_group_a_table (0x66c770; 0: line 0x64);
/// the fixtures (`exe::league_create_fixtures`; 0: line 0x6a); the order (`exe::league_sort_table`(this, 0); 0: line
/// 0x71); KorLeagueCup::create_group_b_stage(0) (0x66cb60: push 0; mov ecx, this -> thiscall, ret 4). Every routine
/// called is thiscall, ret 0 unless said. Each box returns this. Returns this.
/// Original: `KorLeagueCup::KorLeagueCup` (0x66bf00, kor_league_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_kor_league_cup(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), 0x96ce84);
    this.set_year(year as u16 as i16);
    this.set_substitutions_allowed(3);
    this.set_substitutes_named(5);
    this.set_discipline_rule(discipline_rule::KOREA);
    this.set_importance(0xc);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!("mwf_22 korean league cup new");
    this.set_league_flags(8); // mwf22: the Korean League Cup flags
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    this.set_stage_count(2);
    this.set_stages(exe::crt_malloc(8));
    let call = |f: usize| {
        let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(f);
        f(this.addr())
    };
    call(0x66cf20);
    if !ita_cups_create_comp_stats(this) {
        show_error_at(KOR_LEAGUE_CUP_FILE, 0x53);
        return this.addr();
    }
    call(0x66d3a0);
    if call(0x66c870) == 0 {
        show_error_at(KOR_LEAGUE_CUP_FILE, 0x5d);
        return this.addr();
    }
    if call(0x66c770) == 0 {
        show_error_at(KOR_LEAGUE_CUP_FILE, 0x64);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(KOR_LEAGUE_CUP_FILE, 0x6a);
        return this.addr();
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(KOR_LEAGUE_CUP_FILE, 0x71);
        return this.addr();
    }
    exe::korleaguecup_create_group_b_stage(this, 0);
    this.addr()
}

/// Comparer for `construct_kor_league_cup`: only the original runs (it allocates, draws and reads the save file); the
/// plan is checked (`ita_cups_check_constructor_plan`: this, the vtable, the record, the year and, on a new cup, the
/// fixed fields and the statistics).
pub(crate) unsafe extern "thiscall" fn compare_construct_kor_league_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_KOR_LEAGUE_CUP)(this, year, record);
    let error = ita_cups_check_constructor_plan(this, r, year, record, 0x96ce84, &KOREAN_LEAGUE_CUP_FIELDS, false);
    SCOREBOARD.record(error.is_none(), || format!("construct_kor_league_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The ten Korean League Cup clubs: every K League club, sorted by last season's position.
///
/// `+0xf2` = 10 and the list at `+0xee` (`exe::crt_calloc`(4, 10): push 10; push 4 -> cdecl; out of memory, the box of
/// kor_league_cup.cpp line 0x2ab and 0). In club table order (the index compared as a word with `G_CLUB_COUNT`), every
/// club whose division has the id of [`KEY_CLUB_COMP_KOREAN_LEAGUE`] enters the list (not checked against the ten
/// places: an eleventh K League club writes past the list, a defect of the original [inference]). The list is sorted
/// (`exe::crt_qsort`(list, n, 4, 0x4b0780), the comparer of last season's position); with fewer than `+0xf2` clubs, the
/// box of line 0x2bf (and it goes on). Returns 1.
/// Original: `KorLeagueCup::select_teams` (0x66cf20, kor_league_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_select_teams(this: usize) -> u32 {
    write_u32(this + 0xf2, 10);
    let list = exe::crt_calloc(4, 10);
    write_usize(this + 0xee, list);
    if list == 0 {
        show_error_at(0x9eb15c, 0x2ab);
        return 0;
    }
    let mut n = 0u32;
    let mut k = 0i16;
    if read_i32(G_CLUB_COUNT) > 0 {
        loop {
            let c = Club(read_usize(G_CLUB_TABLE) + k as u16 as usize * club::SIZE);
            let d = c.division().addr();
            if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_KOREAN_LEAGUE) {
                n += 1;
                write_usize(read_usize(this + 0xee) + (n as usize - 1) * 4, c.addr());
            }
            k = k.wrapping_add(1);
            if k as i32 >= read_i32(G_CLUB_COUNT) {
                break;
            }
        }
    }
    covered!("mwf_19 korean league cup clubs");
    exe::crt_qsort(read_usize(this + 0xee), n, 4, 0x4b0780usize); // mwf19: by last season's position
    if (n as i32) < read_i32(this + 0xf2) {
        show_error_at(0x9eb15c, 0x2bf);
    }
    1
}

/// Comparer for `kor_league_cup_select_teams`: only the original runs (it allocates the list); the plan, made before,
/// is the K League clubs in club table order sorted by the same comparer, and the original's list must hold them.
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_select_teams(this: usize) -> u32 {
    let mut plan: Vec<usize> = (0..read_i32(G_CLUB_COUNT).max(0) as usize)
        .map(|k| read_usize(G_CLUB_TABLE) + k * club::SIZE)
        .filter(|&c| { let d = read_usize(c + club::DIVISION); d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_KOREAN_LEAGUE) })
        .collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_KOR_LEAGUE_CUP_SELECT_TEAMS)(this);
    if r == 0 || plan.len() > 10 {
        return r; // simplification: out of memory, or more clubs than places (the original writes past the list)
    }
    exe::crt_qsort(plan.as_mut_ptr() as usize, plan.len() as u32, 4, 0x4b0780usize);
    let list = read_usize(this + 0xee);
    let got: Vec<usize> = (0..plan.len()).map(|k| read_usize(list + k * 4)).collect();
    SCOREBOARD.record(got == plan && read_u32(this + 0xf2) == 10, || format!("kor_league_cup_select_teams({:#x}) plan {:x?} list {:x?}", this, plan, got));
    r
}

/// Korean League Cup group A: the cup itself is group A, with the five even-numbered seeds (list places 0, 2, 4, 6 and
/// 8), each taken out of the list.
///
/// `.club_count()` = 5; the table (`exe::crt_malloc` of 0x127 = 5 rows of 0x3b; out of memory, the box of line 0x17f
/// and 0) into `.table()`. For i = 0..4 (a signed byte compared with the word `.club_count()`): row i gets the club at
/// list place 2i (League::init_table_row `exe::league_init_table_row`, thiscall(this, i, club, 0, 0), ret 0x10; the
/// original pushes the dword of the local byte, with stack garbage above it, and the callee reads the byte) and that
/// place is zeroed. Then the group draw news (League::send_group_draw_news `exe::league_send_group_draw_news`). Returns
/// 1.
/// Original: `KorLeagueCup::create_group_a_table` (0x66c770, kor_league_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_create_group_a_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(5);
    let t = exe::crt_malloc(0x127);
    this.set_table(t);
    if t == 0 {
        show_error_at(0x9eb15c, 0x17f);
        return 0;
    }
    covered!("mwf_19 korean league cup group a");
    let mut i: i8 = 0;
    while (i as i16) < this.club_count() {
        let slot = read_usize(this.addr() + 0xee) + i as isize as usize * 8; // the even-numbered seeds
        exe::league_init_table_row(this, i as u8 as u32, read_usize(slot), 0, 0); // mwf19: row i
        write_usize(slot, 0);
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

/// Comparer for `kor_league_cup_create_group_a_table`: only the original runs (init_table_row allocates and the news
/// goes to the human managers); the plan is the clubs at list places 0, 2, 4, 6 and 8, which must be in the table rows
/// in order, with those places zeroed.
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_create_group_a_table(this: usize) -> u32 {
    let list = read_usize(this + 0xee);
    let plan: Vec<usize> = (0..5).map(|k| read_usize(list + k * 8)).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_KOR_LEAGUE_CUP_CREATE_GROUP_A_TABLE)(this);
    let l = League(this);
    let t = l.table();
    let ok = r == 1 && l.club_count() == 5 && t != 0 && plan.iter().enumerate().all(|(k, &c)| read_usize(t + k * 0x3b) == c && read_usize(list + k * 8) == 0);
    SCOREBOARD.record(ok, || format!("kor_league_cup_create_group_a_table({:#x}) returned {}; plan {:x?}", this, r, plan));
    r
}

/// The Korean League Cup next stage: the group stage moves on and, when it leaves stage 0 for stage 1, the knockout
/// stage is built; past the last stage it is an error (the box).
///
/// At the last stage (`.current_stage()` ≥ `.stage_count()` − 1, signed) the box of line 0x21e and nothing else;
/// otherwise `league::CURRENT_STAGE` goes up by one and, when it was 0, KorLeagueCup::create_knockout_stage (0x66cd50,
/// thiscall, ret 0).
/// Original: `KorLeagueCup::advance_stage` (0x66cad0, kor_league_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_advance_stage(this: usize) {
    let this = League(this);
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x9eb15c, 0x21e);
    }
    this.set_current_stage((stage + 1) as usize);
    if stage == 0 {
        covered!("mwf_19 korean league cup knockout");
        exe::korleaguecup_create_knockout_stage(this);
    }
}

/// Comparer for `kor_league_cup_advance_stage`: only the original (it may build the knockout stage); the stage must
/// stay at the last one or go up by one.
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_KOR_LEAGUE_CUP_ADVANCE_STAGE);
    compare_state_league_advance_stage("kor_league_cup_advance_stage", League(this), || o(this))
}

/// The season turnover of the Korean League Cup: the season archived, last season's stages and team list dropped, the
/// year plus one, the ten K League clubs selected again (KorLeagueCup::select_teams, sorted by last season's
/// position), the predicted positions, the rules, group A (KorLeagueCup::create_group_a_table: the even-numbered seeds,
/// the cup itself), its fixtures and order, and the group B stage (KorLeagueCup::create_group_b_stage: the odd-numbered
/// seeds).
///
/// `ita_group_cup_start_new_season` with 0x66cf20, 0x66c770, 0x66cb60 (push 0: the one group stage, ret 4) and the
/// kor_league_cup.cpp boxes at lines 0x1f1, 0x1f8, 0x1fe and 0x205.
/// Original: `KorLeagueCup::start_new_season` (0x66c900, kor_league_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_start_new_season(this: usize) -> u32 {
    ita_group_cup_start_new_season(League(this), 0x66cf20, 0x66c770, 0x66cb60, 1, 0x9eb15c, [0x1f1, 0x1f8, 0x1fe, 0x205], 1, "mwf_9 korean league cup rollover") // mwf9: the Korean League Cup turnover
}

/// Comparer for `kor_league_cup_start_new_season` (original only; the year plus one).
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_KOR_LEAGUE_CUP_START_NEW_SEASON);
    compare_state_league_new_season("kor_league_cup_start_new_season", League(this), || o(this))
}

/// The Korean League Cup group rules: a double round robin, 3 points a win and 1 a draw, tiebreakers 1 and 2, the top
/// two of each group through, no league above or below, club sides, cup category (4), match rule bits 0x83 and a prize
/// fund of 100,000; then the group schedule. The substitutes and changes are not set here.
///
/// Writes the bytes and words of `set_korean_league_cup_rules` (`+0xc7` and `+0xbf`..`+0xc1` 0, `+0x3c` word 2,
/// `+0xc6` 2, `+0xbe` 2, `+0x1c` and `+0x20` −1, `+0xc2` 3, `+0xc3` 1, `+0xc4` 4, `+0xc5` 1, `+0x42` 1, `+0xab` word
/// 0x83, `+0x28` 100,000), then the round table of stage −1 (virtual +0x3c: push 0; push &`+0x3a`; push &`+0xa9`; push
/// −1 -> thiscall, ret 0x10) into `.round_dates()` (`create_main_schedule`). Returns 1.
/// Original: `KorLeagueCup::set_up_rules` (0x66c870, kor_league_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_set_up_rules(this: usize) -> u32 {
    set_korean_league_cup_rules(this);
    covered!("mwf_11 korean league cup rules");
    create_main_schedule(League(this));
    1
}

/// Comparer for `kor_league_cup_set_up_rules`: side by side over the league object without the schedule outputs.
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_KOR_LEAGUE_CUP_SET_UP_RULES)(this), || {
        set_korean_league_cup_rules(this);
        1
    });
    SCOREBOARD.record(same && r == n, || format!("kor_league_cup_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Writes the Korean League Cup rule fields (what `kor_league_cup_set_up_rules` writes before the schedule).
pub(crate) unsafe fn set_korean_league_cup_rules(o: usize) {
    write_u8_at(o, 0xc7, 0);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 0);
    write_u16_at(o, 0x3c, 2); // a double round robin
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0xbe, 2); // the top two of each group go through
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u8_at(o, 0xc2, 3); // mwf11: the League Cup points per win
    write_u8_at(o, 0xc3, 1); // points per draw
    write_u8_at(o, 0xc4, 4); // match category: cup
    write_u8_at(o, 0xc5, 1);
    write_u8_at(o, 0x42, 1); // club sides
    write_u16_at(o, 0xab, 0x83); // match rule bits
    write_u32_at(o, 0x28, 0x186a0); // prize fund 100,000
}

/// The statistics (`exe::crt_operator_new` of 0x5ce, cdecl; `exe::new_comp_stats`(new, this), thiscall, ret 4) into
/// `league::COMP_STATS`; the original's SEH frame covers only that `new`. Returns whether they were created.
pub(crate) unsafe fn ita_cups_create_comp_stats(this: League) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    this.set_comp_stats(s);
    s != 0
}

/// The source file of the Korean League Cup's error boxes.
pub(crate) const KOR_LEAGUE_CUP_FILE: usize = 0x9eb15c;

/// The fixed fields the Korean League Cup constructor writes on a new cup (offset, width, value).
pub(crate) const KOREAN_LEAGUE_CUP_FIELDS: [(usize, u8, u32); 7] = [(league::STAGE_COUNT, 4, 2), (league::STAGE_INDEX, 1, 0xff), (league::SUBSTITUTES_NAMED, 1, 5), (league::SUBSTITUTIONS_ALLOWED, 1, 3),
    (league::DISCIPLINE_RULE, 1, discipline_rule::KOREA as u32), (league::IMPORTANCE, 1, 0xc), (league::LEAGUE_FLAGS, 2, 8)];
