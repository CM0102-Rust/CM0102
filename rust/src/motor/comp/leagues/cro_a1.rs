use super::*;

/// Rule: the Croatian First Division at career creation: discipline rule 0x21 and, with no saved state, no tie-break
/// pairs, the main stage only, the rules, the table, the fixtures, the order, the statistics and the predicted
/// positions (`construct_tiebreak_league`).
/// Original: `CroA1::CroA1` (0x513e10, cro_a1.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_cro_a1(this: usize, year: u32, record: usize) -> usize {
    construct_tiebreak_league(League(this), year, record, &CROATIAN_FIRST_DIVISION, "mwd_1 croatian first division new")
}

/// Comparer for `construct_cro_a1`.
pub(crate) unsafe extern "thiscall" fn compare_construct_cro_a1(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_CRO_A1);
    compare_tiebreak_league_ctor("construct_cro_a1", League(this), year, record, &CROATIAN_FIRST_DIVISION, || o(this, year, record))
}

/// Rule: the Croatian First Division: 16 clubs, 2 legs, 3 points a win and 1 a draw; tie-breaks head to head, goal
/// difference, goals for; points ties across a band go to tie-break matches; nobody goes up, 2 go down to the Second
/// Division (north group); 7 substitutes, 3 substitutions; rule bit 0x40 (`croatian_league_rules`).
/// Original: `CroA1::set_up_rules` (0x5148e0, cro_a1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a1_set_up_rules(this: usize) -> u32 {
    covered!("mwd_2 croatian first division rules");
    apply_league_rules_with_flags(League(this), 0x40, &cro_a1_rules())
}

/// Comparer for `cro_a1_set_up_rules` (the rehearsal of `eng_prm_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_A1_SET_UP_RULES);
    eng_prm_compare_league_rules("cro_a1_set_up_rules", League(this), cro_a1_rules(), || o(this), || cro_a1_set_up_rules(this))
}

/// Rule: the Croatian First Division has 16 clubs, those whose national division is it, in database order; when full,
/// the prize fund (30,000 a share) and TV pays 100,000 to each club.
///
/// `club_count` = 16; the table (`exe::crt_malloc` of 16 × 0x3b; out of memory, the box at line 0x145 and 0); the
/// clubs of the competition of id `KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4` until full
/// (`eng_prm_fill_league_table`); full: the prize fund (League::calc_prize_fund, push 30000 -> thiscall, ret 4),
/// `prize_mode` = 1 and the TV money (League::pay_tv_rights `exe::league_pay_tv_rights`: push 0; push 100000 ->
/// thiscall, ret 8) and 1; otherwise 0.
/// Original: `CroA1::create_table` (0x5147b0, cro_a1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a1_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x10, 0x9b53c0, 0x145, KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4, 0x7530, || {
        covered!("mwd_3 croatian first division tv money");
        eng_prm_pay_tv_rights(this, 0x186a0, 0) // mwd3: the First Division TV money
    })
}

/// Comparer for `cro_a1_create_table` (only the original runs; the planned clubs and the prize fund checked).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_A1_CREATE_TABLE);
    eng_prm_compare_league_table("cro_a1_create_table", League(this), 0x10, KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4, 0x7530, || o(this))
}

/// Rule: the Croatian First Division schedule: 30 rounds from late July to early May (`CROATIAN_LEAGUE_ROUNDS`); stage
/// 0 is the one-round tie-break play-off in May (`croatian_tiebreak_round`).
/// Original: `CroA1::create_schedule` (0x5140e0, cro_a1.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn cro_a1_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    croatian_league_schedule(League(this), stage, rounds, extra, league, 0x9b53c0, "mwd_4 croatian first division calendar")
}

/// Comparer for `cro_a1_create_schedule` (side by side: the outputs and the written table).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_CRO_A1_CREATE_SCHEDULE);
    compare_league_round_table("cro_a1_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| cro_a1_create_schedule(this, stage, r, e, l))
}

