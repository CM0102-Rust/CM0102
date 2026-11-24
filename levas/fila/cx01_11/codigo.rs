// ---------- cx01_11: the Brazilian Champions Cup: the preliminary stage and the next stage (bra_champ_cup.cpp) ----------

/// `BraChampCup::create_group_stages` (0x426470, bra_champ_cup.cpp, thiscall, ret 0).
const FN_BRA_CHAMP_CUP_CREATE_GROUP_STAGES: usize = 0x426470;
/// `BraChampCup::create_semi_final_stage` (0x426720, bra_champ_cup.cpp, thiscall, ret 0).
const FN_BRA_CHAMP_CUP_CREATE_SEMI_FINAL_STAGE: usize = 0x426720;
/// `BraChampCup::create_preliminary_table` (0x426130, bra_champ_cup.cpp, thiscall, ret 0).
const FN_BRA_CHAMP_CUP_CREATE_PRELIMINARY_TABLE: usize = 0x426130;

/// Builds the table of the three-club preliminary (the cup itself is the league): three clubs (`CLUB_COUNT`), the table
/// (`exe::crt_malloc` of 0xb1 bytes; none: the error box of line 0x195 and 0), the first three entrants in rows 0 to 2
/// (`exe::league_init_table_row` with 0, 0), each taken out of the entrant book as it goes in, and the group draw news
/// (`exe::league_send_group_draw_news`). Returns 1.
/// Original: `BraChampCup::create_preliminary_table` (0x426130, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_create_preliminary_table(this: usize) -> u32 {
    let cup = League(this);
    cup.set_club_count(3);
    let table = exe::crt_malloc(0xb1);
    cup.set_table(table);
    if table == 0 {
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x195);
        return 0;
    }
    covered!("cx01_11 preliminary table");
    let mut row = 0i8;
    while (row as i16) < cup.club_count() {
        let entry = cup.entrant_book() + row as isize as usize * 6;
        // the original passes the row as a dword whose high bytes are whatever was on the stack: only the byte is read
        exe::league_init_table_row(this, row as u8 as u32, read_usize(entry), 0, 0);
        write_usize(entry, 0); // the club leaves the entrant book
        row = row.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

/// Starts the preliminary: the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`), then, each with its error
/// box when it answers 0, the preliminary table (`BraChampCup::create_preliminary_table`; line 0x491), the fixtures
/// (`exe::league_create_fixtures`; line 0x497) and the table order (`exe::league_sort_table` with 0; line 0x49e);
/// finally the competition is active.
/// Original: `BraChampCup::start_preliminary_stage` (0x427320, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_start_preliminary_stage(this: usize) {
    let cup = League(this);
    let predicted: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
    predicted(this);
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_BRA_CHAMP_CUP_CREATE_PRELIMINARY_TABLE);
    if table(this) == 0 {
        return show_error_at(BRA_CHAMP_CUP_SOURCE, 0x491);
    }
    if exe::league_create_fixtures(this) == 0 {
        return show_error_at(BRA_CHAMP_CUP_SOURCE, 0x497);
    }
    if exe::league_sort_table(this, 0) == 0 {
        return show_error_at(BRA_CHAMP_CUP_SOURCE, 0x49e);
    }
    covered!("cx01_11 preliminary started");
    cup.set_active(1);
}

/// Moves the Champions Cup on by one stage: at the last stage (`CURRENT_STAGE` ≥ `STAGE_COUNT` − 1, signed) the error
/// box of line 0x219 and nothing else; otherwise the stage goes up by one and, at 0 (the end of the preliminary), the
/// groups are built (`BraChampCup::create_group_stages`); at 2 (the end of the groups), the semi-finals
/// (`BraChampCup::create_semi_final_stage`).
/// Original: `BraChampCup::advance_stage` (0x4263c0, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_advance_stage(this: usize) {
    let cup = League(this);
    let stage = cup.current_stage() as i32;
    if stage >= (cup.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(BRA_CHAMP_CUP_SOURCE, 0x219);
    }
    let stage = stage + 1;
    write_i32(this + league::CURRENT_STAGE, stage);
    let next = match stage {
        0 => FN_BRA_CHAMP_CUP_CREATE_GROUP_STAGES, // the groups after the preliminary
        2 => FN_BRA_CHAMP_CUP_CREATE_SEMI_FINAL_STAGE, // the semi-finals after the groups
        _ => return,
    };
    covered!("cx01_11 next stage built");
    let build: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(next);
    build(this);
}

/// Comparer for `bra_champ_cup_create_preliminary_table`: only the original runs (the rows allocate and the news goes
/// to the humans); the plan made before must hold: 1 returned, three clubs, the first three entrants in the rows in
/// order and taken out of the book.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_create_preliminary_table(this: usize) -> u32 {
    let cup = League(this);
    let plan: Vec<usize> = (0..3).map(|k| read_usize(cup.entrant_book() + k * 6)).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    let table = cup.table();
    let ok = r == 1 && cup.club_count() == 3 && table != 0
        && plan.iter().enumerate().all(|(k, &c)| read_usize(table + k * league_table_row::SIZE) == c && read_usize(cup.entrant_book() + k * 6) == 0);
    SCOREBOARD.record(ok, || format!("bra_champ_cup_create_preliminary_table({:#x}): returned {}, plan {:x?}", this, r, plan));
    r
}

/// Comparer for `bra_champ_cup_start_preliminary_stage`: only the original runs (the table, the fixtures and the order
/// allocate and draw); then the competition must be active with three clubs.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_start_preliminary_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let cup = League(this);
    SCOREBOARD.record(cup.active() == 1 && cup.club_count() == 3, || format!("bra_champ_cup_start_preliminary_stage({:#x}): active {} clubs {}", this, cup.active(), cup.club_count()));
}

/// Comparer for `bra_champ_cup_advance_stage`: only the original runs (the groups and the semi-finals allocate and
/// draw); the stage stays at the last one and otherwise goes up by one (the groups write their own stage afterwards:
/// there it only has to move on).
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_advance_stage(this: usize) {
    let cup = League(this);
    let before = cup.current_stage() as i32;
    let last = before >= (cup.stage_count() as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let after = cup.current_stage() as i32;
    let ok = if last { after == before } else if before + 1 == 0 { after > before } else { after == before + 1 };
    SCOREBOARD.record(ok, || format!("bra_champ_cup_advance_stage({:#x}): stage {} -> {}", this, before, after));
}
