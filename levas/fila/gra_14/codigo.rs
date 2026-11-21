// ---------- batch %%LEVA%%: the news text of an offer made for a person (transfer_manager.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_offer_made_news_text(this, news, full, t), || format!("write_offer_made_news_text(news {:#x}, full {})", news, full));
}
