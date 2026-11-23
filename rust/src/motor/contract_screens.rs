use super::*;

/// Rule: the "Offer Contract" button of a news item opens the contract offer for the player it names.
///
/// `contract_screens_news_offer_contract_action(manager, id)`: the news item (`tel39_news_item`); its slot 2 is the
/// person's id; `contract_screens_offer_contract_to_staff(person, 0, 0, 1)`.
/// Original: `contract_screens_news_offer_contract_action` (0x4dc9b0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_news_offer_contract_action(manager: usize, id: u32) {
    covered!("tel39_2 news offer contract");
    let mut item = [0usize; 0x38];
    let item = item.as_mut_ptr() as usize;
    tel39_news_item(manager, id, item);
    // tel39_2: the player of slot 2
    contract_screens_offer_contract_to_staff(tel39_staff(exe::newsitem_get_slot(item, 2)), 0, 0, 1);
}

/// Rule: the "Negotiate" button of a contract news item opens the negotiation with the person it names; after an
/// accepted transfer offer (news type 0xfa4) it negotiates on that offer.
///
/// `contract_screens_news_negotiate_action(manager, id)`: the news item; its slot 0 is the person's id. With the
/// item's type (+0) 0xfa4: `contract_screens_offer_contract_to_staff(person, TRANSFER_MANAGER::get_offer_slot(slot
/// 7), 1, 0)`; otherwise `(person, 0, 0, 0)`.
/// Original: `contract_screens_news_negotiate_action` (0x4e5770, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_news_negotiate_action(manager: usize, id: u32) {
    covered!("tel39_2 news negotiate");
    let mut item = [0usize; 0x38];
    let item = item.as_mut_ptr() as usize;
    tel39_news_item(manager, id, item);
    let person = tel39_staff(exe::newsitem_get_slot(item, 0));
    if read_u32(item) == NEWS_TYPE_OFFER_ACCEPTED {
        let offer = exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), exe::newsitem_get_slot(item, 7) as i32);
        contract_screens_offer_contract_to_staff(person, offer, 1, 0);
    } else {
        contract_screens_offer_contract_to_staff(person, 0, 0, 0);
    }
}

/// Comparer for `contract_screens_news_offer_contract_action`: only the original runs (it opens a screen; the
/// contract offer's own comparer checks it).
pub(crate) unsafe extern "C" fn compare_contract_screens_news_offer_contract_action(manager: usize, id: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_CONTRACT_SCREENS_NEWS_OFFER_CONTRACT_ACTION)(manager, id)
}

/// Comparer for `contract_screens_news_negotiate_action`: only the original runs (it opens a screen; the contract
/// offer's own comparer checks it).
pub(crate) unsafe extern "C" fn compare_contract_screens_news_negotiate_action(manager: usize, id: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_CONTRACT_SCREENS_NEWS_NEGOTIATE_ACTION)(manager, id)
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
    let staff = Staff(staff);
    let open_offer: unsafe extern "C" fn(usize, u32, u32, u32) = core::mem::transmute(CS_FN_OPEN_OFFER);
    let open_for_transfer: unsafe extern "C" fn(usize, usize, u32, u32, u32) = core::mem::transmute(CS_FN_OPEN_OFFER_FOR_TRANSFER);
    let find_talks: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize = core::mem::transmute(CS_FN_FIND_TALKS_RECORD);
    let offer_job: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(CS_FN_GET_OFFER_JOB);
    let threshold = (staff.loyalty() as i32 + staff.ambition() as i32 + 0x2a
        + staff.professionalism() as i32) as f64 * read_f64(rdata::F64_0_1) + read_f64(rdata::F64_3_5);
    let force = if slot != 0 { 1 } else { force };
    let mgr = tel26_slot_manager();
    if mgr.is_null() {
        tel26_error_box(CS_CPP, 0x193);
        return;
    }
    let club = mgr.club_job();
    if club.is_null() {
        tel26_error_box(CS_CPP, 0x19b);
        return;
    }
    let jobs = cs_jobs_open_to(staff.addr());
    let cm = read_usize(G_CONTRACT_MANAGER);
    let contract = exe::contract_manager_get_contract_for_club(cm, staff, club);
    let job = offer_job(read_usize(G_CONTRACT_MANAGER), staff.addr());
    let talks = find_talks(read_usize(G_CONTRACT_MANAGER), staff.addr(), mgr.addr(), club.addr());
    let mut talks_role: i8 = -1;
    if talks != 0 {
        talks_role = read_i8(talks + 0x14);
        if talks_role > -1 && exe::date_operator_3d_3d(talks + 0xc, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE + 4)) != 0 && jobs < 2 {
            covered!("tel24_1 offer contract after talks today");
            if force != 0 {
                open_for_transfer(slot, staff.addr(), talks_role as u8 as u32, job as u32, 1);
            } else {
                open_offer(staff.addr(), talks_role as u8 as u32, job as u32, 1);
            }
            return;
        }
    }
    if job != 0xb && jobs == 1 {
        covered!("tel24_1 offer contract for the only job");
        if force != 0 {
            open_for_transfer(slot, staff.addr(), talks_role as u8 as u32, job as u32, 1);
        } else {
            open_offer(staff.addr(), talks_role as u8 as u32, job as u32, 1);
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
            open_for_transfer(slot, staff.addr(), role as u32, job as u32, 1);
        } else {
            open_offer(staff.addr(), cs_squad_status(contract) as u32, job as u32, 1);
        }
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, CS_BUILD_SET_ROLE, CS_HANDLE_SET_ROLE, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel24_1 offer contract opens the set role screen");
    set_screen_param(0, staff.addr());
    let player = staff.player().addr();
    let mut best: u8 = 0;
    if contract != 0 && cs_contract_satisfaction(contract, staff.addr()) != 1 {
        let st = cs_squad_status(contract);
        best = st;
        if st == 7 || st == 8 || player == 0 || !cs_status_allowed(staff.addr(), st as u32) {
            best = 0;
        }
    } else {
        let mut best_interest = 3.15f64;
        let reputation = club.reputation() as i32;
        let ambition = staff.ambition() as i32;
        let professionalism = staff.professionalism() as i32;
        let age = staff.age() as i8;
        for role in 1..8u8 {
            let sheet = staff.player();
            if sheet.is_null() {
                break;
            }
            if role == 7 {
                continue;
            }
            // the player's reputation is read again from the record on every test, as the original does
            let player_rep = || read_i16(staff.player().addr() + player_sheet::CURRENT_REPUTATION) as i32;
            let eligible = if role == 1 {
                let veteran = (professionalism + 0x46) * 25 + sheet.current_reputation() as i32 + ambition * 10 >= reputation;
                if age >= 0x18 {
                    veteran
                } else {
                    let r = sheet.current_reputation();
                    r >= 0xcb2 && r as i32 + (ambition + professionalism * 2) * 10 >= reputation && veteran
                }
            } else if contract != 0 && cs_contract_satisfaction(contract, staff.addr()) == 1 && (cs_squad_status(contract) as i8) <= role as i8 {
                false
            } else if role == 2 {
                age >= 0x18 || staff.player().addr() == 0 || player_rep() + (ambition + professionalism * 2 + 0x4b) * 10 >= reputation
            } else if role == 3 {
                age >= 0x18 || staff.player().addr() == 0 || player_rep() + (ambition + professionalism * 2 + 0x7d) * 10 >= reputation
            } else {
                true
            };
            if !eligible || !cs_status_allowed(staff.addr(), role as u32) {
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
    original_fn::<unsafe extern "C" fn(usize, usize, u32, u32)>(IDX_CONTRACT_SCREENS_OFFER_CONTRACT_TO_STAFF)(staff, slot, force, ask)
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
    let staff = Staff(get_screen_param(0));
    let current = get_screen_param(1) as u8 as i8;
    let slot = get_screen_param(2);
    let job = get_screen_param(4) as u8 as i8;
    let talks = get_screen_param(5);
    let expected = get_screen_param(6) as u8 as i8;
    let mut total: i32 = 0;
    if slot != 0 {
        total = exe::transfer_offer_get_total_value(read_usize(slot), 0, 0, 1, 1);
    }
    let jobs = cs_jobs_open_to(staff.addr());
    let club = cs_human_club();
    picture(staff.addr(), 0, 0);
    let mut a = 0u32;
    let mut b = 0u32;
    exe::database_get_club_kit_colours(club, 1, &mut a, &mut b);
    let mut name = [0u8; 500];
    exe::database_format_staff_name(staff, name.as_mut_ptr() as usize, 500, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, b, a, name.as_ptr() as usize);
    lang(TS_TEXT, 0x9afe2c); // "Set Role At Club"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    tel26_add_item(current_human_display(), [6, 0x7d, 10, 0x316, 0x46, 0, 0, 1, 0, 0, 0xc, 7, 0, 0, 0, 0x7d7, staff.addr() as u32, u32::MAX]);
    let lines = ts_dialog_lines();
    let labels_w: [u8; 7] = [2, 0x29, 0, 0, 0, 0, 0];
    let labels = tel26_add_area(current_human_display(), [0x6e, 0x8c, 0x30c, 0x221, 2, labels_w.as_ptr() as u32, lines as u32, 0, 2, 0, u32::MAX]);
    let pale = ts_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = ts_colour(G_GUI_COLOUR_GREY);
    let yellow = ts_colour(G_GUI_COLOUR_YELLOW);
    let mut row: u32 = 0;
    if staff.player().addr() != 0 {
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
        if staff.player().addr() == 0 {
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
        let fee_cap = max_fee(cs_shortlist_manager(), club, staff.addr(), role as u32);
        if exe::database_is_player_job(job) == 0 {
            tel26_area_text([choices, 1, row, 1, 0, 1, 3, grey, TS_TEXT as u32]);
        } else if expected != 0 && talks != 0 {
            let colour = if role == current { yellow } else { grey };
            tel26_area_text([choices, 1, row, 1, 0, 1, 3, colour, TS_TEXT as u32]);
        } else {
            let over_fee = slot != 0 && total > fee_cap && fee_cap != -1;
            if !cs_status_allowed(staff.addr(), role as u32) || over_fee || staff.player().addr() == 0 {
                covered!("tel24_1 set role refused");
                sprintf(TS_TEXT, 0x9afe0c, rt); // "+ %s"
                refused = true;
                if slot != 0 && total > fee_cap && fee_cap != -1 && staff.player().addr() != 0 {
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
            if !cs_can_take_job(k as u32, staff.addr()) {
                continue;
            }
            if k == 10 && staff.non_player().addr() != 0 && staff.player().addr() != 0 {
                continue;
            }
            let n = exe::database_get_job_name(k as u32, 0, 1, core::ptr::null_mut());
            tel26_copy_text(TS_TEXT as *mut u8, n as *const u8);
            let colour = if job == k { yellow } else { pale };
            tel26_area_button([choices, 1, row, 1, 0, 1, 3, colour, TS_TEXT as u32, 0x2f, k as i32 as u32]);
            row += 1;
        }
    }
    if staff.non_player().addr() == 0 || staff.player().addr() == 0 {
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
            if fee_blocked && (club == 0 || club != staff.club_job().addr()) {
                lang(TS_TEXT, 0x9afd60); // "+ Not allowed by the board because of large transfer fee"
            } else {
                lang(TS_TEXT, 0x9afd2c); // "+ Not allowed by the board due to the players age"
            }
            tel26_area_text([labels, 1, row, 1, 0, 1, 3, pale, TS_TEXT as u32]);
        }
        if staff.player().addr() != 0 && staff.non_player().addr() == 0 && (staff.age() as i8) > 0x1e {
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
    if !(1..7).contains(&current) && staff.player().addr() != 0 && exe::database_is_player_job(job) != 0 {
        tel26_area_text([bar, 1, 0, 0x30, grey, 0x2c, 3, pale, TS_TEXT as u32]);
        return;
    }
    tel26_area_button([bar, 1, 0, 0x30, grey, 0xc, 3, pale, TS_TEXT as u32, 2, 0]);
}

/// Comparer of `contract_screens_build_set_role_screen`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_contract_screens_build_set_role_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CONTRACT_SCREENS_BUILD_SET_ROLE_SCREEN);
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
    let offer_ = Offer(if slot != 0 { read_usize(slot) } else { 0 });
    let force = get_screen_param(3);
    let job = get_screen_param(4) as u8;
    let mgr = tel26_slot_manager();
    add_talks(read_usize(G_CONTRACT_MANAGER), staff, mgr.addr(), mgr.club_job().addr(), role as u32);
    let serial = get_screen_param(7) as u32;
    if !offer_.is_null() && ((offer_.serial() as u32) != serial || matches!(offer_.status(), 2 | 3)) {
        changed(offer_.addr(), serial);
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_CONTRACT_SCREENS_HANDLE_SET_ROLE_SCREEN);
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
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff));
    set_screen_param(0x1b, if !c.is_null() { c.grievances() as usize } else { 0 });
    set_screen_param(0x1c, 3);
    set_screen_param(0x1d, 0);
}

/// Comparer of `contract_screens_open_offer_contract_screen`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_contract_screens_open_offer_contract_screen(staff: usize, role: u32, job: u32, mode: u32) {
    original_fn::<unsafe extern "C" fn(usize, u32, u32, u32)>(IDX_CONTRACT_SCREENS_OPEN_OFFER_CONTRACT_SCREEN)(staff, role, job, mode)
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
    let staff = Staff(staff);
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
    set_screen_param(0, read_u32(staff.addr()) as usize);
    if slot != 0 {
        let o = Offer(read_usize(slot));
        set_screen_param(7, (o.player_aware() as i8) as i32 as usize);
        set_screen_param(4, read_u32(read_usize(slot) + offer::SLOT_INDEX) as usize);
        set_screen_param(5, read_u32(read_usize(slot) + offer::SERIAL) as usize);
        let other = read_u32(read_usize(slot) + offer::TARGET_STAFF_ID) != read_u32(staff.addr());
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
    set_screen_param(0x15, cs_budget(club, staff.addr(), fee) as usize);
    let demand = if staff.player().addr() != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(CS_FN_LOWEST_WAGE_DEMAND);
        f(read_usize(G_CONTRACT_MANAGER), staff.addr(), club)
    } else {
        0
    };
    set_screen_param(0x16, demand as usize);
    set_screen_param(0x17, cs_offer_value(staff.addr(), cs_human_club()) as usize);
    let limit = if cs_player_wage_job(job as i8) {
        let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        exe::finance_get_max_wage(fin, staff, role) as f64
    } else {
        cs_staff_wage_limit(club, staff.addr(), job)
    };
    set_screen_param(0x18, ftol(limit) as usize);
    set_screen_param(0x19, 0);
    set_screen_param(0x1a, 0);
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff));
    set_screen_param(0x1b, if !c.is_null() { c.grievances() as usize } else { 0 });
    if !c.is_null() && (exe::staff_contract_check_fifa_protected_period(c, club, 0usize, 0usize) != 0 || slot != 0) {
        set_screen_param(0x1c, 3);
        set_screen_param(0x1d, 0);
        return;
    }
    set_screen_param(0x1c, 0);
    set_screen_param(0x1d, (!c.is_null() && staff.player().addr() != 0) as usize);
}

/// Comparer of `contract_screens_open_offer_contract_screen_for_transfer`: only the original runs (it opens a screen).
pub(crate) unsafe extern "C" fn compare_contract_screens_open_offer_contract_screen_for_transfer(slot: usize, staff: usize, role: u32, job: u32, mode: u32) {
    original_fn::<unsafe extern "C" fn(usize, usize, u32, u32, u32)>(IDX_CONTRACT_SCREENS_OPEN_OFFER_CONTRACT_SCREEN_FOR_TRANSFER)(slot, staff, role, job, mode)
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_CONTRACT_SCREENS_ADD_DISABLED_MENU_ITEM);
    compare_screens_item("contract_screens_add_disabled_menu_item", sm, area, || o(sm, area, text), || contract_screens_add_disabled_menu_item(sm, area, text))
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
    original_fn::<unsafe extern "C" fn()>(IDX_CONTRACT_SCREENS_CLOSE_OFFER_CONTRACT_SCREEN)()
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
    original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32)>(IDX_CONTRACT_SCREENS_OPEN_STAFF_OFFER_CONTRACT_SCREEN)(slot, flag3, flag4, withdraw, window, window_state)
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
    let staff = Staff(staff_at(id as isize).addr());
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
    let contract_club = Club(if c.is_null() || cs_contract_type(c.addr()) == 5 { 0 } else { club_at((c.club() as i32) as isize).addr() });
    let withdraw = get_screen_param(5);
    let window_state = get_screen_param(6) as u8;
    let window = get_screen_param(7);
    picture(staff.addr(), 0, 0);
    let mut a = 0u32;
    let mut b = 0u32;
    exe::database_get_club_kit_colours(club, 1, &mut a, &mut b);
    let mut name = [0u8; 300];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(staff, n, 300, 1);
    let own_club = staff.club_job();
    if !own_club.is_null() {
        sprintf(TS_TEXT, 0x9a3d8c, n, own_club.addr() + club::SHORT_NAME); // "%s (%s)"
        exe::text_set_string(t as usize, TS_TEXT as *const u8);
        exe::text_add_link(t as usize, (staff.club_job().addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
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
    if !contract_club.is_null() && contract_club.addr() != club && get_screen_param(0) == 0 {
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
            let g = (contract_club.short_gender_name() as i8) as i32;
            lang(x, 0x9b0fb8, g, g, contract_club.addr() + club::SHORT_NAME, money.as_ptr() as usize);
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
    if fee != 0 && !contract_club.is_null() {
        text_area(0x6e, 0x8c, 0x30c, 0x221, 2u32, 0u32, lines as i8 as i32, 3u32, pale, TS_TEXT, n, 0x7dau32, staff.addr(), contract_club.addr() + club::SHORT_NAME, 0x7d5u32, contract_club.addr(), 0u32);
    } else {
        text_area(0x6e, 0x8c, 0x30c, 0x221, 2u32, 0u32, lines as i8 as i32, 3u32, pale, TS_TEXT, n, 0x7dau32, staff.addr(), 0u32);
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

/// Comparer of `contract_screens_build_staff_offer_contract_screen`: side by side over the display list and the
/// contract lookup global; without an edited offer (the assert box) only the original runs.
pub(crate) unsafe extern "C" fn compare_contract_screens_build_staff_offer_contract_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CONTRACT_SCREENS_BUILD_STAFF_OFFER_CONTRACT_SCREEN);
    let human_offer: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(CS_FN_HUMAN_CONTRACT_OFFER);
    if read_i32(human_offer(read_usize(G_HUMAN_MANAGER), 0)) == -1 {
        return o();
    }
    compare_by_display_list_with("Staff offer contract dialog", &[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(), || contract_screens_build_staff_offer_contract_screen());
}

/// Builds the offer contract screen: the terms of the contract the human offers a person, on three tabs (basic
/// terms, bonuses, clauses).
///
/// Params read: 0 the person's id, 2 the edited terms, 3 the transfer offer slot, 8 the job, 1 the tab. A person
/// outside the table: the contract_screens.cpp assert box (line 0x56a). On the first build (no edited terms, param 2)
/// new terms are made (`STAFF_CONTRACT::STAFF_CONTRACT`, `set_to_desired_contract(person, club, slot index, role (param
/// 0xd), job, &x, &y)`; param 0x19 the first out value and 0x1a the second (a float) × 1000) and the human's terms
/// filled: from the club's current offer to him (`find_contract_offer`, not of types 5 or 6; its job goes to param 8)
/// or, when the human's terms are for someone else, from the new terms (then `prefill_offer_from_pending`); the wage is
/// cut to the budget of param 0x15. Params 9/0xa the start month and year (his contract's expiry when he is at the
/// terms' club, else today less 32 days), 2 the edited terms, 0xb/0xc the edited terms' expiry month and year.
/// Then the picture; when he plays for the human's club in a player job, a squad button with a popup of the club's
/// first twenty players (the club's squad sorted in place by appearances then by surname; id 3, the player as value)
/// in the club's kit colours; the title (name and club, the club a link 0x7d5) in the human's club colours; "Offer
/// Contract"; the transfer rules button (id 0x31) when the FIFA rules are in force and param 0x1d is set; the tab bar
/// Basic (1) / Bonuses (2) / Clauses (3) for a player job; the tab: `contract_screens_build_basic_terms_tab`,
/// `contract_screens_build_bonuses_tab` or the clauses (below); any other tab the assert box (line 0x662). The
/// clauses: two areas (labels 2:1:0x24, values 2:1:0xe:2:8:2:2:2:6), "Offer player the following clauses:-", the
/// release clauses 0..3 he may have (`is_clause_allowed`; a "*" when the edited terms hold it and he asked for terms),
/// a button each (id 0x13 pale when offered, 0x12 grey when not); the release fee row: without a release clause grey
/// text and "-" / fee / "+" texts; with one, "-" (id 0x28) when the fee is above the minimum (param 0x17 with the
/// minimum fee clause, else 5; the item index in param 0xe), the fee with a popup of the steps from 0 to 15 million
/// (below the minimum disabled, else id 0x26 with the fee), "+" (id 0x27) below 50 million and the last button (id
/// 0x29); "Non Release Fee Clauses:-" and clause 4; "* Requested by the player" on the last row when he asked. Last the
/// menu bar and the bar: without a transfer slot and no open offer (state 4 or 1 or none), Cancel (0x1b) / Offer (1);
/// otherwise Back (0x1b) / Cancel (2) / Offer (1).
/// Original: `contract_screens_build_offer_contract_screen` (0x4deac0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_build_offer_contract_screen() {
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(TS_FN_STAFF_PICTURE);
    let draw: unsafe extern "thiscall" fn(usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(TS_FN_TEXT_DRAW_WRAPPED);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    // simplification: the three MenuItem objects of the tab bar start zeroed apart from what the constructor and the
    // routine write (the original leaves the rest of the frame as it was)
    let mut tabs = [0u8; 0xd4 * 3];
    let mut text = [0usize; 8];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0usize);
    let id = get_screen_param(0) as i32;
    let staff = Staff(staff_at(id as isize).addr());
    let slot = get_screen_param(3);
    let mut slot_index: i32 = -1;
    if slot != 0 && read_usize(slot) != 0 {
        slot_index = read_i32(read_usize(slot) + offer::SLOT_INDEX);
    }
    let mut job = get_screen_param(8) as u8 as i8;
    if staff.is_null() {
        tel26_error_box(CS_CPP, 0x56a);
        exe::drop_text(t);
        return;
    }
    let mut edited = get_screen_param(2);
    let club = cs_human_club();
    let terms = cs_human_terms();
    if edited == 0 {
        covered!("tel24_2 offer screen first build");
        let role = get_screen_param(0xd) as u8;
        let mem = exe::crt_operator_new(CS_TERMS_SIZE as u32);
        edited = if mem != 0 { exe::new_staff_contract(mem) } else { 0 };
        let mut first: u32 = 0;
        let mut second: f32 = 0.0;
        exe::staff_contract_set_to_desired_contract(edited, staff, club, slot_index, role as u32, job as u8 as u32,
            &mut first as *mut u32 as u32, &mut second as *mut f32 as u32);
        set_screen_param(0x19, first as usize);
        set_screen_param(0x1a, ftol(second as f64 * read_f64(rdata::F64_1000)) as usize);
        let current = exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(staff.addr()), read_u32(club));
        if current == 0 || matches!(cs_contract_type(current), 5 | 6) {
            if read_u32(terms) != read_u32(edited) || read_u32(terms + 4) != read_u32(edited + 4) {
                cs_copy_terms(terms, edited);
                let prefill: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(CS_FN_PREFILL_OFFER);
                prefill(read_usize(G_CONTRACT_MANAGER), edited);
                let budget = get_screen_param(0x15) as i32;
                if read_i32(terms + CS_TERMS_FEE) > budget {
                    write_i32(terms + CS_TERMS_FEE, budget);
                }
            }
        } else {
            cs_copy_terms(terms, current);
            let budget = get_screen_param(0x15) as i32;
            if read_i32(terms + CS_TERMS_FEE) > budget {
                write_i32(terms + CS_TERMS_FEE, budget);
            }
            set_screen_param(8, read_i8(current + CS_OFFER_JOB) as i32 as usize);
            job = read_i8(current + CS_OFFER_JOB);
        }
        let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
        if c != 0 && read_u32(c + contract::CLUB) == read_u32(terms + 4) {
            set_screen_param(9, exe::date_get_month(c + contract::CONTRACT_EXPIRES) as i8 as i32 as usize);
            set_screen_param(0xa, read_u16(c + contract::CONTRACT_EXPIRES_YEAR) as usize);
        } else {
            let mut before = [0u32; 2];
            let d = exe::date_sub_days(G_CURRENT_DATE, before.as_mut_ptr() as usize, 0x20);
            set_screen_param(9, exe::date_get_month(d) as i8 as i32 as usize);
            let d = exe::date_sub_days(G_CURRENT_DATE, before.as_mut_ptr() as usize, 0x20);
            set_screen_param(0xa, read_u16(d + 2) as usize);
        }
        set_screen_param(2, edited);
        set_screen_param(0xb, exe::date_get_month(edited + contract::CONTRACT_EXPIRES) as i8 as i32 as usize);
        set_screen_param(0xc, read_u16(edited + contract::CONTRACT_EXPIRES_YEAR) as usize);
    }
    let table = get_screen_param(1) as u8 as i8;
    picture(staff.addr(), 0, 0);
    let mut a = read_u16(G_GUI_COLOUR_BLUE) as u32;
    let mut b = read_u16(G_GUI_COLOUR_PALE_GREY) as u32;
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    let own = Club(if c == 0 || cs_contract_type(c) == 5 { 0 } else { club_at(read_i32(c + contract::CLUB) as isize).addr() });
    if !own.is_null() && own.addr() == club && exe::database_is_player_job(staff.job_for_club() as i8) != 0 {
        covered!("tel24_2 offer screen squad popup");
        exe::database_get_club_kit_colours(own, 1, &mut a, &mut b);
        let button = tel26_add_button([0x69, 0xf, 0x78, 0x23, 0x30, b, 0xc, 1, a, 0, 0, 0]);
        let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x30, job_colour(JOB_COLOUR_31AA), button]);
        let squad = own.addr() + club::SQUAD;
        exe::crt_qsort(squad, 0x32, 4, CS_SORT_BY_APPEARANCES);
        exe::crt_qsort(squad, 0x14, 4, CS_SORT_BY_SURNAME);
        for k in 0..20 {
            let p = read_usize(squad + k * 4);
            if p == 0 || p == staff.addr() {
                continue;
            }
            exe::database_format_staff_name(p, TS_TEXT, 0x7d0, 4);
            tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), ts_colour(G_GUI_COLOUR_YELLOW), 1, 1,
                ts_colour(G_GUI_COLOUR_BLACK), TS_TEXT as u32, 0, 3, read_u32(p), popup]);
        }
    } else {
        a = read_u16(G_GUI_COLOUR_BLUE) as u32;
        b = read_u16(G_GUI_COLOUR_PALE_GREY) as u32;
    }
    if club != 0 {
        exe::database_get_club_kit_colours(club, 1, &mut a, &mut b);
    }
    let mut name = [0u8; 2000];
    let n = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(staff, n, 0x7d0, 1);
    let own_club = staff.club_job().addr();
    if own_club != 0 {
        sprintf(TS_TEXT, 0x9a3d8c, n, own_club + club::SHORT_NAME); // "%s (%s)"
        exe::text_set_string(t as usize, TS_TEXT as *const u8);
        let own_club = staff.club_job().addr();
        exe::text_add_link(t as usize, (own_club + club::SHORT_NAME) as *const u8, 0x7d5, own_club);
    } else {
        tel26_copy_text(TS_TEXT as *mut u8, n as *const u8);
        exe::text_set_string(t as usize, TS_TEXT as *const u8);
    }
    let title = tel26_add_area(current_human_display(), [100, 10, 0x316, 0x46, 1, 0, 1, 0, 0x30, b, u32::MAX]);
    let mut row: i32 = 0;
    draw(t as usize, title, 100, 0x316, &mut row, 7, a, 1, 0);
    lang(TS_TEXT, 0x9aff10); // "Offer Contract"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, TS_TEXT, 1);
    let pale = ts_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = ts_colour(G_GUI_COLOUR_GREY);
    let dark = ts_colour(G_GUI_COLOUR_DARK_BLUE);
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && get_screen_param(0x1d) != 0 {
        lang(TS_TEXT, 0x9afed4, STR_EMPTY); // "Rules<...>"
        tel26_add_button([0x2da, 0x55, 0x311, 0x69, 0x30, dark, 0xc, 1, pale, TS_TEXT as u32, 0x31, 0]);
    }
    let mut top: i32 = 0x8c;
    let mut bottom: i32 = 0x221;
    if exe::database_is_player_job(job) != 0 {
        for (k, (label, id)) in [(0x9afeccusize, 1u32), (0x9afec4, 2), (0x9afebc, 3)].into_iter().enumerate() {
            // a MenuItem: the word 0xffff its constructor writes, the text, the font and the id
            let item = tabs.as_mut_ptr() as usize + k * 0xd4;
            write_u16(item, 0xffff);
            lang(item + 4, label); // "Basic", "Bonuses", "Clauses"
            write_u32(item + 0xcc, 0xc);
            write_u32(item + 0xd0, id);
        }
        exe::gui_add_tab_bar(tabs.as_mut_ptr() as usize, 3, table as i32, &mut top, &mut bottom, 1);
    }
    match table {
        1 => {
            let f: unsafe extern "C" fn(usize, usize, usize, i32, i32) = core::mem::transmute(CS_FN_BUILD_BASIC_TERMS_TAB);
            f(terms, edited, staff.addr(), top, bottom);
        }
        2 => {
            let f: unsafe extern "C" fn(usize, usize, usize, i32, i32) = core::mem::transmute(CS_FN_BUILD_BONUSES_TAB);
            f(terms, edited, staff.addr(), top, bottom);
        }
        3 => cs_build_clauses_tab(terms, edited, staff.addr(), top, bottom),
        _ => tel26_error_box(CS_CPP, 0x662),
    }
    exe::menubar_build(4, 0);
    let w2: [u8; 2] = [3, 1];
    let w3: [u8; 3] = [2, 1, 1];
    let three = |bar: u32| {
        lang(TS_TEXT, TS_BACK);
        tel26_area_button([bar, 0, 0, 0x30, grey, 0xc, 3, pale, TS_TEXT as u32, 0x1b, 0]);
        lang(TS_TEXT, 0x98a1a0); // "Cancel"
        tel26_area_button([bar, 1, 0, 0x30, grey, 0xc, 3, pale, TS_TEXT as u32, 2, 0]);
        lang(TS_TEXT, 0x98e6e8); // "Offer"
        2u32
    };
    let (bar, col) = if slot == 0 {
        let open = exe::contract_manager_find_contract_offer(read_usize(G_CONTRACT_MANAGER), read_u32(staff.addr()), read_u32(club));
        if open == 0 || matches!(read_u8(open + contract_offer::STATE), 4 | 1) {
            let bar = tel26_add_area(current_human_display(), [100, 0x22b, 0x316, 0x24e, 2, w2.as_ptr() as u32, 1, 0, 1, b, u32::MAX]);
            lang(TS_TEXT, 0x98a1a0); // "Cancel"
            tel26_area_button([bar, 0, 0, 0x30, grey, 0xc, 3, pale, TS_TEXT as u32, 0x1b, 0]);
            lang(TS_TEXT, 0x98e6e8); // "Offer"
            (bar, 1)
        } else {
            let bar = tel26_add_area(current_human_display(), [100, 0x22b, 0x316, 0x24e, 3, w3.as_ptr() as u32, 1, 0, 1, b, u32::MAX]);
            (bar, three(bar))
        }
    } else {
        let bar = tel26_add_area(current_human_display(), [100, 0x22b, 0x316, 0x24e, 3, w3.as_ptr() as u32, 1, 0, 1, b, u32::MAX]);
        (bar, three(bar))
    };
    tel26_area_button([bar, col, 0, 0x30, grey, 0xc, 3, pale, TS_TEXT as u32, 1, 0]);
    exe::drop_text(t);
}

