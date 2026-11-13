// ---------- batch 290: an AI club shortlisting a player it is offered (shortlist_manager.cpp) ----------

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
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_TRY_ADD_PLAYER_TO_CLUB_SHORTLIST)(this, player, club, positions) // simplification: original only
}

// ---------- batch 290: the contracts' day: talks, values, loans, renewals and the contract end (contract_manager.cpp) ----------

/// The fixed dates of the contracts' day that `globals.rs` does not name yet (cm0102.exe 3.9.68): today + 550 and + 1095
/// (recomputed on slice 0 with the others), and the six dates of the transfer rules change (today + 7, 30, 42, 60, 70,
/// 91, recomputed for each person with a player sheet before the change), plus the progress text buffer.
#[allow(dead_code)]
pub(crate) mod contract_day_globals {
    pub(crate) const PLUS_550_DAYS: usize = 0xae1008;
    pub(crate) const PLUS_1095_DAYS: usize = 0xae1020;
    pub(crate) const RULE_PLUS_7_DAYS: usize = 0xae1070;
    pub(crate) const RULE_PLUS_30_DAYS: usize = 0xae1018;
    pub(crate) const RULE_PLUS_42_DAYS: usize = 0xae1038;
    pub(crate) const RULE_PLUS_60_DAYS: usize = 0xae1028;
    pub(crate) const RULE_PLUS_70_DAYS: usize = 0xae1078;
    pub(crate) const RULE_PLUS_91_DAYS: usize = 0xae1088;
    /// the status text of the "Updating game data" bar (`langlib_sprintf` writes it, the screen manager shows it)
    pub(crate) const PROGRESS_TEXT: usize = 0xde1f64;
    /// "Updating game data"
    pub(crate) const PROGRESS_FORMAT: usize = 0x9a820c;
    /// qsort comparers of the talks records (21 bytes) and of the decision dates (14 bytes)
    pub(crate) const COMPARE_TALKS: usize = 0x4dc970;
    pub(crate) const COMPARE_DECISIONS: usize = 0x4dc910;
}

/// What the person pass does after a step: go on to the contract end of the day, or go to the next person.
#[derive(PartialEq)]
enum ContractDayFlow {
    End,
    Next,
}

/// Two dates (day i16 at +0, year at +2) are the same day.
unsafe fn contract_day_same(a: usize, b: usize) -> bool {
    read_u16(a + 2) == read_u16(b + 2) && read_u16(a) == read_u16(b)
}

/// The contract record of `person` in the manager's 0x50-byte array through the index at `field` of his 79-byte
/// state record (+0 the contract, +4 the loan); 0 for an id out of range or a negative index (the original's inline
/// lookup over `[this]`, which does not store the cached contract).
unsafe fn contract_day_contract(this: usize, person: Staff, field: usize) -> Contract {
    let id = read_i32(person.addr());
    if id >= read_i32(G_STAFF_COUNT) {
        return Contract(0);
    }
    let k = read_i32(contract_manager_staff_state_record(id as usize) + field);
    if k < 0 {
        return Contract(0);
    }
    Contract(read_usize(this).wrapping_add((k as usize).wrapping_mul(0x50)))
}

/// The club of a contract (`club_at` of its `contract::CLUB`, no check, as the original's inline arithmetic).
unsafe fn contract_day_club(c: Contract) -> Club {
    club_at(read_i32(c.addr() + contract::CLUB) as isize)
}

/// The squad status of a contract: the high nibble of `contract::SQUAD_STATUS`.
unsafe fn contract_day_status(c: Contract) -> u8 {
    read_u8(c.addr() + contract::SQUAD_STATUS) >> 4
}

/// The AI renewal offer of the contracts' day: `exe::contract_manager_offer_contract` with no club, no output, slot −1,
/// the factor and mode 2. Returns its result.
unsafe fn contract_day_offer(this: usize, p: Staff, factor: f64) -> u32 {
    exe::contract_manager_offer_contract(this, p, 0, 0, -1, factor, 2)
}

/// The club's shortlist holds the person (`exe::shortlist_manager_find_club_shortlist_entry` of the transfer manager's
/// shortlist manager, with 0, 0).
unsafe fn contract_day_on_list(club: Club, p: Staff) -> bool {
    let sm = read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER);
    exe::shortlist_manager_find_club_shortlist_entry(sm, club, p, 0, 0) != 0
}

/// The person goes on the clubs' shortlists (`exe::shortlist_manager_add_player_to_club_shortlists` with 1).
unsafe fn contract_day_shortlist(p: Staff) {
    let sm = read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER);
    exe::shortlist_manager_add_player_to_club_shortlists(sm, p, 1);
}

/// The contract expiry news (`post_contract_expiry_news` 0x4d6790, Rust; thiscall, ret 0x10) through its address.
unsafe fn contract_day_expiry_news(this: usize, c: Contract, period: u32, bosman: u32, warn: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x4d6790usize);
    f(this, c.addr(), period, bosman, warn)
}

/// The contract end news (`post_contract_end_news` 0x4da590, Rust; thiscall, ret 0xc) through its address.
unsafe fn contract_day_end_news(this: usize, p: Staff, kind: u32, wide: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x4da590usize);
    f(this, p.addr(), kind, wide)
}

/// The free transfer queue (`queue_free_transfer` 0x8c16b0, Rust; thiscall, ret 0xc) with reason 1 and no
/// compensation (−1), through its address.
unsafe fn contract_day_queue(p: Staff) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x8c16b0usize);
    f(read_usize(G_TRANSFER_MANAGER), p.addr(), 1, u32::MAX);
}

/// Whether the expiry news speaks of a Bosman: the person is 24 or more at `date` (`exe::database_get_staff_age`) and
/// his club uses the Bosman ruling (`exe::transfer_manager_player_club_uses_bosman`).
unsafe fn contract_day_bosman_at(p: Staff, date: usize) -> u32 {
    (exe::database_get_staff_age(p, date as *const u16) >= 0x18 && exe::transfer_manager_player_club_uses_bosman(read_usize(G_TRANSFER_MANAGER), p) != 0) as u32
}

/// The person is retiring (`exe::player_regen_get_retirement_status` ≠ 0).
unsafe fn contract_day_retiring(p: Staff) -> bool {
    exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), p) != 0
}

/// The club has a human manager for the news (`exe::news_club_has_human_manager`).
unsafe fn contract_day_news_human(club: Club) -> bool {
    exe::news_club_has_human_manager(read_usize(G_NEWS), club) != 0
}

/// No offer, exchange offer or free transfer queue entry for the person (`exe::transfer_manager_get_offers_for_player`,
/// `exe::transfer_manager_get_exchange_offers_for_player`, `exe::transfer_manager_is_queued_for_free_transfer`, in this
/// order, stopping at the first).
unsafe fn contract_day_no_offers(p: Staff) -> bool {
    let tm = read_usize(G_TRANSFER_MANAGER);
    exe::transfer_manager_get_offers_for_player(tm, p, 0usize, 0) == 0
        && exe::transfer_manager_get_exchange_offers_for_player(tm, p, 0usize) == 0
        && exe::transfer_manager_is_queued_for_free_transfer(tm, p) == 0
}

/// The refusal of a renewal the AI could not offer: a local contract offer (`exe::new_staff_contract`, its date at
/// +0x51 `exe::date_date_54a110`) for the person at `club`, type 2 (`contract::CONTRACT_TYPE` & 0xc2 | 2), the squad
/// status of his contract, +0x50 = 1, +0x59 = −1, +0x61 = his job; with `mark`, bit 0x80 of his 79-byte record's
/// `STATUS_FLAGS`; then the offer response news (0x4d49a0) with 1.
unsafe fn contract_day_refusal(this: usize, p: Staff, club: Club, c: Contract, mark: bool) {
    let mut o = [0u8; 0x80];
    let l = o.as_mut_ptr() as usize;
    exe::new_staff_contract(l);
    exe::date_date_54a110(l + 0x51);
    write_u32(l, read_u32(p.addr()));
    write_u32(l + contract::CLUB, read_u32(club.addr()));
    write_u8(l + contract::CONTRACT_TYPE, (read_u8(l + contract::CONTRACT_TYPE) & 0xc2) | 2);
    write_u8(l + contract::SQUAD_STATUS, (read_u8(l + contract::SQUAD_STATUS) & 0xf) | (contract_day_status(c) << 4));
    write_u8(l + 0x50, 1);
    write_i32(l + 0x59, -1);
    write_u8(l + 0x61, p.job_for_club());
    if mark {
        let r = contract_manager_staff_state_record(read_usize(p.addr())) + staff_state::STATUS_FLAGS;
        write_u8(r, read_u8(r) | 0x80);
    }
    covered!("mke_2 renewal refused");
    let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x4d49a0usize); // post_offer_response_news
    f(this, l, 1);
}

/// The renewal by squad status of bands 9, 11 and 13 and of the end day: status 1-2 on his club's list 1.4, 3-5 on the
/// list 1.2, otherwise (and status 6) 1.0; status 0 or above 6 no offer.
unsafe fn contract_day_offer_by_status(this: usize, p: Staff, club: Club, st: u8) {
    match st {
        1 | 2 => {
            let f = if contract_day_on_list(club, p) { 1.4 } else { 1.0 }; // mke2: the late renewal of a key player
            contract_day_offer(this, p, f);
        }
        3..=5 => {
            let f = if contract_day_on_list(club, p) { 1.2 } else { 1.0 };
            contract_day_offer(this, p, f);
        }
        6 => {
            contract_day_offer(this, p, 1.0);
        }
        _ => {}
    }
}

/// Slice 0: the fixed dates today + 7, 30, 60, 91, 121, 152, 182, 365, 550 and 1095 (`exe::date_add_days`, copied
/// with `exe::date_operator_3d`).
unsafe fn contract_day_fixed_dates(date: usize) {
    use contract_day_globals::*;
    let dates: [(usize, u32); 10] = [
        (G_CONTRACT_DATE_PLUS_7_DAYS, 7),
        (G_CONTRACT_DATE_PLUS_30_DAYS, 0x1e),
        (G_CONTRACT_DATE_PLUS_60_DAYS, 0x3c),
        (G_CONTRACT_DATE_PLUS_91_DAYS, 0x5b),
        (G_CONTRACT_DATE_PLUS_121_DAYS, 0x79),
        (G_CONTRACT_DATE_PLUS_152_DAYS, 0x98),
        (G_CONTRACT_DATE_PLUS_182_DAYS, 0xb6),
        (G_CONTRACT_DATE_PLUS_365_DAYS, 0x16d),
        (PLUS_550_DAYS, 0x226),
        (PLUS_1095_DAYS, 0x447),
    ];
    for (g, n) in dates {
        let mut t = [0u8; 8];
        let mut u = [0u8; 8];
        let r = exe::date_add_days(date, t.as_mut_ptr() as usize, n);
        exe::date_operator_3d(g, u.as_mut_ptr() as usize, r);
    }
}

/// The contract talks expire (25 − the person's loyalty) × 2 days after their date (+0xc): an expired record is
/// overwritten by the last one (21 bytes), the count goes down and the same place is looked at again; then the records
/// are sorted again (qsort with 0x4dc970).
unsafe fn contract_day_expire_talks(this: usize, date: usize) {
    let mut i: i32 = 0;
    let mut off: i32 = 0;
    while i < read_i32(this + 0x2c) {
        let talks = read_usize(this + contract_manager_fields::TALKS_RECORDS);
        let rec = talks.wrapping_add(off as usize);
        let p = staff_at(read_i32(rec + 8) as isize);
        let days = (25 - p.loyalty() as i32) * 2; // mke2: the talks last (25 − loyalty) × 2 days
        let mut t = [0u8; 8];
        let end = exe::date_add_days(rec + 0xc, t.as_mut_ptr() as usize, days as u32);
        if fill_slot_date_before(end, date) {
            let n = read_i32(this + 0x2c);
            let last = talks.wrapping_add((n as usize).wrapping_mul(21)).wrapping_sub(21);
            core::ptr::copy(last as *const u8, rec as *mut u8, 21);
            write_i32(this + 0x2c, n - 1);
            i -= 1;
            off -= 21;
            covered!("mke_2 talks expired");
        }
        i += 1;
        off += 21;
    }
    let n = read_i32(this + 0x2c);
    if n != 0 {
        exe::crt_qsort(read_usize(this + contract_manager_fields::TALKS_RECORDS), n as u32, 21, contract_day_globals::COMPARE_TALKS);
    }
}

/// The value of the person: at a human club, a player (`exe::database_is_player_job`) with `TRANSFER_STATUS` bit 4 and
/// a sheet is valued again (`exe::contract_manager_calc_player_value` with 1), then capped by an active release fee;
/// at an AI club, a player only on odd days of the year, bit 4 set, valued with 0 and capped; a non-player every day
/// (bit 4 set, valued with 0, no cap); without a contract or a club, 0.
unsafe fn contract_day_value(this: usize, date: usize, p: Staff, club: Club, c: Contract) {
    let cap = |c: Contract| {
        if exe::staff_contract_has_active_release_clause(c) != 0 && read_i32(p.addr() + staff::VALUE) > read_i32(c.addr() + contract::RELEASE_FEE) {
            write_i32(p.addr() + staff::VALUE, read_i32(c.addr() + contract::RELEASE_FEE));
        }
    };
    if club.is_null() || c.is_null() {
        write_i32(p.addr() + staff::VALUE, 0);
        return;
    }
    let ts = c.addr() + contract::TRANSFER_STATUS;
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 && exe::database_is_player_job(p.job_for_club() as i8) != 0 {
        if read_u8(ts) & 4 != 0 && !p.player().is_null() {
            write_i32(p.addr() + staff::VALUE, exe::contract_manager_calc_player_value(this, p, club, 0, -1, 1));
        }
        cap(c);
        return;
    }
    if exe::database_is_player_job(p.job_for_club() as i8) != 0 {
        if (read_i16(date) as i32) % 2 == 0 { // mke2: the AI's players are valued on odd days
            return;
        }
        write_u8(ts, read_u8(ts) | 4);
        write_i32(p.addr() + staff::VALUE, exe::contract_manager_calc_player_value(this, p, club, 0, -1, 0));
        cap(c);
        return;
    }
    write_u8(ts, read_u8(ts) | 4);
    write_i32(p.addr() + staff::VALUE, exe::contract_manager_calc_player_value(this, p, club, 0, -1, 0));
}

/// The loan contract (index +4 of the 79-byte record) ending today: without a club job only the index becomes −1;
/// otherwise the end news 6 (outside contract type 7) and `end_loan` (0x4d07e0). Returns the loan contract (0 none).
unsafe fn contract_day_loan_end(this: usize, date: usize, p: Staff) -> Contract {
    let loan = contract_day_contract(this, p, 4);
    if loan.is_null() {
        return loan;
    }
    if contract_day_same(date, loan.addr() + contract::CONTRACT_EXPIRES) {
        if p.club_job().is_null() {
            write_i32(contract_manager_staff_state_record(read_usize(p.addr())) + 4, -1);
        } else {
            if read_u8(loan.addr() + contract::CONTRACT_TYPE) & 0x3f != 7 {
                contract_day_end_news(this, p, 6, 0);
            }
            covered!("mke_2 loan ended");
            let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4d07e0usize); // end_loan
            f(this, p.addr());
        }
    }
    loan
}

/// Before the transfer rules change (`G_TRANSFER_RULE_CHANGE_DATE` after today), for a person with a player sheet: the
/// six rule dates (today + 7, 30, 42, 60, 70, 91) are recomputed; a contract started over 180 days ago
/// (`exe::date_days_since`) and ending before today + 1095, when the change falls on one of those dates, at a club that
/// does not dislike him (`exe::club_dislikes_staff`) and whose manager does not (`exe::staff_is_disliked_by`), not human,
/// with the person under 32: status 1 renews (1.0); 2 and 3 with current reputation above 6750, or under 24 with
/// potential above 100; 5 with potential above 100.
unsafe fn contract_day_rule_change(this: usize, date: usize, p: Staff, club: Club, c: Contract) {
    use contract_day_globals::*;
    if !fill_slot_date_before(date, G_TRANSFER_RULE_CHANGE_DATE) {
        return;
    }
    let rules: [(usize, u32); 6] = [
        (RULE_PLUS_7_DAYS, 7),
        (RULE_PLUS_30_DAYS, 0x1e),
        (RULE_PLUS_42_DAYS, 0x2a),
        (RULE_PLUS_60_DAYS, 0x3c),
        (RULE_PLUS_70_DAYS, 0x46),
        (RULE_PLUS_91_DAYS, 0x5b),
    ];
    for (g, n) in rules {
        let mut t = [0u8; 8];
        let mut u = [0u8; 8];
        let r = exe::date_add_days(date, t.as_mut_ptr() as usize, n);
        exe::date_operator_3d(g, u.as_mut_ptr() as usize, r);
    }
    let s = c.addr() + contract::DATE_STARTED;
    if exe::date_days_since(date, read_u32(s), read_u32(s + 4)) <= 0xb4 {
        return;
    }
    if !fill_slot_date_before(c.addr() + contract::CONTRACT_EXPIRES, PLUS_1095_DAYS) {
        return;
    }
    if !rules.iter().any(|&(g, _)| contract_day_same(g, G_TRANSFER_RULE_CHANGE_DATE)) {
        return;
    }
    let disliked = exe::club_dislikes_staff(p, club) != 0;
    let m = club.manager();
    if !m.is_null() && exe::staff_is_disliked_by(p, m) != 0 {
        return;
    }
    if disliked || exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 {
        return;
    }
    let age = p.age() as i8;
    if age >= 0x20 {
        return;
    }
    let sheet = p.player().addr();
    let renew = match contract_day_status(c) {
        1 => true,
        2 | 3 => read_i16(sheet + player_sheet::CURRENT_REPUTATION) > 0x1a5e || (age < 0x18 && read_i16(sheet + player_sheet::POTENTIAL_ABILITY) > 100),
        5 => read_i16(sheet + player_sheet::POTENTIAL_ABILITY) > 100,
        _ => false,
    };
    if renew {
        covered!("mke_2 renewal before the rules change");
        contract_day_offer(this, p, 1.0);
    }
}

/// A type 2 contract ending after today + 182, not on loan: outside the FIFA protected period
/// (`exe::staff_contract_check_fifa_protected_period` 0 with the limit flag set), not retiring, a player (job 11): the
/// AI club (not human, under 32) renews by status (1 and 2: 1.0; 3: current reputation above 6750, or under 24 with
/// potential above 100; 5: potential above 100); with a human manager of the contract's club, the expiry news 7 (Bosman
/// by the age at today + 182). Always the next person afterwards.
unsafe fn contract_day_fifa_period(this: usize, p: Staff, club: Club, c: Contract) {
    let mut at_limit: u32 = 0;
    if exe::staff_contract_check_fifa_protected_period(c, 0usize, &mut at_limit as *mut u32 as usize, 0usize) != 0 || at_limit == 0 {
        return;
    }
    if contract_day_retiring(p) || p.job_for_club() != 0xb {
        return;
    }
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
        let age = p.age() as i8;
        if age < 0x20 {
            let sheet = p.player().addr();
            let renew = match contract_day_status(c) {
                1 | 2 => true,
                3 => read_i16(sheet + player_sheet::CURRENT_REPUTATION) > 0x1a5e || (age < 0x18 && read_i16(sheet + player_sheet::POTENTIAL_ABILITY) > 100),
                5 => read_i16(sheet + player_sheet::POTENTIAL_ABILITY) > 100,
                _ => false,
            };
            if renew {
                covered!("mke_2 renewal at the FIFA limit");
                contract_day_offer(this, p, 1.0);
            }
        }
    }
    if contract_day_news_human(contract_day_club(c)) {
        let bosman = contract_day_bosman_at(p, G_CONTRACT_DATE_PLUS_182_DAYS);
        contract_day_expiry_news(this, c, 7, bosman, 1);
    }
}

/// A type 1 contract (expired) with no loan: 31 days after the end, a person who wants to leave
/// (`exe::staff_contract_wants_to_leave`) or leaves on a Bosman, with no offers, exchanges or queue: the end news 1 to a
/// human manager of the contract's club; jobs 12 (always), 13 and 15 (at an AI club) leave the club's squad (the 50
/// places of `club::SQUAD`) and become 5, 6 and 8; the others go to the free transfer queue.
unsafe fn contract_day_expired(this: usize, date: usize, p: Staff, c: Contract) -> ContractDayFlow {
    let mut t = [0u8; 8];
    let after = exe::date_add_days(c.addr() + contract::CONTRACT_EXPIRES, t.as_mut_ptr() as usize, 0x1f);
    if !fill_slot_date_before(after, date) {
        return ContractDayFlow::End;
    }
    if exe::staff_contract_wants_to_leave(c) == 0 && c.leaving_on_bosman() == 0 {
        return ContractDayFlow::End;
    }
    if !contract_day_no_offers(p) {
        return ContractDayFlow::End;
    }
    if contract_day_news_human(contract_day_club(c)) {
        contract_day_end_news(this, p, 1, 0);
    }
    let leave_squad = |p: Staff, job: u8| {
        let cj = p.club_job();
        if !cj.is_null() {
            for k in 0..50usize {
                let a = cj.addr() + club::SQUAD + k * 4;
                if read_usize(a) == p.addr() {
                    write_usize(a, 0);
                }
            }
        }
        p.set_job_for_club(job);
    };
    let job = p.job_for_club();
    if job == 0xc {
        leave_squad(p, 5);
        return ContractDayFlow::Next;
    }
    if job == 0xd && exe::human_manager_is_human_club(contract_day_club(c), 0, 1, 0, 0) == 0 {
        leave_squad(p, 6);
        return ContractDayFlow::Next;
    }
    if p.job_for_club() == 0xf && exe::human_manager_is_human_club(contract_day_club(c), 0, 1, 0, 0) == 0 {
        leave_squad(p, 8);
        return ContractDayFlow::Next;
    }
    covered!("mke_2 expired to the queue");
    contract_day_queue(p);
    ContractDayFlow::Next
}

/// The Bosman rule and the clubs' shortlists for a contract of another type. A player (sheet, job 11, not retiring, no
/// future transfer) already leaving on a Bosman goes on the shortlists on the odd bands (`exe::staff_contract_get_expiry_band`
/// 1, 3, .., 13). Otherwise, for a type 2 contract started over 93 days ago with no future transfer: inside the bands
/// window (band ≠ 0 and ≠ 14), not retiring, a player of the main contract's club (not a loan, with a nation): current
/// reputation above 3750, world above 1750, loyalty below 15, 19 to 30 years, the nation allows free transfers
/// (`exe::nation_allows_free_transfers`), not (at a club above 4750 and under 23), and wanting to leave or
/// `TRANSFER_STATUS` bit 8: he leaves on a Bosman (+0x49 = 1, bit 8 cleared), goes on the shortlists and the Bosman news
/// (0x4d6d70). When those fail, the odd bands put him on the shortlists, band 14 with random(20) = 0. Then the AI
/// renewals (`contract_day_renewals`).
unsafe fn contract_day_bosman(this: usize, date: usize, p: Staff, club: Club, c: Contract) -> ContractDayFlow {
    let tm = read_usize(G_TRANSFER_MANAGER);
    if !p.player().is_null() && p.job_for_club() == 0xb && !contract_day_retiring(p) && exe::transfer_manager_get_future_transfer(tm, p, 0) == 0 {
        if c.leaving_on_bosman() != 0 {
            let band = exe::staff_contract_get_expiry_band(c) as i8;
            if (1..=13).contains(&band) && band % 2 == 1 {
                contract_day_shortlist(p);
            }
            return ContractDayFlow::End;
        }
    } else if c.leaving_on_bosman() != 0 {
        return ContractDayFlow::End;
    }
    if exe::transfer_manager_get_future_transfer(tm, p, 0) != 0 || c.contract_type() & 0x3f != 2 {
        return ContractDayFlow::End;
    }
    let mut t = [0u8; 8];
    let ago = exe::date_sub_days(date, t.as_mut_ptr() as usize, 0x5d);
    if !fill_slot_date_before(c.addr() + contract::DATE_STARTED, ago) {
        return ContractDayFlow::End;
    }
    if exe::staff_contract_get_expiry_band(c) != 0xe && exe::staff_contract_get_expiry_band(c) != 0 && !contract_day_retiring(p) && !p.player().is_null() && p.job_for_club() == 0xb {
        let main = Contract(main_contract_lookup(p));
        if main.is_null() || main.contract_type() & 0x3f == 5 {
            return ContractDayFlow::Next;
        }
        let mc = contract_day_club(main);
        if mc.is_null() {
            return ContractDayFlow::Next;
        }
        let nation = read_usize(mc.addr() + club::NATION);
        if nation == 0 {
            return ContractDayFlow::Next;
        }
        let sheet = p.player().addr();
        let age = p.age() as i8;
        let cj = p.club_job();
        if read_i16(sheet + player_sheet::CURRENT_REPUTATION) > 0xea6
            && read_i16(sheet + player_sheet::WORLD_REPUTATION) > 0x6d6
            && p.loyalty() < 15 // mke2: the Bosman needs loyalty below 15
            && age < 0x1f
            && age > 0x12
            && exe::nation_allows_free_transfers(nation) != 0
            && !(!cj.is_null() && cj.reputation() > 0x128e && age < 0x17)
            && (exe::staff_contract_wants_to_leave(c) != 0 || read_u8(c.addr() + contract::TRANSFER_STATUS) & 8 != 0)
        {
            c.set_leaving_on_bosman(1);
            write_u8(c.addr() + contract::TRANSFER_STATUS, read_u8(c.addr() + contract::TRANSFER_STATUS) & 0xf7);
            contract_day_shortlist(p);
            covered!("mke_2 leaves on a Bosman");
            let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4d6d70usize); // post_bosman_departure_news
            f(this, c.addr(), 0);
            return ContractDayFlow::Next;
        }
        let band = exe::staff_contract_get_expiry_band(c) as i8;
        if (1..=13).contains(&band) && band % 2 == 1 {
            contract_day_shortlist(p);
        } else if band == 14 && exe::utils_random(20) == 0 {
            contract_day_shortlist(p);
        }
    }
    contract_day_renewals(this, date, p, club, c);
    ContractDayFlow::End
}

