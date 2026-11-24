// ---------- cx09_13: the Belgian Third Division constructor and its group B stage (bel_third.cpp) ----------

/// The Belgian Third Division vtable.
const BEL_THIRD_VTABLE: usize = 0x967b48;

/// The Belgian Third Division as a nation division, for the constructor's comparer (`compare_nation_division_ctor`):
/// no bench written, the Belgian discipline rule, 10 stages (group B, the six period tables, the title decider and the
/// two play-offs) and the error box lines of the rules, the table, the fixtures, the order and the statistics.
const BEL_THIRD_DIVISION: NationDivision = NationDivision {
    vtable: BEL_THIRD_VTABLE,
    bench: None,
    discipline: discipline_rule::BELGIUM_LEAGUE,
    stages: 10,
    set_up_rules: 0x4226f0,
    create_table: 0x4224e0,
    source_file: BEL_THIRD_SOURCE,
    box_lines: [0x4a, 0x51, 0x67, 0x58, 0x60],
};

/// Builds the second group of a league played in two groups (stage 0): the round table through virtual
/// `comp_vtable::CREATE_SCHEDULE` (0, &rounds, &extra, 0); a list of `league::CLUB_COUNT` clubs (malloc; none: the box
/// of `lines[0]`) with, in club order (index 0 to `G_CLUB_COUNT` − 2 × `G_NATION_COUNT`, read again at each step), the
/// clubs whose `club::RESERVE_DIVISION` id is the group's `key` and whose `club::DIVISION` is this competition; one past
/// the count shows the box of `lines[1]` and loses its division (`club::DIVISION` = 0, a defect of the original), and
/// the scan goes on. Short of the count: the box of `lines[2]`. Then a League_Stage (`exe::crt_operator_new`(0xee);
/// 0x68b100: this league, the club count, the list, `legs`, the competition, the round table, the rounds, the win, draw
/// and category bytes, the tie-breaks, the places (+0xbe), the year, stage 0, the extra as its label, the importance,
/// 1, 0, the rule bits `league::LEAGUE_FLAGS`, −1, 0, 2) into stage slot 0; none: the box of `lines[3]`; otherwise the
/// list and the round table freed and `league::CURRENT_STAGE` = 0. On the error paths the round table (and the list)
/// are not freed (leaks of the original). Returns whether the stage was built.
unsafe fn create_second_group_stage(l: League, key: usize, legs: u32, file: usize, lines: [u32; 4]) -> bool {
    let this = l.addr();
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = schedule(this, 0, &mut rounds, &mut extra, 0);
    let list = exe::crt_malloc((l.club_count() as i32 as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(file, lines[0]);
        return false;
    }
    let mut n: i16 = 0;
    let mut i = 0i32;
    while i < bel_third_club_scan_count() {
        let club = club_at(i as isize);
        let (division, reserve) = (club.division().addr(), club.reserve_division().addr());
        if division != 0 && reserve != 0 && read_u32(reserve) == read_u32(key) && division == l.competition().addr() {
            if n >= l.club_count() {
                show_error_at(file, lines[1]);
                write_usize(club.addr() + club::DIVISION, 0);
            } else {
                write_usize(list + 4 * n as usize, club.addr());
                n += 1;
            }
        }
        i += 1;
    }
    if n < l.club_count() {
        show_error_at(file, lines[2]);
        return false;
    }
    let p = exe::crt_operator_new(0xee);
    let s = if p != 0 {
        let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        ctor(p, this, l.club_count() as u16 as u32, list, legs, l.competition().addr(), table, rounds as u16 as u32, l.win_points() as u32, l.draw_points() as u32,
            l.match_category() as u32, this + league::TIEBREAKS, this + 0xbe, l.year() as u16 as u32, 0, extra as u16 as u32, l.importance() as u32, 1, 0,
            l.league_flags() as u16 as u32, -1, 0, 2) // +0xbe: the places
    } else {
        0
    };
    write_usize(l.stages(), s);
    if s == 0 {
        show_error_at(file, lines[3]);
        return false;
    }
    exe::crt_free(list);
    exe::crt_free(table);
    l.set_current_stage(0);
    true
}

/// Builds the Belgian Third Division group B (`create_second_group_stage`): the clubs that came from Third Division B,
/// home and away (2 legs), with the bel_third.cpp boxes of lines 0x2d8, 0x2e9, 0x2f6 and 0x301.
/// Original: `BelThird::create_group_b_stage` (0x422970, bel_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_third_create_group_b_stage(this: usize) {
    if create_second_group_stage(League(this), KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION_B, 2, BEL_THIRD_SOURCE, [0x2d8, 0x2e9, 0x2f6, 0x301]) {
        covered!("cx09_13 belgian third division group b");
    }
}