/// Comparer of `contract_screens_build_offer_contract_screen`: side by side over the display list, the human's and
/// the edited terms, the contract lookup global and the squad the popup sorts; on the first build (it makes the
/// edited terms) and without the person only the original runs.
pub(crate) unsafe extern "C" fn compare_contract_screens_build_offer_contract_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CONTRACT_SCREENS_BUILD_OFFER_CONTRACT_SCREEN);
    let staff = staff_at(get_screen_param(0) as i32 as isize).addr();
    if get_screen_param(2) == 0 || staff == 0 {
        return o();
    }
    let mut extra = cs_offer_regions();
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff));
    if !c.is_null() {
        let own = Club(club_at((c.club() as i32) as isize).addr());
        if !own.is_null() {
            extra.push((own.addr() + club::SQUAD, 0xc8));
        }
    }
    compare_by_display_list_with("Offer contract screen", &extra, || o(), || contract_screens_build_offer_contract_screen());
}

/// Answers the offer contract screen.
///
/// A tab (id 0xc) goes to param 1 (-11); a player of the squad popup (id 3) becomes the person (param 0) and the edited
/// terms are dropped from param 2 without freeing them (-11); the rules button (id 0x31) opens the transfer rules
/// screen (-4). Cancel (id 2): for someone else's transfer offer (param 6), when the offer changed (its serial is not
/// param 5's, or it was rejected or withdrawn) the offer is deleted, the slot cleared and the offer changed screen
/// opened with the deleted offer (-11), else the remove exchange player dialog (-4); with a transfer offer the bid
/// confirm dialog (cancel, 2); otherwise the withdraw dialog (`open_staff_offer_contract_screen(0, 0, 0xb, 1, 0, 3)`).
/// Back (id 0x1b) clears the human's terms (id -1) and answers -10. Offer (id 1): the human's terms expire on the day of
/// their start (at most the 28th) of the month and year of params 0xb/0xc; when the transfer offer changed, the offer
/// changed screen (-11), else the offer dialog (`open_staff_offer_contract_screen(slot, aware, job, 0, param 0x1d,
/// param 0x1c)`; aware is the offer's or param 7) (-4). Anything else goes to the basic terms, bonuses and clauses
/// answers, then the menu bar.
/// Original: `contract_screens_handle_offer_contract_screen` (0x4e01e0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_handle_offer_contract_screen(item: u32) -> i32 {
    let item16 = item as i16;
    let open_staff: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32) = core::mem::transmute(CS_FN_OPEN_STAFF_OFFER);
    let changed: unsafe extern "C" fn(usize, u32) = core::mem::transmute(CS_FN_OPEN_OFFER_CHANGED);
    let id = tel26_item_id(item16);
    if id == 0xc {
        set_screen_param(1, tel26_item_value(item16) as usize);
        return -11;
    }
    if id == 3 {
        set_screen_param(0, tel26_item_value(item16) as usize);
        set_screen_param(2, 0);
        return -11;
    }
    if id == 0x31 {
        let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(CS_FN_RULES_SCREEN);
        f(0, 0);
        return -4;
    }
    let offer_changed = |o: usize, serial: u32| read_u32(o + offer::SERIAL) != serial || matches!(read_u8(o + offer::STATUS), 3 | 2);
    if id == 2 {
        covered!("tel24_2 offer screen Cancel");
        let slot = get_screen_param(3);
        let o = if slot != 0 { read_usize(slot) } else { 0 };
        if get_screen_param(6) != 0 {
            let serial = get_screen_param(5) as u32;
            if o != 0 && offer_changed(o, serial) {
                let live = read_usize(slot);
                if live != 0 {
                    exe::drop_transfer_offer(live);
                    exe::crt_operator_delete(live);
                }
                write_usize(slot, 0);
                // the original passes the offer it has just deleted (a dangling pointer) to the screen
                changed(o, serial);
                return -11;
            }
            let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(CS_FN_REMOVE_EXCHANGE_PLAYER);
            f(slot, read_u32(cs_human_terms()));
            return -4;
        }
        if slot != 0 {
            let f: unsafe extern "C" fn(usize, u32, usize) = core::mem::transmute(CS_FN_BID_CONFIRM);
            f(slot, 2, slot);
            return -4;
        }
        open_staff(0, 0, 0xb, 1, 0, 3);
        return -4;
    }
    if id == 0x1b {
        write_i32(cs_human_terms(), -1);
        return -10;
    }
    if id != 1 {
        for f in [CS_FN_BASIC_TERMS_ACTION, CS_FN_BONUSES_ACTION, CS_FN_CLAUSES_ACTION] {
            let f: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(f);
            let r = f(item);
            if r != 0 {
                return r;
            }
        }
        return ts_menubar_answer(item);
    }
    covered!("tel24_2 offer screen Offer");
    let terms = Contract(cs_human_terms());
    let slot = get_screen_param(3);
    let mut o = 0;
    let aware = if slot == 0 {
        get_screen_param(7) as u8
    } else {
        o = read_usize(slot);
        read_u8(o + offer::PLAYER_AWARE)
    };
    let job = get_screen_param(8) as u8;
    let window_state = get_screen_param(0x1c) as u8;
    let window = get_screen_param(0x1d);
    let month = get_screen_param(0xb);
    let year = get_screen_param(0xc);
    let start = terms.addr() + contract::DATE_STARTED;
    let day = if exe::date_get_day_of_month(start) as i16 > 0x1c { 0x1c } else { exe::date_get_day_of_month(start) as i16 as i32 };
    let mut date = [0u16; 4];
    let d = exe::date_date_549f70(&mut date, day as u32, month as u32, year as u32, u32::MAX);
    let mut out = [0u32; 2];
    exe::date_operator_3d(terms.addr() + contract::CONTRACT_EXPIRES, out.as_mut_ptr() as usize, d);
    let serial = get_screen_param(5) as u32;
    if o != 0 && offer_changed(o, serial) {
        changed(o, serial);
        return -11;
    }
    open_staff(slot, aware as u32, job as u32, 0, window as u32, window_state as u32);
    -4
}

/// Comparer of `contract_screens_handle_offer_contract_screen`: the tab and squad player answers side by side over
/// the screen params; the rest only in the original (they open screens, change the terms or the offer).
pub(crate) unsafe extern "C" fn compare_contract_screens_handle_offer_contract_screen(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_CONTRACT_SCREENS_HANDLE_OFFER_CONTRACT_SCREEN);
    ts_compare_answer("Offer contract screen", item, &[0xc, 3], o, contract_screens_handle_offer_contract_screen)
}

/// Builds the bonuses tab of the offer contract screen.
///
/// One area (columns 2:1:0xe:2:8:2:2:2:6, as many rows as fit between `top` and `bottom`) with "Offer player the
/// following bonuses:-". The most a bonus can be is a tenth of the club's maximum wage (`get_max_wage(person, -1)` of
/// the terms' club). For each bonus type he can have (`is_bonus_type_allowed`; goal, assist, clean sheet): the human's
/// bonus is cut to that maximum (a write to the terms); when he asked for terms and the edited bonus is set and within
/// the maximum, a "*" and a note of what he asked (the amount in mode 10, 9 when long). Without a bonus (negative), a
/// grey button (id 0xd) and texts (the amount shown is a tenth of the wage, or his request); with one, a pale button
/// (0xe), "-" (id 0x10, the item index in param 0x12 + type) from 6 up, the amount with a popup of five steps (a fifth
/// of the maximum each; id 0x2a + type, the amount as value), "+" (id 0xf) below the maximum and the last button (id
/// 0x11), and the request note. "* Requested by the player" on the last row when he asked.
/// Original: `contract_screens_build_bonuses_tab` (0x4e30f0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_build_bonuses_tab(terms: usize, edited: usize, staff: usize, top: i32, bottom: i32) {
    let edited = Contract(edited);
    let terms = Contract(terms);
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let allowed: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(CS_FN_BONUS_ALLOWED);
    let bonus_name: unsafe extern "C" fn(u32, usize) = core::mem::transmute(CS_FN_BONUS_NAME);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let pale = ts_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = ts_colour(G_GUI_COLOUR_GREY);
    let dark = ts_colour(G_GUI_COLOUR_DARK_BLUE);
    let marker = ts_colour(0xaebdd4);
    let w: [u8; 9] = [2, 1, 0xe, 2, 8, 2, 2, 2, 6];
    let mut row: i32 = 0;
    let mut text = [0usize; 8];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0usize);
    let mut cash: u32 = 0;
    let mut popups: [i16; 3] = [-1; 3];
    let asked = cs_will_negotiate(edited.addr());
    let rows = ((bottom - top) / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 9, w.as_ptr() as u32, rows as u32, 0, 2, pale, u32::MAX]);
    lang(TS_TEXT, 0x9b0eb4); // "Offer player the following bonuses:-"
    cs_wrapped(t, area, &mut row, pale);
    row += 2;
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club_at((edited.club() as i32) as isize).addr());
    let most = exe::finance_get_max_wage(fin, staff, u32::MAX) / 10;
    let mut request = [0u8; 0x20];
    let req = request.as_mut_ptr() as usize;
    for k in 0..3u32 {
        if allowed(staff, k) == 0 {
            continue;
        }
        let bonus = terms.addr() + CS_TERMS_BONUSES + k as usize * 4;
        if most < read_i32(bonus) {
            write_i32(bonus, most);
        }
        let wanted = read_i32(edited.addr() + CS_TERMS_BONUSES + k as usize * 4);
        let r = row as u32;
        if wanted < 0 || asked == 0 || most < wanted {
            request[0] = 0;
        } else {
            tel26_area_text([area, 1, r, 1, 0, 0xc, 3, marker, 0x989ad8]); // "*"
            cs_money_text(TS_TEXT, wanted as u32, 10, 9);
            sprintf(req, 0x98fb04, TS_TEXT);
        }
        let v = read_i32(bonus);
        if v < 0 {
            bonus_name(k, TS_TEXT);
            tel26_area_button([area, 2, r, 1, 0, 1, 3, grey, TS_TEXT as u32, 0xd, k]);
            tel26_area_button([area, 3, r, 0x30, dark, 0x2c, 3, pale, 0x98ba0c, 0xd, k]); // "-"
            if request[0] == 0 {
                cs_money_text(TS_TEXT, (terms.wage() / 10) as u32, 10, 9);
            } else {
                tel26_copy_text(TS_TEXT as *mut u8, req as *const u8);
            }
            tel26_area_button([area, 4, r, 1, 0, 0x2c, 3, pale, TS_TEXT as u32, 0xd, k]);
            tel26_area_button([area, 5, r, 0x30, dark, 0x2c, 3, pale, 0x9afe7c, 0xd, k]); // "+"
            tel26_area_button([area, 6, r, 0x30, dark, 0x2c, 3, pale, 0x9afe78, 0xd, k]);
            if request[0] != 0 {
                tel26_area_button([area, 8, r, 1, 0, 0x21, 3, marker, req as u32, 0xd, k]);
            }
        } else {
            covered!("tel24_2 bonuses tab bonus set");
            bonus_name(k, TS_TEXT);
            tel26_area_button([area, 2, r, 1, 0, 1, 3, pale, TS_TEXT as u32, 0xe, k]);
            let minus = if v < 6 {
                tel26_area_text([area, 3, r, 0x30, dark, 0x2c, 3, pale, 0x98ba0c])
            } else {
                tel26_area_button([area, 3, r, 0x30, dark, 0xc, 3, pale, 0x98ba0c, 0x10, k])
            };
            set_screen_param(k + 0x12, minus as i16 as i32 as usize);
            cs_money_text(TS_TEXT, v as u32, 10, 9);
            let item = tel26_area_button([area, 4, r, 1, 0, 0xc, 3, pale, TS_TEXT as u32, 0, 0]);
            popups[k as usize] = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), item]) as i16;
            if v < most {
                tel26_area_button([area, 5, r, 0x30, dark, 0xc, 3, pale, 0x9afe7c, 0xf, k]);
            } else {
                tel26_area_text([area, 5, r, 0x30, dark, 0x2c, 3, pale, 0x9afe7c]);
            }
            tel26_area_button([area, 6, r, 0x30, dark, 0xc, 3, pale, 0x9afe78, 0x11, k]);
            if request[0] != 0 {
                tel26_area_button([area, 8, r, 1, 0, 1, 3, marker, req as u32, 0x11, k]);
            }
        }
        row += 1;
    }
    if asked != 0 {
        row = rows as i8 as i32 - 1;
        lang(TS_TEXT, 0x9afe40); // "* Requested by the player"
        cs_wrapped(t, area, &mut row, marker);
    }
    let mut steps = [0i32; 5];
    let mut acc = most;
    for s in steps.iter_mut() {
        *s = acc / 5;
        acc = acc.wrapping_add(most);
    }
    for (k, &popup) in popups.iter().enumerate() {
        if popup == -1 {
            continue;
        }
        for i in 0..5u32 {
            cs_cash_from_table(&mut cash, i, steps.as_ptr() as *const u32, 5);
            exe::cash_format_money(TS_TEXT, 0x7d0, exe::cash_get_display_amount(&cash, 10), 10);
            tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), ts_colour(G_GUI_COLOUR_YELLOW), 1, 1,
                ts_colour(G_GUI_COLOUR_BLACK), TS_TEXT as u32, 0, 0x2a + k as u32, cash, popup as u16 as u32]);
        }
    }
    exe::drop_text(t);
}

/// Comparer of `contract_screens_build_bonuses_tab`: side by side over the display list, the human's and the edited
/// terms (the bonuses are capped in the terms) and the screen params.
pub(crate) unsafe extern "C" fn compare_contract_screens_build_bonuses_tab(terms: usize, edited: usize, staff: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, i32, i32)>(IDX_CONTRACT_SCREENS_BUILD_BONUSES_TAB);
    let extra = [(terms, CS_TERMS_SIZE), (edited, CS_TERMS_SIZE)];
    compare_by_display_list_with("Offer contract bonuses tab", &extra, || o(terms, edited, staff, top, bottom), || contract_screens_build_bonuses_tab(terms, edited, staff, top, bottom));
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
    let edited = Contract(get_screen_param(2));
    let terms = Contract(cs_human_terms());
    if terms.is_null() {
        cs_fatal(0xecb);
    }
    let id = tel26_item_id(item16) as i16;
    if id > 0x29 && id < 0x2e {
        write_u32((terms.addr() as isize - 0x98 + id as isize * 4) as usize, tel26_item_value(item16));
        return -11;
    }
    let kind = tel26_item_value(item16) as u8 as i8;
    let bonus = terms.addr() + CS_TERMS_BONUSES + kind as isize as usize * 4;
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
        if cs_bonus_cap(edited.addr()) <= v {
            cs_widget(plus, 6, 0x2c, 0, 0);
            exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
            return 0;
        }
        plus_data = kind as i32 as u32;
    } else if id == 0x11 {
        let wanted = read_i32(edited.addr() + CS_TERMS_BONUSES + kind as isize as usize * 4);
        let v = if wanted < 0 { edited.wage() / 10 } else { wanted };
        write_i32(bonus, v);
        let cap = cs_bonus_cap(edited.addr());
        if cap < read_i32(bonus) {
            write_i32(bonus, cap);
        }
        return -11;
    } else if id == 0xd {
        let set: unsafe extern "thiscall" fn(usize, u32, i32) = core::mem::transmute(CS_FN_SET_BONUS);
        let wanted = read_i32(edited.addr() + CS_TERMS_BONUSES + kind as isize as usize * 4);
        set(terms.addr(), kind as i32 as u32, if wanted < 0 { terms.wage() / 10 } else { wanted });
        return -11;
    } else if id == 0xe {
        let remove: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(CS_FN_REMOVE_BONUS);
        remove(terms.addr(), tel26_item_value(item16));
        return -11;
    } else {
        return 0;
    }
    cs_widget(plus, 2, 0xc, 0xf, plus_data);
    exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
    0
}

/// Comparer of `contract_screens_handle_bonuses_action`.
pub(crate) unsafe extern "C" fn compare_contract_screens_handle_bonuses_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_CONTRACT_SCREENS_HANDLE_BONUSES_ACTION);
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
    let edited = Contract(get_screen_param(2));
    let terms = Contract(cs_human_terms());
    if terms.is_null() {
        cs_fatal(0x10a1);
    }
    let fee = terms.addr() + contract::RELEASE_FEE;
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
        add(terms.addr(), clause as u8 as u32);
        return -11;
    }
    if id == 0x13 {
        let remove: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(CS_FN_REMOVE_RELEASE_CLAUSE);
        remove(terms.addr(), tel26_item_value(item16));
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
        let minimum = if terms.minimum_fee_rc() == 1 { get_screen_param(0x17) as i32 } else { 5 };
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
        write_u32(fee, edited.release_fee() as u32);
        if terms.minimum_fee_rc() == 1 && read_i32(fee) < get_screen_param(0x17) as i32 {
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_CONTRACT_SCREENS_HANDLE_CLAUSES_ACTION);
    cs_compare_tab_answer("Offer contract clauses tab", item, &[0x28, 0x27], o, contract_screens_handle_clauses_action)
}

