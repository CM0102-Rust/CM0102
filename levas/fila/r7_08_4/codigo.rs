// --- r7_08_4: whether a computer club wants a player it is offered in an exchange (shortlist_manager.cpp) ---

/// The margin of a price over a value on the club's reputation scale: y = 1 − diff / (reputation² / 10); below 0.01 it
/// is 0.01; above 1 (3 at most, and 3 when y is not a number, as the x87 comparisons fall) it becomes y / 3 + 0.67.
pub(crate) unsafe fn wanted_offer_margin(diff: i32, reputation: i16) -> f64 {
    let scale = (reputation as i32 * reputation as i32) / 10;
    let mut y = read_f64(rdata::F64_1) - diff as f64 / scale as f64;
    if read_f64(rdata::F64_0_01) > y {
        return read_f64(rdata::F64_0_01);
    }
    if !(read_f64(rdata::F64_3) >= y) {
        y = read_f64(rdata::F64_3);
    } else if !(y > read_f64(rdata::F64_1)) {
        return y;
    }
    y * read_f64(rdata::F64_0_3333333333) + read_f64(rdata::F64_0_67)
}

/// The score a slot's current target sets for `club_wants_player` (0x843a6b..0x843bf3): his score and value
/// (`exe::shortlist_manager_value_player_for_club` with the slot's valuation, 1), his club's asking price (human:
/// `exe::contract_manager_get_human_asking_price`; AI: `exe::contract_manager_calc_asking_price` 0, 1, 1; no club: 0),
/// the score × `wanted_offer_margin`. `None`: the target keeps the slot. Otherwise the score to beat is the valuation
/// less his injury (`shortlist_offer_injury_penalty`).
unsafe fn wanted_target_score(this: usize, club: Club, target: Staff, slot: ShortlistSlot) -> Option<i32> {
    let cm = read_usize(G_CONTRACT_MANAGER);
    let valuation = slot.valuation();
    let mut score: i32 = 0;
    let value = exe::shortlist_manager_value_player_for_club(this, club, target, slot, &mut score, valuation, 1);
    let owner = Club(club_of_staff_contract(target));
    let ask = if owner.is_null() {
        0
    } else if exe::human_manager_is_human_club(owner, 0, 1, 0, 0) != 0 {
        exe::contract_manager_get_human_asking_price(cm, owner, club, target)
    } else {
        exe::contract_manager_calc_asking_price(cm, owner, club, target, 0, 1, 1)
    };
    let score = ftol(score as f64 * wanted_offer_margin(ask.wrapping_sub(value), club.reputation()));
    if score >= slot.valuation() {
        return None;
    }
    Some(shortlist_offer_injury_penalty(target, score.max(valuation)))
}

/// Whether a slot of the club's shortlist can take the player at all (0x84396d..0x843a6b): a player slot (job 11, not a
/// loan slot) without an offer, whose position the player covers, not one whose target is its replacement, dated before
/// today; a status-5 slot (`FLAGS` & 0x1e = 0xa) takes him up to 24, and one of another nation (his and his club job's
/// nation both differ from the club's) only with a current reputation of 5000 and his reputation as the club's manager
/// sees it (`exe::database_get_staff_reputation_for_viewer`) at least random(1000) + 500; a work permit needed
/// (`exe::transfer_manager_player_needs_work_permit` 1) outside status 1-3 rules the slot out.
unsafe fn wanted_slot_open(player: Staff, club: Club, slot: ShortlistSlot, positions: u16) -> bool {
    if slot.flags() & 1 != 0 || read_u8(slot.addr()) != 0xb || slot.offer_ref() != -1 {
        return false;
    }
    let pos = read_u16(slot.addr() + shortlist_slot::POSITION);
    if pos & positions != pos {
        return false;
    }
    let target = read_i32(slot.addr() + shortlist_slot::TARGET_STAFF_ID);
    if target == read_i32(slot.addr() + shortlist_slot::REPLACEMENT_STAFF_ID) && target != -1 {
        return false;
    }
    if !fill_slot_date_before(slot.addr() + shortlist_slot::DATE, G_CURRENT_DATE) {
        return false;
    }
    if slot.flags() & 0x1e == 0xa {
        if player.age() as i8 > 24 {
            return false;
        }
        let nation = club.nation().addr();
        let job = player.club_job();
        if player.nation().addr() != nation && (job.is_null() || job.nation().addr() != nation) {
            if player.player().current_reputation() < 5000 {
                return false;
            }
            let seen = exe::database_get_staff_reputation_for_viewer(player, club.manager(), 0) as i32;
            if seen < exe::utils_random(1000) + 500 { // r7_08_4: the reputation a foreign youngster needs
                return false;
            }
            covered!("r7_08_4 foreign youngster seen");
        }
    }
    if exe::transfer_manager_player_needs_work_permit(read_usize(G_TRANSFER_MANAGER), player, club, 1) != 0 && !matches!(slot.flags() & 0x1e, 2 | 4 | 6) {
        return false;
    }
    true
}

