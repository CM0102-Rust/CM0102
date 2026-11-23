use super::*;

/// Returns 1 when one of the three release clauses of the contract is active.
///
/// `STAFF_CONTRACT::has_active_release_clause()`: 1 with clause `.non_promotion_rc()` = 2, `.minimum_fee_rc()` = 1 or
/// `.relegation_rc()` = 2; otherwise 0. Called from the contracts' day 0x4c7ac0, the player value `calc_player_value`,
/// the asking price 0x4d2070, the seller's expected answer `shortlist_manager_predict_selling_club_response`, the
/// seller who has to accept `transfer_manager_selling_club_must_accept` and the bid `exe::transfer_offer_build_bid`.
/// TapeCall on every contract: no counter.
/// Original: `STAFF_CONTRACT::has_active_release_clause` (0x867b50, staff_contracts.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn has_active_release_clause(c: usize) -> u32 {
    let c = Contract(c);
    if c.non_promotion_rc() == 2 || c.minimum_fee_rc() == 1 || c.relegation_rc() == 2 {
        return 1; // cpu4: with the clause active
    }
    0
}

/// Checks whether the contract is still in the protected period of the new FIFA rules: 3 years under age 28 at signing,
/// otherwise 2.
///
/// SEH frame only for the local date. `STAFF_CONTRACT::check_fifa_protected_period(_, &in_the_limit, &years)`: clears
/// `in_the_limit` (if not null). Without the new FIFA rules (`G_FIFA_TRANSFER_RULES_IN_FORCE` ≠ 1), 1. A contract (date
/// `contract::DATE_STARTED`) before the rules' date (`G_TRANSFER_RULE_CHANGE_DATE`: unsigned year, signed day), 1.
/// Otherwise the protected period: 3 years with the person's age (`G_STAFF_TABLE` + `staff::SIZE` × `[+0]`) at the
/// contract date (`exe::database_get_staff_age`) below 28, otherwise 2, in `years` (if not null); the duration from
/// today to the date (`exe::date_get_difference`, twice, as in the original) with fewer years than the period, 1; with
/// the exact years and 0 days, `in_the_limit` = 1; 0. Called from the contracts' day 0x4c7ac0, the bid
/// `exe::transfer_offer_build_bid`, the swap with compensation `shortlist_manager_is_compensation_deal` and screens. No
/// counter.
/// Original: `STAFF_CONTRACT::check_fifa_protected_period` (0x868210, staff_contracts.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn check_fifa_protected_period(c: usize, _x: usize, in_the_limit: *mut u32, years: *mut u8) -> u32 {
    let c = Contract(c);
    if !in_the_limit.is_null() {
        *in_the_limit = 0;
    }
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) != 1 {
        return 1; // cpu4: without the new rules
    }
    let (rdia, rano) = (read_i16(G_TRANSFER_RULE_CHANGE_DATE), read_u16(G_TRANSFER_RULE_CHANGE_DATE_YEAR));
    let d = c.addr() + contract::DATE_STARTED;
    let (cdia, cano) = (read_i16(d), read_u16(d + 2));
    if rano > cano || (rano == cano && rdia > cdia) {
        return 1;
    }
    let (d0, d1) = (read_usize(d) as u32, read_usize(d + 4) as u32);
    let mut t = [0u8; 8];
    exe::date_get_difference(G_CURRENT_DATE, t.as_mut_ptr(), d0, d1);
    let person = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(c.addr()).wrapping_mul(staff::SIZE));
    let limit: u8 = if (exe::database_get_staff_age(person, d as *const u16)) < 0x1c { 3 } else { 2 }; // cpu4: the period by age
    if !years.is_null() {
        *years = limit;
    }
    let mut l = [0u8; 8];
    let p = exe::date_get_difference(G_CURRENT_DATE, t.as_mut_ptr(), d0, d1);
    core::ptr::copy_nonoverlapping(p as *const u8, l.as_mut_ptr(), 8);
    let a = u16::from_le_bytes([l[2], l[3]]) as i32;
    if a < limit as i32 {
        return 1; // cpu4: dentro do periodo
    }
    if !in_the_limit.is_null() && a == limit as i32 && u16::from_le_bytes([l[0], l[1]]) == 0 {
        *in_the_limit = 1;
    }
    0
}

/// whether the bonus type applies to the player: type 2 only for goalkeepers, the others only for non-goalkeepers
/// Original: `staff_contract_is_bonus_type_allowed` (0x867aa0, staff_contracts.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_contract_is_bonus_type_allowed(person: usize, kind: u32) -> u32 {
    let person = Staff(person);
    let f = person.player();
    if f.is_null() {
        return 0;
    }
    if (f.goalkeeper() as u8) == 0x14 {
        return (kind as u8 == 2) as u32;
    }
    (kind as u8 != 2) as u32
}

/// The common part of `reset_morale_state` and `init_from_current_contract` (inlined in the latter): clears the contract's happiness, grievances and
/// morale state and the person's opinion links, and sets the nation language level in +0x4f.
///
/// Zeroes the happiness in the contract (+0x44), state +0x4e and the complaints (+0x45); clears the opinion links of
/// the contract's person (`staff_clear_opinion_links` with record [0xae23bc] + id × 0x6e); zeroes +0x40, sets +0x4e to 4, zeroes the
/// happiness (+0x3f) and +0x3e; with the club and its nation (+0x53), the nation's language (+0x77) level for the
/// person (`get_language_fluency` on [0xb74350], without the contract; at most 10) goes in the low 4 bits of +0x4f.
pub(crate) unsafe fn reset_contract_morale(this: Contract, person: usize, club: Club) {
    covered!("cpl_5 morale zeroed");
    this.set_morale((0) as i8);
    this.set_transfer_status(0);
    this.set_grievances((0) as u32);
    exe::staff_clear_opinion_links(read_usize(G_STAFF_TABLE).wrapping_add(read_usize(this.addr()).wrapping_mul(staff::SIZE)));
    this.set_flags((0) as u32);
    this.set_transfer_status(4);
    this.set_matches_left_out(0); // a felicidade
    this.set_unhappy_flag((0) as i8);
    if !club.is_null() && club.nation().addr() != 0 {
        let r = exe::playerstats_get_language_fluency(read_usize(G_PLAYER_STATS), person, club, (club.nation().first_language() as u8) as u32, 0) as i8;
        let c = if r > 10 { 10 } else { r } as u8;
        let b = this.squad_status();
        this.set_squad_status((b & 0xf0) | (c & 0xf));
    }
}

/// clears the contract morale, grievances and happiness, the person's opinion links (`exe::staff_clear_opinion_links`),
/// sets `.set_transfer_status()` to 4 and the club nation language level (`exe::playerstats_get_language_fluency`, up
/// to 10) in `.squad_status()` (through `reset_contract_morale`)
/// Original: `STAFF_CONTRACT::reset_morale_state` (0x866ee0, staff_contracts.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn reset_morale_state(this: usize, person: usize, club: usize) {
    reset_contract_morale(Contract(this), person, Club(club));
}

/// Builds the offer from the current contract: person, club, wage × percent / 100, today, the end date, flags, morale
/// reset as in `reset_morale_state` and the two clauses; without a contract, error box.
///
/// `STAFF_CONTRACT::init_from_current_contract(person, club, end, percentage, without_promotion, multa)`: the transfer
/// or loan proposal from the person's current contract (`exe::contract_manager_get_contract` on `G_CONTRACT_MANAGER`;
/// without a contract, error box 0xf5 and nothing): person id +0, club `.set_club()`, 0 at `.set_unknown()`, wage
/// `.set_wage()` = current wage × `percentage` (signed) / 100, today's date (`G_CURRENT_DATE`) at
/// `contract::DATE_STARTED`, the end at `contract::CONTRACT_EXPIRES`, the bits `.contract_type()` (& 0xc6 | 6), the
/// wage also at `.set_previous_wage()`, 0 at `.set_squad_number()` and `.set_leaving_on_bosman()`, −1 at
/// `.set_transfer_arranged_for()`, the `reset_contract_morale`, 0 at `.set_club_appearances()` (16 bits), +0x1c..=+0x20
/// and `.set_release_fee()`, −1 at `.set_goal_bonus()`/`.set_assist_bonus()`/`.set_clean_sheet_bonus()` and the
/// clauses: `.set_non_promotion_rc()` with `without_promotion`, `.set_minimum_fee_rc()` with `multa` (minimum fee). The
/// date copies (`exe::date_operator_3d`, with no other effect) become 8-byte copies.
/// Original: `STAFF_CONTRACT::init_from_current_contract` (0x85e590, staff_contracts.cpp, thiscall, ret 0x18).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn init_from_current_contract(this: usize, person: usize, club: usize, end: usize, percentage: u32, without_promotion: u32, multa: u32) {
    let this = Contract(this);
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    if c.is_null() {
        show_error_at(0xa8360c, 0xf5);
        return;
    }
    covered!("cpl_5 contract by the current one");
    write_usize(this.addr(), read_usize(person));
    this.set_unknown((0) as u32);
    this.set_club(read_usize(club));
    let salary = ((c.wage() as usize) as i32).wrapping_mul(percentage as u8 as i8 as i32) / 100; // the salary percentage
    this.set_wage((salary as usize) as i32);
    core::ptr::write_unaligned((this.addr() + contract::DATE_STARTED) as *mut [u8; 8], core::ptr::read_unaligned(G_CURRENT_DATE as *const [u8; 8])); // hoje
    core::ptr::write_unaligned((this.addr() + contract::CONTRACT_EXPIRES) as *mut [u8; 8], core::ptr::read_unaligned(end as *const [u8; 8])); // the end of the contract
    let b = (this.contract_type() & 0xc6) | 6;
    this.set_previous_wage(this.wage() as usize);
    this.set_contract_type(b);
    this.set_squad_number(0);
    this.set_leaving_on_bosman(0);
    this.set_transfer_arranged_for(usize::MAX);
    reset_contract_morale(this, person, Club(club));
    this.set_club_appearances((0) as i16);
    for k in 0x1c..=0x20 {
        write_u8(this.addr() + k, 0);
    }
    this.set_goal_bonus((usize::MAX) as i32);
    this.set_assist_bonus((usize::MAX) as i32);
    this.set_clean_sheet_bonus((usize::MAX) as i32);
    this.set_release_fee(0);
    this.set_non_promotion_rc(without_promotion as u8);
    this.set_minimum_fee_rc(multa as u8);
}

/// Returns the 0..=14 band of the contract end date against today and the CONTRACT_MANAGER limit dates
/// (0xae1010..0xae1090).
///
/// `STAFF_CONTRACT::get_expiry_band()`: the 0..=14 band of the contract end (`contract::CONTRACT_EXPIRES`) against
/// today's date (`G_CURRENT_DATE`) and the 16 CONTRACT_MANAGER dates (0xae1010..0xae1090, built at creation): type
/// (`.contract_type()` & 0x3f) 1 or the end already past, 14; type other than 2, 0; after
/// `G_CONTRACT_DATE_PLUS_182_DAYS`, 0; the intervals between
/// `G_CONTRACT_DATE_PLUS_152_DAYS`/`G_CONTRACT_DATE_PLUS_121_DAYS`/`G_CONTRACT_DATE_PLUS_91_DAYS`/`G_CONTRACT_DATE_PLUS_60_DAYS`/`G_CONTRACT_DATE_PLUS_30_DAYS`
/// give the pairs 1-2, 3-4, 5-6, 7-8, 9-10 (the odd one on the exact day of the upper limit:
/// `exe::date_operator_3d_3d`) and, before `G_CONTRACT_DATE_PLUS_30_DAYS`, 11 on the day, 13 on the day of
/// `G_CONTRACT_DATE_PLUS_7_DAYS`, otherwise 12. "After" via `exe::date_operator_3e`. TapeCall on every contract in the
/// daily pass (0x4c7ac0): no counter.
/// Original: `STAFF_CONTRACT::get_expiry_band` (0x867d30, staff_contracts.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_expiry_band(this: usize) -> u8 {
    let this = Contract(this);
    let end = this.addr() + contract::CONTRACT_EXPIRES;
    let m = |g: usize| exe::date_operator_3e(end, read_usize(g) as u32, read_usize(g + 4) as u32) != 0;
    let q = |g: usize| exe::date_operator_3d_3d(end, read_usize(g) as u32, read_usize(g + 4) as u32) != 0;
    if this.contract_type() & 0x3f == 1 {
        return 14;
    }
    if !m(G_CURRENT_DATE) {
        return 14;
    }
    if this.contract_type() & 0x3f != 2 {
        return 0;
    }
    if m(G_CONTRACT_DATE_PLUS_182_DAYS) {
        return 0;
    }
    if m(G_CONTRACT_DATE_PLUS_91_DAYS) { // past the midpoint limit
        if m(G_CONTRACT_DATE_PLUS_121_DAYS) {
            if m(G_CONTRACT_DATE_PLUS_152_DAYS) {
                return if q(G_CONTRACT_DATE_PLUS_182_DAYS) { 1 } else { 2 };
            }
            return if q(G_CONTRACT_DATE_PLUS_152_DAYS) { 3 } else { 4 };
        }
        return if q(G_CONTRACT_DATE_PLUS_121_DAYS) { 5 } else { 6 };
    }
    if m(G_CONTRACT_DATE_PLUS_30_DAYS) {
        if m(G_CONTRACT_DATE_PLUS_60_DAYS) {
            return if q(G_CONTRACT_DATE_PLUS_91_DAYS) { 7 } else { 8 };
        }
        return if q(G_CONTRACT_DATE_PLUS_60_DAYS) { 9 } else { 10 };
    }
    if q(G_CONTRACT_DATE_PLUS_30_DAYS) {
        return 11;
    }
    if q(G_CONTRACT_DATE_PLUS_7_DAYS) { 13 } else { 12 }
}

/// Memory regions the two morale-reset routines write: the contract (0x50 bytes), the person's 79-byte staff state
/// record and 0xb67a34.
pub(crate) unsafe fn morale_reset_regions(this: usize, id: usize) -> Vec<(usize, usize)> {
    let mut r = vec![(this, 0x50), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    let id = read_usize(read_usize(G_STAFF_TABLE).wrapping_add(id.wrapping_mul(staff::SIZE))) as i32;
    if id >= 0 && id < read_usize(G_STAFF_COUNT) as i32 {
        r.push((read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(staff_state::SIZE)), 0x4f));
    }
    r
}

/// Person record (110 bytes in G_STAFF_TABLE) of the contract's id (+0).
pub(crate) unsafe fn staff_of_contract(contract: Contract) -> usize {
    (read_u32(G_STAFF_TABLE) as usize).wrapping_add((read_u32(contract.addr()) as usize).wrapping_mul(staff::SIZE))
}

/// Returns the days from the game date to the contract end date (`contract::CONTRACT_EXPIRES`), through
/// `exe::date_days_since`.
///
/// `STAFF_CONTRACT::days_to_end_date`: the days from the game date (`G_CURRENT_DATE`) to the contract end date
/// (`contract::CONTRACT_EXPIRES`), through `exe::date_days_since` (`dias_ate`, the game date by value). The game reads
/// only the word; it returns `exe::date_days_since`'s whole eax.
/// Original: `STAFF_CONTRACT::days_to_end_date` (0x85fa00, staff_contracts.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn days_to_end_date(this: usize) -> u32 {
    let this = Contract(this);
    exe::date_days_since(this.addr() + contract::CONTRACT_EXPIRES, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) as u32  // the contract end date
}

/// Clears the grievances (`.set_grievances()`), the opinion links (`exe::staff_clear_opinion_links`) and, when asked,
/// checks the withdrawal (`exe::staff_contract_check_transfer_request_withdrawal`).
///
/// `STAFF_CONTRACT::clear_all_grievances(check)`: clears the grievances (`.set_grievances()`), removes the person's
/// opinion links (`exe::staff_clear_opinion_links`) and, with `check`, checks the withdrawal of the transfer request
/// (`exe::staff_contract_check_transfer_request_withdrawal` with 0).
/// Original: `STAFF_CONTRACT::clear_all_grievances` (0x867520, staff_contracts.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn clear_all_grievances(this: usize, check: u32) {
    let this = Contract(this);
    this.set_grievances(0); // the complaints
    exe::staff_clear_opinion_links(staff_of_contract(this));
    if check != 0 {
        exe::staff_contract_check_transfer_request_withdrawal(this, 0);
    }
}

/// The decision of `check_transfer_request_withdrawal` (no effect besides the sheet [0xae1080], written with the person's contract): returns the
/// person when their transfer request should be withdrawn, else None.
///
/// Conditions: the contract's club (outside type 5) exists, the person is a player (+0x61), the contract is with that
/// club (+4 = id), they requested a transfer (bit 3 of +0x4e), not in +0x49, the staff criterion (`staff_criterion`) gives 0,
/// the high nibble of +0x4f is not 7, and there is no agreed contract with another club (`get_agreed_contract_club` on [0xde1f54]).
pub(crate) unsafe fn should_withdraw_transfer_request(this: Contract) -> Option<usize> {
    let person = Staff(staff_of_contract(this));
    let c = Contract(exe::contract_manager_get_contract(read_u32(G_CONTRACT_MANAGER) as usize, person));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr()); // the contract record
    let club = if !c.is_null() && c.contract_type() & 0x3f != 5 {
        let i = (c.club() as u32) as usize;
        (read_u32(G_CLUB_TABLE) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4))
    } else {
        0
    };
    if club == 0 || (person.player().addr() as u32) == 0 || (this.club() as u32) != read_u32(club) {
        return None;
    }
    if this.transfer_status() & 8 == 0 || this.leaving_on_bosman() != 0 {
        return None; // no transfer request
    }
    if exe::staff_contract_wants_to_leave(this) != 0 || this.squad_status() & 0xf0 == 0x70 {
        return None;
    }
    if exe::transfer_manager_get_agreed_contract_club(read_u32(G_TRANSFER_MANAGER) as usize, person, 0) != 0 {
        return None;
    }
    Some(person.addr())
}

/// For a player of the contract's club with a transfer request, no staff criterion and no agreed contract: withdraws
/// the request (`exe::contract_manager_post_transfer_request_withdrawn_news`), morale becomes `staff::PROFESSIONALISM`
/// + `staff::LOYALTY` + 5, `.unhappy_flag()` = 0, `.happiness()` at least 15, and clears each grievance, checking
/// again.
///
/// `STAFF_CONTRACT::check_transfer_request_withdrawal(ctx)`: when the decision (`should_withdraw_transfer_request`)
/// says so, withdraws the request (`exe::contract_manager_post_transfer_request_withdrawn_news` on `G_CONTRACT_MANAGER`
/// with the contract and `ctx`), clears bit 3 of `.transfer_status()`, the morale in the contract (`.set_morale()`)
/// becomes the person's `staff::PROFESSIONALISM` + `staff::LOYALTY` + 5, clears `.unhappy_flag()` and raises
/// `.happiness()` to 15; and each grievance that is set (bits 0x10, 1, 4, 8, 0x100, 0x80, 0x10000 of `.grievances()`,
/// in that order) is cleared, checks again (`exe::staff_contract_check_transfer_request_withdrawal` with the previous
/// `.grievances()`) and removes the opinion link (`exe::staff_remove_opinion_link` with the bit). No SEH, no error box,
/// no random draw.
/// Original: `STAFF_CONTRACT::check_transfer_request_withdrawal` (0x867240, staff_contracts.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn check_transfer_request_withdrawal(this: usize, ctx: u32) {
    let this = Contract(this);
    let Some(pessoa) = should_withdraw_transfer_request(this) else { return };
    exe::contract_manager_post_transfer_request_withdrawn_news(read_u32(G_CONTRACT_MANAGER) as usize, this, ctx);
    this.set_transfer_status(this.transfer_status() & 0xf7); // the request withdrawn
    this.set_morale((read_u8(pessoa + staff::PROFESSIONALISM).wrapping_add(read_u8(pessoa + staff::LOYALTY)).wrapping_add(5)) as i8);
    if (this.unhappy_flag() as u8) != 0 {
        this.set_unhappy_flag((0) as i8);
    }
    if ((this.happiness() as u8) as i8) < 15 {
        this.set_happiness((15) as i8); // the minimum happiness
    }
    for bit in [0x10u32, 1, 4, 8, 0x100, 0x80, 0x10000] {
        let q = this.grievances();
        if q & bit != 0 {
            this.set_grievances(q & !bit); // a queixa apagada
            exe::staff_contract_check_transfer_request_withdrawal(this, q);
            exe::staff_remove_opinion_link(staff_of_contract(this), bit as i32);
        }
    }
}

// 0x54cdb0 only reads: compare the value
compare_value!(compare_days_to_end_date, days_to_end_date, IDX_DAYS_TO_END_DATE, () -> u32);

