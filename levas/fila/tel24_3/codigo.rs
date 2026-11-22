// ---------- batch %%LEVA%%: the contract screens (3): the answers of the bonuses and clauses tabs (contract_screens.cpp, tel24_3) ----------

/// `STAFF_CONTRACT::set_bonus(kind, amount)`, `remove_bonus(kind)`, `add_release_clause(clause)` and
/// `remove_release_clause(clause)` (0x85f7d0, 0x85f7f0, 0x85f7a0, 0x85f7e0; thiscall on the terms).
const CS_FN_SET_BONUS: usize = 0x85f7d0;
const CS_FN_REMOVE_BONUS: usize = 0x85f7f0;
const CS_FN_ADD_RELEASE_CLAUSE: usize = 0x85f7a0;
const CS_FN_REMOVE_RELEASE_CLAUSE: usize = 0x85f7e0;
/// The most a release fee can be raised to with "+" (below 50,000,000).
const CS_RELEASE_FEE_TOP: i32 = 50_000_000;

/// The fatal box of a contract tab answer without the human's terms: the contract_screens.cpp assert box at `line`,
/// the game's fatal cleanup and exit(-1).
unsafe fn cs_fatal(line: u32) -> ! {
    tel26_error_box(CS_CPP, line);
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exe::crt_exit(-1);
    unreachable!()
}

/// A widget of the tab set to a state: flags, text flags, id and value (`set_widget_flags`, `set_widget_text_flags`,
/// `set_widget_id`, `set_widget_data`).
unsafe fn cs_widget(item: u32, flags: u32, text_flags: u32, id: u32, data: u32) {
    exe::screenmanager_set_widget_flags(G_SCREEN_MANAGER, item as usize, flags);
    exe::screenmanager_set_widget_text_flags(G_SCREEN_MANAGER, item, text_flags);
    exe::screenmanager_set_widget_id(G_SCREEN_MANAGER, item, id);
    exe::screenmanager_set_widget_data(G_SCREEN_MANAGER, item, data as usize);
}

/// The most a bonus can be: a tenth of the maximum wage of the edited terms' club for their person
/// (`get_max_wage(person, -1)`).
unsafe fn cs_bonus_cap(edited: usize) -> i32 {
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club_at(read_i32(edited + contract::CLUB) as isize).addr());
    exe::finance_get_max_wage(fin, staff_at(read_i32(edited) as isize).addr(), u32::MAX) / 10
}

