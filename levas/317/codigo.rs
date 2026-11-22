// ---------- mwa_23: the Irish League Cup (ire_lge_cup.cpp): the group stages, the managers' records and the predicted positions; the mould of the Irish league cups' groups ----------

/// Builds group `g` of an Irish league cup (IreLgeCup, NirLgeCup) as a League_Stage, in the original routine's order.
///
/// The round table of stage `g` (virtual `comp_vtable::CREATE_SCHEDULE`(g, &rounds, &label, 0): push 0; push &label;
/// push &rounds; push g -> thiscall, ret 0x10); the club list (`exe::crt_malloc` of 4 × `n`; null: the box of `rows.0`
/// and 0) with the first `n` non-null entrants of `ita_cup_fields::ENTRANT_CLUBS`, scanned in order while the index is
/// below `ita_cup_fields::ENTRANT_CLUB_COUNT` (the list stays short when the pool runs out), each zeroed as it leaves; a
/// League_Stage (`exe::crt_operator_new`(0xee); League_Stage::League_Stage 0x68b100, thiscall, ret 0x58: this cup as the
/// parent, `n` clubs, the list, 1, the competition, the round table and its count, the win, draw and category bytes,
/// the tie-breaks (`league::TIEBREAKS`), a local byte list (`places`, 0, 0, 0) where the Italian cups pass
/// `league::PROMOTED_COUNT` (the places that go through [inference]), the year, `g`, the stage label, the importance,
/// 1, 0, the league flags, −1, 0, 2) into stage slot `g` (0 without memory); the list and the round table freed
/// (`exe::crt_free`); no stage: the box of `rows.1` and 0; otherwise the group draw news of the stage
/// (League::send_group_draw_news), `league::CURRENT_STAGE` = `g` and 1.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn irish_league_cup_group_stage(this: League, g: u32, n: u32, places: u8, file: usize, rows: (u32, u32), branch: &'static str) -> u32 {
    let places: [u8; 4] = [places, 0, 0, 0];
    let (mut rounds, mut label) = (0i16, 0i16);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let table = schedule(this.addr(), g, &mut rounds, &mut label, 0);
    let clubs = exe::crt_malloc(4 * n);
    if clubs == 0 {
        show_error_at(file, rows.0);
        return 0;
    }
    covered!(branch);
    let pool = this.addr() + ita_cup_fields::ENTRANT_CLUBS;
    let (mut k, mut i) = (0u32, 0i32);
    while k < n && i < read_i32(this.addr() + ita_cup_fields::ENTRANT_CLUB_COUNT) {
        let e = read_usize(pool) + 4 * i as usize;
        let c = read_usize(e);
        if c != 0 {
            write_usize(clubs + 4 * k as usize, c); // mwa23: the group clubs in pool order
            write_usize(e, 0);
            k += 1;
        }
        i += 1;
    }
    let p = exe::crt_operator_new(0xee);
    let mut s = 0usize;
    if p != 0 {
        type LeagueStageCtor = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize;
        let ctor: LeagueStageCtor = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        // simplification: the original pushes the round count, the stage label, the year and the flags as stack or
        // register dwords with stale high bytes; League_Stage::League_Stage reads the words and bytes
        s = ctor(p, this.addr(), n, clubs, 1, this.competition().addr(), table, rounds as u16 as u32, this.win_points() as u32,
            this.draw_points() as u32, this.match_category() as u32, this.addr() + league::TIEBREAKS, places.as_ptr() as usize,
            this.year() as u16 as u32, g, label as u16 as u32, this.importance() as u32, 1, 0, this.league_flags() as u16 as u32, 0xffff_ffff, 0, 2);
    }
    write_usize(this.stages() + 4 * g as usize, s);
    exe::crt_free(clubs);
    exe::crt_free(table);
    let s = read_usize(this.stages() + 4 * g as usize);
    if s == 0 {
        show_error_at(file, rows.1);
        return 0;
    }
    exe::league_send_group_draw_news(s);
    this.set_current_stage(g as usize);
    1
}

