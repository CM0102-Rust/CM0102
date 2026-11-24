// --- sx20_11: a contract's trial offer, wage and end texts, bonuses and release clauses (staff_contracts.cpp,
// 0x85e480..0x85fa20) ---

/// The contract status in the low six bits of `contract::CONTRACT_TYPE`: rolling, normal, expired, free transfer.
pub(crate) const CONTRACT_STATUS_ROLLING: u8 = 1;
pub(crate) const CONTRACT_STATUS_NORMAL: u8 = 2;
pub(crate) const CONTRACT_STATUS_EXPIRED: u8 = 3;
pub(crate) const CONTRACT_STATUS_FREE: u8 = 4;
/// The game options' per annum wage bit (`[G_CONFIG] + 4`; per week and per month are `STATS_VIEW_WAGES_PER_WEEK`
/// and `STATS_VIEW_WAGES_PER_MONTH`).
pub(crate) const WAGES_PER_ANNUM: u32 = 0x800000;

/// The contract status (low six bits of `contract::CONTRACT_TYPE`).
pub(crate) unsafe fn contract_status_bits(contract: Contract) -> u8 {
    contract.contract_type() & 0x3f
}

/// The game options' flags (`[G_CONFIG] + 4`), where the wage period bits live.
pub(crate) unsafe fn game_option_flags() -> u32 {
    read_u32(read_usize(G_CONFIG) + 4)
}

/// `langlib_sprintf(out, text, ...)`: the game's translated text.
pub(crate) unsafe fn langlib_text(out: usize, text: usize, args: &[usize]) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    match args {
        [] => f(out, text),
        [a] => f(out, text, *a),
        [a, b] => f(out, text, *a, *b),
        [a, b, c, ..] => f(out, text, *a, *b, *c),
    };
}

/// Fills `this` as a trial offer of `person` at `club` that ends `weeks` weeks from today: the ids, no unknown +8, a
/// wage and previous wage of 200, started today and ending today + `weeks` weeks, status 5 (type bits 0 and 2 set,
/// 1 and 3 to 5 cleared, the two high bits kept), no squad number, morale, grievances, flags, matches left out or unhappiness,
/// transfer status 4, the person's opinion links cleared, the squad status' low nibble the person's fluency in the
/// club nation's first language (at most 10) when the club has a nation, not leaving on a Bosman, no transfer
/// arranged, no appearances, no release clauses, bonuses (-1) or release fee. Returns -1.
/// Original: `STAFF_CONTRACT::init_trial_offer` (0x85e480, staff_contracts.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn staff_contract_init_trial_offer(this: Contract, person: Staff, club: Club, weeks: u32) -> i32 {
    let c = this.addr();
    let (mut scratch, mut end, mut scratch2) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    write_u32(c, read_u32(person.addr()));
    write_u32(c + contract::CLUB, read_u32(club.addr()));
    write_u32(c + contract::UNKNOWN, 0);
    this.set_wage(200); // sx20_11: the trial wage
    exe::date_operator_3d(c + contract::DATE_STARTED, scratch.as_mut_ptr() as usize, G_CURRENT_DATE);
    let until = exe::date_add_weeks(G_CURRENT_DATE, &mut end, weeks);
    exe::date_operator_3d(c + contract::CONTRACT_EXPIRES, scratch2.as_mut_ptr() as usize, until);
    write_u32(c + contract::PREVIOUS_WAGE, 200);
    write_u8(c + contract::SQUAD_NUMBER, 0);
    write_u8(c + contract::MORALE, 0);
    write_u8(c + contract::CONTRACT_TYPE, (this.contract_type() & 0xc5) | 5);
    write_u8(c + contract::TRANSFER_STATUS, 0);
    write_u32(c + contract::GRIEVANCES, 0);
    exe::staff_clear_opinion_links(staff_at(read_i32(c) as isize));
    write_u32(c + contract::FLAGS, 0);
    write_u8(c + contract::TRANSFER_STATUS, 4);
    write_u8(c + contract::MATCHES_LEFT_OUT, 0);
    write_u8(c + contract::UNHAPPY_FLAG, 0);
    if !club.is_null() && !club.nation().is_null() {
        let language = read_i8(club.nation().addr() + nation::FIRST_LANGUAGE) as u8 as u32;
        let fluency = exe::playerstats_get_language_fluency(read_usize(G_PLAYER_STATS), person, club, language, 0) as i8;
        let level = if fluency > 10 { 10 } else { fluency as i32 };
        let status = read_u8(c + contract::SQUAD_STATUS);
        write_u8(c + contract::SQUAD_STATUS, ((status ^ level as u8) & 0xf) ^ status);
    }
    write_u8(c + contract::LEAVING_ON_BOSMAN, 0);
    write_i32(c + contract::TRANSFER_ARRANGED_FOR, -1);
    write_u16(c + contract::CLUB_APPEARANCES, 0);
    for o in [contract::NON_PROMOTION_RC, contract::MINIMUM_FEE_RC, 0x1e, contract::RELEGATION_RC, contract::MANAGER_JOB_RC] {
        write_u8(c + o, 0);
    }
    for o in [contract::GOAL_BONUS, contract::ASSIST_BONUS, contract::CLEAN_SHEET_BONUS] {
        write_i32(c + o, -1);
    }
    write_u32(c + contract::RELEASE_FEE, 0);
    -1
}