/// The AI renewal offers of a contract of another type (`contract_day_offer`, the factor by the path), in this order:
/// - 550 days left (`PLUS_550_DAYS`): a player job not retiring at an AI club, status 1-3 on his club's list: 1.0;
/// - 365 days left (or, on the game's first day, an end before that): a player job not retiring; exactly 365 days: the
///   expiry news 6 (Bosman by the age then); an AI club, status 1-3 or 5 on the list: 1.0, and when it is not made the
///   refusal (`contract_day_refusal`);
/// - bands 3 and 5 (152/121 days): a player job not retiring at an AI club: status 1-3 on the list 1.2 (off it 1.0), 4
///   and 5 1.0;
/// - band 1 (182 days), not retiring: a player gets the expiry news 5 (Bosman at today + 182, warned); another job the
///   news 5 to a human manager or else 1.0; then a player job at an AI club: status 1-3 on the list 1.1, off it 1.0, and
///   when it is not made the refusal with bit 0x80; status 4-6 1.0;
/// - band 7 (91 days), not retiring: a player at a club with a human manager gets the expiry news 1 (Bosman at today +
///   91); at an AI club status 1-2 on the list 1.3 (off it 1.0) with the refusal and bit 0x80 when not made, 3-5 1.1 /
///   1.0, 6 1.0; another job the news 1 to a human manager or else 1.0;
/// - bands 9 and 11 (60/30 days): a player job not retiring at an AI club by status (`contract_day_offer_by_status`);
/// - band 13 (7 days), not retiring: a player gets the expiry news 4 at a club with a human manager (Bosman at today +
///   7), otherwise by status; another job the news 4 to a human manager or else 1.0;
/// - the end reached (today not before the end): the contract becomes type 1 and the end news 2; a player who wants to
///   leave (or `TRANSFER_STATUS` bit 8) leaves on a Bosman and goes on the shortlists; otherwise, unless the main
///   contract's club has a human manager or the club is human: status 1-6 with a loan index (+4 of the 79-byte record ≠
///   −1) by status; another status: a transfer-listed player (`exe::staff_is_transfer_listed`) with a club, no offers,
///   exchanges or queue goes to the free transfer queue; another job at an AI club: 1.0.
unsafe fn contract_day_renewals(this: usize, date: usize, p: Staff, club: Club, c: Contract) {
    use contract_day_globals::*;
    let job = p.job_for_club();
    let player_job = |p: Staff| exe::database_is_player_job(p.job_for_club() as i8) != 0;
    let human = |c: Contract| exe::human_manager_is_human_club(contract_day_club(c), 0, 1, 0, 0) != 0;
    let is_player = !p.player().is_null() && job == 0xb;
    // 550 days
    if contract_day_same(PLUS_550_DAYS, c.addr() + contract::CONTRACT_EXPIRES) && player_job(p) && !contract_day_retiring(p) {
        if human(c) {
            return;
        }
        let st = contract_day_status(c);
        if st == 0 || st > 3 || !contract_day_on_list(contract_day_club(c), p) {
            return;
        }
        covered!("mke_2 renewal at 550 days");
        contract_day_offer(this, p, 1.0);
        return;
    }
    // 365 days
    let exp = c.addr() + contract::CONTRACT_EXPIRES;
    let at_365 = contract_day_same(G_CONTRACT_DATE_PLUS_365_DAYS, exp)
        || (contract_day_same(G_GAME_START_DATE, date) && fill_slot_date_before(exp, G_CONTRACT_DATE_PLUS_365_DAYS));
    if at_365 && player_job(p) && !contract_day_retiring(p) {
        if contract_day_same(G_CONTRACT_DATE_PLUS_365_DAYS, exp) {
            let bosman = contract_day_bosman_at(p, G_CONTRACT_DATE_PLUS_365_DAYS);
            contract_day_expiry_news(this, c, 6, bosman, 0);
        }
        if human(c) {
            return;
        }
        let st = contract_day_status(c);
        if st == 0 || !(st <= 3 || st == 5) || !contract_day_on_list(contract_day_club(c), p) {
            return;
        }
        if contract_day_offer(this, p, 1.0) == 0 {
            contract_day_refusal(this, p, club, c, false);
        }
        return;
    }
    // bands 3 and 5
    if (exe::staff_contract_get_expiry_band(c) == 3 || exe::staff_contract_get_expiry_band(c) == 5) && player_job(p) && !contract_day_retiring(p) {
        if human(c) {
            return;
        }
        match contract_day_status(c) {
            1..=3 => {
                let f = if contract_day_on_list(contract_day_club(c), p) { 1.2 } else { 1.0 };
                contract_day_offer(this, p, f);
            }
            4 | 5 => {
                contract_day_offer(this, p, 1.0);
            }
            _ => {}
        }
        return;
    }
    // band 1
    if exe::staff_contract_get_expiry_band(c) == 1 && !contract_day_retiring(p) {
        if is_player {
            let bosman = contract_day_bosman_at(p, G_CONTRACT_DATE_PLUS_182_DAYS);
            contract_day_expiry_news(this, c, 5, bosman, 1);
        } else if contract_day_news_human(contract_day_club(c)) {
            contract_day_expiry_news(this, c, 5, 0, 0);
        } else {
            contract_day_offer(this, p, 1.0);
        }
        if !player_job(p) || human(c) {
            return;
        }
        match contract_day_status(c) {
            1..=3 => {
                let f = if contract_day_on_list(contract_day_club(c), p) { 1.1 } else { 1.0 };
                if contract_day_offer(this, p, f) == 0 {
                    contract_day_refusal(this, p, club, c, true);
                }
            }
            4..=6 => {
                contract_day_offer(this, p, 1.0);
            }
            _ => {}
        }
        return;
    }
    // band 7
    if exe::staff_contract_get_expiry_band(c) == 7 && !contract_day_retiring(p) {
        if is_player {
            if contract_day_news_human(contract_day_club(c)) {
                let bosman = contract_day_bosman_at(p, G_CONTRACT_DATE_PLUS_91_DAYS);
                contract_day_expiry_news(this, c, 1, bosman, 0);
                return;
            }
            if !player_job(p) {
                return;
            }
            match contract_day_status(c) {
                1 | 2 => {
                    let f = if contract_day_on_list(contract_day_club(c), p) { 1.3 } else { 1.0 };
                    if contract_day_offer(this, p, f) == 0 {
                        contract_day_refusal(this, p, club, c, true);
                    }
                }
                3..=5 => {
                    let f = if contract_day_on_list(contract_day_club(c), p) { 1.1 } else { 1.0 };
                    contract_day_offer(this, p, f);
                }
                6 => {
                    contract_day_offer(this, p, 1.0);
                }
                _ => {}
            }
        } else if contract_day_news_human(contract_day_club(c)) {
            contract_day_expiry_news(this, c, 1, 0, 0);
        } else {
            contract_day_offer(this, p, 1.0);
        }
        return;
    }
    // bands 9 and 11
    if (exe::staff_contract_get_expiry_band(c) == 9 || exe::staff_contract_get_expiry_band(c) == 0xb) && player_job(p) && !contract_day_retiring(p) {
        if human(c) {
            return;
        }
        contract_day_offer_by_status(this, p, contract_day_club(c), contract_day_status(c));
        return;
    }
    // band 13
    if exe::staff_contract_get_expiry_band(c) == 0xd && !contract_day_retiring(p) {
        if is_player {
            if contract_day_news_human(contract_day_club(c)) {
                let bosman = contract_day_bosman_at(p, G_CONTRACT_DATE_PLUS_7_DAYS);
                contract_day_expiry_news(this, c, 4, bosman, 0);
                return;
            }
            contract_day_offer_by_status(this, p, contract_day_club(c), contract_day_status(c));
        } else if contract_day_news_human(contract_day_club(c)) {
            contract_day_expiry_news(this, c, 4, 0, 0);
        } else {
            contract_day_offer(this, p, 1.0);
        }
        return;
    }
    // the end reached
    if fill_slot_date_before(date, exp) {
        return;
    }
    write_u8(c.addr() + contract::CONTRACT_TYPE, (read_u8(c.addr() + contract::CONTRACT_TYPE) & 0xc1) | 1);
    contract_day_end_news(this, p, 2, 0);
    if !is_player {
        if !human(c) {
            contract_day_offer(this, p, 1.0);
        }
        return;
    }
    if exe::staff_contract_wants_to_leave(c) != 0 || read_u8(c.addr() + contract::TRANSFER_STATUS) & 8 != 0 {
        c.set_leaving_on_bosman(1);
        write_u8(c.addr() + contract::TRANSFER_STATUS, read_u8(c.addr() + contract::TRANSFER_STATUS) & 0xf7);
        covered!("mke_2 leaves at the end");
        contract_day_shortlist(p);
        return;
    }
    let main = Contract(main_contract_lookup(p));
    if !main.is_null() && !contract_day_club(main).is_null() {
        let again = Contract(main_contract_lookup(p));
        let cl = if again.is_null() { Club(0) } else { contract_day_club(again) };
        if contract_day_news_human(cl) {
            return;
        }
    }
    if human(c) {
        return;
    }
    let st = contract_day_status(c);
    if (1..=6).contains(&st) {
        if read_i32(contract_manager_staff_state_record(read_usize(p.addr())) + 4) == -1 {
            return;
        }
        covered!("mke_2 renewal at the end");
        contract_day_offer_by_status(this, p, contract_day_club(c), st);
        return;
    }
    if !p.club_job().is_null() && !p.player().is_null() && p.job_for_club() == 0xb && exe::staff_is_transfer_listed(p, 0usize) != 0 && contract_day_no_offers(p) {
        covered!("mke_2 listed to the queue");
        contract_day_queue(p);
    }
}

/// The contract end on the day (today = `CONTRACT_EXPIRES`): a loan contract (type 5) gets the end news 5, the stats
/// archived (`exe::playerstats_archive_player_stats`), the main contract erased (its person −1, the index −1) and its
/// club's wage bill updated (`exe::finance_update_wage_bill`), the person out of the selections
/// (`exe::tactics_remove_player_from_selection` twice) and of the club (`exe::database_remove_staff_from_club`). Another
/// type becomes type 1; leaving on a Bosman, with no loan contract and no offers: the end news 2 and released at once
/// (`exe::transfer_manager_free_transfer_staff`).
unsafe fn contract_day_end(this: usize, date: usize, p: Staff, c: Contract) {
    if !contract_day_same(date, c.addr() + contract::CONTRACT_EXPIRES) {
        return;
    }
    if c.contract_type() & 0x3f == 5 {
        contract_day_end_news(this, p, 5, 0);
        exe::playerstats_archive_player_stats(read_usize(G_PLAYER_STATS), p, -1, 0, 0, -1, 0);
        let rec = contract_manager_staff_state_record(read_usize(p.addr()));
        let k = read_i32(rec);
        if k != -1 {
            let main = read_usize(this).wrapping_add((k as usize).wrapping_mul(0x50));
            let cl = club_at(read_i32(main + contract::CLUB) as isize);
            write_i32(main, -1);
            write_i32(rec, -1);
            let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), cl);
            if fin != 0 {
                exe::finance_update_wage_bill(fin);
            }
        }
        covered!("mke_2 loan contract ended");
        exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), p.club_job(), p, 0, 1);
        exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), p.club_job(), p, 1, 1);
        exe::database_remove_staff_from_club(p.club_job(), p, 0usize, 1, 0);
        return;
    }
    write_u8(c.addr() + contract::CONTRACT_TYPE, (read_u8(c.addr() + contract::CONTRACT_TYPE) & 0xc1) | 1);
    if c.leaving_on_bosman() == 0 {
        return;
    }
    if !contract_day_contract(this, p, 4).is_null() {
        return;
    }
    if exe::transfer_manager_get_offers_for_player(read_usize(G_TRANSFER_MANAGER), p, 0usize, 0) != 0 {
        return;
    }
    contract_day_end_news(this, p, 2, 1);
    covered!("mke_2 released on a Bosman");
    exe::transfer_manager_free_transfer_staff(read_usize(G_TRANSFER_MANAGER), p, p.club_job(), 0, -1, 0);
}

/// One person of the people pass (slice 2).
unsafe fn contract_day_person(this: usize, date: usize, p: Staff) {
    if read_u32(G_GAME_DATA_LOADED_FLAG) == 0 {
        // before the full load, a contract ending today only becomes type 1
        let c = contract_day_contract(this, p, 0);
        if !c.is_null() && contract_day_same(date, c.addr() + contract::CONTRACT_EXPIRES) {
            write_u8(c.addr() + contract::CONTRACT_TYPE, (read_u8(c.addr() + contract::CONTRACT_TYPE) & 0xc1) | 1);
        }
        return;
    }
    let looked = Contract(main_contract_lookup(p));
    let club = if looked.is_null() { Club(0) } else { contract_day_club(looked) };
    contract_day_value(this, date, p, club, contract_day_contract(this, p, 0));
    let loan = contract_day_loan_end(this, date, p);
    let c = contract_day_contract(this, p, 0);
    if c.is_null() {
        return;
    }
    if !p.player().is_null() {
        contract_day_rule_change(this, date, p, club, c);
    }
    if fill_slot_date_before(G_CONTRACT_DATE_PLUS_182_DAYS, c.addr() + contract::CONTRACT_EXPIRES) && c.contract_type() & 0x3f == 2 && loan.is_null() {
        contract_day_fifa_period(this, p, club, c);
        return;
    }
    let flow = if c.contract_type() & 0x3f == 1 {
        if !contract_day_contract(this, p, 4).is_null() {
            ContractDayFlow::End
        } else {
            contract_day_expired(this, date, p, c)
        }
    } else {
        contract_day_bosman(this, date, p, club, c)
    };
    if flow == ContractDayFlow::End {
        contract_day_end(this, date, p, c);
    }
}

/// On days 36 and 200 of the year the decision dates already processed (`DECISION_NEXT_INDEX`) leave the array (the
/// pending ones move down, the dates with `exe::date_operator_3d`); then, when decisions were added (`DECISIONS_ADDED`),
/// the pending part is sorted again (qsort with 0x4dc910) and the counter is reset.
unsafe fn contract_day_decisions(this: usize, date: usize) {
    let day = read_i16(date);
    if day == 0x24 || day == 0xc8 {
        let n = read_i32(this + contract_manager_fields::DECISION_COUNT);
        let k = read_i32(this + contract_manager_fields::DECISION_NEXT_INDEX);
        if n > 0 && k < n {
            let base = read_usize(this + contract_manager_fields::DECISION_DATES);
            let mut dst = base;
            let mut src = base.wrapping_add((k as usize).wrapping_mul(14));
            let mut i = k;
            while i < read_i32(this + contract_manager_fields::DECISION_COUNT) {
                write_u32(dst, read_u32(src));
                let mut t = [0u8; 8];
                exe::date_operator_3d(dst + 4, t.as_mut_ptr() as usize, src + 4);
                write_u8(dst + 0xc, read_u8(src + 0xc));
                write_u8(dst + 0xd, read_u8(src + 0xd));
                dst += 14;
                src += 14;
                i += 1;
            }
            let k = read_i32(this + contract_manager_fields::DECISION_NEXT_INDEX);
            let n = read_i32(this + contract_manager_fields::DECISION_COUNT);
            write_i32(this + contract_manager_fields::DECISION_NEXT_INDEX, 0);
            write_i32(this + contract_manager_fields::DECISION_COUNT, n - k);
        }
    }
}

/// The contracts' day, run at every slice of the day by the day loop (and while running to the start date).
///
/// `daily_update(&date, slice, people)` (the contract manager in ecx). Slice 0 recomputes the fixed dates
/// (`contract_day_fixed_dates`); slice 2 expires the contract talks (`contract_day_expire_talks`) and, with `people` =
/// 1, passes over every person (`contract_day_person`): on 31 December the "Updating game data" bar (one step every
/// 250 people, `exe::screenmanager_send_network_status_message` with staff count × 0.004 steps); before the full load
/// only the contracts ending today become type 1; otherwise the value (`contract_day_value`), the loan end
/// (`contract_day_loan_end`), with a player sheet the renewal before the rules change (`contract_day_rule_change`), a
/// type 2 contract ending after today + 182 with no loan the FIFA period (`contract_day_fifa_period`, then the next
/// person), a type 1 contract with no loan the expired contract (`contract_day_expired`), another type the Bosman rule,
/// the shortlists and the AI renewals (`contract_day_bosman`), and then the contract end on the day
/// (`contract_day_end`). Every slice ends with the decision dates (`contract_day_decisions` after the people pass, then
/// the re-sort and `process_due_decision_dates` 0x4cd9c0 with the date and the slice). The SEH frame only matters for
/// exceptions.
/// Original: `CONTRACT_MANAGER::daily_update` (0x4c7ac0, contract_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn contract_manager_daily_update(this: usize, date: usize, slice: u32, people: u32) {
    let mut table = read_usize(G_STAFF_TABLE);
    let mut d = [0u16; 4];
    let dec31 = exe::date_date_549f70(&mut d, 0x1f, 0xb, read_u16(date + 2) as u32, u32::MAX);
    let dec31 = contract_day_same(dec31, date);
    match slice as u8 {
        0 => contract_day_fixed_dates(date),
        2 => {
            contract_day_expire_talks(this, date);
            let steps = ftol(read_i32(G_STAFF_COUNT) as f64 * read_f64(rdata::F64_0_004));
            let mut progress: i32 = 0;
            let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
            if dec31 {
                text(contract_day_globals::PROGRESS_TEXT, contract_day_globals::PROGRESS_FORMAT);
                exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, contract_day_globals::PROGRESS_TEXT, 0, steps as u32, -1);
            }
            let mut i: i32 = 0;
            while i < read_i32(G_STAFF_COUNT) && people as u8 == 1 {
                if dec31 && i % 250 == 0 {
                    progress += 1;
                    text(contract_day_globals::PROGRESS_TEXT, contract_day_globals::PROGRESS_FORMAT);
                    exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, contract_day_globals::PROGRESS_TEXT, progress, steps as u32, -1);
                }
                contract_day_person(this, date, Staff(table));
                i += 1;
                table += staff::SIZE;
            }
            contract_day_decisions(this, date);
        }
        _ => {}
    }
    if read_i32(this + contract_manager_fields::DECISIONS_ADDED) != 0 {
        let k = read_i32(this + contract_manager_fields::DECISION_NEXT_INDEX);
        let n = read_i32(this + contract_manager_fields::DECISION_COUNT);
        let base = read_usize(this + contract_manager_fields::DECISION_DATES).wrapping_add((k as usize).wrapping_mul(14));
        exe::crt_qsort(base, (n - k) as u32, 14, contract_day_globals::COMPARE_DECISIONS);
        write_i32(this + contract_manager_fields::DECISIONS_ADDED, 0);
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x4cd9c0usize); // process_due_decision_dates
    f(this, date, slice);
}

/// Comparer for `contract_manager_daily_update`: the original only (it offers contracts, writes news, queues free
/// transfers, draws random numbers and moves the decision dates; it cannot run twice).
pub(crate) unsafe extern "thiscall" fn compare_contract_manager_daily_update(this: usize, date: usize, slice: u32, people: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_CONTRACT_MANAGER_DAILY_UPDATE)(this, date, slice, people) // simplification: original only
}

// ---------- batch 290: a person's starting contract (staff_contracts.cpp) ----------

/// The `.rdata` constants of the starting contract that `rdata_consts.rs` does not name yet (cm0102.exe 3.9.68).
#[allow(dead_code)]
pub(crate) mod initial_contract_rdata {
    pub(crate) const F32_32500: usize = 0x96fe64; // f32 = 32500.0
    pub(crate) const F32_21250: usize = 0x96fe60; // f32 = 21250.0
    pub(crate) const F64_150000: usize = 0x96fe58; // f64 = 150000.0
    pub(crate) const F64_9500000: usize = 0x96fe50; // f64 = 9500000.0
    pub(crate) const F64_0_21: usize = 0x96d398; // f64 = 0.21
    pub(crate) const F32_0_25: usize = 0x96d22c; // f32 = 0.25
    pub(crate) const F32_0_125: usize = 0x96e2bc; // f32 = 0.125
}

/// The league of a club (`club::DIVISION` in `G_COMP_OBJECT_TABLE`, a league: `COMP_TYPE` 2) answers a virtual slot
/// of `league_vtable` (`HAS_AUTOMATIC_PROMOTION`, `HAS_RELEGATION`) with ≠ 0; no club, division or league: false.
unsafe fn initial_contract_league_has(club: Club, slot: usize) -> bool {
    let div = read_usize(club.addr() + club::DIVISION);
    if div == 0 {
        return false;
    }
    let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(div) as usize).wrapping_mul(4)));
    if comp == 0 || read_u8(comp + 0x43) != 2 {
        return false;
    }
    let m: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(comp) + slot));
    m(comp) != 0
}

/// A bonus of the starting contract: wage / 20 − random(wage / 40) + 25.
unsafe fn initial_contract_bonus(wage: i32) -> i32 {
    let r = exe::utils_random(wage / 40);
    (wage / 20).wrapping_sub(r).wrapping_add(25)
}

