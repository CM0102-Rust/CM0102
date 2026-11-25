/// The text templates of the offer rejected / withdrawn news, by the meaning of their arguments: the two clubs in
/// slot order (`Std`), with the bid's cash after the first club (`Money`), with the reasons at the end (`StdReasons`,
/// `MoneyReasons`), the cancelled ones with the player before the second club (`Cancelled`, `CancelledMoney`), the
/// two clubs swapped with their genders kept (`Swapped`, the original's order), the clubs and genders swapped together
/// (`Withdrawn`), the delay template (`Delay`, `DelaySwapped`) and the single-club ones (`OneClub`, `OneClubNameFirst`).
#[derive(Clone, Copy)]
enum RejectedArgs {
    Std,
    Money,
    StdReasons,
    MoneyReasons,
    Cancelled,
    CancelledMoney,
    Swapped,
    Withdrawn,
    Delay,
    DelaySwapped,
    OneClub,
    OneClubNameFirst,
}

/// The rejection reasons of a bid (slot 8 flags) as "They <reasons>": up to two of "see him as an indispensible
/// member" (0x200), "a valuable member" (0x400, not with 0x200), "have only recently purchased him" (0x100), "do not
/// want to sell him to a rival club" (0x800), "to another <competition> club" (0x1000, not with 0x800; the
/// competition of slot 11) and "regard him as an important player for the future" (0x2000, without 0x200 and 0x400),
/// joined with "and" (0x9a9f28). Nothing for flags 2 or 8.
unsafe fn offer_rejected_reasons(flags: u32, competition: u32, out: &mut [u8; 0x418]) {
    let fmt = news_text_format();
    let mut reasons = [[0u8; 0xfa]; 5];
    let mut r = 0usize;
    if flags & 0xa == 0 {
        let mut add = |r: &mut usize, template: usize, args: Option<(i32, usize)>| {
            let at = reasons[*r].as_mut_ptr() as usize;
            match args {
                Some((g, name)) => fmt(at, template, g, g, name),
                None => fmt(at, template),
            };
            *r += 1;
        };
        let indispensable = flags & 0x200 != 0;
        let valuable = flags & 0x400 != 0;
        if indispensable {
            add(&mut r, 0xa942d0, None); // "see him as an indispensible member of their squad"
        }
        if valuable && !indispensable {
            add(&mut r, 0xa942a4, None); // "see him as a valuable member of their squad"
        }
        if r < 2 && flags & 0x100 != 0 {
            add(&mut r, 0xa94280, None); // "have only recently purchased him"
        }
        let rival = flags & 0x800 != 0;
        if r < 2 && rival {
            add(&mut r, 0xa94258, None); // "do not want to sell him to a rival club"
        }
        if r < 2 && flags & 0x1000 != 0 && !rival {
            let comp = Competition(read_usize(G_CLUB_COMP_TABLE).wrapping_add((competition as usize).wrapping_mul(0x6b)));
            let g = comp.short_gender_name() as i8 as i32;
            add(&mut r, 0xa9420c, Some((g, comp.addr() + competition::SHORT_NAME))); // "... to another <comp> club"
        }
        if r < 2 && flags & 0x2000 != 0 && !indispensable && !valuable {
            add(&mut r, 0xa941d8, None); // "regard him as an important player for the future"
        }
    }
    let o = out.as_mut_ptr() as usize;
    match r {
        1 => {
            let n = reasons[0].iter().position(|&b| b == 0).unwrap_or(0xf9);
            out[..=n].copy_from_slice(&reasons[0][..=n]);
        }
        2 => {
            fmt(o, 0x9a9f28, reasons[0].as_ptr() as usize, reasons[1].as_ptr() as usize); // "<1> and <2>"
        }
        _ => {}
    }
}