/// Rule: the Croatian turnover, led by the First Division: the season is reopened (+0x4c = 0); promotion and relegation
/// along the chain First Division -> north group -> south group (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`:
/// push 1 -> thiscall, ret 4); the four best third-tier clubs up into the second division and its relegated clubs down
/// (`cro_a1_swap_second_and_third_tier_clubs`, ecx = this); the 32 second division clubs regrouped north and south by
/// the latitude of their city (0x515190, cdecl); the First Division rebuilt (`rebuild_croatian_league`, lines 0x1c0,
/// 0x1c7, 0x1cd, 0x1d4); the division tier of the clubs of both second division groups capped (League::limit_club_division_tier
/// `exe::league_limit_club_division_tier`(group, 1, 3, −3, 1): 4 pushes -> thiscall, ret 0x10); both groups turned over
/// (virtual `comp_vtable::START_NEW_SEASON`); then the predicted positions, the board expectations and the officials
/// (`finish_croatian_turnover`).
/// Original: `CroA1::start_new_season` (0x5149a0, cro_a1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a1_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this.addr(), 1);
    exe::cro_a1_swap_second_and_third_tier_clubs(this); // mwd5: third tier up, second tier down
    exe::cro_a1_realign_second_division_groups();
    covered!("mwd_5 croatian first division turnover");
    if !rebuild_croatian_league(this, 0x5147b0, 0x9b53c0, [0x1c0, 0x1c7, 0x1cd, 0x1d4], true) {
        return 0;
    }
    let (north, south) = (comp_by_global(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH), comp_by_global(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH));
    exe::league_limit_club_division_tier(north, 1, 3, -3, 1);
    exe::league_limit_club_division_tier(south, 1, 3, -3, 1);
    bra_nat_first_call_virtual_no_args(north, comp_vtable::START_NEW_SEASON); // mwd5: the north group turnover
    bra_nat_first_call_virtual_no_args(south, comp_vtable::START_NEW_SEASON);
    finish_croatian_turnover(this)
}

/// Comparer for `cro_a1_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CRO_A1_START_NEW_SEASON);
    compare_state_league_new_season("cro_a1_start_new_season", League(this), || o(this))
}

/// Rule: the Croatian First Division's next stage: at the last stage (+0x30 ≥ +0x2c − 1, signed) the error box (line
/// 0x1fa) and nothing else; otherwise the current stage goes up by one and, on reaching 0 (the league is over), the
/// tie-break play-off is built (`cro_a1_create_playoff_stage`, direct call with ecx = this).
/// Original: `CroA1::advance_stage` (0x514bd0, cro_a1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a1_advance_stage(this: usize) {
    covered!("mwd_6 croatian first division next stage");
    state_league_advance_stage(League(this), 0x9b53c0, 0x1fa, 0x514c60);
}

/// Rule: at the Croatian turnover, the four best clubs of the divisions below the second (the lower division and the
/// four third division groups) go up into the Second Division (north group: the regrouping that follows splits them),
/// and the clubs relegated from both second division groups go down to the lower division.
///
/// A list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; out of memory: the box at line 0x2d2 and return) with the
/// Croatian clubs (`club::NATION` id = [`KEY_NATION_CROATIA`]) of those divisions (`is_croatian_third_tier_club`), in
/// database order (a 16-bit index). With at least 4: the list sorted by strength and the best shuffled
/// (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 4): 3 pushes; add esp, 0xc -> cdecl) and its first four moved
/// to the north group (League::move_club_to_division `exe::league_move_club_to_division`(this, club, the north group's
/// record, 1): thiscall, ret 0xc); then, for each row i below the north group's club count, the relegated rows of the
/// north group and then of the south group (that loop also bounded by the north group's count, as the original) go down
/// (`relegate_croatian_row`). The list is freed in every case.
/// Original: `cro_a1_swap_second_and_third_tier_clubs` (0x514ed0, cro_a1.cpp, thiscall: ecx = the First Division, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a1_swap_second_and_third_tier_clubs(this: usize) {
    let north = comp_by_global(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH);
    let south = comp_by_global(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH);
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9b53c0, 0x2d2);
        return;
    }
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let nation = read_usize(club + club::NATION);
        if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_CROATIA) && is_croatian_third_tier_club(club) {
            write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
            n = n.wrapping_add(1);
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if (n as u16 as i16) >= 4 {
        covered!("mwd_6 croatian third tier up");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4); // mwd6: the best four, shuffled
        for k in 0..4 {
            exe::league_move_club_to_division(this, read_usize(list + 4 * k), comp_record_of(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH), 1); // mwd6: up to the north group
        }
        let count = League(north).club_count() as i32;
        for i in 0..count {
            relegate_croatian_row(this, north, i);
            relegate_croatian_row(this, south, i);
        }
    }
    exe::crt_free(list);
}

/// Comparer for `cro_a1_advance_stage` (side by side on the current stage, only the original when the play-off would
/// be built: `ita_ser_c1a_compare_advance_stage`).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CRO_A1_ADVANCE_STAGE);
    ita_ser_c1a_compare_advance_stage("cro_a1_advance_stage", League(this), || o(this), || cro_a1_advance_stage(this))
}