/// Writes the contract wage into `out` as money in the wage period of the game options: per week as it is (rounding
/// flag 8), per month × 4 (flag 8) or per annum × 52 (flag 4), with `cash_flags` added; style 0 the amount alone,
/// style 2 "<amount> p/w", "p/m" or "p/a", any other "<amount> per week", "per month" or "per annum" (nothing when no
/// period bit is set).
/// Original: `STAFF_CONTRACT::format_wage_text` (0x85e750, staff_contracts.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn staff_contract_format_wage_text(this: Contract, out: usize, style: u8, cash_flags: u8) {
    let period = game_option_flags();
    let (amount, rounding) = if period & STATS_VIEW_WAGES_PER_WEEK != 0 {
        (this.wage(), 8u8)
    } else if period & STATS_VIEW_WAGES_PER_MONTH != 0 {
        (this.wage().wrapping_mul(4), 8)
    } else {
        (this.wage().wrapping_mul(52), 4) // sx20_11: a year of wages
    };
    // simplification: the original passes the flags in a dword whose high bytes are stack leftovers (only the low byte is read)
    let flags = (rounding | cash_flags) as u32;
    let mut money = [0u8; 0xc8];
    exe::cash_format_money(money.as_mut_ptr() as usize, 0xc8, exe::cash_get_display_amount(&(amount as u32), flags), flags);
    let m = money.as_ptr() as usize;
    if style == 0 {
        let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        format(out, 0x991944, m); // "%s"
        return;
    }
    let texts: [usize; 3] = if style == 2 {
        [0xa836c4, 0xa836ac, 0xa83694] // "<%s wage amount> p/w", "p/m", "p/a"
    } else {
        [0xa83678, 0xa8365c, 0xa83640] // "<%s wage amount> per week", "per month", "per annum"
    };
    for (bit, text) in [STATS_VIEW_WAGES_PER_WEEK, STATS_VIEW_WAGES_PER_MONTH, WAGES_PER_ANNUM].into_iter().zip(texts) {
        if period & bit != 0 {
            langlib_text(out, text, &[m]);
            return;
        }
    }
}

/// Writes the contract end as text: "Contract Expired", "Free" (on a free transfer), "Rolling", or the short form of
/// the end date (`Date::get_short_date_string`, 200 bytes, style 0xff).
/// Original: `STAFF_CONTRACT::format_end_date_text` (0x85e8f0, staff_contracts.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn staff_contract_format_end_date_text(this: Contract, out: usize) {
    match contract_status_bits(this) {
        CONTRACT_STATUS_EXPIRED => langlib_text(out, 0xa8370c, &[]), // "Contract Expired"
        CONTRACT_STATUS_FREE => langlib_text(out, 0xa836e4, &[0xad9c64]), // "Free<%s - COMMENT - on a free transfer>"
        CONTRACT_STATUS_ROLLING => langlib_text(out, 0xa836dc, &[]), // "Rolling"
        _ => {
            exe::date_get_short_date_string(this.addr() + contract::CONTRACT_EXPIRES, out, 0xc8, 0xff); // sx20_11: the end date
        }
    }
}

