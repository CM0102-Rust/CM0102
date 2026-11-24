// --- sx20_12: a contract's bonus and clause texts, its expiry text, the clauses a club allows, the wage steps, the
// squad status and the clause abbreviation (staff_contracts.cpp, 0x85f830..0x868020) ---

const FN_CASH_STEP_DOWN: usize = 0x43f580;
/// staff_contracts.cpp, the source path of the error boxes.
const STAFF_CONTRACTS_SOURCE: usize = 0xa8360c;
/// The bonus names (goal, assist, clean sheet) and the invalid one.
const BONUS_NAMES: [usize; 3] = [0xa83744, 0xa83750, 0xa83760]; // "Goal Bonus", "Assist Bonus", "Clean Sheet Bonus"
const BONUS_NAME_INVALID: usize = 0xa83774; // "Invalid bonus type"
/// The release clause names (non promotion, minimum fee, non playing, relegation, manager job) and the invalid one.
const CLAUSE_NAMES: [usize; 5] = [0xa8380c, 0xa837f0, 0xa837d4, 0xa837b8, 0xa8379c];
const CLAUSE_NAME_INVALID: usize = 0xa83788; // "Invalid clause type"
/// The month texts of the expiry ("Jan <%2d - Year>" .. "Dec <%2d - Year>").
const EXPIRY_MONTHS: [usize; 12] = [0xa83908, 0xa838f4, 0xa838e0, 0xa838cc, 0xa838b8, 0xa838a4, 0xa83890, 0xa8387c, 0xa83868, 0xa83854,
                                    0xa83840, 0xa8382c];
/// The squad status names, 0 to 6 ("Undefined", "Key", "First Team", "Rotation", "Backup", "Prospect", "Youngster");
/// above, "Not Needed".
const SQUAD_STATUS_NAMES: [usize; 7] = [0xa83a04, 0xa161cc, 0xa839f8, 0xa839ec, 0xa839e4, 0xa839d8, 0xa839cc];
const SQUAD_STATUS_NOT_NEEDED: usize = 0xa839c0;

/// Writes the name of bonus `kind` (0 goal, 1 assist, 2 clean sheet; any other "Invalid bonus type") into `out`.
unsafe fn bonus_name(kind: i8, out: usize) {
    let name = if (0..3).contains(&kind) { BONUS_NAMES[kind as usize] } else { BONUS_NAME_INVALID };
    langlib_text(out, name, &[]);
}

/// Writes the full name of release clause `clause` (0 to 4; any other "Invalid clause type") into `out`.
unsafe fn release_clause_name(clause: i8, out: usize) {
    let name = if (0..5).contains(&clause) { CLAUSE_NAMES[clause as usize] } else { CLAUSE_NAME_INVALID };
    langlib_text(out, name, &[]);
}

/// Writes the bonus text of the contract panel: with `mode` 1 the bonus name alone; otherwise its amount as cash
/// (display flags 0xa) followed by the name ("<amount> <name>").
/// Defect of the original: for a kind outside 0 to 2 it reads the amount past the three bonuses.
/// Original: `staff_contract_format_bonus_text` (0x85f830, staff_contracts.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn staff_contract_format_bonus_text(this: Contract, kind: i8, out: usize, _unused: u32, mode: u8) {
    let mut name = [0u8; 0xc8];
    bonus_name(kind, name.as_mut_ptr() as usize);
    if mode == 1 {
        copy_c_string(out, name.as_ptr() as usize);
        return;
    }
    let amount = read_u32((this.addr() + contract::GOAL_BONUS).wrapping_add((kind as isize * 4) as usize));
    let mut money = [0u8; 0xc8];
    exe::cash_format_money(money.as_mut_ptr() as usize, 0xc8, exe::cash_get_display_amount(&amount, 0xa), 0xa); // sx20_12: the cash flags
    langlib_text(out, 0xa83720, &[money.as_ptr() as usize, name.as_ptr() as usize]); // "<%s cash amount> <%s - bonus name>"
}

/// Writes the full name of release clause `clause` into `out` for the contract panel.
/// Original: `staff_contract_format_release_clause_name` (0x85f940, staff_contracts.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn staff_contract_format_release_clause_name(_this: usize, clause: i8, out: usize, _a: u32, _b: u32) {
    release_clause_name(clause, out);
}

