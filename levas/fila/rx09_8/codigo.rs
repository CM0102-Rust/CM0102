// ---------- rx09_8: the Swedish Second Division North table, turnover and next stage (swe_second.cpp) ----------

/// The ids of the six Swedish Second Division sections, in stage order: the North section (the league's own table),
/// then East Svealand, West Svealand, East Götaland, West Götaland and South Götaland (stages 0 to 4).
pub(crate) const SWEDISH_SECOND_SECTIONS: [usize; 6] = [KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_NORTH, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_SVEALAND,
    KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_SVEALAND, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_GOTALAND, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_GOTALAND,
    KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_SOUTH_GOTALAND];

/// The clubs of a Swedish Second Division section, in database order: those with a division (`club::DIVISION`) that is
/// this league's record and a reserve division (`club::RESERVE_DIVISION`, here the section) whose id is the section's
/// (`section`, a key).
pub(crate) unsafe fn swedish_second_section_clubs(l: League, section: usize) -> Vec<Club> {
    domestic_clubs().into_iter().filter(|c| {
        let division = read_usize(c.addr() + club::DIVISION);
        let reserve = read_usize(c.addr() + club::RESERVE_DIVISION);
        division != 0 && reserve != 0 && read_usize(reserve) == read_usize(section) && division == l.competition().addr()
    }).collect()
}

/// Marks a B team in row k of a section's table: a club linked to a senior club (`exe::club_get_linked_b_team`(club,
/// &senior, 1) non-zero with the senior flag 0) gets the row status 0xfc (barred from promotion).
pub(crate) unsafe fn mark_b_team_row(t: League, k: usize) {
    let row = t.table() + k * league_table_row::SIZE;
    let mut senior = 0u32;
    if exe::club_get_linked_b_team(read_usize(row), &mut senior, 1) != 0 && senior == 0 {
        write_u8(row + league_table_row::STATUS, 0xfc); // rx09_8: a B team cannot go up
    }
}

/// Builds the table of the North section of the Swedish Second Division: `club_count` = 12, the table
/// (`exe::crt_malloc` of 0x2c4; out of memory: the box at line 0x198 and 0) and, for each club of
/// `swedish_second_section_clubs` (the North), a row (League::init_table_row `exe::league_init_table_row`(this, k, club,
/// 0, 0)) and the B team mark (`mark_b_team_row`). A 13th club shows the box at line 0x1a9, loses its division
/// (`club::DIVISION` = 0) and ends the scan. Returns 1 when the table is full, else 0.
/// Original: `SweSecond::create_table` (0x8921b0, swe_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_second_create_table(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(0xc);
    let t = exe::crt_malloc(0x2c4);
    l.set_table(t);
    if t == 0 {
        show_error_at(SWEDISH_SECOND_SOURCE, 0x198);
        return 0;
    }
    covered!("rx09_8 swedish second north table");
    let mut k: u8 = 0;
    for c in swedish_second_section_clubs(l, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_NORTH) {
        if (k as i8 as i16) >= l.club_count() {
            show_error_at(SWEDISH_SECOND_SOURCE, 0x1a9);
            write_usize(c.addr() + club::DIVISION, 0);
            break;
        }
        exe::league_init_table_row(l, k as u32, c, 0, 0); // rx09_8: the row of the k-th North club
        mark_b_team_row(l, k as i8 as isize as usize);
        k = k.wrapping_add(1);
    }
    ((k as i8 as i16) == l.club_count()) as u32
}

/// Comparer for `swe_second_create_table`: only the original runs (it allocates the table); the club count, the result,
/// the clubs of the rows and the B team marks against the North clubs, planned beforehand.
pub(crate) unsafe extern "thiscall" fn compare_swe_second_create_table(this: usize) -> u32 {
    let l = League(this);
    let plan: Vec<usize> = swedish_second_section_clubs(l, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_NORTH).iter().take(12).map(|c| c.addr()).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    let found: Vec<usize> = if l.table() != 0 { (0..plan.len()).map(|k| aus_nsl_table_row_club(l, k)).collect() } else { Vec::new() };
    let same = r == (plan.len() == 12) as u32 && l.club_count() == 12 && found == plan;
    SCOREBOARD.record(same, || format!("swe_second_create_table({:#x}) original={} plan={:x?} table={:x?}", this, r, plan, found));
    r
}

/// Rule: the Swedish Second Division turnover: last season's stages destroyed, the next year, the rules, the North
/// table, the order and the fixtures, then the other five sections, the predicted positions and the officials
/// (`grouped_league_new_season` with SweSecond::create_table 0x8921b0 and SweSecond::create_section_stage 0x892670,
/// boxes at lines 0x20c, 0x213, 0x21a and 0x220).
/// Original: `SweSecond::start_new_season` (0x892410, swe_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_second_start_new_season(this: usize) -> u32 {
    grouped_league_new_season(League(this), 0x8921b0, 0x892670, 5, SWEDISH_SECOND_SOURCE, [0x20c, 0x213, 0x21a, 0x220], "rx09_8 swedish second turnover")
}

/// Comparer for `swe_second_start_new_season`: only the original runs (it frees and rebuilds the league); the year must
/// go up by one.
pub(crate) unsafe extern "thiscall" fn compare_swe_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("swe_second_start_new_season", League(this), || o(this))
}

/// Rule: the next stage of the Swedish Second Division: past the last stage, the error box; otherwise the current stage
/// goes up by one; on reaching stage 5 (the sections are over) the promotion play-off and the relegation play-off are
/// built.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box at line 0x23e and nothing
/// else. Otherwise the current stage + 1; at 5, SweSecond::create_promotion_play_off_stage (0x892a10) and
/// SweSecond::create_relegation_play_off_stage (0x892bb0) (thiscall, ret 0).
/// Original: `SweSecond::advance_stage` (0x8925d0, swe_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_second_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(SWEDISH_SECOND_SOURCE, 0x23e);
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    covered!("rx09_8 swedish second next stage");
    if stage == 5 {
        call_on_league(0x892a10, l);
        call_on_league(0x892bb0, l);
    }
}

/// Comparer for `swe_second_advance_stage` (`compare_advance_stage_building`: stage 5 builds).
pub(crate) unsafe extern "thiscall" fn compare_swe_second_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_advance_stage_building("swe_second_advance_stage", League(this), |s| s == 5, || o(this), || swe_second_advance_stage(this))
}
