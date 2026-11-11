// ---------- batch %%LEVA%%: the person chooses among his contract offers (contract_manager.cpp) ----------

/// The routines and the `.rdata` double the decision on contract offers uses that `exe_fns.rs` / `rdata_consts.rs` do
/// not name yet (cm0102.exe 3.9.68). The Rust ones are called by address, so they go through their detour.
#[allow(dead_code)]
pub(crate) mod resolve_offers_fns {
    pub(crate) const FN_PLAYER_HAS_PENDING_OFFER: usize = 0x8cd150; // TRANSFER_MANAGER::player_has_pending_offer (thiscall, ret 4)
    pub(crate) const FN_FIND_OFFER_PLAYER_ENTRY: usize = 0x8e82f0; // the person's entry in a transfer offer's list +4 (thiscall, ret 4)
    pub(crate) const FN_HAS_SPARE_NON_PLAYER_RECORD: usize = 0x7a6e20; // PLAYER_REGEN::has_spare_non_player_record (al)
    pub(crate) const FN_TAKE_SPARE_NON_PLAYER_RECORD: usize = 0x7a6e30; // PLAYER_REGEN::take_spare_non_player_record (ret 0xc)
    pub(crate) const FN_REMOVE_TRANSFER_INTEREST: usize = 0x7549b0; // Media::remove_transfer_interest (ret 4)
    pub(crate) const FN_SET_AGREED_CONTRACT_FLAG: usize = 0x8c5410; // TRANSFER_MANAGER (ret 4)
    pub(crate) const FN_HANDLE_PLAYER_REJECTED_TERMS: usize = 0x8c28c0; // TRANSFER_MANAGER (ret 0xc)
    pub(crate) const FN_HANDLE_PLAYER_ACCEPTED_TERMS: usize = 0x8c2180; // TRANSFER_MANAGER (ret 0x14)
    pub(crate) const FN_COMPLETE_ACCEPTED_CONTRACT_OFFER: usize = 0x8c38b0; // TRANSFER_MANAGER (ret 4)
    pub(crate) const FN_POST_OFFER_RESPONSE_NEWS: usize = 0x4d49a0; // contract_manager_post_offer_response_news (ret 8)
    pub(crate) const F64_0_99: usize = 0x968e50; // f64 = 0.99
    pub(crate) const OFFER_SIZE: usize = 0x6f; // a contract offer: the STAFF_CONTRACT (0x50) and contract_offer::STATE..+0x6e

    use super::*;
    #[inline(always)]
    pub(crate) unsafe fn tm_call1(f: usize, a: usize) -> u32 {
        let g: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(f);
        g(read_usize(G_TRANSFER_MANAGER), a)
    }
    #[inline(always)]
    pub(crate) unsafe fn find_offer_player_entry(offer: usize, person: usize) -> usize {
        let g: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FIND_OFFER_PLAYER_ENTRY);
        g(offer, person)
    }
    #[inline(always)]
    pub(crate) unsafe fn has_spare_non_player_record() -> u8 {
        let g: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_HAS_SPARE_NON_PLAYER_RECORD);
        g(read_usize(G_PLAYER_REGEN)) as u8
    }
    #[inline(always)]
    pub(crate) unsafe fn take_spare_non_player_record(id: usize, nation: usize, second: usize) {
        let g: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_TAKE_SPARE_NON_PLAYER_RECORD);
        g(read_usize(G_PLAYER_REGEN), id, nation, second);
    }
    #[inline(always)]
    pub(crate) unsafe fn remove_transfer_interest(person: usize) {
        let g: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_REMOVE_TRANSFER_INTEREST);
        g(read_usize(G_MEDIA), person)
    }
    #[inline(always)]
    pub(crate) unsafe fn handle_player_rejected_terms(person: usize, slot: i32, best: usize) {
        let g: unsafe extern "thiscall" fn(usize, usize, i32, u32) = core::mem::transmute(FN_HANDLE_PLAYER_REJECTED_TERMS);
        g(read_usize(G_TRANSFER_MANAGER), person, slot, best as u32)
    }
    #[inline(always)]
    pub(crate) unsafe fn handle_player_accepted_terms(person: usize, slot: i32, offer: usize, kind: u32, clubs: usize) {
        let g: unsafe extern "thiscall" fn(usize, usize, i32, usize, u32, usize) = core::mem::transmute(FN_HANDLE_PLAYER_ACCEPTED_TERMS);
        g(read_usize(G_TRANSFER_MANAGER), person, slot, offer, kind, clubs)
    }
    #[inline(always)]
    pub(crate) unsafe fn post_offer_response_news(this: usize, offer: usize, flag: u32) {
        let g: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_POST_OFFER_RESPONSE_NEWS);
        g(this, offer, flag)
    }
}

