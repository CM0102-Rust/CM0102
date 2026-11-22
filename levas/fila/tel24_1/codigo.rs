// ---------- batch %%LEVA%%: the contract screens (1): offering a contract, the set role screen, the openers of the offer screen and the staff offer dialog (contract_screens.cpp, tel24_1) ----------

/// contract_screens.cpp's path, for the assert box.
pub(crate) const CS_CPP: usize = 0x9afc8c;
/// `CONTRACT_MANAGER::get_offer_job(staff)` (0x4d44e0, thiscall on the contract manager): the job the club would offer.
const CS_FN_GET_OFFER_JOB: usize = 0x4d44e0;
/// `CONTRACT_MANAGER::find_talks_record(staff, manager, club)` (0x4ca2a0, thiscall): the talks record, or 0.
const CS_FN_FIND_TALKS_RECORD: usize = 0x4ca2a0;
/// `CONTRACT_MANAGER::add_talks_record(staff, manager, club, role)` (0x4c9f60, thiscall).
const CS_FN_ADD_TALKS_RECORD: usize = 0x4c9f60;
/// `staff_can_take_job(job, staff)` (0x4cb990, stdcall; called with the contract manager in ecx).
const CS_FN_STAFF_CAN_TAKE_JOB: usize = 0x4cb990;
/// `get_contract_satisfaction(staff)` (0x866620, thiscall on the contract).
const CS_FN_CONTRACT_SATISFACTION: usize = 0x866620;
/// `shortlist_manager_squad_status_allowed(staff, status)` (0x84b4d0, stdcall; the shortlist manager in ecx).
const CS_FN_SQUAD_STATUS_ALLOWED: usize = 0x84b4d0;
/// `shortlist_manager_squad_status_text(status, buffer, size)` (0x84b330, thiscall on the shortlist manager).
const CS_FN_SQUAD_STATUS_TEXT: usize = 0x84b330;
/// `shortlist_manager_max_fee_for_player_status(club, staff, status)` (0x84b1f0, thiscall on the shortlist manager).
const CS_FN_MAX_FEE_FOR_STATUS: usize = 0x84b1f0;
/// `contract_manager_lowest_wage_demand(staff, club)` (0x4ce450, thiscall on the contract manager).
const CS_FN_LOWEST_WAGE_DEMAND: usize = 0x4ce450;
/// `Cash::step_down(mode)` (0x43f580, thiscall on a cash dword, ret 4): rounds the amount down a step, stores it and
/// returns it.
const CS_FN_CASH_STEP_DOWN: usize = 0x43f580;
/// `HUMAN_MANAGER::get_contract_offer(k)` (0x5fd930, thiscall on the human manager): the human's offer being edited.
pub(crate) const CS_FN_HUMAN_CONTRACT_OFFER: usize = 0x5fd930;
/// The no-op destructor of the edited offer (0x5a53c0, thiscall).
const CS_FN_OFFER_NOOP_DESTRUCTOR: usize = 0x5a53c0;
/// The screens of the set role and offer contract flows (build, input and exit functions).
const CS_BUILD_SET_ROLE: usize = 0x4dd6c0;
const CS_HANDLE_SET_ROLE: usize = 0x4ddf20;
const CS_BUILD_OFFER: usize = 0x4deac0;
const CS_HANDLE_OFFER: usize = 0x4e01e0;
const CS_CLOSE_OFFER: usize = 0x4e06a0;
const CS_BUILD_STAFF_OFFER: usize = 0x4e4820;
const CS_HANDLE_STAFF_OFFER: usize = 0x4e4ea0;
/// The two openers of the offer contract screen (cdecl), by address (they are hooked).
const CS_FN_OPEN_OFFER: usize = 0x4de1f0;
const CS_FN_OPEN_OFFER_FOR_TRANSFER: usize = 0x4de5d0;
/// `transfer_screens_open_offer_changed_screen(offer, serial)` (0x8f2470, cdecl).
const CS_FN_OPEN_OFFER_CHANGED: usize = 0x8f2470;
/// The most a transfer budget shown on the offer screen can be (5,000,000).
const CS_BUDGET_CAP: i32 = 5_000_000;

/// The shortlist manager (`[transfer manager + 0x856]`).
unsafe fn cs_shortlist_manager() -> usize {
    read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856)
}

/// `staff_can_take_job(job, staff)` by address (the low byte of `job` is the job).
unsafe fn cs_can_take_job(job: u32, staff: usize) -> bool {
    let f: unsafe extern "stdcall" fn(u32, usize) -> u32 = core::mem::transmute(CS_FN_STAFF_CAN_TAKE_JOB);
    f(job, staff) != 0
}

/// How many of the jobs 0..15 the person can take.
unsafe fn cs_jobs_open_to(staff: usize) -> i8 {
    let mut n: i8 = 0;
    for job in 0..16u32 {
        if cs_can_take_job(job, staff) {
            n = n.wrapping_add(1);
        }
    }
    n
}

/// `get_contract_satisfaction(staff)` of a contract, by address.
unsafe fn cs_contract_satisfaction(contract: usize, staff: usize) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(CS_FN_CONTRACT_SATISFACTION);
    f(contract, staff)
}

/// `shortlist_manager_squad_status_allowed(staff, status)` by address.
unsafe fn cs_status_allowed(staff: usize, status: u32) -> bool {
    let f: unsafe extern "stdcall" fn(usize, u32) -> u32 = core::mem::transmute(CS_FN_SQUAD_STATUS_ALLOWED);
    f(staff, status) != 0
}

/// The squad status nibble of a contract (`contract::SQUAD_STATUS` >> 4).
unsafe fn cs_squad_status(contract: usize) -> u8 {
    read_u8(contract + contract::SQUAD_STATUS) >> 4
}

/// The contract type of a contract (the low six bits of `contract::CONTRACT_TYPE`).
unsafe fn cs_contract_type(contract: usize) -> u8 {
    read_u8(contract + contract::CONTRACT_TYPE) & 0x3f
}

/// The current slot's human manager's club (`staff::CLUB_JOB` of the slot manager).
unsafe fn cs_human_club() -> usize {
    read_usize(tel26_slot_manager().addr() + staff::CLUB_JOB)
}

/// A cash amount stepped down once in mode 4 (`Cash::step_down`).
unsafe fn cs_step_down(amount: i32) -> u32 {
    let f: unsafe extern "thiscall" fn(*mut i32, u32) -> u32 = core::mem::transmute(CS_FN_CASH_STEP_DOWN);
    let mut cash = amount;
    f(&mut cash, 4)
}