/// Adds release clause `clause` (0 to 4, the flag bytes from `contract::NON_PROMOTION_RC`): the minimum fee clause
/// (1) replaces all others; any other clears the minimum fee one.
/// Original: `STAFF_CONTRACT::add_release_clause` (0x85f7a0, staff_contracts.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn staff_contract_add_release_clause(this: Contract, clause: i8) {
    let c = this.addr();
    if clause == 1 {
        write_u32(c + contract::NON_PROMOTION_RC, 0);
        write_u8(c + contract::MANAGER_JOB_RC, 0);
    } else {
        write_u8(c + contract::MINIMUM_FEE_RC, 0); // sx20_11: the minimum fee clause goes
    }
    write_u8((c + contract::NON_PROMOTION_RC).wrapping_add(clause as isize as usize), 1);
}

/// Stores `amount` as bonus `kind` (0 goal, 1 assist, 2 clean sheet, the dwords from `contract::GOAL_BONUS`).
/// Original: `STAFF_CONTRACT::set_bonus` (0x85f7d0, staff_contracts.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn staff_contract_set_bonus(this: Contract, kind: i8, amount: i32) {
    write_i32((this.addr() + contract::GOAL_BONUS).wrapping_add((kind as isize * 4) as usize), amount);
}

/// Removes release clause `clause` by clearing its flag byte.
/// Original: `STAFF_CONTRACT::remove_release_clause` (0x85f7e0, staff_contracts.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn staff_contract_remove_release_clause(this: Contract, clause: i8) {
    write_u8((this.addr() + contract::NON_PROMOTION_RC).wrapping_add(clause as isize as usize), 0);
}

/// Removes bonus `kind` by writing -1 to its amount.
/// Original: `STAFF_CONTRACT::remove_bonus` (0x85f7f0, staff_contracts.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn staff_contract_remove_bonus(this: Contract, kind: i8) {
    write_i32((this.addr() + contract::GOAL_BONUS).wrapping_add((kind as isize * 4) as usize), -1);
}

/// Whether the contract status is expired.
/// Original: `STAFF_CONTRACT::is_expired` (0x85f9f0, staff_contracts.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn staff_contract_is_expired(this: Contract) -> bool {
    contract_status_bits(this) == CONTRACT_STATUS_EXPIRED
}

/// Whether the contract status is rolling.
/// Original: `STAFF_CONTRACT::is_rolling` (0x85fa20, staff_contracts.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn staff_contract_is_rolling(this: Contract) -> bool {
    contract_status_bits(this) == CONTRACT_STATUS_ROLLING
}

/// Runs a contract routine side by side over the contract (0x50 bytes; none for a null one) and, when it writes a
/// text, over the text: the original writes into `out`, the copy into a local that starts with what `out` held.
/// Contract, text and answer must match.
pub(crate) unsafe fn contract_compare<R: PartialEq + core::fmt::Debug>(name: &str, contract: Contract, out: usize,
        original: impl FnOnce(usize) -> R, copy: impl FnOnce(usize) -> R) -> R {
    let mut local = [0u8; 0x401];
    if out != 0 {
        let before = c_string_bytes(out);
        local[..before.len()].copy_from_slice(&before);
    }
    let lp = local.as_mut_ptr() as usize;
    let whole = [(contract.addr(), 0x50)];
    let regions: &[(usize, usize)] = if contract.is_null() { &[] } else { &whole };
    let (r, c, same, mo, mr) = crate::comparison::side_by_side(regions, || original(out), || copy(if out != 0 { lp } else { 0 }));
    let (to, tc) = if out != 0 { (c_string_bytes(out), c_string_bytes(lp)) } else { (Vec::new(), Vec::new()) };
    SCOREBOARD.record(same && r == c && to == tc, || format!("{} (contract {:#x}): original={:?} {:?} rust={:?} {:?} {:?}", name, contract.addr(), r,
        String::from_utf8_lossy(&to), c, String::from_utf8_lossy(&tc), first_differences(&mo, &mr)));
    r
}

