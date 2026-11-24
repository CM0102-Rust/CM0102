// ---------- batch %%LEVA%%: fining a player (Fine.cpp) ----------

/// `Fine::is_fine_justified(player, period, reason)` (thiscall, ret 0xc -> al).
const FN_FINE_IS_FINE_JUSTIFIED: usize = 0x5a69b0;
/// `post_fine_reaction_news(player, reason, reaction, period, effects)` (stdcall, ret 0x14).
const FN_POST_FINE_REACTION_NEWS: usize = 0x5a74b0;
/// `DISPUTE::file_complaint(person, club, a, b)` (thiscall on `G_DISPUTE`, ret 0x10).
const FN_DISPUTE_FILE_COMPLAINT: usize = 0x56a100;

/// The news of the player's reaction to the fine, through the exe address (the hook routes it).
unsafe fn fine_reaction_news(player: Staff, reason: u32, reaction: u32, period: u32, effects: u32) {
    let post: unsafe extern "stdcall" fn(usize, u32, u32, u32, u32) = core::mem::transmute(FN_POST_FINE_REACTION_NEWS);
    post(player.addr(), reason, reaction, period, effects);
}

/// The player complains to the Player Status Committee (`DISPUTE::file_complaint` on `G_DISPUTE`).
unsafe fn fine_complaint(player: Staff, club: Club, a: u32, b: u32) {
    let file: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_DISPUTE_FILE_COMPLAINT);
    file(read_usize(G_DISPUTE), player.addr(), club.addr(), a, b);
}

/// The contract's morale moved by `delta` (signed byte sums), kept within −100..100.
unsafe fn fine_move_morale(contract: Contract, delta: i8) {
    let sum = contract.morale() as i32 + delta as i32;
    contract.set_morale(if sum > 100 { 100 } else if sum < -100 { -100 } else { contract.morale().wrapping_add(delta) });
}

/// A board confidence word moved by `delta` (16-bit sum), kept within 1..10000.
unsafe fn fine_move_confidence(at: usize, delta: i16) {
    let v = read_i16(at).wrapping_add(delta);
    write_i16(at, if v < 1 { 1 } else if v > 10000 { 10000 } else { v });
}

/// The board's fine news (0x1784, slot 0 = `slot0`) to the club's manager, from a local news item.
unsafe fn fine_board_news(club: Club, slot0: u32) {
    let mut item = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = item.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1784, 0);
    exe::newsitem_set_slot(n, 0, slot0);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

/// Whether the player's profile can take the fine to the committee: a player sheet with current reputation above
/// 6750, world reputation above `world` and ability above 120, aged 20..33.
unsafe fn fine_star_profile(player: Staff, world: i16) -> bool {
    let sheet = player.player();
    !sheet.is_null() && sheet.current_reputation() as i16 > 0x1a5e && sheet.world_reputation() > world && sheet.current_ability() > 0x78
        && (player.age() as i8) > 0x13 && (player.age() as i8) < 0x22
}

/// Whether an unsettled contract (flags or grievances, morale below −25, wanting to leave) backs a complaint.
unsafe fn fine_unsettled(contract: Contract) -> bool {
    (exe::staff_contract_get_flags(contract) != 0 || contract.grievances() != 0) && contract.morale() < -25
        && exe::staff_contract_wants_to_leave(contract) != 0
}

/// A fair fine with no effect: the news (reaction 0), event flag 0x200 on the contract; a star with the profile
/// (`fine_star_profile`, world reputation above 3750), ambition above 15, professionalism, loyalty and temperament below
/// 6, unsettled and random(loyalty + 2) = 0 complains (4, 0); otherwise grievance 0x20 with random(pressure) = 0,
/// loyalty below 5 and random(temperament × (temperament + 5)) = 0 goes absent (`exe::awol_start_absence`).
unsafe fn fine_shrugged_off(player: Staff, club: Club, contract: Contract, period: u32, reason: u32) {
    fine_reaction_news(player, reason, 0, period, 0);
    exe::staff_contract_apply_event_flags(contract.addr(), 0x200, 0);
    if fine_star_profile(player, 0xea6) {
        if player.ambition() > 15 && player.professionalism() < 6 && player.loyalty() < 6 && player.temperament() < 6
            && fine_unsettled(contract) && exe::utils_random(player.loyalty() as i32 + 2) == 0 {
            covered!("sx09c_7 shrugged off complaint");
            fine_complaint(player, club, 4, 0);
        }
    } else if contract.grievances() & 0x20 != 0 && exe::utils_random(player.pressure() as i32) == 0 && player.loyalty() < 5 {
        let t = player.temperament() as i32;
        if exe::utils_random((t + 5) * t) == 0 {
            exe::awol_start_absence(read_usize(G_AWOL), player, 0usize, 0);
        }
    }
}