/// The value a club would put on a player for the offer screen: 70% of `calc_player_value(staff, club, 1, -1, 0)`,
/// stepped down once (0 when the person is not a player).
unsafe fn cs_offer_value(staff: usize, club: usize) -> u32 {
    if read_usize(staff + staff::PLAYER) == 0 {
        return 0;
    }
    let v = exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), staff, club, 1, -1, 0);
    cs_step_down(ftol(v as f64 * read_f64(rdata::F64_0_7)))
}

/// The transfer budget shown on the offer screen: the low dword of `Finance::get_transfer_budget(staff, 0, 0, 1, 0)`
/// of the club, less `fee`, kept within 0..5,000,000.
unsafe fn cs_budget(club: usize, staff: usize, fee: i32) -> i32 {
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    let b = (exe::finance_get_transfer_budget(fin, staff as u32, 0, 0usize, 1, 0) as i32).wrapping_sub(fee);
    if b < 0 { 0 } else if b > CS_BUDGET_CAP { CS_BUDGET_CAP } else { b }
}

/// The wage limit of the offer screen for a non-player job: `Finance::get_max_staff_wage(job)`, raised to the
/// person's current wage (`CONTRACT_MANAGER::get_wage`) when he is already at the club (the contract is looked up
/// twice, `G_CONTRACT_LOOKUP_RESULT` keeps the last).
unsafe fn cs_staff_wage_limit(club: usize, staff: usize, job: u32) -> f64 {
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    let mut limit = exe::finance_get_max_staff_wage(fin, job as i8) as f64;
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c != 0 && club_at(read_i32(c + contract::CLUB) as isize).addr() != 0 && contract_club_cached(Staff(staff)) == club {
        let wage = exe::contract_manager_get_wage(read_usize(G_CONTRACT_MANAGER), staff) as f64;
        if wage > limit {
            limit = wage;
        }
    }
    limit
}

/// Whether a job is one of the player jobs that take the club's player wage limit (11, 12, 13, 15).
fn cs_player_wage_job(job: i8) -> bool {
    matches!(job, 0xb | 0xc | 0xf | 0xd)
}

