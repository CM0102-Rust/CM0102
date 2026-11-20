// ---------- mwa_26: the Northern Irish League Cup (nir_lge_cup.cpp): the knockout, the entrants and the schedule ----------

/// Rule: the Northern Irish League Cup's knockout (stage 3, built when the groups are over) takes the top two of each
/// of the four groups: the first and second of the cup's own table (the first group) and of each of the three group
/// stages, paired in order (winner, runner-up); out of memory the box of line 0x278.
///
/// The club vector (`exe::crt_malloc`(0x20)); slots 0 and 1 the first two rows of `league::TABLE`; slots 2 + 2i and
/// 3 + 2i the first and second rows of stage i's table (League::get_table_row `exe::league_get_table_row`(stage, row):
/// push row; mov ecx, stage -> thiscall, ret 4; the original fills the winners first, then the runners-up); then
/// `create_knockout_stage` (8 clubs, stage 3, no seeds, the box of line 0x297).
/// Original: `NirLgeCup::create_knockout_stage` (0x78d600, nir_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_create_knockout_stage(this: usize) {
    let this = League(this);
    let teams = exe::crt_malloc(0x20);
    if teams == 0 {
        return show_error_at(0xa69698, 0x278);
    }
    covered!("mwa_26 northern irish league cup knockout");
    write_usize(teams, read_usize(this.table()));
    for i in 0..3usize {
        let stage = read_usize(this.stages() + 4 * i);
        write_usize(teams + 8 * (i + 1), read_usize(exe::league_get_table_row(stage, 0)));
    }
    write_usize(teams + 4, read_usize(this.table() + league_table_row::SIZE)); // mwa26: the first group's runner-up
    for i in 0..3usize {
        let stage = read_usize(this.stages() + 4 * i);
        write_usize(teams + 8 * (i + 1) + 4, read_usize(exe::league_get_table_row(stage, 1)));
    }
    create_knockout_stage(this, 8, teams, 3, core::ptr::null(), 0xa69698, 0x297);
}

/// Comparer for `nir_lge_cup_create_knockout_stage` (original only: it builds the stage and draws its first round;
/// `ita_cup_compare_knockout_stage` checks the stage in slot 3).
pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    ita_cup_compare_knockout_stage("nir_lge_cup_create_knockout_stage", League(this), 3, 0, || o(this));
}

/// The body of `nir_lge_cup_collect_entrants`; returns (the routine's return, the entries to compare).
///
/// `ita_cup_fields::ENTRANT_CLUB_COUNT` = 20; the pool (`exe::crt_calloc`(4, 20) into `ita_cup_fields::ENTRANT_CLUBS`;
/// none: the box of line 0x2b3 and 0). First pass over the clubs (`G_CLUB_COUNT`, a signed word index, in table order),
/// those of Northern Ireland (`club::NATION`'s first dword = [`KEY_NATION_NORTHERN_IRELAND`]) with a
/// `club::RESERVE_DIVISION` of id one of the Northern Irish groups A to D (`KEY_CLUB_COMP_NORTHERN_IRISH_GROUP_*`): the
/// first free slot of that group's band of five (A 0 to 4, B 5 to 9, C 10 to 14, D 15 to 19; a full band: nothing).
/// Second pass, the Northern Irish clubs with no reserve division: the first free slot of the 20 (none: nothing).
/// Returns 1.
pub(crate) unsafe fn collect_northern_irish_league_cup_entrants(this: League) -> (i32, i32) {
    let t = this.addr();
    write_i32(t + ita_cup_fields::ENTRANT_CLUB_COUNT, 0x14);
    let v = exe::crt_calloc(4, 0x14);
    write_usize(t + ita_cup_fields::ENTRANT_CLUBS, v);
    if v == 0 {
        show_error_at(0xa69698, 0x2b3);
        return (0, 0);
    }
    let northern = |c: Club| !c.nation().is_null() && read_usize(c.nation().addr()) == read_usize(KEY_NATION_NORTHERN_IRELAND);
    let mut i = 0i16;
    while (i as i32) < read_i32(G_CLUB_COUNT) {
        let c = club_at(i as isize);
        let r = c.reserve_division();
        if northern(c) && !r.is_null() && read_usize(r.addr()) != 0 {
            let id = read_usize(r.addr());
            let band = if id == read_usize(KEY_CLUB_COMP_NORTHERN_IRISH_GROUP_A) { Some((0, 5)) } // mwa26: the group A band
                else if id == read_usize(KEY_CLUB_COMP_NORTHERN_IRISH_GROUP_B) { Some((5, 0xa)) }
                else if id == read_usize(KEY_CLUB_COMP_NORTHERN_IRISH_GROUP_C) { Some((0xa, 0xf)) }
                else if id == read_usize(KEY_CLUB_COMP_NORTHERN_IRISH_GROUP_D) { Some((0xf, 0x14)) }
                else { None };
            if let Some((lo, hi)) = band {
                if let Some(j) = first_free_entrant_slot(v, lo, hi) {
                    write_usize(v + 4 * j, c.addr());
                }
            }
        }
        i = i.wrapping_add(1);
    }
    covered!("mwa_26 northern irish league cup entrants");
    let mut i = 0i16;
    while (i as i32) < read_i32(G_CLUB_COUNT) {
        let c = club_at(i as isize);
        if northern(c) && c.reserve_division().is_null() {
            if let Some(j) = first_free_entrant_slot(v, 0, 0x14) {
                write_usize(v + 4 * j, c.addr()); // mwa26: a club with no group in the first free place
            }
        }
        i = i.wrapping_add(1);
    }
    (1, 0x14)
}

