// --- sx09c_10: the news texts of a dispute, of a fine's reaction and of the coach's report on a fine (dispute.cpp, Fine.cpp) ---

/// `fine_reason_text` (0x5a73a0, Fine.cpp; thiscall(fine, reason, out, size, lower), ret 0x10; sx09b_7).
const FN_FINE_REASON_TEXT: usize = 0x5a73a0;

/// A person of the news slots (the first, second and common name keys) formatted into `name` with
/// `database_format_name_from_ids` (`mode` 5 in a headline, 1 in a body), the id given separately as the original does.
unsafe fn news_slot_person_name(keys: [u32; 3], id: u32, mode: u32, name: &mut [u8; 0x190]) {
    exe::database_format_name_from_ids(keys[0], keys[1], keys[2], name.as_mut_ptr(), 0x190, mode, id);
}

/// Links the person of id `id` and keys `keys` to `name` in `out` (`Text::add_link`, 0x7da) when the keys still match
/// his record (`database_staff_matches_names`).
unsafe fn news_link_person(out: usize, name: &[u8; 0x190], id: u32, keys: [u32; 3]) {
    let person = staff_at(id as i32 as isize);
    if exe::database_staff_matches_names(person, keys[0], keys[1], keys[2], 0) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7da, person);
    }
}

/// Writes the headline or the body of a dispute news item (`dispute_post_news`).
///
/// `dispute_write_news_text(news, full, out)`: the player of slots 0..3, the club id of slot 4 and the outcome of slot 9
/// (slots 5..8 read and, but for 6, unused). Headline: outcome 0 "<player> attempts to tear up his contract"
/// (0x9c3884), 1 "told to remain at <club>" (0x9c38c8), 3 "withdraws complaint about <club>" (0x9c3924), other "free
/// to leave <club>" (0x9c3988); the club's short name (`club::SHORT_NAME`) and its gender byte twice. Body: outcome 0
/// by the reason of slot 6: 1 blocked move (0x9c39e0), 2 wages (0x9c3b18), 3 mistreated (0x9c3c5c), other the
/// manager's verbal attack (0x9c3d84); outcome 1 dismissed (0x9c3ec4) and 3 withdrawn (0x9c3fa8), other upheld
/// (0x9c40a0), with the original's argument orders and the extra 0x64. Formatted into `NEWS_TEXT_BUFFER` and set as the
/// text of `out`; the player linked (0x7da) when his keys match, and the club (0x7d5, its full name) when its id is
/// between 0 and `G_CLUB_COUNT` − 2.
/// Original: `dispute_write_news_text` (0x56a560, dispute.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn dispute_write_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let (id, k1, k2, k3, club_id) = (slot(0), slot(1), slot(2), slot(3), slot(4) as i32);
    slot(5);
    let reason = if full != 0 { slot(6) } else { slot(6); slot(6) };
    if full != 0 {
        slot(7);
        slot(8);
    }
    let outcome = slot(9);
    let mut name = [0u8; 0x190];
    news_slot_person_name([k1, k2, k3], id, if full != 0 { 1 } else { 5 }, &mut name);
    let np = name.as_ptr() as usize;
    let club = read_usize(G_CLUB_TABLE).wrapping_add((club_id as usize).wrapping_mul(club::SIZE));
    let (g, short) = (read_i8(club + club::SHORT_GENDER_NAME) as i32, club + club::SHORT_NAME);
    let b = NEWS_TEXT_BUFFER;
    if full == 0 {
        match outcome {
            0 => fmt(b, 0x9c3884, np),
            1 => fmt(b, 0x9c38c8, g, g, np, short),
            3 => fmt(b, 0x9c3924, g, g, np, short),
            _ => fmt(b, 0x9c3988, g, g, np, short),
        };
    } else {
        covered!("sx09c_10 dispute news body");
        match outcome {
            0 => {
                let tmpl = match reason { 1 => 0x9c39e0, 2 => 0x9c3b18, 3 => 0x9c3c5c, _ => 0x9c3d84 };
                fmt(b, tmpl, g, g, np, short)
            }
            1 => fmt(b, 0x9c3ec4, g, g, 0x64u32, short, np),
            3 => fmt(b, 0x9c3fa8, 0x64u32, g, g, np, short),
            _ => fmt(b, 0x9c40a0, g, g, 0x64u32, short, np),
        };
    }
    exe::text_set_string(out, b as *const u8);
    news_link_person(out, &name, id, [k1, k2, k3]);
    if club_id > -1 && club_id < read_i32(G_CLUB_COUNT) - 1 {
        exe::text_add_link(out, (club + club::NAME) as *const u8, 0x7d5, club);
    }
}

/// Comparer for `dispute_write_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_dispute_write_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    let what = || format!("dispute_write_news_text(news {:#x}, full {})", news, full);
    compare_text_writer(out, || o(this, news, full, out), |t| dispute_write_news_text(this, news, full, t), what);
}

