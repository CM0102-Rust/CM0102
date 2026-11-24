// --- sx07_1: the news item that a person answered a contract offer (contract_manager.cpp) ---

/// The comparer's dry run of `contract_manager_post_offer_response_news`: the copy builds the news item and keeps, for
/// every club it would send it to, the club and the item's bytes, instead of sending it.
pub(crate) static OFFER_RESPONSE_NEWS_DRY_RUN: AtomicBool = AtomicBool::new(false);
/// The sends the dry run kept: (club, the news item's first `news_item::ID` bytes).
pub(crate) static OFFER_RESPONSE_NEWS_SENT: std::sync::Mutex<Vec<(usize, Vec<u8>)>> = std::sync::Mutex::new(Vec::new());

/// Sends the offer response news item to a club's manager (`exe::news_send_to_club_manager`), or keeps it in the dry
/// run.
unsafe fn offer_response_send(n: usize, club: Club) {
    if OFFER_RESPONSE_NEWS_DRY_RUN.load(Relaxed) {
        let bytes = (0..news_item::ID).map(|k| read_u8(n + k)).collect();
        OFFER_RESPONSE_NEWS_SENT.lock().unwrap_or_else(|e| e.into_inner()).push((club.addr(), bytes));
        return;
    }
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

/// Slot 0x11 of the offer response: with `refused` set, 0x3e7 when the current contract with the club ends before
/// `G_CONTRACT_DATE_PLUS_91_DAYS`, 0x3e8 when it ends before `G_CONTRACT_DATE_PLUS_365_DAYS`, otherwise 0 and `refused`
/// is cleared; without it, 0. The contract is read without a null check, as the original does.
unsafe fn offer_response_contract_end_code(contract: Contract, refused: &mut u32) -> u32 {
    if *refused == 0 {
        return 0;
    }
    let end = contract.addr() + contract::CONTRACT_EXPIRES;
    if !contract_manager_date_is_before(G_CONTRACT_DATE_PLUS_91_DAYS, end) {
        return 0x3e7;
    }
    if !contract_manager_date_is_before(G_CONTRACT_DATE_PLUS_365_DAYS, end) {
        return 0x3e8;
    }
    *refused = 0;
    0
}

/// Slot 0x1a of the offer response: an agreed player contract (type 2) of a player near the end of his career, by the
/// age he will have when the offer's contract ends (`exe::database_get_staff_age` on the offer's end date): a top
/// goalkeeper (`.goalkeeper()` = 20) over 37, or a defender, midfielder or attacker rated 15 or more over 36, 35 or 34.
unsafe fn offer_response_veteran(p: Staff, offer: Contract) -> u32 {
    let sheet = p.player();
    if sheet.is_null() || offer.contract_type() & 0x3f != 2 {
        return 0;
    }
    let end = (offer.addr() + contract::CONTRACT_EXPIRES) as *const u16;
    let age = || exe::database_get_staff_age(p, end);
    let veteran = (sheet.goalkeeper() == 0x14 && age() > 0x25)
        || (sheet.defender() >= 0xf && age() > 0x24)
        || (sheet.midfielder() >= 0xf && age() > 0x23)
        || (sheet.attacker() >= 0xf && age() > 0x22);
    veteran as u32
}

/// Posts news item 3000 (0xbb8) that a person accepted or rejected a contract offer, with the offer's terms, and picks
/// the clubs that receive it.
///
/// `post_offer_response_news(offer, refused)` (the contract manager in ecx; SEH frame for the local list and news
/// item). The offer is a `ContractOffer` (the `Contract` terms and the offer fields); the person is `staff_at` of its
/// id, the club `club_at` of `contract_offer::CLUB_ID`, and his contract with the club
/// (`exe::contract_manager_get_contract_for_club`, possibly none). The slots: 0 the person's id, 1-3 his name ids, 4 the
/// club's id, 5 `.state()` (signed), 6-7 the contract end (`contract::CONTRACT_EXPIRES`, signed day and year), 8 the
/// linked transfer offer slot (`.transfer_offer_slot()`), 10 the low 6 bits of `.contract_type()` (signed), 11 1 when
/// the offer's `.mark()` is not a player job (`exe::database_is_player_job`). With a linked transfer offer (slot ≠ −1):
/// its record (`exe::transfer_manager_get_offer_slot`); an empty slot gives the "Error" box (line 0x20c3) and nothing
/// is sent; otherwise slot 9 is its `offer::SERIAL`. Slot 12 the reasons the shortlist interest returns
/// (`exe::shortlist_manager_staff_interest_in_joining_club` with wage 0, the 6-bit contract type and the squad status
/// `.squad_status()` >> 4; the value itself is dropped). Slots 13-16 the club manager's id and name ids (−1 without
/// one). Slot 17 by `offer_response_contract_end_code` (only with `refused`, which it may clear). Slot 18 0x420e9,
/// slot 19 random(3), slot 20 1 when there is a contract with the club and the offer is agreed (state 2), 21 the
/// mark (signed), 22 the wage (`contract::WAGE`), 23 `contract::UNKNOWN`, 24-25 the start date
/// (`contract::DATE_STARTED`), 26 `offer_response_veteran`, 27 bit 7 of the person's `staff_state::STATUS_FLAGS`.
/// Who gets it: when `refused` is still set, or the agreed renewal of a contract with fewer than 180 days left
/// (`exe::staff_contract_days_to_end_date`), the clubs the transfer manager picks for the player's news
/// (`exe::transfer_manager_collect_player_news_recipients`, 0x21a, up to 30; none without a player sheet);
/// otherwise the club, and for an agreed player contract (type 2) also the clubs interested in him
/// (`exe::shortlist_manager_collect_interested_clubs` with 1, 1, 0), each but the club itself.
/// Original: `contract_manager_post_offer_response_news` (0x4d49a0, contract_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn contract_manager_post_offer_response_news(this: usize, offer: usize, refused: u32) {
    let mut refused = refused;
    let mut list = [0u8; 0xe];
    let l = Llist(list.as_mut_ptr() as usize);
    exe::new_llist(l);
    let terms = Contract(offer);
    let o = ContractOffer(offer);
    let p = staff_at(read_i32(offer) as isize);
    let club = club_at(o.club_id() as i32 as isize);
    let contract = Contract(exe::contract_manager_get_contract_for_club(this, p, club));
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbb8, 0);
    let set = |k: i8, v: u32| {
        exe::newsitem_set_slot(n, k, v);
    };
    set(0, read_u32(p.addr()));
    set(1, read_u32(p.first_name() + names::ID));
    set(2, read_u32(p.second_name() + names::ID));
    set(3, read_u32(p.common_name() + names::ID));
    set(4, o.club_id() as u32);
    set(5, o.state() as i8 as i32 as u32);
    set(6, terms.contract_expires() as i32 as u32);
    set(7, terms.contract_expires_year() as u16 as u32);
    set(8, o.transfer_offer_slot() as u32);
    let kind = ((terms.contract_type() << 2) as i8 >> 2) as i32; // sx07_1: the 6-bit contract type
    set(0xa, kind as u32);
    set(0xb, (exe::database_is_player_job(o.mark() as i8) == 0) as u32);
    if o.transfer_offer_slot() as i32 != -1 {
        let slot = read_usize(exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), o.transfer_offer_slot() as i32));
        if slot == 0 {
            show_error_at(0x9a8190, 0x20c3);
            exe::drop_llist(l);
            return;
        }
        set(9, read_u32(slot + offer::SERIAL));
    }
    let mut reasons = 0u32;
    let shortlists = read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER);
    exe::shortlist_manager_staff_interest_in_joining_club(shortlists, p, club, 0, kind as u8 as u32, (terms.squad_status() >> 4) as u32, (&mut reasons) as *mut u32 as u32);
    set(0xc, reasons);
    let manager = club.manager();
    if !manager.is_null() {
        set(0xd, read_u32(manager.addr()));
        set(0xe, read_u32(manager.first_name() + names::ID));
        set(0xf, read_u32(manager.second_name() + names::ID));
        set(0x10, read_u32(manager.common_name() + names::ID));
    } else {
        for k in 0xd..=0x10 {
            set(k, u32::MAX);
        }
    }
    let code = offer_response_contract_end_code(contract, &mut refused); // sx07_1: the contract end code
    set(0x11, code);
    set(0x12, 0x420e9);
    set(0x13, exe::utils_random(3) as u32); // sx07_1: the headline's draw
    let agreed = o.state() == 2;
    set(0x14, (!club.is_null() && !contract.is_null() && agreed) as u32);
    set(0x15, o.mark() as i8 as i32 as u32);
    set(0x16, terms.wage() as u32);
    set(0x17, terms.unknown());
    set(0x18, read_i16(offer + contract::DATE_STARTED) as i32 as u32);
    set(0x19, terms.date_started_year() as u16 as u32);
    set(0x1a, offer_response_veteran(p, terms));
    set(0x1b, (read_u8(contract_manager_staff_state_record(read_usize(p.addr())) + staff_state::STATUS_FLAGS) >> 7) as u32);
    if refused != 0 || (!club.is_null() && !contract.is_null() && exe::staff_contract_days_to_end_date(contract) < 0xb4 && agreed) {
        covered!("sx07_1 offer response to the transfer recipients");
        let tm = read_usize(G_TRANSFER_MANAGER);
        let mut clubs = [0usize; 0x1e];
        let k = if tm != 0 && !p.player().is_null() {
            exe::transfer_manager_collect_player_news_recipients(tm, p, 0x21a, clubs.as_mut_ptr(), 0x1e) as i8
        } else {
            0
        };
        for &c in clubs.iter().take(k.max(0) as usize) {
            offer_response_send(n, Club(c));
        }
    } else {
        covered!("sx07_1 offer response to the club");
        offer_response_send(n, club_at(o.club_id() as i32 as isize)); // sx07_1: the club gets it
        if agreed && terms.contract_type() & 0x3f == 2 {
            exe::shortlist_manager_collect_interested_clubs(shortlists, p, l, 1, 1, 0);
            l.set_cursor(l.head());
            let mut x = exe::llist_get_next(l);
            while x != 0 {
                let id = read_usize(x + 4);
                if id != o.club_id() {
                    offer_response_send(n, club_at(id as i32 as isize));
                }
                x = exe::llist_get_next(l);
            }
        }
    }
    exe::drop_llist(l);
}