/// The effect of a fair fine, by reason (`period` the signed byte): returns (effects bits, the training-state change).
/// 1: professionalism +1 (bit 1) below period + 8 when random(professionalism) ≠ 0, and a sulking star (contract morale
/// below −25, work rate and professionalism below 10, reputation over the club's + 250) works harder (bit 2, morale 0)
/// when random(professionalism) ≠ 0 or bit 1 was set. 2: temperament +1 (bit 4) below period + 8 when
/// random(temperament) = 0. 3: aggression −1 (bit 8) above 14 − period when random(aggression) > random(21), then
/// temperament as in 2 below period + 10. 4: determination +1 (0x10) and work rate +1 (bit 2) below period + 14 by
/// draws, then byte +0xa of the player's training state row halved (above 12, when pressure > random(30): change 1)
/// or raised by random(10) + 1 (below 30, when pressure < random(10): change 2).
unsafe fn fine_effects(player: Staff, club: Club, contract: Contract, period: i32, reason: u8) -> (u8, u8) {
    let sheet = player.player();
    let mut effects = 0u8;
    let mut change = 0u8;
    match reason {
        1 => {
            if (player.professionalism() as i32) < period + 8 && exe::utils_random(player.professionalism() as i32) != 0 {
                effects = 1;
                player.set_professionalism(player.professionalism().wrapping_add(1)); // sx09c_7: professionalism up
            }
            if !sheet.is_null() && contract.morale() < -25 && sheet.work_rate() < 10 && player.professionalism() < 10
                && sheet.current_reputation() as i16 as i32 > club.reputation() as i32 + 250
                && (exe::utils_random(player.professionalism() as i32) != 0 || effects != 0) {
                sheet.set_work_rate(sheet.work_rate().wrapping_add(1));
                effects |= 2;
                contract.set_morale(0);
            }
        }
        2 => {
            if (player.temperament() as i32) < period + 8 && exe::utils_random(player.temperament() as i32) == 0 {
                effects = 4;
                player.set_temperament(player.temperament().wrapping_add(1));
            }
        }
        3 => {
            if !sheet.is_null() && sheet.aggression() as i32 > 0xe - period {
                let r = exe::utils_random(sheet.aggression() as i32);
                if r > exe::utils_random(0x15) {
                    effects = 8;
                    sheet.set_aggression(sheet.aggression().wrapping_sub(1));
                }
            }
            if (player.temperament() as i32) < period + 10 && exe::utils_random(player.temperament() as i32) == 0 {
                player.set_temperament(player.temperament().wrapping_add(1));
                effects |= 4;
            }
        }
        4 => {
            let limit = period + 0xe;
            if (player.determination() as i32) < limit {
                let r = exe::utils_random(player.determination() as i32);
                if r < exe::utils_random(0x15) {
                    let r = exe::utils_random(player.professionalism() as i32);
                    if r > exe::utils_random(10) {
                        effects = 0x10;
                        player.set_determination(player.determination().wrapping_add(1));
                    }
                }
            }
            if !sheet.is_null() && (sheet.work_rate() as i32) < limit && (sheet.work_rate() as i32) < exe::utils_random(0x15) {
                let r = exe::utils_random(player.professionalism() as i32);
                if r > exe::utils_random(10) {
                    sheet.set_work_rate(sheet.work_rate().wrapping_add(1));
                    effects |= 2;
                }
            }
            let row = read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_u32(player.addr()) as usize).wrapping_mul(staff_state::SIZE)) + 0xa;
            let r = exe::utils_random(0x1e);
            if player.pressure() as i32 > r && read_u8(row) > 0xc {
                write_u8(row, ftol(read_u8(row) as f64 * read_f64(rdata::F64_0_5)) as u8);
                change = 1;
            } else if (player.pressure() as i32) < exe::utils_random(10) && read_u8(row) < 0x1e {
                let r = exe::utils_random(10);
                write_u8(row, read_u8(row).wrapping_add((r as u8).wrapping_add(1)));
                change = 2;
            }
        }
        _ => {}
    }
    (effects, change)
}

