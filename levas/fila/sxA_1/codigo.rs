// ---------- batch %%LEVA%%: the automatic reply to a news item (news.cpp) ----------

/// The automatic reply's answer to a news item: `News::set_item_slot` (on `G_NEWS`) for `manager`, the item's id, the
/// reply slot and its value, with the item itself as the copy (or 0, as the original passes for 0xfab).
unsafe fn auto_reply_slot(news: usize, manager: usize, slot: i32, value: i32, copy: usize) {
    exe::news_set_item_slot(read_usize(G_NEWS), manager, read_usize(news + news_item::ID), slot as u32, value as u32, copy as u32);
}

/// Data slot `k` of a news item (a dword at `news_item::DATA` + 4 × k).
unsafe fn auto_reply_data(news: usize, k: usize) -> u32 {
    read_u32(action_slot(news, k))
}

/// The date built from the low words of slots `day_slot` and `day_slot + 1` (`date_from_day_year`, then
/// `date_assign` onto a default date, as the original does).
unsafe fn auto_reply_date(news: usize, day_slot: usize) -> RawDate {
    let (mut d, mut t): (RawDate, RawDate) = ([0; 8], [0; 8]);
    date_default(&mut d);
    let r = date_from_day_year(&mut t, auto_reply_data(news, day_slot) as u16, auto_reply_data(news, day_slot + 1) as u16);
    date_assign(&mut d, r);
    d
}

/// Category 6, type 0x1774 (a job offer): with the human on holiday (`exe::human_manager_is_on_holiday`) and either
/// without a club or with holiday option 1 on (`exe::human_manager_get_holiday_option`), the offer of the club in slot
/// 0 is declined (slot 1 = 2) when that club's reputation is below his club's, and that ends the reply; otherwise it is
/// accepted (slot 1 = 1) and he is appointed (`exe::manager_manager_appoint_manager_to_club`). Not on holiday or
/// without the option: declined. Returns false when the reply ends here.
unsafe fn auto_reply_job_offer(news: usize, manager: usize) -> bool {
    let hm = read_usize(G_HUMAN_MANAGER);
    let current = Staff(manager).club_job();
    if exe::human_manager_is_on_holiday(hm, manager) == 0 || (!current.is_null() && exe::human_manager_get_holiday_option(hm, 1, manager) == 0) {
        auto_reply_slot(news, manager, 1, 2, news);
        return true;
    }
    let club = club_at(auto_reply_data(news, 0) as isize);
    if !current.is_null() && club.reputation() < current.reputation() {
        covered!("sxA_1 job offer declined");
        auto_reply_slot(news, manager, 1, 2, news); // sxA_1: a club below his own is declined
        return false;
    }
    covered!("sxA_1 job offer accepted");
    auto_reply_slot(news, manager, 1, 1, news);
    exe::manager_manager_appoint_manager_to_club(read_usize(G_MANAGER_MANAGER), club, manager);
    true
}

/// Category 2, type 6 (a friendly request, club teams only): the friendly of the club in slot 2 on the date of slots
/// 3 and 4 (`exe::friendly_find_friendly_match`) is accepted (status 1, `exe::friendly_process_friendly_reply`).
unsafe fn auto_reply_friendly(news: usize) {
    let d = auto_reply_date(news, 3);
    let fr = read_usize(G_FRIENDLY);
    let m = exe::friendly_find_friendly_match(fr, d.as_ptr() as usize, club_at(auto_reply_data(news, 2) as isize));
    write_u8(m + friendly_match::STATUS, 1); // sxA_1: the friendly accepted
    exe::friendly_process_friendly_reply(fr, m, 1);
}

