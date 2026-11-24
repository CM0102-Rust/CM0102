// ---------- cx17_5: the World Cup's staff records, progress codes and new season (world_cup.cpp) ----------

/// rule: the World Cup's career records: in the groups a nation through to the round of 16 gets the qualification (stage
/// 0x14, weight 30) and one going out the failed-to-qualify record (weight 30) and ends its campaign; in the knockout a
/// winner of a tie gets the qualification for the next round and a loser the knock-out record, both weighted 50 to 100
/// by round; the champion, the runner-up and the third get their records and end the campaign with every other nation
/// knocked out; the champion's status becomes 4
///
/// `this.update_staff_records_for_status(club, outcome, stage, out, game, final_)` (virtual +0x44). The records go to
/// Staff_Records ([`G_STAFF_RECORDS`]); the competition is `league::COMPETITION`; ending a campaign is
/// `clear_national_team_competition`. Stage below 7 (signed byte, the groups): outcome 6, `european_cup_add_qualified_record`
/// (club, competition, 0, 0x14, 0x1e); 0xfe, `european_cup_add_failed_to_qualify_record`(club, competition, 0x41c, 0x1e)
/// and the campaign ended; any other, nothing. Stage 7 (the knockout): outcome 0, the qualification with the fixture's
/// `round_prefix()`, the round code of the next round of the knockout stage (`.stages()`[7], its round table, round
/// `round_number()` + 1, `cup_round::ROUND_CODE`) and the weight `scale_by_round`(0x32, 0x64, `round_number()`, 5);
/// 0xff, `european_cup_add_knocked_out_record`(club, competition, `round_prefix()`, `round_name()`, the same weight) and
/// the campaign ended; 4, only the campaign ended; 1, 2 and 3, the winners, runners-up and third-place records
/// (`cup_record_final_result` with 0x86a800, 0x86b0b0 and 0x86b710: club, game, final_), for 1 `*out` = 4, then the
/// campaign ended; any other, nothing. Past stage 7, nothing. Returns 0.
/// Original: `WorldCup::update_staff_records_for_status` (0x92ccf0, world_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn world_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, final_: u32) -> u32 {
    let league = League(this);
    let comp = league.competition().addr();
    let stage = stage as u8 as i8;
    if stage < 7 {
        match outcome as u8 {
            6 => european_cup_add_qualified_record(club, comp, 0, 0x14, 0x1e),
            0xfe => {
                covered!("cx17_5 world cup group exit");
                european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x1e);
                clear_national_team_competition(Club(club)); // cx17_5: the group exit ends the campaign
            }
            _ => {}
        }
        return 0;
    }
    if stage != 7 {
        return 0;
    }
    covered!("cx17_5 world cup knockout records");
    let game = Fixture(game);
    // simplification: the original pushes the fixture's words in registers with garbage in the high 16 bits; the
    // records and scale_by_round only read the words
    match outcome as u8 {
        0 => {
            let weight = scale_by_round(0x32, 0x64, game.round_number(), 5);
            let rounds = Cup(read_usize(league.stages() + 7 * 4)).round_table();
            let next = read_u16(rounds.wrapping_add((game.round_number() as i32 as usize).wrapping_mul(cup_round::SIZE)) + cup_round::SIZE + cup_round::ROUND_CODE);
            european_cup_add_qualified_record(club, comp, game.round_prefix() as u16 as u32, next as u32, weight);
            return 0;
        }
        0xff => {
            let weight = scale_by_round(0x32, 0x64, game.round_number(), 5);
            european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
        }
        4 => {}
        1 => {
            covered!("cx17_5 world cup winner");
            cup_record_final_result(FN_STAFF_RECORDS_ADD_CUP_WINNERS_RECORD, club, game, final_);
            write_u8(out, 4); // cx17_5: the champion's status
        }
        2 => cup_record_final_result(FN_STAFF_RECORDS_ADD_CUP_RUNNERS_UP_RECORD, club, game, final_),
        3 => cup_record_final_result(0x86b710, club, game, final_), // Staff_Records: the third place
        _ => return 0,
    }
    clear_national_team_competition(Club(club));
    0
}

/// The World Cup's progress codes (current, best, worst) at `stage` (signed byte): in the groups (below 7) a code 1 or 2
/// becomes 1 at best and 9 now and at worst, 3 becomes 17 and 4 becomes 25; in the knockout (from 7) they stay as they
/// came.
pub(crate) fn world_cup_progress_codes(stage: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    if stage >= 7 {
        return (current, best, worst);
    }
    let band = |v: u8, top: u8| match v {
        1 | 2 => top,
        3 => 0x11, // cx17_5: the third of a group
        4 => 0x19,
        _ => v,
    };
    (band(current, 9), band(best, 1), band(worst, 9))
}