/// A fair fine: the players of the squad lose contract flag 2 (random(professionalism) ≠ 0); unless the player's
/// professionalism and the reason make it certain (above 2 with reason 2 or 3, above 4 with reason 1) and
/// random(professionalism) = 0, he shrugs it off (`fine_shrugged_off`); otherwise the effects (`fine_effects`), the news
/// (reaction 2, the effects) and, after a training-state change at a human club with an assistant whose rating
/// random() beats random(6) + 5, a 17-byte record in the list at `this` + 4: the assistant, the player, the change and
/// today + random(5) + 2 days.
unsafe fn fine_fair(this: usize, player: Staff, club: Club, contract: Contract, period: u32, reason: u32) {
    let cm = read_usize(G_CONTRACT_MANAGER);
    for k in 0..50 {
        let q = club.squad(k);
        if q != 0 && q != player.addr() {
            let qc = exe::contract_manager_get_contract_for_club(cm, q, club);
            if qc != 0 && exe::staff_contract_get_flags(qc) & 2 != 0 && exe::utils_random(player.professionalism() as i32) != 0 {
                exe::staff_contract_clear_flags(qc, 2);
            }
        }
    }
    let prof = player.professionalism();
    let r = reason as u8;
    let certain = (prof > 2 && r == 2) || (prof > 4 && r == 1) || (prof > 2 && r == 3);
    if !certain && exe::utils_random(prof as i32) == 0 {
        fine_shrugged_off(player, club, contract, period, reason);
        return;
    }
    covered!("sx09c_7 fair fine effects");
    let (effects, change) = fine_effects(player, club, contract, period as u8 as i8 as i32, r);
    fine_reaction_news(player, reason, 2, period, effects as u32);
    if change == 0 || exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
        return;
    }
    let assistant = club.assistant_manager();
    if assistant == 0 {
        return;
    }
    let np = Staff(assistant).non_player();
    if np.is_null() {
        return;
    }
    let r = exe::utils_random(exe::non_player_attribute_rating(np, np.man_handling() as i8) as i32);
    if r <= exe::utils_random(6) + 5 {
        return;
    }
    let mut rec = [0u8; 0x11];
    let p = rec.as_mut_ptr() as usize;
    let mut date: RawDate = [0; 8];
    date_default(&mut date);
    write_u32(p, read_u32(assistant));
    write_u32(p + 4, read_u32(player.addr()));
    write_u8(p + 8, change);
    let mut t = [0u8; 8];
    let d = exe::date_add_days(G_CURRENT_DATE, t.as_mut_ptr() as usize, (exe::utils_random(5) + 2) as u32);
    date_assign(&mut date, d);
    rec[9..].copy_from_slice(&date);
    exe::llist_add_tail(read_usize(this + 4), p, 0x11);
}