/// Comparer for the Irish league cups' group stages: the entrants of the group planned beforehand (the first `n`
/// non-null pool entries below the count); only the original runs (it builds a stage with its fixtures and sends the
/// news); then the return 1, the stage in its slot with `n` clubs and the cup's year, the current stage, the planned
/// pool entries zeroed and, in the stage's table (rows of `league_table_row::SIZE`), the planned clubs (in any order).
pub(crate) unsafe fn compare_irish_league_cup_group_stage(name: &str, this: League, g: u32, n: u32, original: impl FnOnce() -> u32) -> u32 {
    let pool = read_usize(this.addr() + ita_cup_fields::ENTRANT_CLUBS);
    let mut plan = Vec::new();
    let mut slots = Vec::new();
    let mut i = 0i32;
    while (plan.len() as u32) < n && i < read_i32(this.addr() + ita_cup_fields::ENTRANT_CLUB_COUNT) {
        let c = read_usize(pool + 4 * i as usize);
        if c != 0 {
            plan.push(c);
            slots.push(i as usize);
        }
        i += 1;
    }
    let r = original();
    let f = League(read_usize(this.stages() + 4 * g as usize));
    let mut ok = r == 1 && !f.is_null() && this.current_stage() as u32 == g && f.club_count() as u32 == n && f.year() == this.year()
        && slots.iter().all(|&k| read_usize(pool + 4 * k) == 0);
    if ok && f.table() != 0 {
        let mut a: Vec<usize> = (0..plan.len()).map(|k| read_usize(f.table() + k * league_table_row::SIZE)).collect();
        let mut b = plan.clone();
        a.sort_unstable();
        b.sort_unstable();
        ok = a == b;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}, group {}) returned {}, stage {:#x}, current {}, plan {:x?}", name, this.addr(), g, r, f.addr(), this.current_stage() as i32, plan));
    r
}

/// Rule: the Irish League Cup's groups 1 to 7 (stages 0 to 6; the cup's own table is the first group): three clubs in
/// stages 0 to 4 and four in stages 5 and 6, the next entrants of the pool in order, in a league stage of their own
/// where one place goes through; then the group draw news; out of memory the box of line 0x240, without the stage that
/// of 0x259 (and 0).
///
/// `irish_league_cup_group_stage` (ItaCCup::create_group_stage's frame, read instruction by instruction: the group size
/// is a local, 3 or 4, the pool scan stops at the entrant count and a local place list replaces the cup's).
/// Original: `IreLgeCup::create_group_stage` (0x6379c0, ire_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let n = if (g as i32) >= 5 { 4 } else { 3 }; // mwa23: four clubs from stage 5
    irish_league_cup_group_stage(League(this), g, n, 1, 0x9e9f4c, (0x240, 0x259), "mwa_23 irish league cup group")
}

/// Comparer for `ire_lge_cup_create_group_stage` (original only, the plan).
pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_IRE_LGE_CUP_CREATE_GROUP_STAGE);
    let n = if (g as i32) >= 5 { 4 } else { 3 };
    compare_irish_league_cup_group_stage("ire_lge_cup_create_group_stage", League(this), g, n, || o(this, g))
}

/// Rule: in the seven Irish League Cup groups (stages below 7), a club going through gets the qualification record
/// (round 0x78, weight 40) and one going out the failed-to-qualify record (weight 40); in the knockout (stage 7) a club
/// going through gets the qualification by round and one going out the knocked-out record (weight 50 to 100 over 3
/// rounds), and the winner and the runner-up their final records.
///
/// `welsh_cup_staff_records` with 7 group stages and 3 knockout rounds (read instruction by instruction against
/// WelLgeCup::update_staff_records_for_status: only the group stage count, the knockout slot and the round count
/// change). Returns 0.
/// Original: `IreLgeCup::update_staff_records_for_status` (0x638110, ire_lge_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    covered!("mwa_23 irish league cup staff records");
    welsh_cup_staff_records(League(this), club, outcome, stage, game, a6, 7, 3) // mwa23: three knockout rounds
}

// simplification: the records go to the history (and the reputation and morale): they cannot run twice. Only the
// original; the plan: return 0.
pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_IRE_LGE_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, a6);
    SCOREBOARD.record(r == 0, || format!("ire_lge_cup_update_staff_records_for_status({:#x}) returned {}", this, r));
    r
}

/// Rule: the Irish League Cup's predicted positions (at creation and at the turnover): the 26 entrants by reputation;
/// the knockout (stage 7) gives 1 to the winner and 2 to the runner-up, then a shared place to the losers of each
/// earlier round; the rest in bands of 8; the board and press expectation of each club [inference].
///
/// `italian_cup_predicted_positions` with the knockout stage 7, bands of 8 and the boxes at lines 0x3ee and 0x3f6 of
/// ire_lge_cup.cpp (read instruction by instruction against ItaCCup::set_predicted_positions: only the file, the lines,
/// the stage and the band change).
/// Original: `IreLgeCup::set_predicted_positions` (0x638420, ire_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0x9e9f4c, (0x3ee, 0x3f6), 7, 8, "mwa_23 irish league cup predicted") // mwa23: the Irish League Cup bands of 8
}