/// Builds the basic terms tab of the offer contract screen: what the person says about the terms, the board's wage
/// limit, and the rows of the wage, signing on fee, dates, contract type, public offer and transfer date.
///
/// Params read: 0x1c the transfer window state, 0x15 the transfer budget, 3 the transfer offer slot, 8 the job, 0xd
/// the role, 0x1d the window mark, 0x18 the wage limit, 0x19 and 0x1a the two out values of the desired contract (the
/// second × 0.001 is the interest, a float), 0x1b the talk flags. The edited terms' club counts only for an id from 1
/// to the club count. One area (columns 2:0xe:2:0xe:2:2:2, as many rows as fit between `top` and `bottom`).
///
/// The person's words, wrapped: he expects terms when he asked for terms (`will_negotiate_offer`), the limit × 1.5 is
/// above the edited wage and either the interest is above 2.75, or his current club (the contract's, not of type 5)
/// is the edited terms' club with the talk flag 2, or the job is 8, 9 or 10 with an edited wage below 3200, or he is
/// under 24 with an edited wage below `cs_young_wage_threshold` + 3250. Then the edited fee (mode 6), and for a
/// month-to-month contract "expects ... weekly wage" (the part time text with the bit 0x80); otherwise the expiry text
/// and, part time, "expects a part time contract"; no fee, "expects a wage of"; an odd person id, "wants"; else
/// "is requesting" (professionalism and loyalty above 10) or "is demanding", all with the fee; the expiry is a link
/// (0xb). Otherwise, at his own club (the current club is the edited one): no contract "does not wish to negotiate",
/// unhappy (grievance 2) with an edited wage below 25000 and a current wage above 2100 "doesn't believe the club can
/// meet his demands", grievances 0x6214 "doesn't want to negotiate ... while he is unhappy", grievance 2 the same
/// doubt for role 1 or the 0x9b09c8 text, else "does not wish". Elsewhere the refusal reasons are built
/// (`build_refusal_reasons_text` with both clubs and managers, -1 for a missing current club); with the screen param
/// 0x16 above the limit × 1.5: interest from 3.15, edited wage below 175000 and not month to month, the expiry text
/// ("expects a wage of", or part time); reasons, the reasons text; interest below 0.75, the club's name text; else
/// "has no interest". Otherwise by the edited squad status 1 to 6: with reasons and interest above 1.25 the status'
/// reasons text, else its "no interest" text; any other status, the staff text for a person with a non-player sheet
/// (else the text buffer is left as it was). The name is a link (0x7da).
///
/// Then the board's limit: a local contract with the limit as wage; "will allow wages up to", or for a player under
/// the club's reputation + 750 his chairman favours "feel he will be a key signing" ("an important member" when he is
/// already at the club; the contract is looked up again into `G_CONTRACT_LOOKUP_RESULT`); the human's wage is cut to
/// the limit; the limit is a link (7). Rows: the wage label by the money option (weekly, monthly, yearly), "-" (id 5;
/// a text up to 100; its index in param 0xf), the wage (popup owner), "+" (id 4) while one step up stays within the
/// limit, "=" (id 6); the signing on fee: "-" (id 0x18, its index in param 0x10) when set, the fee (mode 6, 5 when
/// long), "+" (id 0x17) while one step up stays within the budget, "=" (id 0x16); the start date when it is after
/// today; the expiry (month and year of params 0xb/0xc, or "Month to Month"): "-" (id 0x1d one month after the start,
/// else 0xa; its index in param 0x11) unless the expiry already reaches the request of params 9/0xa or the contract is
/// month to month, "+" (id 0x1d month to month, else 9) unless a fixed contract reaches the maximum length, "=" (id
/// 0xb); the type (part or full time, id 0x1e); "Make offer public" No/Yes (id 0x1a) without a transfer offer and when
/// he is not at the edited terms' club. Popups: eight wage steps (k × limit / 8) when the first is above 100 (id 8,
/// the wage as value), eight fee steps (k × min(budget, 3 million) / 7, id 0x19) when they are more than 1000 apart.
/// With the window mark: a contract with expiry band 0, the next transfer window text (the current one ends / the
/// next opens on its long date); with another band, the move text and the "Transfer date" row (End of contract or
/// Immediate, id 0x30).
/// Original: `contract_screens_build_basic_terms_tab` (0x4e06e0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_build_basic_terms_tab(terms: usize, edited: usize, staff: usize, top: i32, bottom: i32) {
    let edited = Contract(edited);
    let terms = Contract(terms);
    let staff = Staff(staff);
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let draw: unsafe extern "thiscall" fn(usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(TS_FN_TEXT_DRAW_WRAPPED);
    let text_clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(TS_FN_TEXT_CLEAR);
    let pale = ts_colour(G_GUI_COLOUR_PALE_GREY);
    let green = job_colour(JOB_COLOUR_31AA);
    let dark = ts_colour(G_GUI_COLOUR_DARK_BLUE);
    let black = ts_colour(G_GUI_COLOUR_BLACK);
    let yellow = ts_colour(G_GUI_COLOUR_YELLOW);
    let mut row: i32 = 0;
    let widths: [u8; 7] = [2, 0xe, 2, 0xe, 2, 2, 2];
    let mut text = [0usize; 8];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0usize);
    let mut local_terms = [0u8; CS_TERMS_SIZE];
    let local = Contract(local_terms.as_mut_ptr() as usize);
    exe::new_staff_contract(local);
    let mut reasons = [0u8; 400];
    let window_state = get_screen_param(0x1c) as u8;
    let budget = get_screen_param(0x15) as i32;
    let slot = get_screen_param(3);
    let job = get_screen_param(8) as u8 as i8;
    let role = get_screen_param(0xd) as u8 as i8;
    let window = get_screen_param(0x1d);
    let limit = get_screen_param(0x18) as i32 as f64;
    let first = get_screen_param(0x19) as u32;
    let interest = (get_screen_param(0x1a) as i32 as f64 * read_f32(CS_F32_0_001) as f64) as f32;
    let flags = get_screen_param(0x1b) as u32;
    let club_id = edited.club() as i32;
    let club = Club(if club_id > 0 && club_id < read_i32(G_CLUB_COUNT) { club_at(club_id as isize).addr() } else { 0 });
    let rows = ((bottom - top) / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 7, widths.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
    let current = if !c.is_null() && cs_contract_type(c.addr()) != 5 { club_at((c.club() as i32) as isize).addr() } else { 0 };
    let threshold = if !club.is_null() { cs_young_wage_threshold(club.addr(), cs_squad_status(edited.addr())) } else { 0 };
    let mut name = [0u8; 2000];
    let n = name.as_mut_ptr() as usize;
    let mut wage_text = [0u8; 2000];
    let w = wage_text.as_mut_ptr() as usize;
    let mut expiry = [0u8; 200];
    let e = expiry.as_mut_ptr() as usize;
    let wage = edited.wage();
    let expects = cs_will_negotiate(edited.addr()) != 0 && limit * read_f64(rdata::F64_1_5) > wage as f64
        && ((interest as f64) > read_f64(rdata::F64_2_75)
            || (current == club.addr() && flags & 2 != 0)
            || (matches!(job, 8 | 10 | 9) && wage < 0xc80)
            || ((staff.age() as i8) < 0x18 && wage < threshold as i32 + 0xcb2));
    exe::database_format_staff_name(staff, n, 0x7d0, 1);
    cs_format_wage(edited.addr(), w, 2, 2);
    if expects {
        covered!("tel24_4 basic terms he expects terms");
        let mut fee_text = [0u8; 2000];
        let ft = fee_text.as_mut_ptr() as usize;
        let cash = read_u32(edited.addr() + CS_TERMS_FEE);
        exe::cash_format_money(ft, 0x7d0, exe::cash_get_display_amount(&cash, 6), 6);
        let kind = edited.contract_type();
        if kind & 0x3f == 1 {
            // "<name> expects <wage> weekly wage" (a part time variant with the bit 0x80)
            lang(TS_TEXT, if kind & 0x80 != 0 { 0x9b0e4c } else { 0x9b0de4 }, n, w);
        } else {
            cs_format_expiry(edited.addr(), e, 1);
            if kind & 0x80 != 0 {
                lang(TS_TEXT, 0x9b0d58, 100u32, n, w, e); // "expects a part time contract"
            } else if read_i32(edited.addr() + CS_TERMS_FEE) == 0 {
                lang(TS_TEXT, 0x9b0ae8, 100u32, n, w, e); // "expects a wage of"
            } else if read_i32(staff.addr()) % 2 != 0 {
                lang(TS_TEXT, 0x9b0cb8, 100u32, n, w, e, ft); // "wants"
            } else if staff.professionalism() > 10 && staff.loyalty() > 10 {
                lang(TS_TEXT, 0x9b0c08, 100u32, n, w, e, ft); // "is requesting"
            } else {
                lang(TS_TEXT, 0x9b0b58, 100u32, n, w, e, ft); // "is demanding"
            }
            exe::text_add_link(t as usize, e as *const u8, 0xb, 0usize);
        }
        exe::text_set_string(t as usize, TS_TEXT as *const u8);
        exe::text_add_link(t as usize, n as *const u8, 0x7da, staff);
        exe::text_add_link(t as usize, w as *const u8, 6, 0usize);
    } else {
        if current != 0 && current == club.addr() {
            covered!("tel24_4 basic terms at his own club");
            let own = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
            let fmt = if own == 0 {
                0x9b0964 // "does not wish to negotiate a new contract at the current time"
            } else {
                let g = read_u32(own + contract::GRIEVANCES);
                if g & 2 != 0 && wage < 0x61a8 && read_i32(own + contract::WAGE) > 0x834 {
                    0x9b0a94 // "doesn't believe the club can meet his demands"
                } else if g & 0x6214 != 0 {
                    0x9b0a30 // "doesn't want to negotiate a new contract while he is unhappy"
                } else if g & 2 != 0 {
                    if role == 1 { 0x9b0a94 } else { 0x9b09c8 }
                } else {
                    0x9b0964
                }
            };
            lang(TS_TEXT, fmt, n);
        } else {
            covered!("tel24_4 basic terms refusal reasons");
            let refusal: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, f64, usize) -> u8 = core::mem::transmute(CS_FN_REFUSAL_REASONS);
            let (their_club, their_manager) = if current != 0 { (read_u32(current), cs_club_manager_id(current)) } else { (u32::MAX, u32::MAX) };
            let refused = refusal(read_usize(G_TRANSFER_MANAGER), their_club, read_u32(club.addr()), their_manager, cs_club_manager_id(club.addr()), first,
                interest as f64, reasons.as_mut_ptr() as usize) != 0;
            let r = reasons.as_ptr() as usize;
            let asked_for = get_screen_param(0x16) as i32 as f64;
            if limit * read_f64(rdata::F64_1_5) < asked_for {
                let kind = edited.contract_type();
                if !((interest as f64) < read_f64(rdata::F64_3_15)) && wage < 0x2ab98 && kind & 0x3f != 1 {
                    cs_format_expiry(edited.addr(), e, 1);
                    lang(TS_TEXT, if kind & 0x80 != 0 { 0x9b0d58 } else { 0x9b0ae8 }, 100u32, n, w, e);
                    exe::text_add_link(t as usize, e as *const u8, 0xb, 0usize);
                } else if refused {
                    lang(TS_TEXT, 0x9b0844, n, r);
                } else if (interest as f64) < read_f64(rdata::F64_0_75) {
                    let gender = club.gender_name() as i32;
                    lang(TS_TEXT, 0x9b08f8, gender, gender, n, club.addr() + club::NAME);
                } else {
                    lang(TS_TEXT, 0x9b08ac, n); // "has no interest in negotiating terms"
                }
            } else {
                // the squad status' texts: (with reasons, without)
                const BY_STATUS: [(usize, usize); 6] = [(0x9b0844, 0x9b08ac), (0x9b0844, 0x9b08ac), (0x9b07c4, 0x9b075c),
                    (0x9b06e4, 0x9b0684), (0x9b05fc, 0x9b0590), (0x9b0514, 0x9b04b0)];
                let status = cs_squad_status(edited.addr());
                if (1..=6).contains(&status) {
                    let (with, without) = BY_STATUS[status as usize - 1];
                    if refused && (interest as f64) > read_f64(rdata::F64_1_25) {
                        lang(TS_TEXT, with, n, r);
                    } else {
                        lang(TS_TEXT, without, n);
                    }
                } else if staff.non_player().addr() != 0 {
                    lang(TS_TEXT, 0x9b0450, n); // "has no interest in negotiating terms for this position"
                }
            }
        }
        exe::text_set_string(t as usize, TS_TEXT as *const u8);
        exe::text_add_link(t as usize, n as *const u8, 0x7da, staff);
    }
    draw(t as usize, area, 0x78, 0x302, &mut row, 3, pale, 0, 0);
    text_clear(t as usize);
    row += 2;
    // the board's limit
    let limit_int = ftol(limit);
    local.set_wage(limit_int);
    let mut limit_text = [0u8; 200];
    let lt = limit_text.as_mut_ptr() as usize;
    cs_format_wage(local.addr(), lt, 2, 2);
    let player = PlayerSheet(if !staff.is_null() { staff.player().addr() } else { 0 });
    let fmt = if staff.is_null() || club.is_null() || player.is_null()
        || club.reputation() as i32 >= player.current_reputation() as i32 + 0x2ee
        || exe::staff_is_favoured_by(staff, (club.chairman().addr() as u32) as usize) == 0 {
        0x9b0324 // "Your board will allow wages up to"
    } else {
        let again = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff));
        write_usize(G_CONTRACT_LOOKUP_RESULT, again.addr());
        let own = if again.is_null() || cs_contract_type(again.addr()) == 5 { 0 } else { club_at((again.club() as i32) as isize).addr() };
        if own == club.addr() { 0x9b03d8 } else { 0x9b036c } // "an important member" / "a key signing"
    };
    lang(TS_TEXT, fmt, lt);
    if terms.wage() > limit_int {
        terms.set_wage(limit_int);
    }
    exe::text_set_string(t as usize, TS_TEXT as *const u8);
    exe::text_add_link(t as usize, lt as *const u8, 7, limit_int as u32 as usize);
    draw(t as usize, area, 0x78, 0x302, &mut row, 3, pale, 0, 0);
    row += 1;
    tel26_area_text([area, 0, row as u32, 1, 0, 0xc, 3, pale, 0]);
    row += 1;
    // the wage row
    let options = read_u32(read_usize(G_CONFIG) + 4);
    lang(TS_TEXT, if options & 0x200000 != 0 { 0x9b0318 } else if options & 0x400000 != 0 { 0x9b0308 } else { 0x9b02fc });
    tel26_area_button([area, 1, row as u32, 1, green, 1, 3, pale, TS_TEXT as u32, 0, 0]);
    let r = row as u32;
    let minus = if terms.wage() <= 100 {
        tel26_area_text([area, 2, r, 0x30, dark, 0x2c, 3, pale, 0x98ba0c])
    } else {
        tel26_area_button([area, 2, r, 0x30, dark, 0xc, 3, pale, 0x98ba0c, 5, 0])
    };
    set_screen_param(0xf, minus as i16 as i32 as usize);
    cs_format_wage(terms.addr(), w, 0, 2);
    let wage_item = tel26_area_button([area, 3, r, 1, black, 0xc, 3, pale, w as u32, 0, 0]);
    let mut up = terms.wage() as u32;
    exe::cash_step_up(&mut up as *mut u32 as usize, 8);
    let top_cash = limit_int as u32;
    if exe::cash_get_display_amount(&up, 8) > exe::cash_get_display_amount(&top_cash, 8) {
        tel26_area_text([area, 4, r, 0x30, dark, 0x2c, 3, pale, 0x9afe7c]); // "+"
    } else {
        tel26_area_button([area, 4, r, 0x30, dark, 0xc, 3, pale, 0x9afe7c, 4, 0]);
    }
    tel26_area_button([area, 5, r, 0x30, dark, 0xc, 3, pale, 0x9afe78, 6, 0]); // "="
    row += 1;
    // the signing on fee row
    let r = row as u32;
    lang(TS_TEXT, 0x9b02ec); // "Signing On Fee"
    tel26_area_button([area, 1, r, 1, green, 1, 3, pale, TS_TEXT as u32, 0, 0]);
    let fee_minus = if read_i32(terms.addr() + CS_TERMS_FEE) == 0 {
        tel26_area_text([area, 2, r, 0x30, dark, 0x2c, 3, pale, 0x98ba0c])
    } else {
        tel26_area_button([area, 2, r, 0x30, dark, 0xc, 3, pale, 0x98ba0c, 0x18, 0])
    };
    set_screen_param(0x10, fee_minus as i16 as i32 as usize);
    let mut cash = read_u32(terms.addr() + CS_TERMS_FEE);
    cs_money_text(TS_TEXT, cash, 6, 5);
    let fee_item = tel26_area_button([area, 3, r, 1, black, 0xc, 3, pale, TS_TEXT as u32, 0, 0]);
    let mut fee_up = read_u32(terms.addr() + CS_TERMS_FEE);
    if exe::cash_step_up(&mut fee_up as *mut u32 as usize, 4) > budget {
        tel26_area_text([area, 4, r, 0x30, dark, 0x2c, 3, pale, 0x9afe7c]);
    } else {
        tel26_area_button([area, 4, r, 0x30, dark, 0xc, 3, pale, 0x9afe7c, 0x17, 0]);
    }
    tel26_area_button([area, 5, r, 0x30, dark, 0xc, 3, pale, 0x9afe78, 0x16, 0]);
    row += 1;
    // the start date, when it is after today
    let start = terms.addr() + contract::DATE_STARTED;
    let today_year = read_u16(G_CURRENT_DATE + 2);
    let start_year = terms.date_started_year();
    if today_year < start_year || (today_year == start_year && read_i16(G_CURRENT_DATE) < read_i16(start)) {
        let short_date: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(CS_FN_SHORT_DATE);
        lang(TS_TEXT, 0x9b02d8); // "Contract Start Date"
        tel26_area_button([area, 1, row as u32, 1, green, 1, 3, pale, TS_TEXT as u32, 0, 0]);
        short_date(start, TS_TEXT, 0x7d0, 0xff);
        tel26_area_button([area, 3, row as u32, 1, 0, 0xc, 3, pale, TS_TEXT as u32, 0, 0]);
        row += 1;
    }
    // the expiry row
    let r = row as u32;
    lang(TS_TEXT, 0x9b02c4); // "Contract Expires"
    tel26_area_button([area, 1, r, 1, green, 1, 3, pale, TS_TEXT as u32, 0, 0]);
    let asked_month = get_screen_param(9) as u8 as i8;
    let asked_year = get_screen_param(0xa) as u16;
    let month = get_screen_param(0xb) as u8 as i8;
    let year_param = get_screen_param(0xc) as u32;
    let year = year_param & 0xffff;
    let start_year = start_year as u32;
    let start_month = || exe::date_get_month(start) as i8;
    // the expiry one step earlier: a year back while it stays past the first year, else a month back
    let (earlier_month, earlier_year) = if (year == start_year + 1 && month > start_month()) || year > start_year + 1 {
        (month, year_param.wrapping_sub(1) as u16)
    } else if month.wrapping_sub(1) < 0 {
        (11, year_param.wrapping_sub(1) as u16)
    } else {
        (month.wrapping_sub(1), year_param as u16)
    };
    let at_request = if earlier_year == asked_year { earlier_month <= asked_month } else { earlier_year < asked_year };
    let kind = terms.contract_type() & 0x3f;
    let expiry_minus = if at_request || kind == 1 {
        tel26_area_text([area, 2, r, 0x30, dark, 0x2c, 3, pale, 0x98ba0c])
    } else if (year_param as u16 == start_year as u16 && month as i32 == start_month() as i32 + 1)
        || (year == start_year + 1 && month == 0 && start_month() == 11) {
        tel26_area_button([area, 2, r, 0x30, dark, 0xc, 3, pale, 0x98ba0c, 0x1d, 0])
    } else {
        tel26_area_button([area, 2, r, 0x30, dark, 0xc, 3, pale, 0x98ba0c, 0xa, 0])
    };
    set_screen_param(0x11, expiry_minus as i16 as i32 as usize);
    if terms.contract_type() & 0x3f == 1 {
        lang(TS_TEXT, 0x9b02b4); // "Month to Month"
    } else {
        lang(TS_TEXT, 0x9b0288, cs_month_name(month), year); // "<month> <year>"
    }
    tel26_area_button([area, 3, r, 1, 0, 0xc, 3, pale, TS_TEXT as u32, 0, 0]);
    let kind = terms.contract_type() & 0x3f;
    if kind == 2 && year as i32 - read_i8(G_MAX_CONTRACT_LENGTH_YEARS) as i32 >= read_u16(G_CURRENT_DATE_YEAR) as i32 {
        tel26_area_text([area, 4, r, 0x30, dark, 0x2c, 3, pale, 0x9afe7c]);
    } else {
        tel26_area_button([area, 4, r, 0x30, dark, 0xc, 3, pale, 0x9afe7c, if kind == 1 { 0x1d } else { 9 }, 0]);
    }
    tel26_area_button([area, 5, r, 0x30, dark, 0xc, 3, pale, 0x9afe78, 0xb, 0]);
    row += 1;
    // the contract type row
    let r = row as u32;
    lang(TS_TEXT, 0x9b0278); // "Contract Type"
    tel26_area_text([area, 1, r, 1, 0, 1, 3, pale, TS_TEXT as u32]);
    if terms.contract_type() & 0x80 != 0 {
        lang(TS_TEXT, 0x9b026c); // "Part Time"
    } else {
        lang(TS_TEXT, 0x9b0244, STR_EMPTY); // "Full Time"
    }
    tel26_area_button([area, 3, r, 1, green, 0xc, 3, pale, TS_TEXT as u32, 0x1e, 0]);
    row += 1;
    // the public offer row
    let at_club = || {
        let job_club = staff.club_job().addr();
        job_club != 0 && read_i32(job_club) == club_id
    };
    if !at_club() && slot == 0 {
        let r = row as u32;
        lang(TS_TEXT, 0x9b0230); // "Make offer public"
        tel26_area_button([area, 1, r, 1, 0, 1, 3, pale, TS_TEXT as u32, 0x1a, 0]);
        if get_screen_param(7) as u8 == 0 {
            lang(TS_TEXT, TS_NO);
            tel26_area_button([area, 3, r, 1, 0, 0xc, 3, pale, TS_TEXT as u32, 0x1a, 1]);
        } else {
            lang(TS_TEXT, TS_YES);
            tel26_area_button([area, 3, r, 1, 0, 0xc, 3, pale, TS_TEXT as u32, 0x1a, 0]);
        }
    }
    // the wage popup: eight steps of an eighth of the limit
    let mut wage_steps = [0i32; 8];
    for k in 1..=8usize {
        wage_steps[k - 1] = ftol(k as f64 * limit * read_f64(rdata::F64_0_125));
    }
    if wage_steps[0] > 100 {
        covered!("tel24_4 basic terms wage popup");
        let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, green, wage_item]);
        for i in 0..8u32 {
            let v = cs_cash_step_from(&mut cash, i, wage_steps.as_ptr(), 8);
            local.set_wage((v) as i32);
            cs_format_wage(local.addr(), TS_TEXT, 0, 2);
            tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, green, yellow, 1, 1, black, TS_TEXT as u32, 0, 8,
                (local.wage() as u32), popup]);
        }
    }
    // the fee popup: eight steps of a seventh of the budget (at most 3 million)
    if fee_item as i16 != -1 {
        let cap = if budget < 3_000_000 { budget } else { 3_000_000 };
        let mut fee_steps = [0i32; 8];
        for (k, s) in fee_steps.iter_mut().enumerate() {
            *s = (k as i32).wrapping_mul(cap) / 7;
        }
        if fee_steps[1].wrapping_sub(fee_steps[0]) > 1000 {
            let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, green, fee_item]);
            if popup as i16 != -1 {
                covered!("tel24_4 basic terms fee popup");
                for i in 0..8u32 {
                    cs_cash_step_from(&mut cash, i, fee_steps.as_ptr(), 8);
                    exe::cash_format_money(TS_TEXT, 0x7d0, exe::cash_get_display_amount(&cash, 5), 5);
                    tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, green, yellow, 1, 1, black, TS_TEXT as u32, 0, 0x19,
                        cash, popup]);
                }
            }
        }
    }
    row += 2;
    if window != 0 {
        let contract_of = || exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff);
        if contract_of() != 0 && exe::staff_contract_get_expiry_band(contract_of()) == 0 {
            covered!("tel24_4 basic terms next window");
            let mut opens = [0u32; 2];
            let mut ends = [0u32; 2];
            exe::date_date_54a110(opens.as_mut_ptr() as usize);
            exe::date_date_54a110(ends.as_mut_ptr() as usize);
            lang(TS_TEXT, if at_club() { 0x9b01c4 } else { 0x9b00f4 });
            exe::text_set_string(t as usize, TS_TEXT as *const u8);
            exe::transfer_manager_get_next_window_dates(&mut opens, ends.as_mut_ptr() as usize);
            let mut long_date = [0u8; 200];
            let ld = long_date.as_mut_ptr() as usize;
            if read_u16(G_CURRENT_DATE + 2) == (opens[0] >> 16) as u16 && read_u16(G_CURRENT_DATE) == opens[0] as u16 {
                exe::date_get_long_date_string(ends.as_ptr() as usize, ld, 0xc8, 0xff);
                lang(TS_TEXT, 0x9b00c0, ld); // "The current transfer window ends on"
            } else {
                exe::date_get_long_date_string(opens.as_ptr() as usize, ld, 0xc8, 0xff);
                lang(TS_TEXT, 0x9b0090, ld); // "The next transfer window opens on"
            }
            exe::text_append_string(t as usize, TS_TEXT);
            draw(t as usize, area, 0x78, 0x302, &mut row, 3, pale, 0, 0);
        } else if contract_of() != 0 && exe::staff_contract_get_expiry_band(contract_of()) != 0 {
            covered!("tel24_4 basic terms transfer date");
            lang(TS_TEXT, if at_club() { 0x9b0028 } else { 0x9aff4c });
            exe::text_set_string(t as usize, TS_TEXT as *const u8);
            draw(t as usize, area, 0x78, 0x302, &mut row, 3, pale, 0, 0);
            row += 2;
            lang(TS_TEXT, 0x9aff3c); // "Transfer date"
            tel26_area_text([area, 1, row as u32, 1, 0, 1, 3, pale, TS_TEXT as u32]);
            if window_state == 3 {
                lang(TS_TEXT, 0x9aff2c); // "End of contract"
                tel26_area_button([area, 3, row as u32, 1, 0, 0xc, 3, pale, TS_TEXT as u32, 0x30, 0]);
            } else {
                lang(TS_TEXT, 0x9aff20); // "Immediate"
                tel26_area_button([area, 3, row as u32, 1, 0, 0xc, 3, pale, TS_TEXT as u32, 0x30, 3]);
            }
            row += 1;
        }
    }
    let _ = row;
    exe::drop_text(t);
}

/// Comparer of `contract_screens_build_basic_terms_tab`: side by side over the display list, the human's terms (the
/// wage is cut to the limit), the edited terms and the contract lookup global.
pub(crate) unsafe extern "C" fn compare_contract_screens_build_basic_terms_tab(terms: usize, edited: usize, staff: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, i32, i32)>(IDX_CONTRACT_SCREENS_BUILD_BASIC_TERMS_TAB);
    let extra = [(terms, CS_TERMS_SIZE), (edited, CS_TERMS_SIZE), (G_CONTRACT_LOOKUP_RESULT, 4)];
    compare_by_display_list_with("Offer contract basic terms tab", &extra, || o(terms, edited, staff, top, bottom),
        || contract_screens_build_basic_terms_tab(terms, edited, staff, top, bottom));
}

/// Answers the basic terms tab of the offer contract screen (0 when the item is not one of its own).
///
/// Without the human's terms: the fatal box (line 0xb34). The wage: "+" (id 4) reads param 8 (unused) and steps the
/// wage up (`increase_wage_step`), "-" (5) steps it down (`decrease_wage_step`); both reset "-" (param 0xf: enabled
/// id 5, or after "-" disabled up to a wage of 100), write the wage text and set "+" (enabled id 4 while one more step
/// stays within the limit of param 0x18, else disabled), then redraw and answer 0; "=" (6) goes back to the edited
/// wage and a popup step (7, 8) becomes the wage (-11). The signing on fee: "+" (0x17) and "-" (0x18) step it (mode 4)
/// and redraw: "-" (param 0x10) enabled while the fee is set, the fee text (size 200, mode 6, 5 when long), "+"
/// enabled while one more step stays within the budget of param 0x15; "=" (0x16) and a popup step (0x19) set it, cut
/// to the budget (-11). The "+"/"-" neighbours of the wage and the fee get `set_widget_id` where `set_widget_flags`
/// was meant (`cs_widget_state_by_id`). The expiry: "+" (9) makes the contract fixed (type 2) and, unless it was month
/// to month, moves params 0xb/0xc a year on past the first year (from the requested month of param 9) or a month on;
/// "-" (0xa) moves them a year back past next year (or a month back); "-" of the first month (0x1d) turns a month to
/// month contract into a fixed one or a fixed one into month to month; each rewrites the expiry widgets (param 0x11)
/// like the builder and redraws; "=" (0xb) takes the edited expiry and type (-11). The public offer (0x1a) and the
/// transfer date (0x30) go to params 7 and 0x1c (-11). The job (0x1c): a player swaps 11 and 15; anyone else takes the
/// next job up to 10 he can take, and param 0x18 gets its wage limit (the club's maximum wage for the player jobs,
/// else `cs_staff_wage_limit`) (-11). Part time (0x1e) flips the bit 0x80 of the type (-11). The squad status (0x1f)
/// moves down (2→1, 3→2, 4→3, 5→4; 1→5 under 24, else 4), the edited terms are made again
/// (`set_to_desired_contract` with job 11) and params 0x19/0x1a get its two out values (the second × 1000) (-11).
/// Original: `contract_screens_handle_basic_terms_action` (0x4e2280, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_handle_basic_terms_action(item: u32) -> i32 {
    let item16 = item as i16;
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let terms = Contract(cs_human_terms());
    if terms.is_null() {
        cs_fatal(0xb34);
    }
    let staff = Staff(staff_at(get_screen_param(0) as isize).addr());
    let club = cs_human_club();
    let edited = Contract(get_screen_param(2));
    let budget = get_screen_param(0x15) as i32;
    let mut fee = read_i32(terms.addr() + CS_TERMS_FEE);
    let slot = get_screen_param(3);
    let mut slot_index: i32 = -1;
    if slot != 0 && read_usize(slot) != 0 {
        slot_index = read_i32(read_usize(slot) + offer::SLOT_INDEX);
    }
    let asked_month = get_screen_param(9) as u8 as i8;
    let asked_year = get_screen_param(0xa) as u16;
    let mut month = get_screen_param(0xb) as u8 as i8;
    let mut year = get_screen_param(0xc) as u32;
    let redraw = || {
        exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
        0
    };
    let expiry_text = |month: i8, year: u32| {
        lang(TS_TEXT, 0x9b0288, cs_month_name(month), year & 0xffff);
    };
    let fixed_at_maximum = |year: u32| (year & 0xffff) as i32 - read_i8(G_MAX_CONTRACT_LENGTH_YEARS) as i32 >= read_u16(G_CURRENT_DATE_YEAR) as i32;
    match tel26_item_id(item16) {
        4 => {
            covered!("tel24_4 basic terms wage up");
            get_screen_param(8);
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(CS_FN_INCREASE_WAGE_STEP);
            f(terms.addr());
            let limit = get_screen_param(0x18) as u32;
            let minus = get_screen_param(0xf) as u32;
            cs_widget_state(minus, 2, 0xc, 5);
            let mut up = terms.wage() as u32;
            cs_format_wage(terms.addr(), TS_TEXT, 0, 2);
            exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
            exe::cash_step_up(&mut up as *mut u32 as usize, 8);
            if exe::cash_get_display_amount(&up, 8) > exe::cash_get_display_amount(&limit, 8) {
                cs_widget_state_by_id(minus + 2, 6, 0x2c, 0);
            } else {
                cs_widget_state_by_id(minus + 2, 2, 0xc, 4);
            }
            redraw()
        }
        5 => {
            covered!("tel24_4 basic terms wage down");
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(CS_FN_DECREASE_WAGE_STEP);
            f(terms.addr());
            let minus = get_screen_param(0xf) as u32;
            if terms.wage() <= 100 {
                cs_widget_state_by_id(minus, 6, 0x2c, 0);
            } else {
                cs_widget_state_by_id(minus, 2, 0xc, 5);
            }
            cs_format_wage(terms.addr(), TS_TEXT, 0, 2);
            exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
            cs_widget_state(minus + 2, 2, 0xc, 4);
            redraw()
        }
        6 => {
            terms.set_wage(edited.wage());
            -11
        }
        7 | 8 => {
            terms.set_wage((tel26_item_value(item16)) as i32);
            -11
        }
        9 => {
            covered!("tel24_4 basic terms expiry up");
            let kind = terms.contract_type();
            terms.set_contract_type(kind & 0xc2 | 2);
            if kind & 0x3f != 1 {
                let next_year = read_u16(G_CURRENT_DATE_YEAR) as u32 + 1;
                let y = year & 0xffff;
                if (y == next_year && month >= asked_month) || y > next_year {
                    year = year.wrapping_add(1);
                } else {
                    month = month.wrapping_add(1);
                    if month > 11 {
                        year = year.wrapping_add(1);
                        month = 0;
                    }
                }
                set_screen_param(0xb, month as i32 as usize);
                set_screen_param(0xc, (year & 0xffff) as usize);
            }
            let minus = get_screen_param(0x11) as u32;
            cs_widget_state(minus, 2, 0xc, 0xa);
            expiry_text(month, year);
            exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
            if fixed_at_maximum(year) {
                cs_widget_state(minus + 2, 6, 0x2c, 0);
            } else {
                cs_widget_state(minus + 2, 2, 0xc, 9);
            }
            redraw()
        }
        0xa => {
            covered!("tel24_4 basic terms expiry down");
            let next_year = read_u16(G_CURRENT_DATE_YEAR) as u32 + 1;
            let y = year & 0xffff;
            if (y == next_year && month > exe::date_get_month(G_CURRENT_DATE) as i8) || y > next_year {
                year = year.wrapping_add(0xffff);
            } else {
                month = month.wrapping_sub(1);
                if month < 0 {
                    year = year.wrapping_add(0xffff);
                    month = 11;
                }
            }
            set_screen_param(0xb, month as i32 as usize);
            let shown_year = year & 0xffff;
            set_screen_param(0xc, shown_year as usize);
            let minus = get_screen_param(0x11) as u32;
            let start = terms.addr() + contract::DATE_STARTED;
            let start_year = terms.date_started_year() as u32;
            let start_month = || exe::date_get_month(start) as i8;
            let (earlier_month, earlier_year) = if (shown_year == start_year + 1 && month > start_month()) || shown_year > start_year + 1 {
                (month, year.wrapping_sub(1) as u16)
            } else if month.wrapping_sub(1) < 0 {
                (11, year.wrapping_sub(1) as u16)
            } else {
                (month.wrapping_sub(1), year as u16)
            };
            let at_request = if earlier_year == asked_year { earlier_month <= asked_month } else { earlier_year < asked_year };
            if at_request {
                cs_widget_state(minus, 6, 0x2c, 0);
            } else if (year as u16 == start_year as u16 && month as i32 == start_month() as i32 + 1)
                || (shown_year == start_year + 1 && month == 0 && start_month() == 11) {
                cs_widget_state(minus, 2, 0xc, 0x1d);
            } else if terms.contract_type() & 0x3f == 1 {
                cs_widget_state(minus, 6, 0x2c, 0);
            } else {
                cs_widget_state(minus, 2, 0xc, 0xa);
            }
            expiry_text(month, shown_year);
            exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
            cs_widget_state(minus + 2, 2, 0xc, 9);
            redraw()
        }
        0xb => {
            set_screen_param(0xb, exe::date_get_month(edited.addr() + contract::CONTRACT_EXPIRES) as i8 as i32 as usize);
            set_screen_param(0xc, (edited.contract_expires_year() as u16) as usize);
            let kind = terms.contract_type();
            terms.set_contract_type(kind & 0xc0 | edited.contract_type() & 0x3f);
            -11
        }
        0x16 => {
            let wanted = read_i32(edited.addr() + CS_TERMS_FEE);
            write_i32(terms.addr() + CS_TERMS_FEE, wanted);
            if wanted > budget {
                write_i32(terms.addr() + CS_TERMS_FEE, budget);
            }
            -11
        }
        0x17 => {
            covered!("tel24_4 basic terms fee up");
            exe::cash_step_up(&mut fee as *mut i32 as usize, 4);
            write_i32(terms.addr() + CS_TERMS_FEE, fee);
            let minus = get_screen_param(0x10) as u32;
            cs_widget_state(minus, 2, 0xc, 0x18);
            let mut cash = read_u32(terms.addr() + CS_TERMS_FEE);
            cs_money_text_200(cash, 6, 5);
            exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
            if exe::cash_step_up(&mut cash as *mut u32 as usize, 4) > budget {
                cs_widget_state_by_id(minus + 2, 6, 0x2c, 0);
            } else {
                cs_widget_state_by_id(minus + 2, 2, 0xc, 0x17);
            }
            redraw()
        }
        0x18 => {
            covered!("tel24_4 basic terms fee down");
            let f: unsafe extern "thiscall" fn(*mut i32, u32) -> u32 = core::mem::transmute(CS_FN_CASH_STEP_DOWN);
            f(&mut fee, 4);
            write_i32(terms.addr() + CS_TERMS_FEE, fee);
            let minus = get_screen_param(0x10) as u32;
            if read_i32(terms.addr() + CS_TERMS_FEE) == 0 {
                cs_widget_state_by_id(minus, 6, 0x2c, 0);
            } else {
                cs_widget_state_by_id(minus, 2, 0xc, 0x18);
            }
            cs_money_text_200(read_u32(terms.addr() + CS_TERMS_FEE), 6, 5);
            exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
            cs_widget_state(minus + 2, 2, 0xc, 0x17);
            redraw()
        }
        0x19 => {
            let v = tel26_item_value(item16) as i32;
            write_i32(terms.addr() + CS_TERMS_FEE, v);
            if v > budget {
                write_i32(terms.addr() + CS_TERMS_FEE, budget);
            }
            -11
        }
        0x1a => {
            set_screen_param(7, tel26_item_value(item16) as usize);
            -11
        }
        0x1c => {
            covered!("tel24_4 basic terms job");
            let mut job = get_screen_param(8) as u8;
            if staff.player().addr() != 0 {
                job = if job != 0xb { 0xb } else { 0xf };
            } else {
                loop {
                    job = job.wrapping_add(1);
                    if job == 0xb {
                        job = 0;
                    }
                    if cs_can_take_job(job as u32, staff.addr()) {
                        break;
                    }
                }
                let limit = if cs_player_wage_job(job as i8) {
                    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
                    exe::finance_get_max_wage(fin, staff, u32::MAX) as f64
                } else {
                    cs_staff_wage_limit(club, staff.addr(), job as u32)
                };
                set_screen_param(0x18, ftol(limit) as usize);
            }
            set_screen_param(8, job as i8 as i32 as usize);
            -11
        }
        0x1d => {
            covered!("tel24_4 basic terms first month");
            let minus = get_screen_param(0x11) as u32;
            let kind = terms.contract_type();
            if kind & 0x3f == 1 {
                terms.set_contract_type(kind & 0xc2 | 2);
                cs_widget_state(minus, 2, 0xc, 0xa);
                expiry_text(month, year);
                exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
                if fixed_at_maximum(year) {
                    cs_widget_state(minus + 2, 6, 0x2c, 0);
                    return redraw();
                }
            } else {
                terms.set_contract_type(kind & 0xc1 | 1);
                cs_widget_state(minus, 3, 0x2c, 0);
                lang(TS_TEXT, 0x9b02b4); // "Month to Month"
                exe::screenmanager_set_widget_text(G_SCREEN_MANAGER, minus + 1, TS_TEXT);
            }
            cs_widget_state(minus + 2, 2, 0xc, 9);
            redraw()
        }
        0x1e => {
            let kind = terms.contract_type();
            terms.set_contract_type((if kind & 0x80 == 0 { 0x80 } else { 0 }) | kind & 0x7f);
            -11
        }
        0x1f => {
            covered!("tel24_4 basic terms squad status");
            let b = terms.squad_status();
            let status = b >> 4;
            let next = match status {
                1 => Some(if (staff.age() as i8) >= 0x18 { 4 } else { 5 }),
                2 => Some(1),
                3 => Some(2),
                4 => Some(3),
                5 => Some(4),
                _ => None,
            };
            if let Some(s) = next {
                terms.set_squad_status(b & 0xf | s << 4);
            }
            // the status argument: the new status in the low byte over what the register held (the person for
            // status 1, status - 1 otherwise); the callee reads the low byte
            let high = (if status == 1 { staff.addr() as u32 } else { (status as i32 - 1) as u32 }) & 0xffff_ff00;
            let mut first: u32 = 0;
            let mut second: f32 = 0.0;
            exe::staff_contract_set_to_desired_contract(edited, staff, club, slot_index, high | (terms.squad_status() >> 4) as u32,
                0xb, &mut first as *mut u32 as u32, &mut second as *mut f32 as u32);
            set_screen_param(0x19, first as usize);
            set_screen_param(0x1a, ftol(second as f64 * read_f64(rdata::F64_1000)) as usize);
            -11
        }
        0x30 => {
            set_screen_param(0x1c, tel26_item_value(item16) as usize);
            -11
        }
        _ => 0,
    }
}

/// Comparer of `contract_screens_handle_basic_terms_action`: the answers that only change the human's terms or the
/// screen params (`CS_BASIC_PARAM_ANSWERS`) side by side over the screen params, the human's and the edited terms and
/// the contract lookup global; the others (they change widgets and redraw, or remake the edited terms with a draw)
/// and the missing terms (the fatal box) only in the original.
pub(crate) unsafe extern "C" fn compare_contract_screens_handle_basic_terms_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_CONTRACT_SCREENS_HANDLE_BASIC_TERMS_ACTION);
    let terms = cs_human_terms();
    if terms == 0 || !CS_BASIC_PARAM_ANSWERS.contains(&tel26_item_id(item as i16)) {
        // simplification: the widget changes, the redraw and the desired contract's draw do not run twice
        return o(item);
    }
    let mut regions = screen_param_regions();
    regions.push((terms, CS_TERMS_SIZE));
    regions.push((G_CONTRACT_LOOKUP_RESULT, 4));
    let edited = get_screen_param(2);
    if edited != 0 {
        regions.push((edited, CS_TERMS_SIZE));
    }
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(item), || contract_screens_handle_basic_terms_action(item));
    SCOREBOARD.record(equal && ro == rr, || format!("Offer contract basic terms table answer to item {}: original={} rust={} differences at {:?}", item as i16, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Rule: answering a player's transfer request from the news opens the "Transfer Request" screen with the manager,
/// the news id, the club and the player.
///
/// `contract_screens_news_transfer_request_respond_action(manager, id)`: the news item `id` of the manager
/// (`News::find_item_by_id` into a local `NewsItem`) gives slot 0 (the club) and slot 2 (the player); the screen
/// (builder 0x4dcb30, handler 0x4dcd70, single instance) opens and, when it does, takes parameters 0 = manager, 1 = id,
/// 2 = the club, 3 = the player.
/// Original: `contract_screens_news_transfer_request_respond_action` (0x4dca40, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_news_transfer_request_respond_action(manager: usize, id: u32) {
    let mut item = [0u8; 0xe0];
    let p = item.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0, 0);
    exe::news_find_item_by_id(read_usize(G_NEWS), manager, id, p);
    let club = exe::newsitem_get_slot(p, 0);
    let player = exe::newsitem_get_slot(p, 2);
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x4dcb30usize, 0x4dcd70usize, 1, 0usize, 0) != 0 {
        covered!("tel31_3 transfer request opened");
        set_screen_param(0, manager);
        set_screen_param(1, id as usize);
        set_screen_param(2, club as usize);
        set_screen_param(3, player as usize);
    }
}