/// Category 2, type 9 (a tournament invitation, club teams only): the tournament of kind slot 2 (its low byte) on the
/// date of slots 5 and 6 (`exe::friendly_get_tournament`); the club of slot 1 is looked for among its invited clubs
/// from the second on and, when found, its invitation is marked accepted; then `exe::friendly_process_tournament_replies`
/// for that club and the reply (slot 3 = the invitation mark). Not found: the original reads an uninitialised stack
/// byte as the index, so the original routine runs instead (see the header). Returns false in that case.
unsafe fn auto_reply_tournament(news: usize, manager: usize) -> bool {
    let club = club_at(auto_reply_data(news, 1) as isize).addr();
    let d = auto_reply_date(news, 5);
    let fr = read_usize(G_FRIENDLY);
    let t = exe::friendly_get_tournament(fr, read_u8(action_slot(news, 2)) as u32, d.as_ptr() as usize);
    let count = read_i16(t + tournament_slot::TEAM_COUNT);
    let mut k: i8 = 1;
    let mut found = None;
    while (k as i16) < count {
        if read_usize(t + tournament_slot::INVITED_CLUBS + 4 * k as isize as usize) == club {
            found = Some(k);
            break;
        }
        k = k.wrapping_add(1);
    }
    let Some(k) = found else { return false };
    let pending = t + tournament_slot::INVITE_PENDING + k as isize as usize;
    write_u8(pending, 1); // sxA_1: the invitation accepted
    exe::friendly_process_tournament_replies(fr, t, 0, club as u32);
    auto_reply_slot(news, manager, 3, read_i8(pending) as i32, news);
    true
}

/// Category 2, type 0x1e (a national squad call-up, national teams only): the next match of the national side of slot 0
/// from the date of slots 6 (day) and 7 (year) (`FIXTURE_MANAGER::scan_fixtures`, one match into a local list; none:
/// the box at line 0x198b and no reply). If that match is not an international one (`fixture::FLAGS` & 3) or the side
/// is not a national team, squad 0 is picked (`select_squad`) unless already picked; then, if the side has a squad
/// slot for slot 1 (`national_teams_get_squad_slot`), that squad too. The reply: slot 2 = 1.
unsafe fn auto_reply_call_up(news: usize, manager: usize) {
    let mut list = [0u8; 14];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    let index = auto_reply_data(news, 0);
    let side = club_at(index as isize);
    // the original passes the day and the year in registers whose upper words keep the club index arithmetic
    let mut day = (index & 0xffff_0000) | (auto_reply_data(news, 6) & 0xffff);
    let mut year = (index.wrapping_mul(0x91) & 0xffff_0000) | (auto_reply_data(news, 7) & 0xffff);
    let mut slot: u8 = 0;
    let squad = auto_reply_data(news, 1);
    type Scan = unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, i32, i32, i32, u32, i32, i32, i32, i32,
                                            i32, usize, u32, i32, i32, u32) -> u32;
    let scan: Scan = core::mem::transmute(FN_FIXTURE_MANAGER_SCAN_FIXTURES);
    scan(read_usize(G_FIXTURE_MANAGER), &mut year, &mut day, &mut slot, 1, -1, -1, -1, read_u32(side.addr()), -1, 0, -1, -1, 1, l, 2, -2, 1, 1);
    if read_u16(l + llist::COUNT) == 0 {
        show_error_at(0xa693ec, 0x198b);
        exe::drop_llist(l);
        return;
    }
    list_rewind(l);
    let fixture = exe::llist_get_next(l);
    let nt = read_usize(G_NATIONAL_TEAMS);
    let nation = side.nation().addr();
    if read_u8(fixture + fixture::FLAGS) & 3 != 0 || exe::club_is_national_team(side) == 0 {
        if exe::nationalteams_is_squad_selected(nt, nation, 0) == 0 {
            exe::nationalteams_select_squad(nt, nation, 0, G_CURRENT_DATE, fixture, 1);
            exe::nationalteams_set_squad_selected(nt, nation, 0);
        }
    }
    let k = exe::national_teams_get_squad_slot(side, squad) as u32;
    if k != 0 && exe::nationalteams_is_squad_selected(read_usize(G_NATIONAL_TEAMS), nation, k) == 0 {
        covered!("sxA_1 call-up squad");
        exe::nationalteams_select_squad(read_usize(G_NATIONAL_TEAMS), nation, k, G_CURRENT_DATE, fixture, 1); // sxA_1: the slot's squad
        exe::nationalteams_set_squad_selected(read_usize(G_NATIONAL_TEAMS), nation, k);
    }
    auto_reply_slot(news, manager, 2, 1, news);
    exe::drop_llist(l);
}

