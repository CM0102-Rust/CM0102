// ---------- batch %%LEVA%%: an AI club shortlisting a player it is offered (shortlist_manager.cpp) ----------

/// The margin of a price over a value on a slot's scale: y = 1 − (asking − value) / (ceiling × reputation factor);
/// below 0.01 it is 0.01; above 1 (3 at most) it becomes y / 3 + 0.67, as `try_add_player_to_club_shortlist` scales
/// both the current target's and the candidate's scores.
unsafe fn shortlist_offer_margin(diff: i32, max_price: i32, factor: f64) -> f64 {
    let mut y = read_f64(rdata::F64_1) - diff as f64 / (max_price as f64 * factor);
    if read_f64(rdata::F64_0_01) > y {
        return read_f64(rdata::F64_0_01);
    }
    if read_f64(rdata::F64_3) < y {
        y = read_f64(rdata::F64_3);
    } else if !(y > read_f64(rdata::F64_1)) {
        return y;
    }
    y * read_f64(rdata::F64_0_3333333333) + read_f64(rdata::F64_0_67)
}

/// The injury penalty of a shortlist score: with more than 15 days out (`injury_entry::DAYS_LEFT` of
/// `exe::injury_get_player_injury`), (days − 15)² comes off the score.
unsafe fn shortlist_offer_injury_penalty(p: Staff, score: i32) -> i32 {
    let inj = exe::injury_get_player_injury(read_usize(G_INJURY), p);
    if inj != 0 && read_i16(inj + injury_entry::DAYS_LEFT) > 15 {
        let d = read_i16(inj + injury_entry::DAYS_LEFT) as i32 - 15;
        return score.wrapping_sub(d.wrapping_mul(d));
    }
    score
}

