// ---------- cx10_7: the texts of the stand closed, works cancelled and financial crisis news (fin_prm.cpp, finance.CPP) ----------

/// The size of the local text buffer of the finance news text writers (`_chkstk` 0x1388).
const FINANCE_TEXT_SIZE: usize = 0x1388;

/// Runs `write` on a zeroed local text buffer of the finance news writers and gives it to Text `out`
/// (`exe::text_set_string`). A line the original never formats leaves its stack buffer as it was; the copy leaves it
/// empty (see each writer's defect).
unsafe fn finance_news_text(out: usize, write: impl FnOnce(usize)) {
    let mut text = [0u8; FINANCE_TEXT_SIZE];
    let tp = text.as_mut_ptr() as usize;
    write(tp);
    exe::text_set_string(out, tp as *const u8);
}

/// A club's long name and its gender byte (signed), as the finance news texts pass them.
pub(crate) unsafe fn club_long_name(id: u32) -> (usize, u32) {
    let club = club_at(id as i32 as isize);
    (club.addr() + club::NAME, club.gender_name() as i8 as i32 as u32)
}

/// A stadium's name and its gender byte (signed), from the stadium table (`G_STADIUM_TABLE`, `stadium::SIZE`).
pub(crate) unsafe fn stadium_name(id: u32) -> (usize, u32) {
    let s = read_usize(G_STADIUM_TABLE).wrapping_add((id as i32 as isize as usize).wrapping_mul(stadium::SIZE));
    (s + stadium::NAME, read_u8(s + stadium::GENDER_NAME) as i8 as i32 as u32)
}

/// Writes the headline or the body of the news that the safety inspector closed part of the ground (slots: 1 the
/// stadium id or -1, 2 the seats, 3 the kind as a signed byte).
///
/// The headline: "Stand closed" with a kind, else "Terracing closed due to lack of maintainance". The body: with a
/// stadium, a kind and seating (`stadium::SEATING_CAPACITY`), the west stand closed, seating reduced by the seats; a
/// kind and no seating, the north stand closed and no seats left; no kind, the east terrace closed and the capacity
/// (`stadium::CAPACITY`) minus the seats. Without a stadium: a kind, by the seating of the record before the table (see
/// the defect), the west stand with the kind as the number, or the north stand; no kind, the west stand and no seats
/// left. No links.
/// Defect of the original: without a stadium (id -1) the body reads the seating of the record before the stadium table
/// (`[G_STADIUM_TABLE] - 0xe`) and prints the kind (1) as the remaining seating.
/// Original: `FinanceTable::write_stand_closed_news_text` (0x59f9c0, fin_prm.cpp (finance.CPP), thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_stand_closed_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    finance_news_text(out, |buf| {
        if full == 0 {
            if exe::newsitem_get_slot(news, 3) != 0 {
                fmt(buf, 0x9c9300); // "Stand closed"
            } else {
                fmt(buf, 0x9c92d0); // "Terracing closed due to lack of maintainance"
            }
            return;
        }
        let stadium_id = exe::newsitem_get_slot(news, 1) as i32;
        let seats = exe::newsitem_get_slot(news, 2) as i32;
        let kind = exe::newsitem_get_slot(news, 3) as u8 as i8 as i32;
        if stadium_id != -1 {
            let s = Stadium(read_usize(G_STADIUM_TABLE).wrapping_add((stadium_id as isize as usize).wrapping_mul(stadium::SIZE)));
            if kind != 0 {
                if s.seating_capacity() != 0 {
                    fmt(buf, 0x9c9250, seats); // the west stand, seating reduced by <seats>
                } else {
                    fmt(buf, 0x9c9158); // the north stand, no seats remaining
                }
            } else if !s.is_null() {
                covered!("cx10_7 east terrace closed");
                fmt(buf, 0x9c91cc, s.capacity().wrapping_sub(seats)); // the east terrace, capacity down to <n>
            } else {
                fmt(buf, 0x9c9158);
            }
        } else if kind != 0 {
            if read_i32(read_usize(G_STADIUM_TABLE).wrapping_sub(0xe)) != 0 {
                fmt(buf, 0x9c90d0, kind); // the west stand, seating down to <kind>
            } else {
                fmt(buf, 0x9c9064); // the north stand, no seats remaining
            }
        } else {
            fmt(buf, 0x9c8ff0); // the west stand, no seats remaining
        }
    });
}