/// Sets the flags at `.flags()` and, when the person now wants to leave and nothing prevents it, files the transfer
/// request (`exe::contract_manager_handle_transfer_request`).
///
/// `STAFF_CONTRACT::set_flags(marks)`, called by many routines (in motor.rs through its address; before, only the
/// post-match gate detoured it, in comparison mode). Flags above 0x1000: the "Error" box and it carries on (here: the
/// whole original). Sets the flags in `.flags()`; a person (+0 id) who is not of functions (`.job_for_club()`) 0xc, 0xd
/// and 0xf, a contract with `.morale()` positive, without `.leaving_on_bosman()`, who wants to leave
/// (`exe::staff_contract_wants_to_leave`), without a future transfer (`exe::transfer_manager_get_future_transfer` with
/// 0), without bits 1 and 8 of `.transfer_status()`, outside the free transfer queue
/// (`exe::transfer_manager_is_queued_for_free_transfer`) and older than 20 (`.age()`) or with current reputation (the
/// player's `.current_reputation()`) above 4250: the transfer request (`exe::contract_manager_handle_transfer_request`
/// on `G_CONTRACT_MANAGER`). Does not read uninitialised locals.
/// Original: `STAFF_CONTRACT::set_flags` (0x867560, staff_contracts.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn set_flags(c: usize, marks: u32) {
    let c = Contract(c);
    if marks as i32 > 0x1000 {
        // simplification: the ”Error” box (before any write): the whole original
        return original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SET_FLAGS)(c.addr(), marks);
    }
    c.set_flags((c.flags() as usize) as u32 | marks);
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add((read_i32(c.addr()) as usize).wrapping_mul(staff::SIZE)));
    let function = person.job_for_club();
    if function == 0xc || function == 0xd || function == 0xf {
        return;
    }
    let morale = (c.morale() as u8) as i8;
    if morale == 0 || morale < 0 || c.leaving_on_bosman() != 0 {
        return;
    }
    if exe::staff_contract_wants_to_leave(c) == 0 {
        return;
    }
    if exe::transfer_manager_get_future_transfer(read_usize(G_TRANSFER_MANAGER), person, 0) != 0 || c.transfer_status() & 9 != 0 {
        return;
    }
    if exe::transfer_manager_is_queued_for_free_transfer(read_usize(G_TRANSFER_MANAGER), person) != 0 {
        return;
    }
    if (person.age() as u8) as i8 <= 0x14 && person.player().current_reputation() <= 0x109a {
        return;
    }
    covered!("cpw_7 transfer request");
    if MISS_TRAINING_DRY_RUN.load(Relaxed) {
        MISS_TRAINING_DECISION.store(1, Relaxed);
        return;
    }
    exe::contract_manager_handle_transfer_request(read_usize(G_CONTRACT_MANAGER), c);
}

/// Returns 1 to 3 from 100 × the value of the current contract / that of the desired one (with the deserved squad
/// status), with an allowance for recent signings; below 50: 1; below 80: 3; otherwise 2.
///
/// SEH frame with no destructor. `STAFF_CONTRACT::get_contract_satisfaction(person)`, called from the staff's daily
/// pass (0x5ce2d0, with random(50) zero) and from the contract screen: 1 to 3. No person, 1. The squad status they
/// deserve (`FN_CONTRACT_MANAGER_GET_DESERVED_SQUAD_STATUS`) is used if it is lower than the contract's
/// (`contract::SQUAD_STATUS` >> 4) and is neither 0 nor 7; otherwise, the contract's. The desired contract
/// (`exe::staff_contract_set_to_desired_contract` into a local contract, with the contract's club, −1, the status and
/// 0xb) and the ratio 100 × value of the current contract (`calc_contract_value`, as f32) / value of the desired one
/// (zero value: ratio 0). Signed less than 15 × loyalty (`.loyalty()`) + 100 days ago (from `contract::DATE_STARTED`,
/// ceiling 500): ratio + 50 + 7.5 × loyalty − days / 2. Ratio below 50: 1; below 80: 3; otherwise 2.
/// Original: `get_contract_satisfaction` (0x866620, staff_contracts.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_contract_satisfaction(this: usize, person: usize) -> u8 {
    let person = Staff(person);
    if person.is_null() {
        return 1;
    }
    let category: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_DESERVED_SQUAD_STATUS);
    // the local contract (0x50 bytes) as the original leaves it before 0x85fc50; it fills in the rest
    let mut local = [0u8; 0x50];
    let l = local.as_mut_ptr() as usize;
    exe::date_date_54a110(l + contract::DATE_STARTED);
    exe::date_date_54a110(l + contract::CONTRACT_EXPIRES);
    write_i32(l + contract::CLUB, -1);
    write_i32(l + contract::TRANSFER_ARRANGED_FOR, -1);
    let merece = category(person.addr(), this);
    let current = read_u8(this + contract::SQUAD_STATUS) >> 4;
    // simplification: the original pushes the whole dword of the slot where it stored the byte (in the first case, the rest is the pointer
    // of the person; in the second, stack leftover); 0x85fc50 only reads the byte.
    let status = if (merece as i8) < (current as i8) && merece != 0 && merece != 7 {
        (person.addr() as u32 & !0xff) | merece as u32
    } else {
        current as u32
    };
    exe::staff_contract_set_to_desired_contract(l, person, friendly_club_by_id(read_usize(this + contract::CLUB)), -1, status, 0xb, 0, 0);
    let v1 = calc_contract_value(this, person, l) as f32;
    let v2 = calc_contract_value(l, person, l) as f64;
    let mut ratio = if v2 == read_f32(rdata::F32_0) as f64 {
        read_f32(rdata::F32_0) as f64
    } else {
        v1 as f64 * read_f32(rdata::F32_100) as f64 / v2
    };
    let years = (read_i16(G_CURRENT_DATE_YEAR) as u16 as i32).wrapping_sub(read_i16(this + contract::DATE_STARTED_YEAR) as u16 as i32);
    let days = years.wrapping_mul(365).wrapping_sub(read_i16(this + contract::DATE_STARTED) as i32).wrapping_add(read_i16(G_CURRENT_DATE) as i32).clamp(0, 500);
    let adapt = (person.loyalty() as u8) as i8 as i32;
    if days < adapt.wrapping_mul(15).wrapping_add(100) {
        covered!("cpw_1 recently signed");
        ratio = ratio + read_f32(rdata::F32_50) as f64 + adapt as f64 * read_f64(rdata::F64_7_5) - (days / 2) as f64; // cpw1: the discount of the newly signed;
    }
    covered!("cpw_1 satisfaction evaluated");
    if !(ratio >= read_f32(rdata::F32_50) as f64) { // cpw1: the dissatisfaction limit;
        return 1;
    }
    if ratio < read_f32(rdata::F32_80) as f64 { // cpw1: the satisfaction limit;
        3
    } else {
        2
    }
}

/// Sets the wage (`.set_wage()`) to the default for the person at the club
/// (`exe::staff_contract_calc_unset_default_wage`), truncated, minimum 55.
///
/// `STAFF_CONTRACT::set_default_wage(person)`: the wage (`.set_wage()`) is the person's default at their club
/// (`.club_job()`) from `exe::staff_contract_calc_unset_default_wage` (stdcall, 6 arguments, float; ecx goes along, as
/// in the original), truncated (`ftol`), with a minimum of 55. Returns the truncated value (before the minimum).
/// Original: `STAFF_CONTRACT::set_default_wage` (0x8626e0, staff_contracts.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn set_default_wage(this: usize, person: usize) -> u32 {
    let this = Contract(this);
    let person = Staff(person);
    let v = ftol(exe::staff_contract_calc_unset_default_wage(this, person, person.club_job(), 1, -1, 0xb, 0));
    this.set_wage(if v < 0x37 { 0x37 } else { v }); // the minimum of 55
    v as u32
}

/// `.flags()` (a 4-byte routine; the hook covers the first alignment nop)
/// Original: `contract_flags` (0x8676f0, staff_contracts.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn contract_flags(this: usize) -> u32 {
    let this = Contract(this);
    this.flags()
}

/// the empty contract: default dates at `contract::DATE_STARTED` and `contract::CONTRACT_EXPIRES`, `.set_club()` and
/// `.set_transfer_arranged_for()` = -1, `.set_unknown()`, `.set_club_appearances()`, `.set_flags()`,
/// `.set_grievances()`, `.set_leaving_on_bosman()` and `.set_transfer_status()` = 0
/// Original: `STAFF_CONTRACT::STAFF_CONTRACT` (0x85e410, staff_contracts.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn construct_staff_contract(this: usize) -> usize {
    let this = Contract(this);
    exe::date_date_54a110(this.addr() + contract::DATE_STARTED);
    exe::date_date_54a110(this.addr() + contract::CONTRACT_EXPIRES);
    this.set_club((u32::MAX) as usize);
    this.set_transfer_arranged_for((u32::MAX) as usize);
    this.set_leaving_on_bosman(0);
    this.set_club_appearances((0) as i16);
    this.set_transfer_status(0);
    this.set_flags(0);
    this.set_grievances(0);
    this.set_unknown(0);
    this.addr()
}