/// Comparer for `staff_contract_init_trial_offer`.
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_init_trial_offer(this: Contract, person: Staff, club: Club, weeks: u32) -> i32 {
    let original = original_fn::<unsafe extern "thiscall" fn(Contract, Staff, Club, u32) -> i32>(%%I0%%);
    contract_compare("staff_contract_init_trial_offer", this, 0, |_| original(this, person, club, weeks),
        |_| staff_contract_init_trial_offer(this, person, club, weeks))
}

/// Comparer for `staff_contract_format_wage_text`.
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_format_wage_text(this: Contract, out: usize, style: u8, cash_flags: u8) {
    let original = original_fn::<unsafe extern "thiscall" fn(Contract, usize, u8, u8)>(%%I1%%);
    contract_compare("staff_contract_format_wage_text", this, out, |o| original(this, o, style, cash_flags),
        |o| staff_contract_format_wage_text(this, o, style, cash_flags))
}

/// Comparer for `staff_contract_format_end_date_text`.
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_format_end_date_text(this: Contract, out: usize) {
    let original = original_fn::<unsafe extern "thiscall" fn(Contract, usize)>(%%I2%%);
    contract_compare("staff_contract_format_end_date_text", this, out, |o| original(this, o), |o| staff_contract_format_end_date_text(this, o))
}

/// Comparer for `staff_contract_add_release_clause`.
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_add_release_clause(this: Contract, clause: i8) {
    let original = original_fn::<unsafe extern "thiscall" fn(Contract, i8)>(%%I3%%);
    contract_compare("staff_contract_add_release_clause", this, 0, |_| original(this, clause), |_| staff_contract_add_release_clause(this, clause))
}

/// Comparer for `staff_contract_set_bonus`.
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_set_bonus(this: Contract, kind: i8, amount: i32) {
    let original = original_fn::<unsafe extern "thiscall" fn(Contract, i8, i32)>(%%I4%%);
    contract_compare("staff_contract_set_bonus", this, 0, |_| original(this, kind, amount), |_| staff_contract_set_bonus(this, kind, amount))
}

/// Comparer for `staff_contract_remove_release_clause`.
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_remove_release_clause(this: Contract, clause: i8) {
    let original = original_fn::<unsafe extern "thiscall" fn(Contract, i8)>(%%I5%%);
    contract_compare("staff_contract_remove_release_clause", this, 0, |_| original(this, clause), |_| staff_contract_remove_release_clause(this, clause))
}

/// Comparer for `staff_contract_remove_bonus`.
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_remove_bonus(this: Contract, kind: i8) {
    let original = original_fn::<unsafe extern "thiscall" fn(Contract, i8)>(%%I6%%);
    contract_compare("staff_contract_remove_bonus", this, 0, |_| original(this, kind), |_| staff_contract_remove_bonus(this, kind))
}

/// Comparer for `staff_contract_is_expired`.
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_is_expired(this: Contract) -> bool {
    let original = original_fn::<unsafe extern "thiscall" fn(Contract) -> bool>(%%I7%%);
    contract_compare("staff_contract_is_expired", this, 0, |_| original(this), |_| staff_contract_is_expired(this))
}

/// Comparer for `staff_contract_is_rolling`.
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_is_rolling(this: Contract) -> bool {
    let original = original_fn::<unsafe extern "thiscall" fn(Contract) -> bool>(%%I8%%);
    contract_compare("staff_contract_is_rolling", this, 0, |_| original(this), |_| staff_contract_is_rolling(this))
}