/// rule: a nation's World Cup progress: in a group the first two are 1 at best and 9 now and at worst (the round of 16),
/// the third 17 and the fourth 25; in the knockout the codes stay as they came
///
/// The nation's statistics entry (`exe::comp_stats_find_club_entry`, not checked for null) gets the codes of
/// `world_cup_progress_codes` (`write_progress_codes`: best, current, worst, in the original's order).
/// Original: `WorldCup::set_club_progress` (0x92f9e0, world_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn world_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx17_5 world cup progress");
    let e = exe::comp_stats_find_club_entry(League(this).comp_stats(), club);
    let (c, b, w) = world_cup_progress_codes(stage as i8, current as u8, best as u8, worst as u8);
    write_progress_codes(e, c, b, w);
}

/// The six World Cup qualifying sections the World Cup restarts at its turnover, in the original's order.
const WORLD_CUP_QUALIFYING_SECTIONS: [usize; 6] = [KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION,
    KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION,
    KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION];

/// rule: at the World Cup's turnover the six qualifying sections start their new campaign, the finished World Cup is
/// cleared (table, dates, stages) with its statistics archived, the edition moves on four years and the team list and
/// the rules are built again
///
/// `league::SEASON_CLOSED` = 0; each section of `WORLD_CUP_QUALIFYING_SECTIONS` (the competition object of its key in
/// [`G_COMP_OBJECT_TABLE`], not checked for null) restarts (virtual `comp_vtable::START_NEW_SEASON`, thiscall, ret 0;
/// its eax not read). Then `exe::league_free_table(this, 0)`, `league::CLUB_COUNT` = 0, `league::ACTIVE` = 0, the round
/// dates freed and cleared; the statistics, when there, archived (`exe::comp_stats_archive_season_and_reset` with 1);
/// the stages 0..=`current_stage` destroyed (`destroy_comp_stages`; the vector stays). The year + 4 (word),
/// `league::STAGE_INDEX` = 0xff (the main competition), `current_stage` = −1, `league::ENTRANTS_ARE_QUALIFIERS` = 1; the
/// team list (`exe::worldcup_set_up_team_list`, its eax not read) and the rules (virtual `comp_vtable::SET_UP_RULES`):
/// 0, the world_cup.cpp box of line 0x332 and 0; otherwise `league::TABLE` = null and the rules' result.
/// Original: `WorldCup::start_new_season` (0x92d330, world_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_cup_start_new_season(this: usize) -> u32 {
    let league = League(this);
    league.set_season_closed(0);
    for key in WORLD_CUP_QUALIFYING_SECTIONS {
        let section = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(key).wrapping_mul(4)));
        let restart: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(section) + comp_vtable::START_NEW_SEASON));
        restart(section);
    }
    exe::league_free_table(league, 0);
    league.set_club_count(0);
    league.set_active(0);
    if league.round_dates() != 0 {
        exe::crt_free(league.round_dates());
        league.set_round_dates(0);
    }
    if league.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(league.comp_stats(), 1);
    }
    destroy_comp_stages(league);
    covered!("cx17_5 world cup turnover");
    league.set_year(league.year().wrapping_add(4)); // cx17_5: the next edition
    league.set_stage_index(stage_index::MAIN);
    league.set_current_stage(0xffff_ffff);
    league.set_entrants_are_qualifiers(1);
    exe::worldcup_set_up_team_list(league);
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_UP_RULES));
    let r = rules(this);
    if r == 0 {
        // simplification: after the box the original returns whatever the box left in eax; the caller reads 0 or not
        show_error_at(world_cup_knockout_consts::WORLD_CUP_CPP, 0x332);
        return 0;
    }
    league.set_table(0);
    r
}

/// Comparer for `world_cup_update_staff_records_for_status`: the records and the end of the campaigns do not run
/// twice, so only the original runs; the plan: 0 and, for the champion (stage 7, outcome 1), `*out` = 4.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, final_: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(%%I0%%);
    let champion = stage as u8 == 7 && outcome as u8 == 1 && out != 0;
    let r = o(this, club, outcome, stage, out, game, final_);
    SCOREBOARD.record(r == 0 && (!champion || read_u8(out) == 4), || format!("world_cup_update_staff_records_for_status({:#x}, {}, stage {}) returned {}", club, outcome as u8, stage as u8 as i8, r));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_world_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    compare_progress_codes("world_cup_set_club_progress", this, club, stage, (current, best, worst), || o(this, club, stage, current, best, worst),
        || world_cup_set_club_progress(this, club, stage, current, best, worst));
}

/// Comparer for `world_cup_start_new_season`: it restarts the six sections and frees the finished World Cup (it cannot
/// run twice), so only the original runs; the plan: the year four on, the season reopened, the main competition back
/// at no stage with the qualifiers' list, no clubs and inactive, and with the rules set up no table.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    let league = League(this);
    let year = league.year();
    let r = o(this);
    let ok = league.year() == year.wrapping_add(4) && league.season_closed() == 0 && league.current_stage() == 0xffff_ffff
        && league.stage_index() == stage_index::MAIN && read_u8(this + league::ENTRANTS_ARE_QUALIFIERS) == 1 && league.club_count() == 0
        && league.active() == 0 && (r == 0 || league.table() == 0);
    SCOREBOARD.record(ok, || format!("world_cup_start_new_season({:#x}) returned {}: year {} -> {}, stage {:#x}", this, r, year, league.year(), league.current_stage()));
    r
}
