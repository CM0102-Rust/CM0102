use super::*;

/// who takes set piece `tipo` (0..4) on `lado` (0..1): the cell (dword) in the club's tactics row (0x8982b0) at +0x601
/// + row × 0x91 + 0x5e + (side + type × 2) × 4. Above [0xae2364] + 0x52d it becomes −1; negative, 0; a person (0x6e
/// bytes at [0xae23bc]) without +0x61 clears the cell; from the club (+0x39), in a national team (`exe::database_is_national_team`) or with
/// link `exe::nationalteams_is_national_or_all_star_team` they are returned; otherwise it clears. Null club, type or side out of range: the original (error
/// box). Compared side by side (the cell)
/// Original: `stored_taker` (0x897990, tactics.cpp (544 bytes, called by the taker choice 0x6ceb90), thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn stored_taker(this: usize, club: usize, kind: u32, side: u32, human: u32) -> usize {
    let (t, l) = (kind as u8 as i8, side as u8 as i8);
    if club == 0 || !(0..=4).contains(&t) || !(0..=1).contains(&l) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> usize>(IDX_STORED_TAKER)(this, club, kind, side, human);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(club, human);
    if k == -1 {
        covered!("taker without row");
        return 0;
    }
    let cell = read_usize(this + tactics_fields::ROWS).wrapping_add(k as usize * 0x91 + tactics_row::SET_PIECE_TAKERS + (l as i32 + t as i32 * 2) as usize * 4) as *mut i32;
    let v = *cell;
    if v > read_i32(G_STAFF_COUNT).wrapping_add(0x52d) {
        covered!("taker beyond the people");
        *cell = -1;
        return 0;
    }
    if v <= -1 {
        covered!("taker empty");
        return 0;
    }
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add(v as usize * staff::SIZE));
    if (person.player().addr() as u32) == 0 {
        covered!("taker without record");
        *cell = -1;
        return 0;
    }
    if person.club_job() == Club(club) {
        covered!("taker of the club");
        return person.addr();
    }
    if exe::database_is_national_team(club) != 0 || exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), club) != 0 {
        covered!("taker from outside the club");
        return person.addr();
    }
    covered!("taker left the club");
    *cell = -1;
    0
}

/// Saves or restores the club's 20 instructions (dwords of the tactics row + 2) in one of 16 slots of 0x5c bytes
/// (+0x30; used count at +0x5f0), the current human's (`[0xb74c26 + [0xb77c7a] × 0x300]`; `[0xae2364]` − id − 1); when
/// saving, the instructions become −1. Compared side by side (the slots and the row).
///
/// Ret 0x10: saves (`restore` 0) or restores (`restore` != 0) the club's 20 instructions (dwords at +0x601 + row × 0x91
/// + 2, the row from 0x8982b0 with `person`) in one of the 16 spaces of 0x5c bytes starting at +0x30 (club +0x30,
/// person +0x34, used +0x38, instructions +0x3c; used count at +0x5f0). The space is that of the human whose turn it is
/// (`[0xb74c26 + [0xb77c7a] × 0x300]`): `[0xae2364]` - his id - 1, in 0..16. Saving (with `use`), marks the space and
/// copies; then the 20 instructions become -1. Restoring with `use`, the space of the same club and person goes back to
/// the table and is freed; without `use`, frees all the spaces of the club and person. Null club: the original routine
/// (error box); row -1: 0; otherwise 1.
/// Original: `store_instructions` (0x896fe0, tactics.cpp (624 bytes, called by the engine step 0x6b98c0 and others), thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn store_instructions(this: usize, club: usize, person: u32, usar: u32, volta: u32) -> u32 {
    if club == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32>(IDX_STORE_INSTRUCTIONS)(this, club, person, usar, volta);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(club, person);
    if k == -1 {
        return 0;
    }
    let instruction = |i: usize| read_usize(this + tactics_fields::ROWS).wrapping_add(k as usize * 0x91 + 2 + 4 * i) as *mut u32;
    let ocupados = (this + tactics_fields::UNDO_COUNT) as *mut u8;
    // the space of the current human, if any
    let espaco = || {
        let human = read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300);
        if human == 0 {
            return None;
        }
        let e = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(human)).wrapping_sub(1);
        if (0..16).contains(&e) { Some(this + e as usize * 0x5c) } else { None }
    };
    if volta == 0 {
        if usar != 0 {
            if let Some(s) = espaco() {
                covered!("instructions stored");
                if read_u32(s + 0x38) == 0 {
                    *ocupados = (*ocupados).wrapping_add(1);
                }
                write_u32(s + 0x30, club as u32);
                write_u32(s + 0x34, person);
                write_u32(s + 0x38, 1);
                for i in 0..20 {
                    write_u32(s + 0x3c + 4 * i, *instruction(i));
                }
            }
        }
        covered!("instructions deleted");
        for i in 0..20 {
            *instruction(i) = u32::MAX;
        }
        return 1;
    }
    if usar != 0 {
        if let Some(s) = espaco() {
            if read_u32(s + 0x38) != 0 && *((s + 0x30) as *mut u32) == club as u32 && *((s + 0x34) as *mut u32) == person {
                covered!("instructions returned");
                for i in 0..20 {
                    *instruction(i) = read_u32(s + 0x3c + 4 * i);
                }
            }
            if read_u32(s + 0x38) != 0 {
                *ocupados = (*ocupados).wrapping_sub(1);
            }
            write_u32(s + 0x38, 0);
        }
        return 1;
    }
    if *ocupados != 0 {
        for e in 0..16 {
            let s = this + e * 0x5c;
            if read_u32(s + 0x38) != 0 && *((s + 0x30) as *mut u32) == club as u32 && *((s + 0x34) as *mut u32) == person {
                covered!("instructions released");
                write_u32(s + 0x38, 0);
                *ocupados = (*ocupados).wrapping_sub(1);
            }
        }
    }
    1
}

/// copies the club's tactics row (0x91 bytes at +0x601, row from 0x8982b0) to the destination. Null club: the original
/// (error box); row -1: 0. Compared side by side
/// Original: `read_tactics_row` (0x8983f0, tactics.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn read_tactics_row(this: usize, club: usize, target: *mut u8, human: u32) -> u32 {
    if club == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, u32) -> u32>(IDX_READ_TACTICS_ROW)(this, club, target, human);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(club, human);
    if k == -1 {
        return 0;
    }
    covered!("tactics line read");
    core::ptr::copy(read_usize(this + tactics_fields::ROWS).wrapping_add(k as usize * 0x91) as *const u8, target, 0x91);
    1
}

/// the reverse, writes the 0x91 bytes into the row (row -1 or beyond the total +0x605: 0). Called by `store_tactic`
/// (`store_tactic`). Compared side by side
/// Original: `write_tactics_row` (0x8984d0, tactics.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_tactics_row(this: usize, club: usize, source: *const u8, human: u32) -> u32 {
    if club == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, *const u8, u32) -> u32>(IDX_WRITE_TACTICS_ROW)(this, club, source, human);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(club, human);
    if k == -1 || k >= read_i32(this + tactics_fields::ROW_COUNT) {
        return 0;
    }
    covered!("tactics line saved");
    core::ptr::copy(source, read_usize(this + tactics_fields::ROWS).wrapping_add(k as usize * 0x91) as *mut u8, 0x91);
    1
}

/// Memory regions of the tactics copy routines: the source/destination (0x91 bytes) and the club's tactics row, if any,
/// plus the RNG global.
pub(crate) unsafe fn tactics_row_regions(this: usize, club: usize, p: usize, human: u32) -> Vec<(usize, usize)> {
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(club, human);
    let mut reg = vec![(G_MOUSE_LEFT_BUTTON_DOWN, 4), (p, 0x91)];
    if k != -1 && k < read_i32(this + tactics_fields::ROW_COUNT) {
        reg.push((read_usize(this + tactics_fields::ROWS).wrapping_add(k as usize * 0x91), 0x91));
    }
    reg
}