/// Category 2 (club and squad items). Returns false when the original must run instead (`auto_reply_tournament`).
unsafe fn auto_reply_club_items(news: usize, manager: usize, national: bool) -> bool {
    let kind = read_u32(news);
    let staff = || staff_at(auto_reply_data(news, 0) as isize);
    match kind {
        6 if !national => auto_reply_friendly(news),
        9 if !national => return auto_reply_tournament(news, manager),
        0x16 if !national => auto_reply_slot(news, manager, 0, 1, news),
        0x18 if !national => {
            let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(club_at(auto_reply_data(news, 0) as isize).division().addr()) as usize * 4);
            exe::squad_manager_assign_league_squad_numbers(read_usize(G_SQUAD_MANAGER), comp, 0, 1);
            auto_reply_slot(news, manager, 1, 1, news);
        }
        0x19 if !national => {
            exe::squad_manager_select_euro_registered_players(read_usize(G_SQUAD_MANAGER), club_at(auto_reply_data(news, 0) as isize),
                read_u8(action_slot(news, 2)) as u32, 0);
            auto_reply_slot(news, manager, 3, 1, news);
            auto_reply_slot(news, manager, 6, -1, news);
        }
        0x1e if national => auto_reply_call_up(news, manager),
        0x22 if national => {
            covered!("sxA_1 call-up replacement");
            let nation = read_usize(G_NATION_TABLE).wrapping_add((auto_reply_data(news, 11) as usize).wrapping_mul(nation::SIZE));
            exe::nationalteams_call_up_replacement(read_usize(G_NATIONAL_TEAMS), nation, read_u8(action_slot(news, 10)) as u32,
                read_u8(action_slot(news, 9)) as u32, staff());
            auto_reply_slot(news, manager, 7, 1, news);
            if read_u32(news) == 0x28 {
                auto_reply_slot(news, manager, 0, 3, news);
            }
        }
        _ => {
            if kind == 0x28 {
                auto_reply_slot(news, manager, 0, 3, news);
            }
        }
    }
    true
}

/// Category 4, type 0xbbf (the player asks to be listed): with a contract (`exe::contract_manager_get_contract`), the
/// asking price of his club and his value (`exe::contract_manager_calc_player_value`); value × 1.5 above the price:
/// the request is accepted (slot 8 = 1): off the loan list (bit 2 of `contract::TRANSFER_STATUS`), on the transfer list
/// (bit 8), morale +12, on the clubs' shortlists and the listed news. Otherwise refused (slot 8 = 0): the contract's
/// morale goes up by loyalty × 5 + 0x38 (byte arithmetic, within −100..100), morale −12, and if random((the manager's
/// loyalty + 10) × 20 + the contract's morale) < random(50) the contract gets event flag 0x10000. No player sheet or no
/// contract: nothing of that. Then slot 7 = 1.
unsafe fn auto_reply_transfer_request(news: usize, manager: usize) {
    let player = staff_at(auto_reply_data(news, 2) as isize);
    if !player.player().is_null() {
        let cm = read_usize(G_CONTRACT_MANAGER);
        let contract = Contract(exe::contract_manager_get_contract(cm, player));
        if !contract.is_null() {
            let club = club_at(contract.club() as isize);
            let asking = exe::contract_manager_calc_asking_price(cm, club, 0usize, player, 0, 1, 1);
            let value = exe::contract_manager_calc_player_value(cm, player, club_at(contract.club() as isize), 0, -1, 0);
            if value as f64 * read_f64(rdata::F64_1_5) > asking as f64 {
                covered!("sxA_1 transfer request accepted");
                auto_reply_slot(news, manager, 8, 1, news); // sxA_1: the request accepted
                if contract.transfer_status() & 2 != 0 {
                    contract.set_transfer_status(contract.transfer_status() & !2);
                }
                contract.set_transfer_status(contract.transfer_status() | 8);
                exe::staff_adjust_morale(player, 0xc, 0);
                exe::shortlist_manager_add_player_to_club_shortlists(TransferManagerFields(read_usize(G_TRANSFER_MANAGER)).shortlist_manager(), player, 1);
                exe::contract_manager_post_transfer_listed_news(cm, player, club_at(contract.club() as isize));
            } else {
                covered!("sxA_1 transfer request refused");
                auto_reply_slot(news, manager, 8, 0, news);
                let rise = (player.loyalty() as u8).wrapping_mul(5).wrapping_add(0x38) as i8;
                let sum = rise as i32 + contract.morale() as i32;
                contract.set_morale(if sum > 100 { 100 } else if sum < -100 { -100 } else { rise.wrapping_add(contract.morale()) });
                exe::staff_adjust_morale(player, -12i32 as u32, 0);
                let chance = (Staff(manager).loyalty() as i32 + 10) * 20 + contract.morale() as i32;
                let r = exe::utils_random(chance);
                if r < exe::utils_random(0x32) {
                    exe::staff_contract_apply_event_flags(contract.addr(), 0x10000, 0); // sxA_1: the unsettled flag
                }
            }
        }
    }
    auto_reply_slot(news, manager, 7, 1, news);
}