/// Rule: the transfer request screen asks "Put <player> on the transfer list at his own request ?" with Back, No and
/// Yes.
///
/// `contract_screens_build_transfer_request_screen()`: the player (parameter 3) and the club (parameter 2); the
/// player's background picture, the menu bar (4, 0), the title (the player's name, "<%s - player>", in the club's kit
/// colours), the subtitle "Transfer Request", the question as a text area (0x6e, 0x91, 0x30c, 0x217, style 2, font 4,
/// pale grey) and a three-column area (weights 2/1/1) with "Back" (0x22), "No" (0x21) and "Yes" (0x20).
/// Original: `contract_screens_build_transfer_request_screen` (0x4dcb30, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_build_transfer_request_screen() {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let weights: [u8; 3] = [2, 1, 1];
    let (mut title, mut subtitle, mut question) = ([0u8; 0x7d0], [0u8; 0x7d0], [0u8; 0x7d0]);
    let club = club_at(get_screen_param(2) as isize).0;
    let player = staff_at(get_screen_param(3) as isize).0;
    let picture: StaffPictureFn = core::mem::transmute(FN_BACKGROUND_SET_STAFF_PICTURE);
    picture(player, 0, 0);
    exe::menubar_build(4, 0);
    let (mut first, mut second) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut first, &mut second);
    exe::database_format_staff_name(player, JOB_TEXT, 0x7d0, 1);
    text(title.as_mut_ptr() as usize, 0x9afc6c, JOB_TEXT);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, second, first, title.as_ptr() as usize);
    text(subtitle.as_mut_ptr() as usize, 0x9afc58);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, subtitle.as_ptr() as usize, 1);
    text(question.as_mut_ptr() as usize, 0x9afc0c, JOB_TEXT);
    tel26_text_area([0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, job_colour(JOB_COLOUR_3184), question.as_ptr() as u32, 0]);
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 3, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    for (column, label, id) in [(0u32, 0x9880a8usize, REQUEST_BACK), (1, 0x98e8cc, REQUEST_NO), (2, 0x98e8d0, REQUEST_YES)] {
        text(JOB_TEXT, label);
        tel26_area_button([area, column, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, id as u32, 0]);
    }
}

/// Rule: Back closes; No refuses the transfer request (the news marked answered, the player may withdraw it, his
/// contract morale moves by his loyalty and his morale drops 12); Yes lists him (the news marked, off the loan list,
/// transfer listed at his request, morale up 12, onto the clubs' shortlists, the listing news).
///
/// `contract_screens_handle_transfer_request_screen(item)`: the widget's id (or the default command). 0x22: -9.
/// 0x21 and 0x20 need the player's contract with the club (`CONTRACT_MANAGER::get_contract_for_club` of parameter 3
/// and parameter 2; none: -9) and write news slots 7 and 8 of item parameter 1 of the manager (parameter 0)
/// (`News::set_item_slot`). No: slot 7 = 1, slot 8 = 0, `request_refused_answer`, then the contract morale + the
/// byte (loyalty × 5 + 0x38) of the player (wrapping as a byte; the sum clamped to -100..100) and
/// `staff_adjust_morale(player, -12, 0)`. Yes: slots 7 and 8 = 1; when the parameter-0 person is loan listed
/// (`staff_is_loan_listed`) he leaves the loan list (`shortlist_manager_remove_from_loan_list`, the original passes
/// the manager, not the player); the contract's transfer status loses bit 2 and gets bit 8; morale +12; the player
/// goes onto the clubs' shortlists (1); `CONTRACT_MANAGER::post_transfer_listed_news`. Both return -9. Other ids: the
/// menu bar (`menubar_handle_command`, then `menubar_handle_object_link`).
/// Original: `contract_screens_handle_transfer_request_screen` (0x4dcd70, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_handle_transfer_request_screen(item: u32) -> i32 {
    let id = tel26_item_id(item as i16);
    if id != REQUEST_BACK && id != REQUEST_NO && id != REQUEST_YES {
        return manager_menu_bar(item);
    }
    if id == REQUEST_BACK {
        return -9;
    }
    let manager = get_screen_param(0);
    let news_id = get_screen_param(1) as u32;
    let club_id = get_screen_param(2);
    let player_id = get_screen_param(3);
    let player = Staff(staff_at(player_id as isize).0);
    let club = club_at(club_id as isize).0;
    let contract = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club));
    if contract.is_null() {
        return -9;
    }
    let slot: NewsSlotFn = core::mem::transmute(FN_NEWS_SET_ITEM_SLOT);
    if id == REQUEST_NO {
        covered!("tel31_3 request refused");
        slot(read_usize(G_NEWS), manager, news_id, 7, 1, 0);
        slot(read_usize(G_NEWS), manager, news_id, 8, 0, 0);
        request_refused_answer(manager, contract.addr());
        let delta = ((player.loyalty() as u8).wrapping_mul(5)).wrapping_add(0x38) as i8;
        let morale = contract.morale();
        let sum = morale as i32 + delta as i32;
        let new = if sum > 100 { 100 } else if sum < -100 { -100 } else { morale.wrapping_add(delta) };
        contract.set_morale(new);
        exe::staff_adjust_morale(staff_at(player_id as isize).0, (-12i32) as u32, 0);
    } else {
        covered!("tel31_3 request accepted");
        slot(read_usize(G_NEWS), manager, news_id, 7, 1, 0);
        slot(read_usize(G_NEWS), manager, news_id, 8, 1, 0);
        if exe::staff_is_loan_listed(manager, 0usize) != 0 {
            exe::shortlist_manager_remove_from_loan_list(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER), manager);
        }
        let status = contract.transfer_status();
        if status & 2 != 0 {
            contract.set_transfer_status(status & 0xfd);
        }
        contract.set_transfer_status(contract.transfer_status() | 8);
        exe::staff_adjust_morale(staff_at(player_id as isize).0, 0xc, 0);
        exe::shortlist_manager_add_player_to_club_shortlists(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER),
            staff_at(player_id as isize).0, 1);
        exe::contract_manager_post_transfer_listed_news(read_usize(G_CONTRACT_MANAGER), staff_at(player_id as isize).0, club_at(club_id as isize).0);
    }
    -9
}

/// Rule: sending a contract offer to a member of staff: with a transfer offer behind the screen, an offer that changed
/// (another serial, rejected or withdrawn) is dropped and the "offer changed" screen opens; otherwise, for a person of
/// another club, a transfer offer is first made (with the compensation for a player, or his contract end date); then
/// the contract offer goes in and the screen closes.
///
/// `contract_screens_handle_staff_offer_contract_screen(item)`: Back (0x15): -9; other ids but Offer (0x14): the menu
/// bar. Offer: the human manager's contract offer (`HUMAN_MANAGER::get_contract_offer`), its person, the slot's
/// manager; parameter 5 set: `CONTRACT_MANAGER::remove_contract_offer(person, manager's club)` and -10. With
/// parameter 0 (the transfer offer holder): the contract's happiness from its squad status (2: 75, 3: 50, 4 and 5:
/// 0); an offer whose serial is not parameter 2 or whose status is 3 or 2 is freed (`~TRANSFER_OFFER`, delete), the
/// holder emptied, `transfer_screens_open_offer_changed_screen(the old offer, parameter 2)` and -11; otherwise the
/// contract offer goes in with parameters 1 and 2. Without it, a person of another club than the manager's gets a
/// transfer offer (TRANSFER_OFFER(person, manager's club, 0, -1, 1), job = parameter 4; a player with a contract:
/// with parameter 7 and not parameter 6 the compensation fee and completion kind 5, otherwise, outside a type-1
/// contract, kind 3 and the contract's end date; an unsignable player (`TRANSFER_MANAGER::can_sign_player` for jobs
/// 11, 13, 15, outside kind 5) kind 1 or 4; the player told = parameter 3) added to the transfer manager, whose slot
/// and serial go with the contract offer. The contract offer: `add_contract_offer(terms, slot, serial, parameter 4,
/// 0, 1, 0)`; the holder's offer for this person gets its terms mark (or the matching exchange player's mark); the
/// terms' person becomes -1; -10.
/// Original: `contract_screens_handle_staff_offer_contract_screen` (0x4e4ea0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_handle_staff_offer_contract_screen(item: u32) -> i32 {
    let id = tel26_item_id(item as i16);
    if id != OFFER_SEND {
        if id == OFFER_BACK {
            return -9;
        }
        return manager_menu_bar(item);
    }
    let terms = Contract(exe::human_manager_get_contract_offer(read_usize(G_HUMAN_MANAGER), 0));
    let manager = Staff(tel26_slot_manager().0);
    let person = Staff(staff_at(read_i32(terms.addr()) as isize).0);
    let job = get_screen_param(4) as u8;
    let mark = job as u32; // simplification: the original pushes the dword whose low byte is this parameter (the rest is stack leftover)
    let withdraw = get_screen_param(5);
    let told_unset = get_screen_param(6) as u8;
    let compensation = get_screen_param(7);
    if withdraw != 0 {
        covered!("tel31_3 staff offer withdrawn");
        let club = manager.club_job().addr() as u32;
        exe::contract_manager_remove_contract_offer(read_usize(G_CONTRACT_MANAGER), person, club as usize);
        return -10;
    }
    let (mut slot, mut serial) = (-1i32, u32::MAX);
    let holder = get_screen_param(0);
    if holder != 0 {
        let offer = read_usize(holder);
        slot = get_screen_param(1) as i32;
        serial = get_screen_param(2) as u32;
        match (terms.squad_status() >> 4) as i8 as i32 - 2 {
            0 => terms.set_happiness(0x4bu8 as i8),
            1 => terms.set_happiness(0x32u8 as i8),
            2 | 3 => terms.set_happiness(0u8 as i8),
            _ => {}
        }
        let status = read_u8(offer + offer::STATUS);
        if read_u32(offer + offer::SERIAL) != serial || status == 3 || status == 2 {
            covered!("tel31_3 staff offer changed");
            let old = read_usize(holder);
            if old != 0 {
                let dtor: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
                dtor(old);
                exe::crt_operator_delete(old);
            }
            write_usize(holder, 0);
            // the original passes the offer it has just freed
            exe::transfer_screens_open_offer_changed_screen(offer, serial);
            return -11;
        }
    } else if (manager.club_job().addr() as u32) as usize != contract_club_of(person.addr()) {
        covered!("tel31_3 staff offer with a transfer offer");
        let mut local = StaffOfferLocal([0; 0x44]);
        let o = Offer(local.0.as_mut_ptr() as usize);
        let mut date = [read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE + 4)];
        exe::transfer_offer_transfer_offer_8e56a0(o, person, manager.club_job().addr() as u32, 0, -1, 1);
        o.set_job_type(job);
        let mut kind = o.completion_timing();
        if exe::database_is_player_job(person.job_for_club() as i8) != 0 && contract_club_of(person.addr()) != 0 {
            if compensation != 0 && told_unset == 0 {
                let fee = exe::transfer_manager_calc_compensation_fee(read_usize(G_TRANSFER_MANAGER), person, manager.club_job().addr(), 0);
                o.set_fee((fee) as usize);
                kind = kind & 0xf5 | 5;
                o.set_completion_timing(kind);
            } else {
                let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
                if read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 1 {
                    o.set_completion_timing(o.completion_timing() & 0xf3 | 3);
                    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
                    let mut tmp = [0u32; 2];
                    exe::date_operator_3d(date.as_mut_ptr() as usize, tmp.as_mut_ptr() as usize, c + contract::CONTRACT_EXPIRES);
                }
                kind = o.completion_timing();
            }
        }
        if person.player().addr() != 0 && kind & 0xf != 5 && matches!(job, 0xb | 0xd | 0xf) {
            let club = contract_club_of(person.addr());
            if exe::transfer_manager_can_sign_player(read_usize(G_TRANSFER_MANAGER), manager.club_job().addr(), date.as_mut_ptr() as usize,
                person, club, 0, core::ptr::null_mut()) == 0 {
                covered!("tel31_3 staff offer unsignable");
                let k = o.completion_timing();
                o.set_completion_timing(if k & 0xf == 0 { k & 0xf1 | 1 } else { k & 0xf4 | 4 });
            }
        }
        o.set_player_aware(get_screen_param(3) as u8);
        exe::transfer_manager_add_transfer_offer(read_usize(G_TRANSFER_MANAGER), o.addr() as *mut u8, core::ptr::null_mut());
        slot = o.slot_index() as i32;
        serial = o.serial() as u32;
        let dtor: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
        dtor(o.addr());
    }
    exe::contract_manager_add_contract_offer(read_usize(G_CONTRACT_MANAGER), terms.addr() as *mut u8, slot, serial as usize, mark, 0, 1, 0);
    if holder != 0 {
        let offer = read_usize(holder);
        if read_u32(offer + offer::TARGET_STAFF_ID) == read_u32(terms.addr()) {
            write_u8(offer + offer::TERMS_STATE, 1);
        } else {
            let list = read_usize(offer + offer::EXCHANGE_PLAYERS);
            write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
            let mut e = exe::llist_get_next(read_usize(read_usize(holder) + offer::EXCHANGE_PLAYERS));
            while e != 0 {
                if read_u32(e) == read_u32(terms.addr()) {
                    write_u8(e + 4, 1);
                }
                e = exe::llist_get_next(read_usize(read_usize(holder) + offer::EXCHANGE_PLAYERS));
            }
        }
    }
    write_i32(terms.addr(), -1);
    -10
}

/// Rule: terminating a member of staff's contract asks "Terminate <name>'s contract ?" first.
///
/// `contract_screens_open_terminate_contract_screen(person)`: the screen (builder 0x4e5420, handler 0x4e5610, single
/// instance) and, when it opens, parameter 0 = the person.
/// Original: `contract_screens_open_terminate_contract_screen` (0x4e53e0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_open_terminate_contract_screen(person: usize) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x4e5420usize, 0x4e5610usize, 1, 0usize, 0) != 0 {
        covered!("tel31_3 terminate opened");
        set_screen_param(0, person);
    }
}

/// Rule: the terminate contract screen: the person's picture and name in his club's colours, "Terminate Contract",
/// the question centred in the text area and No / Yes.
///
/// `contract_screens_build_terminate_contract_screen()`: the person (parameter 0) and the slot manager's club; the
/// person's background picture, the title (`database_format_staff_name` with 0x12c, 1, in the club's kit colours),
/// the subtitle "Terminate Contract", the question ("Terminate <%s>{s} contract ?" with 0x64) in a text area (0x6e,
/// 0x8c, 0x30c, 0x221, style 2, the vertical offset 0x195 / the height of font 3 as a byte, font 3, pale grey), the
/// menu bar (4, 0) and a two-column area (weights 3/1, coloured with the club's second kit colour) with "No" (2) and
/// "Yes" (1).
/// Original: `contract_screens_build_terminate_contract_screen` (0x4e5420, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_build_terminate_contract_screen() {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let weights: [u8; 2] = [3, 1];
    let mut name = [0u8; 0x12c];
    let club = read_usize(tel26_slot_manager().0 + staff::CLUB_JOB);
    let person = get_screen_param(0);
    let picture: StaffPictureFn = core::mem::transmute(FN_BACKGROUND_SET_STAFF_PICTURE);
    picture(person, 0, 0);
    let (mut first, mut second) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut first, &mut second);
    exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 0x12c, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, second, first, name.as_ptr() as usize);
    text(JOB_TEXT, 0x9b10f4);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let offset = (0x195 / exe::font_get_height(3)) as u8 as i8;
    text(JOB_TEXT, 0x9b10b4, 0x64, name.as_ptr());
    tel26_text_area([0x6e, 0x8c, 0x30c, 0x221, 2, 0, offset as i32 as u32, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
    exe::menubar_build(4, 0);
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr() as u32, 1, 0, 1, second, u32::MAX]);
    text(JOB_TEXT, 0x98e8cc);
    tel26_area_button([area, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, TERMINATE_NO as u32, 0]);
    text(JOB_TEXT, 0x98e8d0);
    tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, TERMINATE_YES as u32, 0]);
}

/// Rule: Yes ends the contract: the contract goes, the person leaves the club and becomes a free agent; a scout's
/// departure refreshes the human club's scouting lists and any departure its coaching assignments. No closes.
///
/// `contract_screens_handle_terminate_contract_screen(item)`: Yes (1): the person (parameter 0), his job (+0x3d)
/// and club (+0x39) read first; `CONTRACT_MANAGER::remove_contract`, `database_remove_staff_from_club(club read
/// again, person, 0, 1, 0)`, `TRANSFER_MANAGER::free_transfer_staff(person, club, 0, 0, 1)`; job 9 (scout) at a human
/// club: `SCOUT_MANAGER::sync_scout_lists` of the club's +0xcf; a human club: `TRAINING_MANAGER::update_coach_assignments
/// (club, 0)`; -10. No (2): -9. Other ids: the menu bar.
/// Original: `contract_screens_handle_terminate_contract_screen` (0x4e5610, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn contract_screens_handle_terminate_contract_screen(item: u32) -> i32 {
    match tel26_item_id(item as i16) {
        TERMINATE_YES => {
            covered!("tel31_3 contract terminated");
            let person = Staff(get_screen_param(0));
            let job = person.job_for_club();
            let club = person.club_job().addr();
            exe::contract_manager_remove_contract(read_usize(G_CONTRACT_MANAGER), person);
            exe::database_remove_staff_from_club(person.club_job().addr(), person, 0usize, 1, 0);
            exe::transfer_manager_free_transfer_staff(read_usize(G_TRANSFER_MANAGER), person, club, 0, 0, 1);
            if job == 9 && exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
                exe::scout_manager_sync_scout_lists(read_usize(G_SCOUT_MANAGER), read_usize(club + 0xcf));
            }
            if club != 0 && exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
                exe::training_manager_update_coach_assignments(read_usize(G_TRAINING_MANAGER), club, 0);
            }
            -10
        }
        TERMINATE_NO => -9,
        _ => manager_menu_bar(item),
    }
}

/// Rule: the squad list orders the players by appearances (player statistic 6), most first, then by home
/// reputation, highest first; anybody without a player sheet counts 0.
///
/// `compare_squad_players_by_appearances_then_reputation(a, b)`, a qsort order over person pointers:
/// `PlayerStats::get_stat(sheet, 6, 1, 6.5)` of each (0.0 without one; the first stored as a float), the larger
/// first; equal: the larger home reputation (`player_sheet::HOME_REPUTATION`, signed word) first; equal: 0.
/// Original: `compare_squad_players_by_appearances_then_reputation` (0x4e5850, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_squad_players_by_appearances_then_reputation(a: *const usize, b: *const usize) -> i32 {
    let (pa, pb) = (*a, *b);
    let sheet = |p: usize| if p != 0 { read_usize(p + staff::PLAYER) } else { 0 };
    let (sa, sb) = (sheet(pa), sheet(pb));
    let va = if sa != 0 { exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), sa, 6, 1, 6.5) as f32 } else { 0.0 };
    let vb = if sb != 0 { exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), sb, 6, 1, 6.5) } else { 0.0 };
    if !(va as f64 >= vb) {
        return 1;
    }
    if va as f64 > vb {
        return -1;
    }
    let ra = if sa != 0 { read_i16(sa + player_sheet::HOME_REPUTATION) } else { 0 };
    let rb = if sb != 0 { read_i16(sb + player_sheet::HOME_REPUTATION) } else { 0 };
    if ra < rb {
        1
    } else if ra <= rb {
        0
    } else {
        -1
    }
}

/// Comparer for `contract_screens_news_transfer_request_respond_action`.
pub(crate) unsafe extern "C" fn compare_contract_screens_news_transfer_request_respond_action(manager: usize, id: u32) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_CONTRACT_SCREENS_NEWS_TRANSFER_REQUEST_RESPOND_ACTION)(manager, id)
}

/// Comparer for `contract_screens_build_transfer_request_screen`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_contract_screens_build_transfer_request_screen() {
    let o = original_fn::<ContractScreenFn>(IDX_CONTRACT_SCREENS_BUILD_TRANSFER_REQUEST_SCREEN);
    compare_by_display_list_with("Transfer Request screen (contract_screens_build_transfer_request_screen)", &[], || o(),
        || contract_screens_build_transfer_request_screen());
}

/// Comparer for `contract_screens_handle_transfer_request_screen`.
pub(crate) unsafe extern "C" fn compare_contract_screens_handle_transfer_request_screen(item: u32) -> i32 {
    // simplification: the answer writes the contract, the news and the shortlists and draws the random generator;
    // it does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_CONTRACT_SCREENS_HANDLE_TRANSFER_REQUEST_SCREEN)(item)
}

/// Comparer for `contract_screens_handle_staff_offer_contract_screen`.
pub(crate) unsafe extern "C" fn compare_contract_screens_handle_staff_offer_contract_screen(item: u32) -> i32 {
    // simplification: the offers are added and freed; it does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_CONTRACT_SCREENS_HANDLE_STAFF_OFFER_CONTRACT_SCREEN)(item)
}

/// Comparer for `contract_screens_open_terminate_contract_screen`.
pub(crate) unsafe extern "C" fn compare_contract_screens_open_terminate_contract_screen(person: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize)>(IDX_CONTRACT_SCREENS_OPEN_TERMINATE_CONTRACT_SCREEN)(person)
}

/// Comparer for `contract_screens_build_terminate_contract_screen`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_contract_screens_build_terminate_contract_screen() {
    let o = original_fn::<ContractScreenFn>(IDX_CONTRACT_SCREENS_BUILD_TERMINATE_CONTRACT_SCREEN);
    compare_by_display_list_with("Terminate Contract screen (contract_screens_build_terminate_contract_screen)", &[], || o(),
        || contract_screens_build_terminate_contract_screen());
}

/// Comparer for `contract_screens_handle_terminate_contract_screen`.
pub(crate) unsafe extern "C" fn compare_contract_screens_handle_terminate_contract_screen(item: u32) -> i32 {
    // simplification: the contract ends and the person leaves the club; it does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_CONTRACT_SCREENS_HANDLE_TERMINATE_CONTRACT_SCREEN)(item)
}

/// Rule: the Testing section: an empty row, the "Testing" heading (translated) and the testers' names in two columns.
///
/// The rows through the credits row helpers (`tel32_blank_row`, `tel32_heading`, then the names or the role rows);
/// the texts are the exe's own, passed by address as the original does.
/// Original: `credits_add_testing_rows` (0x4e92f0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_testing_rows() {
    covered!("tel32_1 testing rows");
    tel32_blank_row();
    comp_lang_format()(JOB_TEXT, 0x9b18dcusize); // "Testing"
    tel32_heading();
    tel32_name_columns(&TEL32_TESTING_NAMES);
}

/// Rule: the Feral Interactive section: an empty row, the heading (copied, not translated) and the producer row.
///
/// The rows through the credits row helpers (`tel32_blank_row`, `tel32_heading`, then the names or the role rows);
/// the texts are the exe's own, passed by address as the original does.
/// Original: `credits_add_feral_rows` (0x4eb110, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_feral_rows() {
    covered!("tel32_1 feral rows");
    tel32_blank_row();
    tel26_copy_text(JOB_TEXT as *mut u8, 0x9b1900 as *const u8); // "Feral Interactive"
    tel32_heading();
    for &(role, name) in TEL32_FERAL_ROLES.iter() {
        tel32_translated_role_row(role, name);
    }
}

/// Rule: the Eidos Interactive section: an empty row, the heading (copied), the role rows (roles translated) and, in English only, the photos thanks row (printed without translation).
///
/// The rows through the credits row helpers (`tel32_blank_row`, `tel32_heading`, then the names or the role rows);
/// the texts are the exe's own, passed by address as the original does.
/// Original: `credits_add_eidos_rows` (0x4eb480, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_eidos_rows() {
    covered!("tel32_1 eidos rows");
    tel32_blank_row();
    tel26_copy_text(JOB_TEXT as *mut u8, 0x9b1b80 as *const u8); // "Eidos Interactive"
    tel32_heading();
    for &(role, name) in TEL32_EIDOS_ROLES.iter() {
        tel32_translated_role_row(role, name);
    }
    if exe::langlib_get_language() != 0 {
        return;
    }
    covered!("tel32_1 the English only row");
    comp_crt_format()(JOB_TEXT, 0x9b1928usize); // "Tickets and Man City photos"
    tel32_role_row(0x9b1914); // Julie and Debbie
}

/// Rule: the translation section: an empty row, the "Translation and Proof-Reading" heading (translated) and one row per translator with the language as the role (translated).
///
/// The rows through the credits row helpers (`tel32_blank_row`, `tel32_heading`, then the names or the role rows);
/// the texts are the exe's own, passed by address as the original does.
/// Original: `credits_add_translation_rows` (0x4eccd0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_translation_rows() {
    covered!("tel32_1 translation rows");
    tel32_blank_row();
    comp_lang_format()(JOB_TEXT, 0x9b1c40usize); // "Translation and Proof-Reading"
    tel32_heading();
    for &(role, name) in TEL32_TRANSLATION_ROLES.iter() {
        tel32_translated_role_row(role, name);
    }
}

/// Comparer for `credits_add_testing_rows`: side by side over the display list (`compare_match_screen_build`) plus the credits
/// build state (`tel32_credits`).
pub(crate) unsafe extern "C" fn compare_credits_add_testing_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CREDITS_ADD_TESTING_ROWS);
    compare_match_screen_build("credits Testing rows", &[(tel32_credits::ROW, tel32_credits::STATE_SIZE)], || o(), || credits_add_testing_rows());
}

/// Comparer for `credits_add_feral_rows`: side by side over the display list (`compare_match_screen_build`) plus the credits
/// build state (`tel32_credits`).
pub(crate) unsafe extern "C" fn compare_credits_add_feral_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CREDITS_ADD_FERAL_ROWS);
    compare_match_screen_build("credits Feral rows", &[(tel32_credits::ROW, tel32_credits::STATE_SIZE)], || o(), || credits_add_feral_rows());
}

/// Comparer for `credits_add_eidos_rows`: side by side over the display list (`compare_match_screen_build`) plus the credits
/// build state (`tel32_credits`).
pub(crate) unsafe extern "C" fn compare_credits_add_eidos_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CREDITS_ADD_EIDOS_ROWS);
    compare_match_screen_build("credits Eidos rows", &[(tel32_credits::ROW, tel32_credits::STATE_SIZE)], || o(), || credits_add_eidos_rows());
}

/// Comparer for `credits_add_translation_rows`: side by side over the display list (`compare_match_screen_build`) plus the credits
/// build state (`tel32_credits`).
pub(crate) unsafe extern "C" fn compare_credits_add_translation_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CREDITS_ADD_TRANSLATION_ROWS);
    compare_match_screen_build("credits Translation rows", &[(tel32_credits::ROW, tel32_credits::STATE_SIZE)], || o(), || credits_add_translation_rows());
}

/// Rule: the Additional Research section: the "Additional Research" heading (translated) and the 698 researchers'
/// names in two columns, closing the last half row.
///
/// The text "Additional Research" goes through `langlib_sprintf` into the screens' text buffer (`JOB_TEXT`); then
/// the heading (`tel32_heading`: highlighted when the item number is screen parameter 0 on the shown page), the names
/// (`tel32_name_columns`: on the shown page two to a row, left 0x82..0x1b8 and right 0x1c2..0x2f8, in the 0xae3184
/// colour; on the other page only the rows are counted, one per name) and `credits_end_name_columns`. Unlike the
/// other sections there is no empty row first. The names are the exe's own texts, passed by address as the
/// original does; the original is one straight 132 KB run of the same row code per name.
/// Original: `credits_add_additional_research_rows` (0x4eda90, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_additional_research_rows() {
    covered!("tel33_1 additional research rows");
    comp_lang_format()(JOB_TEXT, 0x9b4b1cusize); // "Additional Research"
    tel32_heading();
    tel32_name_columns(&ADDITIONAL_RESEARCH_NAMES);
    credits_end_name_columns();
}

/// Comparer for `credits_add_additional_research_rows`: side by side over the display list
/// (`compare_match_screen_build`) plus the credits build state (`tel32_credits`).
pub(crate) unsafe extern "C" fn compare_credits_add_additional_research_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CREDITS_ADD_ADDITIONAL_RESEARCH_ROWS);
    compare_match_screen_build("credits Additional Research rows", &[(tel32_credits::ROW, tel32_credits::STATE_SIZE)], || o(),
        || credits_add_additional_research_rows());
}

/// Rule: opening the credits screen starts it on its first page.
///
/// `exe::screenmanager_open_screen` with `CREDITS_SCREEN_BUILD` and `CREDITS_SCREEN_INPUT` (flag 0, no leave or exit
/// function); when it opened, screen parameters 0, 1 and 2 are set to 0 (no copy).
/// Original: `credits_screen_open` (0x4e5920, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_screen_open() {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, CREDITS_SCREEN_BUILD, CREDITS_SCREEN_INPUT, 0, 0usize, 0) == 0 {
        return;
    }
    covered!("tel33_2 credits opened");
    for k in 0..3 {
        exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, k, 0usize, 0);
    }
}

/// Comparer for `credits_screen_open`: only the original runs (it opens a screen), then the plan is checked: the
/// current screen builds and answers as the credits screen and its parameters 0, 1 and 2 are 0.
pub(crate) unsafe extern "C" fn compare_credits_screen_open() {
    original_fn::<unsafe extern "C" fn()>(IDX_CREDITS_SCREEN_OPEN)();
    let node = current_screen_node(G_SCREEN_MANAGER);
    let ok = node != 0
        && read_usize(node + screen_node_fields::BUILD) == CREDITS_SCREEN_BUILD
        && read_usize(node + screen_node_fields::INPUT) == CREDITS_SCREEN_INPUT
        && (0..3).all(|k| exe::screenmanager_get_screen_param(G_SCREEN_MANAGER, k) == 0);
    SCOREBOARD.record(ok, || format!("credits_screen_open: the current screen is not the credits screen on its first page (node {:#x})", node));
}

/// Rule: the Sports Interactive section: the heading (copied, not translated, no empty row first) and the rows of
/// name and translated role.
///
/// Through the credits row helpers (`tel32_heading`, `tel32_translated_role_row`), the texts by address as the
/// original does: the two chairmen; the managing director (the role translated with the empty text as its argument,
/// the name "Miles"); seven design and programming rows; Grant Appleyard, whose name is translated into `JOB_TEXT`
/// and the role into `CREDITS_SECOND_TEXT` (`credits_text_row`); six more rows (research co-ordination, internet
/// development, sound effects, manual and written history, QA manager).
/// Original: `credits_add_sports_interactive_rows` (0x4e5970, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_sports_interactive_rows() {
    covered!("tel33_2 sports interactive rows");
    tel26_copy_text(JOB_TEXT as *mut u8, 0x9b1240 as *const u8); // "Sports Interactive"
    tel32_heading();
    for &(role, name) in SPORTS_INTERACTIVE_CHAIRMAN_ROWS.iter() {
        tel32_translated_role_row(role, name);
    }
    comp_lang_format()(JOB_TEXT, 0x9b11f0usize, TEL32_NO_TEXT as usize); // "Managing Director<%s - COMMENT - ..>"
    tel32_role_row(0x9b11e8); // Miles
    for &(role, name) in SPORTS_INTERACTIVE_DESIGN_ROWS.iter() {
        tel32_translated_role_row(role, name);
    }
    comp_lang_format()(JOB_TEXT, 0x99a960usize); // "Grant Appleyard"
    comp_lang_format()(CREDITS_SECOND_TEXT, 0x9b11d0usize); // "Design & Programming"
    credits_text_row(JOB_TEXT as u32, CREDITS_SECOND_TEXT as u32);
    for &(role, name) in SPORTS_INTERACTIVE_OTHER_ROWS.iter() {
        tel32_translated_role_row(role, name);
    }
}

