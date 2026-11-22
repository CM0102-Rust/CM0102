// ---------- batch %%LEVA%%: the contract screens: the transfer request, the staff contract offer, the terminate contract screen and the squad order (contract_screens.cpp) ----------

/// A screen builder (cdecl, no arguments).
type ContractScreenFn = unsafe extern "C" fn();
/// `background_set_staff_picture` (0x41c790, cdecl(person, 0, 0)).
type StaffPictureFn = unsafe extern "C" fn(usize, u32, u32);
/// `News::set_item_slot` (0x786d30, thiscall(news, manager, id, slot, value, copy) ret 0x14).
type NewsSlotFn = unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, usize) -> u32;

/// The widget ids of the transfer request screen and of the terminate contract screen.
const REQUEST_YES: u16 = 0x20;
const REQUEST_NO: u16 = 0x21;
const REQUEST_BACK: u16 = 0x22;
const TERMINATE_YES: u16 = 1;
const TERMINATE_NO: u16 = 2;
/// The staff contract offer screen's "Offer" and "Back" widgets.
const OFFER_SEND: u16 = 0x14;
const OFFER_BACK: u16 = 0x15;

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
    let ctor: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let p = item.as_mut_ptr() as usize;
    ctor(p, 0, 0);
    let find: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(0x786b70usize);
    find(read_usize(G_NEWS), manager, id, p);
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
    let picture: StaffPictureFn = core::mem::transmute(0x41c790usize);
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

/// The player's answer to a refused transfer request, from his contract: the request is withdrawn
/// (`STAFF_CONTRACT::apply_event_flags` with 0x10000) at once when the contract morale is under -50 and the screen's
/// parameter-0 person (the manager) has loyalty under 6 and professionalism under 10; otherwise only when
/// random(loyalty × 20 + morale + 50) comes out under random((75 − temperament − professionalism) × 5), those three
/// read from the same parameter-0 person (the original reads the manager's, not the player's).
unsafe fn request_refused_answer(manager: usize, contract: usize) {
    let morale = read_i8(contract + contract::MORALE);
    let (loyalty, professionalism) = (read_i8(manager + staff::LOYALTY), read_i8(manager + staff::PROFESSIONALISM));
    let withdraw = if morale >= -50 || loyalty >= 6 || professionalism >= 10 {
        let a = exe::utils_random((0x4b - read_i8(manager + staff::TEMPERAMENT) as i32 - professionalism as i32) * 5);
        let b = exe::utils_random(loyalty as i32 * 20 + morale as i32 + 0x32);
        b < a
    } else {
        true
    };
    if withdraw {
        covered!("tel31_3 request withdrawn");
        let flags: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_APPLY_EVENT_FLAGS);
        flags(contract, 0x10000, 0);
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
    let player = staff_at(player_id as isize).0;
    let club = club_at(club_id as isize).0;
    let contract = exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club);
    if contract == 0 {
        return -9;
    }
    let slot: NewsSlotFn = core::mem::transmute(FN_NEWS_SET_ITEM_SLOT);
    if id == REQUEST_NO {
        covered!("tel31_3 request refused");
        slot(read_usize(G_NEWS), manager, news_id, 7, 1, 0);
        slot(read_usize(G_NEWS), manager, news_id, 8, 0, 0);
        request_refused_answer(manager, contract);
        let delta = (read_u8(player + staff::LOYALTY).wrapping_mul(5)).wrapping_add(0x38) as i8;
        let morale = read_i8(contract + contract::MORALE);
        let sum = morale as i32 + delta as i32;
        let new = if sum > 100 { 100 } else if sum < -100 { -100 } else { morale.wrapping_add(delta) };
        write_i8(contract + contract::MORALE, new);
        exe::staff_adjust_morale(staff_at(player_id as isize).0, (-12i32) as u32, 0);
    } else {
        covered!("tel31_3 request accepted");
        slot(read_usize(G_NEWS), manager, news_id, 7, 1, 0);
        slot(read_usize(G_NEWS), manager, news_id, 8, 1, 0);
        if exe::staff_is_loan_listed(manager, 0usize) != 0 {
            let remove: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8442d0usize);
            remove(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER), manager);
        }
        let status = read_u8(contract + contract::TRANSFER_STATUS);
        if status & 2 != 0 {
            write_u8(contract + contract::TRANSFER_STATUS, status & 0xfd);
        }
        write_u8(contract + contract::TRANSFER_STATUS, read_u8(contract + contract::TRANSFER_STATUS) | 8);
        exe::staff_adjust_morale(staff_at(player_id as isize).0, 0xc, 0);
        exe::shortlist_manager_add_player_to_club_shortlists(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER),
            staff_at(player_id as isize).0, 1);
        exe::contract_manager_post_transfer_listed_news(read_usize(G_CONTRACT_MANAGER), staff_at(player_id as isize).0, club_at(club_id as isize).0);
    }
    -9
}