/// Comparer for `ire_lge_cup_set_predicted_positions` (`compare_italian_cup_predicted`, side by side).
pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_IRE_LGE_CUP_SET_PREDICTED_POSITIONS);
    compare_italian_cup_predicted("ire_lge_cup_set_predicted_positions", this, ire_lge_cup_set_predicted_positions, || o(this));
}

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_IRE_LGE_CUP_CREATE_KNOCKOUT_STAGE);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_IRE_LGE_CUP_COLLECT_ENTRANTS);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_IRE_LGE_CUP_CREATE_SCHEDULE);
    ita_cup_compare_schedule("ire_lge_cup_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| ire_lge_cup_create_schedule(this, stage, r, e, l), 7)
}

// ---------- mwa_24: the Northern Irish League Cup (nir_lge_cup.cpp): the group stages, the managers' records and the predicted positions ----------

/// Rule: the Northern Irish League Cup's groups 2 to 4 (stages 0 to 2; the cup's own table is the first group): five
/// clubs each, the next entrants of the pool in order, in a league stage of their own where two places go through;
/// then the group draw news; out of memory the box of line 0x241, without the stage that of 0x25a (and 0).
///
/// `irish_league_cup_group_stage` (mwa_23; read instruction by instruction against IreLgeCup::create_group_stage: the
/// group size is fixed at 5 and the local place list holds 2; the pool scan tests the count before the first entry,
/// the same for a group of five).
/// Original: `NirLgeCup::create_group_stage` (0x78d3c0, nir_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_create_group_stage(this: usize, g: u32) -> u32 {
    irish_league_cup_group_stage(League(this), g, 5, 2, 0xa69698, (0x241, 0x25a), "mwa_24 northern irish league cup group") // mwa24: two places through
}

/// Comparer for `nir_lge_cup_create_group_stage` (original only, the plan: `compare_irish_league_cup_group_stage`).
pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_NIR_LGE_CUP_CREATE_GROUP_STAGE);
    compare_irish_league_cup_group_stage("nir_lge_cup_create_group_stage", League(this), g, 5, || o(this, g))
}

/// Rule: in the four Northern Irish League Cup groups (stages below 3), a club going through gets the qualification
/// record (round 0x78, weight 40) and one going out the failed-to-qualify record (weight 40); in the knockout (stage 3)
/// a club going through gets the qualification by round and one going out the knocked-out record (weight 50 to 100
/// over 3 rounds), and the winner and the runner-up their final records.
///
/// `welsh_cup_staff_records` with 3 group stages and 3 knockout rounds (read instruction by instruction against
/// WelLgeCup::update_staff_records_for_status: only the group stage count, the knockout slot and the round count
/// change). Returns 0.
/// Original: `NirLgeCup::update_staff_records_for_status` (0x78d9c0, nir_lge_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    covered!("mwa_24 northern irish league cup staff records");
    welsh_cup_staff_records(League(this), club, outcome, stage, game, a6, 3, 3) // mwa24: the knockout is stage 3
}

// simplification: the records go to the history (and the reputation and morale): they cannot run twice. Only the
// original; the plan: return 0.
pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_NIR_LGE_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, a6);
    SCOREBOARD.record(r == 0, || format!("nir_lge_cup_update_staff_records_for_status({:#x}) returned {}", this, r));
    r
}

/// Rule: the Northern Irish League Cup's predicted positions (at creation and at the turnover): the 20 entrants by
/// reputation; the knockout (stage 3) gives 1 to the winner and 2 to the runner-up, then a shared place to the losers
/// of each earlier round; the rest in bands of 4; the board and press expectation of each club [inference].
///
/// `italian_cup_predicted_positions` with the knockout stage 3, bands of 4 and the boxes at lines 0x3bc and 0x3c4 of
/// nir_lge_cup.cpp (read instruction by instruction against IreLgeCup::set_predicted_positions: the registers are
/// allocated differently; only the file, the lines, the stage and the band change).
/// Original: `NirLgeCup::set_predicted_positions` (0x78dcd0, nir_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0xa69698, (0x3bc, 0x3c4), 3, 4, "mwa_24 northern irish league cup predicted") // mwa24: the Northern Irish League Cup bands of 4
}

