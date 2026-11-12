// ---------- batch %%LEVA%%: the Intertoto match (the UEFA coefficient points); the records of the LDV Vans Trophy and of the Oceania Club Championship (intertoto_cup.cpp, eng_auto_cup.cpp, oceania_club_champ.cpp) ----------

/// The comparer's dry run of `intertoto_cup_process_fixture`: the copy gives the coefficient points without the cup
/// base (Cup::process_fixture, which plays out the whole match and cannot run twice).
static INTERTOTO_FIXTURE_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// Rule: each Intertoto match gives UEFA coefficient points to the clubs' nations: 2 to the winner's nation, 1 to each
/// on a draw (by the score after extra time when there was extra time); then the match goes on as in any cup.
///
/// `IntertotoCup::process_fixture(fixture, a2)` (virtual). The score: `home_goals_extra_time()` and
/// `away_goals_extra_time()` when the first is not 0xff, otherwise `home_goals()` and `away_goals()` (signed bytes).
/// Home ahead: 2 points to the home club's nation (UefaSeeding::add_nation_points `add_uefa_coefficient`([club +
/// `club::NATION`], 2): push 2; push nation; mov ecx, [`G_UEFA_SEEDING`] -> thiscall, ret 8); away ahead: 2 to the away
/// club's; a draw: 1 to the home club's, then 1 to the away club's. Then the cup base (`exe::cup_process_fixture`(this,
/// fixture, a2): push a2; push fixture; mov ecx, this -> thiscall, ret 8).
/// Original: `IntertotoCup::process_fixture` (0x633290, intertoto_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn intertoto_cup_process_fixture(this: usize, game: usize, arg: u32) {
    let g = Fixture(game);
    let (ga, gb) = if (g.home_goals_extra_time() as u8) != 0xff {
        ((g.home_goals_extra_time() as u8) as i8, (g.away_goals_extra_time() as u8) as i8)
    } else {
        ((g.home_goals() as u8) as i8, (g.away_goals() as u8) as i8)
    };
    covered!("cqx_11 intertoto coefficient");
    if ga > gb {
        add_uefa_coefficient(g.home_club().nation().addr(), 2); // cqx11: the win's 2 points
    } else if ga < gb {
        add_uefa_coefficient(g.away_club().nation().addr(), 2);
    } else {
        add_uefa_coefficient(g.home_club().nation().addr(), 1);
        add_uefa_coefficient(g.away_club().nation().addr(), 1);
    }
    if INTERTOTO_FIXTURE_DRY_RUN.load(Relaxed) {
        return;
    }
    exe::cup_process_fixture(this, game, arg); // cqx11: the cup base plays the match out
}

/// Comparer for `intertoto_cup_process_fixture` (the `compare_process_fixture` mould of the Champions League): the copy
/// first in a dry run (only the coefficient points, without counting draws), the two nations' coefficient words
/// (`nation_coefficient_word`) read and put back; then the original; the words after the original must equal the
/// copy's. When the stage advanced in the base (its draws add the qualified clubs' coefficient): no comparison. Without
/// a club or a nation: only the original.
unsafe extern "thiscall" fn compare_intertoto_cup_process_fixture(this: usize, game: usize, arg: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%);
    let g = Fixture(game);
    let (a, b) = if !g.is_null() { (g.home_club(), g.away_club()) } else { (Club(0), Club(0)) };
    if a.is_null() || b.is_null() {
        return o(this, game, arg);
    }
    let (ca, cb) = (nation_coefficient_word(a), nation_coefficient_word(b));
    if ca == 0 || cb == 0 {
        return o(this, game, arg);
    }
    let check = [(ca, 2), (cb, 2)];
    let before = crate::comparison::MemorySnapshot::read(&check);
    let stage = Cup(this).current_stage() as i32;
    INTERTOTO_FIXTURE_DRY_RUN.store(true, Relaxed);
    crate::rng::pause_trace(|| crate::rng::without_counting(|| intertoto_cup_process_fixture(this, game, arg)));
    INTERTOTO_FIXTURE_DRY_RUN.store(false, Relaxed);
    let copy = crate::comparison::MemorySnapshot::read(&check);
    before.write_back();
    o(this, game, arg);
    if Cup(this).current_stage() as i32 != stage {
        return; // simplification: the stage advanced and its draw added coefficient: no comparison
    }
    let original = crate::comparison::MemorySnapshot::read(&check);
    SCOREBOARD.record(original == copy, || format!("intertoto_cup_process_fixture(game {:x}) original={:x?} copy={:x?}", game, original, copy));
}

