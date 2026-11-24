// ---------- sbG_2: the African World Cup qualifying: its constructor, the round tables and the predicted positions (wc_african_cup.cpp) ----------

/// The African section's vtable (WcAfricanCup).
pub(crate) const WC_AFRICAN_CUP_VTABLE: usize = 0x970ad0;

/// WcAfricanCup::rank_teams (thiscall, ret 0), called by the constructor.
pub(crate) const FN_WC_AFRICAN_CUP_RANK_TEAMS: usize = 0x9111d0;

/// The stage count of the African qualifying by its format byte (`wc_cup_turnover_consts::AFRICAN_FORMAT`): 0 or 3 give 5,
/// 1 gives 4, any other 7 (the same choice as the turnover's).
pub(crate) fn african_stage_count(format: u8) -> usize {
    match format {
        0 | 3 => 5,
        1 => 4,
        _ => 7,
    }
}

/// The African World Cup qualifying at career creation.
///
/// Cup::Cup (`exe::new_cup`), the record (`league::COMPETITION`), the vtable, the year `year` (word), `cup::CURRENT_ROUND`
/// 0, the main stage, no current stage (−1), `cup::CLUB_SIDES` 4, the South American section above and the European one
/// below (the competition ids of [`KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT`] and
/// [`KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION`]), 7 substitutes named and 3 allowed, discipline 0 and importance
/// bias 0xa; the campaign year (`qualifying_campaign_year`). If the saved state loads (Cup::load_from_disk
/// `exe::cup_load_from_disk`(this, 1)), returns this.
///
/// Otherwise the entrants (WcAfricanCup::collect_entrants 0x9106a0; 0: the error box of line 0x55 and this); the stage
/// count by the format byte (`african_stage_count`); the statistics (operator new of 0x5ce and Comp_Stats::Comp_Stats into
/// `league::COMP_STATS`; null: the box of line 0x65 and this); the stage vector (`exe::crt_malloc` of 4 × count, or none);
/// `cup::RUN_BY_STAGES` 0 and `cup::STAGE_LABEL` 0; the predicted positions (WcAfricanCup::rank_teams 0x9111d0) and the
/// first round entries (WcAfricanCup::build_first_round_entries 0x910120; only 0 is an error: the box of line 0x7b and
/// this; its own −1 out of memory goes on), all thiscall, ret 0, called directly; the round table of stage −1 (virtual
/// `comp_vtable::CREATE_SCHEDULE`(−1, &`cup::ROUND_COUNT`, &`cup::STAGE_LABEL`, 0)) into `cup::ROUND_TABLE`. A campaign
/// that started before the game's year (`cup::YEAR` below `year`, signed words) stays dormant (`cup::SEASON_SET_UP` 0, a
/// dword). Last the tie tree (Cup::map_fixture_tree `exe::cup_map_fixture_tree`; 0: the box of line 0x90). Returns this.
/// The SEH frame covers only the statistics' new.
/// Original: `WcAfricanCup::WcAfricanCup` (0x90f210, wc_african_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wc_african_cup(this: usize, year: u32, record: usize) -> usize {
    use wc_cup_turnover_consts as wt;
    let cup = Cup(this);
    exe::new_cup(cup);
    write_usize(this + league::COMPETITION, record);
    write_usize(this, WC_AFRICAN_CUP_VTABLE);
    cup.set_year(year as i16);
    cup.set_current_round(0);
    cup.set_stage_index(stage_index::MAIN);
    cup.set_current_stage(usize::MAX);
    cup.set_club_sides(4);
    write_u32(this + league::UPPER_DIVISION_ID, read_u32(KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT));
    write_u32(this + league::LOWER_DIVISION_ID, read_u32(KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION));
    cup.set_substitutes_named(7);
    cup.set_substitutions_allowed(3);
    cup.set_discipline_index(0);
    cup.set_importance_bias(0xa);
    cup.set_year(qualifying_campaign_year(cup.year())); // sbG_2: the campaign of the four-year cycle
    if exe::cup_load_from_disk(cup, 1) != 0 {
        return this;
    }
    covered!("sbG_2 african qualifying");
    if call_section_routine(wt::FN_WC_AFRICAN_CUP_COLLECT_ENTRANTS, this) == 0 {
        show_error_at(wt::WC_AFRICAN_CUP_CPP, 0x55);
        return this;
    }
    cup.set_stage_count(african_stage_count(read_u8(this + wt::AFRICAN_FORMAT)));
    let p = exe::crt_operator_new(0x5ce);
    let stats = if p != 0 { exe::new_comp_stats(p, cup) } else { 0 };
    write_usize(this + league::COMP_STATS, stats);
    if stats == 0 {
        show_error_at(wt::WC_AFRICAN_CUP_CPP, 0x65);
        return this;
    }
    let n = cup.stage_count() as i32;
    cup.set_stages(if n > 0 { exe::crt_malloc((n as u32) << 2) } else { 0 });
    cup.set_run_by_stages(0);
    write_u16(this + cup::STAGE_LABEL, 0);
    call_section_routine(FN_WC_AFRICAN_CUP_RANK_TEAMS, this);
    if call_section_routine(wt::FN_WC_AFRICAN_CUP_SET_UP_FIRST_ROUND, this) == 0 {
        show_error_at(wt::WC_AFRICAN_CUP_CPP, 0x7b);
        return this;
    }
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    cup.set_round_table(create_schedule(this, u32::MAX, this + cup::ROUND_COUNT, this + cup::STAGE_LABEL, 0));
    if cup.year() < year as i16 {
        cup.set_season_set_up(0); // sbG_2: a campaign begun before the game stays dormant
    }
    if exe::cup_map_fixture_tree(cup) == 0 {
        show_error_at(wt::WC_AFRICAN_CUP_CPP, 0x90);
    }
    this
}