/// Generates a person's starting contract (the contract record in ecx, the person, the value) from the database data
/// or, without a usable database wage, from the default contract.
///
/// `generate_initial_contract(person, value)`: the contract's flags reset (`CONTRACT_TYPE` bit 0x80, Bosman, transfer
/// arranged −1, squad status 2, appearances, squad number, +8, morale, grievances, `FLAGS`, `TRANSFER_STATUS` 4, +0x3e,
/// +0x3f) and the person's opinion links cleared (`exe::staff_clear_opinion_links`); with a club job of a nation, the
/// low nibble of `SQUAD_STATUS` is his fluency in its first language (`exe::playerstats_get_language_fluency`, at most
/// 10). The club is his club job, or the club it is the B team of (`exe::club_get_linked_b_team`). A value below 0 is
/// computed (`exe::contract_manager_calc_player_value`) and marks the contract as generated. Without a club: type 3
/// and nothing else. The level: club reputation / 500 + 1 (at a club of professional status 2, one less, between 1 and
/// 4), plus for an international min(4, league standard × 0.25 × caps × 0.125) (0 counts as 1; no nation, 1).
///
/// The database path (a given value, a database wage (`staff::WAGE`) not 0 and at most 15 × the current reputation):
/// type 2, the start the date he joined the club (`DATE_JOINED_CLUB`), with the years since then
/// (`exe::date_get_difference`): a wage above 32500 (over 1 year) or above 21250 for a defender of 20 starts in the
/// game's first year; otherwise the start goes back to today − 2 years (outside 24-29 years, over 2 years) or today − 3
/// (over 3 years); the end is the database's (`DATE_EXPIRES_CLUB`); the wage is the database's. The generated path: the
/// default contract (`exe::staff_contract_default_staff_contract`, job 11, kind 0) gives the wage used for the bonuses;
/// with the game started before today, the contract starts today and ends in a year (previous wage 0); otherwise it
/// starts when he joined (one year back after 2 years) or on 10 July 2000, type 2, the end from the database; the
/// contract wage stays −1 (the f32 −1.0 the original keeps for it).
///
/// The clauses, for a player sheet: manager job (`MANAGER_JOB_RC`) for an old (33+) player of reputation above 3250
/// with id % 3 = 0 and ambition + determination + professionalism + id % 10 + age − 28 above 45, or anyone with a
/// non-player sheet; a player (job 11): the minimum fee (`NON_PROMOTION_RC` = +0x1c) with reputation above 2750 when
/// random(20) + 2 × ambition − 3 × loyalty + reputation / 50 > 12 × level and the league has automatic promotion: fee
/// value + random(value / 4), at least 250000; +0x1e with reputation above 6500 when reputation / 50 − 4 × loyalty −
/// random(20) > 14 × level: v = value + random(value / 4), fee v + max(0.95 v + 150000, 2v) (above 12 million, × 0.21 +
/// 9.5 million), at least 250000; the relegation clause (`RELEGATION_RC`) with reputation above 1750 when reputation /
/// 50 − 2 × loyalty + 3 × ambition − random(20) > 10 × level and the league has relegation: value − random(value / 2),
/// at least 50000, × 0.5 (loyalty < 5, ambition > 10), × 0.75 (ambition > 10) or × 0.85 (loyalty < 15). Each clause
/// clears the minimum fee flag (+0x1d) and the last one sets the release fee. The bonuses from the wage: goals for an
/// attacker of 15+, assists for a midfielder, attacking midfielder or attacker of 15+, clean sheets for a goalkeeper of
/// 20 (wage / 20 − random(wage / 40) + 25; otherwise −1). Without a player sheet no clauses or bonuses. At a club of
/// professional status 2 the type gets bit 0x80; of status 3 the clauses and bonuses are cleared. Then the wage.
///
/// **Defect of the original:** the per-nation wage rules after the clauses (0x85f26d..0x85f74b: South Korea, Japan,
/// the United States and 20 more nations by level², the club's attendance and its professional status) never run: they
/// need the generated contract (the flag at 0) and a wage other than −1.0, and the generated path always keeps −1.0;
/// the copy leaves them out. A person with neither sheet reads an uninitialised reputation (the copy uses 0). Returns
/// the wage (the no-club path returns 0; the callers do not read it). The SEH frame only matters for exceptions.
/// Original: `STAFF_CONTRACT::generate_initial_contract` (0x85e960, staff_contracts.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn generate_initial_contract(this: usize, person: usize, value: i32) -> i32 {
    use initial_contract_rdata::*;
    let c = this;
    let p = Staff(person);
    let mut value = value;
    let mut from_db = true;
    let mut years = [0u8; 8];
    exe::date_date_54a110(years.as_mut_ptr() as usize);
    let mut july = [0u16; 4];
    exe::date_date_549f70(&mut july, 0xa, 6, 0x7d0, u32::MAX);
    write_u8(c + contract::CONTRACT_TYPE, read_u8(c + contract::CONTRACT_TYPE) & 0x7f);
    write_u8(c + contract::LEAVING_ON_BOSMAN, 0);
    write_i32(c + contract::TRANSFER_ARRANGED_FOR, -1);
    write_u8(c + contract::SQUAD_STATUS, (read_u8(c + contract::SQUAD_STATUS) & 0xf) | 0x20);
    write_u16(c + contract::CLUB_APPEARANCES, 0);
    write_u8(c + contract::SQUAD_NUMBER, 0);
    write_u32(c + contract::UNKNOWN, 0);
    let job_club = p.club_job();
    write_u8(c + contract::MORALE, 0);
    write_u8(c + contract::TRANSFER_STATUS, 0);
    write_u32(c + contract::GRIEVANCES, 0);
    exe::staff_clear_opinion_links(staff_at(read_i32(c) as isize));
    write_u32(c + contract::FLAGS, 0);
    write_u8(c + contract::TRANSFER_STATUS, 4);
    write_u8(c + contract::MATCHES_LEFT_OUT, 0);
    write_u8(c + contract::UNHAPPY_FLAG, 0);
    if !job_club.is_null() {
        let nation = read_usize(job_club.addr() + club::NATION);
        if nation != 0 {
            let f = exe::playerstats_get_language_fluency(read_usize(G_PLAYER_STATS), p, job_club, read_u8(nation + nation::FIRST_LANGUAGE) as u32, 0) as i8;
            let v = if f > 10 { 10 } else { f };
            let s = read_u8(c + contract::SQUAD_STATUS);
            write_u8(c + contract::SQUAD_STATUS, (s & 0xf0) | (v as u8 & 0xf));
        }
    }
    let mut club = job_club;
    if !club.is_null() {
        let mut out: u32 = 0;
        let parent = exe::club_get_linked_b_team(club, &mut out, 1);
        if parent != 0 && out == 0 {
            club = Club(parent);
        }
    }
    if value < 0 {
        from_db = false;
        value = exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), p, club, 0, -1, 0);
    }
    if club.is_null() {
        write_u8(c + contract::CONTRACT_TYPE, (read_u8(c + contract::CONTRACT_TYPE) & 0xc3) | 3);
        return 0;
    }
    let sheet = p.player().addr();
    let mut level = ((club.reputation() as i16 as i32) / 500 + 1) as i8;
    if club.professional_status() == 2 {
        level = level.wrapping_sub(1);
        if level < 1 {
            level = 1;
        } else if level > 4 {
            level = 4;
        }
    }
    write_u32(c + contract::CLUB, read_u32(club.addr()));
    let caps = read_u8(p.addr() + staff::INT_APPS);
    if caps != 0 {
        let nation = read_usize(p.addr() + staff::NATION);
        let add: i8 = if nation == 0 {
            1
        } else {
            let std = read_i8(nation + nation::LEAGUE_STANDARD) as f64 * read_f32(F32_0_25) as f64;
            let a = ftol(std * (caps as f64 * read_f32(F32_0_125) as f64)) as i8;
            if a > 4 { 4 } else if a == 0 { 1 } else { a }
        };
        level = level.wrapping_add(add);
    }
    let rep: i16 = if sheet != 0 {
        read_i16(sheet + player_sheet::CURRENT_REPUTATION)
    } else if !p.non_player().is_null() {
        read_i16(p.non_player().addr() + 0xa)
    } else {
        0 // the original reads an uninitialised local
    };
    let wage_f: f32;
    let wage: i32;
    let db_wage = read_i32(p.addr() + staff::WAGE);
    if from_db && db_wage != 0 && db_wage <= rep as i32 * 15 {
        // the database contract
        wage_f = db_wage as f32;
        wage = db_wage;
        write_u8(c + contract::CONTRACT_TYPE, (read_u8(c + contract::CONTRACT_TYPE) & 0xc2) | 2);
        let joined = p.addr() + staff::DATE_JOINED_CLUB;
        exe::date_days_since(G_CURRENT_DATE, read_u32(joined), read_u32(joined + 4));
        let mut t = [0u8; 8];
        let mut u = [0u8; 8];
        let d = exe::date_get_difference(G_CURRENT_DATE, t.as_mut_ptr(), read_u32(joined), read_u32(joined + 4));
        exe::date_operator_3d(years.as_mut_ptr() as usize, u.as_mut_ptr() as usize, d);
        exe::date_operator_3d(c + contract::DATE_STARTED, u.as_mut_ptr() as usize, joined);
        let since = read_u16(years.as_ptr() as usize + 2);
        let sy = c + contract::DATE_STARTED_YEAR;
        if wage_f > read_f32(F32_32500) && since > 1 {
            write_u16(sy, read_u16(G_GAME_START_DATE + 2));
        } else if wage_f > read_f32(F32_21250) && sheet != 0 && read_i8(sheet + player_sheet::DEFENDER) == 20 {
            write_u16(sy, read_u16(G_GAME_START_DATE + 2));
        } else {
            let age = p.age() as i8;
            if !(0x18..=0x1d).contains(&age) && since > 2 {
                write_u16(sy, read_u16(G_CURRENT_DATE + 2).wrapping_sub(2));
            } else if since > 3 {
                write_u16(sy, read_u16(G_CURRENT_DATE + 2).wrapping_sub(3)); // mke3: the database start goes back 3 years
            }
        }
        exe::date_operator_3d(c + contract::CONTRACT_EXPIRES, u.as_mut_ptr() as usize, p.addr() + staff::DATE_EXPIRES_CLUB);
    } else {
        // the generated contract
        wage = ftol(exe::staff_contract_default_staff_contract(c, p, club, 0, u32::MAX, 0xb, 0));
        wage_f = -1.0; // the original stores 0xbf800000
        let mut u = [0u8; 8];
        if fill_slot_date_before(G_GAME_START_DATE, G_CURRENT_DATE) {
            write_u8(c + contract::CONTRACT_TYPE, (read_u8(c + contract::CONTRACT_TYPE) & 0xc2) | 2);
            exe::date_operator_3d(c + contract::DATE_STARTED, u.as_mut_ptr() as usize, G_CURRENT_DATE);
            write_u16(c + contract::CONTRACT_EXPIRES_YEAR, read_u16(G_CURRENT_DATE + 2).wrapping_add(1));
            write_u16(c + contract::CONTRACT_EXPIRES, read_u16(G_CURRENT_DATE));
            write_u32(c + contract::PREVIOUS_WAGE, 0);
        } else {
            let joined = p.addr() + staff::DATE_JOINED_CLUB;
            if fill_slot_date_before(joined, G_CURRENT_DATE) {
                exe::date_operator_3d(c + contract::DATE_STARTED, u.as_mut_ptr() as usize, joined);
                let s = c + contract::DATE_STARTED;
                let mut t = [0u8; 8];
                let d = exe::date_get_difference(G_CURRENT_DATE, t.as_mut_ptr(), read_u32(s), read_u32(s + 4));
                exe::date_operator_3d(years.as_mut_ptr() as usize, u.as_mut_ptr() as usize, d);
                if read_u16(years.as_ptr() as usize + 2) > 2 {
                    write_u16(c + contract::DATE_STARTED_YEAR, read_u16(G_CURRENT_DATE + 2).wrapping_sub(1));
                }
            } else {
                exe::date_operator_3d(c + contract::DATE_STARTED, u.as_mut_ptr() as usize, july.as_ptr() as usize);
            }
            write_u8(c + contract::CONTRACT_TYPE, (read_u8(c + contract::CONTRACT_TYPE) & 0xc2) | 2);
            exe::date_operator_3d(c + contract::CONTRACT_EXPIRES, u.as_mut_ptr() as usize, p.addr() + staff::DATE_EXPIRES_CLUB);
        }
        write_u32(c + contract::UNKNOWN, 0);
    }
    write_u32(c + contract::RELEASE_FEE, 0);
    write_u8(c + contract::MANAGER_JOB_RC, 0);
    write_u8(c + contract::MINIMUM_FEE_RC, 0);
    let clear = |c: usize| {
        for k in 0x1c..=0x20 {
            write_u8(c + k, 0);
        }
        write_i32(c + contract::GOAL_BONUS, -1);
        write_i32(c + contract::ASSIST_BONUS, -1);
        write_i32(c + contract::CLEAN_SHEET_BONUS, -1);
        write_u32(c + contract::RELEASE_FEE, 0);
    };
    if sheet != 0 {
        write_u8(c + contract::MANAGER_JOB_RC, 0);
        let age = p.age() as i8;
        let id = read_i32(p.addr());
        let mut manager_job = false;
        if age > 0x20 && rep > 0xcb2 && id % 3 == 0 {
            let s = read_i8(p.addr() + staff::PROFESSIONALISM) as i32 + read_i8(p.addr() + staff::DETERMINATION) as i32 + read_i8(p.addr() + staff::AMBITION) as i32 + id % 10 + age as i32 - 0x1c;
            manager_job = s > 0x2d;
        }
        if manager_job || !p.non_player().is_null() {
            covered!("mke_3 manager job clause");
            write_u8(c + contract::MINIMUM_FEE_RC, 0);
            write_u8(c + contract::MANAGER_JOB_RC, 1);
        }
        let amb = read_i8(p.addr() + staff::AMBITION) as i32;
        let loy = p.loyalty() as i32;
        let rep = rep as i32;
        let lv = level as i32;
        // the minimum fee (non-promotion) clause
        let mut set = false;
        if p.job_for_club() == 0xb && rep > 0xabe {
            let r = exe::utils_random(20);
            if r + (amb * 2 - loy * 3) + rep / 50 > lv * 12 && initial_contract_league_has(club, league_vtable::HAS_AUTOMATIC_PROMOTION) {
                let fee = (value.wrapping_add(exe::utils_random(value / 4))).max(250000);
                write_u8(c + contract::MINIMUM_FEE_RC, 0);
                write_u8(c + contract::NON_PROMOTION_RC, 1);
                write_i32(c + contract::RELEASE_FEE, fee);
                set = true;
                covered!("mke_3 non-promotion clause");
            }
        }
        if !set {
            write_u8(c + contract::NON_PROMOTION_RC, 0);
        }
        // the +0x1e clause
        set = false;
        if p.job_for_club() == 0xb && rep > 0x1964 {
            let r = exe::utils_random(20);
            if rep / 50 - loy * 4 - r > lv * 14 {
                let v = value.wrapping_add(exe::utils_random(value / 4));
                let a = v as f64 * read_f64(rdata::F64_0_95) + read_f64(F64_150000);
                let b = v.wrapping_mul(2) as f64;
                let mut y = ftol(if a > b { a } else { b });
                if y > 12_000_000 {
                    y = ftol(y as f64 * read_f64(F64_0_21) + read_f64(F64_9500000));
                }
                let fee = v.wrapping_add(y).max(250000);
                write_u8(c + contract::MINIMUM_FEE_RC, 0);
                write_u8(c + 0x1e, 1);
                write_i32(c + contract::RELEASE_FEE, fee);
                set = true;
                covered!("mke_3 clause 0x1e");
            }
        }
        if !set {
            write_u8(c + 0x1e, 0);
        }
        // the relegation clause
        set = false;
        if p.job_for_club() == 0xb && rep > 0x6d6 {
            let r = exe::utils_random(20);
            if rep / 50 - loy * 2 + amb * 3 - r > lv * 10 && initial_contract_league_has(club, league_vtable::HAS_RELEGATION) {
                let v = value.wrapping_sub(exe::utils_random(value / 2)).max(50000);
                let fee = if loy < 5 && amb > 10 {
                    ftol(v as f64 * read_f64(rdata::F64_0_5))
                } else if loy >= 5 && amb > 10 {
                    ftol(v as f64 * read_f64(rdata::F64_0_75))
                } else if loy < 15 {
                    ftol(v as f64 * read_f64(rdata::F64_0_85)) // mke3: the relegation fee × 0.85
                } else {
                    v
                };
                write_u8(c + contract::MINIMUM_FEE_RC, 0);
                write_u8(c + contract::RELEGATION_RC, 1);
                write_i32(c + contract::RELEASE_FEE, fee);
                set = true;
                covered!("mke_3 relegation clause");
            }
        }
        if !set {
            write_u8(c + contract::RELEGATION_RC, 0);
        }
        // the bonuses
        let goal = if read_i8(sheet + player_sheet::ATTACKER) >= 15 { initial_contract_bonus(wage) } else { -1 };
        write_i32(c + contract::GOAL_BONUS, goal);
        let assist = if read_i8(sheet + player_sheet::MIDFIELDER) >= 15 || read_i8(sheet + player_sheet::ATTACKING_MIDFIELDER) >= 15 || read_i8(sheet + player_sheet::ATTACKER) >= 15 {
            initial_contract_bonus(wage)
        } else {
            -1
        };
        write_i32(c + contract::ASSIST_BONUS, assist);
        let clean = if read_i8(sheet + player_sheet::GOALKEEPER) == 20 { initial_contract_bonus(wage) } else { -1 };
        write_i32(c + contract::CLEAN_SHEET_BONUS, clean);
    } else {
        clear(c);
    }
    match club.professional_status() {
        2 => write_u8(c + contract::CONTRACT_TYPE, read_u8(c + contract::CONTRACT_TYPE) | 0x80),
        3 => clear(c),
        _ => {}
    }
    let w = ftol(wage_f as f64);
    write_i32(c + contract::WAGE, w);
    w
}

/// Comparer for `generate_initial_contract`: side by side, the contract record (0x50 bytes), the person's 79-byte
/// state record (the opinion links), the cached contract and the generator's state restored between the two.
pub(crate) unsafe extern "thiscall" fn compare_generate_initial_contract(this: usize, person: usize, value: i32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32) -> i32>(IDX_GENERATE_INITIAL_CONTRACT);
    let id = read_i32(this);
    if id < 0 || id >= read_i32(G_STAFF_COUNT) {
        return o(this, person, value);
    }
    let est = crate::rng::STATE_REGIONS;
    let regions = [(this, 0x50usize), (contract_manager_staff_state_record(id as usize), staff_state::SIZE), (G_CONTRACT_LOOKUP_RESULT, 4usize), est[0], est[1]];
    let (r, _n, same, mo, mr) = crate::comparison::side_by_side_light(&regions,
        || o(this, person, value),
        || crate::rng::without_counting(|| generate_initial_contract(this, person, value)));
    SCOREBOARD.record(same, || format!("generate_initial_contract(person {:#x}, value {}) {:x?} x {:x?}", person, value, mo, mr));
    r
}

// ---------- batch 290: a person moves to his new club (transfer_manager.cpp) ----------

/// The `.rdata` constants of the move to a new club that `rdata_consts.rs` does not name yet (cm0102.exe 3.9.68).
#[allow(dead_code)]
pub(crate) mod move_to_club_rdata {
    pub(crate) const F64_0_0005: usize = 0x96a658; // f64 = 0.0005
    pub(crate) const F64_0_001: usize = 0x96a350; // f64 = 0.001
    pub(crate) const F64_0_003: usize = 0x970410; // f64 = 0.003
    pub(crate) const F64_5000: usize = 0x9688e8; // f64 = 5000.0
    pub(crate) const F64_2_9: usize = 0x970408; // f64 = 0.2222222222222222 (2/9)
    pub(crate) const F64_1_150: usize = 0x96d1d0; // f64 = 0.006666666666666667 (1/150)
    pub(crate) const F64_1_5: usize = 0x968940; // f64 = 1.5
    pub(crate) const F64_0: usize = 0x9689a0; // f64 = 0.0
    pub(crate) const F64_0_6: usize = 0x968e48; // f64 = 0.6
    pub(crate) const F32_0: usize = 0x968970; // f32 = 0.0
    pub(crate) const F64_2500: usize = 0x9690b0; // f64 = 2500.0
}

/// The club of the person's contract (`exe::contract_manager_get_contract`, stored in `G_CONTRACT_LOOKUP_RESULT`); with
/// `not_loan`, a loan contract (type 5) gives no club; no contract, no club.
unsafe fn move_contract_club(p: Staff, not_loan: bool) -> Club {
    let c = Contract(contract_of_staff_cached(p));
    if c.is_null() || (not_loan && c.contract_type() & 0x3f == 5) {
        return Club(0);
    }
    transfer_manager_club_by_id(c.club())
}

/// One person (player or staff) moves to his new club `club` through the completed offer `offer`.
///
/// `move_player_to_club(person, club, offer)` (the transfer manager in ecx): a local contract offer
/// (`exe::new_staff_contract`, its date `exe::date_date_54a110`); the club's manager's note on the person removed
/// (`Notes::remove_note`, 0x795660, with notes); the old job and the old club (the main contract's), and whether he was
/// happy there (the contract with that club, `exe::contract_manager_get_contract_for_club`: happiness above 0 and fewer
/// than 5 matches left out). The agreed contract offer (`exe::contract_manager_find_contract_offer` by the person and
/// the club); without one, the local offer gets the desired contract (`exe::staff_contract_set_to_desired_contract`,
/// job 11) with job 11 at +0x61 and no staff record (+0x63 = −1). A national manager leaving for a non-coach job and a
/// club manager or assistant (jobs 5, 12) leave their posts (`exe::manager_manager_remove_club_manager` with 4).
///
/// The past transfer record (`exe::transfer_manager_add_past_transfer`): the person, his three name ids, the buyer, the
/// seller (the main contract's club, not on loan; −1), the fee (the offer's when it is for him), the type (4 a
/// pre-contract with no fee completing at the contract end, 1 with exchange players, 5 a non-player job, else 0) and
/// today. A sale above 2.5 million and above (resources + ambition + 750) × the club's reputation: with the chairman's
/// interference above random(30), the person enters the chairman's favoured staff (`exe::staff_add_favoured_staff` with
/// random(2)).
///
/// At an AI club, the shortlist slot (one of the first 25) that had him as a player target: his rating
/// (`exe::match_man_score_player_for_selection`, 0x5e82) is kept and the slot's replacement, still at the club, loses
/// 50 happiness (at least −100) and goes on the shortlists. On the club's shortlist pairs
/// (`exe::shortlist_manager_is_on_shortlist`) the club's status becomes 2; his old club's (another club) 4; a loan's
/// owner club 4 too. A human buyer's scout shortlist drops him (`SCOUT_MANAGER::remove_from_shortlist`, 0x7f7a20); his
/// transfer record loses bit 0x10.
///
/// With a loan contract (`exe::contract_manager_get_loan_contract`): going to another club than the loan club (or its
/// main club), the loan ends (`end_loan`) and the bans move (`clear_bans_invalid_for_club`); back to the loan's owner,
/// the stats are archived with the fee, the squad number kept, the held bans dropped, the loan contract removed, he
/// leaves the club and takes the main contract's club and squad flags (by the B team, `exe::staff_set_squad_flags`).
/// Without a loan: a loan-type main contract keeps the squad number and he leaves the club with no club job; otherwise
/// the bans move. A player's squad selection (+0x6d) is 8 when he has played (`exe::playerstats_get_stat` 4 above 0);
/// the stats are archived (with the fee, the loan flag of type 1 and the pre-contract mark) unless done already. With a
/// contract club, the five former clubs move down and the newest gets the club and the years there. The old club drops
/// him (shortlist days 100 if he was not listed, the selections and the club), the virtual players
/// (`exe::virtual_staff_remove_virtual_players`) and he joins the new club (`exe::database_add_staff_to_club` with the
/// offer's job and today), taking the offer's staff record. The vacated jobs: a scout (9) or another non-player job at
/// the old and new clubs (`exe::scout_manager_sync_scout_lists`, `exe::training_manager_update_coach_assignments`,
/// `fill_vacated_staff_job`).
///
/// A player: the value (the offer's total `exe::transfer_offer_get_total_value` or `exe::contract_manager_calc_player_value`);
/// the buyer gains world reputation / 100 (at most 10000), the seller loses it; the reputation floor r from the value
/// (above 5 million × 0.0005 + 2500; above 2.5 million × 0.001; else × 0.003 − 5000): a human club max(reputation / 3 −
/// 750, r), an AI club max(reputation / 2 − 1000, r) and, with the shortlist rating, max(that, (current reputation ×
/// (150 − m) + m × rating × 2/9) / 150) with m the manager's and club's reputations / 200 (or the club's / 150); at most
/// 9500. The unsettlement: random(20 − adaptability) + random(20 − leadership) + random(gap / max(20 × pressure, 1)),
/// the gap the reputation above the seller's (or above his own), × (1.5 − fluency × 0.1), at most 100, added to the
/// 79-byte record's +0xa. His reputations: current = (home or world by nation)/2 + current/2, at least the floor; home
/// at least floor − 2000 + 0.15 × the club's reputation; world at least floor − 8000 + 0.6 × the club's.
///
/// Then the contract (transfer status 4, `exe::contract_manager_sign_contract` with his job), the scout requests
/// (`remove_scout_request`), the talks (`remove_talks_records`), the sign-on fee (`exe::finance_pay_sign_on_fee`); a
/// player's value at the new club and the clubs' shortlists suspended (`exe::shortlist_manager_suspend_clubs_shortlisting_player`);
/// the squad number (`exe::squad_manager_set_squad_number`); the squad flags (2 at a human club with a reserve squad
/// and no B team for status 0 or above 4, else 1); the work permit (`exe::transfer_manager_update_work_permit_on_move`);
/// the retirement bits cleared (`STATUS_FLAGS` & 0xcf); the caps in the transfer record; AWOL cleared
/// (`exe::awol_clear_awol`); the squad reactions (`process_transfer_squad_reactions`). The SEH frame only matters for
/// exceptions.
/// Original: `TRANSFER_MANAGER::move_player_to_club` (0x8c9720, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn transfer_manager_move_player_to_club(this: usize, person: usize, club: usize, offer: usize) {
    let mut local = [0u8; 0x80];
    let l = local.as_mut_ptr() as usize;
    exe::new_staff_contract(l);
    exe::date_date_54a110(l + 0x51);
    let mut record = [0u8; 0x28];
    move_to_club_body(this, Staff(person), Club(club), Offer(offer), l, record.as_mut_ptr() as usize);
}