/// Category 4, type 0xbb8 (a contract offer to a player): with an offer in slot 8, the offer's contract goes to its
/// player (`exe::transfer_manager_offer_contract_to_offer_player`, the club of slot 4 and the person of slot 0); else,
/// with kind 5 in slot 10, a squad number at the club of slot 4 and the reply slot 0xb = 1; otherwise the person of
/// slot 0 is offered a contract (`exe::contract_manager_offer_contract`, 1.05, mode 2).
unsafe fn auto_reply_contract_offer(news: usize, manager: usize) {
    let person = staff_at(auto_reply_data(news, 0) as isize);
    let club = club_at(auto_reply_data(news, 4) as isize);
    let offer = auto_reply_data(news, 8) as i32;
    if offer != -1 {
        let tm = read_usize(G_TRANSFER_MANAGER);
        let slot = exe::transfer_manager_get_offer_slot(tm, offer);
        exe::transfer_manager_offer_contract_to_offer_player(tm, club, person, read_usize(slot));
    } else if read_u8(action_slot(news, 10)) == 5 {
        exe::squad_manager_allocate_squad_number(read_usize(G_SQUAD_MANAGER), club, 0usize);
        auto_reply_slot(news, manager, 0xb, 1, news);
    } else {
        exe::contract_manager_offer_contract(read_usize(G_CONTRACT_MANAGER), person, 0, 0, -1, 1.05, 2); // sxA_1: the 1.05 contract
    }
}

/// Category 4 (players and staff).
unsafe fn auto_reply_player_items(news: usize, manager: usize, national: bool) {
    let kind = read_u32(news);
    if kind == 0xbbf && !national {
        auto_reply_transfer_request(news, manager);
    } else if kind == 0xbc1 && !national {
        let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), staff_at(auto_reply_data(news, 2) as isize));
        if contract != 0 {
            let c = Contract(contract);
            c.set_squad_status((c.squad_status() & 0xf) | 0x30);
        }
        auto_reply_slot(news, manager, 6, 1, news);
        auto_reply_slot(news, manager, 9, -1, news);
    } else if kind == 0xbc2 && !national {
        auto_reply_slot(news, manager, 7, 1, news);
    }
    if read_u32(news) == 0xbb8 && !national {
        auto_reply_contract_offer(news, manager);
    }
}

/// `TRANSFER_MANAGER::accept_loan_offer` (thiscall(slot, mark), ret 8).
const FN_TRANSFER_MANAGER_ACCEPT_LOAN_OFFER: usize = 0x8c0660;