/// Rule: the Jask Consultants section: an empty row, the "Jask Consultants" heading (translated) and the Xbox design
/// and programming row.
///
/// Through the credits row helpers (`tel32_blank_row`, `tel32_heading`, `tel32_translated_role_row`).
/// Original: `credits_add_jask_xbox_rows` (0x4e6a70, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_jask_xbox_rows() {
    covered!("tel33_2 jask xbox rows");
    tel32_blank_row();
    comp_lang_format()(JOB_TEXT, 0x9b127cusize); // "Jask Consultants"
    tel32_heading();
    for &(role, name) in JASK_XBOX_ROWS.iter() {
        tel32_translated_role_row(role, name);
    }
}

/// Rule: the Research section: an empty row, the "Research" heading (translated) and one row per researcher with
/// the nation (translated) as the role.
///
/// Through the credits row helpers; the nations from Argentina to Japan, then South Korea with the name translated
/// too (the nation into `JOB_TEXT`, "Doo-Jin Sa" into `CREDITS_SECOND_TEXT`, `credits_text_row` with the name on the
/// left), then Mexico to Yugoslavia (South Korea again, for the fan club, between Scotland and South Africa).
/// Original: `credits_add_research_rows` (0x4e6dc0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_research_rows() {
    covered!("tel33_2 research rows");
    tel32_blank_row();
    comp_lang_format()(JOB_TEXT, 0x9b16a4usize); // "Research"
    tel32_heading();
    for &(role, name) in RESEARCH_ROWS_ARGENTINA_TO_JAPAN.iter() {
        tel32_translated_role_row(role, name);
    }
    comp_lang_format()(JOB_TEXT, 0x9b14d0usize); // "South Korea"
    comp_lang_format()(CREDITS_SECOND_TEXT, 0x9b14c4usize); // "Doo-Jin Sa"
    credits_text_row(CREDITS_SECOND_TEXT as u32, JOB_TEXT as u32);
    for &(role, name) in RESEARCH_ROWS_MEXICO_TO_YUGOSLAVIA.iter() {
        tel32_translated_role_row(role, name);
    }
}

/// Comparer for `credits_add_sports_interactive_rows`: side by side over the display list (`compare_match_screen_build`)
/// plus the credits build state (`tel32_credits`) and the second text buffer.
pub(crate) unsafe extern "C" fn compare_credits_add_sports_interactive_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CREDITS_ADD_SPORTS_INTERACTIVE_ROWS);
    compare_match_screen_build("credits Sports Interactive rows", &[(tel32_credits::ROW, tel32_credits::STATE_SIZE), (CREDITS_SECOND_TEXT, 0x100)],
        || o(), || credits_add_sports_interactive_rows());
}

/// Comparer for `credits_add_jask_xbox_rows`: side by side over the display list (`compare_match_screen_build`) plus the
/// credits build state (`tel32_credits`).
pub(crate) unsafe extern "C" fn compare_credits_add_jask_xbox_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CREDITS_ADD_JASK_XBOX_ROWS);
    compare_match_screen_build("credits Jask/Xbox rows", &[(tel32_credits::ROW, tel32_credits::STATE_SIZE)], || o(), || credits_add_jask_xbox_rows());
}

/// Comparer for `credits_add_research_rows`: side by side over the display list (`compare_match_screen_build`) plus the
/// credits build state (`tel32_credits`) and the second text buffer.
pub(crate) unsafe extern "C" fn compare_credits_add_research_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CREDITS_ADD_RESEARCH_ROWS);
    compare_match_screen_build("credits Research rows", &[(tel32_credits::ROW, tel32_credits::STATE_SIZE), (CREDITS_SECOND_TEXT, 0x100)],
        || o(), || credits_add_research_rows());
}

/// Rule: the Additional Testing section of the credits: its heading and the 56 external testers in two columns.
///
/// The section gap, "Additional Testing" (translated) as the section's "View" popup entry and heading, the column flag
/// reset to the left, then each tester of `ADDITIONAL_TESTERS` as a two-column name (`tel34_credits_column_name`, the
/// last one closing the section). Off the page shown only the counters move.
/// Original: `credits_add_additional_testing_rows` (0x50e000, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_additional_testing_rows() {
    covered!("tel34_1 additional testing");
    tel34_credits_section_gap();
    tel34_lang_text(0x9b4e88); // "Additional Testing"
    tel34_credits_popup_entry();
    tel34_credits_heading();
    write_u32(CREDITS_RIGHT_COLUMN, 0);
    for (k, &name) in ADDITIONAL_TESTERS.iter().enumerate() {
        tel34_credits_column_name(name, k == ADDITIONAL_TESTERS.len() - 1);
    }
}

/// Rule: the Special Thanks section of the credits: its heading and the name/role rows, some only in English, two
/// with the name in the game's language.
///
/// The section gap, "Special Thanks" (translated) as the section's popup entry and heading; then the rows
/// (`tel34_credits_name_and_role`, the role translated unless said): Webmaster @ SIgames.com, CMzine Editor, CMzine
/// proof reader; in English only (`langlib_get_language` 0) the 16 rows of `SPECIAL_THANKS_ENGLISH` (roles not
/// translated: PR Guru's ×2, For being 'Safe', Top articles and office help, Man City photos, Manchester City, Umbro ×4,
/// Research and quiz questions, Scottish non league research, Teaboy and player histories, Player histories,
/// Stevengage Borough photos ×2); Written Player Histories; in English only For extra curricular stuff (not translated);
/// Manual Design; Maintaining Sanity with "The Sponge Ball" in the game's language; For all your support with "Every CM
/// Fan" in the game's language.
/// Original: `credits_add_special_thanks_rows` (0x511320, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_special_thanks_rows() {
    covered!("tel34_1 special thanks");
    tel34_credits_section_gap();
    tel34_lang_text(0x9b5294); // "Special Thanks"
    tel34_credits_popup_entry();
    tel34_credits_heading();
    for (role, name) in [(0x9b527cusize, 0x9b526cusize), (0x9b525c, 0x9b5248), (0x9b5234, 0x9b522c)] {
        tel34_lang_text(role);
        tel34_credits_name_and_role(name);
    }
    if exe::langlib_get_language() == 0 {
        covered!("tel34_1 special thanks in english");
        for (role, name) in SPECIAL_THANKS_ENGLISH {
            tel34_crt_text(role);
            tel34_credits_name_and_role(name);
        }
    }
    tel34_lang_text(0x9b5014); // "Written Player Histories"
    tel34_credits_name_and_role(0x9b5008);
    if exe::langlib_get_language() == 0 {
        tel34_crt_text(0x9b4fec); // "For extra curricular stuff"
        tel34_credits_name_and_role(0x9b4fdc);
    }
    tel34_lang_text(0x9b4fcc); // "Manual Design"
    tel34_credits_name_and_role(0x9b4fc4);
    tel34_lang_text(0x9b4fb0); // "Maintaining Sanity"
    tel34_credits_name_and_role(tel34_by_language(0x9b4f38, 0x9b4f58, 0x9b4f9c, 0x9b4f88, 0x9b4f48, 0x9b4f78, 0x9b4f6c));
    tel34_lang_text(0x9b4f20); // "For all your support"
    tel34_credits_name_and_role(tel34_by_language(0x9b4e9c, 0x9b4ebc, 0x9b4f0c, 0x9b4ef4, 0x9b4eac, 0x9b4ee0, 0x9b4ed0));
}

/// Rule: the Best Wishes To section of the credits: its heading (no popup entry) and three couples "On getting married".
///
/// The section gap, "Best Wishes To" (translated) as the heading, then "On getting married" (translated) as the role of
/// Woodg and Carol, Honorino and Lorena, and Alby and Seve.
/// Original: `credits_add_best_wishes_rows` (0x512d40, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_best_wishes_rows() {
    covered!("tel34_1 best wishes");
    tel34_credits_section_gap();
    tel34_lang_text(0x9b52ec); // "Best Wishes To"
    tel34_credits_heading();
    for name in [0x9b52c8usize, 0x9b52b4, 0x9b52a4] {
        tel34_lang_text(0x9b52d8); // "On getting married"
        tel34_credits_name_and_role(name);
    }
}

/// Rule: building the Game Credits screen: the title, the buttons, the "View" popup and the scrolling list with every
/// credits section, page 0 then page 1, showing the page of parameter 2.
///
/// The line counter and both pages zeroed, the page shown = parameter 2; the background of picture section 10
/// (`background_set_section_picture` 0x41c8e0, 10, 0); the game title; the menu bar (4, 0); the Back/Next buttons;
/// the subtitle "Game Credits"; the "View" button (0x6e,0x91..0xeb,0xa5) and its popup area (flags 0x130, in
/// [0xae1874]); the list area (0x15d / the font height rows, from y 0xaa); the row counter zeroed. Page 0: Sports
/// Interactive, JASK/Xbox, Research, 0x4e92f0, 0x4eb480, 0x4eb110 and 0x4eccd0 (by address); then page 1 with the line
/// counter zeroed: Additional Research (0x4eda90), Additional Testing, Special Thanks and Best Wishes To; the list
/// scrolled to parameter 0 and parameter 1 = the area.
/// Original: `credits_screen_build` (0x513180, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_screen_build() {
    covered!("tel34_1 credits screen");
    write_u16(CREDITS_SHOWN_PAGE, 0);
    write_u16(CREDITS_PAGE, 0);
    write_u16(CREDITS_LINE, 0);
    write_u16(CREDITS_SHOWN_PAGE, get_screen_param(2) as u16);
    exe::background_set_section_picture(10, 0);
    tel34_game_title();
    exe::menubar_build(4, 0);
    tel26_back_next();
    tel34_lang_text(0x9b52fc); // "Game Credits"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 0);
    tel34_lang_text(0x988004); // "View"
    let button = tel26_add_button([0x6e, 0x91, 0xeb, 0xa5, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0, 0]);
    let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), button]);
    write_u16(CREDITS_POPUP_AREA, popup as u16);
    tel34_credits_list_area(0x15d, 0xaa);
    write_u32(CREDITS_ROW, 0);
    let section = |address: usize| {
        let f: unsafe extern "C" fn() = core::mem::transmute(address);
        f();
    };
    for address in [0x4e5970usize, 0x4e6a70, 0x4e6dc0, 0x4e92f0, 0x4eb480, 0x4eb110, 0x4eccd0] {
        section(address);
    }
    write_u16(CREDITS_PAGE, 1);
    write_u16(CREDITS_LINE, 0);
    section(0x4eda90);
    credits_add_additional_testing_rows();
    credits_add_special_thanks_rows();
    credits_add_best_wishes_rows();
    tel34_credits_list_done();
}

/// Rule: a click on the Game Credits screen keeps the list's scroll row and, on a "View" popup entry (id 100), opens
/// that section: its page and line become the shown page and the selected row (-11, rebuild).
///
/// The command id 0x42b is cleared first. For an item (not -1) parameter 0 = the list's scroll row
/// (`ScreenManager::get_area_scroll_row` of the area in parameter 1); the item's id (or the command id). Id 100: the
/// item's value (0 for -1): parameter 0 = its low word, parameter 2 = its high word, -11. Otherwise the menu bar
/// (`menubar_handle_command`, then `menubar_handle_object_link` when it gives 0).
/// Original: `credits_screen_handle_action` (0x5133c0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_screen_handle_action(raw: u32) -> i32 {
    if read_u16(G_CURRENT_COMMAND_ID) == 0x42b {
        write_u16(G_CURRENT_COMMAND_ID, 0);
    }
    let item = raw as i16;
    if item != -1 {
        let row = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(1));
        set_screen_param(0, row);
    }
    if tel26_item_id(item) == 100 {
        covered!("tel34_1 credits view");
        let value = tel26_item_value(item);
        set_screen_param(0, (value & 0xffff) as usize);
        set_screen_param(2, (value >> 16) as usize);
        return -11;
    }
    let r = exe::menubar_handle_command(raw);
    if r != 0 { r } else { exe::menubar_handle_object_link(raw) }
}

/// Rule: opening the Web Sites screen (builder 0x513a20, handler 0x49cc10, flag 0); when it opens, parameters 0 and
/// 1 are cleared.
/// Original: `web_sites_screen_open` (0x5134c0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn web_sites_screen_open() {
    covered!("tel34_1 open web sites");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x513a20usize, 0x49cc10usize, 0, 0usize, 0) != 0 {
        set_screen_param(0, 0);
        set_screen_param(1, 0);
    }
}

/// Rule: the publisher rows of the Web Sites list: the heading "Eidos Interactive" (not translated), "The publishers"
/// with www.eidos.co.uk, and an empty row.
/// Original: `web_sites_add_publisher_rows` (0x513500, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn web_sites_add_publisher_rows() {
    covered!("tel34_1 publisher rows");
    tel34_copy_to_text(0x9b1b80); // "Eidos Interactive"
    tel34_credits_title_rows();
    tel34_lang_text(0x9b530c); // "The publishers"
    tel34_web_site_row(0x9b531c); // "www.eidos.co.uk"
    tel34_credits_blank_row();
}

/// Rule: the fan club rows of the Web Sites list, read from CM3_DATA\fancluburls.txt: an empty row, the heading
/// "Unofficial Sites" and one row per "http://" link of the file.
///
/// The file opened "r" (`utils_build_file_path` + `fopen`); none: nothing. Each character read (`fgetc` 0x9469e0)
/// goes into a 256-byte buffer (at most 255 kept, always closed); once 7 or more are in, the last seven compared to
/// "http://" without case (`stricmp`): on a match the buffer restarts with the link, read up to EOF or a character up
/// to 0x20 (that character lost), then the rest of the line up to '\n' or EOF is read into a second buffer that is
/// never used (characters below 0x20 skipped), and the link becomes a row across the list (the row counter moves on);
/// the scan goes on writing the first buffer from the length of that unused rest (one index register in the
/// original). EOF ends; the file is closed.
/// Original: `web_sites_add_fan_club_links` (0x513750, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn web_sites_add_fan_club_links() {
    let path = exe::utils_build_file_path(0x9b5348usize, 0x9b5354usize); // "CM3_DATA", "fancluburls.txt"
    let file = exe::crt_fopen(path, 0x9899fcusize); // "r"
    if file == 0 {
        return;
    }
    covered!("tel34_1 fan club links");
    tel34_credits_blank_row();
    tel34_lang_text(0x9b5334); // "Unofficial Sites"
    tel34_credits_title_rows();
    let mut link = [0u8; 0x100];
    let mut rest = [0u8; 0x100];
    let mut n = 0usize;
    let push = |buf: &mut [u8; 0x100], n: &mut usize, c: u8| {
        buf[*n] = c;
        *n = (*n + 1).min(0xff);
        buf[*n] = 0;
    };
    link[0] = 0;
    let mut c = exe::at_9469e0(file);
    while c != -1 {
        push(&mut link, &mut n, c as u8);
        if n >= 7 && exe::crt_stricmp(link.as_ptr().wrapping_add(n - 7) as usize, 0x9b532cusize) == 0 {
            covered!("tel34_1 fan club link");
            n = 0;
            link[0] = 0;
            let mut c = exe::at_9469e0(file);
            while c != -1 && c as u8 > 0x20 {
                push(&mut link, &mut n, c as u8);
                c = exe::at_9469e0(file);
            }
            let mut m = 0usize;
            let mut c = exe::at_9469e0(file);
            while c != -1 && c as u8 != b'\n' {
                if c as u8 >= 0x20 {
                    push(&mut rest, &mut m, c as u8);
                }
                c = exe::at_9469e0(file);
            }
            let row = tel34_credits_next_row();
            tel34_credits_cell(0x82, 0x2f8, row, JOB_COLOUR_3184, link.as_ptr() as usize);
            // the original keeps one index register: the link buffer goes on from the length of the unused rest
            n = m;
        }
        c = exe::at_9469e0(file);
    }
    exe::crt_fclose(file);
}

/// Comparer for `credits_add_additional_testing_rows`: side by side over the display list and the credits state.
pub(crate) unsafe extern "C" fn compare_credits_add_additional_testing_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CREDITS_ADD_ADDITIONAL_TESTING_ROWS);
    compare_by_display_list_with("Game Credits screen (credits_add_additional_testing_rows)", &[CREDITS_STATE], || o(),
        || credits_add_additional_testing_rows());
}

/// Comparer for `credits_add_special_thanks_rows`: side by side over the display list and the credits state.
pub(crate) unsafe extern "C" fn compare_credits_add_special_thanks_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CREDITS_ADD_SPECIAL_THANKS_ROWS);
    compare_by_display_list_with("Game Credits screen (credits_add_special_thanks_rows)", &[CREDITS_STATE], || o(),
        || credits_add_special_thanks_rows());
}

/// Comparer for `credits_add_best_wishes_rows`: side by side over the display list and the credits state.
pub(crate) unsafe extern "C" fn compare_credits_add_best_wishes_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CREDITS_ADD_BEST_WISHES_ROWS);
    compare_by_display_list_with("Game Credits screen (credits_add_best_wishes_rows)", &[CREDITS_STATE], || o(),
        || credits_add_best_wishes_rows());
}

/// Comparer for `credits_screen_build`: the Game Credits screen side by side over the display list and the credits
/// state.
pub(crate) unsafe extern "C" fn compare_credits_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CREDITS_SCREEN_BUILD);
    compare_by_display_list_with("Game Credits screen (credits_screen_build)", &[CREDITS_STATE], || o(), || credits_screen_build());
}

/// Comparer for `credits_screen_handle_action`.
pub(crate) unsafe extern "C" fn compare_credits_screen_handle_action(raw: u32) -> i32 {
    // simplification: the menu bar commands (they open screens) do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_CREDITS_SCREEN_HANDLE_ACTION)(raw)
}

/// Comparer for `web_sites_screen_open`.
pub(crate) unsafe extern "C" fn compare_web_sites_screen_open() {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(IDX_WEB_SITES_SCREEN_OPEN)()
}

/// Comparer for `web_sites_add_publisher_rows`: side by side over the display list and the row state.
pub(crate) unsafe extern "C" fn compare_web_sites_add_publisher_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_WEB_SITES_ADD_PUBLISHER_ROWS);
    compare_by_display_list_with("Web Sites screen (web_sites_add_publisher_rows)", &[CREDITS_STATE], || o(), || web_sites_add_publisher_rows());
}

/// Comparer for `web_sites_add_fan_club_links`: side by side over the display list and the row state (the file is
/// only read, once by each side).
pub(crate) unsafe extern "C" fn compare_web_sites_add_fan_club_links() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_WEB_SITES_ADD_FAN_CLUB_LINKS);
    compare_by_display_list_with("Web Sites screen (web_sites_add_fan_club_links)", &[CREDITS_STATE], || o(), || web_sites_add_fan_club_links());
}

/// Rule: building the Web Sites screen: the title, the scrolling list with the developers' and the official
/// messageboard's addresses, the publisher rows and the fan club links, and the Back/Next buttons.
///
/// A random background picture (`background_set_random_picture`); the game title (heading colours 0 and 1); the menu
/// bar (4, 0); the subtitle "Web Sites"; the list area (0x176 / the font height rows, from y 0x91); the heading "Sports
/// Interactive" (not translated, the row counter from 0); "The developers" with www.sigames.com and "Official
/// Messageboard" with community.sigames.com; the publisher rows (`web_sites_add_publisher_rows`) and the fan club links
/// (`web_sites_add_fan_club_links`); the list scrolled to parameter 0 and parameter 1 = the area; then the Back/Next
/// buttons.
/// Original: `web_sites_screen_build` (0x513a20, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn web_sites_screen_build() {
    covered!("tel34_2 web sites screen");
    exe::background_set_random_picture();
    tel34_game_title();
    exe::menubar_build(4, 0);
    tel34_lang_text(0x9b53b4); // "Web Sites"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 0);
    tel34_credits_list_area(0x176, 0x91);
    tel34_copy_to_text(0x9b1240); // "Sports Interactive"
    write_u32(CREDITS_ROW, 0);
    tel34_credits_title_rows();
    tel34_lang_text(0x9b53a4); // "The developers"
    tel34_web_site_row(0x9b5394); // "www.sigames.com"
    tel34_lang_text(0x9b537c); // "Official Messageboard"
    tel34_web_site_row(0x9b5364); // "community.sigames.com"
    web_sites_add_publisher_rows();
    web_sites_add_fan_club_links();
    tel34_credits_list_done();
    tel26_back_next();
}

/// Comparer for `web_sites_screen_build`: the Web Sites screen side by side over the display list and the row state.
pub(crate) unsafe extern "C" fn compare_web_sites_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_WEB_SITES_SCREEN_BUILD);
    compare_by_display_list_with("Web Sites screen (web_sites_screen_build)", &[CREDITS_STATE], || o(), || web_sites_screen_build());
}

/// The staff record of a staff id (`staff::SIZE` × id into the staff table, in 32 bits like the original).
pub(crate) unsafe fn tel39_staff(id: u32) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add(id.wrapping_mul(staff::SIZE as u32) as usize)
}

/// The manager's news item `id` into a local NewsItem (`NewsItem::NewsItem(0, 0)`, `News::find_item_by_id`).
pub(crate) unsafe fn tel39_news_item(manager: usize, id: u32, item: usize) {
    exe::new_newsitem(item, 0, 0);
    exe::news_find_item_by_id(read_usize(G_NEWS), manager, id, item);
}

/// The shortlist manager (`[transfer manager + 0x856]`).
pub(crate) unsafe fn cs_shortlist_manager() -> usize {
    read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856)
}

/// `staff_can_take_job(job, staff)` by address (the low byte of `job` is the job).
pub(crate) unsafe fn cs_can_take_job(job: u32, staff: usize) -> bool {
    let f: unsafe extern "stdcall" fn(u32, usize) -> u32 = core::mem::transmute(CS_FN_STAFF_CAN_TAKE_JOB);
    f(job, staff) != 0
}

/// How many of the jobs 0..15 the person can take.
pub(crate) unsafe fn cs_jobs_open_to(staff: usize) -> i8 {
    let mut n: i8 = 0;
    for job in 0..16u32 {
        if cs_can_take_job(job, staff) {
            n = n.wrapping_add(1);
        }
    }
    n
}

/// `get_contract_satisfaction(staff)` of a contract, by address.
pub(crate) unsafe fn cs_contract_satisfaction(contract: usize, staff: usize) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(CS_FN_CONTRACT_SATISFACTION);
    f(contract, staff)
}

/// `shortlist_manager_squad_status_allowed(staff, status)` by address.
pub(crate) unsafe fn cs_status_allowed(staff: usize, status: u32) -> bool {
    let f: unsafe extern "stdcall" fn(usize, u32) -> u32 = core::mem::transmute(CS_FN_SQUAD_STATUS_ALLOWED);
    f(staff, status) != 0
}

/// The squad status nibble of a contract (`contract::SQUAD_STATUS` >> 4).
pub(crate) unsafe fn cs_squad_status(contract: usize) -> u8 {
    let contract = Contract(contract);
    contract.squad_status() >> 4
}

/// The contract type of a contract (the low six bits of `contract::CONTRACT_TYPE`).
pub(crate) unsafe fn cs_contract_type(contract: usize) -> u8 {
    let contract = Contract(contract);
    contract.contract_type() & 0x3f
}

/// The current slot's human manager's club (`staff::CLUB_JOB` of the slot manager).
pub(crate) unsafe fn cs_human_club() -> usize {
    read_usize(tel26_slot_manager().addr() + staff::CLUB_JOB)
}

/// The value a club would put on a player for the offer screen: 70% of `calc_player_value(staff, club, 1, -1, 0)`,
/// stepped down once (0 when the person is not a player).
pub(crate) unsafe fn cs_offer_value(staff: usize, club: usize) -> u32 {
    let staff = Staff(staff);
    if staff.player().addr() == 0 {
        return 0;
    }
    let v = exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), staff, club, 1, -1, 0);
    cs_step_down(ftol(v as f64 * read_f64(rdata::F64_0_7)))
}

/// The transfer budget shown on the offer screen: the low dword of `Finance::get_transfer_budget(staff, 0, 0, 1, 0)`
/// of the club, less `fee`, kept within 0..5,000,000.
pub(crate) unsafe fn cs_budget(club: usize, staff: usize, fee: i32) -> i32 {
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    let b = (exe::finance_get_transfer_budget(fin, staff as u32, 0, 0usize, 1, 0) as i32).wrapping_sub(fee);
    if b < 0 { 0 } else if b > CS_BUDGET_CAP { CS_BUDGET_CAP } else { b }
}

/// The wage limit of the offer screen for a non-player job: `Finance::get_max_staff_wage(job)`, raised to the
/// person's current wage (`CONTRACT_MANAGER::get_wage`) when he is already at the club (the contract is looked up
/// twice, `G_CONTRACT_LOOKUP_RESULT` keeps the last).
pub(crate) unsafe fn cs_staff_wage_limit(club: usize, staff: usize, job: u32) -> f64 {
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    let mut limit = exe::finance_get_max_staff_wage(fin, job as i8) as f64;
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
    if !c.is_null() && club_at((c.club() as i32) as isize).addr() != 0 && contract_club_cached(Staff(staff)) == club {
        let wage = exe::contract_manager_get_wage(read_usize(G_CONTRACT_MANAGER), staff) as f64;
        if wage > limit {
            limit = wage;
        }
    }
    limit
}

/// Whether a job is one of the player jobs that take the club's player wage limit (11, 12, 13, 15).
pub(crate) fn cs_player_wage_job(job: i8) -> bool {
    matches!(job, 0xb | 0xc | 0xf | 0xd)
}

/// `strcat(dst, src)`, as the original's inline `repne scasb` / `rep movs`.
pub(crate) unsafe fn cs_append_text(dst: usize, src: usize) {
    let mut end = dst;
    while read_u8(end) != 0 {
        end += 1;
    }
    tel26_copy_text(end as *mut u8, src as *const u8);
}

/// The human manager's contract terms being edited (`HUMAN_MANAGER::get_contract_offer(0)`).
pub(crate) unsafe fn cs_human_terms() -> usize {
    let f: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(CS_FN_HUMAN_CONTRACT_OFFER);
    f(read_usize(G_HUMAN_MANAGER), 0)
}

/// `CONTRACT_MANAGER::will_negotiate_offer(terms)`.
pub(crate) unsafe fn cs_will_negotiate(terms: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(CS_FN_WILL_NEGOTIATE);
    f(read_usize(G_CONTRACT_MANAGER), terms)
}

pub(crate) unsafe fn cs_copy_terms(dst: usize, src: usize) {
    for k in 0..CS_TERMS_SIZE {
        write_u8(dst + k, read_u8(src + k));
    }
}

/// A money value formatted into `dst` (2000 bytes) in `mode`, again in `short` when it takes more than 8 characters
/// (`Cash::get_display_amount` + `cash_format_money`).
pub(crate) unsafe fn cs_money_text(dst: usize, value: u32, mode: u32, short: u32) {
    let cash = value;
    exe::cash_format_money(dst, 0x7d0, exe::cash_get_display_amount(&cash, mode), mode);
    if c_string_len(dst) > 8 {
        exe::cash_format_money(dst, 0x7d0, exe::cash_get_display_amount(&cash, short), short);
    }
}

/// `Cash::set_from_amount_table(index, table, count)` on a cash dword.
pub(crate) unsafe fn cs_cash_from_table(cash: *mut u32, index: u32, table: *const u32, count: u32) {
    let f: unsafe extern "thiscall" fn(*mut u32, u32, *const u32, u32) = core::mem::transmute(CS_FN_CASH_FROM_TABLE);
    f(cash, index, table, count);
}