/// Comparer for `cro_a1_swap_second_and_third_tier_clubs`: only the original runs (it moves clubs and draws); with at
/// least 4 candidates, the clubs of both second division groups whose final status is 3 must end in the lower
/// division (the south group's rows only below the north group's club count, as the original).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_swap_second_and_third_tier_clubs(this: usize) {
    let lower = comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION);
    let candidates = (0..read_usize(G_CLUB_COUNT)).filter(|&i| {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let nation = read_usize(c + club::NATION);
        nation != 0 && read_usize(nation) == read_usize(KEY_NATION_CROATIA) && is_croatian_third_tier_club(c)
    }).count();
    let mut planned: Vec<usize> = Vec::new();
    for g in [KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH] {
        let l = League(comp_by_global(g));
        if l.table() != 0 && candidates >= 4 {
            for i in 0..l.club_count() as i32 {
                let row = l.table() + i as usize * league_table_row::SIZE;
                if read_u8(row + league_table_row::STATUS) == 3 {
                    planned.push(read_usize(row));
                }
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CRO_A1_SWAP_SECOND_AND_THIRD_TIER_CLUBS)(this);
    let wrong: Vec<usize> = planned.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != lower).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("cro_a1_swap_second_and_third_tier_clubs({:#x}): relegated clubs not in the lower division {:x?}", this, wrong));
}

/// Rule: at the end of the Croatian First Division, the clubs level on points with the club right below them (neither
/// relegated) play a one-round tie-break play-off (stage 0, in May): each pair of level rows goes into the play-off and
/// the positions they can still reach are narrowed; with no level pair, the season ends without tie-breaks.
/// Original: `CroA1::create_playoff_stage` (0x514c60, cro_a1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a1_create_playoff_stage(this: usize) {
    create_croatian_tiebreak_playoff(League(this), 0x9b53c0, 0x24b);
}

/// Comparer for `cro_a1_create_playoff_stage`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_create_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CRO_A1_CREATE_PLAYOFF_STAGE);
    compare_croatian_tiebreak_playoff("cro_a1_create_playoff_stage", League(this), || o(this))
}

/// The body of the league constructors that settle points ties with play-off matches (thiscall(this, year, record),
/// ret 8; the SEH frame only covers the operator new of the statistics).
///
/// League::League (`exe::new_league`, thiscall, ret 0); the competition record (+4); the vtable; the year (+0x40, word)
/// and the discipline rule (+0x50); the saved state (League::load_from_temp_file `exe::league_load_from_temp_file`:
/// push 1 -> thiscall, ret 4): loaded, returns this. Otherwise the new league: `TIEBREAK_PAIRS` = 0, `season_closed`
/// (+0x4c) = 1, the main stage (+0x44 = 0xff), current stage −1 (+0x30), no extra stages (+0x2c = 0, +0xc = 0); then,
/// each with the error box of its line (`p.rows`) and returning this when it returns 0: the rules (`p.rules`, direct
/// call, thiscall, ret 0), the table (`p.table`), the fixtures (League::create_fixtures `exe::league_create_fixtures`),
/// the order (League::sort_table `exe::league_sort_table`: push 0 -> thiscall, ret 4) and the statistics (operator new
/// `exe::crt_operator_new` of 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats `exe::new_comp_stats`: push
/// league -> thiscall, ret 4; 0 when out of memory) into +8; last, the predicted positions (League::set_predicted_positions
/// `exe::league_set_predicted_positions`, thiscall, ret 0). Returns this.
pub(crate) unsafe fn construct_tiebreak_league(this: League, year: u32, record: usize, p: &LeagueDivision, branch: &'static str) -> usize {
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.rules);
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.table);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), p.vtabela);
    this.set_year((year as u16) as i16);
    this.set_discipline_rule(p.discipline); // mwd1: the discipline rule
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    write_u8(this.addr() + TIEBREAK_PAIRS, 0);
    this.set_season_closed(1);
    this.set_stage_index(stage_index::MAIN);
    this.set_current_stage(0xffff_ffff);
    this.set_stage_count(0);
    this.set_stages(0);
    if rules(this.addr()) == 0 {
        show_error_at(p.file, p.rows[0]);
        return this.addr();
    }
    if table(this.addr()) == 0 {
        show_error_at(p.file, p.rows[1]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(p.file, p.rows[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(p.file, p.rows[3]);
        return this.addr();
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(p.file, p.rows[4]);
        return this.addr();
    }
    exe::league_set_predicted_positions(this);
    this.addr()
}

/// Sets the bits `flags` in the rule bits (`league::LEAGUE_FLAGS`, byte +0xd9: `or byte ptr [esi + 0xd9], flags`), then
/// writes the (offset, size, value) rules and builds the main stage round table (`eng_prm_apply_league_rules`: virtual
/// +0x3c(−1, &+0xa9, &+0x3a, 0) into +0xba, skipped in the comparer's rehearsal). Returns 1.
pub(crate) unsafe fn apply_league_rules_with_flags(this: League, flags: u8, written: &[(usize, u8, u32)]) -> u32 {
    // The original only ORs the bit in and never clears the field, and it does so before the virtual schedule call
    // (bit 0x40 is the round order): the list carries +0xd9 for the comparer, so it must not be written as a field.
    let rest: Vec<(usize, u8, u32)> = written.iter().copied().filter(|&(d, _, _)| d != league::LEAGUE_FLAGS).collect();
    write_u8(this.addr() + league::LEAGUE_FLAGS, (this.league_flags() as u8) | flags);
    eng_prm_apply_league_rules(this, &rest)
}

/// The rules of the three Croatian leagues (16 clubs each), given the ids of the league above (+0x1c) and below
/// (+0x20).
///
/// 2 legs (+0x3c), 3 points a win (+0xc2) and 1 a draw (+0xc3), league matches (+0xc4 = 2), +0xab = 0 and +0xdb = 0
/// (words), national clubs (+0x42 = 1), tie-breaks 4, 1, 2 (+0xc5..+0xc7: head to head, goal difference, goals for;
/// +0xc8 = 0 ends), nobody goes up directly nor to the play-offs (+0xbe, +0xbf, +0xc0 = 0), 2 go down (+0xc1), +0x52 =
/// 2, 3 substitutions (+0x4a) of 7 substitutes (+0x49) and points ties settled by tie-break matches (+0xe6 = 1); the
/// rule bit 0x40 is set separately (`apply_league_rules_with_flags`). The list also carries +0xd9 for the comparer.
pub(crate) unsafe fn croatian_league_rules(above: u32, below: u32) -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 4), (0xc6, 1, 1), // mwd2: the Croatian points
        (0xc7, 1, 2), (0xc8, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 2), (0x1c, 4, above), (0x52, 1, 2), (0x4a, 1, 3),
        (0x20, 4, below), (0x49, 1, 7), (0xe6, 4, 1), (league::LEAGUE_FLAGS, 1, 0)]
}

