// ---------- rx11_2: the staff records of a nation's fate in the African and CONCACAF World Cup qualifying, and the qualifying sections' season honours (wc_african_cup.cpp, wc_concacaf_cup.cpp) ----------

/// The World Cup's records for a nation that qualifies: the qualification record (`european_cup_add_qualified_record`
/// with the World Cup record of [`KEY_CLUB_COMP_FIFA_WORLD_CUP`], stage 0, round 0 and `weight`), the place in the World
/// Cup (`exe::worldcup_add_qualified_team` on the World Cup object), `*out` = 4 and the national teams' list of
/// qualified nations (NationalTeams::add_qualified_nation(club, record) on [`G_NATIONAL_TEAMS`], thiscall, ret 8). The
/// African play-off writes `*out` after the list, the groups before it: the copy writes it first (nothing reads it in
/// between).
pub(crate) unsafe fn qualify_nation_for_world_cup(club: usize, out: usize, weight: u32) {
    let record = key_comp_record(KEY_CLUB_COMP_FIFA_WORLD_CUP);
    european_cup_add_qualified_record(club, record, 0, 0, weight);
    exe::worldcup_add_qualified_team(comp_object_of_key(KEY_CLUB_COMP_FIFA_WORLD_CUP), club);
    write_u8(out, 4);
    let add: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7779b0usize); // NationalTeams::add_qualified_nation
    add(read_usize(G_NATIONAL_TEAMS), club, record);
}

/// A nation knocked out of a qualifying tie: the knocked-out record (`european_cup_add_knocked_out_record` with the
/// fixture's `round_prefix` and `round_name`, words, and `weight`) and the end of its campaign
/// (`clear_national_team_competition`).
pub(crate) unsafe fn knock_nation_out(club: usize, comp: usize, game: Fixture, weight: u32) {
    // simplification: the two words go in registers with garbage in the high 16 bits; 0x86c000 only uses the words
    european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
    clear_national_team_competition(Club(club));
}

/// rule: in the African qualifying, a first round winner gets the qualification for round 0x14 (weight 25) and a loser
/// the knocked-out record (25) and the end of its campaign; in the groups, a group winner who qualifies for the World
/// Cup (7) gets its record (100) and place, a qualifier for the play-off (6) the qualification for round 0x96 (45), and
/// one going out (−2) the failed-to-qualify record (45) and the end of its campaign; in the play-off, a winner gets the
/// World Cup record (55) and place, a loser the knocked-out record (100) and the end of its campaign
///
/// `this.update_staff_records_for_status(club, outcome, stage, out, game, a6)` (virtual +0x44). The group stages by the
/// format byte (`wc_cup_turnover_consts::AFRICAN_FORMAT`): below stage 5 with format 0, below 4 with 1 or 3, below 6
/// with 2; the play-off: stage 6 with format 2, stage 4 with format 3. Stage −1, outcome 1:
/// `european_cup_add_qualified_record`(club, the competition `league::COMPETITION`, 0, 0x14, 0x19); outcome 2:
/// `knock_nation_out`(weight 0x19). A group, outcome 7: `qualify_nation_for_world_cup`(weight 0x64), returns −1;
/// outcome 6: the qualification (0, 0x96, 0x2d); 0xfe: `european_cup_add_failed_to_qualify_record`(0x14, 0x2d) and
/// `clear_national_team_competition`. The play-off, outcome 1: `qualify_nation_for_world_cup`(0x37), −1; outcome 2:
/// `knock_nation_out`(0x64). Anything else, 0.
/// Original: `WcAfricanCup::update_staff_records_for_status` (0x90fbf0, wc_african_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, _a6: u32) -> u32 {
    let comp = League(this).competition().addr();
    let game = Fixture(game);
    let (outcome, stage) = (outcome as u8, stage as u8 as i8);
    if stage == -1 {
        covered!("rx11_2 african first round records");
        match outcome {
            1 => european_cup_add_qualified_record(club, comp, 0, 0x14, 0x19),
            2 => knock_nation_out(club, comp, game, 0x19),
            _ => {}
        }
        return 0;
    }
    let format = read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT);
    if stage < african_groups_end(format) {
        covered!("rx11_2 african group records");
        match outcome {
            7 => {
                qualify_nation_for_world_cup(club, out, 0x64);
                return u32::MAX;
            }
            6 => european_cup_add_qualified_record(club, comp, 0, 0x96, 0x2d),
            0xfe => {
                european_cup_add_failed_to_qualify_record(club, comp, 0x14, 0x2d);
                clear_national_team_competition(Club(club)); // rx11_2: the African group's eliminated nation ends its campaign
            }
            _ => {}
        }
        return 0;
    }
    if (format == 2 && stage == 6) || (format == 3 && stage == 4) {
        covered!("rx11_2 african play-off records");
        match outcome {
            1 => {
                qualify_nation_for_world_cup(club, out, 0x37); // rx11_2: the African play-off winner to the World Cup
                return u32::MAX;
            }
            2 => knock_nation_out(club, comp, game, 0x64),
            _ => {}
        }
    }
    0
}