/// What the evaluation of a person's contract offers decided (`choose_contract_offer`).
#[derive(Clone, Copy)]
pub(crate) struct ContractOfferChoice {
    /// the chosen offer (0 none)
    pub(crate) best: usize,
    /// its score (the will × the ratio; the will for a loan or a trial); starts at 2.75
    pub(crate) best_score: f64,
    /// the last offer of his own club that he turned down (0 none)
    pub(crate) rejected_own: usize,
    /// an offer already agreed (state 2) was found: nothing is chosen
    pub(crate) accepted: bool,
    /// how many stale offers were taken out of the list
    pub(crate) removed: u32,
}

/// One less contract offer for the person (byte +8 of the 79-byte staff state record).
unsafe fn resolve_offers_count_down(person: Staff) {
    let c = contract_offer_counter(person.addr());
    write_u8(c, read_u8(c).wrapping_sub(1));
}

/// The first part of `resolve_contract_offers`: walks the person's offers and scores each one, keeping the best. With
/// `lists` (the real run) each evaluated offer goes into the first list (its pointer) and its score into the second,
/// the stale offers leave the contract manager's list and the transfer offer's entry gets the squad status; without
/// (`None`, the comparer's plan) nothing is changed: the removals are only counted and the entries' bytes are put
/// back. `desired` is the local STAFF_CONTRACT the desired contract is written into (reused from offer to offer, as in
/// the original).
pub(crate) unsafe fn choose_contract_offer(this: usize, person: Staff, desired: usize, lists: Option<(Llist, Llist)>) -> ContractOfferChoice {
    use resolve_offers_fns::*;
    // k = (professionalism − ambition − 9 + loyalty) × 1/300 (fild of the int, fmul 0x968e60)
    let numerator = person.professionalism() as i32 - person.ambition() as i32 + person.loyalty() as i32 - 9;
    let k = numerator as f64 * read_f64(rdata::F64_0_003333333333);
    let mut c = ContractOfferChoice { best: 0, best_score: 2.75, rejected_own: 0, accepted: false, removed: 0 };
    let mut own: i32 = numerator; // [esp+0x28]: the original's slot holds the numerator until the first renewal test
    let mut will = 0.0f64; // [esp+0x2c]
    let mut put_back: Vec<(usize, u8)> = Vec::new();
    let list = Llist(read_usize(this + contract_manager_fields::CONTRACT_OFFERS));
    if !list.is_null() {
        list.set_cursor(list.head());
        loop {
            let o = exe::llist_get_next(list);
            if o == 0 {
                break;
            }
            let offer = ContractOffer(o);
            if read_usize(o) != read_usize(person.addr()) || offer.decision_valid() == 0 {
                continue;
            }
            let club = club_at(offer.club_id() as isize);
            if offer.transfer_offer_slot() as i32 == -1 {
                // a plain offer from a club that is not the one of his contract (loans aside) is stale
                let current = current_contract_club(person);
                if current != club && offer.contract_type() & 0x3f != 5 {
                    covered!("mkd_4 stale offer");
                    c.removed += 1;
                    if lists.is_some() {
                        exe::llist_remove(list, o);
                        resolve_offers_count_down(person);
                    }
                    continue;
                }
            }
            match offer.state() {
                0 => {}
                2 => {
                    c.accepted = true;
                    continue;
                }
                _ => continue,
            }
            let sl = TransferManagerFields(read_usize(G_TRANSFER_MANAGER)).shortlist_manager();
            let kind = offer.contract_type() & 0x3f;
            let mut score: f64;
            let base: f64; // [esp+0x4c]: the desired contract's value against itself (0 for loans and trials)
            if kind == 5 || kind == 6 {
                // a loan (status 8) or a trial (the squad status the club would give): the will alone
                let status = if kind == 5 { 8 } else { exe::shortlist_manager_get_squad_status_for_club(sl, person, club, 0usize) as u32 };
                let sl = TransferManagerFields(read_usize(G_TRANSFER_MANAGER)).shortlist_manager();
                let w = exe::shortlist_manager_staff_interest_in_joining_club(sl, person, club, 0, 1, status, 0);
                if w <= c.best_score || w < read_f64(rdata::F64_3_15) {
                    score = 0.0;
                } else {
                    covered!("mkd_4 loan or trial");
                    score = w;
                    c.best_score = w;
                    c.best = o;
                }
                base = 0.0;
            } else {
                let status = read_u8(o + contract::SQUAD_STATUS) >> 4;
                if offer.transfer_offer_slot() as i32 != -1 {
                    let slot = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), offer.transfer_offer_slot() as i32);
                    let e = find_offer_player_entry(read_usize(slot), person.addr());
                    if e != 0 {
                        if lists.is_none() {
                            put_back.push((e + 5, read_u8(e + 5)));
                        }
                        write_u8(e + 5, status);
                    }
                }
                let job = offer.mark();
                exe::staff_contract_set_to_desired_contract(desired, person, club, -1, status as u32, job as u32, 0, 0);
                base = exe::staff_contract_calc_contract_value(desired, person, desired) as f64;
                if base <= read_f64(rdata::F64_0) {
                    score = 0.0;
                } else {
                    // the ratio: the offer against the desired contract, less k
                    let v = exe::staff_contract_calc_contract_value(o, person, desired);
                    score = v as f64 / base - k; // mkd4: the ratio less k
                    let sl = TransferManagerFields(read_usize(G_TRANSFER_MANAGER)).shortlist_manager();
                    if person.non_player().is_null() || job == 0xb {
                        will = exe::shortlist_manager_staff_interest_in_joining_club(sl, person, club, 0, 0, status as u32, 0);
                    } else {
                        will = exe::staff_interest_in_job_at_club(sl, person, club, 0, job as u32, 0);
                        if exe::database_is_player_job(job as i8) != 0 {
                            let sl = TransferManagerFields(read_usize(G_TRANSFER_MANAGER)).shortlist_manager();
                            let w = exe::shortlist_manager_staff_interest_in_joining_club(sl, person, club, 0, 0, status as u32, 0);
                            will = (w + will) * read_f64(rdata::F64_0_5);
                        }
                    }
                    // a renewal (an offer from the club of his contract): a ratio of 0.99 lifts the will to 3.15
                    own = 0;
                    if !current_contract_club(person).is_null() {
                        let current = current_contract_club(person);
                        if read_usize(current.addr()) == offer.club_id() {
                            own = 1;
                            if score >= read_f64(F64_0_99) && will <= read_f64(rdata::F64_3_15) {
                                covered!("mkd_4 renewal floor");
                                will = 3.15; // mkd4: the renewal floor
                            }
                        }
                    }
                    if score < read_f64(rdata::F64_0_6) {
                        score = 0.0; // mkd4: the ratio cut
                    }
                }
            }
            // refusals: a staff job without a non-player sheet and no spare record; a work permit with under 90 days
            let job = offer.mark();
            if (exe::database_is_not_plain_player_job(job as i8) != 0 && person.non_player().is_null() && has_spare_non_player_record() == 0)
                || (score > read_f64(rdata::F64_0_5)
                    && exe::transfer_manager_player_needs_work_permit(read_usize(G_TRANSFER_MANAGER), person, club, 0) != 0
                    && exe::date_days_since(o + contract::CONTRACT_EXPIRES, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE + 4)) < 0x5a)
            {
                covered!("mkd_4 refused");
                score = 0.0;
            }
            if let Some((l1, l2)) = lists {
                let at = o;
                exe::llist_add_tail(l1, &at as *const usize as usize, 4);
                exe::llist_add_tail(l2, &score as *const f64 as usize, 8);
            }
            if base == read_f64(rdata::F64_0) {
                continue;
            }
            // the score: ratio ≥ 0.9, or > 0.7 with a will above 3.5, lifts the will to 3.15; > 0.8 with a will above
            // 3.15 keeps it; otherwise the offer is turned down
            let lift = if score >= read_f64(rdata::F64_0_9) {
                true
            } else if score > read_f64(rdata::F64_0_8) && will > read_f64(rdata::F64_3_15) {
                false
            } else if score > read_f64(rdata::F64_0_7) && will > read_f64(rdata::F64_3_5) {
                true
            } else {
                if own != 0 {
                    c.rejected_own = o;
                }
                continue;
            };
            if lift && will <= read_f64(rdata::F64_3_15) {
                will = read_f64(rdata::F64_3_15);
            }
            let s = will * score;
            if s > c.best_score {
                covered!("mkd_4 best offer");
                c.best_score = s;
                c.best = o;
            } else if own != 0 {
                c.rejected_own = o;
            }
        }
    }
    for &(a, v) in put_back.iter().rev() {
        write_u8(a, v);
    }
    // an agreed offer, or a manager candidate for a vacancy: nothing is chosen
    if c.accepted || (!person.non_player().is_null() && exe::manager_manager_is_vacancy_candidate(read_usize(G_MANAGER_MANAGER), person) != 0) {
        c.best = 0;
    }
    c
}