/// The staff criterion (whether the person wants to leave): an 8-bit score (id mod 7, personality ratings, morale, the
/// player's reputation, number of grievance groups) decides, by the grievances, whether it returns 1.
///
/// The criterion of contract `t` (person `[t]` in `G_STAFF_TABLE`, `staff::SIZE` bytes; club `.club()` in
/// `G_CLUB_TABLE`, `club::SIZE` bytes; grievance bits `.grievances()`): 0 if the person is the club's manager or
/// assistant (`.manager()`, `.assistant_manager()`) or has no grievance; otherwise an 8-bit score (id mod 7 +
/// `staff::LOYALTY` + `staff::PRESSURE` + `staff::PROFESSIONALISM`, deductions for low ratings and for a
/// `staff::TEMPERAMENT` of 1, `.morale()` in bands, deductions for the player's `.current_reputation()` above 5750 and
/// 7250, and, with more than one grievance group set, deductions by the number of groups) decides, by the grievances,
/// the ratings and `.transfer_status()`, whether it returns 1.
/// Original: `staff_criterion` (0x866f60, staff_contracts.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn staff_criterion(t: usize) -> u32 {
    let t = Contract(t);
    let p = Staff(read_usize(G_STAFF_TABLE).wrapping_add((read_i32(t.addr()) as usize).wrapping_mul(staff::SIZE)));
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add(((t.club() as i32) as usize).wrapping_mul(club::SIZE)));
    let _u = |a: usize| read_usize(a);
    let manager = club.manager().addr();
    if manager != 0 && p.addr() == manager {
        return 0;
    }
    let auxiliar = club.assistant_manager();
    if auxiliar != 0 && p.addr() == auxiliar {
        return 0;
    }
    let functions = (t.grievances() as usize) as u32;
    if functions == 0 {
        return 0;
    }
    let b = |o: usize| read_i8(p.addr() + o);
    let (b59, b5a, b5b, b5d) = (b(staff::LOYALTY), b(staff::PRESSURE), b(staff::PROFESSIONALISM), b(staff::TEMPERAMENT));
    let b44 = t.morale();
    let mut rating = ((read_i32(p.addr()) % 7) as u8).wrapping_add(b59 as u8).wrapping_add(b5a as u8).wrapping_add(b5b as u8);
    if b5a == 1 {
        rating = rating.wrapping_sub(2);
    }
    if b59 == 1 {
        rating = rating.wrapping_sub(3);
    } else if b59 < 6 {
        rating = rating.wrapping_sub(1);
    }
    if b5b == 1 {
        rating = rating.wrapping_sub(3);
    } else if b5b < 6 {
        rating = rating.wrapping_sub(1);
    }
    if b5d == 1 {
        rating = rating.wrapping_sub(2);
    }
    if b44 > 0x4b {
        rating = rating.wrapping_add(10);
    } else if b44 > 0x32 {
        rating = rating.wrapping_add(5);
    } else if b44 > 0 {
        rating = rating.wrapping_add(2);
    }
    let sheet = p.player();
    if !sheet.is_null() {
        let rep = sheet.current_reputation();
        if rep > 0x1c52 {
            rating = rating.wrapping_sub(5);
        } else if rep > 0x1676 {
            rating = rating.wrapping_sub(3);
        }
    }
    let tecnica = t.flags() & 8 != 0;
    let m1 = functions & 0x1000002;
    let mut k = 0u8;
    for bits in [0x20000u32, 0x40000, 0x1000002, 0x200600, 0x2840, 0x14] {
        if functions & bits != 0 {
            k += 1;
        }
    }
    if tecnica || functions & 0x4582208 != 0 {
        k += 1;
    }
    if k > 1 {
        rating = rating.wrapping_sub(3);
        if b5b < 6 {
            rating = rating.wrapping_sub(k);
        }
        if b5d < 6 {
            rating = rating.wrapping_sub(k);
        }
        if b59 == 1 {
            rating = rating.wrapping_add((k as i8).wrapping_mul(-2) as u8);
        } else if b59 < 6 {
            rating = rating.wrapping_sub(k);
        }
        if b5a < 6 {
            rating = rating.wrapping_sub(k);
        }
    }
    let rating = rating as i8;
    let yes = (b59 < 15 && t.transfer_status() & 1 != 0)
        || (m1 != 0 && b59 < 10 && rating < 25)
        || (functions & 0x20020 != 0 && rating < 25)
        || (functions & 0x200 != 0 && rating < 18)
        || (functions & 4 != 0 && b(staff::AMBITION) > 15 && b5b < 10)
        || (functions & 0x2808 != 0 && b59 < 10 && b5a < 6 && rating < 30)
        || ((functions & 0x82200 != 0 || tecnica) && b5b < 10 && rating < 20)
        || functions & 0x111 != 0
        || (b44 as i32) < -0x46 - b59 as i32;
    yes as u32
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
        let nation = job_club.nation().addr();
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
    let sheet = p.player();
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
    let caps = p.int_apps();
    if caps != 0 {
        let nation = p.nation().addr();
        let add: i8 = if nation == 0 {
            1
        } else {
            let std = read_i8(nation + nation::LEAGUE_STANDARD) as f64 * read_f32(F32_0_25) as f64;
            let a = ftol(std * (caps as f64 * read_f32(F32_0_125) as f64)) as i8;
            if a > 4 { 4 } else if a == 0 { 1 } else { a }
        };
        level = level.wrapping_add(add);
    }
    let rep: i16 = if !sheet.is_null() {
        sheet.current_reputation()
    } else if !p.non_player().is_null() {
        read_i16(p.non_player().addr() + non_player::CURRENT_REPUTATION)
    } else {
        0 // the original reads an uninitialised local
    };
    let wage_f: f32;
    let wage: i32;
    let db_wage = p.wage() as i32;
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
        } else if wage_f > read_f32(F32_21250) && !sheet.is_null() && sheet.defender() == 20 {
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
    if !sheet.is_null() {
        write_u8(c + contract::MANAGER_JOB_RC, 0);
        let age = p.age() as i8;
        let id = read_i32(p.addr());
        let mut manager_job = false;
        if age > 0x20 && rep > 0xcb2 && id % 3 == 0 {
            let s = p.professionalism() as i32 + p.determination() as i32 + p.ambition() as i32 + id % 10 + age as i32 - 0x1c;
            manager_job = s > 0x2d;
        }
        if manager_job || !p.non_player().is_null() {
            covered!("mke_3 manager job clause");
            write_u8(c + contract::MINIMUM_FEE_RC, 0);
            write_u8(c + contract::MANAGER_JOB_RC, 1);
        }
        let amb = p.ambition() as i32;
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
        let goal = if sheet.attacker() >= 15 { initial_contract_bonus(wage) } else { -1 };
        write_i32(c + contract::GOAL_BONUS, goal);
        let assist = if sheet.midfielder() >= 15 || sheet.attacking_midfielder() >= 15 || sheet.attacker() >= 15 {
            initial_contract_bonus(wage)
        } else {
            -1
        };
        write_i32(c + contract::ASSIST_BONUS, assist);
        let clean = if sheet.goalkeeper() == 20 { initial_contract_bonus(wage) } else { -1 };
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

/// The value of contract `this` for `person`, against `offer` (another contract, usually the desired one): the wage
/// with the bonuses, the spread fee and the release clauses, times a length factor; used by the contract
/// satisfaction (`get_contract_satisfaction`) and by the decision on contract offers (0x4ce4e0).
///
/// The local contract is a copy of `offer` or, without an offer, the person's desired contract at the club of `this`
/// (`exe::staff_contract_set_to_desired_contract` with −1, −1 and 0xb); only its end year is read afterwards. Fewer
/// than 7 days to the end of `this` (`exe::date_days_since` of `contract::CONTRACT_EXPIRES`, a word): 0. The value
/// starts at the wage (`contract::WAGE`). With a player sheet (`.player()`), by the position mask
/// (`exe::formation_get_position_mask`): goalkeeper (bit 1) + clean sheet bonus × 0.8 (band 33); defender (6) + goal
/// bonus × 0.3 + assist bonus × 0.2 (band 31); midfielder (0x38) × 0.4 and × 0.7 (band 29); attacker (0x40) × 0.8 and
/// × 0.5 (band 28); a bonus of −1 does not count (the last matching group sets the band). Plus `contract::UNKNOWN` /
/// (days × 0.2222222). A sheet with world reputation (`.world_reputation()`) above 4000 and reputation² × 0.01 above
/// `contract::UNKNOWN`: × 0.5. Each active release clause of `this` (`contract::NON_PROMOTION_RC` + k above 0)
/// multiplies the value (`contract_value_clause_factor`). The length factor: a = band − age (no sheet: the age),
/// within −5..5; base 0.75 when the offer has a minimum fee clause, 0.875 when a > 0 and the offer has a relegation or
/// non-promotion clause, otherwise 1; with a < 0 and a sheet, × (1 − a × 0.01); raised (`game_pow`) to |end year of
/// `this` − end year of the local contract|, within 0.5..2. The value is divided by it when `this` ends in an earlier
/// year, otherwise multiplied; truncated (`ftol`). Defects of the original: without an offer it reads the clauses of
/// the offer through a null pointer (every caller passes one); without a sheet the age stands for "band − age".
/// Original: `STAFF_CONTRACT::calc_contract_value` (0x866830, staff_contracts.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn staff_contract_calc_contract_value(this: usize, person: usize, offer: usize) -> i32 {
    use contract_value_rdata::*;
    let c = Contract(this);
    let p = Staff(person);
    let o = Contract(offer);
    // the local STAFF_CONTRACT as the original sets it up
    let mut local = [0u8; 0x50];
    let l = local.as_mut_ptr() as usize;
    exe::date_date_54a110(l + contract::DATE_STARTED);
    exe::date_date_54a110(l + contract::CONTRACT_EXPIRES);
    write_i32(l + contract::CLUB, -1);
    write_i32(l + contract::TRANSFER_ARRANGED_FOR, -1);
    if offer == 0 {
        // 0x85fc50: push 0; push 0; push 0xb; push -1; push -1; push club; push person; lea ecx, local -> thiscall, ret 0x1c
        let club = read_usize(G_CLUB_TABLE).wrapping_add(c.club().wrapping_mul(club::SIZE));
        exe::staff_contract_set_to_desired_contract(l, p, club, -1, u32::MAX, 0xb, 0, 0);
    } else {
        // the original copies the offer field by field (the two dates through Date::operator=): the same 0x50 bytes
        core::ptr::copy_nonoverlapping(offer as *const u8, l as *mut u8, 0x50);
    }
    let days = exe::date_days_since(this + contract::CONTRACT_EXPIRES, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) as i16;
    if days < 7 {
        return 0;
    }
    covered!("mkd_1 contract value");
    let age = p.age() as i8;
    let wage = c.wage();
    let mut v32 = wage as f32; // [esp+0x84]
    let sheet = p.player();
    let mut band: i8 = -1;
    let mut st: f64;
    if !sheet.is_null() {
        let mask = exe::formation_get_position_mask(sheet) as u8;
        st = v32 as f64;
        if mask & 1 != 0 {
            let cs = c.clean_sheet_bonus();
            if cs > -1 {
                st = cs as f64 * read_f64(rdata::F64_0_8) + v32 as f64;
            }
            band = 0x21;
        }
        let bonus = |st: f64, kg: usize, ka: usize| {
            let mut s = st;
            if c.goal_bonus() > -1 {
                s += c.goal_bonus() as f64 * read_f64(kg);
            }
            if c.assist_bonus() > -1 {
                s += c.assist_bonus() as f64 * read_f64(ka);
            }
            s
        };
        if mask & 6 != 0 {
            st = bonus(st, rdata::F64_0_3, rdata::F64_0_2);
            band = 0x1f;
        }
        if mask & 0x38 != 0 {
            st = bonus(st, rdata::F64_0_4, rdata::F64_0_7);
            band = 0x1d;
        }
        if mask & 0x40 != 0 {
            st = bonus(st, rdata::F64_0_8, rdata::F64_0_5); // mkd1: the attacker's bonuses
            band = 0x1c;
        }
    } else {
        st = wage as f64; // the exact integer (the original keeps the fild on the stack)
    }
    let spread = read_i32(this + contract::UNKNOWN);
    st = st + spread as f64 / (days as i32 as f64 * read_f32(F32_0_2222222239) as f64); // mkd1: the spread fee
    v32 = st as f32;
    if !sheet.is_null() && sheet.world_reputation() > 4000 {
        let rep = sheet.world_reputation() as i32;
        st = v32 as f64;
        if game_pow(rep as f64, read_f64(rdata::F64_2)) * read_f64(rdata::F64_0_01) > spread as f64 {
            st *= read_f32(rdata::F32_0_5) as f64; // the original also stores it in the slot the clause loop then reuses
        }
    }
    // the release clauses of this contract
    let cf = read_i32(this + contract::RELEASE_FEE);
    for k in 0..5usize {
        if read_i8(this + contract::NON_PROMOTION_RC + k) <= 0 {
            continue;
        }
        covered!("mkd_1 release clause");
        st *= contract_value_clause_factor(k, o, cf, age);
    }
    v32 = st as f32;
    // the length factor
    let years = (read_u16(this + contract::CONTRACT_EXPIRES_YEAR) as i32 - read_u16(l + contract::CONTRACT_EXPIRES_YEAR) as i32) as f32;
    let mut a: i8 = if band != -1 { band.wrapping_sub(age) } else { age };
    a = a.clamp(-5, 5);
    let mut f: f32 = 1.0;
    if o.minimum_fee_rc() as i8 > 0 {
        f = 0.75;
    } else if a > 0 && (o.relegation_rc() as i8 > 0 || o.non_promotion_rc() as i8 > 0) {
        f = 0.875;
    }
    if a < 0 && !sheet.is_null() {
        covered!("mkd_1 older than the band");
        f = ((read_f64(rdata::F64_1) - a as i32 as f64 * read_f64(rdata::F64_0_01)) * f as f64) as f32;
    }
    let mut q = game_pow(f as f64, (years as f64).abs()); // mkd1: the length factor
    if q < read_f64(rdata::F64_0_5) {
        q = read_f64(rdata::F64_0_5);
    } else if q > read_f64(rdata::F64_2) {
        q = read_f64(rdata::F64_2);
    }
    ftol(if years < read_f32(rdata::F32_0) { v32 as f64 / q } else { q * v32 as f64 })
}

/// The default wage (a float, per week) of `person` at `club`: what the person is worth as a wage there, before the
/// will and the negotiation. Called by the desired contract (`set_to_desired_contract`, three times), the will to join
/// (`staff_interest_in_joining_club`, the wage gate), the staff's will (0x844780), the bids (0x842e10, 0x83f3a0), the
/// wage bill (`update_wage_bill`) and 0x85e960.
///
/// First the person's contract (`exe::contract_manager_get_contract`, kept in `G_CONTRACT_LOOKUP_RESULT`) and its club
/// (`cur`, none for a contract of type 5). The level: with a club, reputation / 500 + 1 (a byte), and with a current
/// club and a club of professional status 2, minus one within 1..4; without a club, the world reputation of the player
/// (or non-player) sheet / 500, at least 1. The job: the club job (`job_for_club`) when there is a current club, else
/// the national job, else the club job, else 11 for a player, else the national job, else 9.
///
/// Staff jobs (5..10): the level rises to the current club's (reputation / 500 + 1) or, for a non-player with a
/// national job other than 11, to the national team's; the base is the non-player current reputation³ × k1 or, without
/// a non-player sheet, (id / staff count × 4 + 1) × k2 × 0.0001; the wage = level × base × 0.25, then per job: 5 at
/// least 750 and above 25000 → 150000; 6 and 7 at least 500; 8 within 275..5000; 9 within 250..1000; 10 within 200..1000.
/// For jobs 6, 7 and 8 with a current club other than `club`, 150000 (the marker of "not available") when the current
/// club is more than three levels above (job 6: also unless the asked job is 11 or 6; then below 2500 × 1.35), and for
/// 6 and 7 without that, above 25000 → 150000 unless the asked job is 11 or 6. Job 14 and the others: 1000.
///
/// Players (jobs 11, 12, 13, 15): without a player sheet the error message goes to the text buffer and the wage is a
/// draw `rand() × 4 / 32768 × 550`. With a sheet: an unknown squad status (−1) is worked out from the current
/// reputation against the club's reputation and the age (1 star, 2 important, 4 squad, 5 or 6 by the potential); the
/// base by `kind` (3: 85000; 1: 92500 with loyalty < 15 and pressure > 15, else 75000; 2: 75000 from 23 to 31 years,
/// else 82500), for an old (36+) coach asked as job 13 or 15 10000, else by the squad status (5 or 6: 9350 below 2500
/// of reputation, 11000 below 6250) and the best position (attacker, then midfielders, goalkeeper, defenders, with the
/// reputation, age and loyalty bands of the table); a young (19..31) player not tied to the club or its nation whose
/// world reputation passes (loyalty + 230) × 25 − ambition × 10 rises to 75000 / 92500 / 105000; with the FIFA rules,
/// above 11000 at a club of a major nation × 1.15 / 1.125 / 1.1 (current reputation above 6750, by loyalty) or × 1.025
/// (above 4250). Then the reputation sum u = (current + home + world + 4 × max + 50 × (ability + potential)) / 9 +
/// (id / staff count × 8 − 4), at least 1500 or × 1.05 for a foreigner (another region than the club's and the current
/// club's), at least 1; the wage = base × u² × 1e-8, at least 175; × 0.75 or × 0.5 by the state of development of the
/// nation; × the world reputation against the club's level (0.85..1.15); the level caps (2500, 2750 per level below 10
/// and 15; from 15 on, 10% above 5250 / 7500 per level in England, Spain and Italy and 3250 elsewhere); 20% above the
/// position cap (150000 attacker, 125000 midfielders, 100000 defender, 80000; × 1.1 with the FIFA rules at a major
/// nation); and, without a club job and unless he is an old (33+) international with little drive, a long-time member
/// (id % 15 ≠ 0) at a club of reputation up to 4500, × (1.375 − days at the club (at most 250, + 60 in season, 20% above
/// 30) × 0.0125), at least 0.2.
///
/// With a club: for players, the club's reputation^6 (× 0.00017 / 0.00012 / 0.0001) gives a floor by squad status
/// (the wage goes half-way up to 3 × reputation × it × 0.6..1.5 for stars, ..., × 0.02 for status 6), and a contract
/// with grievance 0x10 multiplies by 0.67 / 0.85 / 2.5 / 3.5 / 7.5 by status; then the club's nation (when the person
/// works there or is from there): Japan at least level² × 10 and 25% above 7500; England, Italy, Spain nothing; the
/// United States × 0.75 and caps by the league (MLS 250) and by age and origin (1250 / 2750 / 3250); South Korea,
/// Scotland, Germany, France, Holland, Sweden, Denmark, Norway, Belgium, Portugal, Croatia, Poland, Brazil, Argentina
/// × a factor by club reputation / 50 against the league standard × 10, with their own caps; Wales, Finland, Ireland,
/// Northern Ireland, Australia only caps; the others × 0.3 and 25% above 15000. For players: at most the wage of the
/// current club's contract with squad status 7 (when the current club is another); with `flag` or without a current
/// club (and current reputation below 2250) at most the club's maximum wage (`exe::finance_get_max_wage`; without
/// finances the wage is returned as it is); without `flag` and with a current club, an ambitious (> 10), not loyal
/// (< 6), reputed (> 6750) midfielder or attacker (a 20 in attacker, attacking midfielder or midfielder) over 20 gets
/// × 1.1..1.2 when the wage × 1.25 is below the maximum. At last the minimum: at a club of professional status 1, 75
/// under 20 years, else 100; elsewhere 75 at clubs of reputation above 2250, else 50.
///
/// Defects of the original: the third test of `kind` compares the byte with 0x14244 (never true: the 6500 floor never
/// applies); without a club, a player with a nation (or of level 15 or more) reads the club through a null pointer (no
/// caller passes it).
/// Original: `STAFF_CONTRACT::default_staff_contract` (0x864550, staff_contracts.cpp, stdcall, ret 0x18).
pub(crate) unsafe extern "stdcall" fn staff_contract_default_staff_contract(person: usize, club: usize, flag: u32, status: u32, job: u32, kind: u32) -> f32 {
    let person = Staff(person);
    let club = Club(club);
    use default_wage_rdata::*;
    let f = |a: usize| read_f32(a) as f64;
    let d = |a: usize| read_f64(a);
    let p = person;
    let c = club;
    let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    write_usize(G_CONTRACT_LOOKUP_RESULT, contract.addr());
    let cur: usize = if contract.is_null() || contract.contract_type() & 0x3f == 5 {
        0
    } else {
        read_usize(G_CLUB_TABLE).wrapping_add(contract.club().wrapping_mul(club::SIZE))
    };
    let id = read_i32(person.addr());
    let age = p.age() as i8;
    let sheet = person.player();
    let np = person.non_player().addr();
    let frac = (id as f64 / read_i32(G_STAFF_COUNT) as f64) as f32;
    let rep_of = |cl: usize| read_i16(cl + club::REPUTATION) as i32;
    let mut status = status;
    // the level
    let mut level: i32;
    if !club.is_null() {
        level = ((rep_of(club.addr()) / 500) as u8).wrapping_add(1) as i8 as i32;
        if cur != 0 && c.professional_status() == 2 {
            level -= 1;
            if level < 1 {
                level = 1;
            } else if level > 4 {
                level = 4;
            }
        }
    } else if !sheet.is_null() || np != 0 {
        let q = if !sheet.is_null() { sheet.world_reputation() as i32 / 500 } else { read_i16(np + non_player::WORLD_REPUTATION) as i32 / 500 };
        level = (q.max(1) as u8) as i8 as i32;
    } else {
        level = 1;
    }
    // the job
    let national = person.national_job().addr();
    let job_nation = p.job_for_nation() as i8;
    let job_club = p.job_for_club() as i8;
    let job_sel: i8 = if cur != 0 && job_club != 0 {
        job_club
    } else if national != 0 && job_nation != 0 {
        job_nation
    } else if job_club != 0 {
        job_club
    } else if !sheet.is_null() {
        0xb
    } else if job_nation != 0 {
        job_nation
    } else {
        9
    };
    let asked_job = job as u8 as i8;
    // staff jobs: the level rises to the current club's or the national team's
    let raise = |level: &mut i32| {
        if cur != 0 {
            let lv = rep_of(cur) / 500 + 1;
            if lv > *level {
                *level = lv;
                return;
            }
        }
        if np != 0 && national != 0 && job_nation != 0xb {
            let lv = rep_of(national) / 500 + 1;
            if lv > *level {
                *level = lv;
            }
        }
    };
    let staff_wage = |level: i32, k1: usize, k2: usize| -> f32 {
        let base = if np != 0 {
            let t = read_i16(np + non_player::CURRENT_REPUTATION) as f64;
            t * (t * t) * d(k1)
        } else {
            (frac as f64 * f(rdata::F32_4) + f(rdata::F32_1)) * f(k2) * d(rdata::F64_0_0001)
        };
        (level as f64 * base * d(rdata::F64_0_25)) as f32
    };
    let unavailable = 150000.0f32;
    let mut value: f32;
    match job_sel as i32 - 5 {
        0 => {
            covered!("mkd_2 staff wage");
            raise(&mut level);
            value = staff_wage(level, F64_2_5ENEG_08, F32_25000);
            if (value as f64) < f(F32_750) {
                value = 750.0;
            } else if value as f64 > f(F32_25000) {
                value = unavailable;
            }
        }
        1 | 2 => {
            covered!("mkd_2 staff wage");
            raise(&mut level);
            value = staff_wage(level, rdata::F64_1ENEG_08, rdata::F32_10000);
            if (value as f64) < f(rdata::F32_500) {
                value = 500.0;
            }
            if cur != 0 && cur != club.addr() {
                let above = rep_of(cur) / 500 + 4 > level;
                if job_sel == 6 {
                    if (asked_job == 0xb || asked_job == 6) && !above {
                        if (value as f64) < f(rdata::F32_2500) {
                            value = (value as f64 * d(rdata::F64_1_35)) as f32;
                        }
                    } else {
                        value = unavailable;
                    }
                } else if above {
                    value = unavailable;
                } else if asked_job != 0xb && asked_job != 6 && value as f64 > f(F32_25000) {
                    value = unavailable;
                }
            } else if asked_job != 0xb && asked_job != 6 && value as f64 > f(F32_25000) {
                value = unavailable;
            }
        }
        3 => {
            covered!("mkd_2 staff wage");
            raise(&mut level);
            value = staff_wage(level, F64_5ENEG_09, rdata::F32_5000);
            if (value as f64) < f(F32_275) {
                value = 275.0;
            } else if value as f64 > f(rdata::F32_5000) {
                value = 5000.0;
            }
            if cur != 0 && cur != club.addr() && asked_job != 6 && asked_job != 0xb && rep_of(cur) / 500 + 4 > level {
                value = unavailable;
            }
        }
        4 | 5 => {
            covered!("mkd_2 staff wage");
            raise(&mut level);
            value = staff_wage(level, F64_1ENEG_09, rdata::F32_1000);
            let floor = if job_sel == 9 { 250.0f32 } else { 200.0 };
            if (value as f64) < f(if job_sel == 9 { rdata::F32_250 } else { rdata::F32_200 }) {
                value = floor;
            } else if value as f64 > f(rdata::F32_1000) {
                value = 1000.0;
            }
        }
        6 | 7 | 8 | 10 => {
            if sheet.is_null() {
                // the error message, then a draw
                let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
                format(TEXT_BUFFER, NO_PLAYER_DATA_FORMAT, person.first_name() as u32, person.second_name() as u32, id);
                let r = exe::crt_rand();
                value = ((r as i64 * 4 / 0x8000) as i32 as f64 * f(F32_550)) as f32;
            } else {
                covered!("mkd_2 player wage");
                value = player_default_wage(person.addr(), club.addr(), cur, &mut status, asked_job, kind as u8 as i8, level, frac);
            }
        }
        _ => value = 1000.0,
    }
    if club.is_null() {
        return value;
    }
    let rep = rep_of(club.addr());
    if !sheet.is_null() {
        // the floor by the club's reputation and the squad status
        let x = rep as f64 * if c.professional_status() == 1 {
            d(F64_0_00017)
        } else if rep > 2250 {
            d(F64_0_00012)
        } else {
            d(rdata::F64_0_0001)
        };
        let mut r = game_pow(x, d(rdata::F64_6));
        if r > d(rdata::F64_1) {
            r = 1.0;
        }
        let three = (rep * 3) as f64;
        let half_way = |value: &mut f32, y: f64| {
            if (*value as f64) < y {
                *value = ((*value as f64 + y) * d(rdata::F64_0_5)) as f32;
            }
        };
        let major = |age: i8| age >= 24 && club_is_in_major_nation(club.addr()) != 0;
        let s = status as u8 as i8 as i32 - 1;
        match s {
            0 => {
                let k = if age < 20 { d(rdata::F64_0_6) } else if major(age) { d(rdata::F64_1_5) } else { d(rdata::F64_1_25) };
                half_way(&mut value, three * r * k); // mkd2: the floor of the stars
            }
            1 => {
                let y = if age < 20 { three * r * d(rdata::F64_0_3) } else if major(age) { three * r } else { three * r * d(rdata::F64_0_6) };
                half_way(&mut value, y);
            }
            2 => {
                if age < 20 {
                    half_way(&mut value, three * r * d(rdata::F64_0_25));
                }
                let k = if major(age) { d(rdata::F64_0_6) } else { d(rdata::F64_0_3) };
                half_way(&mut value, three * r * k);
            }
            3 => {
                let k = if age < 20 { d(rdata::F64_0_1) } else if major(age) { d(rdata::F64_0_3) } else { d(rdata::F64_0_15) };
                half_way(&mut value, three * r * k);
            }
            4 => half_way(&mut value, three * r * d(rdata::F64_0_05)),
            5 => half_way(&mut value, three * r * d(rdata::F64_0_02)),
            _ => {}
        }
        let ct = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
        if ct != 0 && read_u8(ct + contract::GRIEVANCES) & 0x10 != 0 && (0..=5).contains(&s) {
            covered!("mkd_2 grievance");
            let k = [rdata::F64_0_67, rdata::F64_0_67, rdata::F64_0_85, rdata::F64_2_5, rdata::F64_3_5, rdata::F64_7_5][s as usize];
            value = (value as f64 * d(k)) as f32;
        }
    }
    // the club's nation, when the person works there or is from there
    let cn = club.nation().addr();
    if cn != 0 {
        let e = person.club_job();
        let sn = person.nation().addr();
        let go = if e.is_null() {
            sn == cn
        } else {
            e.nation().addr() == cn && (sn == cn || p.loyalty() > 3 || p.ambition() < 15)
        };
        if go {
            value = default_wage_by_nation(person.addr(), club.addr(), cn, sn, sheet.addr(), level, value);
        }
    }
    if !sheet.is_null() {
        if cur != 0 && club.addr() != cur {
            let ct = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), person, cur);
            if ct != 0 && read_u8(ct + contract::SQUAD_STATUS) & 0xf0 == 0x70 {
                let w = read_i32(ct + contract::WAGE);
                if value as f64 > w as f64 {
                    value = w as f32;
                }
            }
        }
        let max_wage = || {
            let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
            if fin == 0 { None } else { Some(exe::finance_get_max_wage(fin, person, status)) }
        };
        if flag != 0 || cur == 0 {
            if flag != 0 || sheet.current_reputation() < 2250 {
                let Some(mw) = max_wage() else { return value };
                if (mw as f64) < value as f64 {
                    value = mw as f32; // mkd2: the club's maximum wage
                }
            }
        } else {
            let at = |k: usize| read_i8(sheet.addr() + k);
            if p.ambition() > 10
                && p.loyalty() < 6
                && sheet.current_reputation() > 6750
                && (at(player_sheet::ATTACKER) == 20 || at(player_sheet::ATTACKING_MIDFIELDER) == 20 || at(player_sheet::MIDFIELDER) == 20)
                && age > 20
            {
                let Some(mw) = max_wage() else { return value };
                let loyal1 = p.loyalty() == 1;
                let k = if at(player_sheet::ATTACKING_MIDFIELDER) == 20 {
                    Some(if loyal1 { rdata::F64_1_15 } else { rdata::F64_1_1 })
                } else if value as f64 * d(rdata::F64_1_25) < mw as f64 {
                    if at(player_sheet::MIDFIELDER) == 20 {
                        Some(if loyal1 { rdata::F64_1_1 } else { rdata::F64_1_05 })
                    } else {
                        Some(if loyal1 { rdata::F64_1_2 } else { rdata::F64_1_15 })
                    }
                } else {
                    None
                };
                if let Some(k) = k {
                    covered!("mkd_2 ambitious star");
                    value = (value as f64 * d(k)) as f32;
                }
            }
        }
    }
    // the minimum
    if c.professional_status() == 1 {
        if age < 20 && (value as f64) < f(rdata::F32_75) {
            return 75.0;
        }
        if (value as f64) < f(rdata::F32_100) {
            value = 100.0;
        }
        value
    } else {
        if rep > 2250 && (value as f64) < f(rdata::F32_75) {
            return 75.0;
        }
        if (value as f64) < f(rdata::F32_50) {
            return 50.0;
        }
        value
    }
}

/// Comparer for `staff_contract_default_staff_contract`: the original and the copy from the same state of the last
/// contract looked up (`G_CONTRACT_LOOKUP_RESULT`) and of the CRT's rand() (the draw of a player without a sheet), the
/// returned float bit for bit.
pub(crate) unsafe extern "stdcall" fn compare_staff_contract_default_staff_contract(person: usize, club: usize, flag: u32, status: u32, job: u32, kind: u32) -> f32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32, u32, u32, u32) -> f32>(IDX_STAFF_CONTRACT_DEFAULT_STAFF_CONTRACT);
    let regions = [(G_CONTRACT_LOOKUP_RESULT, 4), (0xad6c30, 4)];
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(person, club, flag, status, job, kind),
        || crate::rng::without_counting(|| staff_contract_default_staff_contract(person, club, flag, status, job, kind)));
    SCOREBOARD.record(same && r.to_bits() == n.to_bits(), || format!("staff_contract_default_staff_contract(person {:x}, club {:x}, {}, {:#x}, {:#x}, {:#x}) original={} rust={} {:x?} x {:x?}",
        person, club, flag, status, job, kind, r, n, mo, mr));
    r
}