/// rule: in the CONCACAF qualifying, in the Caribbean knockout a tie winner gets the qualification for the next round
/// (weight 20), a loser the knocked-out record (20) and the end of its campaign, a group stage qualifier (1) the
/// qualification for round 0x82 (30); in the first Central American stages a qualifier for the semi finals (6 or 1)
/// gets its record (30) and one going out (−2) only the end of its campaign; in the play-off the winner gets the
/// semi-final qualification and the rest the knocked-out record; in the semi-final groups a qualifier (6) gets round
/// 0x96 (75) and one going out the failed-to-qualify record (75); in the final group a qualifier (7) gets the World Cup
/// record (100) and place, one going out the failed-to-qualify record (100)
///
/// `this.update_staff_records_for_status(club, outcome, stage, out, game, a6)` (virtual +0x44), stage a signed byte.
/// Stage −1: outcome 0, `european_cup_add_qualified_record`(club, comp, the fixture's `round_prefix`, the round code of
/// the round after the fixture's (`cup_round::ROUND_CODE` of entry `round_number` + 1 in the cup's `round_table`),
/// 0x14), 0; 0xff, `knock_nation_out`(0x14), 0; 1, the qualification (0x82, 0, 0x1e), 0; any other, the qualification
/// (0, 0x172, 0x1e) and −1. Stage below 2: 6, (0x82, 0, 0x1e); 1, (0, 0x172, 0x1e); 0xfe, `clear_national_team_competition`
/// only; always −1. Stage 2: 1, (0x82, 0, 0x1e); any other, `knock_nation_out`(0x14); 0. Stages 3 to 5: 6, (0x96, 0,
/// 0x4b); 0xfe, `european_cup_add_failed_to_qualify_record`(0x82, 0x4b) and the end of the campaign; 0. Stage 6: 7,
/// `qualify_nation_for_world_cup`(0x64), −1; 0xfe, the failed-to-qualify record (0x96, 0x64) and the end of the
/// campaign, 0. Later stages, 0.
/// Original: `WcConcacafCup::update_staff_records_for_status` (0x915710, wc_concacaf_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, _a6: u32) -> u32 {
    let cup = Cup(this);
    let comp = League(this).competition().addr();
    let game = Fixture(game);
    let (outcome, stage) = (outcome as u8, stage as u8 as i8);
    let qualified = |round_or_stage: (u32, u32), weight: u32| european_cup_add_qualified_record(club, comp, round_or_stage.0, round_or_stage.1, weight);
    match stage {
        -1 => {
            covered!("rx11_2 concacaf knockout records");
            match outcome {
                0 => {
                    let next = cup.round_table() + (game.round_number() as isize as usize).wrapping_mul(cup_round::SIZE) + cup_round::SIZE + cup_round::ROUND_CODE;
                    // simplification: the two words go in registers with garbage in the high 16 bits; 0x86bdd0 only uses the words
                    qualified((game.round_prefix() as u16 as u32, read_u16(next) as u32), 0x14);
                }
                0xff => knock_nation_out(club, comp, game, 0x14),
                1 => qualified((0x82, 0), 0x1e),
                _ => {
                    qualified((0, 0x172), 0x1e);
                    return u32::MAX;
                }
            }
            0
        }
        s if s < 2 => {
            covered!("rx11_2 concacaf first stage records");
            match outcome {
                6 => qualified((0x82, 0), 0x1e),
                1 => qualified((0, 0x172), 0x1e),
                0xfe => clear_national_team_competition(Club(club)), // rx11_2: the CONCACAF first stage loser has no record
                _ => {}
            }
            u32::MAX
        }
        2 => {
            covered!("rx11_2 concacaf play-off records");
            if outcome == 1 {
                qualified((0x82, 0), 0x1e);
            } else {
                knock_nation_out(club, comp, game, 0x14);
            }
            0
        }
        s if s < 6 => {
            covered!("rx11_2 concacaf semi-final group records");
            match outcome {
                6 => qualified((0x96, 0), 0x4b),
                0xfe => {
                    european_cup_add_failed_to_qualify_record(club, comp, 0x82, 0x4b);
                    clear_national_team_competition(Club(club));
                }
                _ => {}
            }
            0
        }
        6 => {
            covered!("rx11_2 concacaf final group records");
            match outcome {
                7 => {
                    qualify_nation_for_world_cup(club, out, 0x64); // rx11_2: the CONCACAF final group qualifier
                    return u32::MAX;
                }
                0xfe => {
                    european_cup_add_failed_to_qualify_record(club, comp, 0x96, 0x64);
                    clear_national_team_competition(Club(club));
                }
                _ => {}
            }
            0
        }
        _ => 0,
    }
}