/// Opens the right screen for offering a contract to a person: straight to the offer screen when the club already
/// knows the role, otherwise the set role screen with the best squad role preselected.
///
/// The interest threshold is (`staff::LOYALTY` + `staff::AMBITION` + 0x2a + `staff::PROFESSIONALISM`) × 0.1 + 3.5;
/// with a transfer offer slot `force` becomes 1. Without the slot's manager, or a manager without a club, the
/// contract_screens.cpp assert box (lines 0x193 and 0x19b). Then it counts the jobs 0..15 the person can take, reads
/// his contract at the club (`get_contract_for_club`), the job the club would offer (`get_offer_job`) and the talks
/// record (`find_talks_record`):
/// - talks held today (`Date::operator==` on the record's date) with a role (`+0x14` ≥ 0) and fewer than two jobs
///   open: the offer screen with that role (for the transfer when `force`);
/// - the offered job is not 0xb and exactly one job is open: the offer screen with the role 0xff (for the transfer
///   when `force`);
/// - not `ask`, fewer than two jobs, a contract whose squad status is 1..6, no contract of type 5
///   (`G_CONTRACT_LOOKUP_RESULT`) and the club contract not of type 6: the offer screen with the contract's squad
///   status (for the transfer with the talks role, or the status when there is none);
/// - otherwise the set role screen (build `contract_screens_build_set_role_screen`, input
///   `contract_screens_handle_set_role_screen`, flag 1): param 0 the person; the preselected role is the contract's
///   status when he is unhappy there (`get_contract_satisfaction` ≠ 1) and the status is allowed (not 7 or 8, a
///   player, `squad_status_allowed`), else 0; with no contract (or a happy one) the squad role 1..7 (7 skipped) of the
///   highest interest (`staff_interest_in_joining_club(club, 1, 0, role)`) that is above 3.15, at most the threshold,
///   allowed and without the 0x8000 flag; roles 1, 2 and 3 for players under 24 need the player's current reputation
///   plus (ambition + professionalism × 2 (+0x4b for role 2, +0x7d for role 3)) × 10 at least the club's reputation
///   (role 1 also a reputation of 3250 or more), and role 1 at any age needs (professionalism + 0x46) × 25 + the
///   reputation + ambition × 10 at least the club's; for the roles after 1, a happy contract whose status is at or
///   below the role skips it. Params 1 and 6 the talks role (or 0 and
///   the preselected role), 2 the transfer slot, 3 `force`, 4 the offered job, 5 the talks record, 7 the slot offer's
///   serial (0 without one).
/// Original: `contract_screens_offer_contract_to_staff` (0x4dd0f0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_offer_contract_to_staff(staff: usize, slot: usize, force: u32, ask: u32) {
    let open_offer: unsafe extern "C" fn(usize, u32, u32, u32) = core::mem::transmute(CS_FN_OPEN_OFFER);
    let open_for_transfer: unsafe extern "C" fn(usize, usize, u32, u32, u32) = core::mem::transmute(CS_FN_OPEN_OFFER_FOR_TRANSFER);
    let find_talks: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize = core::mem::transmute(CS_FN_FIND_TALKS_RECORD);
    let offer_job: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(CS_FN_GET_OFFER_JOB);
    let threshold = (read_i8(staff + staff::LOYALTY) as i32 + read_i8(staff + staff::AMBITION) as i32 + 0x2a
        + read_i8(staff + staff::PROFESSIONALISM) as i32) as f64 * read_f64(rdata::F64_0_1) + read_f64(rdata::F64_3_5);
    let force = if slot != 0 { 1 } else { force };
    let mgr = tel26_slot_manager().addr();
    if mgr == 0 {
        tel26_error_box(CS_CPP, 0x193);
        return;
    }
    let club = read_usize(mgr + staff::CLUB_JOB);
    if club == 0 {
        tel26_error_box(CS_CPP, 0x19b);
        return;
    }
    let jobs = cs_jobs_open_to(staff);
    let cm = read_usize(G_CONTRACT_MANAGER);
    let contract = exe::contract_manager_get_contract_for_club(cm, staff, club);
    let job = offer_job(read_usize(G_CONTRACT_MANAGER), staff);
    let talks = find_talks(read_usize(G_CONTRACT_MANAGER), staff, mgr, club);
    let mut talks_role: i8 = -1;
    if talks != 0 {
        talks_role = read_i8(talks + 0x14);
        if talks_role > -1 && exe::date_operator_3d_3d(talks + 0xc, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE + 4)) != 0 && jobs < 2 {
            covered!("tel24_1 offer contract after talks today");
            if force != 0 {
                open_for_transfer(slot, staff, talks_role as u8 as u32, job as u32, 1);
            } else {
                open_offer(staff, talks_role as u8 as u32, job as u32, 1);
            }
            return;
        }
    }
    if job != 0xb && jobs == 1 {
        covered!("tel24_1 offer contract for the only job");
        if force != 0 {
            open_for_transfer(slot, staff, talks_role as u8 as u32, job as u32, 1);
        } else {
            open_offer(staff, talks_role as u8 as u32, job as u32, 1);
        }
        return;
    }
    let mut set_role = ask != 0 || jobs >= 2 || contract == 0;
    if !set_role {
        let st = cs_squad_status(contract);
        set_role = st >= 7 || st == 0;
    }
    if !set_role {
        let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
        write_usize(G_CONTRACT_LOOKUP_RESULT, c);
        set_role = (c != 0 && cs_contract_type(c) == 5) || cs_contract_type(contract) == 6;
    }
    if !set_role {
        covered!("tel24_1 offer contract with the current status");
        if slot != 0 {
            let role = if talks_role as u8 == 0xff { cs_squad_status(contract) } else { talks_role as u8 };
            open_for_transfer(slot, staff, role as u32, job as u32, 1);
        } else {
            open_offer(staff, cs_squad_status(contract) as u32, job as u32, 1);
        }
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, CS_BUILD_SET_ROLE, CS_HANDLE_SET_ROLE, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel24_1 offer contract opens the set role screen");
    set_screen_param(0, staff);
    let player = read_usize(staff + staff::PLAYER);
    let mut best: u8 = 0;
    if contract != 0 && cs_contract_satisfaction(contract, staff) != 1 {
        let st = cs_squad_status(contract);
        best = st;
        if st == 7 || st == 8 || player == 0 || !cs_status_allowed(staff, st as u32) {
            best = 0;
        }
    } else {
        let mut best_interest = 3.15f64;
        let reputation = read_i16(club + club::REPUTATION) as i32;
        let ambition = read_i8(staff + staff::AMBITION) as i32;
        let professionalism = read_i8(staff + staff::PROFESSIONALISM) as i32;
        let age = read_i8(staff + staff::AGE);
        for role in 1..8u8 {
            let sheet = read_usize(staff + staff::PLAYER);
            if sheet == 0 {
                break;
            }
            if role == 7 {
                continue;
            }
            // the player's reputation is read again from the record on every test, as the original does
            let player_rep = || read_i16(read_usize(staff + staff::PLAYER) + player_sheet::CURRENT_REPUTATION) as i32;
            let eligible = if role == 1 {
                let veteran = (professionalism + 0x46) * 25 + read_i16(sheet + player_sheet::CURRENT_REPUTATION) as i32 + ambition * 10 >= reputation;
                if age >= 0x18 {
                    veteran
                } else {
                    let r = read_i16(sheet + player_sheet::CURRENT_REPUTATION);
                    r >= 0xcb2 && r as i32 + (ambition + professionalism * 2) * 10 >= reputation && veteran
                }
            } else if contract != 0 && cs_contract_satisfaction(contract, staff) == 1 && (cs_squad_status(contract) as i8) <= role as i8 {
                false
            } else if role == 2 {
                age >= 0x18 || read_usize(staff + staff::PLAYER) == 0 || player_rep() + (ambition + professionalism * 2 + 0x4b) * 10 >= reputation
            } else if role == 3 {
                age >= 0x18 || read_usize(staff + staff::PLAYER) == 0 || player_rep() + (ambition + professionalism * 2 + 0x7d) * 10 >= reputation
            } else {
                true
            };
            if !eligible || !cs_status_allowed(staff, role as u32) {
                continue;
            }
            let mut flags: u32 = 0;
            let v = exe::shortlist_manager_staff_interest_in_joining_club(cs_shortlist_manager(), staff, club, 1, 0, role as u32, &mut flags as *mut u32 as u32);
            if !(v > threshold) && v > best_interest && flags & 0x8000 == 0 {
                best_interest = v;
                best = role;
            }
        }
    }
    if talks_role > -1 {
        set_screen_param(1, talks_role as i32 as usize);
        set_screen_param(6, talks_role as i32 as usize);
    } else {
        set_screen_param(1, 0);
        set_screen_param(6, best as i8 as i32 as usize);
    }
    set_screen_param(2, slot);
    set_screen_param(3, force as usize);
    set_screen_param(4, job as i8 as i32 as usize);
    set_screen_param(5, talks);
    let serial = if slot != 0 && read_usize(slot) != 0 { read_u32(read_usize(slot) + offer::SERIAL) as usize } else { 0 };
    set_screen_param(7, serial);
}

/// Comparer of `contract_screens_offer_contract_to_staff`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_contract_screens_offer_contract_to_staff(staff: usize, slot: usize, force: u32, ask: u32) {
    original_fn::<unsafe extern "C" fn(usize, usize, u32, u32)>(%%I0%%)(staff, slot, force, ask)
}

