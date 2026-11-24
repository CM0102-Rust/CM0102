// ---------- cx03_10: the Welsh Premier Cup progress and the knockout stages of the two Welsh league cups (wel_prm_cup.cpp, wel_lge_cup.cpp) ----------

/// rule: in the Welsh Premier Cup groups (stages below 1) the group places become cup progress codes: places 1 to 3
/// give 1 and anything else 9 for the current and best codes; for the worst, places 1 and 2 give 5, place 3 gives 9 and
/// anything else 13; from the knockout on the codes stay as they come.
///
/// `set_club_progress(club, stage, current, best, worst)` (virtual `comp_vtable::SET_CLUB_PROGRESS`, ret 0x14), the
/// codes are bytes (`store_club_progress`).
/// Original: `WelPrmCup::set_club_progress` (0x9290b0, wel_prm_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let (mut current, mut best, mut worst) = (current as u8, best as u8, worst as u8);
    if (stage as u8 as i8) < 1 {
        covered!("cx03_10 welsh premier cup group progress");
        let place = |code: u8| if (1..=3).contains(&code) { 1 } else { 9 };
        current = place(current);
        best = place(best);
        worst = match worst {
            1 | 2 => 5,
            3 => 9,
            _ => 0xd,
        };
    }
    store_club_progress(this, club, current, best, worst);
}

/// rule: the Welsh League Cup knockout (stage 5) takes the six group winners and the two best runners-up (the
/// runners-up ranked in a table of their own, by points and the cup's tie-breaks), then draws the quarter-finals.
///
/// The club vector (`exe::crt_malloc`(0x20); out of memory, the box of line 0x27e): slot 0 the first row of the cup's
/// own table (`league::TABLE`), slots 1 to 5 the first row of the group stages 0 to 4 (`exe::league_get_table_row`).
/// The runners-up: a temporary League_Stage (`exe::crt_operator_new`(0xee); League_Stage::League_Stage 0x68b100,
/// thiscall, ret 0x58: the cup, 6, 0, 0, the competition `+4`, 0, 0, the win and draw points `league::WIN_POINTS` and
/// `league::DRAW_POINTS`, 0, &`league::TIEBREAKS`, &`league::PROMOTED_COUNT`, the year (word), −2, 0, 0, 1, 0, 2, −1,
/// 0, 2) whose table row i receives the second row of group i (the cup's own table, then stages 0 to 4: the 0x3b
/// bytes copied) with its +9 and +5 dwords zeroed, sorted (`exe::league_sort_table`(ranking, 0)); slots 6 and 7 its
/// first two clubs. Then stage 5 as `create_knockout_stage` does it (the rounds by virtual `comp_vtable::CREATE_SCHEDULE`,
/// the Cup_Stage of 8 clubs, no seeds, in `cup::STAGES`[5], the vector and the round table freed), but the temporary
/// ranking is destroyed (virtual 0 with 1) before the stage is checked: without a stage, the box of line 0x2b2;
/// with one, the draw of its first round (`exe::cup_draw_round`(stage, 0)).
/// Original: `WelLgeCup::create_knockout_stage` (0x926d00, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_create_knockout_stage(this: usize) {
    let this = League(this);
    let teams = exe::crt_malloc(0x20);
    if teams == 0 {
        return show_error_at(0xad56f4, 0x27e);
    }
    covered!("cx03_10 welsh league cup knockout");
    write_usize(teams, read_usize(this.table()));
    for g in 0..5usize {
        let stage = read_usize(this.stages() + g * 4);
        write_usize(teams + 4 * (g + 1), read_usize(exe::league_get_table_row(stage, 0))); // the winner of group g + 1
    }
    let p = exe::crt_operator_new(0xee);
    let ranking = League(if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, i32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
            core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        construct(p, this.addr(), 6, 0, 0, this.competition().addr(), 0, 0, this.win_points() as u32, this.draw_points() as u32, 0,
            this.addr() + league::TIEBREAKS, this.addr() + league::PROMOTED_COUNT, this.year() as u16 as u32, -2, 0, 0, 1, 0, 2, -1, 0, 2)
    } else {
        0
    });
    for g in 0..6usize {
        let group = if g == 0 { this } else { League(read_usize(this.stages() + (g - 1) * 4)) };
        let row = ranking.table() + g * league_table_row::SIZE;
        core::ptr::copy_nonoverlapping((group.table() + league_table_row::SIZE) as *const u8, row as *mut u8, league_table_row::SIZE); // the runner-up of group g
        write_usize(row + 9, 0);
        write_usize(row + 5, 0);
    }
    exe::league_sort_table(ranking, 0); // the runners-up ranked
    write_usize(teams + 0x18, read_usize(ranking.table()));
    write_usize(teams + 0x1c, read_usize(ranking.table() + league_table_row::SIZE));
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = rounds_of_cup(this.addr(), 5, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let knockout = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 8, teams, rounds as u16 as u32, this.competition().addr(), table, this.year() as u16 as u32, 5, 1, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages() + 5 * 4, knockout);
    exe::crt_free(teams);
    exe::crt_free(table);
    if !ranking.is_null() {
        let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(ranking.addr())));
        destroy(ranking.addr(), 1);
    }
    let knockout = read_usize(this.stages() + 5 * 4);
    if knockout == 0 {
        return show_error_at(0xad56f4, 0x2b2);
    }
    exe::cup_draw_round(knockout, 0);
}

