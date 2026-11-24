// ---------- sbA_3: the text of the board's finance news (fin_prm.cpp, finance.CPP) ----------

/// A cash amount as the finance news show it: `Cash::get_display_amount` with flag 4 and `cash_format_money` into a
/// 1000-byte buffer.
unsafe fn finance_news_money(amount: u32, buf: &mut [u8; 1000]) {
    let cash = [amount];
    exe::cash_format_money(buf.as_mut_ptr() as usize, 1000, exe::cash_get_display_amount(cash.as_ptr(), 4), 4);
}

/// Writes the headline or the body of the board's finance news (slots: 0 the club id, 1 the first amount, 2 the second
/// amount, 3 the kind).
///
/// The headline by the kind: 1 "Transfer funds made available", 2 "New Board take control of" the club (short name,
/// with its link), 3 and 4 the first amount and "Cash Injection", 5 "Share issue announced", 6 "New sponsorship deal
/// announced", 7 the club (short name, link) "become a PLC". The body, with the two amounts as money (flag 4): kind 1,
/// a second amount above the club's reputation² × 0.025 (truncated) reads, above 50000, both amounts and else the
/// first; at or below it, a negative or a not negative wording with the first; kind 2, the new board of the club (long
/// name, link), with the second amount when it is above 50000 and above that limit, else with the first when it is
/// above 50000, else alone; kinds 3 to 6 one wording each with the first amount; kind 7 the club becoming a PLC (long
/// name, link).
/// Defect of the original: a kind outside 1 to 7 formats nothing and the text is whatever its stack buffer held (the
/// kinds the game posts are 1 to 7); the copy leaves it empty.
/// Original: `FinanceTable::write_finance_boost_news_text` (0x59f3e0, fin_prm.cpp (finance.CPP), thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_finance_boost_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let mut text = [0u8; FINANCE_TEXT_SIZE];
    let buf = text.as_mut_ptr() as usize;
    if full == 0 {
        let id = exe::newsitem_get_slot(news, 0);
        let first = exe::newsitem_get_slot(news, 1);
        let kind = exe::newsitem_get_slot(news, 3);
        let club = club_at(id as i32 as isize);
        let short = club.addr() + club::SHORT_NAME;
        let g = read_u8(club.addr() + club::SHORT_GENDER_NAME) as i8 as i32 as u32;
        match kind {
            1 => { fmt(buf, 0x9c8f98); } // "Transfer funds made available"
            2 => {
                fmt(buf, 0x9c8f60, g, g, short); // "New Board take control of {}<%s - Club (eg. Everton)>{}"
                exe::text_add_link(out, short as *const u8, 0x7d5, club);
            }
            3 | 4 => {
                let mut money = [0u8; 1000];
                finance_news_money(first, &mut money);
                fmt(buf, 0x9c8f00, money.as_ptr()); // "<%s - Cash Amount (eg. $2 million)> Cash Injection"
            }
            5 => { fmt(buf, 0x9c8fb8); } // "Share issue announced"
            6 => { fmt(buf, 0x9c8fd0); } // "New sponsorship deal announced"
            7 => {
                fmt(buf, 0x9c8f34, g, g, short); // "{}<%s - Club (eg. Everton)>{} become a PLC"
                exe::text_add_link(out, short as *const u8, 0x7d5, club);
            }
            _ => {}
        }
    } else {
        let id = exe::newsitem_get_slot(news, 0);
        let first = exe::newsitem_get_slot(news, 1);
        let second = exe::newsitem_get_slot(news, 2) as i32;
        let kind = exe::newsitem_get_slot(news, 3);
        let (mut a, mut b) = ([0u8; 1000], [0u8; 1000]);
        finance_news_money(first, &mut a);
        finance_news_money(second as u32, &mut b);
        let (a, b) = (a.as_ptr() as usize, b.as_ptr() as usize);
        let club = club_at(id as i32 as isize);
        let rep = club.reputation() as f64;
        let limit = || ftol(rep * rep * read_f64(rdata::F64_0_025));
        let (name, g) = club_long_name(id);
        match kind {
            1 => {
                if second > limit() {
                    if second > 50000 {
                        fmt(buf, 0x9c8ca4, a, b); // injected, now <b> available
                    } else {
                        fmt(buf, 0x9c8c10, a); // injected, only a very small amount
                    }
                } else if second < 0 {
                    fmt(buf, 0x9c8b78, a); // injected into the accounts, still in debt
                } else {
                    fmt(buf, 0x9c8ac8, a); // injected into the club, very little left
                }
            }
            2 => {
                if second > 50000 && second > limit() {
                    covered!("sbA_3 new board with funds");
                    fmt(buf, 0x9c8a00, g, g, name, b); // new board, debts cleared, <b> available
                } else if first as i32 > 50000 {
                    fmt(buf, 0x9c8934, g, g, name, a); // new board with <a>
                } else {
                    fmt(buf, 0x9c8878, g, g, name); // new board, a financial package agreed
                }
                exe::text_add_link(out, name as *const u8, 0x7d5, club);
            }
            3 => { fmt(buf, 0x9c8d90, a); } // the board paid <a> of the debts
            4 => { fmt(buf, 0x9c8d2c, a); } // injected for the running costs
            5 => { fmt(buf, 0x9c8e08, a); } // <a> from new shares
            6 => { fmt(buf, 0x9c8e7c, a); } // a new sponsorship deal of <a>
            7 => {
                fmt(buf, 0x9c87d0, g, g, name); // the club became a PLC
                exe::text_add_link(out, name as *const u8, 0x7d5, club);
            }
            _ => {}
        }
    }
    exe::text_set_string(out, buf as *const u8);
}

pub(crate) unsafe extern "thiscall" fn compare_write_finance_boost_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_finance_boost_news_text(this, news, full, t),
        || format!("write_finance_boost_news_text(news {:#x}, full {})", news, full));
}