/// Builds the set role screen: the squad roles a player can be offered and the jobs a person can take.
///
/// Params: 0 the person, 1 the chosen role, 2 the transfer offer slot, 4 the chosen job, 5 the talks record, 6 the role
/// expected by the player; with a slot, the bid's total value (`get_total_value(0, 0, 1, 1)`). The person's picture,
/// the title in the human's club kit colours with his name, the subtitle "Set Role At Club", his name item (id 0x7d7)
/// and two areas of as many rows as fit: the labels (columns 2:0x29) and the choices (2:0x11). For a player, "Squad
/// Status" and the roles 1..7 (7 skipped), each "* role" when it is the expected one: grey text when the job is not a
/// player job; with an expected role and talks, yellow text for the chosen one and grey for the rest; "+ role" in
/// grey when the role is not allowed (`squad_status_allowed`), or the bid is above the role's maximum fee
/// (`max_fee_for_player_status`, -1 none); otherwise a button (id 3, the role as value), yellow when chosen. With
/// more than one job open, "Job Description" and a button (id 0x2f) for every job he can take (job 10 not offered to
/// a player who also has a non-player record), yellow when chosen. Then, unless he is both player and non-player: "*
/// Offered to the player on <date>" (talks) or "* Expected by the player" when the expected role was listed, else "+
/// Not allowed by the board because of large transfer fee" (when the fee blocked a role and he is not at the club) or
/// "... due to the players age" when a role was refused; and "+ Player not interested in ending his playing career at
/// this time" for a player over 30 without a non-player record. Last the menu bar and the bar Back (id 1) / Set (id 2;
/// grey text when no role 1..6 is chosen for a player in a player job).
/// Original: `contract_screens_build_set_role_screen` (0x4dd6c0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_build_set_role_screen() {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let status_text: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(CS_FN_SQUAD_STATUS_TEXT);
    let max_fee: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32 = core::mem::transmute(CS_FN_MAX_FEE_FOR_STATUS);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let staff = get_screen_param(0);
    let current = get_screen_param(1) as u8 as i8;
    let slot = get_screen_param(2);
    let job = get_screen_param(4) as u8 as i8;
    let talks = get_screen_param(5);
    let expected = get_screen_param(6) as u8 as i8;
    let mut total: i32 = 0;
    if slot != 0 {
        total = exe::transfer_offer_get_total_value(read_usize(slot), 0, 0, 1, 1);
    }
    let jobs = cs_jobs_open_to(staff);
    let club = cs_human_club();
    picture(staff, 0, 0);
    let mut a = 0u32;
    let mut b = 0u32;
    exe::database_get_club_kit_colours(club, 1, &mut a, &mut b);
    let mut name = [0u8; 500];
    exe::database_format_staff_name(staff, name.as_mut_ptr() as usize, 500, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, b, a, name.as_ptr() as usize);
    lang(TS_TEXT, 0x9afe2c); // "Set Role At Club"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    tel26_add_item(current_human_display(), [6, 0x7d, 10, 0x316, 0x46, 0, 0, 1, 0, 0, 0xc, 7, 0, 0, 0, 0x7d7, staff as u32, u32::MAX]);
    let lines = ts_dialog_lines();
    let labels_w: [u8; 7] = [2, 0x29, 0, 0, 0, 0, 0];
    let labels = tel26_add_area(current_human_display(), [0x6e, 0x8c, 0x30c, 0x221, 2, labels_w.as_ptr() as u32, lines as u32, 0, 2, 0, u32::MAX]);
    let pale = ts_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = ts_colour(G_GUI_COLOUR_GREY);
    let yellow = ts_colour(G_GUI_COLOUR_YELLOW);
    let mut row: u32 = 0;
    if read_usize(staff + staff::PLAYER) != 0 {
        lang(TS_TEXT, 0x9afe1c); // "Squad Status"
        tel26_area_button([labels, 1, 0, 1, 0, 1, 3, pale, TS_TEXT as u32, 0, 0]);
        row = 2;
    }
    let choices_w: [u8; 2] = [2, 0x11];
    let choices = tel26_add_area(current_human_display(), [0x6e, 0x8c, 0x30c, 0x221, 2, choices_w.as_ptr() as u32, lines as u32, 0, 1, 0, u32::MAX]);
    let mut listed_expected = false;
    let mut refused = false;
    let mut fee_blocked = false;
    let mut role_text = [0u8; 100];
    let rt = role_text.as_mut_ptr() as usize;
    for role in 1..8i8 {
        if read_usize(staff + staff::PLAYER) == 0 {
            break;
        }
        if role == 7 {
            continue;
        }
        status_text(cs_shortlist_manager(), role as u32, rt, 2000);
        if role == expected {
            sprintf(TS_TEXT, 0x9afe14, rt); // "* %s"
            listed_expected = true;
        } else {
            tel26_copy_text(TS_TEXT as *mut u8, rt as *const u8);
        }
        let fee_cap = max_fee(cs_shortlist_manager(), club, staff, role as u32);
        if exe::database_is_player_job(job) == 0 {
            tel26_area_text([choices, 1, row, 1, 0, 1, 3, grey, TS_TEXT as u32]);
        } else if expected != 0 && talks != 0 {
            let colour = if role == current { yellow } else { grey };
            tel26_area_text([choices, 1, row, 1, 0, 1, 3, colour, TS_TEXT as u32]);
        } else {
            let over_fee = slot != 0 && total > fee_cap && fee_cap != -1;
            if !cs_status_allowed(staff, role as u32) || over_fee || read_usize(staff + staff::PLAYER) == 0 {
                covered!("tel24_1 set role refused");
                sprintf(TS_TEXT, 0x9afe0c, rt); // "+ %s"
                refused = true;
                if slot != 0 && total > fee_cap && fee_cap != -1 && read_usize(staff + staff::PLAYER) != 0 {
                    fee_blocked = true;
                }
                tel26_area_text([choices, 1, row, 1, 0, 1, 3, grey, TS_TEXT as u32]);
            } else {
                let colour = if role == current { yellow } else { pale };
                tel26_area_button([choices, 1, row, 1, 0, 1, 3, colour, TS_TEXT as u32, 3, role as i32 as u32]);
            }
        }
        row += 1;
    }
    row += 1;
    if jobs > 1 {
        covered!("tel24_1 set role job list");
        lang(TS_TEXT, 0x9afdfc); // "Job Description"
        tel26_area_button([labels, 1, row, 1, 0, 1, 3, pale, TS_TEXT as u32, 0, 0]);
        row += 2;
        for k in 0..16i8 {
            if !cs_can_take_job(k as u32, staff) {
                continue;
            }
            if k == 10 && read_usize(staff + staff::NON_PLAYER) != 0 && read_usize(staff + staff::PLAYER) != 0 {
                continue;
            }
            let n = exe::database_get_job_name(k as u32, 0, 1, core::ptr::null_mut());
            tel26_copy_text(TS_TEXT as *mut u8, n as *const u8);
            let colour = if job == k { yellow } else { pale };
            tel26_area_button([choices, 1, row, 1, 0, 1, 3, colour, TS_TEXT as u32, 0x2f, k as i32 as u32]);
            row += 1;
        }
    }
    if read_usize(staff + staff::NON_PLAYER) == 0 || read_usize(staff + staff::PLAYER) == 0 {
        if expected != 0 && listed_expected {
            row += 1;
            if talks != 0 {
                let mut date = [0u8; 100];
                exe::date_get_long_date_string_no_year(talks + 0xc, date.as_mut_ptr() as usize, 100, 0xff);
                lang(TS_TEXT, 0x9afdb8, 100, date.as_ptr() as usize); // "* Offered to the player on ..."
            } else {
                lang(TS_TEXT, 0x9afd9c); // "* Expected by the player"
            }
            tel26_area_text([labels, 1, row, 1, 0, 1, 3, pale, TS_TEXT as u32]);
        } else if refused {
            row += 1;
            if fee_blocked && (club == 0 || club != read_usize(staff + staff::CLUB_JOB)) {
                lang(TS_TEXT, 0x9afd60); // "+ Not allowed by the board because of large transfer fee"
            } else {
                lang(TS_TEXT, 0x9afd2c); // "+ Not allowed by the board due to the players age"
            }
            tel26_area_text([labels, 1, row, 1, 0, 1, 3, pale, TS_TEXT as u32]);
        }
        if read_usize(staff + staff::PLAYER) != 0 && read_usize(staff + staff::NON_PLAYER) == 0 && read_i8(staff + staff::AGE) > 0x1e {
            lang(TS_TEXT, 0x9afce8); // "+ Player not interested in ending his playing career at this time"
            row += 1;
            tel26_area_text([labels, 1, row, 1, 0, 1, 3, pale, TS_TEXT as u32]);
        }
    }
    exe::menubar_build(4, 0);
    let bar_w: [u8; 2] = [3, 1];
    let bar = tel26_add_area(current_human_display(), [100, 0x22b, 0x316, 0x24e, 2, bar_w.as_ptr() as u32, 1, 0, 1, b, u32::MAX]);
    lang(TS_TEXT, TS_BACK);
    tel26_area_button([bar, 0, 0, 0x30, grey, 0xc, 3, pale, TS_TEXT as u32, 1, 0]);
    lang(TS_TEXT, 0x9afcc0, STR_EMPTY); // "Set<%s - COMMENT - set squad status>"
    if !(1..7).contains(&current) && read_usize(staff + staff::PLAYER) != 0 && exe::database_is_player_job(job) != 0 {
        tel26_area_text([bar, 1, 0, 0x30, grey, 0x2c, 3, pale, TS_TEXT as u32]);
        return;
    }
    tel26_area_button([bar, 1, 0, 0x30, grey, 0xc, 3, pale, TS_TEXT as u32, 2, 0]);
}