/// The body of `transfer_manager_move_player_to_club` after the local offer is built; `rec` is the past transfer
/// record (0x25 bytes: person, 3 name ids, buyer, seller, fee, type, date).
#[allow(clippy::too_many_arguments)]
unsafe fn move_to_club_body(this: usize, p: Staff, club: Club, offer: Offer, local: usize, rec: usize) {
    use move_to_club_rdata::*;
    let cm = read_usize(G_CONTRACT_MANAGER);
    let sm = read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER);
    let mut pre_contract: u32 = 0; // [esp+0x30]
    let mut found_slot = false; // [esp+0x20]
    let mut archived = false; // [esp+0x10]
    let mut squad_number: u8 = 0; // [esp+0x2c]
    let mut rating: i32 = 0; // [esp+0x18]
    if read_usize(G_NOTES) != 0 && !club.is_null() && !club.manager().is_null() {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x795660usize); // Notes::remove_note
        f(read_usize(G_NOTES), club.manager().addr(), p.addr());
    }
    let old_job = p.job_for_club();
    let old_club = move_contract_club(p, false);
    let mut happy = false;
    let cc = Contract(exe::contract_manager_get_contract_for_club(cm, p, old_club));
    if !cc.is_null() && cc.happiness() > 0 && read_i8(cc.addr() + contract::UNHAPPY_FLAG) < 5 {
        happy = true;
    }
    let mut oc = exe::contract_manager_find_contract_offer(cm, read_u32(p.addr()), read_u32(club.addr()));
    if oc == 0 {
        exe::staff_contract_set_to_desired_contract(local, p, club, -1, u32::MAX, 0xb, 0, 0);
        write_u8(local + 0x61, 0xb);
        write_i32(local + 0x63, -1);
        oc = local;
    }
    let oc = Contract(oc);
    if p.job_for_nation() == 5 && read_usize(p.addr() + staff::NATIONAL_JOB) != 0 && offer.job_type() != 9 {
        exe::manager_manager_remove_club_manager(read_usize(G_MANAGER_MANAGER), read_usize(p.addr() + staff::NATIONAL_JOB), 4, 0, 0);
    }
    let job = p.job_for_club();
    if (job == 5 || job == 0xc) && !p.club_job().is_null() {
        exe::manager_manager_remove_club_manager(read_usize(G_MANAGER_MANAGER), p.club_job(), 4, 0, 0);
    }
    // the past transfer record
    exe::date_date_54a110(rec + 0x1d);
    let seller = move_contract_club(p, true);
    let seller_id: i32 = if seller.is_null() { -1 } else { read_i32(move_contract_club(p, true).addr()) };
    write_i32(rec + 0x10, read_i32(club.addr()));
    write_i32(rec + 0x14, seller_id);
    let mut t = [0u8; 8];
    exe::date_operator_3d(rec + 0x1d, t.as_mut_ptr() as usize, G_CURRENT_DATE);
    write_u32(rec, read_u32(p.addr()));
    write_u32(rec + 4, read_u32(read_usize(p.addr() + staff::FIRST_NAME) + 0x33));
    write_u32(rec + 8, read_u32(read_usize(p.addr() + staff::SECOND_NAME) + 0x33));
    write_u32(rec + 0xc, read_u32(read_usize(p.addr() + staff::COMMON_NAME) + 0x33));
    let kind: u8 = if offer.pre_contract() != 0 && read_i32(offer.addr() + offer::FEE) == 0 && matches!(offer.completion_timing() & 0xf, 3 | 4) {
        pre_contract = 1;
        4
    } else {
        (read_usize(offer.addr() + offer::EXCHANGE_PLAYERS) != 0) as u8
    };
    write_u8(rec + 0x1c, if exe::database_is_player_job(offer.job_type() as i8) == 0 { 5 } else { kind });
    let fee = if read_u32(p.addr()) == read_u32(offer.addr() + offer::TARGET_STAFF_ID) { read_i32(offer.addr() + offer::FEE) } else { 0 };
    write_i32(rec + 0x18, fee);
    let chairman = Staff(read_usize(club.addr() + club::CHAIRMAN));
    if !chairman.is_null() && !chairman.non_player().is_null() {
        let np = chairman.non_player().addr();
        let paid = read_i32(offer.addr() + offer::FEE);
        if paid > 2_500_000
            && (read_i8(np + 0x20) as i32 + read_i8(chairman.addr() + staff::AMBITION) as i32 + 0x2ee).wrapping_mul(club.reputation() as i32) < paid
        {
            let r = exe::utils_random(0x1e);
            if read_i8(read_usize(read_usize(club.addr() + club::CHAIRMAN) + staff::NON_PLAYER) + 0x16) as i32 > r {
                covered!("mke_4 chairman's favourite");
                let r2 = exe::utils_random(2);
                exe::staff_add_favoured_staff(read_usize(club.addr() + club::CHAIRMAN), p, r2 as u32);
            }
        }
    }
    exe::transfer_manager_add_past_transfer(this, rec as *mut u8);
    // the AI buyer's shortlist slot that had him as a target
    let entry = read_usize(sm + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add((read_u32(club.addr()) as usize).wrapping_mul(9));
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 && entry != 0 {
        let slots = read_usize(entry);
        for k in 0..0x19usize {
            let slot = slots.wrapping_add(k * 0x25);
            if read_u32(slot + shortlist_slot::TARGET_STAFF_ID) == read_u32(p.addr()) && read_u8(slot) == 0xb {
                rating = exe::match_man_score_player_for_selection(p, u32::MAX, 0x5e82, club, -1, 0usize);
                found_slot = true;
                let rid = read_i32(slot + shortlist_slot::REPLACEMENT_STAFF_ID);
                if rid > -1 {
                    let rp = staff_at(rid as isize);
                    if move_contract_club(rp, true) == club {
                        let rc = Contract(exe::contract_manager_get_contract_for_club(cm, staff_at(read_i32(slot + shortlist_slot::REPLACEMENT_STAFF_ID) as isize), club));
                        let h = (rc.happiness() as i16).wrapping_sub(0x32);
                        let v: i8 = if h < -100 {
                            -100
                        } else if (h as i8) > 100 {
                            100
                        } else if (h as i8) < -100 {
                            -100
                        } else {
                            h as i8
                        };
                        rc.set_happiness(v);
                        covered!("mke_4 replaced player unsettled");
                        exe::shortlist_manager_add_player_to_club_shortlists(sm, staff_at(read_i32(slot + shortlist_slot::REPLACEMENT_STAFF_ID) as isize), 0);
                    }
                }
                break;
            }
        }
    }
    if exe::shortlist_manager_is_on_shortlist(sm, p, club) != 0 {
        exe::shortlist_manager_update_staff_status(sm, p, club, 2, (offer.job_type() != 0xb) as u32);
    }
    let cj = p.club_job();
    if !cj.is_null() && cj != club {
        exe::shortlist_manager_update_staff_status(sm, p, cj, 4, 2);
    }
    if p.club_job() != move_contract_club(p, true) && exe::contract_manager_get_loan_contract(cm, p) != 0 && !move_contract_club(p, true).is_null() {
        let owner = move_contract_club(p, true);
        exe::shortlist_manager_update_staff_status(sm, p, owner, 4, 2);
    }
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 && exe::scout_manager_is_player_shortlisted(read_usize(G_SCOUT_MANAGER), p, club.manager()) != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7f7a20usize); // SCOUT_MANAGER::remove_from_shortlist
        f(read_usize(G_SCOUT_MANAGER), p.addr(), club.manager().addr());
    }
    let flags = read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add((read_u32(p.addr()) as usize).wrapping_mul(8));
    write_u8(flags, read_u8(flags) & 0xef);
    let clear_bans = |p: Staff, from: Club, to: Club| {
        let f: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x556270usize); // clear_bans_invalid_for_club
        f(read_usize(G_DISCIPLINE), p.addr(), from.addr(), to.addr());
    };
    if exe::contract_manager_get_loan_contract(cm, p) != 0 {
        if club != p.club_job() && exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1) != p.club_job().addr() {
            // to another club: the loan ends
            covered!("mke_4 loan ended by the move");
            let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4d07e0usize); // end_loan
            f(cm, p.addr());
            clear_bans(p, move_contract_club(p, false), club);
        } else {
            // back to the loan's owner
            let owner = move_contract_club(p, false);
            exe::playerstats_archive_player_stats(read_usize(G_PLAYER_STATS), p, fee, 0, 0, read_i32(owner.addr()), 0);
            archived = true;
            squad_number = exe::squad_manager_get_squad_number(read_usize(G_SQUAD_MANAGER), p, p.club_job()) as u8;
            let drop: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5567d0usize); // drop_held_bans
            drop(read_usize(G_DISCIPLINE), p.addr());
            let remove: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4d0bd0usize); // remove_loan_contract
            remove(cm, p.addr());
            exe::database_remove_staff_from_club(p.club_job(), p, 0usize, 1, 0);
            write_usize(p.addr() + staff::CLUB_JOB, move_contract_club(p, false).addr());
            let main = Contract(exe::contract_manager_get_contract(cm, p));
            if exe::database_club_has_reserve_squad(p.club_job()) != 0 {
                exe::staff_set_squad_flags(p, if main.contract_type() & 0x40 == 0 { 1 } else { 2 });
            } else {
                let b = exe::club_get_linked_b_team(p.club_job(), core::ptr::null_mut(), 1);
                exe::staff_set_squad_flags(p, 1);
                if b != 0 {
                    for k in 0..50usize {
                        if read_usize(b + club::SQUAD + k * 4) == p.addr() {
                            write_usize(p.addr() + staff::CLUB_JOB, b);
                        }
                    }
                }
            }
        }
    } else {
        let c = Contract(contract_of_staff_cached(p));
        if !c.is_null() && c.contract_type() & 0x3f == 5 {
            squad_number = exe::squad_manager_get_squad_number(read_usize(G_SQUAD_MANAGER), p, p.club_job()) as u8;
            exe::database_remove_staff_from_club(p.club_job(), p, 0usize, 1, 0);
            write_usize(p.addr() + staff::CLUB_JOB, 0);
        } else {
            clear_bans(p, move_contract_club(p, false), club);
        }
    }
    if !p.player().is_null() {
        if p.squad_selected_for() != 0 {
            let played = exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), p.player(), 4, 1, 6.5);
            p.set_squad_selected_for(if played > read_f32(F32_0) as f64 { 8 } else { 0 });
        } else {
            p.set_squad_selected_for(0);
        }
        if !archived {
            exe::playerstats_archive_player_stats(read_usize(G_PLAYER_STATS), p, fee, (read_u8(rec + 0x1c) == 1) as u32, 0, read_i32(club.addr()), pre_contract);
        }
    }
    // the former clubs
    if !move_contract_club(p, false).is_null() {
        let r = contract_manager_staff_state_record(read_usize(p.addr()));
        for k in 0..4usize {
            write_u32(r + 0xe + k * 5, read_u32(r + 0x13 + k * 5));
            write_u8(r + 0xe + k * 5 + 4, read_u8(r + 0x13 + k * 5 + 4));
        }
        let now = move_contract_club(p, false);
        write_u32(r + 0x22, read_u32(now.addr()));
        write_u8(r + 0x26, read_u8(G_CURRENT_DATE + 2).wrapping_sub(read_u8(p.addr() + staff::DATE_JOINED_CLUB + 2)));
    }
    let cj = p.club_job();
    if !cj.is_null() {
        if exe::staff_is_transfer_listed(p, 0usize) == 0 {
            let e = read_usize(sm + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add((read_u32(cj.addr()) as usize).wrapping_mul(9));
            write_u8(e + club_shortlist::DAYS, 100);
        }
        exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), p.club_job(), p, 1, 1);
        exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), p.club_job(), p, 0, 1);
        exe::database_remove_staff_from_club(p.club_job(), p, 0usize, 1, 0);
    }
    exe::virtual_staff_remove_virtual_players(read_usize(G_VIRTUAL_STAFF), club, 0);
    exe::database_add_staff_to_club(club, p, read_u8(oc.addr() + 0x61) as u32, G_CURRENT_DATE, 1, 0);
    let np = read_i32(oc.addr() + 0x63);
    if np != -1 {
        if p.non_player().is_null() {
            write_usize(p.addr() + staff::NON_PLAYER, read_usize(G_NON_PLAYER_TABLE).wrapping_add((np as usize).wrapping_mul(0x44)));
        }
        write_i32(oc.addr() + 0x63, -1);
    }
    // the vacated jobs
    let fill: unsafe extern "stdcall" fn(usize, u32) = core::mem::transmute(0x8cb6d0usize); // fill_vacated_staff_job
    if old_job == 9 {
        if !old_club.is_null() {
            if exe::human_manager_is_human_club(old_club, 1, 1, 0, 0) != 0 {
                exe::scout_manager_sync_scout_lists(read_usize(G_SCOUT_MANAGER), old_club.manager());
            } else {
                fill(old_club.addr(), 9);
            }
        }
    } else if old_job != 0xb && !old_club.is_null() {
        if exe::human_manager_is_human_club(old_club, 1, 1, 0, 0) != 0 {
            exe::training_manager_update_coach_assignments(read_usize(G_TRAINING_MANAGER), old_club, 0);
        } else {
            fill(old_club.addr(), old_job as u32);
        }
    }
    let job = p.job_for_club();
    if job == 9 {
        if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
            exe::scout_manager_sync_scout_lists(read_usize(G_SCOUT_MANAGER), club.manager());
        }
    } else if job != 0xb && !p.club_job().is_null() && exe::human_manager_is_human_club(p.club_job(), 1, 1, 0, 0) != 0 {
        exe::training_manager_update_coach_assignments(read_usize(G_TRAINING_MANAGER), p.club_job(), 0);
    }
    if !p.player().is_null() && exe::database_is_player_job(p.job_for_club() as i8) != 0 {
        move_to_club_reputations(p, club, offer, seller_id, found_slot, rating);
    }
    // the contract
    write_u8(oc.addr() + contract::TRANSFER_STATUS, 4);
    exe::contract_manager_sign_contract(cm, oc, p.job_for_club() as u32);
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7f8ee0usize); // remove_scout_request
    f(read_usize(G_SCOUT_MANAGER), p.addr(), club.addr());
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x4ca1c0usize); // remove_talks_records
    f(cm, p.addr(), 0, 0);
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    exe::finance_pay_sign_on_fee(fin, read_u32(oc.addr() + contract::UNKNOWN));
    if !p.player().is_null() {
        let human = (exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0) as u32;
        let v = exe::contract_manager_calc_player_value(cm, p, club, 0, -1, human);
        write_i32(p.addr() + staff::VALUE, v);
        exe::shortlist_manager_suspend_clubs_shortlisting_player(sm, p, 1);
    }
    if exe::database_is_player_job(p.job_for_club() as i8) != 0 && exe::squad_manager_club_uses_squad_numbers(read_usize(G_SQUAD_MANAGER), club, 0) != 0 {
        exe::squad_manager_set_squad_number(read_usize(G_SQUAD_MANAGER), p, club, squad_number as u32);
    }
    let reserve = exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0
        && exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1) == 0
        && exe::database_club_has_reserve_squad(club) != 0
        && {
            let st = read_u8(oc.addr() + contract::SQUAD_STATUS) >> 4;
            st == 0 || st > 4
        };
    exe::staff_set_squad_flags(p, if reserve { 2 } else { 1 });
    exe::transfer_manager_update_work_permit_on_move(this, p, club);
    let r = contract_manager_staff_state_record(read_usize(p.addr())) + staff_state::STATUS_FLAGS;
    write_u8(r, read_u8(r) & 0xcf);
    let flags = read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add((read_u32(p.addr()) as usize).wrapping_mul(8));
    write_u8(flags + 7, read_u8(p.addr() + staff::INT_APPS));
    exe::awol_clear_awol(read_usize(G_AWOL), p);
    covered!("mke_4 moved");
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(0x758200usize); // process_transfer_squad_reactions
    f(read_usize(G_MEDIA), p.addr(), club.addr(), old_club.addr(), happy as u32);
}

/// A player's reputations after the move (see `transfer_manager_move_player_to_club`): the clubs' reputations, the floor
/// from the value, the unsettlement and his current, home and world reputations.
unsafe fn move_to_club_reputations(p: Staff, club: Club, offer: Offer, seller_id: i32, found_slot: bool, rating: i32) {
    use move_to_club_rdata::*;
    let cm = read_usize(G_CONTRACT_MANAGER);
    let sheet = p.player().addr();
    let value = if read_u32(p.addr()) == read_u32(offer.addr() + offer::TARGET_STAFF_ID) {
        exe::transfer_offer_get_total_value(offer, 0, 0, 0, 1)
    } else {
        exe::contract_manager_calc_player_value(cm, p, club, 0, -1, 0)
    };
    // the clubs' reputations
    let buyer = club_at(read_i32(club.addr()) as isize);
    let world = read_i16(sheet + player_sheet::WORLD_REPUTATION) as i32;
    write_i16(buyer.addr() + club::REPUTATION, read_i16(buyer.addr() + club::REPUTATION).wrapping_add((world / 100) as i16));
    if read_i16(buyer.addr() + club::REPUTATION) > 0x2710 {
        write_i16(buyer.addr() + club::REPUTATION, 0x2710);
    }
    if seller_id > -1 {
        let s = club_at(seller_id as isize);
        write_i16(s.addr() + club::REPUTATION, read_i16(s.addr() + club::REPUTATION).wrapping_add((-world / 100) as i16));
    }
    // the reputation floor from the value
    let v = value as f64;
    let y = ftol(if value > 0x4c4b40 {
        v * read_f64(F64_0_0005) + read_f64(F64_2500)
    } else if value > 0x2625a0 {
        v * read_f64(F64_0_001)
    } else {
        v * read_f64(F64_0_003) - read_f64(F64_5000)
    }) as i16 as i32;
    let rep = read_i16(buyer.addr() + club::REPUTATION) as i32;
    let mut floor: i32;
    if exe::human_manager_is_human_club(buyer, 0, 1, 0, 0) != 0 {
        let t = rep / 3 - 0x2ee;
        floor = if t > y { t } else { y };
    } else {
        let t = rep / 2 - 1000;
        floor = if t > y { t } else { y };
        if found_slot {
            let m = if !club.manager().is_null() && !club.manager().non_player().is_null() {
                read_i16(club.manager().non_player().addr() + 0xa) as i32 / 200 + club.reputation() as i32 / 200
            } else {
                club.reputation() as i32 / 150
            };
            let q = (read_i16(sheet + player_sheet::CURRENT_REPUTATION) as i32).wrapping_mul(150 - m);
            let r = m.wrapping_mul(rating);
            let z = ftol((q as f64 + r as f64 * read_f64(F64_2_9)) * read_f64(F64_1_150)); // mke4: the shortlist rating lifts the floor
            floor = if floor as i16 as i32 > z { floor as i16 as i32 } else { z };
        }
    }
    if floor as i16 > 0x251c {
        floor = 0x251c;
    }
    // the gap
    let gap = if seller_id != -1 {
        let d = club.reputation() as i32 - read_i16(club_at(seller_id as isize).addr() + club::REPUTATION) as i32;
        if d < 0 { 0 } else { d }
    } else {
        let d = club.reputation() as i32 - read_i16(sheet + player_sheet::CURRENT_REPUTATION) as i32;
        if d < 0 { 0 } else { d }
    };
    let pr = read_i8(p.addr() + staff::PRESSURE) as i32 * 20;
    let div = if pr < 1 { 1 } else { pr };
    let mut u = exe::utils_random(20 - read_i8(p.addr() + staff::ADAPTABILITY) as i32) as u8;
    u = u.wrapping_add(exe::utils_random(20 - read_i8(sheet + player_sheet::LEADERSHIP) as i32) as u8);
    u = u.wrapping_add(exe::utils_random((gap as i16 as i32) / div) as u8);
    let nation = read_usize(club.addr() + club::NATION);
    let lang = if nation == 0 { 0 } else { read_i8(nation + nation::FIRST_LANGUAGE) as i32 };
    let fl = exe::playerstats_get_language_fluency(read_usize(G_PLAYER_STATS), p, club, lang as u32, 0) as i8;
    let mut add = ftol((read_f64(F64_1_5) - fl as f64 * read_f64(rdata::F64_0_1)) * u as f64) as u8; // mke4: the unsettlement after a move
    if add > 100 {
        add = 100;
    }
    let r = contract_manager_staff_state_record(read_usize(p.addr())) + 0xa;
    write_u8(r, read_u8(r).wrapping_add(add));
    // his reputations
    let a = if read_usize(club.addr() + club::NATION) == read_usize(p.addr() + staff::NATION) {
        read_i16(sheet + player_sheet::HOME_REPUTATION) as i32
    } else {
        read_i16(sheet + player_sheet::WORLD_REPUTATION) as i32
    };
    let cur = read_i16(sheet + player_sheet::CURRENT_REPUTATION) as i32;
    write_i16(sheet + player_sheet::CURRENT_REPUTATION, (a / 2 + cur / 2) as i16);
    let fl16 = floor as i16;
    if read_i16(sheet + player_sheet::CURRENT_REPUTATION) <= fl16 {
        write_i16(sheet + player_sheet::CURRENT_REPUTATION, fl16);
    }
    let f = fl16 as i32;
    let mut h = (f - 2000) as f64 + club.reputation() as i32 as f64 * read_f64(rdata::F64_0_15);
    if read_f64(F64_0) > h {
        h = read_f64(F64_0);
    }
    let h = ftol(h) as i16;
    if !(read_i16(sheet + player_sheet::HOME_REPUTATION) > h) {
        write_i16(sheet + player_sheet::HOME_REPUTATION, h);
    }
    let mut w = club.reputation() as i32 as f64 * read_f64(F64_0_6) + (f - 8000) as f64;
    if read_f64(F64_0) > w {
        w = read_f64(F64_0);
    }
    let w = ftol(w) as i16;
    if !(read_i16(sheet + player_sheet::WORLD_REPUTATION) > w) {
        write_i16(sheet + player_sheet::WORLD_REPUTATION, w);
    }
}

/// Comparer for `transfer_manager_move_player_to_club`: the original only (it moves the person between clubs, signs
/// the contract, writes the past transfers, draws random numbers and posts news; it cannot run twice).
pub(crate) unsafe extern "thiscall" fn compare_transfer_manager_move_player_to_club(this: usize, person: usize, club: usize, offer: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_TRANSFER_MANAGER_MOVE_PLAYER_TO_CLUB)(this, person, club, offer) // simplification: original only
}

// ---------- batch 290: the news texts of contract talks, contract expiry and contract end (transfer_manager.cpp, contract_manager.cpp) ----------

/// The shared buffer the news texts are written into (`FN_LANGLIB_SPRINTF`), then given to the Text object.
const NEWS_TEXT_BUFFER: usize = 0xde1f64;

/// `langlib_sprintf` into `NEWS_TEXT_BUFFER` (the game's formatter of the news templates).
unsafe fn news_text_format() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}

/// `Text::append_string` (0x4ae770; thiscall, ret 4): appends a string to a Text object.
unsafe fn news_text_append(out: usize, s: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(0x4ae770usize);
    f(out, s);
}

/// The person's name from the news item's name ids (`exe::database_format_name_from_ids`, 300 bytes, mode `mode`)
/// into `buf`.
unsafe fn news_text_person_name(first: u32, second: u32, common: u32, buf: &mut [u8; 0x12c], mode: u32, id: u32) {
    exe::database_format_name_from_ids(first, second, common, buf.as_mut_ptr(), 0x12c, mode, id);
}

/// Links the person's name to the person (`exe::text_add_link` with `kind`) when the news item's names still match
/// him (`exe::database_staff_matches_names`).
unsafe fn news_text_link_person(out: usize, buf: &[u8; 0x12c], id: u32, first: u32, second: u32, common: u32, kind: u32) {
    let p = staff_at(id as i32 as isize);
    if exe::database_staff_matches_names(p, first, second, common, 0usize) != 0 {
        exe::text_add_link(out, buf.as_ptr(), kind, p);
    }
}

/// Links a club's short name to the club (`exe::text_add_link`, 0x7d5).
unsafe fn news_text_link_club(out: usize, id: u32) {
    let c = club_at(id as i32 as isize);
    exe::text_add_link(out, (c.addr() + club::SHORT_NAME) as *const u8, 0x7d5, c);
}

