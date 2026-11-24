// ---------- cx01_13: the Brazilian Champions Cup: the staff records, a club's progress and the fate news (bra_champ_cup.cpp) ----------

/// Writes a club's staff records for its status in the Champions Cup and returns 0.
///
/// `BraChampCup::update_staff_records_for_status(club, outcome, stage, _, fixture, extra)` (thiscall, ret 0x18). In the
/// preliminary (stage 0): qualified (outcome 6) gets `add_qualified_for_round_record` (stage 0, round 0xa, weight 40),
/// out (0xfe) `add_failed_to_qualify_record` (0x41c, weight 40). At the other stages below 2 (signed): qualified gets
/// round 0x82 (weight 40), out gets 0xa (weight 40). At stage 2 (the knockout): out (0xff) the knocked-out record with
/// the fixture's round prefix and name, weighted 50 to 100 by its round number of 2 (`scale_by_round`); through (0) the
/// qualified record with the round prefix and the next round's code in the knockout stage's round table, the same
/// weight; the winner (1) the cup winners record and the club as the CONMEBOL seeding's Champions Cup winner
/// (`set_bra_champ_cup_winner`); the runner-up (2) the runners-up record. Stages above 2 write nothing.
/// Original: `BraChampCup::update_staff_records_for_status` (0x426900, bra_champ_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, fixture: usize, extra: u32) -> u32 {
    let cup = League(this);
    let comp = cup.competition().addr();
    let (outcome, stage) = (outcome as u8, stage as u8 as i8);
    if stage < 2 {
        let (round, out_stage) = if stage == 0 { (0xa, 0x41c) } else { (0x82, 0xa) };
        match outcome {
            6 => {
                covered!("cx01_13 records: qualified");
                european_cup_add_qualified_record(club, comp, 0, round, 0x28);
            }
            0xfe => {
                covered!("cx01_13 records: failed to qualify");
                european_cup_add_failed_to_qualify_record(club, comp, out_stage, 0x28);
            }
            _ => {}
        }
        return 0;
    }
    if stage != 2 {
        return 0;
    }
    let game = Fixture(fixture);
    match outcome {
        0xff => {
            let weight = scale_by_round(0x32, 0x64, game.round_number(), 2);
            // simplification: the round prefix and name go in registers with stale high halves; 0x86c000 reads the words
            european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
        }
        0 => {
            let weight = scale_by_round(0x32, 0x64, game.round_number(), 2);
            let knockout = Cup(read_usize(cup.stages() + 8));
            let next = read_i16(knockout.round_table() + (game.round_number() as isize as usize).wrapping_mul(cup_round::SIZE) + cup_round::SIZE + cup_round::ROUND_CODE);
            european_cup_add_qualified_record(club, comp, game.round_prefix() as u16 as u32, next as u16 as u32, weight);
        }
        1 => {
            covered!("cx01_13 records: champions cup winner");
            african_nations_record_final_result(0x86a800, club, fixture, extra); // Staff_Records::add_cup_winners_record
            set_bra_champ_cup_winner(read_usize(G_CONMEBOL_SEEDING), club);
        }
        2 => african_nations_record_final_result(0x86b0b0, club, fixture, extra), // Staff_Records::add_cup_runners_up_record
        _ => {}
    }
    0
}

/// A club's position code in the Champions Cup statistics from a raw stage code: in the preliminary 1 and 2 give
/// `first_two`, 3 gives 9; in the groups 1 and 2 give `first_two`, 3 gives 5 and 4 gives 7; 0 and the other codes stay.
fn bra_champ_cup_position_code(preliminary: bool, code: u8, first_two: u8) -> u8 {
    match (preliminary, code) {
        (_, 1 | 2) => first_two,
        (true, 3) => 9,
        (false, 3) => 5,
        (false, 4) => 7,
        _ => code,
    }
}

/// Stores a club's current, best and worst position codes in the competition statistics (the club's entry,
/// `exe::comp_stats_find_club_entry`, not checked), turning the raw stage codes into positions: in the preliminary
/// (stage 0xff) the current and best codes 1 and 2 become 1 and the worst 7, a 3 becomes 9; in the groups (stages
/// below 2, signed) the current and best codes 1 and 2 become 1 and the worst 3, a 3 becomes 5 and a 4 becomes 7; from
/// stage 2 on the codes are stored as they are.
/// Original: `BraChampCup::set_club_progress` (0x426c20, bra_champ_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let entry = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    let stage = stage as u8;
    let (current, best, worst) = (current as u8, best as u8, worst as u8);
    let codes = if stage == 0xff {
        covered!("cx01_13 progress in the preliminary");
        (bra_champ_cup_position_code(true, current, 1), bra_champ_cup_position_code(true, best, 1), bra_champ_cup_position_code(true, worst, 7))
    } else if (stage as i8) < 2 {
        covered!("cx01_13 progress in the groups");
        (bra_champ_cup_position_code(false, current, 1), bra_champ_cup_position_code(false, best, 1), bra_champ_cup_position_code(false, worst, 3))
    } else {
        (current, best, worst)
    };
    entry.set_best_position(codes.1);
    entry.set_current_position(codes.0);
    entry.set_worst_position(codes.2);
}