/// The template and argument order of the body of a rejected bid (slot 7 = 2): by the part exchange flag (slot 6),
/// the kind (slot 8, low word), the loan (slot 9 = 1, plain bids only), the cash (slot 10 ≠ 0) and "the"/"a" (slot 12
/// ≠ 0). Kinds: 0 plain, 5 no squad space, 0x23 no more staff allowed, 6 the delay refused (no seller: the player's
/// template), 0xa dismissed as unacceptable, others rejected with the reasons.
fn offer_rejected_template(exchange: bool, kind: u16, loan: bool, cash: bool, the: bool, seller: bool) -> (usize, RejectedArgs) {
    use RejectedArgs::*;
    let pick = |a: usize, b: usize| if the { a } else { b };
    if !exchange {
        match kind {
            0 if loan => (pick(0xa94148, 0xa940bc), Std),
            0 if cash => (pick(0xa9401c, 0xa93f7c), Money),
            0 => (pick(0xa93ef0, 0xa93e68), Std),
            5 if cash => (0xa93d70, CancelledMoney),
            5 => (0xa93c8c, Cancelled),
            0x23 if cash => (0xa93bb0, CancelledMoney),
            0x23 => (0xa93aec, Cancelled),
            6 if seller => (0xa93a20, Delay),
            6 => (0xa9398c, OneClubNameFirst),
            0xa if cash => (pick(0xa938d0, 0xa93818), Money),
            0xa => (pick(0xa93774, 0xa936d0), Std),
            _ if cash => (pick(0xa9360c, 0xa93548), MoneyReasons),
            _ => (pick(0xa93498, 0xa933ec), StdReasons),
        }
    } else {
        match kind {
            0 if cash => (pick(0xa9333c, 0xa93290), Money),
            0 => (pick(0xa931f8, 0xa93160), Std),
            5 if cash => (0xa93060, Money),
            5 => (0xa92f78, Std),
            0x23 if cash => (0xa92e98, Money),
            0x23 => (0xa92dd0, Std),
            6 => (0xa92cf8, Std),
            0xa if cash => (pick(0xa92c30, 0xa92b68), Money),
            0xa => (pick(0xa92ab8, 0xa92a08), Std),
            _ if cash => (pick(0xa92934, 0xa92864), MoneyReasons),
            _ => (pick(0xa927a8, 0xa926ec), StdReasons),
        }
    }
}

/// The template and argument order of the body of a withdrawn bid (slot 7 ≠ 2), by the part exchange flag (slot 6),
/// the kind (slot 8, low word) and the seller (slot 4 ≠ −1); `None` for a kind without a text.
fn offer_withdrawn_template(exchange: bool, kind: u16, seller: bool) -> Option<(usize, RejectedArgs)> {
    use RejectedArgs::*;
    Some(if exchange {
        match kind {
            0 => (0xa9211c, Withdrawn),
            3 => (0xa92060, Withdrawn), // the fee cannot be afforded
            4 => (0xa91f7c, Withdrawn), // no squad space
            0x22 => (0xa91eb4, Withdrawn), // no more staff allowed
            6 => (0xa92cf8, Swapped),
            2 => (0xa91e20, Withdrawn),
            _ => return None,
        }
    } else {
        match kind {
            0 if seller => (0xa92664, Withdrawn),
            0 => (0xa925fc, OneClub),
            0x16 => (0xa92540, Withdrawn), // no personal terms
            3 => (0xa92490, Withdrawn),
            4 if seller => (0xa923b8, Withdrawn),
            4 => (0xa92304, OneClub),
            0x22 if seller => (0xa9224c, Withdrawn),
            0x22 => (0xa921b4, OneClub),
            6 => (0xa93a20, DelaySwapped),
            2 => (0xa91e20, Withdrawn),
            _ => return None,
        }
    })
}