/// `0x6beeb0` (match_eng.cpp, called by the engine step `engine_loop`) with the calls that change things outside the engine
/// injected (0x6d6f70 commentary, `injure` injures; and those of the player exit 0x6cda20): the match simulated without
/// highlights, in periods of two rounds (counter +0x8ed0, period code +0x8ed4), including extra time, penalties and
/// ratings.
///
/// Per period: the teams' strength (0x6c0c60) with four pairs limited between half and double side 1, and the side that
/// starts (random(2)).
///
/// Per round, for the side in turn: three attacks (the third only if the side wins the strength draw) with a duel of
/// one active player (+0xc5 + 20 against the other's strength); the second duel (0x6c2880, +0xdd + 15) and the third
/// (+0xad + 15); the wear (0x6c2750, +0xb9 + 20); the other side's cards (sending-off 0x6c2310 mode 4, yellow mode 2,
/// the second yellow sends off; more than 4 sent off and the match is abandoned, 3 × 0); the incident type; the score
/// factor (difference, 1, 0.65 after 60 minutes or 0.75), the contest (0x6c06b0/0x6c07a0 or 0x6c0a70) and, with an
/// advantage, the shot (0x6c23f0, 0x6c0890/0x6c0980) and the goal (0x6c2050; goalkeepers `player_table`; comments 0x1fcc or
/// 0x1fcf, or 0x1fce on a missed penalty).
///
/// At minutes 45, 90, 105 and 120 it writes the scores (+0x45, +0x43, +0x47) and decides extra time, golden goal (+0x38
/// & 0x80) and penalties (+0x49/+0x4a, random(5)); the aggregate (+0x4b/+0x4c).
///
/// Afterwards: the home goal (random(4) with the score level), the injuries of the last side in turn (`injure`), the
/// ratings (friendly −2: adjustment by the club's reputation) and the counters +0xf5e2/+0xf618.
pub(crate) unsafe fn quick_simulation_with(m: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let more = |p: *mut u8| *p = (*p).wrapping_add(1);
    let mais16 = |p: *mut i16| *p = (*p).wrapping_add(1);
    let word = |o: usize, side: usize| read_i16(m + o + 2 * side) as i32;
    let jog = |side: usize, i: i8| Engine(m).player(side as i8, i as usize).addr();
    let add = |p: MatchPlayer, d: i16| {
        let q = (p.addr() + match_player::MATCH_RATING) as *mut i16;
        *q = (*q).wrapping_add(d);
    };
    let pt = || read_usize(m + engine::MATCH);
    let pb = |o: usize| (read_usize(m + engine::MATCH) + o) as *mut i8;
    let goals = |side: usize| read_u8(m + engine::SCORE + side * 0x36) as i32;
    let narrate = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, code: u32, c: u8, side: usize| {
        call(FN_MATCH_EVENTS_ADD_EVENT, read_usize(m + engine::COMMENTATOR), [code, m as u32, c as u32, side as u32, u32::MAX]);
    };
    // active goalkeepers (table 1) of the side concede the goal
    let goleiros = |side: usize| {
        for i in 0..20 {
            let q = jog(side, i);
            if read_i8(q + match_player::POSITION) >= 0 && player_table(q) == 1 {
                more((q + match_player::GOALS_CONCEDED) as *mut u8);
            }
        }
    };
    let mut end = false;
    let mut decides = false;
    let mut local_28 = 0u8;
    let mut side = 0usize;
    write_i16(m + engine::CLOCK, 0);
    write_i16(m + engine::PERIOD_END, 0x1ef);
    let penalties = 'periodo: loop {
        team_strength(m, 0);
        team_strength(m, 1);
        for o in [0x1da, 0x1de, 0x1e2, 0x1e6] {
            let (a, x) = (read_i16(m + o) as f64, read_i16(m + o + 2) as f64);
            let middle = x * read_f64(rdata::F64_0_5);
            if !(a >= middle) {
                write_i16(m + o, ftol(middle) as i16);
            } else if a > x + x {
                write_i16(m + o, ftol(x + x) as i16);
            }
        }
        let c7 = random(2) as u8;
        let mut round = 0u8;
        'rodadas: while read_u8(m + engine::GOLDEN_GOAL_SCORED) == 0 {
            side = if c7 != 0 { round as usize } else { (round == 0) as usize };
            let other = (side == 0) as usize;
            let strong = || random(word(engine::TEAM_STRENGTH_2, side) + word(engine::TEAM_STRENGTH, side)) > random(word(engine::TEAM_STRENGTH, other) + word(engine::TEAM_STRENGTH_2, other));
            // the three attacks (the third by strength)
            for k in 0..3 {
                if k >= 2 && !strong() {
                    continue;
                }
                mais16((m + engine::PASSES + side * 0x36) as *mut i16);
                let mut t = 0u8;
                let i = loop {
                    let i = random(0x14) as i8;
                    t += 1;
                    if t >= 0x78 || read_i8(jog(side, i) + match_player::POSITION) >= 0 {
                        break i;
                    }
                };
                if t >= 0x78 {
                    end = true;
                    break;
                }
                let p = jog(side, i);
                let f = (read_f32(p + match_player::PASSING) as f64 + read_f32(rdata::F32_20) as f64) as f32;
                let r = random((word(engine::TEAM_STRENGTH, other) + 200) / 19);
                if !(r as f64 >= f as f64) {
                    mais16((m + engine::PASSES_COMPLETED + side * 0x36) as *mut i16);
                    add(MatchPlayer(p), 0x28);
                } else {
                    add(MatchPlayer(p), -0x1e);
                }
            }
            if strong() {
                mais16((m + engine::TACKLES + side * 0x36) as *mut i16);
                let c = choose_by_role_b(m, side as u32);
                if c == 0xff {
                    end = true;
                    break 'rodadas;
                }
                let p = jog(side, c as i8);
                let f = (read_f32(p + match_player::TACKLING) as f64 + read_f32(rdata::F32_15) as f64) as f32;
                let r = random(word(engine::TEAM_STRENGTH_2, other) / 5);
                if !(r as f64 >= f as f64) {
                    add(MatchPlayer(p), 0x28);
                    mais16((m + engine::TACKLES_WON + side * 0x36) as *mut i16);
                } else {
                    add(MatchPlayer(p), -10);
                }
            }
            if strong() {
                mais16((m + engine::HEADERS + side * 0x36) as *mut i16);
                let mut t = 0u8;
                let i = loop {
                    let i = random(0x14) as i8;
                    t += 1;
                    if t >= 0x78 {
                        end = true;
                        break 'rodadas;
                    }
                    if read_i8(jog(side, i) + match_player::POSITION) >= 0 {
                        break i;
                    }
                };
                let p = jog(side, i);
                let f = (read_f32(p + match_player::JUMPING) as f64 + read_f32(rdata::F32_15) as f64) as f32;
                let r = random((word(engine::TEAM_STRENGTH + 0x18, other) + 100) / 7);
                if !(r as f64 >= f as f64) {
                    mais16((m + engine::HEADERS_WON + side * 0x36) as *mut i16);
                    add(MatchPlayer(p), 0x14);
                } else {
                    add(MatchPlayer(p), -8);
                }
            }
            if random(10) == 0 {
                more((m + engine::THROW_INS + side * 0x36) as *mut u8);
            }
            let p = jog(side, choose_by_role(m, side as u32) as i8);
            let v = ftol(read_f32(p + match_player::MOVEMENT) as f64 + read_f32(rdata::F32_20) as f64);
            if random(v) < random(2) {
                add(MatchPlayer(p), -0xf);
                more((m + engine::OFFSIDES + side * 0x36) as *mut u8);
            }
            // the other side’s cards, or the type of the move
            local_28 = 0;
            let mut c20 = false;
            if random(word(engine::TEAM_STRENGTH + 0x10, other)) > random(0x640) {
                let mo = m + other * 0x36;
                more((mo + engine::SIDE_FOUL_TALLY) as *mut u8);
                let send_off = |c: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
                    let p = jog(other, c as i8);
                    more((p + match_player::RED_CARDS) as *mut u8);
                    more((mo + engine::RED_CARDS) as *mut u8);
                    player_left_pitch_with(Engine(m), MatchPlayer(p), 1, call);
                    team_strength(m, other as i8);
                    narrate(call, event::SENT_OFF, c, other);
                };
                if random(word(engine::TEAM_STRENGTH + 0x10, other)) > random(0x61a8) {
                    covered!("quick simulation sending off");
                    let c = choose_by_condition(m, other as i8, 4);
                    add(MatchPlayer(jog(other, c as i8)), -0xfa);
                    send_off(c, call);
                } else if random(word(engine::TEAM_STRENGTH + 0x10, other)) > random(0x6a4) {
                    covered!("quick simulation give_yellow_card");
                    let c = choose_by_condition(m, other as i8, 2);
                    let p = jog(other, c as i8);
                    add(MatchPlayer(p), -0x32);
                    more((p + match_player::YELLOW_CARDS) as *mut u8);
                    more((mo + engine::YELLOW_CARDS) as *mut u8);
                    if read_i8(p + match_player::YELLOW_CARDS) == 2 {
                        covered!("quick simulation second give_yellow_card");
                        send_off(c, call);
                    }
                    if read_u8(mo + engine::RED_CARDS) > 4 {
                        // abandonment: 3 x 0 for the side whose turn it is
                        covered!("quick simulation abandonment");
                        end = true;
                        write_u8(m + engine::SCORE + other * 0x36, 0);
                        let g = (m + engine::SCORE + side * 0x36) as *mut u8;
                        if *g <= 3 {
                            *g = 3;
                        }
                        let (x, y) = if side != 0 { (3, 0) } else { (0, 3) };
                        *pb(fixture::HOME_GOALS) = x;
                        *pb(fixture::AWAY_GOALS) = y;
                        if *pb(fixture::HOME_AGGREGATE) != -1 && *pb(fixture::HOME_GOALS) as i32 + *pb(fixture::HOME_AGGREGATE) as i32 == *pb(fixture::AWAY_AGGREGATE) as i32 + *pb(fixture::AWAY_GOALS) as i32 {
                            *pb(fixture::HOME_GOALS) = (*pb(fixture::HOME_GOALS)).wrapping_add(1);
                            more((m + engine::SCORE) as *mut u8);
                        }
                        break 'rodadas;
                    }
                }
                if random(0x1e) != 0 {
                    c20 = true;
                    more((mo + engine::SET_PIECES) as *mut u8);
                } else {
                    local_28 = 1;
                }
            } else if random(word(engine::TEAM_STRENGTH_2, side)) > random(3000) {
                c20 = true;
                more((m + engine::CORNER_KICKS + side * 0x36) as *mut u8);
            }
            // the score factor
            let d = goals(side) - goals(other);
            let goal_difference = if side == 0 { *pb(fixture::HOME_AGGREGATE) as i32 - *pb(fixture::AWAY_AGGREGATE) as i32 } else { *pb(fixture::AWAY_AGGREGATE) as i32 - *pb(fixture::HOME_AGGREGATE) as i32 };
            let x = if goal_difference + d > 3 {
                d as f64
            } else if goals(side) + goals(other) != 0 {
                read_f32(rdata::F32_1) as f64
            } else if read_i16(m + engine::CLOCK) > 0x3c {
                read_f32(rdata::F32_0_6499999762) as f64
            } else {
                read_f32(rdata::F32_0_75) as f64
            };
            let (v, repartir_outro): (f64, unsafe extern "thiscall" fn(usize, i8, i16)) = if c20 {
                (word(engine::TEAM_STRENGTH + 0x18, other) as f64 * x * read_f64(rdata::F64_6_1), split_c)
            } else {
                (word(engine::TEAM_STRENGTH, other) as f64 * x * read_f64(rdata::F64_4_8), split_b)
            };
            let r1 = random(ftol(v));
            let l24 = random(word(engine::TEAM_STRENGTH_2, side)).wrapping_sub(r1) as i16;
            if l24 > 0 {
                split_a(m, side as i8, 0x1c);
                repartir_outro(m, other as i8, -0x14);
            } else {
                split_a(m, side as i8, -6);
                repartir_outro(m, other as i8, 0x28);
            }
            // o chute
            if l24 > 0 && random(goals(1) + goals(0)) <= 3 {
                let ml = m + side * 0x36;
                if local_28 != 0 {
                    more((ml + engine::SIDE_SUMMARY) as *mut u8);
                }
                more((ml + engine::SHOTS) as *mut u8);
                let c8 = choose_by_role_e(m, side as u32, c20 as u32) as i8;
                let a = if c20 { random(word(engine::TEAM_STRENGTH + 0x14, side)) } else { random(word(engine::TEAM_STRENGTH + 0x8, side) * (local_28 as i32 + 1)) };
                let us = a as i16 as i32;
                if us + 0xa5 > random(0x172) + (0x168 - l24 as i32) {
                    more((ml + engine::SHOTS_ON_TARGET) as *mut u8);
                    if c20 {
                        split_d(m, side as i8, a as i16);
                    } else {
                        split_e(m, side as i8, a as i16);
                    }
                    if us + 0x2d > random((7 - local_28 as i32 * 3) * word(engine::TEAM_STRENGTH + 0xc, other)) {
                        covered!("quick simulation goal");
                        let c9 = choose_by_role_c(m, side as u32, local_28 as u32);
                        let p = jog(side, c9 as i8);
                        more((p + match_player::GOALS) as *mut u8);
                        add(MatchPlayer(p), 0x1db);
                        goleiros(other);
                        if local_28 != 0 {
                            covered!("quick simulation penalty goal");
                            narrate(call, event::SCORED_PENALTY, c9, side);
                            more((ml + engine::PENALTY_GOALS) as *mut u8);
                            more((p + match_player::PENALTIES_TAKEN) as *mut u8);
                            more((p + match_player::PENALTIES_SCORED) as *mut u8);
                        } else {
                            narrate(call, event::GOAL_ONE, c9, side);
                            if c20 {
                                let o = match random(4) {
                                    0 | 1 => 0xf5c3,
                                    2 => 0xf5c5,
                                    _ => 0xf5c4,
                                };
                                more((ml + o) as *mut u8);
                            }
                        }
                        if c8 > -1 {
                            let q = jog(side, c8);
                            more((q + match_player::ASSISTS) as *mut u8);
                            add(MatchPlayer(q), 0x15e);
                        }
                        more((m + engine::GOALS_CONCEDED + other * 0x36) as *mut u8);
                        more((ml + engine::SCORE) as *mut u8);
                        for (s, delta) in [(side, 0x4b), (other, -0x43)] {
                            for i in 0..20 {
                                let q = jog(s, i);
                                if read_i8(q + match_player::POSITION) >= 0 {
                                    add(MatchPlayer(q), delta);
                                }
                            }
                        }
                        if decides {
                            covered!("quick simulation golden goal");
                            write_u8(m + engine::GOLDEN_GOAL_SCORED, 1);
                            break 'rodadas;
                        }
                    } else {
                        split_f(m, other as i8, 0xe1);
                        if local_28 != 0 {
                            covered!("quick simulation penalty missed");
                            let c = choose_by_role_c(m, side as u32, local_28 as u32);
                            narrate(call, event::MISSED_PENALTY, c, side);
                            let p = jog(side, c as i8);
                            more((p + match_player::PENALTIES_TAKEN) as *mut u8);
                            add(MatchPlayer(p), -500);
                        }
                    }
                }
            }
            round += 1;
            if round >= 2 {
                break;
            }
        }
        // the end of the period
        mais16((m + engine::CLOCK) as *mut i16);
        let t = read_i16(m + engine::CLOCK);
        let w38 = read_u16(pt() + fixture::TIE_RULES);
        let same_in_aggregate = |a: usize, x: usize| *pb(a) as i32 + *pb(fixture::HOME_AGGREGATE) as i32 == *pb(fixture::AWAY_AGGREGATE) as i32 + *pb(x) as i32;
        if end {
            let (a, x) = if t <= 0x5a { (0x43, 0x44) } else { (0x47, 0x48) };
            *pb(a) = goals(0) as i8;
            *pb(x) = goals(1) as i8;
            let desempata = if *pb(fixture::HOME_AGGREGATE) != -1 { same_in_aggregate(0x43, 0x44) } else { *pb(fixture::HOME_GOALS) == *pb(fixture::AWAY_GOALS) && w38 & 0xa3 != 0 };
            if desempata {
                *pb(a) = (*pb(a)).wrapping_add(1);
            }
            break 'periodo false;
        }
        if read_u8(m + engine::GOLDEN_GOAL_SCORED) != 0 {
            *pb(fixture::HOME_GOALS_EXTRA_TIME) = goals(0) as i8;
            *pb(fixture::AWAY_GOALS_EXTRA_TIME) = goals(1) as i8;
            break 'periodo false;
        }
        match t {
            0x2d => {
                write_i16(m + engine::PERIOD_END, 0x3de);
                *pb(fixture::HOME_GOALS_HALF_TIME) = goals(0) as i8;
                *pb(fixture::AWAY_GOALS_HALF_TIME) = goals(1) as i8;
            }
            0x5a => {
                *pb(fixture::HOME_GOALS) = goals(0) as i8;
                *pb(fixture::AWAY_GOALS) = goals(1) as i8;
                write_i16(m + engine::PERIOD_END, 0x483);
                let aggregate = *pb(fixture::HOME_AGGREGATE) != -1;
                if (aggregate && !same_in_aggregate(0x43, 0x44)) || (!aggregate && *pb(fixture::HOME_GOALS) != *pb(fixture::AWAY_GOALS)) {
                    break 'periodo false;
                }
                if w38 & fixture_tie_rules::GOLDEN_GOAL != 0 {
                    covered!("quick simulation extra time with golden goal");
                    decides = true;
                    continue 'periodo;
                }
                if w38 & fixture_tie_rules::EXTRA_TIME != 0 {
                    covered!("quick simulation extra time");
                    continue 'periodo;
                }
                if aggregate && w38 & 4 == 0 && *pb(fixture::AWAY_GOALS) != *pb(fixture::HOME_AGGREGATE) && w38 & 0x4000 == 0 {
                    break 'periodo false;
                }
                break 'periodo w38 & 0x21 != 0;
            }
            0x69 => *((m + engine::PERIOD_END) as *mut i16) = 0x528,
            0x78 => {
                *pb(fixture::HOME_GOALS_EXTRA_TIME) = goals(0) as i8;
                *pb(fixture::AWAY_GOALS_EXTRA_TIME) = goals(1) as i8;
                if *pb(fixture::HOME_AGGREGATE) != -1 {
                    if !same_in_aggregate(0x47, 0x48) || (w38 & 0x4000 != 0 && *pb(fixture::AWAY_GOALS_EXTRA_TIME) != *pb(fixture::HOME_AGGREGATE)) {
                        break 'periodo false;
                    }
                } else if *pb(fixture::HOME_GOALS_EXTRA_TIME) != *pb(fixture::AWAY_GOALS_EXTRA_TIME) {
                    break 'periodo false;
                }
                break 'periodo w38 & 0x21 != 0;
            }
            _ => {}
        }
    };
    if penalties {
        covered!("quick simulation penalties");
        random(5);
        random(5);
        *pb(fixture::HOME_PENALTIES) = random(5) as i8;
        random(5);
        random(5);
        *pb(fixture::AWAY_PENALTIES) = random(5) as i8;
        if *pb(fixture::HOME_PENALTIES) == *pb(fixture::AWAY_PENALTIES) {
            let o = if random(2) != 0 { 0x49 } else { 0x4a };
            *pb(o) = (*pb(o)).wrapping_add(1);
        }
    }
    // o agregado
    if *pb(fixture::HOME_AGGREGATE) != -1 {
        let (a, x) = if *pb(fixture::HOME_GOALS_EXTRA_TIME) != -1 { (0x47, 0x48) } else { (0x43, 0x44) };
        *pb(fixture::HOME_AGGREGATE) = (*pb(fixture::HOME_AGGREGATE)).wrapping_add(*pb(a));
        *pb(fixture::AWAY_AGGREGATE) = (*pb(fixture::AWAY_AGGREGATE)).wrapping_add(*pb(x));
    }
    // the home goal
    let c1 = Club(read_usize((pt()) + fixture::HOME_CLUB));
    if !c1.is_null() && read_usize((pt()) + fixture::COMPETITION) == c1.division().addr() && (*pb(fixture::HOME_GOALS) as i32 + *pb(fixture::AWAY_GOALS) as i32) < 5 && *pb(fixture::HOME_GOALS) == *pb(fixture::AWAY_GOALS) && random(4) == 0 {
        let (r0, r1) = (c1.reputation() as i32, read_i16(read_usize((pt()) + fixture::AWAY_CLUB) + club::REPUTATION) as i32);
        let s = if r0 > r1 { Some(0) } else if r0 + 0xfa < r1 { Some(1) } else { None };
        if let Some(s) = s {
            covered!("quick simulation home goal");
            more((m + engine::SHOTS + s * 0x36) as *mut u8);
            let c = choose_by_role_c(m, s as u32, local_28 as u32);
            let p = jog(s, c as i8);
            more((p + match_player::GOALS) as *mut u8);
            add(MatchPlayer(p), 0x1db);
            goleiros(1 - s);
            narrate(call, event::GOAL_ONE, c, s);
        }
    }
    // the injuries (two passes over the last side of the turn)
    if read_i32(pt()) != -2 {
        for _ in 0..2 {
            for i in 0..20 {
                let p = jog(side, i);
                let person = PlayerSheet(read_usize(p + match_player::PLAYER));
                if read_i8(p + match_player::POSITION) < 0 || person.is_null() {
                    continue;
                }
                let f = read_f32(p + match_player::STAMINA);
                let mut cl = (person.injury_proneness() as u8).wrapping_mul(2).wrapping_add(5);
                if !(f >= read_f32(rdata::F32_6750)) {
                    cl = cl.wrapping_add(0xf);
                } else if !(f >= read_f32(rdata::F32_8000)) {
                    cl = cl.wrapping_add(5);
                }
                if random(cl as i8 as i32) < random(400) {
                    continue;
                }
                let comp = read_usize((pt()) + fixture::COMPETITION);
                let league = read_usize(G_FRIENDLY + 0);
                let ceiling = if comp == 0 || (league != 0 && is_friendly(league, comp) != 0) { 0x5b } else { 0x66 };
                let g = random(ceiling) as u8;
                let kind = match random(10) {
                    0 => 1,
                    1..=3 => 3,
                    4..=6 => 4,
                    7..=9 => 5,
                    _ => continue,
                };
                covered!("quick simulation injury");
                call(FN_CM3_MATCH_ENGINE_INJURE_PLAYER, m, [kind, g as u32, p as u32, 0, 0]);
            }
        }
    }
    // the ratings
    for s in 0..2 {
        for i in 0..20i8 {
            let p = jog(s, i);
            let q = (p + match_player::MATCH_RATING) as *mut i16;
            if read_i32(pt()) == -2 && i < 11 {
                covered!("quick simulation friendly rating");
                let club = Club(if s != 0 { read_usize((pt()) + fixture::AWAY_CLUB) } else { read_usize((pt()) + fixture::HOME_CLUB) });
                let a = (read_i16(p + match_player::EFFECTIVE_ABILITY) as f64 - read_f64(rdata::F64_40)) * read_f64(rdata::F64_50);
                let r = club.reputation() as f64 * read_f64(rdata::F64_1_6) - read_f64(rdata::F64_6000);
                let v = ftol(((a - r) * read_f64(rdata::F64_2ENEG_05) + read_f64(rdata::F64_1)) * *q as f64) as i16;
                *q = v.clamp(0xdac, 0x2ee0);
            }
            let n = (ftol((*q as i32 + 0x1770) as f64 * read_f64(rdata::F64_0_0004) + read_f64(rdata::F64_1_83)) as i8).clamp(1, 10);
            write_i8(p + match_player::MATCH_GRADE, n);
            write_u32(p + match_player::STAMINA_CEILING, read_u32(p + match_player::STAMINA));
        }
    }
    for (a, x) in [(0xf5e2, 0xf5e4), (0xf618, 0xf61a)] {
        if read_i16(m + a) < 10 {
            covered!("quick simulation counter reset");
            write_i16(m + a, 0x14);
            write_i16(m + x, (random(6) + 12) as i16);
        } else if read_i16(m + a) == read_i16(m + x) {
            covered!("quick simulation counter equal");
            let r = random(3);
            write_i16(m + x, read_i16(m + x).wrapping_add((-1i32).wrapping_sub(r) as i16));
        }
    }
}

