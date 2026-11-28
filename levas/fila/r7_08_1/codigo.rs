// --- r7_08_1: a pre-arranged future transfer from the player configuration file (transfer_manager.cpp) ---

/// The release fee a configured future transfer writes when the desired contract has one: the larger of the fee +
/// 250,000 and 1.5 × the fee (the x87 comparison: on a tie the 1.5 × side), truncated.
pub(crate) unsafe fn config_transfer_release_fee(fee: i32) -> i32 {
    let plus = fee.wrapping_add(250_000) as f64;
    let times = fee as f64 * read_f64(rdata::F64_1_5);
    ftol(if plus <= times { times } else { plus })
}

/// Sets up a future transfer the player configuration file arranges (a FUTURE_TRANSFER line of `player_setup.cfg`,
/// applied on game creation): the bid, the contract offer and the future transfer record, and with a free transfer the
/// player's current contract ends the day before.
///
/// `TRANSFER_MANAGER::add_config_future_transfer(p, club, date, fee, wage, years)`, from `player_setup_apply_line`
/// (0x7a3a10). A local TRANSFER_OFFER (`exe::new_transfer_offer`, destroyed at the end) and a local STAFF_CONTRACT
/// (`exe::new_staff_contract`). A fee of −1 becomes the player's `.value()`. The player's current contract
/// (`exe::contract_manager_get_contract`, kept for the end) is read first. A player who already has an offer
/// (`exe::transfer_manager_get_offers_for_player`) gets two "Error" boxes (transfer_manager.cpp lines 0x6ce and 0x6d0,
/// with his name formatted in between into the 0xde1f64 buffer, which nothing reads) and nothing else.
///
/// Otherwise: the bid (`exe::transfer_offer_build_bid` with the fee, and "free" when the fee is 0), the completion timing's
/// low nibble 3 (`offer::COMPLETION_TIMING`: (b & 0xf3) | 3), the fee in `offer::FEE`, job 11 (`offer::JOB_TYPE`), and
/// the offer goes in (`exe::transfer_manager_add_transfer_offer`). The desired contract
/// (`exe::staff_contract_set_to_desired_contract` with the offer's slot, 2, job 11): a wage of −1 keeps its wage within
/// the club's cap (`exe::finance_get_max_wage` with −1), any other wage replaces it; a contract with a release fee gets
/// `config_transfer_release_fee`; years below 1 become random(4) + 2; it starts on the date and ends 365 × years days
/// later (`exe::date_add_days`). The contract offer goes to the contract manager (`exe::contract_manager_add_contract_offer`
/// with the offer's slot and serial, job 11, 0, 0, 1) and the offer becomes a future transfer on the date
/// (`exe::transfer_manager_add_future_transfer`).
///
/// A free transfer (fee 0) of a player under contract (looked up twice, kept in `G_CONTRACT_LOOKUP_RESULT`) at a club
/// whose nation allows free transfers (`exe::nation_allows_free_transfers`): the contract read first is marked
/// `contract::LEAVING_ON_BOSMAN` = 1, loses bit 3 of `contract::TRANSFER_STATUS`, and ends the day before the date
/// (`exe::date_sub_days` 1).
/// Original: `TRANSFER_MANAGER::add_config_future_transfer` (0x8bf060, transfer_manager.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn add_config_future_transfer(this: usize, p: usize, club: usize, date: usize, fee: i32, wage: i32, years: i32) {
    let p = Staff(p);
    let club = Club(club);
    let mut offer_buf = [0u8; 0x44]; // the local TRANSFER_OFFER
    let mut contract_buf = [0u8; 0x50]; // the local STAFF_CONTRACT
    let offer = Offer(offer_buf.as_mut_ptr() as usize);
    let terms = Contract(contract_buf.as_mut_ptr() as usize);
    exe::new_transfer_offer(offer);
    exe::new_staff_contract(terms);
    let fee = if fee == -1 { p.value() as i32 } else { fee };
    let current = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
    if exe::transfer_manager_get_offers_for_player(this, p, 0usize, 0) != 0 {
        covered!("r7_08_1 player already has an offer");
        show_error_box(0xa8d514, 0x6ce);
        exe::database_format_staff_name(p, 0xde1f64usize, 0x7d0, 1);
        show_error_box(0xa8d514, 0x6d0);
        exe::drop_transfer_offer(offer);
        return;
    }
    exe::transfer_offer_build_bid(offer, p, club, 0usize, fee as u32, (fee == 0) as u32, core::ptr::null_mut());
    offer.set_completion_timing((offer.completion_timing() & 0xf3) | 3);
    offer.set_fee(fee as usize);
    offer.set_job_type(0xb);
    exe::transfer_manager_add_transfer_offer(this, offer.addr() as *mut u8, core::ptr::null_mut());
    exe::staff_contract_set_to_desired_contract(terms, p, club, offer.slot_index() as i32, 2, 0xb, 0, 0);
    if wage == -1 {
        let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        let cap = exe::finance_get_max_wage(finance, p, u32::MAX);
        if terms.wage() > cap {
            terms.set_wage(cap); // r7_08_1: the wage within the club's cap
        }
    } else {
        terms.set_wage(wage);
    }
    if terms.release_fee() != 0 {
        terms.set_release_fee(config_transfer_release_fee(fee) as usize);
    }
    let years = if (years as i8) < 1 { (exe::utils_random(4) as u8).wrapping_add(2) as i8 } else { years as i8 }; // r7_08_1: 2 to 5 years
    transfer_manager_assign_date(terms.addr() + contract::DATE_STARTED, date);
    let mut t = [0u32; 2];
    let end = exe::date_add_days(date, t.as_mut_ptr() as usize, (years as i16).wrapping_mul(365) as u16 as u32);
    transfer_manager_assign_date(terms.addr() + contract::CONTRACT_EXPIRES, end);
    exe::contract_manager_add_contract_offer(read_usize(G_CONTRACT_MANAGER), terms.addr() as *mut u8, offer.slot_index(), offer.serial(), 0xb, 0, 0, 1);
    let placed = read_usize(read_usize(this).wrapping_add(offer.slot_index().wrapping_mul(4)));
    exe::transfer_manager_add_future_transfer(this, placed, date as *mut [u32; 2]);
    covered!("r7_08_1 future transfer added");
    if fee == 0 && free_transfer_allowed(p) {
        covered!("r7_08_1 free transfer: the contract ends the day before");
        current.set_leaving_on_bosman(1);
        current.set_transfer_status(current.transfer_status() & 0xf7);
        let mut t = [0u32; 2];
        let day_before = exe::date_sub_days(date, t.as_mut_ptr() as usize, 1);
        transfer_manager_assign_date(current.addr() + contract::CONTRACT_EXPIRES, day_before);
    }
    exe::drop_transfer_offer(offer);
}