/// Rule: in the LDV Vans Trophy, in the sections (stages up to 0) elimination and qualification weigh by round (1 to 80
/// over the 5 rounds) and the section winner gets the record of reaching the final (round 0x96, weight 0x5a); in the
/// final (stage 1) the winner and runner-up get the final records.
///
/// `EngAutoCup::update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the virtual +0x44); returns
/// 0. Stage (signed byte) up to 0, weight = `scale_by_round`(1, 0x50, the fixture's `round_number()`, 5): outcome 0xff
/// or 2, the elimination (`european_cup_add_knocked_out_record`: `round_prefix()`, `round_name()`, the weight); 0, the
/// qualification (`european_cup_add_qualified_record`: `round_prefix()`, the name word +0x6f of the fixture's round in
/// the cup's round table `cup::ROUND_TABLE`, the weight; the high 16 bits the original pushes come from small products
/// of the round: zero); 1, the qualification for the final (0, 0x96, 0x5a). Stage 1: outcome 1 the winner
/// (Staff_Records 0x86a800), 2 the runner-up (0x86b0b0). Anything else: nothing.
/// Original: `EngAutoCup::update_staff_records_for_status` (0x56bee0, eng_auto_cup.cpp, thiscall, ret 0x18).
unsafe extern "thiscall" fn eng_auto_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    let g = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f <= 0 {
        match d {
            0xff | 2 => {
                covered!("cqx_11 ldv trophy knocked out");
                let weight = scale_by_round(1, 0x50, g.round_number(), 5);
                european_cup_add_knocked_out_record(club, comp, g.round_prefix() as u16 as u32, g.round_name() as u16 as u32, weight);
            }
            0 => {
                let weight = scale_by_round(1, 0x50, g.round_number(), 5);
                let name = read_u16(Cup(this).round_table().wrapping_add((g.round_number() as isize * cup_round::SIZE as isize) as usize) + 0x6f) as u32;
                european_cup_add_qualified_record(club, comp, g.round_prefix() as u16 as u32, name, weight);
            }
            1 => european_cup_add_qualified_record(club, comp, 0, 0x96, 0x5a),
            _ => {}
        }
    } else if f == 1 {
        match d {
            1 => cup_record_final_result(0x86a800, club, g, final_),
            2 => cup_record_final_result(0x86b0b0, club, g, final_),
            _ => {}
        }
    }
    0
}

// simplification: the records go to the history: they cannot run twice. Only the original; the plan: return 0.
unsafe extern "thiscall" fn compare_eng_auto_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == 0, || format!("eng_auto_cup_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}

/// Rule: the Oceania Club Championship records: in the groups (stage below 1) the qualified club gets the record of
/// round 0x82 and the club going out the failed record (weight 0x19 both); in the final (stage 1) the winner and
/// runner-up get the final records.
///
/// `OceaniaClubChamp::update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the virtual +0x44);
/// returns 0. Stage (signed byte) below 1: outcome 6, the qualification (`european_cup_add_qualified_record`: 0, 0x82,
/// 0x19); 0xfe, the failure (`european_cup_add_failed_to_qualify_record`: 0x41c, 0x19). Stage 1: outcome 1 the winner
/// (Staff_Records 0x86a800), 2 the runner-up (0x86b0b0). Anything else: nothing.
/// Original: `OceaniaClubChamp::update_staff_records_for_status` (0x797a90, oceania_club_champ.cpp, thiscall, ret 0x18).
unsafe extern "thiscall" fn oceania_club_champ_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < 1 {
        if d == 6 {
            covered!("cqx_11 oceania group qualified");
            european_cup_add_qualified_record(club, comp, 0, 0x82, 0x19);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x19);
        }
    } else if f == 1 {
        match d {
            1 => cup_record_final_result(0x86a800, club, Fixture(game), final_),
            2 => cup_record_final_result(0x86b0b0, club, Fixture(game), final_),
            _ => {}
        }
    }
    0
}

// simplification: the records go to the history: they cannot run twice. Only the original; the plan: return 0.
unsafe extern "thiscall" fn compare_oceania_club_champ_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I2%%)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == 0, || format!("oceania_club_champ_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}
