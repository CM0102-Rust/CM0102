/// The names of up to three exchange players of a news item, read from the slots after `first` (four slots each: id
/// and the three name ids), and the list text the transfer news templates take.
pub(crate) struct NewsPlayerList {
    /// How many players the news item names (its count slot; only the first three are read).
    pub(crate) count: i8,
    pub(crate) ids: [u32; 3],
    pub(crate) first_names: [u32; 3],
    pub(crate) second_names: [u32; 3],
    pub(crate) common_names: [u32; 3],
    pub(crate) names: [[u8; 0x12c]; 3],
    /// "A", "A and B", "A, B and C" or "<number> players" (0x190 bytes, empty without players).
    pub(crate) text: [u8; 0x190],
}

impl NewsPlayerList {
    /// Reads the players from the news item: `count` players from slot `first` on (mode 1 names) and the list text
    /// (one name copied, the two- and three-name templates 0xa91c40 and 0xa91bc0, or the number in words
    /// (`exe::utils_number_to_words`) in 0xa91ba0 for more than three).
    pub(crate) unsafe fn read(news: usize, first: u32, count: i8) -> NewsPlayerList {
        let mut l = NewsPlayerList {
            count,
            ids: [0; 3],
            first_names: [0; 3],
            second_names: [0; 3],
            common_names: [0; 3],
            names: [[0; 0x12c]; 3],
            text: [0; 0x190],
        };
        l.text[0] = read_u8(0xad9c64); // the original starts the list text with the empty string's byte
        if count == 0 {
            return l;
        }
        let mut slot = first;
        for k in 0..core::cmp::min(count, 3) as usize {
            l.ids[k] = exe::newsitem_get_slot(news, slot);
            l.first_names[k] = exe::newsitem_get_slot(news, slot + 1);
            l.second_names[k] = exe::newsitem_get_slot(news, slot + 2);
            l.common_names[k] = exe::newsitem_get_slot(news, slot + 3);
            slot += 4;
            news_text_person_name(l.first_names[k], l.second_names[k], l.common_names[k], &mut l.names[k], 1, l.ids[k]);
        }
        let fmt = news_text_format();
        let text = l.text.as_mut_ptr() as usize;
        let name = |k: usize| l.names[k].as_ptr() as usize;
        match count {
            1 => {
                let n = l.names[0].iter().position(|&b| b == 0).unwrap_or(0x12b);
                l.text[..=n].copy_from_slice(&l.names[0][..=n]);
            }
            2 => {
                fmt(text, 0xa91c40, name(0), name(1)); // "<name 1> and <name 2>"
            }
            3 => {
                fmt(text, 0xa91bc0, name(0), name(1), name(2)); // "<name 1>, <name 2> and <name 3>"
            }
            _ => {
                let mut words = [0u8; 0x34];
                exe::utils_number_to_words(count as i32, words.as_mut_ptr(), 0x32, 1);
                fmt(text, 0xa91ba0, words.as_ptr() as usize); // "<number> players"
            }
        }
        l
    }

    /// Links each named player (one to three of them) to his record when the item's names still match him.
    pub(crate) unsafe fn link(&self, out: usize) {
        if self.count > 0 && self.count <= 3 {
            for k in 0..self.count as usize {
                news_text_link_person(out, &self.names[k], self.ids[k], self.first_names[k], self.second_names[k], self.common_names[k], 0x7d7);
            }
        }
    }
}

