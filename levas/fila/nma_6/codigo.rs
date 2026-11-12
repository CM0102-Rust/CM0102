// ---------- batch %%LEVA%%: the manager's answer to a media story about a player (media.cpp) ----------

/// The calls of `apply_player_story_response` that have no `exe::` wrapper yet (cm0102.exe 3.9.68): the two confidence
/// adjustments and the apology news are hooked (they reach the active copy through the detour), the story effects and
/// the complaint are still original. Each of them may post news.
#[allow(dead_code)]
pub(crate) mod player_story_fns {
    pub(crate) const ADJUST_SUPPORTER_CONFIDENCE: usize = 0x738340; // Media::adjust_supporter_confidence, thiscall, ret 0x14
    pub(crate) const ADJUST_BOARD_CONFIDENCE: usize = 0x7386f0; // Media::adjust_board_confidence, thiscall, ret 0x18
    pub(crate) const APPLY_STORY_EFFECT_FLAGS: usize = 0x736c00; // Media::apply_story_effect_flags, thiscall, ret 0x1c
    pub(crate) const QUEUE_PLAYER_APOLOGY_NEWS: usize = 0x72bbf0; // media_queue_player_apology_news, stdcall, ret 0xc
    pub(crate) const FILE_COMPLAINT: usize = 0x56a100; // DISPUTE::file_complaint, thiscall, ret 0x10
    pub(crate) const F64_3000: usize = 0x96d200; // f64 = 3000.0 (.rdata)
}

/// Which of the player's complaint tests (0x741f20, the byte table of the jump at 0x741d00) a story kind 0..0x23 uses
/// when the manager answers 2: 0 the general test, 1 the strict test, 2 none.
pub(crate) const PLAYER_STORY_COMPLAINT_CASE: [u8; 0x24] = [
    0, 0, 0, 1, 1, 0, 0, 2, 2, 0, 2, 0, 1, 0, 0, 0, 1, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1,
];

/// The piece of `apply_player_story_response` a story kind goes to (the comparer's message).
pub(crate) fn player_story_response_piece(topic: u32) -> &'static str {
    match topic as u16 {
        0x25 => "position rival (0x25)",
        0x1f => "transfer interest (0x1f)",
        0x20 => "outburst (0x20)",
        _ => "other story",
    }
}

/// In the comparer's rehearsal (`MEDIA_REACTION_DRY_RUN`) a call that may post news is not made: the rehearsal stops
/// there (`MEDIA_REACTION_STOPPED`) and the comparison falls back on the original alone. Outside it: false.
unsafe fn player_story_stops_before_news() -> bool {
    if MEDIA_REACTION_DRY_RUN.load(Relaxed) {
        MEDIA_REACTION_STOPPED.store(true, Relaxed);
        return true;
    }
    false
}

/// STAFF_CONTRACT::apply_event_flags(marks, 0) (`exe::staff_contract_apply_event_flags`); in the rehearsal it stops
/// first when the person would request a transfer (`would_request_transfer`).
unsafe fn player_story_event_flags_or_stop(c: Contract, marks: u32) -> bool {
    if MEDIA_REACTION_DRY_RUN.load(Relaxed) && would_request_transfer(c, marks) {
        MEDIA_REACTION_STOPPED.store(true, Relaxed);
        return true;
    }
    exe::staff_contract_apply_event_flags(c, marks, 0);
    false
}

/// A squad member the squad loops of `respond_to_transfer_interest_story` reach: not null, with a player record and an
/// id below the staff count.
unsafe fn is_story_squad_member(p: Staff) -> bool {
    !p.is_null() && !p.player().is_null() && read_i32(p.addr()) < read_i32(G_STAFF_COUNT)
}

/// `Media::apply_story_effect_flags(club, kind, flags, weight, response, 0, player)`: the effects of the story's outcome.
unsafe fn player_story_effects(this: usize, club: Club, kind: u32, flags: u32, weight: f32, response: u32, player: Staff) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, usize) = core::mem::transmute(player_story_fns::APPLY_STORY_EFFECT_FLAGS);
    f(this, club.addr(), kind, flags, weight.to_bits(), response, 0, player.addr())
}

