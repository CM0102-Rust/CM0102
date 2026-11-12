// ---------- batch %%LEVA%%: the transfer rumour text and the team media story (media.cpp) ----------

/// The piece of `write_transfer_rumour_news_text` that ran last, for the comparer's message: the low byte is the lead
/// (0 the headline, 1 the lead of kind 0, 2 of kind 1, 3 of kind 36, 4 the others), the next byte the clubs paragraph
/// (the number of clubs, 2 to 5) and the third the agent line (1 the human manager is the agent, 2 the others).
pub(crate) static TRANSFER_RUMOUR_PIECE: AtomicU32 = AtomicU32::new(0);

/// The text buffer the media texts are formatted into before they go into the Text object.
const RUMOUR_TEXT: usize = 0xde1f64;

/// The club name (`club::NAME`) of club `id` of the club table.
unsafe fn rumour_club_name(id: i32) -> usize {
    club_at(id as isize).addr() + club::NAME
}

/// The grammatical gender byte of the club name (`club::GENDER_NAME`), sign-extended.
unsafe fn rumour_club_gender(id: i32) -> i32 {
    club_at(id as isize).gender_name() as i32
}

/// The short club name (`club::SHORT_NAME`).
unsafe fn rumour_club_short(id: i32) -> usize {
    club_at(id as isize).addr() + club::SHORT_NAME
}

/// The grammatical gender byte of the short club name (`club::SHORT_GENDER_NAME`), sign-extended.
unsafe fn rumour_club_short_gender(id: i32) -> i32 {
    club_at(id as isize).short_gender_name() as i8 as i32
}

/// Appends the text buffer to the Text object (`Text::append_string`, 0x4ae770, thiscall, ret 4).
unsafe fn rumour_append(out: usize, s: usize) {
    let append: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(0x4ae770usize);
    append(out, s);
}

/// The values the transfer rumour text reads from its news item, by the original's frame: `f` holds the words from
/// +0x00 to +0xb0 of the frame (index = offset / 4), where the club list (+0x28, one club per slot from 0x10) runs over
/// the slots kept after it when it has more than five clubs.
struct Rumour {
    f: [u32; 0x2c],
    club: i32,      // slot 4: the bidding club (-1 none)
    lead: i32,      // slot 5: the kind of lead sentence (0, 1, 36 or other)
    position: u32,  // slot 9: the player's position flags
    flag14: u8,     // slot 14: a flag of the agent line
    count: i32,     // slot 15: the number of clubs in the list (0 when the list reached slot 0x32)
    nation: usize,  // the nation chosen for the links and the nationality adjective (0 none)
    local: bool,    // the human manager is of that nation (or there is no nation)
    desc_gender: i32,
}

impl Rumour {
    fn player(&self) -> i32 { self.f[0x50 / 4] as i32 }      // slot 0: the player's id
    fn player_names(&self) -> (u32, u32, u32) { (self.f[0x4c / 4], self.f[0x44 / 4], self.f[0x40 / 4]) } // slots 1-3
    fn fee(&self) -> u32 { self.f[0x5c / 4] }                // slot 6
    fn agent_kind(&self) -> i32 { self.f[0x64 / 4] as i32 }  // slot 7
    fn qualities(&self) -> u32 { self.f[0x48 / 4] }          // slot 8: the player's strength flags
    fn agent(&self) -> i32 { self.f[0x3c / 4] as i32 }       // slot 10: the agent's id (-1 none)
    fn agent_names(&self) -> (u32, u32, u32) { (self.f[0x60 / 4], self.f[0x58 / 4], self.f[0x54 / 4]) } // slots 11-13
    fn listed(&self, i: usize) -> i32 { self.f[0x28 / 4 + i] as i32 }
}