/// Places a player a club is offered (or comes across) in one of the forty slots of the club's shortlist, and returns 1
/// when he is placed or already on it (0 otherwise).
///
/// `try_add_player_to_club_shortlist(player, club, positions)` (the shortlist manager in ecx). A local STAFF_CONTRACT
/// (`exe::new_staff_contract`) receives the desired contract. Leaves with 0: a human club
/// (`exe::human_manager_is_human_club` 0, 1); a human club (1, 1) whose manager is on holiday
/// (`exe::human_manager_is_on_holiday`) with holiday option 4 (`exe::human_manager_get_holiday_option`) equal to 1, or
/// equal to 2 with the player off the human's scout list (`exe::scout_manager_is_player_shortlisted`; the list is
/// remembered for the foreign youngster rule below); the club's shortlist record (`CLUB_SHORTLISTS` + 9 × id) not active
/// (`club_shortlist::ACTIVE` ≠ 1) or with `club_shortlist::DAYS` 100 or more. A player the club already has in the
/// sorted (person, club) pairs returns 1 at once. Then 0 for: the work permit rules (0x84b180, with the club of the
/// player's contract, `club_of_staff_contract`, which also stores the contract in `G_CONTRACT_LOOKUP_RESULT`); a player
/// the club cannot bid for (`exe::transfer_manager_can_club_bid_for_player`); a club without finances. The transfer
/// budget (`exe::finance_get_transfer_budget` with 0, 0, 0, 1, 0, at least 0) and the reputation factor (reputation ×
/// 0.0008, at least 1) follow.
///
/// Each slot (`shortlist_slot`, 0x25 bytes) is skipped when: a loan slot (`FLAGS` bit 0); a non-player job (byte +0 ≠
/// 11); an offer made (`OFFER_REF` ≠ −1); no position, or the slot's `POSITION` not all inside `positions`. A
/// compensation deal (0x84b6f0, with the player's contract and the fee) is noted. With a contract, no compensation, a
/// club reputation below 7000 and the player's `.value()` above the slot's ceiling (`MAX_PRICE`) × 1.3: only a
/// transfer-listed (`exe::staff_is_transfer_listed`) player of a human seller whose value − 1,500,000 is below the
/// ceiling goes on. A slot whose target is its replacement (`TARGET_STAFF_ID` = `REPLACEMENT_STAFF_ID` ≠ −1) or whose
/// `DATE` is not before today is skipped. A status-5 slot (`FLAGS` & 0x1e = 0xa): over 24 years skipped; a player of
/// another nation than the club's (the person's nation and his club's nation both differ), not on the human's scout
/// list, needs a current reputation of 1000 and his reputation as the club's manager sees it
/// (`exe::database_get_staff_reputation_for_viewer`) at least 1000 + random(2000). A work permit needed
/// (`exe::transfer_manager_player_needs_work_permit`) outside status 1-3 skips.
///
/// The ceiling: when 0.2 × the budget is at least the slot's ceiling and there is no compensation deal, the ceiling
/// becomes valuation² × 0.01536, at most 0.8 × the budget (the old one is kept to be put back when the player is not
/// taken). The current target (`TARGET_STAFF_ID`, a person with a player sheet): his score and value
/// (`exe::shortlist_manager_value_player_for_club`, with the slot's valuation), the asking price of his contract's club
/// (human: `exe::contract_manager_get_human_asking_price`; AI: `exe::contract_manager_calc_asking_price`; none: 0), the
/// score × the margin (`shortlist_offer_margin`); a score already at the valuation keeps the slot (next slot; the raised
/// ceiling stays); otherwise the score to beat is the valuation less his injury (`shortlist_offer_injury_penalty`).
/// Without a target, the target becomes −1 and the score to beat the valuation.
///
/// The player: his score and value for the club (with the valuation); at a seller in crisis (`finance::CRISIS_STATE` 1)
/// his value is at most the human seller's value (`exe::contract_manager_calc_player_value`, called again when it is
/// the lower) or his `.value()`; the asking price (none with a compensation deal: 0; a compensation fee below it
/// replaces it, which with the 0 never happens); the seller's predicted answer
/// (`exe::shortlist_manager_predict_selling_club_response`) 2 rejects; the score × the margin less his injury below
/// the score to beat rejects; his interest (`exe::shortlist_manager_staff_interest_in_joining_club` with 1, 0 and the
/// status) below 3.15 rejects. A rejection puts the ceiling back and goes to the next slot. Taken: in a purchase slot
/// the desired wage (`exe::staff_contract_default_staff_contract`, kind = the status, job 11, as f32) × max(3.5 /
/// interest (with 0, 0 and the status; at least 0.1), 0.9) above 1.2 × the club's wage cap (`exe::finance_get_max_wage`)
/// ends the whole search with 0 (the ceiling stays). Otherwise he becomes the slot's target, the old target gets
/// `exe::shortlist_manager_update_staff_status` with 3 and he enters the shortlist pairs (0x8423f0): 1. No slot: 0.
/// The SEH frame only matters for exceptions.
/// Original: `Shortlist_Manager::try_add_player_to_club_shortlist` (0x842e10, shortlist_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn try_add_player_to_club_shortlist(this: usize, player: usize, club: usize, positions: u32) -> u32 {
    let mut wanted = [0u8; 0x58];
    exe::new_staff_contract(wanted.as_mut_ptr() as usize);
    try_add_player_body(this, Staff(player), Club(club), positions, wanted.as_mut_ptr() as usize)
}