/// Comparer for `nir_lge_cup_set_predicted_positions` (`compare_italian_cup_predicted`, side by side).
pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NIR_LGE_CUP_SET_PREDICTED_POSITIONS);
    compare_italian_cup_predicted("nir_lge_cup_set_predicted_positions", this, nir_lge_cup_set_predicted_positions, || o(this));
}

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NIR_LGE_CUP_CREATE_KNOCKOUT_STAGE);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_NIR_LGE_CUP_COLLECT_ENTRANTS);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_NIR_LGE_CUP_CREATE_SCHEDULE);
    ita_cup_compare_schedule("nir_lge_cup_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| nir_lge_cup_create_schedule(this, stage, r, e, l), 3)
}

// ---------- mwa_27: the League of Ireland's schedules (ire_prm.cpp, ire_first.cpp): the Premier and First Divisions' matchdays and the promotion play-off ----------

/// The League of Ireland's opening matchday: Saturday 15 August [inference: weekday 6, code 1] (both divisions share
/// the dates); (day, month from 0, years after the league's year, weekday, code, the match slots).
pub(crate) const IRISH_LEAGUE_OPENING: [FraThirdLeagueRound; 1] = [(15, 7, 0, 6, 1, &[(-1, -1, -1)])]; // mwa27: the opening matchday

/// The extra midweek matchday of a 36-round season: 17 August (weekday 1, code 2).
pub(crate) const IRISH_LEAGUE_EXTRA_AUGUST: [FraThirdLeagueRound; 1] = [(17, 7, 0, 1, 2, &[(-1, -1, -1)])];

/// The weekly matchdays from 22 August to 19 December (weekday 6, code 1).
pub(crate) const IRISH_LEAGUE_AUTUMN: [FraThirdLeagueRound; 17] = [
    (22, 7, 0, 6, 1, &[(-1, -1, -1)]), // 22 aug
    (5, 8, 0, 6, 1, &[(-1, -1, -1)]), // 5 sep
    (12, 8, 0, 6, 1, &[(-1, -1, -1)]), // 12 sep
    (19, 8, 0, 6, 1, &[(-1, -1, -1)]), // 19 sep
    (26, 8, 0, 6, 1, &[(-1, -1, -1)]), // 26 sep
    (3, 9, 0, 6, 1, &[(-1, -1, -1)]), // 3 oct
    (10, 9, 0, 6, 1, &[(-1, -1, -1)]), // 10 oct
    (17, 9, 0, 6, 1, &[(-1, -1, -1)]), // 17 oct
    (24, 9, 0, 6, 1, &[(-1, -1, -1)]), // 24 oct
    (31, 9, 0, 6, 1, &[(-1, -1, -1)]), // 31 oct
    (7, 10, 0, 6, 1, &[(-1, -1, -1)]), // 7 nov
    (14, 10, 0, 6, 1, &[(-1, -1, -1)]), // 14 nov
    (21, 10, 0, 6, 1, &[(-1, -1, -1)]), // 21 nov
    (28, 10, 0, 6, 1, &[(-1, -1, -1)]), // 28 nov
    (5, 11, 0, 6, 1, &[(-1, -1, -1)]), // 5 dec
    (12, 11, 0, 6, 1, &[(-1, -1, -1)]), // 12 dec
    (19, 11, 0, 6, 1, &[(-1, -1, -1)]), // 19 dec
];

/// The Christmas matchday when 25 or 24 December is the weekday-6 day the 26 December round would move to: 22 December
/// midweek (weekday 2, code 2).
pub(crate) const IRISH_LEAGUE_CHRISTMAS_MIDWEEK: [FraThirdLeagueRound; 1] = [(22, 11, 0, 2, 2, &[(-1, -1, -1)])];

/// The Christmas matchday otherwise: 26 December (weekday 6, code 1).
pub(crate) const IRISH_LEAGUE_CHRISTMAS: [FraThirdLeagueRound; 1] = [(26, 11, 0, 6, 1, &[(-1, -1, -1)])];