/// `Media::write_transfer_rumour_news_text(news, body, out)`: writes the headline or the body of a transfer rumour
/// story into `out`, with the links.
///
/// It reads slots 0 to 14 of the news item (`exe::newsitem_get_slot`): the player (0) and his name ids (1-3), the
/// bidding club (4, −1 none), the kind of lead (5), the fee (6), the agent line kind (7), the player's strength (8)
/// and position (9) flags, the agent (10) and his name ids (11-13) and a flag (14); slot 15 is the number of clubs
/// in the story, read from slot 0x10 on (the list stops at slot 0x32 and then counts as empty; the original keeps it
/// in a 5-club frame array, and longer lists run over the slots kept after it, which the copy reproduces). The
/// player's position name (`write_position_name`) and description (`write_player_description`, with its gender).
/// The nation of the story, by (count + agent + lead + club + player) % 6 with a club: 1-2 the club's nation, 3 the
/// player's nation if its reputation is 6000 or more, 4-5 the first listed club's nation; otherwise none. The
/// "local" flag: the human manager of the current screen (`G_SCREEN_RECORDS_MANAGER`) with no nation, of a club
/// (`.club_job()`) of that nation or, without a club, of that nationality.
/// The headline (`transfer_rumour_headline`) or the body: the lead by kind (`transfer_rumour_lead_kind0`,
/// `_kind1`, `_kind36`, `_other`), the clubs paragraph (`transfer_rumour_clubs`), the agent line
/// (`transfer_rumour_agent_line`) and the links (`transfer_rumour_links`).
/// Original: `Media::write_transfer_rumour_news_text` (0x755860, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_transfer_rumour_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let mut f = [0u32; 0x2c];
    f[0x50 / 4] = slot(0);
    f[0x4c / 4] = slot(1);
    f[0x44 / 4] = slot(2);
    f[0x40 / 4] = slot(3);
    let club = slot(4) as i32;
    let lead = slot(5) as i32;
    f[0x5c / 4] = slot(6);
    f[0x64 / 4] = slot(7);
    f[0x48 / 4] = slot(8);
    let position = slot(9);
    f[0x3c / 4] = slot(10);
    f[0x60 / 4] = slot(11);
    f[0x58 / 4] = slot(12);
    f[0x54 / 4] = slot(13);
    let flag14 = slot(14) as u8;
    let mut count = slot(15) as i32;
    if count > 0 {
        let (mut k, mut i) = (0x10u32, 0usize);
        loop {
            if k == 0x32 {
                count = 0; // the list is too long: no clubs
                break;
            }
            f[0x28 / 4 + i] = slot(k);
            k += 1;
            i += 1;
            if i as i32 >= count {
                break;
            }
        }
    }
    let mut r = Rumour { f, club, lead, position, flag14, count, nation: 0, local: false, desc_gender: 0 };
    let mut desc = [0u8; 0xc8];
    let g = write_position_name(r.position, desc.as_mut_ptr() as usize, 0xc8);
    // the original pushes a seventh argument (1) that the description does not read
    r.desc_gender = write_player_description(r.player(), r.position, r.qualities(), desc.as_mut_ptr() as usize, 0xc8, g as u32) as i8 as i32;
    if r.club > -1 {
        let sum = r.count.wrapping_add(r.agent()).wrapping_add(r.lead).wrapping_add(r.club).wrapping_add(r.player());
        r.nation = match sum % 6 {
            1 | 2 => club_at(r.club as isize).nation().addr(),
            3 => {
                let n = staff_at(r.player() as isize).nation();
                if n.reputation() < 0x1770 { 0 } else { n.addr() } // nma5: the player's nation from 6000
            }
            4 | 5 => club_at(r.listed(0) as isize).nation().addr(),
            _ => 0,
        };
    }
    let manager = Staff(read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300))));
    if !manager.is_null() {
        r.local = if r.nation == 0 {
            true
        } else if !manager.club_job().is_null() {
            manager.club_job().nation().addr() == r.nation
        } else {
            manager.nation().addr() == r.nation
        };
    }
    if body == 0 {
        transfer_rumour_headline(&r, out);
        return;
    }
    let (n1, n2, n3) = r.player_names();
    let mut player = [0u8; 0x78];
    let mut player_short = [0u8; 0x78];
    exe::database_format_name_from_ids(n1, n2, n3, player.as_mut_ptr(), 0x78, 1, r.player() as u32);
    exe::database_format_name_from_ids(n1, n2, n3, player_short.as_mut_ptr(), 0x78, 5, r.player() as u32);
    let mut agent = [0u8; 0x78];
    let mut agent_short = [0u8; 0x78];
    if r.agent() > -1 {
        let (a1, a2, a3) = r.agent_names();
        exe::database_format_name_from_ids(a1, a2, a3, agent.as_mut_ptr(), 0x78, 1, r.agent() as u32);
        exe::database_format_name_from_ids(a1, a2, a3, agent_short.as_mut_ptr(), 0x78, 5, r.agent() as u32);
    }
    let fee_value = r.fee();
    let mut fee = [0u8; 0x32];
    let amount = exe::cash_get_display_amount(&fee_value, 6);
    let format_money: unsafe extern "C" fn(usize, u32, i64, u32) -> i32 = core::mem::transmute(0x441c10usize);
    format_money(fee.as_mut_ptr() as usize, 0x32, amount, 6);
    let (p, fp, d) = (player.as_ptr() as usize, fee.as_ptr() as usize, desc.as_ptr() as usize);
    let piece = match r.lead {
        0 => transfer_rumour_lead_kind0(&r, out, p, fp),
        1 => transfer_rumour_lead_kind1(&r, out, p, fp),
        0x24 => transfer_rumour_lead_kind36(&r, out, p, fp, d),
        _ => transfer_rumour_lead_other(&r, out, p, fp),
    };
    transfer_rumour_clubs(&r, out, piece, d);
    transfer_rumour_agent_line(&r, out, player_short.as_ptr() as usize, agent.as_ptr() as usize);
    transfer_rumour_links(&r, out, p, player_short.as_ptr() as usize, agent.as_ptr() as usize);
}

/// The headline of the transfer rumour: the player's short name (style 5) with "unsettled" / "fuels transfer talk"
/// (lead 36: by the player's id even or odd; "eyes ... move" when there is no bidding club and the first listed club
/// is a club), otherwise by the player's id % 3: 0 "transfer speculation", 1 "rumours" (more than one club) or
/// "rumour", 2 "... lead chase for ..." (more than one club, the first one's short name), "... linked with ..." (one
/// club) or "rumour". The name links to the player (`exe::database_staff_matches_names`, type 0x7d7), the bidding
/// club and every listed club (type 0x7d5).
unsafe fn transfer_rumour_headline(r: &Rumour, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("nma_5 rumour headline");
    TRANSFER_RUMOUR_PIECE.store(0, Relaxed);
    let (n1, n2, n3) = r.player_names();
    let mut player = [0u8; 0x190];
    exe::database_format_name_from_ids(n1, n2, n3, player.as_mut_ptr(), 0x190, 5, r.player() as u32);
    let p = player.as_ptr() as usize;
    let first = r.listed(0);
    if r.lead == 0x24 {
        if r.club == -1 && first > r.club {
            let sg = rumour_club_short_gender(first);
            format(RUMOUR_TEXT, 0xa53c94, sg, sg, p, rumour_club_short(first)); // "... eyes ... move"
        } else if r.player() % 2 != 0 {
            format(RUMOUR_TEXT, 0xa53c58, p); // "... fuels transfer talk"
        } else {
            format(RUMOUR_TEXT, 0xa53c24, p); // "... unsettled"
        }
    } else {
        match r.player() % 3 {
            0 => {
                format(RUMOUR_TEXT, 0xa53ae0, p); // "... transfer speculation"
            }
            1 => {
                format(RUMOUR_TEXT, if r.count > 1 { 0xa53b1c } else { 0xa53b4c }, p); // "... rumours" / "... rumour"
            }
            _ => {
                let sg = rumour_club_short_gender(first);
                if r.count > 1 {
                    format(RUMOUR_TEXT, 0xa53bd0, sg, sg, rumour_club_short(first), p); // "... lead chase for ..."
                } else if r.count == 1 {
                    format(RUMOUR_TEXT, 0xa53b7c, sg, sg, p, rumour_club_short(first)); // "... linked with ..."
                } else {
                    format(RUMOUR_TEXT, 0xa53b4c, p);
                }
            }
        }
    }
    exe::text_set_string(out, RUMOUR_TEXT as *const u8);
    let person = staff_at(r.player() as isize);
    if exe::database_staff_matches_names(person, n1, n2, n3, 0usize) != 0 {
        exe::text_add_link(out, p as *const u8, 0x7d7, person); // nma5: the headline's player link
    }
    if r.club > -1 {
        exe::text_add_link(out, rumour_club_name(r.club) as *const u8, 0x7d5, club_at(r.club as isize));
    }
    for i in 0..r.count.max(0) as usize {
        exe::text_add_link(out, rumour_club_name(r.listed(i)) as *const u8, 0x7d5, club_at(r.listed(i) as isize));
    }
}