/// An unfair fine, the board's side: the club's fine count (byte per club in the array at `this` + 8, up to 100, never
/// negative) goes up; at a human club with random(3) ≠ 0 the manager's loyalty drops by random(period + 1) (at least 1).
/// With the board's confidence below 4250: above the chairman's threshold (patience / 4 + 5, or 8) the board acts and
/// the fine stops here (false): with a chairman, for reason 0 one fine over the threshold queues the departure record
/// (6 bytes: 1, 8, the club, in the list at manager manager + 4, flag 2 on the board row) and posts the news (0);
/// otherwise confidence −100 × the chairman's discipline (no chairman: −500), staff and fourth confidence −250 and the
/// news (0). At the threshold: −100 on the two and the news (1). Below it (or with confidence 4250 or more): −50.
unsafe fn fine_unfair_board(club: Club, club_id: u32, period: u32, reason: u32, counts: usize) -> bool {
    let chairman = club.chairman();
    let threshold: i8 = if !chairman.is_null() && !chairman.non_player().is_null() {
        (chairman.non_player().patience() as i8 >> 2).wrapping_add(5)
    } else {
        8
    };
    let count = counts + club_id as usize;
    if read_i8(count) < 100 {
        write_u8(count, read_u8(count).wrapping_add(1));
    }
    if read_i8(count) < 0 {
        write_u8(count, 0);
    }
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 && exe::utils_random(3) != 0 {
        let manager = club.manager();
        let r = exe::utils_random(period as u8 as i8 as i32 + 1);
        manager.set_loyalty(manager.loyalty().wrapping_sub(r as i8));
        if club.manager().loyalty() < 1 {
            club.manager().set_loyalty(1);
        }
    }
    let mm = read_usize(G_MANAGER_MANAGER);
    let board = || read_usize(mm).wrapping_add((read_u32(club.addr()) as usize).wrapping_mul(0x49)); // the board rows
    let confidence = read_i16(board() + board_record::CONFIDENCE);
    let n = read_i8(count);
    if mm != 0 && confidence < 0x109a && n > threshold {
        let np = if chairman.is_null() { NonPlayer(0) } else { chairman.non_player() };
        if !np.is_null() {
            if reason as u8 == 0 && n as i32 == threshold as i32 + 1 {
                covered!("sx09c_7 board departure queued");
                let mut rec = [0u8; 6];
                rec[0] = 1;
                rec[1] = 8;
                rec[2..].copy_from_slice(&club_id.to_le_bytes());
                let list = read_usize(mm + 4);
                if list != 0 {
                    exe::llist_add_tail(list, rec.as_ptr() as usize, 6);
                    let row = read_usize(mm).wrapping_add((club_id as usize).wrapping_mul(0x49));
                    write_u8(row + board_record::FLAGS, read_u8(row + board_record::FLAGS) | 2);
                }
                fine_board_news(club, 0);
                return false;
            }
            fine_move_confidence(board() + board_record::CONFIDENCE, (np.discipline() as i8 as i16).wrapping_mul(-100));
        } else {
            fine_move_confidence(board() + board_record::CONFIDENCE, -500);
        }
        fine_move_confidence(board() + board_record::STAFF_CONFIDENCE, -250); // sx09c_7: the staff lose confidence
        fine_move_confidence(board() + board_record::CONFIDENCE_4, -250);
        fine_board_news(club, 0);
        return false;
    }
    if mm != 0 && confidence < 0x109a && n == threshold {
        fine_move_confidence(board() + board_record::STAFF_CONFIDENCE, -100);
        fine_move_confidence(board() + board_record::CONFIDENCE_4, -100);
        fine_board_news(club, 1);
        return true;
    }
    fine_move_confidence(board() + board_record::STAFF_CONFIDENCE, -50);
    fine_move_confidence(board() + board_record::CONFIDENCE_4, -50);
    true
}