/// Club's tactics row index (0x8982b0) and its start in the table at +0x601 (0x91 bytes per row), or None for row −1.
pub(crate) unsafe fn tactics_row_of_club(this: usize, club: usize, human: u32) -> Option<usize> {
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let l = row(club, human);
    (l != -1).then(|| read_usize(this + tactics_fields::ROWS).wrapping_add((l as usize).wrapping_mul(0x91)))
}

/// Returns the squad slot (0..19) of the club's tactics row whose person id matches the id stored at +0x52 of the row
/// (the designated player, captain per the function map [inference]); none: -1 with `menos_um`, else 0; no row: 0. A
/// null club calls the original (error box).
pub(crate) unsafe extern "thiscall" fn marked_slot(this: usize, club: usize, human: u32, minus_one: u32) -> i8 {
    if club == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i8>(IDX_MARKED_SLOT)(this, club, human, minus_one);
    }
    let Some(linha) = tactics_row_of_club(this, club, human) else { return 0 };
    let persons = read_usize(G_STAFF_TABLE);
    for vaga in 0..20usize {
        let i = read_i32(linha + tactics_row::SELECTION + 4 * vaga);
        if i != -1 && *(persons.wrapping_add((i as usize).wrapping_mul(110)) as *const i32) == read_i32(linha + tactics_row::CAPTAIN) {
            return vaga as i8;
        }
    }
    -((minus_one != 0) as i8)
}