/// The January matchdays of the next year: 3, 16, 23 and 30 (weekday 6, code 1).
pub(crate) const IRISH_LEAGUE_JANUARY: [FraThirdLeagueRound; 4] = [
    (3, 0, 1, 6, 1, &[(-1, -1, -1)]),
    (16, 0, 1, 6, 1, &[(-1, -1, -1)]),
    (23, 0, 1, 6, 1, &[(-1, -1, -1)]),
    (30, 0, 1, 6, 1, &[(-1, -1, -1)]),
];

/// The extra midweek matchday of a 36-round season in the next year: 8 February (weekday 1, code 2).
pub(crate) const IRISH_LEAGUE_EXTRA_FEBRUARY: [FraThirdLeagueRound; 1] = [(8, 1, 1, 1, 2, &[(-1, -1, -1)])];

/// The last eleven matchdays, from 13 February to 23 April of the next year (weekday 6, code 1; 7 March midweek).
pub(crate) const IRISH_LEAGUE_SPRING: [FraThirdLeagueRound; 11] = [
    (13, 1, 1, 6, 1, &[(-1, -1, -1)]), // 13 feb (+1)
    (20, 1, 1, 6, 1, &[(-1, -1, -1)]), // 20 feb (+1)
    (27, 1, 1, 6, 1, &[(-1, -1, -1)]), // 27 feb (+1)
    (7, 2, 1, 1, 2, &[(-1, -1, -1)]), // 7 mar (+1)
    (12, 2, 1, 6, 1, &[(-1, -1, -1)]), // 12 mar (+1)
    (19, 2, 1, 6, 1, &[(-1, -1, -1)]), // 19 mar (+1)
    (26, 2, 1, 6, 1, &[(-1, -1, -1)]), // 26 mar (+1)
    (2, 3, 1, 6, 1, &[(-1, -1, -1)]), // 2 apr (+1)
    (9, 3, 1, 6, 1, &[(-1, -1, -1)]), // 9 apr (+1)
    (16, 3, 1, 6, 1, &[(-1, -1, -1)]), // 16 apr (+1)
    (23, 3, 1, 6, 1, &[(-1, -1, -1)]), // 23 apr (+1)
];

/// The league schedule of the League of Ireland's divisions (the virtual `comp_vtable::CREATE_SCHEDULE` at stage −1;
/// IrePrm and IreFirst write the same rounds, read instruction by instruction with the emulator of the 36 and 34 round
/// paths and both Christmas cases).
///
/// n = 36 rounds unless `league::LEGS` is 3 (then 34); `out` (when given) = 1, `rounds` = n, `extra` = 0; the table
/// (`exe::crt_malloc` of n × 0x41; out of memory, the box of `rows.0` and 0). The rounds, each with match slot 0 empty
/// (`fra_third_write_league_rounds`): `IRISH_LEAGUE_OPENING`; with 36, `IRISH_LEAGUE_EXTRA_AUGUST`;
/// `IRISH_LEAGUE_AUTUMN`; the Christmas round: Date(25 December of the league's year, no weekday) against Date(26
/// December moved to weekday 6) (Date::Date `exe::date_date_549f70`, thiscall, ret 0x10; the two words compared), and
/// if they differ the same with 24 December: equal in either, `IRISH_LEAGUE_CHRISTMAS_MIDWEEK`, otherwise
/// `IRISH_LEAGUE_CHRISTMAS`; `IRISH_LEAGUE_JANUARY`; with 36, `IRISH_LEAGUE_EXTRA_FEBRUARY`; `IRISH_LEAGUE_SPRING`.
/// When the rounds written are not n (never: the original's own check), the box of `rows.1`. Returns the table.
pub(crate) unsafe fn irish_league_schedule(this: League, rounds: *mut i16, extra: *mut i16, out: *mut i32, file: usize, rows: (u32, u32), branch: &'static str) -> usize {
    let n: i16 = if this.legs() != 3 { 0x24 } else { 0x22 };
    if !out.is_null() {
        *out = 1;
    }
    *rounds = n;
    *extra = 0;
    let t = exe::crt_malloc((n as i32 * 0x41) as u32);
    if t == 0 {
        show_error_at(file, rows.0);
        return 0;
    }
    covered!(branch);
    let year = this.year() as u16 as u32;
    let mut r = 0i32;
    let mut put = |table: &[FraThirdLeagueRound]| {
        fra_third_write_league_rounds(t, year, r, table);
        r += table.len() as i32;
    };
    put(&IRISH_LEAGUE_OPENING);
    if n == 0x24 {
        put(&IRISH_LEAGUE_EXTRA_AUGUST);
    }
    put(&IRISH_LEAGUE_AUTUMN);
    let same = |day: u32| {
        let (mut a, mut b) = ([0u16; 4], [0u16; 4]);
        exe::date_date_549f70(&mut a, day, 0xb, year, 0xffff_ffff);
        exe::date_date_549f70(&mut b, 0x1a, 0xb, year, 6);
        a[1] == b[1] && a[0] == b[0]
    };
    let midweek = same(0x19) || same(0x18); // mwa27: the Christmas case
    put(if midweek { &IRISH_LEAGUE_CHRISTMAS_MIDWEEK } else { &IRISH_LEAGUE_CHRISTMAS });
    put(&IRISH_LEAGUE_JANUARY);
    if n == 0x24 {
        put(&IRISH_LEAGUE_EXTRA_FEBRUARY);
    }
    put(&IRISH_LEAGUE_SPRING);
    if r as i16 != n {
        show_error_at(file, rows.1);
    }
    t
}