/// The club of a person's contract, as `CONTRACT_MANAGER::get_contract` leaves it in `G_CONTRACT_LOOKUP_RESULT`:
/// none or a type-5 contract gives no club.
unsafe fn contract_club_of(person: usize) -> usize {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 5 {
        club_at(read_i32(c + contract::CLUB) as isize).0
    } else {
        0
    }
}

/// The local transfer offer the staff contract offer builds (TRANSFER_OFFER, 0x41 bytes) and the date after it.
#[repr(C, align(4))]
struct StaffOfferLocal([u8; 0x44]);

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
    let get_offer: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x5fd930usize);
    let terms = get_offer(read_usize(G_HUMAN_MANAGER), 0);
    let manager = tel26_slot_manager().0;
    let person = staff_at(read_i32(terms) as isize).0;
    let job = get_screen_param(4) as u8;
    let mark = job as u32; // simplification: the original pushes the dword whose low byte is this parameter (the rest is stack leftover)
    let withdraw = get_screen_param(5);
    let told_unset = get_screen_param(6) as u8;
    let compensation = get_screen_param(7);
    if withdraw != 0 {
        covered!("tel31_3 staff offer withdrawn");
        let club = read_u32(manager + staff::CLUB_JOB);
        exe::contract_manager_remove_contract_offer(read_usize(G_CONTRACT_MANAGER), person, club as usize);
        return -10;
    }
    let (mut slot, mut serial) = (-1i32, u32::MAX);
    let holder = get_screen_param(0);
    if holder != 0 {
        let offer = read_usize(holder);
        slot = get_screen_param(1) as i32;
        serial = get_screen_param(2) as u32;
        match (read_u8(terms + contract::SQUAD_STATUS) >> 4) as i8 as i32 - 2 {
            0 => write_u8(terms + contract::HAPPINESS, 0x4b),
            1 => write_u8(terms + contract::HAPPINESS, 0x32),
            2 | 3 => write_u8(terms + contract::HAPPINESS, 0),
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
            let changed: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x8f2470usize);
            changed(offer, serial);
            return -11;
        }
    } else if read_u32(manager + staff::CLUB_JOB) as usize != contract_club_of(person) {
        covered!("tel31_3 staff offer with a transfer offer");
        let mut local = StaffOfferLocal([0; 0x44]);
        let o = local.0.as_mut_ptr() as usize;
        let mut date = [read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE + 4)];
        let ctor: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, u32) -> usize = core::mem::transmute(0x8e56a0usize);
        ctor(o, person, read_u32(manager + staff::CLUB_JOB), 0, -1, 1);
        write_u8(o + offer::JOB_TYPE, job);
        let mut kind = read_u8(o + offer::COMPLETION_TIMING);
        if exe::database_is_player_job(read_i8(person + staff::JOB_FOR_CLUB)) != 0 && contract_club_of(person) != 0 {
            if compensation != 0 && told_unset == 0 {
                let fee = exe::transfer_manager_calc_compensation_fee(read_usize(G_TRANSFER_MANAGER), person, read_usize(manager + staff::CLUB_JOB), 0);
                write_i32(o + offer::FEE, fee);
                kind = kind & 0xf5 | 5;
                write_u8(o + offer::COMPLETION_TIMING, kind);
            } else {
                let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
                if read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 1 {
                    write_u8(o + offer::COMPLETION_TIMING, read_u8(o + offer::COMPLETION_TIMING) & 0xf3 | 3);
                    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
                    let mut tmp = [0u32; 2];
                    exe::date_operator_3d(date.as_mut_ptr() as usize, tmp.as_mut_ptr() as usize, c + contract::CONTRACT_EXPIRES);
                }
                kind = read_u8(o + offer::COMPLETION_TIMING);
            }
        }
        if read_usize(person + staff::PLAYER) != 0 && kind & 0xf != 5 && matches!(job, 0xb | 0xd | 0xf) {
            let club = contract_club_of(person);
            if exe::transfer_manager_can_sign_player(read_usize(G_TRANSFER_MANAGER), read_usize(manager + staff::CLUB_JOB), date.as_mut_ptr() as usize,
                person, club, 0, core::ptr::null_mut()) == 0 {
                covered!("tel31_3 staff offer unsignable");
                let k = read_u8(o + offer::COMPLETION_TIMING);
                write_u8(o + offer::COMPLETION_TIMING, if k & 0xf == 0 { k & 0xf1 | 1 } else { k & 0xf4 | 4 });
            }
        }
        write_u8(o + offer::PLAYER_AWARE, get_screen_param(3) as u8);
        exe::transfer_manager_add_transfer_offer(read_usize(G_TRANSFER_MANAGER), o as *mut u8, core::ptr::null_mut());
        slot = read_i32(o + offer::SLOT_INDEX);
        serial = read_u32(o + offer::SERIAL);
        let dtor: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
        dtor(o);
    }
    exe::contract_manager_add_contract_offer(read_usize(G_CONTRACT_MANAGER), terms as *mut u8, slot, serial as usize, mark, 0, 1, 0);
    if holder != 0 {
        let offer = read_usize(holder);
        if read_u32(offer + offer::TARGET_STAFF_ID) == read_u32(terms) {
            write_u8(offer + offer::TERMS_STATE, 1);
        } else {
            let list = read_usize(offer + offer::EXCHANGE_PLAYERS);
            write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
            let mut e = exe::llist_get_next(read_usize(read_usize(holder) + offer::EXCHANGE_PLAYERS));
            while e != 0 {
                if read_u32(e) == read_u32(terms) {
                    write_u8(e + 4, 1);
                }
                e = exe::llist_get_next(read_usize(read_usize(holder) + offer::EXCHANGE_PLAYERS));
            }
        }
    }
    write_i32(terms, -1);
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
    let picture: StaffPictureFn = core::mem::transmute(0x41c790usize);
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
            let person = get_screen_param(0);
            let job = read_u8(person + staff::JOB_FOR_CLUB);
            let club = read_usize(person + staff::CLUB_JOB);
            exe::contract_manager_remove_contract(read_usize(G_CONTRACT_MANAGER), person);
            exe::database_remove_staff_from_club(read_usize(person + staff::CLUB_JOB), person, 0usize, 1, 0);
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