/// The rules of the Croatian First Division: nothing above, the Second Division north group
/// (`KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH`) below.
pub(crate) unsafe fn cro_a1_rules() -> Vec<(usize, u8, u32)> {
    croatian_league_rules(0xffff_ffff, read_usize(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH) as u32)
}

/// The stage 0 round of the Croatian tie-break play-off (one round of single matches), its team counts from the
/// tie-break pair count: drawn on 7 May of the next year, played on 13 May (weekday 5, leg 1), code 0xa0, no draw news
/// (+0xd = 0x200), two-leg tie-break 3, category 5, `2 × pairs` teams, `pairs` ties and `2 × pairs` new teams (the
/// signed byte widened to a word), +0x20..+0x22 = 0, 2, 4, no prizes.
pub(crate) unsafe fn croatian_tiebreak_round(pairs: u8) -> FraSuperCupRound {
    let p = pairs as i8 as i16 as u16;
    cql12r((7, 4, 1, 0), (0xd, 4, 1, 5, 1, 0), 0xa0, (0, 0x200, 3), 5, (p.wrapping_shl(1), p, p.wrapping_shl(1), 0), (0, 2, 4), (0, 0, 0))
}

/// The body of the three Croatian `create_schedule` (thiscall(this, stage, &rounds, &extra, &league), ret 0x10).
///
/// Stage −1 (the byte 0xff): the league output (if given) = 1, 30 rounds, extra 0; the table (`exe::crt_malloc` of 30 ×
/// 0x41; out of memory: the box at line 0xaa and 0) with `CROATIAN_LEAGUE_ROUNDS` (League::set_round_date and
/// set_round_match_slot, cdecl, through `spa_second_set_league_rounds`; the year pushed as the word +0x40). Stage 0: the
/// league output = 0, 1 round, extra 0; the round (`exe::crt_malloc` of 0x68; out of memory: the box at line 0x118 and
/// 0) with `croatian_tiebreak_round` of the pair count `TIEBREAK_PAIRS` (Cup::set_round_draw_date 0x521e60 and
/// set_round_match_date 0x521eb0, cdecl, the year sign-extended; `fra_super_write_cup_round`). Other stages: 0.
pub(crate) unsafe fn croatian_league_schedule(this: League, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32, file: usize, branch: &'static str) -> usize {
    match stage as u8 {
        0xff => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 30;
            *extra = 0;
            let t = exe::crt_malloc(0x79e);
            if t == 0 {
                show_error_at(file, 0xaa);
                return 0;
            }
            covered!(branch);
            spa_second_set_league_rounds(t, (this.year() as u16) as u32, 0, &CROATIAN_LEAGUE_ROUNDS);
            t
        }
        0 => {
            if !league.is_null() {
                *league = 0;
            }
            *rounds = 1;
            *extra = 0;
            let t = exe::crt_malloc(0x68);
            if t == 0 {
                show_error_at(file, 0x118);
                return 0;
            }
            covered!("mwd_4 croatian tie-break round");
            fra_super_write_cup_round(t, 0, &croatian_tiebreak_round(read_u8(this.addr() + TIEBREAK_PAIRS)), this.year() as i32 as u32);
            t
        }
        _ => 0,
    }
}

