// ---------- rxg_4: a negotiated loan contract submitted (transfer_manager.cpp) ----------

/// The "Error" box of transfer_manager.cpp at `line`, the offer's status 0x10 and the offer scheduled
/// (`exe::transfer_manager_schedule_offer` with 1, 0): the failed submission.
unsafe fn loan_contract_submission_failed(this: usize, offer: usize, line: u32) {
    show_error_at(TRANSFER_MANAGER_CPP, line);
    write_u8(offer + offer::STATUS, 0x10);
    exe::transfer_manager_schedule_offer(this, offer, 1, 0);
}

/// Submits a negotiated loan contract: the loan offer and the person's contract with the agreed share of the wage.
///
/// `TRANSFER_MANAGER::submit_loan_contract_offer(&offer)`: the person (`offer::TARGET_STAFF_ID`) and the club
/// (`offer::BUYER_CLUB_ID`); the loan negotiation item (`transfer_manager_fields::LOAN_NEGOTIATIONS`, a list of 0x50-byte
/// contracts: +0 the person's id, +4 the club's) of the two. With it and the person's contract (`exe::
/// contract_manager_get_contract`): the share of the wage the club pays = item +0xc × 100 / the contract's wage (+0xc),
/// as a signed byte, cut down to tens; then lowered by 10 while above 0 and the club's share (wage × share / 100) is
/// above the club's wage limit (`exe::finance_get_max_wage` with the person and -1); below 0 it is 0. The item's date
/// (+0x2d) becomes two weeks from today (`exe::date_add_weeks`) when it has passed or is less than 7 days away
/// (`exe::date_days_since`). Then a loan offer (`exe::transfer_offer_init_loan_offer`) and a contract from the current
/// one (`exe::staff_contract_init_from_current_contract` with the item's date, the share and the item's bytes +0x1c
/// and +0x1d), both added (`exe::transfer_manager_add_transfer_offer`, `exe::contract_manager_add_contract_offer` with
/// the offer's slot and serial, 0xb, 0, 1, 0), and the local offer destroyed. No item (or no list): the "Error" box at
/// line 0xb11; no contract: line 0xb1a; both mark the offer 0x10 and schedule it.
///
/// The original builds the offer and the contract in stack locals their constructors do not fill whole; the copy
/// starts them zeroed, as `make_loan_offer` does (the residue would go into the stored offer).
/// Original: `TRANSFER_MANAGER::submit_loan_contract_offer` (0x8c0f40, transfer_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn transfer_manager_submit_loan_contract_offer(this: usize, offer_ref: usize) {
    let offer = read_usize(offer_ref);
    let person = staff_at(read_i32(offer + offer::TARGET_STAFF_ID) as isize);
    let club = club_at(read_i32(offer + offer::BUYER_CLUB_ID) as isize);
    let loans = read_usize(this + transfer_manager_fields::LOAN_NEGOTIATIONS);
    let mut item = 0;
    if loans != 0 {
        llist_restart(Llist(loans));
        loop {
            let e = exe::llist_get_next(loans);
            if e == 0 || (read_i32(person.addr()) == read_i32(e) && read_i32(club.addr()) == read_i32(e + loan_negotiation::CLUB_ID)) {
                item = e;
                break;
            }
        }
    }
    if item == 0 {
        return loan_contract_submission_failed(this, offer, 0xb11);
    }
    let contracts = read_usize(G_CONTRACT_MANAGER);
    if exe::contract_manager_get_contract(contracts, person) == 0 {
        return loan_contract_submission_failed(this, offer, 0xb1a);
    }
    covered!("rxg_4 loan contract submitted");
    let wage = read_i32(exe::contract_manager_get_contract(contracts, person) + 0xc); // the contract's wage
    let raw = (read_i32(item + 0xc).wrapping_mul(100) / wage) as i8; // +0xc: the agreed amount
    let mut share = ((raw / 10) as u8).wrapping_mul(10) as i8;
    let limit = exe::finance_get_max_wage(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club), person, -1i32 as u32);
    while share > 0 && read_i32(exe::contract_manager_get_contract(contracts, person) + 0xc).wrapping_mul(share as i32) / 100 > limit {
        share = share.wrapping_sub(10); // rxg_4: ten points less
    }
    let share = share.max(0);
    let date = item + 0x2d; // the item's date (day +0x2d, year +0x2f)
    let today_day = read_i16(G_CURRENT_DATE);
    let today_year = read_u16(G_CURRENT_DATE + 2);
    let passed = today_year > read_u16(date + 2) || (today_year == read_u16(date + 2) && today_day > read_i16(date));
    if passed || exe::date_days_since(date, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE + 4)) < 7 {
        let mut later = [0u8; 8];
        let mut scratch = [0u8; 8];
        let r = exe::date_add_weeks(G_CURRENT_DATE, &mut later, 2);
        exe::date_operator_3d(date, scratch.as_mut_ptr() as usize, r);
    }
    let (b1d, b1c) = (read_i8(item + 0x1d) as i32 as u32, read_i8(item + 0x1c) as i32 as u32);
    let mut o = [0u8; 0x44];
    exe::new_transfer_offer(o.as_mut_ptr() as usize);
    exe::transfer_offer_init_loan_offer(o.as_mut_ptr(), person, club);
    let mut c = [0u8; 0x50];
    exe::new_staff_contract(c.as_mut_ptr() as usize);
    exe::staff_contract_init_from_current_contract(c.as_mut_ptr() as usize, person, club, date as u32, share as u8 as u32, b1c, b1d);
    exe::transfer_manager_add_transfer_offer(this, o.as_mut_ptr(), c.as_mut_ptr());
    let field = |k: usize| u32::from_le_bytes([o[k], o[k + 1], o[k + 2], o[k + 3]]);
    exe::contract_manager_add_contract_offer(contracts, c.as_mut_ptr(), field(offer::SLOT_INDEX) as usize, field(offer::SERIAL) as usize, 0xb, 0, 1, 0);
    exe::drop_transfer_offer(o.as_mut_ptr() as usize);
}

/// Comparer for `transfer_manager_submit_loan_contract_offer`: the original only (it adds the offer and the contract).
pub(crate) unsafe extern "thiscall" fn compare_transfer_manager_submit_loan_contract_offer(this: usize, offer_ref: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, offer_ref)
}