/// Answers the bonuses tab of the offer contract screen (0 when the item is not one of its own).
///
/// Without the human's terms: the fatal box (line 0xecb). A step of a bonus popup (ids 0x2a..0x2d) stores its value in
/// the bonus of the id (id 0x2d lands on the clause bytes, as the original computes it) (-11). "-" (0x10) steps the
/// bonus of the item's type down (`Cash::step_down`, mode 8): below 6 the "-" widget is disabled (flags 6, 0x2c, id 0),
/// else enabled (2, 0xc, id 0x10, the type); the amount widget gets the new amount (mode 10, 9 when long) and "+" is
/// enabled (id 0xf). "+" (0xf) steps it up (mode 8): "-" enabled, the amount, "+" disabled once the bonus reaches the
/// cap (`cs_bonus_cap`). Both redraw the screen and answer 0. The last button (0x11) sets the bonus to the person's
/// request (a tenth of his wage when he asked none), cut to the cap (-11); a grey bonus (0xd) is added
/// (`set_bonus`) at his request or a tenth of the human's wage (-11); a pale bonus (0xe) removed (`remove_bonus`)
/// (-11).
/// Original: `contract_screens_handle_bonuses_action` (0x4e39c0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_handle_bonuses_action(item: u32) -> i32 {
    let item16 = item as i16;
    let edited = get_screen_param(2);
    let terms = cs_human_terms();
    if terms == 0 {
        cs_fatal(0xecb);
    }
    let id = tel26_item_id(item16) as i16;
    if id > 0x29 && id < 0x2e {
        write_u32((terms as isize - 0x98 + id as isize * 4) as usize, tel26_item_value(item16));
        return -11;
    }
    let kind = tel26_item_value(item16) as u8 as i8;
    let bonus = terms + CS_TERMS_BONUSES + kind as isize as usize * 4;
    let plus;
    let plus_data;
    if id == 0x10 {
        covered!("tel24_3 bonus step down");
        let f: unsafe extern "thiscall" fn(*mut i32, u32) -> u32 = core::mem::transmute(CS_FN_CASH_STEP_DOWN);
        let mut v = read_i32(bonus);
        f(&mut v, 8);
        write_i32(bonus, v);
        let minus = get_screen_param(kind as i32 as u32 + 0x12) as u32;
        if v < 6 {
            cs_widget(minus, 6, 0x2c, 0, 0);
        } else {
            cs_widget(minus, 2, 0xc, 0x10, kind as i32 as u32);
        }
        cs_money_text(TS_TEXT, v as u32, 10, 9);
        exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
        plus = minus + 2;
        plus_data = kind as i32 as u32;
    } else if id == 0xf {
        covered!("tel24_3 bonus step up");
        let mut v = read_i32(bonus);
        exe::cash_step_up(&mut v as *mut i32 as usize, 8);
        write_i32(bonus, v);
        let minus = get_screen_param(kind as i32 as u32 + 0x12) as u32;
        cs_widget(minus, 2, 0xc, 0x10, kind as i32 as u32);
        cs_money_text(TS_TEXT, v as u32, 10, 9);
        exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
        plus = minus + 2;
        if cs_bonus_cap(edited) <= v {
            cs_widget(plus, 6, 0x2c, 0, 0);
            exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
            return 0;
        }
        plus_data = kind as i32 as u32;
    } else if id == 0x11 {
        let wanted = read_i32(edited + CS_TERMS_BONUSES + kind as isize as usize * 4);
        let v = if wanted < 0 { read_i32(edited + contract::WAGE) / 10 } else { wanted };
        write_i32(bonus, v);
        let cap = cs_bonus_cap(edited);
        if cap < read_i32(bonus) {
            write_i32(bonus, cap);
        }
        return -11;
    } else if id == 0xd {
        let set: unsafe extern "thiscall" fn(usize, u32, i32) = core::mem::transmute(CS_FN_SET_BONUS);
        let wanted = read_i32(edited + CS_TERMS_BONUSES + kind as isize as usize * 4);
        set(terms, kind as i32 as u32, if wanted < 0 { read_i32(terms + contract::WAGE) / 10 } else { wanted });
        return -11;
    } else if id == 0xe {
        let remove: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(CS_FN_REMOVE_BONUS);
        remove(terms, tel26_item_value(item16));
        return -11;
    } else {
        return 0;
    }
    cs_widget(plus, 2, 0xc, 0xf, plus_data);
    exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
    0
}