/// `Media::adjust_supporter_confidence(club, player, kind, target, response)`.
unsafe fn player_story_supporters(this: usize, club: Club, player: Staff, kind: u32, target: i32, response: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(player_story_fns::ADJUST_SUPPORTER_CONFIDENCE);
    f(this, club.addr(), player.addr(), kind, target as u32, response)
}

/// `Media::adjust_board_confidence(club, player, kind, target, response, 0)`.
unsafe fn player_story_board(this: usize, club: Club, player: Staff, kind: u32, target: i32, response: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(player_story_fns::ADJUST_BOARD_CONFIDENCE);
    f(this, club.addr(), player.addr(), kind, target as u32, response, 0)
}

/// Story 0x25 (a position rival arrived: `warn_squad_of_position_rival`), the manager's answer to the player.
///
/// Answer 1: the supporters' confidence (+8 of the club's row in [[`G_MANAGER_MANAGER`]], 1..10000) +200 when the
/// club favours the player (`exe::club_favours_staff`), otherwise −50; with `.professionalism()` below 10 and
/// draw(professionalism) below 5, `.professionalism()` and `.temperament()` each go down by one (above 1 and with a
/// nonzero draw of their own); morale +10 (`exe::staff_adjust_morale`, limited); with a nonzero draw(temperament) and
/// the player disliking the manager (`exe::staff_is_disliked_by`), the manager leaves his disliked list
/// (`exe::staff_remove_disliked_staff`); with a nonzero draw(`.pressure()`), contract flag 0x800 is cleared.
///
/// Answer 2: supporters −50 when the club favours him, otherwise +200; `.professionalism()` above 10 and above
/// draw(20): the apology news (`media_queue_player_apology_news`, story 0x25) and nothing else; professionalism 6 or
/// more: nothing else. Otherwise, with the contract (looked up again) and draw(pressure) below draw(10), contract flag
/// 0x800 (`set_flags`); without it, pressure, determination and the player record's `.work_rate()` each go up by one
/// (below 20, on a zero draw of their own). Then morale −10; with the contract wanting to leave
/// (`exe::staff_contract_wants_to_leave`): loyalty −1 when draw(loyalty) is below draw(25) (above 1); on a zero
/// draw(loyalty), the manager goes into his disliked list (`exe::staff_add_disliked_staff`, forced) and the contract
/// gets event flags 8 and, on a zero draw(pressure), also 0x2000 (`exe::staff_contract_apply_event_flags` with 0).
///
/// Answer 3: supporters −100, board (+6) −50.
unsafe fn respond_to_position_rival_story(this: usize, player: Staff, club: Club, manager: Staff, contract: Contract, response: u32) {
    covered!("nma_6 position rival");
    match response as i8 {
        1 => {
            covered!("nma_6 position rival backed");
            let v = if exe::club_favours_staff(player, club) != 0 { 200 } else { -50 };
            add_clamped_confidence(manager_manager_row(club) + 8, v);
            if player.professionalism() < 10 && exe::utils_random(player.professionalism() as i32) < 5 {
                if player.professionalism() > 1 && exe::utils_random(player.professionalism() as i32) != 0 {
                    player.set_professionalism(player.professionalism().wrapping_sub(1));
                }
                if player.temperament() > 1 && exe::utils_random(player.temperament() as i32) != 0 {
                    player.set_temperament(player.temperament().wrapping_sub(1));
                }
            }
            exe::staff_adjust_morale(player, 10, 1);
            if exe::utils_random(player.temperament() as i32) != 0 && exe::staff_is_disliked_by(player, manager) != 0 {
                exe::staff_remove_disliked_staff(player, manager);
            }
            if exe::utils_random(player.pressure() as i32) == 0 {
                return;
            }
            if contract.flags() & 0x800 != 0 {
                exe::staff_contract_clear_flags(contract, 0x800);
            }
        }
        2 => {
            covered!("nma_6 position rival criticised");
            let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player));
            let v = if exe::club_favours_staff(player, club) != 0 { -50 } else { 200 };
            add_clamped_confidence(manager_manager_row(club) + 8, v);
            let professionalism = player.professionalism();
            if professionalism > 10 && (professionalism as i32) > exe::utils_random(0x14) {
                if player_story_stops_before_news() {
                    return;
                }
                let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(player_story_fns::QUEUE_PLAYER_APOLOGY_NEWS);
                f(this, player.addr(), club.addr(), 0x25);
                return;
            }
            if professionalism >= 6 {
                return;
            }
            let mut flagged = false;
            if !c.is_null() {
                let a = exe::utils_random(player.pressure() as i32);
                if a < exe::utils_random(10) {
                    if set_contract_flags_or_stop(c.addr(), 0x800) {
                        return;
                    }
                    flagged = true;
                }
            }
            if !flagged {
                if player.pressure() < 0x14 && exe::utils_random(player.pressure() as i32) == 0 {
                    player.set_pressure(player.pressure().wrapping_add(1));
                }
                if player.determination() < 0x14 && exe::utils_random(player.determination() as i32) == 0 {
                    player.set_determination(player.determination().wrapping_add(1));
                }
                let sheet = player.player();
                if sheet.work_rate() < 0x14 && exe::utils_random(sheet.work_rate() as i32) == 0 {
                    sheet.set_work_rate(sheet.work_rate().wrapping_add(1));
                }
            }
            exe::staff_adjust_morale(player, -10i32 as u32, 1);
            if exe::staff_contract_wants_to_leave(c) == 0 {
                return;
            }
            if player.loyalty() > 1 {
                let a = exe::utils_random(player.loyalty() as i32);
                if a < exe::utils_random(0x19) {
                    player.set_loyalty(player.loyalty().wrapping_sub(1));
                }
            }
            if exe::utils_random(player.loyalty() as i32) != 0 {
                return;
            }
            exe::staff_add_disliked_staff(player, manager, 1);
            let r = exe::utils_random(player.pressure() as i32);
            if player_story_event_flags_or_stop(c, 8) {
                return;
            }
            if r == 0 {
                player_story_event_flags_or_stop(c, 0x2000);
            }
        }
        3 => {
            covered!("nma_6 position rival no comment");
            add_clamped_confidence(manager_manager_row(club) + 8, -100);
            add_clamped_confidence(manager_manager_row(club) + 6, -50);
        }
        _ => {}
    }
}

