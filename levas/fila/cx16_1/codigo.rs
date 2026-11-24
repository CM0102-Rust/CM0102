// ---------- cx16_1: the CONCACAF World Cup qualifying: its constructor, the predicted positions and the knockout team table (wc_concacaf_cup.cpp) ----------

/// The CONCACAF section's vtable (WcConcacafCup).
pub(crate) const WC_CONCACAF_CUP_VTABLE: usize = 0x970c24;

/// The CONCACAF World Cup qualifying at career creation.
///
/// Cup::Cup (`exe::new_cup`), the vtable, the year `year` (word), `cup::CURRENT_ROUND` 0, the main stage, no current
/// stage (−1), 7 stages, the record (`league::COMPETITION`), `cup::CLUB_SIDES` 4, no section above (−1) and the Asian
/// section below (the competition id of [`KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION`]), 7 substitutes named and 3
/// allowed, discipline 0 and importance bias 0xa. If the saved state loads (Cup::load_from_disk `exe::cup_load_from_disk`
/// (this, 1): push 1; mov ecx -> thiscall, ret 4), returns this.
///
/// Otherwise the campaign year (`qualifying_campaign_year`: a year one past the four-year cycle goes back one, then up
/// to the cycle), the stage vector (`exe::crt_malloc` of 4 × 7, cdecl), `cup::RUN_BY_STAGES` 0; the entrants
/// (WcConcacafCup::select_entrants 0x916250; 0: the error box of line 0x5e and this); the statistics (operator new of
/// 0x5ce and Comp_Stats::Comp_Stats into `league::COMP_STATS`; null: the box of line 0x66 and this); the predicted
/// positions (WcConcacafCup::rank_teams 0x917db0) and the knockout team table (WcConcacafCup::build_team_table 0x915d40;
/// 0: the box of line 0x70 and this), all thiscall, ret 0, called directly; the round table of stage −1 (virtual
/// `comp_vtable::CREATE_SCHEDULE`(−1, &`cup::ROUND_COUNT`, &`cup::STAGE_LABEL`, 0), thiscall, ret 0x10) into
/// `cup::ROUND_TABLE`. A campaign that started before the game's year (`cup::YEAR` below `year`) stays dormant
/// (`cup::SEASON_SET_UP` 0, a dword); otherwise the two Central American groups
/// (WcConcacafCup::create_central_american_groups 0x916cd0). Last the tie tree (Cup::map_fixture_tree
/// `exe::cup_map_fixture_tree`; 0: the box of line 0x8a). Returns this. The SEH frame covers only the statistics' new.
/// Original: `WcConcacafCup::WcConcacafCup` (0x9149f0, wc_concacaf_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wc_concacaf_cup(this: usize, year: u32, record: usize) -> usize {
    use wc_cup_turnover_consts as wt;
    let cup = Cup(this);
    exe::new_cup(cup);
    write_usize(this, WC_CONCACAF_CUP_VTABLE);
    cup.set_year(year as i16);
    cup.set_current_round(0);
    cup.set_stage_index(stage_index::MAIN);
    cup.set_current_stage(usize::MAX);
    cup.set_stage_count(7);
    write_usize(this + league::COMPETITION, record);
    cup.set_club_sides(4);
    write_u32(this + league::UPPER_DIVISION_ID, u32::MAX);
    write_u32(this + league::LOWER_DIVISION_ID, read_u32(KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION));
    cup.set_substitutes_named(7);
    cup.set_substitutions_allowed(3);
    cup.set_discipline_index(0);
    cup.set_importance_bias(0xa);
    if exe::cup_load_from_disk(cup, 1) != 0 {
        return this;
    }
    covered!("cx16_1 concacaf qualifying");
    cup.set_year(qualifying_campaign_year(cup.year())); // cx16_1: the campaign of the four-year cycle
    let n = cup.stage_count() as i32;
    cup.set_stages(if n > 0 { exe::crt_malloc((n as u32) << 2) } else { 0 });
    cup.set_run_by_stages(0);
    if call_section_routine(wt::FN_WC_CONCACAF_CUP_COLLECT_ENTRANTS, this) == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x5e);
        return this;
    }
    let p = exe::crt_operator_new(0x5ce);
    let stats = if p != 0 { exe::new_comp_stats(p, cup) } else { 0 };
    write_usize(this + league::COMP_STATS, stats);
    if stats == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x66);
        return this;
    }
    call_section_routine(FN_WC_CONCACAF_CUP_RANK_TEAMS, this);
    if call_section_routine(wt::FN_WC_CONCACAF_CUP_SET_UP_SEEDS, this) == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x70);
        return this;
    }
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    cup.set_round_table(create_schedule(this, u32::MAX, this + cup::ROUND_COUNT, this + cup::STAGE_LABEL, 0));
    if cup.year() < year as i16 {
        cup.set_season_set_up(0); // cx16_1: a campaign begun before the game stays dormant
    } else {
        call_section_routine(wt::FN_WC_CONCACAF_CUP_CREATE_CENTRAL_AMERICAN_GROUPS, this);
    }
    if exe::cup_map_fixture_tree(cup) == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x8a);
    }
    this
}