/// Comparer of a contract tab answer: the steps that change the widgets and redraw (`live_ids`) and the missing terms
/// (the fatal box) only in the original; the others (they only change the human's terms) side by side over the terms
/// and the answer.
unsafe fn cs_compare_tab_answer(name: &str, item: u32, live_ids: &[u16], o: unsafe extern "C" fn(u32) -> i32, r: unsafe extern "C" fn(u32) -> i32) -> i32 {
    let terms = cs_human_terms();
    if terms == 0 || live_ids.contains(&tel26_item_id(item as i16)) {
        // simplification: the widget changes and the redraw do not run twice
        return o(item);
    }
    let regions = [(terms, CS_TERMS_SIZE)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(item), || r(item));
    SCOREBOARD.record(equal && ro == rr, || format!("{} answer to item {}: original={} rust={} differences at {:?}", name, item as i16, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer of `contract_screens_handle_bonuses_action`.
pub(crate) unsafe extern "C" fn compare_contract_screens_handle_bonuses_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I0%%);
    cs_compare_tab_answer("Offer contract bonuses tab", item, &[0x10, 0xf], o, contract_screens_handle_bonuses_action)
}

/// Answers the clauses tab of the offer contract screen (0 when the item is not one of its own).
///
/// Without the human's terms: the fatal box (line 0x10a1). A release fee step of the popup (id 0x26) becomes the
/// release fee (-11). A grey clause (0x12) is added (`add_release_clause`); for the minimum fee clause (1) the release
/// fee is raised to param 0x17 first (-11). A pale clause (0x13) is removed (`remove_release_clause`, the item's value)
/// (-11). "-" (0x28) steps the fee down (`Cash::step_down`, mode 4): the "-" widget (param 0xe) stays enabled (2, 0xc,
/// id 0x28) while the fee is above the minimum (param 0x17 with the minimum fee clause, else 5), else disabled (6, 0x2c,
/// id 0); the amount widget gets the fee (mode 6, 5 when long) and "+" is enabled (id 0x27). "+" (0x27) steps it up
/// (mode 4): "-" enabled, the amount, "+" disabled from 50,000,000. Both redraw and answer 0. The last button (0x29)
/// sets the release fee back to the edited terms' (raised to param 0x17 with the minimum fee clause) (-11).
/// Original: `contract_screens_handle_clauses_action` (0x4e4160, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_handle_clauses_action(item: u32) -> i32 {
    let item16 = item as i16;
    let edited = get_screen_param(2);
    let terms = cs_human_terms();
    if terms == 0 {
        cs_fatal(0x10a1);
    }
    let fee = terms + contract::RELEASE_FEE;
    let id = tel26_item_id(item16);
    if id == 0x26 {
        write_u32(fee, tel26_item_value(item16));
        return -11;
    }
    if id == 0x12 {
        let clause = tel26_item_value(item16) as u8 as i8;
        if clause == 1 {
            let minimum = get_screen_param(0x17) as i32;
            if read_i32(fee) < minimum {
                write_i32(fee, minimum);
            }
        }
        let add: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(CS_FN_ADD_RELEASE_CLAUSE);
        add(terms, clause as u8 as u32);
        return -11;
    }
    if id == 0x13 {
        let remove: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(CS_FN_REMOVE_RELEASE_CLAUSE);
        remove(terms, tel26_item_value(item16));
        return -11;
    }
    let plus;
    if id == 0x28 {
        covered!("tel24_3 release fee step down");
        let f: unsafe extern "thiscall" fn(*mut i32, u32) -> u32 = core::mem::transmute(CS_FN_CASH_STEP_DOWN);
        let mut v = read_i32(fee);
        f(&mut v, 4);
        write_i32(fee, v);
        let minus = get_screen_param(0xe) as u32;
        let minimum = if read_u8(terms + contract::MINIMUM_FEE_RC) == 1 { get_screen_param(0x17) as i32 } else { 5 };
        if minimum < v {
            cs_widget(minus, 2, 0xc, 0x28, 0);
        } else {
            cs_widget(minus, 6, 0x2c, 0, 0);
        }
        cs_money_text(TS_TEXT, v as u32, 6, 5);
        exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
        plus = minus + 2;
    } else if id == 0x27 {
        covered!("tel24_3 release fee step up");
        let mut v = read_i32(fee);
        exe::cash_step_up(&mut v as *mut i32 as usize, 4);
        write_i32(fee, v);
        let minus = get_screen_param(0xe) as u32;
        cs_widget(minus, 2, 0xc, 0x28, 0);
        cs_money_text(TS_TEXT, v as u32, 6, 5);
        exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
        plus = minus + 2;
        if v >= CS_RELEASE_FEE_TOP {
            cs_widget(plus, 6, 0x2c, 0, 0);
            exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
            return 0;
        }
    } else if id == 0x29 {
        write_u32(fee, read_u32(edited + contract::RELEASE_FEE));
        if read_u8(terms + contract::MINIMUM_FEE_RC) == 1 && read_i32(fee) < get_screen_param(0x17) as i32 {
            write_u32(fee, get_screen_param(0x17) as u32);
        }
        return -11;
    } else {
        return 0;
    }
    cs_widget(plus, 2, 0xc, 0x27, 0);
    exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
    0
}

/// Comparer of `contract_screens_handle_clauses_action`.
pub(crate) unsafe extern "C" fn compare_contract_screens_handle_clauses_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I1%%);
    cs_compare_tab_answer("Offer contract clauses tab", item, &[0x28, 0x27], o, contract_screens_handle_clauses_action)
}
