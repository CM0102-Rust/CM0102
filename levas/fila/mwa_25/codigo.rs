// ---------- mwa_25: the Irish League Cup (ire_lge_cup.cpp): the knockout, the entrants and the schedule ----------

/// Rule: the Irish League Cup's knockout (stage 7, built when the seven groups are over) takes the eight group
/// winners: the top club of the cup's own table (the first group) and of each of the seven group stages, in order; out
/// of memory the box of line 0x278.
///
/// The club vector (`exe::crt_malloc`(0x20)); slot 0 the first row of `league::TABLE`, slots 1 to 7 the first row of
/// stage i's table (League::get_table_row `exe::league_get_table_row`(stage, 0): push 0; mov ecx, stage -> thiscall,
/// ret 4); then `create_knockout_stage` (8 clubs, stage 7, no seeds, the box of line 0x290; the frame of
/// ItaCup::create_knockout_stage: the round table of stage 7, Cup_Stage::Cup_Stage, the vector and the table freed, the
/// first draw).
/// Original: `IreLgeCup::create_knockout_stage` (0x637c00, ire_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_create_knockout_stage(this: usize) {
    let this = League(this);
    let teams = exe::crt_malloc(0x20);
    if teams == 0 {
        return show_error_at(0x9e9f4c, 0x278);
    }
    covered!("mwa_25 irish league cup knockout");
    write_usize(teams, read_usize(this.table())); // mwa25: the first group's winner
    for i in 0..7usize {
        let stage = read_usize(this.stages() + 4 * i);
        write_usize(teams + 4 * (i + 1), read_usize(exe::league_get_table_row(stage, 0)));
    }
    create_knockout_stage(this, 8, teams, 7, core::ptr::null(), 0x9e9f4c, 0x290);
}

/// Comparer for `ire_lge_cup_create_knockout_stage` (original only: it builds the stage and draws its first round;
/// `ita_cup_compare_knockout_stage` checks the stage in slot 7).
pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    ita_cup_compare_knockout_stage("ire_lge_cup_create_knockout_stage", League(this), 7, 0, || o(this));
}

/// The first free slot of `v` in `lo..hi` (signed words, as the original counts), or None.
pub(crate) unsafe fn first_free_entrant_slot(v: usize, lo: i16, hi: i16) -> Option<usize> {
    let mut j = lo;
    while j < hi {
        if read_usize(v + 4 * j as usize) == 0 {
            return Some(j as usize);
        }
        j += 1;
    }
    None
}