/// The body template of the fine reaction news for an accepted warning (`fine` false) or fine (`fine` true): by the
/// reason (1 unprofessional behaviour, 2 dismissal, 3 violent conduct, 4 poor performance), the effects (slot 11) and
/// the personality (slot 7), and for a grudging warning about behaviour the game flag (slot 12).
fn fine_accepted_template(fine: bool, reason: u32, effects: bool, personality: bool, game: bool) -> usize {
    match (fine, reason) {
        (false, 1) if effects => 0x9cb930,
        (false, 1) if personality => if game { 0x9cb858 } else { 0x9cb7d8 },
        (false, 1) => 0x9cb760,
        (false, 2) => if personality { 0x9cb6e8 } else { 0x9cb678 },
        (false, 3) if effects => 0x9cb4dc,
        (false, 3) => if personality { 0x9cb598 } else { 0x9cba00 },
        (false, 4) if effects => 0x9cb3f8,
        (false, 4) => if personality { 0x9cb310 } else { 0x9cb2a0 },
        (false, _) => 0x9cba00, // "has accepted his warning without comment"
        (true, 1) => if effects { 0x9cacec } else { 0x9cadbc },
        (true, 2) => 0x9cac88,
        (true, 3) if effects => 0x9caa98,
        (true, 3) => if personality { 0x9cabb4 } else { 0x9cab50 },
        (true, 4) => if effects { 0x9ca9c0 } else { 0x9ca958 },
        (true, _) => 0x9cadbc, // "has accepted his fine without comment"
    }
}

/// Writes the headline or the body of the fine reaction news (0xbc6, `post_fine_reaction_news`).
///
/// `write_fine_reaction_news_text(news, full, out)`: the player of slots 0..3 (named with mode 5 in the headline, 1 in
/// the body); the reaction of slot 5 (0 objects, 1 the squad protests, 2 accepts); a warning when slot 8 is set.
/// Headline: warning: 0 "feels warning was harsh" (0x9ca388), 1 "Players protest over ...{s} warning" (0x9ca3c8, with
/// 0x64 first), other "accepts his warning" (0x9ca410); fine: 0x9ca2c4, 0x9ca304 (0x64 first), 0x9ca34c. Body: the reason
/// of slot 4 as text (`fine_reason_text`, 100 bytes, lower case); objects: by the future transfer (slot 10), the type 7
/// contract (slot 9) and the personality (slot 7), else the plain one (warning 0x9cb03c, 0x9caf64, 0x9cae94, 0x9cae0c;
/// fine 0x9ca6d8, 0x9ca5ec, 0x9ca4cc, 0x9ca44c) with the name and the reason; squad protest: with slot 6 set, "Several
/// team members..." (0x9cb1dc / 0x9ca8a0, 0x64 first), otherwise 0x9cb120 / 0x9ca7e8, with the name and the reason;
/// accepts: `fine_accepted_template`; any other reaction: "has accepted his warning/fine without comment". Formatted
/// into `NEWS_TEXT_BUFFER` and set as the text of `out`; the player linked (0x7da) when his keys match.
/// Original: `write_fine_reaction_news_text` (0x5a76e0, Fine.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_fine_reaction_news_text(this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let b = NEWS_TEXT_BUFFER;
    let mut name = [0u8; 0x190];
    let np = name.as_ptr() as usize;
    if full == 0 {
        let (id, k1, k2, k3) = (slot(0), slot(1), slot(2), slot(3));
        slot(4);
        let reaction = slot(5);
        let warning = slot(8) != 0;
        news_slot_person_name([k1, k2, k3], id, 5, &mut name);
        match (warning, reaction) {
            (true, 0) => fmt(b, 0x9ca388, np),
            (true, 1) => fmt(b, 0x9ca3c8, 0x64u32, np),
            (true, _) => fmt(b, 0x9ca410, np),
            (false, 0) => fmt(b, 0x9ca2c4, np),
            (false, 1) => fmt(b, 0x9ca304, 0x64u32, np),
            (false, _) => fmt(b, 0x9ca34c, np),
        };
        exe::text_set_string(out, b as *const u8);
        news_link_person(out, &name, id, [k1, k2, k3]);
        return;
    }
    covered!("sx09c_10 fine reaction body");
    let s: [u32; 13] = core::array::from_fn(|k| slot(k as u32));
    let (id, keys, reason, reaction) = (s[0], [s[1], s[2], s[3]], s[4], s[5]);
    let (rolled, personality, warning, contract7, transfer, effects, game) = (s[6] != 0, s[7] != 0, s[8] != 0, s[9] != 0, s[10] != 0, s[11] != 0, s[12] != 0);
    news_slot_person_name(keys, id, 1, &mut name);
    let mut why = [0u8; 0x64];
    let wp = why.as_mut_ptr() as usize;
    let reason_text: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_FINE_REASON_TEXT);
    reason_text(this, reason, wp, 0x64, 1);
    match reaction {
        0 => {
            let tmpl = match (warning, transfer, contract7, personality) {
                (true, true, _, _) => 0x9cb03c,
                (true, _, true, _) => 0x9caf64,
                (true, _, _, true) => 0x9cae94,
                (true, ..) => 0x9cae0c,
                (false, true, _, _) => 0x9ca6d8,
                (false, _, true, _) => 0x9ca5ec,
                (false, _, _, true) => 0x9ca4cc,
                (false, ..) => 0x9ca44c,
            };
            fmt(b, tmpl, np, wp)
        }
        1 if rolled => fmt(b, if warning { 0x9cb1dc } else { 0x9ca8a0 }, 0x64u32, np, wp),
        1 => fmt(b, if warning { 0x9cb120 } else { 0x9ca7e8 }, np, wp),
        2 => fmt(b, fine_accepted_template(!warning, reason, effects, personality, game), np), // sx09c_10: accepted
        _ => fmt(b, if warning { 0x9cba00 } else { 0x9cadbc }, np),
    };
    exe::text_set_string(out, b as *const u8);
    news_link_person(out, &name, id, keys);
}