/// Rule: the Northern Irish League Cup takes 20 Northern Irish clubs: those the database assigns to the cup's groups A
/// to D (through the club's reserve division), five a group, and, for the free places, the other Northern Irish clubs
/// with no group, in table order.
/// Original: `NirLgeCup::collect_entrants` (0x78d800, nir_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_collect_entrants(this: usize) -> i32 {
    collect_northern_irish_league_cup_entrants(League(this)).0
}

/// Comparer for `nir_lge_cup_collect_entrants` (side by side: `compare_select_entrants`).
pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    compare_select_entrants("nir_lge_cup_collect_entrants", this, || o(this), || collect_northern_irish_league_cup_entrants(League(this)))
}

/// The Northern Irish League Cup's five group matchdays, every other week from 29 August to 24 October (weekday 2,
/// code 2: midweek [inference]); (day, month from 0, weekday, code).
pub(crate) const NORTHERN_IRISH_LEAGUE_CUP_GROUP_ROUNDS: [ItaCupGroupRound; 5] = [(0x1d, 7, 2, 2), (0xc, 8, 2, 2), (0x1a, 8, 2, 2), (0xa, 9, 2, 2), (0x18, 9, 2, 2)]; // mwa26: the group matchdays

/// The Northern Irish League Cup's knockout: quarter-finals drawn on 25 October and played on 31 October (single
/// matches, tie rules 0x203); semi-finals over two legs seven days apart, drawn on 1 November and first played on 14
/// November (tie rule 4, two-leg rule 7); the final drawn on 22 November and played on 28 November (match code 4);
/// category 6; the tie winner's prize a share of the fund (−1).
pub(crate) const NORTHERN_IRISH_LEAGUE_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((0x19, 9, 0, 3), (0x1f, 9, 0, 2, 2, 0), 0x78, (0, 0x203, 0), 6, (8, 4, 8, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((1, 0xa, 0, 3), (0xe, 0xa, 0, 2, 2, 0), 0x82, (0, 4, 7), 6, (4, 2, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0x16, 0xa, 0, 3), (0x1c, 0xa, 0, 2, 2, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// Rule: the Northern Irish League Cup's groups (stages up to 2) play five matchdays, every other week from 29 August
/// to 24 October (a double round robin of five clubs [inference]), each group a league stage (is_league = 1, label
/// 0x3e9 + stage); the knockout (stage 3) has eight clubs: quarter-finals on 31 October, two-legged semi-finals from 14
/// November and the final on 28 November.
///
/// `create_schedule` with the knockout at stage 3, the label 0x3e9, `NORTHERN_IRISH_LEAGUE_CUP_GROUP_ROUNDS`,
/// `NORTHERN_IRISH_LEAGUE_CUP_KNOCKOUT_ROUNDS` and the boxes at lines 0xbc and 0xe3 of nir_lge_cup.cpp (read
/// instruction by instruction against IreLgeCup::create_schedule: only the stage, the dates, the rounds and the lines
/// change). Above stage 3: 0.
/// Original: `NirLgeCup::create_schedule` (0x78cad0, nir_lge_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_schedule(League(this), stage, rounds, extra, league, 3, 0x3e9, &NORTHERN_IRISH_LEAGUE_CUP_GROUP_ROUNDS, &NORTHERN_IRISH_LEAGUE_CUP_KNOCKOUT_ROUNDS, 0xa69698, 0xbc, 0xe3,
        "mwa_26 northern irish league cup groups", "mwa_26 northern irish league cup knockout rounds")
}

/// Comparer for `nir_lge_cup_create_schedule` (side by side: `ita_cup_compare_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    ita_cup_compare_schedule("nir_lge_cup_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| nir_lge_cup_create_schedule(this, stage, r, e, l), 3)
}
