// ---------- cx14_6: the Turkish category B group stages, promotion group and classification groups (tur_second_b.cpp) ----------

/// The round table of stage `stage` of a league (virtual `comp_vtable::CREATE_SCHEDULE`(stage, &rounds, &extra, 0):
/// push 0; push &extra; push &rounds; push stage -> thiscall, ret 0x10), with the rounds and the extra.
pub(crate) unsafe fn stage_round_table(this: League, stage: u32) -> (usize, i16, i16) {
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let t = schedule(this.addr(), stage, &mut rounds, &mut extra, 0);
    (t, rounds, extra)
}

/// A League_Stage of category B (operator new `exe::crt_operator_new` of 0xee; League_Stage::League_Stage
/// `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58; 0 when out of memory): this league, `n` clubs from `list`, two
/// legs, the competition, the round table with its rounds, the win, draw and category bytes and the tie-breaks of this
/// league, the four place counts `places` (up, to the play-offs, to the relegation play-off, down: a local array in
/// the original), the year, the stage, the extra as its label, `importance`, 1, 0, the rule bits `flags`, −1, 0, 2.
/// (The original pushes the rounds and the extra as dwords with stack leftovers above the word; the constructor reads
/// the words.)
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn category_b_league_stage(l: League, n: u32, list: usize, table: usize, rounds: i16, extra: i16, places: &[u8; 4], stage: u32, importance: u32, flags: u32) -> usize {
    let p = exe::crt_operator_new(0xee);
    if p == 0 {
        return 0;
    }
    let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    ctor(p, l.addr(), n, list, 2, l.competition().addr(), table, rounds as u16 as u32, l.win_points() as u32, l.draw_points() as u32, l.match_category() as u32,
        l.addr() + league::TIEBREAKS, places.as_ptr() as usize, l.year() as u16 as u32, stage, extra as u16 as u32, importance, 1, 0, flags, -1, 0, 2)
}

/// The clubs of category B group `group` (0 to 3: groups 2 to 5, [`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G2`] to
/// `_G5`; any other value is group 5), in database order among the first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`]:
/// those with a division and a reserve division, the reserve division's id the group's and the division this league's
/// competition. Returns the first ten and the clubs past the tenth.
pub(crate) unsafe fn category_b_group_clubs(l: League, group: u8) -> (Vec<usize>, Vec<usize>) {
    let key = match group {
        0 => KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G2,
        1 => KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G3,
        2 => KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G4,
        _ => KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G5,
    };
    let (mut first, mut extra) = (Vec::new(), Vec::new());
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2)) {
        let division = read_usize(club + club::DIVISION);
        let reserve = read_usize(club + club::RESERVE_DIVISION);
        if division != 0 && reserve != 0 && read_usize(reserve) == read_usize(key) && division == l.competition().addr() {
            if first.len() < 10 { first.push(club) } else { extra.push(club) }
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    (first, extra)
}

/// Builds category B group `group` + 2 as stage `group` (0 to 3): its round table (`stage_round_table`), a list of 10
/// (`exe::crt_malloc` of 0x28; none: the box at line 0x299) with `category_b_group_clubs`; each club past the tenth
/// shows the box at line 0x2aa and loses its division (`club::DIVISION` = 0, a defect of the original), and the scan
/// goes on. Fewer than ten: the box at line 0x2b7. Then the League_Stage (`category_b_league_stage`: places 0 up, 2 to
/// the play-offs, 0, 0; this league's importance and rule bits `league::LEAGUE_FLAGS`) into stage slot `group` (signed
/// byte); none: the box at line 0x2c2. Otherwise the list and the round table freed and the current stage = `group`. On
/// the error paths the round table (and the list) are not freed (leaks of the original).
/// Original: `TurSecondB::create_group_stage` (0x900780, tur_second_b.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn tur_second_b_create_group_stage(this: usize, group: u32) {
    let l = League(this);
    let group = group as u8;
    let (table, rounds, extra) = stage_round_table(l, group as u32);
    let list = exe::crt_malloc(0x28);
    if list == 0 {
        return show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x299);
    }
    let (first, past) = category_b_group_clubs(l, group);
    for (k, &club) in first.iter().enumerate() {
        write_usize(list + 4 * k, club); // cx14_6: the k-th club of the group
    }
    for &club in &past {
        show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x2aa);
        write_usize(club + club::DIVISION, 0);
    }
    if first.len() < 10 {
        return show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x2b7);
    }
    let slot = group as i8 as isize as usize;
    let stage = category_b_league_stage(l, 10, list, table, rounds, extra, &[0, 2, 0, 0], group as u32, l.importance() as u32, l.league_flags() as u16 as u32); // cx14_6: 2 to the play-offs
    write_usize(l.stages() + 4 * slot, stage);
    if read_usize(l.stages() + 4 * slot) == 0 {
        return show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x2c2);
    }
    covered!("cx14_6 category b group stage");
    exe::crt_free(list);
    exe::crt_free(table);
    l.set_current_stage(slot);
}

