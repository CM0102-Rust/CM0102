// ---------- rx07_1: the non-promotion clauses and the division clauses reset (contract_manager.cpp) ----------

/// The person's main contract (the staff state record's first index) when it is with `club`, else a null contract:
/// a null person, an id past `G_STAFF_COUNT`, no contract index or a contract with another club give none.
pub(crate) unsafe fn main_contract_at_club(this: usize, club: Club, p: usize) -> Contract {
    if p == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) {
        return Contract(0);
    }
    let i = read_i32(contract_manager_staff_state_record(read_usize(p)));
    if i < 0 {
        return Contract(0);
    }
    let c = Contract((i as usize).wrapping_mul(80).wrapping_add(read_usize(this)));
    if c.is_null() || c.club() != read_usize(club.addr()) {
        return Contract(0);
    }
    c
}

/// The main contracts with the club and with its linked B team (none for a B club), in squad order.
pub(crate) unsafe fn main_contracts_of_club_and_b_team(this: usize, club: usize) -> Vec<Contract> {
    let mut out = Vec::new();
    for c in club_and_b_team(club) {
        if c != 0 {
            for k in 0..50 {
                let contract = main_contract_at_club(this, Club(c), Club(c).squad(k));
                if !contract.is_null() {
                    out.push(contract);
                }
            }
        }
    }
    out
}

/// Whether the non-promotion clause of contract `c` is due: still to trigger (`.non_promotion_rc()` = 1) and the
/// current date at least 45 days after the contract started (the years compared unsigned, then the days signed).
pub(crate) unsafe fn non_promotion_clause_due(c: Contract) -> bool {
    if c.non_promotion_rc() != 1 {
        return false;
    }
    let mut due = [0u32; 2];
    let r = exe::date_add_days(c.addr() + contract::DATE_STARTED, due.as_mut_ptr() as usize, 0x2d);
    let (year, day) = (read_u16(G_CURRENT_DATE + 2), read_i16(G_CURRENT_DATE));
    let (due_year, due_day) = (read_u16(r + 2), read_i16(r));
    year > due_year || (year == due_year && day >= due_day)
}

/// Rule: the non-promotion clauses of a club that stayed down. At the club and at its linked B team (a B club hands
/// over to its A club: nothing is done), every squad member (`club::SQUAD`, 50 slots) whose main contract is with that
/// club and has a non-promotion clause still to trigger (`.non_promotion_rc()` = 1) that started at least 45 days ago
/// gets the clause triggered (2) and the release clause news is posted
/// (`exe::transfer_manager_post_release_clause_activated_news` over `G_TRANSFER_MANAGER`, with the league and type 0).
/// Original: `CONTRACT_MANAGER::trigger_non_promotion_clauses` (0x4cced0, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn trigger_non_promotion_clauses(this: usize, club: usize, league: usize) {
    for c in club_and_b_team(club) {
        if c == 0 {
            continue;
        }
        for k in 0..50 {
            let p = Club(c).squad(k);
            let contract = main_contract_at_club(this, Club(c), p);
            if !contract.is_null() && non_promotion_clause_due(contract) {
                covered!("rx07_1 non-promotion clause");
                contract.set_non_promotion_rc(2); // the triggered clause
                exe::transfer_manager_post_release_clause_activated_news(read_usize(G_TRANSFER_MANAGER), p, league, 0, contract);
            }
        }
    }
}

/// Comparer for `trigger_non_promotion_clauses`: the plan before (each main contract's clause byte, 2 where due), then
/// only the original (the news goes to the clubs and is not posted twice); every byte must match the plan.
pub(crate) unsafe extern "thiscall" fn compare_trigger_non_promotion_clauses(this: usize, club: usize, league: usize) {
    let plan: Vec<(Contract, u8)> = main_contracts_of_club_and_b_team(this, club).into_iter()
        .map(|c| (c, if non_promotion_clause_due(c) { 2 } else { c.non_promotion_rc() })).collect();
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%)(this, club, league);
    let wrong: Vec<(usize, u8, u8)> = plan.iter().filter(|(c, v)| c.non_promotion_rc() != *v).map(|(c, v)| (c.addr(), *v, c.non_promotion_rc())).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("trigger_non_promotion_clauses({:x}) {} contracts, (contract, expected, original) {:x?}", club, plan.len(), wrong));
}

/// The division clauses each main contract with the club and its B team keeps after the reset: (contract, relegation
/// clause, non-promotion clause), a clause still to trigger (1) going to 0.
pub(crate) unsafe fn division_clauses_after_reset(this: usize, club: usize) -> Vec<(Contract, u8, u8)> {
    let clear = |v: u8| if v == 1 { 0 } else { v };
    main_contracts_of_club_and_b_team(this, club).into_iter().map(|c| (c, clear(c.relegation_rc()), clear(c.non_promotion_rc()))).collect()
}

/// Rule: a club changing division forgets its pending division clauses. With `clear_numbers`, first every squad member
/// (`club::SQUAD`, 50 slots, a valid id) loses his squad number (`exe::squad_manager_set_squad_number` over
/// `G_SQUAD_MANAGER`, number 0); then, at the club and at its linked B team (a B club stops there), every main contract
/// with that club drops a relegation clause still to trigger (`.relegation_rc()` 1 -> 0) and a non-promotion clause
/// still to trigger (`.non_promotion_rc()` 1 -> 0).
/// Original: `CONTRACT_MANAGER::reset_division_clauses` (0x4cd120, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn reset_division_clauses(this: usize, club: usize, clear_numbers: u32) {
    if club == 0 {
        return;
    }
    if clear_numbers != 0 {
        for k in 0..50 {
            let p = Club(club).squad(k);
            if p != 0 && read_i32(p) < read_i32(G_STAFF_COUNT) {
                covered!("rx07_1 squad number cleared");
                exe::squad_manager_set_squad_number(read_usize(G_SQUAD_MANAGER), p, club, 0);
            }
        }
    }
    for (contract, relegation, non_promotion) in division_clauses_after_reset(this, club) {
        covered!("rx07_1 division clauses reset");
        contract.set_relegation_rc(relegation);
        contract.set_non_promotion_rc(non_promotion);
    }
}

/// Comparer for `reset_division_clauses`: the plan before, then only the original (it clears the squad numbers through
/// the squad manager); each main contract's two clause bytes must match the plan.
pub(crate) unsafe extern "thiscall" fn compare_reset_division_clauses(this: usize, club: usize, clear_numbers: u32) {
    let plan = if club == 0 { Vec::new() } else { division_clauses_after_reset(this, club) };
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I1%%)(this, club, clear_numbers);
    let wrong: Vec<(usize, u8, u8, u8, u8)> = plan.iter().filter(|(c, r, n)| c.relegation_rc() != *r || c.non_promotion_rc() != *n)
        .map(|(c, r, n)| (c.addr(), *r, *n, c.relegation_rc(), c.non_promotion_rc())).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("reset_division_clauses({:x}, {}) {} contracts, (contract, expected rel/np, original rel/np) {:x?}", club, clear_numbers, plan.len(), wrong));
}