/// A loan bid answered by the selling club's prediction (`shortlist_manager_predict_selling_club_response` of seller,
/// player, no fee, buyer, no request, loan): 0 accepts (`accept_loan_offer`), otherwise rejected.
unsafe fn auto_reply_loan_bid(slot: usize, seller: usize, player: usize, buyer: usize) {
    let tm = read_usize(G_TRANSFER_MANAGER);
    if exe::shortlist_manager_predict_selling_club_response(seller, player, 0, buyer, -1, 1) == 0 {
        covered!("sxA_1 loan accepted");
        let accept: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_ACCEPT_LOAN_OFFER);
        accept(tm, slot, 0); // sxA_1: the loan accepted
    } else {
        exe::transfer_manager_reject_loan_offer(tm, slot, 0, 0);
    }
}

/// Category 5 (transfer offers).
unsafe fn auto_reply_transfer_items(news: usize, manager: usize, club: usize, national: bool) {
    let tm = read_usize(G_TRANSFER_MANAGER);
    let offer_slot = |k: usize| exe::transfer_manager_get_offer_slot(tm, auto_reply_data(news, k) as i32);
    let kind = read_u32(news);
    match kind {
        0xfa0 if !national => {
            let slot = offer_slot(6);
            let offer = Offer(read_usize(slot));
            if offer.offer_type() == 1 {
                let buyer = club_at(offer.buyer_club_id() as isize).addr();
                let player = staff_at(offer.target_staff_id() as isize);
                let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
                write_usize(G_CONTRACT_LOOKUP_RESULT, contract);
                let seller = if contract == 0 { 0 } else { club_at(Contract(contract).club() as isize).addr() };
                auto_reply_loan_bid(slot, seller, player.addr(), buyer);
            } else {
                exe::transfer_manager_ai_respond_to_bid(tm, slot, 0);
            }
            return;
        }
        0xfa2 if !national => {
            exe::transfer_manager_review_ai_club_bid(tm, offer_slot(6), 0);
            return;
        }
        0xfac if !national => {
            let slot = offer_slot(7);
            exe::transfer_manager_offer_contract_to_offer_player(tm, club_at(auto_reply_data(news, 5) as isize), staff_at(auto_reply_data(news, 0) as isize), read_usize(slot));
            return;
        }
        0xfa4 if !national => {
            Offer(read_usize(offer_slot(7))).set_response_round(3);
            return;
        }
        0xfa5 if !national => {
            let buyer = club_at(auto_reply_data(news, 16) as isize);
            let offer = read_usize(offer_slot(6));
            let mut reason = 0u32;
            if exe::transfer_manager_check_offer_restrictions(tm, offer, buyer, &mut reason, 1) == 1 {
                exe::transfer_manager_club_rejects_offer(tm, offer, buyer, reason);
            } else {
                exe::transfer_manager_confirm_offer_by_club(tm, offer, buyer);
            }
            return;
        }
        0xfa8 if !national => {
            exe::squad_manager_allocate_squad_number(read_usize(G_SQUAD_MANAGER), club, 0usize);
            auto_reply_slot(news, manager, 7, 1, news);
        }
        _ => {}
    }
    if read_u32(news) == 0xfab && !national {
        let slot = offer_slot(9);
        auto_reply_slot(news, manager, 7, 1, 0);
        auto_reply_slot(news, manager, 8, 0, 0);
        if Offer(read_usize(slot)).status() == 0x11 {
            exe::transfer_manager_withdraw_transfer_offer(tm, slot as *mut usize, 0, 6);
        } else {
            exe::transfer_manager_reject_transfer_offer(tm, slot as *mut usize, 0, 6);
        }
    }
    match read_u32(news) {
        0xfb4 if !national => {
            exe::transfer_manager_withdraw_transfer_offer(tm, offer_slot(0xb) as *mut usize, 0, 0);
        }
        0xfb5 if !national => {
            let seller = club_at(auto_reply_data(news, 5) as isize).addr();
            let slot = offer_slot(6);
            let offer = Offer(read_usize(slot));
            auto_reply_loan_bid(slot, seller, staff_at(offer.target_staff_id() as isize).addr(), club_at(offer.buyer_club_id() as isize).addr());
        }
        0xfb6 if !national => {
            exe::squad_manager_allocate_squad_number(read_usize(G_SQUAD_MANAGER), club_at(auto_reply_data(news, 4) as isize), 0usize);
            auto_reply_slot(news, manager, 7, 1, news);
        }
        0xfc0 => {
            let slot = offer_slot(6);
            if slot != 0 {
                exe::transfer_manager_submit_loan_contract_offer(tm, slot);
            }
        }
        _ => {}
    }
}