/// Comparer of `contract_screens_build_set_role_screen`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_contract_screens_build_set_role_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_by_display_list_with("Set role screen", &[], || o(), || contract_screens_build_set_role_screen());
}

/// Answers the set role screen.
///
/// A role button (id 3) keeps its value in param 1 and a job button (id 0x2f) in param 4 (both redraw, -11); Back (id
/// 1) answers -9. Set (id 2) records the talks (`CONTRACT_MANAGER::add_talks_record(person, manager, club, |role|)`
/// with the slot manager and his club); then, when the transfer offer of param 2 changed since the screen opened
/// (its serial is not param 7's) or was rejected or withdrawn (status 2 or 3), the offer changed screen (-11);
/// otherwise the offer contract screen with the role and the job of param 4 (for the transfer when param 3 is set),
/// answering -4. Anything else goes to the menu bar.
/// Original: `contract_screens_handle_set_role_screen` (0x4ddf20, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_handle_set_role_screen(item: u32) -> i32 {
    let item16 = item as i16;
    let id = tel26_item_id(item16);
    if id == 3 {
        set_screen_param(1, tel26_item_value(item16) as usize);
        return -11;
    }
    if id == 0x2f {
        set_screen_param(4, tel26_item_value(item16) as usize);
        return -11;
    }
    if id == 1 {
        return -9;
    }
    if id != 2 {
        return ts_menubar_answer(item);
    }
    covered!("tel24_1 set role Set");
    let add_talks: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(CS_FN_ADD_TALKS_RECORD);
    let open_offer: unsafe extern "C" fn(usize, u32, u32, u32) = core::mem::transmute(CS_FN_OPEN_OFFER);
    let open_for_transfer: unsafe extern "C" fn(usize, usize, u32, u32, u32) = core::mem::transmute(CS_FN_OPEN_OFFER_FOR_TRANSFER);
    let changed: unsafe extern "C" fn(usize, u32) = core::mem::transmute(CS_FN_OPEN_OFFER_CHANGED);
    let staff = get_screen_param(0);
    let role = (get_screen_param(1) as u8 as i8 as i32).unsigned_abs() as u8;
    let slot = get_screen_param(2);
    let offer_ = if slot != 0 { read_usize(slot) } else { 0 };
    let force = get_screen_param(3);
    let job = get_screen_param(4) as u8;
    let mgr = tel26_slot_manager().addr();
    add_talks(read_usize(G_CONTRACT_MANAGER), staff, mgr, read_usize(mgr + staff::CLUB_JOB), role as u32);
    let serial = get_screen_param(7) as u32;
    if offer_ != 0 && (read_u32(offer_ + offer::SERIAL) != serial || matches!(read_u8(offer_ + offer::STATUS), 2 | 3)) {
        changed(offer_, serial);
        return -11;
    }
    if force != 0 {
        open_for_transfer(slot, staff, role as u32, job as u32, 1);
    } else {
        open_offer(staff, role as u32, job as u32, 1);
    }
    -4
}

/// Comparer of `contract_screens_handle_set_role_screen`: the role, job and Back answers side by side over the
/// screen params; Set and the menu bar only in the original (they record the talks and open screens).
pub(crate) unsafe extern "C" fn compare_contract_screens_handle_set_role_screen(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%);
    ts_compare_answer("Set role screen", item, &[3, 0x2f, 1], o, contract_screens_handle_set_role_screen)
}