/// The clubs of a stage's table, in row order.
pub(crate) unsafe fn category_b_stage_clubs(stage: League) -> Vec<usize> {
    (0..stage.club_count().max(0) as usize).map(|i| aus_nsl_table_row_club(stage, i)).collect()
}

/// Checks a built category B stage against the planned clubs (as a set: the stage may order its rows) and the
/// current stage.
pub(crate) unsafe fn compare_category_b_stage(name: &str, l: League, slot: usize, plan: &[usize]) {
    let stage = League(read_usize(l.stages() + 4 * slot));
    let mut found = if stage.is_null() { Vec::new() } else { category_b_stage_clubs(stage) };
    let mut want = plan.to_vec();
    found.sort_unstable();
    want.sort_unstable();
    SCOREBOARD.record(found == want && l.current_stage() == slot, || format!("{}({:#x}) stage {} current {}: plan {:x?} found {:x?}", name, l.addr(), slot, l.current_stage() as i32, want, found));
}

/// Comparer for `tur_second_b_create_group_stage`: only the original runs (it allocates the stage); with ten clubs in
/// the plan, the stage's clubs and the current stage against it.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_create_group_stage(this: usize, group: u32) {
    let l = League(this);
    let (plan, _) = category_b_group_clubs(l, group as u8);
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%)(this, group);
    if plan.len() == 10 {
        compare_category_b_stage("tur_second_b_create_group_stage", l, group as u8 as i8 as isize as usize, &plan);
    }
}

/// The rows of the five first phase groups (the league's own table, g = −1, then the group stages 0..3), in order.
pub(crate) unsafe fn category_b_first_phase_rows(l: League) -> Vec<(i8, LeagueTableRow)> {
    let mut v = Vec::new();
    for g in -1i8..4 {
        let t = if g == -1 { l } else { League(read_usize(l.stages() + 4 * g as usize)) };
        let mut i = 0i16;
        while i < t.club_count() {
            v.push((g, LeagueTableRow(t.table() + i as usize * league_table_row::SIZE)));
            i += 1;
        }
    }
    v
}

/// The promotion group's clubs: the clubs of the rows of status 1 (through) in the five first phase groups, in order;
/// the first ten and how many more.
pub(crate) unsafe fn category_b_promotion_clubs(l: League) -> (Vec<usize>, usize) {
    let through: Vec<usize> = category_b_first_phase_rows(l).into_iter().filter(|(_, r)| r.status() == 1).map(|(_, r)| read_usize(r.addr())).collect();
    let more = through.len().saturating_sub(10);
    (through.into_iter().take(10).collect(), more)
}

/// Builds the promotion group of category B as stage 4: the clubs of `category_b_promotion_clubs` in a local list of
/// ten (each one past the tenth: the box at line 0x2f2); its round table (`stage_round_table`(4)); the League_Stage
/// (`category_b_league_stage`: places 2 up, 3 to the play-offs, 0, 0; importance 0x14 and rule bits 3) into slot 4;
/// none: the box at line 0x300 (the round table not freed). Otherwise the round table freed and the current stage = 4.
/// With fewer than ten clubs through the original passes stack leftovers in the empty slots and the copy zeros
/// (never seen: the five groups send two each).
/// Original: `TurSecondB::create_promotion_group` (0x900ab0, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_create_promotion_group(this: usize) {
    let l = League(this);
    let (clubs, more) = category_b_promotion_clubs(l);
    let mut list = [0usize; 10];
    list[..clubs.len()].copy_from_slice(&clubs);
    for _ in 0..more {
        show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x2f2);
    }
    let (table, rounds, extra) = stage_round_table(l, 4);
    let stage = category_b_league_stage(l, 10, list.as_ptr() as usize, table, rounds, extra, &[2, 3, 0, 0], 4, 0x14, 3); // cx14_6: 2 up, 3 to the play-offs
    write_usize(l.stages() + 4 * 4, stage);
    if read_usize(l.stages() + 4 * 4) == 0 {
        return show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x300);
    }
    covered!("cx14_6 category b promotion group");
    exe::crt_free(table);
    l.set_current_stage(4);
}

/// Comparer for `tur_second_b_create_promotion_group`: only the original runs; with ten clubs through, the stage's
/// clubs and the current stage against the plan.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_create_promotion_group(this: usize) {
    let l = League(this);
    let (plan, _) = category_b_promotion_clubs(l);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    if plan.len() == 10 {
        compare_category_b_stage("tur_second_b_create_promotion_group", l, 4, &plan);
    }
}

/// The clubs of first phase group g (−1 to 3) that did not go through (rows of status other than 1), in row order;
/// the first eight and how many more.
pub(crate) unsafe fn category_b_classification_clubs(l: League, g: i8) -> (Vec<usize>, usize) {
    let out: Vec<usize> = category_b_first_phase_rows(l).into_iter().filter(|(k, r)| *k == g && r.status() != 1).map(|(_, r)| read_usize(r.addr())).collect();
    let more = out.len().saturating_sub(8);
    (out.into_iter().take(8).collect(), more)
}