/// The automatic reply to one news item of the human's inbox, given for him while on holiday, sacked or hired.
///
/// `NewsItem::auto_respond(manager, club)`: without a manager, the "Error" box (line 0x18d3) and nothing. The item is
/// answered only when its action button (`exe::newsitem_get_action_button` into the shared text 0xde1f64) is the reply
/// button (2). `club` being a national team (`exe::database_is_national_team`) limits the answers to the national
/// ones. By category (`exe::newsitem_get_category`):
/// - 6, type 0x1774: a job offer (`auto_reply_job_offer`); then the category is read again;
/// - 2: a friendly request (6) accepted, a tournament invitation (9) accepted, 0x16 answered 1, the league squad
///   numbers (0x18), the European registration (0x19), a national call-up (0x1e), a call-up replacement (0x22) and
///   0x28 answered 3;
/// - 4: a transfer request (0xbbf), the squad status (0xbc1: +0x4f upper nibble 3), 0xbc2 answered, and a contract
///   offer (0xbb8);
/// - 5: loan bids (0xfa0, 0xfb5) by the seller's prediction, bids reviewed (0xfa2), the offer's contract (0xfac),
///   response round 3 (0xfa4), offer restrictions (0xfa5), a squad number (0xfa8, 0xfb6), withdrawn or rejected bids
///   (0xfab, 0xfb4) and the loan contract (0xfc0).
///
/// Defects of the original: in the tournament invitation, a club not among the invited ones leaves the index byte
/// uninitialised and the original reads the invitation mark through it; the copy runs the original routine in that
/// case instead of inventing the byte. The answer to 0xfab passes no copy (0) to `News::set_item_slot`. The national
/// call-up passes the date words in registers whose upper words keep the club index arithmetic (kept as is).
/// Original: `NewsItem::auto_respond` (0x784100, news.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn auto_respond(news: usize, manager: usize, club: usize) {
    let national = club != 0 && exe::database_is_national_team(club) != 0;
    if manager == 0 {
        show_error_at(0xa693ec, 0x18d3);
        return;
    }
    let mut button = 0u32;
    if exe::newsitem_get_action_button(news, manager, 0xde1f64, 0x7d0, &mut button) != 2 {
        return;
    }
    covered!("sxA_1 automatic reply");
    if exe::newsitem_get_category(news) == 6 && read_u32(news) == 0x1774 && !auto_reply_job_offer(news, manager) {
        return;
    }
    match exe::newsitem_get_category(news) {
        2 => {
            if !auto_reply_club_items(news, manager, national) {
                original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%)(news, manager, club);
            }
        }
        4 => auto_reply_player_items(news, manager, national),
        5 => auto_reply_transfer_items(news, manager, club, national),
        _ => {}
    }
}

/// Comparer for `auto_respond`: only the original runs (the replies move friendlies, jobs and transfers, too much state
/// to set side by side); the item's type and id must not change.
pub(crate) unsafe extern "thiscall" fn compare_auto_respond(news: usize, manager: usize, club: usize) {
    let (kind, id) = (read_u32(news), read_usize(news + news_item::ID));
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%)(news, manager, club);
    SCOREBOARD.record(read_u32(news) == kind && read_usize(news + news_item::ID) == id,
        || format!("auto_respond: item type {:#x} id {} became {:#x} {}", kind, id, read_u32(news), read_usize(news + news_item::ID)));
}