/// rule: the Welsh Premier Cup knockout (stage 1) has eight clubs: the top two of the two groups (the cup's own table
/// and stage 0) and the four clubs that enter there (entrants 8 to 11), seeded alternately 2 and 1 [inference: the
/// group clubs in pot 2, the entering clubs in pot 1], then draws the quarter-finals.
///
/// The club vector (`exe::crt_malloc`(0x20); out of memory, the box of line 0x27c): the first row of the cup's own table,
/// entrant 8 (`ita_cup_fields::ENTRANT_CLUBS`), the first row of stage 0, entrant 9, the second row of stage 0, entrant
/// 10, the second row of the own table, entrant 11 (`exe::league_get_table_row`); then stage 1 (`create_knockout_stage`:
/// 8 clubs, the seeds 2, 1, 2, 1, 2, 1, 2, 1, the box of line 0x297).
/// Original: `WelPrmCup::create_knockout_stage` (0x928650, wel_prm_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_create_knockout_stage(this: usize) {
    let this = League(this);
    let seeds: [u8; 8] = [2, 1, 2, 1, 2, 1, 2, 1];
    let teams = exe::crt_malloc(0x20);
    if teams == 0 {
        return show_error_at(0xad572c, 0x27c);
    }
    covered!("cx03_10 welsh premier cup knockout");
    let entrant = |k: usize| read_usize(ItaCupFields(this.addr()).entrant_clubs() + k * 4);
    let first_group = read_usize(this.stages());
    write_usize(teams, read_usize(this.table()));
    write_usize(teams + 4, entrant(8));
    write_usize(teams + 8, read_usize(exe::league_get_table_row(first_group, 0)));
    write_usize(teams + 0xc, entrant(9));
    write_usize(teams + 0x10, read_usize(exe::league_get_table_row(first_group, 1)));
    write_usize(teams + 0x14, entrant(10));
    write_usize(teams + 0x18, read_usize(this.table() + league_table_row::SIZE)); // the cup's own runner-up
    write_usize(teams + 0x1c, entrant(11));
    create_knockout_stage(this, 8, teams, 1, seeds.as_ptr(), 0xad572c, 0x297);
}

pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I0%%);
    compare_club_progress("wel_prm_cup_set_club_progress", this, club, stage, || o(this, club, stage, current, best, worst),
        || wel_prm_cup_set_club_progress(this, club, stage, current, best, worst));
}

/// Comparer for `wel_lge_cup_create_knockout_stage` (original only: it builds the stage and draws its first round;
/// `ita_cup_compare_knockout_stage` checks the stage in slot 5).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    ita_cup_compare_knockout_stage("wel_lge_cup_create_knockout_stage", League(this), 5, 0, || o(this));
}

/// Comparer for `wel_prm_cup_create_knockout_stage` (original only; the stage in slot 1).
pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    ita_cup_compare_knockout_stage("wel_prm_cup_create_knockout_stage", League(this), 1, 0, || o(this));
}
