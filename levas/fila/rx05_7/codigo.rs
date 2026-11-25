/// Writes the headline or the body of the news that a player's contract cannot be extended because a revised work
/// permit is refused.
///
/// `write_contract_permit_refused_news_text(news, full, out)`: the person of slot 0 (name from slots 1-3; mode 1 in
/// the body, 5 in the headline) and the club of slot 4. The body: "<name>'s contract with <club> may not be extended
/// ..." (0xa9db94, the grammar marker 0x64 after the name, the club's short name and gender), the person's link
/// (0x7d7) and the club's (0x7d5). The headline: "<name> contract offer cancelled" (0xa9db58) and the person's link.
/// Original: `TRANSFER_MANAGER::write_contract_permit_refused_news_text` (0x8e4460, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_contract_permit_refused_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    let (person, s1, s2, s3, s4) = (slot(0), slot(1), slot(2), slot(3), slot(4));
    if full != 0 {
        news_text_person_name(s1, s2, s3, &mut name, 1, person);
        let club = club_at(s4 as i32 as isize);
        let g = club.short_gender_name() as i8 as i32;
        covered!("rx05_7 contract not extended body");
        fmt(NEWS_TEXT_BUFFER, 0xa9db94, 0x64, g, g, np, club.addr() + club::SHORT_NAME); // rx05_7: contract not extended
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
        news_text_link_club(out, s4);
    } else {
        news_text_person_name(s1, s2, s3, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0xa9db58, 0x64, np); // "<name> contract offer cancelled"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
    }
}

/// Comparer for `write_contract_permit_refused_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_contract_permit_refused_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_contract_permit_refused_news_text(this, news, full, t), || format!("write_contract_permit_refused_news_text(news {:#x}, full {})", news, full));
}

/// Writes the headline or the body of the news that a player's work permit is extended for a year, or that it
/// expires because he played too few games.
///
/// `write_work_permit_renewal_news_text(news, full, out)`: the person of slot 0 (name from slots 1-3; mode 1 in the
/// body, 5 in the headline), slot 4 renewed (≠ 0), slot 5 the nation. The body: renewed, "<name>'s work permit has
/// been extended for another year." (0xa9d8d0); otherwise "... has expired and will not be renewed due to him not
/// having played enough games in <nation>." (0xa9d82c, the nation's name and gender); the person's link (0x7d7) and
/// the nation's, to its national team (`exe::database_get_national_team`, 0x7d5), in both. The headline: "work permit
/// renewed" (0xa9d7f4) or "work permit expires" (0xa9d7bc) and the person's link.
/// Original: `TRANSFER_MANAGER::write_work_permit_renewal_news_text` (0x8e3b80, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_work_permit_renewal_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    let (person, s1, s2, s3, renewed, s5) = (slot(0), slot(1), slot(2), slot(3), slot(4), slot(5));
    if full != 0 {
        news_text_person_name(s1, s2, s3, &mut name, 1, person);
        let nation = Nation(nation_by_id(s5 as usize));
        if renewed != 0 {
            fmt(NEWS_TEXT_BUFFER, 0xa9d8d0, 0x64, np); // "... extended for another year."
        } else {
            covered!("rx05_7 work permit expires");
            let g = nation.gender_name() as i8 as i32;
            fmt(NEWS_TEXT_BUFFER, 0xa9d82c, 0x64, g, g, np, nation.addr() + 4); // +4: the nation's name
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
        let team = exe::database_get_national_team(nation);
        exe::text_add_link(out, (nation.addr() + 4) as *const u8, 0x7d5, team);
    } else {
        news_text_person_name(s1, s2, s3, &mut name, 5, person);
        let t = if renewed != 0 { 0xa9d7f4 } else { 0xa9d7bc }; // "work permit renewed" / "work permit expires"
        fmt(NEWS_TEXT_BUFFER, t, 0x64, np);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
    }
}

/// Comparer for `write_work_permit_renewal_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_work_permit_renewal_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_work_permit_renewal_news_text(this, news, full, t), || format!("write_work_permit_renewal_news_text(news {:#x}, full {})", news, full));
}

/// Writes the headline or the body of the news that a proposed transfer collapsed because the buying club will not
/// release the funds.
///
/// `write_funds_refused_news_text(news, full, out)`: the selling club of slot 0 (read only for the body), the buying
/// club of slot 1, the person of slot 2 (name from slots 3-5; mode 1 in the body, 5 in the headline). The body:
/// "<name>'s proposed transfer from <seller> to <buyer> has collapsed as <buyer> will not release the funds"
/// (0xa98e74: the clubs' long names with the genders of their short names, the grammar marker 0x64 after the name),
/// the links to the person (0x7d7), the buyer and the seller (0x7d5). The headline: "<name> transfer to <buyer> falls
/// through" (0xa98e14) with the links to the person and the buyer.
///
/// Defect of the original: the body prints the clubs' long names (`club::NAME`) with the genders of their short
/// names (`club::SHORT_GENDER_NAME`, not `club::GENDER_NAME`). The copy keeps it.
/// Original: `TRANSFER_MANAGER::write_funds_refused_news_text` (0x8dd830, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_funds_refused_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    let (s1, person, s3, s4, s5) = (slot(1), slot(2), slot(3), slot(4), slot(5));
    let buyer = club_at(s1 as i32 as isize);
    if full != 0 {
        let s0 = slot(0);
        news_text_person_name(s3, s4, s5, &mut name, 1, person);
        let seller = club_at(s0 as i32 as isize);
        let (gb, gs) = (buyer.short_gender_name() as i8 as i32, seller.short_gender_name() as i8 as i32);
        let (nb, ns) = (buyer.addr() + club::NAME, seller.addr() + club::NAME);
        covered!("rx05_7 funds refused body");
        fmt(NEWS_TEXT_BUFFER, 0xa98e74, 0x64, gs, gs, gb, gb, gb, gb, np, ns, nb, nb); // rx05_7: the funds refused
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, person, s3, s4, s5, 0x7d7);
        news_text_link_club(out, s1);
        news_text_link_club(out, s0);
    } else {
        news_text_person_name(s3, s4, s5, &mut name, 5, person);
        let g = buyer.short_gender_name() as i8 as i32;
        fmt(NEWS_TEXT_BUFFER, 0xa98e14, 0x64, g, g, np, buyer.addr() + club::SHORT_NAME); // "... falls through"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, person, s3, s4, s5, 0x7d7);
        news_text_link_club(out, s1);
    }
}

/// Comparer for `write_funds_refused_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_funds_refused_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_funds_refused_news_text(this, news, full, t), || format!("write_funds_refused_news_text(news {:#x}, full {})", news, full));
}