/// Opens the offer contract screen (build `contract_screens_build_offer_contract_screen`, input
/// `contract_screens_handle_offer_contract_screen`, flag 1, exit `contract_screens_close_offer_contract_screen`) for a
/// person, and fills its params: 0 his id, 1 `mode`, 8 the job, 0xd the role, 0xa and 0xc 1900, 0x15 the transfer
/// budget (`cs_budget`), 0x17 the value the club puts on him (`cs_offer_value`), 0x18 the wage limit (the club's
/// `get_max_wage(person, -1)` for the jobs 0xb, 0xc, 0xd, 0xf, else `cs_staff_wage_limit`), 0x1b his contract's
/// grievances (`contract::GRIEVANCES`, 0 without a contract), 0x1c 3, the rest (2, 3, 6, 7, 9, 0xb, 0xe..0x14, 0x16,
/// 0x19, 0x1a, 0x1d) 0.
/// Original: `contract_screens_open_offer_contract_screen` (0x4de1f0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_open_offer_contract_screen(staff: usize, role: u32, job: u32, mode: u32) {
    let club = cs_human_club();
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, CS_BUILD_OFFER, CS_HANDLE_OFFER, 1, CS_CLOSE_OFFER, 0) == 0 {
        return;
    }
    covered!("tel24_1 open offer contract screen");
    set_screen_param(0, read_u32(staff) as usize);
    set_screen_param(1, mode as i16 as i32 as usize);
    set_screen_param(2, 0);
    set_screen_param(3, 0);
    set_screen_param(6, 0);
    set_screen_param(7, 0);
    set_screen_param(8, job as i8 as i32 as usize);
    set_screen_param(9, 0);
    set_screen_param(0xa, 0x76c);
    set_screen_param(0xb, 0);
    set_screen_param(0xc, 0x76c);
    set_screen_param(0xd, role as i8 as i32 as usize);
    for k in 0xe..=0x14 {
        set_screen_param(k, 0);
    }
    set_screen_param(0x15, cs_budget(club, staff, 0) as usize);
    set_screen_param(0x16, 0);
    set_screen_param(0x17, cs_offer_value(staff, club) as usize);
    let limit = if cs_player_wage_job(job as i8) {
        let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        exe::finance_get_max_wage(fin, staff, u32::MAX) as f64
    } else {
        cs_staff_wage_limit(club, staff, job)
    };
    set_screen_param(0x18, ftol(limit) as usize);
    set_screen_param(0x19, 0);
    set_screen_param(0x1a, 0);
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
    set_screen_param(0x1b, if c != 0 { read_u32(c + contract::GRIEVANCES) as usize } else { 0 });
    set_screen_param(0x1c, 3);
    set_screen_param(0x1d, 0);
}

/// Comparer of `contract_screens_open_offer_contract_screen`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_contract_screens_open_offer_contract_screen(staff: usize, role: u32, job: u32, mode: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32, u32, u32)>(%%I3%%)(staff, role, job, mode)
}

/// Opens the offer contract screen for the player of a transfer offer (the same screen and functions as
/// `contract_screens_open_offer_contract_screen`) and fills its params: 0 his id, 1 `mode`, 2 0, 3 the offer slot, 8
/// the job, 0xd the role, 0xa and 0xc 1900; with an offer, 7 its "player aware" byte, 4 its slot index, 5 its serial and
/// 6 whether it is for someone else (0 when its target is this person), else 6 and 7 0; 0xe..0x14 0; 0x15 the transfer
/// budget less the bid's fee (`cs_budget`); 0x16 his lowest wage demand for a player
/// (`contract_manager_lowest_wage_demand`), 0x17 the value the club puts on him; 0x18 the wage limit (the club's
/// `get_max_wage(person, role)` for the jobs 0xb, 0xc, 0xd, 0xf, else `cs_staff_wage_limit`); 0x19, 0x1a 0; 0x1b his
/// contract's grievances; 0x1c 3 and 0x1d 0 when his contract is in the FIFA protected period
/// (`check_fifa_protected_period(club, 0, 0)`) or there is an offer, else 0x1c 0 and 0x1d 1 for a player under
/// contract (0 otherwise).
/// Original: `contract_screens_open_offer_contract_screen_for_transfer` (0x4de5d0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_open_offer_contract_screen_for_transfer(slot: usize, staff: usize, role: u32, job: u32, mode: u32) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, CS_BUILD_OFFER, CS_HANDLE_OFFER, 1, CS_CLOSE_OFFER, 0) == 0 {
        return;
    }
    covered!("tel24_1 open offer contract screen for a transfer");
    set_screen_param(1, mode as i16 as i32 as usize);
    set_screen_param(2, 0);
    set_screen_param(3, slot);
    set_screen_param(8, job as i8 as i32 as usize);
    set_screen_param(9, 0);
    set_screen_param(0xa, 0x76c);
    set_screen_param(0xb, 0);
    set_screen_param(0xc, 0x76c);
    set_screen_param(0xd, role as i8 as i32 as usize);
    set_screen_param(0, read_u32(staff) as usize);
    if slot != 0 {
        let o = read_usize(slot);
        set_screen_param(7, read_i8(o + offer::PLAYER_AWARE) as i32 as usize);
        set_screen_param(4, read_u32(read_usize(slot) + offer::SLOT_INDEX) as usize);
        set_screen_param(5, read_u32(read_usize(slot) + offer::SERIAL) as usize);
        let other = read_u32(read_usize(slot) + offer::TARGET_STAFF_ID) != read_u32(staff);
        set_screen_param(6, other as usize);
    } else {
        set_screen_param(6, 0);
        set_screen_param(7, 0);
    }
    for k in 0xe..=0x14 {
        set_screen_param(k, 0);
    }
    let club = cs_human_club();
    let fee = if slot != 0 && read_usize(slot) != 0 { read_i32(read_usize(slot) + offer::FEE) } else { 0 };
    set_screen_param(0x15, cs_budget(club, staff, fee) as usize);
    let demand = if read_usize(staff + staff::PLAYER) != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(CS_FN_LOWEST_WAGE_DEMAND);
        f(read_usize(G_CONTRACT_MANAGER), staff, club)
    } else {
        0
    };
    set_screen_param(0x16, demand as usize);
    set_screen_param(0x17, cs_offer_value(staff, cs_human_club()) as usize);
    let limit = if cs_player_wage_job(job as i8) {
        let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        exe::finance_get_max_wage(fin, staff, role) as f64
    } else {
        cs_staff_wage_limit(club, staff, job)
    };
    set_screen_param(0x18, ftol(limit) as usize);
    set_screen_param(0x19, 0);
    set_screen_param(0x1a, 0);
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
    set_screen_param(0x1b, if c != 0 { read_u32(c + contract::GRIEVANCES) as usize } else { 0 });
    if c != 0 && (exe::staff_contract_check_fifa_protected_period(c, club, 0usize, 0usize) != 0 || slot != 0) {
        set_screen_param(0x1c, 3);
        set_screen_param(0x1d, 0);
        return;
    }
    set_screen_param(0x1c, 0);
    set_screen_param(0x1d, (c != 0 && read_usize(staff + staff::PLAYER) != 0) as usize);
}

/// Comparer of `contract_screens_open_offer_contract_screen_for_transfer`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_contract_screens_open_offer_contract_screen_for_transfer(slot: usize, staff: usize, role: u32, job: u32, mode: u32) {
    original_fn::<unsafe extern "C" fn(usize, usize, u32, u32, u32)>(%%I4%%)(slot, staff, role, job, mode)
}

