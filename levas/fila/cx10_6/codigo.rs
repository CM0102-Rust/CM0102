// ---------- cx10_6: the receivers appeased and ground upgrade plans news and the Date-led record constructor ----------

/// Posts the news 0x1783 that the club's bankers are satisfied (slot 0 the club id) to the club's manager.
///
/// SEH frame only for the local news item, no destructor on the normal path. Called from the week's finance events
/// (`leave_crisis`, out of the crisis). `exe::new_newsitem` (lea ecx, local; push 0; push 0x1783 -> thiscall, ret 8),
/// `exe::newsitem_set_slot` (thiscall, ret 8), `exe::news_send_to_club_manager` (ecx = `[G_NEWS]`; push club; push
/// &local -> thiscall, ret 8) with the club of the id (`club_at`).
/// Original: `finance_post_receivers_appeased_news` (0x5a02f0, fin_prm.cpp (finance.CPP), thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn finance_post_receivers_appeased_news(_this: usize, club_id: u32) {
    covered!("cx10_6 receivers appeased news");
    let mut n = [0u8; 0xe0];
    let p = n.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0x1783, 0);
    exe::newsitem_set_slot(p, 0, club_id);
    exe::news_send_to_club_manager(read_usize(G_NEWS), p, club_at(club_id as i32 as isize));
}

/// Posts the news 0x1780 that the board plans to enlarge or redevelop the ground so the club can enter a competition
/// (slots: 0 the club id, 1 `comp`, 2 `kind` as a signed byte) to the club's manager.
///
/// SEH frame only for the local news item, no destructor on the normal path. Called twice from the stadium upgrade for
/// a league's minimums (`Finance::upgrade_stadium_for_league`, 0x599e00). Calls as in the receivers news.
/// Original: `finance_post_ground_upgrade_plans_news` (0x5a0410, fin_prm.cpp (finance.CPP), thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn finance_post_ground_upgrade_plans_news(_this: usize, club_id: u32, comp: u32, kind: u32) {
    let mut n = [0u8; 0xe0];
    let p = n.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0x1780, 0);
    exe::newsitem_set_slot(p, 0, club_id);
    exe::newsitem_set_slot(p, 1, comp);
    exe::newsitem_set_slot(p, 2, kind as u8 as i8 as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), p, club_at(club_id as i32 as isize));
}

/// Default-constructs a record that starts with a Date (the 16-byte fixed fixture and the other Date-led records that
/// share this constructor): the leading Date's default (`exe::date_date_54a110`), and returns the record.
/// Original: `FixedFixture::FixedFixture` (0x645800, ita_ser_a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn new_fixed_fixture(this: usize) -> usize {
    exe::date_date_54a110(this);
    this
}

/// Comparers: the posters send news (they cannot run twice), only the original runs; the constructor side by side,
/// the original on the record and the copy on a local 16-byte record (the Date's 8 bytes and the return).
pub(crate) unsafe extern "thiscall" fn compare_finance_post_receivers_appeased_news(this: usize, club_id: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%)(this, club_id)
}

pub(crate) unsafe extern "thiscall" fn compare_finance_post_ground_upgrade_plans_news(this: usize, club_id: u32, comp: u32, kind: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32)>(%%I1%%)(this, club_id, comp, kind)
}

pub(crate) unsafe extern "thiscall" fn compare_new_fixed_fixture(this: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%)(this);
    let mut local = [0u8; 16];
    let lp = local.as_mut_ptr() as usize;
    let c = new_fixed_fixture(lp);
    let original = core::slice::from_raw_parts(this as *const u8, 8);
    SCOREBOARD.record(r == this && c == lp && original == &local[..8],
        || format!("new_fixed_fixture({:#x}): original {:x?} rust {:x?}", this, original, &local[..8]));
    r
}