/// Story 0x1f (another club's interest in the player: `process_transfer_interest`), the manager's answer; each answer
/// walks the 50 squad slots (`.squad()`).
///
/// Answer 1 (keeps the player): for the player himself, supporters +200 and board +100 when the club favours him,
/// −100 and −100 when it dislikes him (`exe::club_dislikes_staff`); board +250 when he likes the chairman
/// (`exe::staff_is_favoured_by`), −250 when he dislikes him; morale +10 when his contract does not want to leave;
/// otherwise morale −10 and, with draw(loyalty) below draw(25): loyalty −1 on a zero draw (above 1), the manager into
/// his disliked list (forced) and event flags 8 (and 0x2000 on a zero draw(pressure)). For the others (a player record,
/// a valid id): morale +10 when the player likes them, −5 when he dislikes them, otherwise draw(5) − draw(5).
///
/// Answer 2 (lets him go): for the player, supporters −500 and board −200 when the club favours him, +500 and +200
/// when it dislikes him; board −500 when he likes the chairman, +200 when he dislikes him; morale +10 when his contract
/// wants to leave; otherwise morale −10 and, with draw(loyalty) below draw(25), loyalty −1 on a zero draw (above 1) and
/// the manager into his disliked list. The others: liked, morale professionalism / 2 − 15; disliked, +10; otherwise,
/// with draw(ambition) above draw(20) and draw(professionalism) below draw(20), −5.
///
/// Other answers: morale draw(8) − draw(8) for the player and draw(5) − draw(5) for the others.
unsafe fn respond_to_transfer_interest_story(player: Staff, club: Club, manager: Staff, contract: Contract, response: u32) {
    covered!("nma_6 transfer interest");
    match response as i8 {
        1 => {
            for k in 0..50usize {
                let p = Staff(club.squad(k));
                if p == player {
                    covered!("nma_6 transfer interest kept");
                    if exe::club_favours_staff(player, club) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 8, 200); // nma6: kept, a favourite: supporters +200
                        add_clamped_confidence(manager_manager_row(club) + 6, 100);
                    } else if exe::club_dislikes_staff(player, club) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 8, -100);
                        add_clamped_confidence(manager_manager_row(club) + 6, -100);
                    }
                    if exe::staff_is_favoured_by(player, club.chairman()) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 6, 250);
                    } else if exe::staff_is_disliked_by(player, club.chairman()) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 6, -250);
                    }
                    if exe::staff_contract_wants_to_leave(contract) == 0 {
                        exe::staff_adjust_morale(player, 10, 1);
                        continue;
                    }
                    exe::staff_adjust_morale(player, -10i32 as u32, 1);
                    let a = exe::utils_random(player.loyalty() as i32);
                    if a >= exe::utils_random(0x19) {
                        continue;
                    }
                    if player.loyalty() > 1 && exe::utils_random(player.loyalty() as i32) == 0 {
                        player.set_loyalty(player.loyalty().wrapping_sub(1));
                    }
                    exe::staff_add_disliked_staff(player, manager, 1); // nma6: kept against his will: he dislikes the manager
                    let r = exe::utils_random(player.pressure() as i32);
                    if player_story_event_flags_or_stop(contract, 8) {
                        return;
                    }
                    if r == 0 && player_story_event_flags_or_stop(contract, 0x2000) {
                        return;
                    }
                } else if is_story_squad_member(p) {
                    let delta = if exe::staff_is_favoured_by(player, p) != 0 {
                        10
                    } else if exe::staff_is_disliked_by(player, p) != 0 {
                        -5
                    } else {
                        let a = exe::utils_random(5);
                        (a as u8).wrapping_sub(exe::utils_random(5) as u8) as i8
                    };
                    exe::staff_adjust_morale(p, delta as u32, 1);
                }
            }
        }
        2 => {
            for k in 0..50usize {
                let p = Staff(club.squad(k));
                if p == player {
                    covered!("nma_6 transfer interest released");
                    if exe::club_favours_staff(player, club) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 8, -500); // nma6: released, a favourite: supporters -500
                        add_clamped_confidence(manager_manager_row(club) + 6, -200);
                    } else if exe::club_dislikes_staff(player, club) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 8, 500);
                        add_clamped_confidence(manager_manager_row(club) + 6, 200);
                    }
                    if exe::staff_is_favoured_by(player, club.chairman()) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 6, -500);
                    } else if exe::staff_is_disliked_by(player, club.chairman()) != 0 {
                        add_clamped_confidence(manager_manager_row(club) + 6, 200);
                    }
                    if exe::staff_contract_wants_to_leave(contract) != 0 {
                        exe::staff_adjust_morale(player, 10, 1);
                        continue;
                    }
                    exe::staff_adjust_morale(player, -10i32 as u32, 1);
                    let a = exe::utils_random(player.loyalty() as i32);
                    if a >= exe::utils_random(0x19) {
                        continue;
                    }
                    if player.loyalty() > 1 && exe::utils_random(player.loyalty() as i32) == 0 {
                        player.set_loyalty(player.loyalty().wrapping_sub(1));
                    }
                    exe::staff_add_disliked_staff(player, manager, 1);
                } else if is_story_squad_member(p) {
                    if exe::staff_is_favoured_by(player, p) != 0 {
                        let half = (p.professionalism() as i32 / 2) as u8;
                        exe::staff_adjust_morale(p, half.wrapping_sub(0xf) as i8 as u32, 1);
                    } else if exe::staff_is_disliked_by(player, p) != 0 {
                        exe::staff_adjust_morale(p, 10, 1);
                    } else {
                        let a = exe::utils_random(p.ambition() as i32);
                        if a > exe::utils_random(0x14) {
                            let b = exe::utils_random(p.professionalism() as i32);
                            if b < exe::utils_random(0x14) {
                                exe::staff_adjust_morale(p, -5i32 as u32, 1);
                            }
                        }
                    }
                }
            }
        }
        _ => {
            covered!("nma_6 transfer interest other answer");
            for k in 0..50usize {
                let p = Staff(club.squad(k));
                let range = if p == player {
                    8
                } else if is_story_squad_member(p) {
                    5 // nma6: the squad's mood after the rumour
                } else {
                    continue;
                };
                let a = exe::utils_random(range);
                let delta = (a as u8).wrapping_sub(exe::utils_random(range) as u8);
                exe::staff_adjust_morale(p, delta as i8 as u32, 1);
            }
        }
    }
}