/// Writes the contract end as a month and year ("Jan <year>" .. "Dec <year>") when `style` is 1 and the contract is
/// normal (nothing for a month outside 0 to 11); otherwise as `format_end_date_text`: "Contract Expired", "Free",
/// "Rolling" or the short end date.
/// Original: `STAFF_CONTRACT::format_expiry_text` (0x85fa30, staff_contracts.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn staff_contract_format_expiry_text(this: Contract, out: usize, style: u8) {
    if style == 1 && contract_status_bits(this) == CONTRACT_STATUS_NORMAL {
        let month = exe::date_get_month(this.addr() + contract::CONTRACT_EXPIRES) as i8;
        let year = read_i16(this.addr() + contract::CONTRACT_EXPIRES_YEAR) as i32;
        if (0..12).contains(&month) {
            langlib_text(out, EXPIRY_MONTHS[month as usize], &[year as usize]); // sx20_12: the month of the end
        }
        return;
    }
    staff_contract_format_end_date_text(this, out);
}

/// Writes the name of bonus `kind` into `out`.
/// Original: `staff_contract_get_bonus_type_name` (0x8679a0, staff_contracts.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_contract_get_bonus_type_name(kind: i8, out: usize) {
    bonus_name(kind, out);
}

/// Writes the full name of release clause `clause` into `out`.
/// Original: `staff_contract_get_release_clause_name` (0x867a00, staff_contracts.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_contract_get_release_clause_name(clause: i8, out: usize) {
    release_clause_name(clause, out);
}

/// Whether release clause `clause` can go into a contract of `person` with `club`: non promotion only when the
/// club's league has automatic promotion, minimum fee always, non playing never, relegation only when the league has
/// relegation (the league object of the club's division in `G_COMP_OBJECT_TABLE`, its virtual methods), manager job
/// only for someone older than 31; any other clause no.
/// Original: `staff_contract_is_clause_allowed` (0x867ad0, staff_contracts.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_contract_is_clause_allowed(person: Staff, club: Club, clause: i8) -> u32 {
    let league = if club.division().is_null() {
        0
    } else {
        read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(club.division().addr()) as usize).wrapping_mul(4)))
    };
    let asks = |slot: usize| -> u32 {
        if league == 0 {
            return 0;
        }
        let method: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(league) + slot));
        (method(league) != 0) as u32
    };
    match clause {
        0 => asks(league_vtable::HAS_AUTOMATIC_PROMOTION),
        1 => 1,
        3 => asks(league_vtable::HAS_RELEGATION),
        4 => (read_i8(person.addr() + staff::AGE) > 31) as u32, // sx20_12: the manager job clause age
        _ => 0,
    }
}

/// Moves the contract wage one display step with `step` (`Cash::step_up` or `Cash::step_down`) in the wage period of
/// the game options: per annum on the yearly amount (× 52, flags 4, back by ÷ 52), per month on the monthly one (× 4,
/// flags 8, back by ÷ 4), otherwise on the weekly one (flags 8).
unsafe fn contract_wage_step(this: Contract, step: usize) {
    let step: unsafe extern "thiscall" fn(*mut u32, u32) -> i32 = core::mem::transmute(step);
    let period = game_option_flags();
    let mut cash: u32;
    if period & WAGES_PER_ANNUM != 0 {
        cash = this.wage().wrapping_mul(52) as u32;
        step(&mut cash, 4);
        this.set_wage(cash as i32 / 52);
    } else if period & STATS_VIEW_WAGES_PER_MONTH != 0 {
        cash = (this.wage() << 2) as u32;
        step(&mut cash, 8);
        this.set_wage(cash as i32 / 4);
    } else {
        cash = this.wage() as u32;
        step(&mut cash, 8);
        this.set_wage(cash as i32);
    }
}

/// Raises the contract wage by one display step (`Cash::step_up`) in the wage period of the game options.
/// Original: `STAFF_CONTRACT::increase_wage_step` (0x867b70, staff_contracts.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn staff_contract_increase_wage_step(this: Contract) {
    contract_wage_step(this, FN_CASH_STEP_UP); // sx20_12: one step up
}