/// Sets the captain in the club's tactic: stores in the tactics row (+0x52) the person index of slot `vaga` (0..19).
/// Returns 1 if the slot was filled, 0 if empty (-1) or the row is missing; a null club or bad slot goes to the
/// original (error box).
pub(crate) unsafe extern "thiscall" fn mark_slot(this: usize, club: usize, slot: i8, human: u32) -> u32 {
    if club == 0 || !(0..20).contains(&slot) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32>(IDX_MARK_SLOT)(this, club, slot, human);
    }
    let Some(linha) = tactics_row_of_club(this, club, human) else { return 0 };
    let i = read_i32(linha + tactics_row::SELECTION + 4 * slot as usize);
    if i == -1 {
        return 0;
    }
    write_i32(linha + tactics_row::CAPTAIN, i);
    1
}

/// Returns the person (staff table 0xae23bc, 110 bytes each) referenced by the club's tactics row at +0x56 + 4 x `qual`
/// (0 or 1); 0 when the index is negative or the club has no row. Null club or `qual` out of range go to the original
/// (error box).
pub(crate) unsafe extern "thiscall" fn row_person(this: usize, club: usize, which: i8, human: u32) -> usize {
    if club == 0 || !(0..2).contains(&which) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> usize>(IDX_ROW_PERSON)(this, club, which, human);
    }
    let Some(linha) = tactics_row_of_club(this, club, human) else { return 0 };
    let i = read_i32(linha + tactics_row::PLAYMAKER + 4 * which as usize);
    if i < 0 {
        return 0;
    }
    read_usize(G_STAFF_TABLE).wrapping_add(staff::SIZE * i as usize)
}

