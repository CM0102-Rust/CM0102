/// The reasons a player gives for wanting to leave, as up to five strings of 250 bytes each, and how many there are.
pub(crate) struct LeaveReasons {
    pub(crate) text: [[u8; 0xfa]; 5],
    pub(crate) count: i8,
}

impl LeaveReasons {
    pub(crate) fn new() -> LeaveReasons {
        LeaveReasons { text: [[0; 0xfa]; 5], count: 0 }
    }

    /// Formats the next reason (`langlib_sprintf` with up to three arguments) and counts it.
    pub(crate) unsafe fn add(&mut self, template: usize, args: &[usize]) {
        let fmt = news_text_format();
        let at = self.text[self.count as usize].as_mut_ptr() as usize;
        match *args {
            [a, b, c] => fmt(at, template, a, b, c),
            _ => fmt(at, template),
        };
        self.count += 1;
    }

    /// The address of reason `k`.
    pub(crate) fn at(&self, k: usize) -> usize {
        self.text[k].as_ptr() as usize
    }
}

/// The reasons of the transfer listed news from the contract's grievance flags (`flags`), up to five: 1 "wants to move
/// to a bigger club"; 2 "is unhappy that his pay demands have not been met" (with 0x1000000) or "is unhappy with his
/// current contract"; 0x100 "feels he needs a new challenge in his career"; 0x10 one of three first team phrases by
/// the person's id `person` and the count of interested clubs `count` (signed remainders: person % 3 = count % 3
/// "wants regular...", person % 3 = count % 2 "needs to be playing regular...", otherwise "requires guaranteed...");
/// 0x20 "has found it impossible to adapt to life in <the club's nation>" (with a nation); 0x4000 "believes that he
/// should have been given a more important role in the team"; 0x20000 "wants return to <his nation>" (the person's
/// `.nation()`, short name).
unsafe fn transfer_listed_reasons(flags: u32, person: u32, count: i32, club: Club) -> LeaveReasons {
    let mut r = LeaveReasons::new();
    if flags & 1 != 0 {
        r.add(0x9acb14, &[]); // "wants to move to a bigger club"
    }
    if flags & 2 != 0 {
        r.add(if flags & 0x1000000 == 0 { 0x9acab8 } else { 0x9acae0 }, &[]); // "is unhappy with his current contract" / pay demands
    }
    if r.count < 5 && flags & 0x100 != 0 {
        r.add(0x9aca88, &[]); // "feels he needs a new challenge in his career"
    }
    if r.count < 5 && flags & 0x10 != 0 {
        let p = person as i32 % 3;
        r.add(if p == count % 3 { 0x9aca64 } else if p == count % 2 { 0x9aca34 } else { 0x9aca0c }, &[]); // first team football
    }
    if r.count < 5 && flags & 0x20 != 0 {
        let nation = club.nation();
        if !nation.is_null() {
            let g = read_i8(nation.addr() + nation::GENDER_NAME) as i32 as usize;
            r.add(0x9ac9c0, &[g, g, nation.addr() + 4]); // "has found it impossible to adapt to life in <nation>"
        }
    }
    if r.count < 5 && flags & 0x4000 != 0 {
        r.add(0x9ac974, &[]); // "believes that he should have been given a more important role in the team"
    }
    if r.count < 5 && flags & 0x20000 != 0 {
        let home = staff_at(person as i32 as isize).nation();
        let g = read_i8(home.addr() + nation::SHORT_GENDER_NAME) as i32 as usize;
        r.add(0x9ac93c, &[g, g, home.addr() + nation::SHORT_NAME]); // "wants return to <home nation>"
    }
    r
}