/// Lowers the contract wage by one display step (`Cash::step_down`) in the wage period of the game options.
/// Original: `STAFF_CONTRACT::decrease_wage_step` (0x867c50, staff_contracts.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn staff_contract_decrease_wage_step(this: Contract) {
    contract_wage_step(this, FN_CASH_STEP_DOWN);
}

/// Writes the squad status name (`contract::SQUAD_STATUS` >> 4) into `out` and returns 1; a buffer `size` under 20
/// shows the error box (line 0x213f) and returns 0.
/// Original: `STAFF_CONTRACT::get_squad_status_text` (0x867ef0, staff_contracts.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn staff_contract_get_squad_status_text(this: Contract, out: usize, size: i32) -> u32 {
    if size < 20 {
        show_error_at(STAFF_CONTRACTS_SOURCE, 0x213f);
        return 0;
    }
    let status = (this.squad_status() >> 4) as usize;
    langlib_text(out, if status < 7 { SQUAD_STATUS_NAMES[status] } else { SQUAD_STATUS_NOT_NEEDED }, &[]);
    1
}

/// Writes a short label of the contract's release clauses into `out` and returns the clause state: a minimum fee
/// clause "Min.Fee" (its flag forced to 1) and that flag; non promotion and relegation together "Non Pro." (2 when
/// non promotion is 2), "Rlg." (2 when relegation is 2) or "NP & Rlg" (1); one of them alone "Non Pro." or "Rlg." and
/// its flag; a manager job clause "Man." and its flag; none "-" and 0. A buffer `size` under 20 shows the error box
/// (line 0x2178) and returns 0.
/// Defect of the original: reading the label writes 1 into a minimum fee flag that holds another value.
/// Original: `STAFF_CONTRACT::get_release_clause_abbreviation` (0x868020, staff_contracts.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn staff_contract_get_release_clause_abbreviation(this: Contract, out: usize, size: i32) -> u8 {
    if size < 20 {
        show_error_at(STAFF_CONTRACTS_SOURCE, 0x2178);
        return 0;
    }
    let c = this.addr();
    let flag = |o: usize| read_u8(c + o);
    let comment = 0xad9c64usize;
    if flag(contract::MINIMUM_FEE_RC) != 0 {
        if flag(contract::MINIMUM_FEE_RC) != 1 {
            write_u8(c + contract::MINIMUM_FEE_RC, 1);
        }
        langlib_text(out, 0xa83b34, &[comment]); // "Min.Fee<..>"
        return flag(contract::MINIMUM_FEE_RC);
    }
    let (promotion, relegation) = (flag(contract::NON_PROMOTION_RC), flag(contract::RELEGATION_RC));
    if promotion != 0 && relegation != 0 {
        if promotion == 2 {
            langlib_text(out, 0xa83aec, &[comment]); // "Non Pro.<..>"
            return 2;
        }
        if relegation == 2 {
            langlib_text(out, 0xa83aac, &[comment]); // "Rlg.<..>"
            return 2;
        }
        langlib_text(out, 0xa83a54, &[comment]); // "NP & Rlg<..>"
        return 1; // sx20_12: both clauses
    }
    if promotion != 0 {
        langlib_text(out, 0xa83aec, &[comment]);
        return promotion;
    }
    if relegation != 0 {
        langlib_text(out, 0xa83aac, &[comment]);
        return relegation;
    }
    if flag(contract::MANAGER_JOB_RC) != 0 {
        langlib_text(out, 0xa83a10, &[comment]); // "Man.<..>"
        return flag(contract::MANAGER_JOB_RC);
    }
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    format(out, 0x98ba0c); // "-"
    0
}

/// Comparer for `staff_contract_format_bonus_text`.
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_format_bonus_text(this: Contract, kind: i8, out: usize, unused: u32, mode: u8) {
    let original = original_fn::<unsafe extern "thiscall" fn(Contract, i8, usize, u32, u8)>(%%I0%%);
    contract_compare("staff_contract_format_bonus_text", this, out, |o| original(this, kind, o, unused, mode),
        |o| staff_contract_format_bonus_text(this, kind, o, unused, mode))
}