/// The rebuild common to the three Croatian turnovers (after the First Division's promotion and relegation), in the
/// original's order; returns false after an error box.
///
/// League::free_table (`exe::league_free_table`: push 0 -> thiscall, ret 4); the round dates (+0xba) freed
/// (`exe::crt_free`, cdecl) and cleared; the statistics archived (Comp_Stats::archive_season_and_reset
/// `exe::comp_stats_archive_season_and_reset`: push 1 -> thiscall, ret 4) when there are any; the stage vector (+0xc)
/// freed when there is one (the stage objects are not destroyed); the year (+0x40) + 1; +0xc = 0, +0x2c = 0, current
/// stage −1, `season_closed` = 1 when `reopen` (the Croatian and Russian Premier turnovers; not the Greek ones); then, each with the box of its line (`rows`) and stopping when it returns 0: the
/// rules (virtual `comp_vtable::SET_UP_RULES`), the table (`table`, direct call, thiscall, ret 0), the fixtures
/// (League::create_fixtures) and the order (League::sort_table: push 0).
pub(crate) unsafe fn rebuild_croatian_league(this: League, table: usize, file: usize, rows: [u32; 4], reopen: bool) -> bool {
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    if this.stages() != 0 {
        exe::crt_free(this.stages());
    }
    this.set_year(this.year().wrapping_add(1)); // mwd5: the next year
    this.set_stages(0);
    this.set_stage_count(0);
    this.set_current_stage(0xffff_ffff);
    if reopen {
        this.set_season_closed(1);
    }
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, rows[0]);
        return false;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if table(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return false;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return false;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, rows[3]);
        return false;
    }
    true
}