/// Returns the club's row in the tactics table: for a human-managed club with `humano` set, -1 for a second national
/// team, else the human manager's row after the clubs (staff count - manager id - 1, +15 for a national team; out of
/// range: the original); in other cases the club id.
pub(crate) unsafe extern "stdcall" fn tactics_row(club: usize, human: u32) -> i32 {
    let club = Club(club);
    if exe::human_manager_is_human_club(club, 1, (1) as u32, (0) as u32, 0) != 0 && human != 0 {
        if exe::club_is_national_team(club) != 0 {
            return -1;
        }
        let manager = club.manager().addr();
        if manager == 0 {
            return original_fn::<unsafe extern "stdcall" fn(usize, u32) -> i32>(IDX_TACTICS_ROW)(club.addr(), human);
        }
        let clubs = read_i32(G_CLUB_COUNT);
        let base = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(manager));
        let i = base.wrapping_add(if exe::database_is_national_team(club) != 0 { 0xf } else { -1 }).wrapping_add(clubs);
        if i < 0 || i >= clubs.wrapping_add(0x20) {
            return original_fn::<unsafe extern "stdcall" fn(usize, u32) -> i32>(IDX_TACTICS_ROW)(club.addr(), human);
        }
        return i;
    }
    read_i32(club.addr())
}

/// Returns byte `pos` (0..10) of the club's tactics row (index from 0x8982b0, for the human manager when `humano`) in
/// the table at +0x601 (0x91 bytes per row, from +0x86); row -1: 0. Null club or position out of range: falls back to
/// the original (error box).
pub(crate) unsafe extern "thiscall" fn tactics_byte(this: usize, club: usize, pos: i8, human: u32) -> u8 {
    if club == 0 || !(0..=10).contains(&pos) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u8>(IDX_TACTICS_BYTE)(this, club, pos, human);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let l = row(club, human);
    if l == -1 {
        return 0;
    }
    let table = read_usize(this + tactics_fields::ROWS);
    read_u8(table.wrapping_add((l as usize).wrapping_mul(0x91)) + tactics_row::SHOOTOUT_ORDER + pos as usize)
}

/// Row of the tactic statistics table ([this], 0x28 bytes per tactic) used by the club: word +0 of the club's 0x91-byte
/// tactics row.
pub(crate) unsafe fn tactic_stats_row(this: usize, club: Club) -> usize {
    let id = read_usize(club.addr());
    let k = read_i16(read_usize(this + tactics_fields::ROWS).wrapping_add(id.wrapping_mul(0x91))) as i32;
    read_usize(this).wrapping_add((k as isize * 0x28) as usize)
}

/// Returns x as f64, or 1.0 when x < 1.
pub(crate) fn at_least_one(x: i16) -> f64 {
    if x < 1 { 1.0 } else { x as f64 }
}

/// At the end of a match with no human, adds to each club's tactic row (0x28 bytes) the games, wins, draws, losses,
/// goals and points (3 per win, 1 per draw) and redoes the three double ratios (+0x10/+0x18/+0x20) that the AI
/// formation choice reads.
///
/// Ret 4: `Tactics::record_tactic_result(match)`, from the end of each match in the engine loop (0x6ba9a0, with the
/// aggregate score; fixture +0 ≠ −2). Null match: the box at line 0x1f9 and 0. Home side (+0x1c) or away side (+0x20)
/// managed by a human (`exe::human_manager_is_human_club` with 0, 1, 0, 0; cdecl 5): 0. Otherwise, for each side (null club: the box at line
/// 0x20a and continues), the match record for the club (`fill_match_record_from_fixture`, into a local record that starts with the default date
/// `exe::date_date_54a110`) is added to the club's tactic row (tactic_stats_row): +2 matches, +4 wins (+3 points in +0xe), +6 draws
/// (+1 point), +8 defeats, +0xa goals for (+0x19 of the record, signed), +0xc against (+0x1a); with p = matches, a =
/// for, b = against and c = points (each at least 1), q = c/p, g = a/p, h = g·q², r = p/b: +0x10 = r·h·g, +0x18 =
/// r·g·q²·q, +0x20 = r·r·h (the order of the x87 operations). The computer's formation choice (`exe::match_man_choose_computer_tactic`) reads +2 and
/// the three ratios. Returns 1.
/// Original: `Tactics::record_tactic_result` (0x896290, tactics.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn record_tactic_result(this: usize, game: usize) -> u32 {
    let game = Fixture(game);
    if game.is_null() {
        show_error_at(0xa8b908, 0x1f9);
        return 0;
    }
    for k in 0..2 {
        let c = read_usize(game.addr() + fixture::HOME_CLUB + 4 * k);
        if c != 0 && exe::human_manager_is_human_club(c, 0, 1, 0, 0) != 0 {
            return 0;
        }
    }
    let mut reg = [0u8; 0x20];
    let pr = reg.as_mut_ptr() as usize;
    exe::date_date_54a110(pr);
    for k in 0..2 {
        let c = read_usize(game.addr() + fixture::HOME_CLUB + 4 * k);
        if c == 0 {
            show_error_at(0xa8b908, 0x20a);
            continue;
        }
        let l = tactic_stats_row(this, Club(c));
        exe::fill_match_record_from_fixture(game, c, pr);
        covered!("cpz_5 tactic row");
        write_i16(l + 2, read_i16(l + 2).wrapping_add(1));
        let (pro, contra) = (reg[0x19] as i8, reg[0x1a] as i8);
        if pro > contra {
            write_i16(l + 4, read_i16(l + 4).wrapping_add(1));
            write_i16(l + tactic_stats::POINTS, read_i16(l + tactic_stats::POINTS).wrapping_add(3)); // cpz5: a win is worth 3 points
        } else if pro == contra {
            write_i16(l + 6, read_i16(l + 6).wrapping_add(1));
            write_i16(l + tactic_stats::POINTS, read_i16(l + tactic_stats::POINTS).wrapping_add(1));
        } else {
            write_i16(l + 8, read_i16(l + 8).wrapping_add(1));
        }
        write_i16(l + tactic_stats::GOALS_FOR, read_i16(l + tactic_stats::GOALS_FOR).wrapping_add(pro as i16)); // cpz5: the goals for
        write_i16(l + tactic_stats::GOALS_AGAINST, read_i16(l + tactic_stats::GOALS_AGAINST).wrapping_add(contra as i16));
        let a = at_least_one(read_i16(l + tactic_stats::GOALS_FOR));
        let b = at_least_one(read_i16(l + tactic_stats::GOALS_AGAINST));
        let c = at_least_one(read_i16(l + tactic_stats::POINTS));
        let p = at_least_one(read_i16(l + 2));
        let q = c / p;
        let q2 = q * q;
        let g = a / p;
        let h = g * q2;
        let r = p / b;
        core::ptr::write_unaligned((l + tactic_stats::ALL_OUT_ATTACK_SCORE) as *mut f64 , r * h * g);
        core::ptr::write_unaligned((l + tactic_stats::ATTACK_SCORE) as *mut f64 , r * g * q2 * q); // cpz5: the ratio of the normal stance
        core::ptr::write_unaligned((l + tactic_stats::DEFENSIVE_SCORE) as *mut f64 , r * r * h);
    }
    1
}

