// --- sx01d_6: Media::apply_sent_off_story_response (0x75a340) and Media::apply_wants_first_team_response (0x75b440), media.cpp ---

/// Lowers the player record's `.aggression()` by one when it is above 10 and draw(aggression) is zero (the answer 2
/// of `apply_sent_off_story_response`), or raises it when it is below 20 and the draw is zero (answers 1 and 3).
unsafe fn sent_off_nudge_aggression(sheet: PlayerSheet, calm: bool) {
    let a = sheet.aggression();
    if calm {
        if a > 10 && exe::utils_random(a as i32) == 0 {
            sheet.set_aggression(a.wrapping_sub(1));
        }
    } else if a < 20 && exe::utils_random(a as i32) == 0 {
        sheet.set_aggression(a.wrapping_add(1));
    }
}

/// Raises the player record's `.dirtiness()` by one when it is below 20 and draw(`.professionalism()`) is zero.
unsafe fn sent_off_raise_dirtiness(player: Staff) {
    let sheet = player.player();
    if sheet.dirtiness() < 20 && exe::utils_random(player.professionalism() as i32) == 0 {
        sheet.set_dirtiness(sheet.dirtiness().wrapping_add(1));
    }
}

/// Applies your reply to the story about one of your players sent off.
///
/// Nothing without the player, his `.club_job()`, his `.player()` sheet, `club` with a `.manager()` or his contract at
/// his club (`exe::contract_manager_get_contract_for_club`). The bonus: for a `.current_reputation()` above 7250, 3 for
/// a `.temperament()` of 1 (1 below 5), plus 5 for a `.professionalism()` of 1 (3 below 5, 1 below 10). By the reply:
/// - 2 (you criticise him): the aggression may drop (`sent_off_nudge_aggression`), the `.dirtiness()` drops by one
///   above 1 when draw(professionalism) is not zero; then, when draw(`.loyalty()` + 5) is below draw(6) + the bonus,
///   he stops favouring the manager (`exe::staff_remove_favoured_staff`) or, when he did not favour him, comes to
///   dislike him (`exe::staff_add_disliked_staff`), and the contract takes flag 0x1000 (`exe::staff_contract_set_flags`);
/// - 1 (you back him): the other club's manager dislikes yours; aggression and dirtiness may rise, the
///   `.sportsmanship()` drops by one above 1 when draw(professionalism) is zero; when draw(loyalty) beats draw(10) he
///   stops disliking the manager (`exe::staff_remove_disliked_staff`) or comes to favour him
///   (`exe::staff_add_favoured_staff`) and thanks him (`queue_player_gratitude_news`, kind 0x82); then, when
///   draw(professionalism) is below the bonus + 5, he dislikes the other club's manager;
/// - 3: aggression and dirtiness may rise.
/// Original: `Media::apply_sent_off_story_response` (0x75a340, media.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn apply_sent_off_story_response(this: usize, player: usize, club: usize, other_club: usize, reply: u32) {
    let (player, club, other_club) = (Staff(player), Club(club), Club(other_club));
    if player.is_null() || player.club_job().is_null() || player.player().is_null() || club.is_null() || club.manager().is_null() {
        return;
    }
    let contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, player.club_job()));
    if contract.is_null() {
        return;
    }
    let sheet = player.player();
    let manager = club.manager();
    let mut bonus: i8 = 0;
    if sheet.current_reputation() as i16 > 0x1c52 {
        let t = player.temperament();
        if t == 1 {
            bonus = 3;
        } else if t < 5 {
            bonus = 1;
        }
        let p = player.professionalism();
        if p == 1 {
            bonus += 5;
        } else if p < 5 {
            bonus += 3;
        } else if p < 10 {
            bonus += 1;
        }
    }
    match reply as u8 {
        2 => {
            covered!("sx01d_6 sent off criticised");
            sent_off_nudge_aggression(sheet, true);
            if sheet.dirtiness() > 1 && exe::utils_random(player.professionalism() as i32) != 0 {
                sheet.set_dirtiness(sheet.dirtiness().wrapping_sub(1));
            }
            let a = exe::utils_random(player.loyalty() as i32 + 5);
            if a >= exe::utils_random(6) + bonus as i32 {
                return;
            }
            if exe::staff_is_favoured_by(player, manager) != 0 {
                exe::staff_remove_favoured_staff(player, manager);
            } else {
                exe::staff_add_disliked_staff(player, manager, 1);
            }
            exe::staff_contract_set_flags(contract, 0x1000);
        }
        1 => {
            covered!("sx01d_6 sent off backed");
            if !other_club.is_null() && !other_club.manager().is_null() {
                exe::staff_add_disliked_staff(other_club.manager(), manager, 1);
            }
            sent_off_nudge_aggression(sheet, false);
            sent_off_raise_dirtiness(player);
            if player.sportsmanship() > 1 && exe::utils_random(player.professionalism() as i32) == 0 {
                player.set_sportsmanship(player.sportsmanship().wrapping_sub(1));
            }
            let a = exe::utils_random(player.loyalty() as i32);
            if a > exe::utils_random(10) {
                if exe::staff_is_disliked_by(player, manager) != 0 {
                    exe::staff_remove_disliked_staff(player, manager);
                } else {
                    exe::staff_add_favoured_staff(player, manager, 1);
                }
                queue_player_gratitude_news(this, player.addr(), club.addr(), 0x82, 0);
            }
            if !other_club.is_null() && !other_club.manager().is_null()
                && exe::utils_random(player.professionalism() as i32) < bonus as i32 + 5 {
                exe::staff_add_disliked_staff(player, other_club.manager(), 1);
            }
        }
        3 => {
            sent_off_nudge_aggression(sheet, false);
            sent_off_raise_dirtiness(player);
        }
        _ => {}
    }
}

