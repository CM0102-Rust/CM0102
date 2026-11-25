/// The argument pattern of a collapsed transfer template, by the clubs it names: the buyer `B` (slot 1), the seller
/// `S` (slot 0) and the third club `N` (slot 6), each as its short-name gender twice and its short name, with the
/// grammar marker 0x64 for the name. `SBN` with `Nation` adds two more markers and the buyer's nationality adjective.
#[derive(Clone, Copy)]
enum CollapsedArgs {
    /// 0x64, B, the name, B.
    B,
    /// 0x64, B, B, the name, B, B.
    BB,
    /// 0x64, B, N, the name, B, N.
    BN,
    /// 0x64, S, B, the name, S, B.
    SB,
    /// 0x64, S, B, B, the name, S, B, B.
    SBB,
    /// 0x64, S, B, S, the name, S, B, S.
    SBS,
    /// 0x64, S, B, N, the name, S, B, N.
    SBN,
    /// 0x64, S, B, N, 0x64, 0x64, the name, S, B, N, the adjective.
    SBNNation,
    /// N, 0x64, N, the name.
    N,
    /// N, 0x64, S, N, the name, S.
    NS,
    /// N, 0x64, B, N, the name, B.
    NB,
}

/// The body template of a collapsed transfer by the reason (slot 7 as a word), the loan flag (slot 8 ≠ 0, reason 0xb
/// only), whether there is a seller (slot 0 ≠ −1) and whether the third club is the buyer (reasons 0x17 and 0x18).
fn offer_collapsed_template(reason: i16, loan: bool, seller: bool, third_is_buyer: bool) -> (usize, CollapsedArgs) {
    use CollapsedArgs::*;
    let pick = |with: (usize, CollapsedArgs), without: (usize, CollapsedArgs)| if seller { with } else { without };
    match reason {
        0xb if loan => pick((0xa9b52c, SBN), (0xa9b47c, BN)),
        0xb => pick((0xa9b6bc, SBN), (0xa9b608, BN)),
        0x24 => pick((0xa9b3bc, NS), (0xa9b328, N)),
        0x17 if third_is_buyer => pick((0xa9b25c, NS), (0xa9b1b8, N)),
        0x17 => (0xa9b0f0, NB),
        0x18 if third_is_buyer => pick((0xa9b020, NS), (0xa9af78, N)),
        0x18 => (0xa9aeac, NB),
        0x13 => pick((0xa9ade8, SB), (0xa9ad4c, B)),
        0xe => pick((0xa9ac44, SBN), (0xa9ab68, BN)),
        0x14 => pick((0xa9aa70, SBN), (0xa9a9a4, BN)),
        0x15 => pick((0xa9a8ac, SBN), (0xa9a7e0, BN)),
        0xc => (0xa9a6d8, SBN),
        0xd => (0xa9a5b4, SBNNation),
        0xf => (0xa9a4d0, BN),
        0x10 => (0xa9a3d4, SBN),
        0x11 => pick((0xa9a2e4, SBN), (0xa9a220, BN)),
        3 => pick((0xa9a138, SBB), (0xa99a70, B)),
        4 => pick((0xa9a028, SBB), (0xa99f44, BB)),
        0x22 => pick((0xa99e54, SBB), (0xa99d8c, BB)),
        5 | 0x23 => pick((if reason == 5 { 0xa99c7c } else { 0xa99b88 }, SBS), (0xa99a70, B)),
        _ => pick((0xa99ae8, SB), (0xa99a70, B)),
    }
}

/// The headline template of a collapsed transfer: a loan (slot 8 ≠ 0) "<name> loan to <buyer> cancelled"
/// (0xa99928); otherwise one of four by the buyer's id `b` and the person's id `p` (signed remainders, as the
/// original): b % 3 = p % 4 "<name> deal vetoed" (0xa99a40), b % 4 = p % 2 "<name> deal falls through" (0xa99a08),
/// b % 3 ≠ p % 2 "<name> move to <buyer> cancelled" (0xa99980), otherwise "<name> going nowhere" (0xa999d8).
fn offer_collapsed_headline(loan: bool, b: i32, p: i32) -> usize {
    if loan {
        0xa99928
    } else if b % 3 == p % 4 {
        0xa99a40
    } else if b % 4 == p % 2 {
        0xa99a08
    } else if b % 3 != p % 2 {
        0xa99980
    } else {
        0xa999d8
    }
}