/// The interested clubs of the transfer listed news (slot 14 the count, from slot 15 the ids) in the original's four
/// places, the fourth holding the fee of slot 13 unless a fourth club overwrites it; the slot after the clubs (the
/// "off the list" flag).
///
/// Defect of the original: the clubs go into an array of three followed by the fee, so a fourth club replaces the fee
/// (the money, the reasons and the remainders below then use the club's address) and more clubs write over other
/// locals. The poster names at most three, so it never happens; the copy keeps four places and ignores the rest.
unsafe fn transfer_listed_clubs(news: usize, fee: u32, count: i32) -> ([usize; 4], u32) {
    let mut places = [0usize, 0, 0, fee as usize];
    let mut k = 0;
    while k < count {
        let c = club_at(exe::newsitem_get_slot(news, 0xf + k as u32) as i32 as isize);
        if k < 4 {
            places[k as usize] = c.addr();
        }
        k += 1;
    }
    let after = if count > 0 { 0xf + count as u32 } else { 0xf };
    (places, exe::newsitem_get_slot(news, after))
}

/// Writes the headline or the body of the news that a player is put on the transfer list, with his reasons, the fee
/// and the interested clubs.
///
/// `write_transfer_listed_news_text(news, full, out)`: the club of slot 0, the player of slot 1 (names from slots 2-4)
/// and the manager of slot 5 (names from slots 6-8), each named in 120 bytes in modes 1 and 5; the grievance flags of
/// slot 9, the squad status of slot 10, the position of slot 11 (`exe::formation_get_position_name`, 120 bytes; its
/// gender), a draw of slot 12, the fee of slot 13 (`news_text_money` of the fourth place), the interested clubs
/// (`transfer_listed_clubs`) and the "off the list" flag after them (0 when the manager put him up for sale).
/// The body, with the reasons (`transfer_listed_reasons`, only off the list) joined into 1024 bytes ("gave the reason
/// that he ...", "said that he ... and ...", "said he ..., ... and ...", "indicated that he <reason p%2> and <reason
/// 2+p%2>", or for five one of three phrases by the fee % 3):
/// - draw 0: "<club>'s <position> <name> has been ..." by the squad status (1: "sensationally put up for sale ...")
///   and the flag (0x9aba1c / 0x9abae8 / 0x9ab864 / 0x9ab930), or with the reasons (0x9abd20 / 0x9abbcc); then one
///   club "The news has already alerted the attention of <club>..." (0x9ab7c4), two or three "A projected fee of
///   around <fee> is unlikely to deter ..." (0x9ab6fc / 0x9ab618), otherwise by the fee % 6 "However, <name> will not
///   be allowed to leave for less than <fee>." (0x9ab550) or "<manager> is willing to do business at around <fee>."
///   (0x9ab5b8), appended;
/// - otherwise, with clubs and an odd fee − flags: "<club 1> are ready to spend around <fee> on ..." (0x9ac54c /
///   0x9ac674 by the flag); else "<club> boss <manager> has made his <position> <name> available ..." (0x9ac470, the
///   flag 0), "... has slapped a <fee> price tag on ..." (0x9ac368, squad status 1), "... has agreed to let <name> go
///   on the transfer list at his own request ..." (0x9abf7c without flags, 0x9ac09c with them, 0x9ac1e4 with reasons),
///   then one club "The news will be of interest to <club>..." (0x9abef4) or more "The likes of <club> and <club>..."
///   (0x9abe6c), appended.
/// The headline by ((flags / 5 + draw) × player) % 4 (signed): 0 "<name> put up for sale" (0x9ab3ec, flag 0 and player
/// % 20 > 5) or "<name> on the move" (0x9ab3c0); 1 "<name> set to leave <club>" (0x9ab41c); 2 "<manager> puts <name> on
/// transfer list" (0x9ab46c, flag 0) or "<manager> grants <name>'s transfer request" (0x9ab4c0); 3 "Unhappy <name>
/// transfer listed" (0x9ab518). The links: the club (long name), the player's two names, the manager and each
/// interested club (long and short names).
/// Original: `CONTRACT_MANAGER::write_transfer_listed_news_text` (0x4d7770, contract_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_transfer_listed_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let (s0, person, ids) = (slot(0), slot(1), (slot(2), slot(3), slot(4)));
    let (manager, manager_ids) = (slot(5), (slot(6), slot(7), slot(8)));
    let (flags, squad) = (slot(9), slot(10) as i32);
    let mut position = [0u8; 0x78];
    let pp = position.as_mut_ptr() as usize;
    let gp = exe::formation_get_position_name(slot(11), pp, 0x78, 1) as u8 as i8 as i32;
    let (draw, fee, count) = (slot(12) as i32, slot(13), slot(14) as i32);
    let (places, off_list) = transfer_listed_clubs(news, fee, count);
    let (mut name1, mut name5, mut boss1, mut boss5) = ([0u8; 0x78], [0u8; 0x78], [0u8; 0x78], [0u8; 0x78]);
    news_text_name_n(ids, &mut name1, 1, person);
    news_text_name_n(ids, &mut name5, 5, person);
    news_text_name_n(manager_ids, &mut boss1, 1, manager);
    news_text_name_n(manager_ids, &mut boss5, 5, manager);
    let (n1, n5, b1, b5) = (name1.as_ptr() as usize, name5.as_ptr() as usize, boss1.as_ptr() as usize, boss5.as_ptr() as usize);
    let mut money = [0u8; 0x34];
    news_text_money(places[3] as u32, &mut money);
    let mp = money.as_ptr() as usize;
    let club = club_at(s0 as i32 as isize);
    let (cl, gc) = (club.addr() + club::NAME, club.gender_name() as i32);
    let interested = |k: usize| (places[k] + club::SHORT_NAME, read_i8(places[k] + club::SHORT_GENDER_NAME) as i32);
    let remainder = places[3] as i32;
    if full == 0 {
        let v = ((flags as i32 / 5).wrapping_add(draw)).wrapping_mul(person as i32) % 4;
        match v {
            0 => {
                let t = if off_list == 0 && person as i32 % 0x14 > 5 { 0x9ab3ec } else { 0x9ab3c0 }; // "put up for sale" / "on the move"
                fmt(NEWS_TEXT_BUFFER, t, n5)
            }
            1 => fmt(NEWS_TEXT_BUFFER, 0x9ab41c, gc, gc, n5, cl), // "<name> set to leave <club>"
            2 if off_list == 0 => fmt(NEWS_TEXT_BUFFER, 0x9ab46c, b5, n5), // "<manager> puts <name> on transfer list"
            2 => fmt(NEWS_TEXT_BUFFER, 0x9ab4c0, 0x64, b5, n5), // "<manager> grants <name>'s transfer request"
            _ => fmt(NEWS_TEXT_BUFFER, 0x9ab518, n5), // "Unhappy <name> transfer listed"
        };
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    } else {
        let mut reasons = LeaveReasons::new();
        let mut said = [0u8; 0x400];
        let sp = said.as_mut_ptr() as usize;
        if off_list != 0 {
            reasons = transfer_listed_reasons(flags, person, count, club);
            let k = (person as i32 % 2) as usize;
            match reasons.count {
                1 => fmt(sp, 0x9ac914, reasons.at(0)), // "gave the reason that he <1>"
                2 => fmt(sp, 0x9ac8e0, reasons.at(0), reasons.at(1)), // "said that he <1> and <2>"
                3 => fmt(sp, 0x9ac8a0, reasons.at(0), reasons.at(1), reasons.at(2)), // "said he <1>, <2> and <3>"
                4 => fmt(sp, 0x9ac868, reasons.at(k), reasons.at(2 + k)), // "indicated that he <1> and <2>"
                5 => fmt(sp, [0x9ac778, 0x9ac7c4, 0x9ac810][match remainder % 3 { 0 => 0, 1 => 1, _ => 2 }]), // five reasons
                _ => 0,
            };
        }
        if draw == 0 {
            if reasons.count == 0 {
                let t = match (squad == 1, off_list != 0) {
                    (true, false) => 0x9aba1c, // "... sensationally put up for sale by boss <manager> today."
                    (true, true) => 0x9abae8, // "... after showdown talks with boss <manager> today."
                    (false, false) => 0x9ab864, // "... made available for transfer by his manager <manager>."
                    (false, true) => 0x9ab930, // "... at his own request after a meeting with manager <manager>."
                };
                fmt(NEWS_TEXT_BUFFER, t, gc, gc, gp, gp, cl, pp, n1, b1);
            } else {
                covered!("rxs1_6 transfer listed with the player's reasons");
                let t = if squad == 1 { 0x9abd20 } else { 0x9abbcc };
                fmt(NEWS_TEXT_BUFFER, t, gc, gc, gp, gp, cl, pp, n1, b1, n5, sp); // rxs1_6: listed with reasons
            }
            exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
            match count {
                1 => {
                    let (c0, g0) = interested(0);
                    fmt(NEWS_TEXT_BUFFER, 0x9ab7c4, g0, g0, c0, mp); // "The news has already alerted ..."
                }
                2 => {
                    let ((c0, g0), (c1, g1)) = (interested(0), interested(1));
                    fmt(NEWS_TEXT_BUFFER, 0x9ab6fc, g0, g0, g1, g1, mp, c0, c1); // "A projected fee ... the likes of ..."
                }
                3 => {
                    let ((c0, g0), (c1, g1), (c2, g2)) = (interested(0), interested(1), interested(2));
                    fmt(NEWS_TEXT_BUFFER, 0x9ab618, g0, g0, g1, g1, g2, g2, mp, c0, c1, c2);
                }
                _ if remainder % 6 < 3 => {
                    fmt(NEWS_TEXT_BUFFER, 0x9ab550, n5, mp); // "However, <name> will not be allowed ..."
                }
                _ => {
                    fmt(NEWS_TEXT_BUFFER, 0x9ab5b8, b5, mp); // "<manager> is willing to do business ..."
                }
            }
            news_text_append(out, NEWS_TEXT_BUFFER);
        } else if count != 0 && (remainder.wrapping_sub(flags as i32)) % 2 != 0 {
            let (c0, g0) = interested(0);
            if off_list == 0 {
                fmt(NEWS_TEXT_BUFFER, 0x9ac54c, g0, g0, gc, gc, gp, gp, c0, mp, n1, cl, pp, b1); // "<club> are ready to spend ..."
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9ac674, g0, g0, gc, gc, gp, gp, c0, mp, cl, pp, n1);
            }
            exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        } else {
            if off_list == 0 {
                fmt(NEWS_TEXT_BUFFER, 0x9ac470, gc, gc, gp, gp, cl, b1, pp, n1, mp); // "... made his <position> <name> available ..."
            } else if squad == 1 {
                fmt(NEWS_TEXT_BUFFER, 0x9ac368, gc, gc, gp, gp, cl, b1, mp, n1, pp); // "... slapped a <fee> price tag ..."
            } else if reasons.count == 0 {
                if flags == 0 {
                    fmt(NEWS_TEXT_BUFFER, 0x9abf7c, gc, gc, 0x64, cl, b1, n1, b5, mp); // "... agreed to let <name> go ..."
                } else {
                    fmt(NEWS_TEXT_BUFFER, 0x9ac09c, gc, gc, 0x64, gp, gp, cl, b1, n1, b5, mp, pp);
                }
            } else {
                fmt(NEWS_TEXT_BUFFER, 0x9ac1e4, gc, gc, 0x64, cl, b1, n1, b5, mp, sp); // "... who at a press conference <reasons>."
            }
            exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
            if count == 1 {
                let (c0, g0) = interested(0);
                fmt(NEWS_TEXT_BUFFER, 0x9abef4, g0, g0, c0, n5); // "The news will be of interest to <club>..."
                news_text_append(out, NEWS_TEXT_BUFFER);
            } else if count > 1 {
                let ((c0, g0), (c1, g1)) = (interested(0), interested(1));
                fmt(NEWS_TEXT_BUFFER, 0x9abe6c, g0, g0, g1, g1, c0, c1); // "The likes of <club> and <club> ..."
                news_text_append(out, NEWS_TEXT_BUFFER);
            }
        }
    }
    news_text_link_club_field(out, club, club::NAME);
    let p = staff_at(person as i32 as isize);
    if exe::database_staff_matches_names(p, ids.0, ids.1, ids.2, 0usize) != 0 {
        exe::text_add_link(out, n1 as *const u8, 0x7d7, p);
        exe::text_add_link(out, n5 as *const u8, 0x7d7, p);
    }
    news_text_link_name_n(out, &boss1, manager, manager_ids);
    // at most the four places: the poster names three clubs at most
    for &c in places.iter().take(count.clamp(0, 4) as usize) {
        news_text_link_club_field(out, Club(c), club::NAME);
        news_text_link_club_field(out, Club(c), club::SHORT_NAME);
    }
}