/// The body lead of kind 0: the first listed club (without a bidding club), the bidding club and the first listed
/// club, or both with the fee. Returns the piece (1).
unsafe fn transfer_rumour_lead_kind0(r: &Rumour, out: usize, player: usize, fee: usize) -> u8 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("nma_5 rumour lead kind 0");
    let first = r.listed(0);
    let (g0, n0) = (rumour_club_gender(first), rumour_club_name(first));
    if r.club == -1 {
        format(RUMOUR_TEXT, 0xa5441c, 0x64, g0, g0, player, n0);
    } else {
        let (g4, n4) = (rumour_club_gender(r.club), rumour_club_name(r.club));
        if r.fee() == 0 {
            format(RUMOUR_TEXT, 0xa54398, g4, g4, g0, g0, n4, player, n0);
        } else {
            format(RUMOUR_TEXT, 0xa54300, g4, g4, g0, g0, n4, player, fee, n0);
        }
    }
    exe::text_set_string(out, RUMOUR_TEXT as *const u8);
    1
}

/// The body lead of kind 1: the first listed club and the player (a foreign manager's lead names the nation by its
/// short name), with the bidding club, and with the fee (then the foreign lead uses the nationality adjective, form
/// 0x1c). Returns the piece (2).
unsafe fn transfer_rumour_lead_kind1(r: &Rumour, out: usize, player: usize, fee: usize) -> u8 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("nma_5 rumour lead kind 1");
    let first = r.listed(0);
    let (g0, n0) = (rumour_club_gender(first), rumour_club_name(first));
    let nation = Nation(r.nation);
    if r.fee() == 0 {
        if r.club == -1 {
            if r.local {
                format(RUMOUR_TEXT, 0xa54b14, g0, g0, n0, player);
            } else {
                let sg = nation.short_gender_name() as i8 as i32;
                format(RUMOUR_TEXT, 0xa54a6c, sg, sg, g0, g0, nation.addr() + nation::SHORT_NAME, n0, player);
            }
        } else {
            let (g4, n4) = (rumour_club_gender(r.club), rumour_club_name(r.club));
            if r.local {
                format(RUMOUR_TEXT, 0xa549ac, g0, g0, g4, g4, n0, n4, player);
            } else {
                let sg = nation.short_gender_name() as i8 as i32;
                format(RUMOUR_TEXT, 0xa548d0, sg, sg, g0, g0, g4, g4, nation.addr() + nation::SHORT_NAME, n0, n4, player);
            }
        }
    } else if r.club == -1 {
        format(RUMOUR_TEXT, 0xa54b14, g0, g0, n0, player);
    } else if r.local {
        let (g4, n4) = (rumour_club_gender(r.club), rumour_club_name(r.club));
        format(RUMOUR_TEXT, 0xa54808, g0, g0, g4, g4, n0, n4, player, fee);
    } else {
        let mut adjective = [0u8; 0x78];
        exe::nation_get_nationality_adjective(nation, adjective.as_mut_ptr() as usize, 0x78, 0x1c);
        let (g4, n4) = (rumour_club_gender(r.club), rumour_club_name(r.club));
        format(RUMOUR_TEXT, 0xa54724, 0x64, 0x64, g0, g0, g4, g4, adjective.as_ptr() as usize, n0, n4, player, fee);
    }
    exe::text_set_string(out, RUMOUR_TEXT as *const u8);
    2
}

/// The body lead of kind 36: the first listed club and the player (with his description and its gender when there is
/// a bidding club, and the fee when there is one). Returns the piece (3).
unsafe fn transfer_rumour_lead_kind36(r: &Rumour, out: usize, player: usize, fee: usize, desc: usize) -> u8 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("nma_5 rumour lead kind 36");
    let first = r.listed(0);
    let (g0, n0) = (rumour_club_gender(first), rumour_club_name(first));
    let gd = r.desc_gender;
    if r.club == -1 {
        format(RUMOUR_TEXT, 0xa5504c, 0x64, g0, g0, player, n0);
    } else {
        let (g4, n4) = (rumour_club_gender(r.club), rumour_club_name(r.club));
        if r.fee() == 0 {
            format(RUMOUR_TEXT, 0xa54f7c, g4, g4, gd, gd, g0, g0, n4, desc, player, n0);
        } else {
            format(RUMOUR_TEXT, 0xa54e84, g4, g4, gd, gd, g0, g0, n4, desc, player, n0, fee);
        }
    }
    exe::text_set_string(out, RUMOUR_TEXT as *const u8);
    3
}

/// The body lead of the other kinds: like kind 0 with other texts. Returns the piece (4).
unsafe fn transfer_rumour_lead_other(r: &Rumour, out: usize, player: usize, fee: usize) -> u8 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("nma_5 rumour lead other");
    let first = r.listed(0);
    let (g0, n0) = (rumour_club_gender(first), rumour_club_name(first));
    if r.club == -1 {
        format(RUMOUR_TEXT, 0xa55578, 0x64, g0, g0, player, n0);
    } else {
        let (g4, n4) = (rumour_club_gender(r.club), rumour_club_name(r.club));
        if r.fee() == 0 {
            format(RUMOUR_TEXT, 0xa554a4, 0x64, g0, g0, g4, g4, player, n0, n4);
        } else {
            format(RUMOUR_TEXT, 0xa553f4, 0x64, g4, g4, g0, g0, player, fee, n4, n0); // nma5: the fee in the lead
        }
    }
    exe::text_set_string(out, RUMOUR_TEXT as *const u8);
    4
}