/// The end common to the Croatian turnovers: the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`),
/// the board expectations of each club (League::set_club_season_expectations, thiscall, ret 0) and the renewal of the
/// officials (OfficialsManager::renew_officials(+4): push; mov ecx, [0xb74340] -> thiscall, ret 4), whose eax is
/// returned.
pub(crate) unsafe fn finish_croatian_turnover(this: League) -> u32 {
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Is the club in one of the Croatian divisions below the second (the lower division `KEY_CLUB_COMP_A_LOWER_DIVISION`
/// or the four third division groups: north, east, south, west, compared in that order)?
pub(crate) unsafe fn is_croatian_third_tier_club(club: usize) -> bool {
    let club = Club(club);
    let d = club.division().addr();
    if d == 0 {
        return false;
    }
    let id = read_usize(d);
    [KEY_CLUB_COMP_A_LOWER_DIVISION, KEY_CLUB_COMP_CROATIAN_THIRD_DIVISION_NORTH, KEY_CLUB_COMP_CROATIAN_THIRD_DIVISION_EAST,
        KEY_CLUB_COMP_CROATIAN_THIRD_DIVISION_SOUTH, KEY_CLUB_COMP_CROATIAN_THIRD_DIVISION_WEST].iter().any(|&g| id == read_usize(g))
}

/// Relegates the club of row `i` of a second division group to the lower division (`KEY_CLUB_COMP_A_LOWER_DIVISION`)
/// when its final status is 3 (relegated): League::get_table_row (`exe::league_get_table_row`: push i -> thiscall, ret
/// 4; called again for the club, as the original) and League::relegate_club_to_division (0x6831a0: push 1; push
/// division; push club; mov ecx, this -> thiscall, ret 0xc).
pub(crate) unsafe fn relegate_croatian_row(this: usize, group: usize, i: i32) {
    if read_u8(exe::league_get_table_row(group, i) + league_table_row::STATUS) == 3 {
        let club = read_usize(exe::league_get_table_row(group, i));
        exe::league_relegate_club_to_division(this, club, comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION), 1); // mwd6: down to the lower division
    }
}

/// The body of the two Croatian tie-break play-offs (`create_playoff_stage`, thiscall, ret 0).
///
/// The table scan (16-bit row index i, while i < `club_count`): when i is not the last row and rows i and i + 1 are
/// level on points with neither of status 3, both clubs go into the list (a local array), row i's worst possible
/// position (`league_table_row::WORST_POSSIBLE_POSITION`) becomes i + 2 and row i + 1's best (`BEST_POSSIBLE_POSITION`)
/// i + 1, both statuses 1, and the scan skips row i + 1; otherwise, when i > 0 and row i is level on points with row i
/// − 1, its worst and best possible positions become i + 1. The pair count (the list count / 2, truncated) goes into
/// `TIEBREAK_PAIRS`.
///
/// No pair: no extra stage (+0x2c = 0, current stage −1), no tie-break matches (`league::TIEBREAK_MATCHES` = 0) and the
/// positions and statuses recomputed (virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS`, thiscall, ret 0).
/// Otherwise: the stage 0 round (virtual `comp_vtable::CREATE_SCHEDULE`: push 0; push &extra; push &rounds; push 0 ->
/// thiscall, ret 0x10; `croatian_tiebreak_round`), the stage (operator new `exe::crt_operator_new` of 0xb2 and
/// Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`(league, count, list, rounds, competition, round, year, 0, 1, extra, 0x14,
/// 0, 0, 0, 0): 15 pushes -> thiscall, ret 0x3c; the rounds, extra and year pushed as dwords with stack garbage above
/// the words, which the constructor does not read) into the first slot of the stage vector (+0xc); no stage: the box
/// at line `row` and return; otherwise the round freed (`exe::crt_free`), the first round set up (Cup::set_up_round
/// `exe::cup_set_up_round`: push 0 -> thiscall, ret 4), current stage 0 and the first tie-break code 6
/// (`league::TIEBREAKS`).
pub(crate) unsafe fn create_croatian_tiebreak_playoff(this: League, file: usize, row: u32) {
    let mut list = [0usize; 0x40];
    let mut count: u32 = 0;
    let n = this.club_count();
    let mut i: i16 = 0;
    while i < n {
        let r = this.table() + (i as i32 as usize).wrapping_mul(league_table_row::SIZE);
        let next = r + league_table_row::SIZE;
        if (i as i32) < n as i32 - 1 && read_u16(r + league_table_row::POINTS) == read_u16(next + league_table_row::POINTS)
            && read_u8(r + league_table_row::STATUS) != 3 && read_u8(next + league_table_row::STATUS) != 3 {
            list[(count as u16 as i16 as usize) & 0x3f] = read_usize(r);
            list[((count + 1) as u16 as i16 as usize) & 0x3f] = read_usize(next);
            count += 2;
            write_u8(r + league_table_row::WORST_POSSIBLE_POSITION, (i as u8).wrapping_add(2));
            write_u8(next + league_table_row::BEST_POSSIBLE_POSITION, (i as u8).wrapping_add(1));
            write_u8(r + league_table_row::STATUS, 1);
            write_u8(next + league_table_row::STATUS, 1);
            i += 1;
        } else if i > 0 && read_u16(r + league_table_row::POINTS) == read_u16(r - league_table_row::SIZE + league_table_row::POINTS) {
            write_u8(r + league_table_row::WORST_POSSIBLE_POSITION, (i as u8).wrapping_add(1));
            write_u8(r + league_table_row::BEST_POSSIBLE_POSITION, (i as u8).wrapping_add(1));
        }
        i += 1;
    }
    let pairs = ((count as u16 as i16 as i32) / 2) as u8;
    write_u8(this.addr() + TIEBREAK_PAIRS, pairs);
    if pairs == 0 { // mwd7: no level clubs
        this.set_stage_count(0);
        this.set_current_stage(0xffff_ffff);
        this.set_tiebreak_matches(0);
        bra_nat_first_call_virtual_no_args(this.addr(), league_vtable::UPDATE_POSITIONS_AND_STATUS);
        return;
    }
    covered!("mwd_7 croatian tie-break play-off");
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let t = schedule(this.addr(), 0, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), count, list.as_ptr(), rounds as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages(), stage);
    if read_usize(this.stages()) == 0 {
        show_error_at(file, row);
        return;
    }
    exe::crt_free(t);
    exe::cup_set_up_round(read_usize(this.stages()), 0);
    this.set_current_stage(0);
    write_u8(this.addr() + league::TIEBREAKS, 6);
}

/// Comparer for the Croatian tie-break play-offs: only the original runs (it builds a stage); the pair count must be
/// half the planned clubs (`croatian_tiebreak_plan`) and, with pairs, the stage's team list must hold the planned clubs
/// in order.
pub(crate) unsafe fn compare_croatian_tiebreak_playoff(name: &str, this: League, original: impl FnOnce()) {
    let plan = if this.table() != 0 { croatian_tiebreak_plan(this) } else { Vec::new() };
    original();
    let pairs = read_u8(this.addr() + TIEBREAK_PAIRS);
    let mut ok = pairs as usize == plan.len() / 2;
    let mut found: Vec<usize> = Vec::new();
    if ok && pairs != 0 {
        let stage = if this.stages() != 0 { Cup(read_usize(this.stages())) } else { Cup(0) };
        ok = !stage.is_null() && stage.team_table() != 0;
        if ok {
            found = (0..plan.len()).map(|k| read_usize(stage.team_table() + 6 * k)).collect();
            let (mut a, mut b) = (plan.clone(), found.clone());
            a.sort_unstable();
            b.sort_unstable();
            ok = a == b;
        }
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): pairs {} plan {:x?} stage list {:x?}", name, this.addr(), pairs, plan, found));
}