/// Comparer for `write_transfer_listed_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_transfer_listed_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_transfer_listed_news_text(this, news, full, t), || format!("write_transfer_listed_news_text(news {:#x}, full {})", news, full));
}

/// The reasons of the transfer request news from the contract's grievance flags (`flags`), up to five, in the original's
/// order: 1 bigger club; 2 the agent's better contract (`agent`) or the contract / pay demands; 4 left out; 0x100 new
/// challenge; 8 a different manager; 0x10 regular first team football; 0x20000000 a more ambitious club (only at a club
/// with `.reputation()` below 6750); 0x20 impossible to adapt to life in the club's nation; 0x40 personal reasons; 0x80
/// allowed to talk to clubs; 0x200 treated unfairly; 0x4000 a more important role; 0x400 players not respected; 0x800
/// dislikes a teammate; 0x2000 your style of management; 0x800 again the assistant manager; 0x20000 homesick, or
/// "wants to end his career in <nation>" (with `ending` and a nation id ≠ −1: the nation's short name); 0x400000 the
/// club's finances; 0x2000000 his strongest position; 0x4000000 more than a bit part; 0x200000 more loyalty; 0x40000
/// without 1 the media spotlight; 0x80 of `extra` a manager who listens.
///
/// Defect of the original: "dislikes the assistant manager" tests the teammate's bit 0x800 again, so it only comes
/// with "dislikes one of his teammates" and no flag of its own gives it. The copy keeps it.
unsafe fn transfer_request_reasons(flags: u32, agent: bool, club: Club, ending: bool, nation_id: u32, extra: u32) -> LeaveReasons {
    let mut r = LeaveReasons::new();
    let bit = |b: u32| flags & b != 0;
    if bit(1) {
        r.add(0x9acb14, &[]); // "wants to move to a bigger club"
    }
    if bit(2) {
        let t = if agent { 0x9ad598 } else if bit(0x1000000) { 0x9acae0 } else { 0x9acab8 }; // the agent / pay demands / contract
        r.add(t, &[]);
    }
    for (b, t) in [(4, 0x9ad568), (0x100, 0x9aca88), (8, 0x9ad540), (0x10, 0x9aca64)] {
        if r.count < 5 && bit(b) {
            r.add(t, &[]); // left out, new challenge, different manager, regular first team football
        }
    }
    if read_i16(club.addr() + club::REPUTATION) < 0x1a5e && r.count < 5 && bit(0x20000000) {
        r.add(0x9ad518, &[]); // "wants to move to a more ambitious club"
    }
    if r.count < 5 && bit(0x20) {
        let nation = club.nation();
        if !nation.is_null() {
            let g = read_i8(nation.addr() + nation::GENDER_NAME) as i32 as usize;
            r.add(0x9ad4cc, &[g, g, nation.addr() + 4]); // "is finding it impossible to adapt to life in <nation>"
        }
    }
    for (b, t) in [(0x40, 0x9ad4a8), (0x80, 0x9ad468), (0x200, 0x9ad444), (0x4000, 0x9ad40c), (0x400, 0x9ad3cc), (0x800, 0x9ad3ac), (0x2000, 0x9ad380), (0x800, 0x9ad360)] {
        if r.count < 5 && bit(b) {
            r.add(t, &[]); // personal reasons ... the assistant manager (the teammate's bit again)
        }
    }
    if r.count < 5 && bit(0x20000) {
        if !ending || nation_id == u32::MAX {
            r.add(0x9ad314, &[]); // "is homesick"
        } else {
            let home = Nation(nation_by_id(nation_id as usize));
            let g = read_i8(home.addr() + nation::SHORT_GENDER_NAME) as i32 as usize;
            r.add(0x9ad320, &[g, g, home.addr() + nation::SHORT_NAME]); // "wants to end his career in <nation>"
        }
    }
    for (b, t) in [(0x400000, 0x9ad2e0), (0x2000000, 0x9ad2b8), (0x4000000, 0x9ad284), (0x200000, 0x9ad248)] {
        if r.count < 5 && bit(b) {
            r.add(t, &[]); // finances, strongest position, bit part, loyalty
        }
    }
    if r.count < 5 && bit(0x40000) && !bit(1) {
        r.add(0x9ad224, &[]); // "wants to escape the media spotlight"
    }
    if r.count < 5 && extra & 0x80 != 0 {
        r.add(0x9ad1d8, &[]); // "wants to move to a club where the manager listens to his player's concerns"
    }
    r
}