/// Comparer for `construct_wc_african_cup`: it allocates, reads the save and builds the stages, so only the original
/// runs; then the plan: this returned, the vtable, the record, the campaign year, the sections above and below, the stage
/// count of the format and, for a new campaign begun before the game's year, a dormant cup (`cup::SEASON_SET_UP` 0).
pub(crate) unsafe extern "thiscall" fn compare_construct_wc_african_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    let loaded = league_will_load_saved_state(record);
    let r = o(this, year, record);
    let cup = Cup(this);
    let expected = qualifying_campaign_year(year as i16);
    let dormant = loaded || expected >= year as i16 || cup.season_set_up() == 0;
    let stages = loaded || cup.stage_count() == african_stage_count(read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT));
    let ok = r == this && read_usize(this) == WC_AFRICAN_CUP_VTABLE && read_usize(this + league::COMPETITION) == record
        && built_year_ok(cup.year() as u16, expected as u16) && read_u32(this + league::UPPER_DIVISION_ID) == read_u32(KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT)
        && read_u32(this + league::LOWER_DIVISION_ID) == read_u32(KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION) && stages && dormant;
    SCOREBOARD.record(ok, || format!("construct_wc_african_cup({:#x}, {}, {:#x}): returned {:#x}, year {} expected {}, vtable {:#x}, stages {}, +0x45 {}", this, year as i16, record, r,
        cup.year(), expected, read_usize(this), cup.stage_count(), cup.season_set_up()));
    r
}

/// The African qualifying stage plan of `stage` (signed byte) by the format byte `format`: −1 the first round (one
/// two-legged cup round); the group stages (format 0 below 5, 1 below 4, 2 below 6, 3 below 4; label 0x407 + stage, or
/// 0x472 for stage 5) a league of 10 rounds over two years, or 6 with format 2; the final play-off (format 2 stage 6,
/// format 3 stage 4) one cup round; any other, none.
pub(crate) fn wc_african_cup_stage_plan(stage: u32, format: u8) -> Option<GoldcupStagePlan> {
    let name = "sbG_2 african qualifying round table";
    let s = stage as u8 as i8;
    if s == -1 {
        return Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0xde, ops: &WC_AFRICAN_FIRST_ROUND_OPS, ops2: &[], name });
    }
    let group = (format == 0 && s < 5) || (format == 1 && s < 4) || (format == 2 && s < 6) || (format == 3 && s < 4);
    if group {
        let (rounds, ops): (i16, &'static [GoldcupScheduleOp]) = if format == 2 { (6, &WC_AFRICAN_GROUP_SIX_OPS) } else { (0xa, &WC_AFRICAN_GROUP_TEN_OPS) };
        let extra = if s == 5 { 0x472 } else { (s as i16).wrapping_add(0x407) };
        return Some(GoldcupStagePlan { league: 1, rounds, extra, extra_check: false, size: rounds as u32 * league_round_date::SIZE as u32, row: 0x126, ops, ops2: &[], name });
    }
    if (format == 2 && s == 6) || (format == 3 && s == 4) {
        return Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x171, ops: &WC_AFRICAN_PLAY_OFF_OPS, ops2: &[], name });
    }
    None
}

