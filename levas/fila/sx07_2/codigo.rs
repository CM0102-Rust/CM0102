// --- sx07_2: the text of the contract offer answer news (contract_manager.cpp, 5,424 bytes), in named pieces ---

/// What `write_offer_response_news_text` reads from news item 0xbb8 and prepares before writing (the original's frame).
pub(crate) struct OfferText {
    person: u32,
    names: [u32; 3],
    club: Club,
    state: u8,
    trial: bool,
    reasons: u32,
    manager: i32,
    manager_names: [u32; 3],
    end_code: u32,
    custom: bool,
    draw: i32,
    renewal: bool,
    job: u32,
    wage: i32,
    veteran: bool,
    rebel: bool,
    age: i8,
    home: bool,
    years: i32,
    value: i32,
    name: [u8; 0x190],
    surname: [u8; 0x190],
    manager_name: [u8; 0x190],
    end_date: [u8; 0x12c],
    money: [u8; 0x64],
    reason: [[u8; 0xfa]; 5],
    joined: [u8; 0x400],
    count: i8,
}

impl OfferText {
    fn ptr(b: &[u8]) -> usize {
        b.as_ptr() as usize
    }
    /// The club's short name and its gender (pushed twice).
    unsafe fn short(&self) -> (i32, usize) {
        (self.club.short_gender_name() as i8 as i32, self.club.addr() + club::SHORT_NAME)
    }
    /// The club's full name and its gender (pushed twice).
    unsafe fn long(&self) -> (i32, usize) {
        (self.club.gender_name() as i32, self.club.addr() + club::NAME)
    }
}

/// Reads the slots of news item 0xbb8 and prepares the names, the contract end date, the contract value and its years.
///
/// The slots: 0 the person, 1-3 his name ids, 4 the club, 5 the offer state, 6-7 the contract end, 10 the contract type
/// (5 a trial), 12 the reasons, 13 the club manager and 14-16 his name ids, 17 the contract end code, 18 0x420e9 for
/// the plain offer, 19 the draw, 20 the renewal, 21 the job, 22 the wage, 23 the signing value, 24-25 the contract
/// start, 26 the veteran, 27 the contract rebel. The days from the start to the end (365 per year, the days of the year
/// apart) give the value (whole weeks × the wage + slot 23; from 3,000,000 down to the million, above 1,000,000 down to
/// the half million, as money with `exe::cash_format_money`) and the years (days ÷ 365, rounded). "At home": the club
/// is the human manager's on screen. The names: the person's in modes 1 and 5, the manager's in mode 1 when slot 17 is
/// not negative (with slot 17 as the id, as the original passes it), the end date as a long date.
unsafe fn offer_text_read(news: usize) -> OfferText {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let person = slot(0);
    let names = [slot(1), slot(2), slot(3)];
    let club = club_at(slot(4) as i32 as isize);
    let state = slot(5) as u8;
    let age = read_i8(staff_at(person as i32 as isize).addr() + staff::AGE);
    let mut end = [0u16; 4];
    let (end_year, end_day) = (slot(7), slot(6));
    exe::new_date(end.as_mut_ptr() as usize, end_day, end_year);
    let trial = slot(0xa) as u8 == 5;
    let reasons = slot(0xc);
    let manager = slot(0xd) as i32;
    let manager_names = [slot(0xe), slot(0xf), slot(0x10)];
    let end_code = slot(0x11);
    let custom = slot(0x12) != 0x420e9;
    let draw = slot(0x13) as i32;
    let renewal = slot(0x14) != 0;
    let job = slot(0x15);
    let wage = slot(0x16) as i32;
    let signing = slot(0x17) as i32;
    let mut start = [0u16; 4];
    let (start_year, start_day) = (slot(0x19), slot(0x18));
    exe::new_date(start.as_mut_ptr() as usize, start_day, start_year);
    let days = ((end[1] as i32).wrapping_sub(start[1] as i32)).wrapping_mul(365).wrapping_sub(start[0] as i16 as i32).wrapping_add(end[0] as i16 as i32);
    let mut value = (days / 7).wrapping_mul(wage).wrapping_add(signing); // sx07_2: the contract value
    if value >= 3_000_000 {
        value = value / 1_000_000 * 1_000_000;
    } else if value > 1_000_000 {
        value = value / 500_000 * 500_000;
    }
    let mut t = OfferText {
        person,
        names,
        club,
        state,
        trial,
        reasons,
        manager,
        manager_names,
        end_code,
        custom,
        draw,
        renewal,
        job,
        wage,
        veteran: false,
        rebel: false,
        age,
        home: false,
        years: 0,
        value,
        name: [0; 0x190],
        surname: [0; 0x190],
        manager_name: [0; 0x190],
        end_date: [0; 0x12c],
        money: [0; 0x64],
        reason: [[0; 0xfa]; 5],
        joined: [0; 0x400],
        count: 0,
    };
    let amount = exe::cash_get_display_amount((&value) as *const i32 as *const u32, 6);
    exe::cash_format_money(t.money.as_mut_ptr() as usize, 0x64, amount, 6);
    t.years = ftol(days as f64 * read_f64(rdata::F64_0_002739726027) + read_f64(rdata::F64_0_5));
    t.veteran = slot(0x1a) != 0;
    t.rebel = slot(0x1b) != 0;
    let hm = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
    t.home = hm != 0 && read_usize(hm + 0x39) != 0 && read_u32(read_usize(hm + 0x39)) == slot(4); // sx07_2: at home
    let [n1, n2, n3] = t.names;
    exe::database_format_name_from_ids(n1, n2, n3, t.name.as_mut_ptr(), 0x190, 1, person);
    exe::database_format_name_from_ids(n1, n2, n3, t.surname.as_mut_ptr(), 0x190, 5, person);
    if end_code as i32 > -1 {
        let [m1, m2, m3] = t.manager_names;
        exe::database_format_name_from_ids(m1, m2, m3, t.manager_name.as_mut_ptr(), 0x190, 1, end_code);
    }
    exe::date_get_long_date_string(end.as_mut_ptr() as usize, t.end_date.as_mut_ptr() as usize, 0x12c, 0xff);
    t
}

