use super::*;

/// Decides whether an Italian loan is allowed: refuses outside the first two windows (9) and, in the five listed
/// divisions, with four players on loan from other clubs (0x25). `ITALY_RULES::can_loan_player(club, _player, msg,
/// reason)`: outside the first two windows (`exe::transfer_window_contains_date` with `[this+4]` and `[this+4]` + 0xc):
/// reason 9. A club of one of the five divisions (`KEY_CLUB_COMP_ITALIAN_SERIE_C1_A` to
/// `KEY_CLUB_COMP_ITALIAN_SERIE_C2_C`) with four players on loan from other clubs (ids below `G_STAFF_COUNT`, loan
/// contract `exe::contract_manager_get_loan_contract`) may not (0x25).
/// Original: `ITALY_RULES::can_loan_player` (0x661770, italy_rules.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn italy_rules_can_loan_player(this: usize, club: usize, _player: usize, msg: usize, reason: *mut u16) -> u32 {
    let club = Club(club);
    // 0x8e8c40: push janela; push 0xae2c90; ecx = this -> thiscall, ret 8
    if exe::transfer_window_contains_date(this, G_CURRENT_DATE, read_usize(this + 4)) == 0 && exe::transfer_window_contains_date(this, G_CURRENT_DATE, read_usize(this + 4) + 0xc) == 0 {
        covered!("cqe_6 italy outside the window");
        return refuse_outside_window(this, msg, reason);
    }
    let comp = club.division().addr();
    if comp == 0 {
        return 1;
    }
    let id = read_usize(comp);
    if ![KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C].iter().any(|&g| read_usize(g) == id) {
        return 1;
    }
    // 0x4cf6d0: push person; mov ecx, [0xae1050] -> thiscall, ret 4
    let mut n: i8 = 0;
    for k in 0..50usize {
        let p = club.squad(k);
        if p == 0 || read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 || exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) == 0 {
            continue;
        }
        let cp = game_get_contract_and_store(p);
        let of = if cp != 0 { england_rules_club_of_contract(Contract(cp)) } else { 0 };
        if of != club.addr() {
            n = n.wrapping_add(1);
        }
    }
    if n >= 4 {
        covered!("cqe_6 italy four loaned out");
        if msg != 0 {
            let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
            let g = club.short_gender_name() as i8 as i32;
            text(msg, 0x9ea9dc, g, g, club.addr() + club::SHORT_NAME);
        }
        if !reason.is_null() {
            *reason = 0x25;
        }
        return 0;
    }
    1
}

/// the Italian rule (14): three windows, from 1 July to 27 September (moved to the Thursday [inference]), from 2 to 31
/// January, and from 1 July to 30 April [inference: the free-agent one]
/// Original: `ITALY_RULES::init_transfer_windows` (0x6613f0, italy_rules.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn italy_rules_init_transfer_windows(this: usize) {
    covered!("cqk_2 italy");
    set_transfer_windows(this, 3, &ITALY_TRANSFER_WINDOWS, 0x9ea9a4, 0x52);
}

/// Tells whether a loan with an Italian club can end: in June (month 5) when asked by the loan check (flag), or on a date
/// inside one of the first two Italian windows (the third, free agents only, does not count).
///
/// `ITALY_RULES::can_end_loan(club, player, msg, date, flag)`, the vtable slot `transfer_rules_vtable::CAN_END_LOAN`,
/// from `can_end_loan` and from `transfer_manager_can_loan_player` (flag 1). Without a date, today
/// (`G_CURRENT_DATE`). With `flag` and the date's month (`exe::date_get_month`) June: 1. The date inside window 0 or
/// window 1 of the table at +4 (`exe::transfer_window_contains_date`: push window; push date; mov ecx, this): 1.
/// Otherwise 0 and, with a message, the text with the Italian adjective (`KEY_NATION_ITALY`, `loan_end_refusal_text`).
/// Original: `ITALY_RULES::can_end_loan` (0x6618f0, italy_rules.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn italy_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
    let date = if date == 0 { G_CURRENT_DATE } else { date };
    if flag != 0 && exe::date_get_month(date) == 5 {
        covered!("mkc_2 italy june");
        return 1; // mkc2: Italy, June
    }
    if exe::transfer_window_contains_date(this, date, read_usize(this + 4)) != 0 || exe::transfer_window_contains_date(this, date, read_usize(this + 4) + 0xc) != 0 {
        return 1;
    }
    covered!("mkc_2 italy windows closed");
    if msg != 0 {
        loan_end_refusal_text(KEY_NATION_ITALY, club, player, msg);
    }
    0
}

pub(crate) unsafe extern "thiscall" fn compare_italy_rules_can_end_loan(this: usize, club: usize, player: usize, msg: usize, date: usize, flag: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(IDX_ITALY_RULES_CAN_END_LOAN);
    rules_compare_can_end_loan(o, italy_rules_can_end_loan, "italy_rules_can_end_loan", this, club, player, msg, date, flag)
}