/// Story 0x20 (the player's outburst, `create_player_outburst_story` [inference]), the manager's answer.
///
/// Answer 1: supporters' target 500 when the club favours the player, −1500 when it dislikes him, otherwise −500
/// (`adjust_supporter_confidence`); board's target (`adjust_board_confidence`) trunc(w × 1000 − 2000) when he likes the
/// chairman, trunc(w × −1000 − 2000) when he dislikes him, otherwise −2000, where w = (chairman's `.ambition()` − 10)
/// × 0.05 + 1 (0.5 without a chairman); then the story effects with flags 0x100. Answer 2: supporters' target 3000 when
/// the club dislikes him, board's target trunc(w × 3000) when he dislikes the chairman; then the story effects with
/// flags 0x80. Other answers: nothing.
unsafe fn respond_to_outburst_story(this: usize, player: Staff, club: Club, chairman_weight: f64, stat: f32, response: u32) {
    covered!("nma_6 outburst");
    match response as i8 {
        1 => {
            if player_story_stops_before_news() {
                return;
            }
            let target = if exe::club_favours_staff(player, club) != 0 {
                500
            } else if exe::club_dislikes_staff(player, club) != 0 {
                -1500
            } else {
                -500
            };
            player_story_supporters(this, club, player, 0x20, target, response);
            let board = if exe::staff_is_favoured_by(player, club.chairman()) != 0 {
                ftol(chairman_weight * read_f64(rdata::F64_1000) - read_f64(rdata::F64_2000))
            } else if exe::staff_is_disliked_by(player, club.chairman()) != 0 {
                ftol(chairman_weight * read_f64(rdata::F64_NEG_1000) - read_f64(rdata::F64_2000))
            } else {
                -2000
            };
            player_story_board(this, club, player, 0x20, board, response);
            player_story_effects(this, club, 0x20, 0x100, stat, response, player);
        }
        2 => {
            if player_story_stops_before_news() {
                return;
            }
            if exe::club_dislikes_staff(player, club) != 0 {
                player_story_supporters(this, club, player, 0x20, 3000, response);
            }
            if exe::staff_is_disliked_by(player, club.chairman()) != 0 {
                let board = ftol(chairman_weight * read_f64(player_story_fns::F64_3000));
                player_story_board(this, club, player, 0x20, board, response);
            }
            player_story_effects(this, club, 0x20, 0x80, stat, response, player);
        }
        _ => {}
    }
}