/// The news items that reached the human managers' inboxes since `before` (`human_inbox_positions`): the first
/// `news_item::ID` bytes of each, without `news_item::IMPORTANT`, which the delivery sets.
unsafe fn offer_response_delivered(before: &[(usize, i32, i32)]) -> Vec<Vec<u8>> {
    let news = read_usize(G_NEWS);
    let mut out = Vec::new();
    for &(e, start, next) in before {
        let (start_now, next_now) = (read_i32(e), read_i32(e + 4));
        if start_now == -1 || (start_now == start && next_now == next) {
            continue;
        }
        let mut k = if start_now == start { next } else { start_now };
        while k != next_now {
            let r = read_usize(news).wrapping_add((k as usize).wrapping_mul(NEWS_NEWS_RECORD_SIZE));
            let mut bytes: Vec<u8> = (0..news_item::ID).map(|j| read_u8(r + j)).collect();
            bytes[news_item::IMPORTANT] = 0;
            out.push(bytes);
            k += 1;
            if k == start_now + 0x64 {
                k = start_now;
            }
        }
    }
    out.sort();
    out
}

/// The 16 human inboxes (entry address, ring start, next slot), or none without the news object.
unsafe fn human_inbox_positions() -> Vec<(usize, i32, i32)> {
    let news = read_usize(G_NEWS);
    if news == 0 || read_usize(news + news_fields::HUMAN_INBOXES) == 0 {
        return Vec::new();
    }
    (0..0x10).map(|i| read_usize(news + news_fields::HUMAN_INBOXES) + i * 12).map(|e| (e, read_i32(e), read_i32(e + 4))).collect()
}