/// The fields of an African cup round that depend on the format byte: the first round (stage −1) takes 50 teams in 25
/// ties (format 0), 48 in 24 (formats 1 and 2) or 46 in 23 (any other), all of them new; the final play-off 6 teams in 3
/// ties (format 2) or 4 in 2. The group stages have none.
pub(crate) fn wc_african_cup_stage_fields(stage: u32, format: u8) -> Vec<GoldcupScheduleOp> {
    let s = stage as u8 as i8;
    let (teams, ties) = if s == -1 {
        match format {
            0 => (0x32, 0x19), // sbG_2: 50 teams in the first round
            1 | 2 => (0x30, 0x18),
            _ => (0x2e, 0x17),
        }
    } else if (format == 2 && s == 6) || (format == 3 && s == 4) {
        if format == 2 { (6, 3) } else { (4, 2) }
    } else {
        return Vec::new();
    };
    vec![Cqp12W(cup_round::TIES, ties), Cqp12W(cup_round::TEAMS, teams), Cqp12W(cup_round::NEW_TEAMS, teams)]
}

/// rule: the African World Cup qualifying opens with a two-legged first round (drawn 18 January, played from 8 April;
/// 50, 48 or 46 teams by the exempt nations), then group stages of ten rounds (June to July of the next year) or six
/// with format 2, and with formats 2 and 3 a final two-legged play-off (drawn 29 September, played 20 October of the next
/// year)
///
/// The stage plan of `wc_african_cup_stage_plan` with the fields of `wc_african_cup_stage_fields`
/// (`build_phase_table_with_fields`): each date is a call of the original (league_round_set_date 0x68a160,
/// league_round_set_match_slot 0x68a1c0, cup_set_round_draw_date 0x521e60, cup_set_round_match_date 0x521eb0: pushes;
/// add esp -> cdecl) in the original's order (months from 0 = January); the outputs (`rounds` and `extra` always written,
/// `league` only when not null), the table size and the error-box line when out of memory per stage. The original pushes
/// the year as a word with garbage in the high bytes to 0x68a160, and the last group round's index with the caller's ebx
/// in the high bytes; 0x68a160 reads both words.
/// Original: `WcAfricanCup::create_round_table` (0x90f600, wc_african_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let format = read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT);
    let plan = wc_african_cup_stage_plan(stage, format);
    build_phase_table_with_fields(League(this), plan, &wc_african_cup_stage_fields(stage, format), rounds, extra, league, wc_cup_turnover_consts::WC_AFRICAN_CUP_CPP)
}

/// Comparer for `wc_african_cup_create_round_table`: side by side (`compare_phase_table_with_fields`).
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    let format = read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT);
    let plan = wc_african_cup_stage_plan(stage, format);
    let fields = wc_african_cup_stage_fields(stage, format);
    compare_phase_table_with_fields("wc_african_cup_create_round_table", stage, plan, &fields, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| wc_african_cup_create_round_table(this, stage, r, e, l))
}

/// The African first round (stage −1): drawn 18 January, first legs 8 April, two legs 14 days apart, rule 0x400.
pub(crate) const WC_AFRICAN_FIRST_ROUND_OPS: [GoldcupScheduleOp; 15] = [
    Cqp12S(0, 0x12, 0, 0, 4), Cqp12J(0, 8, 3, 0, 5, 1, 0), Cqp12W(7, 0xa), Cqp12W(9, 0), Cqp12W(0xb, 0), Cqp12W(0xd, 0x400), Cqp12B(0x17, 0xa),
    Cqp12W(0xf, 3), Cqp12W(0x1e, 0), Cqp12B(0x20, 0), Cqp12B(0x21, 2), Cqp12B(0x22, 0xe), Cqp12X(0x5c, 0), Cqp12X(0x60, 0), Cqp12X(0x64, 0),
];