/// Adds a disabled (greyed) item to a popup list of the screen being built: `Display::add_item` of the current slot's
/// display with the flags 2, 0x10, the popup colours (0xae31aa, the text colour 0xaebdec, the highlight colour
/// 0xaebdea), 0x21, 1, the text and the area; no id or value. The colours go as the original pushes them: the word in
/// the low half and, in the high half, what the register held (the area's high word for 0xae31aa and 0xaebdea, the
/// text pointer's for 0xaebdec). The item index comes back.
/// Original: `contract_screens_add_disabled_menu_item` (0x4e0180, contract_screens.cpp, thiscall on the screen
/// manager, ret 8).
pub(crate) unsafe extern "thiscall" fn contract_screens_add_disabled_menu_item(sm: usize, area: u32, text: u32) -> u32 {
    let high = |v: u32, colour: usize| (v & 0xffff_0000) | read_u16(colour) as u32;
    tel26_add_item(screens_current_display(sm), [2, 0, 0, 0, 0, 0, 0, 0x10, high(area, JOB_COLOUR_31AA), high(text, G_GUI_COLOUR_YELLOW), 0x21, 1,
        high(area, G_GUI_COLOUR_BLACK), text, 0, 0, 0, area])
}

/// Comparer of `contract_screens_add_disabled_menu_item`: side by side over what a forwarder writes and the return.
pub(crate) unsafe extern "thiscall" fn compare_contract_screens_add_disabled_menu_item(sm: usize, area: u32, text: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I5%%);
    compare_screens_forwarder("contract_screens_add_disabled_menu_item", sm, || o(sm, area, text), || contract_screens_add_disabled_menu_item(sm, area, text))
}

/// Leaves the offer contract screen: the edited offer kept in param 2 (when there is one) is destroyed (its no-op
/// destructor, operator delete) and param 2 cleared.
/// Original: `contract_screens_close_offer_contract_screen` (0x4e06a0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_close_offer_contract_screen() {
    let edited = get_screen_param(2);
    if edited == 0 {
        return;
    }
    covered!("tel24_1 close offer contract screen");
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(CS_FN_OFFER_NOOP_DESTRUCTOR);
    destroy(edited);
    exe::crt_operator_delete(edited);
    set_screen_param(2, 0);
}

/// Comparer of `contract_screens_close_offer_contract_screen`: only the original runs (it frees the offer).
pub(crate) unsafe extern "C" fn compare_contract_screens_close_offer_contract_screen() {
    original_fn::<unsafe extern "C" fn()>(%%I6%%)()
}

/// Opens the Yes/No dialog that offers (or withdraws) the human's contract offer to a person (build
/// `contract_screens_build_staff_offer_contract_screen`, input `contract_screens_handle_staff_offer_contract_screen`,
/// flag 1) and fills its params: 0 the transfer offer slot, 3 and 4 two flags (bytes), 5 `withdraw`, 6 the transfer
/// window state (byte), 7 `window`; 1 and 2 the slot offer's slot index and serial (-1 without an offer).
/// Original: `contract_screens_open_staff_offer_contract_screen` (0x4e4730, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_open_staff_offer_contract_screen(slot: usize, flag3: u32, flag4: u32, withdraw: u32, window: u32, window_state: u32) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, CS_BUILD_STAFF_OFFER, CS_HANDLE_STAFF_OFFER, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel24_1 open staff offer contract dialog");
    set_screen_param(0, slot);
    set_screen_param(3, flag3 as i8 as i32 as usize);
    set_screen_param(4, flag4 as i8 as i32 as usize);
    set_screen_param(5, withdraw as usize);
    set_screen_param(6, window_state as i8 as i32 as usize);
    set_screen_param(7, window as usize);
    if slot != 0 && read_usize(slot) != 0 {
        set_screen_param(1, read_u32(read_usize(slot) + offer::SLOT_INDEX) as usize);
        set_screen_param(2, read_u32(read_usize(slot) + offer::SERIAL) as usize);
    } else {
        set_screen_param(1, usize::MAX);
        set_screen_param(2, usize::MAX);
    }
}

/// Comparer of `contract_screens_open_staff_offer_contract_screen`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_contract_screens_open_staff_offer_contract_screen(slot: usize, flag3: u32, flag4: u32, withdraw: u32, window: u32, window_state: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32)>(%%I7%%)(slot, flag3, flag4, withdraw, window, window_state)
}