/// Joins the reasons gathered so far into `joined`: one is copied (`"%s"`), two go into 0x9a9f28 ("... and ...").
unsafe fn offer_text_join_reasons(t: &mut OfferText) {
    let fmt = news_text_format();
    match t.count {
        1 => {
            let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
            sprintf(OfferText::ptr(&t.joined), 0x991944, OfferText::ptr(&t.reason[0]));
        }
        2 => {
            fmt(OfferText::ptr(&t.joined), 0x9a9f28, OfferText::ptr(&t.reason[0]), OfferText::ptr(&t.reason[1]));
        }
        _ => {}
    }
}

/// The headline. Agreed (state 2): a trial 0x9aa414; a custom offer 0x9aa2a0; the renewal of a job 11 contract by the
/// draw (0 0x9aa320, 1 0x9aa368 with the years, otherwise 0x9aa3cc); away 0x9aa2d8 with the club; at home 0x9aa2a0.
/// Rejected (state 1 or 3): a trial 0x9aa26c; a custom offer 0x9aa234; the renewal of a job 11 contract: a contract
/// rebel 0x9aa1ec, otherwise by the draw (0 0x9aa104, 1 0x9aa154 with the years, otherwise 0x9aa1bc); away 0x9aa0bc with
/// the club; at home 0x9aa234. Another state: the "Error" box (line 0x223b) and nothing written (false).
unsafe fn offer_text_headline(t: &OfferText) -> bool {
    let fmt = news_text_format();
    let s = OfferText::ptr(&t.surname);
    let (g, short) = t.short();
    match t.state {
        2 => {
            if t.trial {
                fmt(NEWS_TEXT_BUFFER, 0x9aa414, s);
            } else if t.custom {
                fmt(NEWS_TEXT_BUFFER, 0x9aa2a0, s);
            } else if t.job == 0xb && t.renewal {
                match t.draw {
                    0 => fmt(NEWS_TEXT_BUFFER, 0x9aa320, g, g, s, short),
                    1 => fmt(NEWS_TEXT_BUFFER, 0x9aa368, g, g, s, t.years, short),
                    _ => fmt(NEWS_TEXT_BUFFER, 0x9aa3cc, g, g, short, s),
                };
            } else if !t.home {
                fmt(NEWS_TEXT_BUFFER, 0x9aa2d8, g, g, s, short); // sx07_2: accepts the club's offer
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9aa2a0, s);
            }
        }
        1 | 3 => {
            if t.trial {
                fmt(NEWS_TEXT_BUFFER, 0x9aa26c, s);
            } else if t.custom {
                fmt(NEWS_TEXT_BUFFER, 0x9aa234, s);
            } else if t.job == 0xb && t.renewal {
                if t.rebel {
                    fmt(NEWS_TEXT_BUFFER, 0x9aa1ec, s);
                } else {
                    match t.draw {
                        0 => fmt(NEWS_TEXT_BUFFER, 0x9aa104, g, g, s, short),
                        1 => fmt(NEWS_TEXT_BUFFER, 0x9aa154, g, g, s, t.years, short),
                        _ => fmt(NEWS_TEXT_BUFFER, 0x9aa1bc, s),
                    };
                }
            } else if !t.home {
                fmt(NEWS_TEXT_BUFFER, 0x9aa0bc, g, g, s, short);
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9aa234, s);
            }
        }
        _ => {
            show_error_at(0x9a8190, 0x223b);
            return false;
        }
    }
    true
}

