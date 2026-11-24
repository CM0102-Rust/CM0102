// ---------- cx10_5: the finance boost, stand closed and financial crisis news (fin_prm.cpp, finance.CPP) ----------

/// A local news item of `kind` (`exe::new_newsitem` with 0) with the club id in data slot 0, as the finance news
/// posters build it; the 0xe0-byte frame of the original's local NewsItem.
unsafe fn finance_news_item(n: &mut [u8; 0xe0], kind: u32, club_id: u32) -> usize {
    let p = n.as_mut_ptr() as usize;
    exe::new_newsitem(p, kind, 0);
    exe::newsitem_set_slot(p, 0, club_id);
    p
}

/// Posts the board finance news 0x1779 (slots: 0 the club id, 1 `amount`, 2 `budget`, 3 `kind` as a signed byte).
/// Nothing is sent for a club of professional status 3. A takeover (kind 2) or a PLC flotation (kind 7) of a club with a
/// nation goes to every manager of the nation (`exe::news_send_to_nation_managers`); any other goes to the club's
/// manager (`exe::news_send_to_club_manager`).
///
/// SEH frame only for the local news item, no destructor on the normal path. Called from the finance boost
/// (`try_finance_boost_event`), the board takeover and the cash injection. `exe::new_newsitem` (lea ecx, local; push 0;
/// push 0x1779 -> thiscall, ret 8), `exe::newsitem_set_slot` (2 pushes -> thiscall, ret 8), the two senders (ecx =
/// `[G_NEWS]`; 2 pushes -> thiscall, ret 8).
/// Original: `finance_post_finance_boost_news` (0x59f300, fin_prm.cpp (finance.CPP), thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn finance_post_finance_boost_news(_this: usize, club: usize, amount: u32, budget: u32, kind: u32) {
    let club = Club(club);
    let kind = kind as u8 as i8;
    let mut n = [0u8; 0xe0];
    let p = finance_news_item(&mut n, 0x1779, read_u32(club.addr()));
    exe::newsitem_set_slot(p, 1, amount);
    exe::newsitem_set_slot(p, 2, budget);
    exe::newsitem_set_slot(p, 3, kind as i32 as u32);
    if club.professional_status() == 3 {
        return;
    }
    if (kind == 2 || kind == 7) && !club.nation().is_null() {
        covered!("cx10_5 boost news to the nation");
        exe::news_send_to_nation_managers(read_usize(G_NEWS), p, club.nation());
    } else {
        exe::news_send_to_club_manager(read_usize(G_NEWS), p, club);
    }
}

/// Posts the news 0x177a that the safety inspector closed part of the ground (slots: 0 the club id, 1 the stadium id or
/// -1 without a stadium, 2 `seats` as a signed word, 3 `kind` as a signed byte) to the club's manager.
///
/// SEH frame only for the local news item, no destructor on the normal path. Called from the week's finance events
/// (`process_club_finance_events`, squeeze above 50). `exe::new_newsitem`, `exe::newsitem_set_slot` and
/// `exe::news_send_to_club_manager` as in the boost news.
/// Original: `finance_post_stand_closed_news` (0x59f900, fin_prm.cpp (finance.CPP), thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn finance_post_stand_closed_news(_this: usize, club: usize, seats: u32, kind: u32) {
    let club = Club(club);
    let mut n = [0u8; 0xe0];
    let p = finance_news_item(&mut n, 0x177a, read_u32(club.addr()));
    let stadium = club.stadium();
    exe::newsitem_set_slot(p, 1, if stadium.is_null() { 0xffff_ffff } else { read_u32(stadium.addr()) });
    exe::newsitem_set_slot(p, 2, seats as u16 as i16 as i32 as u32);
    exe::newsitem_set_slot(p, 3, kind as u8 as i8 as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), p, club);
}

/// Posts the news 0x177b that the club is in financial crisis (slots: 0 the club id, 1 the wording, random(2) as a
/// signed word) to the club's manager.
///
/// SEH frame only for the local news item, no destructor on the normal path. Called from the week's finance events
/// (`process_club_finance_events`, finance band -2). `exe::utils_random` (push 2; add esp, 4 -> cdecl).
/// Original: `finance_post_financial_crisis_news` (0x59fc60, fin_prm.cpp (finance.CPP), thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn finance_post_financial_crisis_news(_this: usize, club: usize) {
    covered!("cx10_5 financial crisis news");
    let mut n = [0u8; 0xe0];
    let p = finance_news_item(&mut n, 0x177b, read_u32(club));
    let wording = exe::utils_random(2) as u16 as i16 as i32;
    exe::newsitem_set_slot(p, 1, wording as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), p, club);
}

/// Comparers: the posters send news (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_finance_post_finance_boost_news(this: usize, club: usize, amount: u32, budget: u32, kind: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(%%I0%%)(this, club, amount, budget, kind)
}

pub(crate) unsafe extern "thiscall" fn compare_finance_post_stand_closed_news(this: usize, club: usize, seats: u32, kind: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I1%%)(this, club, seats, kind)
}

pub(crate) unsafe extern "thiscall" fn compare_finance_post_financial_crisis_news(this: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, club)
}