/// Fills `this` (a STAFF_CONTRACT) with the contract `person` wants at `club`: the signing fee (+8), the wage (+0xc),
/// the bonuses, the release clauses, the dates and the kind; returns the contract's value (an int) that the callers
/// compare.
///
/// Start: the release fee and the manager-job clause are zeroed; the person's contract
/// (`exe::contract_manager_get_contract`, kept in `G_CONTRACT_LOOKUP_RESULT`) gives the current club `cur` (none for a
/// contract of type 5); without `club`, the club is `cur` (or its linked B team, `exe::club_get_linked_b_team`, when
/// `cur` is not the A team); with `cur`, the current contract `cc` is the one with `cur`
/// (`exe::contract_manager_get_contract_for_club`). The contract gets `contract::CONTRACT_TYPE` = (old & 0x40) | 2,
/// `contract::LEAVING_ON_BOSMAN` 0, `contract::TRANSFER_ARRANGED_FOR` −1, the appearances 0, the person's id and the
/// squad number 0. A status of −1 for a player becomes the deserved one against `cc`
/// (`exe::contract_manager_get_deserved_squad_status`; with grievance 0x10 and not 1, 2) or, without `cc`, the status
/// for the club (`exe::shortlist_manager_get_squad_status_for_club`); it goes to the high nibble of
/// `contract::SQUAD_STATUS`; the grievances of `cc` are kept.
///
/// The level (a byte): with a club, reputation / 500 + 1 (professional status 2: minus one within 1..4) and the club's
/// id at `contract::CLUB`; without one, the world reputation of the player sheet / 500 (no non-player sheet) or the
/// national team's reputation / 500 + 1, at least 3, and the club −1. Internationals add ftol(league standard × 0.25 ×
/// caps × 0.125), at most 4 (0 → 1; no nation → 1). The reputation used by the clauses and the length is the player's
/// current reputation (a player job) or the non-player's current reputation, at least 1.
///
/// The signing fee (player job): `exe::contract_manager_calc_player_value` × 2 / 10, × 0.25 (no club job, current
/// reputation < 2750, club < 2000), × 0.5 (< 4250) or × 0.75 (< 4750); a renewal (club = `cur`): 0 under 19 with
/// < 3750, × 0.1 / 0.5 / 0.25 / 0.5 under 21 with < 4250 (by reputation and pressure), × 0.5 at a club below 3500,
/// then by the deserved status × 0.9 / 0.75 / 0.75 / 0.5 / 0.25; a new club: × 0.5 when `cur` is below 4250 and × 0.5
/// when its nation's league standard is below 10. Staff: 0.
///
/// No club job (`staff::JOB_FOR_CLUB` 0): the "has no job for club" message and the job 11 (player sheet) or 9.
///
/// Staff (not a player job, or no player sheet): w = the default wage (`exe::staff_contract_default_staff_contract`,
/// flag 0, kind 0); the will `exe::staff_interest_in_job_at_club` (written to `will`); w + 2500 with will < 1.75; the
/// value = w × max(3.5 / max(will, 0.1), 0.9); 250000 (the "not available" marker) for job 6 above 2 × the club's
/// reputation + 1200 and 4100, job 8 above reputation + 500 and 3200, job 9 above 0.75 × reputation + 250 and 2100, job
/// 10 the same with 1500, and anything above 250000.
///
/// Players: w = the default wage with kind 0 and job 11. A renewal asks flag 1 (at most the club's maximum wage) when it
/// is cheap: no grievance, no low-nibble mark in the staff state record, current reputation < 7250, fewer than loyalty
/// × 12 + 182 days to the end of `cc` and not leaving on a Bosman; or pressure < 6, under 24, no future transfer
/// (`exe::transfer_manager_get_future_transfer`), not listed (`exe::staff_is_transfer_listed_or_requested`), current
/// reputation < 3250 and < the club's + 250, fewer than 200 − pressure days; or grievances without the bits 0x161a8,
/// fewer than id % 35 + 20 days, no mark and current reputation + ambition × 20 < (loyalty + 170) × 25; or (loyalty
/// > 18, or below 5250 without the mark) with world reputation < 3500. A new club: kind 3 above 6750, else 0 (the tests
/// (100 − ambition − loyalty) × 125000 < 0 and (80 − ...) < 0, which would give kinds 1/2 and status 1/2, never hold
/// with attributes 1..20). Above the current wage: at most max(2 × wage, wage + 8000), and for the famous (world >
/// 6750, w > 10000) above that, 75000 (England, Spain, Italy) or 50000 when the club is `cur` or strong (comp or league
/// reputation × club reputation × 0.05 > 5750), 25% above it. The drive t = world + 25 × (ambition − loyalty) + 10 ×
/// pressure (16 bits): a star at a happy contract, or t above the club's reputation, or a renewal of a deserved star
/// with low loyalty, high ambition and pressure, with fewer than pressure × 10 + 525 days left, left out twice or world
/// ≥ 5750, at a club of a major nation (`club_is_in_major_nation`), 19 to 31 years: w rises to ref × x, ref = the
/// highest wage (`finance::HIGHEST_WAGE`) or 0.0001 × t × 85000 (England) / 105000 (Spain, Italy) when higher (60000
/// for a goalkeeper), x = (ambition − professionalism − loyalty + 4) × 0.0075 within −0.25..0.15, minus 0.1 / 0.05 by
/// age, minus 0.05 below the club's reputation − 1250, plus 0.45 (goalkeeper), 0.7 (defender), 1.0 (attacker, t >
/// 7250, loyalty < 6), 0.95 (t > 7250) or 0.9.
///
/// The will `exe::shortlist_manager_staff_interest_in_joining_club` (written to `will`), q = 3.5 / max(will, 0.1)
/// (stored as f32); with `cc`: grievance 2 caps q at 1.25; other grievances with will < 1.75 add 5000 to w for the
/// restless (pressure > 6, over 23, a future transfer, listed, current reputation > 3249 or above the club's + 250,
/// more than 200 − pressure days); without `cc`, w + 5000 with will < 1.75. q at least 0.9; a renewal caps q: no
/// contract 3.0; grievance 2 (or the young one with exactly 0x10) 1.2, grievance 1 1.75, 0x8400 1.5, others 4.0; no
/// grievance: 1.25 for the loyal (loyalty > id % 5 + 10) with fewer than 30 days, else by age and deserved status (under
/// 19 or over 34: 2.0 / 1.35 / 1.25 / 1.15; under 21 or over 31: 2.0 / 1.5 / 1.35 / 1.15; else 2.0) and 1.1 for the
/// young (< 20, ability < 125 − pressure − ambition, current reputation < 4250, fewer than 120 − pressure − ambition
/// days). The value = q × w; with `cc` (not `this`) paying more, at least its wage × the clubs' reputation ratio (at
/// least 0.5); with the grievance 0x1000000, at least 95% of the highest wage. The marker 250000: above 200000; above
/// 35000 over 34; above 75000 over 31; goalkeeper 20 above 105000; status 7 above 5000, 6 above 8250, 5 above 17500,
/// 4 above 35000, 3 above 57500 with world < 6750 (a human club, or current reputation ≤ 6250, or above 85000 / 105000
/// / 125000 by position and reputation), 2 above 125000 with world < 7250. A dearer current contract at another club
/// blends: value = ((20 − a) × its wage + a × min(w, max(w × 0.5, the maximum wage))) × 0.05, a = 0..20 (grievance
/// 0x100 +10, listed +10, wage above 40000, age over 33, unhappiness and morale), with `exe::finance_get_max_wage`
/// called two to four times.
///
/// Clauses (job 11 with a player sheet): +0x1d 0 and +0x20 1 for the old international (or a non-player sheet); the
/// non-promotion clause (+0x1c, the league's `league_vtable::HAS_AUTOMATIC_PROMOTION`), relegation (+0x1f,
/// `league_vtable::HAS_RELEGATION`), minimum fee (+0x1d) and not playing (+0x1e) with the fee (+0x21) from
/// `exe::contract_manager_calc_player_value` (minimums 150000 / 50000 / 150000 / 1500000); a clause the current
/// contract had at the same club and that is now dropped multiplies the value by 1.05 (1.15 for the minimum fee).
///
/// Dates (with a club): a signing elsewhere gets the window date (`find_next_window_open_date` with no offer,
/// `exe::transfer_manager_find_next_allowed_transfer_date` by the offer's slot), then `contract::DATE_STARTED` is set
/// to today anyway; the end is day `exe::utils_random(28)` + 1 of the season end month
/// (`exe::contract_manager_season_end_month`) of this year (next year when the key nation's season spans two years) +
/// ftol(L + 1) years, L by age and position (3, 4, 3 / 2 / 1.5, 4 or 1), × 0.75 with a clause, by the level against the
/// reputation and the loyalty, by the squad status, by the current reputation, × 0.5 in Finland and Turkey (at most 3),
/// at most 2 / 3 / 4 for a wage below 500 / 1000 / 2000, within 1..`G_MAX_CONTRACT_LENGTH_YEARS` − 1; the kind is 2
/// (with the bit 0x80 when there is no club job, the level is below 2 and a fixed draw by id is 0; in that last case,
/// up to 20 years, kind 1 without the years); a renewal never moves the end before `staff::DATE_EXPIRES_CLUB`.
///
/// Bonuses (+0x10 / +0x14 / +0x18) = ftol(value × draw × 0.025 + value × 0.025 + 25) for attacker / midfielder /
/// goalkeeper 20, else −1; the bit 0x80 of the kind is set below 125 at a club of professional status 2 and cleared
/// above 1250; `contract::WAGE` = ftol(value). The return: ftol(wage (× 0.5 for kind 1, × 1.05 with 0x80) + min(the
/// bonuses / 10 and the clause weights × value, value × 0.1)).
///
/// Defects of the original kept: with an offer without a slot (`exe::transfer_manager_get_offer_slot` 0) it reads the
/// address 0 (`&&` where `||` was meant) before the "Error" box; the reputation of the clauses and the length is left
/// unset (a stack leftover) for a person with a player sheet, a staff job and no non-player sheet (here 0, then 1); the
/// window date is overwritten with today; the kind 1/2 branch of a new club never runs.
/// Original: `STAFF_CONTRACT::set_to_desired_contract` (0x85fc50, staff_contracts.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn staff_contract_set_to_desired_contract(this: usize, person: usize, club: usize, offer: i32, status: u32, job: u32, reasons: usize, will: usize) -> i32 {
    let person = Staff(person);
    use desired_contract_rdata::*;
    let f = |a: usize| read_f32(a) as f64;
    let d = |a: usize| read_f64(a);
    let ct = Contract(this);
    let cm = || read_usize(G_CONTRACT_MANAGER);
    let tm = || read_usize(G_TRANSFER_MANAGER);
    let shortlist = || read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER);
    let today = || (read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP));
    let days_to = |c: usize| {
        let (lo, hi) = today();
        exe::date_days_since(c + contract::CONTRACT_EXPIRES, lo, hi)
    };
    // the club of the person's contract (none for type 5), with the lookup kept in G_CONTRACT_LOOKUP_RESULT
    let lookup = || {
        let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
        write_usize(G_CONTRACT_LOOKUP_RESULT, c);
        if c == 0 || read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5 {
            0
        } else {
            read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + contract::CLUB).wrapping_mul(club::SIZE))
        }
    };
    let is_player_job = |j: u32| exe::database_is_player_job(j as u8 as i8) != 0;
    let rep_of = |c: usize| read_i16(c + club::REPUTATION);
    let mut club = club;
    let mut status = status;
    let set_status = |s: &mut u32, b: u8| *s = *s & !0xff | b as u32;
    ct.set_release_fee((0) as usize);
    ct.set_manager_job_rc(0);
    let mut cc: usize = 0; // the contract with the current club
    let mut grievances: u32 = 0;
    let mut forced_kind: u8 = 0; // the original's [esp+0x3b]: set only by the new-club branch that never runs
    let cur = Club(lookup());
    if club == 0 {
        club = cur.addr();
        if !cur.is_null() {
            let mut is_a: u32 = 0;
            let linked = exe::club_get_linked_b_team(cur, &mut is_a, 1);
            if linked != 0 && is_a == 0 {
                club = linked;
            }
        }
    }
    if !cur.is_null() {
        cc = exe::contract_manager_get_contract_for_club(cm(), person, cur);
    }
    ct.set_leaving_on_bosman(0);
    ct.set_transfer_arranged_for((u32::MAX) as usize);
    ct.set_club_appearances(0u16 as i16);
    ct.set_contract_type(ct.contract_type() & 0x40 | 2);
    write_u32(ct.addr(), read_u32(person.addr()));
    ct.set_squad_number(0);
    let frac = (read_i32(person.addr()) as f64 / read_i32(G_STAFF_COUNT) as f64) as f32; // the fixed draw by id
    let fr = frac as f64;
    let age = person.age() as i8;
    let sheet = person.player();
    let np = person.non_player().addr();
    let amb = person.ambition() as i32;
    let loy = person.loyalty() as i32;
    let pr = person.pressure() as i32;
    // the squad status
    if status as u8 == 0xff && !sheet.is_null() {
        if cc != 0 {
            let mut s = exe::contract_manager_get_deserved_squad_status(cm(), person, cc);
            grievances = read_u32(cc + contract::GRIEVANCES);
            if grievances & 0x10 != 0 && s != 1 {
                s = 2;
            }
            set_status(&mut status, s);
        } else {
            let s = exe::shortlist_manager_get_squad_status_for_club(shortlist(), person, club, 0usize);
            set_status(&mut status, s);
        }
    } else if cc != 0 {
        grievances = read_u32(cc + contract::GRIEVANCES);
    }
    ct.set_squad_status(ct.squad_status() & 0xf | (status as u8) << 4);
    // the level
    let mut level: u8;
    if club != 0 {
        let q = (rep_of(club) as i32 / 500) as u8;
        level = q.wrapping_add(1);
        if read_u8(club + club::PROFESSIONAL_STATUS) == 2 {
            level = q;
            if (level as i8) < 1 {
                level = 1;
            } else if (level as i8) > 4 {
                level = 4;
            }
        }
        ct.set_club((read_u32(club)) as usize);
    } else {
        if np == 0 && !sheet.is_null() {
            level = (sheet.world_reputation() as i32 / 500) as u8;
        } else {
            level = ((rep_of(person.national_job().addr()) as i32 / 500) as u8).wrapping_add(1);
            if (level as i8) < 3 {
                level = 3;
            }
        }
        ct.set_club((u32::MAX) as usize);
    }
    let int_apps = person.int_apps();
    if int_apps != 0 {
        let nation = person.nation();
        let mut add: u8 = 1;
        if !nation.is_null() {
            add = ftol((nation.league_standard() as i8) as f64 * f(rdata::F32_0_25) * (int_apps as f64 * f(F32_0_125))) as u8;
            if (add as i8) > 4 {
                add = 4;
            } else if add == 0 {
                add = 1;
            }
        }
        level = level.wrapping_add(add);
    }
    let lv = level as i8 as i32;
    // the reputation of the clauses and the length
    let mut rep48: i16 = 0; // the original leaves it unset with a sheet, a staff job and no non-player sheet
    if !sheet.is_null() && is_player_job(job) {
        rep48 = sheet.current_reputation();
    } else if np != 0 {
        rep48 = read_i16(np + non_player::CURRENT_REPUTATION);
    }
    if rep48 == 0 {
        rep48 = 1;
    }
    // the signing fee
    let fee = ct.addr() + contract::UNKNOWN;
    let scale_fee = |k: f64| write_i32(fee, ftol(read_i32(fee) as f64 * k));
    if !sheet.is_null() && is_player_job(job) {
        let v = exe::contract_manager_calc_player_value(cm(), person, club, 1, -1, 0);
        write_i32(fee, v.wrapping_mul(2) / 10); // imul 0x66666667; sar 2: / 10
        let cr = sheet.current_reputation();
        if person.club_job().addr() == 0 && cr < 2750 && rep_of(club) < 2000 {
            scale_fee(d(rdata::F64_0_25));
        } else if cr < 4250 {
            scale_fee(d(rdata::F64_0_5));
        } else if cr < 4750 {
            scale_fee(d(rdata::F64_0_75));
        }
        if club == cur.addr() {
            if age < 19 && cr < 3750 {
                write_i32(fee, 0);
            } else if age < 21 && cr < 4250 {
                let k = if pr >= 15 { rdata::F64_0_5 } else if cr < 2750 { rdata::F64_0_1 } else { rdata::F64_0_25 };
                scale_fee(d(k));
            } else if club != 0 && cr < 3500 && rep_of(club) < 3500 {
                scale_fee(d(rdata::F64_0_5));
            }
            let k = match exe::contract_manager_get_deserved_squad_status(cm(), person, cc) as i8 {
                1 => rdata::F64_0_9,
                2 | 3 => rdata::F64_0_75,
                4 => rdata::F64_0_5,
                _ => rdata::F64_0_25,
            };
            scale_fee(d(k));
        } else if !cur.is_null() {
            if rep_of(cur.addr()) < 4250 {
                scale_fee(d(rdata::F64_0_5));
            }
            let n = cur.nation().addr();
            if n != 0 && read_i8(n + nation::LEAGUE_STANDARD) < 10 {
                scale_fee(d(rdata::F64_0_5));
            }
        }
    } else {
        write_i32(fee, 0);
    }
    // no club job
    let mut staff_path = false;
    if person.job_for_club() == 0 {
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        sprintf(TEXT_BUFFER, NO_JOB_FORMAT, read_i32(person.addr()));
        if !sheet.is_null() {
            person.set_job_for_club(0xb);
        } else {
            person.set_job_for_club(9);
            staff_path = true;
        }
    }
    if !staff_path && (sheet.is_null() || !is_player_job(job)) {
        staff_path = true;
    }
    let mut v: f32; // the value ([esp+0x10])
    let mut w: f32; // the wage asked ([esp+0x24])
    if staff_path {
        covered!("mkd_3 staff contract");
        w = exe::staff_contract_default_staff_contract(ct, person, club, 0, status, job, 0) as f32;
        let will_v = exe::staff_interest_in_job_at_club(shortlist(), person, club, 0, job, reasons as u32) as f32;
        if will != 0 {
            write_f32(will, will_v);
        }
        let wv = will_v as f64;
        let mut q = d(rdata::F64_3_5) / if wv < d(rdata::F64_0_1) { d(rdata::F64_0_1) } else { wv };
        if wv < d(rdata::F64_1_75) {
            w = (w as f64 + f(rdata::F32_2500)) as f32;
        }
        if q < d(rdata::F64_0_9) {
            q = f(rdata::F32_0_8999999762);
        }
        v = (q * w as f64) as f32;
        let vv = v as f64;
        let rep = rep_of(club) as i32;
        let unavailable = match job as u8 {
            6 => ((rep * 2 + 1200) as f64) < vv && vv > f(F32_4100),
            8 => ((rep + 500) as f64) < vv && vv > f(F32_3200),
            9 => rep as f64 * d(rdata::F64_0_75) + d(F64_250) < vv && vv > f(rdata::F32_2100),
            10 => rep as f64 * d(rdata::F64_0_75) + d(F64_250) < vv && vv > f(rdata::F32_1500),
            _ => false,
        };
        if unavailable || vv > f(F32_250000) {
            v = 250000.0;
        }
    } else {
        covered!("mkd_3 player contract");
        let cr = sheet.current_reputation();
        let world = sheet.world_reputation();
        let mark = || read_u8(staff_state_entry(person) + staff_state::STATUS_FLAGS) & 0xf;
        if !cur.is_null() && cur.addr() == club {
            // a renewal: flag 1 (at most the club's maximum wage) when it is cheap
            let days = days_to(cc);
            let cheap = (grievances == 0 && mark() == 0 && cr < 7250 && days < loy * 12 + 182 && read_u8(cc + contract::LEAVING_ON_BOSMAN) == 0)
                || (pr < 6
                    && age < 24
                    && exe::transfer_manager_get_future_transfer(tm(), person, 0) == 0
                    && exe::staff_is_transfer_listed_or_requested(person, club) == 0
                    && cr < 3250
                    && cr < rep_of(club).wrapping_add(250)
                    && days_to(cc) < 200 - pr)
                || (grievances != 0
                    && grievances & 0x161a8 == 0
                    && days < read_i32(person.addr()) % 35 + 20
                    && mark() == 0
                    && (cr as i32 + amb * 20) < (loy + 170) * 25)
                || ((loy > 18 || (cr < 5250 && mark() == 0)) && world < 3500);
            w = exe::staff_contract_default_staff_contract(ct, person, club, cheap as u32, status, 0xb, 0) as f32;
        } else if (100 - amb - loy).wrapping_mul(125000) < 0 {
            set_status(&mut status, 1);
            let kind = if pr > 5 || world > 6250 { 1 } else { 2 };
            w = exe::staff_contract_default_staff_contract(ct, person, club, 0, 1, 0xb, kind) as f32;
            forced_kind = 1;
        } else if (80 - amb - loy).wrapping_mul(125000) < 0 {
            if status as u8 != 1 {
                set_status(&mut status, 2);
            }
            w = exe::staff_contract_default_staff_contract(ct, person, club, 0, status, 0xb, 2) as f32;
        } else {
            let kind = if cr > 6750 { 3 } else { 0 };
            w = exe::staff_contract_default_staff_contract(ct, person, club, 0, status, 0xb, kind) as f32;
        }
        // above the current wage
        if cc != 0 {
            let cw = read_i32(cc + contract::WAGE);
            if (cw as f64) < w as f64 {
                let mut cap = if cw.wrapping_mul(2) > cw.wrapping_add(8000) { cw.wrapping_mul(2) } else { cw.wrapping_add(8000) };
                if world > 6750 && w as f64 > f(rdata::F32_10000) && (cap as f64) < w as f64 {
                    let n = if club != 0 { read_usize(club + club::NATION) } else { 0 };
                    let key = if n != 0 { read_i32(n) } else { 0 };
                    let big: i32 = if n != 0 && (key == read_i32(KEY_NATION_ENGLAND) || key == read_i32(KEY_NATION_SPAIN) || key == read_i32(KEY_NATION_ITALY)) { 75000 } else { 50000 };
                    let strong = if cur.addr() == club {
                        true
                    } else if club != 0 {
                        let div = read_usize(club + club::DIVISION);
                        let n = read_usize(club + club::NATION);
                        let r = if div != 0 {
                            ftol((read_i16(div + competition::REPUTATION) as i32).wrapping_mul(rep_of(club) as i32) as f64 * d(rdata::F64_0_05)) as i16
                        } else if n != 0 {
                            ftol((read_i8(n + nation::LEAGUE_STANDARD) as i32).wrapping_mul(rep_of(club) as i32) as f64 * d(rdata::F64_0_05)) as i16
                        } else {
                            rep_of(club)
                        };
                        r > 5750
                    } else {
                        false
                    };
                    if strong {
                        cap = if cap >= big { ftol((w as f64 - big as f64) * d(rdata::F64_0_25) + big as f64) } else { big };
                    }
                }
                if w as f64 > cap as f64 {
                    w = cap as f32;
                }
            }
        }
        let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        let highest = if fin != 0 { read_i32(fin + finance::HIGHEST_WAGE) } else { 0 };
        // the drive
        let t = (world as i32 + (amb - loy) * 25 + pr * 10) as i16;
        let s = status as u8 as i8;
        let happy = |c: usize| c != 0 && read_i8(c + contract::HAPPINESS) > 50;
        let wants_more = (s == 1 && age > 18 && happy(cc))
            || (world > 4250 && (t > rep_of(club) || (t > 6750 && int_apps != 0 && s < 3 && s != 0)) && s != 7)
            || (cur.addr() == club
                && happy(cc)
                && exe::contract_manager_get_deserved_squad_status(cm(), person, cc) == 1
                && loy < 15
                && amb > 10
                && pr > 10
                && t > 6750
                && world > 4250);
        if wants_more
            && (cc == 0 || (days_to(cc) <= pr * 10 + 525 && (read_i8(cc + contract::MATCHES_LEFT_OUT) >= 2 || world >= 5750)))
            && club_is_in_major_nation(club) != 0
            && age > 18
            && age < 32
        {
            covered!("mkd_3 star wage");
            let (mut top, mut reference) = (highest, highest);
            if t > 7250 && person.player().addr() != 0 && sheet.goalkeeper() != 20 && loy != 20 {
                let x = t as i32 as f64 * d(rdata::F64_0_0001);
                let key = read_i32(read_usize(club + club::NATION));
                let k = if key == read_i32(KEY_NATION_ENGLAND) {
                    Some(F32_85000)
                } else if key == read_i32(KEY_NATION_SPAIN) || key == read_i32(KEY_NATION_ITALY) {
                    Some(F32_105000)
                } else {
                    None
                };
                if let Some(k) = k {
                    let e = ftol(x * f(k));
                    if highest < e {
                        top = e;
                        reference = e;
                    }
                }
            }
            if person.player().addr() != 0 && sheet.goalkeeper() == 20 && top > 60000 {
                reference = 60000;
            }
            let mut x = (amb - person.professionalism() as i32 - loy + 4) as f64 * d(rdata::F64_0_0075);
            if x < d(F64_NEG_0_25) {
                x = f(F32_NEG_0_25);
            } else if x > d(rdata::F64_0_15) {
                x = f(F32_0_1500000060);
            }
            if age > 29 {
                if t < 5250 {
                    x -= d(rdata::F64_0_1);
                } else if pr < 15 {
                    x -= d(rdata::F64_0_05);
                }
            } else if t < 4250 && pr < 15 {
                x -= d(rdata::F64_0_05);
            }
            if (cr as i32 + amb * 25) < rep_of(club).wrapping_sub(1250) as i32 {
                x -= d(rdata::F64_0_05);
            }
            x += if sheet.goalkeeper() == 20 {
                d(rdata::F64_0_45)
            } else if sheet.defender() == 20 {
                d(rdata::F64_0_7)
            } else if sheet.attacker() == 20 && t > 7250 && loy < 6 {
                d(rdata::F64_1)
            } else if t > 7250 {
                d(rdata::F64_0_95)
            } else {
                d(rdata::F64_0_9)
            };
            let y = x * reference as f64;
            if (w as f64) < y {
                w = y as f32;
            }
        }
        // the will
        let will_v = exe::shortlist_manager_staff_interest_in_joining_club(shortlist(), person, club, 0, 0, status, reasons as u32);
        if will != 0 {
            write_f32(will, will_v as f32);
        }
        let mut q = (d(rdata::F64_3_5) / if will_v < d(rdata::F64_0_1) { d(rdata::F64_0_1) } else { will_v }) as f32;
        let restless = if cc != 0 {
            if grievances & 2 != 0 {
                if q as f64 > d(rdata::F64_1_25) {
                    q = 1.25;
                }
                false
            } else {
                grievances != 0
                    && will_v < d(rdata::F64_1_75)
                    && (pr > 6
                        || age > 23
                        || exe::transfer_manager_get_future_transfer(tm(), person, 0) != 0
                        || exe::staff_is_transfer_listed_or_requested(person, club) != 0
                        || read_i16(person.player().addr() + player_sheet::CURRENT_REPUTATION) > 3249
                        || read_i16(person.player().addr() + player_sheet::CURRENT_REPUTATION) > rep_of(club).wrapping_add(250)
                        || days_to(cc) > 200 - pr)
            }
        } else {
            will_v < d(rdata::F64_1_75)
        };
        if restless {
            w = (w as f64 + f(rdata::F32_5000)) as f32; // mkd3: the restless ask 5000 more
        }
        let cap_q = |q: &mut f32, limit: usize, to: u32| {
            if *q as f64 > d(limit) {
                *q = f32::from_bits(to);
            }
        };
        if (q as f64) < d(rdata::F64_0_9) {
            q = f32::from_bits(0x3f666666);
        } else if cur.addr() == club {
            if cc == 0 {
                cap_q(&mut q, rdata::F64_3, 0x40400000);
            } else if grievances != 0 {
                if grievances & 2 != 0 || (age < 21 && cr < 4250 && grievances == 0x10) {
                    cap_q(&mut q, rdata::F64_1_2, 0x3f99999a);
                } else if grievances & 1 != 0 {
                    cap_q(&mut q, rdata::F64_1_75, 0x3fe00000);
                } else if grievances & 0x8400 != 0 {
                    cap_q(&mut q, rdata::F64_1_5, 0x3fc00000);
                } else {
                    cap_q(&mut q, rdata::F64_4, 0x40800000);
                }
            } else {
                covered!("mkd_3 renewal demand");
                let loyal = || loy as i8 > ((read_i32(person.addr()) % 5) as u8).wrapping_add(10) as i8 && days_to(cc) < 30;
                if loyal() {
                    cap_q(&mut q, rdata::F64_1_25, 0x3fa00000);
                } else if age < 19 || age > 34 {
                    match exe::contract_manager_get_deserved_squad_status(cm(), person, cc) as i8 {
                        1 => cap_q(&mut q, rdata::F64_2, 0x40000000),
                        2 => cap_q(&mut q, rdata::F64_1_35, 0x3faccccd),
                        3..=5 => cap_q(&mut q, rdata::F64_1_25, 0x3fa00000),
                        _ => cap_q(&mut q, rdata::F64_1_15, 0x3f933333),
                    }
                } else if age < 21 || age > 31 {
                    match exe::contract_manager_get_deserved_squad_status(cm(), person, cc) as i8 {
                        1 => cap_q(&mut q, rdata::F64_2, 0x40000000),
                        2 => cap_q(&mut q, rdata::F64_1_5, 0x3fc00000),
                        3..=5 => cap_q(&mut q, rdata::F64_1_35, 0x3faccccd),
                        _ => cap_q(&mut q, rdata::F64_1_15, 0x3f933333),
                    }
                } else if loyal() {
                    cap_q(&mut q, rdata::F64_1_25, 0x3fa00000);
                } else {
                    cap_q(&mut q, rdata::F64_2, 0x40000000);
                }
                if q as f64 > d(rdata::F64_1_1)
                    && age < 20
                    && (sheet.current_ability() as i32) < 125 - pr - amb
                    && cr < 4250
                    && days_to(cc) < 120 - pr - amb
                {
                    q = f32::from_bits(0x3f8ccccd);
                }
            }
        }
        v = (q as f64 * w as f64) as f32; // mkd3: the value asked
        w = v;
        // at least the current wage scaled by the clubs' reputations
        if cc != 0 && cc != ct.addr() {
            let cw = read_i32(cc + contract::WAGE) as f64;
            if cw > v as f64 {
                let mut r = f(rdata::F32_1);
                let (a, b) = (rep_of(club), rep_of(cur.addr()));
                if a < b {
                    let x = a as i32 as f64 / if b > 100 { b as i32 } else { 100 } as f64;
                    r = if x > d(rdata::F64_0_5) { x } else { d(rdata::F64_0_5) };
                }
                if cw * r > v as f64 {
                    v = (cw * r) as f32;
                }
            }
        }
        if grievances & 0x1000000 != 0 && highest as f64 * d(rdata::F64_0_95) > v as f64 {
            v = (read_i32(fin + finance::HIGHEST_WAGE) as f64 * f(rdata::F32_0_9499999881)) as f32;
        }
        // the "not available" markers
        let vv = v as f64;
        if vv > f(F32_200000) || (age > 34 && vv > f(F32_35000)) || (age > 31 && vv > f(F32_75000)) || (sheet.goalkeeper() == 20 && vv > f(F32_105000)) {
            v = 250000.0;
        }
        let vv = v as f64;
        let mut done = true;
        match s {
            7 if vv > f(rdata::F32_5000) => v = 250000.0,
            6 if vv > f(F32_8250) => v = 250000.0,
            5 if vv > f(F32_17500) => v = 250000.0,
            4 if vv > f(F32_35000) => v = 250000.0,
            3 if vv > f(F32_57500) && world < 6750 => {
                let mut out = exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0;
                if !out {
                    let sh = person.player().addr();
                    out = sh == 0 || {
                        let r = read_i16(sh + player_sheet::CURRENT_REPUTATION);
                        let limit = if read_i8(sh + player_sheet::ATTACKER) == 20 || read_i8(sh + player_sheet::ATTACKING_MIDFIELDER) == 20 {
                            if r > 7250 { F32_125000 } else { F32_105000 }
                        } else if r > 7250 {
                            F32_105000
                        } else {
                            F32_85000
                        };
                        r <= 6250 || vv > f(limit)
                    };
                }
                if out {
                    v = 250000.0;
                }
            }
            2 if vv > f(F32_125000) && world < 7250 => v = 250000.0,
            _ => done = false,
        }
        if !done {
            if vv > f(F32_250000) {
                v = 250000.0;
            } else if cc != 0 {
                let cw = read_i32(cc + contract::WAGE);
                if cw as f64 > w as f64 && cw != 0 && cur.addr() != club {
                    // a dearer contract at another club: blend toward its wage
                    covered!("mkd_3 dearer current contract");
                    let mut a = if read_u32(cc + contract::GRIEVANCES) & 0x100 != 0 { 10.0 } else { 0.0 };
                    if exe::staff_is_transfer_listed_or_requested(person, cur) != 0 {
                        a += d(rdata::F64_10);
                    }
                    if read_i32(cc + contract::WAGE) > 40000 {
                        a += (read_i32(cc + contract::WAGE) - 40000) as f64 * d(rdata::F64_0_00025);
                    }
                    if age > 33 {
                        let k = (38 - age as i32) as f64;
                        a += k + k;
                    }
                    let h = read_i8(cc + contract::HAPPINESS);
                    if h < 0 {
                        a -= h as f64 * d(rdata::F64_0_2);
                    }
                    if read_u32(cc + contract::GRIEVANCES) != 0 {
                        a -= read_i8(cc + contract::MORALE) as f64 * d(rdata::F64_0_2);
                    }
                    if d(rdata::F64_0) > a {
                        a = 0.0;
                    } else if d(rdata::F64_20) < a {
                        a = 20.0;
                    }
                    let half = w as f64 * d(rdata::F64_0_5);
                    let max_wage = || exe::finance_get_max_wage(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club), person, status) as f64;
                    let m = if max_wage() < half { half } else { max_wage() };
                    let x = if (w as f64) < m {
                        w as f64
                    } else if max_wage() < half {
                        half
                    } else {
                        max_wage()
                    };
                    v = (((d(rdata::F64_20) - a) * read_i32(cc + contract::WAGE) as f64 + x * a) * d(rdata::F64_0_05)) as f32;
                }
            }
        }
    }
    // the release clauses
    let clause = |o: usize| ct.addr() + o;
    if person.job_for_club() == 0xb && person.player().addr() != 0 {
        let id = read_i32(person.addr());
        let det = person.determination() as i32;
        let prof = person.professionalism() as i32;
        if (age > 32 && rep48 > 3250 && id % 3 == 0 && age as i32 + prof + det + amb + id % 10 - 28 > 45) || person.non_player().addr() != 0 {
            write_u8(clause(contract::MINIMUM_FEE_RC), 0);
            write_u8(clause(contract::MANAGER_JOB_RC), 1);
        }
        ct.set_release_fee((0) as usize);
        // the club's league answers the virtual `slot` (promotion or relegation)
        let league_has = |slot: usize| {
            if club == 0 {
                return false;
            }
            let div = read_usize(club + club::DIVISION);
            if div == 0 {
                return false;
            }
            let comp = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(div) as usize).wrapping_mul(4))));
            if comp.is_null() || comp.comp_type() != 2 {
                return false;
            }
            let m: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(comp.addr()) + slot));
            m(comp.addr()) != 0
        };
        let value_plus = |k: f64| {
            let v1 = exe::contract_manager_calc_player_value(cm(), person, club, 1, -1, 0);
            v1.wrapping_add(ftol(v1 as f64 * fr * k))
        };
        let extra = |val: i32| {
            let y = val as f64 * d(rdata::F64_0_95) + d(F64_150000);
            let z = val.wrapping_mul(2) as f64;
            let e = ftol(if y > z { y } else { z });
            if e > 2500000 {
                ftol(e as f64 * d(F64_0_21) - d(F64_2500000) + d(F64_2500000))
            } else {
                e
            }
        };
        let dropped = |o: usize| cc != 0 && read_u8(cc + o) != 0 && lookup() == club;
        let q50 = rep48 as i32 / 50;
        // non-promotion
        if rep48 > 2750 && ((lv * 13) as f64) < q50 as f64 + fr * f(rdata::F32_20) - (loy * 2) as f64 + (amb * 2) as f64 && league_has(league_vtable::HAS_AUTOMATIC_PROMOTION) {
            let mut val = value_plus(f(rdata::F32_0_25));
            if cc != 0 && read_u8(cc + contract::GRIEVANCES) & 1 == 0 {
                val = val.wrapping_add(extra(val));
            }
            write_u8(clause(contract::MINIMUM_FEE_RC), 0);
            write_u8(clause(contract::NON_PROMOTION_RC), 1);
            ct.set_release_fee((if val < 150000 { 150000 } else { val }) as usize);
        } else {
            if dropped(contract::NON_PROMOTION_RC) {
                v = (v as f64 * d(rdata::F64_1_05)) as f32;
            }
            write_u8(clause(contract::NON_PROMOTION_RC), 0);
        }
        // relegation
        if rep48 > 2250 && ((lv * 10) as f64) < (q50 + (amb - loy) * 3) as f64 + fr * f(rdata::F32_20) - age as f64 && league_has(league_vtable::HAS_RELEGATION) {
            let mut val = value_plus(f(F32_NEG_0_5));
            if val < 50000 {
                val = 50000;
            }
            if amb > 10 {
                val = ftol(val as f64 * d(if loy < 5 { rdata::F64_0_5 } else { rdata::F64_0_75 }));
            } else if loy < 15 {
                val = ftol(val as f64 * d(rdata::F64_0_85));
            }
            write_u8(clause(contract::MINIMUM_FEE_RC), 0);
            write_u8(clause(contract::RELEGATION_RC), 1);
            ct.set_release_fee((val) as usize);
        } else {
            if dropped(contract::RELEGATION_RC) {
                v = (v as f64 * d(rdata::F64_1_05)) as f32;
            }
            write_u8(clause(contract::RELEGATION_RC), 0);
        }
        // minimum fee
        let min_fee = |val: i32| {
            write_u32(clause(contract::NON_PROMOTION_RC), 0);
            write_u8(clause(contract::MANAGER_JOB_RC), 0);
            write_u8(clause(contract::MINIMUM_FEE_RC), 1);
            write_u8(clause(contract::NON_PROMOTION_RC), 0);
            write_u8(clause(contract::RELEGATION_RC), 0);
            ct.set_release_fee((if val < 150000 { 150000 } else { val }) as usize);
        };
        if club == cur.addr() && grievances & 1 != 0 {
            min_fee(value_plus(f(rdata::F32_0_25)));
        } else {
            let mut set = false;
            if rep48 > 1250 && !(lookup() != 0 && lv >= 18) && ((lv * 10) as f64) < (amb * 4 - loy * 3 + q50 - age as i32) as f64 + fr * f(rdata::F32_10) {
                let mut val = value_plus(f(rdata::F32_0_5));
                if grievances == 0 {
                    val = val.wrapping_add(extra(val));
                }
                min_fee(val);
                set = true;
            }
            if !set && age < 24 {
                let ab = ftol((sheet.current_ability() as i32 + sheet.potential_ability() as i32) as f64 * d(rdata::F64_0_5)) as i16 as i32;
                if ab - (loy * 3) as i16 as i32 + (amb * 2) as i16 as i32 > lv * 15 {
                    let mut val = value_plus(f(rdata::F32_0_5));
                    if grievances == 0 {
                        val = val.wrapping_add(extra(val));
                    }
                    min_fee(val);
                    set = true;
                }
            }
            if !set {
                if dropped(contract::MINIMUM_FEE_RC) {
                    v = (v as f64 * d(rdata::F64_1_15)) as f32;
                }
                write_u8(clause(contract::MINIMUM_FEE_RC), 0);
            }
        }
        // not playing
        if rep48 > 6500 && ((lv * 14) as f64) < (q50 - loy * 4) as f64 - fr * f(rdata::F32_20) {
            let mut val = value_plus(f(rdata::F32_0_5));
            if grievances == 0 {
                val = val.wrapping_add(extra(val));
            }
            write_u8(clause(contract::MINIMUM_FEE_RC), 0);
            write_u8(clause(NON_PLAYING_RC), 1);
            ct.set_release_fee((if val < 1500000 { 1500000 } else { val }) as usize);
        } else {
            write_u8(clause(NON_PLAYING_RC), 0);
        }
    } else {
        for o in contract::NON_PROMOTION_RC..=contract::MANAGER_JOB_RC {
            write_u8(clause(o), 0);
        }
    }
    // the dates and the length
    if club != 0 {
        let mut tmp = [0u8; 8];
        let tp = tmp.as_mut_ptr() as usize;
        if !sheet.is_null() && cur.addr() != club {
            if offer == -1 {
                find_next_window_open_date(tm(), club, ct.addr() + contract::DATE_STARTED);
            } else {
                let slot = exe::transfer_manager_get_offer_slot(tm(), offer);
                // defect of the original: `&&` for `||`; without a slot it reads the address 0 first
                if slot == 0 && core::ptr::read_volatile(core::hint::black_box(slot) as *const i32) == 0 {
                    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
                    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
                    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
                    exe::crt_strtok(SOURCE_PATH, 0, 0, name.as_mut_ptr(), ext.as_mut_ptr());
                    let ver = version(name.as_mut_ptr(), ext.as_mut_ptr(), 0xc4e);
                    sprintf(0xae24d0, 0x9870e8, ver, name.as_ptr(), ext.as_ptr(), 0xc4e);
                    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0);
                    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
                    exe::date_operator_3d(ct.addr() + contract::DATE_STARTED, tp, G_CURRENT_DATE);
                } else {
                    exe::transfer_manager_find_next_allowed_transfer_date(tm(), read_usize(slot), (ct.addr() + contract::DATE_STARTED) as *mut [u32; 2]);
                }
            }
        }
        exe::date_operator_3d(ct.addr() + contract::DATE_STARTED, tp, G_CURRENT_DATE);
        let month = exe::contract_manager_season_end_month(cm(), club);
        let day = exe::utils_random(0x1c) + 1;
        let mut dmy = [0u16; 4];
        let day_of_year = read_u16(exe::date_date_549f70(&mut dmy, day as u32, month as u32, 0x7ce, u32::MAX));
        let k = exe::key_nation_index_of_club(club) as i8 as isize;
        let mut year = read_u16(G_CURRENT_DATE_YEAR);
        if read_u32((G_LEAGUE_TABLE as isize + 72 * k) as usize + key_nation_record::SPLIT_SEASON_FLAG) != 0 {
            year = year.wrapping_add(1);
        }
        let sh = person.player().addr();
        let mut l: f64 = if age < 19 {
            f(rdata::F32_3)
        } else if age < 29 {
            f(rdata::F32_4)
        } else if age < 34 {
            if sh != 0 && read_i8(sh + player_sheet::GOALKEEPER) > 15 {
                f(rdata::F32_3)
            } else if sh != 0 && read_i8(sh + player_sheet::DEFENDER) > 15 {
                f(rdata::F32_2)
            } else {
                f(rdata::F32_1_5)
            }
        } else if np != 0 && sh == 0 && person.job_for_club() != 0xb {
            f(rdata::F32_4)
        } else {
            f(rdata::F32_1)
        };
        if read_i8(clause(contract::RELEGATION_RC)) > 0 || read_i8(clause(contract::NON_PROMOTION_RC)) > 0 {
            l *= d(rdata::F64_0_75);
        }
        if read_i8(clause(contract::MINIMUM_FEE_RC)) > 0 {
            l = f(rdata::F32_1);
        } else {
            let l500 = lv * 500;
            l *= l500 as f64 / if rep48 as i32 > l500 { rep48 as i32 } else { l500 } as f64;
            if age < 32 {
                l += (loy as f64 * d(rdata::F64_0_05) + d(rdata::F64_0_5)) * fr;
            }
        }
        if !sheet.is_null() {
            match status as u8 as i8 {
                1 => l *= d(rdata::F64_1_25),
                3 | 5 => {
                    l *= d(rdata::F64_0_75);
                    if age < 24 && lv > 10 && l < d(rdata::F64_2) {
                        l = f(rdata::F32_2);
                    }
                }
                4 => l *= d(rdata::F64_0_5),
                6 => {
                    l *= d(rdata::F64_0_5);
                    if l > d(rdata::F64_2) {
                        l = f(rdata::F32_2);
                    }
                }
                7 => l = f(rdata::F32_1),
                _ => {}
            }
            let r = sheet.current_reputation() as i32;
            if r > 2250 && r > lv * 675 {
                if r > 3250 && r > lv * 875 {
                    if age > 20 && age < 31 {
                        if loy < 15 {
                            l = f(rdata::F32_1);
                        } else {
                            l -= d(rdata::F64_2);
                        }
                    }
                } else if age <= 20 {
                    l -= d(rdata::F64_1);
                } else if age >= 27 {
                    if age > 29 && age < 32 && l < d(rdata::F64_2) {
                        l += d(rdata::F64_1);
                    }
                } else if loy < 5 {
                    l = f(rdata::F32_1);
                } else if l > f(rdata::F32_3) {
                    l -= d(rdata::F64_1);
                }
            }
        }
        if forced_kind != 0 {
            l = if l >= d(rdata::F64_3) {
                l + d(rdata::F64_1)
            } else if age < 27 {
                f(rdata::F32_5)
            } else if age < 30 {
                f(rdata::F32_4)
            } else {
                f(rdata::F32_3)
            };
        }
        let n = read_usize(club + club::NATION);
        if n != 0 && (read_i32(n) == read_i32(KEY_NATION_FINLAND) || read_i32(n) == read_i32(KEY_NATION_TURKEY)) {
            l *= d(rdata::F64_0_5);
            if l > d(rdata::F64_3) {
                l = f(rdata::F32_3);
            }
        }
        let vv = v as f64;
        let most = if vv < f(rdata::F32_500) {
            Some(rdata::F64_2)
        } else if vv < f(rdata::F32_1000) {
            Some(rdata::F64_3)
        } else if vv < f(rdata::F32_2000) {
            Some(rdata::F64_4)
        } else {
            None
        };
        if let Some(m) = most {
            if l >= d(m) {
                l = d(m);
            }
        }
        let max_years = read_i8(G_MAX_CONTRACT_LENGTH_YEARS) as i32;
        if l > (max_years - 1) as f64 {
            l = max_years as f64 - f(rdata::F32_1);
        } else if l < d(rdata::F64_1) {
            l = f(rdata::F32_1);
        }
        let kind_bits = |or: u8, and: u8| ct.set_contract_type(ct.contract_type() & and | or);
        let in_the_job_low = person.club_job().addr() == 0 && (level as i8) < 2;
        let mut add_years = true;
        if rep48 > (level as i8 as i16).wrapping_mul(500).wrapping_add(1250) {
            if in_the_job_low && ftol(fr * d(rdata::F64_4)) as u8 == 0 {
                kind_bits(0x82, 0xc2);
            } else {
                kind_bits(2, 0xc2);
            }
        } else if in_the_job_low && ftol(fr * d(rdata::F64_5)) as u8 == 0 {
            if age > 20 {
                kind_bits(0x82, 0xc2);
            } else {
                kind_bits(1, 0xc1);
                add_years = false;
            }
        } else {
            kind_bits(2, 0xc2);
        }
        if add_years {
            year = year.wrapping_add(ftol(l + d(rdata::F64_1)) as u16); // mkd3: the years of the contract
        }
        let mut end = [0u8; 8];
        let ep = end.as_mut_ptr() as usize;
        exe::new_date(ep, day_of_year as u32, year as u32);
        // a renewal never ends before the person's date with the club
        let pd = person.addr() + staff::DATE_EXPIRES_CLUB;
        let (py, ey) = (read_u16(pd + 2), read_u16(ep + 2));
        if (py > ey || (py == ey && read_i16(pd) > read_i16(ep))) && lookup() == club {
            exe::date_operator_3d(ep, tp, pd);
        }
        exe::date_operator_3d(ct.addr() + contract::CONTRACT_EXPIRES, tp, ep);
    }
    // the bonuses
    let vv = v as f64;
    let bonus = || ftol(vv * fr * f(F32_0_0250000004) + vv * f(F32_0_0250000004) + f(rdata::F32_25));
    let at = |o: usize| read_i8(sheet.addr() + o);
    if !sheet.is_null() {
        ct.set_goal_bonus(if at(player_sheet::ATTACKER) >= 15 { bonus() } else { -1 });
        let outfield = at(player_sheet::MIDFIELDER) >= 15 || at(player_sheet::ATTACKING_MIDFIELDER) >= 15 || at(player_sheet::ATTACKER) >= 15;
        ct.set_assist_bonus(if outfield { bonus() } else { -1 });
        ct.set_clean_sheet_bonus(if at(player_sheet::GOALKEEPER) == 20 { bonus() } else { -1 });
    }
    let kind = ct.contract_type();
    if vv < f(rdata::F32_125) && club != 0 && read_u8(club + club::PROFESSIONAL_STATUS) == 2 {
        ct.set_contract_type(kind | 0x80);
    } else if vv > f(F32_1250) && kind & 0x80 != 0 {
        ct.set_contract_type(kind & 0x7f);
    }
    let wage = ftol(vv);
    ct.set_wage(wage);
    // the return: the wage and the bonuses and clauses, these at most a tenth of the value
    let kind = ct.contract_type();
    let mut paid = wage as f64;
    if ((kind << 2) as i8 >> 2) == 1 {
        paid *= d(rdata::F64_0_5);
    }
    if kind & 0x80 != 0 {
        paid *= d(rdata::F64_1_05);
    }
    let mut extras = f(rdata::F32_0);
    if !sheet.is_null() {
        let outfield = at(player_sheet::MIDFIELDER) >= 15 || at(player_sheet::ATTACKING_MIDFIELDER) >= 15 || at(player_sheet::ATTACKER) >= 15;
        let keeper = at(player_sheet::GOALKEEPER) == 20;
        for i in 0..3 {
            let b = read_i32(ct.addr() + contract::GOAL_BONUS + 4 * i);
            if b > -1 {
                let x = (b / 10) as f64; // imul 0x66666667; sar 2: / 10
                let plus = if i < 2 { outfield } else { keeper };
                if plus {
                    extras += x;
                } else if i == 2 || keeper {
                    extras -= x;
                }
            }
        }
    }
    for c in 0..5 {
        if read_u8(clause(contract::NON_PROMOTION_RC + c)) != 0 {
            let fee = ct.release_fee() as i32;
            let k = match c {
                1 if fee < 500000 => rdata::F32_0_1000000015,
                1 if fee < 2500000 => rdata::F32_0_05000000075_AT_96D61C,
                1 => F32_0_0133333337,
                2 if fee < 2500000 => rdata::F32_0_1000000015,
                2 => F32_0_0666666701,
                _ if fee < 500000 => F32_0_0666666701,
                _ if fee < 2500000 => F32_0_0399999991,
                _ => F32_0_0199999996,
            };
            extras += vv * f(k);
        }
    }
    if sheet.is_null() {
        for o in contract::MINIMUM_FEE_RC..=contract::MANAGER_JOB_RC {
            write_u8(clause(o), 0);
        }
        ct.set_goal_bonus(-1);
        ct.set_assist_bonus(-1);
        ct.set_clean_sheet_bonus(-1);
    }
    let tenth = (vv * f(rdata::F32_0_1000000015)) as f32 as f64;
    ftol(if extras > tenth { tenth } else { extras } + paid)
}