/// The reasons of an agreed offer: the love for the club (reason bit 8; with bit 0x40 and a manager, also the
/// admiration for him), or, without bit 8, a player under 24 with bit 0x40 and a manager: the belief in him; joined.
unsafe fn offer_text_accept_reasons(t: &mut OfferText) {
    let fmt = news_text_format();
    let mn = OfferText::ptr(&t.manager_name);
    if t.reasons & 8 != 0 {
        fmt(OfferText::ptr(&t.reason[0]), 0x9a9ff0);
        t.count = 1;
        if t.reasons & 0x40 != 0 && t.manager > -1 {
            fmt(OfferText::ptr(&t.reason[1]), 0x9a9fa8, mn);
            t.count = 2;
        }
    } else if t.reasons & 0x40 != 0 && t.age < 0x18 && t.manager > -1 {
        fmt(OfferText::ptr(&t.reason[0]), 0x9a9f4c, mn);
        t.count = 1;
    }
    offer_text_join_reasons(t);
}

/// The body of an agreed offer with reasons. With the admiration bit (0x40) and the veteran: with a manager, above
/// (draw + 1) million 0x9a9e2c (years, money, reasons), otherwise 0x9a9d3c (end date, reasons); without a manager, above
/// that 0x9a9c48 for a contract rebel or 0x9a9b6c (years, money), otherwise 0x9a9aac (end date). Then (not a veteran,
/// or without bit 0x40 and not a veteran) with the love bit (8) and one reason, by the draw: 0 the renewal 0x9a9044 or
/// 0x9a8f8c (end date); 1 above 1,000,000 the renewal 0x9a948c (rebel) or 0x9a936c, else 0x9a9274, and up to 1,000,000
/// the renewal 0x9a9190 else 0x9a90fc (years); otherwise the renewal by the years (one: 0x9a998c for a rebel, 0x9a97bc;
/// more: 0x9a9858 for a rebel, 0x9a9710) or, a new contract, 0x9a9664 when the person's id ends in 0 and the wage is
/// 8,000 or more, else 0x9a95d4. Otherwise a rebel with a wage of 5,000 or more 0x9a8edc, else 0x9a8e50 (reasons).
unsafe fn offer_text_accept_with_reasons(t: &OfferText, admiration: bool) {
    let fmt = news_text_format();
    let (n, s, mo, j, d) = (OfferText::ptr(&t.name), OfferText::ptr(&t.surname), OfferText::ptr(&t.money), OfferText::ptr(&t.joined), OfferText::ptr(&t.end_date));
    let (g, short) = t.short();
    let (gl, long) = t.long();
    let threshold = t.draw.wrapping_mul(5).wrapping_add(5).wrapping_mul(200_000);
    if t.veteran {
        if admiration && t.manager > -1 {
            if t.value > threshold {
                fmt(NEWS_TEXT_BUFFER, 0x9a9e2c, g, g, n, short, t.years, mo, j);
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9a9d3c, g, g, 100, n, short, d, j);
            }
        } else if t.value > threshold {
            if t.rebel {
                fmt(NEWS_TEXT_BUFFER, 0x9a9c48, g, g, n, short, t.years, mo);
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9a9b6c, g, g, n, short, t.years, mo);
            }
        } else {
            fmt(NEWS_TEXT_BUFFER, 0x9a9aac, g, g, 100, n, short, d);
        }
        return;
    }
    if t.reasons & 8 != 0 && t.count == 1 {
        covered!("sx07_2 love for the club");
        match t.draw {
            0 => {
                let tpl = if t.renewal { 0x9a9044 } else { 0x9a8f8c };
                fmt(NEWS_TEXT_BUFFER, tpl, gl, gl, 100, n, long, d);
            }
            1 => {
                if t.value > 1_000_000 {
                    if t.renewal {
                        let tpl = if t.rebel { 0x9a948c } else { 0x9a936c };
                        fmt(NEWS_TEXT_BUFFER, tpl, gl, gl, long, n, t.years, mo, s);
                    } else {
                        fmt(NEWS_TEXT_BUFFER, 0x9a9274, gl, gl, n, t.years, long, mo);
                    }
                } else if t.renewal {
                    fmt(NEWS_TEXT_BUFFER, 0x9a9190, gl, gl, long, n, t.years);
                } else {
                    fmt(NEWS_TEXT_BUFFER, 0x9a90fc, gl, gl, n, t.years, long);
                }
            }
            _ => {
                if t.renewal {
                    match (t.rebel, t.years == 1) {
                        (true, true) => fmt(NEWS_TEXT_BUFFER, 0x9a998c, gl, gl, 100, gl, gl, long, n, s, long),
                        (true, false) => fmt(NEWS_TEXT_BUFFER, 0x9a9858, gl, gl, 100, gl, gl, long, n, t.years, s, long),
                        (false, true) => fmt(NEWS_TEXT_BUFFER, 0x9a97bc, gl, gl, long, n),
                        (false, false) => fmt(NEWS_TEXT_BUFFER, 0x9a9710, gl, gl, long, n, t.years),
                    };
                } else if (t.person as i32) % 10 == 0 && t.wage >= 0x1f40 {
                    fmt(NEWS_TEXT_BUFFER, 0x9a9664, 100, gl, gl, n, t.years, long);
                } else {
                    fmt(NEWS_TEXT_BUFFER, 0x9a95d4, gl, gl, n, t.years, long);
                }
            }
        }
    } else if t.rebel && t.wage >= 0x1388 {
        fmt(NEWS_TEXT_BUFFER, 0x9a8edc, gl, gl, n, t.years, long, j);
    } else {
        fmt(NEWS_TEXT_BUFFER, 0x9a8e50, gl, gl, n, long, j);
    }
}

