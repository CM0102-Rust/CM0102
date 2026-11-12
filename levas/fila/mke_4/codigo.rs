// ---------- batch %%LEVA%%: a person moves to his new club (transfer_manager.cpp) ----------

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
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I0%%)(this, person, club, offer) // simplification: original only
}