/// Writes the headline or the body of the news that a transfer or a player swap was agreed, with the date it goes
/// through.
///
/// `write_transfer_agreed_news_text(news, full, out)`: the person of slot 0 (name from slots 1-3; mode 1 in the body,
/// 5 in the headline), the buying club of slot 4, the selling club of slot 5 (−1 none), the cash part of slot 6 and
/// the total value of slot 8 (both read as floats), the date in slots 10 (day) and 11 (year), and the exchange players
/// from slot 13 (`NewsPlayerList`, count in slot 12).
///
/// The body: when the value is above the cash, the "in a deal worth <value>" templates (0xa9c0c0, or 0xa9bfc4 "plus
/// <players>"); otherwise, with cash above 0, "for <cash>" (0xa9bf10, or 0xa9bd20 with the players); without cash, the
/// swap template 0xa9bc3c with players, else 0xa9be7c (both clubs) or 0xa9be10 (no selling club). The sums go through
/// `news_text_money`; the club names are the short names with their gender, and the person's name follows the grammar
/// marker 0x64. Then "The transfer will go through on <date>" (0xa9bbec, `exe::date_get_long_date_string`) and the
/// links: the buyer, the seller when there is one, the person and the named exchange players (0x7d7).
/// The headline: "<buyer> to sign <name>" (0xa9bb9c) with the links to the buyer and the person.
///
/// Defect of the original: the "deal worth" templates read the selling club without checking slot 5, so a value
/// above the cash with no selling club (−1) would print the record before the club table; it does not happen in the
/// transfers that post this news. The copy keeps it.
/// Original: `TRANSFER_MANAGER::write_transfer_agreed_news_text` (0x8e08e0, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_transfer_agreed_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    if full == 0 {
        let (person, s1, s2, s3, s4) = (slot(0), slot(1), slot(2), slot(3), slot(4));
        news_text_person_name(s1, s2, s3, &mut name, 5, person);
        let buyer = club_at(s4 as i32 as isize);
        let g = buyer.short_gender_name() as i8 as i32;
        fmt(NEWS_TEXT_BUFFER, 0xa9bb9c, g, g, buyer.addr() + club::SHORT_NAME, np); // "<club> to sign <name>"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_club(out, s4);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
        return;
    }
    let (person, s1, s2, s3, s4, s5) = (slot(0), slot(1), slot(2), slot(3), slot(4), slot(5));
    let cash = slot(6) as i32 as f32;
    let value = slot(8) as i32 as f32;
    let (day, year) = (slot(10), slot(11));
    let players = NewsPlayerList::read(news, 0xd, slot(12) as i8);
    news_text_person_name(s1, s2, s3, &mut name, 1, person);
    let mut date = [0u8; 8];
    exe::new_date(date.as_mut_ptr() as usize, day, year);
    let mut when = [0u8; 0xc8];
    exe::date_get_long_date_string(date.as_ptr() as usize, when.as_mut_ptr() as usize, 0xc8, 0xff);
    let buyer = club_at(s4 as i32 as isize);
    let seller = club_at(s5 as i32 as isize);
    let g4 = buyer.short_gender_name() as i8 as i32;
    let g5 = seller.short_gender_name() as i8 as i32;
    let (n4, n5) = (buyer.addr() + club::SHORT_NAME, seller.addr() + club::SHORT_NAME);
    let list = players.text.as_ptr() as usize;
    let mut money = [0u8; 0x34];
    let mp = money.as_ptr() as usize;
    if value > cash {
        news_text_money(value as i32 as u32, &mut money);
        if players.count == 0 {
            covered!("rx05_5 deal worth");
            fmt(NEWS_TEXT_BUFFER, 0xa9c0c0, 0x64, g4, g4, g5, g5, np, n4, n5, mp); // rx05_5: in a deal worth
        } else {
            fmt(NEWS_TEXT_BUFFER, 0xa9bfc4, 0x64, g4, g4, g5, g5, np, n4, n5, mp, list);
        }
    } else {
        news_text_money(cash as i32 as u32, &mut money);
        let paid = cash > read_f32(rdata::F32_0);
        if players.count == 0 {
            if paid {
                fmt(NEWS_TEXT_BUFFER, 0xa9bf10, 0x64, g4, g4, g5, g5, np, n4, n5, mp);
            } else if s5 as i32 != -1 {
                fmt(NEWS_TEXT_BUFFER, 0xa9be7c, 0x64, g4, g4, g5, g5, np, n4, n5);
            } else {
                fmt(NEWS_TEXT_BUFFER, 0xa9be10, 0x64, g4, g4, np, n4);
            }
        } else if paid {
            fmt(NEWS_TEXT_BUFFER, 0xa9bd20, 0x64, g4, g4, g5, g5, np, n4, n5, mp, list);
        } else {
            covered!("rx05_5 player swap agreed");
            fmt(NEWS_TEXT_BUFFER, 0xa9bc3c, 0x64, g4, g4, g5, g5, np, n4, n5, list);
        }
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    fmt(NEWS_TEXT_BUFFER, 0xa9bbec, 0x64, when.as_ptr() as usize); // "The transfer will go through on <date>."
    news_text_append(out, NEWS_TEXT_BUFFER);
    news_text_link_club(out, s4);
    if s5 as i32 != -1 {
        news_text_link_club(out, s5);
    }
    news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
    players.link(out);
}

/// Comparer for `write_transfer_agreed_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_transfer_agreed_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_transfer_agreed_news_text(this, news, full, t), || format!("write_transfer_agreed_news_text(news {:#x}, full {})", news, full));
}