/// Returns the captain stored (+0x52) in the club and team row of the tactics table; no row, 0.
///
/// Ret 8: `Tactics::get_captain(club, team)`, from the tactic screens (0x8a7440). Null club: the box at line 0x527 and
/// 0. The club's exe::tactics_get_club_tactics_index(0x8982b0, thiscall(this, club, team), ret 8; −1: 0); returns dword +0x52 of the row's 0x91-byte
/// record in the tactics table (+0x601).
/// Original: `Tactics::get_captain` (0x8976a0, tactics.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_captain(this: usize, club: usize, team: u32) -> u32 {
    if club == 0 {
        show_error_at(0xa8b908, 0x527);
        return 0;
    }
    let k = exe::tactics_get_club_tactics_index(this, club, team);
    if k == -1 {
        return 0;
    }
    read_usize(read_usize(this + tactics_fields::ROWS).wrapping_add((k as isize * 0x91) as usize) + tactics_row::CAPTAIN) as u32
}

/// Writes the captain (+0x52) in the club and team row of the tactics table and returns 1; no row, 0.
///
/// Ret 0xc: `Tactics::set_captain(club, &person, team)`, when applying the changes of the tactic screen (0x8a80d0).
/// Null club: the box at line 0x579; null pointer: that of 0x581; 0. The club's row (0x8982b0; −1: 0); writes the
/// pointed dword into +0x52 of the row's record (+0x601) and returns 1.
/// Original: `Tactics::set_captain` (0x897880, tactics.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn set_captain(this: usize, club: usize, person: usize, team: u32) -> u32 {
    if club == 0 {
        show_error_at(0xa8b908, 0x579);
        return 0;
    }
    if person == 0 {
        show_error_at(0xa8b908, 0x581);
        return 0;
    }
    let k = exe::tactics_get_club_tactics_index(this, club, team);
    if k == -1 {
        return 0;
    }
    let p = read_usize(this + tactics_fields::ROWS).wrapping_add((k as isize * 0x91) as usize) + tactics_row::CAPTAIN;
    write_u32(p, read_u32(person));
    1
}

/// Club's row in the tactics table (FN_TACTICS_GET_CLUB_TACTICS_INDEX, stdcall(club, human)); −1 without one.
pub(crate) unsafe fn club_tactics_index(club: usize, human: u32) -> i32 {
    let f: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    f(club, human)
}

/// Start of tactics row l (0x91 bytes each, from [this+0x601]).
pub(crate) unsafe fn tactics_row_start(this: usize, l: i32) -> usize {
    read_usize(this + tactics_fields::ROWS).wrapping_add(l.wrapping_mul(0x91) as usize)
}

/// Clears the undo copies (16 × 0x5c bytes at this+0x30) of the club and team and decrements the count at +0x5f0.
pub(crate) unsafe fn drop_undo_copies(this: usize, club: usize, team: u32) {
    if read_u8(this + tactics_fields::UNDO_COUNT) == 0 {
        return;
    }
    for k in 0..0x10usize {
        let e = this + tactics_fields::UNDO_SLOTS + k * 0x5c;
        if read_usize(e + undo_slot::IN_USE) != 0 && read_usize(e) == club && read_usize(e + undo_slot::TEAM) as u32 == team {
            write_u32(e + undo_slot::IN_USE, 0);
            write_u8(this + tactics_fields::UNDO_COUNT, read_u8(this + tactics_fields::UNDO_COUNT).wrapping_sub(1));
        }
    }
}

/// Memory regions of the club's lineup routines: its tactics row (if any) and the undo copies with their count.
pub(crate) unsafe fn lineup_regions(this: usize, club: usize, team: u32) -> Vec<(usize, usize)> {
    let mut reg = vec![(this + tactics_fields::UNDO_SLOTS, 0x5c1)];
    let l = club_tactics_index(club, team);
    if l != -1 {
        reg.push((tactics_row_start(this, l), 0x91));
    }
    reg
}

/// Removes a player from the selection: empties (−1) the selection slots holding the person; with roles, also the two
/// extra slots and the set-piece takers; the undo copies of the club and team are dropped; 1 when removed from a slot
/// or set piece.
///
/// Ret 0x10: `Tactics::remove_player_from_selection(club, person, team, roles)`, from the contracts (change of place in
/// the squad, loan and its end), from leaving the squad, from the automatic line-up (`select_automatically`), from the national
/// sides, from retirement and from the transfers. No tactic row (0x8982b0): 0. The 20 line-up slots (+2 of the row,
/// ids) with the person (id at +0) become −1. With `roles`: the two extra slots (+0x56, +0x5a: `exe::tactics_get_playmaker` and `extra_tactic_slot`)
/// and the takers of the five set pieces on both sides (`stored_taker` and `set_set_piece_taker`) that are that person are emptied. The
/// undo copies of the club and team are dropped. Returns 1 if it removed the person from a slot or set piece. Null club
/// or person: the original routine ("Error" box, 0).
/// Original: `Tactics::remove_player_from_selection` (0x896c60, tactics.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn remove_player_from_selection(this: usize, club: usize, person: usize, team: u32, papeis: u32) -> u32 {
    if club == 0 || person == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(IDX_REMOVE_PLAYER_FROM_SELECTION)(this, club, person, team, papeis); // message box: original only
    }
    let l = club_tactics_index(club, team);
    if l == -1 {
        return 0;
    }
    let mut found = 0;
    for k in 0..20usize { // cpx9: the line-up slots
        let slot = tactics_row_start(this, l) + 2 + 4 * k;
        if read_i32(slot) == read_i32(person) {
            covered!(if k < 11 { "cpx_9 tirou titular" } else { "cpx_9 tirou reserva" });
            write_u32(slot, u32::MAX); // cpx9: the place stays empty
            found = 1;
        }
    }
    if papeis != 0 {
        // simplification: the original pushes the type and the side with leftovers in the high bytes (the row offset and the stack);
        // the calls read only the low byte, and the copy passes clean values
        for qual in 0..2u32 {
            if exe::tactics_get_playmaker(this, club, (qual) as i8, team) == person {
                exe::tactics_set_playmaker(this, club, qual, 0, team);
            }
        }
        for tipo in 0..5u32 {
            for lado in 0..2u32 {
                if exe::tactics_get_set_piece_taker(this, club, (tipo) as i32, (lado) as i32, (team) as i32) == person {
                    covered!("cpx_9 taker removed");
                    exe::tactics_set_set_piece_taker(this, club, tipo, lado, 0, team);
                    found = 1;
                }
            }
        }
    }
    drop_undo_copies(this, club, team);
    found
}

/// Swaps the ids of two selection slots (0..19) and drops the undo copies of the club and team.
///
/// Ret 0x10: `Tactics::swap_selection_slots(club, a, b, team)`, from the squad screen (0x46835e). Slots `a` and `b`
/// (signed bytes, 0..19): swaps the ids of the two line-up slots (+2 + 4 × slot of the tactic row, `exe::utils_swap_bytes` with 4
/// bytes) and the undo copies of the club and team are dropped; returns 1. No row (0x8982b0): 0. Null club or slot out
/// of range: the original routine ("Error" box, 0).
/// Original: `Tactics::swap_selection_slots` (0x8973e0, tactics.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn swap_selection_slots(this: usize, club: usize, a: u32, b: u32, team: u32) -> u32 {
    let (va, vb) = (a as u8 as i8, b as u8 as i8);
    if club == 0 || !(0..=0x13).contains(&va) || !(0..=0x13).contains(&vb) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32>(IDX_SWAP_SELECTION_SLOTS)(this, club, a, b, team); // message box: original only
    }
    let l = club_tactics_index(club, team);
    if l == -1 {
        return 0;
    }
    let start = tactics_row_start(this, l);
    exe::utils_swap_bytes((start + tactics_row::SELECTION + 4 * va as usize) as *mut i32, (start + tactics_row::SELECTION + 4 * vb as usize) as *mut i32, 4);
    drop_undo_copies(this, club, team);
    1
}