/// Comparer for `staff_contract_set_to_desired_contract`: the original and the copy side by side from the same state of
/// the contract (0x50 bytes), the last contract looked up (`G_CONTRACT_LOOKUP_RESULT`), the game's generator, the text
/// buffer (the "no job for club" message), the person's `staff::JOB_FOR_CLUB` and the will and reasons the caller
/// passed; the contract, those and the returned value must match (the callers' comparers already run it twice).
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_set_to_desired_contract(this: usize, person: usize, club: usize, offer: i32, status: u32, job: u32, reasons: usize, will: usize) -> i32 {
    let person = Staff(person);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32, usize, usize) -> i32>(IDX_STAFF_CONTRACT_SET_TO_DESIRED_CONTRACT);
    let mut regions = vec![(this, 0x50usize), (G_CONTRACT_LOOKUP_RESULT, 4), (desired_contract_rdata::TEXT_BUFFER, 0x50), (person.addr() + staff::JOB_FOR_CLUB, 1)];
    regions.extend(crate::rng::STATE_REGIONS);
    if will != 0 {
        regions.push((will, 4));
    }
    if reasons != 0 {
        regions.push((reasons, 4));
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, person.addr(), club, offer, status, job, reasons, will),
        || crate::rng::without_counting(|| staff_contract_set_to_desired_contract(this, person.addr(), club, offer, status, job, reasons, will)));
    SCOREBOARD.record(same && r == n, || format!("staff_contract_set_to_desired_contract(person {:#x}, club {:x}, offer {}, status {:#x}, job {:#x}) original={} rust={} {:x?} x {:x?}",
        read_u32(person.addr()), club, offer, status, job, r, n, mo, mr));
    r
}