/// The player's complaint test of answer 2 to the other stories (`PLAYER_STORY_COMPLAINT_CASE`): a player record with
/// `.current_reputation()` above 6750, `.world_reputation()` above 3750 (the general test also takes a contract
/// `.happiness()` of 100), `.current_ability()` above 120, aged 20 to 33, `.ambition()` above 10, the contract with
/// flags or grievances, its `.morale()` below −25 and wanting to leave, and a zero draw(loyalty). The general test asks
/// `.professionalism()` below 10, `.loyalty()` below 6 and `.temperament()` below 10; the strict one,
/// `.professionalism()` below 6, `.pressure()` above 10 and loyalty or temperament below 6.
unsafe fn player_story_complaint_due(player: Staff, contract: Contract, age: i8, strict: bool) -> bool {
    let sheet = player.player();
    if sheet.is_null() || sheet.current_reputation() <= 0x1a5e {
        return false;
    }
    if sheet.world_reputation() <= 0xea6 && (strict || contract.happiness() != 0x64) {
        return false;
    }
    if sheet.current_ability() <= 0x78 || age <= 0x13 || age >= 0x22 || player.ambition() <= 10 {
        return false;
    }
    let character = if strict {
        player.professionalism() < 6 && player.pressure() > 10 && (player.loyalty() < 6 || player.temperament() < 6)
    } else {
        player.professionalism() < 10 && player.loyalty() < 6 && player.temperament() < 10
    };
    if !character {
        return false;
    }
    if contract.flags() == 0 && contract.grievances() == 0 {
        return false;
    }
    if contract.morale() >= -25 || exe::staff_contract_wants_to_leave(contract) == 0 {
        return false;
    }
    exe::utils_random(player.loyalty() as i32) == 0
}