/// The clubs paragraph of the body: with 2 to 5 listed clubs, the 2nd to the 5th by name (the texts by the lead
/// piece; with 2 clubs, kind 36 adds the player's description and the other kinds, for a foreign manager, the
/// nationality adjective, form 0x18), appended to the text.
unsafe fn transfer_rumour_clubs(r: &Rumour, out: usize, piece: u8, desc: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if !(2..=5).contains(&r.count) {
        TRANSFER_RUMOUR_PIECE.store(piece as u32, Relaxed);
        return;
    }
    covered!("nma_5 rumour clubs");
    TRANSFER_RUMOUR_PIECE.store(piece as u32 | (r.count as u32) << 8, Relaxed);
    let g = |i: usize| rumour_club_gender(r.listed(i));
    let n = |i: usize| rumour_club_name(r.listed(i));
    // the texts of 3, 4 and 5 clubs by piece (kind 0, kind 1, kind 36, other)
    let (t3, t4, t5) = match piece {
        1 => (0xa54224, 0xa54188, 0xa540c8),
        2 => (0xa54628, 0xa54578, 0xa544a4),
        3 => (0, 0xa54c90, 0xa54ba0),
        _ => (0xa55240, 0xa55194, 0xa550c0),
    };
    let gd = r.desc_gender;
    match r.count {
        2 => match piece {
            1 => {
                format(RUMOUR_TEXT, 0xa542a4, g(1), g(1), n(1));
            }
            2 => {
                format(RUMOUR_TEXT, 0xa546b8, g(1), g(1), n(1));
            }
            3 => {
                format(RUMOUR_TEXT, 0xa54df4, g(1), g(1), gd, gd, n(1), desc);
            }
            _ => {
                if r.local {
                    format(RUMOUR_TEXT, 0xa55370, g(1), g(1), n(1));
                } else {
                    let mut adjective = [0u8; 0x78];
                    exe::nation_get_nationality_adjective(r.nation, adjective.as_mut_ptr() as usize, 0x78, 0x18);
                    format(RUMOUR_TEXT, 0xa552cc, 0x64, g(1), g(1), adjective.as_ptr() as usize, n(1));
                }
            }
        },
        3 => {
            if piece == 3 {
                format(RUMOUR_TEXT, 0xa54d4c, gd, gd, g(1), g(1), g(2), g(2), desc, n(1), n(2));
            } else {
                format(RUMOUR_TEXT, t3, g(1), g(1), g(2), g(2), n(1), n(2));
            }
        }
        4 => {
            format(RUMOUR_TEXT, t4, g(1), g(1), g(2), g(2), g(3), g(3), n(1), n(2), n(3));
        }
        _ => {
            format(RUMOUR_TEXT, t5, g(1), g(1), g(2), g(2), g(3), g(3), g(4), g(4), n(1), n(2), n(3), n(4));
        }
    }
    rumour_append(out, RUMOUR_TEXT);
}

/// The agent line of the body, appended to the text (empty for the other kinds): when the agent is the human manager
/// of the current screen, by the agent line kind (slot 7): 1 the player's own words (two texts by the flag of slot
/// 14), 2 three texts (the flag, then without a fee and with a fee), 3 a fixed text; otherwise 1 and 2 quote the
/// agent (his full name) on the bidding club (its short name).
unsafe fn transfer_rumour_agent_line(r: &Rumour, out: usize, player_short: usize, agent: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut line = [0u8; 0x190];
    let l = line.as_mut_ptr() as usize;
    let human = r.agent() > -1
        && staff_at(r.agent() as isize).addr() == read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300)));
    if human {
        covered!("nma_5 rumour agent human");
        TRANSFER_RUMOUR_PIECE.fetch_or(1 << 16, Relaxed);
        match r.agent_kind() {
            1 => {
                if r.flag14 != 0 {
                    format(l, 0xa53e90, 0x64, player_short);
                } else {
                    format(l, 0xa53e20, player_short);
                }
            }
            2 => {
                if r.flag14 != 0 {
                    format(l, 0xa54038, player_short);
                } else if r.fee() == 0 {
                    format(l, 0xa53fb8, player_short);
                } else {
                    format(l, 0xa53f38, player_short);
                }
            }
            3 => {
                format(l, 0xa54094);
            }
            _ => {}
        }
    } else {
        covered!("nma_5 rumour agent quoted");
        TRANSFER_RUMOUR_PIECE.fetch_or(2 << 16, Relaxed);
        let k = r.agent_kind();
        if k == 1 || k == 2 {
            let sg = rumour_club_short_gender(r.club);
            format(l, if k == 1 { 0xa53ce0 } else { 0xa53d7c }, sg, sg, rumour_club_short(r.club), agent, player_short);
        }
    }
    rumour_append(out, l);
}

/// The links of the body: the player's two names (when `exe::database_staff_matches_names`; type 0x7d7), the agent's
/// full name (the same), the bidding club (type 0x7d5); with a nation, its name, short name and the ten forms of its
/// nationality adjective (0x16 to 0x1f) to its national team (`exe::database_get_national_team`); every listed club.
unsafe fn transfer_rumour_links(r: &Rumour, out: usize, player: usize, player_short: usize, agent: usize) {
    covered!("nma_5 rumour links");
    let (n1, n2, n3) = r.player_names();
    let person = staff_at(r.player() as isize);
    if exe::database_staff_matches_names(person, n1, n2, n3, 0usize) != 0 {
        exe::text_add_link(out, player as *const u8, 0x7d7, person);
        exe::text_add_link(out, player_short as *const u8, 0x7d7, person);
    }
    if r.agent() > -1 {
        let (a1, a2, a3) = r.agent_names();
        let a = staff_at(r.agent() as isize);
        if exe::database_staff_matches_names(a, a1, a2, a3, 0usize) != 0 {
            exe::text_add_link(out, agent as *const u8, 0x7d7, a);
        }
    }
    if r.club > -1 {
        exe::text_add_link(out, rumour_club_name(r.club) as *const u8, 0x7d5, club_at(r.club as isize));
    }
    if r.nation != 0 {
        let n = r.nation;
        exe::text_add_link(out, (n + 4) as *const u8, 0x7d5, exe::database_get_national_team(n)); // the nation's name (+4)
        exe::text_add_link(out, (n + nation::SHORT_NAME) as *const u8, 0x7d5, exe::database_get_national_team(n));
        let mut adjective = [0u8; 0x78];
        for form in 0x16..=0x1fu32 {
            exe::nation_get_nationality_adjective(n, adjective.as_mut_ptr() as usize, 0x78, form);
            exe::text_add_link(out, adjective.as_ptr(), 0x7d5, exe::database_get_national_team(n));
        }
    }
    for i in 0..r.count.max(0) as usize {
        exe::text_add_link(out, rumour_club_name(r.listed(i)) as *const u8, 0x7d5, club_at(r.listed(i) as isize));
    }
}