/// The free transfer test of `add_config_future_transfer`, with the original's two contract lookups (both kept in
/// `G_CONTRACT_LOOKUP_RESULT`): a contract, its club, and the club's nation allows free transfers.
unsafe fn free_transfer_allowed(p: Staff) -> bool {
    let c = Contract(contract_of_staff_cached(p));
    if c.is_null() || transfer_manager_club_by_id(c.club()).is_null() {
        return false;
    }
    let c = Contract(contract_of_staff_cached(p));
    let club = if c.is_null() { Club(0) } else { transfer_manager_club_by_id(c.club()) };
    exe::nation_allows_free_transfers(read_usize(club.addr() + club::NATION)) != 0
}

/// Comparer for `add_config_future_transfer`: the original only (it adds an offer, a contract offer and a future
/// transfer: it cannot run twice); the plan is checked on exit: a player without offers has one afterwards, one with
/// offers keeps the same count.
pub(crate) unsafe extern "thiscall" fn compare_add_config_future_transfer(this: usize, p: usize, club: usize, date: usize, fee: i32, wage: i32, years: i32) {
    let before = exe::transfer_manager_get_offers_for_player(this, p, 0usize, 0);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, i32, i32)>(%%I0%%)(this, p, club, date, fee, wage, years);
    let after = exe::transfer_manager_get_offers_for_player(this, p, 0usize, 0);
    let same = if before == 0 { after != 0 } else { after == before };
    SCOREBOARD.record(same, || format!("add_config_future_transfer(person {:#x}, club {:#x}) offers before={} after={}", read_usize(p), read_usize(club), before, after));
}