/// Builds the five classification groups of category B, stages 5 to 9, one for each first phase group g = −1 to 3
/// (stage g + 6): the clubs of `category_b_classification_clubs` in a local list of eight (each one past the eighth:
/// the box at line 0x333), its round table, the League_Stage (`category_b_league_stage` with eight clubs: places 0, 1
/// to the play-offs, 0, 2 down; importance 0x14 and rule bits 2) into its slot. None: the box at line 0x340 and the
/// remaining groups are not built (the round table not freed). Otherwise the round table freed, the current stage =
/// the new one and each of the eight new rows takes the first phase record of its club (the 42 bytes of the totals,
/// `league_table_row::PLAYED` − 4 to +0x36, from the club's row in its first phase group, looked for in row order)
/// with the played counts (overall, home and away) cleared. With fewer than eight clubs the original passes stack
/// leftovers in the empty slots and the copy zeros (never seen: each group of ten keeps eight).
/// Original: `TurSecondB::create_classification_groups` (0x900d00, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_create_classification_groups(this: usize) {
    let l = League(this);
    for g in -1i8..4 {
        let slot = (g + 6) as usize;
        let group = if g == -1 { l } else { League(read_usize(l.stages() + 4 * g as usize)) };
        let (clubs, more) = category_b_classification_clubs(l, g);
        let mut list = [0usize; 8];
        list[..clubs.len()].copy_from_slice(&clubs);
        for _ in 0..more {
            show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x333);
        }
        let (table, rounds, extra) = stage_round_table(l, slot as u32);
        let stage = category_b_league_stage(l, 8, list.as_ptr() as usize, table, rounds, extra, &[0, 1, 0, 2], slot as u32, 0x14, 2); // cx14_6: 1 to the play-offs, 2 down
        write_usize(l.stages() + 4 * slot, stage);
        if read_usize(l.stages() + 4 * slot) == 0 {
            return show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x340);
        }
        exe::crt_free(table);
        l.set_current_stage(slot);
        covered!("cx14_6 category b classification group");
        let stage = League(stage);
        for k in 0..8 {
            let row = stage.table() + k * league_table_row::SIZE;
            let club = read_usize(row);
            let mut j = 0i16;
            while j < group.club_count() {
                let from = group.table() + j as usize * league_table_row::SIZE;
                if read_usize(from) == club {
                    carry_first_phase_record(from, row); // cx14_6: the first phase record carried over
                    break;
                }
                j += 1;
            }
        }
    }
}

/// Copies the totals of a first phase row into a classification row (`rep movsd` of ten dwords and a word from
/// +0xd) and clears the played counts (`league_table_row::PLAYED`, `HOME_PLAYED`, `AWAY_PLAYED`).
pub(crate) unsafe fn carry_first_phase_record(from: usize, to: usize) {
    core::ptr::copy_nonoverlapping((from + 0xd) as *const u8, (to + 0xd) as *mut u8, 0x2a);
    write_u8(to + league_table_row::PLAYED, 0);
    write_u8(to + league_table_row::HOME_PLAYED, 0);
    write_u8(to + league_table_row::AWAY_PLAYED, 0);
}

/// Comparer for `tur_second_b_create_classification_groups`: only the original runs (it allocates the stages); each
/// built stage's clubs against the plan (as a set) and each of its rows against the club's first phase row (the 42
/// bytes from +0xd, the three played counts cleared).
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_create_classification_groups(this: usize) {
    let l = League(this);
    let plans: Vec<(Vec<usize>, Vec<Vec<u8>>)> = (-1i8..4).map(|g| {
        let (clubs, _) = category_b_classification_clubs(l, g);
        let records = clubs.iter().map(|&c| {
            let row = category_b_first_phase_rows(l).into_iter().find(|(k, r)| *k == g && read_usize(r.addr()) == c).map(|(_, r)| r.addr()).unwrap_or(0);
            let mut b = if row != 0 { core::slice::from_raw_parts((row + 0xd) as *const u8, 0x2a).to_vec() } else { Vec::new() };
            if !b.is_empty() {
                b[4] = 0;
                b[0x12] = 0;
                b[0x20] = 0;
            }
            b
        }).collect();
        (clubs, records)
    }).collect();
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    for (k, (clubs, records)) in plans.iter().enumerate() {
        let stage = League(read_usize(l.stages() + 4 * (k + 5)));
        if stage.is_null() || clubs.len() < 8 {
            continue;
        }
        let mut found = category_b_stage_clubs(stage);
        let rows_ok = (0..8).all(|r| {
            let row = stage.table() + r * league_table_row::SIZE;
            let i = clubs.iter().position(|&c| c == read_usize(row));
            i.is_some_and(|i| core::slice::from_raw_parts((row + 0xd) as *const u8, 0x2a) == &records[i][..])
        });
        let mut want = clubs.clone();
        found.sort_unstable();
        want.sort_unstable();
        SCOREBOARD.record(found == want && rows_ok, || format!("tur_second_b_create_classification_groups({:#x}) stage {}: plan {:x?} found {:x?} rows {}", this, k + 5, want, found, rows_ok));
    }
}
