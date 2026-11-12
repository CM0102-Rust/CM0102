// ---------- batch %%LEVA%%: the contracts' day: talks, values, loans, renewals and the contract end (contract_manager.cpp) ----------

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
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I0%%)(this, date, slice, people) // simplification: original only
}