/// The other stories (neither 0x25, 0x1f nor 0x20): the story effects (`Media::apply_story_effect_flags`, weight = the
/// club's stat 3 in its division) with flags 0x500 for answer 1, 0x404 for answer 3 and 0x480 for answer 2; with
/// answer 2, the player of a kind 0..0x23 that passes his complaint test (`player_story_complaint_due`) first files a
/// complaint (`DISPUTE::file_complaint`, reason 4, and 1 on a zero draw(4)). Other answers: nothing.
#[allow(clippy::too_many_arguments)]
unsafe fn respond_to_other_player_story(this: usize, player: Staff, club: Club, contract: Contract, age: i8, stat: f32, topic: u32, response: u32) {
    covered!("nma_6 other story");
    let flags = match response as i8 {
        1 => 0x500, // nma6: the flags of answer 1
        3 => 0x404,
        2 => {
            if player_story_stops_before_news() {
                return;
            }
            let k = topic as i16 as i32 as u32;
            let case = if k > 0x23 { 2 } else { PLAYER_STORY_COMPLAINT_CASE[k as usize] };
            if case != 2 && player_story_complaint_due(player, contract, age, case == 1) {
                covered!("nma_6 other story complaint");
                let reason_b = (exe::utils_random(4) == 0) as u32;
                let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(player_story_fns::FILE_COMPLAINT);
                f(read_usize(G_DISPUTE), player.addr(), club.addr(), 4, reason_b);
            }
            0x480
        }
        _ => return,
    };
    if player_story_stops_before_news() {
        return;
    }
    player_story_effects(this, club, topic, flags, stat, response, player);
}

/// `Media::apply_player_story_response(player, club, topic, response)`: the effects of the manager's answer (1, 2, 3)
/// to a media story about one of his players.
///
/// Nothing without the player, the club, the club's manager (+0xcf) with a staff sheet (`.non_player()`), the player's
/// contract (`exe::contract_manager_get_contract`) or, for a club that is not a national team
/// (`exe::database_is_national_team`), its division's object in `G_COMP_OBJECT_TABLE` with the stats (+8). The weight of
/// the story effects is the club's stat 3 there (`exe::comp_stats_get_club_stat`, as a float; 0 for a national team).
/// Then by topic: 0x25 `respond_to_position_rival_story`, 0x1f `respond_to_transfer_interest_story`, 0x20
/// `respond_to_outburst_story`, the rest `respond_to_other_player_story`.
///
/// Called by the computer manager's answer to a new story (`create_player_media_story`, 0x72d1ed), the outburst
/// (0x733f97, answer 0), the human manager's answer screen (0x743275), the praise story (0x744f52) and the transfer
/// rumour (`process_transfer_interest`, 0x7551dd).
/// Original: `Media::apply_player_story_response` (0x741020, media.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn apply_player_story_response(this: usize, player: usize, club: usize, topic: u32, response: u32) {
    let (player, club) = (Staff(player), Club(club));
    if club.is_null() || player.is_null() {
        return;
    }
    let manager = club.manager();
    if manager.is_null() || manager.non_player().is_null() {
        return;
    }
    let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player));
    if contract.is_null() {
        return;
    }
    let comp_object = |c: Club| read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_usize(c.division().addr())));
    if exe::database_is_national_team(club) == 0 && (club.division().is_null() || comp_object(club) == 0) {
        return;
    }
    let mut stats = 0usize;
    if exe::database_is_national_team(club) == 0 {
        stats = read_usize(comp_object(club) + 8);
        if stats == 0 {
            return;
        }
    }
    let age = player.age() as i8;
    let chairman = club.chairman();
    let chairman_weight = if chairman.is_null() {
        0.5
    } else {
        ((chairman.ambition() as i32) - 10) as f64 * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_1)
    };
    let stat = if stats != 0 { exe::comp_stats_get_club_stat(stats, club, 3) as f32 } else { 0.0 };
    match topic as u16 {
        0x25 => respond_to_position_rival_story(this, player, club, manager, contract, response),
        0x1f => respond_to_transfer_interest_story(player, club, manager, contract, response),
        0x20 => respond_to_outburst_story(this, player, club, chairman_weight, stat, response),
        _ => respond_to_other_player_story(this, player, club, contract, age, stat, topic, response),
    }
}