/// The body of `try_add_player_to_club_shortlist` after the local STAFF_CONTRACT is built.
unsafe fn try_add_player_body(this: usize, player: Staff, club: Club, positions: u32, wanted: usize) -> u32 {
    let mut listed = false;
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 {
        return 0;
    }
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
        let hm = read_usize(G_HUMAN_MANAGER);
        if exe::human_manager_is_on_holiday(hm, exe::database_get_club_manager(club)) != 0 {
            if exe::scout_manager_is_player_shortlisted(read_usize(G_SCOUT_MANAGER), player, exe::database_get_club_manager(club)) != 0 {
                listed = true;
            }
            match exe::human_manager_get_holiday_option(hm, 4, exe::database_get_club_manager(club)) as i8 {
                1 => return 0,
                2 => {
                    if exe::scout_manager_is_player_shortlisted(read_usize(G_SCOUT_MANAGER), player, exe::database_get_club_manager(club)) == 0 {
                        return 0;
                    }
                }
                _ => {}
            }
        }
    }
    let cm = read_usize(G_CONTRACT_MANAGER);
    let tm = read_usize(G_TRANSFER_MANAGER);
    let ft = read_usize(G_FINANCE_TABLE);
    let entry = read_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add((read_u32(club.addr()) as usize).wrapping_mul(9));
    let contract = Contract(exe::contract_manager_get_contract(cm, player));
    let seller = if contract.is_null() { Club(0) } else { transfer_manager_club_by_id(contract.club()) };
    if read_u8(entry + club_shortlist::ACTIVE) != 1 || read_u8(entry + club_shortlist::DAYS) >= 100 {
        return 0;
    }
    if read_usize(this) != 0 && fill_slot_pair_listed(this, player, club) {
        covered!("mke_1 already listed");
        return 1;
    }
    let work_permit: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u8 = core::mem::transmute(0x84b180usize);
    if work_permit(this, player.addr(), club_of_staff_contract(player), club.addr()) == 0 {
        return 0;
    }
    if exe::transfer_manager_can_club_bid_for_player(tm, player, club, core::ptr::null_mut(), 0) == 0 {
        return 0;
    }
    let fin = exe::financetable_get_club_finance(ft, club);
    if fin == 0 {
        return 0;
    }
    let budget = (exe::finance_get_transfer_budget(fin, 0, 0, 0usize, 1, 0) as i32).max(0);
    let mut factor = club.reputation() as i16 as f64 * read_f64(rdata::F64_0_0008);
    if !(factor > read_f64(rdata::F64_1)) {
        factor = read_f64(rdata::F64_1);
    }
    let slots = read_usize(entry);
    // the original's frame keeps these across the slots
    let mut score: i32 = 0;
    let mut cand: i32 = 0;
    let mut fee: i32 = 0;
    for i in 0..40usize {
        let slot = slots.wrapping_add(i * 0x25);
        let mut saved: i32 = -1;
        let flags = read_u8(slot + shortlist_slot::FLAGS);
        if flags & 1 != 0 || read_u8(slot) != 0xb || read_i32(slot + shortlist_slot::OFFER_REF) != -1 {
            continue;
        }
        let pos = read_u16(slot + shortlist_slot::POSITION);
        if pos == 0 || pos & positions as u16 != pos {
            continue;
        }
        let comp = fill_slot_is_compensation_deal(this, slot, contract.addr(), club, &mut fee) != 0;
        if !contract.is_null() && !comp && club.reputation() < 7000
            && player.value() as i32 as f64 > read_i32(slot + shortlist_slot::MAX_PRICE) as f64 * read_f64(rdata::F64_1_3)
        {
            // above the ceiling: only a listed player of a human seller, within 1.5 million
            if seller.is_null() || exe::human_manager_is_human_club(seller, 0, 1, 0, 0) == 0 {
                continue;
            }
            if (player.value() as i32).wrapping_sub(1_500_000) >= read_i32(slot + shortlist_slot::MAX_PRICE) {
                continue;
            }
            if exe::staff_is_transfer_listed(player, 0usize) == 0 {
                continue;
            }
            covered!("mke_1 listed above the ceiling");
        }
        let target = read_i32(slot + shortlist_slot::TARGET_STAFF_ID);
        if target == read_i32(slot + shortlist_slot::REPLACEMENT_STAFF_ID) && target != -1 {
            continue;
        }
        if !fill_slot_date_before(slot + shortlist_slot::DATE, G_CURRENT_DATE) {
            continue;
        }
        let st = read_u8(slot + shortlist_slot::FLAGS) & 0x1e;
        if st == 0xa {
            if player.age() as i8 > 24 {
                continue;
            }
            if !listed {
                let nation = read_usize(club.addr() + club::NATION);
                let job = player.club_job();
                if read_usize(player.addr() + staff::NATION) != nation && (job.is_null() || read_usize(job.addr() + club::NATION) != nation) {
                    if player.player().current_reputation() < 1000 {
                        continue;
                    }
                    let seen = exe::database_get_staff_reputation_for_viewer(player, club.manager(), 0) as i32;
                    if seen < exe::utils_random(2000) + 1000 { // mke1: the reputation a foreign youngster needs
                        continue;
                    }
                    covered!("mke_1 foreign youngster seen");
                }
            }
        }
        if exe::transfer_manager_player_needs_work_permit(tm, player, club, 1) != 0 {
            let st = read_u8(slot + shortlist_slot::FLAGS) & 0x1e;
            if st != 2 && st != 4 && st != 6 {
                continue;
            }
        }
        // a slot's ceiling that the budget passes five times over is raised
        let b = budget as f64;
        if !(b * read_f64(rdata::F64_0_2) < read_i32(slot + shortlist_slot::MAX_PRICE) as f64) && !comp {
            saved = read_i32(slot + shortlist_slot::MAX_PRICE);
            let v = read_i32(slot + shortlist_slot::VALUATION) as f64;
            let m = ftol(v * read_f64(TRY_ADD_F64_0_01536) * v);
            write_i32(slot + shortlist_slot::MAX_PRICE, m);
            let cap = b * read_f64(rdata::F64_0_8); // mke1: the raised ceiling within 0.8 × the budget
            if m as f64 > cap {
                write_i32(slot + shortlist_slot::MAX_PRICE, ftol(cap));
            }
            covered!("mke_1 ceiling raised");
        }
        let max_price = read_i32(slot + shortlist_slot::MAX_PRICE).max(1);
        let valuation = read_i32(slot + shortlist_slot::VALUATION);
        let target = read_i32(slot + shortlist_slot::TARGET_STAFF_ID);
        let previous: Staff;
        if target != -1 && !staff_at(target as isize).player().is_null() {
            let tp = staff_at(target as isize);
            previous = tp;
            let value = exe::shortlist_manager_value_player_for_club(this, club, tp, slot, &mut score, valuation, 1);
            let owner = Club(club_of_staff_contract(tp));
            let ask = if owner.is_null() {
                0
            } else if exe::human_manager_is_human_club(owner, 0, 1, 0, 0) != 0 {
                exe::contract_manager_get_human_asking_price(cm, owner, club, tp)
            } else {
                exe::contract_manager_calc_asking_price(cm, owner, club, tp, 0, 1, 1)
            };
            score = ftol(score as f64 * shortlist_offer_margin(ask.wrapping_sub(value), max_price, factor));
            if score >= read_i32(slot + shortlist_slot::VALUATION) {
                covered!("mke_1 target kept");
                continue;
            }
            if score <= valuation {
                score = valuation;
            }
            score = shortlist_offer_injury_penalty(tp, score);
        } else {
            write_i32(slot + shortlist_slot::TARGET_STAFF_ID, -1);
            score = valuation;
            previous = Staff(0);
        }
        // the player offered
        let mut value = exe::shortlist_manager_value_player_for_club(this, club, player, slot, &mut cand, valuation, 1);
        let mut ask: i32 = 0;
        if !seller.is_null() {
            if exe::financetable_get_club_finance(ft, seller) != 0 && read_u8(exe::financetable_get_club_finance(ft, seller) + finance::CRISIS_STATE) == 1 {
                covered!("mke_1 seller in crisis");
                if exe::human_manager_is_human_club(seller, 0, 1, 0, 0) != 0 {
                    if value >= exe::contract_manager_calc_player_value(cm, player, seller, 0, -1, 0) {
                        value = exe::contract_manager_calc_player_value(cm, player, seller, 0, -1, 0);
                    }
                } else if value >= player.value() as i32 {
                    value = player.value() as i32;
                }
            }
            if !comp {
                ask = if exe::human_manager_is_human_club(seller, 0, 1, 0, 0) != 0 {
                    exe::contract_manager_get_human_asking_price(cm, seller, club, player)
                } else {
                    exe::contract_manager_calc_asking_price(cm, seller, club, player, 0, 1, 1)
                };
            }
        }
        if fee != 0 && comp && fee < ask {
            ask = fee;
        }
        let rejected = 'reject: {
            if !seller.is_null() && !comp && exe::shortlist_manager_predict_selling_club_response(seller, player, value as i64, club, ask as i64, 0) == 2 {
                covered!("mke_1 seller would refuse");
                break 'reject true;
            }
            cand = ftol(cand as f64 * shortlist_offer_margin(ask.wrapping_sub(value), max_price, factor));
            cand = shortlist_offer_injury_penalty(player, cand);
            if cand < score {
                break 'reject true;
            }
            let st = ((read_u8(slot + shortlist_slot::FLAGS) >> 1) & 0xf) as u32;
            if exe::shortlist_manager_staff_interest_in_joining_club(this, player, club, 1, 0, st, 0) < read_f64(rdata::F64_3_15) { // mke1: the interest a player needs
                covered!("mke_1 not interested");
                break 'reject true;
            }
            false
        };
        if rejected {
            if saved != -1 {
                write_i32(slot + shortlist_slot::MAX_PRICE, saved);
            }
            continue;
        }
        if read_u8(slot + shortlist_slot::FLAGS) & 1 == 0 {
            let st = ((read_u8(slot + shortlist_slot::FLAGS) >> 1) & 0xf) as u32;
            let wage = exe::staff_contract_default_staff_contract(wanted, player, club, 0, st, 0xb, 0) as f32;
            let st = ((read_u8(slot + shortlist_slot::FLAGS) >> 1) & 0xf) as u32;
            let d = if exe::shortlist_manager_staff_interest_in_joining_club(this, player, club, 0, 0, st, 0) < read_f64(rdata::F64_0_1) {
                read_f64(rdata::F64_0_1)
            } else {
                let st = ((read_u8(slot + shortlist_slot::FLAGS) >> 1) & 0xf) as u32;
                exe::shortlist_manager_staff_interest_in_joining_club(this, player, club, 0, 0, st, 0)
            };
            let mut q = read_f64(rdata::F64_3_5) / d;
            if q < read_f64(rdata::F64_0_9) {
                q = read_f32(rdata::F32_0_8999999762) as f64;
            }
            let wanted_wage = q * wage as f64;
            let st = ((read_u8(slot + shortlist_slot::FLAGS) >> 1) & 0xf) as u32;
            let cap = exe::finance_get_max_wage(exe::financetable_get_club_finance(ft, club), player, st);
            if (cap as f64 * read_f64(rdata::F64_1_2)) < wanted_wage { // mke1: the wage within 1.2 × the cap
                covered!("mke_1 wage too high");
                return 0;
            }
        }
        write_u32(slot + shortlist_slot::TARGET_STAFF_ID, read_u32(player.addr()));
        if !previous.is_null() {
            exe::shortlist_manager_update_staff_status(this, previous, club, 3, 0);
        }
        let add_entry: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8423f0usize); // add_shortlist_entry
        add_entry(this, player.addr(), club.addr());
        covered!("mke_1 shortlisted");
        return 1;
    }
    0
}

/// f64 0.01536 (`.rdata` 0x96f920): the valuation² factor of a raised shortlist ceiling.
const TRY_ADD_F64_0_01536: usize = 0x96f920;

/// Comparer for `try_add_player_to_club_shortlist`: the original only (it draws random numbers, rewrites the slots'
/// ceilings and targets, the shortlist pairs and the cached contract; it cannot run twice).
pub(crate) unsafe extern "thiscall" fn compare_try_add_player_to_club_shortlist(this: usize, player: usize, club: usize, positions: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(%%I0%%)(this, player, club, positions) // simplification: original only
}
