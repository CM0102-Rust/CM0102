// ---------- batch %%LEVA%%: the news text of a transfer set to complete (transfer_manager.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_transfer_finalised_news_text(this, news, full, t), || format!("write_transfer_finalised_news_text(news {:#x}, full {})", news, full));
}