/// The body of an agreed offer without reasons. A veteran's renewal 0x9a8d90 (end date). Otherwise a renewal or a new
/// contract above (draw + 1) million: when the draw is at least the person's id modulo 3 0x9a8cd0 or 0x9a8ab8
/// (years, money), else 0x9a8c1c or 0x9a8a04; up to that, the renewal 0x9a8b64 or the new contract 0x9a895c (end date).
/// A veteran's new contract takes the new contract's texts.
unsafe fn offer_text_accept_plain(t: &OfferText) {
    let fmt = news_text_format();
    let (n, mo, d) = (OfferText::ptr(&t.name), OfferText::ptr(&t.money), OfferText::ptr(&t.end_date));
    let (g, short) = t.short();
    if t.veteran && t.renewal {
        fmt(NEWS_TEXT_BUFFER, 0x9a8d90, g, g, 100, n, short, d);
        return;
    }
    let renewal = !t.veteran && t.renewal;
    let threshold = t.draw.wrapping_mul(5).wrapping_add(5).wrapping_mul(200_000);
    if t.value > threshold {
        if t.draw >= (t.person as i32) % 3 {
            let tpl = if renewal { 0x9a8cd0 } else { 0x9a8ab8 };
            fmt(NEWS_TEXT_BUFFER, tpl, g, g, n, short, t.years, mo);
        } else if renewal {
            fmt(NEWS_TEXT_BUFFER, 0x9a8c1c, g, g, short, n, t.years, mo);
        } else {
            fmt(NEWS_TEXT_BUFFER, 0x9a8a04, g, g, n, t.years, short, mo);
        }
    } else {
        let tpl = if renewal { 0x9a8b64 } else { 0x9a895c }; // sx07_2: the plain agreement
        fmt(NEWS_TEXT_BUFFER, tpl, g, g, 100, n, short, d);
    }
}