/// Writes the headline or the body of the news that a club may discuss a contract with a person, and the state of the
/// talks.
///
/// `write_contract_talks_news_text(news, full, out)`: the talks state is slot 9 of the news item; when it is 0, the
/// offer in slot 7 of the transfer manager's offers (index past the count reads the first) with the serial of slot 8
/// gives it (`TRANSFER_OFFER::get_player_talks_status`, 0x8e7f10, for the person of slot 0) and, when not 0, it is
/// written back into the stored item of the human manager's inbox (`News::set_item_slot`, 0x786d30, slot 9). The body:
/// the person's name (slots 1-3 and 0, mode 1), the template 0xa94c24 with the clubs of slots 4 and 5 (short names and
/// genders), then 0xa94be4 (state 1) or 0xa94b98 / 0xa94b48 (state 4, by slot 10) with the club of slot 5, or nothing;
/// the links to the club of slot 4 (when ≥ 0) and of slot 5, and to the person (0x7d7) when the names still match. The
/// headline: "Offer contract to" (0xa94b0c) with the name (mode 5) and the person's link.
/// Original: `TRANSFER_MANAGER::write_contract_talks_news_text` (0x8d7fb0, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_contract_talks_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let person = slot(0);
    let mut state = slot(9) as u8;
    if state == 0 {
        let index = slot(7) as i32;
        let serial = slot(8);
        let tm = read_usize(G_TRANSFER_MANAGER);
        let arr = read_usize(tm);
        let at = if index < read_i32(tm + 4) { arr.wrapping_add((index as usize).wrapping_mul(4)) } else { arr };
        let o = read_usize(at);
        if o != 0 && read_u32(o + offer::SERIAL) == serial {
            let status: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(0x8e7f10usize); // TRANSFER_OFFER::get_player_talks_status
            state = status(o, person);
            if state != 0 {
                covered!("mke_5 talks state stored");
                let hm = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
                let set: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, usize) = core::mem::transmute(0x786d30usize); // News::set_item_slot
                set(read_usize(G_NEWS), hm, read_u32(news + 0xd6), 9, state as i8 as i32, 0);
            }
        }
    }
    let mut name = [0u8; 0x12c];
    if full != 0 {
        let (s1, s2, s3, s4, s5, s10) = (slot(1), slot(2), slot(3), slot(4), slot(5), slot(0xa));
        news_text_person_name(s1, s2, s3, &mut name, 1, person);
        let c4 = club_at(s4 as i32 as isize).addr();
        let c5 = club_at(s5 as i32 as isize).addr();
        let g4 = read_i8(c4 + club::SHORT_GENDER_NAME) as i32;
        let g5 = read_i8(c5 + club::SHORT_GENDER_NAME) as i32;
        fmt(NEWS_TEXT_BUFFER, 0xa94c24, g4, g4, g5, g5, c4 + club::SHORT_NAME, c5 + club::SHORT_NAME, name.as_ptr() as usize);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        match state {
            1 => {
                fmt(NEWS_TEXT_BUFFER, 0xa94be4, g5, g5, c5 + club::SHORT_NAME);
            }
            4 => {
                let t = if s10 != 0 { 0xa94b98 } else { 0xa94b48 };
                fmt(NEWS_TEXT_BUFFER, t, g5, g5, c5 + club::SHORT_NAME);
            }
            _ => write_u8(NEWS_TEXT_BUFFER, 0),
        }
        news_text_append(out, NEWS_TEXT_BUFFER);
        if (s4 as i32) > -1 {
            news_text_link_club(out, s4);
        }
        news_text_link_club(out, s5);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
    } else {
        let (s1, s2, s3) = (slot(1), slot(2), slot(3));
        news_text_person_name(s1, s2, s3, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0xa94b0c, name.as_ptr() as usize); // "Offer contract to <name>"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
    }
}

/// Comparer for `write_contract_talks_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_contract_talks_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_CONTRACT_TALKS_NEWS_TEXT);
    compare_text_writer(out, || o(this, news, full, out), |t| write_contract_talks_news_text(this, news, full, t), || format!("write_contract_talks_news_text(news {:#x}, full {})", news, full));
}

/// The body templates of the contract expiry news by the period (slot 6): (another club, own club, other), period 1 = 3
/// months, 2 = 2 months, 3 = next month, 4 = next week, 5 = 6 months, 6 = 12 months; period 7 has one template.
const CONTRACT_EXPIRY_TEMPLATES: [(usize, usize, usize); 6] = [
    (0x9aac74, 0x9aab88, 0x9aab30),
    (0x9aaa68, 0x9aa980, 0x9aa92c),
    (0x9aa86c, 0x9aa78c, 0x9aa740),
    (0x9aa680, 0x9aa5a0, 0x9aa554),
    (0x9aaf0c, 0x9aae24, 0x9aadd0),
    (0x9ab118, 0x9ab02c, 0x9aafd4),
];

/// Writes the headline or the body of the news that a person's contract expires in twelve, six, three, two or one
/// months or next week.
///
/// `write_contract_expiry_news_text(news, full, out)`: the name from slots 3-5 and 2 (mode 1). The body by the period
/// (slot 6, `CONTRACT_EXPIRY_TEMPLATES`; slot 7 not 0 the first template, else slot 8 not 0 the second, else the third;
/// period 7 the template 0x9aad3c; another period leaves the buffer as it was), each with 100 and the name; the link
/// to the club of slot 0 (full name) and to the person (0x7d7) when the names still match. The headline: 0x9aa520
/// ("... contract") with 100 and the name, and the person's link.
/// Original: `CONTRACT_MANAGER::write_contract_expiry_news_text` (0x4d6a00, contract_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_contract_expiry_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let (club, person, s3, s4, s5, period, s7, s8) = (slot(0), slot(2), slot(3), slot(4), slot(5), slot(6), slot(7), slot(8));
    let mut name = [0u8; 0x12c];
    news_text_person_name(s3, s4, s5, &mut name, 1, person);
    let np = name.as_ptr() as usize;
    if full != 0 {
        match period as i32 {
            1..=6 => {
                let (a, b, c) = CONTRACT_EXPIRY_TEMPLATES[(period - 1) as usize];
                let t = if s7 != 0 { a } else if s8 != 0 { b } else { c };
                fmt(NEWS_TEXT_BUFFER, t, 100, np);
            }
            7 => {
                fmt(NEWS_TEXT_BUFFER, 0x9aad3c, 100, np);
            }
            _ => {} // the buffer keeps what it had
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        let c = club_at(club as i32 as isize);
        exe::text_add_link(out, (c.addr() + club::NAME) as *const u8, 0x7d5, c);
    } else {
        fmt(NEWS_TEXT_BUFFER, 0x9aa520, 100, np); // "<name>{s} contract"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    }
    news_text_link_person(out, &name, person, s3, s4, s5, 0x7d7);
}

/// Comparer for `write_contract_expiry_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_contract_expiry_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_CONTRACT_EXPIRY_NEWS_TEXT);
    compare_text_writer(out, || o(this, news, full, out), |t| write_contract_expiry_news_text(this, news, full, t), || format!("write_contract_expiry_news_text(news {:#x}, full {})", news, full));
}

/// Writes the headline or the body of the news that a person leaves when his contract, trial or loan ends.
///
/// `write_contract_end_news_text(news, full, out)`: the club of slot 0, the person of slot 2 (name from slots 3-5, mode
/// 1), the kind of slot 6 (5 trial, 6 loan, 1 ...), the loan club of slot 7, "own" = slot 8 is 1, and whether the club of
/// slot 0 is the current human manager's (`G_SCREEN_RECORDS_MANAGER` of the current slot, its club +0x39). Headline: own
/// and not the human's club 0x9aeff4 ("<name> leaves <club>"); trial 0x9aefbc; loan 0x9aef88; neither kind 1 nor own
/// 0x9aef50 ("contract expires"); otherwise 0x9aef20 ("leaves club"). Body: trial 0x9aeec8; loan 0x9aee54 with the loan
/// club; kind 1 0x9aedc8; own: the human's club 0x9aed5c, otherwise 0x9aeccc with the club; else 0x9aec58. The person's
/// link (0x7da) when the names still match, then (body) the loan club's link when slot 7 ≠ −1, and the club's when slot
/// 0 ≠ −1.
/// Original: `CONTRACT_MANAGER::write_contract_end_news_text` (0x4daa00, contract_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_contract_end_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let (club, person, s3, s4, s5) = (slot(0), slot(2), slot(3), slot(4), slot(5));
    let kind = slot(6) as u8;
    let loan_club = slot(7);
    let own = slot(8) == 1;
    let hm = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
    let human_club = hm != 0 && read_usize(hm + 0x39) != 0 && club == read_u32(read_usize(hm + 0x39));
    let mut name = [0u8; 0x12c];
    news_text_person_name(s3, s4, s5, &mut name, 1, person);
    let np = name.as_ptr() as usize;
    let c0 = club_at(club as i32 as isize).addr();
    if full == 0 {
        if own && !human_club {
            let g = read_i8(c0 + club::SHORT_GENDER_NAME) as i32;
            fmt(NEWS_TEXT_BUFFER, 0x9aeff4, g, g, np, c0 + club::SHORT_NAME);
        } else if kind == 5 {
            fmt(NEWS_TEXT_BUFFER, 0x9aefbc, 100, np);
        } else if kind == 6 {
            fmt(NEWS_TEXT_BUFFER, 0x9aef88, 100, np);
        } else if kind != 1 && !own {
            fmt(NEWS_TEXT_BUFFER, 0x9aef50, 100, np);
        } else {
            fmt(NEWS_TEXT_BUFFER, 0x9aef20, np);
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, person, s3, s4, s5, 0x7da);
        if club as i32 != -1 {
            news_text_link_club(out, club);
        }
        return;
    }
    if kind == 5 {
        fmt(NEWS_TEXT_BUFFER, 0x9aeec8, np);
    } else if kind == 6 {
        let c7 = club_at(loan_club as i32 as isize).addr();
        let g = read_i8(c7 + club::SHORT_GENDER_NAME) as i32;
        fmt(NEWS_TEXT_BUFFER, 0x9aee54, g, g, np, c7 + club::SHORT_NAME);
    } else if kind == 1 {
        fmt(NEWS_TEXT_BUFFER, 0x9aedc8, 100, np);
    } else if own {
        if human_club {
            fmt(NEWS_TEXT_BUFFER, 0x9aed5c, 100, np);
        } else {
            let g = read_i8(c0 + club::SHORT_GENDER_NAME) as i32;
            covered!("mke_5 contract end at the club");
            fmt(NEWS_TEXT_BUFFER, 0x9aeccc, g, g, 100, c0 + club::SHORT_NAME, np);
        }
    } else {
        fmt(NEWS_TEXT_BUFFER, 0x9aec58, 100, np); // mke5: the contract end body
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    news_text_link_person(out, &name, person, s3, s4, s5, 0x7da);
    if loan_club as i32 != -1 {
        news_text_link_club(out, loan_club);
    }
    if club as i32 != -1 {
        news_text_link_club(out, club);
    }
}

/// Comparer for `write_contract_end_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_contract_end_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_CONTRACT_END_NEWS_TEXT);
    compare_text_writer(out, || o(this, news, full, out), |t| write_contract_end_news_text(this, news, full, t), || format!("write_contract_end_news_text(news {:#x}, full {})", news, full));
}

// ---------- batch 290: the news text of an accepted offer (transfer_manager.cpp) ----------

/// Writes the headline or the body of the news that a club accepted an offer, or that a player agreed to talk about a
/// contract.
///
/// `write_offer_accepted_news_text(news, full, out)`: the person of slot 0 (name from slots 1-3; mode 1 in the body,
/// 5 in the headline), the clubs of slots 4 (−1 none) and 5, slot 6 (a talks agreement), slot 9 (1 a loan). The body:
/// a talks agreement 0xa947d8; no club in slot 4 0xa94aa4 (club 5 and the name); a loan 0xa94a14 when club 5 is the
/// current human manager's club (`G_SCREEN_RECORDS_MANAGER` of the current slot, +0x39), else 0xa94988; a transfer
/// 0xa948fc / 0xa94874 the same way; the others with the two clubs (short names, genders) and the name. The links to
/// club 4 (when ≥ 0) and club 5 (0x7d5) and to the person (0x7d7) when the names still match. The headline: "agrees to
/// contract discussion" (0xa94790) without club 4, "loan offer accepted" (0xa94754) for a loan, else "transfer offer
/// accepted" (0xa94714), and the person's link.
/// Original: `TRANSFER_MANAGER::write_offer_accepted_news_text` (0x8d7870, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_offer_accepted_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    if full != 0 {
        let hm = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
        let human_club = if hm != 0 { read_usize(hm + 0x39) } else { 0 };
        let (person, s1, s2, s3, s4, s5) = (slot(0), slot(1), slot(2), slot(3), slot(4), slot(5));
        let talks = slot(6) != 0;
        let loan = slot(9) as u8 == 1;
        news_text_person_name(s1, s2, s3, &mut name, 1, person);
        let c5 = club_at(s5 as i32 as isize).addr();
        let g5 = read_i8(c5 + club::SHORT_GENDER_NAME) as i32;
        let mine = human_club != 0 && read_u32(human_club) == s5;
        if !talks && s4 as i32 == -1 {
            fmt(NEWS_TEXT_BUFFER, 0xa94aa4, g5, g5, np, c5 + club::SHORT_NAME);
        } else {
            let t = if talks {
                0xa947d8
            } else if loan {
                if mine { 0xa94a14 } else { 0xa94988 }
            } else if mine {
                0xa948fc
            } else {
                0xa94874 // mke6: the transfer accepted body
            };
            let c4 = club_at(s4 as i32 as isize).addr();
            let g4 = read_i8(c4 + club::SHORT_GENDER_NAME) as i32;
            covered!("mke_6 offer accepted body");
            fmt(NEWS_TEXT_BUFFER, t, g4, g4, g5, g5, c4 + club::SHORT_NAME, c5 + club::SHORT_NAME, np);
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        if s4 as i32 > -1 {
            news_text_link_club(out, s4);
        }
        news_text_link_club(out, s5);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
    } else {
        let (person, s1, s2, s3, s4) = (slot(0), slot(1), slot(2), slot(3), slot(4));
        let loan = slot(9) as u8 == 1;
        news_text_person_name(s1, s2, s3, &mut name, 5, person);
        let t = if s4 as i32 == -1 {
            0xa94790 // "<name> agrees to contract discussion"
        } else if loan {
            0xa94754 // "<name> loan offer accepted"
        } else {
            0xa94714 // "<name> transfer offer accepted"
        };
        fmt(NEWS_TEXT_BUFFER, t, np);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
    }
}

/// Comparer for `write_offer_accepted_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_offer_accepted_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_OFFER_ACCEPTED_NEWS_TEXT);
    compare_text_writer(out, || o(this, news, full, out), |t| write_offer_accepted_news_text(this, news, full, t), || format!("write_offer_accepted_news_text(news {:#x}, full {})", news, full));
}

// ---------- batch 290: how well a staff member fits a job at a club (manager_manager.cpp) ----------

/// The `.rdata` doubles of `score_staff_for_job` that `rdata_consts.rs` does not name yet (value in cm0102.exe 3.9.68;
/// still read from the exe).
#[allow(dead_code)]
pub(crate) mod staff_job_score_rdata {
    pub(crate) const F64_250: usize = 0x968898; // f64 = 250.0
    pub(crate) const F64_0_0013333333333: usize = 0x96d250; // f64 = 0.0013333333333333333
    pub(crate) const F64_0_0044444444444: usize = 0x96d258; // f64 = 0.0044444444444444444
    pub(crate) const F64_0_475: usize = 0x96d270; // f64 = 0.475
}

/// The nine non-player attributes the score compares between the person and the viewer (the club's chairman), in the
/// order the original goes through them: attacking, coaching technique, directness, discipline, free roles, marking,
/// offside, pressing, youngsters.
const STAFF_JOB_SIMILARITY: [usize; 9] = [non_player::ATTACKING, non_player::COACHING_TECHNIQUE, non_player::DIRECTNESS, non_player::DISCIPLINE,
    non_player::FREE_ROLES, non_player::MARKING, non_player::OFFSIDE, non_player::PRESSING, non_player::YOUNGSTERS];

/// `v` × (`a` × k + 1), truncated (`ftol`): one attribute weight of the score (k read from the exe at `k`).
pub(crate) unsafe fn staff_job_weight(v: i32, a: i8, k: usize) -> i32 {
    ftol((a as i32 as f64 * read_f64(k) + read_f64(rdata::F64_1)) * v as i32 as f64)
}

/// The person's four attitudes (loyalty, determination, pressure, professionalism), each × ((value − 10) × k + 1),
/// truncated after each.
pub(crate) unsafe fn staff_job_attitudes(p: Staff, mut v: i32, k: usize) -> i32 {
    for a in [p.loyalty(), p.determination(), p.pressure(), p.professionalism()] {
        v = staff_job_weight(v, a.wrapping_sub(10), k);
    }
    v
}

/// The similarity with the viewer: `v` × 1.5 (truncated), then for each of `STAFF_JOB_SIMILARITY` × (1 − |difference|
/// × k) (`ks[i]`, an exe address), or, with `div` (the manager jobs), × (1 − |difference| / (F × div[i])), truncated
/// after each.
pub(crate) unsafe fn staff_job_similarity(v: i32, np: usize, viewer_np: usize, ks: &[usize; 9], div: Option<(f64, [usize; 9])>) -> i32 {
    let mut v = ftol(v as f64 * read_f64(rdata::F64_1_5));
    for (i, &a) in STAFF_JOB_SIMILARITY.iter().enumerate() {
        let d = ((read_i8(viewer_np + a) as i32) - (read_i8(np + a) as i32)).abs() as f64;
        let s = match div {
            Some((f, ds)) => read_f64(rdata::F64_1) - d / (f * read_f64(ds[i])),
            None => read_f64(rdata::F64_1) - d * read_f64(ks[i]),
        };
        v = ftol(v as f64 * s);
    }
    v
}

/// `max(v × k, v + 2000)` truncated: the bonus of a favourite (k = 1.1) or of a national team's own nation (k = 1.2).
pub(crate) unsafe fn staff_job_bonus(v: i32, k: usize) -> i32 {
    let a = v as f64 * read_f64(k);
    let b = v.wrapping_add(2000) as f64;
    ftol(if a > b { a } else { b })
}

