// ---------- batch %%LEVA%%: the fights and the dissent of a match player (match_pl.cpp) ----------

/// Commentary with the player as subject: `code` with his commentary id and side (the original pushes -1, -1, 0 for
/// the last three arguments).
unsafe fn narrate_match_player(call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, j: MatchPlayer, code: u32) {
    let m = j.engine();
    call(FN_MATCH_EVENTS_ADD_EVENT, m.commentator(), [code, m.addr() as u32, j.commentary_id() as u32, j.side() as u8 as u32, u32::MAX]);
}

/// Whether the club of the side opposite to `j` (side 0: the away club, otherwise the home club) has a nation whose
/// first language is English: the referee's "off, off, off" line is only said for those.
unsafe fn opposing_club_speaks_english(j: MatchPlayer) -> bool {
    let fixture = Fixture(j.engine().r#match());
    let club = if j.side() == 0 { fixture.away_club() } else { fixture.home_club() };
    let nation = club.nation();
    !nation.is_null() && nation.first_language() == language::ENGLISH
}

/// The card shown to `j` after a dive or a reaction: yellow, second yellow, red or a ticking-off (cards 2, 3, 4 and
/// 9; anything else is not narrated). `factor` (the reaction only) scales the anger of his team-mates first
/// (`exe::cm3_match_engine_scale_side_anger_from_player`). random(8) ≠ 0 and `may_call_over` give, before the card,
/// "the referee calls him over", preceded by "off, off, off" (random(3) ≠ 0 and an English-speaking opposing club).
/// A second yellow or a red sends him off (`player_left_pitch_with`).
unsafe fn announce_card(j: MatchPlayer, card: u8, may_call_over: bool, react: bool, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let (code, factor) = match card {
        2 => (event::YELLOW_CARD, 0.85f32),
        3 => (event::SECOND_YELLOW_CARD, 0.75),
        4 => (event::SENT_OFF, 0.5),
        9 => (event::TICKING_OFF, 0.95),
        _ => return,
    };
    let m = j.engine();
    if react {
        exe::cm3_match_engine_scale_side_anger_from_player(m, j, factor);
    }
    if random(8) != 0 && may_call_over {
        if random(3) != 0 && opposing_club_speaks_english(j) {
            call(FN_MATCH_EVENTS_ADD_EVENT, m.commentator(), [event::SILENT_OFF_OFF_OFF, m.addr() as u32, 0, 0, u32::MAX]);
        }
        narrate_match_player(call, j, event::REFEREE_CALLS_PLAYER_OVER);
    }
    narrate_match_player(call, j, code);
    if card == 3 || card == 4 {
        covered!("sx15_2 sent off after the card");
        player_left_pitch_with(m, j, 0, call);
    }
}

/// A suspected dive by `j`: narrates "looked like a dive", asks the referee for the card of severity 6
/// (`engine_card_by_severity`, flag 0) and announces it (`announce_card`: the call-over may come; a second yellow or
/// a red sends him off).
/// Original: `MATCH_PLAYER::handle_suspected_dive` (0x716c90, match_pl.cpp, thiscall).
pub(crate) unsafe fn handle_suspected_dive_with(j: MatchPlayer, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    covered!("sx15_2 suspected dive");
    narrate_match_player(call, j, event::LOOKED_LIKE_A_DIVE);
    let mut card = 0u8;
    engine_card_by_severity(j.engine().addr(), j.addr(), &mut card, 6, 0); // sx15_2: the dive's severity
    announce_card(j, card, true, false, call);
}

/// Hook entry for `handle_suspected_dive`.
pub(crate) unsafe extern "thiscall" fn handle_suspected_dive(j: usize) {
    handle_suspected_dive_with(MatchPlayer(j), &mut |rotina, this, a| dispatch_fight_calls(rotina, this, a))
}

/// Whether `j` loses his head: random(20 − aggression) = 0 with aggression at least 5 (x87: the f32 20.0 minus the f32
/// aggression, truncated).
unsafe fn loses_head(j: MatchPlayer) -> bool {
    random(ftol_x87(read_f32(rdata::F32_20) as f64 - j.aggression() as f64)) == 0 && j.aggression() as f64 >= read_f64(rdata::F64_5)
}

/// The reaction of `j` to a referee's decision against him, by his anger checks (`exe::match_player_passes_anger_check`
/// with 5, 4, 2, 2 and 1, target 0): level 5 strikes the referee (severity 5) and level 4 pushes him (severity 4) when
/// he loses his head (`loses_head`), otherwise he kicks the ball away (severity 3, and the referee may call him over);
/// the first level 2 also kicks the ball away; the second level 2 (severity 2) and level 1 (severity 1) argue. The card
/// comes from the referee (`engine_card_by_severity` with `flag`) and is announced (`announce_card`, scaling his
/// team-mates' anger by the card). Then, in an English competition (the competition's nation is
/// `KEY_NATION_ENGLAND`), with the ball with the other side, a card other than 0 and 9, the ball on rows 1..10 and a
/// free kick pending (`restart::FREE_KICK` or `restart::INDIRECT_FREE_KICK`): the free kick moves one row towards his
/// goal (clamped 0..11, "moved back 10 yards") and, inside his own penalty area (`exe::is_in_own_penalty_area`),
/// becomes indirect (narrated only when it was direct).
/// Original: `MATCH_PLAYER::react_to_foul_decision` (0x716e10, match_pl.cpp, thiscall, ret 4).
pub(crate) unsafe fn react_to_foul_decision_with(j: MatchPlayer, flag: u32, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let passes = |level: i8| exe::match_player_passes_anger_check(j, level, 0usize) != 0;
    let mut argued = false;
    let mut kick_ball_away = |call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        narrate_match_player(call, j, event::PLAYER_KICKS_BALL_AWAY);
        argued = true;
        3u8
    };
    let severity = if passes(5) {
        if loses_head(j) {
            narrate_match_player(call, j, event::PLAYER_STRIKES_REFEREE);
            Some(5u8)
        } else {
            Some(kick_ball_away(call))
        }
    } else if passes(4) {
        if loses_head(j) {
            narrate_match_player(call, j, event::PLAYER_PUSHES_REFEREE);
            Some(4)
        } else {
            Some(kick_ball_away(call))
        }
    } else if passes(2) {
        Some(kick_ball_away(call))
    } else if passes(2) {
        narrate_match_player(call, j, event::PLAYER_ARGUES_WITH_REFEREE);
        Some(2)
    } else if passes(1) {
        narrate_match_player(call, j, event::PLAYER_ARGUES_WITH_REFEREE);
        Some(1) // sx15_2: arguing, severity 1
    } else {
        None
    };
    let mut card = 0u8;
    if let Some(s) = severity {
        engine_card_by_severity(j.engine().addr(), j.addr(), &mut card, s, flag as u8);
    }
    announce_card(j, card, argued, true, call);
    move_free_kick_back(j, card, call);
}