/// The string and the links (text, type, data) of a Text object (`+0` the string, `+4` the link list; each link holds
/// its text at +0, the type at +0xc8 and the data at +0xcc; walked like the copy constructor 0x4ae5c0 does).
unsafe fn nma5_text_contents(t: usize) -> (Vec<u8>, Vec<(Vec<u8>, u32, u32)>) {
    let text = |p: usize| if p == 0 { Vec::new() } else { core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes().to_vec() };
    let mut links = Vec::new();
    let l = read_usize(t + 4);
    if l != 0 {
        list_rewind(l);
        loop {
            let e = exe::llist_get_next(l);
            if e == 0 {
                break;
            }
            links.push((text(e), read_u32(e + 0xc8), read_u32(e + 0xcc)));
        }
    }
    (text(read_usize(t)), links)
}

/// Comparer for `write_transfer_rumour_news_text`: the original writes into `out`, the copy into a fresh local Text
/// side by side (the random generator, without counting the copy's draws); the strings and the links (text, type and
/// data) must be equal. The message names the piece that ran (headline; the lead of kind 0, 1, 36 or other, the
/// clubs paragraph and its number of clubs, the agent line).
pub(crate) unsafe extern "thiscall" fn compare_write_transfer_rumour_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    let mut local = [0usize; 2];
    let lp = local.as_mut_ptr() as usize;
    let (_, _, same_rng, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || o(this, news, body, out),
        || crate::rng::without_counting(|| write_transfer_rumour_news_text(this, news, body, lp)));
    let (mo, mr) = (nma5_text_contents(out), nma5_text_contents(lp));
    exe::drop_text(local.as_mut_ptr());
    let piece = TRANSFER_RUMOUR_PIECE.load(Relaxed);
    SCOREBOARD.record(same_rng && mo == mr, || {
        let lead = ["headline", "lead kind 0", "lead kind 1", "lead kind 36", "lead other"][(piece & 0xff).min(4) as usize];
        let agent = ["", ", agent line human", ", agent line quoted"][((piece >> 16) & 3).min(2) as usize];
        format!("write_transfer_rumour_news_text(body {}) piece: {}, clubs {}{}; original {:?} {:x?} rust {:?} {:x?}", body, lead, (piece >> 8) & 0xff, agent,
            String::from_utf8_lossy(&mo.0), mo.1.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>(),
            String::from_utf8_lossy(&mr.0), mr.1.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>())
    });
}

/// The random draw of slot 1 of the team story by kind (byte table 0x74bb38, kinds 0x3c to 0x63, jumps 0x74bb20): 0
/// draw(3), 1 draw(4), 2 draw(2) + 1 for a club above reputation 5250 or draw(3), 3 draw(3) and 4 draw(2) (a non-zero
/// draw drops the person), 5 the others (the kind becomes 0x63, a draw(2) that drops the person when non-zero, slot 0).
const TEAM_STORY_DRAW_CASE: [u8; 0x28] = [0, 1, 2, 3, 4, 4, 3, 3, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0, 0, 0, 5, 5, 5, 5, 5, 5, 5, 3, 0, 3, 3, 1, 3, 3, 3, 3, 3];

/// The kinds of team story the manager answers (byte table 0x74bb68, kinds 0x3e to 0x5e; jumps 0x74bb60 and
/// 0x74bba4): 0 for 0x3e, 0x50, 0x51 and 0x5e (no answer), 1 for the others (and every kind outside the table).
const TEAM_STORY_ANSWERED: [u8; 0x21] = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0];

/// The club jobs the team story shows in its person slot (byte table 0x74bb94, jobs 1 to 15): all but 7, 9, 10, 11
/// and 14 (shown as −1).
const TEAM_STORY_JOB_HIDDEN: [u8; 0xf] = [0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1, 0];

/// The comparer's rehearsal of `post_team_media_story`: the copy runs without delivering the news item, without
/// logging the story and without the manager's answer, and keeps what it would do.
pub(crate) static TEAM_STORY_DRY_RUN: AtomicBool = AtomicBool::new(false);
pub(crate) static TEAM_STORY_DELIVERIES: AtomicU8 = AtomicU8::new(0);
pub(crate) static TEAM_STORY_LOGGED: AtomicBool = AtomicBool::new(false);
pub(crate) static TEAM_STORY_LOG_PERSON: AtomicUsize = AtomicUsize::new(0);
pub(crate) static TEAM_STORY_LOG_KIND: AtomicU32 = AtomicU32::new(0);
pub(crate) static TEAM_STORY_LOG_REPLY: AtomicU32 = AtomicU32::new(0);
pub(crate) static mut TEAM_STORY_BUILT: [u8; 0xe0] = [0; 0xe0];

/// Delivers the team story `n` through `G_NEWS`: 1 to the club's manager, 2 to the continent's managers, 3 to the
/// division's, 4 to the nation's, 5 to all; in the rehearsal it only counts the delivery and keeps the news item.
unsafe fn team_story_deliver(n: usize, how: u8, target: usize) {
    if TEAM_STORY_DRY_RUN.load(Relaxed) {
        TEAM_STORY_DELIVERIES.fetch_add(1, Relaxed);
        core::ptr::copy_nonoverlapping(n as *const u8, core::ptr::addr_of_mut!(TEAM_STORY_BUILT) as *mut u8, 0xe0);
        return;
    }
    let news = read_usize(G_NEWS);
    match how {
        1 => exe::news_send_to_club_manager(news, n, target),
        2 => exe::news_send_to_continent_managers(news, n, target),
        3 => exe::news_send_to_division_managers(news, n, target),
        4 => exe::news_send_to_nation_managers(news, n, target),
        _ => exe::news_send_to_all_managers(news, n),
    };
}