/// Comparer for `apply_player_story_response`: the rehearsal of the copy, then the original, by piece
/// (`player_story_response_piece`).
///
/// The copy runs first with `MEDIA_REACTION_DRY_RUN`: it stops (and the comparison falls back on the original alone)
/// at a call that may post news (the apology, the confidence adjustments, the story effects, the complaint: the whole
/// of the outburst and other-story pieces) or at event flags or contract flags that would make the player request a
/// transfer. Otherwise the regions it wrote are kept, put back, the original runs and they must come out equal: the
/// random generator, `G_CONTRACT_LOOKUP_RESULT`, the club's row in [[`G_MANAGER_MANAGER`]] (0x49 bytes), the player's
/// personality bytes (`.ambition()` .. `.temperament()`), his preferences' favourite and disliked staff (24 bytes), his
/// contract (0x50 bytes) and, for him and the 50 squad slots, the player record's work rate and morale. Inside a
/// recorded tape unit: only the original.
pub(crate) unsafe extern "thiscall" fn compare_apply_player_story_response(this: usize, player: usize, club: usize, topic: u32, response: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(%%I0%%);
    if player == 0 || club == 0 || read_usize(G_MANAGER_MANAGER) == 0 || is_recording() {
        return o(this, player, club, topic, response);
    }
    let (p, c) = (Staff(player), Club(club));
    let lookup = read_usize(G_CONTRACT_LOOKUP_RESULT);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((G_CONTRACT_LOOKUP_RESULT, 4));
    reg.push((manager_manager_row(c), 0x49));
    reg.push((p.addr() + staff::AMBITION, 7));
    let pref = read_usize(p.addr() + staff::STAFF_PREFERENCES);
    if pref != 0 {
        reg.push((pref + preferences::STAFF_FAVOURITE_STAFF1, 24));
    }
    let k = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
    if k != 0 {
        reg.push((k, 0x50));
    }
    for s in core::iter::once(player).chain((0..50usize).map(|j| c.squad(j))) {
        let sheet = if s != 0 { Staff(s).player() } else { PlayerSheet(0) };
        if !sheet.is_null() {
            reg.push((sheet.addr() + player_sheet::WORK_RATE, 2));
        }
    }
    write_usize(G_CONTRACT_LOOKUP_RESULT, lookup); // the lookups above do not count
    let before = crate::comparison::MemorySnapshot::read(&reg);
    MEDIA_REACTION_DRY_RUN.store(true, Relaxed);
    MEDIA_REACTION_STOPPED.store(false, Relaxed);
    crate::rng::pause_trace(|| crate::rng::without_counting(|| apply_player_story_response(this, player, club, topic, response)));
    MEDIA_REACTION_DRY_RUN.store(false, Relaxed);
    let mr = crate::comparison::MemorySnapshot::read(&reg);
    before.write_back();
    if MEDIA_REACTION_STOPPED.load(Relaxed) {
        covered!("nma_6 player story only in the original");
        return o(this, player, club, topic, response);
    }
    o(this, player, club, topic, response);
    let mo = crate::comparison::MemorySnapshot::read(&reg);
    SCOREBOARD.record(mo == mr, || {
        let dif: Vec<usize> = mo.0.iter().zip(mr.0.iter()).flat_map(|(a, b)| (0..a.1.len()).filter(move |&i| a.1[i] != b.1[i]).map(move |i| a.0 + i)).take(16).collect();
        format!("apply_player_story_response [{}] (player {:#x}, club {:#x}, topic {:#x}, response {}) differences at {:x?}",
            player_story_response_piece(topic), player, club, topic as u16, response as i8, dif)
    });
}