/// Returns 1 when the current human holds the selection undo copy of this club and team.
///
/// Ret 8: `Tactics::has_selection_undo(club, team)`, from the squad screen (0x458a58). With copies (+0x5f0) and the
/// human whose turn it is (`[0xb74c26 + 0x300 × (i16)[0xb77c7a]]`): his copy (index `[0xae2364]` − id − 1, from 0 to
/// 15) exists (+8) and is of this club (+0) and team (+4): 1; otherwise 0.
/// Original: `Tactics::has_selection_undo` (0x897250, tactics.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn has_selection_undo(this: usize, club: usize, team: u32) -> u32 {
    if read_u8(this + tactics_fields::UNDO_COUNT) == 0 {
        return 0;
    }
    let t = read_usize((read_i16(0xb77c7a) as i32).wrapping_mul(0x300).wrapping_add(0xb74c26) as usize);
    if t == 0 {
        return 0;
    }
    let k = read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(t)).wrapping_sub(1);
    if !(0..0x10).contains(&k) {
        return 0;
    }
    let e = this + tactics_fields::UNDO_SLOTS + k as usize * 0x5c;
    (read_usize(e + undo_slot::IN_USE) != 0 && read_usize(e) == club && read_usize(e + undo_slot::TEAM) as u32 == team) as u32
}

/// Club's row in the tactics table (FN_TACTICS_GET_CLUB_TACTICS_INDEX); −1 without one (duplicate of club_tactics_index).
pub(crate) unsafe fn club_tactics_row(club: usize, team: u32) -> i32 {
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    row(club, team)
}

/// Address of dword cell c of tactics row k: [this + 0x601] + 0x91 × k + c.
pub(crate) unsafe fn tactics_cell(this: usize, k: i32, c: usize) -> usize {
    (read_u32(this + tactics_fields::ROWS) as usize).wrapping_add((k as isize * 0x91) as usize).wrapping_add(c)
}

/// The club's selection slot in the tactics table becomes empty (−1) and the stored slots of the same club and team are
/// released.
///
/// Ret 0xc: `Tactics::clear_selection_slot(club, slot, team)`: slot (0..=19) of the club's line-up in the tactics table
/// (row from 0x8982b0, dword at +2 + 4 × slot) becomes −1 and, with stored spaces (+0x5f0), those of the same club and
/// team (16 spaces of 0x5c bytes starting at +0x30: club +0x30, team +0x34, used +0x38) are released (+0x38 = 0, +0x5f0
/// − 1). Returns 1; no row, 0. Null club or slot out of range: the original routine (box).
/// Original: `Tactics::clear_selection_slot` (0x896e90, tactics.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn clear_selection_slot(this: usize, club: usize, slot: u32, team: u32) -> u32 {
    let v = slot as u8 as i8;
    if club == 0 || !(0..=0x13).contains(&v) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(IDX_CLEAR_SELECTION_SLOT)(this, club, slot, team);
    }
    let k = club_tactics_row(club, team);
    if k == -1 {
        return 0;
    }
    covered!("cpq_13 clear slot");
    write_i32(tactics_cell(this, k, 2 + 4 * v as usize), -1); // the place
    if read_u8(this + tactics_fields::UNDO_COUNT) != 0 {
        for e in 0..16 {
            let a = this + tactics_fields::UNDO_SLOTS + 0x5c * e;
            if read_u32(a + undo_slot::IN_USE) != 0 && read_u32(a) as usize == club && read_u32(a + undo_slot::TEAM) == team {
                write_u32(a + undo_slot::IN_USE, 0);
                write_u8(this + tactics_fields::UNDO_COUNT, read_u8(this + tactics_fields::UNDO_COUNT).wrapping_sub(1));
            }
        }
    }
    1
}

/// Stores the taker of a set piece (first or second choice); without a person, clears it and the second choice moves up
/// to the first.
///
/// Ret 0x14: `Tactics::set_set_piece_taker(club, role, choice, person, team)`: taker `choice` (0..=1) of set piece
/// `role` (0..=4) in the club's tactic row (0x8982b0; dword at +0x5e + 4 × (choice + 2 × role)). With `person` (pointer
/// to the id), writes the id. Without it, the cell becomes −1 and, if it is the first choice, the next one with a taker
/// (> −1) moves up to it and becomes −1. Returns 1; no row, 0. Null club, role or choice out of range: the original
/// routine (box).
/// Original: `Tactics::set_set_piece_taker` (0x897bb0, tactics.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn set_set_piece_taker(this: usize, club: usize, role: u32, choice: u32, person: usize, team: u32) -> u32 {
    let (p, e) = (role as u8 as i8, choice as u8 as i8);
    if club == 0 || !(0..=4).contains(&p) || !(0..=1).contains(&e) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32>(IDX_SET_SET_PIECE_TAKER)(this, club, role, choice, person, team);
    }
    let k = club_tactics_row(club, team);
    if k == -1 {
        return 0;
    }
    let cell = |j: i8| tactics_cell(this, k, 0x5e + 4 * (j as i32 + 2 * p as i32) as usize) as *mut i32;
    if person != 0 {
        core::ptr::write_unaligned(cell(e), read_u32(person) as i32);
        return 1;
    }
    covered!("cpq_13 taker cleared");
    core::ptr::write_unaligned(cell(e), -1); // no taker
    if e < 1 {
        let mut j = e + 1;
        while j < 2 {
            let c = core::ptr::read_unaligned(cell(j));
            if c > -1 {
                covered!("cpq_13 taker moves up");
                core::ptr::write_unaligned(cell(e), c);
                core::ptr::write_unaligned(cell(j), -1);
                return 1;
            }
            j += 1;
        }
    }
    1
}

/// the formation record (0x18e3 bytes) in the two tables
/// Original: `formation_record` (0x896570, tactics.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn formation_record(t: usize, f: i16, human: u32) -> usize {
    let primeiras = read_i32(t + tactics_fields::PRESET_TACTIC_COUNT);
    let limit = if human != 0 { primeiras } else { read_i32(t + tactics_fields::CUSTOM_TACTIC_COUNT).wrapping_add(primeiras) };
    let f = f as i32;
    if f < 0 || f >= limit {
        return original_fn::<unsafe extern "thiscall" fn(usize, i16, u32) -> usize>(IDX_FORMATION_RECORD)(t, f as i16, human);
    }
    if f < primeiras {
        read_usize(t + tactics_fields::PRESET_TACTICS).wrapping_add(f as usize * formation_fields::SIZE)
    } else {
        read_usize(t + tactics_fields::CUSTOM_TACTICS).wrapping_add((f - primeiras) as usize * formation_fields::SIZE)
    }
}

compare_value!(compare_formation_record, formation_record, IDX_FORMATION_RECORD, (f: i16, humano: u32) -> usize);

/// Sets the two extra slots (+0x56, +0x5a) of the club's row.
///
/// Ret 0x10: the two extra slots (+0x56 and +0x5a) of the row of `club` in the tactics table (0x8982b0 with `human`; no
/// row, 0): with `person`, `*person` goes into slot `which` 0/1; without it, the slot becomes −1 and, for slot 0, slot
/// 1 moves down to 0 and becomes −1. Returns 1. Null `club` or `which` out of range: the original routine (error box).
/// Original: `extra_tactic_slot` (0x897ee0, tactics.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn extra_tactic_slot(t: usize, club: usize, which: i8, person: *const u32, human: u32) -> u32 {
    if club == 0 || !(0..=1).contains(&which) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, *const u32, u32) -> u32>(IDX_EXTRA_TACTIC_SLOT)(t, club, which, person, human);
    }
    let tactics_row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let l = tactics_row(club, human);
    if l == -1 {
        return 0;
    }
    let row = table_row(t, l);
    let home = (row + tactics_row::PLAYMAKER + 4 * which as usize) as *mut u32;
    if !person.is_null() {
        covered!("extra slot saved");
        core::ptr::write_unaligned(home, *person);
        return 1;
    }
    covered!("extra slot deleted");
    core::ptr::write_unaligned(home, u32::MAX);
    if which == 0 {
        let second = read_u32(row + tactics_row::SECOND_PLAYMAKER);
        write_u32(row + tactics_row::PLAYMAKER, second);
        write_u32(row + tactics_row::SECOND_PLAYMAKER, u32::MAX);
    }
    1
}