/// `Media::post_team_media_story(club, kind, person, game, variant, extra)`: posts a media story (news 0xbdf) about a
/// club or national team, logs it and lets a computer manager answer it.
///
/// Nothing before the news start date (`before_news_start_date`), without a club, without a division (unless a
/// national team, `exe::database_is_national_team`), without a nation, without a stadium or its city (unless a
/// national team), without a manager or his sheet (`.non_player()`); a national team only for kinds 0x5a and 0x44;
/// nothing if the story is already in the log (`exe::media_has_logged_story`, type 0xbdf with the kind) or the person
/// is a player below reputation 3500. News item 0xbdf (local, `exe::new_newsitem`): 0 = −1; 1 = the draw by kind
/// (`TEAM_STORY_DRAW_CASE`; some kinds drop the person on a non-zero draw, the others become kind 0x63); 2 = the tone:
/// with the club's reputation at most 4500 + draw(1500) and not a national team, 2 (draw(3) ≤ 1) or 4; otherwise by
/// draw(100): 0 → 6, below 8 → 3, below 35 → 5, below 60 → 0, otherwise 1 (national team or draw(3) non-zero) or 2;
/// 3 = the nation's id; 4 = the club's id; 9 = the kind; 10 = the computer manager's answer
/// (`exe::media_pick_ai_manager_response`, for the answered kinds of `TEAM_STORY_ANSWERED` at a non-human club,
/// otherwise 0); 5 and 6 = the manager's ambition above 15 with temperament below 6, or ambition below 15 with
/// temperament above 14, both with reputation above 5750; 7 = the competition (kinds 0x5a/0x44: the nation's current
/// competition, `exe::nationalteams_get_current_competition`, only if a finals one; 0x5c: the continental
/// competition, `club::CONTINENTAL_COMP_ID`, only if there is one; otherwise the division); 0xb-0xe the manager and
/// his names; 0xf-0x12 the person and his names or −1; 0x13 = whether there is a game (then its record,
/// `exe::fill_match_record_from_fixture` and `exe::write_match_record_to_news`, from slot 0x14 on). From the next
/// free slot: the variant, whether the club was the last one placed in the competition
/// (`exe::comp_util_get_latest_placed_club`), the next hosts (`exe::hostcountry_find_next_hosts`), the extra word, the
/// person's club and club job (jobs 7, 9, 10, 11, 14 and those above 15 as −1) or national team and job (not job 11)
/// or −1 −1, whether he has caps, the shoot-out (0 none, 1 won, 2 lost; nothing for a level one) with a game, and a
/// pundit (`exe::punditlist_pick_random_pundit` by the tone and the nation). Delivery: a non-human club, on a
/// draw(2) non-zero, to its manager; otherwise kinds 0x5a/0x44 to all; 0x5c to the continent; a division above
/// reputation 15 with the club at 9000 or more, to the continent; with the club above 8000 + draw(3000) − (30 × the
/// variant + 1000 above variant 20), kinds 0x50/0x51 above variant 15 and 0x41, 0x5d, 0x61, 0x62, 0x63 to the continent;
/// kind 0x3d to the division on a draw(3) of 0, otherwise to the manager; the others to the division (club below
/// 5000) or to the nation (an English club from 6000 also to Scotland, a Scottish one from 6500 to England, the
/// others also to the club's manager). Then the log (`log_story`) and, for an answered kind at a non-human club, the
/// answer (`Media::apply_team_story_response`, 0x73e870).
/// Original: `Media::post_team_media_story` (0x74b060, media.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn post_team_media_story(this: usize, club: usize, kind: u32, person: usize, game: usize, variant: u32, extra: u32) {
    let club = Club(club);
    let mut person = Staff(person);
    let mut kind = kind;
    let mut record = [0u8; 0x20]; // the match record (a Date at its start)
    exe::date_date_54a110(record.as_mut_ptr() as usize);
    if before_news_start_date() || club.is_null() {
        return;
    }
    let national = || exe::database_is_national_team(club) != 0;
    if club.division().is_null() && !national() {
        return;
    }
    if club.nation().is_null() {
        return;
    }
    if club.stadium().is_null() && !national() {
        return;
    }
    if !national() && club.stadium().city() == 0 {
        return;
    }
    let manager = club.manager();
    if manager.is_null() || manager.non_player().is_null() {
        return;
    }
    let k = kind as u8;
    if national() && k != 0x5a && k != 0x44 {
        return;
    }
    if exe::media_has_logged_story(this, person, club, 0xbdf, k as i8 as i16 as u16 as u32, core::ptr::null()) != 0 {
        return;
    }
    if !person.is_null() && !person.player().is_null() && person.player().current_reputation() < 0xdac {
        return;
    }
    covered!("nma_5 team story");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xbdf, 0);
    exe::newsitem_set_slot(n, 0, u32::MAX);
    let e = (k as i8 as i32).wrapping_sub(0x3c);
    let case = if (0..0x28).contains(&e) { TEAM_STORY_DRAW_CASE[e as usize] } else { 5 };
    let draw = match case {
        0 => exe::utils_random(3),
        1 => exe::utils_random(4),
        2 => {
            if club.reputation() > 0x1482 {
                (exe::utils_random(2) as u8).wrapping_add(1) as i8 as i32
            } else {
                exe::utils_random(3)
            }
        }
        3 | 4 => {
            let d = exe::utils_random(if case == 3 { 3 } else { 2 });
            if d != 0 {
                person = Staff(0);
            }
            d
        }
        _ => {
            covered!("nma_5 team story other kind");
            kind = (kind & !0xff) | 0x63;
            if exe::utils_random(2) != 0 {
                person = Staff(0);
            }
            0
        }
    };
    let k = kind as u8 as i8;
    exe::newsitem_set_slot(n, 1, draw as u32);
    let tone: u32 = if (club.reputation() as i32) > exe::utils_random(0x5dc) + 0x1194 || national() {
        let d = exe::utils_random(100) as u8 as i8;
        if d == 0 {
            6
        } else if d < 8 {
            3
        } else if d < 0x23 {
            5
        } else if d < 0x3c {
            0
        } else if national() || exe::utils_random(3) != 0 {
            1
        } else {
            2
        }
    } else if (0..=1).contains(&exe::utils_random(3)) {
        2
    } else {
        4
    };
    exe::newsitem_set_slot(n, 2, tone);
    exe::newsitem_set_slot(n, 3, read_u32(club.nation().addr()));
    exe::newsitem_set_slot(n, 4, read_u32(club.addr()));
    exe::newsitem_set_slot(n, 9, k as i32 as u32);
    let a = (k as i32).wrapping_sub(0x3e);
    let answered = !((0..0x21).contains(&a) && TEAM_STORY_ANSWERED[a as usize] == 0);
    let reply = if answered && exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
        exe::media_pick_ai_manager_response(this, manager, person, k as i16 as u16 as u32) // nma5: the AI manager's answer
    } else {
        0
    };
    exe::newsitem_set_slot(n, 0xa, reply);
    let rep = manager.non_player().current_reputation();
    exe::newsitem_set_slot(n, 5, (manager.ambition() > 0xf && manager.temperament() < 6 && rep > 0x1676) as u32);
    exe::newsitem_set_slot(n, 6, (manager.ambition() < 0xf && manager.temperament() > 0xe && rep > 0x1676) as u32);
    let k = k as u8;
    let comp = if k == 0x5a || k == 0x44 {
        let nt = read_usize(G_NATIONAL_TEAMS);
        if exe::national_teams_is_finals_comp(nt, exe::nationalteams_get_current_competition(nt, club.nation())) == 0 {
            return;
        }
        read_u32(exe::nationalteams_get_current_competition(read_usize(G_NATIONAL_TEAMS), club.nation()))
    } else if k == 0x5c {
        let c = club.continental_comp_id();
        if c == u32::MAX {
            return;
        }
        c
    } else {
        read_u32(club.division().addr())
    };
    exe::newsitem_set_slot(n, 7, comp);
    exe::newsitem_set_slot(n, 0xb, read_u32(manager.addr()));
    exe::newsitem_set_slot(n, 0xc, read_u32(manager.first_name() + names::ID));
    exe::newsitem_set_slot(n, 0xd, read_u32(manager.second_name() + names::ID));
    exe::newsitem_set_slot(n, 0xe, read_u32(manager.common_name() + names::ID));
    if !person.is_null() {
        exe::newsitem_set_slot(n, 0xf, read_u32(person.addr()));
        exe::newsitem_set_slot(n, 0x10, read_u32(person.first_name() + names::ID));
        exe::newsitem_set_slot(n, 0x11, read_u32(person.second_name() + names::ID));
        exe::newsitem_set_slot(n, 0x12, read_u32(person.common_name() + names::ID));
    } else {
        for s in 0xf..=0x12 {
            exe::newsitem_set_slot(n, s, u32::MAX);
        }
    }
    let mut c: u8 = 0x14;
    if game != 0 {
        exe::newsitem_set_slot(n, 0x13, 1);
        exe::fill_match_record_from_fixture(game, club, record.as_mut_ptr() as usize);
        exe::write_match_record_to_news(record.as_mut_ptr() as usize, n, &mut c);
    } else {
        exe::newsitem_set_slot(n, 0x13, 0);
    }
    let mut put = |v: u32| {
        exe::newsitem_set_slot(n, c as i8, v);
        c = c.wrapping_add(1);
    };
    put(variant as i8 as i32 as u32);
    let latest = exe::comp_util_get_latest_placed_club(read_usize(G_CLUB_COMP_TABLE).wrapping_add((comp as usize).wrapping_mul(0x6b)), 0);
    put((latest == club.addr()) as u32);
    let (mut host_a, mut host_b) = (0u32, u32::MAX);
    exe::hostcountry_find_next_hosts(read_usize(G_HOST_COUNTRY), comp, &mut host_a, &mut host_b);
    put(host_a);
    put(host_b);
    put(extra);
    let caps = if !person.is_null() {
        let cj = person.club_job();
        let nj = person.national_job();
        if !cj.is_null() {
            put(read_u32(cj.addr()));
            let j = person.job_for_club() as i8 as i32;
            put(if (1..=0xf).contains(&j) && TEAM_STORY_JOB_HIDDEN[(j - 1) as usize] == 0 { j as u32 } else { u32::MAX });
        } else if !nj.is_null() && person.job_for_nation() != 0xb {
            put(read_u32(nj.addr()));
            put(person.job_for_nation() as i8 as i32 as u32);
        } else {
            put(u32::MAX);
            put(u32::MAX);
        }
        person.int_apps() != 0
    } else {
        put(u32::MAX);
        put(u32::MAX);
        false
    };
    put(caps as u32);
    if game != 0 {
        let g = Fixture(game);
        let (h, w) = (g.home_penalties(), g.away_penalties());
        if h == -1 {
            put(0);
        } else if h > w {
            put(if g.home_club() == club { 1 } else { 2 });
        } else if h < w {
            put(if g.away_club() == club { 1 } else { 2 });
        }
    }
    let nation = club.nation();
    let pundit = exe::punditlist_pick_random_pundit(read_usize(G_PUNDIT_LIST), club, tone, if !nation.is_null() { read_i32(nation.addr()) } else { -1 });
    put(pundit as u32);
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) == 0 && exe::utils_random(2) != 0 {
        team_story_deliver(n, 1, club.addr());
    } else {
        team_story_deliver_by_kind(n, club, k, variant as u8 as i8);
    }
    if TEAM_STORY_DRY_RUN.load(Relaxed) {
        TEAM_STORY_LOGGED.store(true, Relaxed);
        TEAM_STORY_LOG_PERSON.store(person.addr(), Relaxed);
        TEAM_STORY_LOG_KIND.store(kind, Relaxed);
        TEAM_STORY_LOG_REPLY.store(reply, Relaxed);
        return;
    }
    log_story(this, person.addr(), club.addr(), 0xbdf, kind, reply);
    if answered && exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 {
        let respond: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize) = core::mem::transmute(0x73e870usize);
        respond(this, club.addr(), k as i8 as i16 as u16 as u32, reply, person.addr());
    }
}