/// Returns (in st0, an f32 value) the default yearly wage of a person at a club: a second compiled copy of
/// `STAFF_CONTRACT::default_staff_contract` (0x864550) used to fill the wages that are not set (the regens' contract,
/// `set_default_wage`, and the initial shortlists, 0x83a1b0).
///
/// `(person, club, check, status, job, floor)`: first the contract (`exe::contract_manager_get_contract`, stored in
/// `G_CONTRACT_LOOKUP_RESULT`) and its club (none for a loan, type 5); the level = the club's reputation / 500 + 1 (byte
/// arithmetic; with a current club and a club of professional status 2, one less, within 1..4), or without a club the
/// world reputation of the player (or staff) sheet / 500, at least 1. The job is the person's job at the club (with a
/// current club), else his national job, else the club job, else 11 with a player sheet, else 9. By the job:
/// - 5..10 (staff): the level rises to the current club's (or the national side's, outside job 11) reputation / 500 + 1;
///   the wage = level × (current reputation³ × k with a staff sheet, else (id / staff count × 4 + 1) × m × 0.0001) ×
///   0.25, with k / m per job (2.5e-8 / 25000, 1e-8 / 10000, 1e-8 / 10000, 5e-9 / 5000, 1e-9 / 1000, 1e-9 / 1000) and the
///   floors and caps 750 (above 25000 → 150000), 500, 500, 275..5000, 250..1000, 200..1000; jobs 6 to 8 at another club
///   are worth 150000 when the current club's reputation / 500 + 4 is above the level (job 6 offering job 6 or 11 below
///   2500: × 1.35);
/// - 11, 12, 13, 15 (players): without a player sheet, a message and `rand() × 4 / 32768` × 550; with no status (−1), the
///   status by reputation and age (1 a star of 24..33, 2 above 0.75 × the club's, 4 above 0.5 or older than 24, 5 / 6
///   by potential × 50 against 0.75 × the club's); the base by position and loyalty (attacker 50000..65000, midfield
///   40000..52500, goalkeeper 20000..25000, defence 25000..30000; 8500 / 10000 for status 5..6 with low reputation,
///   10000 for coaches of 36 or more), raised 2.5..15 % in a major nation with the FIFA rules; score s = (current + home
///   + world + 4 × max(reputations, floor) + 50 × (ability + potential)) / 9 + the id draw, at least 1500 (× 1.05) for
///   a foreigner from another region; wage = base × s² × 1e-8, at least 175; × 0.75 or 0.5 by the nation's development;
///   × the player's reputation against the club's (0.85..1.15); caps by level (2500, 2750 or 3250 per level; 5000 soft
///   in England, Spain and Italy); a soft cap by position (40000..85000); × (1.375 − days at the club × 0.0125) for a
///   free agent (+60 days in season), at least 0.2;
/// - 14 and the others: 1000.
///
/// With a club and a player sheet: the club's offer by status (club reputation × 3 × (reputation × 0.0001..0.00017)⁶,
/// at most 1, × 0.05..1.5 by status and age; the average when it is higher), × 0.67..7.5 by status with grievance 0x10;
/// the nation's market (Japan floor level² × 10, USA × 0.75 and caps by age and league, the league-standard factor of
/// the minor nations and their soft caps); at most the wage of a current contract of status 7; the club's wage cap
/// (`exe::finance_get_max_wage`) when checking or without a club (reputation below 2250), and the ambition bonus
/// (1.05..1.15) of a star; the floors 50 / 75 (pro) or 100 / 75 (amateur, under 20). No club: the wage as computed.
/// Original: `staff_contract_calc_unset_default_wage` (0x862720, staff_contracts.cpp, stdcall, ret 0x18).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "stdcall" fn staff_contract_calc_unset_default_wage(person: usize, club: usize, check: u32, status: u32, job: u32, floor: u32) -> f64 {
    use unset_default_wage_rdata::*;
    let p = Staff(person);
    let c = Club(club);
    let contract = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
    write_usize(G_CONTRACT_LOOKUP_RESULT, contract.addr());
    let cur = if contract.is_null() || contract.contract_type() & 0x3f == 5 {
        Club(0)
    } else {
        club_at((contract.club() as i32) as isize)
    };
    let age = p.age() as i8;
    let sheet = p.player();
    let np = p.non_player();
    let frac = (read_i32(p.addr()) as f64 / read_i32(G_STAFF_COUNT) as f64) as f32; // fild id; fidiv count; fstp dword
    let job_arg = job as u8 as i8;
    let mut status = status; // [esp+0x40]: the byte can be rewritten (the dword goes to get_max_wage)
    // the level: the club's reputation / 500 + 1 (byte arithmetic)
    let mut lvl: i32;
    if !c.is_null() {
        lvl = ((c.reputation() as i32 / 500) as u8).wrapping_add(1) as i8 as i32;
        if !cur.is_null() && c.professional_status() == 2 {
            lvl -= 1;
            lvl = lvl.clamp(1, 4);
        }
    } else {
        let q = if !sheet.is_null() {
            Some(sheet.world_reputation() as i32 / 500)
        } else if !np.is_null() {
            Some(np.world_reputation() as i32 / 500)
        } else {
            None
        };
        lvl = match q {
            Some(q) => (if q < 1 { 1 } else { q }) as u8 as i8 as i32,
            None => 1,
        };
    }
    // the job the wage is computed for
    let jfc = p.job_for_club() as i8;
    let jfn = p.job_for_nation() as i8;
    let own_job: i8 = if !cur.is_null() && jfc != 0 {
        jfc
    } else if !p.national_job().is_null() && jfn != 0 {
        jfn
    } else if jfc != 0 {
        jfc
    } else if !sheet.is_null() {
        11
    } else if jfn != 0 {
        jfn
    } else {
        9
    };
    // the level rises to the current club's (or the national side's) reputation / 500 + 1 (staff jobs)
    let raise_level = |lvl: &mut i32| {
        if !cur.is_null() {
            let t = cur.reputation() as i32 / 500 + 1;
            if t > *lvl {
                *lvl = t;
                return;
            }
        }
        let nat = p.national_job();
        if !np.is_null() && !nat.is_null() && jfn != 11 {
            let t = nat.reputation() as i32 / 500 + 1;
            if t > *lvl {
                *lvl = t;
            }
        }
    };
    // the staff wage: level × (current reputation³ × k, or (the id draw × 4 + 1) × m × 0.0001) × 0.25
    let staff_wage = |lvl: i32, k: usize, m: usize| -> f32 {
        let x = if !np.is_null() {
            let r = np.current_reputation() as i32 as f64;
            r * r * r * read_f64(k)
        } else {
            ((frac as f64 * read_f32(rdata::F32_4) as f64 + read_f32(rdata::F32_1) as f64) * read_f32(m) as f64) * read_f64(rdata::F64_0_0001)
        };
        (lvl as f64 * x * read_f64(rdata::F64_0_25)) as f32
    };
    // another club's job when the current club's reputation / 500 + 4 is above the level
    let above_current = |lvl: i32| !cur.is_null() && cur != c && cur.reputation() as i32 / 500 + 4 > lvl;
    let a5_player_or_6 = job_arg == 11 || job_arg == 6;
    let mut v: f32;
    covered!("mkd_6 default wage");
    match own_job {
        5 => {
            raise_level(&mut lvl);
            v = staff_wage(lvl, F64_2_5ENEG_08, F32_25000);
            if read_f32(F32_750) <= v {
                if v > read_f32(F32_25000) {
                    v = 150000.0;
                }
            } else {
                v = 750.0;
            }
        }
        6 => {
            raise_level(&mut lvl);
            v = staff_wage(lvl, rdata::F64_1ENEG_08, rdata::F32_10000);
            if v < read_f32(rdata::F32_500) {
                v = 500.0;
            }
            if !cur.is_null() && cur != c {
                // at another club: 150000 unless the job offered is 6 or 11 and the level reaches the current club's
                if a5_player_or_6 && !above_current(lvl) {
                    if v < read_f32(rdata::F32_2500) {
                        v = (v as f64 * read_f64(rdata::F64_1_35)) as f32;
                    }
                } else {
                    covered!("mkd_6 another club's staff job");
                    v = 150000.0;
                }
            } else if !a5_player_or_6 && v > read_f32(F32_25000) {
                v = 150000.0;
            }
        }
        7 => {
            raise_level(&mut lvl);
            v = staff_wage(lvl, rdata::F64_1ENEG_08, rdata::F32_10000);
            if v < read_f32(rdata::F32_500) {
                v = 500.0;
            }
            if above_current(lvl) {
                v = 150000.0;
            } else if !a5_player_or_6 && v > read_f32(F32_25000) {
                v = 150000.0;
            }
        }
        8 => {
            raise_level(&mut lvl);
            v = staff_wage(lvl, F64_5ENEG_09, rdata::F32_5000);
            if read_f32(F32_275) <= v {
                if v > read_f32(rdata::F32_5000) {
                    v = 5000.0;
                }
            } else {
                v = 275.0;
            }
            if job_arg != 6 && job_arg != 11 && above_current(lvl) {
                v = 150000.0;
            }
        }
        9 | 10 => {
            raise_level(&mut lvl);
            v = staff_wage(lvl, F64_1ENEG_09, rdata::F32_1000);
            let floor_ = if own_job == 9 { rdata::F32_250 } else { rdata::F32_200 };
            if read_f32(floor_) <= v {
                if v > read_f32(rdata::F32_1000) {
                    v = 1000.0;
                }
            } else {
                v = read_f32(floor_);
            }
        }
        11 | 12 | 13 | 15 => {
            if sheet.is_null() {
                // the original writes the error into the game's text buffer and draws the wage
                let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
                format(G_TEXT_BUFFER, MSG_NO_PLAYER_DATA, p.first_name() as u32, p.second_name() as u32, read_i32(p.addr()));
                let r = exe::crt_rand();
                let q = (r as i64 * 4) / 0x8000; // __allmul, __alldiv
                v = (q as i32 as f64 * read_f32(F32_550) as f64) as f32;
            } else {
                v = unset_default_wage_player(p, c, cur, sheet, age, frac, lvl, job_arg, floor as u8 as i8, &mut status);
            }
        }
        _ => v = 1000.0,
    }
    if c.is_null() {
        return v as f64;
    }
    if !sheet.is_null() {
        // what the club would pay by squad status: club reputation × 3 × (reputation × k)⁶ (at most 1) × the status' factor
        let rep = c.reputation() as i32;
        let x = if c.professional_status() == 1 {
            rep as f64 * read_f64(F64_0_00017)
        } else if c.reputation() > 0x8ca {
            rep as f64 * read_f64(F64_0_00012)
        } else {
            rep as f64 * read_f64(rdata::F64_0_0001)
        };
        let mut pw = game_pow(x, read_f64(rdata::F64_6));
        if pw > read_f64(rdata::F64_1) {
            pw = 1.0;
        }
        let r3 = (rep * 3) as f64 * pw;
        let major = || club_is_in_major_nation(c.addr()) != 0;
        let st = status as u8 as i8;
        let y: Option<f64> = match st {
            1 => Some(r3 * read_f64(if age < 0x14 { rdata::F64_0_6 } else if age < 0x18 || !major() { rdata::F64_1_25 } else { rdata::F64_1_5 })),
            2 => Some(if age < 0x14 { r3 * read_f64(rdata::F64_0_3) } else if age < 0x18 || !major() { r3 * read_f64(rdata::F64_0_6) } else { r3 }),
            3 => {
                if age < 0x14 {
                    let y0 = r3 * read_f64(rdata::F64_0_25);
                    if (v as f64) < y0 {
                        v = ((v as f64 + y0) * read_f64(rdata::F64_0_5)) as f32;
                    }
                }
                Some(r3 * read_f64(if age < 0x18 || !major() { rdata::F64_0_3 } else { rdata::F64_0_6 }))
            }
            4 => Some(r3 * read_f64(if age < 0x14 { rdata::F64_0_1 } else if age < 0x18 || !major() { rdata::F64_0_15 } else { rdata::F64_0_3 })),
            5 => Some(r3 * read_f64(rdata::F64_0_05)),
            6 => Some(r3 * read_f64(rdata::F64_0_02)),
            _ => None,
        };
        if let Some(y) = y {
            if (v as f64) < y {
                covered!("mkd_6 club's offer by status");
                v = ((v as f64 + y) * read_f64(rdata::F64_0_5)) as f32; // mkd6: the average with the club's offer
            }
        }
        let k = (st as i32 - 1) as u32;
        let c2 = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
        if !c2.is_null() && (c2.grievances() as u8) & 0x10 != 0 && k <= 5 {
            let m = match k {
                0 | 1 => rdata::F64_0_67,
                2 => rdata::F64_0_85,
                3 => rdata::F64_2_5,
                4 => rdata::F64_3_5,
                _ => rdata::F64_7_5,
            };
            v = (v as f64 * read_f64(m)) as f32;
        }
    }
    // the market of the club's nation
    let nat = c.nation();
    if !nat.is_null() {
        let cj = p.club_job();
        let pn = p.nation();
        let applies = if !cj.is_null() {
            cj.nation() == nat && (pn == nat || p.loyalty() > 3 || p.ambition() < 0xf)
        } else {
            pn == nat
        };
        if applies {
            covered!("mkd_6 nation's market");
            let r50 = c.reputation() as i32 / 50;
            let id = read_u32(nat.addr());
            let is = |k: usize| id == read_u32(k);
            let prof = c.professional_status() == 1;
            if is(KEY_NATION_JAPAN) {
                let fl = lvl * lvl * 10;
                if (v as f64) < fl as f64 {
                    v = fl as f32;
                }
                v = unset_default_wage_soft_cap(v, read_f64(rdata::F64_7500), read_f64(rdata::F64_0_25));
            } else if is(KEY_NATION_ENGLAND) || is(KEY_NATION_ITALY) || is(KEY_NATION_SPAIN) {
            } else if is(KEY_NATION_UNITED_STATES) {
                v = (v as f64 * read_f64(rdata::F64_0_75)) as f32;
                let d = c.division().addr();
                if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_AMERICAN_A_LEAGUE) {
                    v = unset_default_wage_soft_cap(v, read_f64(F64_250), read_f64(rdata::F64_0_15));
                } else if pn == nat {
                    let young = age <= 0x1f && (age >= 0x18 || sheet.is_null() || sheet.world_reputation() >= 0xea6);
                    v = unset_default_wage_soft_cap(v, read_f64(if young { F64_2750 } else { rdata::F64_1250 }), read_f64(rdata::F64_0_2));
                } else {
                    v = unset_default_wage_soft_cap(v, read_f64(F64_3250), read_f64(rdata::F64_0_25));
                }
            } else {
                let scale = |v: f32, base: f64| (v as f64 * unset_default_wage_standard_factor(nat, r50, base)) as f32;
                if is(KEY_NATION_SOUTH_KOREA) {
                    v = scale(v, read_f64(rdata::F64_0_25));
                    v = unset_default_wage_soft_cap(v, read_f64(rdata::F64_500), read_f64(rdata::F64_0_5));
                    v = unset_default_wage_soft_cap(v, read_f64(F64_1500), read_f64(rdata::F64_0_5));
                } else if is(KEY_NATION_SCOTLAND) {
                    v = scale(v, read_f64(rdata::F64_0_35));
                    v = unset_default_wage_soft_cap(v, read_f64(rdata::F64_10000), read_f64(rdata::F64_0_25));
                } else if is(KEY_NATION_GERMANY) || is(KEY_NATION_FRANCE) || is(KEY_NATION_HOLLAND) {
                    v = scale(v, read_f64(rdata::F64_0_75));
                    v = unset_default_wage_soft_cap(v, read_f64(F64_20000), read_f64(rdata::F64_0_25));
                } else if is(KEY_NATION_SWEDEN) {
                    v = scale(v, read_f64(rdata::F64_0_25));
                    v = if prof {
                        unset_default_wage_soft_cap(v, read_f64(rdata::F64_500), read_f64(rdata::F64_0_2))
                    } else {
                        unset_default_wage_soft_cap(v, read_f64(rdata::F64_150), read_f64(rdata::F64_0_2))
                    };
                } else if is(KEY_NATION_WALES) || is(KEY_NATION_FINLAND) || is(KEY_NATION_REPUBLIC_OF_IRELAND) || is(KEY_NATION_NORTHERN_IRELAND) || is(KEY_NATION_AUSTRALIA) {
                    v = if prof {
                        unset_default_wage_soft_cap(v, read_f64(rdata::F64_200), read_f64(rdata::F64_0_15))
                    } else {
                        unset_default_wage_soft_cap(v, read_f64(rdata::F64_100), read_f64(rdata::F64_0_15))
                    };
                } else if is(KEY_NATION_DENMARK) || is(KEY_NATION_NORWAY) || is(KEY_NATION_POLAND) {
                    v = scale(v, read_f64(rdata::F64_0_5));
                    v = if prof {
                        unset_default_wage_soft_cap(v, read_f64(F64_350), read_f64(rdata::F64_0_1))
                    } else {
                        unset_default_wage_soft_cap(v, read_f64(F64_250), read_f64(rdata::F64_0_1))
                    };
                } else if is(KEY_NATION_BELGIUM) {
                    v = scale(v, read_f64(rdata::F64_0_45));
                    v = if prof {
                        unset_default_wage_soft_cap(v, read_f64(rdata::F64_500), read_f64(rdata::F64_0_2))
                    } else {
                        unset_default_wage_soft_cap(v, read_f64(F64_250), read_f64(rdata::F64_0_2))
                    };
                } else if is(KEY_NATION_PORTUGAL) || is(KEY_NATION_CROATIA) {
                    v = scale(v, read_f64(rdata::F64_0_5));
                    v = if prof {
                        unset_default_wage_soft_cap(v, read_f64(rdata::F64_500), read_f64(rdata::F64_0_2))
                    } else {
                        unset_default_wage_soft_cap(v, read_f64(F64_250), read_f64(rdata::F64_0_2))
                    };
                } else if is(KEY_NATION_BRAZIL) || is(KEY_NATION_ARGENTINA) {
                    v = scale(v, read_f64(rdata::F64_0_45));
                    v = unset_default_wage_soft_cap(v, read_f64(F64_25000), read_f64(rdata::F64_0_25));
                } else {
                    // the other nations: × 0.3, the soft cap tested on the unrounded product
                    let x = v as f64 * read_f64(rdata::F64_0_3);
                    v = x as f32;
                    if x > read_f64(rdata::F64_15000) {
                        v = ((v as f64 - read_f64(rdata::F64_15000)) * read_f64(rdata::F64_0_25) + read_f64(rdata::F64_15000)) as f32;
                    }
                }
            }
        }
    }
    if !sheet.is_null() {
        // at most the wage of a current contract of squad status 7
        if !cur.is_null() && c != cur {
            let k = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, cur);
            if k != 0 && read_u8(k + contract::SQUAD_STATUS) & 0xf0 == 0x70 {
                let w = read_i32(k + contract::WAGE) as f64;
                if v as f64 > w {
                    v = w as f32;
                }
            }
        }
        let mut ambition = check == 0 && !cur.is_null();
        if check != 0 || (cur.is_null() && sheet.current_reputation() < 0x8ca) {
            // the club's wage cap
            let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), c);
            if fin == 0 {
                return v as f64;
            }
            let mw = exe::finance_get_max_wage(fin, p, status) as f64;
            if mw < v as f64 {
                covered!("mkd_6 club's wage cap");
                v = mw as f32;
            }
            ambition = false;
        }
        if ambition
            && p.ambition() > 10
            && p.loyalty() < 6
            && sheet.current_reputation() > 0x1a5e
            && (sheet.attacker() == 0x14 || sheet.attacking_midfielder() == 0x14 || sheet.midfielder() == 0x14)
            && age > 0x14
        {
            exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, cur); // the result is not used
            let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), c);
            if fin == 0 {
                return v as f64;
            }
            let mw = exe::finance_get_max_wage(fin, p, status) as f64;
            if v as f64 * read_f64(rdata::F64_1_25) < mw {
                let m = if sheet.attacking_midfielder() == 0x14 {
                    rdata::F64_1_1
                } else if sheet.midfielder() == 0x14 {
                    rdata::F64_1_05
                } else {
                    rdata::F64_1_15
                };
                v = (v as f64 * read_f64(m)) as f32;
            }
        }
    }
    // the floors
    if c.professional_status() != 1 {
        if c.reputation() > 0x8ca && (v as f64) < read_f32(rdata::F32_75) as f64 {
            return 75.0;
        }
        if (v as f64) < read_f32(rdata::F32_50) as f64 {
            return 50.0;
        }
        v as f64
    } else {
        if age < 0x14 && (v as f64) < read_f32(rdata::F32_75) as f64 {
            return 75.0;
        }
        if (v as f64) < read_f32(rdata::F32_100) as f64 {
            v = 100.0;
        }
        v as f64
    }
}