/// Comparer for `apply_sent_off_story_response`: it draws, changes the player's character and relations and may post
/// news, so it cannot run twice; only the original runs (the reply on the media response screen).
pub(crate) unsafe extern "thiscall" fn compare_apply_sent_off_story_response(this: usize, player: usize, club: usize, other_club: usize, reply: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32)>(%%I0%%)(this, player, club, other_club, reply)
}

/// Applies your reply to a player who wants first team football.
///
/// Nothing without the player, his `.club_job()`, his `.player()` sheet, `club` with a `.manager()` or his contract at
/// his club (`exe::contract_manager_get_contract_for_club`). By the reply (the contract's `.morale()` always kept
/// between -100 and 100, `add_contract_morale`):
/// - 2: when the `.pressure()` is above 1 and draw(pressure) is not zero, the pressure drops by one and the morale by
///   25; otherwise, below 20, the pressure rises by one, the sheet's `.work_rate()` may rise (below 20, a zero draw) and
///   the morale drops by 5;
/// - 1 (you promise him football): transfer listed or asking to be (`exe::staff_is_transfer_listed_or_requested`):
///   the morale drops by 25. Otherwise with the contract's `.unhappy_flag()` above 1, unless the media's first logged
///   story is a 0xbe8 of this club with person 0, the contract takes the event 0x8000000
///   (`exe::staff_contract_apply_event_flags`), his reaction story is queued (`queue_wants_first_team_story`, 2) and the
///   morale drops by 15; in the other case, with a `.loyalty()` above 5 and a non-zero draw(loyalty), the reaction 1 is
///   queued and the contract flag 0x200 cleared (`exe::staff_contract_clear_flags`), then the morale rises by 15;
/// - 3: when draw(`.pressure()`) is below draw(`.temperament()`), the contract takes flag 0x80; the morale drops by 5.
///
/// Defect of the original: the test of the logged story looks only at the media log's first record.
/// Original: `Media::apply_wants_first_team_response` (0x75b440, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn apply_wants_first_team_response(this: usize, player: usize, club: usize, reply: u32) {
    let (player, club) = (Staff(player), Club(club));
    if player.is_null() || player.club_job().is_null() || player.player().is_null() || club.is_null() || club.manager().is_null() {
        return;
    }
    let contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, player.club_job()));
    if contract.is_null() {
        return;
    }
    let queue = |reaction: u32| {
        // Media::queue_wants_first_team_story (0x75a640, thiscall, ret 8) by address: it is ported in sx01d_2
        let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x75a640usize);
        f(read_usize(G_MEDIA), player.addr(), reaction)
    };
    match reply as u8 {
        2 => {
            covered!("sx01d_6 first team refused");
            let p = player.pressure();
            if p > 1 && exe::utils_random(p as i32) != 0 {
                player.set_pressure(p - 1);
                add_contract_morale(contract.addr(), -25);
                return;
            }
            if p >= 20 {
                return;
            }
            player.set_pressure(p + 1);
            let sheet = player.player();
            let w = sheet.work_rate();
            if w < 20 && exe::utils_random(w as i32) == 0 {
                sheet.set_work_rate(w + 1);
            }
            add_contract_morale(contract.addr(), -5);
        }
        1 => {
            covered!("sx01d_6 first team promised");
            if exe::staff_is_transfer_listed_or_requested(player, 0) != 0 {
                add_contract_morale(contract.addr(), -25);
                return;
            }
            let log = read_usize(this + media_fields::STORY_LOG);
            let already = read_i32(this + media_fields::STORY_LOG_COUNT) > 0
                && read_u32(log + story_log_entry::TYPE) == 0xbe8
                && read_i16(log + story_log_entry::SUBTYPE) as i32 == read_i32(club.addr())
                && read_u32(log + story_log_entry::PERSON_A) == 0;
            if contract.unhappy_flag() > 1 && !already {
                exe::staff_contract_apply_event_flags(contract, 0x8000000, 0);
                queue(2);
                add_contract_morale(contract.addr(), -15);
                return;
            }
            let l = player.loyalty();
            if l > 5 && exe::utils_random(l as i32) != 0 {
                queue(1);
                if exe::staff_contract_get_flags(contract) & 0x200 != 0 {
                    exe::staff_contract_clear_flags(contract, 0x200);
                }
            }
            add_contract_morale(contract.addr(), 15);
        }
        3 => {
            let t = exe::utils_random(player.temperament() as i32);
            if exe::utils_random(player.pressure() as i32) < t {
                exe::staff_contract_set_flags(contract, 0x80);
            }
            add_contract_morale(contract.addr(), -5);
        }
        _ => {}
    }
}

/// Comparer for `apply_wants_first_team_response`: it draws, changes the contract and may queue news, so it cannot run
/// twice; only the original runs (the reply on the media response screen).
pub(crate) unsafe extern "thiscall" fn compare_apply_wants_first_team_response(this: usize, player: usize, club: usize, reply: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I1%%)(this, player, club, reply)
}