/// The text of the current human's display wrapped into `area` (x 0x78..0x302) from `row`, font 3.
pub(crate) unsafe fn cs_wrapped(t: *mut usize, area: u32, row: &mut i32, colour: u32) {
    let draw: unsafe extern "thiscall" fn(usize, u32, i32, i32, *mut i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(TS_FN_TEXT_DRAW_WRAPPED);
    exe::text_set_string(t as usize, TS_TEXT as *const u8);
    draw(t as usize, area, 0x78, 0x302, row, 3, colour, 0, 0);
}

/// The clauses tab of the offer contract screen (inline in `contract_screens_build_offer_contract_screen`).
pub(crate) unsafe fn cs_build_clauses_tab(terms: usize, edited: usize, staff: usize, top: i32, bottom: i32) {
    let terms = Contract(terms);
    let lang: TsLang = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let allowed: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(CS_FN_CLAUSE_ALLOWED);
    let clause_name: unsafe extern "C" fn(u32, usize) = core::mem::transmute(CS_FN_CLAUSE_NAME);
    let add_menu_item: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(CS_FN_ADD_MENU_ITEM);
    let pale = ts_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = ts_colour(G_GUI_COLOUR_GREY);
    let dark = ts_colour(G_GUI_COLOUR_DARK_BLUE);
    let marker = ts_colour(0xaebdd4);
    let w_labels: [u8; 3] = [2, 1, 0x24];
    let w_values: [u8; 9] = [2, 1, 0xe, 2, 8, 2, 2, 2, 6];
    let mut row: i32 = 0;
    let mut popup: u32 = u32::MAX;
    let mut text = [0usize; 8];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0usize);
    let mut any = false;
    let mut cash: u32 = 0;
    let steps = CS_RELEASE_FEE_STEPS;
    let asked = cs_will_negotiate(edited);
    let rows = ((bottom - top) / exe::font_get_height(3)) as u8;
    let labels = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 3, w_labels.as_ptr() as u32, rows as u32, 0, 2, pale, u32::MAX]);
    let values = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x30c, bottom as u32, 9, w_values.as_ptr() as u32, rows as u32, 0, 1, pale, u32::MAX]);
    lang(TS_TEXT, 0x9afe94); // "Offer player the following clauses:-"
    cs_wrapped(t, labels, &mut row, pale);
    row += 2;
    let terms_club = || club_at(read_i32(terms.addr() + 4) as isize).addr();
    let clause_row = |k: u32, row: &mut i32, any: &mut bool| {
        if read_i32(terms.addr() + 4) == -1 || allowed(staff, terms_club(), k) == 0 {
            return;
        }
        if read_u8(edited + CS_TERMS_CLAUSES + k as usize) != 0 && asked != 0 {
            tel26_area_text([labels, 1, *row as u32, 1, 0, 0xc, 3, marker, 0x989ad8]); // "*"
        }
        clause_name(k, TS_TEXT);
        if read_u8(terms.addr() + CS_TERMS_CLAUSES + k as usize) == 1 {
            tel26_area_button([labels, 2, *row as u32, 1, 0, 1, 3, pale, TS_TEXT as u32, 0x13, k]);
            *any = true;
        } else {
            tel26_area_button([labels, 2, *row as u32, 1, 0, 1, 3, grey, TS_TEXT as u32, 0x12, k]);
        }
        *row += 1;
    };
    for k in 0..4u32 {
        clause_row(k, &mut row, &mut any);
    }
    row += 1;
    let r = row as u32;
    let minimum: i32;
    if !any {
        covered!("tel24_2 clauses without a release clause");
        lang(TS_TEXT, 0x9afe80); // "Clause Release Fee"
        tel26_area_text([labels, 2, r, 1, 0, 1, 3, grey, TS_TEXT as u32]);
        tel26_area_text([values, 3, r, 0x30, dark, 0x2c, 3, pale, 0x98ba0c]); // "-"
        let mut fee = terms.release_fee() as u32;
        cs_cash_display(&mut fee, 6);
        if c_string_len(TS_TEXT) > 8 {
            cs_cash_display(&mut fee, 5);
        }
        tel26_area_text([values, 4, r, 1, 0, 0x2c, 3, pale, TS_TEXT as u32]);
        tel26_area_text([values, 5, r, 0x30, dark, 0x2c, 3, pale, 0x9afe7c]); // "+"
        tel26_area_text([values, 6, r, 0x30, dark, 0x2c, 3, pale, 0x9afe78]);
        minimum = fee as i32;
    } else {
        covered!("tel24_2 clauses with a release clause");
        let fee = terms.release_fee() as i32;
        lang(TS_TEXT, 0x9afe80); // "Clause Release Fee"
        tel26_area_text([labels, 2, r, 1, 0, 1, 3, pale, TS_TEXT as u32]);
        minimum = if terms.minimum_fee_rc() == 1 { get_screen_param(0x17) as i32 } else { 5 };
        let minus = if minimum < fee {
            tel26_area_button([values, 3, r, 0x30, dark, 0xc, 3, pale, 0x98ba0c, 0x28, 0])
        } else {
            tel26_area_text([values, 3, r, 0x30, dark, 0x2c, 3, pale, 0x98ba0c])
        };
        set_screen_param(0xe, minus as i16 as i32 as usize);
        cs_money_text(TS_TEXT, fee as u32, 6, 5);
        let item = tel26_area_button([values, 4, r, 1, 0, 0xc, 3, pale, TS_TEXT as u32, 0, 0]);
        popup = exe::screenmanager_add_popup_area(G_SCREEN_MANAGER, item);
        if fee < 50_000_000 {
            tel26_area_button([values, 5, r, 0x30, dark, 0xc, 3, pale, 0x9afe7c, 0x27, 0]);
        } else {
            tel26_area_text([values, 5, r, 0x30, dark, 0x2c, 3, pale, 0x9afe7c]);
        }
        tel26_area_button([values, 6, r, 0x30, dark, 0xc, 3, pale, 0x9afe78, 0x29, 0]);
    }
    row += 2;
    lang(TS_TEXT, 0x9afe5c); // "Non Release Fee Clauses:-"
    cs_wrapped(t, labels, &mut row, pale);
    row += 2;
    clause_row(4, &mut row, &mut any);
    if asked != 0 {
        row = rows as i8 as i32 - 1;
        lang(TS_TEXT, 0x9afe40); // "* Requested by the player"
        cs_wrapped(t, labels, &mut row, marker);
    }
    if popup as u16 != 0xffff {
        for k in 0..9u32 {
            cs_cash_from_table(&mut cash, k, steps.as_ptr(), 9);
            cs_cash_display(&mut cash, 5);
            if (cash as i32) < minimum {
                contract_screens_add_disabled_menu_item_by_address(popup, TS_TEXT as u32);
            } else {
                add_menu_item(G_SCREEN_MANAGER, popup, TS_TEXT, 0x26, cash);
            }
        }
    }
    exe::drop_text(t);
}

/// The regions an offer screen build writes beyond the display list: the human's terms, the edited terms and the
/// contract lookup global.
pub(crate) unsafe fn cs_offer_regions() -> Vec<(usize, usize)> {
    let mut r = vec![(G_CONTRACT_LOOKUP_RESULT, 4)];
    let terms = cs_human_terms();
    if terms != 0 {
        r.push((terms, CS_TERMS_SIZE));
    }
    let edited = get_screen_param(2);
    if edited != 0 {
        r.push((edited, CS_TERMS_SIZE));
    }
    r
}

/// The fatal box of a contract tab answer without the human's terms: the contract_screens.cpp assert box at `line`,
/// the game's fatal cleanup and exit(-1).
pub(crate) unsafe fn cs_fatal(line: u32) -> ! {
    tel26_error_box(CS_CPP, line);
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exe::crt_exit(-1);
    unreachable!()
}

/// A widget of the tab set to a state: flags, text flags, id and value (`set_widget_flags`, `set_widget_text_flags`,
/// `set_widget_id`, `set_widget_data`).
pub(crate) unsafe fn cs_widget(item: u32, flags: u32, text_flags: u32, id: u32, data: u32) {
    exe::screenmanager_set_widget_flags(G_SCREEN_MANAGER, item as usize, flags);
    exe::screenmanager_set_widget_text_flags(G_SCREEN_MANAGER, item, text_flags);
    exe::screenmanager_set_widget_id(G_SCREEN_MANAGER, item, id);
    exe::screenmanager_set_widget_data(G_SCREEN_MANAGER, item, data as usize);
}

/// The most a bonus can be: a tenth of the maximum wage of the edited terms' club for their person
/// (`get_max_wage(person, -1)`).
pub(crate) unsafe fn cs_bonus_cap(edited: usize) -> i32 {
    let edited = Contract(edited);
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club_at((edited.club() as i32) as isize).addr());
    exe::finance_get_max_wage(fin, staff_at(read_i32(edited.addr()) as isize).addr(), u32::MAX) / 10
}

/// Comparer of a contract tab answer: the steps that change the widgets and redraw (`live_ids`) and the missing terms
/// (the fatal box) only in the original; the others (they only change the human's terms) side by side over the terms
/// and the answer.
pub(crate) unsafe fn cs_compare_tab_answer(name: &str, item: u32, live_ids: &[u16], o: unsafe extern "C" fn(u32) -> i32, r: unsafe extern "C" fn(u32) -> i32) -> i32 {
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

/// `STAFF_CONTRACT::format_wage_text(buffer, mode, style)` of contract terms.
pub(crate) unsafe fn cs_format_wage(terms: usize, dst: usize, mode: u32, style: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(CS_FN_FORMAT_WAGE_TEXT);
    f(terms, dst, mode, style);
}

/// `STAFF_CONTRACT::format_expiry_text(buffer, style)` of contract terms.
pub(crate) unsafe fn cs_format_expiry(terms: usize, dst: usize, style: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(CS_FN_FORMAT_EXPIRY_TEXT);
    f(terms, dst, style);
}

/// The short name of a month ("Aug"), by its low byte.
pub(crate) unsafe fn cs_month_name(month: i8) -> usize {
    let f: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(CS_FN_SHORT_MONTH_NAME);
    f(month as u8 as u32)
}

/// `Cash::set_from_amount_table(index, table, count)` on a cash dword, returning the amount it stores.
pub(crate) unsafe fn cs_cash_step_from(cash: *mut u32, index: u32, table: *const i32, count: u32) -> u32 {
    let f: unsafe extern "thiscall" fn(*mut u32, u32, *const i32, u32) -> u32 = core::mem::transmute(CS_FN_CASH_FROM_TABLE);
    f(cash, index, table, count)
}

/// A money value formatted into `TS_TEXT` with the size 200 in `mode`, again in `short` when it takes more than 8
/// characters (the answers of the tab pass 0xc8 where the builders pass 2000).
pub(crate) unsafe fn cs_money_text_200(value: u32, mode: u32, short: u32) {
    let cash = value;
    exe::cash_format_money(TS_TEXT, 0xc8, exe::cash_get_display_amount(&cash, mode), mode);
    if c_string_len(TS_TEXT) > 8 {
        exe::cash_format_money(TS_TEXT, 0xc8, exe::cash_get_display_amount(&cash, short), short);
    }
}

/// A widget of the tab set to a state: flags, text flags and id (`set_widget_flags`, `set_widget_text_flags`,
/// `set_widget_id`).
pub(crate) unsafe fn cs_widget_state(item: u32, flags: u32, text_flags: u32, id: u32) {
    exe::screenmanager_set_widget_flags(G_SCREEN_MANAGER, item as usize, flags);
    exe::screenmanager_set_widget_text_flags(G_SCREEN_MANAGER, item, text_flags);
    exe::screenmanager_set_widget_id(G_SCREEN_MANAGER, item, id);
}

/// The same three steps as the original writes them for the wage and fee "+"/"-" neighbours: `set_widget_id` where
/// `set_widget_flags` was meant (the id is then set again), so the widget's flags stay as they were.
pub(crate) unsafe fn cs_widget_state_by_id(item: u32, flags: u32, text_flags: u32, id: u32) {
    exe::screenmanager_set_widget_id(G_SCREEN_MANAGER, item, flags);
    exe::screenmanager_set_widget_text_flags(G_SCREEN_MANAGER, item, text_flags);
    exe::screenmanager_set_widget_id(G_SCREEN_MANAGER, item, id);
}

/// The wage below which a young player (under 24) of the edited terms' club still expects terms: the club's
/// reputation turned into a wage by the squad status of the edited terms (0 below the bands).
///
/// Status 1: above 7250 reputation × 1.25 + 10000, above 6250 reputation + 7500, above 4750 reputation × 0.75 + 2500.
/// Status 2: above 7250 reputation + 7500, above 6250 × 0.75 + 5000, above 4750 × 0.5. Status 3: above 7250 × 0.75 +
/// 5000, above 6250 × 0.5 + 2500, above 4750 × 0.25. Status 5: above 7250 × 0.5, above 6250 × 0.25. Status 4 and the
/// rest: 0. The result is truncated (`_ftol`) and kept as a word.
pub(crate) unsafe fn cs_young_wage_threshold(club: usize, status: u8) -> i16 {
    let club = Club(club);
    let r = club.reputation();
    let rep = r as f64;
    let d = |a: usize| read_f64(a);
    let v = match status {
        1 if r > 0x1c52 => rep * d(rdata::F64_1_25) + d(rdata::F64_10000),
        1 if r > 0x186a => rep + d(rdata::F64_7500),
        1 if r > 0x128e => rep * d(rdata::F64_0_75) + d(CS_F64_2500),
        2 if r > 0x1c52 => rep + d(rdata::F64_7500),
        2 if r > 0x186a => rep * d(rdata::F64_0_75) + d(rdata::F64_5000),
        2 if r > 0x128e => rep * d(rdata::F64_0_5),
        3 if r > 0x1c52 => rep * d(rdata::F64_0_75) + d(rdata::F64_5000),
        3 if r > 0x186a => rep * d(rdata::F64_0_5) + d(CS_F64_2500),
        3 if r > 0x128e => rep * d(rdata::F64_0_25),
        5 if r > 0x1c52 => rep * d(rdata::F64_0_5),
        5 if r > 0x186a => rep * d(rdata::F64_0_25),
        _ => return 0,
    };
    ftol(v) as i16
}

/// The manager id of a club (`club::MANAGER`), or -1 without one.
pub(crate) unsafe fn cs_club_manager_id(club: usize) -> u32 {
    let club = Club(club);
    let m = club.manager().addr();
    if m != 0 { read_u32(m) } else { u32::MAX }
}

/// Copies a C string with its terminator (the original's inline strcpy).
pub(crate) unsafe fn tel26_copy_text(dst: *mut u8, src: *const u8) {
    let mut i = 0;
    loop {
        let b = *src.add(i);
        *dst.add(i) = b;
        if b == 0 {
            break;
        }
        i += 1;
    }
}

/// `ScreenManager::add_area_button` (0x8040a0): area, column, row, five style fields, colour, text, id, value.
pub(crate) unsafe fn tel26_area_button(f: [u32; 11]) -> u32 {
    type AreaButton = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: AreaButton = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// The player's answer to a refused transfer request, from his contract: the request is withdrawn
/// (`STAFF_CONTRACT::apply_event_flags` with 0x10000) at once when the contract morale is under -50 and the screen's
/// parameter-0 person (the manager) has loyalty under 6 and professionalism under 10; otherwise only when
/// random(loyalty × 20 + morale + 50) comes out under random((75 − temperament − professionalism) × 5), those three
/// read from the same parameter-0 person (the original reads the manager's, not the player's).
pub(crate) unsafe fn request_refused_answer(manager: usize, contract: usize) {
    let contract = Contract(contract);
    let manager = Staff(manager);
    let morale = contract.morale();
    let (loyalty, professionalism) = (manager.loyalty(), manager.professionalism());
    let withdraw = if morale >= -50 || loyalty >= 6 || professionalism >= 10 {
        let a = exe::utils_random((0x4b - manager.temperament() as i32 - professionalism as i32) * 5);
        let b = exe::utils_random(loyalty as i32 * 20 + morale as i32 + 0x32);
        b < a
    } else {
        true
    };
    if withdraw {
        covered!("tel31_3 request withdrawn");
        let flags: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_APPLY_EVENT_FLAGS);
        flags(contract.addr(), 0x10000, 0);
    }
}

/// The club of a person's contract, as `CONTRACT_MANAGER::get_contract` leaves it in `G_CONTRACT_LOOKUP_RESULT`:
/// none or a type-5 contract gives no club.
pub(crate) unsafe fn contract_club_of(person: usize) -> usize {
    let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    write_usize(G_CONTRACT_LOOKUP_RESULT, c.addr());
    if !c.is_null() && c.contract_type() & 0x3f != 5 {
        club_at((c.club() as i32) as isize).0
    } else {
        0
    }
}

/// Rule: an empty row across the list.
///
/// The row counter goes up by 1; on the shown page, an empty text cell from x 0x82 to 0x2f8 with the next item
/// number, in the 0xae3184 colour.
pub(crate) unsafe fn tel32_blank_row() {
    tel32_bump_row(1);
    if tel32_shown() {
        let i = tel32_next_item();
        tel32_cell(0x82, 0x2f8, i, job_colour(JOB_COLOUR_3184), TEL32_NO_TEXT);
    }
}

/// Rule: the section heading whose text is already in the screens' text buffer (`JOB_TEXT`).
///
/// The heading item goes into `tel32_credits::HEADING_AREA` (kind 0x10, colours 0xae31aa, 0xaebdec and 0xaebdea):
/// flags 0x2002 when the item number equals screen parameter 0 and the page is the shown one (the selected section
/// [inference]), otherwise flags 2 with the value 0x64 and the page in the high word over the row counter in the low
/// one. On the shown page, two cells as wide as the text in font 3 plus 0x82 (from x 0x82), each with the next item
/// number: the text in the 0xaebdec colour, then an empty one with style 0xc400 and that colour as its first colour.
/// The row counter goes up by 2.
pub(crate) unsafe fn tel32_heading() {
    let shown = tel32_shown();
    let bdea = job_colour(JOB_COLOUR_BDEA);
    let bdec = job_colour(JOB_COLOUR_BDEC);
    let c31aa = job_colour(JOB_COLOUR_31AA);
    let heading_area = read_u16(tel32_credits::HEADING_AREA) as u32;
    let text = JOB_TEXT as u32;
    if read_u32(tel32_credits::ITEM) as usize == get_screen_param(0) && shown {
        covered!("tel32_1 the selected heading");
        tel32_item([0x2002, 0, 0, 0, 0, 0, 0, 0x10, c31aa, bdec, 1, 1, bdea, text, 0, 0, 0, heading_area]);
    } else {
        let value = (read_u16(tel32_credits::PAGE) as u32) << 16 | read_u16(tel32_credits::ROW) as u32;
        tel32_item([2, 0, 0, 0, 0, 0, 0, 0x10, c31aa, bdec, 1, 1, bdea, text, 0, 0x64, value, heading_area]);
    }
    if shown {
        // the original passes the whole dword at 0xae1870 here; Guio::init keeps the word
        let area = read_u32(tel32_credits::AREA);
        let x2 = (exe::font_text_width(3usize, JOB_TEXT as *const u8) as u32).wrapping_add(0x82);
        let i = tel32_next_item();
        tel32_item([1, 0x82, 0, x2, 0, 0, i, 1, 0, 0, 1, 3, bdec, text, 0, 0, 0, area]);
        let x2 = (exe::font_text_width(3usize, JOB_TEXT as *const u8) as u32).wrapping_add(0x82);
        let i = tel32_next_item();
        tel32_item([1, 0x82, 0, x2, 0, 0, i, 0xc400, bdec, 0, 0, 0, 0, 0, 0, 0, 0, area]);
    }
    tel32_bump_row(2);
}

/// Rule: a row with a name on the left and the role (already in `JOB_TEXT`) on the right.
///
/// The row counter goes up by 1; on the shown page, the name from x 0x82 to 0x1b8 (0xae3184 colour) with the current
/// item number, without taking it, and the role from 0x1c2 to 0x2f8 (0xaebdde colour) taking it.
pub(crate) unsafe fn tel32_role_row(name: u32) {
    tel32_bump_row(1);
    if tel32_shown() {
        tel32_cell(0x82, 0x1b8, read_u32(tel32_credits::ITEM), job_colour(JOB_COLOUR_3184), name);
        let i = tel32_next_item();
        tel32_cell(0x1c2, 0x2f8, i, job_colour(TEL32_COLOUR_BDDE), JOB_TEXT as u32);
    }
}

/// A role row whose role is translated (`langlib_sprintf` of the role text, no arguments).
pub(crate) unsafe fn tel32_translated_role_row(role: u32, name: u32) {
    comp_lang_format()(JOB_TEXT, role as usize);
    tel32_role_row(name);
}

/// Rule: a list of names in two columns.
///
/// The column starts at 0. For each name, on the shown page: in the left column the name goes from x 0x82 to 0x1b8
/// with the current item number (not taken), in the right one from 0x1c2 to 0x2f8 taking it (0xae3184 colour), and
/// the column flips (1 − column). Then, when the column is 0, the row counter goes up by 1: once per two names on the
/// shown page, but once per name on the other one (the column does not flip there; a quirk of the original: the
/// row values of the hidden page's later headings do not match the rows the page has when shown [inference]).
pub(crate) unsafe fn tel32_name_columns(names: &[u32]) {
    write_u32(tel32_credits::COLUMN, 0);
    for &name in names {
        if tel32_shown() {
            if read_u32(tel32_credits::COLUMN) == 0 {
                tel32_cell(0x82, 0x1b8, read_u32(tel32_credits::ITEM), job_colour(JOB_COLOUR_3184), name);
            } else {
                let i = tel32_next_item();
                tel32_cell(0x1c2, 0x2f8, i, job_colour(JOB_COLOUR_3184), name);
            }
            write_u32(tel32_credits::COLUMN, 1u32.wrapping_sub(read_u32(tel32_credits::COLUMN)));
        }
        if read_u32(tel32_credits::COLUMN) == 0 {
            tel32_bump_row(1);
        }
    }
}

/// Rule: a two-column name list that ended on the right-hand column is closed: the half row counts.
///
/// When `tel32_credits::COLUMN` is not 0 (the last name went into the left column on the shown page), the item
/// number goes up by 1 and the row counter by 1; otherwise nothing.
pub(crate) unsafe fn credits_end_name_columns() {
    if read_u32(tel32_credits::COLUMN) != 0 {
        covered!("tel33_1 the half row closed");
        tel32_next_item();
        tel32_bump_row(1);
    }
}

/// Rule: a credits row with both texts given (a name or a role translated into one of the text buffers).
///
/// As `tel32_role_row`: the row counter goes up by 1; on the shown page, `name` from x 0x82 to 0x1b8 (0xae3184
/// colour) with the current item number, without taking it, and `role` from 0x1c2 to 0x2f8 (0xaebdde colour)
/// taking it.
pub(crate) unsafe fn credits_text_row(name: u32, role: u32) {
    tel32_bump_row(1);
    if tel32_shown() {
        tel32_cell(0x82, 0x1b8, read_u32(tel32_credits::ITEM), job_colour(JOB_COLOUR_3184), name);
        let i = tel32_next_item();
        tel32_cell(0x1c2, 0x2f8, i, job_colour(TEL32_COLOUR_BDDE), role);
    }
}

/// The text buffer of the screens formatted from `format` translated to the game's language.
pub(crate) unsafe fn tel34_lang_text(format: usize) {
    let f: Tel34FormatFn = core::mem::transmute(FN_LANGLIB_SPRINTF);
    f(JOB_TEXT, format);
}

/// The text buffer of the screens formatted from `format` as it is (not translated).
pub(crate) unsafe fn tel34_crt_text(format: usize) {
    let f: Tel34FormatFn = core::mem::transmute(FN_CRT_SPRINTF);
    f(JOB_TEXT, format);
}

/// Copies a C string into the screens' text buffer (the original's inline `strcpy`).
pub(crate) unsafe fn tel34_copy_to_text(src: usize) {
    tel26_copy_text(JOB_TEXT as *mut u8, src as *const u8);
}

/// The row counter moves on by one; returns the row before.
pub(crate) unsafe fn tel34_credits_next_row() -> u32 {
    let row = read_u32(CREDITS_ROW);
    write_u32(CREDITS_ROW, row.wrapping_add(1));
    row
}

/// A text row cell of the list: `Display::add_item` of kind 1 from x1 to x2 at `row`, flags 1, font 3, the colour
/// word at `colour` and the text, in the list area.
pub(crate) unsafe fn tel34_credits_cell(x1: u32, x2: u32, row: u32, colour: usize, text: usize) {
    tel26_add_item(current_human_display(), [1, x1, 0, x2, 0, 0, row, 1, 0, 0, 1, 3, job_colour(colour), text as u32, 0, 0, 0,
        read_u16(CREDITS_AREA) as u32]);
}

/// An empty row across the list (the row counter moves on).
pub(crate) unsafe fn tel34_credits_blank_row() {
    let row = tel34_credits_next_row();
    tel34_credits_cell(0x82, 0x2f8, row, JOB_COLOUR_3184, 0xad9c64);
}

/// The heading rows of a section (the text buffer): the text as wide as it is in font 3 (`font_text_width`) from
/// 0x82 in the heading colour [0xaebdec], and under it a line item (flags 0xc400) as wide; the row counter moves on
/// by two.
pub(crate) unsafe fn tel34_credits_title_rows() {
    let width = exe::font_text_width(3usize, JOB_TEXT as *const u8) as u32;
    let row = tel34_credits_next_row();
    tel34_credits_cell(0x82, width.wrapping_add(0x82), row, JOB_COLOUR_BDEC, JOB_TEXT);
    let width = exe::font_text_width(3usize, JOB_TEXT as *const u8) as u32;
    let row = tel34_credits_next_row();
    tel26_add_item(current_human_display(), [1, 0x82, 0, width.wrapping_add(0x82), 0, 0, row, 0xc400, job_colour(JOB_COLOUR_BDEC),
        0, 0, 0, 0, 0, 0, 0, 0, read_u16(CREDITS_AREA) as u32]);
}

/// The gap before a credits section: the line counter moves on and, on the page shown, an empty row.
pub(crate) unsafe fn tel34_credits_section_gap() {
    tel34_credits_line_add(1);
    if tel34_credits_on_page() {
        tel34_credits_blank_row();
    }
}

/// The section's entry in the "View" popup (the text buffer): ticked (0x2002, no id) when the list's selected row
/// (screen parameter 0) is the current row and the page is the one shown; otherwise an item of kind 2 with id 100
/// and the value page << 16 | line (what `credits_screen_handle_action` opens). Both in the popup colours.
pub(crate) unsafe fn tel34_credits_popup_entry() {
    let selected = get_screen_param(0) as u32;
    let ticked = read_u32(CREDITS_ROW) == selected && tel34_credits_on_page();
    let (kind, id, value) = if ticked {
        (0x2002, 0, 0)
    } else {
        (2, 0x64, (read_u16(CREDITS_PAGE) as u32) << 16 | read_u16(CREDITS_LINE) as u32)
    };
    tel26_add_item(current_human_display(), [kind, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), job_colour(JOB_COLOUR_BDEC), 1, 1,
        job_colour(JOB_COLOUR_BDEA), JOB_TEXT as u32, 0, id, value, read_u16(CREDITS_POPUP_AREA) as u32]);
}

/// The heading of a section (the text buffer): on the page shown the heading rows; the line counter moves on by two.
pub(crate) unsafe fn tel34_credits_heading() {
    if tel34_credits_on_page() {
        tel34_credits_title_rows();
    }
    tel34_credits_line_add(2);
}

/// A name of a two-column section: on the page shown it goes in the left column (0x82..0x1b8) or in the right one
/// (0x1c2..0x2f8, the row counter moving on), and the column flips. Then, for any name but the last, the line counter
/// moves on when the flag is back on the left column; after the last one an unpaired left name closes its row (the
/// row counter moves on) and the line counter moves on.
pub(crate) unsafe fn tel34_credits_column_name(name: usize, last: bool) {
    if tel34_credits_on_page() {
        if read_u32(CREDITS_RIGHT_COLUMN) == 0 {
            tel34_credits_cell(0x82, 0x1b8, read_u32(CREDITS_ROW), JOB_COLOUR_3184, name);
        } else {
            let row = tel34_credits_next_row();
            tel34_credits_cell(0x1c2, 0x2f8, row, JOB_COLOUR_3184, name);
        }
        write_u32(CREDITS_RIGHT_COLUMN, 1u32.wrapping_sub(read_u32(CREDITS_RIGHT_COLUMN)));
    }
    let right = read_u32(CREDITS_RIGHT_COLUMN) != 0;
    if !last {
        if !right {
            tel34_credits_line_add(1);
        }
        return;
    }
    if right {
        tel34_credits_next_row();
    }
    tel34_credits_line_add(1);
}

/// A name and its role (the role already in the text buffer): the line counter moves on and, on the page shown, the
/// name on the left (0x82..0x1b8) and the role on the right (0x1c2..0x2f8, in the role colour [0xaebdde]); the row
/// counter moves on.
pub(crate) unsafe fn tel34_credits_name_and_role(name: usize) {
    tel34_credits_line_add(1);
    if tel34_credits_on_page() {
        tel34_credits_cell(0x82, 0x1b8, read_u32(CREDITS_ROW), JOB_COLOUR_3184, name);
        let row = tel34_credits_next_row();
        tel34_credits_cell(0x1c2, 0x2f8, row, CREDITS_ROLE_COLOUR, JOB_TEXT);
    }
}

/// A text by the game's language (`langlib_get_language`): 2 French, 4 Italian, 5 Spanish, 6 Danish, 9 Portuguese,
/// 10 Swedish, any other English (the original's jump table over language − 2).
pub(crate) unsafe fn tel34_by_language(english: usize, french: usize, italian: usize, spanish: usize, danish: usize, portuguese: usize, swedish: usize) -> usize {
    match exe::langlib_get_language() {
        2 => french,
        4 => italian,
        5 => spanish,
        6 => danish,
        9 => portuguese,
        10 => swedish,
        _ => english,
    }
}

/// The scrolling list of a credits-like screen: the rows per page = `height` / font 3's height (signed), kept in
/// [0xae1872], and the list area (x1 0x6e, y1 `y1`, to 0x30c/0x217, one column, `rows`, 2, no owner) in [0xae1870].
pub(crate) unsafe fn tel34_credits_list_area(height: i32, y1: u32) {
    let rows = height.wrapping_div(exe::font_get_height(3)) as u8;
    write_u8(CREDITS_AREA_ROWS, rows);
    let area = tel26_add_area(current_human_display(), [0x6e, y1, 0x30c, 0x217, 1, 0, rows as u32, 0, 2, 0, u32::MAX]);
    write_u16(CREDITS_AREA, area as u16);
}

/// The screen title of the credits and web sites screens: the game title (`get_game_title` 0x839570) in the heading
/// colours 0 and 1 (`get_heading_colour`).
pub(crate) unsafe fn tel34_game_title() {
    let title: unsafe extern "C" fn() -> usize = core::mem::transmute(FN_GET_GAME_TITLE);
    let text = title();
    let second = exe::get_heading_colour(1) as u32;
    let first = exe::get_heading_colour(0) as u32;
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, first, second, text);
}

/// After the list is built: the list scrolls to the selected row (screen parameter 0, `Area::set_scroll_row`) when
/// there is an area, and parameter 1 keeps the area index.
pub(crate) unsafe fn tel34_credits_list_done() {
    let selected = get_screen_param(0);
    let area = read_u16(CREDITS_AREA) as i16;
    if area != -1 {
        exe::area_set_scroll_row(display_area_at(current_human_display(), area), selected);
    }
    set_screen_param(1, area as isize as usize);
}

/// A web site row (its label already in the text buffer): the address across the list (0x82..0x2f8) and the label
/// on the right (0x1c2..0x2f8, in the role colour); the row counter moves on.
pub(crate) unsafe fn tel34_web_site_row(address: usize) {
    tel34_credits_cell(0x82, 0x2f8, read_u32(CREDITS_ROW), JOB_COLOUR_3184, address);
    let row = tel34_credits_next_row();
    tel34_credits_cell(0x1c2, 0x2f8, row, CREDITS_ROLE_COLOUR, JOB_TEXT);
}

/// A cash amount stepped down once in mode 4 (`Cash::step_down`).
pub(crate) unsafe fn cs_step_down(amount: i32) -> u32 {
    let f: unsafe extern "thiscall" fn(*mut i32, u32) -> u32 = core::mem::transmute(CS_FN_CASH_STEP_DOWN);
    let mut cash = amount;
    f(&mut cash, 4)
}

/// `Cash::format_display_amount` of a cash dword into `TS_TEXT` (2000 bytes).
pub(crate) unsafe fn cs_cash_display(cash: *mut u32, mode: u32) {
    let f: unsafe extern "thiscall" fn(*mut u32, usize, u32, u32) = core::mem::transmute(CS_FN_CASH_FORMAT_DISPLAY);
    f(cash, TS_TEXT, 0x7d0, mode);
}

/// `contract_screens_add_disabled_menu_item` by address (it is hooked).
pub(crate) unsafe fn contract_screens_add_disabled_menu_item_by_address(area: u32, text: u32) {
    exe::contract_screens_add_disabled_menu_item(G_SCREEN_MANAGER, area, text);
}

/// Rows are added only when the sections' page is the page shown.
pub(crate) unsafe fn tel32_shown() -> bool {
    read_u16(tel32_credits::PAGE) == read_u16(tel32_credits::SHOWN_PAGE)
}

/// Takes the next item number (`tel32_credits::ITEM`, then one more).
pub(crate) unsafe fn tel32_next_item() -> u32 {
    let i = read_u32(tel32_credits::ITEM);
    write_u32(tel32_credits::ITEM, i.wrapping_add(1));
    i
}

/// Moves the row counter on by `k`.
pub(crate) unsafe fn tel32_bump_row(k: u16) {
    write_u16(tel32_credits::ROW, read_u16(tel32_credits::ROW).wrapping_add(k));
}

