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