/// rule: when a World Cup qualifying section closes its season, every national team entered in it leaves it
///
/// NationalTeams::clear_competition_for_comp_clubs(the competition record `league::COMPETITION`) on
/// [`G_NATIONAL_TEAMS`] (`world_cup_knockout_consts::FN_NATIONALTEAMS_CLEAR_COMPETITION_FOR_COMP_CLUBS`, thiscall, ret
/// 4), whose eax it returns. Shared by the African, Asian, CONCACAF, European and Oceania sections (virtual +0x30) and
/// one more class (vtable 0x96a0d4).
/// Original: `WcAfricanCup::record_season_honours` (0x90fbe0, wc_african_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_qualifying_record_season_honours(this: usize) -> u32 {
    covered!("rx11_2 qualifying season honours");
    let clear: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(world_cup_knockout_consts::FN_NATIONALTEAMS_CLEAR_COMPETITION_FOR_COMP_CLUBS);
    clear(read_usize(G_NATIONAL_TEAMS), League(this).competition().addr())
}

/// The plan of a qualifying staff records writer (only the original runs: the records go to the managers' histories
/// and the World Cup place and the end of a campaign do not run twice): the return value and, for a nation that
/// qualifies for the World Cup, `*out` = 4 and the World Cup's entrant count one more when it had room.
pub(crate) unsafe fn compare_qualifying_staff_records(name: &str, club: usize, outcome: u32, stage: u32, out: usize, planned: (u32, bool), original: impl FnOnce() -> u32) -> u32 {
    let (ret, to_world_cup) = planned;
    let world_cup = League(comp_object_of_key(KEY_CLUB_COMP_FIFA_WORLD_CUP));
    let watch = to_world_cup && out != 0 && !world_cup.is_null();
    let n = if watch { world_cup.entrant_count() } else { 0 };
    let room = watch && n < world_cup.entrant_capacity();
    let r = original();
    let same = r == ret && (!watch || (read_u8(out) == 4 && world_cup.entrant_count() == if room { n + 1 } else { n }));
    SCOREBOARD.record(same, || format!("{}({:#x}, outcome {}, stage {}) returned {:#x} plan {:#x}, list {}", name, club, outcome as u8, stage as u8 as i8, r, ret, n));
    r
}

/// The stage where the African groups end by the format byte (`wc_cup_turnover_consts::AFRICAN_FORMAT`): 5 for format
/// 0, 4 for 1 and 3, 6 for 2; any other format has no group stage.
pub(crate) fn african_groups_end(format: u8) -> i8 {
    match format {
        0 => 5,
        1 | 3 => 4,
        2 => 6,
        _ => i8::MIN,
    }
}

/// The planned return of the African records (and whether the nation enters the World Cup).
pub(crate) fn african_staff_records_plan(format: u8, stage: i8, outcome: u8) -> (u32, bool) {
    let group = stage != -1 && stage < african_groups_end(format);
    let play_off = matches!((format, stage), (2, 6) | (3, 4));
    if (group && outcome == 7) || (play_off && outcome == 1) { (u32::MAX, true) } else { (0, false) }
}

/// The planned return of the CONCACAF records (and whether the nation enters the World Cup).
pub(crate) fn concacaf_staff_records_plan(stage: i8, outcome: u8) -> (u32, bool) {
    match stage {
        -1 => (if matches!(outcome, 0 | 0xff | 1) { 0 } else { u32::MAX }, false),
        s if s < 2 => (u32::MAX, false),
        6 if outcome == 7 => (u32::MAX, true),
        _ => (0, false),
    }
}

pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(%%I0%%);
    let plan = african_staff_records_plan(read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT), stage as u8 as i8, outcome as u8);
    compare_qualifying_staff_records("wc_african_cup_update_staff_records_for_status", club, outcome, stage, out, plan, || o(this, club, outcome, stage, out, game, a6))
}

pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(%%I1%%);
    let plan = concacaf_staff_records_plan(stage as u8 as i8, outcome as u8);
    compare_qualifying_staff_records("wc_concacaf_cup_update_staff_records_for_status", club, outcome, stage, out, plan, || o(this, club, outcome, stage, out, game, a6))
}

/// Comparer for `wc_qualifying_record_season_honours`: the national teams leave the competition, which does not run
/// twice (and the original's club list is not freed: a second run would leak another), so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_wc_qualifying_record_season_honours(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this)
}
