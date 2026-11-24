// ---------- cx16_11: the European World Cup qualifying stages: moving on, the runners-up stage and the section's own group (wc_europe_league.cpp) ----------

/// WcEuropeLeague routines called directly (thiscall): the play-off stage (ret 0) and the group size (ret 4).
pub(crate) const FN_WC_EUROPE_LEAGUE_CREATE_PLAY_OFF_STAGE: usize = 0x919910;
pub(crate) const FN_WC_EUROPE_LEAGUE_GET_GROUP_SIZE: usize = 0x918dd0;

/// rule: the European qualifying moves on one stage at a time; reaching the runners-up stage or the play-off stage
/// builds the play-off; past the last stage it is an error
///
/// At or past the last stage (`league::STAGE_COUNT` − 1, signed), the error box of line 0x37f. Otherwise
/// `league::CURRENT_STAGE` + 1 and, when the new stage is the runners-up stage (+0x100) or the play-off stage (+0x101),
/// signed bytes, WcEuropeLeague::create_play_off_stage (0x919910, thiscall, ret 0, called directly).
/// Original: `WcEuropeLeague::advance_stage` (0x919620, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_advance_stage(this: usize) {
    let league = League(this);
    let current = league.current_stage() as i32;
    if current >= (league.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x37f);
    }
    let next = current + 1;
    league.set_current_stage(next as usize);
    covered!("cx16_11 european next stage");
    if next == read_u8(this + 0x100) as i8 as i32 || next == read_u8(this + 0x101) as i8 as i32 { // cx16_11: the play-off when the runners-up stage comes
        call_section_routine(FN_WC_EUROPE_LEAGUE_CREATE_PLAY_OFF_STAGE, this);
    }
}

/// Comparer for `wc_europe_league_advance_stage`: it builds stages, so only the original runs; the stage stays or moves
/// on by one.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_advance_stage(this: usize) {
    let before = League(this).current_stage() as i32;
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let after = League(this).current_stage() as i32;
    SCOREBOARD.record(after == before || after == before + 1, || format!("wc_europe_league_advance_stage({:#x}): stage {} -> {}", this, before, after));
}

/// rule: the European runners-up are ranked in a league stage of their own, one runner-up per group (the last group
/// index + 2), with the section's points, category and tie-breaks 1, 2, 5, its best placed going through
///
/// `league::PROMOTED_COUNT` 0; the stage (`exe::crt_operator_new` of 0xee and League_Stage::League_Stage
/// `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58: the section, the last group (+0xff, signed) + 2, no teams, 0, the
/// record, no round table, 0 rounds, `league::WIN_POINTS`, `league::DRAW_POINTS`, `league::MATCH_CATEGORY`, the
/// tie-breaks {1, 2, 5, 0}, the places at `league::PROMOTED_COUNT` (the section's own four bytes), the year, the
/// runners-up stage index (+0x100), the label 0x3fc, importance 0, 4, 0, flags 2, −1, 0, 2; the original pushes the count,
/// the bytes and the year with garbage in the high bytes, which the constructor does not read: the copy passes clean
/// values) into `league::STAGES`[+0x100] (null not checked); `league::PROMOTED_COUNT` 1; `league::CURRENT_STAGE` the
/// runners-up stage index; then the table filled (WcEuropeLeague::fill_runners_up_table
/// `exe::wceuropeleague_fill_runners_up_table`, thiscall, ret 0). The SEH frame covers only the stage's new.
/// Original: `WcEuropeLeague::create_runners_up_stage` (0x91a140, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_create_runners_up_stage(this: usize) {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let league = League(this);
    let tiebreaks: [u8; 4] = [1, 2, 5, 0];
    let n = (read_u8(this + 0xff) as i8 as i16).wrapping_add(2); // cx16_11: one runner-up per group
    let index = read_u8(this + 0x100) as i8;
    league.set_promoted_count(0);
    covered!("cx16_11 european runners-up stage");
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(p, this, n as u16 as u32, 0, 0, league.competition().addr(), 0, 0, league.win_points() as u32, league.draw_points() as u32, league.match_category() as u32,
            tiebreaks.as_ptr() as usize, this + league::PROMOTED_COUNT, league.year() as u16 as u32, index as u8 as u32, 0x3fc, 0, 4, 0, 2, -1, 0, 2)
    } else {
        0
    };
    write_usize(league.stages().wrapping_add((index as isize as usize).wrapping_mul(4)), stage);
    league.set_promoted_count(1);
    league.set_current_stage(index as isize as usize);
    exe::wceuropeleague_fill_runners_up_table(this);
}