/// Returns 1 when a computer club wants a player it is offered (an exchange player in a bid), at the given value: he fits
/// a slot of the club's shortlist better than its current target; with `commit` he becomes that slot's target.
///
/// `Shortlist_Manager::club_wants_player(player, club, value, commit)`, from the human's exchange offer
/// (`open_add_part_exchange_player_screen` 0x47c0b0) and the computer's answer to it (`ai_respond_to_bid` 0x8c8090).
/// The seller is the club of the player's contract (`exe::contract_manager_get_contract`, kept in
/// `G_CONTRACT_LOOKUP_RESULT`). The position mask (`exe::formation_get_position_mask`) gains the derived bits 2→4,
/// 8→0x10, 0x20→0x10, 0x80→0x100 and 0x800→0x400. 0: the club's shortlist record (`CLUB_SHORTLISTS` + 9 × id) not
/// active (`club_shortlist::ACTIVE` ≠ 1) or with `club_shortlist::DAYS` 100 or more. A player the club already has in
/// the sorted (person, club) pairs gives 1 at once. 0: the work permit rules (0x84b180, with the club of his contract)
/// or a player the club cannot bid for (`exe::transfer_manager_can_club_bid_for_player` 0, 1).
///
/// Each of the 40 slots (`wanted_slot_open`): the score to beat is the current target's (`wanted_target_score`; a target
/// that keeps the slot moves on to the next slot) or, without one, the valuation. The player: his score and value for
/// the club (`exe::shortlist_manager_value_player_for_club` with the valuation, 0); at a seller in crisis
/// (`finance::CRISIS_STATE` 1) his value is at most the human seller's value (`exe::contract_manager_calc_player_value`,
/// called again when it is the lower) or his `.value()`; his score × `wanted_offer_margin(value offered − value)` less
/// his injury (`shortlist_offer_injury_penalty`) must reach the score to beat, and his interest
/// (`exe::shortlist_manager_staff_interest_in_joining_club` 1, 0, the slot's status) must reach 2.75. Wanted: with
/// `commit` he becomes the slot's target, the old target gets `exe::shortlist_manager_update_staff_status` 3 and he
/// enters the pairs (`add_shortlist_entry`); 1. No slot: 0.
///
/// Unlike its sibling `try_add_player_to_club_shortlist` it takes slots without a position (0 is inside any mask).
/// Original: `Shortlist_Manager::club_wants_player` (0x843800, shortlist_manager.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn club_wants_player(this: usize, player: usize, club: usize, value: i32, commit: u32) -> u32 {
    let player = Staff(player);
    let club = Club(club);
    let cm = read_usize(G_CONTRACT_MANAGER);
    let ft = read_usize(G_FINANCE_TABLE);
    let seller = Club(club_of_staff_contract(player));
    let entry = read_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add((read_u32(club.addr()) as usize).wrapping_mul(9));
    // simplification: 0x5b7d20 returns only ax and the slots use 16 bits of the mask
    let mut m = exe::formation_get_position_mask(player.player());
    for (has, add) in [(2u16, 4u16), (8, 0x10), (0x20, 0x10), (0x80, 0x100), (0x800, 0x400)] {
        if m & has != 0 {
            m |= add;
        }
    }
    if read_u8(entry + club_shortlist::ACTIVE) != 1 || read_u8(entry + club_shortlist::DAYS) >= 100 {
        return 0;
    }
    if read_usize(this) != 0 && fill_slot_pair_listed(this, player, club) {
        covered!("r7_08_4 already listed");
        return 1;
    }
    if exe::shortlist_manager_passes_work_permit_check(this, player, club_of_staff_contract(player), club) == 0 {
        return 0;
    }
    if exe::transfer_manager_can_club_bid_for_player(read_usize(G_TRANSFER_MANAGER), player, club, core::ptr::null_mut(), 1) == 0 {
        return 0;
    }
    for i in 0..40usize {
        let slot = ShortlistSlot(read_usize(entry).wrapping_add(i * 0x25));
        if !wanted_slot_open(player, club, slot, m) {
            continue;
        }
        let valuation = slot.valuation();
        let target = read_i32(slot.addr() + shortlist_slot::TARGET_STAFF_ID);
        let (previous, score) = if target != -1 {
            let tp = staff_at(target as isize);
            match wanted_target_score(this, club, tp, slot) {
                Some(s) => (tp, s),
                None => {
                    covered!("r7_08_4 target kept");
                    continue;
                }
            }
        } else {
            (Staff(0), valuation)
        };
        let mut cand: i32 = 0;
        let mut worth = exe::shortlist_manager_value_player_for_club(this, club, player, slot, &mut cand, valuation, 0);
        if !seller.is_null() && exe::financetable_get_club_finance(ft, seller) != 0 && read_u8(exe::financetable_get_club_finance(ft, seller) + finance::CRISIS_STATE) == 1 {
            covered!("r7_08_4 seller in crisis");
            if exe::human_manager_is_human_club(seller, 0, 1, 0, 0) != 0 {
                if worth >= exe::contract_manager_calc_player_value(cm, player, seller, 0, -1, 0) {
                    worth = exe::contract_manager_calc_player_value(cm, player, seller, 0, -1, 0);
                }
            } else if worth >= player.value() as i32 {
                worth = player.value() as i32;
            }
        }
        let cand = ftol(cand as f64 * wanted_offer_margin(value.wrapping_sub(worth), club.reputation()));
        if shortlist_offer_injury_penalty(player, cand) < score {
            continue;
        }
        let status = ((slot.flags() >> 1) & 0xf) as u32;
        if !(exe::shortlist_manager_staff_interest_in_joining_club(this, player, club, 1, 0, status, 0) >= read_f64(rdata::F64_2_75)) { // r7_08_4: the interest needed
            continue;
        }
        covered!("r7_08_4 wanted");
        if commit != 0 {
            write_u32(slot.addr() + shortlist_slot::TARGET_STAFF_ID, read_u32(player.addr()));
            if !previous.is_null() {
                exe::shortlist_manager_update_staff_status(this, previous, club, 3, 0);
            }
            let add_entry: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_ADD_SHORTLIST_ENTRY); // add_shortlist_entry
            add_entry(this, player.addr(), club.addr()); // r7_08_4: into the pairs
        }
        return 1;
    }
    0
}

/// Comparer for `club_wants_player`: with `commit` the original only (it rewrites the slot's target, the old target's
/// status and the pairs); otherwise the original and the copy side by side on the answer, the cached contract and the
/// random generator.
pub(crate) unsafe extern "thiscall" fn compare_club_wants_player(this: usize, player: usize, club: usize, value: i32, commit: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32, u32) -> u32>(%%I0%%);
    if commit != 0 {
        return o(this, player, club, value, commit);
    }
    let est = crate::rng::STATE_REGIONS;
    let regions = [(G_CONTRACT_LOOKUP_RESULT, 4usize), est[0], est[1]];
    let (r, n, same, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this, player, club, value, 0),
        || crate::rng::without_counting(|| club_wants_player(this, player, club, value, 0)));
    SCOREBOARD.record(same && r == n, || format!("club_wants_player(person {:#x}, club {:#x}, value {}) original={} rust={} {:x?} x {:x?}",
        read_u32(player), read_u32(club), value, r, n, mo, mr));
    r
}