/// Rule: the League of Ireland Premier Division plays 36 matchdays (34 when the clubs meet three times), from 15
/// August to 23 April of the next year, almost all on Saturdays [inference: weekday 6], with two midweek rounds in a
/// 36-round season (17 August and 8 February) and one on 7 March; the Christmas round is on 26 December, or midweek on
/// 22 December when Christmas or Christmas Eve is the Saturday; only the league stage.
///
/// Only stage −1 (the byte; otherwise 0 with nothing written): `irish_league_schedule` with the boxes at lines 0xad and
/// 0x156 of ire_prm.cpp.
/// Original: `IrePrm::create_schedule` (0x639e90, ire_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ire_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    irish_league_schedule(League(this), rounds, extra, out, 0x9ea080, (0xad, 0x156), "mwa_27 irish premier schedule")
}

/// Comparer for `ire_prm_create_schedule` (side by side: `compare_c1_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_ire_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_IRE_PRM_CREATE_SCHEDULE);
    compare_c1_schedule("ire_prm_create_schedule", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, l| ire_prm_create_schedule(this, stage, r, e, l))
}

/// The Irish promotion play-off: one round of two clubs over two legs two days apart [the gap byte the original
/// writes], drawn on 24 April and first played on 27 April of the next year (weekday 3, code 1); code 0xa0, tie rule 4
/// and two-leg rule 7; category 5; no prize.
pub(crate) const IRISH_PLAY_OFF: EngCcCupCupRound = cql10r((0x18, 3, 1, 0), (0x1b, 3, 1, 3, 1, 0), 0xa0, (0, 4, 7), 5, (2, 1, 2, 0), (0, 2, 2), (0, 0, 0)); // mwa27: the play-off on 27 April

/// Rule: the League of Ireland First Division plays on the Premier Division's dates (36 matchdays, or 34 when the
/// clubs meet three times); then the promotion play-off against the Premier Division's play-off club, two legs from 27
/// April.
///
/// Stage −1: `irish_league_schedule` with the boxes at lines 0xb5 and 0x15e of ire_first.cpp. Stage 0: `out` (when
/// given) = 0, `rounds` = 1, `extra` = 0, the table (`exe::crt_malloc` of 0x68; out of memory, the box at line 0x171 and
/// 0) with `IRISH_PLAY_OFF` (`eng_cc_cup_write_cup_round`; the original writes the same fields in another order).
/// Other stages: 0.
/// Original: `IreFirst::create_schedule` (0x634f40, ire_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ire_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let this = League(this);
    match stage as u8 {
        0xff => irish_league_schedule(this, rounds, extra, out, 0x9e9ed0, (0xb5, 0x15e), "mwa_27 irish first schedule"),
        0 => {
            if !out.is_null() {
                *out = 0;
            }
            *rounds = 1;
            *extra = 0;
            let t = exe::crt_malloc(0x68);
            if t == 0 {
                show_error_at(0x9e9ed0, 0x171);
                return 0;
            }
            covered!("mwa_27 irish play-off round");
            eng_cc_cup_write_cup_round(t, 0, &IRISH_PLAY_OFF, this.year() as i32 as u32);
            t
        }
        _ => 0,
    }
}

/// Comparer for `ire_first_create_schedule` (side by side: `compare_c1_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_ire_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_IRE_FIRST_CREATE_SCHEDULE);
    compare_c1_schedule("ire_first_create_schedule", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, l| ire_first_create_schedule(this, stage, r, e, l))
}