/// `Display::add_item` (0x55f990) on the current slot's Display: the 18 fields.
pub(crate) unsafe fn tel32_item(f: [u32; 18]) {
    tel26_add_item(screens_current_display(G_SCREEN_MANAGER), f);
}

/// A text cell of the rows' area: x from `x1` to `x2`, item `item`, font 3, colour `colour`, text `text`.
pub(crate) unsafe fn tel32_cell(x1: u32, x2: u32, item: u32, colour: u32, text: u32) {
    tel32_item([1, x1, 0, x2, 0, 0, item, 1, 0, 0, 1, 3, colour, text, 0, 0, 0, tel32_area()]);
}

/// The page being built is the page shown.
pub(crate) unsafe fn tel34_credits_on_page() -> bool {
    read_u16(CREDITS_PAGE) == read_u16(CREDITS_SHOWN_PAGE)
}

/// The line counter moves on by `n` (a word).
pub(crate) unsafe fn tel34_credits_line_add(n: u16) {
    write_u16(CREDITS_LINE, read_u16(CREDITS_LINE).wrapping_add(n));
}

/// The rows' area, zero-extended (the original pushes the register with only its low half loaded; `Guio::init`
/// keeps the word).
pub(crate) unsafe fn tel32_area() -> u32 {
    read_u16(tel32_credits::AREA) as u32
}

/// The news type of a contract news item that comes with a transfer offer.
pub(crate) const NEWS_TYPE_OFFER_ACCEPTED: u32 = 0xfa4;

/// contract_screens.cpp's path, for the assert box.
pub(crate) const CS_CPP: usize = 0x9afc8c;

/// `CONTRACT_MANAGER::get_offer_job(staff)` (0x4d44e0, thiscall on the contract manager): the job the club would offer.
pub(crate) const CS_FN_GET_OFFER_JOB: usize = 0x4d44e0;

/// `CONTRACT_MANAGER::find_talks_record(staff, manager, club)` (0x4ca2a0, thiscall): the talks record, or 0.
pub(crate) const CS_FN_FIND_TALKS_RECORD: usize = 0x4ca2a0;

/// `CONTRACT_MANAGER::add_talks_record(staff, manager, club, role)` (0x4c9f60, thiscall).
pub(crate) const CS_FN_ADD_TALKS_RECORD: usize = 0x4c9f60;

/// `staff_can_take_job(job, staff)` (0x4cb990, stdcall; called with the contract manager in ecx).
pub(crate) const CS_FN_STAFF_CAN_TAKE_JOB: usize = 0x4cb990;

/// `get_contract_satisfaction(staff)` (0x866620, thiscall on the contract).
pub(crate) const CS_FN_CONTRACT_SATISFACTION: usize = 0x866620;

/// `shortlist_manager_squad_status_allowed(staff, status)` (0x84b4d0, stdcall; the shortlist manager in ecx).
pub(crate) const CS_FN_SQUAD_STATUS_ALLOWED: usize = 0x84b4d0;

/// `shortlist_manager_squad_status_text(status, buffer, size)` (0x84b330, thiscall on the shortlist manager).
pub(crate) const CS_FN_SQUAD_STATUS_TEXT: usize = 0x84b330;

/// `shortlist_manager_max_fee_for_player_status(club, staff, status)` (0x84b1f0, thiscall on the shortlist manager).
pub(crate) const CS_FN_MAX_FEE_FOR_STATUS: usize = 0x84b1f0;

/// `contract_manager_lowest_wage_demand(staff, club)` (0x4ce450, thiscall on the contract manager).
pub(crate) const CS_FN_LOWEST_WAGE_DEMAND: usize = 0x4ce450;

/// `Cash::step_down(mode)` (0x43f580, thiscall on a cash dword, ret 4): rounds the amount down a step, stores it and
/// returns it.
pub(crate) const CS_FN_CASH_STEP_DOWN: usize = 0x43f580;

/// `HUMAN_MANAGER::get_contract_offer(k)` (0x5fd930, thiscall on the human manager): the human's offer being edited.
pub(crate) const CS_FN_HUMAN_CONTRACT_OFFER: usize = 0x5fd930;

/// The no-op destructor of the edited offer (0x5a53c0, thiscall).
pub(crate) const CS_FN_OFFER_NOOP_DESTRUCTOR: usize = 0x5a53c0;

/// The screens of the set role and offer contract flows (build, input and exit functions).
pub(crate) const CS_BUILD_SET_ROLE: usize = 0x4dd6c0;

pub(crate) const CS_HANDLE_SET_ROLE: usize = 0x4ddf20;

pub(crate) const CS_BUILD_OFFER: usize = 0x4deac0;

pub(crate) const CS_HANDLE_OFFER: usize = 0x4e01e0;

pub(crate) const CS_CLOSE_OFFER: usize = 0x4e06a0;

pub(crate) const CS_BUILD_STAFF_OFFER: usize = 0x4e4820;

pub(crate) const CS_HANDLE_STAFF_OFFER: usize = 0x4e4ea0;

/// The two openers of the offer contract screen (cdecl), by address (they are hooked).
pub(crate) const CS_FN_OPEN_OFFER: usize = 0x4de1f0;

pub(crate) const CS_FN_OPEN_OFFER_FOR_TRANSFER: usize = 0x4de5d0;

/// `transfer_screens_open_offer_changed_screen(offer, serial)` (0x8f2470, cdecl).
pub(crate) const CS_FN_OPEN_OFFER_CHANGED: usize = 0x8f2470;

/// The most a transfer budget shown on the offer screen can be (5,000,000).
pub(crate) const CS_BUDGET_CAP: i32 = 5_000_000;

/// The size of the contract terms the offer screen edits (`STAFF_CONTRACT`, 0x50 bytes).
pub(crate) const CS_TERMS_SIZE: usize = 0x50;

/// `STAFF_CONTRACT`'s bonus amounts (goal, assist, clean sheet: three dwords from +0x10) and its release clause flags
/// (five bytes from +0x1c; index 1 is the minimum fee clause, 4 the manager job clause).
pub(crate) const CS_TERMS_BONUSES: usize = 0x10;

pub(crate) const CS_TERMS_CLAUSES: usize = 0x1c;

/// The amount of the terms capped to the transfer budget of param 0x15 (`STAFF_CONTRACT` +8) [inference: the fee
/// part of the terms].
pub(crate) const CS_TERMS_FEE: usize = 0x8;

/// The job a contract offer is for (byte +0x61 of a CONTRACT_MANAGER offer).
pub(crate) const CS_OFFER_JOB: usize = 0x61;

/// The fees of the release fee popup (9 steps) and the base of the bonus popups.
pub(crate) const CS_RELEASE_FEE_STEPS: [u32; 9] = [0, 100_000, 250_000, 500_000, 1_000_000, 2_500_000, 5_000_000, 10_000_000, 15_000_000];

/// `CONTRACT_MANAGER::will_negotiate_offer(terms)` (0x4d4660, thiscall ret 4): whether the person asked for terms.
pub(crate) const CS_FN_WILL_NEGOTIATE: usize = 0x4d4660;

/// `CONTRACT_MANAGER::prefill_offer_from_pending(terms)` (0x4d4830, thiscall ret 4).
pub(crate) const CS_FN_PREFILL_OFFER: usize = 0x4d4830;

/// `staff_contract_is_clause_allowed(staff, club, clause)` (0x867ad0, cdecl).
pub(crate) const CS_FN_CLAUSE_ALLOWED: usize = 0x867ad0;

/// `staff_contract_get_release_clause_name(clause, buffer)` (0x867a00, cdecl).
pub(crate) const CS_FN_CLAUSE_NAME: usize = 0x867a00;

/// `staff_contract_is_bonus_type_allowed(staff, kind)` (0x867aa0, cdecl).
pub(crate) const CS_FN_BONUS_ALLOWED: usize = 0x867aa0;

/// `staff_contract_get_bonus_type_name(kind, buffer)` (0x8679a0, cdecl).
pub(crate) const CS_FN_BONUS_NAME: usize = 0x8679a0;

/// `Cash::format_display_amount(buffer, size, mode)` (0x450aa0, thiscall on a cash dword, ret 0xc).
pub(crate) const CS_FN_CASH_FORMAT_DISPLAY: usize = 0x450aa0;

/// `Cash::set_from_amount_table(index, table, count)` (0x43f720, thiscall on a cash dword, ret 0xc).
pub(crate) const CS_FN_CASH_FROM_TABLE: usize = 0x43f720;

/// `ScreenManager::add_menu_item(area, text, id, value)` (0x415640, thiscall, ret 0x10).
pub(crate) const CS_FN_ADD_MENU_ITEM: usize = 0x415640;

/// The three tab answers of the offer screen (cdecl(item) -> int), the tab builders (cdecl) and the other screens it
/// opens, by address.
pub(crate) const CS_FN_BASIC_TERMS_ACTION: usize = 0x4e2280;

pub(crate) const CS_FN_BONUSES_ACTION: usize = 0x4e39c0;

pub(crate) const CS_FN_CLAUSES_ACTION: usize = 0x4e4160;

pub(crate) const CS_FN_BUILD_BASIC_TERMS_TAB: usize = 0x4e06e0;

pub(crate) const CS_FN_BUILD_BONUSES_TAB: usize = 0x4e30f0;

pub(crate) const CS_FN_OPEN_STAFF_OFFER: usize = 0x4e4730;

pub(crate) const CS_FN_RULES_SCREEN: usize = 0x77b700;

pub(crate) const CS_FN_REMOVE_EXCHANGE_PLAYER: usize = 0x8f2870;

pub(crate) const CS_FN_BID_CONFIRM: usize = 0x8ef1e0;

/// The qsort comparers that order a club's squad for the squad popup.
pub(crate) const CS_SORT_BY_APPEARANCES: usize = 0x4e5850;

pub(crate) const CS_SORT_BY_SURNAME: usize = 0x53ff60;

/// `STAFF_CONTRACT::set_bonus(kind, amount)`, `remove_bonus(kind)`, `add_release_clause(clause)` and
/// `remove_release_clause(clause)` (0x85f7d0, 0x85f7f0, 0x85f7a0, 0x85f7e0; thiscall on the terms).
pub(crate) const CS_FN_SET_BONUS: usize = 0x85f7d0;

pub(crate) const CS_FN_REMOVE_BONUS: usize = 0x85f7f0;

pub(crate) const CS_FN_ADD_RELEASE_CLAUSE: usize = 0x85f7a0;

pub(crate) const CS_FN_REMOVE_RELEASE_CLAUSE: usize = 0x85f7e0;

/// The most a release fee can be raised to with "+" (below 50,000,000).
pub(crate) const CS_RELEASE_FEE_TOP: i32 = 50_000_000;

/// `STAFF_CONTRACT::format_wage_text(buffer, mode, style)` (0x85e750, thiscall on the terms, ret 0xc).
pub(crate) const CS_FN_FORMAT_WAGE_TEXT: usize = 0x85e750;

/// `STAFF_CONTRACT::format_expiry_text(buffer, style)` (0x85fa30, thiscall on the terms, ret 8).
pub(crate) const CS_FN_FORMAT_EXPIRY_TEXT: usize = 0x85fa30;

/// `STAFF_CONTRACT::increase_wage_step()` and `decrease_wage_step()` (0x867b70, 0x867c50, thiscall on the terms, ret 0).
pub(crate) const CS_FN_INCREASE_WAGE_STEP: usize = 0x867b70;

pub(crate) const CS_FN_DECREASE_WAGE_STEP: usize = 0x867c50;

/// `transfer_manager_build_refusal_reasons_text(club, his club, manager, his manager, x, interest, buffer)` (0x8e4ae0,
/// thiscall on the transfer manager, ret 0x20; the interest a double): whether there are reasons, written to `buffer`.
pub(crate) const CS_FN_REFUSAL_REASONS: usize = 0x8e4ae0;

/// `Date::get_short_date_string(buffer, size, flags)` (0x54a650, thiscall on the date, ret 0xc).
pub(crate) const CS_FN_SHORT_DATE: usize = 0x54a650;

/// `date_get_short_month_name(month)` (0x54d210, cdecl; only the low byte of the month is read): "Jan".."Dec".
pub(crate) const CS_FN_SHORT_MONTH_NAME: usize = 0x54d210;

/// The doubles 2500.0 and the float 0.001 of the basic terms tab.
pub(crate) const CS_F64_2500: usize = 0x9690b0;

pub(crate) const CS_F32_0_001: usize = 0x9690c0;

/// The answers of the basic terms tab that only change the human's terms or the screen params (the wage and fee
/// back to the request, a popup step, the expiry back to the request, the public offer, the job, part time, the
/// transfer date): side by side in the comparer.
pub(crate) const CS_BASIC_PARAM_ANSWERS: [u16; 10] = [6, 7, 8, 0xb, 0x16, 0x19, 0x1a, 0x1c, 0x1e, 0x30];

/// A screen builder (cdecl, no arguments).
pub(crate) type ContractScreenFn = unsafe extern "C" fn();

/// `background_set_staff_picture` (0x41c790, cdecl(person, 0, 0)).
pub(crate) type StaffPictureFn = unsafe extern "C" fn(usize, u32, u32);

/// `News::set_item_slot` (0x786d30, thiscall(news, manager, id, slot, value, copy) ret 0x14).
pub(crate) type NewsSlotFn = unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, usize) -> u32;

/// The widget ids of the transfer request screen and of the terminate contract screen.
pub(crate) const REQUEST_YES: u16 = 0x20;

pub(crate) const REQUEST_NO: u16 = 0x21;

pub(crate) const REQUEST_BACK: u16 = 0x22;

pub(crate) const TERMINATE_YES: u16 = 1;

pub(crate) const TERMINATE_NO: u16 = 2;

/// The staff contract offer screen's "Offer" and "Back" widgets.
pub(crate) const OFFER_SEND: u16 = 0x14;

pub(crate) const OFFER_BACK: u16 = 0x15;

/// The local transfer offer the staff contract offer builds (TRANSFER_OFFER, 0x41 bytes) and the date after it.
#[repr(C, align(4))]
pub(crate) struct StaffOfferLocal(pub(crate) [u8; 0x44]);

/// The colour word of the role column (0xaebdde).
pub(crate) const TEL32_COLOUR_BDDE: usize = 0xaebdde;

/// The empty text of the exe (0xad9c64).
pub(crate) const TEL32_NO_TEXT: u32 = 0xad9c64;

/// The Testing names (exe texts), in the order of the original.
pub(crate) const TEL32_TESTING_NAMES: [u32; 32] = [
    0x9b18d0, // Dan Gravell
    0x9b18c0, // Glenn Wakeford
    0x9b18a4, // Eamonn 'Whitewashman' Kenny
    0x9b1898, // John Holt
    0x9b1888, // Stephen Norrie
    0x9b187c, // Chris Lake
    0x9b1868, // Rohit Alimchandani
    0x9b185c, // Tom Sheldon
    0x9b1850, // Alex Bell
    0x9b182c, // Stuart 'DisplacedSeagull' Fairway
    0x9b181c, // Steven Buckley
    0x9b1810, // Bambo Akani
    0x9b17fc, // Franck Zuffellato
    0x9b1898, // John Holt
    0x9b17ec, // Mark Cosgrove
    0x9b17dc, // Peter Blackwell
    0x9b17c0, // Chris 'SpursLodger' Bedwell
    0x9b17b4, // Will Tully
    0x9b179c, // Paul 'Keeper' Stewart
    0x9b1790, // James Yorke
    0x9b1780, // Adrian Osborne
    0x9b1770, // Andrew Jolly
    0x9b1758, // Jared 'Hoopy' McGeehan
    0x9b1748, // Marco Camilleri
    0x9b1738, // Graham Stark
    0x9b172c, // Robert Pang
    0x9b171c, // David Stainer
    0x9b1708, // Craig 'Ter' Hunter
    0x9b16f0, // Gordon 'Gudge' Dunbar
    0x9b16d4, // Jeroen 'Reddiablo' Thijssen
    0x9b16bc, // Francis 'Teaboy' Cole
    0x9b16b0, // Daniel Nash
];

/// The Feral rows: (role, name), exe texts, in the order of the original.
pub(crate) const TEL32_FERAL_ROLES: [(u32, u32); 1] = [
    (0x9b18f4, 0x9b18e4), // Producer: David Stephen
];

/// The Eidos rows: (role, name), exe texts, in the order of the original.
pub(crate) const TEL32_EIDOS_ROLES: [(u32, u32); 24] = [
    (0x9b18f4, 0x9b1b70), // Producer: Andrew Norman
    (0x9b1b5c, 0x9b1b50), // Associate Producer: Steve Didd
    (0x9b1b3c, 0x9b1b28), // Marketing Manager: Jonathan Rosenblatt
    (0x9b1b1c, 0x9b1b0c), // PR Manager: Steve Starvis
    (0x9b1af4, 0x9b1ae4), // Localisation QA Manager: Marco Vernetti
    (0x9b1ac8, 0x9b1ab8), // Localisation Lead Tester: Arnaud Messager
    (0x9b1aa8, 0x9b1a98), // Localisation: Maike Köhler
    (0x9b1aa8, 0x9b1a88), // Localisation: Iain Willows
    (0x9b1a6c, 0x9b1a5c), // Group Localisation Manager: Flavia Timiani
    (0x9b1118, 0x9b1a4c), // QA Manager: Chris Rowley
    (0x9b1a34, 0x9b1a28), // Assistant QA Manager: John Ree
    (0x9b1a18, 0x9b1b50), // QA Co-ordinator: Steve Didd
    (0x9b1a14, 0x9b1a08), // QA: Ron Jackson
    (0x9b1a14, 0x9b19f8), // QA: Tony Peterken
    (0x9b1a14, 0x9b19e4), // QA: Scott Sutherland
    (0x9b1a14, 0x9b19d8), // QA: Joe Neate
    (0x9b1a14, 0x9b19cc), // QA: Ray Mullen
    (0x9b1a14, 0x9b19c0), // QA: Steve Addis
    (0x9b1a14, 0x9b19b0), // QA: Marlon Grant
    (0x9b1a14, 0x9b19a0), // QA: Lawrence Day
    (0x9b1a14, 0x9b1990), // QA: Jon Redington
    (0x9b1a14, 0x9b1980), // QA: Markus Poltorp
    (0x9b1970, 0x9b1960), // MC Co-ordinator: Jason Walker
    (0x9b1954, 0x9b1944), // Mastering: Phil Spencer
];

/// The Translation rows: (role, name), exe texts, in the order of the original.
pub(crate) const TEL32_TRANSLATION_ROLES: [(u32, u32); 13] = [
    (0x9b1c38, 0x9b1c30), // Danish: Babel
    (0x9b1c38, 0x9b1c20), // Danish: Peter Højholt
    (0x9b1c18, 0x9b1c00), // Dutch: Ruud van de Moosdijk
    (0x9b1bf8, 0x9b1570), // French: Emmanuel Rochard
    (0x9b1bf0, 0x9b1be4), // German: Thomas Knie
    (0x9b1bf0, 0x9b1bd4), // German: Alexander Barth
    (0x9b1bcc, 0x9b14f4), // Italian: Alberto Scotta 'Panoz'
    (0x9b1bc0, 0x9b1458), // Norwegian: Vegard Rønningen
    (0x9b1bc0, 0x9b146c), // Norwegian: Tore Ludvigsen
    (0x9b1bb4, 0x9b1418), // Portuguese: 'Zé' Guilherme Chieira
    (0x9b1bac, 0x9b1358), // Spanish: Iván Abella Villar
    (0x9b1ba4, 0x9b1328), // Swedish: Jan Juhlin
    (0x9b1ba4, 0x9b1b94), // Swedish: David Landerin
];

