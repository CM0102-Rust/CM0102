// ---------- batch %%LEVA%%: the news text of a player's answer to a contract (transfer_manager.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_contract_response_news_text(this, news, full, t), || format!("write_contract_response_news_text(news {:#x}, full {})", news, full));
}