/// How well `staff` fits job `job` at `club`, as seen by `viewer` (a staff member; by default the club's chairman):
/// −10000 for "never". Used by the manager market (`consider_sacking_manager`, `add_job_vacancy`,
/// `best_manager_candidate_score`, 0x69ca00) and by the shortlists of the staff search.
///
/// Without a non-player sheet (`.non_player()`): −10000. A viewer without a sheet is replaced by the club's chairman
/// (`.chairman()`), and without one either, no viewer. The person's role: `.job_for_club()`, else `.job_for_nation()`,
/// else 11. The score starts at the reputation as the viewer sees it (`exe::database_get_staff_reputation_for_viewer`,
/// non-player; without a viewer, a local staff record with id 0, the club's nation and the club as its club job, as the
/// original builds it). In role 12 or 5 at a club with reputation (`.reputation()`) above 4250: + current ability × 25,
/// and with world reputation above 3250 also + potential ability × 5. Already at the club (`.club_job()`): + 100 for a
/// human manager (id ≥ `G_STAFF_COUNT` − 16), otherwise + loyalty². A favourite of the club
/// (`exe::club_favours_staff`): max(× 1.1, + 2000); otherwise rejected by it (`exe::club_dislikes_staff`): × 0.25. The
/// same with the viewer (`exe::staff_is_favoured_by`, `exe::staff_is_disliked_by`).
///
/// By the job (other jobs and 11: −10000):
/// - 5 and 12 (manager jobs): a human manager skips straight to the common part. By the role: 12 × 0.95; 6 and 13 at
///   the same club or national team (`.national_job()`) × 0.75, otherwise × 0.6 and −10000 above reputation 5750; 7 ×
///   0.5 / × 0.3 and 5250; 8 × 0.6 / × 0.475 and 5250; 11 × 0.95 / × 0.75 and 5250; 15 × 0.6 / × 0.4 and 5750; 16 ×
///   0.75 and, outside the club, 5750; 9 × 0.2 and 3250; 10, 14 and the others × 0.001 and 5750; 5 nothing. Age
///   (`.age()`) above 59: × 0.875 at a national team above reputation 7000, × 0.9 above 7250, else × 0.95; below 38
///   with world reputation below 4250: × (age × 0.05 − 0.95). Below 45 at a club below reputation 7000: + min(potential
///   − current, 50)² × the viewer's patience (10 without a viewer) × 0.1. With a viewer, the similarity (difference
///   divided by F × 100, 250, 100, 150, 150, 300, 300, 200, 150, F = 1.25 − the viewer's interference × 0.025); the
///   weights coaching goalkeepers and coaching (× 0.000833), judgement, judging potential, man handling, motivating
///   (× 0.002), tactics (× 0.008 above reputation 7250, × 0.004 above 5750, else × 0.002); the attitudes × 0.005.
/// - 6, 7, 13, 14 (coaches): similarity × 0.00667, 0.00667, 0.01, 0.00444, 0.00667, 0.01, 0.01, 0.00667, 0.004;
///   weights × 0.002, 0.002, 0.00133, 0.00133, 0.000667, 0.00133, tactics × 0.002; attitudes × 0.005.
/// - 8, 15 (assistant jobs): similarity as the coaches but coaching technique × 0.01; weights × 0.005, 0.005, 0.00133,
///   0.00133, 0.00133, 0.000833, tactics × 0.004; attitudes × 0.02.
/// - 9 (scout): similarity × 0.002 each; judgement and judging potential × 0.005; attitudes × 0.002.
/// - 10 (physio): physiotherapy × 0.01; attitudes × 0.02.
///
/// The common part. A club at the person's nation, at the nation of the club job or of the national job (the first one
/// whose nation is the club's), else the person's nation. A foreigner (`exe::database_is_staff_foreign`, groups) at a
/// club above reputation 4750 whose club job is not in the club's country, coming from a weaker league
/// (`.league_standard()` of his nation or of his club's nation, the larger, below the club nation's) and not from a
/// club with reputation 6750 or more: −10000 outside roles 12 and 5, there × max(1 + difference × 0.025, 0.75).
/// National team (`exe::database_is_national_team`): its own nation max(× 1.2, + 2000); not foreign × (0.75 −
/// reputation × 0.000025); foreign × 0.35; above reputation 6250 × (world reputation × 0.000025 + 0.5) outside 12/5,
/// foreign × (× 0.00005 + 0.5), otherwise × (× 0.000025 + 0.75). A human manager there: by the reputation the viewer
/// sees, below 4250 the score is 0, below 6750 × 0.75; below 7250 and before 23/6/2003 × 0.75. Club above reputation
/// 7250 with a division and a nation of league standard above 10: + (the club job's reputation, or the world
/// reputation, − the club's) × 0.15 (truncated to a word); above 8250 with world reputation below 4250: −10000 for a
/// foreigner, someone without a club job or division, or from a division below the club's (`.reputation()` of
/// `.division()`); then, outside job 9, the reputation factor as at the national team. Other clubs: a foreigner outside
/// job 9 × (world reputation × 0.00005 + 0.5). Then × (language fluency × 0.05 + 0.5) (`exe::playerstats_get_language_fluency`
/// in the club nation's `.first_language()`, by contract). Last, the chosen nation's `.state_of_development()`: 1 as
/// is; 2 × 0.75 when the club's nation is 1; 3 × 0.25 when the club's nation is 1 or 2; other × 0.05; no nation × 0.25.
/// Defect of the original: without any of those nations the routine reads the development of a stack value (the
/// person's professionalism − 10, or garbage) as a nation; every staff member has a nation, so the copy takes "no
/// nation" there.
/// Original: `score_staff_for_job` (0x69d1b0, manager_manager.cpp, stdcall, ret 0x10; ecx unused).
pub(crate) unsafe extern "thiscall" fn score_staff_for_job(_this: usize, staff: usize, club: usize, viewer: usize, job: u32) -> i32 {
    use staff_job_score_rdata::*;
    const NEVER: i32 = -10000;
    let p = Staff(staff);
    let club = Club(club);
    let np = p.non_player().addr();
    if np == 0 {
        return NEVER;
    }
    let mut viewer = Staff(viewer);
    if viewer.is_null() || viewer.non_player().is_null() {
        viewer = club.chairman();
        if viewer.is_null() || viewer.non_player().is_null() {
            viewer = Staff(0);
        }
    }
    let age = p.age() as i8;
    let mut role = p.job_for_club();
    if role == 0 {
        role = p.job_for_nation();
        if role == 0 {
            role = 11;
        }
    }
    let manager_role = role == 12 || role == 5;
    let human = read_i32(p.addr()) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10);
    let mut v = if !viewer.is_null() {
        exe::database_get_staff_reputation_for_viewer(p, viewer, 1) as i32
    } else {
        // the local staff record of the original: the dates constructed, id 0, the club's nation, the club as club job
        let mut local = [0u8; staff::SIZE];
        let l = local.as_mut_ptr() as usize;
        for d in [staff::DATE_OF_BIRTH, staff::DATE_JOINED_NATION, 0x31, staff::DATE_JOINED_CLUB, staff::DATE_EXPIRES_CLUB] {
            exe::date_date_54a110(l + d);
        }
        write_u32(l + staff::NATION, club.nation().addr() as u32);
        write_u32(l + staff::CLUB_JOB, club.addr() as u32);
        exe::database_get_staff_reputation_for_viewer(p, l, 1) as i32
    };
    let npr = NonPlayer(np);
    if manager_role && club.reputation() > 0x109a {
        v += npr.current_ability() as i32 * 25;
        if npr.world_reputation() > 0xcb2 {
            v += read_i16(np + non_player::POTENTIAL_ABILITY) as i32 * 5;
        }
    }
    if p.club_job() == club {
        if human {
            v += 100;
        } else {
            let l = p.loyalty() as i32;
            v += l * l;
        }
    }
    if exe::club_favours_staff(p, club) != 0 {
        v = staff_job_bonus(v, rdata::F64_1_1);
    } else if exe::club_dislikes_staff(p, club) != 0 {
        v = ftol(v as f64 * read_f64(rdata::F64_0_25));
    }
    if exe::staff_is_favoured_by(p, viewer) != 0 {
        v = staff_job_bonus(v, rdata::F64_1_1);
    } else if exe::staff_is_disliked_by(p, viewer) != 0 {
        v = ftol(v as f64 * read_f64(rdata::F64_0_25));
    }
    let mul = |v: i32, k: usize| ftol(v as f64 * read_f64(k));
    let rep = club.reputation();
    let at_club = p.club_job() == club || p.national_job() == club;
    let vnp = if viewer.is_null() { 0 } else { viewer.non_player().addr() };
    use non_player as n;
    let job = job as u8;
    match job {
        5 | 12 => {
            if !human {
                covered!("sma_1 manager job");
                // the role: (factor at the club or national team, factor elsewhere, reputation limit elsewhere)
                let (here, away, limit): (usize, usize, i16) = match role {
                    5 => (0, 0, 0),
                    6 | 13 => (rdata::F64_0_75, rdata::F64_0_6, 0x1676),
                    7 => (rdata::F64_0_5, rdata::F64_0_3, 0x1482),
                    8 => (rdata::F64_0_6, F64_0_475, 0x1482),
                    11 => (rdata::F64_0_95, rdata::F64_0_75, 0x1482),
                    15 => (rdata::F64_0_6, rdata::F64_0_4, 0x1676),
                    12 => (rdata::F64_0_95, rdata::F64_0_95, i16::MAX),
                    16 => (rdata::F64_0_75, rdata::F64_0_75, 0x1676),
                    9 => (rdata::F64_0_2, rdata::F64_0_2, 0xcb2), // also at the club
                    _ => (rdata::F64_0_001, rdata::F64_0_001, 0x1676), // 10, 14 and the others, also at the club
                };
                if role != 5 {
                    let checked_here = matches!(role, 9 | 10 | 14) || !(5..=16).contains(&role);
                    if at_club && !checked_here {
                        v = mul(v, here);
                    } else {
                        v = mul(v, away);
                        if rep > limit {
                            return NEVER;
                        }
                    }
                }
                if age > 0x3b {
                    v = if exe::database_is_national_team(club) != 0 && rep > 0x1b58 {
                        mul(v, rdata::F64_0_875)
                    } else if rep > 0x1c52 {
                        mul(v, rdata::F64_0_9)
                    } else {
                        mul(v, rdata::F64_0_95)
                    };
                } else if age < 0x26 && npr.world_reputation() < 0x109a {
                    v = ftol((age as i32 as f64 * read_f64(rdata::F64_0_05) - read_f64(rdata::F64_0_95)) * v as f64);
                }
                if age < 0x2d && rep < 0x1b58 {
                    covered!("sma_1 young manager potential");
                    let gap = (read_i16(np + n::POTENTIAL_ABILITY) as i32 - npr.current_ability() as i32).min(50);
                    let patience = if vnp != 0 { read_i8(vnp + n::PATIENCE) as i32 } else { 10 };
                    v += ftol(game_pow(gap as f64, read_f64(rdata::F64_2)) * patience as f64 * read_f64(rdata::F64_0_1)); // sma1: the potential bonus
                }
                if vnp != 0 {
                    let f = read_f64(rdata::F64_1_25) - read_i8(vnp + n::INTERFERENCE) as i32 as f64 * read_f64(rdata::F64_0_025);
                    let ds = [rdata::F64_100, F64_250, rdata::F64_100, rdata::F64_150, rdata::F64_150, rdata::F64_300, rdata::F64_300, rdata::F64_200, rdata::F64_150];
                    v = staff_job_similarity(v, np, vnp, &ds, Some((f, ds)));
                }
                for (a, k) in [(n::COACHING_GKS, rdata::F64_0_0008333333333), (n::COACHING, rdata::F64_0_0008333333333), (n::JUDGEMENT, rdata::F64_0_002),
                    (n::JUDGING_POTENTIAL, rdata::F64_0_002), (n::MAN_HANDLING, rdata::F64_0_002), (n::MOTIVATING, rdata::F64_0_002)] {
                    v = staff_job_weight(v, read_i8(np + a), k);
                }
                let k = if rep > 0x1c52 { rdata::F64_0_008 } else if rep > 0x1676 { rdata::F64_0_004 } else { rdata::F64_0_002 };
                v = staff_job_weight(v, read_i8(np + n::TACTICS), k); // sma1: the manager's tactics weight
                v = staff_job_attitudes(p, v, rdata::F64_0_005);
            }
        }
        6 | 7 | 13 | 14 | 8 | 15 => {
            let assistant = job == 8 || job == 15;
            if vnp != 0 {
                let t = if assistant { rdata::F64_0_01 } else { rdata::F64_0_006666666667 };
                let ks = [rdata::F64_0_006666666667, t, rdata::F64_0_01, F64_0_0044444444444, rdata::F64_0_006666666667, rdata::F64_0_01, rdata::F64_0_01,
                    rdata::F64_0_006666666667, rdata::F64_0_004];
                v = staff_job_similarity(v, np, vnp, &ks, None);
            }
            let ws: [(usize, usize); 7] = if assistant {
                [(n::COACHING_GKS, rdata::F64_0_005), (n::COACHING, rdata::F64_0_005), (n::JUDGEMENT, F64_0_0013333333333), (n::JUDGING_POTENTIAL, F64_0_0013333333333),
                    (n::MAN_HANDLING, F64_0_0013333333333), (n::MOTIVATING, rdata::F64_0_0008333333333), (n::TACTICS, rdata::F64_0_004)]
            } else {
                [(n::COACHING_GKS, rdata::F64_0_002), (n::COACHING, rdata::F64_0_002), (n::JUDGEMENT, F64_0_0013333333333), (n::JUDGING_POTENTIAL, F64_0_0013333333333),
                    (n::MAN_HANDLING, rdata::F64_0_0006666666667), (n::MOTIVATING, F64_0_0013333333333), (n::TACTICS, rdata::F64_0_002)]
            };
            for (a, k) in ws {
                v = staff_job_weight(v, read_i8(np + a), k);
            }
            v = staff_job_attitudes(p, v, if assistant { rdata::F64_0_02 } else { rdata::F64_0_005 }); // sma1: the coaches' attitudes
        }
        9 => {
            if vnp != 0 {
                v = staff_job_similarity(v, np, vnp, &[rdata::F64_0_002; 9], None);
            }
            v = staff_job_weight(v, read_i8(np + n::JUDGEMENT), rdata::F64_0_005);
            v = staff_job_weight(v, read_i8(np + n::JUDGING_POTENTIAL), rdata::F64_0_005);
            v = staff_job_attitudes(p, v, rdata::F64_0_002);
        }
        10 => {
            v = staff_job_weight(v, read_i8(np + n::PHYSIOTHERAPY), rdata::F64_0_01);
            v = staff_job_attitudes(p, v, rdata::F64_0_02);
        }
        _ => return NEVER,
    }
    // the nation that sets the development factor at the end
    let cn = club.nation();
    let cj = p.club_job();
    let nj = p.national_job();
    let nation = if !cj.is_null() && cj.nation() == cn {
        cn.addr()
    } else if !nj.is_null() && nj.nation() == cn {
        cn.addr()
    } else {
        p.nation().addr()
    };
    // a foreigner from a weaker league
    if rep > 0x128e && !cn.is_null() && !p.nation().is_null() && exe::database_is_staff_foreign(p, cn, 1) != 0 && (cj.is_null() || cj.nation() != cn) {
        let mut own = p.nation().league_standard() as i8;
        let mut weaker = true;
        if !cj.is_null() {
            let jn = cj.nation();
            if !jn.is_null() && jn.league_standard() as i8 > own {
                own = jn.league_standard() as i8;
            }
            weaker = cj.reputation() < 0x1a5e;
        }
        let theirs = cn.league_standard() as i8;
        if weaker && own < theirs {
            covered!("sma_1 foreigner from a weaker league");
            let mut f = (own as i32 - theirs as i32) as f64 * read_f64(rdata::F64_0_025) + read_f64(rdata::F64_1);
            if f < read_f64(rdata::F64_0_75) {
                f = read_f32(rdata::F32_0_75) as f64;
            }
            if !manager_role {
                return NEVER;
            }
            v = ftol(v as f64 * f);
        }
    }
    let foreign = || exe::database_is_staff_foreign(p, cn, 1) != 0;
    // the reputation factor: (world reputation × k1 + k2) by the role and the nationality
    let rep_factor = |v: i32| {
        let w = npr.world_reputation() as i32 as f64;
        let f = if !manager_role {
            w * read_f64(rdata::F64_2_5ENEG_05) + read_f64(rdata::F64_0_5)
        } else if foreign() {
            w * read_f64(rdata::F64_5ENEG_05) + read_f64(rdata::F64_0_5)
        } else {
            w * read_f64(rdata::F64_2_5ENEG_05) + read_f64(rdata::F64_0_75)
        };
        ftol(f * v as f64)
    };
    if exe::database_is_national_team(club) != 0 {
        covered!("sma_1 national team");
        if p.nation() == cn {
            v = staff_job_bonus(v, rdata::F64_1_2); // sma1: the national team's own nation
        } else if !foreign() {
            v = ftol((read_f64(rdata::F64_0_75) - rep as i32 as f64 * read_f64(rdata::F64_2_5ENEG_05)) * v as f64);
        } else {
            v = mul(v, rdata::F64_0_35);
        }
        if rep > 0x186a {
            v = rep_factor(v);
        }
        if human {
            let r = exe::database_get_staff_reputation_for_viewer(p, viewer, 1);
            if r < 0x109a {
                v = 0;
            } else if r < 0x1a5e {
                v = mul(v, rdata::F64_0_75);
            }
            if r < 0x1c52 {
                let mut d = [0u16; 4];
                exe::date_date_549f70(&mut d, 0x17, 6, 0x7d3, u32::MAX);
                let today = read_u16(G_CURRENT_DATE_YEAR);
                if d[1] > today || (d[1] == today && d[0] as i16 > read_i16(G_CURRENT_DATE)) {
                    v = mul(v, rdata::F64_0_75);
                }
            }
        }
    } else if rep > 0x1c52 && !club.division().is_null() && !cn.is_null() && cn.league_standard() as i8 > 10 {
        covered!("sma_1 big club");
        let d = if !cj.is_null() { cj.reputation() as i32 - rep as i32 } else { npr.world_reputation() as i32 - rep as i32 };
        v += ftol(d as f64 * read_f64(rdata::F64_0_15)) as i16 as i32;
        if rep > 0x203a && npr.world_reputation() < 0x109a {
            if foreign() || cj.is_null() || cj.division().is_null() || cj.division().reputation() < club.division().reputation() {
                return NEVER;
            }
        }
        if job != 9 {
            v = rep_factor(v);
        }
    } else if foreign() && job != 9 {
        v = ftol((npr.world_reputation() as i32 as f64 * read_f64(rdata::F64_5ENEG_05) + read_f64(rdata::F64_0_5)) * v as f64);
    }
    if !cn.is_null() {
        let fl = exe::playerstats_get_language_fluency(read_usize(G_PLAYER_STATS), p, club, cn.first_language() as u8 as u32, 1) as i8;
        v = ftol((fl as i32 as f64 * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_0_5)) * v as f64); // sma1: the language factor
    }
    if nation == 0 {
        return mul(v, rdata::F64_0_25);
    }
    let club_dev = || if cn.is_null() { None } else { Some(cn.state_of_development() as i8) };
    match Nation(nation).state_of_development() as i8 {
        1 => v,
        2 => if club_dev() == Some(1) { mul(v, rdata::F64_0_75) } else { v },
        3 => if matches!(club_dev(), Some(1) | Some(2)) { mul(v, rdata::F64_0_25) } else { v },
        _ => mul(v, rdata::F64_0_05),
    }
}

compare_value!(compare_score_staff_for_job, score_staff_for_job, IDX_SCORE_STAFF_FOR_JOB, (staff: usize, club: usize, viewer: usize, job: u32) -> i32);

// ---------- batch 290: appointing a manager to a club (manager_manager.cpp) ----------

/// Exe routines `appoint_manager_to_club` calls that are already Rust but have no `exe::` wrapper (called through the
/// exe, so the comparison mode runs their comparers).
pub(crate) mod appoint_fns {
    pub(crate) const POST_MANAGER_APPOINTMENT_NEWS: usize = 0x6a2e00; // MANAGER_MANAGER::post_manager_appointment_news
    pub(crate) const RESET_BOARD_FOR_NEW_MANAGER: usize = 0x694c80; // MANAGER_MANAGER::reset_board_for_new_manager
    pub(crate) const MANAGER_MANAGER_CPP: usize = 0x9f18a4; // "E:\dev\CM3\cm3 00-01\cm3\code\manager_manager.cpp"
}

/// The bits of the contract's grievance flags (`+0x45`) the new manager clears for each player of the squad, in the
/// original's order.
const NEW_MANAGER_CLEARED_GRIEVANCES: [u32; 7] = [8, 0x200, 0x400, 0x2000, 0x10000, 0x80000, 0x800000];

/// Takes the person out of the old job before the move: the player data from the old club's squad
/// (`exe::manager_manager_remove_player_from_squad`) and the staff data from its staff
/// (`exe::manager_manager_remove_staff_from_club`), by the old role, and returns the code of the appointment news
/// (1 player, 2 staff, 3 manager). Without the data the role needs, the game's error box with the line of the
/// original (`show_error_at`). `lines` are the error lines of the club or of the national team version.
pub(crate) unsafe fn appoint_leave_old_job(this: usize, p: Staff, old: Club, role: u8, national: bool) -> u32 {
    use appoint_fns::MANAGER_MANAGER_CPP as FILE;
    let staff_out = |line: u32| {
        if !p.non_player().is_null() {
            exe::manager_manager_remove_staff_from_club(this, p, old);
        } else {
            show_error_at(FILE, line);
        }
    };
    let player_out = |line: u32| {
        if !p.player().is_null() {
            exe::manager_manager_remove_player_from_squad(this, p, old);
        } else {
            show_error_at(FILE, line);
        }
    };
    let l = |club: u32, nation: u32| if national { nation } else { club };
    match role {
        5 => {
            staff_out(l(0x34c2, 0x3526));
            3
        }
        6 | 8 => {
            staff_out(l(0x34b9, 0x351d));
            2
        }
        11 => {
            player_out(l(0x34a0, 0x3504));
            1
        }
        12 => {
            player_out(l(0x34a9, 0x350d));
            staff_out(l(0x34af, 0x3513));
            if national { 2 } else { 3 } // the player-manager of a club is news 3, of a national team 2
        }
        15 => {
            player_out(l(0x3491, 0x34f5));
            staff_out(l(0x3497, 0x34fb));
            2
        }
        _ => {
            if !p.non_player().is_null() {
                exe::manager_manager_remove_staff_from_club(this, p, old);
            }
            if !p.player().is_null() {
                exe::manager_manager_remove_player_from_squad(this, p, old);
            }
            2
        }
    }
}

/// Moves `manager` to `club` as its manager: vacancies at the old job, the appointment news, the new job, the squad
/// slot of a player-manager, the board reset and the grievances of the squad.
///
/// `MANAGER_MANAGER::appoint_manager_to_club(club, manager)`. The old job: at a national team, the person's national
/// job (`.national_job()`) when he manages one (`.job_for_nation()` 5 or 12); at a club, his club job (`.club_job()`).
/// With an old job: leaving a club he managed (`.job_for_club()` 5 or 12) for a club, the old club gets a vacancy
/// (`exe::manager_manager_add_job_vacancy`) when its nation's league is selected (`nation::LEAGUE_SELECTED`) and its
/// division (`.division()`) has a competition object (`G_COMP_OBJECT_TABLE`); leaving a national team he managed for
/// a national team, when the old one has reputation (`.reputation()`) above 3500 and its nation no continent
/// (`.continent()`); moving to a club, a player-manager or role 15 loses his playing contract
/// (`exe::contract_manager_remove_contract`). The board's `board_record::LAST_REVIEW_DATE`-like date at +0x20 of the
/// club's board row becomes 23/7/1971 (`exe::date_date_549f70`, `exe::date_operator_3d`); without a non-player sheet,
/// the error box (line 0x3480). The old job is left by role (`appoint_leave_old_job`) and the appointment news posted
/// (`post_manager_appointment_news`, code 1..3; without an old job of that kind, with no old club and code 2); at a
/// national team `.job_for_nation()` becomes 5. A human manager: the automatic replies (`exe::news_auto_respond_all`)
/// and, when the club's nation is selected (bit 1 of `nation::LEAGUE_SELECTED`), the old club's nation stops being
/// "the human's" (bit 2 cleared, bit 1 set) unless another human manages in a division of that nation, and the new
/// nation gets bit 2 instead of bit 1. The joining date (`staff::DATE_JOINED_NATION` or `staff::DATE_JOINED_CLUB`)
/// becomes today, the club's `.manager()` the person; at a national team his `.national_job()` the club and job 5; at
/// a club his `.club_job()` the club and job 5, or 12 (player-manager, in the first free of the 50 `.squad()` slots)
/// when his player reputation (`.current_reputation()`) + 750 is above the club's. A human: the scout lists
/// (`exe::scout_manager_sync_scout_lists`) and the training schedules (`exe::training_manager_reassign_squad_schedules`).
/// Then the board reset (`reset_board_for_new_manager`) and, at a club, each squad player's contract
/// (`exe::contract_manager_get_contract`) loses the grievance bits of `NEW_MANAGER_CLEARED_GRIEVANCES`
/// (`exe::staff_contract_clear_grievances`).
/// Original: `MANAGER_MANAGER::appoint_manager_to_club` (0x69ebc0, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn appoint_manager_to_club(this: usize, club: usize, manager: usize) {
    let club = Club(club);
    let p = Staff(manager);
    let national = exe::database_is_national_team(club) != 0;
    let manages = |role: u8| role == 5 || role == 12;
    let old = if national && !p.national_job().is_null() && manages(p.job_for_nation()) {
        p.national_job()
    } else if !national && !p.club_job().is_null() {
        p.club_job()
    } else {
        Club(0)
    };
    if !old.is_null() {
        if !national && manages(p.job_for_club()) && old == p.club_job() {
            let n = old.nation();
            let d = old.division();
            if !n.is_null() && n.league_selected() != 0 && !d.is_null() && read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(d.addr()) as usize * 4) != 0 {
                covered!("sma_2 vacancy at the old club");
                exe::manager_manager_add_job_vacancy(this, old, p); // sma2: the old club's vacancy
            }
        } else if national && manages(p.job_for_nation()) && old == p.national_job() && old.reputation() > 0xdac && old.nation().continent() == 0 {
            exe::manager_manager_add_job_vacancy(this, old, p);
        }
        if !national && (p.job_for_club() == 15 || p.job_for_club() == 12) {
            exe::contract_manager_remove_contract(read_usize(G_CONTRACT_MANAGER), p);
        }
    }
    let mut d = [0u16; 4];
    let mut tmp = [0u8; 8];
    exe::date_date_549f70(&mut d, 0x17, 7, 0x7b3, u32::MAX);
    exe::date_operator_3d(board_row(this, club) + 0x20, tmp.as_mut_ptr() as usize, d.as_ptr() as usize);
    if p.non_player().is_null() {
        show_error_at(appoint_fns::MANAGER_MANAGER_CPP, 0x3480);
    }
    let news: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(appoint_fns::POST_MANAGER_APPOINTMENT_NEWS);
    if !national {
        if old.is_null() || old != p.club_job() {
            news(this, p.addr(), 0, club.addr(), 2);
        } else {
            let code = appoint_leave_old_job(this, p, old, p.job_for_club(), false);
            news(this, p.addr(), old.addr(), club.addr(), code);
        }
    } else {
        if old.is_null() || old != p.national_job() {
            news(this, p.addr(), 0, club.addr(), 2);
        } else {
            let code = appoint_leave_old_job(this, p, old, p.job_for_nation(), true);
            news(this, p.addr(), old.addr(), club.addr(), code);
        }
        if p.job_for_nation() != 5 {
            p.set_job_for_nation(5);
        }
    }
    let human = read_i32(p.addr()) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10);
    if human {
        covered!("sma_2 human manager");
        exe::news_auto_respond_all(read_usize(G_NEWS), p, old);
        let cn = club.nation();
        if !cn.is_null() && cn.league_selected() & 1 != 0 {
            if !old.is_null() {
                let on = old.nation();
                if !on.is_null() {
                    let count = read_i32(G_STAFF_COUNT);
                    let other = (count - 0x10..count).any(|i| {
                        let s = Staff(read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE)));
                        let c = s.club_job();
                        s.classification() != 0 && s != p && !c.is_null() && !c.division().is_null() && c.division().nation() == on
                    });
                    if !other {
                        on.set_league_selected(on.league_selected() & 0xfd);
                        old.nation().set_league_selected(old.nation().league_selected() | 1);
                    }
                }
            }
            cn.set_league_selected(cn.league_selected() & 0xfe);
            cn.set_league_selected(cn.league_selected() | 2); // sma2: the human's league mark
        }
    }
    let joined = if national { p.addr() + staff::DATE_JOINED_NATION } else { p.addr() + staff::DATE_JOINED_CLUB };
    exe::date_operator_3d(joined, tmp.as_mut_ptr() as usize, G_CURRENT_DATE);
    club.set_manager(p);
    if !national {
        p.set_club_job(club);
        let mut job = 5;
        let pl = p.player();
        if !pl.is_null() && (club.reputation() as i32) < pl.current_reputation() as i32 + 0x2ee {
            if let Some(i) = (0..50).find(|&i| club.squad(i) == 0) {
                covered!("sma_2 player-manager");
                club.set_squad(i, p.addr());
                job = 0xc; // sma2: the player-manager
            }
        }
        p.set_job_for_club(job);
    } else {
        p.set_national_job(club);
        p.set_job_for_nation(5);
    }
    if human {
        exe::scout_manager_sync_scout_lists(read_usize(G_SCOUT_MANAGER), p);
        exe::training_manager_reassign_squad_schedules(read_usize(G_TRAINING_MANAGER), p, old, club.addr() as u32); // sma2: the human's training
    }
    let reset: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(appoint_fns::RESET_BOARD_FOR_NEW_MANAGER);
    reset(this, club.addr()); // sma2: the new board
    if !national {
        for i in 0..50 {
            let s = Staff(club.squad(i));
            if s.is_null() || s.player().is_null() {
                continue;
            }
            let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), s);
            if c == 0 {
                continue;
            }
            for bit in NEW_MANAGER_CLEARED_GRIEVANCES {
                if read_u32(c + 0x45) & bit != 0 {
                    exe::staff_contract_clear_grievances(c, bit); // sma2: the grievances the new manager clears
                }
            }
        }
    }
}

/// Comparer for `appoint_manager_to_club`.
///
/// The move cannot run twice (news, vacancies, contracts): only the original runs, and the outcome it guarantees is
/// checked: the club's `.manager()` is the person, with his job at the club or national team set (5, or 12 for a
/// player-manager in the club's squad).
pub(crate) unsafe extern "thiscall" fn compare_appoint_manager_to_club(this: usize, club: usize, manager: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_APPOINT_MANAGER_TO_CLUB)(this, club, manager);
    let c = Club(club);
    let p = Staff(manager);
    let ok = if exe::database_is_national_team(c) != 0 {
        p.national_job() == c && p.job_for_nation() == 5
    } else {
        p.club_job() == c && (p.job_for_club() == 5 || (p.job_for_club() == 12 && (0..50).any(|i| c.squad(i) == manager)))
    };
    SCOREBOARD.record(c.manager() == p && ok, || format!("appoint_manager_to_club(club {:x}, manager {:x}): manager {:x}, club job {:x}/{}, national job {:x}/{}",
        club, manager, c.manager().addr(), p.club_job().addr(), p.job_for_club(), p.national_job().addr(), p.job_for_nation()));
}

// ---------- batch 290: club records: the season snapshot and the record news fields (club_records.cpp) ----------

/// Exe addresses of the record news routines without an `exe::` wrapper.
pub(crate) mod record_news_fns {
    pub(crate) const WRITE_RUN_RECORD_TO_NEWS: usize = 0x7e5420; // writes a run record (+0x14 bytes) into a news item, cdecl
    pub(crate) const DATE_CONSTRUCT_645800: usize = 0x645800; // a match record's constructor: only its Date (0x54a110)
    pub(crate) const CLUB_RECORDS_CPP: usize = 0x98b6a0; // "E:\dev\CM3\cm3 00-01\cm3\code\club_records.cpp"
}

/// Constructs a club's season history snapshot (0x389 bytes): runs the Date constructors of its records in the
/// original's order: the dates at +0x12 and +0x35 (`exe::date_date_54a110`); the match records at +0x58, +0x7d, +0x9c,
/// +0xbb, +0xda and +0xf9 (0x645800, which only constructs their Date); the run records at +0x118, +0x12c, +0x140
/// and +0x154 (`run_record_construct`); the record holders at +0x169, +0x184 and +0x19f (`record_holder_construct`),
/// the match record at +0x1b7; the holders at +0x1d7, +0x1f5, +0x211, +0x22d, +0x249 and +0x263; two blocks of a Date,
/// a holder at +8 and a match record at +0x20 (+0x27b and +0x2ba); the holder at +0x2fb; the transfer fee records at
/// +0x313 (`transfer_fee_record_construct`) and +0x33c (`transfer_fee_record_construct_2`); the holder at +0x371.
/// Returns `this`.
/// Original: `season_history_entry_construct` (0x448260, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn season_history_entry_construct(this: usize) -> usize {
    let mtch: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(record_news_fns::DATE_CONSTRUCT_645800);
    exe::date_date_54a110(this + 0x12);
    exe::date_date_54a110(this + 0x35);
    for o in [0x58, 0x7d, 0x9c, 0xbb, 0xda, 0xf9] {
        mtch(this + o);
    }
    for o in [0x118, 0x12c, 0x140, 0x154] {
        run_record_construct(this + o);
    }
    for o in [0x169, 0x184, 0x19f] {
        record_holder_construct(this + o);
    }
    mtch(this + 0x1b7);
    for o in [0x1d7, 0x1f5, 0x211, 0x22d, 0x249, 0x263] {
        record_holder_construct(this + o);
    }
    for b in [0x27b, 0x2ba] {
        covered!("sma_3 season snapshot");
        exe::date_date_54a110(this + b);
        record_holder_construct(this + b + 8); // sma3: the holder of the block
        mtch(this + b + 0x20);
    }
    record_holder_construct(this + 0x2fb);
    transfer_fee_record_construct(this + 0x313);
    transfer_fee_record_construct_2(this + 0x33c);
    record_holder_construct(this + 0x371);
    this
}