/// Writes the headline or the body of the news that a club rejected, dismissed or cancelled an offer, or that the
/// bidder withdrew it, with the reasons.
///
/// `write_offer_rejected_news_text(news, full, out)`: the person of slot 0 (name from slots 1-3; mode 1 in the body,
/// 5 in the headline), the selling club of slot 4 (−1 none), the bidding club of slot 5, slot 6 a part exchange, slot
/// 7 the outcome (2 rejected by the seller, 3 withdrawn by the bidder), slot 8 the kind and the reason flags (low
/// word), slot 9 a loan (1), slot 10 the cash of the bid (as money text when not 0, `exe::cash_format_money` with 200
/// bytes), slot 11 a competition (−1 none), slot 12 "the" rather than "a".
///
/// The body (`offer_rejected_template`, `offer_withdrawn_template`) in `NEWS_TEXT_BUFFER`, the reasons
/// (`offer_rejected_reasons`) for the rejected kinds without their own text; then the links: the seller when there is
/// one, the bidder, the person (0x7d7) and the competition of slot 11 (0x7d0). The headline for a rejection: "transfer
/// cancelled" (0xa91c98, kind 6), "loan bid rejected" (0xa91de4) or "transfer bid rejected" (0xa91da4); for a
/// withdrawal: "loan bid withdrawn" (0xa91d68) or "transfer bid withdrawn" (0xa91d28) for kind 0, "<bidder> give up on
/// <name>" (0xa91cd4) with flag 2, else "transfer cancelled"; and the person's link.
///
/// Defects of the original: for a rejected delay of a withdrawn bid (kind 6) and for the part exchange delay the
/// clubs are printed swapped while their genders keep the slot order (`Swapped`, `DelaySwapped`); an outcome other
/// than 2 or 3 in the headline, or a withdrawal kind without a text, writes no text and sets whatever the shared
/// buffer still holds. The copy keeps them.
/// Original: `TRANSFER_MANAGER::write_offer_rejected_news_text` (0x8d5420, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_offer_rejected_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    if full == 0 {
        let (person, s1, s2, s3, s5) = (slot(0), slot(1), slot(2), slot(3), slot(5));
        let outcome = slot(7) as u8;
        let kind = slot(8);
        let loan = slot(9) as u8 == 1;
        news_text_person_name(s1, s2, s3, &mut name, 5, person);
        let cancelled = || fmt(NEWS_TEXT_BUFFER, 0xa91c98, 0x64, np); // "<name> transfer cancelled"
        if outcome == 2 {
            if kind as u16 == 6 {
                cancelled();
            } else if loan {
                fmt(NEWS_TEXT_BUFFER, 0xa91de4, np); // "<name> loan bid rejected"
            } else {
                fmt(NEWS_TEXT_BUFFER, 0xa91da4, np); // "<name> transfer bid rejected"
            }
        } else if outcome == 3 {
            if kind as u16 == 0 {
                let t = if loan { 0xa91d68 } else { 0xa91d28 }; // "<name> loan / transfer bid withdrawn"
                fmt(NEWS_TEXT_BUFFER, t, np);
            } else if kind & 2 != 0 {
                let bidder = club_at(s5 as i32 as isize);
                let g = bidder.short_gender_name() as i8 as i32;
                fmt(NEWS_TEXT_BUFFER, 0xa91cd4, g, g, bidder.addr() + club::SHORT_NAME, np); // "<club> give up on <name>"
            } else {
                cancelled();
            }
        }
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
        return;
    }
    let (person, s1, s2, s3, s4, s5) = (slot(0), slot(1), slot(2), slot(3), slot(4), slot(5));
    let exchange = slot(6) != 0;
    let outcome = slot(7) as u8;
    let flags = slot(8);
    let loan = slot(9) as u8 == 1;
    let cash = slot(10);
    let competition = slot(11);
    let the = slot(12) != 0;
    news_text_person_name(s1, s2, s3, &mut name, 1, person);
    let mut money = [0u8; 0xc8];
    if cash != 0 {
        let amount = exe::cash_get_display_amount(&cash, 6);
        exe::cash_format_money(money.as_mut_ptr() as usize, 0xc8, amount, 6);
    }
    let seller = s4 as i32 != -1;
    let mut reasons = [0u8; 0x418];
    let chosen = if outcome == 2 {
        offer_rejected_reasons(flags, competition, &mut reasons);
        Some(offer_rejected_template(exchange, flags as u16, loan, cash != 0, the, seller))
    } else {
        offer_withdrawn_template(exchange, flags as u16, seller)
    };
    if let Some((template, args)) = chosen {
        covered!("rx05_6 offer rejected body");
        // the seller (slot 4) is only read by the templates that name him: −1 would point before the club table
        let c4 = || club_at(s4 as i32 as isize);
        let c5 = club_at(s5 as i32 as isize);
        let g4 = || c4().short_gender_name() as i8 as i32;
        let n4 = || c4().addr() + club::SHORT_NAME;
        let (g5, n5) = (c5.short_gender_name() as i8 as i32, c5.addr() + club::SHORT_NAME);
        let (mp, rp) = (money.as_ptr() as usize, reasons.as_ptr() as usize);
        let b = NEWS_TEXT_BUFFER;
        use RejectedArgs::*;
        match args {
            Std => fmt(b, template, g4(), g4(), g5, g5, n4(), n5, np),
            Money => fmt(b, template, g4(), g4(), g5, g5, n4(), mp, n5, np),
            StdReasons => fmt(b, template, g4(), g4(), g5, g5, n4(), n5, np, rp),
            MoneyReasons => fmt(b, template, g4(), g4(), g5, g5, n4(), mp, n5, np, rp),
            Cancelled => fmt(b, template, g4(), g4(), g5, g5, n4(), np, n5),
            CancelledMoney => fmt(b, template, g4(), g4(), g5, g5, n4(), mp, np, n5),
            Swapped => fmt(b, template, g4(), g4(), g5, g5, n5, n4(), np),
            Withdrawn => fmt(b, template, g5, g5, g4(), g4(), n5, n4(), np),
            Delay => fmt(b, template, g4(), g4(), g5, g5, 0x64, n4(), n5, np), // rx05_6: the delay refused
            DelaySwapped => fmt(b, template, g4(), g4(), g5, g5, 0x64, n5, n4(), np),
            OneClub => fmt(b, template, g5, g5, n5, np),
            OneClubNameFirst => fmt(b, template, g5, g5, np, n5),
        };
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if seller {
        news_text_link_club(out, s4);
    }
    news_text_link_club(out, s5);
    news_text_link_person(out, &name, person, s1, s2, s3, 0x7d7);
    if competition as i32 != -1 {
        let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((competition as usize).wrapping_mul(0x6b));
        exe::text_add_link(out, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
    }
}

/// Comparer for `write_offer_rejected_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_offer_rejected_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_offer_rejected_news_text(this, news, full, t), || format!("write_offer_rejected_news_text(news {:#x}, full {})", news, full));
}