/// Comparer for `staff_contract_calc_unset_default_wage`: original and copy side by side over
/// `G_CONTRACT_LOOKUP_RESULT`, the start of the text buffer (the no-player message) and the generator's state.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "stdcall" fn compare_staff_contract_calc_unset_default_wage(person: usize, club: usize, check: u32, status: u32, job: u32, floor: u32) -> f64 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32, u32, u32, u32) -> f64>(IDX_STAFF_CONTRACT_CALC_UNSET_DEFAULT_WAGE);
    let est = crate::rng::STATE_REGIONS;
    let regions = [(G_CONTRACT_LOOKUP_RESULT, 4usize), (unset_default_wage_rdata::G_TEXT_BUFFER, 0x80), est[0], est[1]];
    let (r, n, same, mo, mr) = crate::comparison::side_by_side_light(&regions,
        || o(person, club, check, status, job, floor),
        || crate::rng::without_counting(|| staff_contract_calc_unset_default_wage(person, club, check, status, job, floor)));
    SCOREBOARD.record(same && r.to_bits() == n.to_bits(), || format!("staff_contract_calc_unset_default_wage(person {:#x}, club {:#x}, {}, {}, {}, {}) original={} rust={} {:x?} x {:x?}",
        person, club, check, status as u8 as i8, job as u8, floor as u8, r, n, mo, mr));
    r
}