/// Builds the Belgian Third Division: League::League (`exe::new_league`), the competition record, the vtable, the
/// year and the Belgian discipline rule; then the saved state (`exe::league_load_from_temp_file`(1)): loaded, done.
/// Otherwise a new league: stage index 0xff, current stage −1, 10 stages and their vector (malloc of 0x28, unchecked);
/// then, each failure showing the bel_third.cpp box of its line and ending: the rules (BelThird::set_up_rules 0x4226f0,
/// line 0x4a), the table (BelThird::fill_league_table 0x4224e0, 0x51), the order (`exe::league_sort_table`(0), 0x58:
/// before the fixtures, unlike the other divisions), the statistics (operator new of 0x5ce and
/// `exe::new_comp_stats` into `league::COMP_STATS`, 0x60) and the fixtures (`exe::league_create_fixtures`, 0x67); then
/// the group B stage (0x422970) and the predicted positions (0x424030), all called directly (through their detours).
/// Returns the object.
/// Original: `BelThird::BelThird` (0x4216f0, bel_third.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bel_third(this: usize, year: u32, record: usize) -> usize {
    let l = League(this);
    exe::new_league(this);
    write_usize(this + league::COMPETITION, record);
    write_usize(this, BEL_THIRD_VTABLE);
    l.set_year(year as u16 as i16);
    l.set_discipline_rule(discipline_rule::BELGIUM_LEAGUE);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this;
    }
    covered!("cx09_13 new belgian third division");
    l.set_stage_index(stage_index::MAIN);
    l.set_current_stage(0xffff_ffff);
    l.set_stage_count(10);
    l.set_stages(exe::crt_malloc(0x28));
    let call = |f: usize| {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(f);
        f(this)
    };
    if call(0x4226f0) == 0 {
        show_error_at(BEL_THIRD_SOURCE, 0x4a);
        return this;
    }
    if call(0x4224e0) == 0 {
        show_error_at(BEL_THIRD_SOURCE, 0x51);
        return this;
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(BEL_THIRD_SOURCE, 0x58);
        return this;
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, this) } else { 0 };
    l.set_comp_stats(s);
    if s == 0 {
        show_error_at(BEL_THIRD_SOURCE, 0x60);
        return this;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(BEL_THIRD_SOURCE, 0x67);
        return this;
    }
    for f in [0x422970usize, 0x424030] {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
        f(this);
    }
    this
}

/// Comparer for `bel_third_create_group_b_stage`: only the original (it builds the stage); afterwards stage 0 must exist
/// and be the current one.
pub(crate) unsafe extern "thiscall" fn compare_bel_third_create_group_b_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let l = League(this);
    let s = read_usize(l.stages());
    SCOREBOARD.record(s != 0 && l.current_stage() as i32 == 0, || format!("bel_third_create_group_b_stage({:#x}): stage {:#x}, current {}", this, s, l.current_stage() as i32));
}

/// Comparer for `construct_bel_third`: only the original (it allocates and loads files), checked by
/// `compare_nation_division_ctor`.
pub(crate) unsafe extern "thiscall" fn compare_construct_bel_third(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_nation_division_ctor("construct_bel_third", League(this), year, record, &BEL_THIRD_DIVISION, || o(this, year, record))
}
