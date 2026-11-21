// ---------- batch %%LEVA%%: the news text of a completed signing (transfer_manager.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_transfer_completed_news_text(this, news, full, t), || format!("write_transfer_completed_news_text(news {:#x}, full {})", news, full));
}