/// Comparer for `construct_wc_concacaf_cup`: it allocates, reads the save and builds the stages, so only the original
/// runs; then the plan: this returned, the vtable, the record, the campaign year, the Asian section below, 7 stages and,
/// for a new campaign begun before the game's year, a dormant cup (`cup::SEASON_SET_UP` 0).
pub(crate) unsafe extern "thiscall" fn compare_construct_wc_concacaf_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    let loaded = league_will_load_saved_state(record);
    let r = o(this, year, record);
    let cup = Cup(this);
    let expected = qualifying_campaign_year(year as i16);
    let dormant = loaded || expected >= year as i16 || cup.season_set_up() == 0;
    let ok = r == this && read_usize(this) == WC_CONCACAF_CUP_VTABLE && read_usize(this + league::COMPETITION) == record
        && built_year_ok(cup.year() as u16, expected as u16) && read_u32(this + league::LOWER_DIVISION_ID) == read_u32(KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION)
        && cup.stage_count() == 7 && dormant;
    SCOREBOARD.record(ok, || format!("construct_wc_concacaf_cup({:#x}, {}, {:#x}): returned {:#x}, year {} expected {}, vtable {:#x}, +0x45 {}", this, year as i16, record, r, cup.year(), expected, read_usize(this), cup.season_set_up()));
    r
}

/// WcConcacafCup::rank_teams (thiscall, ret 0), called by the constructor.
pub(crate) const FN_WC_CONCACAF_CUP_RANK_TEAMS: usize = 0x917db0;

/// The predicted positions of the CONCACAF entrants in reputation order, for `n` entrants of which `exempt` are exempt
/// (+0xba): the first 3 − exempt get 1; then, from that place up to the sixth, the place itself plus one (a byte); the
/// seventh to ninth get 7, the tenth to twelfth 10, and from the thirteenth up to n − exempt, 13.
pub(crate) fn concacaf_predicted_positions(n: i32, exempt: i32) -> Vec<u32> {
    let seeded = 3 - exempt;
    let mut v: Vec<u32> = (0..seeded.max(0)).map(|_| 1).collect();
    let mut j = seeded as i16;
    while j < 6 {
        v.push((j as u8).wrapping_add(1) as u32);
        j = j.wrapping_add(1);
    }
    v.extend((6..12).map(|s| if s < 9 { 7 } else { 10 })); // cx16_1: the middle bands
    v.extend((12..n - exempt).map(|_| 13));
    v
}

/// rule: the CONCACAF qualifying predicts the places by reputation: the three best (fewer by the exempt hosts and
/// holder) first, then 4, 5 and 6, the next three 7, the next three 10 and the rest 13; every entrant's national team
/// becomes busy with this competition
///
/// Without statistics (`league::COMP_STATS`), the error box of line 0x7cf; a copy of the entrant list (+0xb2, the count
/// +0xb6 dword; `exe::crt_malloc` of 4 × count, cdecl; out of memory, the box of line 0x7d7) sorted by reputation
/// (`predicted_by_reputation_sort`: qsort with 0x4b07d0). The North American national teams are released
/// (`exe::nationalteams_release_competitions_of_continent`([`G_NATIONAL_TEAMS`], the continent of
/// [`KEY_CONTINENT_NORTH_AMERICA`] in [`G_CONTINENT_TABLE`], 0xc6 bytes each, 0): push 0; push; mov ecx -> thiscall, ret
/// 8); the places of `concacaf_predicted_positions`(count, exempt +0xba, signed byte) go in order to the sorted copy
/// (`exe::comp_stats_set_club_final_position`, thiscall, ret 8; the original pushes the places of the second band with
/// stack garbage in the high bytes: the callee stores only the byte); then the first count − exempt of the copy that
/// are there and have a nation become busy with the record (`exe::nationalteams_set_current_competition`, thiscall,
/// ret 8); the copy freed (`exe::crt_free`).
/// Original: `WcConcacafCup::rank_teams` (0x917db0, wc_concacaf_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_rank_teams(this: usize) {
    use wc_cup_turnover_consts as wt;
    let stats = read_usize(this + league::COMP_STATS);
    if stats == 0 {
        return show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x7cf);
    }
    let n = read_u32(this + 0xb6); // the entrant count
    let v = exe::crt_malloc(n << 2);
    if v == 0 {
        return show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x7d7);
    }
    core::ptr::copy_nonoverlapping(read_usize(this + wt::CUP_ENTRANT_LIST) as *const u8, v as *mut u8, (n << 2) as usize);
    predicted_by_reputation_sort(v, n);
    let national_teams = read_usize(G_NATIONAL_TEAMS);
    exe::nationalteams_release_competitions_of_continent(national_teams, read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_NORTH_AMERICA).wrapping_mul(0xc6)), 0);
    let exempt = read_u8(this + 0xba) as i8 as i32; // the exempt North Americans (hosts and holder)
    covered!("cx16_1 concacaf predicted positions");
    for (k, p) in concacaf_predicted_positions(n as i32, exempt).into_iter().enumerate() {
        exe::comp_stats_set_club_final_position(stats, read_usize(v + k * 4), p);
    }
    for k in 0..(n as i32 - exempt).max(0) as usize {
        let team = Club(read_usize(v + k * 4));
        if !team.is_null() && !team.nation().is_null() {
            exe::nationalteams_set_current_competition(national_teams, team.nation().addr(), read_usize(this + league::COMPETITION));
        }
    }
    exe::crt_free(v);
}