/// The person decides on his contract offers (the decision date of type 3 fell due): each open offer gets a score and
/// the best one above 2.75 is accepted; the others are turned down.
///
/// k = (`professionalism` − `ambition` − 9 + `loyalty`) × 1/300. With a pending transfer offer
/// (`TRANSFER_MANAGER::player_has_pending_offer`): a new type 3 decision date two days from today
/// (`exe::contract_manager_add_decision_date` at the current slice) and nothing else. Otherwise the offers in
/// `contract_manager_fields::CONTRACT_OFFERS` for the person with `contract_offer::DECISION_VALID` set
/// (`choose_contract_offer`): a plain offer (`contract_offer::TRANSFER_OFFER_SLOT` −1) from a club that is not the one
/// of his contract (stored in `G_CONTRACT_LOOKUP_RESULT`), outside loans, is stale: removed from the list and from the
/// counter (byte +8 of the 79-byte record). State 2 (already agreed) marks an agreed offer; other states are skipped.
/// State 0: a loan (type 5) is worth the will (`staff_interest_in_joining_club`, loan 1, status 8) and a trial (type
/// 6) the will with the squad status the club gives (`get_squad_status_for_club`); taken at once when above the best
/// so far and at least 3.15. Any other: the squad status (high nibble of `contract::SQUAD_STATUS`) goes to the
/// transfer offer's entry for the person; the desired contract at the club (`exe::staff_contract_set_to_desired_contract`
/// with −1, the status and the job `contract_offer::MARK`); ratio = value(offer) / value(desired)
/// (`exe::staff_contract_calc_contract_value`, both against the desired) − k; the will is
/// `staff_interest_in_joining_club` for a player (no non-player sheet, or job 11), otherwise
/// `exe::staff_interest_in_job_at_club`, averaged with the first for a player job; an offer from the club of his
/// contract is a renewal: a ratio of at least 0.99 lifts a will up to 3.15 to 3.15; a ratio below 0.6 is 0. Refused
/// (0): a job other than plain player for someone without a non-player sheet when no spare record is left
/// (`PLAYER_REGEN::has_spare_non_player_record`), or a ratio above 0.5 with a work permit needed
/// (`exe::transfer_manager_player_needs_work_permit`) and the offer's end less than 90 days away. Each one goes into
/// two local lists (pointer, score). Then: ratio ≥ 0.9, or above 0.7 with a will above 3.5, lifts the will to at least
/// 3.15; above 0.8 with a will above 3.15 keeps it; otherwise turned down. Score = will × ratio; the highest above the
/// best so far is chosen; a renewal not chosen is remembered. An agreed offer, or a manager candidate for a vacancy
/// (`exe::manager_manager_is_vacancy_candidate`, with a non-player sheet), chooses nothing.
///
/// Second pass over the evaluated offers: the chosen one becomes state 2 (`Media::remove_transfer_interest`,
/// `TRANSFER_MANAGER::set_agreed_contract_flag`); the others state 1, with the rejected terms
/// (`TRANSFER_MANAGER::handle_player_rejected_terms`, the chosen offer as the third argument) when tied to a transfer
/// offer or the response news (`contract_manager_post_offer_response_news`, flag 1 for the turned-down renewal of a
/// player in job 11 when nothing is chosen). None chosen: a turned-down renewal sets bit 0x80 of
/// `staff_state::STATUS_FLAGS`. Chosen, outside loans and trials, with a score above 10: when random(max(40 − score,
/// 5)) < `loyalty` and the club has a manager (reputation above 4500, or random(4) = 0), the person comes to favour
/// the manager (random(2) ≠ 0, `exe::staff_add_favoured_staff`) or the club (`exe::staff_add_favoured_club`).
///
/// Third pass: with a chosen offer, an offer of the same club without a transfer completes the contract
/// (`TRANSFER_MANAGER::complete_accepted_contract_offer` with the local copy; on success, a staff job for someone
/// without a non-player sheet takes a spare record (`PLAYER_REGEN::take_spare_non_player_record`; an id outside
/// 0..=count shows the line 0x13b4 box), a human club of his contract updates the coaches and the scouts' lists, and
/// the player value is recomputed (`exe::contract_manager_calc_player_value`, `contract::TRANSFER_STATUS` bit 4 of
/// the copy)); it leaves the list (counter − 1, `exe::transfer_manager_clear_agreed_contract_flag`). With a transfer
/// it stays. An offer of another club leaves the list (into `contract_manager_fields::HELD_OFFERS` when the chosen one
/// has a transfer; its club into the third local list). With none chosen, every offer outside state 3 leaves. A failed
/// completion ends here. Finally: with a transfer, the accepted terms (`TRANSFER_MANAGER::handle_player_accepted_terms`
/// with the copy, the job dword and the clubs list; a spare record first for a staff job without a sheet), otherwise
/// the response news of the copy; bit 0x80 cleared. Defect of the original: the spare record id is accepted up to the
/// count itself (`> count` is the error, not `>=`).
/// Original: `CONTRACT_MANAGER::resolve_contract_offers` (0x4ce4e0, contract_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn resolve_contract_offers(this: usize, person: usize) {
    use resolve_offers_fns::*;
    let p = Staff(person);
    let mut chosen_buf = [0u8; 0x70]; // [esp+0x88]: the local copy of the chosen offer
    let chosen = chosen_buf.as_mut_ptr() as usize;
    exe::new_staff_contract(chosen);
    exe::date_date_54a110(chosen + contract_offer::OFFER_DATE);
    let mut buf = [[0u8; 0x10]; 3];
    let l1 = Llist(buf[0].as_mut_ptr() as usize); // the evaluated offers (pointers)
    let l2 = Llist(buf[1].as_mut_ptr() as usize); // their scores (doubles)
    let l3 = Llist(buf[2].as_mut_ptr() as usize); // the clubs of the other offers (ids)
    exe::new_llist(l1);
    exe::new_llist(l2);
    let mut desired_buf = [0u8; 0x50];
    let desired = desired_buf.as_mut_ptr() as usize;
    exe::new_staff_contract(desired);
    exe::new_llist(l3);
    'body: {
        if tm_call1(FN_PLAYER_HAS_PENDING_OFFER, person) != 0 {
            covered!("mkd_4 postponed");
            let mut d = [0u32; 2];
            exe::date_add_days(G_CURRENT_DATE, d.as_mut_ptr() as usize, 2);
            exe::contract_manager_add_decision_date(this, person, d.as_ptr() as *const u8, read_u8(G_CURRENT_DAY_SLICE) as u32, 3);
            break 'body;
        }
        let c = choose_contract_offer(this, p, desired, Some((l1, l2)));
        let best = c.best;
        // second pass: states, news, rejected terms
        l2.set_cursor(l2.head());
        l1.set_cursor(l1.head());
        loop {
            let node = exe::llist_get_next(l1);
            if node == 0 {
                break;
            }
            let _score = read_f64(exe::llist_get_next(l2));
            let o = read_usize(node);
            if o == best && !c.accepted {
                ContractOffer(o).set_state(2);
                remove_transfer_interest(person);
                tm_call1(FN_SET_AGREED_CONTRACT_FLAG, person);
            } else {
                ContractOffer(o).set_state(1);
                let slot = ContractOffer(o).transfer_offer_slot() as i32;
                if slot != -1 {
                    handle_player_rejected_terms(person, slot, best);
                } else {
                    let flag = c.rejected_own == o && !p.player().is_null() && read_u8(c.rejected_own + contract_offer::MARK) == 0xb && best == 0;
                    post_offer_response_news(this, o, flag as u32);
                }
            }
        }
        let state = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(staff_state::SIZE));
        if best == 0 {
            if c.rejected_own != 0 {
                covered!("mkd_4 renewal turned down");
                write_u8(state + staff_state::STATUS_FLAGS, read_u8(state + staff_state::STATUS_FLAGS) | 0x80);
            }
        } else {
            // the original copies the chosen offer field by field (the dates through Date::operator=): the same 0x6f bytes
            core::ptr::copy_nonoverlapping(best as *const u8, chosen as *mut u8, OFFER_SIZE);
            let kind = read_u8(chosen + contract::CONTRACT_TYPE) & 0x3f;
            if kind != 5 && kind != 6 && c.best_score > read_f64(rdata::F64_10) {
                let mut n = read_f64(rdata::F64_40) - c.best_score;
                if n < read_f64(rdata::F64_5) {
                    n = read_f64(rdata::F64_5);
                }
                let r = exe::utils_random(ftol(n));
                let id = read_i32(chosen + contract_offer::CLUB_ID);
                if (p.loyalty() as i32) > r && id > -1 {
                    let club = club_at(id as isize);
                    if !club.manager().is_null() && (club.reputation() as i16 > 0x1194 || exe::utils_random(4) == 0) {
                        covered!("mkd_4 favoured");
                        if exe::utils_random(2) != 0 {
                            exe::staff_add_favoured_staff(p, club.manager(), 1);
                        } else {
                            exe::staff_add_favoured_club(p, club, 1);
                        }
                    }
                }
            }
        }
        // third pass: complete, hold or drop the offers
        let list = || Llist(read_usize(this + contract_manager_fields::CONTRACT_OFFERS));
        let mut failed = false;
        l1.set_cursor(l1.head());
        loop {
            let node = exe::llist_get_next(l1);
            if node == 0 {
                break;
            }
            let o = read_usize(node);
            if best != 0 {
                let transfer = read_i32(chosen + contract_offer::TRANSFER_OFFER_SLOT);
                if read_usize(o + contract_offer::CLUB_ID) == read_usize(chosen + contract_offer::CLUB_ID) {
                    if transfer != -1 {
                        continue;
                    }
                    failed = tm_call1(FN_COMPLETE_ACCEPTED_CONTRACT_OFFER, chosen) == 0;
                    if !failed {
                        covered!("mkd_4 contract completed");
                        if exe::database_is_not_plain_player_job(read_u8(chosen + contract_offer::MARK) as i8) != 0 && p.non_player().is_null() {
                            let (mut id, mut nation, mut second) = (0i32, 0u32, 0u32);
                            take_spare_non_player_record(&mut id as *mut i32 as usize, &mut nation as *mut u32 as usize, &mut second as *mut u32 as usize);
                            if id < 0 || id > read_i32(G_NON_PLAYER_COUNT) {
                                show_contract_error_box(0x13b4);
                            } else {
                                p.set_non_player(NonPlayer(read_usize(G_NON_PLAYER_TABLE).wrapping_add((id as usize).wrapping_mul(0x44))));
                            }
                        }
                        let club = current_contract_club(p);
                        let mut flag = None;
                        if !club.is_null() && exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
                            exe::training_manager_update_coach_assignments(read_usize(G_TRAINING_MANAGER), p.club_job(), 0);
                            exe::scout_manager_sync_scout_lists(read_usize(G_SCOUT_MANAGER), club.manager());
                            if read_u8(chosen + contract::TRANSFER_STATUS) & 4 != 0 && !p.player().is_null() {
                                flag = Some(1);
                            }
                        } else if !p.player().is_null() {
                            write_u8(chosen + contract::TRANSFER_STATUS, read_u8(chosen + contract::TRANSFER_STATUS) | 4);
                            flag = Some(0);
                        }
                        if let Some(f) = flag {
                            let v = exe::contract_manager_calc_player_value(this, p, club, 0, -1, f);
                            p.set_value(v as usize);
                        }
                    }
                    exe::llist_remove(list(), o);
                    resolve_offers_count_down(p);
                    exe::transfer_manager_clear_agreed_contract_flag(read_usize(G_TRANSFER_MANAGER), person as *const u32);
                    continue;
                }
                if transfer != -1 {
                    // held until the transfer goes through
                    if read_usize(this + contract_manager_fields::HELD_OFFERS) == 0 {
                        let m = exe::crt_operator_new(0xe);
                        let h = if m != 0 { exe::new_llist(m) } else { 0 };
                        write_usize(this + contract_manager_fields::HELD_OFFERS, h);
                    }
                    exe::llist_add_tail(read_usize(this + contract_manager_fields::HELD_OFFERS), o, OFFER_SIZE as u32);
                }
                exe::llist_add_tail(l3, o + contract_offer::CLUB_ID, 4);
            } else if ContractOffer(o).state() == 3 {
                continue;
            }
            exe::llist_remove(list(), o);
            resolve_offers_count_down(p);
        }
        if failed || best == 0 {
            break 'body;
        }
        let transfer = read_i32(chosen + contract_offer::TRANSFER_OFFER_SLOT);
        if transfer != -1 {
            covered!("mkd_4 accepted terms");
            if exe::database_is_not_plain_player_job(read_u8(chosen + contract_offer::MARK) as i8) != 0 && p.non_player().is_null() {
                take_spare_non_player_record(best + contract_offer::RESERVED_RECORD, best + contract_offer::RESERVED_RECORD_ARG1, best + contract_offer::RESERVED_RECORD_ARG2);
            }
            handle_player_accepted_terms(person, transfer, chosen, read_u32(chosen + contract_offer::MARK), l3.addr());
        } else {
            post_offer_response_news(this, chosen, 0);
        }
        let state = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(staff_state::SIZE));
        write_u8(state + staff_state::STATUS_FLAGS, read_u8(state + staff_state::STATUS_FLAGS) & 0x7f);
    }
    exe::drop_llist(l3);
    exe::drop_llist(l2);
    exe::drop_llist(l1);
}