// ---------- comparers ----------

/// Comparer for `contract_screens_news_transfer_request_respond_action`.
pub(crate) unsafe extern "C" fn compare_contract_screens_news_transfer_request_respond_action(manager: usize, id: u32) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I0%%)(manager, id)
}

/// Comparer for `contract_screens_build_transfer_request_screen`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_contract_screens_build_transfer_request_screen() {
    let o = original_fn::<ContractScreenFn>(%%I1%%);
    compare_by_display_list_with("Transfer Request screen (contract_screens_build_transfer_request_screen)", &[], || o(),
        || contract_screens_build_transfer_request_screen());
}

/// Comparer for `contract_screens_handle_transfer_request_screen`.
pub(crate) unsafe extern "C" fn compare_contract_screens_handle_transfer_request_screen(item: u32) -> i32 {
    // simplification: the answer writes the contract, the news and the shortlists and draws the random generator;
    // it does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%)(item)
}

/// Comparer for `contract_screens_handle_staff_offer_contract_screen`.
pub(crate) unsafe extern "C" fn compare_contract_screens_handle_staff_offer_contract_screen(item: u32) -> i32 {
    // simplification: the offers are added and freed; it does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I3%%)(item)
}

/// Comparer for `contract_screens_open_terminate_contract_screen`.
pub(crate) unsafe extern "C" fn compare_contract_screens_open_terminate_contract_screen(person: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize)>(%%I4%%)(person)
}

/// Comparer for `contract_screens_build_terminate_contract_screen`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_contract_screens_build_terminate_contract_screen() {
    let o = original_fn::<ContractScreenFn>(%%I5%%);
    compare_by_display_list_with("Terminate Contract screen (contract_screens_build_terminate_contract_screen)", &[], || o(),
        || contract_screens_build_terminate_contract_screen());
}

/// Comparer for `contract_screens_handle_terminate_contract_screen`.
pub(crate) unsafe extern "C" fn compare_contract_screens_handle_terminate_contract_screen(item: u32) -> i32 {
    // simplification: the contract ends and the person leaves the club; it does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I6%%)(item)
}

compare_order!(compare_compare_squad_players_by_appearances_then_reputation, compare_squad_players_by_appearances_then_reputation, %%I7%%);