/// Writes the headline or the body of the news that a proposed transfer or loan collapsed, was vetoed by the board or
/// was cancelled, with the reason.
///
/// `write_offer_collapsed_news_text(news, full, out)`: the buyer of slot 1, the person of slot 2 (name from slots 3-5
/// into 300 bytes; mode 1 in the body, 5 in the headline), the loan flag of slot 8. The headline by
/// `offer_collapsed_headline`; the ones naming the buyer take its short name and gender after the grammar marker, the
/// "going nowhere" one only the name. The body: the seller of slot 0 (−1 none), the third club of slot 6, the reason of
/// slot 7 (a word), the template and arguments by `offer_collapsed_template`; reason 0xd adds the nationality
/// adjective of the buyer's `.nation()` (100 bytes, `exe::nation_get_nationality_adjective` with 2). Then the links to
/// the person (0x7d7), the buyer and, in the body, the seller when there is one (0x7d5).
///
/// Defect of the original: reasons 0xc, 0xd and 0x10 name the seller even when slot 0 is −1 (the record before the club
/// table). The copy keeps it.
/// Original: `TRANSFER_MANAGER::write_offer_collapsed_news_text` (0x8de980, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_offer_collapsed_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    let (b, person, ids) = (slot(1), slot(2), (slot(3), slot(4), slot(5)));
    let loan = slot(8) as u8 != 0;
    let (cb, gb) = news_text_club(b);
    if full == 0 {
        news_text_name_n(ids, &mut name, 5, person);
        let t = offer_collapsed_headline(loan, b as i32, person as i32);
        match t {
            0xa99928 | 0xa99980 => fmt(NEWS_TEXT_BUFFER, t, 0x64, gb, gb, np, cb), // "<name> loan to / move to <buyer> cancelled"
            0xa999d8 => fmt(NEWS_TEXT_BUFFER, t, np), // "<name> going nowhere"
            _ => fmt(NEWS_TEXT_BUFFER, t, 0x64, np), // "<name> deal vetoed / falls through"
        };
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_name_n(out, &name, person, ids);
        news_text_link_club(out, b);
        return;
    }
    let (s, n, reason) = (slot(0), slot(6), slot(7) as i16);
    news_text_name_n(ids, &mut name, 1, person);
    let has_seller = s != u32::MAX;
    let (t, args) = offer_collapsed_template(reason, loan, has_seller, n == b);
    // the seller and the third club are read only by the templates that name them, as the original does
    let seller = || news_text_club(s);
    let third = || news_text_club(n);
    use CollapsedArgs::*;
    match args {
        B => fmt(NEWS_TEXT_BUFFER, t, 0x64, gb, gb, np, cb),
        BB => fmt(NEWS_TEXT_BUFFER, t, 0x64, gb, gb, gb, gb, np, cb, cb),
        BN => {
            let (cn, gn) = third();
            fmt(NEWS_TEXT_BUFFER, t, 0x64, gb, gb, gn, gn, np, cb, cn)
        }
        SB => {
            let (cs, gs) = seller();
            fmt(NEWS_TEXT_BUFFER, t, 0x64, gs, gs, gb, gb, np, cs, cb)
        }
        SBB => {
            let (cs, gs) = seller();
            fmt(NEWS_TEXT_BUFFER, t, 0x64, gs, gs, gb, gb, gb, gb, np, cs, cb, cb)
        }
        SBS => {
            let (cs, gs) = seller();
            fmt(NEWS_TEXT_BUFFER, t, 0x64, gs, gs, gb, gb, gs, gs, np, cs, cb, cs)
        }
        SBN => {
            let ((cs, gs), (cn, gn)) = (seller(), third());
            covered!("rxs1_5 collapsed transfer naming three clubs");
            fmt(NEWS_TEXT_BUFFER, t, 0x64, gs, gs, gb, gb, gn, gn, np, cs, cb, cn) // rxs1_5: three clubs
        }
        SBNNation => {
            let mut adjective = [0u8; 0x64];
            let ap = adjective.as_mut_ptr() as usize;
            exe::nation_get_nationality_adjective(club_at(b as i32 as isize).nation(), ap, 0x64, 2);
            let ((cs, gs), (cn, gn)) = (seller(), third());
            fmt(NEWS_TEXT_BUFFER, t, 0x64, gs, gs, gb, gb, gn, gn, 0x64, 0x64, np, cs, cb, cn, ap)
        }
        N => {
            let (cn, gn) = third();
            fmt(NEWS_TEXT_BUFFER, t, gn, gn, 0x64, cn, np)
        }
        NS => {
            let ((cs, gs), (cn, gn)) = (seller(), third());
            fmt(NEWS_TEXT_BUFFER, t, gn, gn, 0x64, gs, gs, cn, np, cs)
        }
        NB => {
            let (cn, gn) = third();
            fmt(NEWS_TEXT_BUFFER, t, gn, gn, 0x64, gb, gb, cn, np, cb)
        }
    };
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    news_text_link_name_n(out, &name, person, ids);
    news_text_link_club(out, b);
    if has_seller {
        news_text_link_club(out, s);
    }
}

/// Comparer for `write_offer_collapsed_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_offer_collapsed_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_offer_collapsed_news_text(this, news, full, t), || format!("write_offer_collapsed_news_text(news {:#x}, full {})", news, full));
}