/// Comparer for `resolve_contract_offers`.
///
/// The decision allocates, draws, posts news and signs contracts (it cannot run twice), so only the original runs; the
/// plan is the copy's evaluation (`choose_contract_offer` without lists: nothing removed, the entries put back; the
/// generator, the lookup and the static dates restored). After the original: with nothing chosen, the person's contract
/// is the same (club, wage, end) and bit 0x80 of `staff_state::STATUS_FLAGS` is the old one plus the turned-down
/// renewal; with an offer chosen, the contract is either the same (a failed completion, a loan, a transfer still to go
/// through) or has the chosen club and wage, and the bit is cleared or untouched. With a pending transfer offer (the
/// decision is postponed) only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_resolve_contract_offers(this: usize, person: usize) {
    use resolve_offers_fns::*;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let p = Staff(person);
    if tm_call1(FN_PLAYER_HAS_PENDING_OFFER, person) != 0 {
        return o(this, person);
    }
    let mut regions = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    regions.extend(TRANSFER_MANAGER_DATE_STATICS);
    regions.extend(crate::rng::STATE_REGIONS);
    let before = crate::comparison::MemorySnapshot::read(&regions);
    let mut desired_buf = [0u8; 0x50];
    let desired = desired_buf.as_mut_ptr() as usize;
    exe::new_staff_contract(desired);
    let plan = crate::rng::pause_trace(|| crate::rng::without_counting(|| choose_contract_offer(this, p, desired, None)));
    before.write_back();
    let terms = || {
        let c = get_contract(person);
        if c.is_null() { (usize::MAX, 0, 0) } else { (c.club(), c.wage(), read_u64(c.addr() + contract::CONTRACT_EXPIRES)) }
    };
    let state = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(staff_state::SIZE)) + staff_state::STATUS_FLAGS;
    let (t0, bit0) = (terms(), read_u8(state) & 0x80);
    let chosen = if plan.best != 0 { Some((read_usize(plan.best + contract_offer::CLUB_ID), read_i32(plan.best + contract::WAGE))) } else { None };
    o(this, person);
    let (t1, bit1) = (terms(), read_u8(state) & 0x80);
    let same = match chosen {
        None => t1 == t0 && bit1 == (bit0 | if plan.rejected_own != 0 { 0x80 } else { 0 }),
        Some((club, wage)) => (t1 == t0 || (t1.0 == club && t1.1 == wage)) && (bit1 == 0 || bit1 == bit0),
    };
    SCOREBOARD.record(same, || format!(
        "resolve_contract_offers(person {:#x}): plan best {:?} score {} renewal turned down {} agreed {}; contract {:x?} -> {:x?}, bit {:#x} -> {:#x}",
        read_usize(person), chosen, plan.best_score, plan.rejected_own != 0, plan.accepted, t0, t1, bit0, bit1));
}