/// The delivery of the team story when it does not go to the club's manager alone (see `post_team_media_story`).
unsafe fn team_story_deliver_by_kind(n: usize, club: Club, k: u8, variant: i8) {
    if k == 0x5a || k == 0x44 {
        covered!("nma_5 team story to all");
        return team_story_deliver(n, 5, 0);
    }
    let continent = club.nation().continent();
    if k == 0x5c {
        return team_story_deliver(n, 2, continent);
    }
    let div = club.division();
    if !div.is_null() && club.reputation() >= 0x2328 && div.reputation() > 0xf {
        return team_story_deliver(n, 2, continent);
    }
    if !div.is_null() && div.reputation() > 0xf {
        let bonus = if variant > 0x14 { (variant as i32) * 30 + 1000 } else { 0 };
        if (club.reputation() as i32) > exe::utils_random(0xbb8) + 0x1f40 - bonus {
            if k == 0x50 || k == 0x51 {
                if variant > 0xf {
                    return team_story_deliver(n, 2, continent);
                }
                return team_story_deliver_in_country(n, club, div);
            }
            if [0x41, 0x62, 0x63, 0x61, 0x5d].contains(&k) {
                return team_story_deliver(n, 2, continent);
            }
        }
    }
    if k == 0x3d {
        if div.is_null() || exe::utils_random(3) != 0 {
            return team_story_deliver(n, 1, club.addr());
        }
        return team_story_deliver(n, 3, div.addr());
    }
    team_story_deliver_in_country(n, club, div)
}