/// The clubs that go to the tie-break play-off, in the order of the stage's list: scanning the table rows i from the
/// top, a row and the next one form a pair when they are level on points (`league_table_row::POINTS`) and neither has
/// the final status 3 (relegated); the pair takes both rows and the scan goes on two rows below.
pub(crate) unsafe fn croatian_tiebreak_plan(this: League) -> Vec<usize> {
    let mut v = Vec::new();
    let n = this.club_count() as i32;
    let mut i = 0i32;
    while i < n {
        let row = this.table() + i as usize * league_table_row::SIZE;
        let next = row + league_table_row::SIZE;
        if i < n - 1 && read_u16(row + league_table_row::POINTS) == read_u16(next + league_table_row::POINTS)
            && read_u8(row + league_table_row::STATUS) != 3 && read_u8(next + league_table_row::STATUS) != 3 {
            v.push(read_usize(row));
            v.push(read_usize(next));
            i += 1;
        }
        i += 1;
    }
    v
}

/// The Croatian First Division (1. HNL): discipline rule 0x21 (the Croatian one), the rules, table and fixtures.
pub(crate) const CROATIAN_FIRST_DIVISION: LeagueDivision = LeagueDivision {
    vtabela: 0x9690c4,
    discipline: 0x21,
    rules: 0x5148e0,
    table: 0x5147b0,
    file: 0x9b53c0,
    rows: [0x50, 0x57, 0x5e, 0x67, 0x6e],
    low: false,
};

/// The 30 rounds of the three Croatian leagues (16 clubs, 2 legs), from late July to early May: mostly on Saturdays
/// (weekday 5, code 1), four midweek rounds (weekday 1, code 2: 7 September, 12 October, 29 February of the next year
/// [as the original: 29 February even when that year is not a leap year]); no TV slots; the winter break from
/// mid-December to mid-February.
pub(crate) const CROATIAN_LEAGUE_ROUNDS: [SpaSecondScheduleRound; 30] = [
    (24, 6, 0, 5, 1, &[(-1, -1, -1)]), // 0: 24/jul
    (31, 6, 0, 5, 1, &[(-1, -1, -1)]), // 1: 31/jul
    (28, 7, 0, 5, 1, &[(-1, -1, -1)]), // 2: 28/ago
    (7, 8, 0, 1, 2, &[(-1, -1, -1)]), // 3: 7/set
    (11, 8, 0, 5, 1, &[(-1, -1, -1)]), // 4: 11/set
    (18, 8, 0, 5, 1, &[(-1, -1, -1)]), // 5: 18/set
    (25, 8, 0, 5, 1, &[(-1, -1, -1)]), // 6: 25/set
    (2, 9, 0, 5, 1, &[(-1, -1, -1)]), // 7: 2/out
    (12, 9, 0, 1, 2, &[(-1, -1, -1)]), // 8: 12/out
    (16, 9, 0, 5, 1, &[(-1, -1, -1)]), // 9: 16/out
    (23, 9, 0, 5, 1, &[(-1, -1, -1)]), // 10: 23/out
    (30, 9, 0, 5, 1, &[(-1, -1, -1)]), // 11: 30/out
    (6, 10, 0, 5, 1, &[(-1, -1, -1)]), // 12: 6/nov
    (20, 10, 0, 5, 1, &[(-1, -1, -1)]), // 13: 20/nov
    (27, 10, 0, 5, 1, &[(-1, -1, -1)]), // 14: 27/nov
    (4, 11, 0, 5, 1, &[(-1, -1, -1)]), // 15: 4/dez
    (11, 11, 0, 5, 1, &[(-1, -1, -1)]), // 16: 11/dez
    (19, 1, 1, 5, 1, &[(-1, -1, -1)]), // 17: 19/fev (+1 ano)
    (29, 1, 1, 1, 2, &[(-1, -1, -1)]), // 18: 29/fev (+1 ano)
    (26, 1, 1, 5, 1, &[(-1, -1, -1)]), // 19: 26/fev (+1 ano)
    (4, 2, 1, 5, 1, &[(-1, -1, -1)]), // 20: 4/mar (+1 ano)
    (11, 2, 1, 5, 1, &[(-1, -1, -1)]), // 21: 11/mar (+1 ano)
    (18, 2, 1, 5, 1, &[(-1, -1, -1)]), // 22: 18/mar (+1 ano)
    (25, 2, 1, 5, 1, &[(-1, -1, -1)]), // 23: 25/mar (+1 ano)
    (1, 3, 1, 5, 1, &[(-1, -1, -1)]), // 24: 1/abr (+1 ano)
    (8, 3, 1, 5, 1, &[(-1, -1, -1)]), // 25: 8/abr (+1 ano)
    (15, 3, 1, 5, 1, &[(-1, -1, -1)]), // 26: 15/abr (+1 ano)
    (22, 3, 1, 5, 1, &[(-1, -1, -1)]), // 27: 22/abr (+1 ano)
    (29, 3, 1, 5, 1, &[(-1, -1, -1)]), // 28: 29/abr (+1 ano)
    (6, 4, 1, 5, 1, &[(-1, -1, -1)]), // 29: 6/mai (+1 ano)
];