/// Comparer for `contract_manager_post_offer_response_news`, as a dry run: the copy runs first without sending (it
/// keeps each send), then the original from the same random generator state. Compares the generator's state and the
/// news items that reached the human managers' inboxes (the only thing the delivery keeps:
/// `send_to_club_manager` only delivers to a human manager) with the copy's sends to clubs with a human manager. The
/// empty linked transfer offer (the "Error" box) runs the original only.
pub(crate) unsafe extern "thiscall" fn compare_contract_manager_post_offer_response_news(this: usize, offer: usize, refused: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%);
    let slot = ContractOffer(offer).transfer_offer_slot() as i32;
    if slot != -1 && read_usize(exe::transfer_manager_get_offer_slot(read_usize(G_TRANSFER_MANAGER), slot)) == 0 {
        return o(this, offer, refused);
    }
    let inboxes = human_inbox_positions();
    let rng = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    OFFER_RESPONSE_NEWS_SENT.lock().unwrap_or_else(|e| e.into_inner()).clear();
    OFFER_RESPONSE_NEWS_DRY_RUN.store(true, Relaxed);
    crate::rng::pause_trace(|| crate::rng::without_counting(|| contract_manager_post_offer_response_news(this, offer, refused)));
    OFFER_RESPONSE_NEWS_DRY_RUN.store(false, Relaxed);
    let rng_rust = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let sent = core::mem::take(&mut *OFFER_RESPONSE_NEWS_SENT.lock().unwrap_or_else(|e| e.into_inner()));
    rng.write_back();
    o(this, offer, refused);
    let rng_original = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let staff_count = read_i32(G_STAFF_COUNT);
    let mut expected: Vec<Vec<u8>> = sent
        .iter()
        .filter(|(c, _)| {
            let m = Club(*c).manager();
            !m.is_null() && read_i32(m.addr()) >= staff_count.wrapping_sub(0x10)
        })
        .map(|(_, b)| {
            let mut b = b.clone();
            b[news_item::IMPORTANT] = 0;
            b
        })
        .collect();
    expected.sort();
    let delivered = offer_response_delivered(&inboxes);
    let same = rng_original == rng_rust && (inboxes.is_empty() || delivered == expected);
    SCOREBOARD.record(same, || {
        format!(
            "contract_manager_post_offer_response_news(offer {:#x}, refused {}) rng same {}; {} sends, human items original {:x?} rust {:x?}",
            offer,
            refused,
            rng_original == rng_rust,
            sent.len(),
            delivered,
            expected
        )
    });
}