/// Comparer for `season_history_entry_construct`: side by side over the 0x389 bytes of the snapshot, and the return.
pub(crate) unsafe extern "thiscall" fn compare_season_history_entry_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_SEASON_HISTORY_ENTRY_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x389)], || o(this), || season_history_entry_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("season_history_entry_construct {:x?} x {:x?}", mo, mr));
    r
}

/// One field of a broken record as the news item gets it.
#[derive(Clone, Copy)]
pub(crate) enum RecordNewsField {
    /// a byte, zero-extended
    Byte(usize),
    /// a signed byte
    SByte(usize),
    /// a word, zero-extended
    Word(usize),
    /// a signed word
    SWord(usize),
    /// a dword
    Dword(usize),
    /// a float × 100, truncated (`ftol`)
    Percent(usize),
    /// a match record (`exe::write_match_record_to_news`)
    Match(usize),
    /// a record holder's name reference (`exe::write_staff_name_ref_to_news`)
    Holder(usize),
    /// a run record (0x7e5420)
    Run(usize),
}

/// The fields of each record type (1..=32) in the season record, in the original's order.
pub(crate) fn record_news_fields(kind: u8) -> &'static [RecordNewsField] {
    use RecordNewsField::*;
    match kind {
        1 => &[Byte(0), Dword(1), Word(5)],
        2 => &[Byte(7), Dword(8), Word(0xc)],
        3 => &[Dword(0xe), Match(0x12)],
        4 => &[Dword(0x31), Match(0x35)],
        5 => &[Dword(0x54), Match(0x58)],
        6 => &[Dword(0x77), Word(0x7b)],
        7 => &[Match(0x7d)],
        8 => &[Match(0x9c)],
        9 => &[Match(0xbb)],
        10 => &[Match(0xda)],
        11 => &[Match(0xf9)],
        12 => &[Run(0x118)],
        13 => &[Run(0x12c)],
        14 => &[Run(0x140)],
        15 => &[Run(0x154)],
        16 => &[SByte(0x168), Holder(0x169), Word(0x181)],
        17 => &[SByte(0x183), Holder(0x184), Word(0x19c)],
        18 => &[SByte(0x19e), Holder(0x19f), Match(0x1b7)],
        19 => &[SByte(0x1d6), Holder(0x1d7), Word(0x1ef)],
        20 => &[Percent(0x1f1), Holder(0x1f5), SByte(0x20d), Word(0x20e)],
        21 => &[SByte(0x210), Holder(0x211), Word(0x229)],
        22 => &[SByte(0x22b), SByte(0x22c), Holder(0x22d), Word(0x245)],
        23 => &[SWord(0x247), Holder(0x249)],
        24 => &[SWord(0x261), Holder(0x263)],
        25 => &[SWord(0x27b), Word(0x27d), Holder(0x283), Match(0x29b)],
        26 => &[SWord(0x2ba), Word(0x2bc), Holder(0x2c2), Match(0x2da)],
        27 => &[SWord(0x2f9), Holder(0x2fb)],
        28 => &[Dword(0x313), Holder(0x317), Dword(0x32f), SByte(0x33b)],
        29 => &[Dword(0x33c), Holder(0x340), Dword(0x358), SByte(0x364)],
        30 => &[Dword(0x365)],
        31 => &[Dword(0x36b)],
        32 => &[Holder(0x371)],
        _ => &[],
    }
}

/// Writes the club id, the record type and the fields of one broken club record into the slots of a news item.
///
/// `club_records_write_record_news_fields(club_id, record, kind, news)`: without the record (error line 0x1829) or
/// the news item (0x1831), the game's error box and 0. Slot 0 the club id, slot 1 the type (`kind`, signed), then the
/// type's fields (`record_news_fields`) from slot 2 on (`exe::newsitem_set_slot`; the match, holder and run records
/// write their own slots and advance the index). A type outside 1..=32: the error box (line 0x1989) after slots 0 and
/// 1, and 0. Otherwise 1.
/// Original: `club_records_write_record_news_fields` (0x453120, club_records.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_records_write_record_news_fields(club_id: u32, record: usize, kind: u32, news: usize) -> u32 {
    use record_news_fns::CLUB_RECORDS_CPP as FILE;
    if record == 0 {
        show_error_at(FILE, 0x1829);
        return 0;
    }
    if news == 0 {
        show_error_at(FILE, 0x1831);
        return 0;
    }
    let mut k: u8 = 1;
    exe::newsitem_set_slot(news, 0, club_id);
    let kind = kind as u8 as i8;
    exe::newsitem_set_slot(news, 1, kind as i32 as u32); // sma3: the record type
    k += 1;
    let fields = record_news_fields(kind as u8);
    if fields.is_empty() {
        show_error_at(FILE, 0x1989);
        return 0;
    }
    covered!("sma_3 record news fields");
    let run: unsafe extern "C" fn(usize, usize, *mut u8) -> u32 = core::mem::transmute(record_news_fns::WRITE_RUN_RECORD_TO_NEWS);
    let slot = |v: u32, k: &mut u8| {
        exe::newsitem_set_slot(news, *k as i8, v);
        *k = k.wrapping_add(1);
    };
    for f in fields {
        let r = record;
        match *f {
            RecordNewsField::Byte(o) => slot(read_u8(r + o) as u32, &mut k),
            RecordNewsField::SByte(o) => slot(read_i8(r + o) as i32 as u32, &mut k), // sma3: a signed byte field
            RecordNewsField::Word(o) => slot(read_u16(r + o) as u32, &mut k),
            RecordNewsField::SWord(o) => slot(read_i16(r + o) as i32 as u32, &mut k),
            RecordNewsField::Dword(o) => slot(read_u32(r + o), &mut k),
            RecordNewsField::Percent(o) => slot(ftol(read_f32(r + o) as f64 * read_f32(rdata::F32_100) as f64) as u32, &mut k), // sma3: the percentage
            RecordNewsField::Match(o) => {
                exe::write_match_record_to_news(r + o, news, &mut k);
            }
            RecordNewsField::Holder(o) => exe::write_staff_name_ref_to_news(r + o, news, &mut k),
            RecordNewsField::Run(o) => {
                run(r + o, news, &mut k);
            }
        }
    }
    1
}

/// Comparer for `club_records_write_record_news_fields`: side by side over the news item (0xe0 bytes) and the return;
/// without the record or the news item, or with a type outside 1..=32, only the original (the error box).
pub(crate) unsafe extern "C" fn compare_club_records_write_record_news_fields(club_id: u32, record: usize, kind: u32, news: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, u32, usize) -> u32>(IDX_CLUB_RECORDS_WRITE_RECORD_NEWS_FIELDS);
    if record == 0 || news == 0 || record_news_fields(kind as u8).is_empty() {
        return o(club_id, record, kind, news);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(news, 0xe0)], || o(club_id, record, kind, news),
        || club_records_write_record_news_fields(club_id, record, kind, news));
    SCOREBOARD.record(igual && r == n, || format!("club_records_write_record_news_fields(kind {}) {:x?} x {:x?}", kind as u8, mo, mr));
    r
}

// ---------- batch 290: the national team tables of a new game (national_teams.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\national_teams.cpp", the file of the error box.
const NATIONAL_TEAMS_CPP: usize = 0xa5b2a4;

/// Builds the per-nation table of a new game: counts each nation's people, sizes the senior and under-21 reserve
/// pools, allocates them, builds the pools and picks the first squads.
///
/// `NationalTeams::init_tables()`. The table (`national_teams_fields::NATION_RECORDS`): `G_NATION_COUNT` records of
/// 0x6e bytes (`exe::crt_calloc`; failing, the error box at line 0xd9 and nothing else); the queued competitions
/// (`national_teams_fields::NEXT_COMPETITIONS`, `exe::crt_malloc`, one dword per nation) and the Olympic marks
/// (`national_teams_fields::OLYMPIC_NATION_MARKS`, a byte per nation). For each staff member with a nation
/// (`.nation()`) the nation's count (word +0) goes up; a national job (`.national_job()`) held in a player role
/// (`exe::database_is_player_job` of `.job_for_nation()`) is left (`exe::database_remove_staff_from_club`, flags 0, 1,
/// 0). For each nation: no current or queued competition (−1); the senior pool starts
/// (`national_squad_record::RESERVE_START`) at the running total and takes the count within 10..100 (byte +0xc), its
/// first level that many (`national_squad_record::RESERVE_LEVEL_CAPACITY`) or, with 100, 10 and the other three levels
/// (+0x12, +0x16, +0x1a) 30 each; the under-21 pool starts (+0x1d) at its own running total and takes the count within
/// 10..50 (+0x21), its first level (+0x23) the senior size or, with 50, 5 and the other three (+0x27, +0x2b, +0x2f)
/// 15 each. The two pools (`national_teams_fields::RESERVE_POOLS`, sizes at +0x14 and +0x18, a dword per slot; failing,
/// the error box at line 0x12c or 0x137). Then the pools are built (`exe::nationalteams_build_selection_pools`) and,
/// for each nation from the second one with a continent (`.continent()`), the senior and under-21 squads are picked
/// (`exe::nationalteams_select_squad`, types 0 and 2, today), the called-up marks cleared
/// (`national_squad_record::CALLED_UP_MARK`, `national_squad_record::U21_CALLED_UP_MARK`, +0x57) and the B squad's
/// largest size (+0x56) set to 24. Both pools are sized by the senior total (the under-21 one is larger than it needs;
/// the under-21 total only sets the starts).
/// Original: `NationalTeams::init_tables` (0x76a840, national_teams.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nationalteams_init_tables(this: usize) {
    use national_squad_record as r;
    let n = read_i32(G_NATION_COUNT);
    let t = exe::crt_calloc(n as u32, 0x6e);
    write_usize(this + national_teams_fields::NATION_RECORDS, t);
    if t == 0 {
        show_error_at(NATIONAL_TEAMS_CPP, 0xd9);
        return;
    }
    write_usize(this + national_teams_fields::NEXT_COMPETITIONS, exe::crt_malloc((n as u32).wrapping_mul(4)));
    write_usize(this + national_teams_fields::OLYMPIC_NATION_MARKS, exe::crt_calloc(1, read_u32(G_NATION_COUNT)));
    for i in 0..read_i32(G_STAFF_COUNT) {
        let s = Staff(read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE)));
        let nation = s.nation();
        if !nation.is_null() {
            let c = read_usize(this + national_teams_fields::NATION_RECORDS) + read_u32(nation.addr()) as usize * 0x6e;
            write_u16(c, read_u16(c).wrapping_add(1));
        }
        if !s.national_job().is_null() && exe::database_is_player_job(s.job_for_nation() as i8) != 0 {
            exe::database_remove_staff_from_club(s.national_job(), s, 0usize, 1, 0);
        }
    }
    let (mut senior, mut u21) = (0u32, 0u32);
    for i in 0..read_i32(G_NATION_COUNT) as usize {
        let rec = |o: usize| read_usize(this + national_teams_fields::NATION_RECORDS) + i * 0x6e + o;
        write_i32(rec(r::CURRENT_COMPETITION), -1);
        write_i32(read_usize(this + national_teams_fields::NEXT_COMPETITIONS) + i * 4, -1);
        write_u32(rec(r::RESERVE_START), senior);
        let count = read_i16(rec(0));
        let size = count.clamp(10, 100) as u8; // sma4: the senior pool size
        write_u8(rec(0xc), size);
        senior += size as u32;
        if size < 100 {
            write_u8(rec(r::RESERVE_LEVEL_CAPACITY), size);
        } else {
            covered!("sma_4 full senior pool");
            write_u8(rec(r::RESERVE_LEVEL_CAPACITY), 10);
            for o in [0x12, 0x16, 0x1a] {
                write_u8(rec(o), 0x1e);
            }
        }
        write_u32(rec(0x1d), u21);
        let size21 = count.clamp(10, 0x32) as u8; // sma4: the under-21 pool size
        write_u8(rec(0x21), size21);
        u21 += size21 as u32;
        if size21 < 0x32 {
            write_u8(rec(0x23), read_u8(rec(0xc)));
        } else {
            write_u8(rec(0x23), 5);
            for o in [0x27, 0x2b, 0x2f] {
                write_u8(rec(o), 0xf);
            }
        }
    }
    write_u32(this + 0x14, senior);
    let p = exe::crt_calloc(senior, 4);
    write_usize(this + national_teams_fields::RESERVE_POOLS, p);
    if p == 0 {
        show_error_at(NATIONAL_TEAMS_CPP, 0x12c);
        return;
    }
    write_u32(this + 0x18, senior);
    let p = exe::crt_calloc(senior, 4);
    write_usize(this + national_teams_fields::RESERVE_POOLS + 4, p);
    if p == 0 {
        show_error_at(NATIONAL_TEAMS_CPP, 0x137);
        return;
    }
    exe::nationalteams_build_selection_pools(this);
    for i in 1..read_i32(G_NATION_COUNT) as usize {
        let nation = Nation(read_usize(G_NATION_TABLE) + i * 0x122);
        if nation.continent() == 0 {
            continue;
        }
        covered!("sma_4 first squads");
        exe::nationalteams_select_squad(this, nation, 0, G_CURRENT_DATE, 0usize, 0);
        exe::nationalteams_select_squad(this, Nation(read_usize(G_NATION_TABLE) + i * 0x122), 2, G_CURRENT_DATE, 0usize, 0);
        let rec = read_usize(this + national_teams_fields::NATION_RECORDS) + i * 0x6e;
        write_u8(rec + r::CALLED_UP_MARK, 0);
        write_u8(rec + r::U21_CALLED_UP_MARK, 0);
        write_u8(rec + 0x56, 0x18); // sma4: the B squad's largest size
        write_u8(rec + 0x57, 0);
    }
}

/// Comparer for `nationalteams_init_tables`.
///
/// The tables are allocated once per game: only the original runs, and the sizes it leaves are checked against the
/// rule (each nation's senior pool within 10..100 and under-21 within 10..50 of its count, the starts the running
/// totals, both pool sizes the senior total).
pub(crate) unsafe extern "thiscall" fn compare_nationalteams_init_tables(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NATIONALTEAMS_INIT_TABLES)(this);
    let t = read_usize(this + national_teams_fields::NATION_RECORDS);
    if t == 0 || read_usize(this + national_teams_fields::RESERVE_POOLS + 4) == 0 {
        return;
    }
    let (mut senior, mut u21, mut ok) = (0u32, 0u32, true);
    for i in 0..read_i32(G_NATION_COUNT) as usize {
        let rec = t + i * 0x6e;
        let count = read_i16(rec);
        ok &= read_u32(rec + national_squad_record::RESERVE_START) == senior && read_u32(rec + 0x1d) == u21;
        ok &= read_u8(rec + 0xc) == count.clamp(10, 100) as u8 && read_u8(rec + 0x21) == count.clamp(10, 0x32) as u8;
        senior += read_u8(rec + 0xc) as u32;
        u21 += read_u8(rec + 0x21) as u32;
    }
    ok &= read_u32(this + 0x14) == senior && read_u32(this + 0x18) == senior;
    SCOREBOARD.record(ok, || format!("nationalteams_init_tables: totals {} / {} against {} / {}", read_u32(this + 0x14), read_u32(this + 0x18), senior, u21));
}

// ---------- batch 290: the discipline rule sets of the ruling bodies (discipline.cpp) ----------

/// When a ruling body of the table is built.
#[derive(Clone, Copy)]
pub(crate) enum RuleSetWhen {
    /// always
    Always,
    /// when the league of the nation (the `KEY_NATION_*` global) is selected (`nation::LEAGUE_SELECTED`)
    Nation(usize),
    /// when the continent (the `KEY_CONTINENT_*` global) passes 0x6699d0
    Continent(usize),
    /// always, the constructor's argument 0 for data older than the version (`G_GAME_DATA_VERSION`, absolute value)
    OlderThan(i32),
    /// when the nation is selected, and the argument 0 for data older than the version
    NationOlderThan(usize, i32),
}

/// The 43 ruling bodies of `Discipline::create_rule_sets` in the original's order: the slot (byte offset in the
/// table), when it is built and its constructor (thiscall(object, argument), each 0x23 bytes).
pub(crate) const RULE_SETS: [(usize, RuleSetWhen, usize); 43] = {
    use RuleSetWhen::*;
    [(0x00, Always, 0x7df180), (0x04, Always, 0x7dd400), (0x08, Always, 0x7e29e0),
     (0x0c, Nation(KEY_NATION_ARGENTINA), 0x7db100),
     (0x10, Nation(KEY_NATION_BELGIUM), 0x7dbca0), (0x14, Nation(KEY_NATION_BELGIUM), 0x7db8f0),
     (0x18, Nation(KEY_NATION_BRAZIL), 0x7dc080), (0x1c, Nation(KEY_NATION_BRAZIL), 0x7dc3c0),
     (0x20, Nation(KEY_NATION_DENMARK), 0x7dcae0),
     (0x24, Nation(KEY_NATION_ENGLAND), 0x7dcf90),
     (0x28, Nation(KEY_NATION_FINLAND), 0x7dd990), (0x2c, Nation(KEY_NATION_FINLAND), 0x7dd660),
     (0x30, Nation(KEY_NATION_FRANCE), 0x7ddcc0),
     (0x34, Nation(KEY_NATION_GERMANY), 0x7de620), (0x38, Nation(KEY_NATION_GERMANY), 0x7de2e0),
     (0x3c, Nation(KEY_NATION_HOLLAND), 0x7ded90),
     (0x40, Nation(KEY_NATION_REPUBLIC_OF_IRELAND), 0x7df480),
     (0x44, Nation(KEY_NATION_ITALY), 0x7dfd70), (0x48, Nation(KEY_NATION_ITALY), 0x7df960),
     (0x4c, Nation(KEY_NATION_JAPAN), 0x7e0490), (0x50, Nation(KEY_NATION_JAPAN), 0x7e01b0),
     (0x54, Nation(KEY_NATION_NORWAY), 0x7e1150), (0x58, Nation(KEY_NATION_NORWAY), 0x7e0ef0),
     (0x5c, Nation(KEY_NATION_PORTUGAL), 0x7e1a60),
     (0x60, Nation(KEY_NATION_SCOTLAND), 0x7e2470), (0x64, Nation(KEY_NATION_SCOTLAND), 0x7e21a0),
     (0x68, Nation(KEY_NATION_SPAIN), 0x7e2f80), (0x6c, Nation(KEY_NATION_SPAIN), 0x7e2c30),
     (0x70, Nation(KEY_NATION_SWEDEN), 0x7e3650), (0x74, Nation(KEY_NATION_SWEDEN), 0x7e32d0),
     (0x78, Nation(KEY_NATION_UNITED_STATES), 0x7e40f0),
     (0x7c, Nation(KEY_NATION_POLAND), 0x7e1610),
     (0x80, Nation(KEY_NATION_GREECE), 0x7de960),
     (0x84, Nation(KEY_NATION_CROATIA), 0x7dc700),
     (0x88, Nation(KEY_NATION_AUSTRALIA), 0x7db5b0),
     (0x8c, Continent(KEY_CONTINENT_OCEANIA), 0x7e13d0),
     (0x90, Nation(KEY_NATION_NORTHERN_IRELAND), 0x7e0af0),
     (0x94, Nation(KEY_NATION_TURKEY), 0x7e3d60), (0x98, Nation(KEY_NATION_TURKEY), 0x7e3a10),
     (0x9c, Nation(KEY_NATION_RUSSIA), 0x7e1e00),
     (0xa0, Nation(KEY_NATION_WALES), 0x7e43c0),
     (0xa4, OlderThan(0x3e044), 0x7db360),
     (0xa8, NationOlderThan(KEY_NATION_SOUTH_KOREA, 0x3e047), 0x7e0730)]
};

/// The continent test of the Oceania rule set (0x6699d0, cdecl(continent) -> nonzero).
const CONTINENT_IS_ACTIVE: usize = 0x6699d0;

/// Allocates the 43-entry ruling body table and constructs one 0x23-byte ruling body object per active rule set.
///
/// `Discipline::create_rule_sets(arg)`: the table (`exe::crt_calloc` of 0x2b dwords) in the discipline object's first
/// dword; failing, the error box (discipline.cpp line 0x161e) and nothing else. Then, in `RULE_SETS` order, each
/// ruling body whose condition holds: a nation's by its league being selected (the `nation::LEAGUE_SELECTED` byte of
/// the nation the key names), Oceania's by its continent (0x6699d0 on `G_CONTINENT_TABLE` + key × 0xc6), the last two
/// (the South Korean one also by the nation) with the constructor's argument 0 when the absolute data version
/// (`G_GAME_DATA_VERSION`) is below 0x3e044 / 0x3e047, otherwise `arg`; each one `exe::crt_operator_new(0x23)` and
/// its constructor (null when the allocation fails), stored in its slot.
/// Original: `Discipline::create_rule_sets` (0x55d420, discipline.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn discipline_create_rule_sets(this: usize, arg: u32) {
    let t = exe::crt_calloc(0x2b, 4);
    write_usize(this, t);
    if t == 0 {
        show_error_at(0x9b8f18, 0x161e); // "E:\dev\CM3\cm3 00-01\cm3\code\discipline.cpp"
        return;
    }
    let selected = |key: usize| read_u8(read_usize(G_NATION_TABLE) + read_u32(key) as usize * 0x122 + nation::LEAGUE_SELECTED) != 0;
    let old = |v: i32| read_i32(G_GAME_DATA_VERSION).wrapping_abs() < v;
    for (slot, when, ctor) in RULE_SETS {
        let a = match when {
            RuleSetWhen::Always => arg,
            RuleSetWhen::Nation(k) => {
                if !selected(k) {
                    continue;
                }
                arg
            }
            RuleSetWhen::Continent(k) => {
                let active: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(CONTINENT_IS_ACTIVE);
                if active(read_usize(G_CONTINENT_TABLE) + read_u32(k) as usize * 0xc6) == 0 {
                    continue;
                }
                arg
            }
            RuleSetWhen::OlderThan(v) => if old(v) { 0 } else { arg },
            RuleSetWhen::NationOlderThan(k, v) => {
                if !selected(k) {
                    continue;
                }
                if old(v) { 0 } else { arg }
            }
        };
        if slot == 0x24 {
            covered!("sma_5 english rule set");
        }
        let a = if slot == 0x24 { a } else { a }; // sma5: the English rule set's argument (the mutation's anchor)
        let o = exe::crt_operator_new(0x23);
        let v = if o != 0 {
            let f: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(ctor);
            f(o, a)
        } else {
            0
        };
        write_usize(read_usize(this) + slot, v);
    }
}

/// Comparer for `discipline_create_rule_sets`.
///
/// The ruling bodies are built once per game: only the original runs, and the table it leaves is checked: one object
/// exactly in the slots whose condition holds, none in the others.
pub(crate) unsafe extern "thiscall" fn compare_discipline_create_rule_sets(this: usize, arg: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_DISCIPLINE_CREATE_RULE_SETS)(this, arg);
    let t = read_usize(this);
    if t == 0 {
        return;
    }
    let selected = |key: usize| read_u8(read_usize(G_NATION_TABLE) + read_u32(key) as usize * 0x122 + nation::LEAGUE_SELECTED) != 0;
    let mut wrong = Vec::new();
    for (slot, when, _) in RULE_SETS {
        let built = match when {
            RuleSetWhen::Nation(k) | RuleSetWhen::NationOlderThan(k, _) => selected(k),
            RuleSetWhen::Continent(_) => continue, // the continent test is not repeated here
            _ => true,
        };
        if built != (read_usize(t + slot) != 0) {
            wrong.push(slot);
        }
    }
    SCOREBOARD.record(wrong.is_empty(), || format!("discipline_create_rule_sets: slots {:x?} against the selected leagues", wrong));
}

// ---------- batch 290: the board's season expectations and confidence texts (manager_manager.cpp) ----------