/// The Croatian chain, run by the First Division: the final positions of the First Division and of both second division
/// groups (virtual `league_vtable::RECORD_FINAL_POSITIONS`, First, north, south), then the exchanges First–north and
/// First–south (`exe::league_promote_and_relegate`(upper, lower, 1, 1, −1, −1) on this league). The argument is not
/// read.
/// Original: `CroA1::process_promotion_relegation_chain` (0x5150f0, cro_a1.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cro_a1_process_promotion_relegation_chain(this: usize, _arg: u32) {
    let first = comp_by_global(KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4);
    for key in [KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH] {
        let league = comp_by_global(key);
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(league) + league_vtable::RECORD_FINAL_POSITIONS));
        f(league);
    }
    covered!("cx09_19 croatian chain");
    for key in [KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH] {
        exe::league_promote_and_relegate(this, first, comp_by_global(key), 1, 1, -1, -1);
    }
}

/// Regroups the Croatian second division by latitude (`croatian_second_division_by_latitude`): the first 16 clubs go
/// to the north group and the other 16 to the south group (`club::DIVISION`); a club that changes group and has a human
/// manager (`exe::news_club_has_human_manager` on `G_NEWS`) gets a local news item (`exe::new_newsitem`(0x13, 0): slot 0
/// the club's id, slot 1 the new group's id) sent to its manager (`exe::news_send_to_club_manager`). The original walks
/// all 32 slots even with fewer clubs, reading its uninitialised stack (a defect; the copy's empty slots are 0 and
/// would fault the same way).
/// Original: `cro_a1_realign_second_division_groups` (0x515190, cro_a1.cpp, cdecl).
pub(crate) unsafe extern "C" fn cro_a1_realign_second_division_groups() {
    let (north, south) = (comp_record_of_key(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH), comp_record_of_key(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH));
    let (list, _) = croatian_second_division_by_latitude();
    covered!("cx09_19 croatian second division regrouped");
    for (k, &club) in list.iter().enumerate() {
        let group = if k < 16 { north } else { south };
        if read_usize(club + club::DIVISION) != group && exe::news_club_has_human_manager(read_usize(G_NEWS), club) != 0 {
            let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
            let n = local.as_mut_ptr() as usize;
            exe::new_newsitem(n, 0x13, 0);
            exe::newsitem_set_slot(n, 0, read_u32(club));
            exe::newsitem_set_slot(n, 1, read_u32(group));
            exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
        }
        write_usize(club + club::DIVISION, group);
    }
}

/// Comparer for `cro_a1_process_promotion_relegation_chain`: only the original (it moves the clubs).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_process_promotion_relegation_chain(this: usize, arg: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_CRO_A1_PROCESS_PROMOTION_RELEGATION_CHAIN)(this, arg)
}

/// Comparer for `cro_a1_realign_second_division_groups`: only the original (it sends news); the copy's order, made
/// before, must match the groups the original gave.
pub(crate) unsafe extern "C" fn compare_cro_a1_realign_second_division_groups() {
    let (list, n) = croatian_second_division_by_latitude();
    original_fn::<unsafe extern "C" fn()>(IDX_CRO_A1_REALIGN_SECOND_DIVISION_GROUPS)();
    let (north, south) = (comp_record_of_key(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH), comp_record_of_key(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH));
    let wrong: Vec<(usize, usize)> = list[..n].iter().enumerate()
        .filter(|&(k, &c)| read_usize(c + club::DIVISION) != if k < 16 { north } else { south })
        .map(|(k, &c)| (k, c)).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("cro_a1_realign_second_division_groups: {} clubs, (slot, club) in the wrong group {:x?}", n, wrong));
}

/// The 32 Croatian second division clubs, ordered by the latitude of their stadium city: the clubs (index 0 to
/// `G_CLUB_COUNT`, read once) whose `club::DIVISION` is the north or the south group record (up to 32, in club order),
/// sorted by qsort with Database's latitude order (0x53fd40). Returns the list (32 slots) and the count.
pub(crate) unsafe fn croatian_second_division_by_latitude() -> ([usize; 32], usize) {
    let (north, south) = (comp_record_of_key(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH), comp_record_of_key(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH));
    let mut list = [0usize; 32];
    let mut n = 0usize;
    let count = read_i32(G_CLUB_COUNT);
    let mut i = 0i32;
    while i < count {
        let club = club_at(i as isize);
        let division = club.division().addr();
        if division == north || division == south {
            list[n] = club.addr();
            n += 1;
            if n == 32 {
                break;
            }
        }
        i += 1;
    }
    exe::crt_qsort(list.as_mut_ptr() as usize, n as u32, 4, 0x53fd40usize);
    (list, n)
}