/// The African group rounds with ten rounds (formats other than 2): from 17 June to 17 July of the next year.
pub(crate) const WC_AFRICAN_GROUP_TEN_OPS: [GoldcupScheduleOp; 20] = [
    Cqp12D(0, 0x11, 5, 0, 5, 1, 0), Cqp12L(0, 0, -1, -1, -1, 0), Cqp12D(1, 8, 6, 0, 6, 1, 0), Cqp12L(1, 0, -1, -1, -1, 0),
    Cqp12D(2, 0x1b, 0, 1, 5, 1, 0), Cqp12L(2, 0, -1, -1, -1, 0), Cqp12D(3, 0x18, 1, 1, 5, 1, 0), Cqp12L(3, 0, -1, -1, -1, 0),
    Cqp12D(4, 0xa, 2, 1, 5, 1, 0), Cqp12L(4, 0, -1, -1, -1, 0), Cqp12D(5, 0x15, 3, 1, 5, 1, 0), Cqp12L(5, 0, -1, -1, -1, 0),
    Cqp12D(6, 5, 4, 1, 5, 1, 0), Cqp12L(6, 0, -1, -1, -1, 0), Cqp12D(7, 0x1e, 5, 1, 5, 1, 0), Cqp12L(7, 0, -1, -1, -1, 0),
    Cqp12D(8, 0xe, 6, 1, 5, 1, 0), Cqp12L(8, 0, -1, -1, -1, 0), Cqp12D(9, 0x11, 6, 1, 2, 2, 0), Cqp12L(9, 0, -1, -1, -1, 0),
];

/// The African group rounds with six rounds (format 2): the ten-round dates without the first, the fourth, the eighth
/// and the last.
pub(crate) const WC_AFRICAN_GROUP_SIX_OPS: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 8, 6, 0, 6, 1, 0), Cqp12L(0, 0, -1, -1, -1, 0), Cqp12D(1, 0x1b, 0, 1, 5, 1, 0), Cqp12L(1, 0, -1, -1, -1, 0),
    Cqp12D(2, 0xa, 2, 1, 5, 1, 0), Cqp12L(2, 0, -1, -1, -1, 0), Cqp12D(3, 0x15, 3, 1, 5, 1, 0), Cqp12L(3, 0, -1, -1, -1, 0),
    Cqp12D(4, 5, 4, 1, 5, 1, 0), Cqp12L(4, 0, -1, -1, -1, 0), Cqp12D(5, 0xe, 6, 1, 5, 1, 0), Cqp12L(5, 0, -1, -1, -1, 0),
];

/// The African final play-off (format 2 stage 6, format 3 stage 4): drawn 29 September, first legs 20 October of the
/// next year, round code 0x96.
pub(crate) const WC_AFRICAN_PLAY_OFF_OPS: [GoldcupScheduleOp; 15] = [
    Cqp12S(0, 0x1d, 8, 1, 0), Cqp12J(0, 0x14, 9, 1, 5, 1, 0), Cqp12W(7, 0x96), Cqp12W(9, 0), Cqp12W(0xb, 0), Cqp12W(0xd, 0), Cqp12B(0x17, 0xa),
    Cqp12W(0xf, 3), Cqp12W(0x1e, 0), Cqp12B(0x20, 0), Cqp12B(0x21, 2), Cqp12B(0x22, 0xe), Cqp12X(0x5c, 0), Cqp12X(0x60, 0), Cqp12X(0x64, 0),
];