/// Comparer for `wc_europe_league_create_runners_up_stage`: it builds a stage, so only the original runs; then the
/// plan: the current stage is the runners-up stage, which exists with one row per group.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_create_runners_up_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let league = League(this);
    let index = read_u8(this + 0x100) as i8;
    let stage = League(read_usize(league.stages().wrapping_add((index as isize as usize).wrapping_mul(4))));
    let n = (read_u8(this + 0xff) as i8 as i16).wrapping_add(2);
    let ok = league.current_stage() as i32 == index as i32 && !stage.is_null() && stage.club_count() == n;
    SCOREBOARD.record(ok, || format!("wc_europe_league_create_runners_up_stage({:#x}): stage {} index {}, {} rows of {}", this, league.current_stage() as i32, index, if stage.is_null() { -1 } else { stage.club_count() as i32 }, n));
}

/// rule: the first European qualifying group is the section itself: its size by the exempt count (group −1), its
/// nations from the first drawn slots, and the group draw news
///
/// `league::CLUB_COUNT` = WcEuropeLeague::get_group_size(−1) (0x918dd0, thiscall, ret 4); `league::TABLE` = a table of
/// that many `league_table_row::SIZE`-byte rows (`exe::crt_malloc`; out of memory, the error box of line 0x2b2 and 0);
/// `league::GAMES_PER_CLUB` = (count − 1) × `league::LEGS` (words); row i with the drawn slot i (+0xf7, a pointer to the
/// slot table) (`exe::league_init_table_row`(this, i, team, 0, 0), thiscall, ret 0x10; the original pushes i with
/// garbage in the high bytes: the routine reads the byte); the group draw news (`exe::league_send_group_draw_news`); 1.
/// Original: `WcEuropeLeague::init_own_group_table` (0x9190d0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_init_own_group_table(this: usize) -> u32 {
    let league = League(this);
    let size: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_WC_EUROPE_LEAGUE_GET_GROUP_SIZE);
    league.set_club_count(size(this, u32::MAX));
    let t = exe::crt_malloc((league.club_count() as i32 as u32).wrapping_mul(league_table_row::SIZE as u32));
    league.set_table(t);
    if t == 0 {
        show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x2b2);
        return 0;
    }
    league.set_games_per_club(league.club_count().wrapping_sub(1).wrapping_mul(league.legs()));
    covered!("cx16_11 european own group");
    let slots = read_usize(this + 0xf7); // the drawn group slots
    let mut i = 0i8;
    while (i as i16) < league.club_count() {
        exe::league_init_table_row(league, i as u8 as u32, read_usize(slots + i as isize as usize * 4), 0, 0); // cx16_11: slot i in row i
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(league);
    1
}

/// Comparer for `wc_europe_league_init_own_group_table`: it allocates the table and sends news, so only the original
/// runs; then the plan: 1, the group size and each row with its drawn slot in order.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_init_own_group_table(this: usize) -> u32 {
    let slots = read_usize(this + 0xf7);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this);
    let league = League(this);
    if r != 1 || slots == 0 || league.table() == 0 {
        return r;
    }
    let n = league.club_count().clamp(0, 16) as usize;
    let rows: Vec<usize> = (0..n).map(|k| read_usize(league.table() + k * league_table_row::SIZE)).collect();
    let plan: Vec<usize> = (0..n).map(|k| read_usize(slots + k * 4)).collect();
    SCOREBOARD.record(rows == plan, || format!("wc_europe_league_init_own_group_table({:#x}): rows {:x?} plan {:x?}", this, rows, plan));
    r
}