/// Writes the headline or the body of the news that a player or his agent requests a transfer, with his reasons and
/// the answer.
///
/// `write_transfer_request_news_text(news, full, out)`: the club of slot 0, the player of slot 2 (names from slots 3-5
/// into 300 bytes; mode 1 in the body, 5 in the headline), the answer of slots 7 (answered) and 8 (1 granted). The
/// headline: "<name> requests move" (0x9acb34) and the person's link (0x7d7). The body: the reasons
/// (`transfer_request_reasons` with the flags of slot 6, the agent of slot 9, the nation of slot 10, the flag of slot
/// 11 and slot 12) joined into 1024 bytes ("<1>", "<1> and <2>", "<1>, <2> and <3>", "said that amongst other things,
/// he <reason p%2> and <reason 2+p%2>", or for five "claimed that he would do anything to get away ..."). None: "<name>
/// is unhappy at <club> and has requested a transfer." (0x9acba4). More reasons than player % 3 + 2: "A furious <name>
/// has demanded a transfer from <club>. He <reasons>." (0x9ace88) without the agent or when (flags − player) % 3 = 0,
/// otherwise the agent "has demanded ... immediately" (0x9ad020 with the reasons up to three, 0x9acf20 "several very
/// good reasons"). Otherwise "<name> has requested to leave <club>. He <reasons>." (0x9acc1c) without the agent or when
/// (flags − player) % 3 ≠ 0, otherwise the agent "has requested ..." (0x9acdb0 up to three, 0x9acca4). The club's long
/// name and gender; with an answer, "The request has been granted." (0x9acb84) or "... turned down." (0x9acb60) is
/// appended. Then the links to the club (long name) and the person.
/// Original: `CONTRACT_MANAGER::write_transfer_request_news_text` (0x4d8990, contract_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_transfer_request_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let fmt = news_text_format();
    let (s0, person, ids) = (slot(0), slot(2), (slot(3), slot(4), slot(5)));
    let (flags, answered, granted, agent) = (slot(6), slot(7), slot(8) as i32, slot(9) as u8 != 0);
    let (nation_id, ending, extra) = (slot(10), slot(11) as u8 != 0, slot(12));
    let mut name = [0u8; 0x12c];
    let np = name.as_ptr() as usize;
    if full == 0 {
        news_text_name_n(ids, &mut name, 5, person);
        fmt(NEWS_TEXT_BUFFER, 0x9acb34, np); // "<name> requests move"
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        news_text_link_name_n(out, &name, person, ids);
        return;
    }
    let club = club_at(s0 as i32 as isize);
    let reasons = transfer_request_reasons(flags, agent, club, ending, nation_id, extra);
    let mut said = [0u8; 0x400];
    let sp = said.as_mut_ptr() as usize;
    let k = (person as i32 % 2) as usize;
    match reasons.count {
        1 => {
            let crt_format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
            crt_format(sp, 0x991944, reasons.at(0)); // "%s"
        }
        2 => {
            fmt(sp, 0x9a9f28, reasons.at(0), reasons.at(1)); // "<1> and <2>"
        }
        3 => {
            fmt(sp, 0x9ad1a0, reasons.at(0), reasons.at(1), reasons.at(2)); // "<1>, <2> and <3>"
        }
        4 => {
            fmt(sp, 0x9ad158, reasons.at(k), reasons.at(2 + k)); // "said that amongst other things, he <1> and <2>"
        }
        5 => {
            fmt(sp, 0x9ad100); // "claimed that he would do anything to get away ..."
        }
        _ => {}
    }
    let mut name5 = [0u8; 0x12c];
    news_text_name_n(ids, &mut name, 1, person);
    news_text_name_n(ids, &mut name5, 5, person);
    let n5 = name5.as_ptr() as usize;
    let (cl, gc) = (club.addr() + club::NAME, club.gender_name() as i32);
    let n = reasons.count as i32;
    let plain = !agent || (flags.wrapping_sub(person) as i32) % 3 == 0;
    if n == 0 {
        fmt(NEWS_TEXT_BUFFER, 0x9acba4, gc, gc, np, cl); // "<name> is unhappy at <club> and has requested a transfer."
    } else if person as i32 % 3 + 2 < n {
        if plain {
            fmt(NEWS_TEXT_BUFFER, 0x9ace88, gc, gc, np, cl, sp); // "A furious <name> has demanded a transfer ..."
        } else if n < 4 {
            fmt(NEWS_TEXT_BUFFER, 0x9ad020, 0x64, gc, gc, np, cl, n5, sp); // the agent has demanded, with the reasons
        } else {
            fmt(NEWS_TEXT_BUFFER, 0x9acf20, 0x64, gc, gc, np, cl, n5); // the agent: "several very good reasons"
        }
    } else if !agent || (flags.wrapping_sub(person) as i32) % 3 != 0 {
        covered!("rxs1_6 transfer request with the player's reasons");
        fmt(NEWS_TEXT_BUFFER, 0x9acc1c, gc, gc, np, cl, sp); // rxs1_6: requested to leave
    } else if n < 4 {
        fmt(NEWS_TEXT_BUFFER, 0x9acdb0, 0x64, gc, gc, np, cl, n5, sp); // the agent has requested, with the reasons
    } else {
        fmt(NEWS_TEXT_BUFFER, 0x9acca4, 0x64, gc, gc, np, cl, n5); // the agent: "unhappy ... for several reasons"
    }
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    if answered != 0 {
        fmt(NEWS_TEXT_BUFFER, if granted == 1 { 0x9acb84 } else { 0x9acb60 }); // "The request has been granted / turned down."
        news_text_append(out, NEWS_TEXT_BUFFER);
    }
    news_text_link_club_field(out, club, club::NAME);
    news_text_link_name_n(out, &name, person, ids);
}

/// Comparer for `write_transfer_request_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_transfer_request_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    compare_text_writer(out, || o(this, news, full, out), |t| write_transfer_request_news_text(this, news, full, t), || format!("write_transfer_request_news_text(news {:#x}, full {})", news, full));
}