/// The squad's reaction to an unfair fine: each team-mate with a sheet (not the player) whose loyalty, lowered for a
/// player far above him (reputation + 1250 below the player's) and for a star (over 3250 and over the club's),
/// and lowered again (or left above 15) when the player is favoured by him (`exe::staff_is_favoured_by`), is 0 or
/// wins random(loyalty) = 0, takes his side: contract event flag 0x400, the first one posts the news (reaction 1), and
/// with random(professionalism) = 0 he dislikes the manager.
unsafe fn fine_squad_reaction(player: Staff, club: Club, period: u32, reason: u32) {
    let cm = read_usize(G_CONTRACT_MANAGER);
    let mut first = true;
    for k in 0..50 {
        let q = Staff(club.squad(k));
        if q.is_null() || q.player().is_null() || q.addr() == player.addr() {
            continue;
        }
        let mut l = q.loyalty();
        let own = player.player().current_reputation() as i16;
        if (q.player().current_reputation() as i16 as i32) + 0x4e2 < own as i32 {
            l = if l > 5 { l.wrapping_sub(2) } else { l.wrapping_sub(1) };
        }
        if own > 0xcb2 && own > club.reputation() {
            l = if l > 5 { l - 2 } else if l != 0 { l.wrapping_sub(1) } else { l };
        }
        let mut sides = false;
        if exe::staff_is_favoured_by(player, q) != 0 {
            if l > 15 {
                sides = true;
            } else if l > 5 {
                l -= 2;
            } else if l == 0 {
                sides = true;
            } else {
                l = l.wrapping_sub(1);
            }
        }
        if !sides && l != 0 && exe::utils_random(l as i32) != 0 {
            continue;
        }
        covered!("sx09c_7 team-mate takes sides");
        let qc = exe::contract_manager_get_contract_for_club(cm, q, club);
        if qc != 0 {
            exe::staff_contract_apply_event_flags(qc, 0x400, 0);
        }
        if first {
            fine_reaction_news(player, reason, 1, period, 0);
            first = false;
        }
        if exe::utils_random(q.professionalism() as i32) == 0 {
            exe::staff_add_disliked_staff(q, club.manager(), 1);
        }
    }
}

/// An unfair fine, the player's side. Returns false when the fine stops before its record (a one-week fine that the
/// player lets go: random(professionalism) ≠ 0 or random(2) ≠ 0).
unsafe fn fine_unfair_player(player: Staff, club: Club, period: u32, reason: u32) -> bool {
    let contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club));
    if contract.is_null() {
        return true;
    }
    let p = period as u8;
    let sheet = player.player();
    if contract.grievances() == 0 && (sheet.is_null() || sheet.current_reputation() as i16 <= player.club_job().reputation()) {
        // he takes it: morale −15 or −5 per week
        fine_reaction_news(player, reason, 0, period, 0);
        exe::staff_contract_apply_event_flags(contract.addr(), 0x200, 0);
        let per = if exe::utils_random(player.loyalty() as i32) == 0 { 0xf1u8 } else { 0xfb };
        fine_move_morale(contract, p.wrapping_mul(per) as i8);
    } else {
        fine_move_morale(contract, p.wrapping_mul(0xec) as i8); // sx09c_7: morale −20 per week
        if exe::utils_random(player.loyalty() as i32) == 0 {
            covered!("sx09c_7 player resents the fine");
            exe::staff_contract_apply_event_flags(contract.addr(), 0x200, 0);
            if exe::utils_random(player.professionalism() as i32 / 2) == 0 {
                exe::staff_add_disliked_club(player, club, 1);
            }
            if exe::utils_random(player.professionalism() as i32 / 2) == 0 {
                exe::staff_add_disliked_staff(player, club.manager(), 1);
            }
            let chairman = club.chairman();
            if !chairman.is_null() && exe::utils_random(chairman.temperament() as i32) == 0 {
                exe::staff_add_disliked_staff(chairman, player, 1);
            }
            if p == 1 && (exe::utils_random(player.professionalism() as i32) != 0 || exe::utils_random(2) != 0) {
                return false;
            }
            fine_squad_reaction(player, club, period, reason);
        } else {
            fine_reaction_news(player, reason, 0, period, 0);
            exe::staff_contract_apply_event_flags(contract.addr(), 0x200, 0);
            let per = if exe::utils_random(player.loyalty() as i32) == 0 { 0xf1u8 } else { 0xfb };
            fine_move_morale(contract, p.wrapping_mul(per) as i8);
        }
        if contract.grievances() & 0x20 != 0 && exe::utils_random(player.pressure() as i32) == 0 && player.loyalty() < 10 {
            exe::awol_start_absence(read_usize(G_AWOL), player, 0usize, 0);
        }
    }
    if fine_star_profile(player, 0xcb2) && player.ambition() > 10 && player.professionalism() < 10 && player.loyalty() < 10
        && player.temperament() < 10 && fine_unsettled(contract)
        && (reason as u8 == 0 || exe::utils_random(player.loyalty() as i32) == 0) {
        covered!("sx09c_7 unfair fine complaint");
        fine_complaint(player, club, 3, 1);
    }
    true
}