/// Exe routines of the board texts without an `exe::` wrapper (still original, or Rust hooked and called through the
/// exe so the comparison runs their comparers).
pub(crate) mod board_text_fns {
    pub(crate) const BUILD_BOARD_EXPECTATION_TEXT: usize = 0x694440; // thiscall(this, club, flags, buf, size), ret 0x10
    pub(crate) const BUILD_BOARD_CONFIDENCE_TEXT: usize = 0x690d80; // thiscall(this, club, flags, buf, size), ret 0x10
    pub(crate) const BUILD_BOARD_NAMED_PLAYERS_TEXT: usize = 0x693850; // thiscall(this, buf, club, ids) -> al count
}

/// The expectation sentence of the board, by the flags: (title flag 1, the six format pairs of the other flags).
/// Each entry is the text without 0x800 and with 0x800; the three columns are flag 0x400, neither 0x400 nor 0x100,
/// and 0x100.
const EXPECTATION_TEXTS: [(u32, [[usize; 2]; 3]); 5] = [
    (4, [[0xa041e8, 0xa04268], [0xa03f98, 0xa03ffc], [0xa040a0, 0xa04124]]),
    (8, [[0xa03e58, 0xa03edc], [0xa03c18, 0xa03c7c], [0xa03d18, 0xa03d9c]]),
    (0x40, [[0xa03ab0, 0xa03b44], [0xa03828, 0xa038a0], [0xa03958, 0xa039e8]]),
    (0x10, [[0xa036d8, 0xa03760], [0xa03480, 0xa034ec], [0xa03598, 0xa0361c]]),
    (0x20, [[0xa0331c, 0xa033ac], [0xa030a4, 0xa03118], [0xa031cc, 0xa03258]]),
];

/// Writes the board's season expectation sentence for a club from the expectation flag bits into `buf`.
///
/// `build_board_expectation_text(club, flags, buf, size)` (the size is not used; `this` neither). Each sentence is a
/// language format (`FN_LANGLIB_SPRINTF`) with the club's gender twice (`club::GENDER_NAME`) and its name
/// (`club::NAME`). Flag 1 (the title): by 0x400 / neither 0x400 nor 0x100 / 0x100 and by 0x800, with a second version
/// when the club's nation (`.nation()`) has a continent (`.continent()`) other than Europe (`KEY_CONTINENT_EUROPE`).
/// Flag 2: with
/// 0x400 or 0x100 one text by 0x800; otherwise by 0x800 and by the virtual +0x98 of the division's competition object
/// (`G_COMP_OBJECT_TABLE`, the top-division test). Then the first of 4, 8, 0x40, 0x10 set (`EXPECTATION_TEXTS`);
/// without 0x20, the "improve the squad" text 0xa03038; with it, the 0x20 texts.
/// Original: `MANAGER_MANAGER::build_board_expectation_text` (0x694440, manager_manager.cpp, thiscall, ret 0x10; ecx
/// unused).
pub(crate) unsafe extern "thiscall" fn build_board_expectation_text(_this: usize, club: usize, flags: u32, buf: usize, _size: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let c = Club(club);
    write_u8(buf, 0);
    let g = read_i8(club + club::GENDER_NAME) as i32;
    let say = |text: usize| {
        format(buf, text, g, g, club + club::NAME);
    };
    let f = |bit: u32| flags & bit != 0;
    let col = if f(0x400) { 0 } else if !f(0x100) { 1 } else { 2 };
    let late = f(0x800) as usize;
    if f(1) {
        covered!("sma_6 title expectation");
        let n = c.nation();
        let outside_europe = !n.is_null() && n.continent() != 0 && read_u32(n.continent()) != read_u32(KEY_CONTINENT_EUROPE);
        // (continental, other) by the column and 0x800
        let t: [[[usize; 2]; 2]; 3] = [[[0xa04a60, 0xa04ae8], [0xa04b80, 0xa04c4c]], [[0xa046a0, 0xa046f0], [0xa04744, 0xa047c0]],
            [[0xa04840, 0xa048a4], [0xa0490c, 0xa049b4]]];
        say(t[col][late][outside_europe as usize]); // sma6: the title sentence
        return;
    }
    if f(2) {
        if col != 1 {
            say(if late == 1 { 0xa04390 } else { 0xa0432c });
            return;
        }
        let d = c.division();
        let top = !d.is_null() && call_comp_virtual(read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(d.addr()) as usize * 4), 0x98) != 0;
        say(match (late, top) {
            (0, true) => 0xa044b0,
            (0, false) => 0xa04438,
            (_, true) => 0xa045e4,
            _ => 0xa04528,
        });
        return;
    }
    for (bit, texts) in EXPECTATION_TEXTS {
        if bit == 0x20 && !f(0x20) {
            say(0xa03038);
            return;
        }
        if f(bit) {
            say(texts[col][late]);
            return;
        }
    }
}

/// Comparer for `build_board_expectation_text`: side by side over the buffer (the size the caller gives, at most 0x1388).
pub(crate) unsafe extern "thiscall" fn compare_build_board_expectation_text(this: usize, club: usize, flags: u32, buf: usize, size: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32)>(IDX_BUILD_BOARD_EXPECTATION_TEXT);
    let n = (size as usize).clamp(1, 0x1388);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(buf, n)], || o(this, club, flags, buf, size), || build_board_expectation_text(this, club, flags, buf, size));
    SCOREBOARD.record(igual, || format!("build_board_expectation_text(flags {:#x}) {:?} x {:?}", flags,
        String::from_utf8_lossy(&mo.0[0].1[..0x80.min(n)]), String::from_utf8_lossy(&mr.0[0].1[..0x80.min(n)])));
}

/// Writes the headline or body of a board's season expectations.
///
/// `MANAGER_MANAGER::news_board_expectations_text(news, body, out)`: the news slots 0 (club id) and 1 (the flags,
/// `exe::newsitem_get_slot`). Body: the expectation sentence (`build_board_expectation_text`, through the exe).
/// Headline (`FN_LANGLIB_SPRINTF` of a fixed text): a national team (id above clubs − 2 × nations) "F.A. Announce
/// Expectations"; flag 0x400 "Board expect glory", 2 "want the title", 0x85 "hope for successful season", 0x30
/// "expecting difficult season", 0x308 "look forward to new season", otherwise "announce expectations". Then the text
/// into `out` (`exe::text_set_string`) and the club's link (`exe::text_add_link`, 0x7d5, name `club::NAME`), also on
/// the headline.
/// Original: `MANAGER_MANAGER::news_board_expectations_text` (0x6a4ae0, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_board_expectations_text(this: usize, news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut text = vec![0u8; 0x1388];
    let t = text.as_mut_ptr() as usize;
    let id = exe::newsitem_get_slot(news, 0);
    let flags = exe::newsitem_get_slot(news, 1);
    let club = read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(club::SIZE));
    if body != 0 {
        covered!("sma_6 expectations body");
        let build: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) = core::mem::transmute(board_text_fns::BUILD_BOARD_EXPECTATION_TEXT);
        build(this, club, flags, t, 0x1388);
    } else {
        let h = if id as i32 > read_i32(G_CLUB_COUNT) - 2 * read_i32(G_NATION_COUNT) {
            0xa0abcc
        } else if flags & 0x400 != 0 {
            0xa0abb8
        } else if flags & 2 != 0 {
            0xa0aba0
        } else if flags & 0x85 != 0 {
            0xa0ab7c
        } else if flags & 0x30 != 0 {
            0xa0ab58
        } else if flags & 0x308 != 0 {
            0xa0ab34
        } else {
            0xa0ab18
        };
        format(t, h); // sma6: the expectations headline
    }
    exe::text_set_string(out, t as *const u8);
    exe::text_add_link(out, (club + club::NAME) as *const u8, 0x7d5, club);
}

/// Comparer for `news_board_expectations_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_news_board_expectations_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_NEWS_BOARD_EXPECTATIONS_TEXT);
    compare_text_writer(out, || o(this, news, body, out), |t| news_board_expectations_text(this, news, body, t),
        || format!("news_board_expectations_text(body {})", body));
}

/// Writes the headline or body of the board's (or FA's) confidence update, with links to the players it names.
///
/// `MANAGER_MANAGER::news_board_confidence_update_text(news, body, out)`: slots 0 (club id) and 1 (the flags).
/// Headline: "Football Association Confidence Update" for a national team (`exe::database_is_national_team`), "Board
/// Confidence Update" otherwise. Body: the confidence sentence (0x690d80) and the named players' sentence (0x693850,
/// up to 15 ids, returns how many); with players, both joined by "%s\n\n%s", otherwise the first one. The text into
/// `out` (`exe::text_set_string`); each named player (id not −1) whose formatted name (`exe::database_format_staff_name`,
/// 0x96, style 1) is in the text (`exe::crt_strstr`) gets a link (0x7db); the club gets its link (0x7d5) when its name
/// is in the text.
/// Original: `MANAGER_MANAGER::news_board_confidence_update_text` (0x6a4d40, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_board_confidence_update_text(this: usize, news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let mut ids = [-1i32; 15];
    let mut name = [0u8; 0x98];
    let mut text = vec![0u8; 0x1388];
    let mut first = vec![0u8; 0x1388];
    let mut players = vec![0u8; 0x5dc];
    let t = text.as_mut_ptr() as usize;
    let id = exe::newsitem_get_slot(news, 0);
    let flags = exe::newsitem_get_slot(news, 1);
    let club = read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(club::SIZE));
    let mut count: i32 = 0;
    if body == 0 {
        format(t, if exe::database_is_national_team(club) != 0 { 0xa0ac00 } else { 0xa0abe8 });
    } else {
        covered!("sma_6 confidence body");
        let conf: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) = core::mem::transmute(board_text_fns::BUILD_BOARD_CONFIDENCE_TEXT);
        conf(this, club, flags, first.as_mut_ptr() as usize, 0x1388);
        let named: unsafe extern "thiscall" fn(usize, usize, usize, *mut i32) -> u8 = core::mem::transmute(board_text_fns::BUILD_BOARD_NAMED_PLAYERS_TEXT);
        count = named(this, players.as_mut_ptr() as usize, club, ids.as_mut_ptr()) as i8 as i32;
        if count != 0 {
            sprintf(t, 0x991948, first.as_ptr() as usize, players.as_ptr() as usize); // sma6: two sentences
        } else {
            sprintf(t, 0x991944, first.as_ptr() as usize);
        }
    }
    exe::text_set_string(out, t as *const u8);
    for &p in ids.iter().take(count.max(0) as usize) {
        if p == -1 {
            continue;
        }
        let person = read_usize(G_STAFF_TABLE).wrapping_add((p as usize).wrapping_mul(staff::SIZE));
        exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 0x96, 1);
        if exe::crt_strstr(t, name.as_ptr() as usize) != 0 {
            exe::text_add_link(out, name.as_ptr(), 0x7db, person);
        }
    }
    if exe::crt_strstr(t, club + club::NAME) != 0 {
        exe::text_add_link(out, (club + club::NAME) as *const u8, 0x7d5, club);
    }
}

/// Comparer for `news_board_confidence_update_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_news_board_confidence_update_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_NEWS_BOARD_CONFIDENCE_UPDATE_TEXT);
    compare_text_writer(out, || o(this, news, body, out), |t| news_board_confidence_update_text(this, news, body, t),
        || format!("news_board_confidence_update_text(body {})", body));
}

// ---------- batch 290: the manager appointment news text (manager_manager.cpp) ----------

/// Exe routines of the manager news texts without an `exe::` wrapper (Rust hooked, called through the exe).
pub(crate) mod manager_text_fns {
    pub(crate) const GET_SLOT_STRING: usize = 0x785a20; // NewsItem::get_slot_string: thiscall(news, k, buf, n), ret 0xc
    pub(crate) const GET_JOB_NAME: usize = 0x53ad00; // database_get_job_name: cdecl(job, short, capital, &gender) -> str
}

/// The manager's name of a manager news into `name` (0x190 bytes): a human manager (id ≥ `G_STAFF_COUNT` − 16) from
/// the two strings of the news (slot 8, then the slot whose index slot 7 holds; `get_slot_string`, 0x33 each;
/// `exe::database_format_name_from_strings`, style 1, −1); anyone else from the three name ids (`ids`,
/// `exe::database_format_name_from_ids`, style `style`).
pub(crate) unsafe fn manager_news_name(news: usize, id: u32, ids: [u32; 3], name: &mut [u8; 0x190], style: u32) {
    if id as i32 >= read_i32(G_STAFF_COUNT) - 0x10 {
        let get: unsafe extern "thiscall" fn(usize, u32, *mut u8, i32) -> u32 = core::mem::transmute(manager_text_fns::GET_SLOT_STRING);
        let (mut first, mut second) = ([0u8; 0x34], [0u8; 0x34]);
        get(news, 8, first.as_mut_ptr(), 0x33);
        get(news, exe::newsitem_get_slot(news, 7), second.as_mut_ptr(), 0x33);
        exe::database_format_name_from_strings(first.as_ptr() as usize, second.as_ptr() as usize, 0usize, name.as_mut_ptr() as usize, 0x190, 1, -1);
    } else {
        exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x190, style, id);
    }
}

/// Writes the headline or body of the news that a club appointed or promoted someone to be its manager.
///
/// `news_manager_appointment_text(news, body, out)`: the slots 0 (the person's id), 1..3 (the name ids), 4 (the old
/// club id, −1 none), 5 (the club id), 6 (the news code: 1 player, 2 staff, 3 manager) and 7 (the old job). The name
/// (`manager_news_name`: style 5 in the headline, 1 in the body). Headline: 0xa0a3a8 with the club's short gender and
/// short name (`club::SHORT_GENDER_NAME`, `club::SHORT_NAME`) and the name; links: the person (0x7da) when the name
/// ids still match (`exe::database_staff_matches_names`) and the club (0x7d5). Body (formats into 0xde1f64,
/// `FN_LANGLIB_SPRINTF`, with the genders `club::GENDER_NAME` and names `club::NAME`): no old club 0xa09e18; code 1: the
/// same club 0xa09f14, otherwise 0xa09e78 (both clubs); code 2: without a club job (`.job_for_club()`) 0xa09f8c, at the
/// same club by the old job: 0 0xa0a184, 11 0xa0a1f4, other 0xa0a274 with the job's name and gender
/// (`database_get_job_name`); from another club: 11 0xa0a0e4, other 0xa09fe4; code 3: 0xa0a2f8; any other code leaves
/// the buffer as it was (the original's). Links: the person when the ids match (0x7dc for a national team club, 0x7d7
/// otherwise), the club and the old club (0x7d5).
/// Original: `news_manager_appointment_text` (0x6a3060, manager_manager.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn news_manager_appointment_text(news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let s: [u32; 8] = core::array::from_fn(|k| exe::newsitem_get_slot(news, k as u32));
    let id = s[0];
    let ids = [s[1], s[2], s[3]];
    let person = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    let club_at = |k: u32| read_usize(G_CLUB_TABLE).wrapping_add((k as usize).wrapping_mul(club::SIZE));
    let new = club_at(s[5]);
    let mut name = [0u8; 0x190];
    let nm = name.as_ptr() as usize;
    let buf = 0xde1f64usize;
    if body == 0 {
        manager_news_name(news, id, ids, &mut name, 5);
        let g = read_i8(new + club::SHORT_GENDER_NAME) as i32;
        format(buf, 0xa0a3a8, g, g, new + club::SHORT_NAME, nm);
        exe::text_set_string(out, buf as *const u8);
        if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
            exe::text_add_link(out, name.as_ptr(), 0x7da, person);
        }
        exe::text_add_link(out, (new + club::NAME) as *const u8, 0x7d5, new);
        return;
    }
    covered!("sma_7 appointment body");
    manager_news_name(news, id, ids, &mut name, 1);
    let old_id = s[4] as i32;
    let old = club_at(s[4]);
    let gn = read_i8(new + club::GENDER_NAME) as i32;
    let go = || read_i8(old + club::GENDER_NAME) as i32;
    let (newn, oldn) = (new + club::NAME, old + club::NAME);
    if old_id == -1 {
        format(buf, 0xa09e18, gn, gn, newn, nm);
    } else {
        let job_name = |gender: &mut i8| {
            let f: unsafe extern "C" fn(u32, u32, u32, *mut i8) -> usize = core::mem::transmute(manager_text_fns::GET_JOB_NAME);
            f(s[7] as u8 as u32, 0, 0, gender)
        };
        match s[6] as u8 as i8 {
            1 => {
                if s[5] == s[4] {
                    format(buf, 0xa09f14, gn, gn, newn, nm);
                } else {
                    format(buf, 0xa09e78, gn, gn, go(), go(), newn, oldn, nm);
                }
            }
            2 => {
                let old_job = s[7] as u8;
                if Staff(person).job_for_club() == 0 {
                    format(buf, 0xa09f8c, gn, gn, newn, nm);
                } else if s[5] == s[4] {
                    match old_job {
                        0 => {
                            format(buf, 0xa0a184, gn, gn, newn, nm);
                        }
                        0xb => {
                            format(buf, 0xa0a1f4, gn, gn, newn, nm);
                        }
                        _ => {
                            let mut gt: i8 = 0;
                            let title = job_name(&mut gt);
                            let gt = gt as i32;
                            format(buf, 0xa0a274, gn, gn, gt, gt, newn, title, nm); // sma7: promoted at the club
                        }
                    }
                } else {
                    let mut gt: i8 = 0;
                    let title = job_name(&mut gt);
                    let gt = gt as i32;
                    if old_job == 0xb {
                        format(buf, 0xa0a0e4, gn, gn, go(), go(), gt, gt, newn, oldn, title, nm);
                    } else {
                        format(buf, 0xa09fe4, gn, gn, go(), go(), gt, gt, go(), go(), gt, gt, newn, oldn, title, nm, oldn, title);
                    }
                }
            }
            3 => {
                format(buf, 0xa0a2f8, gn, gn, go(), go(), newn, nm, oldn);
            }
            _ => {}
        }
    }
    exe::text_set_string(out, buf as *const u8);
    if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
        let kind = if s[5] as i32 > -1 && (s[5] as i32) < read_i32(G_CLUB_COUNT) && exe::database_is_national_team(new) != 0 { 0x7dc } else { 0x7d7 };
        exe::text_add_link(out, name.as_ptr(), kind, person);
    }
    exe::text_add_link(out, newn as *const u8, 0x7d5, new);
    if old_id != -1 {
        exe::text_add_link(out, oldn as *const u8, 0x7d5, old);
    }
}

/// Comparer for `news_manager_appointment_text` (`compare_text_writer`).
pub(crate) unsafe extern "stdcall" fn compare_news_manager_appointment_text(news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(IDX_NEWS_MANAGER_APPOINTMENT_TEXT);
    compare_text_writer(out, || o(news, body, out), |t| news_manager_appointment_text(news, body, t),
        || format!("news_manager_appointment_text(body {})", body));
}

/// Writes the headline or body of a story about board pressure on a manager: a shortlist for his job, a rumour, or the
/// FA's view of a national team manager.
///
/// `news_manager_under_pressure_text(news, body, out)`: slots 0 (the manager's id), 1..3 (his name ids), 4 (the club
/// id), 5..7 (the chairman's name ids, 5 = −1 without one), 8 (the reason, 1..4), 9 (the kind: 1 rumour), 10 (the
/// chairman's id), 11 (the rumour). The names (`exe::database_format_name_from_ids`, style 5 in the headline, 1 in the
/// body), the club's name (`club::SHORT_NAME` in the headline, `club::NAME` in the body, through "%s") and its
/// nation's adjective (`exe::nation_get_nationality_adjective`, 100, 9). A national team with a nation (`club::NATION`)
/// takes the FA texts (genders 100). Headline: kind 1 0xa06ff8 / FA 0xa07054; otherwise (after 0xa06f98 when there is
/// no reason, a chairman name and no rumour: written and then overwritten, as in the original) 0xa06ed8 / FA 0xa06f38.
/// Body: kind 1: FA 0xa06e50; rumour 1 0xa06d18, other rumour 0xa06dc0; none 0xa06bec. Reason 1..4: with a chairman
/// 0xa0669c, 0xa06444, 0xa068f0, 0xa06b28 (and his name); without one the club text (0xa064f8, 0xa062f4, 0xa06778,
/// 0xa069b8) or the FA one (0xa065c4, 0xa06394, 0xa0682c, 0xa06a68). Other reasons: without a chairman 0xa06058 / FA
/// 0xa060d4; with a rumour 0xa06160; otherwise 0xa061f0. Then the text (`exe::text_set_string`), the manager's link
/// (0x7d7) when the ids match (`exe::database_staff_matches_names`) and the club's link (0x7d5) on the adjective (FA,
/// body) or the club name.
/// Original: `news_manager_under_pressure_text` (0x6a1190, manager_manager.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn news_manager_under_pressure_text(news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let s: [u32; 12] = core::array::from_fn(|k| exe::newsitem_get_slot(news, k as u32));
    let style = if body != 0 { 1 } else { 5 };
    let (mut name, mut chair, mut club_name, mut adj) = ([0u8; 0x190], [0u8; 0x190], [0u8; 0x190], [0u8; 0x64]);
    exe::database_format_name_from_ids(s[1], s[2], s[3], name.as_mut_ptr(), 0x190, style, s[0]);
    if s[5] as i32 == -1 {
        chair[0] = 0;
    } else {
        exe::database_format_name_from_ids(s[5], s[6], s[7], chair.as_mut_ptr(), 0x190, style, s[10]);
    }
    let club = read_usize(G_CLUB_TABLE).wrapping_add((s[4] as usize).wrapping_mul(club::SIZE));
    sprintf(club_name.as_mut_ptr() as usize, 0x991944, club + if body != 0 { club::NAME } else { club::SHORT_NAME });
    exe::nation_get_nationality_adjective(read_usize(club + club::NATION), adj.as_mut_ptr() as usize, 0x64, 9);
    let (nm, ch, cl, ad) = (name.as_ptr() as usize, chair.as_ptr() as usize, club_name.as_ptr() as usize, adj.as_ptr() as usize);
    let fa = exe::database_is_national_team(club) != 0 && read_usize(club + club::NATION) != 0;
    let buf = 0xde1f64usize;
    let (reason, kind, rumour) = (s[8] as u8, s[9] as u8, s[11]);
    if body == 0 {
        let g = read_i8(club + club::SHORT_GENDER_NAME) as i32;
        if kind == 1 {
            if fa {
                format(buf, 0xa07054, 100, 100, 100, ad, nm);
            } else {
                format(buf, 0xa06ff8, g, g, 100, cl, nm);
            }
        } else {
            if reason == 0 && s[5] != 0 && rumour == 0 {
                format(buf, 0xa06f98, g, g, cl, nm);
            }
            if fa {
                format(buf, 0xa06f38, 100, 100, ad, nm);
            } else {
                format(buf, 0xa06ed8, g, g, cl, nm);
            }
        }
    } else {
        covered!("sma_7 pressure body");
        let g = read_i8(club + club::GENDER_NAME) as i32;
        if kind == 1 {
            if fa {
                format(buf, 0xa06e50, 100, 100, ad, nm);
            } else if rumour != 0 {
                format(buf, if rumour == 1 { 0xa06d18 } else { 0xa06dc0 }, g, g, cl, nm);
            } else {
                format(buf, 0xa06bec, g, g, cl, nm, nm);
            }
        } else if (1..=4).contains(&reason) {
            // (with the chairman, the club, the FA) by the reason
            let t = [[0xa0669c, 0xa064f8, 0xa065c4], [0xa06444, 0xa062f4, 0xa06394], [0xa068f0, 0xa06778, 0xa0682c], [0xa06b28, 0xa069b8, 0xa06a68]][reason as usize - 1];
            if s[5] as i32 != -1 {
                format(buf, t[0], g, g, 100, cl, ch, nm); // sma7: the reason with the chairman
            } else if fa {
                format(buf, t[2], 100, 100, 100, ad, nm);
            } else {
                format(buf, t[1], g, g, 100, cl, nm);
            }
        } else if s[5] as i32 == -1 {
            if fa {
                format(buf, 0xa060d4, 100, 100, 100, ad, nm);
            } else {
                format(buf, 0xa06058, g, g, 100, cl, nm);
            }
        } else if rumour != 0 {
            format(buf, 0xa06160, g, g, 100, cl, ch, nm);
        } else {
            format(buf, 0xa061f0, g, g, nm, cl, ch);
        }
    }
    exe::text_set_string(out, buf as *const u8);
    let person = read_usize(G_STAFF_TABLE).wrapping_add((s[0] as usize).wrapping_mul(staff::SIZE));
    if exe::database_staff_matches_names(person, s[1], s[2], s[3], 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7d7, person);
    }
    if body != 0 && fa {
        exe::text_add_link(out, adj.as_ptr(), 0x7d5, club);
    } else {
        exe::text_add_link(out, club_name.as_ptr(), 0x7d5, club);
    }
}

/// Comparer for `news_manager_under_pressure_text` (`compare_text_writer`).
pub(crate) unsafe extern "stdcall" fn compare_news_manager_under_pressure_text(news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(IDX_NEWS_MANAGER_UNDER_PRESSURE_TEXT);
    compare_text_writer(out, || o(news, body, out), |t| news_manager_under_pressure_text(news, body, t),
        || format!("news_manager_under_pressure_text(body {})", body));
}
