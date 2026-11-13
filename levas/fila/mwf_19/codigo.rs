// ---------- mwf_19: the Korean League Cup clubs, group A and next stage (kor_league_cup.cpp) ----------

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
            let c = read_usize(G_CLUB_TABLE) + k as u16 as usize * club::SIZE;
            let d = read_usize(c + club::DIVISION);
            if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_KOREAN_LEAGUE) {
                n += 1;
                write_usize(read_usize(this + 0xee) + (n as usize - 1) * 4, c);
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
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
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
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this);
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
        let knockout: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x66cd50usize);
        knockout(this.addr());
    }
}

/// Comparer for `kor_league_cup_advance_stage`: only the original (it may build the knockout stage); the stage must
/// stay at the last one or go up by one.
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_state_league_advance_stage("kor_league_cup_advance_stage", League(this), || o(this))
}