/// Fines a player of a club for `period` weeks for `reason`, from the AI manager's discipline (AWOL) and the
/// Discipline screen.
///
/// `Fine::fine_player(player, period, reason)`: a player with a sheet, a valid id, a contract
/// (`exe::contract_manager_get_contract`, on `G_CONTRACT_LOOKUP_RESULT`) and a club contract
/// (`exe::contract_manager_get_contract_for_club`); reason 0 or 1 for a player absent without leave
/// (`exe::awol_is_player_awol`) becomes 5 (its low byte). Whether he finds it fair (`Fine::is_fine_justified`):
/// - fair: `fine_fair`;
/// - unfair: the board (`fine_unfair_board`; when it acts the fine stops there) and the player (`fine_unfair_player`).
/// Finally a 14-byte record in the list at `this`: the player, the reason, the verdict and today + `period` weeks.
///
/// Defects of the original: the effects byte goes to the news in a dword whose upper bytes are never set (the news
/// reads the byte); with no manager manager the original reads address 0 (never happens).
/// Original: `Fine::fine_player` (0x5a5980, Fine.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn fine_player(this: usize, player: usize, period: u32, reason: u32) {
    let mut date: RawDate = [0; 8];
    date_default(&mut date);
    let player = Staff(player);
    if player.is_null() || player.player().is_null() || read_i32(player.addr()) >= read_i32(G_STAFF_COUNT) {
        return;
    }
    let cm = read_usize(G_CONTRACT_MANAGER);
    let k = exe::contract_manager_get_contract(cm, player);
    write_usize(G_CONTRACT_LOOKUP_RESULT, k);
    if k == 0 {
        return;
    }
    let club = club_at(Contract(k).club() as isize);
    if club.is_null() {
        return;
    }
    let k = exe::contract_manager_get_contract(cm, player); // looked up again, as the original
    write_usize(G_CONTRACT_LOOKUP_RESULT, k);
    let club_id = read_u32(club_at(Contract(k).club() as isize).addr());
    let contract = Contract(exe::contract_manager_get_contract_for_club(cm, player, club));
    if contract.is_null() {
        return;
    }
    let mut reason = reason;
    if reason as u8 <= 1 && exe::awol_is_player_awol(read_usize(G_AWOL), player, 0) != 0 {
        reason = (reason & !0xff) | 5; // sx09c_7: absent without leave
    }
    let justified: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u8 = core::mem::transmute(FN_FINE_IS_FINE_JUSTIFIED);
    let fair = justified(this, player.addr(), period, reason);
    covered!("sx09c_7 fine");
    if fair != 0 {
        fine_fair(this, player, club, contract, period, reason);
    } else if !fine_unfair_board(club, club_id, period, reason, read_usize(this + 8)) || !fine_unfair_player(player, club, period, reason) {
        return;
    }
    let mut t = [0u8; 8];
    let d = exe::date_add_weeks(G_CURRENT_DATE, &mut t, period);
    date_assign(&mut date, d);
    let mut rec = [0u8; 0xe];
    rec[..4].copy_from_slice(&read_u32(player.addr()).to_le_bytes());
    rec[4] = reason as u8;
    rec[5] = fair;
    rec[6..].copy_from_slice(&date);
    exe::llist_add_tail(read_usize(this), rec.as_ptr() as usize, 0xe); // sx09c_7: the fine recorded
}

/// Comparer for `fine_player`: only the original runs (the fine moves the squad, the board, the news and the
/// disputes); its list at `this` must grow by at most one record.
pub(crate) unsafe extern "thiscall" fn compare_fine_player(this: usize, player: usize, period: u32, reason: u32) {
    let fines = read_usize(this);
    let before = read_i16(fines + llist::COUNT);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I0%%)(this, player, period, reason);
    let after = read_i16(fines + llist::COUNT);
    SCOREBOARD.record(after == before || after == before.wrapping_add(1), || format!("fine_player: fines {} -> {}", before, after));
}
