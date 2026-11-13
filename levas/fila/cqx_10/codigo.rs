// ---------- batch %%LEVA%%: the Paulista's points for a result; the records of the Asian Cup Winners' Cup and of the Merconorte (bra_reg_sp.cpp, asia_cup_winner.cpp, con_merc_cup.cpp) ----------

/// Rule: the São Paulo state league gives the league points for a result (the win and draw points of its rules),
/// passing the query to the League base.
///
/// `BraRegSp::points_for_result(club, fixture)` (the virtual of the points query): League::points_for_result
/// (0x68aad0, thiscall(this, club, fixture), ret 8; this goes through in ecx untouched), whose return it passes on.
/// Original: `BraRegSp::points_for_result` (0x43c4a0, bra_reg_sp.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn bra_reg_sp_points_for_result(this: usize, club: usize, fixture: usize) -> u32 {
    covered!("cqx_10 paulista points");
    let league_points: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x68aad0usize);
    league_points(this, club, fixture) as u32 // cqx10: the League's points
}

/// Comparer for `bra_reg_sp_points_for_result`: side by side (the query only reads); the returned byte (the caller
/// reads al) must be equal.
unsafe extern "thiscall" fn compare_bra_reg_sp_points_for_result(this: usize, club: usize, fixture: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%)(this, club, fixture);
    let c = bra_reg_sp_points_for_result(this, club, fixture);
    SCOREBOARD.record(r as u8 == c as u8, || format!("bra_reg_sp_points_for_result({:#x}, {:#x}) original {} copy {}", club, fixture, r as u8, c as u8));
    r
}

/// Rule: the Asian Cup Winners' Cup records: in the zone knockouts (stage 0), elimination and qualification weigh by
/// round (1 to 80 over the 3 rounds) and the zone winner gets the record of round 0x82 (weight 0x5a); in the final
/// (stage 1) the winner, the runner-up and the third get the final records.
///
/// `AsiaCupWinner::update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the virtual +0x44).
/// Stage (signed byte) below 1, weight = `scale_by_round`(1, 0x50, the fixture's `round_number()`, 3): outcome 0xff or
/// 2, the elimination (`european_cup_add_knocked_out_record`: `round_prefix()`, `round_name()`, the weight); 0, the
/// qualification (`european_cup_add_qualified_record`: `round_prefix()`, the name word +0x6f of the fixture's round in
/// the cup's round table `cup::ROUND_TABLE`, the weight); 1, the qualification (0, 0x82, 0x5a); returns 0. Stage 1:
/// outcome 1 the winner (Staff_Records 0x86a800), 2 the runner-up (0x86b0b0), 3 the third (0x86b710), returns 0;
/// otherwise −1. Above 1: 0. (The high 16 bits of the words the original pushes come from small products of the round:
/// zero.)
/// Original: `AsiaCupWinner::update_staff_records_for_status` (0x40e260, asia_cup_winner.cpp, thiscall, ret 0x18).
unsafe extern "thiscall" fn asia_cup_winner_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    let g = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < 1 {
        match d {
            0xff | 2 => {
                let weight = scale_by_round(1, 0x50, g.round_number(), 3);
                european_cup_add_knocked_out_record(club, comp, g.round_prefix() as u16 as u32, g.round_name() as u16 as u32, weight);
            }
            0 => {
                covered!("cqx_10 cup winners cup zone qualified");
                let weight = scale_by_round(1, 0x50, g.round_number(), 3);
                let name = read_u16(Cup(this).round_table().wrapping_add((g.round_number() as isize * cup_round::SIZE as isize) as usize) + 0x6f) as u32;
                european_cup_add_qualified_record(club, comp, g.round_prefix() as u16 as u32, name, weight);
            }
            1 => european_cup_add_qualified_record(club, comp, 0, 0x82, 0x5a),
            _ => {}
        }
        0
    } else if f == 1 {
        match d {
            1 => cup_record_final_result(0x86a800, club, g, final_),
            2 => cup_record_final_result(0x86b0b0, club, g, final_),
            3 => cup_record_final_result(0x86b710, club, g, final_),
            _ => return u32::MAX,
        }
        0
    } else {
        0
    }
}

// simplification: the records go to the history: they cannot run twice. Only the original; the plan: the return (−1
// in the final for an outcome other than 1 to 3, otherwise 0).
unsafe extern "thiscall" fn compare_asia_cup_winner_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let plan = if f == 1 && !(1..=3).contains(&d) { u32::MAX } else { 0 };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == plan, || format!("asia_cup_winner_update_staff_records_for_status({:#x}, {}, {}) returned {:#x}, plan {:#x}", club, d, f, r, plan));
    r
}

/// Rule: the Merconorte records: in the groups (stages below 3) the qualified club gets the record of round 0x82 and
/// the club going out the failed record (weight 0x28 both); in the knockout (stage 3) elimination and qualification
/// weigh by round (50 to 100 over the 2 rounds) and the winner and runner-up get the final records.
///
/// `ConMercCup::update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the virtual +0x44); returns
/// 0. Stage (signed byte) below 3: outcome 6, the qualification (`european_cup_add_qualified_record`: 0, 0x82, 0x28);
/// 0xfe, the failure (`european_cup_add_failed_to_qualify_record`: 0x41c, 0x28). Stage 3, weight =
/// `scale_by_round`(0x32, 0x64, the fixture's `round_number()`, 2): 0xff, the elimination
/// (`european_cup_add_knocked_out_record`: `round_prefix()`, `round_name()`, the weight); 0, the qualification with
/// `round_prefix()` and the name word +0x6f of the fixture's round in the round table of stage 3 ([`cup::STAGES`][3] +
/// `cup::ROUND_TABLE`; the high 16 bits the original pushes are the stage pointer's); 1, the winner (0x86a800); 2, the
/// runner-up (0x86b0b0). Anything else, and above stage 3: nothing.
/// Original: `ConMercCup::update_staff_records_for_status` (0x4c2940, con_merc_cup.cpp, thiscall, ret 0x18).
unsafe extern "thiscall" fn con_merc_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    let g = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < 3 {
        if d == 6 {
            covered!("cqx_10 merconorte group qualified");
            european_cup_add_qualified_record(club, comp, 0, 0x82, 0x28);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x28);
        }
    } else if f == 3 {
        match d {
            0xff => {
                let weight = scale_by_round(0x32, 0x64, g.round_number(), 2);
                european_cup_add_knocked_out_record(club, comp, g.round_prefix() as u16 as u32, g.round_name() as u16 as u32, weight);
            }
            0 => {
                let weight = scale_by_round(0x32, 0x64, g.round_number(), 2);
                let knockout = read_usize(read_usize(this + cup::STAGES) + 0xc);
                let name = read_u16(read_usize(knockout + cup::ROUND_TABLE).wrapping_add((g.round_number() as isize * cup_round::SIZE as isize) as usize) + 0x6f) as u32;
                european_cup_add_qualified_record(club, comp, g.round_prefix() as u16 as u32, (knockout as u32 & 0xffff_0000) | name, weight);
            }
            1 => cup_record_final_result(0x86a800, club, g, final_),
            2 => cup_record_final_result(0x86b0b0, club, g, final_),
            _ => {}
        }
    }
    0
}

// simplification: the records go to the history: they cannot run twice. Only the original; the plan: return 0.
unsafe extern "thiscall" fn compare_con_merc_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I2%%)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == 0, || format!("con_merc_cup_update_staff_records_for_status({:#x}, {}, {}) returned {}", club, outcome as u8, stage as u8, r));
    r
}