/// The body of an agreed offer: a trial 0x9aa010 (end date); otherwise the reasons (`offer_text_accept_reasons`), then a
/// custom offer 0x9a8b64 (end date), with reasons `offer_text_accept_with_reasons`, without them
/// `offer_text_accept_plain`.
unsafe fn offer_text_accepted_body(t: &mut OfferText) {
    let fmt = news_text_format();
    let (g, short) = t.short();
    if t.trial {
        fmt(NEWS_TEXT_BUFFER, 0x9aa010, g, g, 100, OfferText::ptr(&t.name), short, OfferText::ptr(&t.end_date));
        return;
    }
    offer_text_accept_reasons(t);
    if t.custom {
        fmt(NEWS_TEXT_BUFFER, 0x9a8b64, g, g, 100, OfferText::ptr(&t.name), short, OfferText::ptr(&t.end_date));
    } else if t.count != 0 {
        offer_text_accept_with_reasons(t, t.reasons & 0x40 != 0);
    } else {
        offer_text_accept_plain(t);
    }
}

/// The body of a rejected offer (state 1): a trial 0x9a88e8 with the club's short name. Otherwise the
/// reasons: unhappy with the manager (reason bit 0x100, with a manager) and wanting to get away (bit 4), joined; the
/// person's name again (300 bytes). Away: with slot 17 = 0x3e7 0x9a87a0 / 0x9a8704, with 0x3e8 0x9a8650 / 0x9a85c0 (with
/// / without the reasons); otherwise (or at home) 0x9a852c / 0x9a84bc, all with the club's full name.
unsafe fn offer_text_rejected_body(t: &mut OfferText) {
    let fmt = news_text_format();
    if t.trial {
        let (g, short) = t.short();
        fmt(NEWS_TEXT_BUFFER, 0x9a88e8, g, g, OfferText::ptr(&t.name), short);
        return;
    }
    if t.reasons & 0x100 != 0 && t.manager > -1 {
        fmt(OfferText::ptr(&t.reason[0]), 0x9a8898, OfferText::ptr(&t.manager_name));
        t.count = 1;
    }
    if t.reasons & 4 != 0 {
        fmt(OfferText::ptr(&t.reason[t.count as usize]), 0x9a8860);
        t.count += 1;
    }
    offer_text_join_reasons(t);
    let [n1, n2, n3] = t.names;
    exe::database_format_name_from_ids(n1, n2, n3, t.name.as_mut_ptr(), 0x12c, 1, t.person);
    let (n, j) = (OfferText::ptr(&t.name), OfferText::ptr(&t.joined));
    let (gl, long) = t.long();
    let with = t.count != 0;
    match (t.home, t.end_code) {
        (false, 0x3e7) => {
            if with {
                fmt(NEWS_TEXT_BUFFER, 0x9a87a0, gl, gl, n, long, j)
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9a8704, gl, gl, n, long)
            }
        }
        (false, 0x3e8) => {
            if with {
                fmt(NEWS_TEXT_BUFFER, 0x9a8650, 100, gl, gl, n, long, j)
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9a85c0, 100, gl, gl, n, long)
            }
        }
        _ => {
            if with {
                fmt(NEWS_TEXT_BUFFER, 0x9a852c, gl, gl, n, long, j) // sx07_2: rejected with reasons
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9a84bc, gl, gl, n, long)
            }
        }
    };
}

