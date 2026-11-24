// ---------- cx10_9: the texts of the receivers appeased and ground upgrade plans news (fin_prm.cpp, finance.CPP) ----------

/// Writes the headline or the body of the news that the club's bankers are satisfied (slot 0 the club id).
///
/// The headline: "Receivers appeased". The body: the club's bankers (long name) let it refuse and negotiate transfer
/// offers again. No links.
/// Original: `FinanceTable::write_receivers_appeased_news_text` (0x5a0380, fin_prm.cpp (finance.CPP), thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_receivers_appeased_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    finance_news_text(out, |buf| {
        if full == 0 {
            fmt(buf, 0x9c9da0); // "Receivers appeased"
            return;
        }
        let (name, g) = club_long_name(exe::newsitem_get_slot(news, 0));
        fmt(buf, 0x9c9d10, g, g, name); // the bankers let the club refuse offers again
    });
}

/// Writes the headline or the body of the news that the board plans to enlarge or redevelop the ground to enter a
/// competition (slots: 1 the club competition id, 2 the kind as a signed byte).
///
/// The headline: "Stadium expansion plans announced". The body: kind 0, plans to redevelop the stadium; 1, to enlarge
/// it; both to allow entry into the competition (long name and gender from the club competition table
/// `G_CLUB_COMP_TABLE`, 0x6b bytes a record). No links.
/// Defect of the original: a kind other than 0 or 1 formats nothing and the text is whatever its stack buffer held; the
/// copy leaves it empty.
/// Original: `FinanceTable::write_ground_upgrade_plans_news_text` (0x5a04c0, fin_prm.cpp (finance.CPP), thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_ground_upgrade_plans_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    finance_news_text(out, |buf| {
        if full == 0 {
            fmt(buf, 0x9c9b40); // "Stadium expansion plans announced"
            return;
        }
        let comp_id = exe::newsitem_get_slot(news, 1);
        let kind = exe::newsitem_get_slot(news, 2) as u8 as i8;
        let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp_id as i32 as isize as usize).wrapping_mul(0x6b)); // the club competition record size
        let (name, g) = (comp + competition::NAME, read_u8(comp + competition::GENDER_NAME) as i8 as i32 as u32);
        match kind {
            0 => fmt(buf, 0x9c9db4, g, g, name), // redevelop the stadium to enter <comp>
            1 => fmt(buf, 0x9c9e34, g, g, name), // enlarge the stadium to enter <comp>
            _ => 0,
        };
    });
}

pub(crate) unsafe extern "thiscall" fn compare_write_receivers_appeased_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_receivers_appeased_news_text(this, news, full, t),
        || format!("write_receivers_appeased_news_text(news {:#x}, full {})", news, full));
}

pub(crate) unsafe extern "thiscall" fn compare_write_ground_upgrade_plans_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_ground_upgrade_plans_news_text(this, news, full, t),
        || format!("write_ground_upgrade_plans_news_text(news {:#x}, full {})", news, full));
}