/// The body of `ire_lge_cup_collect_entrants`; returns (the routine's return, the entries to compare).
///
/// `ita_cup_fields::ENTRANT_CLUB_COUNT` = 26; the pool (`exe::crt_calloc`(4, 26) into `ita_cup_fields::ENTRANT_CLUBS`;
/// none: the box of line 0x2af and 0). First pass over the clubs (`G_CLUB_COUNT`, a signed word index, in table order),
/// those of the Republic of Ireland (`club::NATION`'s first dword = [`KEY_NATION_REPUBLIC_OF_IRELAND`]): with a
/// `club::RESERVE_DIVISION` whose id is one of the Irish groups A to E (`KEY_CLUB_COMP_IRISH_GROUP_*`), the first free
/// slot of that group's band (A 0 to 5, B 6 to 11, C 12 to 17, D 18 to 21, E 22 to 25; a full band: nothing); with any
/// other reserve division, nothing; with none (or an id 0) but a `club::DIVISION`, a local list of up to 100. The list
/// is sorted and shuffled by `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 20) (cdecl); while the clubs placed
/// are fewer than 26, each listed club goes to the first free slot of the band of its division's regional league
/// (Leinster 0 to 5, Connacht 12 to 17, Munster 18 to 21, Ulster 22 to 25, any other 6 to 11). The first three bands of
/// six are shuffled (`exe::comp_util_shuffle_pointer_array`(band, 6)). If fewer than 26 were placed, each empty slot
/// takes the first club in table order with a division, of the nation record of [`KEY_NATION_REPUBLIC_OF_IRELAND`]
/// (`G_NATION_TABLE` + id × 0x122) and not yet in the pool. Returns 1.
pub(crate) unsafe fn collect_irish_league_cup_entrants(this: League) -> (i32, i32) {
    let t = this.addr();
    write_i32(t + ita_cup_fields::ENTRANT_CLUB_COUNT, 0x1a);
    let v = exe::crt_calloc(4, 0x1a);
    write_usize(t + ita_cup_fields::ENTRANT_CLUBS, v);
    if v == 0 {
        show_error_at(0x9e9f4c, 0x2af);
        return (0, 0);
    }
    let mut list = [0usize; 100];
    let (mut n, mut placed) = (0i16, 0i32);
    let mut i = 0i16;
    while (i as i32) < read_i32(G_CLUB_COUNT) {
        let c = club_at(i as isize);
        let nation = c.nation();
        if !nation.is_null() && read_usize(nation.addr()) == read_usize(KEY_NATION_REPUBLIC_OF_IRELAND) {
            let r = c.reserve_division();
            if !r.is_null() && read_usize(r.addr()) != 0 {
                let id = read_usize(r.addr());
                let band = if id == read_usize(KEY_CLUB_COMP_IRISH_GROUP_A) { Some((0, 6)) } // mwa25: the group A band
                    else if id == read_usize(KEY_CLUB_COMP_IRISH_GROUP_B) { Some((6, 0xc)) }
                    else if id == read_usize(KEY_CLUB_COMP_IRISH_GROUP_C) { Some((0xc, 0x12)) }
                    else if id == read_usize(KEY_CLUB_COMP_IRISH_GROUP_D) { Some((0x12, 0x16)) }
                    else if id == read_usize(KEY_CLUB_COMP_IRISH_GROUP_E) { Some((0x16, 0x1a)) }
                    else { None };
                if let Some((lo, hi)) = band {
                    if let Some(j) = first_free_entrant_slot(v, lo, hi) {
                        write_usize(v + 4 * j, c.addr());
                        placed += 1;
                    }
                }
            } else if !c.division().is_null() && n < 100 {
                list[n as usize] = c.addr();
                n += 1;
            }
        }
        i = i.wrapping_add(1);
    }
    covered!("mwa_25 irish league cup entrants");
    exe::comp_util_sort_and_shuffle_top_clubs(list.as_ptr() as usize, n as i32 as u32, 0x14);
    let mut k = 0i16;
    while k < n && placed < read_i32(t + ita_cup_fields::ENTRANT_CLUB_COUNT) {
        let c = Club(list[k as usize]);
        let id = read_usize(c.division().addr());
        let (lo, hi) = if id == read_usize(KEY_CLUB_COMP_IRISH_LEINSTER_SENIOR_LEAGUE) { (0, 6) }
            else if id == read_usize(KEY_CLUB_COMP_IRISH_CONNACHT_SENIOR_LEAGUE) { (0xc, 0x12) } // mwa25: the Connacht band
            else if id == read_usize(KEY_CLUB_COMP_IRISH_MUNSTER_SENIOR_LEAGUE) { (0x12, 0x16) }
            else if id == read_usize(KEY_CLUB_COMP_IRISH_ULSTER_SENIOR_LEAGUE) { (0x16, 0x1a) }
            else { (6, 0xc) };
        if let Some(j) = first_free_entrant_slot(v, lo, hi) {
            write_usize(v + 4 * j, c.addr());
            placed += 1;
        }
        k += 1;
    }
    exe::comp_util_shuffle_pointer_array(v, 6);
    exe::comp_util_shuffle_pointer_array(v + 0x18, 6);
    exe::comp_util_shuffle_pointer_array(v + 0x30, 6);
    if placed < read_i32(t + ita_cup_fields::ENTRANT_CLUB_COUNT) {
        let mut j = 0i16;
        while (j as i32) < read_i32(t + ita_cup_fields::ENTRANT_CLUB_COUNT) {
            let count = read_i32(t + ita_cup_fields::ENTRANT_CLUB_COUNT);
            if read_usize(v + 4 * j as usize) == 0 {
                let ireland = read_usize(G_NATION_TABLE).wrapping_add(read_usize(KEY_NATION_REPUBLIC_OF_IRELAND).wrapping_mul(0x122));
                let mut q = 0i32;
                while q < read_i32(G_CLUB_COUNT) {
                    let c = club_at(q as isize);
                    if !c.division().is_null() && c.nation().addr() == ireland
                        && !(0..count.max(0) as usize).any(|k| read_usize(v + 4 * k) == c.addr()) {
                        write_usize(v + 4 * j as usize, c.addr());
                        break;
                    }
                    q += 1;
                }
            }
            j = j.wrapping_add(1);
        }
    }
    (1, 0x1a)
}