/// The English ten-yard rule after dissent (see `react_to_foul_decision_with`).
unsafe fn move_free_kick_back(j: MatchPlayer, card: u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let m = j.engine();
    let other = (j.side() == 0) as u32;
    if m.possession_side() as i32 != other as i32 || card == 0 || card == 9 {
        return;
    }
    if m.ball_y() <= 0 || m.ball_y() >= 11 {
        return;
    }
    if m.restart_type() != restart::FREE_KICK && m.restart_type() != restart::INDIRECT_FREE_KICK {
        return;
    }
    let competition = Fixture(m.r#match()).competition();
    if competition.is_null() || competition.nation().is_null() || read_u32(competition.nation().addr()) != read_u32(KEY_NATION_ENGLAND) {
        return;
    }
    covered!("sx15_2 free kick moved back");
    let y = if j.side() != 0 { m.ball_y() - 1 } else { m.ball_y() + 1 };
    m.set_ball_y(y.clamp(0, 11)); // sx15_2: one row towards his goal
    call(FN_MATCH_EVENTS_ADD_EVENT, m.commentator(), [event::FREEKICK_MOVED_BACK_10_YARDS, m.addr() as u32, u32::MAX, other, u32::MAX]);
    if exe::is_in_own_penalty_area(j.side() as u8, m.ball_x(), m.ball_y()) != 0 {
        if m.restart_type() == restart::FREE_KICK {
            call(FN_MATCH_EVENTS_ADD_EVENT, m.commentator(), [event::FREEKICK_BECOMES_INDIRECT, m.addr() as u32, u32::MAX, other, u32::MAX]);
        }
        m.set_restart_type(restart::INDIRECT_FREE_KICK);
    }
}

/// Hook entry for `react_to_foul_decision`.
pub(crate) unsafe extern "thiscall" fn react_to_foul_decision(j: usize, flag: u32) {
    react_to_foul_decision_with(MatchPlayer(j), flag, &mut |rotina, this, a| dispatch_fight_calls(rotina, this, a))
}

/// The provoked opponent's confidence (`match_player::CONFIDENCE`, f32): (`base` − temperament × `step`) × confidence,
/// floored at 2 × (25 − temperament) and capped at 100 (x87: the product kept unrounded for the comparisons).
unsafe fn provoke_confidence(e: MatchPlayer, base: f64, step: f64) {
    let temperament = e.staff().temperament() as i32;
    let mut v = (base - temperament as f64 * step) * e.confidence() as f64;
    let floor = ((25 - temperament) * 2) as f32;
    if v <= floor as f64 {
        v = floor as f64;
    }
    e.set_confidence(v as f32);
    if v > read_f64(rdata::F64_100) {
        e.set_confidence(100.0);
    }
}

/// The opponent `j` picks a fight with, in his own grid cell, and the act: the first occupant of the other side (with
/// staff and player records) for whom an anger check passes (`exe::match_player_passes_anger_check` with the occupant
/// as target). Level 4: headbutt (his side's anger × 2.25, `provoke_confidence` 4.0/0.16). Level 3: spit when
/// random(1000, or 300 when `j`'s nation does not speak English) < 10 (× 2.5), otherwise by random(10): 0..3 kick, 4..8
/// punch, 9 elbow (× 2.0), all with 4.0/0.16. Level 2: shove (random(2) ≠ 0 or already squaring up) or square up
/// (× 1.15, 3.1/0.1). Returns (commentary, move, opponent), the opponent null when none.
unsafe fn pick_fight_opponent(j: MatchPlayer) -> (u32, u8, MatchPlayer) {
    let m = j.engine();
    let cell = m.cell(j.cell_x(), j.cell_y()).addr();
    let other = (j.side() == 0) as usize;
    let count = || read_i8(cell + grid_cell::OCCUPANT_COUNT + other);
    let (mut code, mut mv, mut target) = (u32::MAX, 0u8, MatchPlayer(0));
    let mut k = 0usize;
    while code == u32::MAX && (k as i32) < count() as i32 {
        let e = MatchPlayer(read_usize(cell + grid_cell::OCCUPANTS + (other * 11 + k) * 4));
        k += 1;
        if e.is_null() || e.staff().is_null() || e.player().is_null() {
            continue;
        }
        let provoke = |code_mv: (u32, usize), factor: f32, base: f64, step: f64| {
            exe::cm3_match_engine_scale_side_anger(m, e.side(), factor);
            provoke_confidence(e, base, step);
            (code_mv.0, code_mv.1 as u8, e)
        };
        if exe::match_player_passes_anger_check(j, 4, e) != 0 {
            (code, mv, target) = provoke((event::PLAYER_HEADBUTTS_OPPONENT, moves::HEADBUTT), 2.25, 4.0, read_f64(0x96d330)); // f64 = 0.16
        } else if exe::match_player_passes_anger_check(j, 3, e) != 0 {
            let english = j.staff().nation().first_language() == language::ENGLISH;
            if random(if english { 1000 } else { 300 }) < 10 {
                (code, mv, target) = provoke((event::PLAYER_SPITS_AT_OPPONENT, moves::SPIT), 2.5, 4.0, read_f64(0x96d330));
            } else {
                let act = match random(10) {
                    0..=3 => (event::PLAYER_KICKS_OPPONENT, moves::KICK_OPPONENT),
                    4..=8 => (event::PLAYER_PUNCHES_OPPONENT, moves::PUNCH),
                    _ => (event::PLAYER_ELBOWS_OPPONENT, moves::ELBOW),
                };
                (code, mv, target) = provoke(act, 2.0, read_f64(rdata::F64_4), read_f64(0x96d330)); // sx15_2: the blows
            }
        } else if exe::match_player_passes_anger_check(j, 2, e) != 0 {
            let act = if random(2) != 0 || j.action() as usize == moves::SQUARE_UP {
                (event::PLAYER_PUSHES_OPPONENT, moves::SHOVE)
            } else {
                (event::PLAYER_SQUARES_UP_TO_OPPONENT, moves::SQUARE_UP)
            };
            (code, mv, target) = provoke(act, 1.15, read_f64(rdata::F64_3_1), read_f64(rdata::F64_0_1));
        }
    }
    (code, mv, target)
}

/// An angry `j` picks a fight in his grid cell (`pick_fight_opponent`); none: returns 0. Otherwise his action becomes
/// the move. At `depth` > 1, random(5) ≠ 0 and at least random(3) + 2 pending incidents: "a big punch-up" and the
/// depth becomes −50 (the chain goes on to −40); otherwise, at `depth` ≥ 0, the act is narrated with both players. The
/// players on the pitch who are not busy (`match_player::BUSY` < 2) pass the level-1 anger check and run to his cell
/// (`exe::match_player_move_to_grid_cell`, a tape unit). The referee judges the act (`exe::cm3_match_engine_referee_judge_tackle`,
/// the move and −1): with a verdict, the severity is added to his pending incidents, or a new incident (player,
/// opponent, severity, move, card) is queued. The opponent reacts in turn (this routine with `depth` + 1) while `depth`
/// is 0..9 or below −40 and fewer than 5 incidents are pending. Returns 1.
/// Original: `MATCH_PLAYER::start_fight` (0x716630, match_pl.cpp, thiscall, ret 4).
pub(crate) unsafe fn start_fight_with(j: MatchPlayer, depth: i8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let m = j.engine();
    let (code, mv, target) = pick_fight_opponent(j);
    if target.is_null() {
        return 0;
    }
    covered!("sx15_2 fight");
    j.set_action(mv as i8 as i16 as u16);
    let mut next = depth;
    let mut brawl = false;
    if depth > 1 && random(5) != 0 && m.pending_incident_count() as i32 >= random(3) + 2 {
        call(FN_MATCH_EVENTS_ADD_EVENT, m.commentator(), [event::BIG_PUNCH_UP, m.addr() as u32, u32::MAX, j.side() as u8 as u32, u32::MAX]);
        next = -50;
        brawl = true;
    }
    if !brawl && depth >= 0 {
        let pair = target.commentary_id() as u32 | (target.side() as u8 as u32) << 8;
        call(FN_MATCH_EVENTS_ADD_EVENT, m.commentator(), [code, m.addr() as u32, j.commentary_id() as u32, j.side() as u8 as u32, pair]);
    }
    for side in 0..2i8 {
        for k in 0..20 {
            let p = m.player(side, k);
            if (0..=10).contains(&p.position()) && p.busy() < 2 && p.addr() != j.addr() && exe::match_player_passes_anger_check(p, 1, 0usize) != 0 {
                call(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL, p.addr(), [j.cell_x() as u8 as u32, j.cell_y() as u8 as u32, 0, 0, 0]);
            }
        }
    }
    let (mut card, mut severity, mut advantage) = (0u8, 0i8, 0u8);
    if exe::cm3_match_engine_referee_judge_tackle(m, j, target, mv as i8, -1, &mut card, &mut severity, &mut advantage) != 0 {
        queue_fight_incident(m, j, target, card, mv, severity);
    }
    if ((0..10).contains(&next) || next < -40) && m.pending_incident_count() < 5 {
        call(FN_MATCH_PLAYER_START_FIGHT, target.addr(), [next.wrapping_add(1) as u8 as u32, 0, 0, 0, 0]);
    }
    1
}

/// The referee's verdict on a fight: `severity` is added to every pending incident of `j`; when he has none, a new
/// incident (`j`, `target`, `severity`, the move, the card) is appended (11-byte entries from
/// `engine::PENDING_INCIDENTS`).
unsafe fn queue_fight_incident(m: Engine, j: MatchPlayer, target: MatchPlayer, card: u8, mv: u8, severity: i8) {
    let entry = |k: i8| Engine((m.addr() as isize + 11 * k as isize) as usize);
    let mut found = false;
    for k in 0..m.pending_incident_count() {
        if entry(k).pending_incidents() == j.addr() {
            entry(k).set_pending_incident_severity(entry(k).pending_incident_severity().wrapping_add(severity));
            found = true;
        }
    }
    if found {
        return;
    }
    let e = entry(m.pending_incident_count());
    e.set_pending_incidents(j.addr());
    e.set_pending_incident_victim(target.addr());
    e.set_pending_incident_reason(card as i8);
    e.set_pending_incident_play(mv as i8);
    e.set_pending_incident_severity(severity); // sx15_2: the new incident's severity
    m.set_pending_incident_count(m.pending_incident_count() + 1);
}

/// Hook entry for `start_fight`.
pub(crate) unsafe extern "thiscall" fn start_fight(j: usize, depth: u32) -> u8 {
    start_fight_with(MatchPlayer(j), depth as i8, &mut |rotina, this, a| dispatch_fight_calls(rotina, this, a))
}

/// Rust-mode dispatcher of the fights' injected calls: the moves to a grid cell and the next fight go to the game
/// routines; the rest (commentary, sending-off) as in the tackle (`dispatch_tackle_calls`).
pub(crate) unsafe fn dispatch_fight_calls(routine: usize, this: usize, a: [u32; 5]) -> u32 {
    match routine {
        FN_MATCH_PLAYER_MOVE_TO_GRID_CELL => exe::match_player_move_to_grid_cell(this, a[0] as i8, a[1] as i8),
        FN_MATCH_PLAYER_START_FIGHT => {
            let f: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(FN_MATCH_PLAYER_START_FIGHT);
            f(this, a[0]) as u32
        }
        _ => dispatch_tackle_calls(routine, this, a),
    }
}

/// Mask of the arguments the tape compares for each injected call of the fights.
fn fight_call_mask(routine: usize) -> [u32; 5] {
    match routine {
        0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
        0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
        0x6f46b0 => [0xff, 0xff, 0, 0, 0],
        0x6e1aa0 => [0; 5],
        _ => [0xff, 0, 0, 0, 0],
    }
}

/// Regions compared for the fights: the random state, the whole engine, the two globals the sending-off touches and
/// the tactics rows (the captain) of both clubs.
unsafe fn fight_regions(m: Engine) -> Vec<(usize, usize)> {
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m.addr(), ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let t = Fixture(m.r#match());
    for club in [t.home_club(), t.away_club()] {
        reg.extend(club_rows(club.addr()));
    }
    reg
}

/// The comparison gate of the three: inside another routine's tape that records them (`RECORD_716`) the call is one
/// unit, compared on its own inside; inside a tape that does not, the original runs (as the old gate did); outside any
/// tape, `compare` runs.
unsafe fn fight_gate(routine: usize, j: usize, a1: u32, original: impl FnOnce() -> u32, compare: impl FnOnce() -> u32) -> u32 {
    if !is_recording() {
        return compare();
    }
    if !RECORD_716.load(Relaxed) {
        return original();
    }
    call_recording(routine, j, [a1, 0, 0, 0, 0], compare)
}

/// Comparer for `start_fight`.
pub(crate) unsafe extern "thiscall" fn compare_start_fight(j: usize, depth: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u8>(%%I0%%);
    fight_gate(FN_MATCH_PLAYER_START_FIGHT, j, depth, || o(j, depth) as u32, || {
        let jp = MatchPlayer(j);
        let write = RECORD_716.swap(true, Relaxed);
        let (r, v, igual, sequencia, k) = compare_com_fita(fight_regions(jp.engine()), || {
            zero_stack_below();
            o(j, depth)
        }, |chamar| start_fight_with(jp, depth as i8, &mut |rotina, this, args| chamar(rotina, this, args, fight_call_mask(rotina))));
        RECORD_716.store(write, Relaxed);
        SCOREBOARD.record(igual && sequencia && r == v, || format!("start_fight(depth {}) original={} rust={} {} calls: {}", depth as i8, r, v, k, tape_detail()));
        r as u32
    }) as u8
}

/// Comparer for `handle_suspected_dive`.
pub(crate) unsafe extern "thiscall" fn compare_handle_suspected_dive(j: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    fight_gate(FN_MATCH_PLAYER_HANDLE_SUSPECTED_DIVE, j, 0, || {
        o(j);
        0
    }, || {
        let jp = MatchPlayer(j);
        let write = RECORD_716.swap(true, Relaxed);
        let (_, _, igual, sequencia, k) = compare_com_fita(fight_regions(jp.engine()), || o(j), |chamar| {
            handle_suspected_dive_with(jp, &mut |rotina, this, args| chamar(rotina, this, args, fight_call_mask(rotina)))
        });
        RECORD_716.store(write, Relaxed);
        SCOREBOARD.record(igual && sequencia, || format!("handle_suspected_dive() {} calls: {}", k, tape_detail()));
        0
    });
}

/// Comparer for `react_to_foul_decision`.
pub(crate) unsafe extern "thiscall" fn compare_react_to_foul_decision(j: usize, flag: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I2%%);
    fight_gate(FN_MATCH_PLAYER_REACT_TO_FOUL_DECISION, j, flag, || {
        o(j, flag);
        0
    }, || {
        let jp = MatchPlayer(j);
        let write = RECORD_716.swap(true, Relaxed);
        let (_, _, igual, sequencia, k) = compare_com_fita(fight_regions(jp.engine()), || {
            zero_stack_below();
            o(j, flag)
        }, |chamar| react_to_foul_decision_with(jp, flag, &mut |rotina, this, args| chamar(rotina, this, args, fight_call_mask(rotina))));
        RECORD_716.store(write, Relaxed);
        SCOREBOARD.record(igual && sequencia, || format!("react_to_foul_decision(flag {}) {} calls: {}", flag as u8, k, tape_detail()));
        0
    });
}
