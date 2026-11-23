// ---------- batch 324: the news text of a transfer set to complete (transfer_manager.cpp) ----------

/// `Date::get_date_string_no_weekday` (0x54c3b0; thiscall(date, out, size, time of day), ret 0xc): the date as
/// ordinal day, month name and year (time of day 0xff: none).
pub(crate) unsafe fn date_string_no_weekday(date: usize, out: usize, size: u32, time_of_day: u32) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x54c3b0usize);
    f(date, out, size, time_of_day)
}

/// The cash amount `value` as display text (`exe::cash_get_display_amount` and `exe::cash_format_money`, unit 6) into
/// `buf` (0x32 bytes).
pub(crate) unsafe fn news_text_money(value: u32, buf: &mut [u8; 0x34]) {
    let amount = exe::cash_get_display_amount(&value, 6);
    exe::cash_format_money(buf.as_mut_ptr() as usize, 0x32, amount, 6);
}

/// Writes the headline or the body of the news that a transfer, a contract or a player exchange is set to complete,
/// with the confirmations, the cancellations or a delay.
///
/// `write_transfer_finalised_news_text(news, full, out)` (the 0xfa5 news of `post_transfer_finalised_news`): slots 0
/// (the person), 1..3 (his name ids), 4 (the buyer), 5 (the seller, −1 none), 6 and 7 (the offer's index and serial),
/// 8 (the fee), 9 (the offer status), 10 (the total value), 11 (the exchange players), 12 (`pre_contract`), 13 (the
/// completion timing), 14 and 15 (the completion day and year), 17 (the job type).
///
/// First, for the human manager of the current slot (`G_SCREEN_RECORDS_MANAGER`), the status is brought up to date
/// unless it is already final (0xe, 3, 0x10, 2, 0x12, 0x11 or −1): the offer at the index (`G_TRANSFER_MANAGER`, an
/// index past the count reads the first) with the same serial gives its status (0: the news keeps its own); a missing
/// offer stores −1. At the manager's club (the buyer): 0x12 and 3 are stored as they are, 0xd, 0xe and 0x13 become
/// 0xd; at the other club: 0x11 and 2 as they are, 0xc, 0xe and 0x13 become 0xc; other statuses store nothing. Storing
/// goes through `News::set_item_slot` (slot 9, into the inbox and into this news item).
///
/// The body: the name (mode 1); the fee text (`news_text_money`) of the total when it is above the fee and there is no
/// pre-contract (with the exchange: 0xa9705c with the number of players, "a player" 0xa97210 or 0xa91ba0 with the
/// number in words, `exe::utils_number_to_words`; without: 0xa97158), otherwise of the fee: without the exchange, a fee
/// above 0 and no pre-contract 0xa96fa8, else with a seller 0xa96f10, else the contract offer 0xa96e98; with the
/// exchange, a fee above 0 0xa96da8, else 0xa96cfc. Then, with a completion timing, the completion date (0xa96ca8,
/// `date_string_no_weekday`) appended. Then by the status: 0xe both clubs confirmed (0xa96c7c with a seller, else the
/// buyer confirmed 0xa96c38); 0xd the buyer confirmed the transfer (0xa96c38 with a seller) or the deal (0xa96bf8);
/// with a seller, 0xc the seller confirmed (0xa96c38; nothing with a pre-contract); 3 the buyer cancelled (0xa96bb4);
/// 2 the seller cancelled (0xa96bb4; nothing without a seller); 0x10 cancelled (0xa96b90); 0x12 the buyer asked for a
/// week's delay (0xa96b3c), 0x11 the seller (0xa96b3c); otherwise nothing; a non-empty sentence is appended. The links
/// to the buyer, the seller (when ≥ 0) and the person (0x7d7) when the names still match.
///
/// The headline: the name (mode 5); a job that is not the plain player job (`exe::database_is_not_plain_player_job`)
/// with a pre-contract "set for role" (0xa96ae4), else with a total value "set to buy" (0xa96a90), else "set to sign"
/// (0xa96a3c); the links to the buyer and to the person.
/// Original: `TRANSFER_MANAGER::write_transfer_finalised_news_text` (0x8da8f0, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_transfer_finalised_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let mut status = slot(9) as u8;
    let hm = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
    if hm != 0 && ![0xe, 3, 0x10, 2, 0x12, 0x11, 0xff].contains(&status) {
        let index = slot(6) as i32;
        let serial = slot(7);
        let tm = read_usize(G_TRANSFER_MANAGER);
        let arr = read_usize(tm);
        let at = if index < read_i32(tm + 4) { arr.wrapping_add((index as usize).wrapping_mul(4)) } else { arr };
        let o = Offer(read_usize(at));
        let set: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, usize) = core::mem::transmute(FN_NEWS_SET_ITEM_SLOT); // News::set_item_slot
        let store = |v: u8| set(read_usize(G_NEWS), hm, read_u32(news + 0xd6), 9, v as i8 as i32, news);
        if o.is_null() || (o.serial() as u32) != serial {
            store(0xff);
            status = 0xff;
        } else {
            let st = o.status();
            if st != 0 {
                let buyer = club_at(o.buyer_club_id() as i32 as isize).addr();
                let new = if read_usize(hm + 0x39) == buyer {
                    match st { 0x12 | 3 => Some(st), 0xd | 0xe | 0x13 => Some(0xd), _ => None }
                } else {
                    match st { 0x11 | 2 => Some(st), 0xc | 0xe | 0x13 => Some(0xc), _ => None }
                };
                if let Some(v) = new {
                    covered!("gra_11 finalised status stored");
                    store(v);
                    status = v;
                }
            }
        }
    }
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    if full != 0 {
        covered!("gra_11 finalised body");
        let (person, s1, s2, s3, buyer, seller) = (slot(0), slot(1), slot(2), slot(3), slot(4), slot(5));
        let (fee, total) = (slot(8) as i32, slot(0xa) as i32);
        let swap = slot(0xb) as u8;
        let pre_contract = slot(0xc) as u8;
        let timing = slot(0xd) as u8;
        let (day, year) = (slot(0xe), slot(0xf));
        let mut money = [0u8; 0x34];
        let mut players = [0u8; 0x34];
        if swap != 0 {
            exe::utils_number_to_words(swap as i8 as i32, money.as_mut_ptr(), 0x32, 1);
            if swap == 1 {
                fmt(players.as_mut_ptr() as usize, 0xa97210); // "a player"
            } else {
                fmt(players.as_mut_ptr() as usize, 0xa91ba0, money.as_ptr() as usize); // "<number> players"
            }
        }
        news_text_person_name(s1, s2, s3, &mut name, 1, person);
        let c4 = club_at(buyer as i32 as isize).addr();
        let c5 = club_at(seller as i32 as isize).addr();
        let (g4, g5) = (read_i8(c4 + club::SHORT_GENDER_NAME) as i32, read_i8(c5 + club::SHORT_GENDER_NAME) as i32);
        let (sn4, sn5) = (c4 + club::SHORT_NAME, c5 + club::SHORT_NAME);
        let (mp, pp) = (money.as_ptr() as usize, players.as_ptr() as usize);
        if total > fee && pre_contract == 0 {
            news_text_money(total as u32, &mut money);
            if swap == 0 {
                fmt(NEWS_TEXT_BUFFER, 0xa97158, g5, g5, 100, g4, g4, sn5, np, sn4, mp);
            } else {
                fmt(NEWS_TEXT_BUFFER, 0xa9705c, g5, g5, 100, g4, g4, mp, pp, sn5, np, sn4);
            }
        } else {
            news_text_money(fee as u32, &mut money);
            if swap == 0 {
                if fee > 0 && pre_contract == 0 {
                    fmt(NEWS_TEXT_BUFFER, 0xa96fa8, 100, g5, g5, 100, g4, g4, mp, sn5, np, sn4);
                } else if seller as i32 > -1 {
                    fmt(NEWS_TEXT_BUFFER, 0xa96f10, g5, g5, 100, g4, g4, sn5, np, sn4); // gra11: the transfer set to complete
                } else {
                    fmt(NEWS_TEXT_BUFFER, 0xa96e98, 100, g4, g4, np, sn4);
                }
            } else if fee > 0 {
                fmt(NEWS_TEXT_BUFFER, 0xa96da8, g5, g5, 100, g4, g4, mp, pp, sn5, np, sn4);
            } else {
                fmt(NEWS_TEXT_BUFFER, 0xa96cfc, g5, g5, 100, g4, g4, sn5, np, sn4);
            }
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        if timing != 0 {
            covered!("gra_11 completion date");
            let mut date = [0u8; 8];
            let mut when = [0u8; 0xc8];
            exe::new_date(date.as_mut_ptr() as usize, day, year);
            date_string_no_weekday(date.as_ptr() as usize, when.as_mut_ptr() as usize, 0xc8, 0xff);
            fmt(NEWS_TEXT_BUFFER, 0xa96ca8, 100, when.as_ptr() as usize);
            news_text_append(out, NEWS_TEXT_BUFFER);
        }
        let has_seller = seller as i32 != -1;
        let written = match status {
            0xe => {
                if has_seller { fmt(NEWS_TEXT_BUFFER, 0xa96c7c); } else { fmt(NEWS_TEXT_BUFFER, 0xa96c38, g4, g4, sn4); }
                true
            }
            0xd => {
                if has_seller { fmt(NEWS_TEXT_BUFFER, 0xa96c38, g4, g4, sn4); } else { fmt(NEWS_TEXT_BUFFER, 0xa96bf8, g4, g4, sn4); }
                true
            }
            0xc if has_seller => {
                if pre_contract == 0 { fmt(NEWS_TEXT_BUFFER, 0xa96c38, g5, g5, sn5); }
                pre_contract == 0
            }
            3 => { fmt(NEWS_TEXT_BUFFER, 0xa96bb4, g4, g4, sn4); true }
            2 if has_seller => { fmt(NEWS_TEXT_BUFFER, 0xa96bb4, g5, g5, sn5); true }
            0x10 => { fmt(NEWS_TEXT_BUFFER, 0xa96b90); true }
            0x12 => { fmt(NEWS_TEXT_BUFFER, 0xa96b3c, g4, g4, sn4); true }
            0x11 => { fmt(NEWS_TEXT_BUFFER, 0xa96b3c, g5, g5, sn5); true }
            _ => false,
        };
        if !written {
            write_u8(NEWS_TEXT_BUFFER, 0);
        } else if read_u8(NEWS_TEXT_BUFFER) != 0 {
            news_text_append(out, NEWS_TEXT_BUFFER);
        }
        news_text_link_club(out, buyer); // gra11: the buyer's link in the body
        if has_seller {
            news_text_link_club(out, seller);
        }
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
    } else {
        let (person, s1, s2, s3, buyer) = (slot(0), slot(1), slot(2), slot(3), slot(4));
        let total = slot(0xa);
        let pre_contract = slot(0xc) as u8;
        let job = slot(0x11) as u8;
        news_text_person_name(s1, s2, s3, &mut name, 5, person);
        let c4 = club_at(buyer as i32 as isize).addr();
        let g4 = read_i8(c4 + club::SHORT_GENDER_NAME) as i32;
        let sn4 = c4 + club::SHORT_NAME;
        if exe::database_is_not_plain_player_job(job as i8) != 0 && pre_contract != 0 {
            fmt(NEWS_TEXT_BUFFER, 0xa96ae4, g4, g4, np, sn4); // "<name> set for <club> role"
        } else if total != 0 {
            fmt(NEWS_TEXT_BUFFER, 0xa96a90, g4, g4, sn4, np); // "<club> set to buy <name>"
        } else {
            fmt(NEWS_TEXT_BUFFER, 0xa96a3c, g4, g4, sn4, np); // "<club> set to sign <name>"
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_club(out, buyer);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
    }
}

/// Comparer for `write_transfer_finalised_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_transfer_finalised_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_TRANSFER_FINALISED_NEWS_TEXT);
    compare_text_writer(out, || o(this, news, full, out), |t| write_transfer_finalised_news_text(this, news, full, t), || format!("write_transfer_finalised_news_text(news {:#x}, full {})", news, full));
}

// ---------- batch 324: the news text of a player's answer to a contract (transfer_manager.cpp) ----------

/// `transfer_manager_build_refusal_reasons_text` (0x8e4ae0; stdcall(selling club, buying club, two unused, reason
/// flags, interest, out), ret 0x20): up to two refusal reasons from the flags into `out`; the reason count.
pub(crate) unsafe fn refusal_reasons_text(seller: u32, buyer: u32, a: u32, b: u32, flags: u32, interest: f64, out: usize) -> u8 {
    let f: unsafe extern "stdcall" fn(u32, u32, u32, u32, u32, f64, usize) -> u8 = core::mem::transmute(0x8e4ae0usize);
    f(seller, buyer, a, b, flags, interest, out)
}

/// Writes the headline or the body of the news that a player rejected a contract or a loan, or agreed terms with a
/// buying club.
///
/// `write_contract_response_news_text(news, full, out)` (the 0xfa4 news): slots 0..3 (the player and his name ids), 4
/// (the selling club, −1 none), 5 (the buying club), 6 (the answer: 2 agreed, 3 rejected, 4 rejected with the
/// reasons), 9 (1 a loan), 10 and 11 (the two wages, or fees, compared), 12..15 (a second person and his name ids),
/// 16 (the club he joins instead, −1 none), 17 (6 a loan), 18..21 (an agent and his name ids, mode 1), 22..25 (another
/// person and his name ids, mode 1), 26 (the reason flags), 27 (the interest × 1000, read as a float × 0.001f), 28 (the
/// number of rejected clubs) and 29.. (up to three of those clubs).
///
/// The body (the player's name in mode 1; c4 the selling club, c5 the buying one; "standard" = the selling club's short
/// gender twice, the buying one's twice, the selling club's short name, the name, the buying club's short name;
/// "swapped" = the buying club's gender twice, the selling one's twice, the name, the buying and the selling club's
/// short names):
/// - not agreed (answer ≠ 2) and no other club: answer 3: rejected 0xa9696c (standard), without a selling club
///   0xa968cc; answer 4: without a selling club 0xa96464; a loan: interest < 1.0 "no intention of going on loan"
///   0xa96828, else 0xa96788 (standard); different wages: the money (`news_text_money` of slot 10, 0x64 bytes) and the
///   second person's name (mode 1) in 0xa9663c, the second person's link (0x7d7) added before the text when the names
///   still match; equal wages: with refusal reasons (`refusal_reasons_text`) and interest > 1.25 0xa9657c with them,
///   else 0xa964d8; other answers write nothing (the buffer stays as it was).
/// - agreed or joining another club (16 ≠ −1: that club becomes the buying one and slot 17 = 6 makes it a loan):
///   without a selling club by the flags (0x10 0xa956f8; 0x40 with the other person 0xa9561c; else 0xa955b4); a loan:
///   interest < 5.5 with flag 0x20000 "close proximity" 0xa963a4, else 0xa9630c; otherwise by the flags: 0x10, 8 and
///   0x4000 0xa96210 (swapped); 0x10 and 0x4000 0xa96110; 0x10 and 0x200000 with a European buying club (nation →
///   continent = `KEY_CONTINENT_EUROPE`) 0xa96018; 0x10 or interest > 10.5 0xa95770; 0x4000 and 0x200000 with a
///   European club 0xa95f1c; 0x4000 0xa95e2c; 0x200000 with a European club 0xa95d44; 0x40 with the other person
///   0xa95c44; 0x800 or 4 0xa95814 (swapped); 0x100 with the agent 0xa95b30; 0x40000000 with a wage above 100000, not
///   a loan, by 8 and 0x400 (0xa95a34) or by 0x400 alone (0xa95964); otherwise 0xa958d0.
/// Then the text, the player's link (0x7d7, always), the buying club's and the selling club's (when ≥ 0) links (0x7d5);
/// the rejected clubs (one 0xa95570, two 0xa95510, three 0xa95490, more 0xa953e4 with the number) appended with their
/// links, and the other person's link (0x7d7) when slot 22 ≥ 0.
///
/// The headline (the name in mode 5): not agreed and no other club: a loan 0xa95384, else by the player id mod 3 "snubs"
/// 0xa95334 (0) or "rejects" 0xa952e4; agreed or another club: a loan "set for loan deal" 0xa95244, else by (club id +
/// player id) mod 3 "set for" 0xa95298 (0) or "accepts offer" 0xa951f4; the player's link (always) and the club's.
/// Original: `TRANSFER_MANAGER::write_contract_response_news_text` (0x8d9480, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_contract_response_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let s: [u32; 28] = core::array::from_fn(|k| if (6..9).contains(&k) && k != 6 { 0 } else { slot(k as u32) });
    let (s0, s1, s2, s3, s4) = (s[0], s[1], s[2], s[3], s[4]);
    let mut s5 = s[5];
    let answer = s[6] as u8;
    let s9 = s[9] as u8;
    let (s10, s11, s12, s13, s14, s15, s16) = (s[10] as i32, s[11] as i32, s[12], s[13], s[14], s[15], s[16] as i32);
    let flags = s[26];
    let interest = (s[27] as i32 as f64 * read_f32(0x9690c0) as f64) as f32 as f64; // f32 = 0.001 (x87, 53 bits, stored as f32)
    let count = slot(0x1c) as u8 as i8;
    let shown = if count > 3 { 3 } else { count };
    let mut clubs = [0usize; 3];
    let mut i = 0i8;
    while i < shown {
        clubs[i as usize] = club_at(exe::newsitem_get_slot(news, (i as u8).wrapping_add(0x1d) as u32) as i32 as isize).addr();
        i += 1;
    }
    let mut agent = [0u8; 0x190];
    let mut other = [0u8; 0x190];
    if s[18] as i32 > -1 {
        exe::database_format_name_from_ids(s[19], s[20], s[21], agent.as_mut_ptr(), 0x190, 1, s[18]);
    }
    if s[22] as i32 > -1 {
        exe::database_format_name_from_ids(s[23], s[24], s[25], other.as_mut_ptr(), 0x190, 1, s[22]);
    }
    let mut reasons = [0u8; 0x190];
    let mut name = [0u8; 0x190];
    let np = name.as_ptr() as usize;
    let player = staff_at(s0 as i32 as isize);
    if full != 0 {
        exe::database_format_name_from_ids(s1, s2, s3, name.as_mut_ptr(), 0x190, 1, s0);
        let club = |id: u32| club_at(id as i32 as isize).addr();
        let european = |c: usize| {
            let nation = read_usize(c + club::NATION);
            if nation == 0 { return false; }
            let continent = read_usize(nation + nation::CONTINENT);
            continent != 0 && read_u32(continent) == read_u32(KEY_CONTINENT_EUROPE)
        };
        if answer != 2 && s16 == -1 {
            let (c4, c5) = (club(s4), club(s5));
            let (g4, g5) = (read_i8(c4 + club::SHORT_GENDER_NAME) as i32, read_i8(c5 + club::SHORT_GENDER_NAME) as i32);
            let (sn4, sn5) = (c4 + club::SHORT_NAME, c5 + club::SHORT_NAME);
            if answer == 3 {
                if s4 as i32 != -1 {
                    fmt(NEWS_TEXT_BUFFER, 0xa9696c, g4, g4, g5, g5, sn4, np, sn5);
                } else {
                    fmt(NEWS_TEXT_BUFFER, 0xa968cc, g5, g5, np, sn5);
                }
            } else if answer == 4 {
                if s4 as i32 == -1 {
                    fmt(NEWS_TEXT_BUFFER, 0xa96464, g5, g5, np, sn5);
                } else if s9 == 1 {
                    if interest < read_f64(rdata::F64_1) {
                        fmt(NEWS_TEXT_BUFFER, 0xa96828, g4, g4, g5, g5, sn4, np, sn5);
                    } else {
                        fmt(NEWS_TEXT_BUFFER, 0xa96788, g4, g4, g5, g5, sn4, np, sn5);
                    }
                } else if s10 != s11 {
                    covered!("gra_12 rejected over the money");
                    let mut money = [0u8; 0x64];
                    let amount = exe::cash_get_display_amount(&(s10 as u32), 6);
                    exe::cash_format_money(money.as_mut_ptr() as usize, 0x64, amount, 6);
                    let mut second = [0u8; 0x12c];
                    exe::database_format_name_from_ids(s13, s14, s15, second.as_mut_ptr(), 0x12c, 1, s12);
                    fmt(NEWS_TEXT_BUFFER, 0xa9663c, g4, g4, g5, g5, g4, g4, sn4, np, sn5, sn4, second.as_ptr() as usize, money.as_ptr() as usize);
                    let p12 = staff_at(s12 as i32 as isize);
                    if exe::database_staff_matches_names(p12, s13, s14, s15, 0usize) != 0 {
                        exe::text_add_link(out, second.as_ptr(), 0x7d7, p12);
                    }
                } else if refusal_reasons_text(s4, s5, s[18], s[22], flags, interest, reasons.as_mut_ptr() as usize) != 0
                    && interest > read_f64(rdata::F64_1_25)
                {
                    fmt(NEWS_TEXT_BUFFER, 0xa9657c, g4, g4, g5, g5, sn4, np, sn5, reasons.as_ptr() as usize);
                } else {
                    fmt(NEWS_TEXT_BUFFER, 0xa964d8, g4, g4, g5, g5, sn4, np, sn5);
                }
            }
        } else {
            // the loan byte: slot 9, or with another club (slot 16) whether slot 17 is 6
            let b13 = if s16 != -1 {
                s5 = s16 as u32;
                (s[17] as u8 == 6) as u8
            } else {
                s9
            };
            let (c4, c5) = (club(s4), club(s5));
            let (g4, g5) = (read_i8(c4 + club::SHORT_GENDER_NAME) as i32, read_i8(c5 + club::SHORT_GENDER_NAME) as i32);
            let (sn4, sn5) = (c4 + club::SHORT_NAME, c5 + club::SHORT_NAME);
            let standard = |t: usize| { fmt(NEWS_TEXT_BUFFER, t, g4, g4, g5, g5, sn4, np, sn5); };
            let swapped = |t: usize| { fmt(NEWS_TEXT_BUFFER, t, g5, g5, g4, g4, np, sn5, sn4); };
            if s4 as i32 == -1 {
                if flags as u8 & 0x10 != 0 {
                    fmt(NEWS_TEXT_BUFFER, 0xa956f8, g5, g5, np, sn5);
                } else if flags as u8 & 0x40 != 0 && s[22] as i32 > -1 {
                    fmt(NEWS_TEXT_BUFFER, 0xa9561c, g5, g5, 100, np, sn5, other.as_ptr() as usize);
                } else {
                    fmt(NEWS_TEXT_BUFFER, 0xa955b4, g5, g5, np, sn5);
                }
            } else if b13 == 1 {
                if interest < read_f64(rdata::F64_5_5) && flags & 0x20000 != 0 {
                    standard(0xa963a4);
                } else {
                    standard(0xa9630c);
                }
            } else {
                covered!("gra_12 agreed terms");
                let f10 = flags & 0x10 != 0;
                if f10 && flags & 8 != 0 && flags & 0x4000 != 0 {
                    swapped(0xa96210);
                } else if f10 && flags & 0x4000 != 0 {
                    standard(0xa96110);
                } else if f10 && flags & 0x200000 != 0 && european(c5) {
                    standard(0xa96018);
                } else if f10 || interest > read_f64(rdata::F64_10_5) {
                    standard(0xa95770);
                } else if flags & 0x4000 != 0 && flags & 0x200000 != 0 && european(c5) {
                    fmt(NEWS_TEXT_BUFFER, 0xa95f1c, g4, g4, 100, 100, g5, g5, sn4, np, sn5);
                } else if flags & 0x4000 != 0 {
                    standard(0xa95e2c);
                } else if flags & 0x200000 != 0 && european(c5) {
                    standard(0xa95d44);
                } else if flags & 0x40 != 0 && s[22] as i32 > -1 {
                    fmt(NEWS_TEXT_BUFFER, 0xa95c44, g4, g4, g5, g5, 100, sn4, np, sn5, other.as_ptr() as usize);
                } else if flags & 0x800 != 0 || flags & 4 != 0 {
                    swapped(0xa95814);
                } else if flags & 0x100 != 0 && s[18] as i32 > -1 {
                    fmt(NEWS_TEXT_BUFFER, 0xa95b30, g5, g5, g4, g4, np, sn5, sn4, agent.as_ptr() as usize);
                } else if flags & 0x40000000 != 0 && (s11 > 100000 || s10 > 100000) && b13 == 0 && flags & 0x400 != 0 {
                    swapped(if flags & 8 != 0 { 0xa95a34 } else { 0xa95964 });
                } else {
                    standard(0xa958d0); // gra12: the plain agreed terms
                }
            }
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        exe::text_add_link(out, name.as_ptr(), 0x7d7, player);
        let c5 = club(s5);
        exe::text_add_link(out, (c5 + club::SHORT_NAME) as *const u8, 0x7d5, c5);
        if s4 as i32 != -1 {
            news_text_link_club(out, s4);
        }
        let sentence = match count {
            1 => {
                let c = clubs[0];
                let g = read_i8(c + club::SHORT_GENDER_NAME) as i32;
                fmt(NEWS_TEXT_BUFFER, 0xa95570, g, g, c + club::SHORT_NAME);
                true
            }
            2 => {
                let (c0, c1) = (clubs[0], clubs[1]);
                let (g0, g1) = (read_i8(c0 + club::SHORT_GENDER_NAME) as i32, read_i8(c1 + club::SHORT_GENDER_NAME) as i32);
                fmt(NEWS_TEXT_BUFFER, 0xa95510, g0, g0, g1, g1, c0 + club::SHORT_NAME, c1 + club::SHORT_NAME);
                true
            }
            n if n >= 3 => {
                let [c0, c1, c2] = clubs;
                let g = |c: usize| read_i8(c + club::SHORT_GENDER_NAME) as i32;
                let (g0, g1, g2) = (g(c0), g(c1), g(c2));
                let (n0, n1, n2) = (c0 + club::SHORT_NAME, c1 + club::SHORT_NAME, c2 + club::SHORT_NAME);
                if n == 3 {
                    fmt(NEWS_TEXT_BUFFER, 0xa95490, g0, g0, g1, g1, g2, g2, n0, n1, n2);
                } else {
                    fmt(NEWS_TEXT_BUFFER, 0xa953e4, g0, g0, g1, g1, g2, g2, n as i32, n0, n1, n2);
                }
                true
            }
            _ => false,
        };
        if sentence {
            covered!("gra_12 rejected clubs");
            news_text_append(out, NEWS_TEXT_BUFFER);
            for &c in clubs.iter().take(shown.max(0) as usize) {
                exe::text_add_link(out, (c + club::SHORT_NAME) as *const u8, 0x7d5, c);
            }
        }
        if s[22] as i32 > -1 {
            exe::text_add_link(out, other.as_ptr(), 0x7d7, staff_at(s[22] as i32 as isize));
        }
    } else {
        exe::database_format_name_from_ids(s1, s2, s3, name.as_mut_ptr(), 0x190, 5, s0);
        let t;
        if answer != 2 && s16 == -1 {
            t = if s9 == 1 {
                0xa95384 // "<name> rejects loan offer from <club>"
            } else if (s0 as i32) % 3 == 0 {
                0xa95334 // "<name> snubs <club>"
            } else {
                0xa952e4 // "<name> rejects <club>"
            };
        } else {
            let on_loan = if s16 != -1 {
                s5 = s16 as u32;
                s[17] as u8 == 6
            } else {
                s9 == 1
            };
            t = if on_loan {
                0xa95244 // "<name> set for <club> loan deal"
            } else if (s5 as i32).wrapping_add(s0 as i32) % 3 == 0 {
                0xa95298 // "<name> set for <club>"
            } else {
                0xa951f4 // "<name> accepts <club> offer"
            };
        }
        let c = club_at(s5 as i32 as isize).addr();
        let g = read_i8(c + club::SHORT_GENDER_NAME) as i32;
        fmt(NEWS_TEXT_BUFFER, t, g, g, np, c + club::SHORT_NAME);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        exe::text_add_link(out, name.as_ptr(), 0x7d7, player);
        exe::text_add_link(out, (c + club::SHORT_NAME) as *const u8, 0x7d5, c);
    }
}

/// Comparer for `write_contract_response_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_contract_response_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_CONTRACT_RESPONSE_NEWS_TEXT);
    compare_text_writer(out, || o(this, news, full, out), |t| write_contract_response_news_text(this, news, full, t), || format!("write_contract_response_news_text(news {:#x}, full {})", news, full));
}

// ---------- batch 324: the news text of a completed signing (transfer_manager.cpp) ----------

/// Writes the headline or the body of the news that a club signed or appointed a person, for a fee, on a free
/// transfer or in a player swap.
///
/// `write_transfer_completed_news_text(news, full, out)` (the 0xfa8 news). The body: slots 0..3 (the person and his
/// name ids, mode 1), 4 (the buying club, always linked), 5 (the selling club, −1 none), 6 (the fee), 9 (a pending
/// move: 0 the move is done), 10 and 11 (the joining day and year), 12 (the job), 13 (the appearance fee clause), 14
/// (the swapped players); from slot 15 on, read in turn: four slots per swapped player for up to three of them (the
/// person and his three name ids; their names, mode 1, joined as "A", "A and B" (0xa91c40), "A, B and C" (0xa91bc0)
/// or, for more, the number in words, "<n> players" 0xa91ba0), then the kind of move (0 / 1 / other), then a flag
/// that, when not 0, brings two more slots (the strengths and the positions: `exe::write_position_name` then
/// `exe::write_player_description` (person, positions, strengths, 0xc8, the gender of the position, 1) give the player
/// type and its gender; without the flag the "gender" is the slot counter, as in the original).
///
/// - A pending move (slot 9 ≠ 0) of a job that is not the plain player's (`exe::database_is_not_plain_player_job`):
///   the job name (`exe::database_get_job_name`, long form, lower case, with its gender) and, with a joining year, the
///   joining date (`exe::new_date`, `exe::date_get_long_date_string`): appointed from another club 0xa98d20 or not
///   0xa98c64; without the year: 0xa98bcc / 0xa98b58.
/// - A pending move of a player with no fee: with a joining year (the date), from another club by the kind: 0
///   "snapped up" 0xa9882c, 1 "agreed to sign" 0xa98948, other "signed from" 0xa98a6c; with no other club: 0xa985e8,
///   0xa986c8, 0xa98780; without the year, from another club: 0xa983d4, 0xa98490, 0xa9855c; with no other club:
///   0xa98240, 0xa982d4, 0xa9836c.
/// - Otherwise (the move done, or a fee): the fee text (`news_text_money`); without swapped players and a fee above 0
///   by the kind 0xa98028, 0xa980c8, 0xa981a4; with no fee as the no-year texts above; with swapped players and a fee
///   0xa97d30, 0xa97e30, 0xa97f54; with no fee 0xa97aac, 0xa97b94, 0xa97c64.
/// Then the text and the links of the two clubs; with the appearance clause, the fee text of fee + clause is appended
/// ("may be increased" 0xa97a54 with a fee, "a fee may be payable" 0xa97a00 without) unless it equals the fee text
/// (a defect of the original: on the paths that never wrote the fee text it compares with whatever the stack held;
/// the copy compares with an empty text); the person's link (0x7d7) and the swapped players' links (for 1..3 of them)
/// when the names still match.
///
/// The headline: slots 0..4 and 15 (the name in mode 5); by (slot 15 × club + person) mod 4 (C remainder): 0 "get"
/// 0xa978cc, 1 "moves to" 0xa97914, 2 "seal deal" 0xa97968, otherwise "sign" 0xa979b8; the club's link and the
/// person's (when the names still match).
/// Original: `TRANSFER_MANAGER::write_transfer_completed_news_text` (0x8dc370, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_transfer_completed_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    if full == 0 {
        let (s0, s1, s2, s3, s4) = (slot(0), slot(1), slot(2), slot(3), slot(4));
        let s15 = slot(0xf) as u8 as i8 as i32;
        exe::database_format_name_from_ids(s1, s2, s3, name.as_mut_ptr(), 0x12c, 5, s0);
        let c = club_at(s4 as i32 as isize).addr();
        let g = read_i8(c + club::SHORT_GENDER_NAME) as i32;
        let sn = c + club::SHORT_NAME;
        match s15.wrapping_mul(s4 as i32).wrapping_add(s0 as i32) % 4 {
            0 => { fmt(NEWS_TEXT_BUFFER, 0xa978cc, g, g, sn, np); } // "<club> get <name>"
            1 => { fmt(NEWS_TEXT_BUFFER, 0xa97914, 100, g, g, np, sn); } // "<name> moves to <club>"
            2 => { fmt(NEWS_TEXT_BUFFER, 0xa97968, g, g, 3, sn, np); } // "<club> seal <name> deal"
            _ => { fmt(NEWS_TEXT_BUFFER, 0xa979b8, g, g, sn, np); } // "<club> sign <name>"
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_club(out, s4);
        news_text_link_person(out, &name, s0, s1, s2, s3, 0x7d7);
        return;
    }
    covered!("gra_13 completed body");
    let mut kind_text = [0u8; 0xc8]; // the player type (write_player_description)
    let mut players = [0u8; 0x190];
    let (s0, s1, s2, s3, buyer, seller, fee) = (slot(0), slot(1), slot(2), slot(3), slot(4), slot(5), slot(6) as i32);
    let pending = slot(9) as u8;
    let (day, year) = (slot(0xa), slot(0xb));
    let job = slot(0xc) as u8;
    let clause = slot(0xd);
    let swaps = slot(0xe) as u8 as i8;
    let mut counter: u8 = 0xf;
    let mut next = || { let k = counter; counter = counter.wrapping_add(1); exe::newsitem_get_slot(news, k as u32) };
    let mut swap_names = [[0u8; 0x12c]; 3];
    let mut swap_ids = [[0u32; 4]; 3]; // the person and his three name ids
    if swaps != 0 {
        let shown = if swaps < 3 { swaps } else { 3 };
        let mut i = 0i8;
        while i < shown {
            let ids = [next(), next(), next(), next()];
            swap_ids[i as usize] = ids;
            exe::database_format_name_from_ids(ids[1], ids[2], ids[3], swap_names[i as usize].as_mut_ptr(), 0x12c, 1, ids[0]);
            i += 1;
        }
        let pp = players.as_mut_ptr() as usize;
        match swaps {
            1 => players[..0x12c].copy_from_slice(&swap_names[0]),
            2 => { fmt(pp, 0xa91c40, swap_names[0].as_ptr() as usize, swap_names[1].as_ptr() as usize); }
            3 => { fmt(pp, 0xa91bc0, swap_names[0].as_ptr() as usize, swap_names[1].as_ptr() as usize, swap_names[2].as_ptr() as usize); }
            _ => {
                let mut words = [0u8; 0x34];
                exe::utils_number_to_words(swaps as i32, words.as_mut_ptr(), 0x32, 1);
                fmt(pp, 0xa91ba0, words.as_ptr() as usize); // "<number> players"
            }
        }
    }
    let kind = next() as u8 as i8;
    // the player type's gender, or without one the slot counter (as in the original)
    let type_gender;
    if next() != 0 {
        let strengths = exe::newsitem_get_slot(news, counter as u32);
        counter = counter.wrapping_add(1);
        let positions = exe::newsitem_get_slot(news, counter as u32);
        let position_gender = exe::write_position_name(positions, kind_text.as_mut_ptr() as usize, 0xc8, 1);
        type_gender = exe::write_player_description(s0 as i32, positions, strengths, kind_text.as_mut_ptr() as usize, 0xc8, position_gender as u32, 1);
    } else {
        type_gender = counter;
    }
    exe::database_format_name_from_ids(s1, s2, s3, name.as_mut_ptr(), 0x12c, 1, s0);
    let c4 = club_at(buyer as i32 as isize).addr();
    let c5 = club_at(seller as i32 as isize).addr();
    let (g4, g5) = (read_i8(c4 + club::SHORT_GENDER_NAME) as i32, read_i8(c5 + club::SHORT_GENDER_NAME) as i32);
    let (sn4, sn5) = (c4 + club::SHORT_NAME, c5 + club::SHORT_NAME);
    let n = type_gender as i8 as i32;
    let kt = kind_text.as_ptr() as usize;
    let has_seller = seller as i32 > -1;
    let mut money = [0u8; 0x34];
    let mp = money.as_ptr() as usize;
    let pl = players.as_ptr() as usize;
    let with_date = year as u16 != 0;
    let mut when = [0u8; 0xc8];
    let wp = when.as_ptr() as usize;
    let long_date = |when: &mut [u8; 0xc8]| {
        let mut date = [0u8; 8];
        exe::new_date(date.as_mut_ptr() as usize, day, year);
        exe::date_get_long_date_string(date.as_ptr() as usize, when.as_mut_ptr() as usize, 0xc8, 0xff);
    };
    // the "no year" texts of a player with no fee, shared with the done move without swaps
    let free_no_year = |kind: i8| {
        if has_seller {
            match kind {
                0 => { fmt(NEWS_TEXT_BUFFER, 0xa983d4, g4, g4, n, n, g5, g5, sn4, kt, np, sn5); }
                1 => { fmt(NEWS_TEXT_BUFFER, 0xa98490, g4, g4, n, n, g5, g5, sn4, kt, np, sn5); }
                _ => { fmt(NEWS_TEXT_BUFFER, 0xa9855c, g4, g4, g5, g5, sn4, np, sn5); }
            }
        } else {
            match kind {
                0 => { fmt(NEWS_TEXT_BUFFER, 0xa98240, g4, g4, n, n, sn4, kt, np); }
                1 => { fmt(NEWS_TEXT_BUFFER, 0xa982d4, g4, g4, n, n, sn4, kt, np); }
                _ => { fmt(NEWS_TEXT_BUFFER, 0xa9836c, g4, g4, sn4, np); }
            }
        }
    };
    let not_plain = exe::database_is_not_plain_player_job(job as i8) != 0;
    if pending != 0 && not_plain {
        covered!("gra_13 appointment");
        let mut job_gender = pending as i8;
        if with_date {
            long_date(&mut when);
        }
        let r = exe::database_get_job_name(job as u32, 0, 0, &mut job_gender);
        let jg = job_gender as i32;
        match (with_date, has_seller) {
            (true, true) => { fmt(NEWS_TEXT_BUFFER, 0xa98d20, g4, g4, g5, g5, jg, jg, g4, g4, 100, sn4, sn5, np, r, sn4, wp); }
            (true, false) => { fmt(NEWS_TEXT_BUFFER, 0xa98c64, g4, g4, jg, jg, 100, sn4, np, r, wp); }
            (false, true) => { fmt(NEWS_TEXT_BUFFER, 0xa98bcc, g4, g4, g5, g5, jg, jg, sn4, sn5, np, r); }
            (false, false) => { fmt(NEWS_TEXT_BUFFER, 0xa98b58, g4, g4, jg, jg, sn4, np, r); }
        }
    } else if pending != 0 && fee == 0 {
        covered!("gra_13 free transfer");
        if with_date {
            long_date(&mut when);
            if has_seller {
                match kind {
                    0 => { fmt(NEWS_TEXT_BUFFER, 0xa9882c, g4, g4, n, n, g5, g5, g4, g4, 100, sn4, kt, np, sn5, sn4, wp); }
                    1 => { fmt(NEWS_TEXT_BUFFER, 0xa98948, g4, g4, g5, g5, n, n, g4, g4, 100, sn4, sn5, kt, np, sn4, wp); }
                    _ => { fmt(NEWS_TEXT_BUFFER, 0xa98a6c, g4, g4, g5, g5, g4, g4, 100, sn4, np, sn5, sn4, wp); }
                }
            } else {
                match kind {
                    0 => { fmt(NEWS_TEXT_BUFFER, 0xa985e8, g4, g4, n, n, 100, sn4, kt, np, wp); }
                    1 => { fmt(NEWS_TEXT_BUFFER, 0xa986c8, g4, g4, 100, np, sn4, wp); }
                    _ => { fmt(NEWS_TEXT_BUFFER, 0xa98780, g4, g4, 100, sn4, np, wp); }
                }
            }
        } else {
            free_no_year(kind);
        }
    } else {
        news_text_money(fee as u32, &mut money);
        if swaps == 0 {
            if fee > 0 {
                match kind {
                    0 => { fmt(NEWS_TEXT_BUFFER, 0xa98028, g4, g4, g5, g5, sn4, sn5, np, mp); }
                    1 => { fmt(NEWS_TEXT_BUFFER, 0xa980c8, g4, g4, g5, g5, n, n, sn4, sn5, kt, np, mp); }
                    _ => { fmt(NEWS_TEXT_BUFFER, 0xa981a4, g4, g4, g5, g5, sn4, np, sn5, mp); } // gra13: signed for a fee
                }
            } else {
                free_no_year(kind);
            }
        } else if fee > 0 {
            match kind {
                0 => { fmt(NEWS_TEXT_BUFFER, 0xa97d30, g4, g4, n, n, g5, g5, sn4, kt, np, sn5, mp, pl); }
                1 => { fmt(NEWS_TEXT_BUFFER, 0xa97e30, g4, g4, g5, g5, n, n, sn4, sn5, kt, np, mp, pl); }
                _ => { fmt(NEWS_TEXT_BUFFER, 0xa97f54, g4, g4, g5, g5, sn4, np, sn5, mp, pl); }
            }
        } else {
            match kind {
                0 => { fmt(NEWS_TEXT_BUFFER, 0xa97aac, g4, g4, g5, g5, n, n, sn4, sn5, kt, np, pl); }
                1 => { fmt(NEWS_TEXT_BUFFER, 0xa97b94, g4, g4, g5, g5, sn4, np, sn5, pl); }
                _ => { fmt(NEWS_TEXT_BUFFER, 0xa97c64, g4, g4, g5, g5, sn4, np, sn5, pl); }
            }
        }
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    news_text_link_club(out, buyer);
    if seller as i32 != -1 {
        news_text_link_club(out, seller);
    }
    if clause != 0 {
        let mut total = [0u8; 0x34];
        news_text_money(clause.wrapping_add(fee as u32), &mut total);
        let same = money.iter().position(|&b| b == 0).map(|e| &money[..e]) == total.iter().position(|&b| b == 0).map(|e| &total[..e]);
        if !same {
            covered!("gra_13 appearance clause");
            if fee != 0 {
                fmt(NEWS_TEXT_BUFFER, 0xa97a54, total.as_ptr() as usize); // "The fee may be increased to ..."
            } else {
                fmt(NEWS_TEXT_BUFFER, 0xa97a00, total.as_ptr() as usize); // "A fee of ... may be payable ..."
            }
            news_text_append(out, NEWS_TEXT_BUFFER);
        }
    }
    news_text_link_person(out, &name, s0, s1, s2, s3, 0x7d7);
    if swaps > 0 && swaps <= 3 {
        for i in 0..swaps as usize {
            let [id, a, b, c] = swap_ids[i];
            let person = staff_at(id as i32 as isize);
            if exe::database_staff_matches_names(person, a, b, c, 0usize) != 0 {
                exe::text_add_link(out, swap_names[i].as_ptr(), 0x7d7, person);
            }
        }
    }
}

/// Comparer for `write_transfer_completed_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_transfer_completed_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_TRANSFER_COMPLETED_NEWS_TEXT);
    compare_text_writer(out, || o(this, news, full, out), |t| write_transfer_completed_news_text(this, news, full, t), || format!("write_transfer_completed_news_text(news {:#x}, full {})", news, full));
}

// ---------- batch 324: the news text of an offer made for a person (transfer_manager.cpp) ----------

/// Writes the headline or the body of the news that a club offered a fee, players, a contract or a job for a person.
///
/// `write_offer_made_news_text(news, full, out)` (the 0xfa0 news). First the offer status (slot 9): when it is 0, the
/// offer at the index of slot 6 (an index past the count reads the first) with the serial of slot 7 gives its status
/// (`offer::STATUS`, when not 0), stored back through `News::set_item_slot` (slot 9, into the inbox of the human
/// manager of the current slot, even when there is none, and into this news item).
///
/// The body: slots 0..3 (the person and his name ids, mode 1), 4 (the bidding club: its long name and gender, always
/// linked by the long name), 5 (the selling club, −1 none), 8 (the fee), 10 (the total value), 11 (a contract offer),
/// 12 and 13 (the start day and year), 14 (the kind: 1, 2, 3 and 5), 15 (the contract kind: 1 and 3), 16 (the flags:
/// 1 and 2), 17 (a third club), 18 (the job), 23 (the swapped players); from slot 24 on, four slots per swapped player
/// for up to three of them (their names, mode 1, joined as in the other transfer texts) and then the job slot (read
/// without advancing). "The seller is someone else" = the selling club exists and its manager (`club::MANAGER`) is
/// not the human manager of the current slot.
/// - A contract offer (slot 11): the job name (`exe::database_get_job_name`, with its gender); a job that is not the
///   plain player's with a name: 0xa91afc with the seller or 0xa91a80; otherwise by the seller: contract kind 1 or 3
///   0xa91668 (someone else) / 0xa91314; with a start year (the long date of slots 12 and 13) by the kind 3 (0xa91980
///   / 0xa91590), 5 (0xa91870 / 0xa914a8) or other (0xa91794 / 0xa913f4); without the year 0xa91708 / 0xa91390.
/// - A transfer with the total above the fee: the two money texts (`news_text_money`); without swapped players, the
///   seller someone else: kind 1 0xa91260, 2 0xa911b4, else by the flags (1 and 3 with a third club 0xa910ec with the
///   fee, without 0xa9101c; otherwise 0xa90d98 with the total); not someone else: 1 0xa90f8c, 2 0xa90f08, else by the
///   flags (1 and 2: a third club 0xa910ec, else 0xa90e3c with the fee; 2 alone 0xa90e3c; otherwise 0xa90d98); with
///   swapped players: 0xa90ca8, 0xa90bc4, 0xa90ae4 (someone else) / 0xa90a1c, 0xa9095c, 0xa908a0.
/// - Otherwise (the fee text): without swapped players and a fee above 0, someone else: 1 0xa907f0, 2 0xa90748, else
///   flags 1 and 2 (a third club 0xa90674, else 0xa905b0), flag 1 with a third club and a fee of at least 500000
///   0xa910ec, flag 2 0xa905b0, otherwise 0xa9050c; not someone else: 0xa90480, 0xa903fc, 0xa90380; no fee: 0xa902f0,
///   0xa90268, 0xa901e4 / 0xa90178, 0xa90114, 0xa900b4; with swapped players and a fee: 0xa8ffc8, 0xa8fee4, 0xa8fe08 /
///   0xa8fd40, 0xa8fc84, 0xa8fbcc; with no fee: 0xa8faf4, 0xa8fa28, 0xa8f960 / 0xa8f8b0, 0xa8f808, 0xa8f764.
/// Then the text; a contract offer with a fee: the compensation due to the seller (0xa8f6ec); otherwise by the status
/// the seller's sentence (1 0xa8f6a0, 5 0xa8f660, 2 0xa8f620, 0x11 0xa8f5d4, else nothing); a non-empty sentence is
/// appended. The links: the bidding club (long name), the seller (short name, when ≥ 0), the person and the swapped
/// players (0x7d7, for 1..3 of them) when the names still match.
///
/// The headline: "transfer bid for" (0xa8f578) with the club of slot 4 and the name (mode 5); its link and the
/// person's.
/// Original: `TRANSFER_MANAGER::write_offer_made_news_text` (0x8d3520, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_offer_made_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let hm = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
    let mut status = slot(9) as u8;
    if status == 0 {
        let index = slot(6) as i32;
        let serial = slot(7);
        let tm = read_usize(G_TRANSFER_MANAGER);
        let arr = read_usize(tm);
        let at = if index < read_i32(tm + 4) { arr.wrapping_add((index as usize).wrapping_mul(4)) } else { arr };
        let o = Offer(read_usize(at));
        if !o.is_null() && (o.serial() as u32) == serial && o.status() != 0 {
            covered!("gra_14 offer status stored");
            status = o.status();
            let set: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, usize) = core::mem::transmute(FN_NEWS_SET_ITEM_SLOT); // News::set_item_slot
            let hm2 = read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize);
            set(read_usize(G_NEWS), hm2, read_u32(news + 0xd6), 9, status as i8 as i32, news);
        }
    }
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    if full == 0 {
        let (s0, s1, s2, s3, s4) = (slot(0), slot(1), slot(2), slot(3), slot(4));
        exe::database_format_name_from_ids(s1, s2, s3, name.as_mut_ptr(), 0x12c, 5, s0);
        let c = club_at(s4 as i32 as isize).addr();
        let g = read_i8(c + club::SHORT_GENDER_NAME) as i32;
        fmt(NEWS_TEXT_BUFFER, 0xa8f578, g, g, c + club::SHORT_NAME, np); // "<club> transfer bid for <name>"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_club(out, s4);
        news_text_link_person(out, &name, s0, s1, s2, s3, 0x7d7);
        return;
    }
    covered!("gra_14 offer body");
    let mut players = [0u8; 0x190];
    let (s0, s1, s2, s3, bidder, seller) = (slot(0), slot(1), slot(2), slot(3), slot(4), slot(5));
    let fee = slot(8) as i32;
    let total = slot(0xa) as i32;
    let contract = slot(0xb) as u8;
    let (day, year) = (slot(0xc), slot(0xd));
    let kind = slot(0xe) as u8;
    let contract_kind = slot(0xf) as u8;
    let flags = slot(0x10) as u8;
    let third = slot(0x11) as i32;
    let _job_slot = slot(0x12) as u8;
    let swaps = slot(0x17) as u8 as i8;
    let someone_else = if seller as i32 != -1 {
        let m = read_usize(club_at(seller as i32 as isize).addr() + club::MANAGER);
        m == 0 || m != hm
    } else {
        false
    };
    let mut counter: u8 = 0x18;
    let mut swap_names = [[0u8; 0x12c]; 3];
    let mut swap_ids = [[0u32; 4]; 3];
    if swaps != 0 {
        let shown = if swaps < 3 { swaps } else { 3 };
        let mut i = 0i8;
        while i < shown {
            let mut ids = [0u32; 4];
            for v in ids.iter_mut() {
                *v = exe::newsitem_get_slot(news, counter as u32);
                counter = counter.wrapping_add(1);
            }
            swap_ids[i as usize] = ids;
            exe::database_format_name_from_ids(ids[1], ids[2], ids[3], swap_names[i as usize].as_mut_ptr(), 0x12c, 1, ids[0]);
            i += 1;
        }
        let pp = players.as_mut_ptr() as usize;
        match swaps {
            1 => players[..0x12c].copy_from_slice(&swap_names[0]),
            2 => { fmt(pp, 0xa91c40, swap_names[0].as_ptr() as usize, swap_names[1].as_ptr() as usize); }
            3 => { fmt(pp, 0xa91bc0, swap_names[0].as_ptr() as usize, swap_names[1].as_ptr() as usize, swap_names[2].as_ptr() as usize); }
            _ => {
                let mut words = [0u8; 0x34];
                exe::utils_number_to_words(swaps as i32, words.as_mut_ptr(), 0x32, 1);
                fmt(pp, 0xa91ba0, words.as_ptr() as usize); // "<number> players"
            }
        }
    }
    exe::database_format_name_from_ids(s1, s2, s3, name.as_mut_ptr(), 0x12c, 1, s0);
    let job = exe::newsitem_get_slot(news, counter as u32) as u8;
    let c4 = club_at(bidder as i32 as isize).addr();
    let c5 = club_at(seller as i32 as isize).addr();
    let g4 = read_i8(c4 + club::GENDER_NAME) as i32; // the bidding club by its long name
    let n4 = c4 + club::NAME;
    let g5 = read_i8(c5 + club::SHORT_GENDER_NAME) as i32;
    let sn5 = c5 + club::SHORT_NAME;
    // the third club (slot 17) by its long name, read only on the paths that use it (it is −1 on the others)
    let c17 = || club_at(third as isize).addr();
    let pl = players.as_ptr() as usize;
    let mut money = [0u8; 0x34];
    let mp = money.as_ptr() as usize;
    if contract != 0 {
        let mut job_gender = flags as i8; // the original writes the gender over the flags' byte
        let r = exe::database_get_job_name(job as u32, 0, 0, &mut job_gender);
        let jg = job_gender as i32;
        if exe::database_is_not_plain_player_job(job as i8) != 0 && r != 0 {
            covered!("gra_14 job offer");
            if seller as i32 > -1 {
                fmt(NEWS_TEXT_BUFFER, 0xa91afc, g4, g4, g5, g5, jg, jg, n4, sn5, np, r);
            } else {
                fmt(NEWS_TEXT_BUFFER, 0xa91a80, g4, g4, jg, jg, n4, np, r);
            }
        } else {
            let date_text = |when: &mut [u8; 0xc8]| {
                let mut date = [0u8; 8];
                exe::new_date(date.as_mut_ptr() as usize, day, year);
                exe::date_get_long_date_string(date.as_ptr() as usize, when.as_mut_ptr() as usize, 0xc8, 0xff);
            };
            let mut when = [0u8; 0xc8];
            let wp = when.as_ptr() as usize;
            if someone_else {
                if contract_kind == 1 || contract_kind == 3 {
                    fmt(NEWS_TEXT_BUFFER, 0xa91668, g4, g4, g5, g5, n4, sn5, np);
                } else if year as u16 != 0 {
                    date_text(&mut when);
                    match kind {
                        3 => { fmt(NEWS_TEXT_BUFFER, 0xa91980, g4, g4, g5, g5, 100, n4, sn5, np, wp); }
                        5 => { fmt(NEWS_TEXT_BUFFER, 0xa91870, g4, g4, g5, g5, 100, n4, sn5, np, wp); }
                        _ => { fmt(NEWS_TEXT_BUFFER, 0xa91794, g4, g4, g5, g5, n4, sn5, np); }
                    }
                } else {
                    fmt(NEWS_TEXT_BUFFER, 0xa91708, g4, g4, g5, g5, n4, sn5, np);
                }
            } else if contract_kind == 1 || contract_kind == 3 {
                fmt(NEWS_TEXT_BUFFER, 0xa91314, g4, g4, n4, np);
            } else if year as u16 != 0 {
                date_text(&mut when);
                match kind {
                    3 => { fmt(NEWS_TEXT_BUFFER, 0xa91590, g4, g4, 100, n4, np, wp); }
                    5 => { fmt(NEWS_TEXT_BUFFER, 0xa914a8, g4, g4, 100, n4, np, wp); }
                    _ => { fmt(NEWS_TEXT_BUFFER, 0xa913f4, g4, g4, n4, np); }
                }
            } else {
                fmt(NEWS_TEXT_BUFFER, 0xa91390, g4, g4, n4, np); // "<club> have offered a contract to <name>"
            }
        }
    } else if total > fee {
        let mut fee_text = [0u8; 0x34];
        news_text_money(total as u32, &mut money);
        news_text_money(fee as u32, &mut fee_text);
        let fp = fee_text.as_ptr() as usize;
        if swaps == 0 {
            if someone_else {
                match kind {
                    1 => { fmt(NEWS_TEXT_BUFFER, 0xa91260, g4, g4, g5, g5, n4, mp, sn5, np); }
                    2 => { fmt(NEWS_TEXT_BUFFER, 0xa911b4, g4, g4, g5, g5, n4, mp, sn5, np); }
                    _ => {
                        if flags & 3 != 0 {
                            if third != -1 {
                                fmt(NEWS_TEXT_BUFFER, 0xa910ec, g4, g4, read_i8(c17() + club::GENDER_NAME) as i32, read_i8(c17() + club::GENDER_NAME) as i32, n4, c17() + club::NAME, np, fp);
                            } else {
                                fmt(NEWS_TEXT_BUFFER, 0xa9101c, g4, g4, g5, g5, n4, fp, sn5, np);
                            }
                        } else {
                            fmt(NEWS_TEXT_BUFFER, 0xa90d98, g4, g4, g5, g5, n4, mp, sn5, np);
                        }
                    }
                }
            } else {
                match kind {
                    1 => { fmt(NEWS_TEXT_BUFFER, 0xa90f8c, g4, g4, n4, mp, np); }
                    2 => { fmt(NEWS_TEXT_BUFFER, 0xa90f08, g4, g4, n4, mp, np); }
                    _ => {
                        if flags & 1 != 0 && flags & 2 != 0 && third != -1 {
                            fmt(NEWS_TEXT_BUFFER, 0xa910ec, g4, g4, read_i8(c17() + club::GENDER_NAME) as i32, read_i8(c17() + club::GENDER_NAME) as i32, n4, c17() + club::NAME, np, fp);
                        } else if flags & 2 != 0 {
                            fmt(NEWS_TEXT_BUFFER, 0xa90e3c, g4, g4, g5, g5, n4, fp, sn5, np);
                        } else {
                            fmt(NEWS_TEXT_BUFFER, 0xa90d98, g4, g4, g5, g5, n4, mp, sn5, np);
                        }
                    }
                }
            }
        } else if someone_else {
            let t = match kind { 1 => 0xa90ca8, 2 => 0xa90bc4, _ => 0xa90ae4 };
            fmt(NEWS_TEXT_BUFFER, t, g4, g4, g5, g5, n4, mp, pl, sn5, np);
        } else {
            let t = match kind { 1 => 0xa90a1c, 2 => 0xa9095c, _ => 0xa908a0 };
            fmt(NEWS_TEXT_BUFFER, t, g4, g4, n4, mp, pl, np);
        }
    } else {
        news_text_money(fee as u32, &mut money);
        if swaps == 0 {
            if fee > 0 {
                if someone_else {
                    match kind {
                        1 => { fmt(NEWS_TEXT_BUFFER, 0xa907f0, g4, g4, g5, g5, n4, mp, sn5, np); }
                        2 => { fmt(NEWS_TEXT_BUFFER, 0xa90748, g4, g4, g5, g5, n4, mp, sn5, np); }
                        _ => {
                            if flags & 1 != 0 && flags & 2 != 0 {
                                if third != -1 {
                                    fmt(NEWS_TEXT_BUFFER, 0xa90674, g4, g4, read_i8(c17() + club::GENDER_NAME) as i32, read_i8(c17() + club::GENDER_NAME) as i32, n4, c17() + club::NAME, np, mp);
                                } else {
                                    fmt(NEWS_TEXT_BUFFER, 0xa905b0, g4, g4, g5, g5, n4, mp, sn5, np);
                                }
                            } else if flags & 1 != 0 && third != -1 && fee >= 500000 {
                                fmt(NEWS_TEXT_BUFFER, 0xa910ec, g4, g4, read_i8(c17() + club::GENDER_NAME) as i32, read_i8(c17() + club::GENDER_NAME) as i32, n4, c17() + club::NAME, np, mp);
                            } else if flags & 2 != 0 {
                                fmt(NEWS_TEXT_BUFFER, 0xa905b0, g4, g4, g5, g5, n4, mp, sn5, np);
                            } else {
                                fmt(NEWS_TEXT_BUFFER, 0xa9050c, g4, g4, g5, g5, n4, mp, sn5, np); // gra14: the plain offer for a fee
                            }
                        }
                    }
                } else {
                    let t = match kind { 1 => 0xa90480, 2 => 0xa903fc, _ => 0xa90380 };
                    fmt(NEWS_TEXT_BUFFER, t, g4, g4, n4, mp, np);
                }
            } else if someone_else {
                let t = match kind { 1 => 0xa902f0, 2 => 0xa90268, _ => 0xa901e4 };
                fmt(NEWS_TEXT_BUFFER, t, g4, g4, g5, g5, n4, sn5, np);
            } else {
                let t = match kind { 1 => 0xa90178, 2 => 0xa90114, _ => 0xa900b4 };
                fmt(NEWS_TEXT_BUFFER, t, g4, g4, n4, np);
            }
        } else if fee > 0 {
            if someone_else {
                let t = match kind { 1 => 0xa8ffc8, 2 => 0xa8fee4, _ => 0xa8fe08 };
                fmt(NEWS_TEXT_BUFFER, t, g4, g4, g5, g5, n4, mp, pl, sn5, np);
            } else {
                let t = match kind { 1 => 0xa8fd40, 2 => 0xa8fc84, _ => 0xa8fbcc };
                fmt(NEWS_TEXT_BUFFER, t, g4, g4, n4, mp, pl, np);
            }
        } else if someone_else {
            let t = match kind { 1 => 0xa8faf4, 2 => 0xa8fa28, _ => 0xa8f960 };
            fmt(NEWS_TEXT_BUFFER, t, g4, g4, g5, g5, n4, pl, sn5, np);
        } else {
            let t = match kind { 1 => 0xa8f8b0, 2 => 0xa8f808, _ => 0xa8f764 };
            fmt(NEWS_TEXT_BUFFER, t, g4, g4, n4, pl, np);
        }
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if contract != 0 && fee != 0 {
        covered!("gra_14 compensation due");
        news_text_money(fee as u32, &mut money);
        fmt(NEWS_TEXT_BUFFER, 0xa8f6ec, g5, g5, sn5, mp); // "<club> will be due compensation"
    } else {
        match status {
            1 => { fmt(NEWS_TEXT_BUFFER, 0xa8f6a0, g5, g5, sn5); }
            5 => { fmt(NEWS_TEXT_BUFFER, 0xa8f660, g5, g5, sn5); }
            2 => { fmt(NEWS_TEXT_BUFFER, 0xa8f620, g5, g5, sn5); }
            0x11 => { fmt(NEWS_TEXT_BUFFER, 0xa8f5d4, g5, g5, sn5); }
            _ => write_u8(NEWS_TEXT_BUFFER, 0),
        }
    }
    if read_u8(NEWS_TEXT_BUFFER) != 0 {
        news_text_append(out, NEWS_TEXT_BUFFER);
    }
    exe::text_add_link(out, n4 as *const u8, 0x7d5, c4);
    if seller as i32 != -1 {
        news_text_link_club(out, seller);
    }
    news_text_link_person(out, &name, s0, s1, s2, s3, 0x7d7);
    if swaps > 0 && swaps <= 3 {
        for i in 0..swaps as usize {
            let [id, a, b, c] = swap_ids[i];
            let person = staff_at(id as i32 as isize);
            if exe::database_staff_matches_names(person, a, b, c, 0usize) != 0 {
                exe::text_add_link(out, swap_names[i].as_ptr(), 0x7d7, person);
            }
        }
    }
}

/// Comparer for `write_offer_made_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_offer_made_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_OFFER_MADE_NEWS_TEXT);
    compare_text_writer(out, || o(this, news, full, out), |t| write_offer_made_news_text(this, news, full, t), || format!("write_offer_made_news_text(news {:#x}, full {})", news, full));
}

// ---------- batch 324: the news text writer (news.cpp) ----------

/// The size of a competition record of `G_CLUB_COMP_TABLE`.
const NEWS_COMP_SIZE: usize = 0x6b;

/// The piece of the last text written (for the comparer's message).
static mut NEWS_LAST_PIECE: &str = "";

/// The game's text formatter (`langlib_sprintf`: `{}` markers take the gender ints).
unsafe fn news_format() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}
/// The C runtime `sprintf`.
unsafe fn news_sprintf() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_CRT_SPRINTF)
}
/// The club at index `i` of `G_CLUB_TABLE` (32-bit wrapping, no bounds check, like the original).
unsafe fn news_club(i: u32) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add((i as usize).wrapping_mul(club::SIZE))
}
/// The competition at index `i` of `G_CLUB_COMP_TABLE`.
unsafe fn news_comp(i: u32) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add((i as usize).wrapping_mul(NEWS_COMP_SIZE))
}
/// The nation at index `i` of `G_NATION_TABLE`.
unsafe fn news_nation(i: u32) -> usize {
    read_usize(G_NATION_TABLE).wrapping_add((i as usize).wrapping_mul(nation::SIZE))
}
/// The person with id `i` of `G_STAFF_TABLE`.
unsafe fn news_staff(i: u32) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE))
}
/// A gender byte (sign-extended, as the original's `movsx`).
unsafe fn news_g(a: usize) -> i32 {
    read_i8(a) as i32
}
/// `Text::add_link(out, text, kind, target)`.
unsafe fn news_link(out: usize, text: usize, kind: u32, target: usize) {
    exe::text_add_link(out, text as *const u8, kind, target);
}
/// `Text::set_string(out, text)`.
unsafe fn news_set(out: usize, text: usize) {
    exe::text_set_string(out, text as *const u8);
}
/// The C `strcmp(a, b) == 0`.
unsafe fn news_streq(a: usize, b: usize) -> bool {
    let mut k = 0;
    loop {
        let (x, y) = (read_u8(a + k), read_u8(b + k));
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
        k += 1;
    }
}

/// The news.cpp "Error" message box for source line `line`: the file name and extension (`exe::crt_strtok`, the
/// C runtime splitpath, of the news.cpp path 0xa693ec), the version, "v%s %s.%s %d" at 0xae24d0, the box
/// (`exe::gui_show_message_box`) and the mouse button released.
unsafe fn news_error_box(line: u32) {
    let (mut name, mut ext) = ([0u8; 0x100], [0u8; 0x100]);
    exe::crt_strtok(0xa693ec, 0, 0, name.as_mut_ptr(), ext.as_mut_ptr());
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), line);
    news_sprintf()(0xae24d0, 0x9870e8, v, name.as_ptr(), ext.as_ptr(), line); // "v%s %s.%s %d"
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0); // "Error"
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// How a news type is written by another routine: (category, type, writer).
#[derive(Clone, Copy)]
enum NewsWriter {
    /// A `thiscall(object, news, body, out)` method of the object held by a global.
    Global(usize, usize),
    /// A `thiscall(object, news, body, out)` method of a fixed object.
    Object(usize, usize),
    /// A `cdecl(news, body, out)` routine.
    Plain(usize),
}

/// The types written by another routine, by category (`NewsItem::get_category`) and type, in the original's order.
const NEWS_WRITERS: &[(u8, u32, NewsWriter)] = {
    use NewsWriter::*;
    &[
        (2, 0x1d, Global(0xb74318, 0x76e3b0)),
        (2, 0x1e, Global(0xb74318, 0x76f370)),
        (2, 0x20, Global(0xb74318, 0x76fa40)),
        (2, 0x21, Global(0xb74318, 0x76ff40)),
        (2, 0x22, Global(0xb74318, 0x7703f0)),
        (2, 0x24, Global(0xb74318, 0x770d20)),
        (2, 0x25, Global(0xb74434, 0x7e7280)),
        (2, 0x2c, Global(0xb74318, 0x777ca0)),
        (2, 0x2f, Global(0xde1f58, 0x906130)),
        (2, 0x2e, Global(0xb6b7c8, 0x75c5a0)),
        (2, 0x2a, Global(0xde2f74, 0x924160)),
        (2, 0x2, Global(0xde2f74, 0x923dd0)),
        (2, 0x29, Global(0xb67a44, 0x6aa910)),
        (2, 0x2b, Global(0xb67a44, 0x6aadb0)),
        (2, 0x18, Global(0xdd7ef0, 0x85af40)),
        (2, 0x19, Global(0xdd7ef0, 0x85b190)),
        (7, 0x3e8, Global(0xad9dac, 0x44c650)),
        (3, 0x7d1, Global(0xb63d28, 0x631580)),
        (3, 0x7d2, Global(0xae28f0, 0x5576e0)),
        (3, 0x7d3, Global(0xae28f0, 0x55b8b0)),
        (6, 0x1779, Global(0xae2a38, 0x59f3e0)),
        (6, 0x177f, Global(0xae2a38, 0x5a0230)),
        (6, 0x1780, Global(0xae2a38, 0x5a04c0)),
        (6, 0x177d, Global(0xae2a38, 0x5a0050)),
        (6, 0x177a, Global(0xae2a38, 0x59f9c0)),
        (6, 0x177b, Global(0xae2a38, 0x59fcf0)),
        (6, 0x1783, Global(0xae2a38, 0x5a0380)),
        (6, 0x1786, Global(0xb67a44, 0x6a9470)),
        (6, 0x1784, Global(0xae2a3c, 0x5a7ed0)),
        (6, 0x177c, Global(0xae2a38, 0x59feb0)),
        (6, 0x1775, Global(0xb67a44, 0x6a0690)),
        (6, 0x1778, Global(0xb67a44, 0x6a0ad0)),
        (6, 0x1774, Global(0xb67a44, 0x6a5120)),
        (6, 0x1777, Global(0xb67a44, 0x6a1190)),
        (6, 0x1776, Global(0xb67a44, 0x69fef0)),
        (6, 0x1770, Global(0xb67a44, 0x6a1d60)),
        (6, 0x1773, Global(0xb67a44, 0x69c6b0)),
        (6, 0x177e, Global(0xb67a44, 0x6a4550)),
        (6, 0x1771, Global(0xb67a44, 0x6a4ae0)),
        (6, 0x1772, Global(0xb67a44, 0x6a4d40)),
        (6, 0x1782, Global(0xb67a44, 0x6a5e30)),
        (6, 0x1781, Global(0xb67a44, 0x6a5b40)),
        (6, 0x1788, Global(0xae2a38, 0x59fb90)),
        (6, 0x1789, Global(0xb67a44, 0x6aab70)),
        (6, 0x178a, Global(0xb67a44, 0x6ac190)),
        (6, 0x178e, Global(0xb6b7c8, 0x758fe0)),
        (6, 0x178d, Global(0xb67a44, 0x6ac880)),
        (6, 0x178f, Global(0xb67a44, 0x6adb70)),
        (6, 0x1790, Global(0xb6b7c8, 0x74af60)),
        (6, 0x178c, Plain(0x58a660)),
        (4, 0xbbb, Global(0xb7434c, 0x7aa4c0)),
        (4, 0xbdb, Global(0xb6b7c8, 0x72bd50)),
        (4, 0xbdd, Global(0xb6b7c8, 0x744f80)),
        (4, 0xbdc, Global(0xb6b7c8, 0x743d30)),
        (4, 0xbd0, Global(0xb6b7c8, 0x72d270)),
        (4, 0xbdf, Global(0xb6b7c8, 0x74bbd0)),
        (4, 0xbda, Global(0xb6b7c8, 0x733000)),
        (4, 0xbde, Plain(0x589480)),
        (4, 0xbcf, Global(0xb6b7c8, 0x7435a0)),
        (4, 0xbd9, Global(0xb6b7c8, 0x733fc0)),
        (4, 0xbce, Global(0xb7434c, 0x7a8eb0)),
        (4, 0xbcd, Plain(0x5886d0)),
        (4, 0xbba, Global(0xb7434c, 0x7a9650)),
        (4, 0xbb8, Global(0xae1050, 0x4d4fb0)),
        (4, 0xbb9, Global(0xae1050, 0x4daa00)),
        (4, 0xbbc, Global(0xae1050, 0x4daec0)),
        (4, 0xbbe, Global(0xae1050, 0x4d6ee0)),
        (4, 0xbbd, Global(0xae1050, 0x4d6a00)),
        (4, 0xbbf, Global(0xae1050, 0x4d8990)),
        (4, 0xbe1, Plain(0x589880)),
        (4, 0xbe4, Global(0xb6b7c8, 0x75d210)),
        (4, 0xbe5, Global(0xb6b7c8, 0x75b9c0)),
        (4, 0xbe2, Plain(0x589d60)),
        (4, 0xbe3, Plain(0x58a210)),
        (4, 0xbe6, Global(0xb6b7c8, 0x759a10)),
        (4, 0xbe9, Global(0xae2a30, 0x56a560)),
        (4, 0xbe8, Global(0xb6b7c8, 0x75a940)),
        (4, 0xbea, Global(0xae1050, 0x4d7770)),
        (4, 0xbc0, Global(0xae1050, 0x4d95e0)),
        (4, 0xbc1, Global(0xae1050, 0x4d9a90)),
        (4, 0xbc2, Global(0xae1050, 0x4da190)),
        (4, 0xbc5, Global(0xad9c6c, 0x419c60)),
        (4, 0xbc6, Global(0xae2a3c, 0x5a76e0)),
        (4, 0xbc7, Plain(0x587e90)),
        (4, 0xbc8, Plain(0x5883f0)),
        (4, 0xbc9, Global(0xad9c6c, 0x41a8e0)),
        (4, 0xbcb, Global(0xb6b7c8, 0x7572a0)),
        (4, 0xbca, Plain(0x5889e0)),
        (4, 0xbc3, Global(0xde1f54, 0x8e3b80)),
        (4, 0xbc4, Global(0xae1050, 0x4d65c0)),
        (4, 0xbcc, Global(0xde1f54, 0x8e4460)),
        (5, 0xfa0, Global(0xde1f54, 0x8d3520)),
        (5, 0xfa1, Global(0xde1f54, 0x8d5420)),
        (5, 0xfbc, Global(0xb6b7c8, 0x755860)),
        (5, 0xfbd, Plain(0x587370)),
        (5, 0xfbe, Plain(0x587800)),
        (5, 0xfa2, Global(0xde1f54, 0x8d6e80)),
        (5, 0xfa3, Global(0xde1f54, 0x8d7870)),
        (5, 0xfac, Global(0xde1f54, 0x8d7fb0)),
        (5, 0xfa4, Global(0xde1f54, 0x8d9480)),
        (5, 0xfa5, Global(0xde1f54, 0x8da8f0)),
        (5, 0xfa7, Global(0xde1f54, 0x8db580)),
        (5, 0xfa6, Global(0xde1f54, 0x8dba80)),
        (5, 0xfa8, Global(0xde1f54, 0x8dc370)),
        (5, 0xfa9, Global(0xde1f54, 0x8dd830)),
        (5, 0xfab, Global(0xde1f54, 0x8ddc70)),
        (5, 0xfad, Global(0xde1f54, 0x8de320)),
        (5, 0xfae, Global(0xde1f54, 0x8d84e0)),
        (5, 0xfaf, Global(0xde1f54, 0x8d8af0)),
        (5, 0xfb0, Global(0xde1f54, 0x8de980)),
        (5, 0xfb1, Global(0xde1f54, 0x8dffe0)),
        (5, 0xfb2, Global(0xde1f54, 0x8e05a0)),
        (5, 0xfb3, Global(0xde1f54, 0x8e08e0)),
        (5, 0xfb4, Global(0xde1f54, 0x8e1370)),
        (5, 0xfb5, Global(0xde1f54, 0x8e19c0)),
        (5, 0xfb6, Global(0xde1f54, 0x8e20a0)),
        (5, 0xfb7, Global(0xde1f54, 0x8e2570)),
        (5, 0xfb8, Global(0xae1050, 0x4db2e0)),
        (5, 0xfb9, Global(0xde1f54, 0x8e2cd0)),
        (5, 0xfba, Global(0xde1f54, 0x8e32c0)),
        (5, 0xfbb, Global(0xde1f54, 0x8e3780)),
        (5, 0xfbf, Global(0xde1f54, 0x8e3f90)),
        (5, 0xfc0, Global(0xde1f54, 0x8e47a0)),
        (5, 0xfc2, Plain(0x58aa60)),
        (5, 0xfc1, Plain(0x58a900)),
        (1, 0x138d, Plain(0x588f00)),
        (1, 0x138f, Global(0xad9c6c, 0x41b030)),
        (1, 0x1390, Global(0xb67a44, 0x69c910)),
        (1, 0x1391, Object(0xadadf4, 0x487f60)),
        (1, 0x1393, Global(0xb7433c, 0x795f50)),
        (1, 0x1394, Global(0xae2a3c, 0x5a8090)),
        (1, 0x1392, Object(0xb74344, 0x7a1cd0)),
        (1, 0x138e, Global(0xb74448, 0x7f9d10)),
        (1, 0x138c, Global(0xb67a44, 0x6a6310)),
        (1, 0x1389, Plain(0x7b7840)),
        (1, 0x138a, Plain(0x7b81f0)),
        (1, 0x138b, Plain(0x7b95c0)),
        (6, 0x1787, Global(0xb67a44, 0x6a3060)),
        (4, 0xbe0, Global(0xb6b7c8, 0x738d50)),
        (5, 0xfc3, Global(0xde1f54, 0x8e51d0)),
    ]
};

/// Writes the headline (`body` 0) or the body of a news item into `out`, returning 1 (0 for a type-26 news whose
/// manager record is missing, and after an error box).
///
/// `NewsItem::write_text(news, manager, out, body, flag)`: with no manager or no `out`, the news.cpp error box (lines
/// 0x9f and 0xa7) and 0. Otherwise `out` is cleared (`exe::text_clear`) and the news is written by its category
/// (`exe::newsitem_get_category`) and type (the news `TYPE` dword, +0): most types go to the writer of their subject
/// (`NEWS_WRITERS`: the media, the coach report, the transfers, the competitions, the discipline, the records...); the
/// types the routine writes itself are the pieces below (category 2: the match moved, televised or rearranged, the host
/// bids, the finances, the season's end, the job offers...; category 3: the match report; category 7: the club
/// record; category 1: the follow-up news). Types 27, 28, 31 and 35 call their writers with other arguments (see the
/// code). An unknown type gives the error box of line 0x1226 and 0.
/// Original: `NewsItem::write_text` (0x77bc80, news.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn newsitem_write_text(news: usize, manager: usize, out: usize, body: u32, flag: u32) -> u32 {
    NEWS_LAST_PIECE = "";
    if manager == 0 {
        news_error_box(0x9f);
        return 0;
    }
    if out == 0 {
        news_error_box(0xa7);
        return 0;
    }
    let clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4afa40usize); // Text::clear
    clear(out);
    let cat = exe::newsitem_get_category(news);
    let ty = read_u32(news);
    let three = |f: usize, this: usize| {
        let w: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(f);
        w(this, news, body, out);
    };
    if cat == 2 {
        match ty {
            0x1b => {
                // the club record news: (this, f5, f9, news, out, body)
                let w: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize, u32) -> u32 = core::mem::transmute(0x4143f0usize);
                w(read_usize(0xad9c60), read_u32(news + 5), read_u32(news + 9), news, out, body);
                return 1;
            }
            0x1c => {
                let w: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x414420usize);
                w(read_usize(0xad9c60), news, out, body);
                return 1;
            }
            0x1f => {
                three(0x48eb00, read_usize(read_usize(0xadadfc).wrapping_add((read_u32(news + 9) as usize).wrapping_mul(4))));
                return 1;
            }
            0x23 => {
                three(0x48ee40, read_usize(read_usize(0xadadfc).wrapping_add((read_u32(news + 5) as usize).wrapping_mul(4))));
                return 1;
            }
            0x1a => {
                let coach = read_usize(read_usize(0xadadfc).wrapping_add((read_u32(news + 5) as usize).wrapping_mul(4)));
                if coach == 0 {
                    return 0;
                }
                three(0x48e590, coach);
                return 1;
            }
            1 => return news_match_rearranged(news, body, out),
            3 => return news_match_televised(news, body, out),
            4 => return news_host_bids(news, body, out),
            5 => return news_host_chosen(news, body, out),
            6 => return news_friendly_proposal(news, body, out),
            7 => return news_friendly_answered(news, body, out),
            8 => return news_friendly_cancelled(news, body, out),
            9 => return news_tournament_invitation(news, body, out),
            10 => return news_tournament_answered(news, body, out),
            11 => return news_tournament_cancelled(news, body, out),
            12 => return news_comp_report(news, body, out),
            13 => return news_fixture_list(news, body, out),
            16 => return news_comp_match_report(news, body, out),
            17 => return news_playoff_place(news, body, out, false),
            18 => return news_playoff_place(news, body, out, true),
            19 => return news_leagues_reshuffled(news, body, out),
            20 => return news_cup_draw(news, body, out),
            21 => return news_group_draw(news, body, out),
            22 => return news_intertoto_invitation(news, body, out),
            23 => return news_intertoto_chosen(news, body, out),
            0x26 => return news_cup_draw_open(news, body, out),
            0x27 => return news_cup_draw_today(news, body, out),
            0x28 => return news_world_club_bye(news, body, out),
            _ => {}
        }
    } else if cat == 7 && ty == 0x3e9 {
        return news_club_record(news, body, out);
    } else if cat == 3 && ty == 0x7d0 {
        return news_match_report(news, body, out);
    } else if cat == 1 && ty == 0x1388 {
        return news_follow_up(news, manager, out, body, flag);
    }
    if let Some(&(_, _, w)) = NEWS_WRITERS.iter().find(|e| e.0 == cat && e.1 == ty) {
        match w {
            NewsWriter::Global(g, f) => three(f, read_usize(g)),
            NewsWriter::Object(o, f) => three(f, o),
            NewsWriter::Plain(f) => {
                let w: unsafe extern "C" fn(usize, u32, usize) -> u32 = core::mem::transmute(f);
                w(news, body, out);
            }
        }
        return 1;
    }
    news_error_box(0x1226);
    0
}

/// Type 1, the match rearranged. Slots: +5 and +9 the two clubs, +0xd the competition, +0x11 and +0x15 the stage and
/// the round (words), +0x19/+0x1d the old date and +0x35/+0x39 the new one (`Date::Date` of two words each), +0x21 a
/// club, +0x25 another club and +0x29 a competition of the clash (+0x21 = −1 none).
///
/// Body: the round's label (`exe::comp_util_format_comp_stage_round_label`, form 2; "Friendly" and "Reserve" get a
/// lower-case first letter) and its gender (`exe::comp_util_get_label_gender`), both dates without the year
/// (`exe::date_get_long_date_string_no_year`); with a clash 0xa692ec (the clubs' names and genders, the label, the
/// dates, the clash clubs' and competition's short names and short genders), otherwise 0xa691c8. Links: the two clubs
/// (0x7d5), the competition on the label (0x7d0), and with a clash its two clubs' short names and its competition's.
/// Headline 0xa69184 with the clubs' short names and short genders and their links.
/// Original: part of `NewsItem::write_text` (0x77bfeb..0x77c46a).
unsafe fn news_match_rearranged(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "match_rearranged";
    let f = news_format();
    let (c1, c2) = (news_club(read_u32(news + 5)), news_club(read_u32(news + 9)));
    if body == 0 {
        f(NEWS_TEXT_BUFFER, 0xa69184, news_g(c1 + club::SHORT_GENDER_NAME), news_g(c1 + club::SHORT_GENDER_NAME),
          news_g(c2 + club::SHORT_GENDER_NAME), news_g(c2 + club::SHORT_GENDER_NAME), c1 + club::SHORT_NAME, c2 + club::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
        news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
        return 1;
    }
    covered!("gra_1 match rearranged body");
    let comp = news_comp(read_u32(news + 0xd));
    let (stage, round) = (read_u16(news + 0x11) as u32, read_u16(news + 0x15) as u32);
    let mut label = [0u8; 0xc8];
    let lb = label.as_mut_ptr() as usize;
    exe::comp_util_format_comp_stage_round_label(comp, stage, round, 2, label.as_mut_ptr(), 0xc8);
    if news_streq(lb, 0x9db958) {
        label[0] = b'f'; // "Friendly"
    } else if news_streq(lb, 0x9b5b40) {
        label[0] = b'r'; // "Reserve"
    }
    let lg = exe::comp_util_get_label_gender(comp as u32, stage, round, 2) as i8 as i32;
    let (mut d1, mut d2) = ([0u8; 8], [0u8; 8]);
    let (mut s1, mut s2) = ([0u8; 0xc8], [0u8; 0xc8]);
    news_date(d1.as_mut_ptr() as usize, read_u16(news + 0x19), read_u16(news + 0x1d));
    exe::date_get_long_date_string_no_year(d1.as_ptr() as usize, s1.as_mut_ptr() as usize, 0xc8, 0xff);
    news_date(d2.as_mut_ptr() as usize, read_u16(news + 0x35), read_u16(news + 0x39));
    exe::date_get_long_date_string_no_year(d2.as_ptr() as usize, s2.as_mut_ptr() as usize, 0xc8, 0xff);
    let (s1, s2) = (s1.as_ptr() as usize, s2.as_ptr() as usize);
    let clash = read_i32(news + 0x21);
    let (g1, g2) = (news_g(c1 + club::GENDER_NAME), news_g(c2 + club::GENDER_NAME));
    if clash != -1 {
        let comp2 = news_comp(read_u32(news + 0x29));
        let cc = news_club(read_u32(news + 0x25));
        let cb = news_club(clash as u32);
        let (gk, gc, gb) = (news_g(comp2 + competition::SHORT_GENDER_NAME), news_g(cc + club::SHORT_GENDER_NAME), news_g(cb + club::SHORT_GENDER_NAME));
        f(NEWS_TEXT_BUFFER, 0xa692ec, g1, g1, g2, g2, lg, lg, 0x64, gb, gb, gc, gc, gk, gk, 0x64, c1 + club::NAME, c2 + club::NAME,
          lb, s1, cb + club::SHORT_NAME, cc + club::SHORT_NAME, comp2 + competition::SHORT_NAME, s2);
    } else {
        f(NEWS_TEXT_BUFFER, 0xa691c8, lg, lg, g1, g1, g2, g2, 0x64, 0x64, lb, c1 + club::NAME, c2 + club::NAME, s2, s1);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c1 + club::NAME, 0x7d5, c1);
    news_link(out, c2 + club::NAME, 0x7d5, c2);
    news_link(out, lb, 0x7d0, comp);
    if clash != -1 {
        covered!("gra_1 match rearranged clash");
        let cb = news_club(clash as u32);
        news_link(out, cb + club::SHORT_NAME, 0x7d5, cb);
        let cc = news_club(read_u32(news + 0x25));
        news_link(out, cc + club::SHORT_NAME, 0x7d5, cc);
        let comp2 = news_comp(read_u32(news + 0x29));
        news_link(out, comp2 + competition::SHORT_NAME, 0x7d0, comp2);
    }
    1
}

/// `Date::Date(this, a, b)` (8 bytes: the two words and the leap-year flag).
unsafe fn news_date(d: usize, a: u16, b: u16) {
    let ctor: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    ctor(d, a as u32, b as u32);
}

/// Type 3, the match chosen for live television. Slots: +5 and +9 the clubs, +0xd the competition, +0x11 and +0x15
/// the stage and round (words), +0x19/+0x29 and +0x25 the old and new date words (+0x29 the year of the new date,
/// +0x19 the day of the old one compared with the new +0x25), +0x21 and +0x2d the old and new time of day (bytes).
///
/// Body: the round's label (as in type 1, "Friendly"/"Reserve" lower-cased) and its gender; a new day: 0xa690f0 with
/// the new date without the year; the same day at another time of day: 0xa69044 with the time of day's name and
/// gender (`exe::date_get_time_of_day_name`, `exe::date_get_time_of_day_gender`); the same day and time: 0xa68fc0.
/// Links: the two clubs' names (0x7d5) and the competition on the label (0x7d0). Headline 0xa68f7c with the short names.
/// Original: part of `NewsItem::write_text` (0x77c473..0x77c846).
unsafe fn news_match_televised(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "match_televised";
    let f = news_format();
    let (c1, c2) = (news_club(read_u32(news + 5)), news_club(read_u32(news + 9)));
    if body == 0 {
        f(NEWS_TEXT_BUFFER, 0xa68f7c, news_g(c1 + club::SHORT_GENDER_NAME), news_g(c1 + club::SHORT_GENDER_NAME),
          news_g(c2 + club::SHORT_GENDER_NAME), news_g(c2 + club::SHORT_GENDER_NAME), c1 + club::SHORT_NAME, c2 + club::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
        news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
        return 1;
    }
    covered!("gra_1 match televised body");
    let comp = news_comp(read_u32(news + 0xd));
    let (stage, round) = (read_u16(news + 0x11) as u32, read_u16(news + 0x15) as u32);
    let mut label = [0u8; 0xc8];
    let lb = label.as_mut_ptr() as usize;
    exe::comp_util_format_comp_stage_round_label(comp, stage, round, 2, label.as_mut_ptr(), 0xc8);
    if news_streq(lb, 0x9db958) {
        label[0] = b'f';
    } else if news_streq(lb, 0x9b5b40) {
        label[0] = b'r';
    }
    let lg = exe::comp_util_get_label_gender(comp as u32, stage, round, 2) as i8 as i32;
    let (g1, g2) = (news_g(c1 + club::GENDER_NAME), news_g(c2 + club::GENDER_NAME));
    let (old_time, new_time) = (read_u8(news + 0x21), read_u8(news + 0x2d));
    if read_u16(news + 0x25) != read_u16(news + 0x19) {
        covered!("gra_1 match televised new day");
        let mut d = [0u8; 8];
        let mut s = [0u8; 0xc8];
        news_date(d.as_mut_ptr() as usize, read_u16(news + 0x25), read_u16(news + 0x29));
        exe::date_get_long_date_string_no_year(d.as_ptr() as usize, s.as_mut_ptr() as usize, 0xc8, 0xff);
        f(NEWS_TEXT_BUFFER, 0xa690f0, g1, g1, g2, g2, lg, lg, 0x64, c1 + club::NAME, c2 + club::NAME, lb, s.as_ptr() as usize);
        news_set(out, NEWS_TEXT_BUFFER);
    } else {
        if old_time != new_time {
            covered!("gra_1 match televised time of day");
            let name = exe::date_get_time_of_day_name(new_time as u32, 0);
            let tg = exe::date_get_time_of_day_gender(new_time as u32) as i32;
            f(NEWS_TEXT_BUFFER, 0xa69044, g1, g1, g2, g2, lg, lg, tg, c1 + club::NAME, c2 + club::NAME, lb, name);
        } else {
            f(NEWS_TEXT_BUFFER, 0xa68fc0, g1, g1, g2, g2, lg, lg, c1 + club::NAME, c2 + club::NAME, lb);
        }
        news_set(out, NEWS_TEXT_BUFFER);
    }
    news_link(out, c1 + club::NAME, 0x7d5, c1);
    news_link(out, c2 + club::NAME, 0x7d5, c2);
    news_link(out, lb, 0x7d0, comp);
    1
}

/// Type 4, the bids to host a competition. Slots: +5 the competition, +9 the year (word), +0xd, +0x15 and +0x1d three
/// bidding nations and +0x11 the joint bidder of the first one (−1 none).
///
/// Body: first the nations' links (0x7d5 on the nation's name, to its national team, `exe::database_get_national_team`):
/// the first and its joint bidder, or each of the three that is set; then 0xa68ee0 (three nations) or 0xa68e1c (with
/// the joint bid: four), with the genders, the names, the year and the competition's name; the competition's link
/// (0x7d0). Headline 0xa68de0 with the year and the competition's short name.
/// Original: part of `NewsItem::write_text` (0x77c84f..0x77cb6c).
unsafe fn news_host_bids(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "host_bids";
    let f = news_format();
    let comp = news_comp(read_u32(news + 5));
    let year = read_u16(news + 9) as u32;
    if body == 0 {
        let g = news_g(comp + competition::SHORT_GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa68de0, g, g, year, comp + competition::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 host bids body");
    let n = [read_i32(news + 0xd), read_i32(news + 0x11), read_i32(news + 0x15), read_i32(news + 0x19), read_i32(news + 0x1d)];
    let team_link = |i: i32| {
        let nat = news_nation(i as u32);
        news_link(out, nat + 4, 0x7d5, exe::database_get_national_team(nat));
    };
    for k in 0..3 {
        if k == 0 && n[0] > -1 && n[1] > -1 {
            team_link(n[0]);
            team_link(n[1]);
        } else if n[2 * k] > -1 {
            team_link(n[2 * k]);
        }
    }
    let gc = news_g(comp + competition::GENDER_NAME);
    let nat = |i: i32| news_nation(i as u32);
    if n[1] == -1 {
        let (a, b, c) = (nat(n[0]), nat(n[2]), nat(n[4]));
        f(NEWS_TEXT_BUFFER, 0xa68ee0, news_g(a + 0x37), news_g(a + 0x37), news_g(b + 0x37), news_g(b + 0x37), news_g(c + 0x37),
          news_g(c + 0x37), gc, gc, a + 4, b + 4, c + 4, year, comp + competition::NAME);
    } else {
        covered!("gra_1 host bids joint");
        let (a, j, b, c) = (nat(n[0]), nat(n[1]), nat(n[2]), nat(n[4]));
        f(NEWS_TEXT_BUFFER, 0xa68e1c, news_g(a + 0x37), news_g(a + 0x37), news_g(j + 0x37), news_g(j + 0x37), news_g(b + 0x37),
          news_g(b + 0x37), news_g(c + 0x37), news_g(c + 0x37), gc, gc, a + 4, j + 4, b + 4, c + 4, year, comp + competition::NAME);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, comp + competition::NAME, 0x7d0, comp);
    1
}

/// Type 5, the host of a competition chosen. Slots: +5 the competition, +9 the year (word), +0xd the host nation (−1
/// none) and +0x11 its co-host (−1 none).
///
/// Body: with a host, 0xa68d60 (co-hosts) or 0xa68d00 (one host) with the genders, the names, the year and the
/// competition's name, the text, and each host's link (0x7d5, to its national team); without a host no text; then
/// the competition's link (0x7d0). Headline 0xa68ccc with the year and the competition's short name.
/// Original: part of `NewsItem::write_text` (0x77cb75..0x77cd8e).
unsafe fn news_host_chosen(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "host_chosen";
    let f = news_format();
    let comp = news_comp(read_u32(news + 5));
    let year = read_u16(news + 9) as u32;
    if body == 0 {
        let g = news_g(comp + competition::SHORT_GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa68ccc, g, g, year, comp + competition::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 host chosen body");
    let (h, co) = (read_i32(news + 0xd), read_i32(news + 0x11));
    let gc = news_g(comp + competition::GENDER_NAME);
    let team_link = |nat: usize| news_link(out, nat + 4, 0x7d5, exe::database_get_national_team(nat));
    if h > -1 {
        let a = news_nation(h as u32);
        if co > -1 {
            covered!("gra_1 host chosen co-hosts");
            let b = news_nation(co as u32);
            f(NEWS_TEXT_BUFFER, 0xa68d60, news_g(a + 0x37), news_g(a + 0x37), news_g(b + 0x37), news_g(b + 0x37), gc, gc, a + 4, b + 4,
              year, comp + competition::NAME);
            news_set(out, NEWS_TEXT_BUFFER);
            team_link(a);
            team_link(b);
        } else {
            f(NEWS_TEXT_BUFFER, 0xa68d00, news_g(a + 0x37), news_g(a + 0x37), gc, gc, a + 4, year, comp + competition::NAME);
            news_set(out, NEWS_TEXT_BUFFER);
            team_link(a);
        }
    }
    news_link(out, comp + competition::NAME, 0x7d0, comp);
    1
}

/// The friendly invitation's answer text writer (`friendly_check_match_invitation`, 0x5ca090, thiscall on
/// `G_FRIENDLY` 0xae2c74: the inviting club, the invited, the club asked, the date's two words, the match, the buffer
/// and its size 0x190; nonzero when it wrote the whole answer).
unsafe fn news_friendly_answer(a: usize, b: usize, c: usize, w1: u32, w2: u32, m: usize, buf: usize) -> u32 {
    let w: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, usize, usize, u32) -> u32 = core::mem::transmute(0x5ca090usize);
    w(read_usize(0xae2c74), a, b, c, w1, w2, m, buf, 0x190)
}

/// `strcpy(dst, src)`.
unsafe fn news_strcpy(dst: usize, src: usize) {
    let mut k = 0;
    loop {
        let c = read_u8(src + k);
        write_u8(dst + k, c);
        if c == 0 {
            break;
        }
        k += 1;
    }
}

/// Type 6, the friendly proposal. Slots: +5 and +9 the two clubs, +0xd the host club, +0x11 and +0x15 the date
/// (words), +0x19 the answer (3 accepted, 4 rejected).
///
/// Body: the date (`Date::Date`, assigned with `exe::date_operator_3d`), the host's friendly match on that date
/// (`exe::friendly_find_friendly_match`) and the date's long text. When the first club is the host it "offered" the
/// match (0xa68c38 at the first club's stadium, whose name is copied with `sprintf` used as format, or 0xa68bc0
/// without a stadium: the first club then the second); otherwise the first club "challenged" the second (0xa68aac or
/// 0xa68a2c, the second club then the first; the stadium is still the first club's). The answer paragraph: accepted
/// 0xa68b84 or rejected 0xa68b48 with the answering club (the second club in the offer, the first in the challenge),
/// joined as "%s\n\n%s" in the buffer; any other answer by `friendly_check_match_invitation` (the first paragraph alone
/// when it returns nonzero, otherwise joined with what it left in the second). Links: the two clubs' short names
/// (0x7d5). Headline 0xa689d4 with the short names and their links.
/// Original: part of `NewsItem::write_text` (0x77cd97..0x77d2e7).
unsafe fn news_friendly_proposal(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "friendly_proposal";
    let f = news_format();
    let sprintf = news_sprintf();
    let (mut d, mut d2, mut tmp) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let dp = d.as_mut_ptr() as usize;
    exe::date_date_54a110(dp);
    let host = read_u32(news + 0xd);
    let (w1, w2) = (read_u16(news + 0x11), read_u16(news + 0x15));
    let (i1, i2) = (read_u32(news + 5), read_u32(news + 9));
    let answer = read_u8(news + 0x19);
    news_date(d2.as_mut_ptr() as usize, w1, w2);
    exe::date_operator_3d(dp, tmp.as_mut_ptr() as usize, d2.as_ptr() as usize);
    let (c1, c2) = (news_club(i1), news_club(i2));
    let sg = |c: usize| news_g(c + club::SHORT_GENDER_NAME);
    if body == 0 {
        f(NEWS_TEXT_BUFFER, 0xa689d4, sg(c1), sg(c1), sg(c2), sg(c2), c1 + club::SHORT_NAME, c2 + club::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
        news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
        return 1;
    }
    covered!("gra_1 friendly proposal body");
    let (mut first, mut second, mut stadium_name, mut date_text) = ([0u8; 0x400], [0u8; 0x190], [0u8; 0x12c], [0u8; 0x194]);
    let (p1, p2, ps, pd) = (first.as_mut_ptr() as usize, second.as_mut_ptr() as usize, stadium_name.as_mut_ptr() as usize, date_text.as_mut_ptr() as usize);
    let m = exe::friendly_find_friendly_match(read_usize(0xae2c74), dp, news_club(host));
    exe::date_get_long_date_string(dp, pd, 0x64, 0xff);
    let offer = i1 == host;
    let (lead, other) = if offer { (c1, c2) } else { (c2, c1) };
    let stadium = read_usize(c1 + club::STADIUM);
    let (with_stadium, without) = if offer { (0xa68c38, 0xa68bc0) } else { (0xa68aac, 0xa68a2c) };
    if stadium != 0 {
        sprintf(ps, stadium + 4);
        let gs = news_g(stadium + 0x37);
        f(p1, with_stadium, sg(lead), sg(lead), sg(other), sg(other), gs, gs, 0x64, lead + club::SHORT_NAME, other + club::SHORT_NAME, ps, pd);
    } else {
        f(p1, without, sg(lead), sg(lead), sg(other), sg(other), 0x64, lead + club::SHORT_NAME, other + club::SHORT_NAME, pd);
    }
    let answering = if offer { c2 } else { c1 };
    let joined = match answer {
        3 => {
            f(p2, 0xa68b84, sg(answering), sg(answering), answering + club::SHORT_NAME);
            true
        }
        4 => {
            f(p2, 0xa68b48, sg(answering), sg(answering), answering + club::SHORT_NAME);
            true
        }
        _ => {
            covered!("gra_1 friendly proposal answer");
            news_friendly_answer(c1, c2, answering, w1 as u32, w2 as u32, m, p2) == 0
        }
    };
    if joined {
        sprintf(NEWS_TEXT_BUFFER, 0x991948, p1, p2); // "%s\n\n%s"
    } else {
        news_strcpy(NEWS_TEXT_BUFFER, p1);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
    news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
    1
}

/// The date of two words of a news, as the original builds it: a default `Date` (`exe::date_date_54a110`) assigned
/// (`exe::date_operator_3d`) from `Date::Date(a, b)`.
unsafe fn news_assigned_date(d: usize, a: u16, b: u16) {
    let (mut d2, mut tmp) = ([0u8; 8], [0u8; 8]);
    exe::date_date_54a110(d);
    news_date(d2.as_mut_ptr() as usize, a, b);
    exe::date_operator_3d(d, tmp.as_mut_ptr() as usize, d2.as_ptr() as usize);
}

/// Type 7, the answer to a friendly offer. Slots: +5 and +9 the two clubs, +0xd the club that made the offer,
/// +0x11 and +0x15 the date (words), +0x19 the answer (3 accepted, 4 rejected). The answering club is the second
/// when the first made the offer, otherwise the first.
///
/// Body: the date's long text (`exe::date_get_long_date_string`); accepted 0xa68948 or rejected 0xa688bc with the
/// answering club and the offering club (genders, names) and the date; any other answer leaves the buffer as it was.
/// Links: the first and second clubs' names (0x7d5). Headline: accepted 0xa68880 or rejected 0xa68844 with the
/// answering club's short name, and its link.
/// Original: part of `NewsItem::write_text` (0x77d2f0..0x77d4f3).
unsafe fn news_friendly_answered(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "friendly_answered";
    let f = news_format();
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    let (i1, i2, offering) = (read_u32(news + 5), read_u32(news + 9), read_u32(news + 0xd));
    let answer = read_u8(news + 0x19);
    let answering = news_club(if offering == i1 { i2 } else { i1 });
    news_assigned_date(dp, read_u16(news + 0x11), read_u16(news + 0x15));
    if body == 0 {
        let g = news_g(answering + club::SHORT_GENDER_NAME);
        match answer {
            3 => {
                f(NEWS_TEXT_BUFFER, 0xa68880, g, g, answering + club::SHORT_NAME);
            }
            4 => {
                f(NEWS_TEXT_BUFFER, 0xa68844, g, g, answering + club::SHORT_NAME);
            }
            _ => {}
        }
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, answering + club::SHORT_NAME, 0x7d5, answering);
        return 1;
    }
    covered!("gra_1 friendly answered body");
    let mut s = [0u8; 0x194];
    let sp = s.as_mut_ptr() as usize;
    exe::date_get_long_date_string(dp, sp, 0x64, 0xff);
    let o = news_club(offering);
    let (ga, go) = (news_g(answering + club::GENDER_NAME), news_g(o + club::GENDER_NAME));
    match answer {
        3 => {
            f(NEWS_TEXT_BUFFER, 0xa68948, ga, ga, go, go, 0x64, answering + club::NAME, o + club::NAME, sp);
        }
        4 => {
            f(NEWS_TEXT_BUFFER, 0xa688bc, ga, ga, go, go, 0x64, answering + club::NAME, o + club::NAME, sp);
        }
        _ => {}
    }
    news_set(out, NEWS_TEXT_BUFFER);
    let (c1, c2) = (news_club(i1), news_club(i2));
    news_link(out, c1 + club::NAME, 0x7d5, c1);
    news_link(out, c2 + club::NAME, 0x7d5, c2);
    1
}

/// Type 8, the friendly cancelled. Slots: +5 and +9 the two clubs, +0xd and +0x11 the date (words).
///
/// Body 0xa6878c with the clubs' short genders and short names and the date's long text; headline 0xa68734 with the
/// short names. Links: the two clubs' short names (0x7d5).
/// Original: part of `NewsItem::write_text` (0x77d4fc..0x77d625).
unsafe fn news_friendly_cancelled(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "friendly_cancelled";
    let f = news_format();
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    let (c1, c2) = (news_club(read_u32(news + 5)), news_club(read_u32(news + 9)));
    news_assigned_date(dp, read_u16(news + 0xd), read_u16(news + 0x11));
    let (g1, g2) = (news_g(c1 + club::SHORT_GENDER_NAME), news_g(c2 + club::SHORT_GENDER_NAME));
    if body != 0 {
        covered!("gra_1 friendly cancelled body");
        let mut s = [0u8; 0x194];
        let sp = s.as_mut_ptr() as usize;
        exe::date_get_long_date_string(dp, sp, 0x64, 0xff);
        f(NEWS_TEXT_BUFFER, 0xa6878c, g1, g1, g2, g2, 0x64, c1 + club::SHORT_NAME, c2 + club::SHORT_NAME, sp);
    } else {
        f(NEWS_TEXT_BUFFER, 0xa68734, g1, g1, g2, g2, c1 + club::SHORT_NAME, c2 + club::SHORT_NAME);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
    news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
    1
}

/// Type 9, the invitation to a friendly tournament. Slots: +5 the inviting club, +9 the invited club, +0xd the
/// tournament's index (byte), +0x11 the answer (2 none yet, 3 accepted, 4 declined), +0x19 and +0x1d the date (words).
///
/// Body: the date (assigned as in the friendlies) and the tournament on it (`friendly_find_tournament`, 0x5c89a0,
/// thiscall on `G_FRIENDLY`: the index byte and the date). With the tournament: 0xa68698 with the clubs' short names,
/// the tournament's name (+0x31) and its start date's long text (+4); accepted 0xa68658 or declined 0xa68618 with the
/// invited club's short name, joined as "%s\n\n%s"; any other answer by `friendly_tournament_answer` (0x5ca290: the
/// invited club, the tournament, the buffer, 0x190; the first paragraph alone when it returns nonzero). Links: both
/// clubs' short names (0x7d5) and the tournament's name (0x7d1). Without it: 0xa6858c with the news date's long
/// text; accepted or declined with the invited club's full name (and its short gender); no answer yet (2) the first
/// paragraph alone; links of both clubs. Headline 0xa6854c with the inviting club's short name, and its link.
///
/// Defect of the original: without the tournament, an answer other than 2, 3 or 4 joins the first paragraph with a
/// buffer never written (stack leftovers); the copy joins it with an empty text.
/// Original: part of `NewsItem::write_text` (0x77d62e..0x77da1a).
unsafe fn news_tournament_invitation(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "tournament_invitation";
    let f = news_format();
    let sprintf = news_sprintf();
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    let (c1, c2) = (news_club(read_u32(news + 5)), news_club(read_u32(news + 9)));
    let index = read_u8(news + 0xd);
    let answer = read_u8(news + 0x11);
    let sg = |c: usize| news_g(c + club::SHORT_GENDER_NAME);
    if body == 0 {
        exe::date_date_54a110(dp);
        f(NEWS_TEXT_BUFFER, 0xa6854c, sg(c1), sg(c1), c1 + club::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
        return 1;
    }
    covered!("gra_1 tournament invitation body");
    news_assigned_date(dp, read_u16(news + 0x19), read_u16(news + 0x1d));
    let find: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x5c89a0usize);
    let tour = find(read_usize(0xae2c74), index as u32, dp);
    let (mut first, mut second, mut date_text) = ([0u8; 0x400], [0u8; 0x190], [0u8; 0x194]);
    let (p1, p2, pd) = (first.as_mut_ptr() as usize, second.as_mut_ptr() as usize, date_text.as_mut_ptr() as usize);
    let joined;
    if tour != 0 {
        exe::date_get_long_date_string(tour + 4, pd, 0x64, 0xff);
        let tname = tour + 0x31;
        f(p1, 0xa68698, sg(c1), sg(c1), sg(c2), sg(c2), 0x64, c1 + club::SHORT_NAME, c2 + club::SHORT_NAME, tname, pd);
        joined = match answer {
            3 => {
                f(p2, 0xa68658, sg(c2), sg(c2), c2 + club::SHORT_NAME);
                true
            }
            4 => {
                f(p2, 0xa68618, sg(c2), sg(c2), c2 + club::SHORT_NAME);
                true
            }
            _ => {
                covered!("gra_1 tournament invitation answer");
                let w: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(0x5ca290usize);
                w(read_usize(0xae2c74), c2, tour, p2, 0x190) == 0
            }
        };
    } else {
        covered!("gra_1 tournament invitation no tournament");
        exe::date_get_long_date_string(dp, pd, 0x64, 0xff);
        f(p1, 0xa6858c, sg(c1), sg(c1), sg(c2), sg(c2), 0x64, c1 + club::SHORT_NAME, c2 + club::SHORT_NAME, pd);
        joined = match answer {
            3 => {
                f(p2, 0xa68658, sg(c2), sg(c2), c2 + club::NAME);
                true
            }
            4 => {
                f(p2, 0xa68618, sg(c2), sg(c2), c2 + club::NAME);
                true
            }
            2 => false,
            _ => true,
        };
    }
    if joined {
        sprintf(NEWS_TEXT_BUFFER, 0x991948, p1, p2); // "%s\n\n%s"
    } else {
        news_strcpy(NEWS_TEXT_BUFFER, p1);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
    news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
    if tour != 0 {
        news_link(out, tour + 0x31, 0x7d1, tour);
    }
    1
}

/// Type 10, the answer to a friendly tournament invitation. Slots: +5 the inviting club, +9 the invited club, +0xd
/// and +0x11 the date (words), +0x15 the answer (3 accepted, 4 declined), +0x19 the tournament's index (byte).
///
/// Body: the tournament on the date (as in type 9) and the date's text (`Date::get_date_string`, 0x54c3b0, 0x64,
/// 0xff). With the tournament: accepted 0xa6848c or declined 0xa683cc (the invited club, then the inviting club, their
/// genders and names, the tournament's name and the date), links of both clubs' names (0x7d5, invited first) and of
/// the tournament (0x7d1); without it 0xa68328 or 0xa68284 and the two clubs' links. Any other answer leaves the buffer
/// as it was. Headline: accepted 0xa6823c or rejected 0xa681f4 with the invited club's short name, and its link.
/// Original: part of `NewsItem::write_text` (0x77da23..0x77dd7d).
unsafe fn news_tournament_answered(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "tournament_answered";
    let f = news_format();
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    let (c1, c2) = (news_club(read_u32(news + 5)), news_club(read_u32(news + 9)));
    let answer = read_u8(news + 0x15);
    let index = read_u8(news + 0x19);
    news_assigned_date(dp, read_u16(news + 0xd), read_u16(news + 0x11));
    if body == 0 {
        let g = news_g(c2 + club::SHORT_GENDER_NAME);
        match answer {
            3 => {
                f(NEWS_TEXT_BUFFER, 0xa6823c, g, g, c2 + club::SHORT_NAME);
            }
            4 => {
                f(NEWS_TEXT_BUFFER, 0xa681f4, g, g, c2 + club::SHORT_NAME);
            }
            _ => {}
        }
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
        return 1;
    }
    covered!("gra_1 tournament answered body");
    let find: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x5c89a0usize);
    let tour = find(read_usize(0xae2c74), index as u32, dp);
    let mut s = [0u8; 0x194];
    let sp = s.as_mut_ptr() as usize;
    let date_string: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x54c3b0usize);
    date_string(dp, sp, 0x64, 0xff);
    let (g1, g2) = (news_g(c1 + club::GENDER_NAME), news_g(c2 + club::GENDER_NAME));
    if tour != 0 {
        let tname = tour + 0x31;
        match answer {
            3 => {
                f(NEWS_TEXT_BUFFER, 0xa6848c, g2, g2, g1, g1, 0x64, c2 + club::NAME, c1 + club::NAME, tname, sp);
            }
            4 => {
                f(NEWS_TEXT_BUFFER, 0xa683cc, g2, g2, g1, g1, 0x64, c2 + club::NAME, c1 + club::NAME, tname, sp);
            }
            _ => {}
        }
    } else {
        covered!("gra_1 tournament answered no tournament");
        match answer {
            3 => {
                f(NEWS_TEXT_BUFFER, 0xa68328, g2, g2, g1, g1, 0x64, c2 + club::NAME, c1 + club::NAME, sp);
            }
            4 => {
                f(NEWS_TEXT_BUFFER, 0xa68284, g2, g2, g1, g1, 0x64, c2 + club::NAME, c1 + club::NAME, sp);
            }
            _ => {}
        }
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c2 + club::NAME, 0x7d5, c2);
    news_link(out, c1 + club::NAME, 0x7d5, c1);
    if tour != 0 {
        news_link(out, tour + 0x31, 0x7d1, tour);
    }
    1
}

/// Type 11, the friendly tournament cancelled. Slots: +5 the organising club, +9 and +0xd the date (words).
///
/// Body 0xa68174 with the club's short name and the date's long text; headline 0xa68130 with the short name. Link:
/// the club's short name (0x7d5).
/// Original: part of `NewsItem::write_text` (0x77dd86..0x77de79).
unsafe fn news_tournament_cancelled(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "tournament_cancelled";
    let f = news_format();
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    let c = news_club(read_u32(news + 5));
    news_assigned_date(dp, read_u16(news + 9), read_u16(news + 0xd));
    let g = news_g(c + club::SHORT_GENDER_NAME);
    if body != 0 {
        covered!("gra_1 tournament cancelled body");
        let mut s = [0u8; 0x194];
        let sp = s.as_mut_ptr() as usize;
        exe::date_get_long_date_string(dp, sp, 0x64, 0xff);
        f(NEWS_TEXT_BUFFER, 0xa68174, g, g, 0x64, c + club::SHORT_NAME, sp);
    } else {
        f(NEWS_TEXT_BUFFER, 0xa68130, g, g, c + club::SHORT_NAME);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c + club::SHORT_NAME, 0x7d5, c);
    1
}

/// Type 12, the coach's report. Slots: +5 a club, +9 the coach (index of the coach table 0xadadfc), +0xd and +0x11
/// two bytes, +0x15 a word, +0x19 a dword. Without the coach's record the news.cpp error box (line 0x61f) and 0;
/// otherwise the coach's virtual text writer (slot +0x48 of its table: the club, the byte +0x11, the byte +0xd, the
/// word, 0, 0, the dword, 0, `body`, `out`) and 1.
/// Original: part of `NewsItem::write_text` (0x77de82..0x77df0b).
unsafe fn news_comp_report(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "comp_report";
    let coach = read_usize(read_usize(0xadadfc).wrapping_add((read_u32(news + 9) as usize).wrapping_mul(4)));
    if coach == 0 {
        news_error_box(0x61f);
        return 0;
    }
    covered!("gra_1 comp report");
    let w: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> u32 =
        core::mem::transmute(read_usize(read_usize(coach) + 0x48));
    w(coach, read_u32(news + 5), read_u8(news + 0x11) as u32, read_u8(news + 0xd) as u32, read_u16(news + 0x15) as u32, 0, 0,
      read_u32(news + 0x19), 0, body, out);
    1
}

/// Type 13, the league's fixture list decided. Slots: +5 the competition, +9 the club, +0xd the season's year (word);
/// the club's first three games at +0x11 + 16·k (k = 0..2): the date's two words (+0x11, +0x15), the opponent
/// (+0x19) and home (+0x1d, word, nonzero at home).
///
/// The season's name: "<year>/<(year+1) % 100>" (0xa680e0) when the key nation of the competition
/// (`exe::key_nation_index_of_comp`) has a season over two years (the dword at 0xb63d77 + nation·0x48), otherwise
/// "<year>" (0xa680a4), with the competition's short name. Headline 0xa67f20 with the season's name and the
/// competition's link (0x7d0). Body: each game "<opponent> (home)" (0xa68078) or "(away)" (0xa6804c) with the
/// opponent's short name, and its date's long text (`Date::Date` assigned, `exe::date_get_long_date_string`, 0x32);
/// then 0xa67f68 with the club's short name, the three games and the three dates; links: the competition's short name
/// (0x7d0), the club's and the three opponents' short names (0x7d5).
/// Original: part of `NewsItem::write_text` (0x77df14..0x77e2e2).
unsafe fn news_fixture_list(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "fixture_list";
    let f = news_format();
    let comp = news_comp(read_u32(news + 5));
    let year = read_u16(news + 0xd) as i32;
    let nation = exe::key_nation_index_of_comp(comp) as i8 as i32;
    let mut season = [0u8; 0xc8];
    let sp = season.as_mut_ptr() as usize;
    let gs = news_g(comp + competition::SHORT_GENDER_NAME);
    if read_u32((0xb63d77usize).wrapping_add((nation as usize).wrapping_mul(0x48))) != 0 {
        f(sp, 0xa680e0, gs, gs, year, (year + 1) % 100, comp + competition::SHORT_NAME);
    } else {
        f(sp, 0xa680a4, gs, gs, year, comp + competition::SHORT_NAME);
    }
    if body == 0 {
        f(NEWS_TEXT_BUFFER, 0xa67f20, gs, gs, sp);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 fixture list body");
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    exe::date_date_54a110(dp);
    let c = news_club(read_u32(news + 9));
    let (mut games, mut dates) = ([0u8; 0x3d * 3], [0u8; 0x32 * 3]);
    let (gp, dtp) = (games.as_mut_ptr() as usize, dates.as_mut_ptr() as usize);
    let mut opp = [0usize; 3];
    for k in 0..3 {
        let base = news + 0x11 + 16 * k;
        let (w1, w2) = (read_u16(base), read_u16(base + 4));
        let o = news_club(read_u32(base + 8));
        let home = read_u16(base + 0xc) != 0;
        opp[k] = o;
        let g = news_g(o + club::SHORT_GENDER_NAME);
        f(gp + 0x3d * k, if home { 0xa68078 } else { 0xa6804c }, g, g, o + club::SHORT_NAME);
        let (mut d2, mut tmp) = ([0u8; 8], [0u8; 8]);
        news_date(d2.as_mut_ptr() as usize, w1, w2);
        exe::date_operator_3d(dp, tmp.as_mut_ptr() as usize, d2.as_ptr() as usize);
        exe::date_get_long_date_string(dp, dtp + 0x32 * k, 0x32, 0xff);
    }
    let sg = |x: usize| news_g(x + club::SHORT_GENDER_NAME);
    f(NEWS_TEXT_BUFFER, 0xa67f68, sg(c), sg(c), sg(opp[0]), sg(opp[1]), sg(opp[2]), 0x64, 0x64, 0x64, c + club::SHORT_NAME, gp,
      gp + 0x3d, gp + 0x7a, dtp, dtp + 0x32, dtp + 0x64);
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
    news_link(out, c + club::SHORT_NAME, 0x7d5, c);
    for o in opp {
        news_link(out, o + club::SHORT_NAME, 0x7d5, o);
    }
    1
}

/// Type 16, the coach's match report. Slots: +5 a club, +9 the coach (index of the coach table 0xadadfc), +0xd and
/// +0x11 two bytes, +0x15 a dword, and the fixture from slot 5 on (`comp_util_read_fixture_from_news_slots`, 0x4b0010,
/// cdecl: the news, the fixture, the slot cursor). Without the coach's record 0 (no box); otherwise the coach's virtual
/// text writer (slot +0x48: the club, the byte +0xd, the fixture's dwords +0x42 and +0x32, the fixture, the word of
/// the slot after the fixture, the dword +0x15, the byte +0x11, `body`, `out`) and 1.
/// Original: part of `NewsItem::write_text` (0x77e2eb..0x77e37d).
unsafe fn news_comp_match_report(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "comp_match_report";
    let (b1, b2, d, c) = (read_u8(news + 0xd), read_u8(news + 0x11), read_u32(news + 0x15), read_u32(news + 5));
    let mut fixture = [0u8; 0xc8];
    let fp = fixture.as_mut_ptr() as usize;
    let mut k: u8 = 5;
    let read: unsafe extern "C" fn(usize, usize, *mut u8) -> u32 = core::mem::transmute(0x4b0010usize);
    read(news, fp, &mut k);
    let coach = read_usize(read_usize(0xadadfc).wrapping_add((read_u32(news + 9) as usize).wrapping_mul(4)));
    if coach == 0 {
        return 0;
    }
    covered!("gra_1 comp match report");
    let slot = read_u16(news.wrapping_add(5).wrapping_add((k as i8 as isize as usize).wrapping_mul(4))) as u32;
    let w: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize) -> u32 =
        core::mem::transmute(read_usize(read_usize(coach) + 0x48));
    w(coach, c, b1 as u32, read_u32(fp + 0x42), read_u32(fp + 0x32), fp, slot, d, b2 as u32, body, out);
    1
}

/// Types 17 and 18, a play-off place won by the form over a third or a quarter of the season. Slots: +5 the club,
/// +9 the part (1, 2, 3...), +0xd the competition.
///
/// Body: by the part, the third (17: 0xa67e88 first, 0xa67df0 middle, 0xa67d58 last) or the quarter (18: 0xa67c98,
/// 0xa67bfc, 0xa67b64, 0xa67acc), with the club's and the competition's genders and names; links: the club's name
/// (0x7d5) and the competition's (0x7d0). Headline (both) 0xa67d30 with the club's short name, and its link.
/// Original: part of `NewsItem::write_text` (0x77e386..0x77e56f).
unsafe fn news_playoff_place(news: usize, body: u32, out: usize, quarters: bool) -> u32 {
    NEWS_LAST_PIECE = if quarters { "playoff_place_quarter" } else { "playoff_place_third" };
    let f = news_format();
    let c = news_club(read_u32(news + 5));
    let part = read_u8(news + 9);
    let comp = news_comp(read_u32(news + 0xd));
    if body == 0 {
        let g = news_g(c + club::SHORT_GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa67d30, g, g, c + club::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c + club::SHORT_NAME, 0x7d5, c);
        return 1;
    }
    covered!("gra_1 playoff place body");
    let fmt = if quarters {
        match part {
            1 => 0xa67c98,
            2 => 0xa67bfc,
            3 => 0xa67b64,
            _ => 0xa67acc,
        }
    } else {
        match part {
            1 => 0xa67e88,
            2 => 0xa67df0,
            _ => 0xa67d58,
        }
    };
    let (gc, gk) = (news_g(c + club::GENDER_NAME), news_g(comp + competition::GENDER_NAME));
    f(NEWS_TEXT_BUFFER, fmt, gc, gc, gk, gk, c + club::NAME, comp + competition::NAME);
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c + club::NAME, 0x7d5, c);
    news_link(out, comp + competition::NAME, 0x7d0, comp);
    1
}

/// Type 19, the nation's leagues reshuffled. Slots: +5 the club, +9 the competition. Without the club's stadium, the
/// stadium's city (+0x38), the competition or its nation (`competition::NATION`), the text "League reshuffled error
/// message" alone.
///
/// Body 0xa67a2c with the club's name, the competition's short name, the city's name and the nation's name (and their
/// genders, `city::GENDER_NAME` for the city); links: the club (0x7d5), the competition's short name (0x7d0) and the
/// nation's name to its national team (0x7d5, `exe::database_get_national_team`). Headline 0xa67a00 with the nation's
/// short name, and its link.
/// Original: part of `NewsItem::write_text` (0x77e578..0x77e6f4).
unsafe fn news_leagues_reshuffled(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "leagues_reshuffled";
    let f = news_format();
    let c = news_club(read_u32(news + 5));
    let comp = news_comp(read_u32(news + 9));
    let stadium = if c != 0 { read_usize(c + club::STADIUM) } else { 0 };
    let city = if stadium != 0 { read_usize(stadium + 0x38) } else { 0 };
    let nation = if city != 0 && comp != 0 { read_usize(comp + competition::NATION) } else { 0 };
    if nation == 0 {
        news_set(out, 0xa679e0); // "League reshuffled error message"
        return 1;
    }
    if body == 0 {
        let g = news_g(nation + 0x52);
        f(NEWS_TEXT_BUFFER, 0xa67a00, g, g, nation + 0x38);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, nation + 0x38, 0x7d5, exe::database_get_national_team(nation));
        return 1;
    }
    covered!("gra_1 leagues reshuffled body");
    let (gc, gk, gt, gn) = (news_g(c + club::GENDER_NAME), news_g(comp + competition::SHORT_GENDER_NAME), news_g(city + city::GENDER_NAME), news_g(nation + 0x37));
    f(NEWS_TEXT_BUFFER, 0xa67a2c, gc, gc, gk, gk, gt, gt, gn, gn, c + club::NAME, comp + competition::SHORT_NAME, city + 4, nation + 4);
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c + club::NAME, 0x7d5, c);
    news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
    news_link(out, nation + 4, 0x7d5, exe::database_get_national_team(nation));
    1
}

/// The round's label of a competition in `buf` (0xc8 bytes, `exe::comp_util_format_comp_stage_round_label` with the
/// form; "Friendly" and "Reserve" get a lower-case first letter) and its gender (`exe::comp_util_get_label_gender`).
unsafe fn news_round_label(comp: usize, stage: u16, round: u16, form: u32, buf: &mut [u8; 0xc8]) -> i32 {
    let lb = buf.as_mut_ptr() as usize;
    exe::comp_util_format_comp_stage_round_label(comp, stage as u32, round as u32, form, buf.as_mut_ptr(), 0xc8);
    if news_streq(lb, 0x9db958) {
        buf[0] = b'f'; // "Friendly"
    } else if news_streq(lb, 0x9b5b40) {
        buf[0] = b'r'; // "Reserve"
    }
    exe::comp_util_get_label_gender(comp as u32, stage as u32, round as u32, form) as i8 as i32
}

/// Type 20, the cup draw. Slots: +5 the competition, +9 and +0xd the stage and round (words), +0x11 the drawn club,
/// +0x15 its opponent, +0x19 nonzero for two legs, +0x1d the stadium (−1 unknown), +0x21 and +0x25 the date (words).
///
/// Body: 0xa67958 with the drawn club and the opponent (short names and genders) and the round's label (form 2), its
/// text and links (the two clubs 0x7d5, the competition on the label 0x7d0); then a second paragraph appended
/// (`exe::text_append_string`): with the stadium, its name — with the city's name when it differs and is not
/// "Unknown": "%s (%s)" (0x9a3d8c), or "%s, %s" (0x98f6ac) in languages 0 and 1; the city's name alone when they are
/// equal or the stadium is "Unknown"; the stadium's when it has no city — and 0xa678cc (first leg) or 0xa67854 (the
/// game) with the date's long text; without the stadium 0xa677f8 or 0xa677a8 with the date. Headline 0xa6776c with the
/// round's label (form 3), and its link.
/// Original: part of `NewsItem::write_text` (0x77e6fd..0x77ed30).
unsafe fn news_cup_draw(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "cup_draw";
    let f = news_format();
    let sprintf = news_sprintf();
    let comp = news_comp(read_u32(news + 5));
    let (stage, round) = (read_u16(news + 9), read_u16(news + 0xd));
    let mut label = [0u8; 0xc8];
    let lb = label.as_ptr() as usize;
    if body == 0 {
        let lg = news_round_label(comp, stage, round, 3, &mut label);
        f(NEWS_TEXT_BUFFER, 0xa6776c, lg, lg, lb);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, lb, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 cup draw body");
    let (drawn, opponent) = (news_club(read_u32(news + 0x11)), news_club(read_u32(news + 0x15)));
    let two_legs = read_u32(news + 0x19) != 0;
    let stadium = read_i32(news + 0x1d);
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    news_date(dp, read_u16(news + 0x21), read_u16(news + 0x25));
    let lg = news_round_label(comp, stage, round, 2, &mut label);
    let (ga, gb) = (news_g(drawn + club::SHORT_GENDER_NAME), news_g(opponent + club::SHORT_GENDER_NAME));
    f(NEWS_TEXT_BUFFER, 0xa67958, ga, ga, gb, gb, lg, lg, drawn + club::SHORT_NAME, opponent + club::SHORT_NAME, lb);
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, drawn + club::SHORT_NAME, 0x7d5, drawn);
    news_link(out, opponent + club::SHORT_NAME, 0x7d5, opponent);
    news_link(out, lb, 0x7d0, comp);
    let mut s = [0u8; 0xc8];
    let sp = s.as_mut_ptr() as usize;
    exe::date_get_long_date_string(dp, sp, 0x64, 0xff);
    if stadium != -1 {
        covered!("gra_1 cup draw stadium");
        let pair = if exe::langlib_get_language() == 0 || exe::langlib_get_language() == 1 { 0x98f6ac } else { 0x9a3d8c };
        let st = read_usize(G_STADIUM_TABLE).wrapping_add((stadium as usize).wrapping_mul(stadium::SIZE));
        let city = read_usize(st + stadium::CITY);
        let mut place = [0u8; 0x138];
        let pp = place.as_mut_ptr() as usize;
        if city != 0 {
            if !news_streq(st + stadium::NAME, city + 4) && !news_streq(st + stadium::NAME, 0xa67950) {
                sprintf(pp, pair, st + stadium::NAME, city + 4); // "%s (%s)" or "%s, %s"
            } else {
                news_strcpy(pp, city + 4); // the same name, or "Unknown"
            }
        } else {
            news_strcpy(pp, st + stadium::NAME);
        }
        let g = news_g(st + stadium::GENDER_NAME);
        f(NEWS_TEXT_BUFFER, if two_legs { 0xa678cc } else { 0xa67854 }, g, g, 0x64, pp, sp);
    } else {
        f(NEWS_TEXT_BUFFER, if two_legs { 0xa677f8 } else { 0xa677a8 }, 0x64, sp);
    }
    exe::text_append_string(out, NEWS_TEXT_BUFFER);
    1
}

/// Type 21, the group draw. Slots: +5 the competition, +9 the stage (word), +0xd the club, +0x11 and +0x15 the date
/// of the first group game (words), +0x19 the number of the other clubs in the group (byte), +0x1d on their ids (one
/// dword each).
///
/// Body: more than 8 clubs gives the news.cpp error box (line 0x942) and 8. The list of the clubs' short names "A and
/// B", "A, B and C"... up to 8 (0xa673cc, 0xa67360, 0xa672d4, 0xa6721c, 0xa67148, 0xa67054, 0xa66f40; the genders in
/// pairs, then the names); the stage's name (`exe::comp_util_get_stage_name`, 0x64, the stage, 0); 0xa66e6c with the
/// club, the stage, the competition and the list, and its text; with the date's second word set, 0xa66e20 with the
/// date's long text appended. Links: the club (0x7d5), the competition (0x7d0), the stage's name (0x7e5, to ((stage &
/// 0x7ff) | competition << 11) << 11) and the listed clubs (0x7d5). Headline: by the stage's parent
/// (`exe::comp_util_get_parent_stage_id`): 0x41c 0xa66de0, 10 0xa66d94, 0x14 0xa66d48, 0x82 0xa66cfc, 0x449 0xa66cb4,
/// 0x44a 0xa66c6c, others no text; with the competition's short name, and its link.
///
/// Defect of the original: with fewer than 2 other clubs the list buffer is never written (stack leftovers go into
/// the text); the copy uses an empty list.
/// Original: part of `NewsItem::write_text` (0x77f3ed..0x77fd91).
unsafe fn news_group_draw(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "group_draw";
    let f = news_format();
    let ci = read_u32(news + 5);
    let comp = news_comp(ci);
    let stage = read_u16(news + 9);
    if body == 0 {
        let fmt = match exe::comp_util_get_parent_stage_id(stage as i16 as i32) {
            0x41c => 0xa66de0,
            0xa => 0xa66d94,
            0x14 => 0xa66d48,
            0x82 => 0xa66cfc,
            0x449 => 0xa66cb4,
            0x44a => 0xa66c6c,
            _ => 0,
        };
        if fmt != 0 {
            let g = news_g(comp + competition::SHORT_GENDER_NAME);
            f(NEWS_TEXT_BUFFER, fmt, g, g, comp + competition::SHORT_NAME);
        }
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 group draw body");
    let c = news_club(read_u32(news + 0xd));
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    news_date(dp, read_u16(news + 0x11), read_u16(news + 0x15));
    let mut n = read_u8(news + 0x19) as i8;
    if n > 8 {
        news_error_box_keep(0x942);
        n = 8;
        write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
    }
    let mut ids = [0u32; 10];
    let mut k = 0i8;
    while k < n && k < 10 {
        ids[k as usize] = read_u32(news + 0x1d + 4 * k as usize);
        k += 1;
    }
    let mut list = [0u8; 0x190];
    let lp = list.as_mut_ptr() as usize;
    let cl: [usize; 8] = core::array::from_fn(|i| news_club(ids[i]));
    let sg = |i: usize| news_g(cl[i] + club::SHORT_GENDER_NAME);
    let sn = |i: usize| cl[i] + club::SHORT_NAME;
    match n {
        2 => {
            f(lp, 0xa673cc, sg(0), sg(0), sg(1), sg(1), sn(0), sn(1));
        }
        3 => {
            f(lp, 0xa67360, sg(0), sg(0), sg(1), sg(1), sg(2), sg(2), sn(0), sn(1), sn(2));
        }
        4 => {
            f(lp, 0xa672d4, sg(0), sg(0), sg(1), sg(1), sg(2), sg(2), sg(3), sg(3), sn(0), sn(1), sn(2), sn(3));
        }
        5 => {
            f(lp, 0xa6721c, sg(0), sg(0), sg(1), sg(1), sg(2), sg(2), sg(3), sg(3), sg(4), sg(4), sn(0), sn(1), sn(2), sn(3), sn(4));
        }
        6 => {
            f(lp, 0xa67148, sg(0), sg(0), sg(1), sg(1), sg(2), sg(2), sg(3), sg(3), sg(4), sg(4), sg(5), sg(5), sn(0), sn(1), sn(2), sn(3),
              sn(4), sn(5));
        }
        7 => {
            f(lp, 0xa67054, sg(0), sg(0), sg(1), sg(1), sg(2), sg(2), sg(3), sg(3), sg(4), sg(4), sg(5), sg(5), sg(6), sg(6), sn(0), sn(1),
              sn(2), sn(3), sn(4), sn(5), sn(6));
        }
        8 => {
            f(lp, 0xa66f40, sg(0), sg(0), sg(1), sg(1), sg(2), sg(2), sg(3), sg(3), sg(4), sg(4), sg(5), sg(5), sg(6), sg(6), sg(7), sg(7),
              sn(0), sn(1), sn(2), sn(3), sn(4), sn(5), sn(6), sn(7));
        }
        _ => {}
    }
    let mut stage_name = [0u8; 0xc8];
    let sp = stage_name.as_mut_ptr() as usize;
    exe::comp_util_get_stage_name(sp as *mut u8, 0x64, stage as u32, 0);
    let (gc, gk) = (news_g(c + club::SHORT_GENDER_NAME), news_g(comp + competition::SHORT_GENDER_NAME));
    f(NEWS_TEXT_BUFFER, 0xa66e6c, gc, gc, 0x64, gk, gk, c + club::SHORT_NAME, sp, comp + competition::SHORT_NAME, lp);
    news_set(out, NEWS_TEXT_BUFFER);
    if read_u16(dp + 2) != 0 {
        covered!("gra_1 group draw start date");
        let mut s = [0u8; 0xc8];
        exe::date_get_long_date_string(dp, s.as_mut_ptr() as usize, 0x64, 0xff);
        f(NEWS_TEXT_BUFFER, 0xa66e20, 0x64, s.as_ptr() as usize);
        exe::text_append_string(out, NEWS_TEXT_BUFFER);
    }
    news_link(out, c + club::SHORT_NAME, 0x7d5, c);
    news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
    news_link(out, sp, 0x7e5, ((((stage as u32) & 0x7ff) | (ci << 11)) << 11) as usize);
    for i in 0..n.max(0) as usize {
        let x = news_club(ids[i]);
        news_link(out, x + club::SHORT_NAME, 0x7d5, x);
    }
    1
}

/// The news.cpp error box without releasing the mouse button (the caller does it after its own fix-up, as the
/// original's inline box of type 21).
unsafe fn news_error_box_keep(line: u32) {
    let (mut name, mut ext) = ([0u8; 0x100], [0u8; 0x100]);
    exe::crt_strtok(0xa693ec, 0, 0, name.as_mut_ptr(), ext.as_mut_ptr());
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), line);
    news_sprintf()(0xae24d0, 0x9870e8, v, name.as_ptr(), ext.as_ptr(), line); // "v%s %s.%s %d"
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0); // "Error"
}

/// Type 22, the invitation to apply for the Intertoto Cup. Slots: +5 nonzero when answered, +9 the club, +0x11
/// nonzero when it applied.
///
/// Body 0xa66bf4 with the club's short name and the Intertoto Cup's name (`KEY_CLUB_COMP_INTER_TOTO_CUP`); when
/// answered, "The application has been made." (0xa66bd0) or "The invitation has been declined." (0xa66bac) appended;
/// links: the club's short name (0x7d5) and the cup's name (0x7d0). Headline 0xa66b80 with the cup's short name, and
/// its link.
/// Original: part of `NewsItem::write_text` (0x77fd9a..0x77feb5).
unsafe fn news_intertoto_invitation(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "intertoto_invitation";
    let f = news_format();
    let comp = news_comp(read_u32(KEY_CLUB_COMP_INTER_TOTO_CUP));
    let c = news_club(read_u32(news + 9));
    if body == 0 {
        let g = news_g(comp + competition::SHORT_GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa66b80, g, g, comp + competition::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 intertoto invitation body");
    let (gc, gk) = (news_g(c + club::SHORT_GENDER_NAME), news_g(comp + competition::GENDER_NAME));
    f(NEWS_TEXT_BUFFER, 0xa66bf4, gc, gc, gk, gk, c + club::SHORT_NAME, comp + competition::NAME);
    news_set(out, NEWS_TEXT_BUFFER);
    if read_u32(news + 5) != 0 {
        f(NEWS_TEXT_BUFFER, if read_u32(news + 0x11) != 0 { 0xa66bd0 } else { 0xa66bac });
        exe::text_append_string(out, NEWS_TEXT_BUFFER);
    }
    news_link(out, c + club::SHORT_NAME, 0x7d5, c);
    news_link(out, comp + competition::NAME, 0x7d0, comp);
    1
}

/// Type 23, the club chosen for the Intertoto Cup. Slot: +5 the club.
///
/// Body 0xa66af0 with the club's short name and the Intertoto Cup's name; headline 0xa66aa8 with both short names.
/// Links: the club's short name (0x7d5) and the cup's name or short name (0x7d0).
/// Original: part of `NewsItem::write_text` (0x77febe..0x77ffb6).
unsafe fn news_intertoto_chosen(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "intertoto_chosen";
    let f = news_format();
    let comp = news_comp(read_u32(KEY_CLUB_COMP_INTER_TOTO_CUP));
    let c = news_club(read_u32(news + 5));
    let gc = news_g(c + club::SHORT_GENDER_NAME);
    let name = if body != 0 {
        let gk = news_g(comp + competition::GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa66af0, gc, gc, gk, gk, c + club::SHORT_NAME, comp + competition::NAME);
        comp + competition::NAME
    } else {
        let gk = news_g(comp + competition::SHORT_GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa66aa8, gc, gc, gk, gk, c + club::SHORT_NAME, comp + competition::SHORT_NAME);
        comp + competition::SHORT_NAME
    };
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c + club::SHORT_NAME, 0x7d5, c);
    news_link(out, name, 0x7d0, comp);
    1
}

/// The club asked to skip the FA Cup for the World Club Championship (the key club index at 0x9d0240).
const NEWS_KEY_CLUB_WORLD_CLUB_BYE: usize = 0x9d0240;

/// Type 40, the FA's offer of a bye to the club playing the World Club Championship. Slot: +5 the answer (2
/// accepted, 3 rejected).
///
/// Body 0xa669ac with the club's (0x9d0240), the FA Cup's (`KEY_CLUB_COMP_ENGLISH_FA_CUP`) and the World Club
/// Championship's (`KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP`) short names; accepted 0xa66984 or rejected 0xa66934
/// appended; links: the club (0x7d5) and the two competitions (0x7d0). Headline 0xa668f4 with the club's short name, and
/// its link.
/// Original: part of `NewsItem::write_text` (0x780039..0x7801e9).
unsafe fn news_world_club_bye(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "world_club_bye";
    let f = news_format();
    let c = news_club(read_u32(NEWS_KEY_CLUB_WORLD_CLUB_BYE));
    let gc = news_g(c + club::SHORT_GENDER_NAME);
    if body == 0 {
        f(NEWS_TEXT_BUFFER, 0xa668f4, gc, gc, c + club::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c + club::SHORT_NAME, 0x7d5, c);
        return 1;
    }
    covered!("gra_1 world club bye body");
    let fa = news_comp(read_u32(KEY_CLUB_COMP_ENGLISH_FA_CUP));
    let wcc = news_comp(read_u32(KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP));
    let (gf, gw) = (news_g(fa + competition::SHORT_GENDER_NAME), news_g(wcc + competition::SHORT_GENDER_NAME));
    f(NEWS_TEXT_BUFFER, 0xa669ac, gc, gc, gf, gf, gw, gw, c + club::SHORT_NAME, fa + competition::SHORT_NAME, wcc + competition::SHORT_NAME);
    news_set(out, NEWS_TEXT_BUFFER);
    if read_u32(news + 5) == 2 {
        f(NEWS_TEXT_BUFFER, 0xa66984);
        exe::text_append_string(out, NEWS_TEXT_BUFFER);
    }
    if read_u32(news + 5) == 3 {
        f(NEWS_TEXT_BUFFER, 0xa66934);
        exe::text_append_string(out, NEWS_TEXT_BUFFER);
    }
    news_link(out, c + club::SHORT_NAME, 0x7d5, c);
    news_link(out, fa + competition::SHORT_NAME, 0x7d0, fa);
    news_link(out, wcc + competition::SHORT_NAME, 0x7d0, wcc);
    1
}

/// Type 38, the cup draw with ties still to be played. Slots: +5 the competition, +9 and +0xd the stage and round
/// (words), +0x11 and +0x15 the club (or the two clubs of the tie) on one side, +0x19 and +0x1d those of the other
/// side (+0x15 and +0x1d −1 when the side is a single club).
///
/// Body: the round's label (form 2) and its gender; "A or B will play either C or D" 0xa676a0, "A or B will play C"
/// 0xa675f8, or (+0x15 = −1) "A will play either C or D" 0xa6754c, with the clubs' short names and genders and the
/// label; links: the clubs +0x11 and +0x19, +0x15 and +0x1d when set (0x7d5), the competition on the label (0x7d0).
/// Headline 0xa6776c with the round's label (form 3), and its link.
/// Original: part of `NewsItem::write_text` (0x77ed39..0x77f20c).
unsafe fn news_cup_draw_open(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "cup_draw_open";
    let f = news_format();
    let comp = news_comp(read_u32(news + 5));
    let (stage, round) = (read_u16(news + 9), read_u16(news + 0xd));
    let mut label = [0u8; 0xc8];
    let lb = label.as_ptr() as usize;
    if body == 0 {
        let lg = news_round_label(comp, stage, round, 3, &mut label);
        f(NEWS_TEXT_BUFFER, 0xa6776c, lg, lg, lb);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, lb, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 cup draw open body");
    let (a, b, c, d) = (read_u32(news + 0x11), read_i32(news + 0x15), read_u32(news + 0x19), read_i32(news + 0x1d));
    let lg = news_round_label(comp, stage, round, 2, &mut label);
    let sg = |x: usize| news_g(x + club::SHORT_GENDER_NAME);
    let (ca, cc) = (news_club(a), news_club(c));
    if b != -1 {
        let cb = news_club(b as u32);
        if d != -1 {
            let cd = news_club(d as u32);
            f(NEWS_TEXT_BUFFER, 0xa676a0, sg(ca), sg(ca), sg(cb), sg(cb), sg(cc), sg(cc), sg(cd), sg(cd), lg, lg, ca + club::SHORT_NAME,
              cb + club::SHORT_NAME, cc + club::SHORT_NAME, cd + club::SHORT_NAME, lb);
        } else {
            f(NEWS_TEXT_BUFFER, 0xa675f8, sg(ca), sg(ca), sg(cb), sg(cb), sg(cc), sg(cc), lg, lg, ca + club::SHORT_NAME, cb + club::SHORT_NAME,
              cc + club::SHORT_NAME, lb);
        }
    } else {
        let cd = news_club(d as u32);
        f(NEWS_TEXT_BUFFER, 0xa6754c, sg(ca), sg(ca), sg(cc), sg(cc), sg(cd), sg(cd), lg, lg, ca + club::SHORT_NAME, cc + club::SHORT_NAME,
          cd + club::SHORT_NAME, lb);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, ca + club::SHORT_NAME, 0x7d5, ca);
    news_link(out, cc + club::SHORT_NAME, 0x7d5, cc);
    if b != -1 {
        let cb = news_club(b as u32);
        news_link(out, cb + club::SHORT_NAME, 0x7d5, cb);
    }
    if d != -1 {
        let cd = news_club(d as u32);
        news_link(out, cd + club::SHORT_NAME, 0x7d5, cd);
    }
    news_link(out, lb, 0x7d0, comp);
    1
}

/// Type 39, the cup draw made today. Slots: +5 the competition, +9 and +0xd the stage and round (words), +0x11 the
/// club, +0x21 nonzero for the general story.
///
/// Body: the round's label (form 1, without the lower-casing of the other types) and its gender; the general story
/// 0xa674f4 with the label, or the club's 0xa67458 with its short name, and the club's link (0x7d5) in that case; the
/// competition's link on the label (0x7d0). Headline 0xa67414 with the label (form 2), and its link.
/// Original: part of `NewsItem::write_text` (0x77f215..0x77f3e4).
unsafe fn news_cup_draw_today(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "cup_draw_today";
    let f = news_format();
    let comp = news_comp(read_u32(news + 5));
    let (stage, round) = (read_u16(news + 9) as u32, read_u16(news + 0xd) as u32);
    let c = news_club(read_u32(news + 0x11));
    let general = read_u32(news + 0x21) != 0;
    let mut label = [0u8; 0xc8];
    let lb = label.as_ptr() as usize;
    let form = if body != 0 { 1 } else { 2 };
    exe::comp_util_format_comp_stage_round_label(comp, stage, round, form, label.as_mut_ptr(), 0xc8);
    let lg = exe::comp_util_get_label_gender(comp as u32, stage, round, form) as i8 as i32;
    if body == 0 {
        f(NEWS_TEXT_BUFFER, 0xa67414, lg, lg, lb);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, lb, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 cup draw today body");
    if general {
        f(NEWS_TEXT_BUFFER, 0xa674f4, lg, lg, lb);
    } else {
        let g = news_g(c + club::SHORT_GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa67458, g, g, lg, lg, c + club::SHORT_NAME, lb);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    if !general {
        news_link(out, c + club::SHORT_NAME, 0x7d5, c);
    }
    news_link(out, lb, 0x7d0, comp);
    1
}

/// Category 7, type 0x3e9, a competition record. Slot: +5 the competition.
///
/// Body: the competition's object (`G_COMP_OBJECT_TABLE`) and its records object (+8) write it (`comp_records_write
/// _news_text`, 0x4a7f50, thiscall on the records object: the news and `out`); without them the news.cpp error box
/// (line 0xb51) and 0. Headline 0xa668cc "New {competition} record" with the competition's short name, and its link
/// (0x7d0).
/// Original: part of `NewsItem::write_text` (0x78022b..0x7802cb).
unsafe fn news_club_record(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "comp_record";
    let ci = read_u32(news + 5);
    if body != 0 {
        let obj = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((ci as usize).wrapping_mul(4)));
        let records = if obj != 0 { read_usize(obj + 8) } else { 0 };
        if records == 0 {
            news_error_box(0xb51);
            return 0;
        }
        covered!("gra_1 comp record body");
        let w: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x4a7f50usize);
        w(records, news, out);
        return 1;
    }
    let comp = news_comp(ci);
    let g = news_g(comp + competition::SHORT_GENDER_NAME);
    news_format()(NEWS_TEXT_BUFFER, 0xa668cc, g, g, comp + competition::SHORT_NAME);
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
    1
}

/// The name of an injury type into `buf` (`injury_get_type_name`, 0x62ef30, stdcall; the original loads `G_INJURY`
/// into ecx).
unsafe fn news_injury_name(kind: u8, buf: usize) {
    let w: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(0x62ef30usize);
    w(read_usize(G_INJURY), kind as u32, buf, 0x32, 1);
}
/// The grammatical gender of an injury type's name (`injury_get_type_name_gender`, 0x6306e0).
unsafe fn news_injury_gender(kind: u8) -> i32 {
    let w: unsafe extern "thiscall" fn(usize, u32) -> i8 = core::mem::transmute(0x6306e0usize);
    w(read_usize(G_INJURY), kind as u32) as i32
}
/// The weekday's name of a date (`Date::get_weekday`, `date_get_weekday_name` 0x54d170).
unsafe fn news_weekday_name(date: usize) -> usize {
    let w: unsafe extern "C" fn(i32) -> usize = core::mem::transmute(0x54d170usize);
    w(exe::date_get_weekday(date))
}
/// The weekday's gender of a date in the current language (`exe::date_get_weekday_gender`).
unsafe fn news_weekday_gender(date: usize) -> i32 {
    let wd = exe::date_get_weekday(date);
    exe::date_get_weekday_gender(exe::langlib_get_language() as u32, wd as u32) as i32
}

/// Category 3, type 0x7d0, the injury or illness news. Slots: +5 the player, +9..+0x11 his name ids, +0x15 the injury
/// type, +0x19/+0x1d the day it happened and +0x25/+0x29 the date compared with the name ids (words), +0x21 the days
/// out, +0x2d the club he was injured against (−1 in training), +0x31 the other club of that match, +0x35 the
/// competition, +0x39 the nation where an illness was caught, +0x3d the player's club (−1 none), +0x41 0x420e9 when
/// the match record that follows (from slot 0x10, `exe::read_match_record_from_news`) is the match of the injury.
///
/// Headline (the short name, style 5): by the type's category (`exe::injury_get_type_category`): treatments (12) by
/// the type (0x4d..0x51: faith healing, physiotherapy, surgery, radiotherapy, leave of absence; others surgery);
/// illnesses (0) "struck down with" the illness's name; injuries over 100 days "injury shocker" / "Injury blow" / "out
/// for" the duration (`format_duration_text`) by (type + player) % 3, the others "injured" / "suffers injury" / "out
/// for"; the player's link (0x7d8) when his name ids still match.
///
/// Body (the full name, style 1; the injury's name and the duration first): an illness 0xa6655c (caught at the club in
/// the nation) or 0xa664e4, with the links of the player (when the ids match), the nation (to its national team) and the
/// club. An injury in a match: in Swedish, for types 0x3b and 0x47, a text of its own (`sprintf` 0xa6648c); with the
/// match record, by the score (a win 0xa661e4, a draw 0xa66084, a defeat 0xa65f24: the round's label in form 1 and
/// "%d-%d" of the higher and lower goals); without it 0xa6633c; the competition's gender is 0x64 for the friendly
/// competition (`KEY_CLUB_COMP_FRIENDLY`). Links: the competition (when not a reserve or minor one,
/// `exe::comp_is_reserve_or_minor_comp`), the player (always), the two clubs, and with the record the match on the score
/// (0x7d3, when `exe::match_stats_fixture_entry_matches`) and the record's away club's name. An injury in training: in
/// Swedish, for types 0x3b and 0x47, 0xa65edc or 0xa65e94 by the weekday name's last letter; for another club's player
/// (not the human manager's club, `G_SCREEN_RECORDS_MANAGER`), treatments with the club's short name (a recurring
/// injury, `Injury::get_recurring_injury_type`, names the illness treated: 0xa65908, 0xa659d4, 0xa65aa4, 0xa65b40;
/// otherwise 0xa65d58, 0xa65ca8, 0xa65c10, 0xa65aa4, 0xa65e04) and injuries 0xa65814; for the human manager's club or no
/// club, treatments 0xa656f4, 0xa65664, 0xa65574, 0xa655ec, 0xa65780 and injuries 0xa654a4; links: the player and his
/// club.
/// Original: part of `NewsItem::write_text` (0x7802e7..0x78164b).
unsafe fn news_match_report(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "injury";
    let f = news_format();
    let sprintf = news_sprintf();
    let mut rec = [0u8; 0x20];
    let rp = rec.as_mut_ptr() as usize;
    exe::date_date_54a110(rp);
    let player = read_u32(news + 5);
    let ids = [read_u32(news + 9), read_u32(news + 0xd), read_u32(news + 0x11)];
    let kind = read_u8(news + 0x15);
    let (mut d1, mut d2) = ([0u8; 8], [0u8; 8]);
    let (d1p, d2p) = (d1.as_mut_ptr() as usize, d2.as_mut_ptr() as usize);
    news_date(d1p, read_u16(news + 0x19), read_u16(news + 0x1d));
    let days = read_i32(news + 0x21);
    news_date(d2p, read_u16(news + 0x25), read_u16(news + 0x29));
    let against = read_i32(news + 0x2d);
    let other = read_u32(news + 0x31);
    let comp_i = read_u32(news + 0x35);
    let nation_i = read_i32(news + 0x39);
    let club_i = read_i32(news + 0x3d);
    let in_match = read_u32(news + 0x41) != 0x420e9;
    let mut cursor: u8 = 0x10;
    exe::read_match_record_from_news(news, &mut cursor, rec.as_mut_ptr());
    let mut label = [0u8; 0xc8];
    label[0] = read_u8(0xad9c64);
    let (ga, gb) = (rec[0x19] as i8, rec[0x1a] as i8);
    let mut score = [0u8; 0xc];
    let scp = score.as_mut_ptr() as usize;
    sprintf(scp, 0x98c114, ga.max(gb) as i32, ga.min(gb) as i32); // "%d-%d"
    let human = read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300)));
    let job = if human != 0 { read_usize(human + 0x39) } else { 0 };
    let mine = if job != 0 { club_i == -1 || club_i == read_i32(job) } else { club_i == -1 };
    let mut name = [0u8; 0x12c];
    let np = name.as_mut_ptr() as usize;
    let staff = news_staff(player);
    let (mut ill, mut dur) = ([0u8; 0xc8], [0u8; 0xc8]);
    let (ip, dp) = (ill.as_mut_ptr() as usize, dur.as_mut_ptr() as usize);
    let duration: unsafe extern "C" fn(i32, usize, u32, u32) -> i32 = core::mem::transmute(0x7e5780usize);
    let category = || exe::injury_get_type_category(read_usize(G_INJURY), kind as i8);
    if body == 0 {
        exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x69, 5, player);
        let cat = category();
        if cat == 0xc {
            let fmt = match (kind as i8 as i32).wrapping_sub(0x4d) {
                0 => 0xa6684c, // faith healing
                1 => 0xa66810, // physiotherapy
                3 => 0xa667c8, // radiotherapy
                4 => 0xa66894, // leave of absence
                _ => 0xa66790, // surgery
            };
            f(NEWS_TEXT_BUFFER, fmt, np);
        } else if cat == 0 {
            news_injury_name(kind, ip);
            f(NEWS_TEXT_BUFFER, 0xa66740, np, ip);
        } else {
            let r = (kind as i8 as i32).wrapping_add(player as i32).wrapping_rem(3);
            let fmt = match (days > 100, r) {
                (true, 0) => 0xa6668c,
                (true, 1) => 0xa666c0,
                (false, 0) => 0xa6662c,
                (false, 1) => 0xa66658,
                _ => 0,
            };
            if fmt != 0 {
                f(NEWS_TEXT_BUFFER, fmt, np);
            } else {
                duration(days, dp, 0x32, 1);
                f(NEWS_TEXT_BUFFER, 0xa666f4, np, dp);
            }
        }
        news_set(out, NEWS_TEXT_BUFFER);
        if exe::database_staff_matches_names(staff, ids[0], ids[1], ids[2], d2p) != 0 {
            news_link(out, np, 0x7d8, staff);
        }
        return 1;
    }
    covered!("gra_1 injury body");
    exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x69, 1, player);
    news_injury_name(kind, ip);
    duration(days, dp, 0x32, 1);
    let cat = category();
    if cat == 0 {
        covered!("gra_1 injury illness");
        let a = news_injury_gender(kind);
        let b = news_injury_gender(kind);
        if nation_i != -1 && club_i != -1 {
            let (nat, cl) = (news_nation(nation_i as u32), news_club(club_i as u32));
            let (gc, gn) = (news_g(cl + club::SHORT_GENDER_NAME), news_g(nat + 0x52));
            f(NEWS_TEXT_BUFFER, 0xa6655c, b, a, gc, gc, gn, gn, np, dp, ip, cl + club::SHORT_NAME, nat + 0x38);
        } else {
            f(NEWS_TEXT_BUFFER, 0xa664e4, b, a, np, dp, ip);
        }
        news_set(out, NEWS_TEXT_BUFFER);
        if exe::database_staff_matches_names(staff, ids[0], ids[1], ids[2], d2p) != 0 {
            news_link(out, np, 0x7d8, staff);
        }
        if nation_i != -1 {
            let nat = news_nation(nation_i as u32);
            news_link(out, nat + 0x38, 0x7d5, exe::database_get_national_team(nat));
        }
        if club_i != -1 {
            let cl = news_club(club_i as u32);
            news_link(out, cl + club::SHORT_NAME, 0x7d5, cl);
        }
        return 1;
    }
    if against == -1 {
        return news_injury_in_training(out, kind, np, dp, ip, d1p, staff, club_i, mine);
    }
    covered!("gra_1 injury in a match");
    let (cs, co, comp) = (news_club(against as u32), news_club(other), news_comp(comp_i));
    if exe::langlib_get_language() == 0xa && (kind == 0x3b || kind == 0x47) {
        let wd = news_weekday_name(d1p);
        sprintf(NEWS_TEXT_BUFFER, 0xa6648c, np, dp, ip, comp + competition::NAME, cs + club::SHORT_NAME, co + club::SHORT_NAME, wd);
    } else {
        let gk = if comp_i == read_u32(KEY_CLUB_COMP_FRIENDLY) { 0x64 } else { news_g(comp + competition::GENDER_NAME) as i8 as i32 };
        let (gcs, gco) = (news_g(cs + club::SHORT_GENDER_NAME), news_g(co + club::SHORT_GENDER_NAME));
        if in_match {
            let wd = news_weekday_name(d1p);
            let gwd = news_weekday_gender(d1p);
            let a = news_injury_gender(kind);
            let b = news_injury_gender(kind);
            f(NEWS_TEXT_BUFFER, 0xa6633c, b, a, gk, gk, gcs, gcs, gco, gco, gwd, 0x64, np, dp, ip, comp + competition::NAME,
              cs + club::SHORT_NAME, co + club::SHORT_NAME, wd);
        } else {
            covered!("gra_1 injury with the match record");
            let rc = news_comp(read_u32(rp + 8));
            let lg = news_round_label(rc, read_u16(rp + 0xc), read_u16(rp + 0xe), 1, &mut label);
            let lb = label.as_ptr() as usize;
            let wd = news_weekday_name(d1p);
            let gwd = news_weekday_gender(d1p);
            let a = news_injury_gender(kind);
            let b = news_injury_gender(kind);
            let (csn, con) = (cs + club::SHORT_NAME, co + club::SHORT_NAME);
            if ga > gb {
                f(NEWS_TEXT_BUFFER, 0xa661e4, b, a, gcs, gcs, lg, lg, gco, gco, gwd, 0x64, np, dp, ip, csn, scp, lb, con, wd);
            } else if ga == gb {
                f(NEWS_TEXT_BUFFER, 0xa66084, 0x64, 0x64, b, a, gcs, gcs, lg, lg, gco, gco, gwd, 0x64, np, dp, ip, csn, scp, lb, con, wd);
            } else {
                f(NEWS_TEXT_BUFFER, 0xa65f24, b, a, gcs, gcs, lg, lg, gco, gco, gwd, 0x64, np, dp, ip, csn, scp, lb, con, wd);
            }
        }
    }
    news_set(out, NEWS_TEXT_BUFFER);
    if exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), comp) == 0 {
        news_link(out, comp + competition::NAME, 0x7d0, comp);
    }
    news_link(out, np, 0x7d8, staff);
    news_link(out, cs + club::SHORT_NAME, 0x7d5, cs);
    news_link(out, co + club::SHORT_NAME, 0x7d5, co);
    if rec[0x19] != 0xff && !in_match {
        let fixture = read_u32(rp + 0x1b);
        if exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), fixture as i32, read_i32(rp + 0x11), read_i32(rp + 0x15),
            read_u16(rp) as i32, read_u16(rp + 2) as i32) != 0
        {
            news_link(out, scp, 0x7d3, fixture as usize);
        }
        let away = news_club(read_u32(rp + 0x15));
        news_link(out, away + club::NAME, 0x7d5, away);
    }
    1
}

/// The body of an injury in training (see `news_match_report`): the text, the player's link (0x7d8, always) and his
/// club's (0x7d5, when he has one).
/// Original: part of `NewsItem::write_text` (0x780fe1..0x78164b).
unsafe fn news_injury_in_training(out: usize, kind: u8, np: usize, dp: usize, ip: usize, d1p: usize, staff: usize, club_i: i32, mine: bool) -> u32 {
    covered!("gra_1 injury in training");
    let f = news_format();
    let sprintf = news_sprintf();
    let category = || exe::injury_get_type_category(read_usize(G_INJURY), kind as i8);
    if exe::langlib_get_language() == 0xa && (kind == 0x3b || kind == 0x47) {
        let mut day = [0u8; 0x40];
        news_strcpy(day.as_mut_ptr() as usize, news_weekday_name(d1p));
        if day[0] != 0 {
            let len = day.iter().position(|&c| c == 0).unwrap_or(0);
            let k = (day[len - 1] as i8 as i32).wrapping_sub(0x43);
            // the Swedish genitive: no extra "s" after C, S, X or Z
            let plain = (0..=0x37).contains(&k) && read_u8(0x782b04 + k as usize) == 0;
            sprintf(NEWS_TEXT_BUFFER, if plain { 0xa65edc } else { 0xa65e94 }, np, dp, ip, day.as_ptr() as usize);
        }
    } else if !mine && club_i != -1 {
        let cl = news_club(club_i as u32);
        let gcl = news_g(cl + club::SHORT_GENDER_NAME);
        let csn = cl + club::SHORT_NAME;
        if category() == 0xc {
            let recur: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62ed30usize);
            let r = recur(read_usize(G_INJURY), staff);
            if r == 0xff {
                match (kind as i8 as i32).wrapping_sub(0x4d) {
                    0 => { f(NEWS_TEXT_BUFFER, 0xa65d58, gcl, gcl, csn, np, dp); }
                    1 => { f(NEWS_TEXT_BUFFER, 0xa65ca8, gcl, gcl, csn, np, dp); }
                    3 => { f(NEWS_TEXT_BUFFER, 0xa65aa4, gcl, gcl, csn, np); }
                    4 => { f(NEWS_TEXT_BUFFER, 0xa65e04, np); }
                    _ => { f(NEWS_TEXT_BUFFER, 0xa65c10, gcl, gcl, csn, np, dp); }
                }
            } else {
                covered!("gra_1 injury recurring");
                news_injury_name(r, ip);
                match (kind as i8 as i32).wrapping_sub(0x4d) {
                    0 => {
                        let a = news_injury_gender(r);
                        let b = news_injury_gender(r);
                        f(NEWS_TEXT_BUFFER, 0xa65908, gcl, gcl, b, a, csn, np, dp, ip);
                    }
                    1 => {
                        let a = news_injury_gender(r);
                        let b = news_injury_gender(r);
                        f(NEWS_TEXT_BUFFER, 0xa659d4, gcl, gcl, b, a, csn, np, dp, ip);
                    }
                    3 => { f(NEWS_TEXT_BUFFER, 0xa65aa4, gcl, gcl, csn, np); }
                    _ => {
                        let a = news_injury_gender(r);
                        let b = news_injury_gender(r);
                        f(NEWS_TEXT_BUFFER, 0xa65b40, gcl, gcl, 0x64, b, a, csn, np, dp, ip);
                    }
                }
            }
        } else {
            let wd = news_weekday_name(d1p);
            let gwd = news_weekday_gender(d1p);
            let a = news_injury_gender(kind);
            let b = news_injury_gender(kind);
            f(NEWS_TEXT_BUFFER, 0xa65814, gcl, gcl, 0x64, 0x64, 0x64, b, a, gwd, 0x64, csn, np, dp, ip, wd);
        }
    } else if category() == 0xc {
        match (kind as i8 as i32).wrapping_sub(0x4d) {
            0 => { f(NEWS_TEXT_BUFFER, 0xa656f4, np, dp); }
            1 => { f(NEWS_TEXT_BUFFER, 0xa65664, np, dp); }
            3 => { f(NEWS_TEXT_BUFFER, 0xa655ec, np); }
            4 => { f(NEWS_TEXT_BUFFER, 0xa65780, np); }
            _ => { f(NEWS_TEXT_BUFFER, 0xa65574, np, dp); }
        }
    } else {
        let wd = news_weekday_name(d1p);
        let gwd = news_weekday_gender(d1p);
        let a = news_injury_gender(kind);
        let b = news_injury_gender(kind);
        f(NEWS_TEXT_BUFFER, 0xa654a4, b, a, gwd, 0x64, np, dp, ip, wd);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, np, 0x7d8, staff);
    if club_i > -1 {
        let cl = news_club(club_i as u32);
        news_link(out, cl + club::SHORT_NAME, 0x7d5, cl);
    }
    1
}

/// Category 1, type 0x1388, a manager's message. Slots: +5 the sending manager, +9 nonzero for a private message,
/// +0xd the id of the news it answers (0 none), slot 3 its text.
///
/// Headline: "Private message from %s" (0xa65470) or "Public message from %s" (0xa65448) with the manager's name
/// (`exe::database_format_staff_name`, 0xc8, 1). Body: when it answers a news and `flag` is 0, the quoted news is looked
/// up (`News::find_item_by_id` 0x786b70 on the news object 0xb7431c, into a blank news item dated with the game's
/// date 0xae2c90 and day slice 0xae2c88) and, when found, its body is written (this routine, flag 1) into a new text
/// and appended as "> " text "\n>\n"; then the message's text (slot 3, `NewsItem::get_slot_string` 0x785a20, 0x7d0) is
/// appended. The body only appends to `out`.
/// Original: part of `NewsItem::write_text` (0x78284a..0x782a02).
unsafe fn news_follow_up(news: usize, manager: usize, out: usize, body: u32, flag: u32) -> u32 {
    NEWS_LAST_PIECE = "manager_message";
    let sender = read_u32(news + 5);
    let private = read_u32(news + 9) != 0;
    let quoted = read_u32(news + 0xd);
    if body == 0 {
        let mut name = [0u8; 0xc8];
        exe::database_format_staff_name(news_staff(sender), name.as_mut_ptr() as usize, 0xc8, 1);
        news_format()(NEWS_TEXT_BUFFER, if private { 0xa65470 } else { 0xa65448 }, name.as_ptr() as usize);
        news_set(out, NEWS_TEXT_BUFFER);
        return 1;
    }
    covered!("gra_1 manager message body");
    if quoted != 0 && flag == 0 {
        covered!("gra_1 manager message quote");
        let mut item = [0u8; 0xe0];
        let ip = item.as_mut_ptr() as usize;
        exe::date_date_54a110(ip + news_item::DATE);
        let mut tmp = [0u8; 8];
        exe::date_operator_3d(ip + news_item::DATE, tmp.as_mut_ptr() as usize, 0xae2c90usize);
        write_u8(ip + news_item::DAY_SLICE, read_u8(0xae2c88));
        let mut text = [0usize; 2];
        let tp = text.as_mut_ptr() as usize;
        let ctor: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x4ae590usize);
        ctor(tp, 0);
        let find: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(0x786b70usize);
        if find(read_usize(0xb7431c), 0, quoted, ip) != 0 {
            let write: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x77bc80usize);
            write(ip, manager, tp, 1, 1);
            exe::text_append_string(out, 0xa654a0usize); // "> "
            exe::text_append_string(out, text[0]);
            exe::text_append_string(out, 0xa6549cusize); // "\n>\n"
        }
        exe::drop_text(text.as_mut_ptr());
    }
    let get: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(0x785a20usize);
    get(news, 3, NEWS_TEXT_BUFFER, 0x7d0);
    exe::text_append_string(out, NEWS_TEXT_BUFFER);
    1
}

/// Compares `NewsItem::write_text`: the original writes into `out` and the copy into a local text, side by side
/// (`compare_text_writer`: the text, the links and the random generator), and the return values. Without a manager or
/// `out` (the error box) only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_newsitem_write_text(news: usize, manager: usize, out: usize, body: u32, flag: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(IDX_NEWSITEM_WRITE_TEXT);
    if manager == 0 || out == 0 {
        return o(news, manager, out, body, flag);
    }
    let (ro, rr) = (core::cell::Cell::new(0u32), core::cell::Cell::new(0u32));
    let (cat, ty) = (exe::newsitem_get_category(news), read_u32(news));
    compare_text_writer(out, || ro.set(o(news, manager, out, body, flag)), |t| rr.set(newsitem_write_text(news, manager, t, body, flag)),
        || format!("newsitem_write_text(category {}, type {:#x}, body {}, flag {}, piece {})", cat, ty, body, flag, NEWS_LAST_PIECE));
    SCOREBOARD.record(ro.get() == rr.get(), || format!("newsitem_write_text(type {:#x}) returned {} x {}", ty, ro.get(), rr.get()));
    ro.get()
}

// ---------- batch 324: the club records after a match (club_records.cpp) ----------

/// Offsets in a club record (season or all-time, 0x38b bytes) used by `update_records_for_match`, beyond the ones in
/// `club_record`. Each value record is followed by its holder (`record_holder`, 0x18 bytes) and/or its match
/// (`club_match_record`, 0x1f bytes, `club_match_record::FIXTURE_ID` −1 = no record); "−1" marks an empty value.
pub(crate) mod club_match_records {
    pub(crate) const HIGHEST_ATTENDANCE: usize = 0xe; // i32, then the match at +0x12
    pub(crate) const HIGHEST_GATE: usize = 0x31; // i32 (gate receipts), then the match at +0x35
    pub(crate) const LOWEST_ATTENDANCE: usize = 0x54; // i32, then the match at +0x58
    pub(crate) const AVERAGE_ATTENDANCE: usize = 0x77; // i32: the season's home attendance / home games
    pub(crate) const AVERAGE_ATTENDANCE_YEAR: usize = 0x7b; // u16: 0 in the season record [inference]
    pub(crate) const BIGGEST_WIN: usize = 0x7d; // match
    pub(crate) const BIGGEST_LEAGUE_WIN: usize = 0x9c; // match
    pub(crate) const BIGGEST_DEFEAT: usize = 0xbb; // match
    pub(crate) const HIGHEST_SCORING: usize = 0xda; // match
    pub(crate) const HIGHEST_SCORING_LEAGUE: usize = 0xf9; // match
    pub(crate) const WON_IN_ROW: usize = 0x118; // run_record (i32 games, start and end dates)
    pub(crate) const LOST_IN_ROW: usize = 0x12c; // run_record
    pub(crate) const UNBEATEN: usize = 0x140; // run_record
    pub(crate) const WINLESS: usize = 0x154; // run_record
    pub(crate) const TOP_SCORER: usize = 0x168; // i8 goals, holder +0x169, year (u16) +0x181
    pub(crate) const TOP_LEAGUE_SCORER: usize = 0x183; // i8 goals, holder +0x184, year +0x19c
    pub(crate) const GOALS_IN_MATCH: usize = 0x19e; // i8 goals, holder +0x19f, match +0x1b7
    pub(crate) const ASSISTS: usize = 0x1d6; // i8, holder +0x1d7, year +0x1ef
    pub(crate) const MAN_OF_MATCH: usize = 0x210; // i8, holder +0x211, year +0x229
    pub(crate) const DISCIPLINE: usize = 0x22b; // i8 yellow cards, i8 red cards +0x22c, holder +0x22d, year +0x245
    pub(crate) const YOUNGEST: usize = 0x27b; // the holder's birth date, holder +0x283, match +0x29b
    pub(crate) const OLDEST: usize = 0x2ba; // the holder's birth date, holder +0x2c2, match +0x2da
    /// The holder (`record_holder`) after a one-byte value; the year word after the holder.
    pub(crate) const HOLDER: usize = 1;
    pub(crate) const YEAR: usize = 0x19;
    /// After a birth date: the holder and the match.
    pub(crate) const AGE_HOLDER: usize = 8;
    pub(crate) const AGE_MATCH: usize = 0x20;
}

/// Offsets in a club's 0x6a-byte entry (`club_records_fields::CLUB_ENTRIES`) used by `update_records_for_match`.
pub(crate) mod club_entry_runs {
    pub(crate) const HOME_GAMES: usize = 0x11; // i8: home games this season
    pub(crate) const HOME_GAMES_TOTAL: usize = 0x12; // i16: home games counted for the attendance news [inference: never reset]
    pub(crate) const HOME_ATTENDANCE: usize = 0x14; // i32: the season's home attendance
    pub(crate) const WON_RUN: usize = 0x18; // run_record: the current run of wins
    pub(crate) const LOST_RUN: usize = 0x2c; // run_record: of defeats
    pub(crate) const UNBEATEN_RUN: usize = 0x40; // run_record: without losing
    pub(crate) const WINLESS_RUN: usize = 0x54; // run_record: without winning
}

/// The record types (club_records_get_record_type_label 0x4490b0) that `update_records_for_match` updates.
pub(crate) mod club_record_kind {
    pub(crate) const HIGHEST_ATTENDANCE: u8 = 3;
    pub(crate) const HIGHEST_GATE: u8 = 4;
    pub(crate) const LOWEST_ATTENDANCE: u8 = 5;
    pub(crate) const BIGGEST_WIN: u8 = 7;
    pub(crate) const BIGGEST_LEAGUE_WIN: u8 = 8;
    pub(crate) const BIGGEST_DEFEAT: u8 = 9;
    pub(crate) const HIGHEST_SCORING: u8 = 0xa;
    pub(crate) const HIGHEST_SCORING_LEAGUE: u8 = 0xb;
    pub(crate) const WON_IN_ROW: u8 = 0xc;
    pub(crate) const LOST_IN_ROW: u8 = 0xd;
    pub(crate) const UNBEATEN: u8 = 0xe;
    pub(crate) const WINLESS: u8 = 0xf;
    pub(crate) const TOP_SCORER: u8 = 0x10;
    pub(crate) const TOP_LEAGUE_SCORER: u8 = 0x11;
    pub(crate) const GOALS_IN_MATCH: u8 = 0x12;
    pub(crate) const ASSISTS: u8 = 0x13;
    pub(crate) const MAN_OF_MATCH: u8 = 0x15;
    pub(crate) const DISCIPLINE: u8 = 0x16;
    pub(crate) const LEAGUE_APPEARANCES: u8 = 0x17;
    pub(crate) const LEAGUE_GOALS: u8 = 0x18;
    pub(crate) const YOUNGEST: u8 = 0x19;
    pub(crate) const OLDEST: u8 = 0x1a;
    pub(crate) const INTERNATIONAL_CAPS: u8 = 0x1b;
}

/// The comparer's dry run of `update_records_for_match`: `Some((n, queued))` makes the copy look only at the first `n`
/// items of the real news queue and put its own in `queued` instead of the queue.
pub(crate) static MATCH_RECORDS_NEWS_DRY_RUN: std::sync::Mutex<Option<(i32, Vec<(u32, u8)>)>> = std::sync::Mutex::new(None);

/// Queues the record news of `club` for the last of `kinds`, unless a news item of one of `kinds` (looked up in that
/// order, each over the whole queue) is already queued for the club. Returns the kind found, or `None` when it queued.
pub(crate) unsafe fn queue_match_record_news(this: usize, club: Club, kinds: &[u8]) -> Option<u8> {
    let id = read_u32(club.addr());
    let mut dry = MATCH_RECORDS_NEWS_DRY_RUN.lock().unwrap();
    let real = match &*dry {
        Some((n, _)) => *n,
        None => read_i32(this + club_records_fields::NEWS_QUEUE_COUNT),
    };
    let queue = read_usize(this + club_records_fields::NEWS_QUEUE);
    for &kind in kinds {
        let in_queue = (0..real.max(0) as usize).any(|k| read_u32(queue + 6 * k) == id && read_u8(queue + 6 * k + record_news_item::RECORD_TYPE) == kind);
        if in_queue || dry.as_ref().map_or(false, |(_, v)| v.contains(&(id, kind))) {
            return Some(kind);
        }
    }
    let kind = kinds[kinds.len() - 1];
    match &mut *dry {
        Some((_, v)) => v.push((id, kind)),
        None => {
            drop(dry);
            exe::club_records_queue_record_news(this, id, kind as u32);
        }
    }
    None
}

/// The attendance records of a home match (types 3, 4 and 5): the value `value` of the match beats the season record at
/// `at` (`beats(value, record)`, or no record: −1) → the season record takes the value and the match (`match_record_assign`
/// of the club's view of the match); if it also beats the all-time record: the news when the club has played 50 or more
/// home games (`club_entry_runs::HOME_GAMES_TOTAL`), and the all-time record takes the value and the match.
unsafe fn attendance_record(this: usize, club: Club, entry: usize, records: (usize, usize), view: usize, kind: u8, at: usize, value: i32, beats: fn(i32, i32) -> bool) {
    let (season, all_time) = records;
    if exe::club_records_record_type_applies(kind as u32, club, 1) == 0 || read_u8(view + club_match_record::VENUE) != 1 || value <= 0 {
        return;
    }
    let s = read_i32(season + at);
    if s != -1 && !beats(value, s) {
        return;
    }
    write_i32(season + at, value);
    match_record_assign(season + at + 4, view);
    let a = read_i32(all_time + at);
    if a != -1 && !beats(value, a) {
        return;
    }
    covered!("gra_15 attendance record ever");
    if read_i16(entry + club_entry_runs::HOME_GAMES_TOTAL) >= 50 {
        queue_match_record_news(this, club, &[kind]);
    }
    write_i32(all_time + at, value);
    match_record_assign(all_time + at + 4, view);
}

/// A match record (types 7 to 0xb): the club's view of the match beats the season record at `at` (`better(view,
/// record)`, or no record: fixture id −1) → it is copied there; if it also beats the all-time record: with `news`, the
/// news (skipped when one of `kinds` is queued), and it is copied to the all-time record.
unsafe fn result_record(this: usize, club: Club, records: (usize, usize), view: usize, kinds: &[u8], at: usize, better: unsafe extern "C" fn(usize, usize) -> u32, news: bool) {
    let (season, all_time) = records;
    if read_i32(season + at + club_match_record::FIXTURE_ID) != -1 && better(view, season + at) == 0 {
        return;
    }
    match_record_assign(season + at, view);
    if read_i32(all_time + at + club_match_record::FIXTURE_ID) != -1 && better(view, all_time + at) == 0 {
        return;
    }
    if news {
        queue_match_record_news(this, club, kinds);
    }
    match_record_assign(all_time + at, view);
}

/// One more game in the run at `run` of the club's entry: the count goes up (the first game sets the start date) and
/// the end date is today ([`G_CURRENT_DATE`]). If the run beats the season record at `at` (type `kind`, when it
/// applies; −1 = no record), the season record takes the count and both dates; if it also beats the all-time record:
/// the news when the run is at least `news_from` games long and did not start on the all-time record's start date (the
/// same run already recorded), and the all-time record takes the count and the dates.
unsafe fn run_record(this: usize, club: Club, entry: usize, records: (usize, usize), kind: u8, run: usize, at: usize, news_from: i32) {
    let (season, all_time) = records;
    let n = read_i32(entry + run).wrapping_add(1);
    write_i32(entry + run, n);
    if n == 1 {
        copy_record_date(entry + run + run_record::START_DATE, G_CURRENT_DATE);
    }
    copy_record_date(entry + run + run_record::END_DATE, G_CURRENT_DATE);
    if exe::club_records_record_type_applies(kind as u32, club, 1) == 0 {
        return;
    }
    let s = read_i32(season + at);
    if s != -1 && read_i32(entry + run) <= s {
        return;
    }
    write_i32(season + at, read_i32(entry + run));
    copy_record_date(season + at + run_record::START_DATE, entry + run + run_record::START_DATE);
    copy_record_date(season + at + run_record::END_DATE, entry + run + run_record::END_DATE);
    let a = read_i32(all_time + at);
    if a != -1 && read_i32(entry + run) <= a {
        return;
    }
    if read_i32(entry + run) >= news_from {
        let (start, recorded) = (entry + run + run_record::START_DATE, all_time + at + run_record::START_DATE);
        if read_u16(recorded + 2) != read_u16(start + 2) || read_u16(recorded) != read_u16(start) {
            covered!("gra_15 run record ever");
            queue_match_record_news(this, club, &[kind]);
        }
    }
    write_i32(all_time + at, read_i32(entry + run));
    copy_record_date(all_time + at + run_record::START_DATE, entry + run + run_record::START_DATE);
    copy_record_date(all_time + at + run_record::END_DATE, entry + run + run_record::END_DATE);
}

/// The club's records of the match (the part of `update_records_for_match` before the players).
///
/// One more game this season (`club_record_entry::SEASON_GAMES`); the club's view of the match
/// (`exe::fill_match_record_from_fixture` into `view`). At home (`club_match_record::VENUE` 1): one more home game
/// (`club_entry_runs::HOME_GAMES`, `club_entry_runs::HOME_GAMES_TOTAL`), the attendance (`match_record::ATTENDANCE`)
/// added to the season's (`club_entry_runs::HOME_ATTENDANCE`) and the season average
/// (`club_match_records::AVERAGE_ATTENDANCE`) = total / home games, its year word 0. Then, each when its type applies
/// to the club (`exe::club_records_record_type_applies`, period 1), through `attendance_record`, `result_record` and
/// `run_record`:
/// - highest attendance, highest gate receipts (`match_record::GATE_RECEIPTS`) and lowest attendance, at home and with
///   a value above 0;
/// - biggest win (won, `match_record_is_bigger_win`; news by 6 goals or more), biggest league win (the same in a league
///   division, `exe::comp_util_is_league_division`; not again when the biggest win's news is queued), biggest defeat
///   (lost, `match_record_is_bigger_defeat`; 6 or more), highest scoring game (goals, `match_record_has_more_goals`;
///   news from 8 goals) and highest scoring league game;
/// - the runs: a win adds to the run of wins (news from 6) and ends the runs of defeats and without winning; a defeat
///   adds to the defeats (6) and ends the wins and the unbeaten run; a win or a draw adds to the unbeaten run (12) and
///   ends the defeats; a draw or a defeat adds to the run without winning (12) and ends the wins.
/// Goals are the club's view (`club_match_record::GOALS_FOR`/`GOALS_AGAINST`, signed bytes, the extra-time score if
/// any).
unsafe fn update_club_match_records(this: usize, f: Fixture, result: usize, club: Club, entry: usize, records: (usize, usize), view: usize) {
    use club_match_records as r;
    use club_record_kind as k;
    let (season, _) = records;
    write_u8(entry + club_record_entry::SEASON_GAMES, read_u8(entry + club_record_entry::SEASON_GAMES).wrapping_add(1));
    exe::fill_match_record_from_fixture(f, club, view);
    let attendance = read_i32(result + match_record::ATTENDANCE);
    if read_u8(view + club_match_record::VENUE) == 1 {
        covered!("gra_15 home match");
        let home = read_i8(entry + club_entry_runs::HOME_GAMES).wrapping_add(1);
        write_i16(entry + club_entry_runs::HOME_GAMES_TOTAL, read_i16(entry + club_entry_runs::HOME_GAMES_TOTAL).wrapping_add(1));
        write_i8(entry + club_entry_runs::HOME_GAMES, home);
        let total = read_i32(entry + club_entry_runs::HOME_ATTENDANCE).wrapping_add(attendance);
        write_i32(entry + club_entry_runs::HOME_ATTENDANCE, total);
        // ponytail: the original divides by zero (a fault) after 256 home games in a season; the copy writes 0
        write_i32(season + r::AVERAGE_ATTENDANCE, if home != 0 { total.wrapping_div(home as i32) } else { 0 });
        write_u16(season + r::AVERAGE_ATTENDANCE_YEAR, 0);
    }
    attendance_record(this, club, entry, records, view, k::HIGHEST_ATTENDANCE, r::HIGHEST_ATTENDANCE, attendance, |v, rec| v > rec);
    attendance_record(this, club, entry, records, view, k::HIGHEST_GATE, r::HIGHEST_GATE, read_i32(result + match_record::GATE_RECEIPTS), |v, rec| v > rec);
    attendance_record(this, club, entry, records, view, k::LOWEST_ATTENDANCE, r::LOWEST_ATTENDANCE, attendance, |v, rec| v < rec);
    let goals = || (read_i8(view + club_match_record::GOALS_FOR) as i32, read_i8(view + club_match_record::GOALS_AGAINST) as i32);
    let applies = |kind: u8| exe::club_records_record_type_applies(kind as u32, club, 1) != 0;
    let league = || exe::comp_util_is_league_division(read_i32(f.addr())) != 0;
    let (gf, ga) = goals();
    if applies(k::BIGGEST_WIN) && gf > ga {
        result_record(this, club, records, view, &[k::BIGGEST_WIN], r::BIGGEST_WIN, match_record_is_bigger_win, gf - ga >= 6);
    }
    if applies(k::BIGGEST_LEAGUE_WIN) && league() && gf > ga {
        covered!("gra_15 league win");
        result_record(this, club, records, view, &[k::BIGGEST_WIN, k::BIGGEST_LEAGUE_WIN], r::BIGGEST_LEAGUE_WIN, match_record_is_bigger_win, gf - ga >= 6);
    }
    if applies(k::BIGGEST_DEFEAT) && gf < ga {
        result_record(this, club, records, view, &[k::BIGGEST_DEFEAT], r::BIGGEST_DEFEAT, match_record_is_bigger_defeat, ga - gf >= 6);
    }
    if applies(k::HIGHEST_SCORING) && ga + gf > 0 {
        result_record(this, club, records, view, &[k::HIGHEST_SCORING], r::HIGHEST_SCORING, match_record_has_more_goals, ga + gf >= 8);
    }
    if applies(k::HIGHEST_SCORING_LEAGUE) && league() && ga + gf > 0 {
        result_record(this, club, records, view, &[k::HIGHEST_SCORING, k::HIGHEST_SCORING_LEAGUE], r::HIGHEST_SCORING_LEAGUE, match_record_has_more_goals, ga + gf >= 8);
    }
    use club_entry_runs as e;
    if gf > ga {
        covered!("gra_15 won");
        run_record(this, club, entry, records, k::WON_IN_ROW, e::WON_RUN, r::WON_IN_ROW, 6);
        write_i32(entry + e::LOST_RUN, 0);
        write_i32(entry + e::WINLESS_RUN, 0);
    } else if gf < ga {
        run_record(this, club, entry, records, k::LOST_IN_ROW, e::LOST_RUN, r::LOST_IN_ROW, 6);
        write_i32(entry + e::WON_RUN, 0);
        write_i32(entry + e::UNBEATEN_RUN, 0);
    }
    if gf >= ga {
        run_record(this, club, entry, records, k::UNBEATEN, e::UNBEATEN_RUN, r::UNBEATEN, 12);
        write_i32(entry + e::LOST_RUN, 0);
    }
    if gf <= ga {
        run_record(this, club, entry, records, k::WINLESS, e::WINLESS_RUN, r::WINLESS, 12);
        write_i32(entry + e::WON_RUN, 0);
    }
}

/// What a player record needs: the records object, the club, its entry, the season and all-time records, the person,
/// the season year (`exe::key_nation_get_club_season_year`) and the club's view of the match.
#[derive(Clone, Copy)]
struct PlayerRecordsCtx {
    this: usize,
    club: Club,
    entry: usize,
    season: usize,
    all_time: usize,
    person: Staff,
    year: u16,
    view: usize,
}

impl PlayerRecordsCtx {
    unsafe fn applies(self, kind: u8) -> bool {
        exe::club_records_record_type_applies(kind as u32, self.club, 1) != 0
    }

    /// The club has 3 or more seasons recorded (`club_record_entry::SEASONS`).
    unsafe fn seasoned(self) -> bool {
        read_i16(self.entry + club_record_entry::SEASONS) >= 3
    }

    /// The all-time holder of the record at `at` is someone else or held it in another season.
    unsafe fn new_holder(self, at: usize) -> bool {
        use club_match_records as r;
        read_u32(self.all_time + at + r::HOLDER + record_holder::STAFF_ID) != read_u32(self.person.addr()) || read_u16(self.all_time + at + r::YEAR) != self.year
    }

    /// A one-byte season tally (`at`, −1 = none) beaten by `value` (signed comparison).
    unsafe fn byte_beats(rec: usize, value: u8) -> bool {
        read_i8(rec) == -1 || value as i32 > read_i8(rec) as i32
    }

    /// The byte tally record at `at` of type `kind` with `value`: the season record takes it, the holder and year 0;
    /// beating the all-time one too: `news()` decides the news (the extra points it returns are added to `points`) and
    /// the all-time record takes the value, the holder and the season year.
    unsafe fn tally(self, kind: u8, at: usize, value: u8, points: &mut i32, news: impl FnOnce() -> i32) {
        use club_match_records as r;
        if !self.applies(kind) || !Self::byte_beats(self.season + at, value) {
            return;
        }
        write_u8(self.season + at, value);
        exe::fill_staff_name_ref(self.person, self.season + at + r::HOLDER);
        write_u16(self.season + at + r::YEAR, 0);
        if !Self::byte_beats(self.all_time + at, value) {
            return;
        }
        *points = points.wrapping_add(news());
        write_u8(self.all_time + at, value);
        exe::fill_staff_name_ref(self.person, self.all_time + at + r::HOLDER);
        write_u16(self.all_time + at + r::YEAR, self.year);
    }

    /// The age (`exe::date_get_difference`: years word +2, days word +0) at `date` of someone born on `birth`.
    unsafe fn age(date: usize, birth: usize) -> (u16, i16) {
        let mut out = [0u8; 8];
        let p = exe::date_get_difference(date, out.as_mut_ptr(), read_u32(birth), read_u32(birth + 4));
        (read_u16(p + 2), read_i16(p))
    }

    /// The person's age today ([`G_CURRENT_DATE`]).
    unsafe fn age_today(self) -> (u16, i16) {
        Self::age(G_CURRENT_DATE, self.person.addr() + staff::DATE_OF_BIRTH)
    }

    /// The age record at `at` (youngest: `younger`, oldest: not) beaten by the person: the record's holder at its
    /// match is older (younger) in years, or in days within the same year; no record also beats it.
    unsafe fn age_beats(self, at: usize, younger: bool) -> bool {
        use club_match_records as r;
        if read_i32(at + r::AGE_MATCH + club_match_record::FIXTURE_ID) == -1 {
            return true;
        }
        let (ry, rd) = Self::age(at + r::AGE_MATCH, at);
        let (cy, cd) = self.age_today();
        if younger {
            if ry != cy { ry > cy } else { rd > cd }
        } else if ry != cy {
            ry < cy
        } else {
            rd < cd
        }
    }

    /// The age record: the birth date, the holder and the match, in the record at `at`.
    unsafe fn take_age_record(self, at: usize) {
        use club_match_records as r;
        copy_record_date(at, self.person.addr() + staff::DATE_OF_BIRTH);
        exe::fill_staff_name_ref(self.person, at + r::AGE_HOLDER);
        match_record_assign(at + r::AGE_MATCH, self.view);
    }
}

/// The youngest and oldest player records (types 0x19 and 0x1a, `PlayerRecordsCtx::age_beats`). The all-time news:
/// with 3 or more seasons, a youngest player under 20 or an oldest over 35 (in years today) who is not the all-time
/// holder; an oldest player's news multiplies the points by 1.25 (the points as a 16-bit value, truncated).
unsafe fn player_age_records(c: PlayerRecordsCtx, points: &mut i32) {
    use club_match_records as r;
    use club_record_kind as k;
    for (kind, at, younger) in [(k::YOUNGEST, r::YOUNGEST, true), (k::OLDEST, r::OLDEST, false)] {
        if !c.applies(kind) || !c.age_beats(c.season + at, younger) {
            continue;
        }
        c.take_age_record(c.season + at);
        if !c.age_beats(c.all_time + at, younger) {
            continue;
        }
        if c.seasoned() {
            let (years, _) = c.age_today();
            let notable = if younger { years < 20 } else { years > 35 };
            if notable && read_u32(c.all_time + at + r::AGE_HOLDER + record_holder::STAFF_ID) != read_u32(c.person.addr()) {
                queue_match_record_news(c.this, c.club, &[kind]);
                if !younger {
                    covered!("gra_15 oldest player ever");
                    *points = ftol((*points as i16) as f64 * read_f64(rdata::F64_1_25));
                }
            }
        }
        c.take_age_record(c.all_time + at);
    }
}

/// The 16-bit league tallies (types 0x17 appearances and 0x18 goals, `club_record::LEAGUE_APPEARANCES` /
/// `club_record::LEAGUE_GOALS`, the holder after the value): the player's league count at the club
/// (`get_club_league_stat`, field 1 or 2) beats the season record (−1 none, signed) → the season record takes it and
/// the holder; beating the all-time one too: with 3 or more seasons and another holder, the news and 100 points; the
/// all-time record takes the count and the holder.
unsafe fn league_tally(c: PlayerRecordsCtx, kind: u8, at: usize, field: i8, pdata: usize, points: &mut i32) {
    let value = ftol(get_club_league_stat(read_usize(G_PLAYER_STATS), pdata, field) as f64) as i16;
    if !c.applies(kind) {
        return;
    }
    let beats = |rec: usize| read_i16(rec) == -1 || value > read_i16(rec);
    if !beats(c.season + at) {
        return;
    }
    write_i16(c.season + at, value);
    exe::fill_staff_name_ref(c.person, c.season + at + 2);
    if !beats(c.all_time + at) {
        return;
    }
    if c.seasoned() && read_u32(c.all_time + at + 2 + record_holder::STAFF_ID) != read_u32(c.person.addr()) {
        covered!("gra_15 league tally ever");
        queue_match_record_news(c.this, c.club, &[kind]);
        *points = points.wrapping_add(100);
    }
    write_i16(c.all_time + at, value);
    exe::fill_staff_name_ref(c.person, c.all_time + at + 2);
}

/// The player's records of the match (one lineup row of `update_records_for_match`); returns the points that decide
/// the liking for him (`favoured_by_points`), or `None` when it stops before them.
///
/// Most capped player (type 0x1b, all-time only): at a national side (`exe::database_is_national_team`) that is not a
/// second national side (`exe::club_is_national_team`), in a match with the home side mark (`fixture::HOME_SIDE_MARK`),
/// the person's caps (`staff::INT_APPS`) above the record (`club_record::INTERNATIONAL_CAPS`, −1 none): the news when
/// he is not the holder, and the record takes the caps and the holder. A friendly stops here. The player's statistics
/// record of the match category (5 at a national side, 6 otherwise, `exe::playerstats_get_category_record`; missing:
/// the "No player stats" box, as in the original) give the season counts (`exe::player_stats_get_record_value`, weight
/// 6.5). The points: a rating (`lineup_row::RATING`) above 7 gives (rating − 6)^4; with goals (`lineup_row::GOALS`):
/// (goals + 1)^4, the top scorer (0x10, the season's goals; news with 3+ seasons, more than 3 goals and a new holder or
/// season: +100), in a league division the top league scorer (0x11, `exe::playerstats_get_stat` category 2; news with 3+
/// seasons and a new holder or season: +100, but nothing when the top scorer's news is queued) and the most goals in a
/// match (0x12, the match's goals and the match; news from 4 goals: +50); with assists: (assists + 1)^3 and the most
/// assists (0x13; news with 3+ seasons and a new holder or season: +50); the man of the match
/// (`match_record::MAN_OF_THE_MATCH_ID`): +30 and the most man of the match awards (0x15; news when the season count is
/// above 3, with 3+ seasons and a new holder or season: +50); a card in the match: the worst discipline (0x16, red cards
/// × 3 + yellow cards; news with 3+ seasons, 2+ yellows, 2+ reds and a new holder or season); in a league division the
/// league appearances and goals (`league_tally`); the youngest and oldest players (`player_age_records`).
unsafe fn update_player_match_records(c: PlayerRecordsCtx, f: Fixture, result: usize, row: usize, pdata: usize) -> Option<i32> {
    use club_match_records as r;
    use club_record_kind as k;
    let (club, person) = (c.club, c.person);
    if exe::database_is_national_team(club) != 0 && exe::club_is_national_team(club) == 0 && f.home_side_mark() != 0 && c.applies(k::INTERNATIONAL_CAPS) {
        let caps = person.int_apps() as i16;
        let rec = read_i16(c.all_time + club_record::INTERNATIONAL_CAPS);
        if rec == -1 || caps > rec {
            covered!("gra_15 most capped");
            if read_u32(c.all_time + club_record::INTERNATIONAL_CAPS + 2 + record_holder::STAFF_ID) != read_u32(person.addr()) {
                queue_match_record_news(c.this, club, &[k::INTERNATIONAL_CAPS]);
            }
            write_i16(c.all_time + club_record::INTERNATIONAL_CAPS, caps);
            exe::fill_staff_name_ref(person, c.all_time + club_record::INTERNATIONAL_CAPS + 2);
        }
    }
    if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), f.competition()) != 0 {
        return None;
    }
    let stats = exe::playerstats_get_category_record(read_usize(G_PLAYER_STATS), pdata, if exe::database_is_national_team(club) != 0 { 5 } else { 6 });
    if stats == 0 {
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        let comp = f.competition().addr();
        sprintf(0xde1f64, 0x98b6ec, club.addr() + 4, if comp == 0 { 0x98b714 } else { comp + 4 }); // "No player stats for %s playing in %s"
        show_error_at(0x98b6a0, 0x31a);
    }
    let count = |which: u32| ftol(exe::player_stats_get_record_value(stats, which, 6.5)) as u8;
    let league = || exe::comp_util_is_league_division(read_i32(f.addr())) != 0;
    let mut points = 0i32;
    covered!("gra_15 player points");
    let rating = read_u8(row + lineup_row::RATING);
    if rating > 7 {
        points = ftol(game_pow((rating as i32 - 6) as f64, read_f64(rdata::F64_4)));
    }
    let goals = read_u8(row + lineup_row::GOALS);
    if goals != 0 {
        points = points.wrapping_add(ftol(game_pow((goals as i32 + 1) as f64, read_f64(rdata::F64_4))));
        let scored = count(2);
        c.tally(k::TOP_SCORER, r::TOP_SCORER, scored, &mut points, || {
            if c.seasoned() && scored > 3 && c.new_holder(r::TOP_SCORER) {
                covered!("gra_15 top scorer ever");
                queue_match_record_news(c.this, club, &[k::TOP_SCORER]);
                100
            } else {
                0
            }
        });
        if league() {
            let scored = ftol(exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), pdata, 2, 2, 6.5)) as u8;
            c.tally(k::TOP_LEAGUE_SCORER, r::TOP_LEAGUE_SCORER, scored, &mut points, || {
                if c.seasoned() && c.new_holder(r::TOP_LEAGUE_SCORER) {
                    match queue_match_record_news(c.this, club, &[k::TOP_SCORER, k::TOP_LEAGUE_SCORER]) {
                        Some(k::TOP_SCORER) => 0,
                        _ => 100,
                    }
                } else {
                    0
                }
            });
        }
        if c.applies(k::GOALS_IN_MATCH) && PlayerRecordsCtx::byte_beats(c.season + r::GOALS_IN_MATCH, goals) {
            write_u8(c.season + r::GOALS_IN_MATCH, goals);
            exe::fill_staff_name_ref(person, c.season + r::GOALS_IN_MATCH + r::HOLDER);
            match_record_assign(c.season + r::GOALS_IN_MATCH + 0x19, c.view);
            if PlayerRecordsCtx::byte_beats(c.all_time + r::GOALS_IN_MATCH, goals) {
                if goals >= 4 {
                    covered!("gra_15 goals in a match ever");
                    queue_match_record_news(c.this, club, &[k::GOALS_IN_MATCH]);
                    points = points.wrapping_add(50);
                }
                write_u8(c.all_time + r::GOALS_IN_MATCH, goals);
                exe::fill_staff_name_ref(person, c.all_time + r::GOALS_IN_MATCH + r::HOLDER);
                match_record_assign(c.all_time + r::GOALS_IN_MATCH + 0x19, c.view);
            }
        }
    }
    let assists = read_u8(row + lineup_row::ASSISTS);
    if assists != 0 {
        points = points.wrapping_add(ftol(game_pow((assists as i32 + 1) as f64, read_f64(rdata::F64_3))));
        let made = count(5);
        c.tally(k::ASSISTS, r::ASSISTS, made, &mut points, || {
            if c.seasoned() && c.new_holder(r::ASSISTS) {
                queue_match_record_news(c.this, club, &[k::ASSISTS]);
                50
            } else {
                0
            }
        });
    }
    if read_u32(row + lineup_row::PLAYER_ID) == read_u32(result + match_record::MAN_OF_THE_MATCH_ID) {
        covered!("gra_15 man of the match");
        let awards = count(0xc);
        points = points.wrapping_add(30);
        c.tally(k::MAN_OF_MATCH, r::MAN_OF_MATCH, awards, &mut points, || {
            if read_i8(c.season + r::MAN_OF_MATCH) > 3 && c.seasoned() && c.new_holder(r::MAN_OF_MATCH) {
                queue_match_record_news(c.this, club, &[k::MAN_OF_MATCH]);
                50
            } else {
                0
            }
        });
    }
    if read_u8(row + lineup_row::YELLOW_CARDS) != 0 || read_u8(row + lineup_row::RED_CARD) != 0 {
        let (yellow, red) = (count(10), count(11));
        let score = red as i32 * 3 + yellow as i32;
        let beats = |rec: usize| read_i8(rec) == -1 || score > read_i8(rec + 1) as i32 * 3 + read_i8(rec) as i32;
        let at = r::DISCIPLINE;
        if c.applies(k::DISCIPLINE) && beats(c.season + at) {
            covered!("gra_15 discipline");
            write_u8(c.season + at, yellow);
            write_u8(c.season + at + 1, red);
            exe::fill_staff_name_ref(person, c.season + at + 2);
            write_u16(c.season + at + 0x1a, 0);
            if beats(c.all_time + at) {
                let other = read_u32(c.all_time + at + 2 + record_holder::STAFF_ID) != read_u32(person.addr()) || read_u16(c.all_time + at + 0x1a) != c.year;
                if c.seasoned() && yellow >= 2 && red >= 2 && other {
                    queue_match_record_news(c.this, club, &[k::DISCIPLINE]);
                }
                write_u8(c.all_time + at, yellow);
                write_u8(c.all_time + at + 1, red);
                exe::fill_staff_name_ref(person, c.all_time + at + 2);
                write_u16(c.all_time + at + 0x1a, c.year);
            }
        }
    }
    if league() {
        league_tally(c, k::LEAGUE_APPEARANCES, club_record::LEAGUE_APPEARANCES, 1, pdata, &mut points);
        if read_u8(row + lineup_row::GOALS) != 0 {
            league_tally(c, k::LEAGUE_GOALS, club_record::LEAGUE_GOALS, 2, pdata, &mut points);
        }
    }
    player_age_records(c, &mut points);
    Some(points)
}

/// The liking for a player who did well (the end of each lineup row of `update_records_for_match`).
///
/// The points (as a 16-bit value) must be 40 or more, the person in the staff table, the match's reputation `rep` 5 or
/// more and the person without a future transfer (`exe::transfer_manager_get_future_transfer`, when there is a transfer
/// manager). Against a rival (`club::RIVAL1`..3) the points × (rep × 0.08 + 1); in a clash (`clash`,
/// `exe::comp_util_get_league_clash_type` above 0) × (rep × 0.08 + 1) again; otherwise, not against a rival, when rep is
/// below the club's reputation / 500, × rep × 500 / reputation, kept within 0.2 and 1. Then three draws against
/// `utils_random(4000) + 250`: the chairman (`club::CHAIRMAN`) and the manager (`club::MANAGER`) favour the player
/// (`exe::staff_add_favoured_staff`, forced) and the club does (`exe::club_add_favoured_staff`); and each squad member
/// (`club::SQUAD`, 50) who is a player (`staff::PLAYER`) and not him, against `utils_random(7000) + 250`.
unsafe fn favoured_by_points(club: Club, person: Staff, points: i32, rep: i16, rival: bool, clash: bool) {
    let mut p = points;
    if (p as i16) < 40 {
        return;
    }
    if !person.is_null() && read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) {
        return;
    }
    if rep < 5 {
        return;
    }
    let tm = read_usize(G_TRANSFER_MANAGER);
    if tm != 0 && exe::transfer_manager_get_future_transfer(tm, person, 0) != 0 {
        return;
    }
    covered!("gra_15 liking draws");
    let boost = |p: i32| ftol((rep as f64 * read_f64(rdata::F64_0_08) + read_f64(rdata::F64_1)) * (p as i16) as f64);
    if p as i16 != 0 {
        if rival {
            covered!("gra_15 against a rival");
            p = boost(p);
        }
        if clash {
            p = boost(p);
        } else if !rival {
            let reputation = club.reputation() as i32;
            if (rep as i32) < reputation / 500 {
                covered!("gra_15 small match");
                let share = ((rep as i32 * 500) / reputation) as f64;
                let share = if share < read_f64(rdata::F64_0_2) {
                    read_f64(rdata::F64_0_2)
                } else if share > read_f64(rdata::F64_1) {
                    read_f64(rdata::F64_1)
                } else {
                    share
                };
                p = ftol((p as i16) as f64 * share);
            }
        }
    }
    let s = (p as i16) as i32;
    if s > random(4000) + 250 {
        exe::staff_add_favoured_staff(read_usize(club.addr() + club::CHAIRMAN), person, 1);
    }
    if s > random(4000) + 250 {
        exe::staff_add_favoured_staff(read_usize(club.addr() + club::MANAGER), person, 1);
    }
    if s > random(4000) + 250 {
        covered!("gra_15 club favours");
        exe::club_add_favoured_staff(club, person, 1);
    }
    for k in 0..50 {
        let m = read_usize(club.addr() + club::SQUAD + 4 * k);
        if m != 0 && read_usize(m + staff::PLAYER) != 0 && m != person.addr() && s > random(7000) + 250 {
            exe::staff_add_favoured_staff(m, person, 1);
        }
    }
}

/// Updates both clubs' season and all-time records after a match, from attendance and scores to the players', queues
/// the record news and lets the club, its chairman, manager and players favour the players who did well.
///
/// `Club_Records::update_records_for_match(fixture, result)` (SEH frame without a destructor). A null fixture or result:
/// the error boxes (lines 0x10c and 0x11b) and 0 (the original); a reserve or minor competition
/// (`exe::comp_is_reserve_or_minor_comp` of [`G_RESERVE_COMP`]): 0. The match's reputation: 1, or the competition's
/// (`competition::REPUTATION`; × 1.5, truncated, in the round 0x96 without prefix), at least 1. Then for the home side
/// and the away side: rival (the opponent is one of `club::RIVAL1`..3), clash (`exe::comp_util_get_league_clash_type`
/// above 0), the season and all-time records (`club_season_record`/`club_all_time_record`, created when missing) and the
/// club's entry (`club_records_entry`); unless it is a friendly of a club that is not a national side, the season year
/// and the club's records (`update_club_match_records`); then for each of the 20 lineup rows that played
/// (`match_lineup_row`) the player's records (`update_player_match_records`) and the liking (`favoured_by_points`); out
/// of friendlies, the season's best average rating is reset (`club_record::SEASON_RATING` −1.0, recomputed by
/// `update_season_rating_record`). Every news item is queued only when the same club and type is not queued yet
/// (`queue_match_record_news`). Returns 1.
/// Original: `Club_Records::update_records_for_match` (0x444930, club_records.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn update_records_for_match(this: usize, fixture: usize, result: usize) -> u32 {
    let original = || original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_UPDATE_RECORDS_FOR_MATCH)(this, fixture, result);
    if fixture == 0 {
        return original(); // the box (line 0x10c)
    }
    let f = Fixture(fixture);
    if exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), f.competition()) != 0 {
        return 0;
    }
    if result == 0 {
        return original(); // the box (line 0x11b)
    }
    let mut rep: i16 = 1;
    let comp = f.competition();
    if !comp.is_null() {
        rep = comp.reputation();
        if f.round_prefix() == 0 && f.round_name() == 0x96 {
            covered!("gra_15 round 0x96");
            rep = ftol(rep as f64 * read_f64(rdata::F64_1_5)) as i16;
        }
        if rep < 1 {
            rep = 1;
        }
    }
    let mut view = [0u8; 0x20]; // the club's view of the match (club_match_record), built by the side that plays
    let view = view.as_mut_ptr() as usize;
    exe::date_date_54a110(view);
    for side in 0..2usize {
        let (club, opponent) = if side == 0 { (read_usize(fixture + fixture::HOME_CLUB), read_usize(fixture + fixture::AWAY_CLUB)) } else { (read_usize(fixture + fixture::AWAY_CLUB), read_usize(fixture + fixture::HOME_CLUB)) };
        let club = Club(club);
        let rival = club.rival1() == opponent || club.rival2() == opponent || club.rival3() == opponent;
        let clash = (exe::comp_util_get_league_clash_type(club, opponent) as i8) > 0;
        let season = club_season_record(this, club.addr(), 1);
        let all_time = club_all_time_record(this, club.addr(), 1);
        let entry = club_records_entry(this, club.addr());
        let mut year = 0u16;
        if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), f.competition()) == 0 || exe::database_is_national_team(club) != 0 {
            year = exe::key_nation_get_club_season_year(club);
            update_club_match_records(this, f, result, club, entry, (season, all_time), view);
        }
        for k in 0..20 {
            let Some((row, person, pdata)) = match_lineup_row(result, side, k) else { continue };
            let c = PlayerRecordsCtx { this, club, entry, season, all_time, person: Staff(person), year, view };
            if let Some(points) = update_player_match_records(c, f, result, row, pdata) {
                favoured_by_points(club, Staff(person), points, rep, rival, clash);
            }
        }
        if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), f.competition()) == 0 {
            write_u32(season + club_record::SEASON_RATING, 0xbf800000); // -1.0
        }
    }
    1
}

/// Comparer for `update_records_for_match`.
///
/// Side by side from the same state, after creating the four records in the original's order (home season, home
/// all-time, away season, away all-time: `get_season_record` and `get_all_time_record` allocate) and fetching them again
/// (a new one may move the others). Compared, each by name: the four records (0x38b bytes) and the two club entries
/// (0x6a), the generator (`crate::rng::STATE_REGIONS`), the favoured lists that the liking may change (each club's
/// three favoured and disliked staff, `club::FAV_STAFF1`, and those of its chairman, manager and 50 squad members,
/// `preferences::STAFF_FAVOURITE_STAFF1`), the news queue items the original appended (it may reallocate the queue:
/// the copy runs dry, `MATCH_RECORDS_NEWS_DRY_RUN`, looking only at the items queued before) and the return value. A
/// null fixture or result or a reserve competition: only the original.
pub(crate) unsafe extern "thiscall" fn compare_update_records_for_match(this: usize, fixture: usize, result: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_UPDATE_RECORDS_FOR_MATCH);
    if fixture == 0 || result == 0 || exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), Fixture(fixture).competition()) != 0 {
        return o(this, fixture, result);
    }
    let clubs = [Club(read_usize(fixture + fixture::HOME_CLUB)), Club(read_usize(fixture + fixture::AWAY_CLUB))];
    for c in clubs {
        club_season_record(this, c.addr(), 1);
        club_all_time_record(this, c.addr(), 1);
    }
    let mut named: Vec<(&str, Vec<(usize, usize)>)> = Vec::new();
    let mut likes = crate::rng::STATE_REGIONS.to_vec();
    for (side, c) in clubs.iter().enumerate() {
        let name = if side == 0 { ["home season", "home all-time", "home entry"] } else { ["away season", "away all-time", "away entry"] };
        named.push((name[0], vec![(club_season_record(this, c.addr(), 0), 0x38b)]));
        named.push((name[1], vec![(club_all_time_record(this, c.addr(), 0), 0x38b)]));
        named.push((name[2], vec![(club_records_entry(this, c.addr()), 0x6a)]));
        likes.push((c.addr() + club::FAV_STAFF1, 24));
        let staff = [read_usize(c.addr() + club::CHAIRMAN), read_usize(c.addr() + club::MANAGER)].into_iter().chain((0..50).map(|k| read_usize(c.addr() + club::SQUAD + 4 * k)));
        for s in staff {
            if s != 0 && Staff(s).staff_preferences() != 0 {
                likes.push((Staff(s).staff_preferences() + preferences::STAFF_FAVOURITE_STAFF1, 24));
            }
        }
    }
    named.push(("generator and favoured lists", likes));
    let regions: Vec<(usize, usize)> = named.iter().flat_map(|(_, r)| r.iter().copied()).collect();
    let before_count = read_i32(this + club_records_fields::NEWS_QUEUE_COUNT);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, fixture, result), || {
        *MATCH_RECORDS_NEWS_DRY_RUN.lock().unwrap() = Some((before_count, Vec::new()));
        crate::rng::without_counting(|| update_records_for_match(this, fixture, result))
    });
    let queued_rust = MATCH_RECORDS_NEWS_DRY_RUN.lock().unwrap().take().map(|(_, v)| v).unwrap_or_default();
    let queue = read_usize(this + club_records_fields::NEWS_QUEUE);
    let queued_orig: Vec<(u32, u8)> = (before_count.max(0)..read_i32(this + club_records_fields::NEWS_QUEUE_COUNT)).map(|k| (read_u32(queue + 6 * k as usize), read_u8(queue + 6 * k as usize + record_news_item::RECORD_TYPE))).collect();
    SCOREBOARD.record(equal && ro == rr && queued_orig == queued_rust, || {
        let flat = |m: &crate::comparison::MemorySnapshot| m.0.iter().flat_map(|(_, v)| v.iter().copied()).collect::<Vec<u8>>();
        let (a, b) = (flat(&mo), flat(&mr));
        let mut k = 0;
        let mut differ = Vec::new();
        for (name, r) in &named {
            let n: usize = r.iter().map(|x| x.1).sum();
            if let Some(off) = (0..n).find(|&i| a.get(k + i) != b.get(k + i)) {
                differ.push(format!("{} (+{:#x}: {:02x?} x {:02x?})", name, off, a.get(k + off), b.get(k + off)));
            }
            k += n;
        }
        format!("update_records_for_match(fixture {:#x}) returned {} x {}, differ in {:?}, news original {:?} rust {:?}", fixture, ro, rr, differ, queued_orig, queued_rust)
    });
    ro
}

// ---------- batch 324: the news text of a ban (discipline.cpp) ----------

/// `Discipline::get_scope_text(rules, out, size, appeal_pending, form)` (0x55ce40, discipline.cpp; thiscall, ret 0x14):
/// the scope of a ban ("European", "domestic"...) from the four rule-set bytes into `out`; returns the scope's
/// grammatical gender.
const FN_DISCIPLINE_GET_SCOPE_TEXT: usize = 0x55ce40;
/// `Date::get_date_string_no_weekday(date, out, size, time_of_day)` (0x54c3b0, Date.cpp; thiscall, ret 0xc).
const FN_DATE_GET_DATE_STRING_NO_WEEKDAY: usize = 0x54c3b0;

/// The ban templates `[plain, a further ban, starting on a date]`, by the count of the ban: `[N matches, one match,
/// days]` (a count below 0 is a ban in days). Points: "after collecting <n> points".
const BAN_POINTS: [[u32; 3]; 3] = [
    [0x9c08e8, 0x9c098c, 0x9c080c],
    [0x9c0b08, 0x9c0b98, 0x9c0a3c],
    [0x9bb388, 0x9bb438, 0x9bb2a0],
];
/// "after collecting <n> yellow cards in successive matches".
const BAN_SUCCESSIVE_YELLOWS: [[u32; 3]; 3] = [
    [0x9be6a8, 0x9be768, 0x9be5b0],
    [0x9be918, 0x9be9c4, 0x9be834],
    [0x9ba49c, 0x9ba568, 0x9ba398],
];
/// "after collecting <n> yellow cards".
const BAN_YELLOWS: [[u32; 3]; 3] = [
    [0x9beb58, 0x9bec04, 0x9bea78],
    [0x9bed88, 0x9bee20, 0x9becb8], // gra16: one match
    [0x9ba728, 0x9ba7dc, 0x9ba63c],
];
/// The bans that come from a match, `[count][the player's club won, drew, lost, the competition named][form]`: the
/// result forms name the score ("<club>'s 2-1 win over <club>"), the competition form names the competition and the
/// two clubs (never for a ban in days). "after collecting a yellow card in".
const BAN_YELLOW_IN_MATCH: [[[u32; 3]; 4]; 3] = [
    [[0x9bfc50, 0x9bfd4c, 0x9bfb1c], [0x9beff0, 0x9bf0ec, 0x9beebc], [0x9bf620, 0x9bf71c, 0x9bf4ec], [0x9c0294, 0x9c03a8, 0x9c0148]],
    [[0x9bff70, 0x9c0058, 0x9bfe50], [0x9bf314, 0x9bf3fc, 0x9bf1f4], [0x9bf944, 0x9bfa2c, 0x9bf824], [0x9c0600, 0x9c0704, 0x9c04c8]],
    [[0x9bb088, 0x9bb190, 0x9baf4c], [0x9ba9dc, 0x9baae4, 0x9ba89c], [0x9bad34, 0x9bae3c, 0x9babf4], [0, 0, 0]],
];
/// "after being sent off in" (a ban in days: "after collecting a red card in").
const BAN_SENT_OFF: [[[u32; 3]; 4]; 3] = [
    [[0x9bda60, 0x9bdb50, 0x9bd938], [0x9bce78, 0x9bcf6c, 0x9bcd50], [0x9bd46c, 0x9bd560, 0x9bd344], [0x9be068, 0x9be174, 0x9bdf24]],
    [[0x9bdd60, 0x9bde40, 0x9bdc4c], [0x9bd180, 0x9bd260, 0x9bd068], [0x9bd774, 0x9bd854, 0x9bd65c], [0x9be3b8, 0x9be4b0, 0x9be288]],
    [[0x9ba188, 0x9ba28c, 0x9ba04c], [0x9b9ae8, 0x9b9bec, 0x9b99ac], [0x9b9e38, 0x9b9f3c, 0x9b9cfc], [0, 0, 0]],
];
/// "after his misconduct in".
const BAN_MISCONDUCT: [[[u32; 3]; 4]; 3] = [
    [[0x9bc200, 0x9bc2f0, 0x9bc0d8], [0x9bb618, 0x9bb70c, 0x9bb4f0], [0x9bbc0c, 0x9bbd00, 0x9bbae4], [0x9bc808, 0x9bc914, 0x9bc6c4]],
    [[0x9bc500, 0x9bc5e0, 0x9bc3ec], [0x9bb920, 0x9bba00, 0x9bb808], [0x9bbf14, 0x9bbff4, 0x9bbdfc], [0x9bcb58, 0x9bcc50, 0x9bca28]],
    [[0x9b97a8, 0x9b98a4, 0x9b9674], [0x9b9138, 0x9b9234, 0x9b9004], [0x9b9470, 0x9b956c, 0x9b933c], [0, 0, 0]],
];

/// The match record a ban news item carries (`read_match_record_from_news`, 0x1f bytes): the date (+0 day, +2 year),
/// the club competition (+8), the round and stage words (+0xc, +0xe), the player's club (+0x11) and its opponent
/// (+0x15), their goals (+0x19, +0x1a) and the fixture index (+0x1b).
struct BanMatch([u8; 0x20]);

impl BanMatch {
    fn at(&self, k: usize) -> usize {
        self.0.as_ptr() as usize + k
    }
    unsafe fn comp_index(&self) -> i32 {
        read_i32(self.at(8))
    }
    unsafe fn club(&self) -> usize {
        club_at(read_i32(self.at(0x11)) as isize).addr()
    }
    unsafe fn opponent(&self) -> usize {
        club_at(read_i32(self.at(0x15)) as isize).addr()
    }
    unsafe fn goals(&self) -> (i8, i8) {
        (read_i8(self.at(0x19)), read_i8(self.at(0x1a)))
    }
    unsafe fn fixture(&self) -> u32 {
        read_u32(self.at(0x1b))
    }
    /// The club competition's record (`G_CLUB_COMP_TABLE`, 0x6b bytes; its names where `competition::` has them).
    unsafe fn comp(&self) -> usize {
        read_usize(G_CLUB_COMP_TABLE).wrapping_add((self.comp_index() as usize).wrapping_mul(0x6b))
    }
}

/// The date `Date::Date` (0x54a110) leaves before the readers overwrite it: day 0x28 of 1972 (0x7b4), flag 1.
const BAN_DEFAULT_DATE: [u8; 8] = [0x28, 0, 0xb4, 0x07, 1, 0, 0, 0];

/// The arguments of one template, in the order the original pushes them (langlib_sprintf is variadic: the unused
/// tail of the array is passed and never read).
struct BanArgs {
    v: [usize; 16],
    n: usize,
}

impl BanArgs {
    fn new() -> Self {
        BanArgs { v: [0; 16], n: 0 }
    }
    fn push(&mut self, x: usize) {
        self.v[self.n] = x;
        self.n += 1;
    }
    /// A gender byte, sign-extended as the original pushes it, twice.
    fn gender(&mut self, g: i8) {
        self.push(g as i32 as usize);
        self.push(g as i32 as usize);
    }
}

/// `strcat(dst, src)` on two NUL-terminated buffers (the original's inline `repne scasb` / `rep movs`).
unsafe fn ban_strcat(dst: usize, src: usize) {
    let mut d = dst;
    while read_u8(d) != 0 {
        d += 1;
    }
    let mut s = src;
    loop {
        let c = read_u8(s);
        write_u8(d, c);
        if c == 0 {
            break;
        }
        d += 1;
        s += 1;
    }
}

/// The sentence on how the match ended, from the player's club's goals `g1` and the opponent's `g2` (`d` = g1 − g2,
/// `s` = g1 + g2): won by more than 5 "ran riot with a <score> victory" (0x9c0f50), by more than 3 "ran out easy
/// winners" (0x9c0efc), otherwise won "won a thrilling game" (s > 5, 0x9c0ea8) or "won the match" (0x9c0e5c); 0-0 "a
/// stalemate" (0x9c0da8); another draw "a thrilling draw" (s > 5, 0x9c0e1c) or "a draw" (0x9c0de4); lost by more
/// than 5 "a humiliating defeat" (0x9c0d30), by more than 3 "were easily beaten" (0x9c0cdc), otherwise "were beaten in
/// a cracking match" (s > 5, 0x9c0c7c) or "were beaten" (0x9c0c30). The club forms take the club's short name and
/// gender; the draws only the score.
unsafe fn ban_result_sentence(out: usize, m: &BanMatch, score: usize) {
    let fmt = news_text_format();
    let (g1, g2) = m.goals();
    let (g1, g2) = (g1 as i32, g2 as i32);
    let (d, s) = (g1 - g2, g1 + g2);
    let c = m.club();
    let g = read_i8(c + club::SHORT_GENDER_NAME) as i32;
    let name = c + club::SHORT_NAME;
    let team = if d > 5 {
        0x9c0f50
    } else if d > 3 {
        0x9c0efc
    } else if g1 > g2 {
        if s > 5 { 0x9c0ea8 } else { 0x9c0e5c }
    } else if g1 == g2 {
        let draw = if g1 == 0 { 0x9c0da8 } else if s > 5 { 0x9c0e1c } else { 0x9c0de4 };
        fmt(out, draw, score);
        return;
    } else if d < -5 {
        0x9c0d30
    } else if d < -3 {
        0x9c0cdc
    } else if s > 5 {
        0x9c0c7c
    } else {
        0x9c0c30
    };
    fmt(out, team, g, g, name, score);
}

/// Writes the headline or the body of the news that a player was banned (types 0x7d2, and 0x7d3 through
/// `write_further_ban_news_text`).
///
/// `write_ban_news_text(news, full, out)`. The news item's slots: 0 the appeal made, 2 the count (matches; below 0 the
/// ban is in days), 3 the days, 4 and 5 the day and year the ban starts (year 0: no date), 6..9 the four rule sets
/// (`Discipline::get_scope_text`), 10 the kind (0 points, 1 yellow cards, 2 a sending-off, 3 misconduct, 4 yellow
/// cards in successive matches), 11 the number (points or cards; for kind 1, 1 = "a yellow card in" a match), 12 the
/// appeal pending, 14 "a further" ban; from slot 17 the person (`read_staff_name_ref_from_news`), then the match flag
/// and the match record (`read_match_record_from_news`, `BanMatch`). With a match, the competition is named when it is
/// not a reserve or minor competition (`exe::comp_is_reserve_or_minor_comp` of `G_RESERVE_COMP`) unless it is a league
/// division (`exe::comp_util_is_league_division`) in round 0x418, 0x419 or 0xa0 or with a stage word; the score link
/// holds when the fixture still matches (`exe::match_stats_fixture_entry_matches`). The person's links hold when his
/// names still match (`exe::database_staff_matches_names`).
///
/// The headline: "<name> banned for one match" (0x9c1230), "for <n> matches" (0x9c11e4) or, in days, "for <length>"
/// (0x9c1198, `format_duration_text`, 0x1e bytes), name in mode 5; the person's link.
///
/// The body (name in mode 1; the date text when there is a year; the score "<more>-<fewer>" with a match; the scope
/// text of form 4, whose gender is dropped):
/// - rule set 9 = 0 with an appeal pending, the assault on a referee: "banned for <days> days after his assault on a
///   referee in the <competition> match between" (0x9c1078, the competition's short name and gender) when named,
///   otherwise "in the match between" (0x9c0fa8); with a match and the club's goals ≥ 0 the sentence on how it ended
///   is appended (`ban_result_sentence`) with the score's link; then the person's and the two clubs' links (no
///   appeal note);
/// - otherwise the scope text again (form 5 for one match, else 6, its gender kept) and the template of the kind
///   (`BAN_*`: by the count, by "a further" ban, then by a start date, then plain), the arguments in the original's
///   order: the scope's gender twice, 0x64 for the date (twice for one match by points), the competition's and the
///   clubs' genders, the name, the count, the scope, the days, the date, then the number or the competition and the
///   clubs. A match kind not in days names the competition (long name) when it is named, with the sentence on how it
///   ended appended when there is a match; otherwise the club's result (won, drew, lost) picks the form with the
///   score. With the appeal made, "You have appealed..." (0x9b8f68) follows after a blank line. The links: the
///   person, the two clubs, the score.
///
/// Defects of the original (`DEFEITOS_DO_ORIGINAL`): without a match (the bans by points or by cards) the original
/// links the two clubs and the score of a match record it never read (uninitialised stack, `B+0x20..0x3f` and the
/// fixture flag `B+0xd8`): the copy leaves those three links out (its comparer drops them from the original's list);
/// a kind above 4 formats no template and prints the uninitialised buffer: the copy prints an empty text.
/// Original: `Discipline::write_ban_news_text` (0x5576e0, discipline.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_ban_news_text(this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let appealed = slot(0) as u8;
    slot(1);
    let count = slot(2) as u8 as i8;
    let days = slot(3) as u16 as i16 as i32;
    let start_day = slot(4);
    let start_year = slot(5);
    let rules = [slot(6) as u8, slot(7) as u8, slot(8) as u8, slot(9) as u8];
    let kind = slot(10) as u8;
    let number = slot(11) as u8 as i8 as i32;
    let pending = slot(12);
    slot(13);
    let further = slot(14) as u8;
    // the person (four ids and a birth date, from slot 17), then the match flag and the match record
    let mut person = [0u8; 0x18];
    person[0x10..].copy_from_slice(&BAN_DEFAULT_DATE);
    let mut m = BanMatch([0u8; 0x20]);
    m.0[..8].copy_from_slice(&BAN_DEFAULT_DATE);
    let mut next: u8 = 17;
    exe::read_staff_name_ref_from_news(news, &mut next, person.as_mut_ptr());
    let pp = person.as_ptr() as usize;
    let has_match = slot(next as u32) != 0;
    next = next.wrapping_add(1);
    let mut named = false;
    if has_match {
        covered!("gra_16 ban with a match");
        exe::read_match_record_from_news(news, &mut next, m.0.as_mut_ptr());
        if exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), m.comp()) == 0 {
            named = true;
        }
        if exe::comp_util_is_league_division(m.comp_index()) != 0 {
            let round = read_u16(m.at(0xc));
            if round == 0x418 || round == 0x419 || round == 0xa0 || read_u16(m.at(0xe)) != 0 {
                named = false;
            }
        }
    }
    let (first, second, common, id) = (read_u32(pp), read_u32(pp + 4), read_u32(pp + 8), read_u32(pp + 0xc));
    let staff = staff_at(id as i32 as isize);
    let person_ok = exe::database_staff_matches_names(staff, first, second, common, pp + 0x10) != 0;
    let fixture_ok = has_match
        && exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), m.fixture() as i32, read_i32(m.at(0x11)),
            read_i32(m.at(0x15)), read_i32(m.at(0)), read_i32(m.at(2))) != 0;
    let mut name = [0u8; 0xc8];
    let np = name.as_ptr() as usize;
    if full == 0 {
        exe::database_format_name_from_ids(first, second, common, name.as_mut_ptr(), 0xc8, 5, id);
        if count > -1 {
            if count == 1 {
                fmt(NEWS_TEXT_BUFFER, 0x9c1230, np); // "<name> banned for one match"
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9c11e4, np, count as i32); // "<name> banned for <n> matches"
            }
        } else {
            let mut length = [0u8; 0x20];
            format_duration_text(days, length.as_mut_ptr() as usize, 0x1e, 0);
            fmt(NEWS_TEXT_BUFFER, 0x9c1198, np, length.as_ptr() as usize); // "<name> banned for <length>"
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        if person_ok {
            exe::text_add_link(out, name.as_ptr(), 0x7d9, staff);
        }
        return;
    }
    covered!("gra_16 ban body");
    exe::database_format_name_from_ids(first, second, common, name.as_mut_ptr(), 0xc8, 1, id);
    let mut date_text = [0u8; 0x64];
    let dp = date_text.as_mut_ptr() as usize;
    if start_year as u16 != 0 {
        let mut date = [0u8; 8];
        exe::new_date(date.as_mut_ptr() as usize, start_day, start_year);
        let date_string: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_GET_DATE_STRING_NO_WEEKDAY);
        date_string(date.as_ptr() as usize, dp, 0x64, 0xff);
    }
    let mut score = [0u8; 0xc];
    let sp = score.as_mut_ptr() as usize;
    let (g1, g2) = m.goals();
    if has_match {
        fmt(sp, 0x9c116c, g1.max(g2) as i32, g1.min(g2) as i32); // "<more>-<fewer>"
    }
    let scope_text: unsafe extern "thiscall" fn(usize, *const u8, usize, u32, u32, u32) -> i8 = core::mem::transmute(FN_DISCIPLINE_GET_SCOPE_TEXT);
    let mut scope = [0u8; 0x64];
    let scp = scope.as_mut_ptr() as usize;
    scope_text(this, rules.as_ptr(), scp, 0x64, pending, 4);
    let mut text = [0u8; 0x2710];
    let tp = text.as_mut_ptr() as usize;
    let c1 = m.club();
    let c2 = m.opponent();
    let comp = m.comp();
    let link_clubs = |out: usize| {
        exe::text_add_link(out, (c1 + club::SHORT_NAME) as *const u8, 0x7d5, c1);
        exe::text_add_link(out, (c2 + club::SHORT_NAME) as *const u8, 0x7d5, c2);
    };
    if rules[3] == 0 && pending != 0 {
        covered!("gra_16 assault on a referee");
        let (gc1, gc2) = (read_i8(c1 + club::SHORT_GENDER_NAME) as i32, read_i8(c2 + club::SHORT_GENDER_NAME) as i32);
        let (n1, n2) = (c1 + club::SHORT_NAME, c2 + club::SHORT_NAME);
        if named {
            let gc = read_i8(comp + competition::SHORT_GENDER_NAME) as i32;
            fmt(tp, 0x9c1078, gc, gc, gc1, gc1, gc2, gc2, np, days, comp + competition::SHORT_NAME, n1, n2);
        } else {
            fmt(tp, 0x9c0fa8, gc1, gc1, gc2, gc2, np, days, n1, n2);
        }
        exe::text_set_string(out, tp as *const u8);
        if has_match && g1 > -1 {
            ban_result_sentence(tp, &m, sp);
            news_text_append(out, tp);
            if fixture_ok {
                exe::text_add_link(out, score.as_ptr(), 0x7d3, m.fixture() as usize);
            }
        }
        if person_ok {
            exe::text_add_link(out, name.as_ptr(), 0x7d9, staff);
        }
        if has_match {
            link_clubs(out);
        }
        return;
    }
    // the count: 0 = N matches, 1 = one match, 2 = days
    let (row, form) = if count < 0 { (2usize, 6) } else if count == 1 { (1, 5) } else { (0, 6) };
    let scope_gender = scope_text(this, rules.as_ptr(), scp, 0x64, pending, form);
    let variant = if further != 0 { 1 } else if start_year as u16 != 0 { 2 } else { 0 };
    let match_forms = match kind {
        1 if number == 1 => Some(&BAN_YELLOW_IN_MATCH),
        2 => Some(&BAN_SENT_OFF),
        3 => Some(&BAN_MISCONDUCT),
        _ => None,
    };
    let plain_forms = match kind {
        0 => Some(&BAN_POINTS),
        1 if number != 1 => Some(&BAN_YELLOWS),
        4 => Some(&BAN_SUCCESSIVE_YELLOWS),
        _ => None,
    };
    let competition_form = row != 2 && named;
    let template = if let Some(t) = plain_forms {
        Some(t[row][variant])
    } else if let Some(t) = match_forms {
        let result = if competition_form { 3 } else if g1 > g2 { 0 } else if g1 == g2 { 1 } else { 2 };
        Some(t[row][result][variant])
    } else {
        None
    };
    if let Some(template) = template {
        covered!("gra_16 ban template");
        let mut a = BanArgs::new();
        a.gender(scope_gender);
        if variant == 2 {
            a.push(0x64);
            if kind == 0 && row == 1 {
                a.push(0x64);
            }
        }
        if match_forms.is_some() {
            if competition_form {
                a.gender(read_i8(comp + competition::GENDER_NAME));
            }
            a.gender(read_i8(c1 + club::SHORT_GENDER_NAME));
            a.gender(read_i8(c2 + club::SHORT_GENDER_NAME));
        }
        a.push(np);
        if row == 0 {
            a.push(count as i32 as usize);
        }
        a.push(scp);
        if row == 2 {
            a.push(days as usize);
        }
        if variant == 2 {
            a.push(dp);
        }
        if match_forms.is_none() {
            a.push(number as usize);
        } else if competition_form {
            a.push(comp + competition::NAME);
            a.push(c1 + club::SHORT_NAME);
            a.push(c2 + club::SHORT_NAME);
        } else {
            a.push(c1 + club::SHORT_NAME);
            a.push(sp);
            a.push(c2 + club::SHORT_NAME);
        }
        let v = a.v;
        fmt(tp, template as usize, v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9], v[10], v[11], v[12], v[13], v[14], v[15]);
        if competition_form && has_match {
            covered!("gra_16 ban match result");
            let mut result = [0u8; 0x3e8];
            ban_result_sentence(result.as_mut_ptr() as usize, &m, sp);
            ban_strcat(tp, result.as_ptr() as usize);
        }
    }
    let crt_format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    if appealed != 0 {
        covered!("gra_16 ban appealed");
        let mut appeal = [0u8; 0x3d8];
        fmt(appeal.as_mut_ptr() as usize, 0x9b8f68); // "You have appealed against this decision..."
        crt_format(NEWS_TEXT_BUFFER, 0x991948, tp, appeal.as_ptr() as usize); // "%s\n\n%s"
    } else {
        crt_format(NEWS_TEXT_BUFFER, 0x991944, tp); // "%s"
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if person_ok {
        exe::text_add_link(out, name.as_ptr(), 0x7d9, staff);
    }
    if has_match {
        link_clubs(out);
        if fixture_ok {
            exe::text_add_link(out, score.as_ptr(), 0x7d3, m.fixture() as usize);
        }
    }
}

/// Comparer for `write_ban_news_text`: the text side by side (`compare_text_writer`). The body of a ban without a match
/// is compared without the clubs' and the score's links, which the original takes from uninitialised stack.
pub(crate) unsafe extern "thiscall" fn compare_write_ban_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_WRITE_BAN_NEWS_TEXT);
    let what = || format!("write_ban_news_text(news {:#x}, full {})", news, full);
    if full == 0 || exe::newsitem_get_slot(news, 23) != 0 {
        compare_text_writer(out, || o(this, news, full, out), |t| write_ban_news_text(this, news, full, t), what);
        return;
    }
    covered!("gra_16 ban without a match compared");
    let mut local = [0usize; 2];
    let lp = local.as_mut_ptr() as usize;
    let (_, _, same_rng, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || o(this, news, full, out),
        || crate::rng::without_counting(|| write_ban_news_text(this, news, full, lp)));
    let keep = |t: (Vec<u8>, Vec<(Vec<u8>, u32, u32)>)| (t.0, t.1.into_iter().filter(|l| l.1 != 0x7d5 && l.1 != 0x7d3).collect::<Vec<_>>());
    let (mo, mr) = (keep(text_contents(out)), keep(text_contents(lp)));
    exe::drop_text(local.as_mut_ptr());
    SCOREBOARD.record(same_rng && mo == mr, || format!("{}: original {:?} {:x?} rust {:?} {:x?}", what(),
        String::from_utf8_lossy(&mo.0), mo.1.len(), String::from_utf8_lossy(&mr.0), mr.1.len()));
}

// ---------- batch 324: the news of a cup tie's result (comp_util.cpp) ----------

/// The round name code of the final (`fixture::ROUND_NAME` without its variant, with no `fixture::ROUND_PREFIX`).
const ROUND_FINAL: i16 = 0x96;

/// How a tie was settled (`fixture::HOME_RESULT_MARK`, else `fixture::AWAY_RESULT_MARK`), as the cup tie news reads
/// it; any other mark goes by the verdict.
mod tie_mark {
    pub(crate) const EXTRA_TIME: i8 = 3;
    pub(crate) const PENALTIES: i8 = 4;
    pub(crate) const SHOOTOUT: i8 = 5;
    pub(crate) const AWAY_GOALS: i8 = 6;
    pub(crate) const GOLDEN_GOAL: i8 = 7;
    pub(crate) const HIGHER_SEED: i8 = 8;
}

/// The verdict on the result that the caller passes to the cup tie news (the wording of an ordinary result).
mod tie_verdict {
    pub(crate) const EXCELLENT: u8 = 2;
    pub(crate) const FINE: u8 = 3;
    pub(crate) const DISAPPOINTING: u8 = 5;
    pub(crate) const SHOCK: u8 = 6;
    pub(crate) const VERY_DISAPPOINTING: u8 = 7;
}

/// What a cup tie sentence names after the club (with its gender pair before the club's).
#[derive(Clone, Copy, PartialEq)]
enum TieMiddle {
    /// The round label and its gender.
    Round,
    /// The competition's name and gender ("have won the <competition>"); the label then becomes the competition's
    /// name, for its link.
    Trophy,
    /// The competition's name with the round label's gender; the label then becomes the competition's name.
    TrophyRoundGender,
    /// The competition's name and gender, the label kept (a final settled by a forfeit).
    TrophyKeepLabel,
    /// The next stage's name and gender.
    NextStage,
    /// Nothing ("are through to the next stage").
    Nothing,
}

/// The goals in a cup tie sentence.
#[derive(Clone, Copy, PartialEq)]
enum TieScore {
    /// The club's goals, then the opponent's.
    OwnFirst,
    /// The opponent's goals, then the club's; the score link's text is rewritten the same way.
    OpponentFirst,
    /// No goals (a forfeit).
    Nothing,
}

/// The pieces of the cup tie news of one club (`comp_util_write_cup_tie_result_news`).
struct TieNews {
    club: Club,
    opponent: Club,
    comp: Competition,
    stadium: Stadium,
    own_goals: i32,
    opponent_goals: i32,
    label_gender: i32,
    next_gender: i32,
    won: bool,
    final_round: bool,
    single_match: bool,
    mark: i8,
    verdict: u8,
    label: [u8; 500],
    next: [u8; 100],
    venue: [u8; 300],
    score: [u8; 16],
}

impl TieNews {
    /// Writes the sentence `format` into `NEWS_TEXT_BUFFER`. The arguments: the club's gender twice, the middle's
    /// gender twice (when there is a middle), the opponent's gender twice, the stadium's gender twice (with `venue`);
    /// then the club's short name, the middle's text, the goals, the opponent's short name and the venue text (with
    /// `venue`). The goals with the opponent first also rewrite the score text (format 0x9a3d94) that way; a trophy
    /// middle (not the forfeit one) copies the competition's name into the label.
    unsafe fn say(&mut self, format: usize, middle: TieMiddle, score: TieScore, venue: bool) {
        let gender = |g: u8| g as i8 as i32 as usize;
        let comp = self.comp;
        let middle_part = match middle {
            TieMiddle::Round => Some((self.label_gender as usize, self.label.as_ptr() as usize)),
            TieMiddle::Trophy | TieMiddle::TrophyKeepLabel => Some((gender(comp.gender_name()), comp.name_addr())),
            TieMiddle::TrophyRoundGender => Some((self.label_gender as usize, comp.name_addr())),
            TieMiddle::NextStage => Some((self.next_gender as usize, self.next.as_ptr() as usize)),
            TieMiddle::Nothing => None,
        };
        let mut a = [0usize; 14];
        let mut n = 0;
        let mut put = |v: usize| {
            a[n] = v;
            n += 1;
        };
        let club_gender = gender(self.club.short_gender_name());
        put(club_gender);
        put(club_gender);
        if let Some((g, _)) = middle_part {
            put(g);
            put(g);
        }
        let opponent_gender = gender(self.opponent.short_gender_name());
        put(opponent_gender);
        put(opponent_gender);
        if venue {
            let stadium_gender = gender(self.stadium.gender_name());
            put(stadium_gender);
            put(stadium_gender);
        }
        put(self.club.addr() + club::SHORT_NAME);
        if let Some((_, t)) = middle_part {
            put(t);
        }
        match score {
            TieScore::OwnFirst => {
                put(self.own_goals as usize);
                put(self.opponent_goals as usize);
            }
            TieScore::OpponentFirst => {
                put(self.opponent_goals as usize);
                put(self.own_goals as usize);
            }
            TieScore::Nothing => {}
        }
        put(self.opponent.addr() + club::SHORT_NAME);
        if venue {
            put(self.venue.as_ptr() as usize);
        }
        let fmt = news_text_format();
        fmt(NEWS_TEXT_BUFFER, format, a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13]);
        if score == TieScore::OpponentFirst {
            fmt(self.score.as_mut_ptr() as usize, 0x9a3d94, self.opponent_goals, self.own_goals); // "<goals>-<goals>"
        }
        if matches!(middle, TieMiddle::Trophy | TieMiddle::TrophyRoundGender) {
            let name = comp.name_addr();
            let mut k = 0;
            loop {
                let c = read_u8(name + k);
                self.label[k] = c;
                if c == 0 {
                    break;
                }
                k += 1;
            }
        }
    }

    /// `say` with the venue when the fixture has a stadium (`with_venue`), else without it (`without`).
    unsafe fn say_at(&mut self, with_venue: usize, without: usize, middle: TieMiddle, score: TieScore) {
        if !self.stadium.is_null() {
            self.say(with_venue, middle, score, true)
        } else {
            self.say(without, middle, score, false)
        }
    }

    /// The forfeit lost: "have lost in the <round> by being the lower seeded team" (0x99ecec when the opponent's mark
    /// is below the club's, else 0x99ec54).
    unsafe fn say_forfeit_lost(&mut self, own_mark: i8, opponent_mark: i8) {
        covered!("gra_24 forfeit lost");
        let format = if opponent_mark < own_mark { 0x99ecec } else { 0x99ec54 };
        self.say(format, TieMiddle::Round, TieScore::Nothing, false)
    }

    /// The defeats on a settling mark, shared by the three kinds of sentence: away goals, the lower seed, extra time
    /// (the opponent's goals first), penalties, the shootout and the golden goal, with and without the venue.
    unsafe fn say_mark_defeat(&mut self) {
        use TieMiddle::Round;
        use TieScore::{OpponentFirst, OwnFirst};
        match self.mark {
            tie_mark::AWAY_GOALS => self.say_at(0x9a0cb4, 0x9a2294, Round, OwnFirst),
            tie_mark::HIGHER_SEED => self.say_at(0x9a0a60, 0x9a2070, Round, OwnFirst),
            tie_mark::EXTRA_TIME => self.say_at(0x9a3a0c, 0x9a3964, Round, OpponentFirst),
            tie_mark::PENALTIES => self.say_at(0x9a3754, 0x9a1d40, Round, OwnFirst),
            tie_mark::SHOOTOUT => self.say_at(0x9a3544, 0x9a1b5c, Round, OwnFirst),
            _ => self.say_at(0x99fea4, 0x9a1954, Round, OwnFirst), // the golden goal
        }
    }

    /// An ordinary defeat by the verdict: disappointing (0x9a2b50 with the venue in a single match, else 0x9a2ab0),
    /// very disappointing (0x9a29f0 / 0x9a294c), any other verdict (single match 0x9a2630 / 0x9a25a0, two legs
    /// 0x9a26dc); the opponent's goals first. A shock defeat is written by the caller; in a round with no stage after
    /// it the other verdicts have their own two-legs sentence with the venue (`say_round_result`).
    unsafe fn say_verdict_defeat(&mut self) {
        use TieMiddle::Round;
        use TieScore::OpponentFirst;
        let with_venue = self.single_match && !self.stadium.is_null();
        match self.verdict {
            tie_verdict::DISAPPOINTING => self.say(if with_venue { 0x9a2b50 } else { 0x9a2ab0 }, Round, OpponentFirst, with_venue),
            tie_verdict::VERY_DISAPPOINTING => self.say(if with_venue { 0x9a29f0 } else { 0x9a294c }, Round, OpponentFirst, with_venue),
            _ if self.single_match => self.say_at(0x9a2630, 0x9a25a0, Round, OpponentFirst),
            _ => self.say(0x9a26dc, Round, OpponentFirst, false),
        }
    }

    /// The tie decided in a round with no stage after it named: "have won in the <round>" or, in the final, "have won
    /// the <competition>", or the defeat (block of `comp_util_write_cup_tie_result_news` for a next stage below 1 other
    /// than −2).
    unsafe fn say_round_result(&mut self, own_mark: i8, opponent_mark: i8) {
        use TieMiddle::{Round, Trophy, TrophyKeepLabel, TrophyRoundGender};
        use TieScore::{OpponentFirst, OwnFirst};
        covered!("gra_24 round result");
        if own_mark < 0 || opponent_mark < 0 {
            if !self.won {
                return self.say_forfeit_lost(own_mark, opponent_mark);
            }
            covered!("gra_24 forfeit won");
            let club_first = own_mark < opponent_mark;
            if self.final_round {
                return self.say(if club_first { 0x99eef8 } else { 0x99ee60 }, TrophyKeepLabel, TieScore::Nothing, false);
            }
            return self.say(if club_first { 0x99edec } else { 0x99ed54 }, Round, TieScore::Nothing, false);
        }
        // The wins on a settling mark: (in a round with/without the venue, in the final with/without the venue).
        let mark_win = match self.mark {
            tie_mark::AWAY_GOALS => Some((0x9a0e20, 0x9a0d78, 0x9a0f8c, 0x9a0ee4)),
            tie_mark::EXTRA_TIME => Some((0x9a0840, 0x9a079c, 0x9a09a0, 0x9a08fc)),
            tie_mark::PENALTIES => Some((0x9a058c, 0x9a04f0, 0x9a06e4, 0x9a0644)),
            tie_mark::SHOOTOUT => Some((0x9a02e0, 0x9a0244, 0x9a0438, 0x9a0398)),
            tie_mark::GOLDEN_GOAL => Some((0x9a0010, 0x99ff68, 0x9a0180, 0x9a00d4)),
            _ => None,
        };
        if self.mark == tie_mark::HIGHER_SEED {
            if self.won {
                return self.say_at(0x9a0be4, 0x9a0b30, Round, OwnFirst); // the higher seed wins the round, even a final
            }
            return self.say_mark_defeat();
        }
        if let Some((round, round_plain, trophy, trophy_plain)) = mark_win {
            if !self.won {
                return self.say_mark_defeat();
            }
            if !self.final_round {
                return self.say_at(round, round_plain, Round, OwnFirst);
            }
            covered!("gra_24 trophy won on a mark");
            return self.say_at(trophy, trophy_plain, Trophy, OwnFirst);
        }
        match self.verdict {
            tie_verdict::SHOCK if !self.won => {
                if self.single_match {
                    self.say_at(0x9a1638, 0x9a15a0, Round, OpponentFirst)
                } else {
                    self.say(0x9a16ec, Round, OpponentFirst, false)
                }
            }
            // A shock win, an excellent and a fine result (these two whatever the outcome): (single match in a round
            // with/without the venue, in the final with/without the venue; two legs in a round, in the final).
            tie_verdict::SHOCK | tie_verdict::EXCELLENT | tie_verdict::FINE => {
                let (round, round_plain, trophy, trophy_plain, legs, legs_trophy) = match self.verdict {
                    tie_verdict::SHOCK => (0x99fb6c, 0x99fad8, 0x99fcb4, 0x99fc1c, 0x99fd64, 0x99fe04),
                    tie_verdict::EXCELLENT => (0x99f788, 0x99f6f0, 0x99f8d8, 0x99f83c, 0x99f990, 0x99fa34),
                    _ => (0x99f3bc, 0x9a2cb4, 0x99f500, 0x99f46c, 0x99f5b0, 0x99f650),
                };
                match (self.single_match, self.final_round) {
                    (true, false) => self.say_at(round, round_plain, Round, OwnFirst),
                    (true, true) => self.say_at(trophy, trophy_plain, Trophy, OwnFirst),
                    (false, false) => self.say(legs, Round, OwnFirst, false),
                    (false, true) => self.say(legs_trophy, Trophy, OwnFirst, false),
                }
            }
            tie_verdict::DISAPPOINTING | tie_verdict::VERY_DISAPPOINTING => self.say_verdict_defeat(),
            _ if self.won => match (self.single_match, self.final_round) {
                (true, false) => self.say_at(0x99f0b0, 0x99f020, Round, OwnFirst), // gra24: the plain win in a round
                (true, true) => self.say_at(0x99f1e4, 0x99f158, TrophyRoundGender, OwnFirst),
                (false, false) => self.say(0x99f28c, Round, OwnFirst, false),
                (false, true) => self.say(0x99f324, Trophy, OwnFirst, false),
            },
            _ if self.single_match => self.say_at(0x9a2630, 0x9a25a0, Round, OpponentFirst),
            _ => self.say_at(0x99ef6c, 0x9a26dc, Round, OpponentFirst), // only here two legs have a venue variant
        }
    }

    /// The tie that sends the club "through to the next stage" without naming it, or the defeat (next stage −2).
    unsafe fn say_through_unnamed(&mut self) {
        use TieMiddle::{Nothing, Round};
        use TieScore::{OpponentFirst, OwnFirst};
        covered!("gra_24 through to the next stage");
        // The wins on a settling mark (with/without the venue).
        let mark_win = match self.mark {
            tie_mark::AWAY_GOALS => Some((0x9a23dc, 0x9a233c)),
            tie_mark::HIGHER_SEED => Some((0x9a21d0, 0x9a2124)),
            tie_mark::EXTRA_TIME => Some((0x9a1fbc, 0x9a1f24)),
            tie_mark::PENALTIES => Some((0x9a1e74, 0x9a1de0)),
            tie_mark::SHOOTOUT => Some((0x9a1c90, 0x9a1bfc)),
            tie_mark::GOLDEN_GOAL => Some((0x9a1aa0, 0x9a1a00)),
            _ => None,
        };
        if let Some((with_venue, plain)) = mark_win {
            if self.won {
                return self.say_at(with_venue, plain, Nothing, OwnFirst);
            }
            return self.say_mark_defeat();
        }
        match self.verdict {
            tie_verdict::SHOCK if !self.won => {
                if self.single_match {
                    self.say_at(0x9a1638, 0x9a15a0, Round, OpponentFirst)
                } else {
                    self.say(0x9a16ec, Round, OpponentFirst, false)
                }
            }
            // A shock win, an excellent and a fine result: (single match with/without the venue, two legs).
            tie_verdict::SHOCK | tie_verdict::EXCELLENT | tie_verdict::FINE => {
                let (with_venue, plain, legs) = match self.verdict {
                    tie_verdict::SHOCK => (0x9a1818, 0x9a178c, 0x9a18c0),
                    tie_verdict::EXCELLENT => (0x9a1458, 0x9a13c8, 0x9a1504),
                    _ => (0x9a1290, 0x9a1204, 0x9a1334),
                };
                if self.single_match {
                    self.say_at(with_venue, plain, Nothing, OwnFirst)
                } else {
                    self.say(legs, Nothing, OwnFirst, false)
                }
            }
            tie_verdict::DISAPPOINTING | tie_verdict::VERY_DISAPPOINTING => self.say_verdict_defeat(),
            _ if self.won => {
                if self.single_match {
                    self.say_at(0x9a10d4, 0x9a1050, Nothing, OwnFirst)
                } else {
                    self.say(0x9a1174, Nothing, OwnFirst, false)
                }
            }
            _ => self.say_verdict_defeat(),
        }
    }

    /// The tie that sends the club through to the named next stage, or the defeat (next stage 1 or above).
    unsafe fn say_through_to_stage(&mut self, own_mark: i8, opponent_mark: i8) {
        use TieMiddle::{NextStage, Round};
        use TieScore::{OpponentFirst, OwnFirst};
        covered!("gra_24 through to a named stage");
        if own_mark < 0 || opponent_mark < 0 {
            if !self.won {
                return self.say_forfeit_lost(own_mark, opponent_mark);
            }
            covered!("gra_24 forfeit through");
            let format = if own_mark < opponent_mark { 0x9a252c } else { 0x9a2494 };
            return self.say(format, NextStage, TieScore::Nothing, false);
        }
        match self.mark {
            // Away goals and the higher seed: no venue variant; the defeat is the plain sentence.
            tie_mark::AWAY_GOALS => {
                if self.won {
                    self.say(0x9a3ce4, NextStage, OwnFirst, false)
                } else {
                    self.say(0x9a2294, Round, OwnFirst, false)
                }
            }
            tie_mark::HIGHER_SEED => {
                if self.won {
                    self.say(0x9a3c30, NextStage, OwnFirst, false)
                } else {
                    self.say(0x9a2070, Round, OwnFirst, false)
                }
            }
            tie_mark::EXTRA_TIME | tie_mark::PENALTIES | tie_mark::SHOOTOUT | tie_mark::GOLDEN_GOAL => {
                if !self.won {
                    return self.say_mark_defeat();
                }
                let (with_venue, plain) = match self.mark {
                    tie_mark::EXTRA_TIME => (0x9a3b70, 0x9a3acc),
                    tie_mark::PENALTIES => (0x9a38ac, 0x9a380c),
                    tie_mark::SHOOTOUT => (0x9a369c, 0x9a35fc),
                    _ => (0x9a3480, 0x9a33d4),
                };
                self.say_at(with_venue, plain, NextStage, OwnFirst)
            }
            _ => match self.verdict {
                tie_verdict::SHOCK if self.won => {
                    if self.single_match {
                        self.say_at(0x9a3284, 0x9a31ec, NextStage, OwnFirst)
                    } else {
                        self.say(0x9a3334, NextStage, OwnFirst, false)
                    }
                }
                tie_verdict::SHOCK => {
                    if self.single_match {
                        self.say_at(0x9a308c, 0x9a2fe0, Round, OpponentFirst)
                    } else {
                        self.say(0x9a3148, Round, OpponentFirst, false)
                    }
                }
                // An excellent and a fine result, whatever the outcome: a single match is "have won in the <round>",
                // two legs "are through to <stage>".
                tie_verdict::EXCELLENT => {
                    if self.single_match {
                        self.say_at(0x9a2e84, 0x9a2de8, Round, OwnFirst)
                    } else {
                        self.say(0x9a2f38, NextStage, OwnFirst, false)
                    }
                }
                tie_verdict::FINE => {
                    if self.single_match {
                        self.say_at(0x99f3bc, 0x9a2cb4, Round, OwnFirst)
                    } else {
                        self.say(0x9a2d48, NextStage, OwnFirst, false)
                    }
                }
                tie_verdict::DISAPPOINTING if !self.single_match => self.say(0x9a2c0c, Round, OpponentFirst, false),
                tie_verdict::DISAPPOINTING | tie_verdict::VERY_DISAPPOINTING => self.say_verdict_defeat(),
                _ if self.won => {
                    if self.single_match {
                        self.say_at(0x9a2808, 0x9a2778, NextStage, OwnFirst)
                    } else {
                        self.say(0x9a28b4, NextStage, OwnFirst, false)
                    }
                }
                _ => self.say_verdict_defeat(),
            },
        }
    }
}

/// Writes the news sentence of a club's cup tie result (won, through, lost) into the Text `text`, with the score,
/// the round, the venue and the links.
///
/// `comp_util_write_cup_tie_result_news(club_index, fixture, verdict, outcome, next_stage, text)` (cdecl). The club is
/// `club_at(club_index)`; at home (`club_index` = `.home_club_id()`) the opponent is `.away_club()`, the club's mark
/// `.home_result_mark()` and the opponent's `.away_result_mark()`, away the other way round; the settling mark is the
/// home mark, else the away mark. The goals: the aggregate (`.home_aggregate()` other than −1), else after extra time
/// (`.home_goals_extra_time()` other than −1), else at full time, the club's first; the score text "<goals>-<goals>"
/// (format 0x9a3d94, 16 bytes). Won: `outcome` (byte) other than 2, 4 and −1. The round: `.round_name()` without its
/// variant (`round_name_without_variant`); the final is round 0x96 with no `.round_prefix()`.
///
/// The round label (form 1, 500 bytes, `exe::comp_util_format_comp_stage_round_label`): in the World Cup Oceania
/// qualifying section with a club (home, then away) whose nation's continent is not Oceania, and the same for the Asian
/// section and Asia, the label of the FIFA World Cup row with round 0xa0 and its gender; otherwise the competition's,
/// stage and round label, gendered by the raw round name (`exe::comp_util_get_stage_name_gender`), else the stage,
/// else the competition's gender (`.gender_name()`), else 1. With a stadium (`.stadium()`), the venue: its name, or
/// "<name>, <city>" (0x98f6ac) in languages 0 and 1 and "<name> (<city>)" (0x9a3d8c) in the others
/// (`exe::langlib_get_language`; `FN_CRT_SPRINTF`).
///
/// By `next_stage` (word): 1 or above names the next stage (`exe::comp_util_get_stage_name`, 100 bytes, and its gender;
/// `TieNews::say_through_to_stage`); −2 "the next stage" (`TieNews::say_through_unnamed`); otherwise the round or the
/// trophy (`TieNews::say_round_result`). Each picks the sentence by the mark, then the verdict (`tie_verdict`), the
/// outcome, the final, a single match (`.home_aggregate()` −1) and the stadium (`TieNews::say`).
///
/// Then the text is set (`exe::text_set_string`) and linked: the club's and the opponent's short names (0x7d5); the
/// score text (0x7d3, the match statistics slot) when the statistics entry is the fixture's
/// (`exe::match_stats_fixture_entry_matches`: slot, home and away ids, day, year); with the competition current on the
/// fixture's date (`is_comp_current_on_date` of `G_FRIENDLY`) the label (2000, the competition), and the next stage's
/// name when there is one (2000). The next stage's buffer starts with the byte at 0xad9c64.
/// Original: `comp_util_write_cup_tie_result_news` (0x4b0b80, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_write_cup_tie_result_news(club_index: i32, fixture: usize, verdict: u32, outcome: u32, next_stage: u32, text: usize) {
    covered!("gra_24 cup tie news");
    let f = Fixture(fixture);
    let club = club_at(club_index as isize);
    let (home_mark, away_mark) = (f.home_result_mark(), f.away_result_mark());
    let mark = if home_mark != 0 { home_mark } else { away_mark };
    let at_home = club_index as usize == f.home_club_id();
    let (opponent, own_mark, opponent_mark) = if at_home { (f.away_club(), home_mark, away_mark) } else { (f.home_club(), away_mark, home_mark) };
    let (home_goals, away_goals) = if f.home_aggregate() != -1 {
        (f.home_aggregate(), f.away_aggregate())
    } else if f.home_goals_extra_time() != -1 {
        (f.home_goals_extra_time(), f.away_goals_extra_time())
    } else {
        (f.home_goals(), f.away_goals())
    };
    let (own_goals, opponent_goals) = if at_home { (home_goals as i32, away_goals as i32) } else { (away_goals as i32, home_goals as i32) };
    let mut news = TieNews {
        club,
        opponent,
        comp: f.competition(),
        stadium: Stadium(f.stadium()),
        own_goals,
        opponent_goals,
        label_gender: 0,
        next_gender: 0,
        won: !matches!(outcome as u8, 2 | 4 | 0xff),
        final_round: false,
        single_match: f.home_aggregate() == -1,
        mark,
        verdict: verdict as u8,
        label: [0; 500],
        next: [0; 100],
        venue: [0; 300],
        score: [0; 16],
    };
    news.next[0] = read_u8(0xad9c64);
    let fmt = news_text_format();
    fmt(news.score.as_mut_ptr() as usize, 0x9a3d94, own_goals, opponent_goals);
    let raw_round = f.round_name();
    let round = round_name_without_variant(raw_round);
    let prefix = f.round_prefix();
    news.final_round = round == ROUND_FINAL && prefix == 0;
    let comp = news.comp;
    let outside = |continent_key: usize| {
        let away_from = |c: Club| {
            let nation = c.nation();
            if nation.is_null() {
                return false;
            }
            let continent = nation.continent();
            continent != 0 && read_i32(continent) != read_i32(continent_key)
        };
        away_from(f.home_club()) || away_from(f.away_club())
    };
    let comp_id = f.comp_id();
    let world_cup_label = if comp_id == read_i32(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION) {
        outside(KEY_CONTINENT_OCEANIA)
    } else if comp_id == read_i32(KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION) {
        outside(KEY_CONTINENT_ASIA)
    } else {
        false
    };
    let label = news.label.as_mut_ptr();
    if world_cup_label {
        covered!("gra_24 world cup qualifying label");
        let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_i32(KEY_CLUB_COMP_FIFA_WORLD_CUP) as usize).wrapping_mul(0x6b));
        exe::comp_util_format_comp_stage_round_label(world_cup, 0, 0xa0, 1, label, 500);
        news.label_gender = exe::comp_util_get_stage_name_gender(0xa0, 0) as i8 as i32;
    } else {
        exe::comp_util_format_comp_stage_round_label(comp, prefix as u16 as u32, round as u16 as u32, 1, label, 500);
        let gender = if raw_round != 0 {
            exe::comp_util_get_stage_name_gender(raw_round as u16 as u32, 0)
        } else if prefix != 0 {
            exe::comp_util_get_stage_name_gender(prefix as u16 as u32, 0)
        } else if comp.is_null() {
            1
        } else {
            comp.gender_name()
        };
        news.label_gender = gender as i8 as i32;
    }
    let stadium = news.stadium;
    if !stadium.is_null() {
        let lang = exe::langlib_get_language();
        let format = if lang == 0 || exe::langlib_get_language() == 1 { 0x98f6ac } else { 0x9a3d8c }; // "%s, %s" / "%s (%s)"
        let city = stadium.city();
        if city == 0 {
            let name = stadium.name_addr();
            let mut k = 0;
            loop {
                let c = read_u8(name + k);
                news.venue[k] = c;
                if c == 0 {
                    break;
                }
                k += 1;
            }
        } else {
            covered!("gra_24 venue with the city");
            let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
            sprintf(news.venue.as_mut_ptr() as usize, format, stadium.name_addr(), city + 4); // the city's name
        }
    }
    let stage = next_stage as u16 as i16;
    if stage >= 1 {
        exe::comp_util_get_stage_name(news.next.as_mut_ptr(), 100, next_stage, 0);
        news.next_gender = exe::comp_util_get_stage_name_gender(next_stage, 0) as i8 as i32;
        news.say_through_to_stage(own_mark, opponent_mark);
    } else if stage == -2 {
        news.say_through_unnamed();
    } else {
        news.say_round_result(own_mark, opponent_mark);
    }
    exe::text_set_string(text, NEWS_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
    exe::text_add_link(text, (opponent.addr() + club::SHORT_NAME) as *const u8, 0x7d5, opponent); // gra24: the opponent's link
    let slot = f.match_stats_slot();
    if exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), slot as i32, f.home_club_id() as i32, f.away_club_id() as i32, f.day() as i16 as i32, f.year() as i16 as i32) != 0 {
        exe::text_add_link(text, news.score.as_ptr(), 0x7d3, slot);
    }
    let mut date = [0u8; 8];
    exe::new_date(date.as_mut_ptr() as usize, f.day() as u32, f.year() as u32);
    if is_comp_current_on_date(read_usize(G_FRIENDLY), comp.addr(), date.as_ptr() as usize) != 0 {
        exe::text_add_link(text, news.label.as_ptr(), 2000, comp);
        if news.next[0] != 0 {
            exe::text_add_link(text, news.next.as_ptr(), 2000, comp);
        }
    }
}

/// Comparer for `comp_util_write_cup_tie_result_news`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "C" fn compare_comp_util_write_cup_tie_result_news(club_index: i32, fixture: usize, verdict: u32, outcome: u32, next_stage: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(i32, usize, u32, u32, u32, usize)>(IDX_COMP_UTIL_WRITE_CUP_TIE_RESULT_NEWS);
    compare_text_writer(text, || o(club_index, fixture, verdict, outcome, next_stage, text),
        |t| comp_util_write_cup_tie_result_news(club_index, fixture, verdict, outcome, next_stage, t),
        || format!("comp_util_write_cup_tie_result_news(club {}, fixture {:#x}, verdict {}, outcome {}, next stage {})", club_index, fixture, verdict as u8, outcome as u8 as i8, next_stage as u16 as i16));
}

// ---------- batch 324: the news text of a broken club record (club_records.cpp) ----------

/// The constructors the original runs over its local copy of the broken record (0x38b bytes, the `club_record`
/// layout), in its order: the match records (0x645800: only their Date), the four run records, the tallies and the
/// counts, the goals-in-a-match and rating records, the youngest and oldest player records, the two transfer fee
/// records and the fans' player of the year. All thiscall, returning `this`.
const RECORD_NEWS_CONSTRUCTORS: [(usize, usize); 27] = [
    (0x12, 0x645800), (0x35, 0x645800), (0x58, 0x645800), (0x7d, 0x645800), (0x9c, 0x645800), (0xbb, 0x645800),
    (0xda, 0x645800), (0xf9, 0x645800),
    (0x118, 0x450ad0), (0x12c, 0x450b20), (0x140, 0x450b70), (0x154, 0x450bc0),
    (0x168, 0x450c60), (0x183, 0x450c60), (0x19e, 0x450c10), (0x1d6, 0x450c60), (0x1f1, 0x453c50), (0x210, 0x450c60),
    (0x22b, 0x450d30), (0x247, 0x450d30), (0x261, 0x450d30), (0x27b, 0x450c70), (0x2ba, 0x450cd0), (0x2f9, 0x450d30),
    (0x313, 0x4485d0), (0x33c, 0x448620), (0x371, 0x450d40),
];

/// `read_run_record_from_news(item, &slot, out)` (0x7e5570, record_utils.cpp; cdecl): the inverse of 0x7e5420.
const FN_READ_RUN_RECORD_FROM_NEWS: usize = 0x7e5570;
/// `database_format_years_between_dates(end, start, text, size)` (0x540340, Database.cpp; cdecl).
const FN_DATABASE_FORMAT_YEARS_BETWEEN_DATES: usize = 0x540340;
/// The empty string the original sets the Text to first (0xad9c64).
const EMPTY_STRING: usize = 0xad9c64;

/// A match record inside the local record copy (`read_match_record_from_news`, 0x1f bytes): the date (+0 day, +2
/// year), the club competition (+8), the round and stage words (+0xc, +0xe), the venue (+0x10: 1 at home, 2 away),
/// the club (+0x11) and its opponent (+0x15), their goals (+0x19, +0x1a) and the fixture index (+0x1b).
#[derive(Clone, Copy)]
struct RecordMatch(usize);

impl RecordMatch {
    unsafe fn comp_index(self) -> i32 {
        read_i32(self.0 + 8)
    }
    unsafe fn comp(self) -> usize {
        read_usize(G_CLUB_COMP_TABLE).wrapping_add((self.comp_index() as usize).wrapping_mul(0x6b))
    }
    unsafe fn venue(self) -> u8 {
        read_u8(self.0 + 0x10)
    }
    unsafe fn opponent(self) -> Club {
        club_at(read_i32(self.0 + 0x15) as isize)
    }
    unsafe fn goals(self) -> (i8, i8) {
        (read_i8(self.0 + 0x19), read_i8(self.0 + 0x1a))
    }
    /// The round label (`exe::comp_util_format_comp_stage_round_label`, form 2, 200 bytes) and its gender
    /// (`exe::comp_util_get_label_gender`).
    unsafe fn round_label(self, label: &mut [u8; 0xc8]) -> i32 {
        let (round, stage) = (read_u32(self.0 + 0xc), read_u32(self.0 + 0xe));
        exe::comp_util_format_comp_stage_round_label(self.comp(), round, stage, 2, label.as_mut_ptr(), 0xc8);
        exe::comp_util_get_label_gender(self.comp() as u32, round, stage, 2) as i8 as i32
    }
    /// The score's link (0x7d3) when the fixture still matches (`exe::match_stats_fixture_entry_matches`).
    unsafe fn link_score(self, out: usize, score: &[u8]) {
        let (day, year) = (read_u16(self.0) as i32, read_u16(self.0 + 2) as i32);
        let fixture = read_i32(self.0 + 0x1b);
        if exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), fixture, read_i32(self.0 + 0x11), read_i32(self.0 + 0x15), day, year) != 0 {
            exe::text_add_link(out, score.as_ptr(), 0x7d3, fixture as u32 as usize);
        }
    }
}

/// A record holder's name reference (0x18 bytes: three name ids, the person's id at +0xc, his birth date at +0x10).
#[derive(Clone, Copy)]
struct RecordHolder(usize);

impl RecordHolder {
    /// The holder's name (`exe::database_format_name_from_ids`, mode 1, 200 bytes).
    unsafe fn name(self, buf: &mut [u8; 0xc8]) {
        exe::database_format_name_from_ids(read_u32(self.0), read_u32(self.0 + 4), read_u32(self.0 + 8), buf.as_mut_ptr(), 0xc8, 1, read_u32(self.0 + 0xc));
    }
    /// The person's link (0x7d7) when his names still match (`exe::database_staff_matches_names`).
    unsafe fn link(self, out: usize, name: &[u8; 0xc8]) {
        let staff = staff_at(read_i32(self.0 + 0xc) as isize);
        if exe::database_staff_matches_names(staff, read_u32(self.0), read_u32(self.0 + 4), read_u32(self.0 + 8), self.0 + 0x10) != 0 {
            exe::text_add_link(out, name.as_ptr(), 0x7d7, staff);
        }
    }
}

/// A sum of money as the game shows it (`Cash::format`, 0x450aa0: `exe::cash_get_display_amount` with `flags`, then
/// `exe::cash_format_money` into 200 bytes).
unsafe fn record_money_text(value: u32, flags: u32, buf: &mut [u8; 0xc8]) {
    let cash = [value];
    exe::cash_format_money(buf.as_mut_ptr() as usize, 0xc8, exe::cash_get_display_amount(cash.as_ptr(), flags), flags);
}

/// The club's link (0x7d5), under the given name.
unsafe fn link_record_club(out: usize, name: usize, club: Club) {
    exe::text_add_link(out, name as *const u8, 0x7d5, club); // gra18: the club's link
}

/// Reads the broken record of type `kind` from the news item's slots into the local record copy `r`: the inverse of
/// `club_records_write_record_news_fields` (`record_news_fields`): bytes, words and dwords stored at their offsets, the
/// rating (type 20) as the slot × 0.01 in a float, the match, holder and run records by their readers; types 25 and 26
/// read the day and the year of the date at +0x27b / +0x2ba into a Date (`exe::new_date`, then
/// `exe::date_operator_3d`). Returns false for a type outside 1..=32.
unsafe fn read_record_news_fields(news: usize, kind: u8, r: usize, k: &mut u8) -> bool {
    use RecordNewsField::*;
    let fields = record_news_fields(kind);
    if fields.is_empty() {
        return false;
    }
    let slot = |k: &mut u8| {
        let v = exe::newsitem_get_slot(news, *k as u32);
        *k = k.wrapping_add(1);
        v
    };
    let mut rest = fields;
    if kind == 25 || kind == 26 {
        covered!("gra_18 record date");
        let base = if kind == 25 { 0x27b } else { 0x2ba };
        let (day, year) = (slot(k), slot(k));
        let mut date = [0u8; 8];
        let mut scratch = [0u8; 8];
        exe::new_date(date.as_mut_ptr() as usize, day, year);
        exe::date_operator_3d(r + base, scratch.as_mut_ptr() as usize, date.as_ptr() as usize);
        rest = &fields[2..];
    }
    let run: unsafe extern "C" fn(usize, *mut u8, usize) -> u32 = core::mem::transmute(FN_READ_RUN_RECORD_FROM_NEWS);
    for f in rest {
        match *f {
            Byte(o) | SByte(o) => write_u8(r + o, slot(k) as u8),
            Word(o) | SWord(o) => write_u16(r + o, slot(k) as u16),
            Dword(o) => write_u32(r + o, slot(k)),
            Percent(o) => write_f32(r + o, (slot(k) as i32 as f64 * read_f64(rdata::F64_0_01)) as f32),
            Match(o) => {
                exe::read_match_record_from_news(news, k, (r + o) as *mut u8);
            }
            Holder(o) => {
                exe::read_staff_name_ref_from_news(news, k, (r + o) as *mut u8);
            }
            Run(o) => {
                run(news, k, r + o);
            }
        }
    }
    true
}

/// Writes the headline or the body of the news that a club broke one of its records (news type 0x3e8, from
/// `send_record_news`).
///
/// `write_record_news_text(news, full, out)`. Without the Text, the error box (club_records.cpp, line 0xee7) and 0;
/// the Text is cleared and set empty; without the news item, the box (0xef3) and 0. The local record copy is
/// constructed as the original does (`RECORD_NEWS_CONSTRUCTORS`); slot 0 is the club id, slot 1 the record type, then
/// the type's fields (`read_record_news_fields`; an unknown type: the box at line 0x1b02, and on). A club id outside
/// the club table: the box (0xefe) and 0.
///
/// The headline (`full` = 0): type 32 "<club> <Fans Player of the Year> decided" (0x98de00, the award name 0x98de58),
/// with the club's link and the award's link (0x7e6, to the club); otherwise "New <club> record" for a national side
/// (`exe::database_is_national_team`, 0x98dde0) or "New <club> club record" (0x98ddb8), with the club's link. 1.
///
/// The body, by type, each only when its record holds (otherwise an empty text), with the club's short name and gender
/// (`club::SHORT_NAME`, `club::SHORT_GENDER_NAME`) and its link:
/// - 1, 2: the final league position (the ordinal of position + 1, `exe::utils_format_ordinal`) in the competition
///   (its short name and gender), record high (0x98dd28) or low (0x98dc98), when the competition is set; the
///   competition's link (0x7d0);
/// - 3, 5: the attendance at home to the opponent in the round (the label of form 2 and its gender), record high
///   (0x98dbf4) or low (0x98da98); 4: the gate receipts (`record_money_text`, flags 0x22, 0x98db3c); the opponent's
///   link and the round label's link (0x7d0, to the competition); 6: the average attendance (0x98da2c);
/// - 7, 8, 9: the record win, league win and defeat: the score "%d-%d" (the C sprintf; the club's goals first, in the
///   defeat the opponent's first), the template by the venue (at home, away, against); 10, 11: the highest scoring
///   game and league game, the score with the higher count first, the template by the result (win, defeat, draw);
///   the links of the opponent, the round and the score (0x7d3, when the fixture still matches);
/// - 12, 13: the runs of wins and defeats (more than one game) between the start and the end dates
///   (`exe::date_get_long_date_string_no_year`); 14, 15: unbeaten and without winning, from the start date;
/// - 16, 17: a player's goals and league goals in the season; 18: the goals in one match (with the match: venue,
///   opponent, round); 19: assists; 20: the average rating (above −1.0, "%4.2f" of the float) with the games; 21: man
///   of the match awards; 22: yellow and red cards; 23, 24: league appearances and goals in his club career; 25, 26:
///   the youngest and oldest appearance (the age by `database_format_years_between_dates` from the birth date to
///   the match, the template by the venue); 27: the most caps for the nation; 28, 29: the record purchase and sale
///   (the fee with flags 6, the other club, "part-exchange" when the flag is set); 32: the fans' player of the year
///   (the club's long name and gender, "Player of the Year" 0x98c380 with its link 0x7e6). The player's name (mode 1)
///   and his link (0x7d7) when his names still match;
/// - 30, 31: the season's transfer spending and income (flags 6).
///
/// An unknown type: the box (line 0x14a7) and 0; otherwise 1.
/// Original: `club_records_write_record_news_text` (0x44c650, club_records.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_record_news_text(_this: usize, news: usize, full: u32, out: usize) -> u32 {
    use record_news_fns::CLUB_RECORDS_CPP as FILE;
    let fmt = news_text_format();
    if out == 0 {
        show_error_at(FILE, 0xee7);
        return 0;
    }
    let clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4afa40usize); // Text::clear
    clear(out);
    exe::text_set_string(out, EMPTY_STRING as *const u8);
    if news == 0 {
        show_error_at(FILE, 0xef3);
        return 0;
    }
    covered!("gra_18 record news");
    let mut record = [0u8; 0x390];
    let r = record.as_mut_ptr() as usize;
    for (o, f) in RECORD_NEWS_CONSTRUCTORS {
        let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(f);
        construct(r + o);
    }
    let mut k: u8 = 1;
    let id = exe::newsitem_get_slot(news, 0) as i32;
    let kind = exe::newsitem_get_slot(news, k as u32) as u8;
    k += 1;
    let known = read_record_news_fields(news, kind, r, &mut k);
    if !known {
        show_error_at(FILE, 0x1b02);
    }
    if id < 0 || id > read_i32(G_CLUB_COUNT) - 1 {
        show_error_at(FILE, 0xefe);
        return 0;
    }
    let club = club_at(id as isize);
    let name = club.addr() + club::SHORT_NAME;
    let gender = read_i8(club.addr() + club::SHORT_GENDER_NAME) as i32;
    if full == 0 {
        if kind == 0x20 {
            let mut award = [0u8; 0x100];
            fmt(award.as_mut_ptr() as usize, 0x98de58); // "Fans Player of the Year"
            fmt(NEWS_TEXT_BUFFER, 0x98de00, gender, gender, name, award.as_ptr() as usize);
            exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
            link_record_club(out, name, club);
            exe::text_add_link(out, award.as_ptr(), 0x7e6, club);
        } else {
            let headline = if exe::database_is_national_team(club) != 0 { 0x98dde0 } else { 0x98ddb8 };
            fmt(NEWS_TEXT_BUFFER, headline, gender, gender, name);
            exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
            link_record_club(out, name, club);
        }
        return 1;
    }
    if !known {
        show_error_at(FILE, 0x14a7);
        return 0;
    }
    covered!("gra_18 record news body");
    let mut text = [0u8; 1000];
    let tp = text.as_mut_ptr() as usize;
    let set = |out: usize| {
        exe::text_set_string(out, tp as *const u8);
    };
    let (mut b1, mut b2, mut b3) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
    let mut score = [0u8; 20];
    let sp = score.as_mut_ptr() as usize;
    let crt_format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let comp_at = |idx: i32| read_usize(G_CLUB_COMP_TABLE).wrapping_add((idx as usize).wrapping_mul(0x6b));
    match kind {
        // the final league position, high and low
        1 | 2 => {
            let (pos, idx) = if kind == 1 { (read_u8(r), read_i32(r + 1)) } else { (read_u8(r + 7), read_i32(r + 8)) };
            if idx > -1 {
                let ordinal = exe::utils_format_ordinal(pos as i32 + 1, -1); // ponytail: the original copies it into 20 bytes first
                let comp = comp_at(idx);
                let cg = read_i8(comp + competition::SHORT_GENDER_NAME) as i32;
                let template = if kind == 1 { 0x98dd28 } else { 0x98dc98 };
                fmt(tp, template, gender, gender, cg, cg, name, ordinal, comp + competition::SHORT_NAME);
                set(out);
                link_record_club(out, name, club);
                exe::text_add_link(out, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
            }
        }
        // the attendance high and low and the gate receipts, at home to the opponent in a round
        3 | 4 | 5 => {
            let (value_at, m) = match kind { 3 => (0xe, 0x12), 4 => (0x31, 0x35), _ => (0x54, 0x58) };
            let value = read_i32(r + value_at);
            if value > -1 {
                let m = RecordMatch(r + m);
                let opp = m.opponent();
                let og = read_i8(opp.addr() + club::SHORT_GENDER_NAME) as i32;
                let rg = m.round_label(&mut b1);
                let oname = opp.addr() + club::SHORT_NAME;
                if kind == 4 {
                    record_money_text(value as u32, 0x22, &mut b2);
                    fmt(tp, 0x98db3c, gender, gender, og, og, rg, rg, name, b2.as_ptr() as usize, oname, b1.as_ptr() as usize);
                } else {
                    let template = if kind == 3 { 0x98dbf4 } else { 0x98da98 };
                    fmt(tp, template, gender, gender, og, og, rg, rg, name, value, oname, b1.as_ptr() as usize);
                }
                set(out);
                link_record_club(out, name, club);
                link_record_club(out, oname, opp);
                exe::text_add_link(out, b1.as_ptr(), 0x7d0, m.comp());
            }
        }
        // the average attendance
        6 => {
            let value = read_i32(r + 0x77);
            if value > -1 {
                fmt(tp, 0x98da2c, gender, gender, name, value);
                set(out);
                link_record_club(out, name, club);
            }
        }
        // the record win, league win and defeat, the highest scoring game and league game
        7..=11 => {
            let m = RecordMatch(r + [0x7d, 0x9c, 0xbb, 0xda, 0xf9][kind as usize - 7]);
            if m.comp_index() > -1 {
                let (g1, g2) = m.goals();
                let (a, b) = match kind {
                    9 => (g2, g1),
                    10 | 11 => (g1.max(g2), g1.min(g2)),
                    _ => (g1, g2),
                };
                crt_format(sp, 0x98c114, a as i32, b as i32); // "%d-%d"
                let opp = m.opponent();
                let og = read_i8(opp.addr() + club::SHORT_GENDER_NAME) as i32;
                let oname = opp.addr() + club::SHORT_NAME;
                let rg = m.round_label(&mut b1);
                let by_venue = |home: usize, away: usize, against: usize| match m.venue() {
                    1 => home,
                    2 => away,
                    _ => against,
                };
                let by_result = |win: usize, defeat: usize, draw: usize| if g1 > g2 { win } else if g1 < g2 { defeat } else { draw };
                let template = match kind {
                    7 => by_venue(0x98d998, 0x98d908, 0x98d878),
                    8 => by_venue(0x98d7dc, 0x98d740, 0x98d6a4),
                    9 => by_venue(0x98d608, 0x98d570, 0x98d4d8),
                    10 => by_result(0x98d438, 0x98d394, 0x98d2f8),
                    _ => by_result(0x98d250, 0x98d1a4, 0x98d0fc),
                };
                fmt(tp, template, gender, gender, og, og, rg, rg, name, sp, oname, b1.as_ptr() as usize);
                set(out);
                link_record_club(out, name, club);
                link_record_club(out, oname, opp);
                exe::text_add_link(out, b1.as_ptr(), 0x7d0, m.comp());
                m.link_score(out, &score);
            }
        }
        // the runs: won, lost (start and end dates), unbeaten, without winning (start date)
        12..=15 => {
            let run = r + [0x118, 0x12c, 0x140, 0x154][kind as usize - 12];
            let games = read_i32(run);
            if games > 1 {
                let start = run + run_record::START_DATE;
                if kind <= 13 {
                    exe::date_get_long_date_string_no_year(start, b1.as_mut_ptr() as usize, 0xc8, 0xff);
                    exe::date_get_long_date_string_no_year(run + run_record::END_DATE, b2.as_mut_ptr() as usize, 0xc8, 0xff);
                    let template = if kind == 12 { 0x98d074 } else { 0x98cfec };
                    fmt(tp, template, gender, gender, 0x64, 0x64, name, games, b1.as_ptr() as usize, b2.as_ptr() as usize);
                } else {
                    exe::date_get_long_date_string_no_year(start, b1.as_mut_ptr() as usize, 0xc8, 0xff);
                    let template = if kind == 14 { 0x98cf80 } else { 0x98cf0c };
                    fmt(tp, template, gender, gender, 0x64, name, games, b1.as_ptr() as usize);
                }
                set(out);
                link_record_club(out, name, club);
            }
        }
        // the player records with a count: goals, league goals, assists, man of the match, cards, career
        16 | 17 | 19 | 21 | 22 | 23 | 24 | 27 => {
            let (count, holder) = match kind {
                16 => (read_i8(r + 0x168) as i32, 0x169),
                17 => (read_i8(r + 0x183) as i32, 0x184),
                19 => (read_i8(r + 0x1d6) as i32, 0x1d7),
                21 => (read_i8(r + 0x210) as i32, 0x211),
                22 => (read_i8(r + 0x22b) as i32, 0x22d),
                23 => (read_i16(r + 0x247) as i32, 0x249),
                24 => (read_i16(r + 0x261) as i32, 0x263),
                _ => (read_i16(r + 0x2f9) as i32, 0x2fb),
            };
            if count > -1 {
                let h = RecordHolder(r + holder);
                h.name(&mut b1);
                let player = b1.as_ptr() as usize;
                match kind {
                    16 => fmt(tp, 0x98ceac, 0x64, gender, gender, player, count, name),
                    17 => fmt(tp, 0x98ce44, 0x64, gender, gender, player, count, name),
                    19 => fmt(tp, 0x98cbb4, 0x64, gender, gender, count, player, name),
                    21 => fmt(tp, 0x98ca88, 0x64, gender, gender, count, player, name),
                    22 => fmt(tp, 0x98c9fc, 0x64, gender, gender, player, count, read_i8(r + 0x22c) as i32, name),
                    23 => fmt(tp, 0x98c980, 0x64, gender, gender, count, player, name),
                    24 => fmt(tp, 0x98c90c, 0x64, gender, gender, count, player, name),
                    _ => fmt(tp, 0x98c668, gender, gender, player, name, count),
                };
                set(out);
                link_record_club(out, name, club);
                h.link(out, &b1);
            }
        }
        // the goals in one match
        18 => {
            let goals = read_i8(r + 0x19e) as i32;
            if goals > -1 {
                let h = RecordHolder(r + 0x19f);
                h.name(&mut b1);
                let m = RecordMatch(r + 0x1b7);
                let opp = m.opponent();
                let og = read_i8(opp.addr() + club::SHORT_GENDER_NAME) as i32;
                let oname = opp.addr() + club::SHORT_NAME;
                let rg = m.round_label(&mut b2);
                let template = match m.venue() { 1 => 0x98cd8c, 2 => 0x98ccd8, _ => 0x98cc24 };
                fmt(tp, template, 0x64, gender, gender, og, og, rg, rg, goals, b1.as_ptr() as usize, name, oname, b2.as_ptr() as usize);
                set(out);
                link_record_club(out, name, club);
                link_record_club(out, oname, opp);
                exe::text_add_link(out, b2.as_ptr(), 0x7d0, m.comp());
                h.link(out, &b1);
            }
        }
        // the average rating
        20 => {
            let rating = read_f32(r + club_record::SEASON_RATING);
            if rating as f64 > read_f64(rdata::F64_NEG_1) {
                let h = RecordHolder(r + 0x1f5);
                h.name(&mut b1);
                fmt(tp, 0x98cb10, 0x64, gender, gender, rating as f64, b1.as_ptr() as usize, read_i8(r + club_record::SEASON_RATING_GAMES) as i32, name);
                set(out);
                link_record_club(out, name, club);
                h.link(out, &b1);
            }
        }
        // the youngest and the oldest appearance
        25 | 26 => {
            let base = if kind == 25 { 0x27b } else { 0x2ba };
            let m = RecordMatch(r + base + 0x20);
            if m.comp_index() > -1 {
                let h = RecordHolder(r + base + 8);
                h.name(&mut b1);
                let years: unsafe extern "C" fn(usize, usize, usize, i32) -> u32 = core::mem::transmute(FN_DATABASE_FORMAT_YEARS_BETWEEN_DATES);
                years(m.0, r + base, sp, 5);
                let opp = m.opponent();
                let og = read_i8(opp.addr() + club::SHORT_GENDER_NAME) as i32;
                let oname = opp.addr() + club::SHORT_NAME;
                let rg = m.round_label(&mut b2);
                let template = match m.venue() { 1 => 0x98c850, 2 => 0x98c798, _ => 0x98c6e0 };
                fmt(tp, template, 0x64, gender, gender, og, og, rg, rg, b1.as_ptr() as usize, sp, name, oname, b2.as_ptr() as usize);
                set(out);
                link_record_club(out, name, club);
                link_record_club(out, oname, opp);
                exe::text_add_link(out, b2.as_ptr(), 0x7d0, m.comp());
                h.link(out, &b1);
            }
        }
        // the record purchase and sale
        28 | 29 => {
            let base = if kind == 28 { club_record::FEE_PAID } else { club_record::FEE_RECEIVED };
            let fee = read_i32(r + base);
            if fee > -1 {
                let h = RecordHolder(r + base + 4);
                h.name(&mut b1);
                let other = club_at(read_i32(r + base + 0x1c) as isize);
                let xg = read_i8(other.addr() + club::SHORT_GENDER_NAME) as i32;
                let xname = other.addr() + club::SHORT_NAME;
                record_money_text(fee as u32, 6, &mut b2);
                let exchange = read_u8(r + base + 0x28) != 0;
                let (money, player) = (b2.as_ptr() as usize, b1.as_ptr() as usize);
                if kind == 28 {
                    let template = if exchange { 0x98c5d8 } else { 0x98c554 };
                    fmt(tp, template, 0x64, xg, xg, gender, gender, money, player, xname, name);
                } else {
                    let template = if exchange { 0x98c4c8 } else { 0x98c448 };
                    fmt(tp, template, 0x64, gender, gender, xg, xg, money, player, name, xname);
                }
                set(out);
                link_record_club(out, name, club);
                link_record_club(out, xname, other);
                h.link(out, &b1);
            }
        }
        // the season's transfer spending and income
        30 | 31 => {
            let value = read_i32(r + if kind == 30 { club_record::TRANSFER_SPENDING } else { club_record::TRANSFER_INCOME });
            if value > -1 {
                record_money_text(value as u32, 6, &mut b1);
                fmt(tp, if kind == 30 { 0x98c3f0 } else { 0x98c394 }, gender, gender, b1.as_ptr() as usize, name);
                set(out);
                link_record_club(out, name, club);
            }
        }
        // the fans' player of the year
        _ => {
            let h = RecordHolder(r + club_record::FANS_PLAYER);
            if read_i32(h.0 + 0xc) > -1 {
                h.name(&mut b1);
                let long_name = club.addr() + club::NAME;
                let lg = read_i8(club.addr() + club::GENDER_NAME) as i32;
                fmt(b3.as_mut_ptr() as usize, 0x98c380); // "Player of the Year"
                fmt(tp, 0x98c2f0, lg, lg, 0x64, b1.as_ptr() as usize, long_name, b3.as_ptr() as usize);
                set(out);
                link_record_club(out, long_name, club);
                exe::text_add_link(out, b3.as_ptr(), 0x7e6, club);
                h.link(out, &b1);
            }
        }
    }
    1
}

/// Comparer for `write_record_news_text`: the text side by side (`compare_text_writer`) and the return. Without the
/// Text or the news item, with a club id outside the table or an unknown type (the error boxes), only the original.
pub(crate) unsafe extern "thiscall" fn compare_write_record_news_text(this: usize, news: usize, full: u32, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(IDX_WRITE_RECORD_NEWS_TEXT);
    if out == 0 || news == 0 {
        return o(this, news, full, out);
    }
    let id = exe::newsitem_get_slot(news, 0) as i32;
    let kind = exe::newsitem_get_slot(news, 1) as u8;
    if id < 0 || id > read_i32(G_CLUB_COUNT) - 1 || record_news_fields(kind).is_empty() {
        return o(this, news, full, out);
    }
    let (ro, rr) = (core::cell::Cell::new(0u32), core::cell::Cell::new(0u32));
    compare_text_writer(out, || ro.set(o(this, news, full, out)), |t| rr.set(write_record_news_text(this, news, full, t)),
        || format!("write_record_news_text(type {}, full {})", kind, full));
    SCOREBOARD.record(ro.get() == rr.get(), || format!("write_record_news_text(type {}) returned {} x {}", kind, ro.get(), rr.get()));
    ro.get()
}

// ---------- batch 324: the board's weekly confidence update (manager_manager.cpp) ----------

/// Offsets used by `weekly_board_confidence_update` that have no name elsewhere.
#[allow(dead_code)]
pub(crate) mod board_week_fields {
    /// Board row: the finance mood, moved with the confidence by the club's wealth band (i16) [inference].
    pub(crate) const FINANCE_MOOD: usize = 0x10;
    /// comp_stats_club: two bytes summed and compared with the games played × 1.6..2.1 by the "on fire" story [unknown].
    pub(crate) const STAT_69: usize = 0x69;
    pub(crate) const STAT_6A: usize = 0x6a;
    /// comp_stats_club: the last date of each run (8 bytes; run + 0xc).
    pub(crate) const WINNING_RUN_DATE: usize = 0x25;
    pub(crate) const LOSING_RUN_DATE: usize = 0x39;
    pub(crate) const UNBEATEN_RUN_DATE: usize = 0x4d;
    pub(crate) const WINLESS_RUN_DATE: usize = 0x61;
}

/// Exe addresses `weekly_board_confidence_update` uses besides its calls (cm0102.exe 3.9.68). The calls without an
/// `exe::` wrapper go by literal address: 0x72b810 Media::pick_former_international_pundit (thiscall(club, subject),
/// ret 8), 0x72b940 Media::pick_rival_manager_id (thiscall(club), ret 4), 0x48f340 Comp::get_first_schedule_date
/// (thiscall(out), ret 4), 0x8cc030 TRANSFER_MANAGER::get_past_transfers (thiscall, ret 0x18).
#[allow(dead_code)]
pub(crate) mod board_week_fns {
    pub(crate) const MEDIA_STORY_ENTRY: usize = 0x74b060; // Media::post_team_media_story (the comparer's recorder)
}

/// .rdata constants of `weekly_board_confidence_update` not in `rdata` (cm0102.exe 3.9.68).
#[allow(dead_code)]
pub(crate) mod board_week_rdata {
    pub(crate) const F64_9: usize = 0x96d230; // f64 = 9.0
    pub(crate) const F64_22: usize = 0x96af40; // f64 = 22.0
    pub(crate) const F64_2500: usize = 0x9690b0; // f64 = 2500.0
    pub(crate) const F64_3000: usize = 0x96d200; // f64 = 3000.0
    pub(crate) const F64_NEG_0_25: usize = 0x96d218; // f64 = -0.25
}

/// The league size N the board's pre-season stories read before it is written (see `weekly_board_confidence_update`):
/// the last one of the previous call, the copy's stand-in for the original's stack garbage.
static BOARD_WEEK_LEAGUE_SIZE: AtomicU8 = AtomicU8::new(0);

/// One club's pass of `weekly_board_confidence_update` (the original's locals).
struct BoardWeek {
    this: usize,
    index: usize,
    club: usize,
    manager: usize,
    sheet: usize,
    confidence_before: i16,
    league_size: i8,
    told_finals_hopes: bool,
    told_finals_doubts: bool,
    entry: usize,
    division_object: usize,
    proj_best: u8,
    proj_normal: u8,
    proj_worst: u8,
    x: f32,
    weight: f64,
}

impl BoardWeek {
    /// The club's board row: [this] + loop index × 0x49 (re-read every time, as the original).
    unsafe fn row(&self) -> usize {
        read_usize(self.this).wrapping_add(self.index.wrapping_mul(0x49))
    }
    unsafe fn add(&self, field: usize, v: i16) {
        let a = self.row() + field;
        write_i16(a, read_i16(a).wrapping_add(v));
    }
    unsafe fn confidence(&self) -> i16 {
        read_i16(self.row() + board_record::CONFIDENCE)
    }
    unsafe fn set_confidence(&self, v: i16) {
        write_i16(self.row() + board_record::CONFIDENCE, v);
    }
}

/// The competition object of a competition record: [[G_COMP_OBJECT_TABLE] + record id × 4].
unsafe fn board_week_comp_object(record: usize) -> usize {
    read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(record) as usize).wrapping_mul(4)))
}

/// The non-player sheet of the club's chairman (0 without chairman or sheet).
unsafe fn board_week_chairman_sheet(club: usize) -> usize {
    let c = read_usize(club + club::CHAIRMAN);
    if c == 0 { 0 } else { read_usize(c + staff::NON_PLAYER) }
}

/// The chairman's patience factor: 1 → `one`, 20 → 0.25, otherwise (21 − patience) × 0.1 kept within 0.5..2.
unsafe fn board_week_patience_factor(patience: i8, one: usize) -> f64 {
    if patience == 1 {
        return read_f32(one) as f64;
    }
    if patience == 20 {
        return read_f32(rdata::F32_0_25) as f64;
    }
    let m = (21 - patience as i32) as f64 * read_f64(rdata::F64_0_1);
    if m > read_f64(rdata::F64_2) {
        read_f32(rdata::F32_2) as f64
    } else if m < read_f64(rdata::F64_0_5) {
        read_f32(rdata::F32_0_5) as f64
    } else {
        m
    }
}

/// Who comments a board story: on a draw(n) other than 0 a former international (`pick_former_international_pundit`),
/// otherwise a rival's manager (`pick_rival_manager_id`, none when below 0).
unsafe fn board_week_story_person(club: usize, n: i32) -> usize {
    if random(n) != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x72b810usize);
        f(read_usize(G_MEDIA), club, 0)
    } else {
        let f: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(0x72b940usize);
        let id = f(read_usize(G_MEDIA), club);
        if id > -1 { staff_at(id as isize).addr() } else { 0 }
    }
}

/// Posts a board story about the club (`Media::post_team_media_story(club, kind, person, 0, 0, extra)`); in the
/// comparer's replay it checks the call against the original's tape and applies the story's recorded effects instead.
unsafe fn board_week_post_story(club: usize, kind: u32, person: usize, extra: i32) {
    if board_week_replay_story(club, kind, person, extra) {
        return;
    }
    exe::media_post_team_media_story(read_usize(G_MEDIA), club, kind, person as u32, 0, 0, extra);
}

/// `MANAGER_MANAGER::weekly_board_confidence_update()`: every Sunday (daily_update 0x68f9c0), for each club with a
/// manager and his non-player sheet that can be managed (`exe::club_can_be_managed`; a national team only with a
/// nation on a continent), the board's, supporters' and other confidences in the manager, the board stories in the
/// media and the manager's reputations.
///
/// Per club (row = [this] + loop index × 0x49): `decay_board_week`; the national team's finals stories
/// (`national_finals_stories`); otherwise the league week (`league_board_week`, which may post one story and moves the
/// confidence by the form, the projected position against the target and the chairman); then
/// `board_week_floors_and_reputation` (the continental floor, the 0..10000 clamps, the chairman's and the club's likes
/// and dislikes, the manager's reputations and the wealth band). Defect of the original (`docs/DEFEITOS_DO_ORIGINAL.md`,
/// approved to fix only in the all-Rust exe): the pre-season stories read the league size N ([esp+0x2b]) before it is
/// written (0x6984ad, 0x6986ff, 0x6987d6): the first club reads stack garbage, the next ones the N of the previous
/// club's league. Ported faithfully: N survives from club to club; the first one gets the last N of the previous call
/// (`BOARD_WEEK_LEAGUE_SIZE`; the comparer passes the byte the original really reads).
/// Original: `MANAGER_MANAGER::weekly_board_confidence_update` (0x697c40, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn weekly_board_confidence_update(this: usize) {
    let n = board_week_run(this, BOARD_WEEK_LEAGUE_SIZE.load(Relaxed) as i8);
    BOARD_WEEK_LEAGUE_SIZE.store(n as u8, Relaxed);
}

/// The club loop of `weekly_board_confidence_update` from the given stale league size; returns the last N.
unsafe fn board_week_run(this: usize, league_size: i8) -> i8 {
    let mut s = BoardWeek {
        this, index: 0, club: 0, manager: 0, sheet: 0, confidence_before: 0, league_size,
        told_finals_hopes: false, told_finals_doubts: false, entry: 0, division_object: 0,
        proj_best: 0, proj_normal: 0, proj_worst: 0, x: 0.0, weight: 0.0,
    };
    let base = read_usize(G_CLUB_TABLE);
    while (s.index as i32) < read_i32(G_CLUB_COUNT) {
        s.club = base.wrapping_add(s.index.wrapping_mul(club::SIZE));
        board_week_club(&mut s);
        s.index += 1;
    }
    s.league_size
}

/// One club of the weekly update (see `weekly_board_confidence_update`).
unsafe fn board_week_club(s: &mut BoardWeek) {
    let club = s.club;
    if exe::database_is_national_team(club) != 0 {
        let nation = read_usize(club + club::NATION);
        if nation == 0 || read_u32(nation + nation::CONTINENT) == 0 {
            return;
        }
    }
    let manager = read_usize(club + club::MANAGER);
    if manager == 0 || read_usize(manager + staff::NON_PLAYER) == 0 || exe::club_can_be_managed(club) == 0 {
        return;
    }
    s.manager = manager;
    s.sheet = read_usize(manager + staff::NON_PLAYER);
    decay_board_week(s);
    if !national_finals_stories(s) {
        league_board_week(s);
    }
    board_week_floors_and_reputation(s);
}

/// The weekly pull of the supporters' confidence towards the board's: the supporters (`board_record::SUPPORTERS_CONFIDENCE`)
/// kept within half and double the board's (`board_record::CONFIDENCE`), then the board's moves by 10% of the gap
/// (confidence − truncate((supporters − confidence) × −0.1)); the change also goes to `board_record::FAVOURITES_MOOD`.
unsafe fn decay_board_week(s: &mut BoardWeek) {
    let row = s.row();
    let confidence = read_i16(row + board_record::CONFIDENCE) as i32;
    let supporters = read_i16(row + board_record::SUPPORTERS_CONFIDENCE) as i32;
    covered!("gra_17 decay");
    let half = confidence / 2;
    let v = if half > supporters { half } else if confidence * 2 < supporters { confidence * 2 } else { supporters };
    write_i16(row + board_record::SUPPORTERS_CONFIDENCE, v as i16);
    let row = s.row();
    let supporters = read_i16(row + board_record::SUPPORTERS_CONFIDENCE) as i32;
    s.confidence_before = read_i16(row + board_record::CONFIDENCE);
    let d = ftol((supporters - s.confidence_before as i32) as f64 * read_f64(rdata::F64_NEG_0_1));
    write_i16(row + board_record::CONFIDENCE, s.confidence_before.wrapping_sub(d as i16));
    s.add(board_record::FAVOURITES_MOOD, s.confidence().wrapping_sub(s.confidence_before));
}

/// Compares two dates by year (unsigned) then day (signed), as the original's inline tests: −1, 0 or 1.
unsafe fn board_week_date_order(a: usize, b: usize) -> i32 {
    let (ya, yb) = (read_u16(a + 2), read_u16(b + 2));
    if ya != yb {
        return if ya < yb { -1 } else { 1 };
    }
    let (da, db) = (read_i16(a), read_i16(b));
    if da < db { -1 } else if da > db { 1 } else { 0 }
}

/// The national team's finals stories: only the nation's main national team (`exe::database_get_national_team`), on a
/// draw(10) of 0, with its current competition (`exe::nationalteams_get_current_competition`) a finals one
/// (`exe::national_teams_is_finals_comp`) with an object, starting (`Comp::get_first_schedule_date`) more than 10 and
/// less than 100 days from today. Returns false otherwise (the club goes to the league week). With it, nothing more
/// without the competition's statistics, the team's entry or with a match played; N = the competition's entrant count
/// (low byte). Once per call each: with the best position 1 and the previous final position below N / 5 − draw(3) + 2,
/// story 0x5a (the hopes; the person by `board_week_story_person` with a draw(4)); otherwise, with the final position
/// above draw(N² / 200 + 1) + 1 and below N / 5 + N / 10 + 2 and the team's reputation above 350 × the competition's,
/// story 0x44 (the doubts).
unsafe fn national_finals_stories(s: &mut BoardWeek) -> bool {
    let club = s.club;
    if exe::database_get_national_team(read_usize(club + club::NATION)) != club || random(10) != 0 {
        return false;
    }
    let comp = exe::nationalteams_get_current_competition(read_usize(G_NATIONAL_TEAMS), read_usize(club + club::NATION));
    if exe::national_teams_is_finals_comp(read_usize(G_NATIONAL_TEAMS), comp) == 0 || board_week_comp_object(comp) == 0 {
        return false;
    }
    let first_schedule = |out: &mut [u32; 2]| -> usize {
        let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x48f340usize);
        f(board_week_comp_object(comp), out.as_mut_ptr() as usize)
    };
    let (mut soon, mut first) = ([0u32; 2], [0u32; 2]);
    let d = exe::date_add_days(G_CURRENT_DATE, soon.as_mut_ptr() as usize, 10);
    let soon_copy = [read_u32(d), read_u32(d + 4)];
    if board_week_date_order(soon_copy.as_ptr() as usize, first_schedule(&mut first)) >= 0 {
        return false;
    }
    let (mut later, mut first2) = ([0u32; 2], [0u32; 2]);
    let d = exe::date_add_days(G_CURRENT_DATE, later.as_mut_ptr() as usize, 100);
    let later_copy = [read_u32(d), read_u32(d + 4)];
    if board_week_date_order(later_copy.as_ptr() as usize, first_schedule(&mut first2)) <= 0 {
        return false;
    }
    let stats = read_usize(board_week_comp_object(comp) + league::COMP_STATS);
    if stats == 0 {
        return true;
    }
    let entry = exe::comp_stats_find_club_entry(stats, club);
    if entry == 0 || read_u8(entry + comp_stats_club::MATCHES_PLAYED) != 0 {
        return true;
    }
    let n = read_i8(board_week_comp_object(comp) + league::ENTRANT_COUNT);
    s.league_size = n;
    let n = n as i32;
    if !s.told_finals_hopes && read_u8(entry + comp_stats_club::BEST_POSITION) == 1 {
        let r = random(3);
        if (read_u8(entry + comp_stats_club::FINAL_POSITION) as i32) < n / 5 - r + 2 {
            covered!("gra_17 finals hopes");
            let person = board_week_story_person(club, 4);
            board_week_post_story(club, 0x5a, person, -1);
            s.told_finals_hopes = true;
            return true;
        }
    }
    if s.told_finals_doubts {
        return true;
    }
    let last = read_u8(entry + comp_stats_club::FINAL_POSITION) as i32;
    if last <= random(n * n / 200 + 1) + 1 || last >= n / 5 + n / 10 + 2
        || read_i16(club + club::REPUTATION) as i32 <= read_i16(comp + competition::REPUTATION) as i32 * 350
    {
        return true;
    }
    covered!("gra_17 finals doubts");
    let person = board_week_story_person(club, 4);
    board_week_post_story(club, 0x44, person, -1);
    s.told_finals_doubts = true;
    true
}

/// The league week: nothing without a division with an object, its statistics (`league::COMP_STATS`) and the club's
/// entry (`exe::comp_stats_find_club_entry`). The projected positions (`exe::comp_stats_get_club_projected_positions`:
/// best, normal, worst), the form factor (`board_week_form_factor`), with best ≠ worst in the entry the form scaled by
/// the position against the previous final position, otherwise (no game played) the pre-season stories
/// (`preseason_board_stories`); the chairman's patience and the new manager's grace (`board_week_chairman_and_grace`);
/// then the change goes in (`apply_board_week_form`) and, with best ≠ worst and outside the new manager's grace, the
/// position against the target moves the confidence (`board_week_position`).
unsafe fn league_board_week(s: &mut BoardWeek) {
    let club = s.club;
    let division = read_usize(club + club::DIVISION);
    if division == 0 || board_week_comp_object(division) == 0 {
        return;
    }
    let stats = read_usize(board_week_comp_object(division) + league::COMP_STATS);
    if stats == 0 {
        return;
    }
    s.entry = exe::comp_stats_find_club_entry(stats, club);
    if s.entry == 0 {
        return;
    }
    let stats = read_usize(board_week_comp_object(read_usize(club + club::DIVISION)) + league::COMP_STATS);
    exe::comp_stats_get_club_projected_positions(stats, club, &mut s.proj_best, &mut s.proj_normal, &mut s.proj_worst);
    s.x = board_week_form_factor(s);
    let division = read_usize(club + club::DIVISION);
    let division_rep = read_i16(division + competition::REPUTATION);
    let club_rep = read_i16(club + club::REPUTATION);
    if division_rep != 0 && club_rep != 0 {
        if division_rep > 7 {
            let q = ((division_rep as i32 + 1) * 500) as f64 / club_rep as i32 as f64;
            let q = if q > read_f64(rdata::F64_1_25) {
                read_f32(rdata::F32_1_25) as f64
            } else if q < read_f64(rdata::F64_0_25) {
                read_f32(rdata::F32_0_25) as f64
            } else {
                q
            };
            let x = s.x as f64;
            s.x = if x < 0.0 && (q < read_f64(rdata::F64_1) || q > read_f64(rdata::F64_1)) { (x / q) as f32 } else { (q * x) as f32 };
        }
        if s.x < 0.0 && read_u8(club + club::PROFESSIONAL_STATUS) != 1 {
            s.x = (s.x as f64 * read_f64(rdata::F64_0_75)) as f32;
        }
        let e = s.entry;
        if read_u8(e + comp_stats_club::BEST_POSITION) != read_u8(e + comp_stats_club::WORST_POSITION) {
            let (current, last) = (read_u8(e + comp_stats_club::CURRENT_POSITION), read_u8(e + comp_stats_club::FINAL_POSITION));
            if current < last && s.x < 0.0 {
                s.x = (s.x as f64 * read_f64(rdata::F64_0_05)) as f32;
            } else if current == last && s.x < 0.0 {
                s.x = (s.x as f64 * read_f64(rdata::F64_0_1)) as f32;
            } else if current > last && s.x > 0.0 {
                s.x = (s.x as f64 * read_f64(rdata::F64_0_25)) as f32;
            }
        } else if read_u8(e + comp_stats_club::MATCHES_PLAYED) == 0 {
            preseason_board_stories(s);
        }
        board_week_chairman_and_grace(s);
    }
    apply_board_week_form(s);
    board_week_position(s);
}

/// The form factor x (f32): the current run in the league, counted only when its last game was less than 7 days ago
/// (`exe::date_days_since`): wins above 1 (× 25), defeats above 1 (× −30), winless above 2 (× −5) or unbeaten above 2
/// (× 5), in that order; the run n weighs n² below 4, otherwise min(n, 5)² × 0.2 + 9. A negative x is scaled by the
/// division's reputation × 0.1 (within 0.25..2) and, outside national teams, with the squad investment
/// (`board_record::SQUAD_INVESTMENT`) above the spending (`board_record::SPENDING`), by their ratio (at most 2.5), the
/// transfer mood (`board_record::TRANSFER_MOOD`) losing 150.
unsafe fn board_week_form_factor(s: &mut BoardWeek) -> f32 {
    let e = s.entry;
    let recent = |date: usize| exe::date_days_since(G_CURRENT_DATE, read_u32(e + date), read_u32(e + date + 4)) < 7;
    let (n, weight): (i16, i32) = if read_i32(e + comp_stats_club::WINNING_RUN) > 1 && recent(board_week_fields::WINNING_RUN_DATE) {
        covered!("gra_17 winning run");
        (read_i16(e + comp_stats_club::WINNING_RUN), 25)
    } else if read_i32(e + comp_stats_club::LOSING_RUN) > 1 && recent(board_week_fields::LOSING_RUN_DATE) {
        (read_i16(e + comp_stats_club::LOSING_RUN), -30)
    } else if read_i32(e + comp_stats_club::WINLESS_RUN) > 2 && recent(board_week_fields::WINLESS_RUN_DATE) {
        (read_i16(e + comp_stats_club::WINLESS_RUN), -5)
    } else if read_i32(e + comp_stats_club::UNBEATEN_RUN) > 2 && recent(board_week_fields::UNBEATEN_RUN_DATE) {
        (read_i16(e + comp_stats_club::UNBEATEN_RUN), 5)
    } else {
        (0, 0)
    };
    let f = if n >= 4 {
        let m = n.min(5) as i32;
        (m * m) as f64 * read_f64(rdata::F64_0_2) + read_f64(board_week_rdata::F64_9)
    } else {
        (n as i32 * n as i32) as f64
    };
    let mut x = (weight as f64 * f) as f32;
    if x < 0.0 {
        let m = read_i16(read_usize(s.club + club::DIVISION) + competition::REPUTATION) as i32 as f64 * read_f64(rdata::F64_0_1);
        let m = if m > read_f64(rdata::F64_2) {
            read_f32(rdata::F32_2) as f64
        } else if m < read_f64(rdata::F64_0_25) {
            read_f32(rdata::F32_0_25) as f64
        } else {
            m
        };
        x = (m * x as f64) as f32;
        if exe::database_is_national_team(s.club) == 0 {
            let row = s.row();
            if read_f64(row + board_record::SQUAD_INVESTMENT) > read_f64(row + board_record::SPENDING) {
                let r = read_f64(row + board_record::SQUAD_INVESTMENT) / read_f64(row + board_record::SPENDING);
                let r = if r > read_f64(rdata::F64_2_5) { read_f32(rdata::F32_2_5) as f64 } else { r };
                s.add(board_record::TRANSFER_MOOD, -150);
                x = (r * x as f64) as f32;
            }
        }
    }
    x
}

/// The pre-season stories (no game played, best = worst in the entry), with the division in a selected nation's league
/// (`nation::LEAGUE_SELECTED`), a manager and a top division (`league::UPPER_DIVISION_ID` = −1). N is the stale league
/// size (the defect, see `weekly_board_confidence_update`); positions are the previous final position. With the
/// position below N / 4 or the reputation above 6500 + draw(2000): the spending story (`board_week_spending_story`).
/// Otherwise, below N / 5 − draw(3) + 1: story 0x5d (title favourites); otherwise, above 1 and below N / 5 + N / 10:
/// story 0x42 (European hopes); the person by `board_week_story_person` with a draw(2).
unsafe fn preseason_board_stories(s: &mut BoardWeek) {
    let club = s.club;
    let in_league = || {
        let nation = read_usize(club + club::NATION);
        read_usize(club + club::DIVISION) != 0 && nation != 0 && read_u8(nation + nation::LEAGUE_SELECTED) != 0
            && read_usize(club + club::MANAGER) != 0
            && read_i32(board_week_comp_object(read_usize(club + club::DIVISION)) + league::UPPER_DIVISION_ID) == -1
    };
    let last = read_u8(s.entry + comp_stats_club::FINAL_POSITION) as i32;
    let n = s.league_size as i32;
    if in_league() && (last < n / 4 || read_i16(club + club::REPUTATION) as i32 > random(2000) + 6500) {
        board_week_spending_story(s);
        return;
    }
    if in_league() && last < n / 5 - random(3) + 1 {
        covered!("gra_17 preseason favourites");
        let person = board_week_story_person(club, 2);
        board_week_post_story(club, 0x5d, person, -1);
        return;
    }
    if in_league() && last > 1 && last < n / 5 + n / 10 {
        covered!("gra_17 preseason european hopes");
        let person = board_week_story_person(club, 2);
        board_week_post_story(club, 0x42, person, -1);
    }
}

/// The spending story: the club's transfers (`TRANSFER_MANAGER::get_past_transfers`) from the later of today − 100
/// days and the manager's arrival (`staff::DATE_JOINED_CLUB`) to the current year: what it paid minus what it got;
/// each purchase above 2,500,000 + draw(5,000,000) is a big one, and above 7,500,000 + draw(5,000,000) its player is
/// the star (the last such). With more than draw(2) + 1 big ones and the net above 15,000,000 + draw(10,000,000):
/// story 0x5b with the net as the extra (the star only with at least draw(2) + 2 big ones).
unsafe fn board_week_spending_story(s: &mut BoardWeek) {
    let club = s.club;
    let mut since = [0u32; 2];
    let d = exe::date_sub_days(G_CURRENT_DATE, since.as_mut_ptr() as usize, 100);
    let mut start = [read_u32(d), read_u32(d + 4)];
    let joined = read_usize(club + club::MANAGER) + staff::DATE_JOINED_CLUB;
    let joined_copy = [read_u32(joined), read_u32(joined + 4)];
    if board_week_date_order(joined_copy.as_ptr() as usize, start.as_ptr() as usize) > 0 {
        let mut tmp = [0u32; 2];
        exe::date_operator_3d(start.as_mut_ptr() as usize, tmp.as_mut_ptr() as usize, joined);
    }
    let start_year = read_u32(start.as_ptr() as usize + 2);
    let year = read_u16(G_CURRENT_DATE + 2) as u32;
    let (mut list, mut count) = (0usize, 0i32);
    let today_month = exe::date_get_month(G_CURRENT_DATE) as u32;
    let start_month = exe::date_get_month(start.as_ptr() as usize) as u32;
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, *mut usize, *mut i32) = core::mem::transmute(0x8cc030usize);
    f(read_usize(G_TRANSFER_MANAGER), start_month, start_year, today_month, year, &mut list, &mut count);
    let (mut net, mut big, mut star) = (0i32, 0i32, 0usize);
    let id = read_u32(club);
    for k in 0..count.max(0) as usize {
        let t = list + k * 0x25;
        if read_u32(t + 0x10) == id {
            let fee = read_i32(t + 0x18);
            net = net.wrapping_add(fee);
            if fee > random(5_000_000) + 2_500_000 {
                big += 1;
            }
            if read_i32(t + 0x18) > random(5_000_000) + 7_500_000 {
                star = staff_at(read_i32(t) as isize).addr();
            }
        } else if read_u32(t + 0x14) == id {
            net = net.wrapping_sub(read_i32(t + 0x18));
        }
    }
    if list != 0 {
        exe::crt_free(list);
    }
    if big > random(2) + 1 && net > random(10_000_000) + 15_000_000 {
        covered!("gra_17 spending story");
        if big < random(2) + 2 {
            star = 0;
        }
        board_week_post_story(club, 0x5b, star, net);
    }
}

/// The chairman's patience on a negative x (the patience reset to 10 below 1; factor 4 at 1, 0.25 at 20, otherwise
/// (21 − patience) × 0.1 within 0.5..2); without a chairman's sheet, and after it, a negative x with the board's
/// new-manager flag (`board_record_flags::NEW_MANAGER`) is × 0.2 in the manager's first 180 days, otherwise × 0.75.
unsafe fn board_week_chairman_and_grace(s: &mut BoardWeek) {
    let club = s.club;
    let sheet = board_week_chairman_sheet(club);
    if sheet != 0 {
        if !(s.x < 0.0) {
            return;
        }
        if read_i8(sheet + non_player::PATIENCE) < 1 {
            write_u8(sheet + non_player::PATIENCE, 10);
        }
        let m = board_week_patience_factor(read_i8(board_week_chairman_sheet(club) + non_player::PATIENCE), rdata::F32_4);
        s.x = (m * s.x as f64) as f32;
    }
    let manager = read_usize(club + club::MANAGER);
    if s.x < 0.0 && manager != 0 && read_u8(s.row() + board_record::FLAGS) & board_record_flags::NEW_MANAGER != 0 {
        let days = exe::date_days_since(G_CURRENT_DATE, read_u32(manager + staff::DATE_JOINED_CLUB), read_u32(manager + staff::DATE_JOINED_CLUB + 4));
        s.x = (s.x as f64 * if days < 180 { read_f64(rdata::F64_0_2) } else { read_f64(rdata::F64_0_75) }) as f32;
    }
}

/// The form goes in: d = truncate(x) on the board (`board_record::CONFIDENCE`), the supporters and the season's league
/// confidence (`board_record::LEAGUE_CONFIDENCE`), truncate(x × 1.25) on `board_record::FAVOURITES_MOOD`. Then N = the
/// division's club count (`league::CLUB_COUNT`, low byte) and the position weight w = (22 / N, or 1 without clubs),
/// with a division of reputation 10+ and the club's reputation above 500 × it, × ((rep − 500 × div) + (rep − 5000)) ×
/// 0.0002 + 1; w = w^1.5 (`game_pow`).
unsafe fn apply_board_week_form(s: &mut BoardWeek) {
    let d = ftol(s.x as f64) as i16;
    s.add(board_record::CONFIDENCE, d);
    s.add(board_record::SUPPORTERS_CONFIDENCE, d);
    s.add(board_record::FAVOURITES_MOOD, ftol(s.x as f64 * read_f64(rdata::F64_1_25)) as i16);
    s.add(board_record::LEAGUE_CONFIDENCE, d);
    let division = read_usize(s.club + club::DIVISION);
    s.division_object = board_week_comp_object(division);
    s.league_size = read_i8(s.division_object + league::CLUB_COUNT);
    let mut w = if s.league_size > 0 { read_f64(board_week_rdata::F64_22) / s.league_size as f64 } else { 1.0 };
    let division_rep = read_i16(division + competition::REPUTATION) as i32;
    let club_rep = read_i16(s.club + club::REPUTATION) as i32;
    if division_rep >= 10 && club_rep > division_rep * 500 {
        w = (((club_rep - division_rep * 500) as f64 + (club_rep - 5000) as f64) * read_f64(rdata::F64_0_0002) + read_f64(rdata::F64_1)) * w;
    }
    s.weight = game_pow(w, read_f64(rdata::F64_1_5));
}

/// The position against the target (only with best ≠ worst in the entry and without the new-manager flag): with even
/// the worst projection better than the previous final position, the flag `board_record_flags::PROJECTED_ABOVE_TARGET`
/// is set with +750 (already set: +750 on the board below 4000); with even the best one worse, the flag
/// `board_record_flags::PROJECTED_BELOW_TARGET` with −750 (already set: −750 above 6500); otherwise up to 4 games
/// played +5 above the final position, +1 on it and −5 below; after that the league stories
/// (`league_board_stories`) and the expectation (`board_week_expectation`). The change goes in by
/// `apply_board_week_position`.
unsafe fn board_week_position(s: &mut BoardWeek) {
    let e = s.entry;
    if read_u8(e + comp_stats_club::BEST_POSITION) == read_u8(e + comp_stats_club::WORST_POSITION) {
        return;
    }
    let flags = read_u8(s.row() + board_record::FLAGS);
    if flags & board_record_flags::NEW_MANAGER != 0 {
        return;
    }
    let last = read_u8(e + comp_stats_club::FINAL_POSITION);
    let delta: i32 = if read_u8(e + comp_stats_club::WORST_POSITION) < last {
        if flags & board_record_flags::PROJECTED_ABOVE_TARGET == 0 {
            write_u8(s.row() + board_record::FLAGS, flags | board_record_flags::PROJECTED_ABOVE_TARGET);
            750
        } else {
            if s.confidence() < 4000 {
                s.set_confidence(s.confidence().wrapping_add(750));
            }
            0
        }
    } else if read_u8(e + comp_stats_club::BEST_POSITION) > last {
        if flags & board_record_flags::PROJECTED_BELOW_TARGET == 0 {
            write_u8(s.row() + board_record::FLAGS, flags | board_record_flags::PROJECTED_BELOW_TARGET);
            -750
        } else {
            if s.confidence() > 6500 {
                s.set_confidence(s.confidence().wrapping_sub(750));
            }
            0
        }
    } else if read_u8(e + comp_stats_club::MATCHES_PLAYED) <= 4 {
        covered!("gra_17 early weeks");
        let current = read_u8(e + comp_stats_club::CURRENT_POSITION);
        if current < last { 5 } else if current == last { 1 } else { -5 }
    } else {
        league_board_stories(s);
        board_week_expectation(s)
    };
    apply_board_week_position(s, delta as i16);
}

/// The league stories (from the 5th game; at most one, in this order; "in a league" = division, a selected nation's
/// league, a manager; top = `league::UPPER_DIVISION_ID` −1; N = the division's club count; positions: current and final
/// of the entry, best/normal/worst projected; "runs" = wins above draw(a) + 2 or unbeaten above draw(b) + 4, "bad runs"
/// = defeats above draw(a) + 2 or winless above draw(b) + 4; the person by `board_week_story_person` with a draw(2)
/// unless "none"):
/// 0x5f below the top, 10 + draw(10) to 25 + draw(5) games, current below 4, final below 6 − draw(3), best within the
/// promotion and play-off places, worst not 1, runs (5, 5), draw(3) of 0; 0x60 more than 20 + draw(10) games, wins
/// above 1 or unbeaten above draw(3) + 2, a league with relegation (`league_vtable::HAS_RELEGATION`), current − draw(3)
/// + draw(3) above N − relegated, best below N − N / 2 − relegated, worst at least N − relegated; 0x43 more than
/// 10 + draw(200) games, unbeaten and winless below 3, (draw(6) × 0.1 + 1.6) × games above the entry's two stat bytes,
/// current − draw(4) + draw(4) = N / 2, best above N / 5, worst − draw(2) + draw(2) below N − relegated; at the top:
/// 0x63 (current 2..3, final 4..5) and 0x61 (current 2..3, final above 5), 10 + draw(10) to 25 + draw(5) games, best 1,
/// worst not 1, runs (5, 5); 0x62 10 + draw(10) to 29 games, current, final and best 1, worst not 1, runs (3, 4);
/// below the top 0x40, 25 + draw(10) games, current 2..8, final below 7, best within and worst outside the promotion
/// places, bad runs (3, 4); at the top 0x41, 25 + draw(10) games, current below 6, final below 5, normal 2 or 3, best
/// 1, worst not 1, bad runs (3, 4); 0x5e (none) reputation below 7250 + draw(2000), more than 5 + draw(10) and fewer
/// than 25 − draw(5) games, current above the final and below 5, best 1, worst not 1, wins non-zero and above 4 or
/// unbeaten above 7; 0x3e (none) current below the final, 5 + draw(10) to 25 − draw(5) games, defeats non-zero and above
/// 4 or winless above 7, relegation, current at least N − relegated and best above it; 0x3f more than 15 + draw(15)
/// games, defeats above 1 or winless above draw(3) + 2, relegation, draw(2) + current above N − relegated, normal at
/// least that, best below it and above N + N / 2 − relegated (never: the original's test); 0x3c (none, −250 on
/// `board_record::FAVOURITES_MOOD`) more than 20 games, final − current below −5, best below the final, the club
/// disliking the manager (`exe::club_dislikes_staff`) or a draw(4) of 0, the supporters (row by club id) below 4275,
/// no continental competition, defeats above draw(4) + 2 or winless above draw(7) + 3.
unsafe fn league_board_stories(s: &mut BoardWeek) {
    let club = s.club;
    let e = s.entry;
    let played = read_u8(e + comp_stats_club::MATCHES_PLAYED) as i32;
    let current = read_u8(e + comp_stats_club::CURRENT_POSITION) as i32;
    let last = read_u8(e + comp_stats_club::FINAL_POSITION) as i32;
    let (best, worst) = (s.proj_best as i32, s.proj_worst as i32);
    let division = || read_usize(club + club::DIVISION);
    let object = || board_week_comp_object(division());
    let nation_ok = || {
        let nation = read_usize(club + club::NATION);
        nation != 0 && read_u8(nation + nation::LEAGUE_SELECTED) != 0 && read_usize(club + club::MANAGER) != 0
    };
    let run = |field: usize| read_i32(e + field);
    let n = || s.league_size as i32;
    let relegated = || read_i8(object() + league::RELEGATED_COUNT) as i32;
    let promotion_places = || read_i8(object() + league::PROMOTION_PLAYOFF_COUNT) as i32 + read_i8(object() + league::PROMOTED_COUNT) as i32;
    let story = |kind: u32| {
        let person = board_week_story_person(club, 2);
        board_week_post_story(club, kind, person, -1);
    };
    // 0x5f: the promotion push
    if division() != 0 && nation_ok() && read_i32(s.division_object + league::UPPER_DIVISION_ID) != -1
        && played >= random(10) + 10 && played < random(5) + 25 && current < 4 && last < 6 - random(3)
        && best <= promotion_places() && worst != 1
        && (run(comp_stats_club::WINNING_RUN) > random(5) + 2 || run(comp_stats_club::UNBEATEN_RUN) > random(5) + 4)
        && random(3) == 0
    {
        covered!("gra_17 story 0x5f");
        return story(0x5f);
    }
    if division() == 0 {
        return board_week_story_0x63(s);
    }
    // 0x60: relegation trouble
    if nation_ok() && played > random(10) + 20
        && (run(comp_stats_club::WINNING_RUN) > 1 || run(comp_stats_club::UNBEATEN_RUN) > random(3) + 2)
        && call_comp_virtual(object(), league_vtable::HAS_RELEGATION) != 0
    {
        let safe = n() - relegated();
        let v = current - random(3);
        let v = v + random(3);
        if v > safe && best < n() - n() / 2 - relegated() && worst >= safe {
            covered!("gra_17 story 0x60");
            return story(0x60);
        }
    }
    // 0x43: the steady mid-table
    if division() != 0 && nation_ok() && played > random(200) + 10
        && run(comp_stats_club::UNBEATEN_RUN) < 3 && run(comp_stats_club::WINLESS_RUN) < 3
    {
        let stat = (read_u8(e + board_week_fields::STAT_6A) as i32 + read_u8(e + board_week_fields::STAT_69) as i32) as f64;
        let r = random(6);
        if (r as f64 * read_f64(rdata::F64_0_1) + read_f64(rdata::F64_1_6)) * played as f64 > stat {
            let a = current - random(4);
            let a = a + random(4);
            if a == n() / 2 && best > n() / 5 {
                let b = worst - random(2);
                let b = b + random(2);
                if b < n() - relegated() {
                    covered!("gra_17 story 0x43");
                    return story(0x43);
                }
            }
        }
    }
    board_week_story_0x63(s)
}

/// The league stories from 0x63 on (see `league_board_stories`).
unsafe fn board_week_story_0x63(s: &mut BoardWeek) {
    let club = s.club;
    let e = s.entry;
    let played = read_u8(e + comp_stats_club::MATCHES_PLAYED) as i32;
    let current = read_u8(e + comp_stats_club::CURRENT_POSITION) as i32;
    let last = read_u8(e + comp_stats_club::FINAL_POSITION) as i32;
    let (best, normal, worst) = (s.proj_best as i32, s.proj_normal as i32, s.proj_worst as i32);
    let division = || read_usize(club + club::DIVISION);
    let object = || board_week_comp_object(division());
    let in_league = || {
        let nation = read_usize(club + club::NATION);
        division() != 0 && nation != 0 && read_u8(nation + nation::LEAGUE_SELECTED) != 0 && read_usize(club + club::MANAGER) != 0
    };
    let top = || read_i32(object() + league::UPPER_DIVISION_ID) == -1;
    let run = |field: usize| read_i32(e + field);
    let n = || s.league_size as i32;
    let relegated = || read_i8(object() + league::RELEGATED_COUNT) as i32;
    let promotion_places = || read_i8(object() + league::PROMOTION_PLAYOFF_COUNT) as i32 + read_i8(object() + league::PROMOTED_COUNT) as i32;
    let story = |kind: u32| {
        let person = board_week_story_person(club, 2);
        board_week_post_story(club, kind, person, -1);
    };
    let good_runs = |a: i32, b: i32| run(comp_stats_club::WINNING_RUN) > random(a) + 2 || run(comp_stats_club::UNBEATEN_RUN) > random(b) + 4;
    let bad_runs = |a: i32, b: i32| run(comp_stats_club::LOSING_RUN) > random(a) + 2 || run(comp_stats_club::WINLESS_RUN) > random(b) + 4;
    // 0x63 and 0x61: chasing the title from the second or third place
    let title_chase: [(u32, fn(i32) -> bool); 2] = [(0x63, |l| l > 3 && l < 6), (0x61, |l| l > 5)];
    for (kind, last_ok) in title_chase {
        if in_league() && top() && played >= random(10) + 10 && played < random(5) + 25 && current < 4 && current > 1
            && last_ok(last) && best == 1 && worst != 1 && good_runs(5, 5)
        {
            covered!("gra_17 story 0x63 0x61");
            return story(kind);
        }
    }
    // 0x62: top of the table
    if in_league() && top() && played >= random(10) + 10 && played < 30 && current == 1 && last == 1 && best == 1 && worst != 1
        && good_runs(3, 4)
    {
        covered!("gra_17 story 0x62");
        return story(0x62);
    }
    // 0x40: slipping from the promotion places
    if in_league() && !top() && played >= random(10) + 25 && current < 9 && current > 1 && last < 7
        && best <= promotion_places() && worst > promotion_places() && bad_runs(3, 4)
    {
        covered!("gra_17 story 0x40");
        return story(0x40);
    }
    // 0x41: slipping from the title race
    if in_league() && top() && played >= random(10) + 25 && current < 6 && last < 5 && (normal == 2 || normal == 3) && best == 1
        && worst != 1 && bad_runs(3, 4)
    {
        covered!("gra_17 story 0x41");
        return story(0x41);
    }
    if division() == 0 {
        return board_week_story_0x3c(s);
    }
    // 0x5e: the surprise package
    if in_league() && (read_i16(club + club::REPUTATION) < (random(2000) + 7250) as i16) && played > random(10) + 5
        && played < 25 - random(5) && current < last && current < 5 && best == 1 && worst != 1
        && run(comp_stats_club::WINNING_RUN) != 0 && (run(comp_stats_club::WINNING_RUN) > 4 || run(comp_stats_club::UNBEATEN_RUN) > 7)
    {
        covered!("gra_17 story 0x5e");
        return board_week_post_story(club, 0x5e, 0, -1);
    }
    // 0x3e: the relegation zone
    if in_league() && current > last && played > random(10) + 5 && played < 25 - random(5)
        && run(comp_stats_club::LOSING_RUN) != 0 && (run(comp_stats_club::LOSING_RUN) > 4 || run(comp_stats_club::WINLESS_RUN) > 7)
        && call_comp_virtual(object(), league_vtable::HAS_RELEGATION) != 0
    {
        let safe = n() - relegated();
        if current >= safe && best < safe {
            covered!("gra_17 story 0x3e");
            return board_week_post_story(club, 0x3e, 0, -1);
        }
    }
    // 0x3f: the relegation fight (the original's last test can never hold)
    if in_league() && played > random(15) + 15
        && (run(comp_stats_club::LOSING_RUN) > 1 || run(comp_stats_club::WINLESS_RUN) > random(3) + 2)
        && call_comp_virtual(object(), league_vtable::HAS_RELEGATION) != 0
    {
        let safe = n() - relegated();
        if random(2) + current > safe && normal >= safe && best < safe && best > n() / 2 - relegated() + n() {
            covered!("gra_17 story 0x3f");
            return story(0x3f);
        }
    }
    board_week_story_0x3c(s)
}

/// The last league story, 0x3c (see `league_board_stories`).
unsafe fn board_week_story_0x3c(s: &mut BoardWeek) {
    let club = s.club;
    let e = s.entry;
    let last = read_u8(e + comp_stats_club::FINAL_POSITION);
    let current = read_u8(e + comp_stats_club::CURRENT_POSITION);
    if read_u8(e + comp_stats_club::MATCHES_PLAYED) > 20 && (last as i16 - current as i16) < -5 && s.proj_best > last
        && (exe::club_dislikes_staff(read_usize(club + club::MANAGER), club) != 0 || random(4) == 0)
        && (club == 0 || read_i16(read_usize(s.this).wrapping_add((read_i32(club) as usize).wrapping_mul(0x49)) + board_record::SUPPORTERS_CONFIDENCE) < 4275)
        && read_i32(club + club::CONTINENTAL_COMP_ID) == -1
        && (read_i32(e + comp_stats_club::LOSING_RUN) > random(4) + 2 || read_i32(e + comp_stats_club::WINLESS_RUN) > random(7) + 3)
    {
        covered!("gra_17 story 0x3c");
        board_week_post_story(club, 0x3c, 0, -1);
        s.add(board_record::FAVOURITES_MOOD, -250);
    }
}

/// The expectation against the previous final position (from the 5th game, after the stories): below it (better)
/// by more than 4, +15 a place + 40 (the board raised to 6500 instead, with no change, when below it); otherwise +10 a
/// place (raised to 4275 instead); on it, +5 (below 4275: raised to 4275 instead on a draw(patience) other than 0, or a
/// draw(10) without a chairman's sheet); above it (worse) by `board_week_overshoot`.
unsafe fn board_week_expectation(s: &mut BoardWeek) -> i32 {
    let e = s.entry;
    let (current, last) = (read_u8(e + comp_stats_club::CURRENT_POSITION), read_u8(e + comp_stats_club::FINAL_POSITION));
    if current < last {
        let d = (last as i16).wrapping_sub(current as i16);
        if d > 4 {
            if s.confidence() < 6500 {
                s.set_confidence(6500);
                return 0;
            }
            return d as i32 * 15 + 40;
        }
        if s.confidence() < 4275 {
            s.set_confidence(4275);
            return 0;
        }
        return d as i32 * 10;
    }
    if current == last {
        if s.confidence() < 4275 && board_week_patience_draw(s.club, 10) {
            s.set_confidence(4275);
            return 0;
        }
        return 5;
    }
    board_week_overshoot(s, current, last)
}

/// A draw(the chairman's patience) other than 0, or a draw(`n`) other than 0 without the chairman's sheet.
unsafe fn board_week_patience_draw(club: usize, n: i32) -> bool {
    let sheet = board_week_chairman_sheet(club);
    if sheet != 0 { random(read_i8(sheet + non_player::PATIENCE) as i32) != 0 } else { random(n) != 0 }
}

/// Worse than the final position: c = max((2 − games) × 2, truncate(final × w − current × w)) (16 bits). With the
/// title or promotion expected (`expectation_flags::TITLE_CHALLENGE`, `expectation_flags::PROMOTION`) and c below 0,
/// the current position within the division's play-off places (`league::PROMOTION_PLAYOFF_COUNT`, non-zero) counts as
/// c = 0; else with the play-offs expected (`expectation_flags::PLAYOFFS`) the same, the board raised to 6500. Then by
/// c: above −3, −5 (above 9500 the board pulled to 9500 instead when games ≥ patience / 2 + 2c + 10; below 4275 raised
/// to 4275 on the patience draw); −3 and −4, 10c − 25 (above 6500 to 6500 when games ≥ patience / 2 + c + 12; below
/// 3500 to 3500 on the patience draw); −5 to −9, 25c + 60 (above 6500 to 6500 when games ≥ patience / 2 + 10; above
/// 4275 to 4275 when games ≥ patience / 2 + 15; below 3000 to 3000 on the patience draw); −10 and below, 30c + 105 (the
/// same, but below 3500 to 3500 when draw(patience) > draw(30) + 1, or on a draw(5) of 0 without the chairman's sheet).
/// Each pull replaces the change (0).
unsafe fn board_week_overshoot(s: &mut BoardWeek, current: u8, last: u8) -> i32 {
    let club = s.club;
    let played = read_u8(s.entry + comp_stats_club::MATCHES_PLAYED) as i32;
    let v = ftol(last as i32 as f64 * s.weight - current as i32 as f64 * s.weight) as i16 as i32;
    let mut c = (2 - played) * 2;
    if !(c > v) {
        c = v;
    }
    let flags = read_u16(s.row() + board_record::EXPECTATION_FLAGS) as u8;
    let division = read_usize(club + club::DIVISION);
    let in_playoffs = || {
        let o = board_week_comp_object(read_usize(club + club::DIVISION));
        o != 0 && read_i8(o + league::PROMOTION_PLAYOFF_COUNT) != 0 && current as i32 <= read_i8(o + league::PROMOTION_PLAYOFF_COUNT) as i32
    };
    let mut pulled = false;
    if flags & (expectation_flags::TITLE_CHALLENGE | expectation_flags::PROMOTION) as u8 != 0 && (c as i16) < 0 && division != 0 {
        if in_playoffs() {
            c = 0;
            pulled = true;
        }
    } else if flags & expectation_flags::PLAYOFFS as u8 != 0 && (c as i16) < 0 && division != 0 && in_playoffs() {
        if s.confidence() < 6500 {
            s.set_confidence(6500);
        }
        c = 0;
        pulled = true;
    }
    let c16 = c as i16;
    let patience = || read_i8(board_week_chairman_sheet(club) + non_player::PATIENCE) as i32 / 2;
    let has_chairman = || board_week_chairman_sheet(club) != 0;
    if pulled || c16 > -3 {
        let conf = s.confidence();
        if conf > 9500 {
            if has_chairman() && played >= patience() + c16 as i32 * 2 + 10 {
                s.set_confidence(9500);
                return 0;
            }
        } else if conf < 4275 && board_week_patience_draw(club, 10) {
            s.set_confidence(4275);
            return 0;
        }
        return -5;
    }
    if c16 > -5 {
        let conf = s.confidence();
        if conf > 6500 {
            if has_chairman() && played >= patience() + c16 as i32 + 12 {
                s.set_confidence(6500);
                return 0;
            }
        } else if conf < 3500 && board_week_patience_draw(club, 10) {
            s.set_confidence(3500);
            return 0;
        }
        return c * 10 - 25;
    }
    let (delta, low) = if c16 > -10 { (c * 25 + 60, 3000) } else { (c * 30 + 105, 3500) };
    let conf = s.confidence();
    if conf > 6500 && has_chairman() && played >= patience() + 10 {
        s.set_confidence(6500);
        return 0;
    }
    if conf > 4275 {
        if has_chairman() && played >= patience() + 15 {
            s.set_confidence(4275);
            return 0;
        }
    } else if conf < low {
        let pull = if c16 > -10 {
            board_week_patience_draw(club, 10)
        } else {
            let sheet = board_week_chairman_sheet(club);
            if sheet != 0 {
                let a = random(read_i8(sheet + non_player::PATIENCE) as i32);
                a > random(30) + 1
            } else {
                random(5) == 0
            }
        };
        if pull {
            s.set_confidence(low as i16);
            return 0;
        }
    }
    delta
}

/// The position change goes in: d = truncate(change × the division's reputation × 0.1, within 0.5..2); for a club of
/// reputation above 5000 and above 400 × the division's, with final × w − current × w below −4, the final position
/// below 5 / w and d negative: d = truncate(min(2d, games × gap × 1.5)). A negative d × the chairman's patience factor
/// (patience outside 1..20 reset to 10; 3 at 1, 0.25 at 20). Outside national teams: with the squad investment above
/// the spending, a negative d × their ratio (at most 3) and the transfer mood −75; otherwise a positive d × 1.05 and the
/// transfer mood +5. A negative d × 0.75 for a club not fully professional, and again for a Welsh or Irish club (the
/// nations `KEY_NATION_WALES`, `KEY_NATION_REPUBLIC_OF_IRELAND`, `KEY_NATION_NORTHERN_IRELAND`). With a continental
/// competition and d negative, board and supporters lose truncate(d × −0.25); otherwise the board takes d and the
/// supporters truncate(d × 1.05).
unsafe fn apply_board_week_position(s: &mut BoardWeek, delta: i16) {
    let club = s.club;
    let e = s.entry;
    let division_rep = read_i16(read_usize(club + club::DIVISION) + competition::REPUTATION) as i32;
    let m = division_rep as f64 * read_f64(rdata::F64_0_1);
    let m = if m > read_f64(rdata::F64_2) {
        read_f32(rdata::F32_2) as f64
    } else if m < read_f64(rdata::F64_0_5) {
        read_f32(rdata::F32_0_5) as f64
    } else {
        m
    };
    let mut d = ftol(delta as i32 as f64 * m) as i16;
    let club_rep = read_i16(club + club::REPUTATION) as i32;
    if club_rep > 5000 && club_rep > division_rep * 400 {
        let (last, current) = (read_u8(e + comp_stats_club::FINAL_POSITION) as i32, read_u8(e + comp_stats_club::CURRENT_POSITION) as i32);
        let gap = last as f64 * s.weight - current as f64 * s.weight;
        if gap < read_f64(rdata::F64_NEG_4) && (last as f64) < read_f64(rdata::F64_5) / s.weight && d < 0 {
            let a = (d as i32 * 2) as f64;
            let u = read_u8(e + comp_stats_club::MATCHES_PLAYED) as i32 as f64 * gap * read_f64(rdata::F64_1_5);
            d = ftol(if a < u { a } else { u }) as i16;
        }
    }
    let sheet = board_week_chairman_sheet(club);
    if sheet != 0 && d < 0 {
        let p = read_i8(sheet + non_player::PATIENCE);
        if p < 1 || p > 20 {
            write_u8(sheet + non_player::PATIENCE, 10);
        }
        let f = board_week_patience_factor(read_i8(board_week_chairman_sheet(club) + non_player::PATIENCE), rdata::F32_3);
        d = ftol(d as i32 as f64 * f) as i16;
    }
    if exe::database_is_national_team(club) == 0 {
        let row = s.row();
        if read_f64(row + board_record::SQUAD_INVESTMENT) > read_f64(row + board_record::SPENDING) {
            if d < 0 {
                let r = read_f64(row + board_record::SQUAD_INVESTMENT) / read_f64(row + board_record::SPENDING);
                let r = if r > read_f64(rdata::F64_3) { read_f32(rdata::F32_3) as f64 } else { r };
                d = ftol(d as i32 as f64 * r) as i16;
            }
            s.add(board_record::TRANSFER_MOOD, -75);
        } else if d > 0 {
            d = ftol(d as i32 as f64 * read_f64(rdata::F64_1_05)) as i16;
            s.add(board_record::TRANSFER_MOOD, 5);
        }
    }
    if d < 0 {
        if read_u8(club + club::PROFESSIONAL_STATUS) != 1 {
            d = ftol(d as i32 as f64 * read_f64(rdata::F64_0_75)) as i16;
        }
        let nation = read_usize(club + club::NATION);
        if d < 0 && nation != 0 {
            let id = read_u32(nation);
            if id == read_u32(KEY_NATION_WALES) || id == read_u32(KEY_NATION_REPUBLIC_OF_IRELAND) || id == read_u32(KEY_NATION_NORTHERN_IRELAND) {
                d = ftol(d as i32 as f64 * read_f64(rdata::F64_0_75)) as i16;
            }
        }
    }
    if read_i32(club + club::CONTINENTAL_COMP_ID) != -1 && d < 0 {
        let a = ftol(d as i32 as f64 * read_f64(board_week_rdata::F64_NEG_0_25)) as i16;
        s.add(board_record::CONFIDENCE, a.wrapping_neg());
        s.add(board_record::SUPPORTERS_CONFIDENCE, a.wrapping_neg());
    } else {
        s.add(board_record::CONFIDENCE, d);
        s.add(board_record::SUPPORTERS_CONFIDENCE, ftol(d as i32 as f64 * read_f64(rdata::F64_1_05)) as i16);
    }
}

/// The end of every club's pass: with a continental competition other than the Intertoto Cup
/// (`KEY_CLUB_COMP_INTER_TOTO_CUP`; the club read again by index from `G_CLUB_TABLE`) the board at least 6500, or
/// 4275 when European success is expected (`expectation_flags_byte1::EUROPEAN_SUCCESS`) and the projection is below
/// the target (`board_record_flags::PROJECTED_BELOW_TARGET`); the board within 0..10000; at most 5250 when the manager
/// is disliked by the chairman (`exe::staff_is_disliked_by`), else at least 4250 when favoured
/// (`exe::staff_is_favoured_by`); the supporters within 0..10000, at most 3000 when the club dislikes the manager
/// (`exe::club_dislikes_staff`), else at least 7000 when it favours him (`exe::club_favours_staff`). Without the flags
/// 1, 4 and 8 of `board_record::FLAGS`: the manager's reputations (`update_board_week_reputation`) and, outside
/// national teams, the wealth band (`board_week_wealth_band`).
unsafe fn board_week_floors_and_reputation(s: &mut BoardWeek) {
    let club = s.club;
    let row = s.row();
    let european = read_u16(row + board_record::EXPECTATION_FLAGS) & ((expectation_flags_byte1::EUROPEAN_SUCCESS as u16) << 8);
    let continental = read_i32(read_usize(G_CLUB_TABLE).wrapping_add(s.index.wrapping_mul(club::SIZE)) + club::CONTINENTAL_COMP_ID) != -1
        && read_i32(club + club::CONTINENTAL_COMP_ID) != read_i32(KEY_CLUB_COMP_INTER_TOTO_CUP);
    if continental {
        let floor = if european != 0 && read_u8(row + board_record::FLAGS) & board_record_flags::PROJECTED_BELOW_TARGET != 0 { 4275 } else { 6500 };
        if read_i16(row + board_record::CONFIDENCE) < floor {
            write_i16(row + board_record::CONFIDENCE, floor);
        }
    }
    let conf = s.confidence();
    let change = conf.wrapping_sub(s.confidence_before);
    if conf > 10000 {
        s.set_confidence(10000);
    } else if conf < 0 {
        s.set_confidence(0);
    }
    let chairman = read_usize(club + club::CHAIRMAN);
    if exe::staff_is_disliked_by(s.manager, chairman) != 0 {
        s.set_confidence(s.confidence().min(5250));
    } else if exe::staff_is_favoured_by(s.manager, chairman) != 0 {
        s.set_confidence(s.confidence().max(4250));
    }
    let a = s.row() + board_record::SUPPORTERS_CONFIDENCE;
    let v = read_i16(a);
    if v > 10000 {
        write_i16(a, 10000);
    } else if v < 0 {
        write_i16(a, 0);
    }
    if exe::club_dislikes_staff(s.manager, club) != 0 {
        let a = s.row() + board_record::SUPPORTERS_CONFIDENCE;
        write_i16(a, read_i16(a).min(3000));
    } else if exe::club_favours_staff(s.manager, club) != 0 {
        let a = s.row() + board_record::SUPPORTERS_CONFIDENCE;
        write_i16(a, read_i16(a).max(7000));
    }
    let flags = read_u8(s.row() + board_record::FLAGS);
    if flags & (board_record_flags::NEW_MANAGER | board_record_flags::PROJECTED_ABOVE_TARGET | board_record_flags::PROJECTED_BELOW_TARGET) != 0 {
        return;
    }
    let mut d = change;
    if d < 0 {
        let rep = read_i16(club + club::REPUTATION);
        d = if rep > 5000 {
            ftol(read_f64(board_week_rdata::F64_2500) / rep as i32 as f64 * d as i32 as f64)
        } else {
            ftol(d as i32 as f64 * read_f64(rdata::F64_0_5))
        } as i16;
    }
    update_board_week_reputation(s, d);
    if exe::database_is_national_team(club) == 0 {
        board_week_wealth_band(s);
    }
}

/// The reputation band of a reputation: v / 1000 + 16, at least 22 (signed byte).
fn board_week_band(v: i16) -> i8 {
    let b = ((v as i32 / 1000) as i8).wrapping_add(16);
    if b < 22 { 22 } else { b }
}

/// Keeps a value pulled outside [lo, hi] at the nearer bound, the bounds limited to 1..10000 (as the original's tests:
/// only a value outside is touched).
fn board_week_pull(v: i32, lo: i32, hi: i32) -> i32 {
    if v > hi {
        if hi > 10000 { 10000 } else { hi }
    } else if v < lo {
        if lo < 1 { 1 } else { lo }
    } else {
        v
    }
}

/// The manager's reputations (`non_player::CURRENT_REPUTATION`, `non_player::HOME_REPUTATION`,
/// `non_player::WORLD_REPUTATION`) move with the board's change d (16 bits; a fall scaled by 2500 / reputation above
/// 5000, otherwise halved): targets = reputation + d, pulled within the club's reputation ± 3000 (world: −5000..+1000).
/// Each one's speed n = its band (`board_week_band`) − 19; for the home one 23 − club reputation / 500 when the manager
/// is foreign to the club's nation (`staff::NATION`), and for the world one 23 − club reputation / 500 outside national
/// teams (n 0 counts as 1). New = ((n − 1) × old + target) / n, kept within old − f × 250 and old + f × 300 (home:
/// 375; world: ± f × 500 rounded to f32) with f = 2 − old × 0.0001; capped at the club's reputation (or 50 × its
/// division's, the larger) within 5000..10000 but not below the old value (world: 2/3 of it within 3000..10000), and at
/// least 1. Each reputation becomes new / 2 + old / 2.
unsafe fn update_board_week_reputation(s: &mut BoardWeek, d: i16) {
    let club = s.club;
    let sheet = s.sheet;
    let home = read_i16(sheet + non_player::HOME_REPUTATION);
    let current = read_i16(sheet + non_player::CURRENT_REPUTATION);
    let world = read_i16(sheet + non_player::WORLD_REPUTATION);
    let club_rep = read_i16(club + club::REPUTATION) as i32;
    let current_target = board_week_pull(current.wrapping_add(d) as i32, club_rep - 3000, club_rep + 3000);
    let home_target = board_week_pull(home.wrapping_add(d) as i32, club_rep - 3000, club_rep + 3000);
    let world_target = board_week_pull(world.wrapping_add(d) as i32, club_rep - 5000, club_rep + 1000);
    covered!("gra_17 manager reputation");
    let (current_band, home_band, world_band) = (board_week_band(current), board_week_band(home), board_week_band(world));
    let nonzero = |n: i16| if n == 0 { 1 } else { n };
    let n_current = nonzero((current_band as i16).wrapping_sub(19));
    let foreign = exe::database_is_national_team(club) == 0
        && read_u32(club + club::NATION) != read_u32(s.manager + staff::NATION);
    let n_home = nonzero(if foreign { (23 - read_i16(club + club::REPUTATION) as i32 / 500) as i16 } else { (home_band as i16).wrapping_sub(19) });
    let n_world = nonzero(if exe::database_is_national_team(club) != 0 {
        (world_band as i16).wrapping_sub(19)
    } else {
        (23 - read_i16(club + club::REPUTATION) as i32 / 500) as i16
    });
    let step = |n: i16, old: i16, target: i32| ((n as i32 - 1).wrapping_mul(old as i32).wrapping_add(target as i16 as i32)) / n as i32;
    let new_current = step(n_current, current, current_target) as i16;
    let new_home = step(n_home, home, home_target) as i16;
    let new_world = step(n_world, world, world_target) as i16;
    let limit = |old: i16, new: i16, f: f32, down: f64, up: f64| {
        let o = old as i32 as f32 as f64;
        let nf = new as i32 as f32 as f64;
        let lo = o - f as f64 * down;
        if lo > nf {
            return ftol(lo);
        }
        let hi = f as f64 * up + o;
        if hi < nf { ftol(hi) } else { ftol(nf) }
    };
    let factor = |v: i16| read_f64(rdata::F64_2) - v as i32 as f64 * read_f64(rdata::F64_0_0001);
    let final_current = limit(current, new_current, factor(current) as f32, read_f32(rdata::F32_250) as f64, read_f32(rdata::F32_300) as f64) as i16;
    let final_home = limit(home, new_home, factor(home) as f32, read_f32(rdata::F32_250) as f64, read_f32(rdata::F32_375) as f64) as i16;
    let k = (factor(world) * read_f32(rdata::F32_500) as f64) as f32 as f64;
    let wf = world as i32 as f32 as f64;
    let nwf = new_world as i32 as f32 as f64;
    let final_world = if wf - k > nwf { ftol(wf - k) } else if k + wf < nwf { ftol(k + wf) } else { ftol(nwf) } as i16;
    let mut cap_rep = read_i16(club + club::REPUTATION);
    let division = read_usize(club + club::DIVISION);
    if division != 0 {
        let e = read_i16(division + competition::REPUTATION) as i32 * 50;
        if e > cap_rep as i32 {
            cap_rep = e as i16;
        }
    }
    let band = |v: i16| if v < 5000 { 5000 } else if v > 10000 { 10000 } else { v };
    let cap = |c: i16, old: i16, v: i16| {
        let c = if c > old { c } else { old };
        let v = if v > c { c } else { v };
        if v < 1 { 1 } else { v }
    };
    let final_home = cap(band(cap_rep), home, final_home);
    let final_current = cap(band(cap_rep), current, final_current);
    let w = cap_rep as i32 as f64 * read_f64(rdata::F64_0_6666666667);
    let w = if !(read_f64(board_week_rdata::F64_3000) <= w) {
        read_f64(board_week_rdata::F64_3000)
    } else if read_f64(rdata::F64_10000) < w {
        read_f64(rdata::F64_10000)
    } else {
        w
    };
    let final_world = cap(ftol(w) as i16, world, final_world);
    write_i16(sheet + non_player::CURRENT_REPUTATION, (final_current as i32 / 2 + current as i32 / 2) as i16);
    write_i16(sheet + non_player::HOME_REPUTATION, (final_home as i32 / 2 + home as i32 / 2) as i16);
    write_i16(sheet + non_player::WORLD_REPUTATION, (final_world as i32 / 2 + world as i32 / 2) as i16);
}

/// The wealth band (`exe::finance_get_wealth_band` of `exe::financetable_get_club_finance`) against the one the board
/// took at the season's start (`board_record::WEALTH_BAND`, row by club id), on the board and on
/// `board_week_fields::FINANCE_MOOD`: band 2 below 2 → +35 (reputation below 3500) or +15; band 1 below 1 → +30 or
/// +10; band 0: from below 0 → +25 or +5, from above → −5; band −1: from below 0 → +5, otherwise −50 / −125 for a plc
/// (`club::PLC`) or −25 / −50; band −2: from −2 → +5, otherwise −75 / −175 (plc) or −50 / −75; another band shows the
/// manager_manager.cpp box (line 0x28aa). Then the board within 0..10000.
unsafe fn board_week_wealth_band(s: &mut BoardWeek) {
    let club = s.club;
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    if finance != 0 {
        let band = exe::finance_get_wealth_band(finance) as i8;
        let taken = read_i8(read_usize(s.this).wrapping_add((read_i32(club) as usize).wrapping_mul(0x49)) + board_record::WEALTH_BAND);
        let low = read_i16(club + club::REPUTATION) < 3500;
        let plc = read_u8(club + club::PLC) != 0;
        let (board, mood): (i16, i16) = match band {
            2 => if taken >= 2 { (0, 0) } else if low { (35, 35) } else { (15, 15) },
            1 => if taken >= 1 { (0, 0) } else if low { (30, 30) } else { (10, 10) },
            0 => if taken < 0 { if low { (25, 25) } else { (5, 5) } } else if taken > 0 { (-5, -5) } else { (0, 0) },
            -1 => if taken < 0 { (5, 5) } else if plc { (-50, -125) } else { (-25, -50) },
            -2 => if taken == -2 { (5, 5) } else if plc { (-75, -175) } else { (-50, -75) },
            _ => {
                show_error_at(0x9f18a4, 0x28aa);
                (0, 0)
            }
        };
        if board != 0 || mood != 0 {
            covered!("gra_17 wealth band");
            s.add(board_record::CONFIDENCE, board);
            s.add(board_week_fields::FINANCE_MOOD, mood);
        }
    }
    let conf = s.confidence();
    if conf > 10000 {
        s.set_confidence(10000);
    } else if conf < 0 {
        s.set_confidence(0);
    }
}

// ---------- the comparer: the original runs, the stories are taped, the copy replays them ----------

/// One story the original posted during the weekly update: the arguments, the compared state just before it and the
/// bytes it changed (address, new value).
struct BoardStoryCall {
    args: [u32; 4],
    before: Vec<u8>,
    changes: Vec<(usize, u8)>,
}

/// The tape of the stories: the compared regions, the calls, the next one to replay and the first mismatch.
struct BoardStoryTape {
    regions: Vec<(usize, usize)>,
    calls: Vec<BoardStoryCall>,
    next: usize,
    broken: Option<String>,
    recording: bool,
    depth: u32,
    target: usize,
}

static mut BOARD_STORY_TAPE: Option<BoardStoryTape> = None;

unsafe fn board_story_flat(regions: &[(usize, usize)]) -> Vec<u8> {
    let mut v = Vec::with_capacity(regions.iter().map(|r| r.1).sum());
    for &(a, n) in regions {
        v.extend_from_slice(core::slice::from_raw_parts(a as *const u8, n));
    }
    v
}

unsafe fn board_story_write(regions: &[(usize, usize)], flat: &[u8]) {
    let mut k = 0;
    for &(a, n) in regions {
        core::ptr::copy_nonoverlapping(flat.as_ptr().add(k), a as *mut u8, n);
        k += n;
    }
}

/// The first difference between two flat copies, by region (index, offset).
fn board_story_first_difference(regions: &[(usize, usize)], a: &[u8], b: &[u8]) -> Option<(usize, usize, u8, u8)> {
    let mut k = 0;
    for (i, &(_, n)) in regions.iter().enumerate() {
        if let Some(o) = (0..n).find(|&o| a[k + o] != b[k + o]) {
            return Some((i, o, a[k + o], b[k + o]));
        }
        k += n;
    }
    None
}

/// The recorder the comparer puts at the story entry (0x74b060) while the original runs: tapes the state before, calls
/// the hooked story and tapes what changed.
unsafe extern "thiscall" fn board_story_recorder(this: usize, club: usize, kind: u32, person: usize, game: usize, variant: u32, extra: u32) {
    let tape = (*core::ptr::addr_of_mut!(BOARD_STORY_TAPE)).as_mut().unwrap();
    let f: unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, u32, u32) = core::mem::transmute(tape.target);
    if tape.depth > 0 || !tape.recording {
        return f(this, club, kind, person, game, variant, extra);
    }
    tape.depth += 1;
    let before = board_story_flat(&tape.regions);
    f(this, club, kind, person, game, variant, extra);
    let tape = (*core::ptr::addr_of_mut!(BOARD_STORY_TAPE)).as_mut().unwrap();
    tape.depth -= 1;
    let after = board_story_flat(&tape.regions);
    let mut changes = Vec::new();
    let mut k = 0;
    for &(a, n) in &tape.regions {
        for o in 0..n {
            if before[k + o] != after[k + o] {
                changes.push((a + o, after[k + o]));
            }
        }
        k += n;
    }
    tape.calls.push(BoardStoryCall { args: [club as u32, kind, person as u32, extra], before, changes });
}

/// In the comparer's replay, the copy's story: the next taped call must have the same arguments and the same state
/// before it; its recorded changes are applied. Returns false outside the replay.
unsafe fn board_week_replay_story(club: usize, kind: u32, person: usize, extra: i32) -> bool {
    let tape = match (*core::ptr::addr_of_mut!(BOARD_STORY_TAPE)).as_mut() {
        Some(t) if !t.recording => t,
        _ => return false,
    };
    if tape.broken.is_some() {
        return true;
    }
    let args = [club as u32, kind, person as u32, extra as u32];
    let Some(call) = tape.calls.get(tape.next) else {
        tape.broken = Some(format!("story {} {:x?} the original did not post", tape.next, args));
        return true;
    };
    if call.args != args {
        tape.broken = Some(format!("story {}: copy {:x?} original {:x?}", tape.next, args, call.args));
        return true;
    }
    let now = board_story_flat(&tape.regions);
    if let Some((i, o, x, y)) = board_story_first_difference(&tape.regions, &now, &call.before) {
        tape.broken = Some(format!("state before story {} {:x?}: region {} (+{:#x}) copy {:02x} original {:02x}", tape.next, args, i, o, x, y));
        return true;
    }
    for &(a, b) in &call.changes {
        write_u8(a, b);
    }
    tape.next += 1;
    true
}

/// Calls the original as the game would and returns the byte it finds at its league size local ([esp+0x2b] of its
/// frame: the aligned frame base − 0x2b1), read just before the call.
unsafe fn board_week_call_original(f: usize, this: usize) -> u8 {
    let mut residue: u32 = 0;
    core::arch::asm!(
        "lea eax, [esp - 8]",
        "and eax, -8",
        "movzx eax, byte ptr [eax - 0x2b1]",
        "mov dword ptr [edi], eax",
        "call edx",
        in("edi") &mut residue as *mut u32,
        in("edx") f,
        in("ecx") this,
        clobber_abi("C"),
    );
    residue as u8
}

/// Comparer for `weekly_board_confidence_update`: the routine posts stories (news, the log, the manager's answer) and
/// cannot run twice. The original runs as the game's, with a recorder on the story entry (the `jmp` of the hook at
/// 0x74b060 pointed at `board_story_recorder` meanwhile) that tapes, for every story, its arguments, the compared state
/// before it and what it changed; the compared state is the board table ([this], clubs × 0x49), the random generator and
/// every club's chairman patience and manager reputations. Then the state before is put back and the copy runs from
/// it with the league size the original found on its stack: each story of the copy must be the next one taped, with
/// the same arguments and the same state before it, and gets the taped changes; at the end every story must have been
/// replayed and the state must equal the original's (which stays).
pub(crate) unsafe extern "thiscall" fn compare_weekly_board_confidence_update(this: usize) {
    let o = original_fn::<usize>(IDX_WEEKLY_BOARD_CONFIDENCE_UPDATE);
    let entry = board_week_fns::MEDIA_STORY_ENTRY;
    if read_u8(entry) != 0xe9 || read_usize(this) == 0 {
        board_week_call_original(o, this); // ponytail: no hook on the story entry (or no table): only the original
        return;
    }
    let target = entry.wrapping_add(5).wrapping_add(read_i32(entry + 1) as usize);
    let count = read_i32(G_CLUB_COUNT).max(0) as usize;
    let mut regions: Vec<(usize, usize)> = vec![(read_usize(this), count * 0x49)];
    regions.extend_from_slice(&crate::rng::STATE_REGIONS);
    let base = read_usize(G_CLUB_TABLE);
    for i in 0..count {
        let club = base + i * club::SIZE;
        let sheet = board_week_chairman_sheet(club);
        if sheet != 0 {
            regions.push((sheet + non_player::PATIENCE, 1));
        }
        let manager = read_usize(club + club::MANAGER);
        if manager != 0 && read_usize(manager + staff::NON_PLAYER) != 0 {
            regions.push((read_usize(manager + staff::NON_PLAYER) + non_player::HOME_REPUTATION, 6));
        }
    }
    let before = board_story_flat(&regions);
    *core::ptr::addr_of_mut!(BOARD_STORY_TAPE) = Some(BoardStoryTape { regions, calls: Vec::new(), next: 0, broken: None, recording: true, depth: 0, target });
    let patch = |to: usize| {
        let mut old = 0u32;
        crate::win::VirtualProtect(entry as *mut core::ffi::c_void, 5, crate::win::PAGE_EXECUTE_READWRITE, &mut old);
        write_i32(entry + 1, to.wrapping_sub(entry + 5) as i32);
        crate::win::VirtualProtect(entry as *mut core::ffi::c_void, 5, old, &mut old);
        crate::win::FlushInstructionCache(crate::win::GetCurrentProcess(), entry as *const core::ffi::c_void, 5);
    };
    patch(board_story_recorder as *const () as usize);
    let residue = board_week_call_original(o, this);
    patch(target);
    let tape = (*core::ptr::addr_of_mut!(BOARD_STORY_TAPE)).as_mut().unwrap();
    tape.recording = false;
    let after = board_story_flat(&tape.regions);
    board_story_write(&tape.regions, &before);
    let n = crate::rng::without_counting(|| board_week_run(this, residue as i8));
    let tape = (*core::ptr::addr_of_mut!(BOARD_STORY_TAPE)).take().unwrap();
    let copy = board_story_flat(&tape.regions);
    let difference = board_story_first_difference(&tape.regions, &copy, &after);
    board_story_write(&tape.regions, &after);
    BOARD_WEEK_LEAGUE_SIZE.store(n as u8, Relaxed);
    let ok = tape.broken.is_none() && tape.next == tape.calls.len() && difference.is_none();
    SCOREBOARD.record(ok, || {
        let place = difference.map(|(i, o, x, y)| {
            let what = if i == 0 { format!("board row of club {} +{:#x}", o / 0x49, o % 0x49) } else if i < 3 { format!("random generator +{:#x}", o) } else { format!("region {} +{:#x}", i, o) };
            format!("{} copy {:02x} original {:02x}", what, x, y)
        });
        format!("weekly_board_confidence_update(residue N {}): stories {} replayed {} {:?}; state {:?}", residue as i8, tape.calls.len(), tape.next, tape.broken, place)
    });
}

// ---------- batch 324: a played cup match (Cup.cpp) ----------

/// Exe routines that `cup_process_fixture` calls without an `exe::` wrapper (Rust hooked, called through the exe), and
/// the two Cup virtual slots past Comp that decide a match (Cup vtable 0x9693c8: 0x51c020 and 0x51f2f0).
pub(crate) mod cup_fixture_fns {
    pub(crate) const FIND_TREE_SLOT_FOR_FIXTURE: usize = 0x519c90; // Cup::find_tree_slot_for_fixture: thiscall(cup, fixture), ret 4
    pub(crate) const RECORD_TEAM_PROGRESS: usize = 0x521b30; // Cup::record_team_progress: thiscall(cup, team, round), ret 8
    pub(crate) const PAY_ROUND_PRIZE_MONEY: usize = 0x522240; // Cup::pay_round_prize_money: thiscall(cup, winner, loser, round, fixture), ret 0x10
    pub(crate) const APPLY_ROUND_RULES_TO_MATCH: usize = 0x521d60; // Cup::apply_round_rules_to_match: thiscall(cup, fixture, slot), ret 8
    pub(crate) const FIND_NEXT_FIXTURE: usize = 0x5ac920; // FIXTURE_MANAGER::find_next_fixture: thiscall, 11 arguments, ret 0x2c
    pub(crate) const DECIDE_FIXTURE_WINNER: usize = 0x98; // virtual: Cup::decide_fixture_winner(fixture), ret 4
    pub(crate) const DECIDE_TIE_WINNER: usize = 0x9c; // virtual: Cup::decide_tie_winner(tie, fixture), ret 8
    pub(crate) const CUP_CPP: usize = 0x9b55ac; // "E:\dev\CM3\cm3 00-01\cm3\code\comp\Cup.cpp"
    pub(crate) const DRAWN_GAME_TEXT: usize = 0x9b5678; // "CUP::process_fixture() - drawn game in cup (Round %d of %s)"
    pub(crate) const THIRD_MATCH_TEXT: usize = 0x9b5628; // "CUP::process_fixture() : third match should not have a replay (round %d in %s)"
}

/// Round record and tie offsets that only `cup_process_fixture` names.
pub(crate) mod cup_fixture_fields {
    /// i32 in the round record: paid to both clubs of every match of the round when above 0 [inference: a match fee].
    pub(crate) const ROUND_MATCH_PRIZE: usize = 0x5c;
    /// u16 in the round record: the tie rules of the second replay and of the deciding match after two legs (the first
    /// replay takes `cup_round::TWO_LEG_TIE_RULES`) [inference].
    pub(crate) const ROUND_LATER_TIE_RULES: usize = 0x11;
    /// i8 in the tie: the result marks (fixture `HOME_RESULT_MARK`/`AWAY_RESULT_MARK`) of side A (`cup_tie::HOME_TEAM`)
    /// and side B in the tie's last match (docs/MAPA_SAVE.md §6.1).
    pub(crate) const TIE_HOME_MARK: usize = 4;
    pub(crate) const TIE_AWAY_MARK: usize = 5;
    /// The tie's three result blocks (10 bytes each: the fixture's goals, from `fixture::HOME_GOALS`), by matches played.
    pub(crate) const TIE_RESULT_BLOCKS: [usize; 3] = [0x14, 0x1e, 0x28];
}

/// What the cup's decision (`cup_fixture_fns::DECIDE_FIXTURE_WINNER` / `DECIDE_TIE_WINNER`) returns.
pub(crate) mod cup_decision {
    pub(crate) const ERROR: i32 = -1;
    pub(crate) const DRAW: i32 = 0; // a single match drawn: a replay
    pub(crate) const HOME: i32 = 1; // the home side goes through
    pub(crate) const AWAY: i32 = 2; // the away side goes through
    pub(crate) const FIRST_LEG: i32 = 3; // a first leg played: the second leg is already in the calendar
    pub(crate) const LEVEL: i32 = 4; // level after two legs: a deciding match
}

/// The team outcome byte (`cup_team_entry::STATUS`) that `cup_process_fixture` writes.
pub(crate) mod cup_team_status {
    pub(crate) const WINNER: u8 = 1;
    pub(crate) const RUNNER_UP: u8 = 2;
    pub(crate) const THIRD: u8 = 3;
    pub(crate) const FOURTH: u8 = 4;
    pub(crate) const OUT: u8 = 0xff;
}

/// The address of virtual slot `slot` of the cup.
unsafe fn cup_fixture_virtual(cup: Cup, slot: usize) -> usize {
    read_usize(read_usize(cup.addr()) + slot)
}

/// The cup's decision on a played match: the tie rule (`cup_fixture_fns::DECIDE_TIE_WINNER`) in a two-legged round
/// (`cup_round::LEGS` 2) before its third match, otherwise the match rule (`cup_fixture_fns::DECIDE_FIXTURE_WINNER`).
pub(crate) unsafe fn cup_fixture_decision(cup: Cup, round: i32, tie: usize, fixture: usize, played: i32) -> i32 {
    if read_u8(round_record(cup, round) + cup_round::LEGS) == 2 && played != 2 {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(cup_fixture_virtual(cup, cup_fixture_fns::DECIDE_TIE_WINNER));
        f(cup.addr(), tie, fixture)
    } else {
        let f: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(cup_fixture_virtual(cup, cup_fixture_fns::DECIDE_FIXTURE_WINNER));
        f(cup.addr(), fixture)
    }
}

/// The tie's result marks in side order: the fixture's (`HOME_RESULT_MARK`, `AWAY_RESULT_MARK`) when its home club is the
/// club of side A (`cup_tie::HOME_TEAM`), otherwise swapped.
pub(crate) unsafe fn cup_tie_marks(cup: Cup, tie: usize, fixture: usize) -> (u8, u8) {
    let (home, away) = (read_u8(fixture + fixture::HOME_RESULT_MARK), read_u8(fixture + fixture::AWAY_RESULT_MARK));
    if read_usize(fixture + fixture::HOME_CLUB) == read_usize(team_entry(cup, read_i16(tie + cup_tie::HOME_TEAM) as i32)) {
        (home, away)
    } else {
        (away, home)
    }
}

/// The tie's winner and loser (team indices) after decision `r` (`cup_decision::HOME`/`AWAY`): side A
/// (`cup_tie::HOME_TEAM`) wins when it is the fixture's home club and the home side went through, or when it is not and
/// the away side went through; otherwise side B (`cup_tie::AWAY_TEAM`).
pub(crate) unsafe fn cup_tie_sides(cup: Cup, tie: usize, fixture: usize, r: i32) -> (i16, i16) {
    let (a, b) = (read_i16(tie + cup_tie::HOME_TEAM), read_i16(tie + cup_tie::AWAY_TEAM));
    let a_at_home = read_usize(team_entry(cup, a as i32)) == read_usize(fixture + fixture::HOME_CLUB);
    if (r == cup_decision::HOME) == a_at_home {
        (a, b)
    } else {
        (b, a)
    }
}

/// The third-place round: the last round when its code is `round_name::THIRD_PLACE_PLAYOFF`, otherwise −1.
pub(crate) unsafe fn cup_third_place_round(cup: Cup) -> i32 {
    let n = cup.round_count() as i32;
    if read_i16(round_record(cup, n - 1) + cup_round::ROUND_CODE) == round_name::THIRD_PLACE_PLAYOFF {
        n - 1
    } else {
        -1
    }
}

/// Whether every one of the cup's `total` teams (the low word, signed) has an outcome (`cup_team_entry::STATUS` ≠ 0),
/// with `status(k)` the outcome of team k: 1, or 0 at the first team without one. The value of `Cup::run_by_stages`
/// after a final.
pub(crate) unsafe fn cup_all_teams_decided(total: i32, status: impl Fn(i16) -> u8) -> u8 {
    let n = total as i16;
    let mut k: i16 = 0;
    while k < n {
        if status(k) == 0 {
            return 0;
        }
        k += 1;
    }
    1
}

/// A one-round cup without legs whose tie has had its replays (`cup_tie::REPLAYS_PLAYED` ≥ `cup_round::SINGLE_LEG_FLAG`):
/// a draw (or a level tie) ends it there.
pub(crate) unsafe fn cup_replays_exhausted(cup: Cup, round: i32, tie: usize) -> bool {
    let r = round_record(cup, round);
    cup.round_count() == 1 && (read_u8(r + cup_round::LEGS) as i8) < 2 && read_i8(tie + cup_tie::REPLAYS_PLAYED) >= read_i8(r + cup_round::SINGLE_LEG_FLAG)
}

/// Writes `text` (`"%d"` the round, `"%s"` the competition's name) into the news text buffer, then the error box at
/// `line`.
unsafe fn cup_fixture_error(cup: Cup, text: usize, round: i32, line: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    format(NEWS_TEXT_BUFFER, text, round, read_usize(cup.addr() + league::COMPETITION) + competition::NAME);
    show_error_at(cup_fixture_fns::CUP_CPP, line);
}

/// Calls the cup's virtual `comp_vtable::ANNOUNCE_TEAM_STATUS` (`Cup::add_team_result_news`) for team `team`'s club with
/// outcome `status`, the cup's `stage_index()`, the fixture and the caller's argument.
unsafe fn cup_announce_team(cup: Cup, team: i16, status: u32, fixture: usize, a2: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(cup_fixture_virtual(cup, comp_vtable::ANNOUNCE_TEAM_STATUS));
    f(cup.addr(), read_usize(team_entry(cup, team as i32)), status, cup.stage_index() as u32, fixture, a2);
}

/// `Cup::record_team_progress(team, round)` through the exe.
unsafe fn cup_record_progress(cup: Cup, team: i16, round: i16) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(cup_fixture_fns::RECORD_TEAM_PROGRESS);
    f(cup.addr(), team as i32 as u32, round as u16 as u32);
}

/// The time slice of a match on `date`: 1 on the weekday codes 5 and 6 (the weekend), otherwise 2.
unsafe fn cup_weekend_slice(date: usize) -> u8 {
    let w = exe::date_get_weekday(date) as i16;
    if w == 5 || w == 6 {
        1
    } else {
        2
    }
}

/// `date` + `n` days (`exe::date_add_days`; the count is read as a word).
unsafe fn cup_date_plus(date: usize, n: i32) -> [u8; 8] {
    let mut out = [0u8; 8];
    exe::date_add_days(date, out.as_mut_ptr() as usize, n as u32);
    out
}

/// Swaps the clubs and the club ids of a fixture being built.
unsafe fn cup_swap_fixture_sides(p: Fixture) {
    let (home, away, home_id, away_id) = (p.home_club(), p.away_club(), p.home_club_id(), p.away_club_id());
    p.set_home_club(away);
    p.set_away_club(home);
    p.set_home_club_id(away_id);
    p.set_away_club_id(home_id);
}

/// A fixture being built that is still at the away club's stadium (`club::STADIUM`) moves to the home club's: the
/// stadium and its id (−1 with no stadium).
unsafe fn cup_follow_home_stadium(p: Fixture) {
    if read_usize(p.addr() + fixture::STADIUM) != read_usize(p.away_club().addr() + club::STADIUM) {
        return;
    }
    let stadium = read_usize(p.home_club().addr() + club::STADIUM);
    write_usize(p.addr() + fixture::STADIUM, stadium);
    write_usize(p.addr() + fixture::STADIUM_ID, if stadium != 0 { read_usize(stadium) } else { usize::MAX });
}

/// Sets the ten result bytes of a fixture being built (`fixture::HOME_GOALS` to `fixture::AWAY_AGGREGATE`) to 0xff, "not
/// played".
unsafe fn cup_clear_fixture_result(p: Fixture) {
    for k in 0..10 {
        write_u8(p.addr() + fixture::HOME_GOALS + k, 0xff);
    }
}

/// A first leg played whose second leg is already in the calendar (`cup_decision::FIRST_LEG`).
///
/// The tie's next match is the second (`cup_tie::NEXT_MATCH` 1). The second leg: the tie's next fixture from the day
/// after the match on (`find_next_fixture`, forward, the competition, round and both clubs). Not found: the error box
/// (line 0x4fe). Found: unless the rules (`fixture::TIE_RULES`) have 0x10 (points) or 0x2000, it takes this match's
/// score as its first-leg score, reversed (`fixture::HOME_AGGREGATE` = this match's away goals, `AWAY_AGGREGATE` = its
/// home goals) and the round rules (`apply_round_rules_to_match` with the tree slot); with them, this match's result
/// marks, reversed. Returns 3.
unsafe fn cup_first_leg_played(cup: Cup, f: Fixture, tie: usize, slot: i32) -> i32 {
    write_u8(tie + cup_tie::NEXT_MATCH, 1);
    let day_after = cup_date_plus(make_date(f.day() as u32, f.year() as u32).as_ptr() as usize, 1);
    let (mut day, mut year) = cup_date_day_year(&day_after);
    let mut slice = 0u8;
    let find: unsafe extern "thiscall" fn(usize, *mut u16, *mut u16, *mut u8, u32, i32, i32, i32, i32, u32, u32, u32) -> usize = core::mem::transmute(cup_fixture_fns::FIND_NEXT_FIXTURE);
    let next = find(read_usize(G_FIXTURE_MANAGER), &mut year, &mut day, &mut slice, 1, f.comp_id(), f.round_number() as i32, f.away_club_id() as i32, f.home_club_id() as i32, u32::MAX, u32::MAX, u32::MAX);
    if next == 0 {
        show_error_at(cup_fixture_fns::CUP_CPP, 0x4fe);
        return cup_decision::FIRST_LEG;
    }
    let rules = f.tie_rules();
    if rules & 0x10 == 0 && rules & 0x2000 == 0 {
        covered!("gra_25 second leg takes the first-leg score");
        write_u8(next + fixture::HOME_AGGREGATE, read_u8(f.addr() + fixture::AWAY_GOALS));
        write_u8(next + fixture::AWAY_AGGREGATE, read_u8(f.addr() + fixture::HOME_GOALS));
        let apply: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(cup_fixture_fns::APPLY_ROUND_RULES_TO_MATCH);
        apply(cup.addr(), next, slot);
        return cup_decision::FIRST_LEG;
    }
    write_u8(next + fixture::HOME_RESULT_MARK, read_u8(f.addr() + fixture::AWAY_RESULT_MARK));
    write_u8(next + fixture::AWAY_RESULT_MARK, read_u8(f.addr() + fixture::HOME_RESULT_MARK));
    cup_decision::FIRST_LEG
}

/// A draw (or a level tie) in a one-round cup whose replays are over (`cup_replays_exhausted`).
///
/// A tie fed from another (`cup_tie::FEEDER_TIE` ≠ −1): the "drawn game" text and the error box at `line`; 0. Otherwise
/// both teams get outcome 1, their progress is recorded (`record_team_progress`, the round) and the cup is marked
/// finished when every team has an outcome (`Cup::set_run_by_stages`, `cup_all_teams_decided`); 1.
unsafe fn cup_end_on_draw(cup: Cup, f: Fixture, tie: usize, feeder_tie: i32, total: i32, line: u32) -> i32 {
    let round = f.round_number();
    if feeder_tie != -1 {
        cup_fixture_error(cup, cup_fixture_fns::DRAWN_GAME_TEXT, round as i32, line);
        return 0;
    }
    covered!("gra_25 cup ends on a draw");
    let (a, b) = (read_i16(tie + cup_tie::HOME_TEAM), read_i16(tie + cup_tie::AWAY_TEAM));
    write_u8(team_entry(cup, a as i32) + cup_team_entry::STATUS, cup_team_status::WINNER);
    write_u8(team_entry(cup, b as i32) + cup_team_entry::STATUS, cup_team_status::WINNER);
    cup_record_progress(cup, a, round);
    cup_record_progress(cup, b, round);
    cup.set_run_by_stages(cup_all_teams_decided(total, |k| read_u8(team_entry(cup, k as i32) + cup_team_entry::STATUS)));
    1
}

/// A single match drawn (`cup_decision::DRAW`): the replay.
///
/// A copy of the match; before the first replay its date moves `cup_round::LEG_GAP_DAYS` days on (the byte unsigned);
/// the fixture manager finds it a free date (`exe::fixture_manager_find_new_fixture_date`); the time slice by the
/// weekday (`cup_weekend_slice`); one more replay in the tie (`cup_tie::REPLAYS_PLAYED`); the round name the base code
/// (`exe::comp_util_get_base_round_id`) + 1 for the first replay, + 2 for the next; the sides swapped (the replay at the
/// other ground) and the stadium following the home club (`cup_follow_home_stadium`); the bits 0x100, 0x200 and 0x800 of
/// `fixture::SQUAD_RULES` cleared, the result bytes "not played", no statistics slot. The rules: after the first match
/// `cup_round::TWO_LEG_TIE_RULES`, with bit 0x400 of `SQUAD_RULES` when `cup_round::SINGLE_LEG_FLAG` is above 1; after
/// the second `cup_fixture_fields::ROUND_LATER_TIE_RULES`, without 0x400; after a third: the "third match" text and the
/// error box (line 0x5aa), 0. The stage index of the match, the result marks 0, the date at least tomorrow and into the
/// calendar (`add_fixture_to_calendar`). Returns 1.
unsafe fn cup_schedule_replay(cup: Cup, f: Fixture, slot: i32, played: i32) -> i32 {
    let round = f.round_number() as i32;
    let tie = || round_tie(cup, round, slot);
    let mut buf = [0u8; fixture::SIZE];
    core::ptr::copy_nonoverlapping(f.addr() as *const u8, buf.as_mut_ptr(), fixture::SIZE);
    let p = Fixture(buf.as_mut_ptr() as usize);
    if read_u8(tie() + cup_tie::REPLAYS_PLAYED) == 0 {
        let gap = read_u8(round_record(cup, round) + cup_round::LEG_GAP_DAYS) as i32;
        let d = cup_date_plus(make_date(f.day() as u32, f.year() as u32).as_ptr() as usize, gap);
        let (day, year) = cup_date_day_year(&d);
        p.set_day(day);
        p.set_year(year);
    }
    exe::fixture_manager_find_new_fixture_date(read_usize(G_FIXTURE_MANAGER), p);
    p.set_time_slice(cup_weekend_slice(make_date(p.day() as u32, p.year() as u32).as_ptr() as usize));
    write_u8(tie() + cup_tie::REPLAYS_PLAYED, read_u8(tie() + cup_tie::REPLAYS_PLAYED).wrapping_add(1));
    let step = if read_u8(tie() + cup_tie::REPLAYS_PLAYED) == 1 { 1 } else { 2 };
    p.set_round_name((exe::comp_util_get_base_round_id(read_u32(p.addr() + fixture::ROUND_NAME)) as u16).wrapping_add(step) as i16);
    cup_swap_fixture_sides(p); // the replay at the other ground
    cup_follow_home_stadium(p);
    p.set_flags(p.flags() & 0xf4);
    cup_clear_fixture_result(p);
    p.set_match_stats_slot(usize::MAX);
    let r = round_record(cup, round);
    match played {
        0 => {
            covered!("gra_25 first replay");
            p.set_tie_rules(read_u16(r + cup_round::TWO_LEG_TIE_RULES));
            if (read_u8(r + cup_round::SINGLE_LEG_FLAG) as i8) > 1 {
                p.set_flags(p.flags() | 4);
            } else {
                p.set_flags(p.flags() & !4);
            }
        }
        1 => {
            covered!("gra_25 second replay");
            p.set_tie_rules(read_u16(r + cup_fixture_fields::ROUND_LATER_TIE_RULES));
            p.set_flags(p.flags() & !4);
        }
        _ => {
            cup_fixture_error(cup, cup_fixture_fns::THIRD_MATCH_TEXT, round, 0x5aa);
            return 0;
        }
    }
    p.set_stage_index(f.stage_index());
    p.set_home_result_mark(result_mark::NONE);
    p.set_away_result_mark(result_mark::NONE);
    add_fixture_to_calendar(p);
    1
}

/// A tie level after two legs (`cup_decision::LEVEL`): the deciding match.
///
/// A copy of the match, 4 days later with rule 0x2000, otherwise 7; no first-leg score; the time slice by the weekday;
/// one more replay in the tie (`cup_tie::REPLAYS_PLAYED`); the round name the base code + 1 when
/// `cup_round::SINGLE_LEG_FLAG` is set, otherwise + 5 (the decider); the sides swapped unless rule 0x2000, the stadium
/// following the home club; the result bytes "not played", no statistics slot, the rules
/// `cup_fixture_fields::ROUND_LATER_TIE_RULES`, the bits 0x100 to 0x800 of `fixture::SQUAD_RULES` cleared; the date at
/// least tomorrow and into the calendar. The result marks stay the match's. Returns 1.
unsafe fn cup_schedule_deciding_match(cup: Cup, f: Fixture, slot: i32) -> i32 {
    let round = f.round_number() as i32;
    let mut buf = [0u8; fixture::SIZE];
    core::ptr::copy_nonoverlapping(f.addr() as *const u8, buf.as_mut_ptr(), fixture::SIZE);
    let p = Fixture(buf.as_mut_ptr() as usize);
    let points = f.tie_rules() & 0x2000 != 0;
    let d = cup_date_plus(make_date(p.day() as u32, p.year() as u32).as_ptr() as usize, if points { 4 } else { 7 });
    let (day, year) = cup_date_day_year(&d);
    p.set_year(year);
    p.set_day(day);
    write_u8(p.addr() + fixture::HOME_AGGREGATE, 0xff);
    write_u8(p.addr() + fixture::AWAY_AGGREGATE, 0xff);
    p.set_time_slice(cup_weekend_slice(d.as_ptr() as usize));
    let tie = round_tie(cup, round, slot);
    write_u8(tie + cup_tie::REPLAYS_PLAYED, read_u8(tie + cup_tie::REPLAYS_PLAYED).wrapping_add(1));
    let step = if read_u8(round_record(cup, round) + cup_round::SINGLE_LEG_FLAG) != 0 { 1 } else { 5 };
    p.set_round_name((exe::comp_util_get_base_round_id(read_u32(p.addr() + fixture::ROUND_NAME)) as u16).wrapping_add(step) as i16);
    if !points {
        covered!("gra_25 deciding match");
        cup_swap_fixture_sides(p);
    }
    cup_follow_home_stadium(p);
    cup_clear_fixture_result(p);
    p.set_match_stats_slot(usize::MAX);
    p.set_tie_rules(read_u16(round_record(cup, round) + cup_fixture_fields::ROUND_LATER_TIE_RULES));
    p.set_flags(p.flags() & 0xf0);
    add_fixture_to_calendar(p);
    1
}

/// The match of a next-round tie whose two teams are known, built when the tie's fixture was already due
/// (`cup_tie::FIXTURE_STATE` 3).
///
/// In a local fixture (not zeroed by the original: the bits 0xf0 of `fixture::FLAGS` keep stack leftovers, docs/
/// MAPA_SAVE.md; here zero): the competition and its id (−1 without one); from round `round` the code
/// (`cup_round::ROUND_CODE`, + 3 in a two-legged round), the cup's `stage_label()` and `year()`, the round number; the
/// cup's virtual `comp_vtable::SET_FIXTURE_SUBSTITUTES`; then the round's draw rules, tie rules and match category;
/// bits 0x100, 0x200 and 0x800 of `SQUAD_RULES` cleared, the result bytes "not played", no statistics slot, the match's
/// stage index, no stage (0xff), both side marks 1. Single-legged with tie rule 0x800 (the higher seed at home), the
/// rules become 0x1000 when the home team's seed (`cup_team_entry::SEED_LEVEL`) is not below the away team's, otherwise
/// 0x800. Bit 0x400 of `SQUAD_RULES` when single-legged with `cup_round::SINGLE_LEG_FLAG` above 0. The next round's date
/// (0 in the last round). The clubs of the tie's teams and their ids (no club: the error box, line 0x804 or 0x80f, and
/// −1). The date: today when the round's date is not after the day after tomorrow, otherwise the round's; the slice the
/// round's (`cup_round::TIME_SLICE`); the stadium (`exe::stadium_set_fixture_venue` with `cup_round::VENUE_VALUE` and the
/// cup's `venue_stadium_id()`); a round date not after the day after tomorrow asks the fixture manager for a free date.
/// The result bytes again, the stage index, the result marks 0; at least tomorrow and into the calendar. If the cup is
/// in [`G_COMP_OBJECT_TABLE`] (by `comp_id`): with a human manager at either club, news item 0x14 to both (0 the
/// competition id, 1 the stage label, 2 the round code, 3 and 4 the clubs, the human's first, 5 two legs, 6 the stadium
/// id, 7 the day, 8 the year); in a two-legged round, the second leg: code + 4, bit 0x400 by `SINGLE_LEG_FLAG` above 0,
/// on the later of the round's date + `cup_round::LEG_GAP_DAYS` and three days after the first leg (that one asks the
/// fixture manager for a free date), the slice by the weekday, the sides swapped, the stadium, bits 0x100 to 0x800
/// cleared, the result bytes, the stage index, the result marks 0, at least tomorrow and into the calendar. Returns 1.
unsafe fn cup_create_next_round_fixture(cup: Cup, f: Fixture, round: i32, tie_index: i32, comp_id: i32) -> i32 {
    let stage = f.stage_index();
    let mut buf = [0u8; fixture::SIZE];
    let p = Fixture(buf.as_mut_ptr() as usize);
    let comp = read_usize(cup.addr() + league::COMPETITION);
    if comp != 0 {
        write_usize(p.addr() + fixture::COMPETITION, comp);
        write_usize(p.addr() + fixture::COMP_ID, read_usize(comp));
    } else {
        write_usize(p.addr() + fixture::COMP_ID, usize::MAX);
        write_usize(p.addr() + fixture::COMPETITION, 0);
    }
    let rr = || round_record(cup, round);
    let code = read_i16(rr() + cup_round::ROUND_CODE);
    p.set_round_name(if read_u8(rr() + cup_round::LEGS) == 2 { code.wrapping_add(3) } else { code });
    p.set_round_prefix(cup.stage_label());
    p.set_season_year(cup.year() as u16);
    p.set_round_number(read_i16(rr() + cup_round::ROUND_NUMBER));
    let substitutes: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(cup_fixture_virtual(cup, comp_vtable::SET_FIXTURE_SUBSTITUTES));
    substitutes(cup.addr(), p.addr());
    let r = rr();
    p.set_round_draw_rules(read_u16(r + cup_round::DRAW_RULES));
    p.set_tie_rules(read_u16(r + cup_round::TIE_RULES));
    p.set_category(read_u8(r + cup_round::MATCH_CATEGORY));
    p.set_flags(p.flags() & 0xf4);
    cup_clear_fixture_result(p);
    p.set_match_stats_slot(usize::MAX);
    p.set_stage_index(stage);
    p.set_stage(0xff);
    p.set_home_side_mark(1);
    p.set_away_side_mark(1);
    let tie = || round_tie(cup, round, tie_index);
    if (read_u8(r + cup_round::LEGS) as i8) < 2 && p.tie_rules() & 0x800 != 0 {
        covered!("gra_25 higher seed at home");
        let seed = |t: i16| read_i8(team_entry(cup, t as i32) + cup_team_entry::SEED_LEVEL);
        p.set_tie_rules(if seed(read_i16(tie() + cup_tie::HOME_TEAM)) >= seed(read_i16(tie() + cup_tie::AWAY_TEAM)) { 0x1000 } else { 0x800 });
    }
    if (read_u8(r + cup_round::SINGLE_LEG_FLAG) as i8) > 0 && (read_u8(r + cup_round::LEGS) as i8) < 2 {
        p.set_flags(p.flags() | 4);
    } else {
        p.set_flags(p.flags() & !4);
    }
    if round < cup.round_count() as i32 - 1 {
        p.set_next_round_day(read_u16(r + cup_round::SIZE + cup_round::DAY));
        p.set_next_round_year((read_i8(r + cup_round::SIZE + cup_round::YEAR_OFFSET) as i16).wrapping_add(cup.year()) as u16);
    } else {
        p.set_next_round_day(0);
        p.set_next_round_year(0);
    }
    for (side, line) in [(cup_tie::HOME_TEAM, 0x804u32), (cup_tie::AWAY_TEAM, 0x80f)] {
        let club = read_usize(team_entry(cup, read_i16(tie() + side) as i32));
        let (at, id_at) = if side == cup_tie::HOME_TEAM { (fixture::HOME_CLUB, fixture::HOME_CLUB_ID) } else { (fixture::AWAY_CLUB, fixture::AWAY_CLUB_ID) };
        write_usize(p.addr() + at, club);
        if club != 0 {
            write_usize(p.addr() + id_at, read_usize(club));
        } else {
            show_error_at(cup_fixture_fns::CUP_CPP, line);
            write_usize(p.addr() + id_at, usize::MAX);
        }
    }
    let soon = cup_date_plus(G_CURRENT_DATE, 2);
    let (soon_day, soon_year) = (read_i16(soon.as_ptr() as usize), read_u16(soon.as_ptr() as usize + 2) as i32);
    let due = || {
        let r = rr();
        let year = read_i8(r + cup_round::YEAR_OFFSET) as i32 + cup.year() as i32;
        year < soon_year || (year == soon_year && soon_day >= read_i16(r + cup_round::DAY))
    };
    let r = rr();
    if due() {
        covered!("gra_25 next round match due now");
        p.set_year(read_u16(G_CURRENT_DATE_YEAR));
        p.set_day(read_u16(G_CURRENT_DATE));
    } else {
        p.set_year((read_i8(r + cup_round::YEAR_OFFSET) as i16).wrapping_add(cup.year()) as u16);
        p.set_day(read_u16(r + cup_round::DAY));
    }
    p.set_time_slice(read_u8(r + cup_round::TIME_SLICE));
    exe::stadium_set_fixture_venue(read_usize(G_STADIUM), read_usize(r + cup_round::VENUE_VALUE), p, cup.venue_stadium_id());
    if due() {
        exe::fixture_manager_find_new_fixture_date(read_usize(G_FIXTURE_MANAGER), p);
    }
    cup_clear_fixture_result(p);
    p.set_stage_index(stage);
    p.set_home_result_mark(result_mark::NONE);
    p.set_away_result_mark(result_mark::NONE);
    add_fixture_to_calendar(p);
    if read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((comp_id as usize).wrapping_mul(4))) == 0 {
        return 1;
    }
    let (home, away) = (p.home_club(), p.away_club());
    if cup_has_human_manager(home.addr()) || cup_has_human_manager(away.addr()) {
        covered!("gra_25 next round news");
        let mut news = [0u32; 0x38]; // the local news item (0xe0 bytes)
        let n = news.as_mut_ptr() as usize;
        exe::new_newsitem(n, 0x14, 0);
        let r = rr();
        exe::newsitem_set_slot(n, 0, read_u32(read_usize(cup.addr() + league::COMPETITION)));
        exe::newsitem_set_slot(n, 1, cup.stage_label() as i32 as u32);
        exe::newsitem_set_slot(n, 2, read_i16(r + cup_round::ROUND_CODE) as i32 as u32);
        exe::newsitem_set_slot(n, 5, (read_u8(r + cup_round::LEGS) == 2) as u32);
        exe::newsitem_set_slot(n, 6, p.stadium_id() as u32);
        exe::newsitem_set_slot(n, 7, p.day() as u32);
        exe::newsitem_set_slot(n, 8, p.year() as u32);
        let (first, second) = if cup_has_human_manager(home.addr()) { (p.home_club_id(), p.away_club_id()) } else { (p.away_club_id(), p.home_club_id()) };
        exe::newsitem_set_slot(n, 3, first as u32);
        exe::newsitem_set_slot(n, 4, second as u32);
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, home);
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, away);
    }
    let r = rr();
    if read_u8(r + cup_round::LEGS) != 2 {
        return 1;
    }
    covered!("gra_25 next round second leg");
    if (read_u8(r + cup_round::SINGLE_LEG_FLAG) as i8) > 0 {
        p.set_flags(p.flags() | 4);
    } else {
        p.set_flags(p.flags() & !4);
    }
    p.set_round_name(read_i16(r + cup_round::ROUND_CODE).wrapping_add(4));
    let first_leg = make_date(p.day() as u32, p.year() as u32);
    let round_day = make_date(read_u16(r + cup_round::DAY) as u32, (read_i8(r + cup_round::YEAR_OFFSET) as i16).wrapping_add(cup.year()) as u16 as u32);
    let by_round = cup_date_plus(round_day.as_ptr() as usize, read_u8(rr() + cup_round::LEG_GAP_DAYS) as i32);
    let by_first_leg = cup_date_plus(first_leg.as_ptr() as usize, 3);
    let ((rd, ry), (fd, fy)) = (cup_date_day_year(&by_round), cup_date_day_year(&by_first_leg));
    if ry < fy || (ry == fy && (rd as i16) < (fd as i16)) {
        p.set_year(fy);
        p.set_day(fd);
        exe::fixture_manager_find_new_fixture_date(read_usize(G_FIXTURE_MANAGER), p);
    } else {
        p.set_year(ry);
        p.set_day(rd);
    }
    p.set_time_slice(cup_weekend_slice(make_date(p.day() as u32, p.year() as u32).as_ptr() as usize));
    cup_swap_fixture_sides(p);
    exe::stadium_set_fixture_venue(read_usize(G_STADIUM), read_usize(rr() + cup_round::VENUE_VALUE), p, cup.venue_stadium_id());
    p.set_flags(p.flags() & 0xf4);
    cup_clear_fixture_result(p);
    p.set_stage_index(stage);
    p.set_home_result_mark(result_mark::NONE);
    p.set_away_result_mark(result_mark::NONE);
    add_fixture_to_calendar(p);
    1
}

/// A tie decided (`cup_decision::HOME`/`AWAY`): the winner goes on, the loser goes out.
///
/// The winner and loser by `cup_tie_sides`. In the last tie of the tree (`cup_tie::FEEDER_TIE` −1): outcomes 1 and 2,
/// or 3 and 4 in the third-place round (`round_name::THIRD_PLACE_PLAYOFF`), the cup's result news for both
/// (`cup_announce_team`), their progress (`record_team_progress`), the round's prize money (`pay_round_prize_money`) and
/// the cup marked finished when every team has an outcome (`cup_all_teams_decided`). Otherwise the winner takes its side
/// of the next tie (`cup_tie::FEEDER_ROUND`/`FEEDER_TIE`; side A when `cup_tie::FEEDER_SIDE` is set, else side B) and
/// the news; in the semi-final (round = rounds − 3) of a cup with a third-place round (`cup_third_place_round`) the loser
/// takes the same side of the same tie there and the news with outcome −1, otherwise it is out (outcome 0xff) with the
/// news; then the progress of both and the prize money. When the round is before the last one drawn
/// (`next_draw_round()` − 1), the next round was drawn, its tie's fixture was due (`cup_tie::FIXTURE_STATE` 3) and both
/// its teams are known: the home sides of that tie (`exe::cup_set_home_teams_for_round`) and its match
/// (`cup_create_next_round_fixture`). Returns 1.
unsafe fn cup_advance_tie_winner(cup: Cup, f: Fixture, a2: u32, slot: i32, r: i32, feeder: (i32, i32), total: i32, comp_id: i32) -> i32 {
    let (feeder_tie, feeder_round) = feeder;
    let round = f.round_number();
    let third = cup_third_place_round(cup);
    let tie = round_tie(cup, round as i32, slot);
    let (w, l) = cup_tie_sides(cup, tie, f.addr(), r);
    covered!("gra_25 tie decided");
    let status = |t: i16| read_u8(team_entry(cup, t as i32) + cup_team_entry::STATUS);
    let pay = |w: i16, l: i16| {
        let f_: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize) = core::mem::transmute(cup_fixture_fns::PAY_ROUND_PRIZE_MONEY);
        f_(cup.addr(), team_entry(cup, w as i32), team_entry(cup, l as i32), round as u16 as u32, f.addr());
    };
    if feeder_tie == -1 {
        covered!("gra_25 final decided");
        let (sw, sl) = if read_i16(round_record(cup, round as i32) + cup_round::ROUND_CODE) == round_name::THIRD_PLACE_PLAYOFF {
            (cup_team_status::THIRD, cup_team_status::FOURTH)
        } else {
            (cup_team_status::WINNER, cup_team_status::RUNNER_UP)
        };
        write_u8(team_entry(cup, w as i32) + cup_team_entry::STATUS, sw);
        write_u8(team_entry(cup, l as i32) + cup_team_entry::STATUS, sl);
        cup_announce_team(cup, w, status(w) as u32, f.addr(), a2);
        cup_announce_team(cup, l, status(l) as u32, f.addr(), a2);
        cup_record_progress(cup, w, round);
        cup_record_progress(cup, l, round);
        pay(w, l);
        cup.set_run_by_stages(cup_all_teams_decided(total, |k| status(k)));
        return 1;
    }
    let side = if read_u16(tie + cup_tie::FEEDER_SIDE) != 0 { cup_tie::HOME_TEAM } else { cup_tie::AWAY_TEAM };
    write_i16(round_tie(cup, feeder_round, feeder_tie) + side, w);
    cup_announce_team(cup, w, status(w) as u32, f.addr(), a2);
    if round as i32 == cup.round_count() as i32 - 3 && third > -1 {
        covered!("gra_25 semi-final loser to the third-place match");
        write_i16(round_tie(cup, third, feeder_tie) + side, l);
        cup_announce_team(cup, l, u32::MAX, f.addr(), a2);
    } else {
        write_u8(team_entry(cup, l as i32) + cup_team_entry::STATUS, cup_team_status::OUT);
        cup_announce_team(cup, l, status(l) as u32, f.addr(), a2);
    }
    cup_record_progress(cup, w, round);
    cup_record_progress(cup, l, round);
    pay(w, l);
    let drawn = cup.next_draw_round() as i8 as i32;
    if round as i32 >= drawn - 1 || feeder_round >= drawn {
        return 1;
    }
    let next = round_tie(cup, feeder_round, feeder_tie);
    if read_u8(next + cup_tie::FIXTURE_STATE) != 3 {
        return 1;
    }
    if read_i16(next + cup_tie::HOME_TEAM) == -1 || read_i16(next + cup_tie::AWAY_TEAM) == -1 {
        return 1;
    }
    covered!("gra_25 next round match");
    exe::cup_set_home_teams_for_round(cup, feeder_round as u32, feeder_tie as u32);
    cup_create_next_round_fixture(cup, f, feeder_round, feeder_tie, comp_id)
}

/// Processes a played cup match: pays the match fee, stores the result in the tie, decides it and moves the cup on (the
/// second leg, a replay, a deciding match, or the winner to the next round and the loser out).
///
/// `Cup::process_fixture(fixture, a2)` (SEH frame without a destructor; slot `comp_vtable::ADD_RESULT`, called after
/// every cup match, also by the European cups' and the Intertoto's own `process_fixture`). A top-level cup
/// (`stage_index()` 0xff) handed a match of one of its stages (`fixture::STAGE_INDEX` ≠ 0xff) passes it to that stage's
/// `process_fixture` (the stage at `stages()`, index up to `current_stage()`; outside: the error box, line 0x473, 0); a
/// match of another stage: 0. The competition statistics record the match (`exe::comp_stats_record_match_result` on the
/// `league::COMP_STATS` of the cup's entry in [`G_COMP_OBJECT_TABLE`]). A match from before the cup's `year()`: −1. The
/// round's match fee (`cup_fixture_fields::ROUND_MATCH_PRIZE`, above 0) goes to both clubs' finances
/// (`exe::finance_add_prize_money`). The match's tie (`find_tree_slot_for_fixture`; none: the error box, line 0x4a3,
/// −1); its result block by the matches played (`cup_tie::REPLAYS_PLAYED` + `cup_tie::NEXT_MATCH`: 0, 1 or 2, otherwise
/// the box at line 0x4be and −1) takes the match's ten result bytes; the cup decides (`cup_fixture_decision`) and the
/// tie keeps the result marks in side order (`cup_tie_marks`). By the decision: −1 the box (line 0x4e6) and −1; a first
/// leg `cup_first_leg_played`; a draw `cup_end_on_draw` (line 0x51f) when the replays are exhausted, otherwise
/// `cup_schedule_replay`; a level tie `cup_end_on_draw` (line 0x5c7) or `cup_schedule_deciding_match`; a winner
/// `cup_advance_tie_winner`; anything else the box (line 0x78c) and −1.
/// Original: `Cup::process_fixture` (0x51a150, Cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn cup_process_fixture(this: usize, fixture: usize, a2: u32) -> i32 {
    if fixture == 0 {
        // the original reads the stage index (+0x42) of the null fixture before its box (line 0x467): it faults there
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(IDX_CUP_PROCESS_FIXTURE)(this, fixture, a2);
    }
    let cup = Cup(this);
    let f = Fixture(fixture);
    let stage = f.stage_index();
    let comp_id = read_i32(read_usize(this + league::COMPETITION));
    if cup.stage_index() == 0xff && stage != 0xff {
        if (stage as i8 as i32) <= cup.current_stage() as i32 && (stage as i8) >= 0 {
            covered!("gra_25 match of a stage");
            let s = read_usize(cup.stages() + 4 * stage as usize);
            let process: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(read_usize(read_usize(s) + comp_vtable::ADD_RESULT));
            return process(s, fixture, a2);
        }
        show_error_at(cup_fixture_fns::CUP_CPP, 0x473);
        return 0;
    }
    if cup.stage_index() != stage {
        return 0;
    }
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((comp_id as usize).wrapping_mul(4)));
    if object != 0 && read_usize(object + league::COMP_STATS) != 0 {
        exe::comp_stats_record_match_result(read_usize(object + league::COMP_STATS), fixture, a2 as usize, 0, 0);
    }
    if (f.year() as i32) < cup.year() as i32 {
        return -1;
    }
    let mut total = 0i32; // the teams of the whole cup (each round's new teams)
    for k in 0..(cup.round_count() as i32).max(0) {
        total = total.wrapping_add(read_u16(round_record(cup, k) + cup_round::NEW_TEAMS) as i32);
    }
    let round = f.round_number() as i32;
    let fee = read_i32(round_record(cup, round) + cup_fixture_fields::ROUND_MATCH_PRIZE);
    if fee > 0 {
        covered!("gra_25 match fee");
        exe::finance_add_prize_money(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), f.home_club()), fee);
        let fee = read_i32(round_record(cup, round) + cup_fixture_fields::ROUND_MATCH_PRIZE);
        exe::finance_add_prize_money(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), f.away_club()), fee);
    }
    let find: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(cup_fixture_fns::FIND_TREE_SLOT_FOR_FIXTURE);
    let slot = find(this, fixture);
    if slot == -1 {
        show_error_at(cup_fixture_fns::CUP_CPP, 0x4a3);
        return -1;
    }
    let tie = || round_tie(cup, round, slot);
    let feeder_tie = read_i16(tie() + cup_tie::FEEDER_TIE) as i32;
    let played = read_i8(tie() + cup_tie::REPLAYS_PLAYED) as i32 + read_i8(tie() + cup_tie::NEXT_MATCH) as i32;
    if !(0..3).contains(&played) {
        show_error_at(cup_fixture_fns::CUP_CPP, 0x4be);
        return -1;
    }
    core::ptr::copy_nonoverlapping((fixture + fixture::HOME_GOALS) as *const u8, (tie() + cup_fixture_fields::TIE_RESULT_BLOCKS[played as usize]) as *mut u8, 10);
    let feeder_round = read_i16(tie() + cup_tie::FEEDER_ROUND) as i32;
    let r = cup_fixture_decision(cup, round, tie(), fixture, played);
    let (a, b) = cup_tie_marks(cup, tie(), fixture);
    write_u8(tie() + cup_fixture_fields::TIE_HOME_MARK, a);
    write_u8(tie() + cup_fixture_fields::TIE_AWAY_MARK, b);
    match r {
        cup_decision::ERROR => {
            show_error_at(cup_fixture_fns::CUP_CPP, 0x4e6);
            -1
        }
        cup_decision::FIRST_LEG => cup_first_leg_played(cup, f, tie(), slot),
        cup_decision::DRAW if cup_replays_exhausted(cup, round, tie()) => cup_end_on_draw(cup, f, tie(), feeder_tie, total, 0x51f),
        cup_decision::DRAW => cup_schedule_replay(cup, f, slot, played),
        cup_decision::LEVEL if cup_replays_exhausted(cup, round, tie()) => cup_end_on_draw(cup, f, tie(), feeder_tie, total, 0x5c7),
        cup_decision::LEVEL => cup_schedule_deciding_match(cup, f, slot),
        cup_decision::HOME | cup_decision::AWAY => cup_advance_tie_winner(cup, f, a2, slot, r, (feeder_tie, feeder_round), total, comp_id),
        _ => {
            show_error_at(cup_fixture_fns::CUP_CPP, 0x78c);
            -1
        }
    }
}

/// What `cup_process_fixture` will do to the tie, worked out by the comparer from the state before.
struct CupFixturePlan {
    tie: usize,
    tie_after: [u8; 0x35],
    marks: (u8, u8),
    result: i32,
    statuses: Vec<(i16, u8)>,
    next: Option<(usize, usize, i16, bool)>, // the next tie, the winner's side, the winner, whether its home sides may be re-set
    third: Option<(usize, i16)>,
    finished: Option<u8>,
}

/// The comparer's plan for `cup_process_fixture` (None: a case it does not plan; only the original runs).
///
/// From the state before: the tie (`find_tree_slot_for_fixture`) and its matches played; the decision taken on copies
/// (the tie with the result block written and the fixture, 0x4f bytes, so the original's own decision is the only one
/// that touches the real fixture: with rule 0x2000 the decision changes the marks it reads); then, through the same
/// helpers as the copy (`cup_tie_marks`, `cup_tie_sides`, `cup_replays_exhausted`, `cup_third_place_round`,
/// `cup_all_teams_decided`), the tie afterwards (block, marks, next match, replays), the outcomes, the next tie's and the
/// third-place tie's side, the cup's finished mark and the return value.
unsafe fn cup_process_fixture_plan(cup: Cup, fixture: usize) -> Option<CupFixturePlan> {
    if fixture == 0 {
        return None;
    }
    let f = Fixture(fixture);
    if cup.stage_index() != f.stage_index() || (f.year() as i32) < cup.year() as i32 {
        return None;
    }
    let round = f.round_number() as i32;
    let slot = find_tree_slot_for_fixture(cup.addr(), fixture);
    if slot == -1 {
        return None;
    }
    let tie = round_tie(cup, round, slot);
    let played = read_i8(tie + cup_tie::REPLAYS_PLAYED) as i32 + read_i8(tie + cup_tie::NEXT_MATCH) as i32;
    if !(0..3).contains(&played) {
        return None;
    }
    let mut t = [0u8; 0x35];
    core::ptr::copy_nonoverlapping(tie as *const u8, t.as_mut_ptr(), 0x35);
    core::ptr::copy_nonoverlapping((fixture + fixture::HOME_GOALS) as *const u8, t.as_mut_ptr().add(cup_fixture_fields::TIE_RESULT_BLOCKS[played as usize]), 10);
    let mut fl = [0u8; fixture::SIZE];
    core::ptr::copy_nonoverlapping(fixture as *const u8, fl.as_mut_ptr(), fixture::SIZE);
    let (ta, fa) = (t.as_mut_ptr() as usize, fl.as_mut_ptr() as usize);
    let r = cup_fixture_decision(cup, round, ta, fa, played);
    let marks = (read_u8(fa + fixture::HOME_RESULT_MARK), read_u8(fa + fixture::AWAY_RESULT_MARK));
    let (a, b) = cup_tie_marks(cup, ta, fa);
    t[cup_fixture_fields::TIE_HOME_MARK] = a;
    t[cup_fixture_fields::TIE_AWAY_MARK] = b;
    let feeder_tie = read_i16(ta + cup_tie::FEEDER_TIE) as i32;
    let feeder_round = read_i16(ta + cup_tie::FEEDER_ROUND) as i32;
    let mut total = 0i32;
    for k in 0..(cup.round_count() as i32).max(0) {
        total = total.wrapping_add(read_u16(round_record(cup, k) + cup_round::NEW_TEAMS) as i32);
    }
    let mut plan = CupFixturePlan { tie, tie_after: t, marks, result: -1, statuses: Vec::new(), next: None, third: None, finished: None };
    let finished = |changes: &[(i16, u8)]| {
        cup_all_teams_decided(total, |k| changes.iter().find(|c| c.0 == k).map(|c| c.1).unwrap_or_else(|| read_u8(team_entry(cup, k as i32) + cup_team_entry::STATUS)))
    };
    match r {
        cup_decision::FIRST_LEG => {
            plan.tie_after[cup_tie::NEXT_MATCH] = 1;
            plan.result = 3;
        }
        cup_decision::DRAW | cup_decision::LEVEL if cup_replays_exhausted(cup, round, ta) => {
            if feeder_tie == -1 {
                plan.statuses = vec![(read_i16(ta + cup_tie::HOME_TEAM), 1), (read_i16(ta + cup_tie::AWAY_TEAM), 1)];
                plan.finished = Some(finished(&plan.statuses));
                plan.result = 1;
            } else {
                plan.result = 0;
            }
        }
        cup_decision::DRAW | cup_decision::LEVEL => {
            plan.tie_after[cup_tie::REPLAYS_PLAYED] = plan.tie_after[cup_tie::REPLAYS_PLAYED].wrapping_add(1);
            plan.result = if r == cup_decision::DRAW && played >= 2 { 0 } else { 1 };
        }
        cup_decision::HOME | cup_decision::AWAY => {
            let (w, l) = cup_tie_sides(cup, ta, fixture, r);
            plan.result = 1;
            if feeder_tie == -1 {
                let third_round = read_i16(round_record(cup, round) + cup_round::ROUND_CODE) == round_name::THIRD_PLACE_PLAYOFF;
                plan.statuses = if third_round { vec![(w, 3), (l, 4)] } else { vec![(w, 1), (l, 2)] };
                plan.finished = Some(finished(&plan.statuses));
            } else {
                let side = if read_u16(ta + cup_tie::FEEDER_SIDE) != 0 { cup_tie::HOME_TEAM } else { cup_tie::AWAY_TEAM };
                let next = round_tie(cup, feeder_round, feeder_tie);
                plan.next = Some((next, side, w, read_u8(next + cup_tie::FIXTURE_STATE) == 3));
                let third = cup_third_place_round(cup);
                if round == cup.round_count() as i32 - 3 && third > -1 {
                    plan.third = Some((round_tie(cup, third, feeder_tie) + side, l));
                } else {
                    plan.statuses = vec![(l, cup_team_status::OUT)];
                }
            }
        }
        _ => plan.result = -1,
    }
    Some(plan)
}

/// Comparer for `cup_process_fixture`: only the original runs (it adds fixtures, posts news, pays prize money and
/// records progress: it cannot run twice), checked against the plan made beforehand (`cup_process_fixture_plan`): the
/// return value, the whole tie (0x35 bytes: the result block, the marks in side order, the next match, the replays),
/// the fixture's result marks, the outcomes of the teams it decides, the winner in the next tie (on its side, or on
/// either when the next tie's home sides are set again), the loser in the third-place tie and the cup's finished mark.
/// A match of a stage, of another stage, from before the cup's year or without a tie: only the original.
pub(crate) unsafe extern "thiscall" fn compare_cup_process_fixture(this: usize, fixture: usize, a2: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(IDX_CUP_PROCESS_FIXTURE);
    let cup = Cup(this);
    let Some(plan) = cup_process_fixture_plan(cup, fixture) else {
        return o(this, fixture, a2);
    };
    let ret = o(this, fixture, a2);
    let mut tie = [0u8; 0x35];
    core::ptr::copy_nonoverlapping(plan.tie as *const u8, tie.as_mut_ptr(), 0x35);
    let marks = (read_u8(fixture + fixture::HOME_RESULT_MARK), read_u8(fixture + fixture::AWAY_RESULT_MARK));
    let mut wrong: Vec<String> = Vec::new();
    if ret != plan.result {
        wrong.push(format!("returned {} x plan {}", ret, plan.result));
    }
    if let Some(k) = (0..0x35).find(|&k| tie[k] != plan.tie_after[k]) {
        wrong.push(format!("tie +{:#x}: {:02x?} x plan {:02x?}", k, &tie[..], &plan.tie_after[..]));
    }
    if marks != plan.marks {
        wrong.push(format!("fixture marks {:?} x plan {:?}", marks, plan.marks));
    }
    for &(t, s) in &plan.statuses {
        let got = read_u8(team_entry(cup, t as i32) + cup_team_entry::STATUS);
        if got != s {
            wrong.push(format!("team {} outcome {} x plan {}", t, got, s));
        }
    }
    if let Some((next, side, w, may_swap)) = plan.next {
        let (h, a) = (read_i16(next + cup_tie::HOME_TEAM), read_i16(next + cup_tie::AWAY_TEAM));
        if read_i16(next + side) != w && !(may_swap && (h == w || a == w)) {
            wrong.push(format!("next tie {} {} x winner {} at +{}", h, a, w, side));
        }
    }
    if let Some((at, l)) = plan.third {
        if read_i16(at) != l {
            wrong.push(format!("third-place tie {} x loser {}", read_i16(at), l));
        }
    }
    if let Some(done) = plan.finished {
        if cup.run_by_stages() != done {
            wrong.push(format!("finished mark {} x plan {}", cup.run_by_stages(), done));
        }
    }
    SCOREBOARD.record(wrong.is_empty(), || format!("cup_process_fixture(cup {:#x}, fixture {:#x}, round {}) {:?}", this, fixture, Fixture(fixture).round_number(), wrong));
    ret
}

// ---------- batch 324: building the player statistics (player_stats.cpp) ----------

/// The CD check of the player statistics: among the drives (GetLogicalDriveStringsA, 200 bytes into `drives`) the first
/// CD drive (GetDriveTypeA 5) holding "cm0102.exe" (`exe::file_find_first` 0, then closed, `exe::file_find_close`);
/// none: the "Insert CD" box (langlib_sprintf of 0x987df4 and 0x987dc4, the box with 1) and again, forever. The buffer
/// keeps what the last GetLogicalDriveStringsA wrote: the original reuses those stack bytes as the language counters
/// of the history pass (`PlayerStatsHistoryPass::days`). cm0102-1080 has no CD requirement here: the jne on the drive
/// type (0x7bfdc4) became nops and the found test (0x7bfe06) became xor; the copy reads those bytes from the running exe.
/// Original: part of `PlayerStats::PlayerStats` (0x7bfd50, player_stats.cpp).
unsafe fn player_stats_check_cd(drives: &mut [u8; 0xc8]) {
    let drive_strings: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let drive_type: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = read_u8(0x7bfdc4) == 0x90;
    let always_found = read_u8(0x7bfe06) != 0x85; // the original has `test eax, eax` (85 c0)
    loop {
        if drive_strings(0xc8, drives.as_mut_ptr()) != 0 && drives[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = drives.as_ptr() as usize + i;
                if drive_type(root) == 5 || any_drive {
                    let mut name = [0u8; 0x104];
                    let mut found = [0u8; 0x140];
                    let nm = name.as_mut_ptr() as usize;
                    format(nm, 0x987e00, root, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    let f = found.as_mut_ptr() as usize;
                    if exe::file_find_first(nm, f, 0) == 0 || always_found {
                        return;
                    }
                    exe::file_find_close(f);
                }
                // simplification: the original walks with an i8 (steps of 4): it would only pass 127 with more than 32
                // drives, impossible
                i += 4;
                if i >= 0xc8 || drives[i] == 0 {
                    break;
                }
            }
        }
        let mut title = [0u8; 0x100];
        let mut msg = [0u8; 0x100];
        text(title.as_mut_ptr() as usize, 0x987df4); // "Insert CD"
        text(msg.as_mut_ptr() as usize, 0x987dc4); // "Please insert the CM01/02 CD and click on OK"
        exe::gui_show_message_box(title.as_ptr() as usize, msg.as_ptr() as usize, 1);
    }
}

/// The fluency a language weight or a days counter gives over the person's threshold: (w − threshold) / 50, at most 10
/// (16-bit values, signed).
fn player_stats_fluency_over(w: u16, threshold: u16) -> u32 {
    let v = (w as i16 as i32 - threshold as i16 as i32) / 50; // gra_27 the fluency divisor
    if v > 10 { 10 } else { v as u8 as u32 }
}

/// The person's language threshold: (20 − adaptability) × 10, in 16 bits.
unsafe fn player_stats_language_threshold(p: Staff) -> u16 {
    (20i16.wrapping_sub(p.adaptability() as i16)).wrapping_mul(10) as u16
}

/// Adds `fluency` to the person's language: with a language table, the record found by bsearch gains it (at most 10,
/// signed); without the table or the record, a new record with `sort` 1 (`exe::playerstats_add_language`).
unsafe fn player_stats_raise_language(this: usize, p: Staff, language: u8, fluency: u8) {
    if read_usize(this + player_stats_fields::LANGUAGES) != 0 {
        let r = find_language_record(this, p.addr(), language);
        if r != 0 {
            let f = read_u8(r + language_entry::FLUENCY).wrapping_add(fluency);
            write_u8(r + language_entry::FLUENCY, f);
            if (f as i8) > 10 {
                write_u8(r + language_entry::FLUENCY, 10);
            }
            return;
        }
    }
    exe::playerstats_add_language(this, p, language as u32, fluency as u32, 1);
}

/// The storage of a new game: the category records (100 of 0x20 bytes, malloc 0xc80), the per-player entries
/// (`G_PLAYER_DATA_COUNT` of 0x27 bytes: the eight dwords −1, the league games and goals and the ratings 0), the rating
/// averages (680.0 over 100.0 twice) and the empty language table. Out of memory: the fatal error box (lines 0x79, 0x85).
unsafe fn player_stats_init_storage(this: usize) {
    write_u32(this + player_stats_fields::CATEGORY_CAPACITY, 0x64);
    write_u32(this + player_stats_fields::CATEGORY_COUNT, 0);
    write_u32(this + player_stats_fields::FREE_CATEGORY_COUNT, 0);
    let c = exe::crt_malloc(0xc80);
    write_usize(this + player_stats_fields::CATEGORY_RECORDS, c);
    if c == 0 {
        show_player_stats_error(0x79);
        fatal_exit();
    }
    let n = read_i32(G_PLAYER_DATA_COUNT);
    write_i32(this + 0x34, n); // the number of per-player entries
    let e = exe::crt_malloc((n as u32).wrapping_mul(0x27));
    write_usize(this + player_stats_fields::PLAYER_ENTRIES, e);
    if e == 0 {
        show_player_stats_error(0x85);
        fatal_exit();
    }
    write_f64(this + player_stats_fields::RATING_SUM, 680.0);
    write_f64(this + player_stats_fields::RATED_COUNT, 100.0);
    write_f64(this + player_stats_fields::FLAGGED_RATING_SUM, 680.0);
    write_f64(this + player_stats_fields::FLAGGED_RATED_COUNT, 100.0);
    for k in 0..n.max(0) as usize {
        let r = e + k * 0x27;
        for j in 0..8 {
            write_u32(r + 4 * j, 0xffff_ffff);
        }
        write_u16(r + player_stats_entry::CLUB_LEAGUE_GAMES, 0);
        write_u16(r + player_stats_entry::CLUB_LEAGUE_GOALS, 0);
        write_u8(r + 0x26, 0);
        write_u8(r + 0x25, 0);
        write_u8(r + 0x24, 0);
    }
    write_u32(this + player_stats_fields::LANGUAGES, 0);
    write_u32(this + player_stats_fields::LANGUAGE_CAPACITY, 0);
    write_u32(this + player_stats_fields::LANGUAGE_COUNT, 0);
}

/// The languages every person starts with, and the last threshold computed (the original keeps it in a local the
/// history pass reads before setting it).
///
/// For each person: languages 1..64 by the weight of his two nations (`exe::player_stats_get_native_language_points`):
/// a weight that is not 0 and reaches the threshold (`player_stats_language_threshold`) adds the language with
/// `player_stats_fluency_over` (unsorted). A non-player (no `.player()`, with `.non_player()`) at a club whose nation is
/// not his own: the years at the club (low byte of the game's start year, `G_GAME_START_DATE_YEAR`, minus the low byte of
/// `.date_joined_club_year()`; negative or 0: nothing; at most 10) × adaptability × 0.05 (truncated) in the club nation's
/// first language (`player_stats_raise_language`). In the end the table is sorted (qsort with 0x7c6d40).
unsafe fn player_stats_initial_languages(this: usize) -> u16 {
    let mut threshold = 0u16;
    for i in 0..read_i32(G_STAFF_COUNT).max(0) as isize {
        let p = staff_at(i);
        threshold = player_stats_language_threshold(p);
        for k in 1..=0x40u8 {
            let w = exe::player_stats_get_native_language_points(p, k as i8) as u16;
            if w != 0 && (w as i16) >= (threshold as i16) {
                covered!("gra_27 native language");
                exe::playerstats_add_language(this, p, k as u32, player_stats_fluency_over(w, threshold), 0);
            }
        }
        if !p.player().is_null() || p.non_player().is_null() {
            continue;
        }
        let club = p.club_job();
        if club.is_null() {
            continue;
        }
        let nation = club.nation();
        if nation.is_null() || nation.addr() == p.nation().addr() {
            continue;
        }
        let years = read_u8(G_GAME_START_DATE_YEAR).wrapping_sub(p.date_joined_club_year() as u8) as i8;
        if years < 0 {
            continue;
        }
        let years = if years > 10 { 10 } else { years }; // gra_27 the years abroad
        if years == 0 {
            continue;
        }
        covered!("gra_27 language of the club abroad");
        let fluency = ftol((years as i32 * p.adaptability() as i32) as f64 * read_f64(rdata::F64_0_05)) as u8;
        player_stats_raise_language(this, p, nation.first_language() as u8, fluency);
    }
    exe::crt_qsort(read_usize(this + player_stats_fields::LANGUAGES), read_u32(this + player_stats_fields::LANGUAGE_COUNT), 6, 0x7c6d40);
    threshold
}

/// The state the history pass carries from record to record (registers and locals of the original).
struct PlayerStatsHistoryPass {
    /// The person of the previous record (0 none).
    previous: usize,
    /// The career list being built (`exe::new_llist`), 0 none.
    list: usize,
    /// The two nodes of the previous record (category 6 and category 2).
    nodes: [usize; 2],
    /// The previous record's year and the low word of its club id (only the year is compared).
    last_year: u32,
    /// The 0x27-byte entry of the current player.
    entry: usize,
    /// The language threshold of the person being counted.
    threshold: u16,
    /// Days per language 1..64 (index 0 unused): the original's stack buffer, which starts with the drive list left by
    /// the CD check and is only zeroed when a person with a player sheet begins.
    days: [u16; 0x41],
}

/// Writes the finished career list of the player to the history file (FILE_LLIST 0x58e9b0 on
/// `player_stats_fields::HISTORY_FILE`, the positions in the entry's `player_stats_entry::HISTORY_START` /
/// `player_stats_entry::HISTORY_LAST`) and destroys it.
unsafe fn player_stats_flush_history(this: usize, s: &mut PlayerStatsHistoryPass) {
    let write_list: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x58e9b0usize);
    write_list(read_usize(this + player_stats_fields::HISTORY_FILE), s.entry + player_stats_entry::HISTORY_START, s.entry + player_stats_entry::HISTORY_LAST, s.list);
    exe::drop_llist(s.list);
    exe::crt_operator_delete(s.list);
}

/// The days abroad of the person who has just finished (`s.previous`): for each language 1..64 while he has a nation,
/// at a club whose nation speaks it and his own nation does not, the counter becomes at least the days since he joined
/// that club (`exe::date_days_since` from `G_GAME_START_DATE`, asked twice when it wins, as in the original); each
/// counter that is not 0 and reaches his threshold raises the language by `player_stats_fluency_over`.
unsafe fn player_stats_languages_abroad(this: usize, s: &mut PlayerStatsHistoryPass) {
    let p = Staff(s.previous);
    for k in 1..=0x40u8 {
        if p.nation().is_null() {
            break;
        }
        let club = p.club_job();
        if !club.is_null() && read_u8(club.nation().addr() + nation::FIRST_LANGUAGE) == k && read_u8(p.nation().addr() + nation::FIRST_LANGUAGE) != k {
            let since = || exe::date_days_since(G_GAME_START_DATE, p.date_joined_club() as u32, p.date_joined_club_leap_year() as u32) as u16;
            let d = since();
            if !((s.days[k as usize] as i16) > (d as i16)) {
                s.days[k as usize] = since();
            }
        }
        let w = s.days[k as usize];
        if w != 0 && (w as i16) >= (s.threshold as i16) {
            covered!("gra_27 language from the career");
            player_stats_raise_language(this, p, k, player_stats_fluency_over(w, s.threshold) as u8);
        }
    }
}

/// One record of the history pass (17 bytes of staff_history.dat, already sorted by person and year), from 1900 up to
/// the year before the game's start (`G_GAME_START_DATE_YEAR`).
///
/// No person: the list of the previous player is written (only when there was one) and the previous person is
/// forgotten (the two nodes are not: the original keeps their pointers into the freed list). A person without a player
/// sheet only becomes the previous person. A new person: the list of the previous player is written; the days abroad
/// of the previous person (`player_stats_languages_abroad`); the counters are zeroed, the threshold is the new one's
/// and the entry is his player's. Then the record: a 0x2b-byte node (year, club id or −2, games, goals, fee −1, mark
/// 0x30) whose previous pair of nodes gets the loan bit of the record (+0xe: mark & 0xf9 | 1; otherwise & 0xf8); a
/// year earlier than the previous record's is the error box (line 0x1e0); the node goes into the list twice (category
/// 6, mark 0x30, and category 2, mark 0x10; `exe::llist_add_tail`). A club whose nation is neither of the person's two
/// and whose language is 1..64 adds 2 × (3 × (games + 30) − today's year + the year) days to that language, and for
/// the person's current club at least (today's year − the year he joined) × 365 − the day he joined + today's day, ×
/// 0.7; the current club also adds the games and goals into the entry. In `plan` the list (only marked as there), the
/// nodes and the box are skipped.
unsafe fn player_stats_history_record(this: usize, rec: usize, s: &mut PlayerStatsHistoryPass, plan: bool) {
    let year = read_u16(rec + staff_history::YEAR);
    if (year as i16) < 0x76c || (year as i16 as i32) >= read_u16(G_GAME_START_DATE_YEAR) as i32 {
        return;
    }
    let person = read_usize(rec + staff_history::STAFF_ID);
    if person == 0 {
        if s.previous != 0 && s.list != 0 {
            if !plan {
                player_stats_flush_history(this, s);
            }
            s.list = 0;
            s.last_year = 0;
            s.previous = 0;
        }
        return;
    }
    let p = Staff(person);
    if p.player().is_null() {
        s.previous = person;
        return;
    }
    if person != s.previous {
        if s.list != 0 {
            if !plan {
                player_stats_flush_history(this, s);
            }
            s.nodes = [0, 0];
            s.list = 0;
            s.last_year = 0;
        }
        s.entry = read_usize(this + player_stats_fields::PLAYER_ENTRIES).wrapping_add((read_i32(p.player().addr()) as usize).wrapping_mul(0x27));
        if s.previous != 0 {
            player_stats_languages_abroad(this, s);
        }
        s.days = [0; 0x41];
        s.threshold = player_stats_language_threshold(p);
    }
    let games = read_u8(rec + 0xf);
    let goals = read_u8(rec + 0x10);
    let mut node = [0u8; 0x2b];
    node[0..2].copy_from_slice(&year.to_le_bytes());
    let club = read_usize(rec + staff_history::CLUB);
    let id = if club != 0 { read_u32(club) } else { 0xffff_fffe };
    node[2..6].copy_from_slice(&id.to_le_bytes());
    node[0x14..0x16].copy_from_slice(&0xffffu16.to_le_bytes());
    node[0x2a] = 0x30;
    node[6] = games;
    node[8] = goals;
    node[0x26..0x2a].copy_from_slice(&0xffff_ffffu32.to_le_bytes());
    if !plan && s.nodes[0] != 0 && s.nodes[1] != 0 {
        for &n in &s.nodes {
            let b = read_u8(n + 0x2a);
            write_u8(n + 0x2a, if read_u8(rec + 0xe) != 0 { (b & 0xf9) | 1 } else { b & 0xf8 });
        }
    }
    if !plan && year < s.last_year as u16 {
        show_player_stats_error(0x1e0);
    }
    s.last_year = u32::from_le_bytes([node[0], node[1], node[2], node[3]]);
    if plan {
        s.list = 1; // the plan keeps no list, only that there is one (it decides the flushes and the forgetting)
    } else {
        if s.list == 0 {
            let l = exe::crt_operator_new(0xe);
            s.list = if l != 0 { exe::new_llist(l) } else { 0 };
        }
        node[0x2a] = (node[0x2a] & 7) | 0x30;
        s.nodes[0] = exe::llist_add_tail(s.list, node.as_ptr() as usize, 0x2b);
        node[0x2a] = (node[0x2a] & 7) | 0x10;
        s.nodes[1] = exe::llist_add_tail(s.list, node.as_ptr() as usize, 0x2b);
    }
    let own_club = p.club_job();
    if (id as i32) > -1 {
        let n = club_at(id as i32 as isize).nation();
        let second = p.second_nation();
        if !n.is_null() && n.addr() != p.nation().addr() && (second.is_null() || n.addr() != second.addr()) {
            let language = n.first_language();
            if language > 0 && language <= 0x40 {
                covered!("gra_27 days abroad from the history");
                let slot = &mut s.days[language as usize];
                let add = (3u32.wrapping_mul(games as u32 + 0x1e)).wrapping_sub(read_u16(G_CURRENT_DATE_YEAR) as u32).wrapping_add(year as u32).wrapping_mul(2);
                *slot = slot.wrapping_add(add as u16);
                if !own_club.is_null() && read_u32(own_club.addr()) == id {
                    let years = (read_u16(G_CURRENT_DATE_YEAR) as i32).wrapping_sub(p.date_joined_club_year() as i32);
                    let days = years.wrapping_mul(365).wrapping_sub(p.date_joined_club() as u16 as i16 as i32).wrapping_add(read_i16(G_CURRENT_DATE) as i32);
                    let scaled = days as f64 * read_f64(rdata::F64_0_7); // gra_27 the days at the current club
                    let have = *slot as i16 as f64;
                    *slot = ftol(if scaled > have { scaled } else { have }) as u16;
                }
            }
        }
    }
    if !own_club.is_null() && read_u32(own_club.addr()) == id {
        covered!("gra_27 games at the current club");
        let g = s.entry + player_stats_entry::CLUB_LEAGUE_GAMES;
        write_u16(g, read_u16(g).wrapping_add(games as u16));
        let a = s.entry + player_stats_entry::CLUB_LEAGUE_GOALS;
        write_u16(a, read_u16(a).wrapping_add(goals as u16));
    }
    s.previous = person;
}

/// The progress bar of the history passes: every 500 records, "Player histories" at `done` of `total`
/// (`exe::screenmanager_send_network_status_message`).
unsafe fn player_stats_history_progress(i: i32, done: i32, total: i32, label: &[u8; 0x100]) {
    if i % 500 == 0 {
        exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, label.as_ptr() as usize, done, total as u32, -1);
    }
}

/// The real player histories of a new game (staff_history.dat, `G_STAFF_HISTORY_TABLE`, `G_STAFF_HISTORY_COUNT` records
/// of 17 bytes), turned into career lists, games and goals at the current club, club career records and languages.
///
/// Only without "norealplhist" on the command line, with byte 0xb74348 clear and byte 0x9b5981 set; without the table:
/// the error box (line 0xef) and nothing else. (1) Sorted by club and person (qsort with 0x7c6d90); goals without games
/// become 0; each run of records of the same person at the same club (with a player sheet and a club) sums games and
/// goals and goes to the club's career records (0x448cb0 on `G_CLUB_RECORDS`, only the words are read); with
/// "checkgks" on the command line a goalkeeper with 6 goals or more in a record is the error box (line 0x139). (2)
/// Sorted by person and year (qsort with 0x7c6de0); a record of the same person and year as the next one, at another
/// club, when the record after that has the first club again (and the same person, another year) swaps with the next
/// one. (3) The pass of `player_stats_history_record` over every record, then the last list is written. (4) The table
/// is freed (0x527ee0). `table` is the table to work on (the game's, or the comparer's copy in `plan`, which skips the
/// progress bar, the career records, the boxes, the lists and the freeing).
unsafe fn player_stats_real_histories(this: usize, table: usize, threshold: u16, drives: &[u8; 0xc8], plan: bool) {
    if exe::crt_strstr(0xae2c98, 0xa78300) != 0 || read_u8(0xb74348) != 0 || read_u8(0x9b5981) == 0 {
        return;
    }
    if read_usize(G_STAFF_HISTORY_TABLE) == 0 {
        if !plan {
            show_player_stats_error(0xef);
        }
        return;
    }
    let mut label = [0u8; 0x100];
    if !plan {
        let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        text(label.as_mut_ptr() as usize, 0x9b5084); // "Player histories"
    }
    let count = || read_i32(G_STAFF_HISTORY_COUNT);
    let rec = |i: i32| table.wrapping_add((i as usize).wrapping_mul(0x11));
    exe::crt_qsort(table, count() as u32, 0x11, 0x7c6d90);
    let (mut person, mut club, mut games, mut goals) = (0usize, 0usize, 0u16, 0u16);
    let update_records: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x448cb0usize);
    let mut i = 0;
    while i <= count() - 1 {
        if !plan {
            player_stats_history_progress(i, i + 1, count().wrapping_mul(2), &label);
        }
        let r = rec(i);
        if read_u8(r + 0x10) != 0 && read_u8(r + 0xf) == 0 {
            write_u8(r + 0x10, 0);
        }
        let p = read_usize(r + staff_history::STAFF_ID);
        if p != 0 && !Staff(p).player().is_null() && read_usize(r + staff_history::CLUB) != 0 {
            let c = read_usize(r + staff_history::CLUB);
            if p != person || c != club {
                if person != 0 && club != 0 && !plan {
                    covered!("gra_27 career records of a club");
                    update_records(read_usize(G_CLUB_RECORDS), person, club, games as u32, goals as u32);
                }
                club = c;
                person = p;
                games = 0;
                goals = 0;
            }
            if !plan && exe::crt_strstr(0xae2c98, 0xa782f4) != 0 {
                let sheet = Staff(p).player();
                if !sheet.is_null() && exe::formation_get_position_mask(sheet) == 1 && read_u8(r + 0x10) >= 6 {
                    player_stats_goalkeeper_goals_box(p);
                }
            }
            games = games.wrapping_add(read_u8(r + 0xf) as u16);
            goals = goals.wrapping_add(read_u8(r + 0x10) as u16);
        }
        i += 1;
    }
    if person != 0 && club != 0 && !plan {
        update_records(read_usize(G_CLUB_RECORDS), person, club, games as u32, goals as u32);
    }
    exe::crt_qsort(table, count() as u32, 0x11, 0x7c6de0);
    let mut j = 1;
    while count() - 2 > 1 && j < count() - 2 {
        let (a, b, c) = (rec(j - 1), rec(j), rec(j + 1));
        let pa = read_u32(a + staff_history::STAFF_ID);
        let ya = read_u16(a + staff_history::YEAR);
        let ca = read_u32(a + staff_history::CLUB);
        if pa == read_u32(b + staff_history::STAFF_ID)
            && ya != read_u16(c + staff_history::YEAR)
            && ya == read_u16(b + staff_history::YEAR)
            && ca != read_u32(b + staff_history::CLUB)
            && ca == read_u32(c + staff_history::CLUB)
            && pa == read_u32(c + staff_history::STAFF_ID)
        {
            covered!("gra_27 season order swapped");
            let mut t = [0u8; 0x11];
            core::ptr::copy_nonoverlapping(a as *const u8, t.as_mut_ptr(), 0x11);
            core::ptr::copy_nonoverlapping(b as *const u8, a as *mut u8, 0x11);
            core::ptr::copy_nonoverlapping(t.as_ptr(), b as *mut u8, 0x11);
        }
        j += 1;
    }
    let mut s = PlayerStatsHistoryPass { previous: 0, list: 0, nodes: [0, 0], last_year: 0, entry: 0, threshold, days: [0; 0x41] };
    for k in 1..0x41usize {
        s.days[k] = u16::from_le_bytes([drives[2 * k - 2], drives[2 * k - 1]]);
    }
    let mut i = 0;
    while i < count() {
        if !plan {
            player_stats_history_progress(i, count() + i + 1, count().wrapping_mul(2), &label);
        }
        player_stats_history_record(this, rec(i), &mut s, plan);
        i += 1;
    }
    if s.list != 0 && !plan {
        player_stats_flush_history(this, &mut s);
    }
    if !plan {
        let free_history: unsafe extern "C" fn() = core::mem::transmute(0x527ee0usize);
        free_history();
    }
}

/// The "checkgks" box: "Too many goals for goalkeeper - " with the person's name (`exe::database_format_staff_name`,
/// 2000 bytes, mode 1) and the text at 0x99cae4, in the buffer at 0xde1f64, then the error box of line 0x139.
unsafe fn player_stats_goalkeeper_goals_box(person: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    format(0xde1f64, 0xa782d0);
    let len = |a: usize| (0..).take_while(|&k| read_u8(a + k) != 0).count();
    exe::database_format_staff_name(person, 0xde1f64 + len(0xde1f64), 0x7d0, 1);
    let end = 0xde1f64 + len(0xde1f64);
    let n = len(0x99cae4);
    core::ptr::copy_nonoverlapping(0x99cae4 as *const u8, end as *mut u8, n + 1);
    show_player_stats_error(0x139);
}

/// Builds the player statistics of a new game, or loads them from the saved game.
///
/// `PlayerStats::PlayerStats(path)`, the subsystem load (`setup.rs`, LoadStep 0x11): the CD check
/// (`player_stats_check_cd`); the history file (operator new of 8 bytes, FILE_LLIST 0x58e280 on "CM3_TEMP\player stats
/// history.tmp" with 0x2b-byte nodes, into `player_stats_fields::HISTORY_FILE`; failing: the fatal error box of line
/// 0x5e). With `path`, the statistics come from the saved game (0x7c48b0); when it answers, nothing else. Otherwise:
/// the storage (`player_stats_init_storage`), the languages every person starts with
/// (`player_stats_initial_languages`) and the real player histories (`player_stats_real_histories`). Returns `this`.
/// Original: `PlayerStats::PlayerStats` (0x7bfd50, player_stats.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_player_stats(this: usize, path: usize) -> usize {
    let mut drives = [0u8; 0xc8];
    player_stats_check_cd(&mut drives);
    let file = exe::crt_operator_new(8);
    let file = if file != 0 {
        let construct: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x58e280usize);
        construct(file, exe::utils_build_file_path(0x987da0, 0xa78340), 0x2b)
    } else {
        0
    };
    write_usize(this + player_stats_fields::HISTORY_FILE, file);
    if file == 0 {
        show_player_stats_error(0x5e);
        fatal_exit();
    }
    if path != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7c48b0usize);
        if load(this, path) != 0 {
            return this;
        }
    }
    player_stats_init_storage(this);
    let threshold = player_stats_initial_languages(this);
    player_stats_real_histories(this, read_usize(G_STAFF_HISTORY_TABLE), threshold, &drives, false);
    this
}

/// Comparer of `construct_player_stats` (comparison mode: see the header of motor.rs).
///
/// With a saved game only the original runs. In a new game the copy makes the plan first in a scratch object, with
/// the drive list of the moment and a copy of the history table (`plan`: no CD loop, no history file, no lists, no
/// career records, no boxes, the table not freed); then the original builds the real object (it sorts and frees the
/// game's table). Compared: the return, the capacities and counts, the four rating doubles, every per-player entry but
/// the two history positions, and the whole language table. The plan's arrays are freed.
pub(crate) unsafe extern "thiscall" fn compare_construct_player_stats(this: usize, path: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CONSTRUCT_PLAYER_STATS);
    if path != 0 {
        return o(this, path);
    }
    let drive_strings: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let mut drives = [0u8; 0xc8];
    drive_strings(0xc8, drives.as_mut_ptr());
    let mut plan = [0u8; 0x48];
    let po = plan.as_mut_ptr() as usize;
    let table = read_usize(G_STAFF_HISTORY_TABLE);
    let mut copy: Vec<u8> = Vec::new();
    if table != 0 {
        let n = (read_i32(G_STAFF_HISTORY_COUNT).max(0) as usize) * 0x11;
        copy.extend_from_slice(core::slice::from_raw_parts(table as *const u8, n));
    }
    player_stats_init_storage(po);
    let threshold = player_stats_initial_languages(po);
    player_stats_real_histories(po, copy.as_mut_ptr() as usize, threshold, &drives, true);
    let r = o(this, path);
    let mut wrong: Vec<String> = Vec::new();
    for off in [0x8usize, 0xc, 0x34, 0x38, 0x40, 0x44] {
        if read_u32(this + off) != read_u32(po + off) {
            wrong.push(format!("+{:#x} {} x {}", off, read_u32(this + off), read_u32(po + off)));
        }
    }
    for off in (0x10usize..0x30).step_by(8) {
        if read_u64(this + off) != read_u64(po + off) {
            wrong.push(format!("double +{:#x}", off));
        }
    }
    if read_u32(this + 0x34) == read_u32(po + 0x34) {
        let (eo, ep) = (read_usize(this + player_stats_fields::PLAYER_ENTRIES), read_usize(po + player_stats_fields::PLAYER_ENTRIES));
        for k in 0..read_i32(po + 0x34).max(0) as usize {
            let (a, b) = (eo + k * 0x27, ep + k * 0x27);
            let same = (0..0x18).chain(0x20..0x27).all(|j| read_u8(a + j) == read_u8(b + j));
            if !same && wrong.len() < 8 {
                wrong.push(format!("entry {} games {}/{} goals {}/{}", k, read_u16(a + 0x20), read_u16(b + 0x20), read_u16(a + 0x22), read_u16(b + 0x22)));
            }
        }
    }
    if read_u32(this + player_stats_fields::LANGUAGE_COUNT) == read_u32(po + player_stats_fields::LANGUAGE_COUNT) {
        let n = read_u32(po + player_stats_fields::LANGUAGE_COUNT) as usize * 6;
        let (lo, lp) = (read_usize(this + player_stats_fields::LANGUAGES), read_usize(po + player_stats_fields::LANGUAGES));
        if let Some(k) = (0..n).find(|&j| read_u8(lo + j) != read_u8(lp + j)) {
            wrong.push(format!("language record {} (person {}, language {}, fluency {} x {})", k / 6, read_u32(lo + k / 6 * 6), read_u8(lo + k / 6 * 6 + 4), read_u8(lo + k / 6 * 6 + 5), read_u8(lp + k / 6 * 6 + 5)));
        }
    }
    for off in [player_stats_fields::CATEGORY_RECORDS, player_stats_fields::PLAYER_ENTRIES, player_stats_fields::LANGUAGES] {
        if read_usize(po + off) != 0 {
            exe::crt_free(read_usize(po + off));
        }
    }
    SCOREBOARD.record(r == this && wrong.is_empty(), || format!("construct_player_stats({:#x}) {:?}", this, wrong));
    r
}

// ---------- batch 324: the scout's working day (player_search.cpp) ----------

/// Owner of the search: the manager's staff index (dword at +0 of PLAYER_SEARCH).
const SCOUT_DAY_OWNER: usize = 0x0;
/// Signed byte at +4: the list's slot, the first field of every scout news item.
const SCOUT_DAY_SLOT: usize = 0x4;
/// Assignment: the club to report on (id, −1 none).
const SCOUT_DAY_TARGET_CLUB: usize = 0x1a1;
/// Reports already made in this assignment (byte, counts the scout's working days).
const SCOUT_DAY_REPORTS_DONE: usize = 0x1b5;
/// The date the competition assignment watches next (TCMDate).
const SCOUT_DAY_COMPETITION_DATE: usize = 0x1be;
/// Capacity of the result array (`player_search_fields::RESULTS`).
const SCOUT_DAY_RESULT_CAPACITY: usize = 0x1f2;
/// Dword the working day clears after every report.
const SCOUT_DAY_REPORT_MARK: usize = 0x2a4;
/// The PLAYER_SEARCH bytes the comparer saves (the routine writes up to +0x2a7).
const SCOUT_DAY_OBJECT_SIZE: usize = 0x2a8;
/// Size of a search result.
const SCOUT_DAY_RESULT_SIZE: usize = 0x2d;

/// How one working day runs: for the game, or as the comparer's dry run (news kept instead of delivered, the knowledge
/// records saved before they change, the reassignment not made).
struct ScoutDayRun {
    dry: bool,
    /// The news items the dry run would deliver: kind (+0..+3) and data (+5..+0xcc).
    news: Vec<[u8; 0xcd]>,
    /// The fog of war records the dry run touched, with their bytes before.
    knowledge: Vec<(usize, [u8; 9])>,
    /// The dry run reached the end of the assignment (start_scout_assignment or 0x7aee80).
    reassigned: bool,
}

impl ScoutDayRun {
    /// Delivers the news item to the manager (`exe::news_deliver_to_manager` on `G_NEWS`, free 0); dry: keeps it.
    unsafe fn deliver(&mut self, item: usize, manager: Staff) {
        if self.dry {
            let mut b = [0u8; 0xcd];
            core::ptr::copy_nonoverlapping(item as *const u8, b.as_mut_ptr(), 0xcd);
            self.news.push(b);
            return;
        }
        exe::news_deliver_to_manager(read_usize(G_NEWS), item, manager, 0);
    }

    /// The manager's fog of war gets to know the player through the scout (`exe::fog_of_war_map_raise_knowledge`); dry:
    /// the 9-byte record is saved first.
    unsafe fn raise_knowledge(&mut self, map: usize, player: Staff, scout: Staff) {
        if self.dry && !player.is_null() && read_i32(player.addr()) < read_i32(G_STAFF_COUNT) {
            let rec = read_usize(map).wrapping_add((read_i32(player.addr()) as usize).wrapping_mul(9));
            let mut b = [0u8; 9];
            core::ptr::copy_nonoverlapping(rec as *const u8, b.as_mut_ptr(), 9);
            self.knowledge.push((rec, b));
        }
        exe::fog_of_war_map_raise_knowledge(map, player, scout);
    }
}

/// The player's club by his contract (`exe::contract_manager_get_contract` on `G_CONTRACT_MANAGER`, the answer kept in
/// `G_CONTRACT_LOOKUP_RESULT`): the club of the contract's `contract::CLUB` id, or null.
unsafe fn scout_day_contract_club(player: Staff) -> Club {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c != 0 { club_at(read_i32(c + contract::CLUB) as isize) } else { Club(0) }
}

/// Removes result `r` by copying the last one over it (the two dates by `exe::date_operator_3d`); one result fewer.
unsafe fn scout_day_remove_result(this: usize, r: usize) {
    let n = read_i32(this + player_search_fields::RESULT_COUNT);
    let last = read_usize(this + player_search_fields::RESULTS).wrapping_add(((n - 1) as usize).wrapping_mul(SCOUT_DAY_RESULT_SIZE));
    for off in [0usize, 4, 8, 0xc] {
        write_u32(r + off, read_u32(last + off));
    }
    let mut tmp = [0u32; 2];
    exe::date_operator_3d(r + 0x10, tmp.as_mut_ptr() as usize, last + 0x10);
    exe::date_operator_3d(r + search_result::LAST_UPDATE_DATE, tmp.as_mut_ptr() as usize, last + search_result::LAST_UPDATE_DATE);
    write_u8(r + search_result::FLAGS, read_u8(last + search_result::FLAGS));
    write_u32(r + 0x21, read_u32(last + 0x21));
    write_u32(r + 0x25, read_u32(last + 0x25));
    for off in 0x29..0x2d {
        write_u8(r + off, read_u8(last + off));
    }
    write_i32(this + player_search_fields::RESULT_COUNT, n - 1);
}

/// Whether result `r` stays in the scout's list: the scout has a club and the player a player sheet; with the
/// opposition flag, always; otherwise the player's nation (that of his club, or his own) has to be the assigned nation
/// and region; for the base region only players under 20 without a club, from the region of the scout club's nation
/// or worth more than 40000 (+0x25); from the scout club's nation, only those below 4000 of reputation for the scout
/// (`exe::database_get_staff_reputation_for_viewer`); never a player of the scout's own club (`scout_day_contract_club`)
/// nor one about to retire (`exe::player_regen_get_retirement_status`).
unsafe fn scout_day_keeps(this: usize, r: usize, player: Staff, scout: Staff) -> bool {
    let mut nation = Nation(0);
    let club = player.club_job();
    if !club.is_null() {
        nation = club.nation();
    }
    if nation.is_null() {
        nation = player.nation();
    }
    let scout_club = scout.club_job();
    if scout_club.is_null() || player.player().is_null() {
        return false;
    }
    if read_u32(this + player_search_fields::ASSIGNED_OPPONENT) != 0 {
        return true;
    }
    let assigned = read_i32(this + player_search_fields::ASSIGNED_NATION);
    if assigned > -1 && !nation.is_null() && read_i32(nation.addr()) != assigned {
        return false;
    }
    let region = read_i8(this + player_search_fields::ASSIGNED_REGION);
    if region > -1 && !nation.is_null() && nation.actual_region() as i8 != region {
        return false;
    }
    if read_u32(this + player_search_fields::ASSIGNED_BASE) != 0 {
        if player.age() as i8 >= 20 {
            return false;
        }
        let same_region = !nation.is_null() && nation.actual_region() == scout_club.nation().actual_region();
        if !same_region && read_i32(r + 0x25) <= 40000 {
            return false;
        }
        if !player.club_job().is_null() {
            return false;
        }
    }
    if nation.addr() == scout.club_job().nation().addr() && exe::database_get_staff_reputation_for_viewer(player, scout, 0) >= 4000 {
        return false;
    }
    let club = scout_day_contract_club(player);
    if club.addr() == scout.club_job().addr() {
        return false;
    }
    exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), player) == 0
}

/// One player reported (results `r` with bit 0 of `search_result::FLAGS`): the result is dated today and loses the bit;
/// one fewer to report, one more reported. The work permit for the scout's club (`exe::transfer_manager_player_needs_work_permit`
/// and, when needed, `exe::transfer_manager_is_work_permit_granted` in the club nation). Without the opposition flag,
/// a player graded 4 or more (now or potential, +0x29/+0x2a) gets the report news 0x138b in `item`: slot, the scout,
/// age, positions, the player, his club id (−1 none), the two grades, the assignment (nation, region, competition,
/// base), three zeros, the work permit (1 and the answer when needed; otherwise 2 for a player of membership group 2
/// by his second nation when the scout club's region is 14, else 0, and 0), the two words of 0x7bd9d0, 0x420e9 and the
/// player's strength flags (`exe::staff_get_player_strength_flags`); delivered to the manager. `SCOUT_DAY_REPORT_MARK`
/// is cleared.
#[allow(clippy::too_many_arguments)]
unsafe fn scout_day_report(this: usize, run: &mut ScoutDayRun, r: usize, player: Staff, club: Club, scout: Staff, manager: Staff, left: &mut i32, reported: &mut i32) {
    exe::date_operator_3d(r + search_result::LAST_UPDATE_DATE, [0u32; 2].as_mut_ptr() as usize, G_CURRENT_DATE);
    write_u8(r + search_result::FLAGS, read_u8(r + search_result::FLAGS) & 0xfe);
    *left -= 1; // gra_26 one fewer to report
    *reported += 1;
    let scout_club = scout.club_job();
    let needed = exe::transfer_manager_player_needs_work_permit(read_usize(G_TRANSFER_MANAGER), player, scout_club, 1);
    let granted = if needed == 0 { 1 } else { (exe::transfer_manager_is_work_permit_granted(read_usize(G_TRANSFER_MANAGER), player, scout_club.nation(), 0, 0, 1) != 0) as u32 };
    if read_u32(this + player_search_fields::ASSIGNED_OPPONENT) == 0 {
        let (now, potential) = (read_i8(r + 0x29), read_i8(r + 0x2a));
        if (granted == 0 && now >= 5) || potential >= 5 || now >= 4 || potential >= 4 {
            covered!("gra_26 player report");
            let mut item = [0u8; 0x100];
            let it = item.as_mut_ptr() as usize;
            exe::new_newsitem(it, 0x138b, 0);
            let mut n: u8 = 0;
            let age = player.age() as i8;
            let positions = exe::formation_get_position_mask(player.player()) as i16;
            let mut name = [0u8; 0x20];
            exe::fill_staff_name_ref(player, name.as_mut_ptr() as usize);
            let club_id = if club.is_null() { -1 } else { read_i32(club.addr()) };
            let slot = |n: &mut u8, v: u32| {
                exe::newsitem_set_slot(it, *n as i8, v);
                *n = n.wrapping_add(1);
            };
            slot(&mut n, read_i8(this + SCOUT_DAY_SLOT) as i32 as u32);
            exe::write_staff_name_ref_to_news(this + player_search_fields::SCOUT_NAME_REF, it, &mut n);
            slot(&mut n, age as i32 as u32);
            slot(&mut n, positions as i32 as u32);
            exe::write_staff_name_ref_to_news(name.as_ptr() as usize, it, &mut n);
            slot(&mut n, club_id as u32);
            slot(&mut n, now as i32 as u32);
            slot(&mut n, potential as i32 as u32);
            slot(&mut n, read_u32(this + player_search_fields::ASSIGNED_NATION));
            slot(&mut n, read_i8(this + player_search_fields::ASSIGNED_REGION) as i32 as u32);
            slot(&mut n, read_u32(this + player_search_fields::ASSIGNED_COMP));
            slot(&mut n, read_u32(this + player_search_fields::ASSIGNED_BASE));
            slot(&mut n, 0);
            slot(&mut n, 0);
            slot(&mut n, 0);
            if needed != 0 {
                slot(&mut n, 1);
                slot(&mut n, granted);
            } else {
                let second = player.second_nation();
                let group2 = player.nation().group_membership() != 2 && !second.is_null() && second.group_membership() == 2 && scout_club.nation().actual_region() == 0xe;
                slot(&mut n, if group2 { 2 } else { 0 });
                slot(&mut n, 0);
            }
            let mut words = [0i16; 2];
            let words_of: unsafe extern "C" fn(usize, usize, *mut i16, *mut i16) = core::mem::transmute(0x7bd9d0usize);
            words_of(player.addr(), scout.addr(), words.as_mut_ptr().add(1), words.as_mut_ptr());
            slot(&mut n, words[1] as i32 as u32);
            slot(&mut n, words[0] as i32 as u32);
            slot(&mut n, 0x420e9);
            // the index byte is taken before the strength flags are asked (the original bumps it first; nothing reads it after)
            exe::newsitem_set_slot(it, n as i8, exe::staff_get_player_strength_flags(player, club));
            run.deliver(it, manager);
        }
    }
    write_u32(this + SCOUT_DAY_REPORT_MARK, 0);
}

/// Whether result `r` goes into the final report's best players: a player with a player sheet graded above 3 now (or 3
/// with a reputation above 0x1a5e) who needs no work permit for `club`.
unsafe fn scout_day_shortlisted(r: usize, club: usize) -> bool {
    let p = staff_at(read_i32(r + 0xc) as isize);
    if p.is_null() || p.player().is_null() {
        return false;
    }
    let grade = read_i8(r + 0x29);
    let good = (read_i16(p.player().addr() + player_sheet::CURRENT_REPUTATION) > 0x1a5e && grade == 3) || grade > 3;
    good && exe::transfer_manager_player_needs_work_permit(read_usize(G_TRANSFER_MANAGER), p, club, 0) == 0
}

/// The 0x1a-byte flags of a shortlisted player (attributes 20 marks positions, the extremes of the attributes, the
/// goals per game in the attack, the form against the average rating, youth, age, a new arrival and injury).
unsafe fn scout_day_player_flags(p: Staff, pd: usize) -> u32 {
    let b = |o: usize| read_i8(pd + o);
    let mut f = 0u32;
    if b(player_sheet::DEFENDER) == 20 {
        f = 0x200;
    }
    if b(player_sheet::ATTACKER) == 20 {
        f |= if b(player_sheet::CENTRAL) == 20 {
            0x100
        } else if b(player_sheet::ATTACKING_MIDFIELDER) == 20 || b(player_sheet::MIDFIELDER) == 20 {
            0x1000
        } else if b(player_sheet::DEFENDER) == 20 || b(player_sheet::DEFENSIVE_MIDFIELDER) == 20 {
            0x2000
        } else {
            0x100
        };
    }
    if b(player_sheet::ATTACKING_MIDFIELDER) == 20 || b(player_sheet::DEFENSIVE_MIDFIELDER) == 20 || b(player_sheet::MIDFIELDER) == 20 {
        f |= if b(player_sheet::DEFENDER) == 20 { 0x2000 } else { 0x400 };
        if b(player_sheet::LEFT_SIDE) == 20 || b(player_sheet::RIGHT_SIDE) == 20 {
            f |= 0x1000;
        }
    }
    if b(player_sheet::GOALKEEPER) == 20 {
        f |= 0x800;
    }
    let jumping = b(player_sheet::JUMPING);
    if jumping > 18 && b(player_sheet::HEADING) > 14 && b(player_sheet::STRENGTH) > 15 {
        f |= 0x20;
    } else if jumping < 6 {
        f |= 0x10;
    }
    if b(player_sheet::WORK_RATE) > 14 && b(player_sheet::AGGRESSION) > 10 && b(player_sheet::TACKLING) > 14 {
        f |= 2;
    }
    let pace = b(player_sheet::PLAYER_PACE);
    if pace > 18 {
        f |= 4;
    } else if pace < 6 {
        f |= 8;
    }
    let consistency = b(player_sheet::CONSISTENCY);
    if consistency > 15 {
        f |= 0x80000;
    } else if consistency < 5 {
        f |= 0x400000;
    }
    if b(player_sheet::LEADERSHIP) > 14 {
        f |= 0x80;
    }
    let stats = read_usize(G_PLAYER_STATS);
    let games = exe::playerstats_get_stat(stats, pd, 6, 1, 6.5);
    if games > read_f32(rdata::F32_5) as f64 {
        let games = if games > 1.0 { games } else { 1.0 };
        let goals = exe::playerstats_get_stat(stats, pd, 6, 2, 6.5);
        let ratio = goals / games;
        if b(player_sheet::ATTACKER) > 15 {
            if ratio < read_f64(rdata::F64_0_25) {
                f |= 0x200000;
            } else if ratio > read_f64(rdata::F64_0_65) {
                f |= 0x100000;
            }
        }
        exe::playerstats_get_stat(stats, pd, 6, 0x11, 6.5);
        let count = read_f64(stats + player_stats_fields::RATED_COUNT);
        let average = if count == 0.0 || count.is_nan() { read_f64(rdata::F64_6_8) } else { read_f64(stats + player_stats_fields::RATING_SUM) / count } as f32;
        let form = exe::playerstats_get_recent_form(stats, pd) as i32 as f64 * read_f64(rdata::F64_0_2);
        if form > average as f64 + read_f64(rdata::F64_0_25) {
            f |= 0x800000;
        } else if form < average as f64 - read_f64(rdata::F64_0_35) {
            f |= 0x1000000;
        }
    }
    if b(player_sheet::VISION) > 14 {
        f |= 1;
    }
    if b(player_sheet::STRENGTH) > 14 {
        f |= 0x4000;
    }
    if (p.age() as i8) < 22 && read_i16(pd + player_sheet::CURRENT_REPUTATION) < 0x1676 {
        f |= 0x8000;
    }
    if p.age() as i8 > 34 {
        f |= 0x10000;
    }
    if exe::date_days_since(G_CURRENT_DATE, p.date_joined_club() as u32, p.date_joined_club_leap_year() as u32) < 0x41 {
        f |= 0x20000;
    }
    if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
        f |= 0x40000;
    }
    f
}

/// The final report's best players of the target club's search: how many there are (only those
/// `scout_day_shortlisted` for `club`), the first two after the sort (qsort with 0x7be640) as seven slots each (+0x21 of
/// the result, 0 when the player's contract happiness is below 25; the player; the first three dwords of the result; the
/// flags of `scout_day_player_flags`; his current ability). Out of memory: the error box (line 0xd72) and the day ends
/// with 0 (`None`).
unsafe fn scout_day_best_players(this: usize, item: usize, n: &mut u8, club: usize) -> Option<()> {
    let count = || read_i32(this + player_search_fields::RESULT_COUNT);
    let result = |j: i32| read_usize(this + player_search_fields::RESULTS).wrapping_add((j as usize).wrapping_mul(SCOUT_DAY_RESULT_SIZE));
    let mut found = 0i32;
    let mut j = 0;
    while j < count() {
        if scout_day_shortlisted(result(j), club) {
            found += 1;
        }
        j += 1;
    }
    let mut shown = found;
    if found != 0 {
        let buf = exe::crt_malloc((found as u32).wrapping_mul(0x1a));
        if buf == 0 {
            show_error_at(0xa701b0, 0xd72);
            return None;
        }
        let mut filled = 0i32;
        let mut j = 0;
        while j < count() && filled < found {
            let r = result(j);
            if scout_day_shortlisted(r, club) {
                let p = staff_at(read_i32(r + 0xc) as isize);
                let e = buf + filled as usize * 0x1a;
                write_u32(e, read_u32(r + 0x21));
                let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
                if c != 0 && read_i8(c + contract::HAPPINESS) < 0x19 {
                    write_u32(e, 0);
                }
                write_u32(e + 4, read_u32(r + 0xc));
                write_u32(e + 8, read_u32(r));
                write_u32(e + 0xc, read_u32(r + 4));
                write_u32(e + 0x10, read_u32(r + 8));
                write_u16(e + 0x18, read_u16(p.player().addr() + player_sheet::CURRENT_ABILITY));
                write_u32(e + 0x14, 0);
                let pd = p.player().addr();
                if pd != 0 {
                    covered!("gra_26 best players of the club");
                    write_u32(e + 0x14, scout_day_player_flags(p, pd));
                }
                filled += 1;
            }
            j += 1;
        }
        if filled != found {
            show_error_at(0xa701b0, 0xe36);
        }
        exe::crt_qsort(buf, found as u32, 0x1a, 0x7be640);
        if shown > 2 {
            shown = 2;
        }
        exe::newsitem_set_slot(item, *n as i8, shown as u32);
        *n = n.wrapping_add(1);
        let mut k = 0;
        while k < shown && k < 2 {
            let e = buf + k as usize * 0x1a;
            for off in [0usize, 4, 8, 0xc, 0x10, 0x14] {
                exe::newsitem_set_slot(item, *n as i8, read_u32(e + off));
                *n = n.wrapping_add(1);
            }
            exe::newsitem_set_slot(item, *n as i8, read_i16(e + 0x18) as i32 as u32);
            *n = n.wrapping_add(1);
            k += 1;
        }
        exe::crt_free(buf);
    } else {
        exe::newsitem_set_slot(item, *n as i8, 0);
        *n = n.wrapping_add(1);
    }
    Some(())
}

/// The last day of the assignment: the summary news 0x138a (slot, the scout, players reported, 1, the assignment's six
/// fields and the target club); with a target club its best players (`scout_day_best_players`) and, when the manager
/// has a club, fifteen answers about the target club (eleven Database.cpp questions 0x549100..0x5493c0, the manager's
/// club id, the target's reputation, 0x549a30 between the two, byte +4 of its club state row); otherwise fourteen zeros.
/// Delivered when there are results (and, with the opposition flag, a target club); otherwise the scout starts the same
/// assignment again (`exe::player_search_start_scout_assignment`); without the opposition flag the assignment then
/// ends (0x7aee80). `None`: out of memory, the day ends with 0.
unsafe fn scout_day_final_report(this: usize, run: &mut ScoutDayRun, manager: Staff, own: Club, reported: i32, permit_club: usize) -> Option<()> {
    covered!("gra_26 final report");
    let mut item = [0u8; 0x100];
    let it = item.as_mut_ptr() as usize;
    exe::new_newsitem(it, 0x138a, 0);
    exe::newsitem_set_slot(it, 0, read_i8(this + SCOUT_DAY_SLOT) as i32 as u32);
    let mut n: u8 = 1;
    exe::write_staff_name_ref_to_news(this + player_search_fields::SCOUT_NAME_REF, it, &mut n);
    let opponent = read_u32(this + player_search_fields::ASSIGNED_OPPONENT);
    for v in [
        reported as u32,
        1,
        read_u32(this + player_search_fields::ASSIGNED_NATION),
        read_i8(this + player_search_fields::ASSIGNED_REGION) as i32 as u32,
        read_u32(this + player_search_fields::ASSIGNED_COMP),
        read_u32(this + player_search_fields::ASSIGNED_BASE),
        opponent,
        read_u32(this + SCOUT_DAY_TARGET_CLUB),
    ] {
        exe::newsitem_set_slot(it, n as i8, v);
        n = n.wrapping_add(1);
    }
    let mut target = Club(0);
    let target_id = read_i32(this + SCOUT_DAY_TARGET_CLUB);
    if target_id != -1 {
        target = club_at(target_id as isize);
        // the club the work permit is asked for is whatever the original left in [esp+0x30]: the club of the last
        // player reported today, or (with nobody reported) the count of flagged results, 0 then
        scout_day_best_players(this, it, &mut n, permit_club)?;
    }
    if target_id != -1 && !own.is_null() {
        covered!("gra_26 target club answers");
        let q = |a: usize| -> u32 {
            let f: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(a);
            f(target.addr()) as i8 as i32 as u32
        };
        let mut answers = [0u32; 15];
        for (k, a) in [0x549100usize, 0x5492c0, 0x549200, 0x549470, 0x5495b0, 0x5494d0, 0x549690, 0x549770, 0x549940, 0x549850, 0x5493c0].iter().enumerate() {
            answers[k] = q(*a);
        }
        answers[11] = read_u32(own.addr());
        answers[12] = target.reputation() as i32 as u32;
        let between: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x549a30usize);
        answers[13] = between(target.addr(), own.addr()) as i8 as i32 as u32;
        answers[14] = read_i8(read_usize(G_CLUB_STATE_TABLE).wrapping_add((read_i32(target.addr()) as usize).wrapping_mul(6)) + 4) as i32 as u32;
        for v in answers {
            exe::newsitem_set_slot(it, n as i8, v);
            n = n.wrapping_add(1);
        }
    } else {
        for _ in 0..14 {
            exe::newsitem_set_slot(it, n as i8, 0);
            n = n.wrapping_add(1);
        }
    }
    if (opponent == 0 || !target.is_null()) && read_i32(this + player_search_fields::RESULT_COUNT) != 0 {
        run.deliver(it, manager);
    } else if run.dry {
        run.reassigned = true;
    } else {
        covered!("gra_26 assignment started again");
        exe::player_search_start_scout_assignment(
            this,
            read_u32(this + player_search_fields::ASSIGNED_NATION),
            read_u8(this + player_search_fields::ASSIGNED_REGION) as u32,
            read_u32(this + player_search_fields::ASSIGNED_COMP),
            read_u32(this + player_search_fields::ASSIGNED_BASE),
            opponent,
        );
    }
    if read_u32(this + player_search_fields::ASSIGNED_OPPONENT) == 0 {
        if run.dry {
            run.reassigned = true;
        } else {
            let end_assignment: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7aee80usize);
            end_assignment(this);
        }
    }
    Some(())
}

/// The scout's working day for a human manager's search list (list type 4).
///
/// `PLAYER_SEARCH::run_scout_day()`, called by the scouts' day (`scout_manager.rs`) for each of the human managers'
/// lists. 0 when the list is not of type 4, without a scout (`player_search_fields::SCOUT_ID`) whose name still matches
/// (`exe::database_staff_matches_names`) or without any assignment (nation, region, competition, base, opposition). 1
/// without doing anything until the day of `player_search_fields::SCOUTING_END_DATE`. On that day: the first day
/// (`SCOUT_DAY_REPORTS_DONE` 0) is the news 0x1389 (slot, the scout and the four assignment fields). Then: the days of
/// the assignment are 1 (competition or opposition) or 8; the results are tidied (0x7b1520); each result shows the
/// player to the manager's fog of war (`G_ATTRIBUTE_MASKING_ENABLED`) and stays only when `scout_day_keeps` (otherwise
/// the last one takes its place); the kept ones with bit 0 of `search_result::FLAGS` are the flagged; sorted (qsort with
/// 0x7bce50), or the array freed when empty. How many to report today: before the last day (50 + 75..124)% × results /
/// (days × 100), on the last day all the flagged; at least 1 and at most the flagged. Each: a random flagged result
/// (`exe::utils_random` of the count until one with the bit), `scout_day_report`, and every other flagged result of the
/// same club right after it (even past the number). On the last day the summary (`scout_day_final_report`); halfway
/// (days / 2) the news 0x138a with the count, 0 and the six assignment fields. In the end the next working day: two
/// days after `SCOUT_DAY_COMPETITION_DATE` for a competition, otherwise in 2..8 days (`exe::utils_random`(7) + 2); one
/// more day done; 1.
/// Original: `PLAYER_SEARCH::run_scout_day` (0x7b3e20, player_search.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn run_scout_day(this: usize) -> u32 {
    let mut run = ScoutDayRun { dry: false, news: Vec::new(), knowledge: Vec::new(), reassigned: false };
    scout_day(this, &mut run)
}

/// The working day itself (`run_scout_day`), for the game or as the comparer's dry run.
unsafe fn scout_day(this: usize, run: &mut ScoutDayRun) -> u32 {
    if read_u8(this + player_search_fields::LIST_TYPE) != 4 {
        return 0;
    }
    let scout_id = read_i32(this + player_search_fields::SCOUT_ID);
    if scout_id <= -1 {
        return 0;
    }
    let scout = staff_at(scout_id as isize);
    if exe::database_staff_matches_names(scout, read_u32(this + 0x174), read_u32(this + 0x178), read_u32(this + 0x17c), this + 0x184) == 0 || scout.is_null() {
        return 0;
    }
    if read_i32(this + player_search_fields::ASSIGNED_NATION) == -1
        && read_u8(this + player_search_fields::ASSIGNED_REGION) == 0xff
        && read_i32(this + player_search_fields::ASSIGNED_COMP) == -1
        && read_u32(this + player_search_fields::ASSIGNED_BASE) == 0
        && read_u32(this + player_search_fields::ASSIGNED_OPPONENT) == 0
    {
        return 0;
    }
    let manager = staff_at(read_i32(this + SCOUT_DAY_OWNER) as isize);
    let own = manager.club_job();
    if read_u16(this + player_search_fields::SCOUTING_END_YEAR) != read_u16(G_CURRENT_DATE_YEAR) || read_u16(this + player_search_fields::SCOUTING_END_DATE) != read_u16(G_CURRENT_DATE) {
        return 1;
    }
    let done = read_i8(this + SCOUT_DAY_REPORTS_DONE);
    if done == 0 {
        covered!("gra_26 first day");
        let mut item = [0u8; 0x100];
        let it = item.as_mut_ptr() as usize;
        exe::new_newsitem(it, 0x1389, 0);
        exe::newsitem_set_slot(it, 0, read_i8(this + SCOUT_DAY_SLOT) as i32 as u32);
        let mut n: u8 = 1;
        exe::write_staff_name_ref_to_news(this + player_search_fields::SCOUT_NAME_REF, it, &mut n);
        for v in [
            read_u32(this + player_search_fields::ASSIGNED_NATION),
            read_i8(this + player_search_fields::ASSIGNED_REGION) as i32 as u32,
            read_u32(this + player_search_fields::ASSIGNED_COMP),
            read_u32(this + player_search_fields::ASSIGNED_BASE),
        ] {
            exe::newsitem_set_slot(it, n as i8, v);
            n = n.wrapping_add(1);
        }
        run.deliver(it, manager);
        return scout_day_next_day(this);
    }
    let days: i32 = if read_i32(this + player_search_fields::ASSIGNED_COMP) > -1 || read_u32(this + player_search_fields::ASSIGNED_OPPONENT) != 0 { 1 } else { 8 };
    let mut flagged = 0i32;
    // the original's [esp+0x30]: the count of flagged results, then the club of each reported player's run; the final
    // report asks the work permit for it
    let mut permit_club = 0usize;
    let tidy: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x7b1520usize);
    tidy(this, 0);
    let count = || read_i32(this + player_search_fields::RESULT_COUNT);
    let result = |j: i32| read_usize(this + player_search_fields::RESULTS).wrapping_add((j as usize).wrapping_mul(SCOUT_DAY_RESULT_SIZE));
    if count() != 0 {
        let mut i = 0;
        while i < count() {
            let r = result(i);
            let player = staff_at(read_i32(r + 0xc) as isize);
            if read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 {
                let map = exe::fog_of_war_get_manager_map(read_usize(G_FOG_OF_WAR), manager);
                if map != 0 {
                    run.raise_knowledge(map, player, scout);
                }
            }
            if scout_day_keeps(this, r, player, scout) {
                if read_u8(r + search_result::FLAGS) & 1 != 0 {
                    flagged += 1;
                    permit_club = permit_club.wrapping_add(1);
                }
            } else {
                covered!("gra_26 result dropped");
                scout_day_remove_result(this, r);
                i -= 1;
            }
            i += 1;
        }
        if count() != 0 {
            exe::crt_qsort(read_usize(this + player_search_fields::RESULTS), count() as u32, SCOUT_DAY_RESULT_SIZE as u32, 0x7bce50);
        } else {
            exe::crt_free(read_usize(this + player_search_fields::RESULTS));
            write_usize(this + player_search_fields::RESULTS, 0);
            write_u32(this + SCOUT_DAY_RESULT_CAPACITY, 0);
        }
    }
    let wanted = if (done as i32) < days {
        (exe::utils_random(50) + 0x4b).wrapping_mul(count()) / (days * 100) // gra_26 how many today
    } else {
        flagged
    };
    let mut left = if wanted.max(1) >= flagged {
        flagged
    } else if wanted <= 1 {
        1
    } else {
        wanted
    };
    let mut reported = count() - flagged;
    while left > 0 {
        let (r, player, club) = loop {
            let r = result(exe::utils_random(count()));
            let player = staff_at(read_i32(r + 0xc) as isize);
            let club = scout_day_contract_club(player);
            if read_u8(r + search_result::FLAGS) & 1 != 0 {
                break (r, player, club);
            }
        };
        scout_day_report(this, run, r, player, club, scout, manager, &mut left, &mut reported);
        let same = scout_day_contract_club(staff_at(read_i32(r + 0xc) as isize));
        permit_club = same.addr();
        if same.is_null() {
            continue;
        }
        let mut j = 0;
        while j < count() {
            let rj = result(j);
            if scout_day_contract_club(staff_at(read_i32(rj + 0xc) as isize)).addr() == same.addr() {
                let pj = staff_at(read_i32(rj + 0xc) as isize);
                let cj = scout_day_contract_club(pj);
                if read_u8(rj + search_result::FLAGS) & 1 != 0 {
                    covered!("gra_26 same club reported");
                    scout_day_report(this, run, rj, pj, cj, scout, manager, &mut left, &mut reported);
                }
            }
            j += 1;
        }
    }
    if done as i32 == days {
        if scout_day_final_report(this, run, manager, own, reported, permit_club).is_none() {
            return 0;
        }
    } else if done as i32 == days / 2 {
        covered!("gra_26 halfway news");
        let mut item = [0u8; 0x100];
        let it = item.as_mut_ptr() as usize;
        exe::new_newsitem(it, 0x138a, 0);
        exe::newsitem_set_slot(it, 0, read_i8(this + SCOUT_DAY_SLOT) as i32 as u32);
        let mut n: u8 = 1;
        exe::write_staff_name_ref_to_news(this + player_search_fields::SCOUT_NAME_REF, it, &mut n);
        for v in [
            reported as u32,
            0,
            read_u32(this + player_search_fields::ASSIGNED_NATION),
            read_i8(this + player_search_fields::ASSIGNED_REGION) as i32 as u32,
            read_u32(this + player_search_fields::ASSIGNED_COMP),
            read_u32(this + player_search_fields::ASSIGNED_BASE),
            read_u32(this + player_search_fields::ASSIGNED_OPPONENT),
            read_u32(this + SCOUT_DAY_TARGET_CLUB),
        ] {
            exe::newsitem_set_slot(it, n as i8, v);
            n = n.wrapping_add(1);
        }
        run.deliver(it, manager);
    }
    scout_day_next_day(this)
}

/// The next working day: two days after `SCOUT_DAY_COMPETITION_DATE` for a competition assignment, otherwise
/// `exe::utils_random`(7) + 2 days from today (`exe::date_add_days`, copied into
/// `player_search_fields::SCOUTING_END_DATE`); one more day done (`SCOUT_DAY_REPORTS_DONE`). Returns 1.
unsafe fn scout_day_next_day(this: usize) -> u32 {
    let mut out = [0u32; 2];
    let mut tmp = [0u32; 2];
    let d = if read_i32(this + player_search_fields::ASSIGNED_COMP) > -1 {
        exe::date_add_days(this + SCOUT_DAY_COMPETITION_DATE, out.as_mut_ptr() as usize, 2)
    } else {
        let n = exe::utils_random(7) + 2; // gra_26 the next working day
        exe::date_add_days(G_CURRENT_DATE, out.as_mut_ptr() as usize, n as u32)
    };
    exe::date_operator_3d(this + player_search_fields::SCOUTING_END_DATE, tmp.as_mut_ptr() as usize, d);
    write_u8(this + SCOUT_DAY_REPORTS_DONE, read_u8(this + SCOUT_DAY_REPORTS_DONE).wrapping_add(1));
    1
}

/// Comparer of `run_scout_day` (comparison mode: see the header of motor.rs), as a dry run (model
/// `compare_post_recovery_news`).
///
/// The copy runs first from the state of the moment as a dry run: on a copy of the result array (allocated by the game's
/// malloc, which the routine may free), keeping the news items instead of delivering them, saving each fog of war
/// record before it changes and without restarting or ending the assignment. Then the PLAYER_SEARCH object, the
/// records, the random generator and `G_CONTRACT_LOOKUP_RESULT` go back and the original runs. Compared: the return;
/// each news item the original delivered into the manager's inbox ring (kind and data) against the dry run's, in order;
/// the fog of war records; and, unless the assignment was restarted or ended, the object (all but the array pointer),
/// the result array, the generator and the contract lookup.
pub(crate) unsafe extern "thiscall" fn compare_run_scout_day(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_RUN_SCOUT_DAY);
    let mut regions: Vec<(usize, usize)> = Vec::from(crate::rng::STATE_REGIONS);
    regions.push((G_CONTRACT_LOOKUP_RESULT, 4));
    regions.push((this, SCOUT_DAY_OBJECT_SIZE));
    let before = crate::comparison::MemorySnapshot::read(&regions);
    let results = read_usize(this + player_search_fields::RESULTS);
    if results != 0 {
        let n = read_i32(this + player_search_fields::RESULT_COUNT).max(1) as usize * SCOUT_DAY_RESULT_SIZE;
        let copy = exe::crt_malloc(n as u32);
        core::ptr::copy_nonoverlapping(results as *const u8, copy as *mut u8, n);
        write_usize(this + player_search_fields::RESULTS, copy);
    }
    let mut run = ScoutDayRun { dry: true, news: Vec::new(), knowledge: Vec::new(), reassigned: false };
    let rd = crate::rng::pause_trace(|| crate::rng::without_counting(|| scout_day(this, &mut run)));
    let dry_state = crate::comparison::MemorySnapshot::read(&regions);
    let dry_results: Vec<u8> = {
        let a = read_usize(this + player_search_fields::RESULTS);
        let n = read_i32(this + player_search_fields::RESULT_COUNT).max(0) as usize * SCOUT_DAY_RESULT_SIZE;
        let v = if a != 0 { core::slice::from_raw_parts(a as *const u8, n).to_vec() } else { Vec::new() };
        if a != 0 {
            exe::crt_free(a);
        }
        v
    };
    let dry_knowledge: Vec<[u8; 9]> = run.knowledge.iter().map(|(a, _)| {
        let mut b = [0u8; 9];
        core::ptr::copy_nonoverlapping(*a as *const u8, b.as_mut_ptr(), 9);
        b
    }).collect();
    for (a, b) in run.knowledge.iter().rev() {
        core::ptr::copy_nonoverlapping(b.as_ptr(), *a as *mut u8, 9);
    }
    before.write_back();
    let news = read_usize(G_NEWS);
    let manager = read_i32(this + SCOUT_DAY_OWNER);
    let idx = manager.wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10);
    let inbox = if idx >= 0 && news != 0 { read_usize(news + news_fields::HUMAN_INBOXES) + idx as usize * 12 } else { 0 };
    let next_before = if inbox != 0 && read_i32(inbox) != -1 { Some(read_i32(inbox + 4)) } else { None };
    let r = o(this);
    let mut wrong: Vec<String> = Vec::new();
    if r != rd {
        wrong.push(format!("returned {} x {}", r, rd));
    }
    if !run.news.is_empty() {
        if inbox == 0 || read_i32(inbox) == -1 {
            wrong.push(format!("{} news items expected, no inbox", run.news.len()));
        } else {
            let start = read_i32(inbox);
            let mut slot = next_before.unwrap_or(start);
            for (k, expected) in run.news.iter().enumerate() {
                let item = read_usize(news).wrapping_add((slot as usize).wrapping_mul(NEWS_NEWS_RECORD_SIZE));
                let got = core::slice::from_raw_parts(item as *const u8, 0xcd);
                if got[0..4] != expected[0..4] || got[5..] != expected[5..] {
                    let field = (5..0xcd).find(|&j| got[j] != expected[j]).map(|j| (j - 5) / 4);
                    wrong.push(format!("news {} kind {:#x} x {:#x}, first different field {:?}", k, read_u32(item), u32::from_le_bytes([expected[0], expected[1], expected[2], expected[3]]), field));
                }
                slot += 1;
                if slot == start + 0x64 {
                    slot = start;
                }
            }
        }
    }
    for (k, (a, _)) in run.knowledge.iter().enumerate() {
        if core::slice::from_raw_parts(*a as *const u8, 9) != dry_knowledge[k] {
            wrong.push(format!("fog of war record {:#x}", a));
        }
    }
    if !run.reassigned {
        let after = crate::comparison::MemorySnapshot::read(&regions);
        let pointer = this + player_search_fields::RESULTS;
        for ((a, o), (_, d)) in after.0.iter().zip(dry_state.0.iter()) {
            let differs = (0..o.len()).any(|j| o[j] != d[j] && !(*a == this && (pointer - this..pointer - this + 4).contains(&j)));
            if differs {
                wrong.push(format!("region {:#x}", a));
            }
        }
        let a = read_usize(pointer);
        let n = read_i32(this + player_search_fields::RESULT_COUNT).max(0) as usize * SCOUT_DAY_RESULT_SIZE;
        let got: Vec<u8> = if a != 0 { core::slice::from_raw_parts(a as *const u8, n).to_vec() } else { Vec::new() };
        if got != dry_results {
            wrong.push("the result array".into());
        }
    }
    SCOREBOARD.record(wrong.is_empty(), || format!("run_scout_day(search {:#x}, day {}) {:?}", this, read_i8(this + SCOUT_DAY_REPORTS_DONE), wrong));
    r
}