/// Rule: the Irish League Cup takes 26 Irish clubs: those the database assigns to the cup's groups A to E (through the
/// club's reserve division: six each in A, B and C, four each in D and E) and, for the free places, the other Irish
/// clubs ranked by reputation with a shuffle among the best 20, each into the band of its regional league (Leinster
/// with A, Connacht with C, Munster with D, Ulster with E, the rest with B); the first three bands are shuffled; places
/// still free take the first Irish clubs in table order not yet in.
/// Original: `IreLgeCup::collect_entrants` (0x637dd0, ire_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_collect_entrants(this: usize) -> i32 {
    collect_irish_league_cup_entrants(League(this)).0
}

/// Comparer for `ire_lge_cup_collect_entrants` (side by side with the random generator restored: `compare_select_entrants`).
pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    compare_select_entrants("ire_lge_cup_collect_entrants", this, || o(this), || collect_irish_league_cup_entrants(League(this)))
}

/// The Irish League Cup's three group matchdays: 20 September (weekday 2, code 2: midweek [inference]), 24 September
/// (weekday 6, code 1: the weekend) and 27 September (weekday 2, code 2); (day, month from 0, weekday, code).
pub(crate) const IRISH_LEAGUE_CUP_GROUP_ROUNDS: [ItaCupGroupRound; 3] = [(0x14, 8, 2, 2), (0x18, 8, 6, 1), (0x1b, 8, 2, 2)]; // mwa25: the first group matchday

/// The Irish League Cup's knockout: quarter-finals drawn on 28 September and played on 8 November, semi-finals drawn on
/// 9 November and played on 29 November, single matches with tie rule 3; the final over two legs 21 days apart, drawn
/// on 30 November and first played on 24 January of the next year (two-leg tie rule 7); category 6; the tie winner's
/// prize a share of the fund (−1).
pub(crate) const IRISH_LEAGUE_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((0x1c, 8, 0, 3), (8, 0xa, 0, 2, 2, 0), 0x78, (0, 3, 0), 6, (8, 4, 8, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((9, 0xa, 0, 3), (0x1d, 0xa, 0, 2, 2, 0), 0x82, (0, 3, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0x1e, 0xa, 0, 3), (0x18, 0, 1, 2, 2, 0), 0x96, (0, 0, 7), 6, (2, 1, 0, 0), (0, 2, 0x15), (0, 0xffff_ffff, 0)),
];

/// Rule: the Irish League Cup's groups (stages up to 6) play three matchdays in late September (20, 24 and 27; one
/// round robin of three clubs, or four in the last two groups), each group a league stage (is_league = 1, label 0x3e9
/// + stage); the knockout (stage 7) has eight clubs: quarter-finals on 8 November and semi-finals on 29 November (single
/// matches), a two-legged final from 24 January of the next year.
///
/// `create_schedule` with the knockout at stage 7, the label 0x3e9, `IRISH_LEAGUE_CUP_GROUP_ROUNDS`,
/// `IRISH_LEAGUE_CUP_KNOCKOUT_ROUNDS` and the boxes at lines 0xbc and 0xdb of ire_lge_cup.cpp (read instruction by
/// instruction against ItaCCup::create_schedule: only the stage, the label, the dates, the rounds and the lines
/// change). Above stage 7: 0.
/// Original: `IreLgeCup::create_schedule` (0x637120, ire_lge_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_schedule(League(this), stage, rounds, extra, league, 7, 0x3e9, &IRISH_LEAGUE_CUP_GROUP_ROUNDS, &IRISH_LEAGUE_CUP_KNOCKOUT_ROUNDS, 0x9e9f4c, 0xbc, 0xdb,
        "mwa_25 irish league cup groups", "mwa_25 irish league cup knockout rounds")
}

/// Comparer for `ire_lge_cup_create_schedule` (side by side: `ita_cup_compare_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    ita_cup_compare_schedule("ire_lge_cup_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| ire_lge_cup_create_schedule(this, stage, r, e, l), 7)
}