/// Comparer for `write_fine_reaction_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_fine_reaction_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%);
    let what = || format!("write_fine_reaction_news_text(news {:#x}, full {})", news, full);
    compare_text_writer(out, || o(this, news, full, out), |t| write_fine_reaction_news_text(this, news, full, t), what);
}

/// Writes the headline or the body of the coach's report on a fine (0x1394, `post_coach_fine_feedback_news`).
///
/// `write_coach_fine_feedback_news_text(news, full, out)`: the coach of slots 0..3 and the player of slots 4..7 (the
/// body; the headline names the second person by the keys of slots 11..13 with the coach's id), the outcome of slot 8
/// and the variant of slot 9 (body) or 14 (headline). Headline: variant 1 "<coach> pleased by <player>{s} reaction"
/// (0x9cbc44, with 0x64 first), otherwise "<coach> thinks <player> can't handle the pressure" (0x9cbbe4). Body: outcome
/// 1 by the variant: 0 "helped to motivate" (0x9cbd54), 1 "had the right effect" (0x9cbdf8), other "helped to focus"
/// (0x9cbedc); other outcomes "is having trouble handling the pressure" (0x9cbc9c). Formatted into a local buffer and
/// set as the text of `out`; the coach linked (0x7da) when his keys match.
/// Original: `write_coach_fine_feedback_news_text` (0x5a8090, Fine.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_coach_fine_feedback_news_text(_this: usize, news: usize, full: u32, out: usize) {
    let fmt = news_text_format();
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let (mut coach, mut player) = ([0u8; 0x190], [0u8; 0x190]);
    let mut text = [0u8; 0x1378];
    let (cp, pp, tp) = (coach.as_ptr() as usize, player.as_ptr() as usize, text.as_mut_ptr() as usize);
    let (id, keys);
    if full != 0 {
        covered!("sx09c_10 coach feedback body");
        let s: [u32; 10] = core::array::from_fn(|k| slot(k as u32));
        (id, keys) = (s[0], [s[1], s[2], s[3]]);
        news_slot_person_name(keys, id, 1, &mut coach);
        news_slot_person_name([s[5], s[6], s[7]], s[4], 1, &mut player);
        let tmpl = if s[8] == 1 {
            match s[9] { 0 => 0x9cbd54, 1 => 0x9cbdf8, _ => 0x9cbedc }
        } else {
            0x9cbc9c
        };
        fmt(tp, tmpl, cp, pp);
    } else {
        let s: [u32; 4] = core::array::from_fn(|k| slot(k as u32));
        (id, keys) = (s[0], [s[1], s[2], s[3]]);
        let t: [u32; 5] = core::array::from_fn(|k| slot(10 + k as u32));
        news_slot_person_name(keys, id, 5, &mut coach);
        news_slot_person_name([t[1], t[2], t[3]], id, 5, &mut player); // the original passes the coach's id
        if t[4] == 1 {
            fmt(tp, 0x9cbc44, 0x64u32, cp, pp);
        } else {
            fmt(tp, 0x9cbbe4, cp, pp);
        }
    }
    exe::text_set_string(out, tp as *const u8);
    news_link_person(out, &coach, id, keys);
}

/// Comparer for `write_coach_fine_feedback_news_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_coach_fine_feedback_news_text(this: usize, news: usize, full: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I2%%);
    let what = || format!("write_coach_fine_feedback_news_text(news {:#x}, full {})", news, full);
    compare_text_writer(out, || o(this, news, full, out), |t| write_coach_fine_feedback_news_text(this, news, full, t), what);
}