/// What the predicted positions leave in the game for each entrant of the list: its place in the statistics and its
/// national team's current competition.
pub(crate) unsafe fn concacaf_entrant_marks(this: usize) -> Vec<(usize, u8, usize)> {
    let (stats, list, n) = (read_usize(this + league::COMP_STATS), read_usize(this + 0xb2), read_u32(this + 0xb6) as i32);
    if stats == 0 || list == 0 || !(0..=64).contains(&n) {
        return Vec::new();
    }
    (0..n as usize)
        .map(|k| {
            let team = Club(read_usize(list + k * 4));
            if team.is_null() {
                return (0, 0, 0);
            }
            let e = exe::comp_stats_find_club_entry(stats, team);
            let place = if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 };
            let busy = if !team.nation().is_null() { exe::nationalteams_get_current_competition(read_usize(G_NATIONAL_TEAMS), team.nation().addr()) } else { 0 };
            (team.addr(), place, busy)
        })
        .collect()
}

/// Comparer for `wc_concacaf_cup_rank_teams`: the original runs, then the copy again on the state it left (the places
/// and the busy marks are written again the same way); each entrant's place and its national team's competition must
/// not change.
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_rank_teams(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let a = concacaf_entrant_marks(this);
    if a.is_empty() {
        return;
    }
    wc_concacaf_cup_rank_teams(this);
    let b = concacaf_entrant_marks(this);
    SCOREBOARD.record(a == b, || format!("wc_concacaf_cup_rank_teams({:#x}): original {:x?} rust {:x?}", this, a, b));
}

/// rule: the CONCACAF qualifying knockout holds the first 24 Caribbean entrants, fewer by the exempt Caribbean nations
///
/// `cup::TEAM_COUNT` = 0x18 − the exempt Caribbean count (+0xbb, signed byte; a word); `cup::TEAM_TABLE` = a table of
/// that many 6-byte rows (`exe::crt_malloc`, cdecl; out of memory, the error box of line 0x350 and −1); row i gets the
/// entrant of slot i of the list (+0xb2) and its seed and status bytes (+4, +5) 0. Returns 1.
/// Original: `WcConcacafCup::build_team_table` (0x915d40, wc_concacaf_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_build_team_table(this: usize) -> i32 {
    let cup = Cup(this);
    cup.set_team_count((0x18i16).wrapping_sub(read_u8(this + 0xbb) as i8 as i16)); // cx16_1: 24 Caribbean places
    let t = exe::crt_malloc((cup.team_count() as i32 as u32).wrapping_mul(6));
    cup.set_team_table(t);
    if t == 0 {
        show_error_at(wc_cup_turnover_consts::WC_CONCACAF_CUP_CPP, 0x350);
        return -1;
    }
    covered!("cx16_1 concacaf team table");
    let list = read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    let mut i = 0i16;
    while i < cup.team_count() {
        let row = t + i as usize * 6;
        write_usize(row, read_usize(list + i as usize * 4));
        write_u8(row + 4, 0);
        write_u8(row + 5, 0);
        i += 1;
    }
    1
}

/// Comparer for `wc_concacaf_cup_build_team_table`: the original builds its table; the copy then builds its own, which
/// must have the same count and rows; the copy's table is freed and the original's put back.
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_build_team_table(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%)(this);
    let cup = Cup(this);
    let (t, n) = (cup.team_table(), cup.team_count());
    if o != 1 || t == 0 || !(0..=64).contains(&n) {
        return o;
    }
    let rows = |t: usize, n: i16| core::slice::from_raw_parts(t as *const u8, n as usize * 6).to_vec();
    let a = rows(t, n);
    let r = wc_concacaf_cup_build_team_table(this);
    let (t2, n2) = (cup.team_table(), cup.team_count());
    let b = if t2 != 0 && n2 == n { rows(t2, n2) } else { Vec::new() };
    if t2 != 0 && t2 != t {
        exe::crt_free(t2);
    }
    cup.set_team_table(t);
    cup.set_team_count(n);
    SCOREBOARD.record(r == o && a == b, || format!("wc_concacaf_cup_build_team_table({:#x}): {} teams, original {:x?} rust {:x?}", this, n, a, b));
    o
}