/// The Additional Research names (exe texts), in the order of the original (698).
pub(crate) const ADDITIONAL_RESEARCH_NAMES: [u32; 698] = [
    0x9b4b0c, // Tony Persoglia
    0x9b4afc, // Jeremy Lowe
    0x9b4aec, // Andrew Marler
    0x9b4adc, // Simon Treanor
    0x9b4ac8, // Nick Vatmanidis
    0x9b4ab8, // Jerko Zuvela
    0x9b4aac, // Albert Au
    0x9b4a98, // Matthew Cheeseman
    0x9b4a88, // Wayne Dingle
    0x9b4a78, // James Doughty
    0x9b4a6c, // Derek Lau
    0x9b4a5c, // Stephen Lewis
    0x9b4a4c, // Minkey Sellers
    0x9b4a38, // Michelle Taplin
    0x9b4a24, // Bailey Thorndyke
    0x9b4a14, // Clinton Tolkin
    0x9b4a04, // Mark Thompson
    0x9b49f0, // Albert 'Ali' Kropf
    0x9b49d4, // Matthias 'Opul' Obermayr
    0x9b49bc, // Bernd 'BTC' Kronawetter
    0x9b49a0, // Alexander 'Shodan' Schnider
    0x9b4984, // Alexander 'Shadohawk' Muhr
    0x9b4974, // Tarek Mustafa
    0x9b4958, // Thomas 'Divan' Kutschera
    0x9b4940, // Yannick Boudringhien
    0x9b492c, // Christophe Boulaert
    0x9b491c, // Olivier Brackx
    0x9b490c, // Sepp De Coninck
    0x9b48fc, // Bart Deckers
    0x9b48ec, // Cédric Dejonghe
    0x9b48d8, // Petro Depraetere
    0x9b48cc, // Kim Depree
    0x9b48b8, // Diederik Dewaele
    0x9b48a8, // Pascal D'Helft
    0x9b4898, // Narcisse Genin
    0x9b4884, // Stéphane Glowacki
    0x9b4870, // Remi Keirsebilck
    0x9b4864, // Juan Hamers
    0x9b4854, // Benoît Humbled
    0x9b4840, // Emmanuel Hoornaert
    0x9b4830, // John Jespers
    0x9b4824, // Paul Jacobs
    0x9b4818, // Tom Jottier
    0x9b4804, // Christoph Lavens
    0x9b47f4, // Thierry Lepage
    0x9b47e0, // Olivier Mathurin
    0x9b47cc, // Jessie Meuleman
    0x9b47c0, // Yves Meyen
    0x9b47b0, // Nicolas Muller
    0x9b47a0, // Frank Nielandt
    0x9b4790, // Jonas Onkelinx
    0x9b4784, // Bart Packo
    0x9b4770, // David Philippart
    0x9b4760, // Jeroen Pots
    0x9b474c, // Frederik Rambout
    0x9b4740, // Tim Rollé
    0x9b4730, // Dave Stynen
    0x9b4720, // Luc Sysmans
    0x9b470c, // Vincent Tibackx
    0x9b46f8, // Jan Valkenborgh
    0x9b46e4, // Ronan Van Aubel
    0x9b46cc, // Franky Vandenberghe
    0x9b46b4, // Yves Van Den Berghe
    0x9b469c, // Jom Vangoidtsenhoven
    0x9b4688, // Kris Vanmunster
    0x9b4674, // Tim Van Speybroeck
    0x9b4660, // Pieter Vantieghem
    0x9b464c, // Mathias Vissers
    0x9b463c, // Stefan Wauters
    0x9b462c, // Gilles Wilmart
    0x9b4618, // Evert Winkelmans
    0x9b4608, // Marc Van Lierop
    0x9b45f0, // Thiago 'Azulas' de Rose
    0x9b45d8, // Paulo 'PMLF' Freitas
    0x9b45c8, // Thiago Kuceki
    0x9b45b4, // Rodrigo 'Biro' Osna
    0x9b4598, // Thiago 'Lee Hodges' Porto
    0x9b458c, // Alex Amaral
    0x9b4580, // Igor Jordão
    0x9b4570, // Gustavo Militão
    0x9b4564, // Filipe Melo
    0x9b4554, // Alan Capitanio
    0x9b4548, // Genaro Vaz
    0x9b453c, // Léo Fauth
    0x9b452c, // Matheus Antunes
    0x9b451c, // Eduardo Pacheco
    0x9b450c, // Juliano Lorenz
    0x9b4500, // Luiz Guidi
    0x9b44f0, // Daniel Cardoso
    0x9b44e4, // Liu Xing
    0x9b44d8, // Wang Zhixin
    0x9b44cc, // Zhang Zhen
    0x9b44bc, // Zhou Zhanxiang
    0x9b44ac, // Zhuang Xiaofei
    0x9b4494, // Matthew Li (Hong Kong)
    0x9b4484, // Joso Puljic
    0x9b4470, // Berislav Jagustin
    0x9b4464, // Alen Brkic
    0x9b4450, // Tomislav Jakopanec
    0x9b4440, // Dario Karnik
    0x9b442c, // Antonio Kuzmanic
    0x9b4420, // Orsat Lasic
    0x9b440c, // Ivan Prgomet Selak
    0x9b4400, // Ivan Sunic
    0x9b43f0, // Sinisa Topalov
    0x9b43e0, // Ivan Buconjic
    0x9b43d4, // Nino Burek
    0x9b43c8, // Dario Kozul
    0x9b43b8, // Zoran Mokrovcak
    0x9b43ac, // Josip Music
    0x9b4394, // Jari-Pekka Aarrekivi
    0x9b437c, // Sebastian Aniszewski
    0x9b436c, // Jani Arponen
    0x9b435c, // Mikko Aspelin
    0x9b434c, // Jörgen Bolin
    0x9b4338, // Joakim Eriksson
    0x9b4328, // Thomas Forsten
    0x9b4318, // Kalle Helin
    0x9b4304, // Patrik Herrgård
    0x9b42f4, // Kim Ittonen
    0x9b42e4, // Juho Kauppi
    0x9b42d4, // Juha Korhonen
    0x9b42c0, // Teemu Kärenlampi
    0x9b42b0, // Jarno Laiho
    0x9b429c, // Raine Lindqvist
    0x9b428c, // Antti Luode
    0x9b4280, // Esko Lämsä
    0x9b426c, // Tuomas Makkonen
    0x9b425c, // Jukka Mononen
    0x9b4248, // Andrei Neuvonen
    0x9b4238, // Jari Okkonen
    0x9b4228, // Patrik Olin
    0x9b4218, // Tapani Olkku
    0x9b4204, // Hannu Piiparinen
    0x9b41f4, // Jussi Pirinen
    0x9b41e4, // Johan Prest
    0x9b41d0, // Kimmo Rouhiainen
    0x9b41c0, // Mikko Salonen
    0x9b41a8, // Jari-Pekka Siivonen
    0x9b4198, // Sakari Similä
    0x9b4184, // Mikael Sorjonen
    0x9b4174, // Patrick Suves
    0x9b4164, // Tuomas Vaalto
    0x9b4154, // Risto Kuustera
    0x9b4140, // Arnaud Guelfucci
    0x9b4134, // B@ptiste
    0x9b4128, // B.Chalut
    0x9b411c, // Barthfan
    0x9b410c, // CharlieRoach
    0x9b40f8, // Finton Sirockin
    0x9b40e8, // Forza Tolosa
    0x9b40d8, // Jordan Dessoye
    0x9b40d0, // Romjet
    0x9b40c4, // Striker
    0x9b40b4, // Thomas Croux
    0x9b40a4, // Ronald Demohn
    0x9b4094, // Robert Demohn
    0x9b4084, // Günther Demohn
    0x9b4070, // Alisan Tasdelen
    0x9b405c, // Christian Meyer
    0x9b404c, // Stefan Hack
    0x9b403c, // Wilhelm Döbel
    0x9b4028, // Giorgos Tartaris
    0x9b4018, // Tasos Varsos
    0x9b4000, // Nikolaos Giannoutsos
    0x9b3fec, // Riccardo Accossato
    0x9b3fd8, // Gianpaolo Altamura
    0x9b3fc0, // Ugo 'Grifone' Amoretti
    0x9b3fa4, // Stefano 'Stark' Andreoli
    0x9b3f94, // Luca Argiolas
    0x9b3f84, // Davide Atria
    0x9b3f70, // Stefano Benericetti
    0x9b3f5c, // Pierpaolo Benvoluti
    0x9b3f50, // Luca Biondi
    0x9b3f44, // Andrea Boni
    0x9b3f2c, // Alessandro Borgonovo
    0x9b3f18, // Saverio Bozzolan
    0x9b3f08, // Franco Bramante
    0x9b3ef4, // Daniele Brunengo
    0x9b3edc, // Diego 'El Cholo' Bruno
    0x9b3ecc, // Emanuele Burri
    0x9b3ec0, // Luca Calia
    0x9b3eb4, // Enzo Cameli
    0x9b3ea0, // Alessandro Camera
    0x9b3e88, // Alessandro Cammarota
    0x9b3e74, // Davide Camporese
    0x9b3e64, // Canavese Calcio
    0x9b3e54, // Sabrina Caralli
    0x9b3e44, // Umberto Carbone
    0x9b3e30, // Walter Cavalcante
    0x9b3e20, // Santolo Ceglia
    0x9b3e10, // Fabrizio Celani
    0x9b3df4, // Cristiano 'Zoran' Cinacchio
    0x9b3de8, // Simone Cola
    0x9b3dd4, // Nicola Colasante
    0x9b3dc4, // Ivano Colombo
    0x9b3db4, // Andrea Consonni
    0x9b3da8, // Mario Costa
    0x9b3d98, // Ernesto Covino
    0x9b3d78, // Lorenzo 'Lord Lorenz' Cozzella
    0x9b3d68, // Niccolò Curti
    0x9b3d58, // Daniele Curzi
    0x9b3d44, // Diego Dalla Costa
    0x9b3d20, // Simone 'OlivoAscolano' Dalle Crode
    0x9b3d00, // Severina 'Mrs Panoz' Daniele
    0x9b3ce8, // Alessandro De Cillis
    0x9b3cd8, // Gianni De Gese
    0x9b3cc4, // Vincenzo D'Amico
    0x9b3ca8, // Nicola 'Santos81' D'Egidio
    0x9b3c94, // Enzo Della Morte
    0x9b3c80, // Andrea Della Valle
    0x9b3c6c, // Juza Delle Nuvole
    0x9b3c54, // Alessandro Desiderio
    0x9b3c44, // Franco Di Carlo
    0x9b3c24, // Federico 'Uccello' Di Cesare
    0x9b3c04, // Francesco 'Armida' Di Giovanni
    0x9b3bf4, // Luigi Di Guilmi
    0x9b3bd4, // Cristiano 'Emerald' Dionello
    0x9b3bbc, // Carlo 'Nerone' D'Orazio
    0x9b3bac, // Massimo Fagotto
    0x9b3b9c, // Andrea Ferrarin
    0x9b3b80, // Marco 'SuperMarco' Ferro
    0x9b3b68, // Michele 'Nagol' Fidati
    0x9b3b58, // Luca Filipasso
    0x9b3b48, // Matteo Foresti
    0x9b3b38, // Giorgio Forlano
    0x9b3b28, // Alberto Forte
    0x9b3b18, // Andrea Fortina
    0x9b3b00, // Valerio Francavillese
    0x9b3ae8, // Maria Gemma Galliano
    0x9b3ad4, // Ivan Garberoglio
    0x9b3ac0, // Angelo Gennarelli
    0x9b3aa0, // Fabio 'wrightpatterson' Gentile
    0x9b3a90, // Fabio Germanà
    0x9b3a80, // Fabio Ghiglione
    0x9b3a6c, // Alberto Giacomel
    0x9b3a5c, // Andrea Gilioli
    0x9b3a4c, // Gianpaolo Giolo
    0x9b3a3c, // Tommaso Grandi
    0x9b3a2c, // Luca Graniero
    0x9b3a1c, // Helmut Isella
    0x9b3a0c, // Andrea Ladas
    0x9b39f8, // Raffaele Larovere
    0x9b39e8, // Ettore Leali
    0x9b39cc, // Luca 'Fantaluca' Lezzeni
    0x9b39bc, // Marco Lombardo
    0x9b39ac, // Fabio Malerba
    0x9b399c, // Valerio Malerba
    0x9b398c, // Andrea Manetti
    0x9b397c, // Filippo Marroni
    0x9b396c, // Riccardo Marzi
    0x9b3954, // Guido 'Fox' Marzucchi
    0x9b3940, // Gabriele Mattiussi
    0x9b3928, // Mauro 'Ronald' Mercuri
    0x9b3914, // Emanuele Messina
    0x9b3904, // Nevio Millia
    0x9b38e8, // Marco 'Elvis' Montermini
    0x9b38d0, // Matteo 'Il Moro' Morigi
    0x9b38c0, // Davide Muraro
    0x9b38b4, // Theo Nuvoli
    0x9b389c, // Alvaro 'Alvios' Odierna
    0x9b388c, // Andreas Pagani
    0x9b3874, // Andrea 'Xain' Pandolfo
    0x9b3864, // Michele Panno
    0x9b3854, // Antonio Pantano
    0x9b383c, // Marco 'Panzer' Paone
    0x9b3820, // Stefano 'Bobanteo' Parola
    0x9b3810, // Enzo Petrassi
    0x9b3800, // Ivan Piastra
    0x9b37f4, // Manuel Pini
    0x9b37e4, // Giuseppe Piva
    0x9b37d8, // Mauro Polli
    0x9b37c4, // Daniele Pompignoli
    0x9b37a8, // Giulio 'last_one' Pompili
    0x9b3794, // Alessandro Porta
    0x9b3784, // Enrico Pozzi
    0x9b3768, // Massimo 'Joanlui' Prosperi
    0x9b3754, // Gianluca Purrello
    0x9b3740, // Michele Purrello
    0x9b3730, // Corrado Putzu
    0x9b371c, // Davide Quagliuolo
    0x9b3704, // Salvatore Quattrocchi
    0x9b36f4, // Enrico Rimondot
    0x9b36dc, // Stefania 'Oliva' Rinero
    0x9b36d0, // Bruno Röhl
    0x9b36bc, // Massimo Romagnoli
    0x9b36b0, // Mauro Rosa
    0x9b36a0, // Davide Rotunno
    0x9b3694, // Gianni Sala
    0x9b3680, // Enrico Santonastaso
    0x9b3670, // Marco Santucci
    0x9b3658, // Christian Scarpellini
    0x9b3648, // Daniele Scotta
    0x9b3638, // Virgilio Scotta
    0x9b362c, // Mauro Sgoku
    0x9b361c, // Sergio Solli
    0x9b360c, // David Stefanini
    0x9b35ec, // Giampietro 'Jumpiter' Stocco
    0x9b35d8, // Lorenzo Tamborrino
    0x9b35c4, // Giammarco Tammaro
    0x9b35b4, // Massimo Tavazzi
    0x9b35a4, // Riccardo Tesei
    0x9b3590, // Stefano Tiraferri
    0x9b357c, // Giuseppe Todisco
    0x9b356c, // Claudio Tomaino
    0x9b355c, // Daniele Tosatto
    0x9b3548, // Alessandro Vario
    0x9b3538, // Davide Vecchini
    0x9b3524, // Leonardo Venturelli
    0x9b3508, // Alessandro '8,666' Vetta
    0x9b34f8, // Andrea Vezzoso
    0x9b34e8, // Vanni Volponi
    0x9b34d4, // Enrico Zaccherelli
    0x9b34c4, // Andrew Dolan
    0x9b34b8, // David Owens
    0x9b34a8, // Eamon Donohoe
    0x9b3494, // Conor Fitzgerald
    0x9b3488, // Fran Hoey
    0x9b347c, // Ian Tilson
    0x9b345c, // Johnny Ward (eircomleague.net)
    0x9b344c, // Donal Cahalan
    0x9b343c, // Makoto Karibe
    0x9b3428, // Kenji Fukushima
    0x9b3414, // Takamichi Imazu
    0x9b33fc, // Haruyoshi Kitaguchi
    0x9b33e4, // Masatoshi Shiraishi
    0x9b33d4, // Andrés Aguayo
    0x9b33c0, // Valeria Brabata
    0x9b33b0, // Eddie McMullan
    0x9b33a4, // Mark Hill
    0x9b3398, // Greg Boyd
    0x9b338c, // Tim Duff
    0x9b337c, // David Kennedy
    0x9b3368, // Johnny McKinstry
    0x9b3358, // David Finlay
    0x9b3348, // Stuart Gilmore
    0x9b333c, // Jim Doran
    0x9b332c, // Bertie Fulton
    0x9b331c, // Steven Vennard
    0x9b3308, // Brian McWilliams
    0x9b32fc, // Per Wie
    0x9b32e8, // Cato Engebakken
    0x9b32d8, // Anders Mittet
    0x9b32c8, // Hein Henriksen
    0x9b32b8, // Christian Rein
    0x9b32a8, // Tord Martinsen
    0x9b3298, // Sigurd Eggen
    0x9b3284, // Hans Martin Nakkim
    0x9b3274, // Jørgen Nyhus
    0x9b325c, // Torkel Haugan Hansen
    0x9b324c, // Terje Larsen
    0x9b3234, // Chris Robin Johansen
    0x9b3228, // Jan Moe
    0x9b3218, // Henning Tveit
    0x9b3208, // Jørgen Reitan
    0x9b31f4, // Kristian Råsberg
    0x9b31d8, // Lars Christian Bjørknes
    0x9b31c8, // Vegard Enlid
    0x9b31b8, // Tor Mogstad
    0x9b31a0, // Pål Richard Lilleseth
    0x9b3190, // Trond Alsvik
    0x9b317c, // Bjørnar Thomassen
    0x9b3168, // Alexander Solum
    0x9b3158, // John Slinning
    0x9b314c, // Knut Lima
    0x9b3134, // Steinar Hanche Aker
    0x9b3124, // Espen Johansen
    0x9b3114, // Sandro Kvernmo
    0x9b3100, // Kim Kalden López
    0x9b30f0, // Sandro Kvernmo
    0x9b30e0, // Håvard Valved
    0x9b30d8, // LooZ^
    0x9b30d0, // KDAN
    0x9b30b8, // Krzysztof Bartoszek
    0x9b30a8, // Sir Cycorus
    0x9b3098, // Karol Cygan
    0x9b3088, // Adrian Dampc
    0x9b3074, // Albert Demidowski
    0x9b3060, // Michal Florczyk
    0x9b3048, // Marcin Harasimowicz
    0x9b3038, // Pawel Hochstim
    0x9b3024, // Damian Jamrozik
    0x9b3014, // Dag Janowski
    0x9b3000, // Przemyslaw Kania
    0x9b2ff0, // Wojtek Kuczera
    0x9b2fdc, // Kamil Kurzynoga
    0x9b2fcc, // Tomasz Marek
    0x9b2fb4, // Tadeusz Placzkiewicz
    0x9b2fa4, // Kamil Przegon
    0x9b2f94, // Rafal Rezler
    0x9b2f84, // Bartosz Slaby
    0x9b2f74, // Bartosz Syrek
    0x9b2f60, // Lucas Szaniawski
    0x9b2f4c, // Marek Tuszewicki
    0x9b2f3c, // Piotr Wawulski
    0x9b2f2c, // Marcin Wesolek
    0x9b2f18, // Marcin Wisniewski
    0x9b2f04, // Arkadiusz Wojna
    0x9b2ef4, // Eddy Zielinski
    0x9b2ee0, // Zelislaw Zyzynski
    0x9b2ed0, // Konrad Gawron
    0x9b2ec4, // João Costa
    0x9b2eb4, // André Narciso
    0x9b2ea4, // António Lopes
    0x9b2e94, // António Pedro
    0x9b2e80, // Bruno Magalhães
    0x9b2e70, // Carlos Carloto
    0x9b2e60, // Carlos Couto
    0x9b2e50, // Carlos Pereira
    0x9b2e40, // Carlos Veiga
    0x9b2e30, // Davide Ribeiro
    0x9b2e20, // Duarte Barbosa
    0x9b2e14, // Fábio Poço
    0x9b2e00, // Fernando Veloso
    0x9b2dec, // Gabriel Satxibala
    0x9b2ddc, // Gonçalo Durão
    0x9b2dcc, // João Campos
    0x9b2dbc, // João Inverno
    0x9b2dac, // João Marques
    0x9b2d9c, // Jorge Almeida
    0x9b2d8c, // Luis Filipe
    0x9b2d7c, // Marco Antonio
    0x9b2d68, // Nélson Patriarca
    0x9b2d58, // Nuno Caetano
    0x9b2d48, // Nuno Machado
    0x9b2d3c, // Nuno Lobo
    0x9b2d24, // Paulo Alexandre Pinho
    0x9b2d14, // Paulo Roberto
    0x9b2d00, // Paulo Sacramento
    0x9b2ce8, // Pedro Brandão Correia
    0x9b2ccc, // Pedro Silvério Gonçalves
    0x9b2cb8, // Pedro Marcelino
    0x9b2ca0, // Pedro Miguel Ribeiro
    0x9b2c88, // Pedro Santana Cepeda
    0x9b2c78, // Pedro Sousa
    0x9b2c64, // Ricardo Gouveia
    0x9b2c50, // Ricardo Rodrigues
    0x9b2c40, // Rui Boieiro
    0x9b2c30, // Rui Couceiro
    0x9b2c20, // Rui Malheiro
    0x9b2c10, // Rui Parrula
    0x9b2c04, // Rui Silva
    0x9b2bf4, // Sandro Cruz
    0x9b2be8, // Tiago Cid
    0x9b2bd8, // Tiago Sousa
    0x9b2bc4, // Vasco Ravasqueira
    0x9b2bb0, // Vítor Aires Quintã
    0x9b2b9c, // Andrew Kravtchenko
    0x9b2b88, // Mikhail Chistyakov
    0x9b2b78, // Maxim Chumakov
    0x9b2b64, // Sergey Gerassimov
    0x9b2b54, // Ilia Kovalenko
    0x9b2b40, // Alexandr Kremez
    0x9b2b30, // Alexey Moiseev
    0x9b2b20, // Ivan Rundqvist
    0x9b2b0c, // Vitaly Sklyarov
    0x9b2af8, // Antony Tutunnik
    0x9b2ae8, // Vadim Vasiljev
    0x9b2ad8, // Steven Torney
    0x9b2acc, // Craig Feroz
    0x9b2abc, // Gavin Maxwell
    0x9b2aa8, // Iain Bartholomew
    0x9b2a98, // Paul O'Neill
    0x9b2a88, // Keith Johnston
    0x9b2a78, // Iain Caldwell
    0x9b2a6c, // Jimmy Bell
    0x9b2a60, // Alan Hunter
    0x9b2a54, // Alan Thomas
    0x9b2a44, // Gary Henderson
    0x9b2a30, // Stephen McConnachie
    0x9b2a24, // Ross Wilson
    0x9b2a18, // Allan Ross
    0x9b2a08, // Stuart Douglas
    0x9b29fc, // Scott Airth
    0x9b29ec, // Jamie Pendrigh
    0x9b29dc, // Robin Karfoot
    0x9b29cc, // Craig Buchanan
    0x9b29bc, // Robert Chappell
    0x9b29ac, // Andrew Harkes
    0x9b29a0, // Allan Hosey
    0x9b2990, // David McCann
    0x9b2984, // Paul Ferrie
    0x9b2974, // Jamie MacKay
    0x9b2968, // David Burns
    0x9b2958, // Alan Stewart
    0x9b294c, // Bruce Hale
    0x9b2940, // Jamie Wire
    0x9b292c, // Scottie MacLennan
    0x9b291c, // Osiris Navarro
    0x9b290c, // J.A. Vidania
    0x9b28ec, // Alfredo del Castillo Morgado
    0x9b28dc, // Domingo García
    0x9b28cc, // Eduardo Corral
    0x9b28b8, // David Franco Vilar
    0x9b28a8, // Pablo Blázquez
    0x9b2890, // Raúl Moreno Domínguez
    0x9b287c, // Ross van der Linde
    0x9b2868, // Andreas Koshiaris
    0x9b285c, // Sean Scott
    0x9b2848, // Johan Pettersson
    0x9b2838, // Jonas Dermark
    0x9b2828, // Per Antonsson
    0x9b2814, // Fredrik Gustafsson
    0x9b2804, // Joel Skjutar
    0x9b27f4, // Karl Alfredsson
    0x9b27e0, // Fredrik Ljunggren
    0x9b27d4, // Johan Frost
    0x9b27c0, // Andreas Ljungqvist
    0x9b27ac, // Markus Niklasson
    0x9b279c, // Staffan Runemo
    0x9b2788, // Tobias Andersson
    0x9b2778, // Luigi Riccio
    0x9b2768, // Boris Jelenic
    0x9b2758, // Branko Jelenic
    0x9b2744, // Moreno Invernizzi
    0x9b2734, // Mehmet Erikler
    0x9b2720, // Serdar Yesilyurt
    0x9b270c, // Selçuk Yesilyurt
    0x9b26fc, // Murat Odabasi
    0x9b26ec, // Orhun Çetin
    0x9b26d8, // Inci Tastabanoglu
    0x9b26c4, // Burçay Tiftikçi
    0x9b26b4, // Emre Özcanv
    0x9b26a4, // Hemi Behmoaras
    0x9b2694, // Simge Ölçer
    0x9b2684, // Okan Yurdakul
    0x9b2674, // Tolga Kaprol
    0x9b265c, // Mehmet Hakan Gökdogan
    0x9b264c, // Bülent Çelik
    0x9b2640, // Dogan Evin
    0x9b262c, // Halim Gürvardar
    0x9b261c, // Tahsin Peker
    0x9b2608, // Irfan Saraloglu
    0x9b25fc, // Faik Demir
    0x9b25f0, // Efe Avsar
    0x9b25e4, // Murat Özder
    0x9b25d8, // Onur Eyibas
    0x9b25c8, // Mustafa Budak
    0x9b25bc, // Emir Tonguç
    0x9b25ac, // Baris Konuklar
    0x9b25a0, // Chris Jones
    0x9b2594, // Tony Sharp
    0x9b2584, // Andy Glester
    0x9b2570, // Graham Armsworth
    0x9b2560, // Glenn Watkin
    0x9b2554, // Daniel Fox
    0x9b2544, // Russ Kelsall
    0x9b2534, // Kieran Robson
    0x9b2520, // Martin Pritchard
    0x9b2514, // Ian Evans
    0x9b2504, // Stuart Barnes
    0x9b24f8, // Dave Austin
    0x9b24e8, // John Holding
    0x9b24d8, // Jonathan Maltby
    0x9b24c8, // Alan Biggins
    0x9b24b8, // Robert Trand
    0x9b24ac, // Malc Buss
    0x9b249c, // Rob Fernandes
    0x9b2490, // Antony Addy
    0x9b2480, // James Parker
    0x9b2470, // Darrien Bold
    0x9b245c, // Andrea Thrussell
    0x9b244c, // Wayne Spackman
    0x9b243c, // Graeme Dixon
    0x9b2430, // Robert Rea
    0x9b2420, // Mark Halliwell
    0x9b2410, // Steve Wilkes
    0x9b2404, // Mark Denman
    0x9b23f4, // Daniel Whymark
    0x9b23e4, // Alan Roberts
    0x9b23d4, // Simon Davies
    0x9b23c4, // Steve Baxter
    0x9b23b4, // Mark Buckingham
    0x9b23a4, // Steve Bishop
    0x9b2398, // Jim Wheeler
    0x9b238c, // Steve Rose
    0x9b2380, // Peter Zeeuw
    0x9b236c, // Doug Kirkpatrick
    0x9b235c, // Stuart Townsend
    0x9b2350, // Jeff Trice
    0x9b2344, // Mark Ollard
    0x9b2334, // Colin Hamilton
    0x9b2328, // Peter Gowan
    0x9b231c, // John Lawson
    0x9b2310, // Mark Clough
    0x9b2304, // Steve Hales
    0x9b22f4, // Ian Fitzgerald
    0x9b22e4, // Richard Walker
    0x9b22d0, // David Garthwaite
    0x9b22c4, // Rich Self
    0x9b22b8, // Nick Bell
    0x9b22ac, // Ed Prosser
    0x9b229c, // Andy Medcalf
    0x9b2290, // Jim Bristow
    0x9b2280, // James Gardner
    0x9b2274, // Adam Lees
    0x9b2260, // Steven Petryszyn
    0x9b2250, // Brian Spittles
    0x9b2244, // Matt Peers
    0x9b2238, // Mark Waters
    0x9b222c, // Simon Mills
    0x9b221c, // Paul Edwards
    0x9b220c, // Jonathan Miller
    0x9b2200, // Tommy Rance
    0x9b21f4, // Pete Gresty
    0x9b21e8, // Sam Vardy
    0x9b21dc, // Dai Nelmes
    0x9b21d0, // Bob Fischer
    0x9b21c4, // Chris Rice
    0x9b21b4, // Paul Johnson
    0x9b21a4, // David Moscrop
    0x9b2198, // Kim Griggs
    0x9b218c, // Ian Scholes
    0x9b217c, // Ian Marriott
    0x9b216c, // Gary Spencer
    0x9b215c, // Alex Metcalfe
    0x9b214c, // Damon Farrant
    0x9b2140, // Ian Hesketh
    0x9b2130, // Matt Walklett
    0x9b2120, // Jonathan Oakes
    0x9b210c, // Andrew Stillwell
    0x9b2100, // James Black
    0x9b20f4, // Steve James
    0x9b20e8, // Ray Curry
    0x9b20d8, // Elinor Cooper
    0x9b20c8, // Andrew Hawkins
    0x9b20bc, // Dave Albon
    0x9b20b0, // Nick Clarke
    0x9b20a0, // Andy Britcliffe
    0x9b2090, // John Simpkins
    0x9b2084, // Rob O'Neill
    0x9b2074, // Ian Whitfield
    0x9b2064, // Paul Manzotti
    0x9b2054, // Paul Boreham
    0x9b2048, // Tom Lewis
    0x9b2038, // Ben Laughlin
    0x9b2028, // Stuart Govier
    0x9b201c, // Andy Barlow
    0x9b200c, // Mark Bradshaw
    0x9b2000, // Dave Birks
    0x9b1ff0, // Iain Watkins
    0x9b1fe4, // Ian Drayton
    0x9b1fd4, // Richard Hughes
    0x9b1fc8, // Noel Pearce
    0x9b1fbc, // Ian John
    0x9b1fac, // Andy Fellows
    0x9b1f9c, // Graham Jones
    0x9b1f90, // Mark Currie
    0x9b1f80, // Steve Barlow
    0x9b1f74, // Mike Carey
    0x9b1f64, // Chris Goodwin
    0x9b1f50, // Malcolm Williams
    0x9b1f40, // Daniel Butcher
    0x9b1f30, // Mark Williams
    0x9b1f20, // Paul Knapper
    0x9b1f10, // Richard Gibson
    0x9b1f00, // Will Thornton
    0x9b1ef4, // Sam Watt
    0x9b1ee4, // Andy Wilkinson
    0x9b1ed8, // Tom Povey
    0x9b1ec8, // Colin Cavanah
    0x9b1ebc, // Craig Buck
    0x9b1ea8, // Graham Bambrough
    0x9b1e94, // Adam Etherington
    0x9b1e88, // Danny Gee
    0x9b1e7c, // Mel Thomas
    0x9b1e70, // Mike Smith
    0x9b1e5c, // Christopher Evans
    0x9b1e44, // Rhidian 'Egg' Jenkins
    0x9b1e34, // Milos Tasic
    0x9b1e20, // Sinisa Novkovic
    0x9b1e08, // Aleksandar Miladinovic
    0x9b1dfc, // Ivan Simic
    0x9b1dec, // Driss Merabtene
    0x9b1dd8, // Giusepe Imbugria
    0x9b1dc4, // Tarek El Shaarani
    0x9b1db8, // Reda Maher
    0x9b1da0, // Amandeep Singh Mahal
    0x9b1d90, // Jacques Foetz
    0x9b1d80, // Mart Jyrisoo
    0x9b1d70, // Marek Ivanson
    0x9b1d60, // Peter Kucharik
    0x9b1d50, // Pedro Campos
    0x9b1d3c, // Davorin Stevanovic
    0x9b1d2c, // Faris Al Bugami
    0x9b1d1c, // Nedko Kyuchukov
    0x9b1d08, // Freyr Kolbeinsson
    0x9b1cf8, // Tony Gamatin
    0x9b1ce8, // Ruslan Ulbajev
    0x9b1cd8, // Béres  Endre
    0x9b1cc8, // Laith Majali
    0x9b1cb4, // Dmitriy Belcencov
    0x9b1ca8, // Ali Yasan
    0x9b1c98, // Andrey Muravyov
    0x9b1c8c, // Manav Puree
    0x9b1c80, // Lica Sever
    0x9b1c74, // Mark Muscat
    0x9b1c60, // Andreas Michaelides
];

/// The credits screen's build and input functions (`credits_screen_build`, `credits_screen_input`).
pub(crate) const CREDITS_SCREEN_BUILD: usize = 0x513180;

pub(crate) const CREDITS_SCREEN_INPUT: usize = 0x5133c0;

/// The second text buffer of the screens (a name or a role translated apart from `JOB_TEXT`) [inference].
pub(crate) const CREDITS_SECOND_TEXT: usize = 0xae1098;

/// (role, name), exe texts, in the order of the original.
pub(crate) const SPORTS_INTERACTIVE_CHAIRMAN_ROWS: [(u32, u32); 2] = [
    (0x9b1234, 0x99b240), // Chairman: Oliver Collyer
    (0x9b1234, 0x99b170), // Chairman: Paul Collyer
];

/// (role, name), exe texts, in the order of the original.
pub(crate) const SPORTS_INTERACTIVE_DESIGN_ROWS: [(u32, u32); 7] = [
    (0x9b11d0, 0x99b3d4), // Design & Programming: Paul Norman
    (0x9b11d0, 0x99b074), // Design & Programming: Kevin Turner
    (0x9b11d0, 0x99adf0), // Design & Programming: Marc Vaughan
    (0x9b11d0, 0x99acbc), // Design & Programming: Svein Kvernoey
    (0x9b11d0, 0x99ab28), // Design & Programming: Keith Flannery
    (0x9b11d0, 0x9b11c0), // Design & Programming: Miles Dinneen
    (0x9b11d0, 0x9b11b4), // Design & Programming: Sam Deane
];

/// (role, name), exe texts, in the order of the original.
pub(crate) const SPORTS_INTERACTIVE_OTHER_ROWS: [(u32, u32); 6] = [
    (0x9b119c, 0x99aebc), // Research Co-ordination: Mark Woodger
    (0x9b119c, 0x9b118c), // Research Co-ordination: Peter Sottrel
    (0x9b1174, 0x99af88), // Internet Development: Marc Duffy
    (0x9b1164, 0x9b1154), // Sound Effects: Graham Miles
    (0x9b1138, 0x9b1124), // Manual & Written History: Alexander Deacon
    (0x9b1118, 0x9b1108), // QA Manager: Joe O'Rielly
];

/// (role, name), exe texts, in the order of the original.
pub(crate) const JASK_XBOX_ROWS: [(u32, u32); 1] = [
    (0x9b1260, 0x9b1254), // Xbox Design & Programming: Simon Page
];

/// (role, name), exe texts, in the order of the original.
pub(crate) const RESEARCH_ROWS_ARGENTINA_TO_JAPAN: [(u32, u32); 18] = [
    (0x9b1698, 0x9b1688), // Argentina: Daniel Szmid
    (0x9b167c, 0x9b166c), // Australia: David Bintley
    (0x9b1664, 0x9b1654), // Austria: Gregor Eigner
    (0x9b164c, 0x9b1638), // Belgium: Fabrice Leclercq
    (0x9b164c, 0x9b1628), // Belgium: Kristof Terreur
    (0x9b1620, 0x9b1610), // Brazil: Ndongo Bastos
    (0x9b1608, 0x9b15fc), // China: Frank Ng
    (0x9b15f4, 0x9b15e0), // Croatia: Tihomir Kovacevic
    (0x9b15d0, 0x9b15c0), // Czech Republic: Tony Grasser
    (0x9b15b8, 0x9b15a4), // Denmark: Jesper Krogsgaard
    (0x9b159c, 0x9b158c), // Finland: Jarkko Nyman
    (0x9b1584, 0x9b1570), // France: Emmanuel Rochard
    (0x9b1568, 0x9b1558), // Germany: Ronald Demohn
    (0x9b1550, 0x9b153c), // Greece: Spyros Giannoutsos
    (0x9b1534, 0x9b1524), // Holland: René Schipper
    (0x9b1534, 0x9b1514), // Holland: Martin Barelds
    (0x9b150c, 0x9b14f4), // Italy: Alberto Scotta 'Panoz'
    (0x9b14ec, 0x9b14dc), // Japan: Yoshimi Kano
];

/// (role, name), exe texts, in the order of the original.
pub(crate) const RESEARCH_ROWS_MEXICO_TO_YUGOSLAVIA: [(u32, u32); 21] = [
    (0x9b14bc, 0x9b14a8), // Mexico: Martin del Palacio
    (0x9b1494, 0x9b1484), // Northern Ireland: Gordon Smyth
    (0x9b147c, 0x9b146c), // Norway: Tore Ludvigsen
    (0x9b147c, 0x9b1458), // Norway: Vegard Rønningen
    (0x9b1450, 0x9b143c), // Poland: Krzysztof Scibiorek
    (0x9b1430, 0x9b1418), // Portugal: 'Zé' Guilherme Chieira
    (0x9b1404, 0x9b13f4), // Republic of Ireland: Darren Matthews
    (0x9b13ec, 0x9b13dc), // Russia: Alex Cieszynscy
    (0x9b13ec, 0x9b13c8), // Russia: Maxim Yevtushenko
    (0x9b13bc, 0x9b13ac), // Scotland: Stuart Milne
    (0x9b14d0, 0x9b1394), // South Korea: South Korea CM Fan Club
    (0x9b1384, 0x9b1374), // South Africa: Sean Riskowitz
    (0x9b136c, 0x9b1358), // Spain: Iván Abella Villar
    (0x9b136c, 0x9b133c), // Spain: Honorino Zamora, 'Nordin'
    (0x9b1334, 0x9b1328), // Sweden: Jan Juhlin
    (0x9b1334, 0x9b1310), // Sweden: Andreas Broström Ling
    (0x9b1304, 0x9b12f4), // Switzerland: Christian Guei
    (0x9b12ec, 0x9b12dc), // Turkey: Türker Tozar
    (0x9b12d4, 0x9b12c8), // U.S.A.: Tony Roche
    (0x9b12c0, 0x9b12b0), // Wales: Neil Vaughan
    (0x9b12a4, 0x9b1290), // Yugoslavia: Dusan Stamenkovic
];

pub(crate) const CREDITS_ROW: usize = 0xae186c;

pub(crate) const CREDITS_AREA: usize = 0xae1870;

pub(crate) const CREDITS_AREA_ROWS: usize = 0xae1872;

pub(crate) const CREDITS_POPUP_AREA: usize = 0xae1874;

pub(crate) const CREDITS_RIGHT_COLUMN: usize = 0xae1878;

pub(crate) const CREDITS_PAGE: usize = 0xae187c;

pub(crate) const CREDITS_SHOWN_PAGE: usize = 0xae187e;

/// The colour of a credit's role (the right column of the name/role rows) [0xaebdde].
pub(crate) const CREDITS_ROLE_COLOUR: usize = 0xaebdde;

/// The credits state, as a region for the comparers.
pub(crate) const CREDITS_STATE: (usize, usize) = (CREDITS_LINE, 0x18);

/// The external testers of the Additional Testing section, in the original's order.
pub(crate) const ADDITIONAL_TESTERS: [usize; 56] = [
    0x9b4e78, 0x9b4e64, 0x9b4e54, 0x9b4e3c, 0x9b4e2c, 0x9b4e18,
    0x9b4e08, 0x9b4804, 0x9b4dec, 0x9b4ddc, 0x9b4dcc, 0x9b4db8,
    0x9b4da4, 0x9b4d90, 0x9b4d78, 0x9b4d68, 0x9b34c4, 0x9b34b8,
    0x9b34a8, 0x9b3494, 0x9b3488, 0x9b347c, 0x9b345c, 0x9b4d50,
    0x9b4d38, 0x9b4d28, 0x9b4d18, 0x9b4d04, 0x9b4cf8, 0x9b4cec,
    0x9b4cd8, 0x9b4ccc, 0x9b4cbc, 0x9b4ca8, 0x9b4c94, 0x9b4c7c,
    0x9b4c64, 0x9b4c58, 0x9b4c48, 0x9b4c30, 0x9b4c1c, 0x9b4c0c,
    0x9b4bfc, 0x9b2c78, 0x9b4bec, 0x9b4bdc, 0x9b4bcc, 0x9b4bb4,
    0x9b4ba4, 0x9b4b90, 0x9b4b84, 0x9b4b74, 0x9b4b64, 0x9b4b58,
    0x9b4b44, 0x9b4b30,
];

/// The name/role rows of the Special Thanks section shown in English only (language 0), the roles not translated.
pub(crate) const SPECIAL_THANKS_ENGLISH: [(usize, usize); 16] = [
    (0x9b5220, 0x9b5210), (0x9b5220, 0x9b5200), (0x9b51ec, 0x99aa30), (0x9b51cc, 0x9b51bc), (0x9b51ac, 0x9b51a0),
    (0x9b5190, 0x9b5184), (0x9b517c, 0x9b515c), (0x9b517c, 0x9b513c), (0x9b517c, 0x9b5120), (0x9b517c, 0x9b5110),
    (0x9b50f4, 0x9b50e4), (0x9b50c4, 0x9b2940), (0x9b50a8, 0x9b5098), (0x9b5084, 0x9b185c), (0x9b5068, 0x9b5050),
    (0x9b5068, 0x9b5030),
];

/// The credits list state, shared by every section builder of the Game Credits screen (and, for the row and the
/// area, by the Web Sites screen): the line counter [0xae1868] (a word: the low word of a section's popup value),
/// the row counter [0xae186c], the list area [0xae1870] (a word; the rows per page in the byte [0xae1872]), the
/// "View" popup area [0xae1874], the right-column flag of the two-column sections [0xae1878], the page being built
/// [0xae187c] and the page shown [0xae187e] (both words; screen parameter 2).
pub(crate) const CREDITS_LINE: usize = 0xae1868;