/// Comparer for `staff_contract_format_release_clause_name`.
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_format_release_clause_name(this: usize, clause: i8, out: usize, a: u32, b: u32) {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, i8, usize, u32, u32)>(%%I1%%);
    contract_compare("staff_contract_format_release_clause_name", Contract(0), out, |o| original(this, clause, o, a, b),
        |o| staff_contract_format_release_clause_name(this, clause, o, a, b))
}

/// Comparer for `staff_contract_format_expiry_text`.
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_format_expiry_text(this: Contract, out: usize, style: u8) {
    let original = original_fn::<unsafe extern "thiscall" fn(Contract, usize, u8)>(%%I2%%);
    contract_compare("staff_contract_format_expiry_text", this, out, |o| original(this, o, style), |o| staff_contract_format_expiry_text(this, o, style))
}

/// Comparer for `staff_contract_get_bonus_type_name`.
pub(crate) unsafe extern "C" fn compare_staff_contract_get_bonus_type_name(kind: i8, out: usize) {
    let original = original_fn::<unsafe extern "C" fn(i8, usize)>(%%I3%%);
    contract_compare("staff_contract_get_bonus_type_name", Contract(0), out, |o| original(kind, o), |o| staff_contract_get_bonus_type_name(kind, o))
}

/// Comparer for `staff_contract_get_release_clause_name`.
pub(crate) unsafe extern "C" fn compare_staff_contract_get_release_clause_name(clause: i8, out: usize) {
    let original = original_fn::<unsafe extern "C" fn(i8, usize)>(%%I4%%);
    contract_compare("staff_contract_get_release_clause_name", Contract(0), out, |o| original(clause, o),
        |o| staff_contract_get_release_clause_name(clause, o))
}

/// Comparer for `staff_contract_is_clause_allowed`.
pub(crate) unsafe extern "C" fn compare_staff_contract_is_clause_allowed(person: Staff, club: Club, clause: i8) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(Staff, Club, i8) -> u32>(%%I5%%)(person, club, clause);
    let n = staff_contract_is_clause_allowed(person, club, clause);
    SCOREBOARD.record(r == n, || format!("staff_contract_is_clause_allowed({:#x}, {:#x}, {}) original={} rust={}", person.addr(), club.addr(), clause, r, n));
    r
}

/// Comparer for `staff_contract_increase_wage_step`.
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_increase_wage_step(this: Contract) {
    let original = original_fn::<unsafe extern "thiscall" fn(Contract)>(%%I6%%);
    contract_compare("staff_contract_increase_wage_step", this, 0, |_| original(this), |_| staff_contract_increase_wage_step(this))
}

/// Comparer for `staff_contract_decrease_wage_step`.
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_decrease_wage_step(this: Contract) {
    let original = original_fn::<unsafe extern "thiscall" fn(Contract)>(%%I7%%);
    contract_compare("staff_contract_decrease_wage_step", this, 0, |_| original(this), |_| staff_contract_decrease_wage_step(this))
}

/// Comparer for `staff_contract_get_squad_status_text`.
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_get_squad_status_text(this: Contract, out: usize, size: i32) -> u32 {
    let original = original_fn::<unsafe extern "thiscall" fn(Contract, usize, i32) -> u32>(%%I8%%);
    if size < 20 {
        return original(this, out, size); // the error box: once
    }
    contract_compare("staff_contract_get_squad_status_text", this, out, |o| original(this, o, size), |o| staff_contract_get_squad_status_text(this, o, size))
}

/// Comparer for `staff_contract_get_release_clause_abbreviation`.
pub(crate) unsafe extern "thiscall" fn compare_staff_contract_get_release_clause_abbreviation(this: Contract, out: usize, size: i32) -> u8 {
    let original = original_fn::<unsafe extern "thiscall" fn(Contract, usize, i32) -> u8>(%%I9%%);
    if size < 20 {
        return original(this, out, size); // the error box: once
    }
    contract_compare("staff_contract_get_release_clause_abbreviation", this, out, |o| original(this, o, size),
        |o| staff_contract_get_release_clause_abbreviation(this, o, size))
}