/// The places an African entrant at sorted index `k` gets by the format byte, in the order the original writes them:
/// format 0 bands of five (1, 6, 11, 16, 21, then 26), format 1 bands of four (1, 5, 9, 13, 17, 21, then 25), format 2
/// bands 3/3/6/6/6 (1, 4, 7, 13, 19, then 25), format 3 bands of four (3, 5, 9, 13, 17, 21, then 25; the first two get 1
/// and then 3: a defect of the original, the best two end with 3), any other format none.
pub(crate) fn african_predicted_places(k: i16, format: u8) -> &'static [u32] {
    let band = |limits: &[i16]| limits.iter().position(|&l| k < l).unwrap_or(limits.len());
    const FORMAT_0: [&[u32]; 6] = [&[1], &[6], &[0xb], &[0x10], &[0x15], &[0x1a]];
    const FORMAT_1: [&[u32]; 7] = [&[1], &[5], &[9], &[0xd], &[0x11], &[0x15], &[0x19]];
    const FORMAT_2: [&[u32]; 6] = [&[1], &[4], &[7], &[0xd], &[0x13], &[0x19]];
    const FORMAT_3: [&[u32]; 8] = [&[1, 3], &[3], &[5], &[9], &[0xd], &[0x11], &[0x15], &[0x19]];
    match format {
        0 => FORMAT_0[band(&[5, 10, 15, 20, 25])], // sbG_2: bands of five
        1 => FORMAT_1[band(&[4, 8, 12, 16, 20, 24])],
        2 => FORMAT_2[band(&[3, 6, 12, 18, 24])],
        3 => FORMAT_3[band(&[2, 4, 8, 12, 16, 20, 24])],
        _ => &[],
    }
}

/// rule: the African qualifying predicts the places by reputation in bands that depend on how many nations are exempt;
/// every entrant's national team becomes busy with this competition
///
/// Without statistics (`league::COMP_STATS`), the error box of line 0x556; a copy of the entrant list
/// (`wc_cup_turnover_consts::CUP_ENTRANT_LIST`, the count +0xb6 dword; `exe::crt_malloc` of 4 × count, cdecl; out of
/// memory, the box of line 0x55e) sorted by reputation (`predicted_by_reputation_sort`: qsort with 0x4b07d0). The African
/// national teams are released (`exe::nationalteams_release_competitions_of_continent`([`G_NATIONAL_TEAMS`], the continent
/// of [`KEY_CONTINENT_AFRICA`] in [`G_CONTINENT_TABLE`], 0xc6 bytes each, 0)); then, for each k (a signed word) below the
/// count, the places of `african_predicted_places`(k, the format byte read again each time) go to entrant k
/// (`exe::comp_stats_set_club_final_position`) and its national team becomes busy with the record
/// (`exe::nationalteams_set_current_competition`, the nation `club::NATION` of the entrant, neither checked). The copy is
/// freed (`exe::crt_free`).
/// Original: `WcAfricanCup::rank_teams` (0x9111d0, wc_african_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_rank_teams(this: usize) {
    use wc_cup_turnover_consts as wt;
    let stats = read_usize(this + league::COMP_STATS);
    if stats == 0 {
        return show_error_at(wt::WC_AFRICAN_CUP_CPP, 0x556);
    }
    let n = read_u32(this + 0xb6); // the entrant count
    let v = exe::crt_malloc(n << 2);
    if v == 0 {
        return show_error_at(wt::WC_AFRICAN_CUP_CPP, 0x55e);
    }
    core::ptr::copy_nonoverlapping(read_usize(this + wt::CUP_ENTRANT_LIST) as *const u8, v as *mut u8, (n << 2) as usize);
    predicted_by_reputation_sort(v, n);
    let national_teams = read_usize(G_NATIONAL_TEAMS);
    exe::nationalteams_release_competitions_of_continent(national_teams, read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_AFRICA).wrapping_mul(0xc6)), 0);
    covered!("sbG_2 african predicted positions");
    let mut k = 0i16;
    while (k as i32) < read_i32(this + 0xb6) {
        let team = read_usize(v + k as usize * 4);
        for &place in african_predicted_places(k, read_u8(this + wt::AFRICAN_FORMAT)) {
            exe::comp_stats_set_club_final_position(stats, team, place);
        }
        exe::nationalteams_set_current_competition(national_teams, read_usize(team + club::NATION), read_usize(this + league::COMPETITION));
        k = k.wrapping_add(1);
    }
    exe::crt_free(v);
}

/// Comparer for `wc_african_cup_rank_teams`: the original runs, then the copy again on the state it left (the places
/// and the busy marks are written again the same way); each entrant's place and its national team's competition
/// (`concacaf_entrant_marks`, the same fields) must not change.
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_rank_teams(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let a = concacaf_entrant_marks(this);
    if a.is_empty() {
        return;
    }
    wc_african_cup_rank_teams(this);
    let b = concacaf_entrant_marks(this);
    SCOREBOARD.record(a == b, || format!("wc_african_cup_rank_teams({:#x}, format {}): original {:x?} rust {:x?}", this, read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT), a, b));
}