/// Writes the headline or the body of the news that stadium or training works are shelved, cancelled or refused
/// (slots: 1 the subject, 2 the reason).
///
/// Subjects 1 to 4 and 6 are the stadium, any other the training ground. The headline: for the stadium, reason 0
/// "Stadium Expansion plans shelved", 1 "Stadium Expansion cancelled", other "Local council blocks planning
/// permission"; else "Training ground improvements cancelled". The body: for the stadium, reason 0 or 1 the expansion
/// on hold for lack of funding, other the planning permission denied; else the training plans cancelled for lack of
/// funds. Both slots are read for the headline too. No links.
/// Original: `FinanceTable::write_works_cancelled_news_text` (0x59fb90, fin_prm.cpp (finance.CPP), thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_works_cancelled_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let subject = exe::newsitem_get_slot(news, 1) as i32;
    let reason = exe::newsitem_get_slot(news, 2) as i32;
    let stadium = subject > 0 && (subject <= 4 || subject == 6);
    finance_news_text(out, |buf| {
        if full == 0 {
            let line = match (stadium, reason) {
                (true, 0) => 0x9c9484, // "Stadium Expansion plans shelved"
                (true, 1) => 0x9c94a4, // "Stadium Expansion cancelled"
                (true, _) => 0x9c94c0, // "Local council blocks planning permission"
                (false, _) => 0x9c945c, // "Training ground improvements cancelled"
            };
            fmt(buf, line);
        } else if stadium {
            covered!("cx10_7 stadium works cancelled body");
            fmt(buf, if (0..=1).contains(&reason) { 0x9c93f0 } else { 0x9c9370 });
        } else {
            fmt(buf, 0x9c9310); // the training plans cancelled
        }
    });
}

/// Writes the headline or the body of the financial crisis news (slots: 0 the club id, 1 the wording as a signed word).
///
/// The headline: "Receivers called in" with a wording, else "Club in financial crisis". The body: wording 0, the
/// club's board called an emergency meeting; 1, receivers appointed (both: the club can no longer refuse offers); the
/// club's link (long name, 0x7d5) is added before the text is set.
/// Defect of the original: a wording other than 0 or 1 formats nothing and the text is whatever its stack buffer held
/// (the wording is random(2), so it never happens); the copy leaves it empty.
/// Original: `FinanceTable::write_financial_crisis_news_text` (0x59fcf0, fin_prm.cpp (finance.CPP), thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_financial_crisis_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let mut text = [0u8; FINANCE_TEXT_SIZE];
    let buf = text.as_mut_ptr() as usize;
    if full == 0 {
        if exe::newsitem_get_slot(news, 1) as u16 != 0 {
            fmt(buf, 0x9c96a4); // "Receivers called in"
        } else {
            fmt(buf, 0x9c9688); // "Club in financial crisis"
        }
    } else {
        let id = exe::newsitem_get_slot(news, 0);
        let wording = exe::newsitem_get_slot(news, 1) as u16 as i16;
        let (name, g) = club_long_name(id);
        match wording {
            0 => fmt(buf, 0x9c94ec, g, g, name), // the emergency board meeting
            1 => fmt(buf, 0x9c95c4, g, g, name), // the receivers appointed
            _ => 0,
        };
        let club = club_at(id as i32 as isize);
        exe::text_add_link(out, (club.addr() + club::NAME) as *const u8, 0x7d5, club);
    }
    exe::text_set_string(out, buf as *const u8);
}

pub(crate) unsafe extern "thiscall" fn compare_write_stand_closed_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_stand_closed_news_text(this, news, full, t),
        || format!("write_stand_closed_news_text(news {:#x}, full {})", news, full));
}

pub(crate) unsafe extern "thiscall" fn compare_write_works_cancelled_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_works_cancelled_news_text(this, news, full, t),
        || format!("write_works_cancelled_news_text(news {:#x}, full {})", news, full));
}

pub(crate) unsafe extern "thiscall" fn compare_write_financial_crisis_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_financial_crisis_news_text(this, news, full, t),
        || format!("write_financial_crisis_news_text(news {:#x}, full {})", news, full));
}
