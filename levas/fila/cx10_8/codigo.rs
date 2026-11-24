// ---------- cx10_8: the texts of the dividend, stadium works and club relocation news (fin_prm.cpp, finance.CPP) ----------

/// Writes the headline or the body of the shareholder dividend news (slots: 1 the amount, 2 the PLC flag as a byte).
///
/// The headline: "Shareholders dividend payout". The body: the board announces dividends totalling the amount, shown
/// through `Cash::get_display_amount` (flags 4) and `cash_format_money` (0x3e8 bytes, flags 4). Both slots are read;
/// the PLC flag picks between two calls with the same line, so it changes nothing. No links. The text goes through a
/// local buffer (`FINANCE_TEXT_SIZE`).
/// Original: `FinanceTable::write_dividend_news_text` (0x59feb0, fin_prm.cpp (finance.CPP), thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_dividend_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    finance_news_text(out, |buf| {
        if full == 0 {
            fmt(buf, 0x9c9720); // "Shareholders dividend payout"
            return;
        }
        let amount = exe::newsitem_get_slot(news, 1);
        let _plc = exe::newsitem_get_slot(news, 2) as u8;
        let mut money = [0u8; 0x3e8];
        let mp = money.as_mut_ptr() as usize;
        exe::cash_format_money(mp, 0x3e8, exe::cash_get_display_amount(&amount, 4), 4);
        fmt(buf, 0x9c96b8, mp); // "The board have announced dividends totalling <amount> ..."
    });
}

/// Writes the headline or the body of the stadium works news (slots: 1 the number, 2 the kind as a signed byte).
///
/// The headline by kind: 4 "Stadium purchase announced", 5 "Shock stadium sale announced", 6 "Undersoil heating
/// installed", 7 "Stadium Facilities reduced", 8 "Training facility upgraded", any other "Stadium expansion plans
/// announced". The body by kind: 1 the stadium enlarged by the number of seats, 2 the standing area for that many
/// supporters, 3 that many seats installed, 4 the purchase, 5 the sale and 6 the undersoil heating of the stadium whose
/// id is the number (its name), 7 the heating failed and not repaired, 8 the training facilities completed. No links.
/// Defect of the original: a body kind outside 1..8 formats nothing and the text is whatever its stack buffer held; the
/// copy leaves it empty.
/// Original: `FinanceTable::write_stadium_works_news_text` (0x5a0050, fin_prm.cpp (finance.CPP), thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_stadium_works_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    finance_news_text(out, |buf| {
        if full == 0 {
            let line = match exe::newsitem_get_slot(news, 2) as u8 as i8 {
                4 => 0x9c9b9c, // "Stadium purchase announced"
                5 => 0x9c9bb8, // "Shock stadium sale announced"
                6 => 0x9c9b80, // "Undersoil heating installed"
                7 => 0x9c9bd8, // "Stadium Facilities reduced"
                8 => 0x9c9b64, // "Training facility upgraded"
                _ => 0x9c9b40, // "Stadium expansion plans announced"
            };
            fmt(buf, line);
            return;
        }
        let number = exe::newsitem_get_slot(news, 1);
        let kind = exe::newsitem_get_slot(news, 2) as u8 as i8;
        match kind {
            1 => fmt(buf, 0x9c987c, number), // enlarge the stadium by <n> seats
            2 => fmt(buf, 0x9c9800, number), // the standing area for <n> more supporters
            3 => fmt(buf, 0x9c97ac, number), // install <n> seats
            4 | 5 | 6 => {
                covered!("cx10_8 stadium works body with the stadium");
                let (name, g) = stadium_name(number);
                let line = match kind {
                    4 => 0x9c98cc, // the purchase
                    5 => 0x9c9958, // the sale
                    _ => 0x9c9a00, // the undersoil heating
                };
                fmt(buf, line, g, g, name)
            }
            7 => fmt(buf, 0x9c9ac8), // the heating failed
            8 => fmt(buf, 0x9c9740), // the training facilities completed
            _ => 0,
        };
    });
}

/// Writes the headline or the body of the club relocation news (slots: 0 the club id, 1 read and unused, 2 the stadium
/// id).
///
/// The headline: "Club relocation plans completed". The body: the club's board (long name) announces the move to its
/// new stadium (name). No links.
/// Original: `FinanceTable::write_club_relocation_news_text` (0x5a0230, fin_prm.cpp (finance.CPP), thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_club_relocation_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    finance_news_text(out, |buf| {
        if full == 0 {
            fmt(buf, 0x9c9cf0); // "Club relocation plans completed"
            return;
        }
        let club_id = exe::newsitem_get_slot(news, 0);
        exe::newsitem_get_slot(news, 1);
        let stadium_id = exe::newsitem_get_slot(news, 2);
        let (sname, sg) = stadium_name(stadium_id);
        let (cname, cg) = club_long_name(club_id);
        fmt(buf, 0x9c9bf4, cg, cg, sg, sg, cname, sname); // the club's move to its new stadium
    });
}

pub(crate) unsafe extern "thiscall" fn compare_write_dividend_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_dividend_news_text(this, news, full, t),
        || format!("write_dividend_news_text(news {:#x}, full {})", news, full));
}

pub(crate) unsafe extern "thiscall" fn compare_write_stadium_works_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_stadium_works_news_text(this, news, full, t),
        || format!("write_stadium_works_news_text(news {:#x}, full {})", news, full));
}

pub(crate) unsafe extern "thiscall" fn compare_write_club_relocation_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_club_relocation_news_text(this, news, full, t),
        || format!("write_club_relocation_news_text(news {:#x}, full {})", news, full));
}