/// Builds the dialog that confirms offering (or withdrawing) the human's contract offer.
///
/// The person is the one of the human manager's edited offer (`HUMAN_MANAGER::get_contract_offer(0)`, its first
/// dword the person's id); none (-1): the contract_screens.cpp assert box, line 0x11a0. His contract club is the club
/// of his contract unless it is of type 5. His picture, a title area (100,10..0x316,0x46) in the slot manager's club kit
/// colours with "name (club short name)" (the club name a link 0x7d5 that points to the human's own club, as the
/// original pushes it) or just the name when he has no club, the subtitle "Offer Contract" or "Withdraw Contract
/// Offer" (param 5). The text: "Withdraw the contract offer to <name> ?" or "Offer contract to <name> ?"; when he is
/// under contract elsewhere and there is no transfer slot (param 0), the compensation fee
/// (`transfer_manager_calc_compensation_fee(person, club, 1 unless the window is closed and param 6 is 0)`) is added
/// when positive ("... compensation of <fee>", the fee in mode 6). With param 7 set: param 6 = 0 adds when the next
/// international transfer window starts (or that the move happens in the current one when it starts today); 3, that
/// he moves at the end of his contract. The text area (links: the name 0x7da to the person and, with a fee, the club
/// 0x7d5), the menu bar and the bar No (id 0x15) / Yes (id 0x14).
/// Original: `contract_screens_build_staff_offer_contract_screen` (0x4e4820, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_build_staff_offer_contract_screen() {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let human_offer: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(CS_FN_HUMAN_CONTRACT_OFFER);
    let draw: unsafe extern "thiscall" fn(usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(TS_FN_TEXT_DRAW_WRAPPED);
    let text_area: unsafe extern "C" fn(u32, ...) = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let mut text = [0usize; 8];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0usize);
    let mut fee: i32 = 0;
    let mut date = [0u32; 2];
    exe::date_date_54a110(date.as_mut_ptr() as usize);
    let id = read_i32(human_offer(read_usize(G_HUMAN_MANAGER), 0));
    let club = cs_human_club();
    if id == -1 {
        tel26_error_box(CS_CPP, 0x11a0);
        exe::drop_text(t);
        return;
    }
    covered!("tel24_1 staff offer contract dialog");
    let staff = staff_at(id as isize).addr();
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    let contract_club = if c == 0 || cs_contract_type(c) == 5 { 0 } else { club_at(read_i32(c + contract::CLUB) as isize).addr() };
    let withdraw = get_screen_param(5);
    let window_state = get_screen_param(6) as u8;
    let window = get_screen_param(7);
    picture(staff, 0, 0);
    let mut a = 0u32;
    let mut b = 0u32;
    exe::database_get_club_kit_colours(club, 1, &mut a, &mut b);
    let mut name = [0u8; 300];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(staff, n, 300, 1);
    let own_club = read_usize(staff + staff::CLUB_JOB);
    if own_club != 0 {
        sprintf(TS_TEXT, 0x9a3d8c, n, own_club + club::SHORT_NAME); // "%s (%s)"
        exe::text_set_string(t as usize, TS_TEXT as *const u8);
        exe::text_add_link(t as usize, (read_usize(staff + staff::CLUB_JOB) + club::SHORT_NAME) as *const u8, 0x7d5, club);
    } else {
        tel26_copy_text(TS_TEXT as *mut u8, n as *const u8);
        exe::text_set_string(t as usize, TS_TEXT as *const u8);
    }
    let title = tel26_add_area(current_human_display(), [100, 10, 0x316, 0x46, 1, 0, 1, 0, 0x30, b, u32::MAX]);
    let mut row: i32 = 0;
    draw(t as usize, title, 100, 0x316, &mut row, 7, a, 1, 0);
    lang(TS_TEXT, if withdraw != 0 { 0x9b109c } else { 0x9aff10 }); // "Withdraw Contract Offer" / "Offer Contract"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let lines = ts_dialog_lines();
    let mut money = [0u8; 200];
    if contract_club != 0 && contract_club != club && get_screen_param(0) == 0 {
        let open = (window == 0 || window_state != 0) as u32;
        fee = exe::transfer_manager_calc_compensation_fee(read_usize(G_TRANSFER_MANAGER), staff, club, open);
        if fee > 0 {
            covered!("tel24_1 staff offer compensation fee");
            let cash = fee as u32;
            exe::cash_format_money(money.as_mut_ptr() as usize, 200, exe::cash_get_display_amount(&cash, 6), 6);
        }
    }
    let mut extra = [0u8; 500];
    let x = extra.as_mut_ptr() as usize;
    if withdraw != 0 {
        lang(TS_TEXT, 0x9b1054, n); // "Withdraw the contract offer to <name> ?"
    } else {
        lang(TS_TEXT, 0x9b1018, n); // "Offer contract to <name> ?"
        if fee != 0 {
            let g = read_i8(contract_club + club::SHORT_GENDER_NAME) as i32;
            lang(x, 0x9b0fb8, g, g, contract_club + club::SHORT_NAME, money.as_ptr() as usize);
            cs_append_text(TS_TEXT, x);
        }
    }
    if window != 0 {
        let mut add = true;
        if window_state == 0 {
            exe::transfer_manager_get_next_window_dates(&mut date, 0usize);
            let today = [read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE + 4)];
            if today[0] as u16 == date[0] as u16 && (today[0] >> 16) as u16 == (date[0] >> 16) as u16 {
                lang(x, 0x9b0f24); // "If accepted, the player will move in the current international transfer window"
            } else {
                let mut when = [0u8; 100];
                exe::date_get_long_date_string(date.as_mut_ptr() as usize, when.as_mut_ptr() as usize, 100, 0xff);
                lang(x, 0x9b0f78, when.as_ptr() as usize); // "The next international transfer window starts on <date>."
            }
        } else if window_state == 3 {
            lang(x, 0x9b0edc); // "If accepted, the player will move at the end of his current contract"
        } else {
            add = false;
        }
        if add {
            cs_append_text(TS_TEXT, x);
        }
    }
    let pale = ts_colour(G_GUI_COLOUR_PALE_GREY);
    if fee != 0 && contract_club != 0 {
        text_area(0x6e, 0x8c, 0x30c, 0x221, 2u32, 0u32, lines as i8 as i32, 3u32, pale, TS_TEXT, n, 0x7dau32, staff, contract_club + club::SHORT_NAME, 0x7d5u32, contract_club, 0u32);
    } else {
        text_area(0x6e, 0x8c, 0x30c, 0x221, 2u32, 0u32, lines as i8 as i32, 3u32, pale, TS_TEXT, n, 0x7dau32, staff, 0u32);
    }
    exe::menubar_build(4, 0);
    let bar_w: [u8; 2] = [3, 1];
    let bar = tel26_add_area(current_human_display(), [100, 0x22b, 0x316, 0x24e, 2, bar_w.as_ptr() as u32, 1, 0, 1, b, u32::MAX]);
    let grey = ts_colour(G_GUI_COLOUR_GREY);
    lang(TS_TEXT, TS_NO);
    tel26_area_button([bar, 0, 0, 0x30, grey, 0xc, 3, pale, TS_TEXT as u32, 0x15, 0]);
    lang(TS_TEXT, TS_YES);
    tel26_area_button([bar, 1, 0, 0x30, grey, 0xc, 3, pale, TS_TEXT as u32, 0x14, 0]);
    exe::drop_text(t);
}

/// `strcat(dst, src)`, as the original's inline `repne scasb` / `rep movs`.
pub(crate) unsafe fn cs_append_text(dst: usize, src: usize) {
    let mut end = dst;
    while read_u8(end) != 0 {
        end += 1;
    }
    tel26_copy_text(end as *mut u8, src as *const u8);
}

/// Comparer of `contract_screens_build_staff_offer_contract_screen`: side by side over the display list and the
/// contract lookup global; without an edited offer (the assert box) only the original runs.
pub(crate) unsafe extern "C" fn compare_contract_screens_build_staff_offer_contract_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I8%%);
    let human_offer: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(CS_FN_HUMAN_CONTRACT_OFFER);
    if read_i32(human_offer(read_usize(G_HUMAN_MANAGER), 0)) == -1 {
        return o();
    }
    compare_by_display_list_with("Staff offer contract dialog", &[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(), || contract_screens_build_staff_offer_contract_screen());
}