/// The links: the club's short name (0x7d5); the person's two names (0x7d7) when the news item's names still match him
/// (`exe::database_staff_matches_names`); the manager's name (0x7d7) when there is one and his names still match.
unsafe fn offer_text_links(t: &OfferText, out: usize) {
    exe::text_add_link(out, (t.club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, t.club);
    let p = staff_at(t.person as i32 as isize);
    let [n1, n2, n3] = t.names;
    if exe::database_staff_matches_names(p, n1, n2, n3, 0usize) != 0 {
        exe::text_add_link(out, t.name.as_ptr(), 0x7d7, p);
        exe::text_add_link(out, t.surname.as_ptr(), 0x7d7, p);
    }
    if t.manager > -1 {
        let m = staff_at(t.manager as isize);
        let [m1, m2, m3] = t.manager_names;
        if exe::database_staff_matches_names(m, m1, m2, m3, 0usize) != 0 {
            exe::text_add_link(out, t.manager_name.as_ptr(), 0x7d7, m);
        }
    }
}

/// Writes the headline or the body of news item 0xbb8, that a person accepted or rejected a contract or trial offer.
///
/// `write_offer_response_news_text(news, full, out)` (the contract manager in ecx, not read): reads the item
/// (`offer_text_read`). The headline (`full` = 0) by `offer_text_headline`; another state than 1, 2 or 3 gives the
/// "Error" box and writes nothing. The body: agreed (state 2) `offer_text_accepted_body`, rejected (state 1)
/// `offer_text_rejected_body`, state 3 0x9a841c (willing to negotiate, with the club's short name); another state
/// leaves the shared text buffer as it was. Then the text (`exe::text_set_string`) and the links (`offer_text_links`).
/// Defect of the original: the manager's name is formatted with slot 17 (the contract end code) as the person id and
/// only when that is not negative; the body of another state shows whatever the shared buffer had.
/// Original: `contract_manager_write_offer_response_news_text` (0x4d4fb0, contract_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn contract_manager_write_offer_response_news_text(_this: usize, news: usize, full: u32, out: usize) {
    covered!("sx07_2 offer response text");
    let mut t = offer_text_read(news);
    if full == 0 {
        if !offer_text_headline(&t) {
            return;
        }
    } else {
        match t.state {
            2 => offer_text_accepted_body(&mut t),
            1 => offer_text_rejected_body(&mut t),
            3 => {
                let (g, short) = t.short();
                news_text_format()(NEWS_TEXT_BUFFER, 0x9a841c, g, g, OfferText::ptr(&t.name), short);
            }
            _ => {}
        }
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    offer_text_links(&t, out);
}

/// Comparer for `contract_manager_write_offer_response_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_contract_manager_write_offer_response_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(
        out,
        || o(this, news, full, out),
        |t| contract_manager_write_offer_response_news_text(this, news, full, t),
        || format!("contract_manager_write_offer_response_news_text(news {:#x}, full {})", news, full),
    );
}