/// Writes the news text of a club's fate in the Champions Cup into Text `text`.
///
/// Same arguments as `comp_build_fate_news_text` (club, outcome, stage, the stage's name id, fixture, next stage, _,
/// verdict, body, text; thiscall, ret 0x28). In the preliminary (stage 0xff) and the groups (other stages below 2):
/// through (outcome 6) the body is the group qualification line (`exe::comp_util_write_group_qualify_news(club, stage,
/// competition, 6, body, text)`) and the headline "through to group phase" (from the preliminary) or "through to semi
/// final" (from the groups) with the club, linked; out (0xfe) the group line with the stage name id instead of the
/// stage (`exe::comp_util_write_group_qualify_news(club, name, competition, −2, body, text)`); other outcomes nothing.
/// From stage 2 on, the shared cup lines: the tie result in the body (`exe::comp_util_write_cup_tie_result_news`), the
/// win or defeat in the headline (`exe::comp_util_write_cup_win_or_defeat_news`).
/// Defect of the original [inference]: the qualified line passes the stage index where the out line passes the stage's
/// name id.
/// Original: `BraChampCup::build_fate_news_text` (0x425fd0, bra_champ_cup.cpp, thiscall, ret 0x28).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_build_fate_news_text(this: usize, club: i32, outcome: u32, stage: u32, name: u32, fixture: usize, next_stage: u32,
    _a7: u32, verdict: u32, body: u32, text: usize) {
    let comp = League(this).competition();
    let s = stage as u8;
    if s == 0xff || (s as i8) < 2 {
        match outcome as u8 {
            6 if body != 0 => {
                covered!("cx01_13 fate: group qualified");
                exe::comp_util_write_group_qualify_news(club as u32, stage, comp, 6, body, text);
            }
            6 => {
                covered!("cx01_13 fate: through headline");
                let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
                let c = club_at(club as isize);
                let cg = c.short_gender_name() as i8 as i32;
                let team = c.addr() + club::SHORT_NAME;
                // "through to group phase" from the preliminary, "through to semi final" from the groups
                lang_sprintf(0xde1f64, if s == 0xff { 0x98a25c } else { 0x98a224 }, cg, cg, team);
                exe::text_set_string(text, 0xde1f64 as *const u8);
                exe::text_add_link(text, team as *const u8, 0x7d5, c);
            }
            0xfe => {
                covered!("cx01_13 fate: group out");
                exe::comp_util_write_group_qualify_news(club as u32, name, comp, -2, body, text);
            }
            _ => {}
        }
        return;
    }
    if body != 0 {
        exe::comp_util_write_cup_tie_result_news(club as u32, fixture as u32, verdict, outcome, next_stage, text);
    } else {
        exe::comp_util_write_cup_win_or_defeat_news(club as u32, fixture as u32, outcome, text);
    }
}

/// Comparer for `bra_champ_cup_update_staff_records_for_status`: the records go to the managers' history and the
/// winner to the CONMEBOL seeding (they cannot run twice); only the original runs and it must answer 0.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, fixture: usize, extra: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I0%%)(this, club, outcome, stage, a4, fixture, extra);
    SCOREBOARD.record(r == 0, || format!("bra_champ_cup_update_staff_records_for_status({:#x}, club {:#x}, {}, stage {}): returned {}", this, club, outcome as u8, stage as u8 as i8, r));
    r
}

/// Comparer for `bra_champ_cup_set_club_progress`: the three position bytes of the club's entry side by side.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    let stats = League(this).comp_stats();
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if entry == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(entry + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst),
        || bra_champ_cup_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("bra_champ_cup_set_club_progress({:#x}, club {:#x}, stage {}, {}, {}, {}): original {:x?} rust {:x?}", this, club, stage as u8 as i8,
        current as u8, best as u8, worst as u8, mo, mr));
}

/// Comparer for `bra_champ_cup_build_fate_news_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_build_fate_news_text(this: usize, club: i32, outcome: u32, stage: u32, name: u32, fixture: usize, next_stage: u32,
    a7: u32, verdict: u32, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(%%I2%%);
    compare_text_writer(text, || o(this, club, outcome, stage, name, fixture, next_stage, a7, verdict, body, text),
        |t| bra_champ_cup_build_fate_news_text(this, club, outcome, stage, name, fixture, next_stage, a7, verdict, body, t),
        || format!("bra_champ_cup_build_fate_news_text({:#x}, club {}, outcome {}, stage {}, body {})", this, club, outcome as u8, stage as u8 as i8, body));
}