/// The team story inside the club's country: without a division to the club's manager; below reputation 5000 to the
/// division's managers; otherwise to the nation's managers, and then an English club from 6000 also to the Scottish
/// managers, a Scottish one from 6500 to the English ones, the others to the club's manager.
unsafe fn team_story_deliver_in_country(n: usize, club: Club, div: Competition) {
    if div.is_null() {
        return team_story_deliver(n, 1, club.addr());
    }
    if club.reputation() < 0x1388 {
        return team_story_deliver(n, 3, div.addr());
    }
    covered!("nma_5 team story to the nation");
    team_story_deliver(n, 4, club.nation().addr());
    let id = read_u32(club.nation().addr());
    let nation_record = |key: usize| read_usize(G_NATION_TABLE).wrapping_add((read_u32(key) as usize).wrapping_mul(nation::SIZE));
    if id == read_u32(KEY_NATION_ENGLAND) && club.reputation() >= 0x1770 {
        team_story_deliver(n, 4, nation_record(KEY_NATION_SCOTLAND));
    } else if id == read_u32(KEY_NATION_SCOTLAND) && club.reputation() >= 0x1964 {
        team_story_deliver(n, 4, nation_record(KEY_NATION_ENGLAND));
    } else {
        team_story_deliver(n, 1, club.addr());
    }
}

/// Comparer for `post_team_media_story`: the copy runs as a rehearsal (without delivering, logging or answering) and
/// the original from the same random state. Must be equal: the copy's random draws are the beginning of the
/// original's (the deliveries' automatic replies and the answer draw afterwards); the original logged the story
/// (one more record of type 0xbdf for the club, the person, the kind and the answer, dated today) exactly when the
/// copy would; and each human manager whose inbox (News +8: 12 bytes per human) advanced received the copy's news
/// item (type +0..4 and data +5..0xd6); with no delivery planned, no inbox advances.
pub(crate) unsafe extern "thiscall" fn compare_post_team_media_story(this: usize, club: usize, kind: u32, person: usize, game: usize, variant: u32, extra: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, u32, u32)>(%%I1%%);
    let news = read_usize(G_NEWS);
    let boxes = if news != 0 { read_usize(news + 8) } else { 0 };
    let boxes_before: Vec<(i32, i32)> = if boxes != 0 { (0..0x10).map(|k| (read_i32(boxes + 12 * k), read_i32(boxes + 12 * k + 4))).collect() } else { Vec::new() };
    let log_before = read_i32(this + media_fields::STORY_LOG_COUNT);
    let before = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    TEAM_STORY_DRY_RUN.store(true, Relaxed);
    TEAM_STORY_DELIVERIES.store(0, Relaxed);
    TEAM_STORY_LOGGED.store(false, Relaxed);
    let (_, rr) = crate::rng::trace(|| crate::rng::without_counting(|| post_team_media_story(this, club, kind, person, game, variant, extra)));
    TEAM_STORY_DRY_RUN.store(false, Relaxed);
    let (deliveries, logged, built) = (TEAM_STORY_DELIVERIES.load(Relaxed), TEAM_STORY_LOGGED.load(Relaxed), *core::ptr::addr_of!(TEAM_STORY_BUILT));
    before.write_back();
    let (_, ro) = crate::rng::trace(|| o(this, club, kind, person, game, variant, extra));
    // the log: a record of this story, dated today, exactly when the copy logs one
    let mut found = false;
    let log_after = read_i32(this + media_fields::STORY_LOG_COUNT);
    if logged {
        let p = TEAM_STORY_LOG_PERSON.load(Relaxed);
        let want_person = if p != 0 { read_u32(p) } else { u32::MAX };
        for i in 0..log_after.max(0) as usize {
            let e = read_usize(this).wrapping_add(i * 0x17);
            if read_u32(e + story_log_entry::TYPE) == 0xbdf && read_u32(e + story_log_entry::PERSON_B) == read_u32(club)
                && read_u32(e + story_log_entry::PERSON_A) == want_person
                && read_i16(e + story_log_entry::SUBTYPE) == TEAM_STORY_LOG_KIND.load(Relaxed) as u8 as i8 as i16
                && read_u8(e + story_log_entry::RESPONSE) == TEAM_STORY_LOG_REPLY.load(Relaxed) as u8
                && read_u32(e) == read_u32(G_CURRENT_DATE)
            {
                found = true;
                break;
            }
        }
    }
    let log_same = if logged { found && log_after > log_before } else { log_after == log_before };
    // the human managers' inboxes
    let (mut delivered, mut news_same) = (0, true);
    let boxes = if news != 0 { read_usize(news + 8) } else { 0 };
    if boxes != 0 && boxes_before.len() == 0x10 {
        for (k, &(start, next)) in boxes_before.iter().enumerate() {
            let e = boxes + 12 * k;
            if read_i32(e + 4) == next && read_i32(e) == start {
                continue;
            }
            delivered += 1;
            let v = if start == -1 { read_i32(e) } else { next }; // the first news item opens the ring at the start
            let s = read_usize(news).wrapping_add((v as usize).wrapping_mul(0xdf));
            let g = core::slice::from_raw_parts(s as *const u8, 0xdf);
            if g[0..4] != built[0..4] || g[5..0xd6] != built[5..0xd6] {
                news_same = false;
            }
        }
    }
    let same = ro.len() >= rr.len() && ro[..rr.len()] == rr[..] && log_same && news_same && (deliveries != 0 || delivered == 0);
    SCOREBOARD.record(same, || format!("post_team_media_story(club {:#x}, kind {:#x}) draws original {:?} rust {:?}; logged {} found {} log {} -> {}; deliveries {} delivered {} news equal {}",
        club, kind as u8, ro, rr, logged, found, log_before, log_after, deliveries, delivered, news_same));
}