/// Returns the club's tactic: for a human club (without +0x9a01 or with holiday option 2), the slot for the team and
/// the manager's active block; otherwise, by the index in the club row, the preset block (below the preset count) or
/// the club's own.
///
/// Ret 8: `Tactics::get_club_tactic(club, team)`, from the formation of each match (`exe::match_man_choose_computer_tactic`), from the player's
/// weaknesses (`staff_get_player_weakness_flags`), from the media and from the screens. A human club (`exe::human_manager_is_human_club` with 1, 1) whose manager (+0xcf)
/// does not have +0x9a01 (`exe::human_manager_is_on_holiday`) or has holiday option 2 (`exe::human_manager_get_holiday_option`): the slot for the team (`select_tactic_slot`) and his active
/// block (`get_active_tactic`). For the others, the index (signed word) in the club's row (id × 0x91 in `[this+0x601]`): below the
/// count of preset tactics (`[this+0x5fd]`) the preset block (`[this+0x5f9]`), otherwise its own (`[this+0x5f1]`, index
/// − count); blocks of 0x18e3 bytes. Null club: the original routine ("Error" box, 0).
/// Original: `Tactics::get_club_tactic` (0x896660, tactics.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_club_tactic(this: usize, club: usize, team: u32) -> usize {
    let club = Club(club);
    if club.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_GET_CLUB_TACTIC)(this, club.addr(), team); // message box: original only
    }
    if exe::human_manager_is_human_club(club, 1, (1) as u32, (0) as u32, 0) != 0 {
        let manager = read_usize(G_HUMAN_MANAGER);
        if exe::human_manager_is_on_holiday(manager, club.manager().addr()) == 0 || exe::human_manager_get_holiday_option(manager, 2, club.manager().addr()) != 0 {
            covered!("cpx_8 human tactic");
            exe::human_manager_select_tactic_slot(manager, (team) as i32, club, club.manager().addr()); // cpx8: the slot by team
            return exe::human_manager_get_active_tactic(manager, club.manager().addr());
        }
    }
    let id = read_i32(club.addr());
    let index = read_i16(read_usize(this + tactics_fields::ROWS).wrapping_add(id.wrapping_mul(0x91) as usize)) as i32;
    let prontas = read_i32(this + tactics_fields::PRESET_TACTIC_COUNT);
    if index < prontas { // cpx8: preset or own tactic
        covered!("cpx_8 preset tactic");
        read_usize(this + tactics_fields::PRESET_TACTICS).wrapping_add(index.wrapping_mul((formation_fields::SIZE as i32)) as usize) // cpx8: o bloco pronto
    } else {
        covered!("cpx_8 own tactic");
        read_usize(this + tactics_fields::CUSTOM_TACTICS).wrapping_add(index.wrapping_sub(prontas).wrapping_mul((formation_fields::SIZE as i32)) as usize) // cpx8: the own block
    }
}

/// Returns the club's selection slot (0..19) holding the player, or 0xff.
///
/// Ret 0xc: the slot (0..19) of the club's line-up where the person is: the club's tactic row (0x8982b0 with `team`; -1
/// returns 0) and the 20 slots (dwords at +0x601 + row × 0x91 + 2) compared with the id; not there: 0xff. Null club or
/// person: the original routine (error box).
/// Original: `Tactics::find_selection_slot` (0x8972c0, tactics.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn find_selection_slot(tactic: usize, club: usize, person: *const i32, team: u32) -> u8 {
    if club == 0 || person.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, *const i32, u32) -> u8>(IDX_FIND_SELECTION_SLOT)(tactic, club, person, team);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(club, team);
    if k == -1 {
        return 0;
    }
    let id = *person;
    let slots = read_usize(tactic + tactics_fields::ROWS).wrapping_add((k as usize).wrapping_mul(0x91) + 2);
    match (0..20).find(|&i| read_i32(slots + 4 * i) == id) {
        Some(i) => i as u8,
        None => 0xff,
    }
}

/// Stores the person in slot 0..19 of the club's row (0x8982b0) and clears the club's pending instructions.
///
/// Ret 0x10: writes `*person` into slot `slot` 0..19 of the row of `club` (0x8982b0 with `human`; no row, 0) and
/// deletes, among the 16 pending instructions (+0x30, 0x5c bytes: club, human, active), the active ones of that club
/// and `human`, decrementing counter +0x5f0. Returns 1. Null `club` or `person`, slot out of range: the original
/// routine (error box).
/// Original: `set_tactic_slot` (0x896a90, tactics.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn set_tactic_slot(t: usize, club: usize, slot: i8, person: *const u32, human: u32) -> u32 {
    if club == 0 || !(0..=19).contains(&slot) || person.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, *const u32, u32) -> u32>(IDX_SET_TACTIC_SLOT)(t, club, slot, person, human);
    }
    let tactics_row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let l = tactics_row(club, human);
    if l == -1 {
        return 0;
    }
    write_u32(table_row(t, l) + 2 + 4 * slot as usize, *person);
    let pendentes = (t + tactics_fields::UNDO_COUNT) as *mut u8;
    if *pendentes != 0 {
        for k in 0..16 {
            let e = t + tactics_fields::UNDO_SLOTS + 0x5c * k;
            let active = (e + undo_slot::IN_USE) as *mut u32;
            if *active != 0 && *(e as *const usize) == club && *((e + undo_slot::TEAM) as *const u32) == human {
                *active = 0;
                *pendentes = (*pendentes).wrapping_sub(1);
            }
        }
    }
    1
}

/// Returns the slot's person; without a record, or from another nation in a national side, the slot becomes −1.
///
/// Ret 0xc: the person (in `[0xae23bc]`, 0x6e bytes) of slot `slot` 0..19 of the row of `club`; 0 with no row or an
/// empty (negative) slot. A slot with a person without a player sheet (+0x61), or, in a national side (`exe::database_is_national_team`), a
/// person without a nation (+0x24) or from another nation (+0x53): the slot becomes −1 and returns 0. Null `club` or
/// slot out of range: the original routine (error box).
/// Original: `tactic_slot_person` (0x8968f0, tactics.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn tactic_slot_person(t: usize, club: usize, slot: i8, human: u32) -> usize {
    let club = Club(club);
    if club.is_null() || !(0..=19).contains(&slot) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> usize>(IDX_TACTIC_SLOT_PERSON)(t, club.addr(), slot, human);
    }
    let tactics_row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let l = tactics_row(club.addr(), human);
    if l == -1 {
        return 0;
    }
    let home = (table_row(t, l) + 2 + 4 * slot as usize) as *mut i32;
    let id = core::ptr::read_unaligned(home);
    if id <= -1 {
        return 0;
    }
    let persons = read_usize(G_STAFF_TABLE);
    let p = Staff(persons.wrapping_add((id as usize).wrapping_mul(staff::SIZE)));
    if p.player().is_null() {
        core::ptr::write_unaligned(home, -1);
        return 0;
    }
    if exe::database_is_national_team(club) != 0 {
        let nation = Nation(p.national_job().addr());
        if nation.is_null() || nation.three_letter_name() != (club.nation().addr() as u32) {
            core::ptr::write_unaligned(home, -1);
            return 0;
        }
    }
    p.addr()
}

/// stores the formation in the club row
/// Original: `set_tactic_formation` (0x8967d0, tactics.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn set_tactic_formation(t: usize, club: usize, formation: i16, human: u32) -> u32 {
    let limit = read_i32(t + tactics_fields::PRESET_TACTIC_COUNT)
        .wrapping_add(read_i32(t + tactics_fields::CUSTOM_TACTIC_COUNT))
        .wrapping_sub(1);
    if club == 0 || formation < 0 || formation as i32 > limit {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i16, u32) -> u32>(IDX_SET_TACTIC_FORMATION)(t, club, formation, human);
    }
    let tactics_row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let l = tactics_row(club, human);
    if l == -1 {
        return 0;
    }
    write_i16(table_row(t, l), formation);
    1
}