/// The league of a club (`club::DIVISION` in `G_COMP_OBJECT_TABLE`, a league: `COMP_TYPE` 2) answers a virtual slot
/// of `league_vtable` (`HAS_AUTOMATIC_PROMOTION`, `HAS_RELEGATION`) with ≠ 0; no club, division or league: false.
pub(crate) unsafe fn initial_contract_league_has(club: Club, slot: usize) -> bool {
    let div = club.division().addr();
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
pub(crate) unsafe fn initial_contract_bonus(wage: i32) -> i32 {
    let r = exe::utils_random(wage / 40);
    (wage / 20).wrapping_sub(r).wrapping_add(25)
}

/// A date (day i16 at +0, year u16 at +2) before another, compared as the original does: the year unsigned, the day
/// signed.
pub(crate) unsafe fn fill_slot_date_before(a: usize, b: usize) -> bool {
    read_u16(a + 2) < read_u16(b + 2) || (read_u16(a + 2) == read_u16(b + 2) && read_i16(a) < read_i16(b))
}

/// The multiplier of one release clause of the contract (index `k` from `contract::NON_PROMOTION_RC`: 0 non-promotion,
/// 1 minimum fee, 2 non-playing, 3 relegation, 4 manager job), from the release fees of the contract (`cf`) and of the
/// offer (`of`) in the bands below 1 million, 5 million (7.5 million in case 0) and above, the offer's own clauses and
/// the person's age. Case 0 reads the offer's minimum fee clause, not its non-promotion one (as in the original).
pub(crate) unsafe fn contract_value_clause_factor(k: usize, offer: Contract, cf: i32, age: i8) -> f64 {
    use contract_value_rdata::*;
    const M1: i32 = 1_000_000;
    const M5: i32 = 5_000_000;
    const M7_5: i32 = 7_500_000;
    let f = |a: usize| read_f32(a) as f64;
    let of = || offer.release_fee() as i32;
    // the three bands of cases 1 (24..28), 3 and 0: equal or higher offer fee 1.75 / 1.5 / 1.2, lower 1.7 / 1.45 / 1.15
    let banded = |top: i32| {
        let o = of();
        if cf < M1 && o < M1 {
            if o >= cf { f(rdata::F32_1_75) } else { f(F32_1_700000048) }
        } else if cf < top && o < top {
            if o >= cf { f(rdata::F32_1_5) } else { f(F32_1_450000048) }
        } else if o >= cf {
            f(F32_1_200000048)
        } else {
            f(rdata::F32_1_149999976)
        }
    };
    match k {
        0 => {
            if offer.minimum_fee_rc() as i8 > 0 { banded(M7_5) } else { f(F32_1_200000048) }
        }
        1 => {
            let o = of();
            if offer.minimum_fee_rc() as i8 <= 0 {
                if cf < M1 && o < M1 { f(F32_1_200000048) } else if cf < M5 && o < M5 { f(rdata::F32_1_049999952) } else { f(F32_1_00999999) }
            } else if age > 28 {
                if cf < M1 && o < M1 { f(rdata::F32_1_100000024) } else { f(rdata::F32_1_049999952) }
            } else if age < 24 {
                if cf < M1 && o < M1 {
                    if o < cf { f(F32_1_950000048) } else { 2.0 } // the original doubles (fadd st0, st0)
                } else if cf < M5 && o < M5 {
                    if o >= cf { f(rdata::F32_1_5) } else { f(F32_1_450000048) }
                } else {
                    f(rdata::F32_1_25)
                }
            } else {
                banded(M5)
            }
        }
        2 => {
            if cf < M5 && of() < M5 { f(F32_1_799999952) } else { f(F32_1_200000048) }
        }
        3 => {
            if offer.relegation_rc() as i8 <= 0 { f(rdata::F32_1_100000024) } else { banded(M5) }
        }
        _ => read_f64(rdata::F64_1_25),
    }
}

/// The player part of `staff_contract_default_staff_contract` (a player sheet, jobs 11, 12, 13, 15): the squad status
/// when unknown (written into the low byte of `status`), the base by `kind`, the job and the position, and the
/// adjustments up to the decline by days at the club (see the header of the routine).
pub(crate) unsafe fn player_default_wage(person: usize, club: usize, cur: usize, status: &mut u32, asked_job: i8, kind: i8, level: i32, frac: f32) -> f32 {
    let person = Staff(person);
    let club = Club(club);
    let cur = Club(cur);
    use default_wage_rdata::*;
    let f = |a: usize| read_f32(a) as f64;
    let d = |a: usize| read_f64(a);
    let p = person;
    let c = club;
    let age = p.age() as i8;
    let sheet = person.player().addr();
    let at = |k: usize| read_i8(sheet + k);
    let w16 = |k: usize| read_i16(sheet + k);
    let sb = w16(player_sheet::CURRENT_REPUTATION);
    let sd = w16(player_sheet::WORLD_REPUTATION);
    let s9 = w16(player_sheet::HOME_REPUTATION);
    let loyalty = p.loyalty();
    // the squad status when unknown
    if *status as u8 as i8 == -1 && !club.is_null() {
        let rep = c.reputation();
        let new = if sb > rep && age >= 24 && age < 34 {
            1
        } else {
            let x = rep as f64 * d(rdata::F64_0_75);
            if sb as f64 > x && age >= 21 {
                2
            } else if (sb as f64 > rep as f64 * d(rdata::F64_0_5) && age >= 17) || age > 24 {
                4
            } else if (w16(player_sheet::POTENTIAL_ABILITY) as i32 * 50) as f64 > x {
                5
            } else {
                6
            }
        };
        *status = (*status & !0xff) | new;
    }
    let st_s = *status as u8 as i8;
    let v75 = f(F32_75000);
    let v92 = f(F32_92500);
    let v105 = f(F32_105000);
    // the base
    let base = if kind == 3 {
        F32_85000
    } else if kind == 1 {
        if loyalty < 15 && p.pressure() > 15 { F32_92500 } else { F32_75000 }
    } else if kind == 2 {
        if age > 22 && age < 32 { F32_75000 } else { F32_82500 }
    } else if (asked_job == 13 || asked_job == 15) && age > 35 {
        rdata::F32_10000
    } else if (st_s == 6 || st_s == 5) && sb < 2500 {
        F32_9350
    } else if (st_s == 6 || st_s == 5) && sb < 6250 {
        F32_11000
    } else if at(player_sheet::ATTACKER) >= 15 {
        if sb > 6750 && age < 31 {
            if loyalty < 15 { F32_82500 } else { F32_70000 }
        } else {
            F32_55000
        }
    } else if at(player_sheet::DEFENSIVE_MIDFIELDER) >= 15 || at(player_sheet::ATTACKING_MIDFIELDER) >= 15 || at(player_sheet::MIDFIELDER) >= 15 {
        if sb > 7250 && age < 31 {
            if loyalty < 14 { F32_67750 } else { F32_60000 }
        } else {
            F32_44000
        }
    } else if at(player_sheet::GOALKEEPER) == 20 {
        if sd > 7250 && loyalty < 3 && age < 34 { F32_37500 } else { F32_22000 }
    } else if at(player_sheet::SWEEPER) < 15 && at(player_sheet::DEFENDER) < 15 {
        F32_27500
    } else if sb > 7500 && loyalty < 6 && age < 33 {
        F32_50000
    } else {
        F32_27500
    };
    let mut st = f(base); // the x87 top of the stack (not always rounded to a float)
    let mut value = st as f32;
    // a young player not tied to the club or its nation, with a world reputation above his loyalty and ambition
    let mut free = true;
    if !club.is_null() {
        let cn = club.nation().addr();
        if cn != 0 && (c.reputation() < 5750 || read_i8(cn + nation::LEAGUE_STANDARD) < 12) {
            let e = person.club_job();
            if e == club || (!e.is_null() && e.nation().addr() == cn) {
                free = false;
            }
        }
    }
    if age > 18 && age < 32 && free && sd as i32 > (loyalty as i32 + 230) * 25 - p.ambition() as i32 * 10 {
        let new = if at(player_sheet::GOALKEEPER) == 20 {
            if st < v75 { Some(v75) } else { None }
        } else if st == v92 {
            Some(v105)
        } else if st == v75 {
            Some(if sd > 7250 { v105 } else { v92 })
        } else if sd > 7250 {
            if st < v92 { Some(v92) } else { None }
        } else if st < v75 {
            Some(v75)
        } else {
            None
        };
        if let Some(n) = new {
            covered!("mkd_2 young star");
            st = n;
            value = n as f32;
        }
    }
    // the FIFA rules at a club of a major nation
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && st > f(F32_11000) && !club.is_null() {
        st = value as f64;
        if club_is_in_major_nation(club.addr()) != 0 {
            if sb > 6750 {
                st *= if loyalty == 1 { d(rdata::F64_1_15) } else if loyalty < 6 { d(F64_1_125) } else { d(rdata::F64_1_1) };
            } else if sb > 4250 {
                st *= d(rdata::F64_1_025);
            }
        }
    }
    // the reputation sum
    let mut m = if s9 > sb { s9 } else { sb };
    if sd > m {
        m = sd;
    }
    if sd < 1750 && m > 3250 {
        m = (3250 - ftol((m as f64 - d(F64_3250)) * d(rdata::F64_NEG_0_2))) as i16;
    }
    if kind == 1 {
        if m < 7250 {
            m = 7250;
        }
    } else if kind == 2 {
        if m < 6750 {
            m = 6750;
        }
    } // (the original's third test, kind == 0x14244, can never hold)
    if (st == v75 || st == v92 || st == v105) && m < 7250 {
        m = 7250;
    }
    let total = sb as i32 + s9 as i32 + sd as i32 + 4 * m as i32
        + 50 * (w16(player_sheet::POTENTIAL_ABILITY) as i32 + w16(player_sheet::CURRENT_ABILITY) as i32);
    let mut u = (total / 9).wrapping_add(ftol(frac as f64 * f(rdata::F32_8) - f(rdata::F32_4))) as u32;
    let sn = person.nation();
    if !sn.is_null() {
        let cn = club.nation().addr();
        if cn != 0 {
            let region = read_i8(cn + nation::ACTUAL_REGION);
            if (sn.actual_region() as i8) != region {
                let foreign_too = if cur.is_null() {
                    true
                } else {
                    let kn = cur.nation();
                    !kn.is_null() && (kn.actual_region() as i8) != region
                };
                if foreign_too {
                    if u < 1500 {
                        u = 1500;
                    } else {
                        u = ftol(u as u64 as f64 * d(rdata::F64_1_05)) as u32;
                    }
                }
            }
        }
    }
    if u <= 1 {
        u = 1;
    }
    let product = st * u.wrapping_mul(u) as u64 as f64 * d(rdata::F64_1ENEG_08); // mkd2: the reputation squared
    value = product as f32;
    if product < f(F32_175) {
        value = 175.0;
    }
    // the state of development of the nation
    let curn = Nation(if !cur.is_null() { cur.nation().addr() } else { 0 });
    let k: i8 = if !curn.is_null() {
        curn.state_of_development() as i8
    } else if sd > 4250 {
        1
    } else if !sn.is_null() {
        sn.state_of_development() as i8
    } else {
        3
    };
    if k != 1 {
        value = (value as f64 * if k == 2 { f(rdata::F32_0_75) } else { f(rdata::F32_0_5) }) as f32;
    }
    // the world reputation against the club's level
    if !club.is_null() && c.reputation() != 0 {
        let mut x = sd as f64;
        if c.reputation() < 2500 {
            x *= f(F32_0_0004);
        } else {
            x /= (level * 500) as f64;
        }
        if x < d(rdata::F64_0_85) {
            x = f(rdata::F32_0_8500000238);
        }
        if x > d(rdata::F64_1_15) {
            x = f(rdata::F32_1_149999976);
        }
        value = (x * value as f64) as f32;
    }
    // the level caps
    let squeeze = |value: &mut f32, cap: i32| {
        if *value as f64 > cap as f64 {
            *value = ((*value as f64 - cap as f64) * d(rdata::F64_0_1) + cap as f64) as f32;
        }
    };
    if level < 10 || level < 15 {
        let cap = if level < 10 { level * 2500 } else { level * 2750 };
        if value as f64 > cap as f64 {
            value = cap as f32;
        }
    } else {
        let cn = club.nation().addr();
        let key = if cn != 0 { read_i32(cn) } else { 0 };
        if cn != 0 && (key == read_i32(KEY_NATION_ENGLAND) || key == read_i32(KEY_NATION_SPAIN) || key == read_i32(KEY_NATION_ITALY)) {
            squeeze(&mut value, if club.plc() != 0 { level * 5250 } else { level * 7500 });
        } else {
            squeeze(&mut value, level * 3250);
        }
    }
    // the cap of the position
    let mut cap: f32 = if at(player_sheet::ATTACKER) == 20 {
        150000.0
    } else if at(player_sheet::ATTACKING_MIDFIELDER) == 20 || at(player_sheet::MIDFIELDER) == 20 || at(player_sheet::DEFENSIVE_MIDFIELDER) == 20 {
        125000.0
    } else if at(player_sheet::DEFENDER) == 20 {
        100000.0
    } else {
        80000.0
    };
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && !club.is_null() && club_is_in_major_nation(club.addr()) != 0 && sb > 7250 {
        cap = (cap as f64 * d(rdata::F64_1_1)) as f32;
    }
    if value > cap {
        value = ((value as f64 - cap as f64) * d(rdata::F64_0_2) + cap as f64) as f32;
    }
    // the decline by the days at the club
    if person.club_job().addr() == 0 {
        let settled = age >= 33
            && p.int_apps() != 0
            && at(player_sheet::WORK_RATE) as i32 + p.determination() as i32 <= 30
            && read_i32(person.addr()) % 15 != 0
            && c.reputation() <= 4500;
        if !settled {
            covered!("mkd_2 days at the club");
            let days = exe::date_days_since(G_CURRENT_DATE, person.date_joined_club() as u32, person.date_joined_club_leap_year() as u32);
            let mut x = if days as f64 > f(rdata::F32_250) { 250.0f32 } else { days as f32 } as f64;
            if exe::club_is_in_season(club, G_CURRENT_DATE) != 0 {
                x += f(rdata::F32_60);
            }
            if x > f(F32_30) {
                x = (x - f(F32_30)) * d(rdata::F64_0_2) + d(rdata::F64_30);
            }
            let mut k = f(F32_1_375) - x * f(F32_0_0125); // mkd2: the decline by days
            if f(F32_0_2000000030) > k {
                k = f(F32_0_2000000030);
            }
            value = (k * value as f64) as f32;
        }
    }
    value
}

/// The part of `staff_contract_default_staff_contract` by the club's nation `cn` (the person works there or is from
/// there; see the header of the routine).
pub(crate) unsafe fn default_wage_by_nation(person: usize, club: usize, cn: usize, sn: usize, sheet: usize, level: i32, value: f32) -> f32 {
    let cn = Nation(cn);
    let sheet = PlayerSheet(sheet);
    let person = Staff(person);
    let club = Club(club);
    use default_wage_rdata::*;
    let d = |a: usize| read_f64(a);
    let c = club;
    let mut value = value;
    let q = c.reputation() as i32 / 50;
    let key = read_i32(cn.addr());
    let is = |k: usize| key == read_i32(k);
    let league = cn.league_standard() as i8;
    let over = q as i16 as i32 > league as i32 * 10;
    // the club's reputation against the league standard, within k..1
    let factor = |k: usize| {
        let mut x = d(k);
        if over {
            let dv = if league < 1 { 1 } else { league as i32 };
            x = q as i16 as f64 / dv as f64 * d(rdata::F64_0_1) * d(k);
            if x > d(rdata::F64_1) {
                x = d(rdata::F64_1);
            }
        }
        x
    };
    let scale = |value: &mut f32, x: f64| *value = (*value as f64 * x) as f32;
    let squeeze = |value: &mut f32, cap: usize, k: usize| {
        if *value as f64 > d(cap) {
            *value = ((*value as f64 - d(cap)) * d(k) + d(cap)) as f32;
        }
    };
    let pro = c.professional_status() == 1;
    let age = person.age() as i8;
    if is(KEY_NATION_JAPAN) {
        let floor = level * level * 10;
        if (value as f64) < floor as f64 {
            value = floor as f32;
        }
        squeeze(&mut value, rdata::F64_7500, rdata::F64_0_25);
    } else if is(KEY_NATION_ENGLAND) || is(KEY_NATION_ITALY) || is(KEY_NATION_SPAIN) {
    } else if is(KEY_NATION_UNITED_STATES) {
        scale(&mut value, d(rdata::F64_0_75));
        let comp = club.division().addr();
        if comp != 0 && read_i32(comp) == read_i32(KEY_CLUB_COMP_AMERICAN_A_LEAGUE) {
            squeeze(&mut value, F64_250, rdata::F64_0_15);
        } else if sn == cn.addr() {
            let sh = person.player();
            if age > 31 || (age < 24 && !sh.is_null() && sh.world_reputation() < 3750) {
                squeeze(&mut value, rdata::F64_1250, rdata::F64_0_2);
            } else {
                squeeze(&mut value, F64_2750, rdata::F64_0_2);
            }
        } else {
            squeeze(&mut value, F64_3250, rdata::F64_0_25);
        }
    } else if is(KEY_NATION_SOUTH_KOREA) {
        scale(&mut value, factor(rdata::F64_0_25));
        squeeze(&mut value, rdata::F64_500, rdata::F64_0_5);
        squeeze(&mut value, F64_1500, rdata::F64_0_5);
    } else if is(KEY_NATION_SCOTLAND) {
        scale(&mut value, factor(rdata::F64_0_25));
        if over {
            squeeze(&mut value, rdata::F64_10000, rdata::F64_0_25);
        } else if !sheet.is_null() {
            let sd = sheet.world_reputation();
            if sd < 1250 {
                squeeze(&mut value, rdata::F64_1250, rdata::F64_0_25);
            } else if (age < 20 || age > 32) && sd < 2750 {
                squeeze(&mut value, F64_3250, rdata::F64_0_25);
            } else {
                squeeze(&mut value, F64_7250, rdata::F64_0_25);
            }
        } else {
            squeeze(&mut value, F64_7250, rdata::F64_0_25);
        }
    } else if is(KEY_NATION_GERMANY) || is(KEY_NATION_FRANCE) || is(KEY_NATION_HOLLAND) {
        scale(&mut value, factor(rdata::F64_0_75));
        squeeze(&mut value, F64_20000, rdata::F64_0_25);
    } else if is(KEY_NATION_SWEDEN) {
        scale(&mut value, factor(rdata::F64_0_25));
        if pro { squeeze(&mut value, rdata::F64_500, rdata::F64_0_2) } else { squeeze(&mut value, rdata::F64_150, rdata::F64_0_2) }
    } else if is(KEY_NATION_WALES) || is(KEY_NATION_FINLAND) || is(KEY_NATION_REPUBLIC_OF_IRELAND) || is(KEY_NATION_NORTHERN_IRELAND) || is(KEY_NATION_AUSTRALIA) {
        if pro { squeeze(&mut value, rdata::F64_200, rdata::F64_0_15) } else { squeeze(&mut value, rdata::F64_100, rdata::F64_0_15) }
    } else if is(KEY_NATION_DENMARK) || is(KEY_NATION_NORWAY) {
        scale(&mut value, factor(rdata::F64_0_5));
        if pro { squeeze(&mut value, F64_350, rdata::F64_0_1) } else { squeeze(&mut value, F64_250, rdata::F64_0_1) }
    } else if is(KEY_NATION_BELGIUM) || is(KEY_NATION_PORTUGAL) || is(KEY_NATION_CROATIA) || is(KEY_NATION_POLAND) {
        scale(&mut value, factor(if is(KEY_NATION_BELGIUM) { rdata::F64_0_45 } else { rdata::F64_0_5 }));
        if pro { squeeze(&mut value, rdata::F64_500, rdata::F64_0_2) } else { squeeze(&mut value, F64_250, rdata::F64_0_2) }
    } else if is(KEY_NATION_BRAZIL) || is(KEY_NATION_ARGENTINA) {
        scale(&mut value, factor(rdata::F64_0_45));
        squeeze(&mut value, F64_25000, rdata::F64_0_25);
    } else {
        covered!("mkd_2 other nation");
        let pv = value as f64 * d(rdata::F64_0_3); // mkd2: the other nations
        value = pv as f32;
        if pv > d(rdata::F64_15000) {
            value = ((value as f64 - d(rdata::F64_15000)) * d(rdata::F64_0_25) + d(rdata::F64_15000)) as f32;
        }
    }
    value
}

/// `(v - t) × m + t` above `t`, in the f32 slot of the original ([esp+0x14]).
pub(crate) fn unset_default_wage_soft_cap(v: f32, t: f64, m: f64) -> f32 {
    if v as f64 > t {
        ((v as f64 - t) * m + t) as f32
    } else {
        v
    }
}

/// The league-standard factor of the minor nations: `base`, or (reputation / 50) / standard × 0.1 × `base` (at most 1)
/// when the club's reputation / 50 is above 10 × the standard of the nation (`nation::LEAGUE_STANDARD`, at least 1).
pub(crate) unsafe fn unset_default_wage_standard_factor(nation: Nation, r50: i32, base: f64) -> f64 {
    let ls = nation.league_standard() as i8;
    if r50 <= ls as i32 * 10 {
        return base;
    }
    let den = if ls < 1 { 1 } else { ls as i32 };
    let k = (r50 as f64 / den as f64) * read_f64(rdata::F64_0_1) * base;
    if k > read_f64(rdata::F64_1) {
        read_f64(rdata::F64_1)
    } else {
        k
    }
}

/// The player's wage of `staff_contract_calc_unset_default_wage` (jobs 11, 12, 13 and 15, with a player sheet): the
/// squad status by reputation and age when it is −1 (written back into `status`), the base by position and loyalty, the
/// score of the reputations and abilities, the nation's development, the reputation ratio, the caps by level and
/// position and the free agent's factor.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn unset_default_wage_player(p: Staff, c: Club, cur: Club, sheet: PlayerSheet, age: i8, frac: f32, lvl: i32, job_arg: i8, floor: i8, status: &mut u32) -> f32 {
    use unset_default_wage_rdata::*;
    let mut st = *status as u8 as i8;
    if st == -1 && !c.is_null() {
        // the squad status the club would give, by reputation and age
        let pr = sheet.current_reputation();
        let cr = c.reputation();
        st = if pr > cr && (0x18..0x22).contains(&age) {
            1
        } else {
            let d = cr as i32 as f64 * read_f64(rdata::F64_0_75);
            if pr as i32 as f64 > d && age >= 0x15 {
                2
            } else if (pr as i32 as f64 > cr as i32 as f64 * read_f64(rdata::F64_0_5) && age >= 0x11) || age > 0x18 {
                4
            } else if (sheet.potential_ability() as i32 * 50) as f64 > d {
                5
            } else {
                6
            }
        };
        *status = (*status & 0xffff_ff00) | st as u8 as u32;
    }
    // the base by position and loyalty
    let loy = p.loyalty();
    let fac: f32 = if (job_arg == 0xd || job_arg == 0xf) && age > 0x23 {
        read_f32(rdata::F32_10000)
    } else if (st == 6 || st == 5) && sheet.current_reputation() < 0x9c4 {
        read_f32(F32_8500)
    } else if (st == 6 || st == 5) && sheet.current_reputation() < 0x186a {
        read_f32(rdata::F32_10000)
    } else if sheet.attacker() >= 0xf {
        if sheet.current_reputation() > 0x1a5e {
            read_f32(if loy < 0xf { F32_65000 } else { F32_55000 })
        } else {
            read_f32(F32_50000)
        }
    } else if sheet.defensive_midfielder() >= 0xf || sheet.attacking_midfielder() >= 0xf || sheet.midfielder() >= 0xf {
        if sheet.current_reputation() > 0x1c52 {
            read_f32(if loy < 0xe { F32_52500 } else { F32_45000 })
        } else {
            read_f32(F32_40000)
        }
    } else if sheet.goalkeeper() == 0x14 {
        read_f32(if sheet.world_reputation() > 0x1e46 && loy < 3 { F32_25000 } else { F32_20000 })
    } else if sheet.sweeper() < 0xf && sheet.defender() == 0 {
        read_f32(F32_25000)
    } else if sheet.world_reputation() > 0x1d4c && loy < 6 {
        read_f32(F32_30000)
    } else {
        read_f32(F32_25000)
    };
    let mut base = fac as f64;
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && fac > read_f32(F32_11000) && !c.is_null() {
        if club_is_in_major_nation(c.addr()) != 0 {
            if sheet.current_reputation() > 0x1a5e {
                base = fac as f64 * read_f64(if loy == 1 { rdata::F64_1_15 } else if loy < 6 { F64_1_125 } else { rdata::F64_1_1 });
            } else if sheet.current_reputation() > 0x109a {
                base = fac as f64 * read_f64(rdata::F64_1_025);
            }
        }
    }
    // s = (current + home + world + 4 × the greatest (with the floor) + 50 × (ability + potential)) / 9 + the id draw
    let (rc, rh, rw) = (sheet.current_reputation(), sheet.home_reputation(), sheet.world_reputation());
    let mut mx = rc;
    if rh > mx {
        mx = rh;
    }
    if rw > mx {
        mx = rw;
    }
    if floor == 1 {
        if mx < 0x1c52 {
            mx = 0x1c52;
        }
    } else if floor == 2 && mx < 0x1a5e {
        mx = 0x1a5e;
    }
    let draw = ftol(frac as f64 * read_f32(rdata::F32_8) as f64 - read_f32(rdata::F32_4) as f64);
    let sum = (sheet.potential_ability() as i32 + sheet.current_ability() as i32) * 50 + mx as i32 * 4 + rh as i32 + rc as i32 + rw as i32;
    let mut s = (sum / 9).wrapping_add(draw) as u32;
    // a foreigner from another region: at least 1500, × 1.05
    let pn = p.nation();
    let foreign = !pn.is_null() && {
        let cn = c.nation();
        !cn.is_null()
            && pn.actual_region() != cn.actual_region()
            && (cur.is_null() || (!cur.nation().is_null() && cur.nation().actual_region() != cn.actual_region()))
    };
    if foreign {
        covered!("mkd_6 foreigner from another region");
        if s < 0x5dc {
            s = 0x5dc;
        } else {
            s = ftol(s as u64 as f64 * read_f64(rdata::F64_1_05)) as u32;
            if s <= 1 {
                s = 1;
            }
        }
    } else if s <= 1 {
        s = 1;
    }
    let sq = s.wrapping_mul(s) as u64 as f64;
    let x = base * sq * read_f64(rdata::F64_1ENEG_08); // mkd6: the wage by the score
    let mut v = x as f32;
    if x < read_f32(F32_175) as f64 {
        v = 175.0;
    }
    // the development of the current club's (or the person's) nation
    let dev = if !cur.is_null() && !cur.nation().is_null() {
        cur.nation().state_of_development() as i8
    } else if !pn.is_null() {
        pn.state_of_development() as i8
    } else {
        3
    };
    match dev {
        1 => {}
        2 => v = (v as f64 * read_f32(rdata::F32_0_75) as f64) as f32, // mkd6: the nation's development
        _ => v = (v as f64 * read_f32(rdata::F32_0_5) as f64) as f32,
    }
    // the player's reputation against the club's, within 0.85..1.15
    if !c.is_null() && c.reputation() != 0 {
        let mut r = rc as i32 as f64;
        if c.reputation() < 0x9c4 {
            r *= read_f32(F32_0_0004) as f64;
        } else {
            r /= (lvl * 500) as f64;
        }
        if r < read_f64(rdata::F64_0_85) {
            r = read_f32(rdata::F32_0_8500000238) as f64;
        }
        if r > read_f64(rdata::F64_1_15) {
            r = read_f32(rdata::F32_1_149999976) as f64;
        }
        v = (r * v as f64) as f32;
    }
    // the caps by level (soft in England, Spain and Italy)
    let hard = |v: &mut f32, cap: i32| {
        if *v as f64 > cap as f64 {
            *v = cap as f32;
        }
    };
    let soft = |v: &mut f32, cap: i32| {
        if *v as f64 > cap as f64 {
            *v = ((*v as f64 - cap as f64) * read_f64(rdata::F64_0_1) + cap as f64) as f32;
        }
    };
    if lvl < 10 {
        hard(&mut v, lvl * 2500);
    } else if lvl < 15 {
        hard(&mut v, lvl * 2750);
    } else {
        let n = c.nation(); // the original reads +0x53 of the club (a null club with a level of 15 would fault)
        let big = !n.is_null() && {
            let id = read_u32(n.addr());
            id == read_u32(KEY_NATION_ENGLAND) || id == read_u32(KEY_NATION_SPAIN) || id == read_u32(KEY_NATION_ITALY)
        };
        if !big {
            hard(&mut v, lvl * 3250);
        } else if c.plc() != 0 {
            soft(&mut v, lvl * 3250);
        } else {
            soft(&mut v, lvl * 5000);
        }
    }
    // the soft cap by position
    let mut lim: f32 = if sheet.attacker() == 0x14 {
        85000.0
    } else if sheet.attacking_midfielder() == 0x14 || sheet.midfielder() == 0x14 {
        75000.0
    } else if sheet.defensive_midfielder() == 0x14 {
        65000.0
    } else if sheet.defender() == 0x14 {
        50000.0
    } else {
        40000.0
    };
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && v > read_f32(F32_11000) && !c.is_null() && club_is_in_major_nation(c.addr()) != 0 && rc > 0x1c52 {
        lim = (lim as f64 * read_f64(rdata::F64_1_1)) as f32;
    }
    if v > lim {
        v = ((v as f64 - lim as f64) * read_f64(rdata::F64_0_2) + lim as f64) as f32;
    }
    // the free agent: × (1.375 − days at the club × 0.0125), at least 0.2 (veterans of the national side excepted)
    if p.club_job().is_null() {
        let veteran = age >= 0x21
            && p.int_apps() != 0
            && sheet.work_rate() as i32 + p.determination() as i32 <= 0x1e
            && read_i32(p.addr()) % 0xf != 0
            && c.reputation() <= 0x1194;
        if !veteran {
            covered!("mkd_6 free agent");
            let days = exe::date_days_since(G_CURRENT_DATE, p.date_joined_club() as u32, p.date_joined_club_leap_year() as u32);
            let mut t = days as f32 as f64;
            if exe::club_is_in_season(c, G_CURRENT_DATE) != 0 {
                t += read_f32(rdata::F32_60) as f64;
            }
            t = read_f32(F32_1_375) as f64 - t * read_f32(F32_0_0125) as f64;
            if !(read_f32(F32_0_2) as f64 <= t) {
                t = read_f32(F32_0_2) as f64;
            }
            v = (t * v as f64) as f32;
        }
    }
    v
}
