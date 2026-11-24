// ---------- sbD_1: the texts of the not-for-sale, dressing room unrest and unhappy-with-role stories (european_cup.cpp) ----------

/// A person written into four slots of a news item (as `set_news_person_slots` writes him): the id and the three name
/// ids (first, second, common).
#[derive(Clone, Copy)]
pub(crate) struct NewsPerson {
    pub(crate) id: u32,
    pub(crate) names: [u32; 3],
}

impl NewsPerson {
    /// Reads the person from slots `first`..`first + 3` of the news item (`exe::newsitem_get_slot`).
    pub(crate) unsafe fn read(news: usize, first: u32) -> Self {
        let slot = |k: u32| exe::newsitem_get_slot(news, k);
        NewsPerson { id: slot(first), names: [slot(first + 1), slot(first + 2), slot(first + 3)] }
    }

    /// Whether a person is there: the stories write −1 into the id slot for nobody.
    pub(crate) fn present(self) -> bool {
        self.id as i32 > -1
    }

    /// Formats the name into `buf` (`exe::database_format_name_from_ids`, room for the whole buffer): style 1 the full
    /// name, 5 the short one. Returns the buffer's address.
    pub(crate) unsafe fn format(self, buf: &mut [u8], style: u32) -> usize {
        exe::database_format_name_from_ids(self.names[0], self.names[1], self.names[2], buf.as_mut_ptr(), buf.len() as u32, style, self.id);
        buf.as_ptr() as usize
    }

    /// Links each of the formatted names to the person (`exe::text_add_link` with `kind`) when he still bears the names
    /// (`exe::database_staff_matches_names`).
    pub(crate) unsafe fn link(self, text: usize, names: &[usize], kind: u32) {
        let person = staff_at(self.id as i32 as isize);
        if exe::database_staff_matches_names(person, self.names[0], self.names[1], self.names[2], 0usize) != 0 {
            for &name in names {
                exe::text_add_link(text, name as *const u8, kind, person);
            }
        }
    }
}

/// Writes which newspaper carries a player's story into `buf`: the `local` template or the `national` one, with the
/// empty string for its comment. Returns the buffer's address.
pub(crate) unsafe fn story_newspaper(buf: &mut [u8; 0x34], local: bool, local_template: usize, national_template: usize) -> usize {
    story_lang!(buf.as_mut_ptr(), if local { local_template } else { national_template }, EMPTY_STRING);
    buf.as_ptr() as usize
}

/// Writes the text of a story about one player (slots 0..3) into the shared buffer [`PRAISE_TEXT_BUFFER`]: the body
/// with the full name (style 1) and the template `body_template` chooses from slot 5, or the headline with the short
/// name (style 5) and `headline`; the name is linked to the player (0x7da).
pub(crate) unsafe fn player_story_text(news: usize, body: u32, text: usize, body_template: impl FnOnce(u32) -> usize, headline: usize) {
    let player = NewsPerson::read(news, 0);
    let mut name = [0u8; 0x190];
    if body != 0 {
        exe::newsitem_get_slot(news, 4);
        let template = body_template(exe::newsitem_get_slot(news, 5));
        let full = player.format(&mut name, 1);
        story_lang!(PRAISE_TEXT_BUFFER, template, full);
    } else {
        let short = player.format(&mut name, 5);
        story_lang!(PRAISE_TEXT_BUFFER, headline, short);
    }
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    player.link(text, &[name.as_ptr() as usize], 0x7da);
}

/// Writes the headline or body of the story that a club will not sell a player (news 0xfbe): slots 0..3 the player,
/// 5 the club, 6 the wording and 7..10 the chairman (−1 in slot 7 when he is not quoted). The player's full name
/// (style 1) and short name (style 5) and, when quoted, the chairman's full name, each in 0x78 bytes; the text goes into
/// the shared buffer [`PRAISE_TEXT_BUFFER`] with the club's gender twice and short name first:
/// - headline: 0x9c4d00 ("<club> dismiss <player> transfer talk") with the short name;
/// - wording 0: with the chairman, 0x9c4ef4 (chairman, player) when his id is odd, otherwise 0x9c4df0 (player,
///   chairman, short name); without him 0x9c4d60 (player);
/// - wording 1: 0x9c5040 (chairman, player) or 0x9c4fb0 (player);
/// - any other wording: with the chairman, 0x9c52b8 with the gender of the club's stadium name, then the player, the
///   chairman, the short name and the stadium's name when the club has a stadium, otherwise 0x9c5184 (player,
///   chairman, short name); without him 0x9c5100 (player).
/// Links: both of the player's names to him (0x7d7, when he still bears the names), the chairman's name to him, and the
/// club's short name to the club (0x7d5).
/// Original: `news_not_for_sale_text` (0x587800, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_not_for_sale_text(news: usize, body: u32, text: usize) {
    let player = NewsPerson::read(news, 0);
    exe::newsitem_get_slot(news, 4);
    let club = club_at(exe::newsitem_get_slot(news, 5) as i32 as isize);
    let wording = exe::newsitem_get_slot(news, 6);
    let chairman = NewsPerson::read(news, 7);
    let (mut full_buf, mut short_buf, mut chairman_buf) = ([0u8; 0x78], [0u8; 0x78], [0u8; 0x78]);
    let full = player.format(&mut full_buf, 1);
    let short = player.format(&mut short_buf, 5);
    let quoted = chairman.present();
    let chair = if quoted { chairman.format(&mut chairman_buf, 1) } else { chairman_buf.as_ptr() as usize };
    let g = club.short_gender_name() as i8 as i32;
    let club_name = club.addr() + club::SHORT_NAME;
    let out = PRAISE_TEXT_BUFFER;
    if body == 0 {
        story_lang!(out, 0x9c4d00, g, g, club_name, short); // "<club> dismiss <player> transfer talk"
    } else {
        covered!("sbD_1 not for sale text body");
        match wording {
            0 if quoted && (chairman.id as i32) % 2 != 0 => story_lang!(out, 0x9c4ef4, g, g, club_name, chair, full),
            0 if quoted => story_lang!(out, 0x9c4df0, g, g, club_name, full, chair, short),
            0 => story_lang!(out, 0x9c4d60, g, g, club_name, full),
            1 if quoted => story_lang!(out, 0x9c5040, g, g, club_name, chair, full),
            1 => story_lang!(out, 0x9c4fb0, g, g, club_name, full),
            _ if quoted => {
                let stadium = club.stadium();
                if !stadium.is_null() {
                    let sg = stadium.gender_name() as i8 as i32;
                    story_lang!(out, 0x9c52b8, g, g, sg, club_name, full, chair, short, stadium.name_addr());
                } else {
                    story_lang!(out, 0x9c5184, g, g, club_name, full, chair, short);
                }
            }
            _ => story_lang!(out, 0x9c5100, g, g, club_name, full),
        }
    }
    exe::text_set_string(text, out as *const u8);
    player.link(text, &[full, short], 0x7d7);
    if quoted {
        chairman.link(text, &[chair], 0x7d7);
    }
    exe::text_add_link(text, club_name as *const u8, 0x7d5, club);
}

/// Writes the headline or body of the dressing room unrest story (news 0xbc7): slots 0..3 the player, 4..7 the other
/// party, 8 the kind and 9 the wording, into the shared buffer [`PRAISE_TEXT_BUFFER`].
/// - Body, with both full names (style 1, 0x190 bytes): kind 0 (a feud) the player alone in 0x9c5678, 0x9c55f0 or
///   0x9c5574 by the wording 0, 1 or 2, otherwise 0x9c550c; kind 1 0x9c5708 (player, other); any other kind, by the
///   wording, 0 0x9c5a3c (100, player, other), 1 0x9c59a4, 2 0x9c592c, 3 0x9c58a8, 4 0x9c5800, otherwise 0x9c577c
///   (player, other). Links: each name to its person (0x7da, when the names still match).
/// - Headline, with the player's short name (style 5): kind 0 "<player> in training ground bust-up" (0x9c5408) for
///   the wording 0 or 1, otherwise "<player> voices his concern" (0x9c5448); kind 1 "<player> speaks out on feud"
///   (0x9c5480); any other kind "<player> voices concern" (0x9c54b8) for the wording 1, 2 or 3, otherwise
///   "Dressing room unrest reported" (0x9c54ec). Link: the name to the player.
/// Original: `news_dressing_room_unrest_text` (0x587e90, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_dressing_room_unrest_text(news: usize, body: u32, text: usize) {
    let player = NewsPerson::read(news, 0);
    let out = PRAISE_TEXT_BUFFER;
    let mut player_buf = [0u8; 0x190];
    if body != 0 {
        let other = NewsPerson::read(news, 4);
        let kind = exe::newsitem_get_slot(news, 8);
        let wording = exe::newsitem_get_slot(news, 9);
        let mut other_buf = [0u8; 0x190];
        let a = player.format(&mut player_buf, 1);
        let b = other.format(&mut other_buf, 1);
        covered!("sbD_1 dressing room unrest text body");
        match kind {
            0 => {
                let template = match wording {
                    0 => 0x9c5678,
                    1 => 0x9c55f0,
                    2 => 0x9c5574,
                    _ => 0x9c550c,
                };
                story_lang!(out, template, a);
            }
            1 => story_lang!(out, 0x9c5708, a, b),
            _ if wording == 0 => story_lang!(out, 0x9c5a3c, 0x64, a, b),
            _ => {
                let template = match wording {
                    1 => 0x9c59a4,
                    2 => 0x9c592c,
                    3 => 0x9c58a8,
                    4 => 0x9c5800,
                    _ => 0x9c577c,
                };
                story_lang!(out, template, a, b);
            }
        }
        exe::text_set_string(text, out as *const u8);
        player.link(text, &[a], 0x7da);
        other.link(text, &[b], 0x7da);
    } else {
        let kind = exe::newsitem_get_slot(news, 8);
        let wording = exe::newsitem_get_slot(news, 9);
        let a = player.format(&mut player_buf, 5);
        match kind {
            0 if wording == 0 || wording == 1 => story_lang!(out, 0x9c5408, a), // "<player> in training ground bust-up"
            0 => story_lang!(out, 0x9c5448, a),                                 // "<player> voices his concern"
            1 => story_lang!(out, 0x9c5480, a),                                 // "<player> speaks out on feud"
            _ if (1..=3).contains(&wording) => story_lang!(out, 0x9c54b8, a),   // "<player> voices concern"
            _ => story_lang!(out, 0x9c54ec),                                    // "Dressing room unrest reported"
        }
        exe::text_set_string(text, out as *const u8);
        player.link(text, &[a], 0x7da);
    }
}

/// Writes the headline or body of the story that a player is unhappy with his role (news 0xbc8; slots 0..3 the player,
/// 5 whether he is transfer listed, read as a byte): the body 0x9c5b80 when he is listed, otherwise 0x9c5b00, with
/// his full name; the headline "<player> unhappy with role" (0x9c5ac8) with his short name (`player_story_text`).
/// Original: `news_unhappy_with_role_text` (0x5883f0, european_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn news_unhappy_with_role_text(news: usize, body: u32, text: usize) {
    covered!("sbD_1 unhappy with role text");
    player_story_text(news, body, text, |listed| if listed as u8 != 0 { 0x9c5b80 } else { 0x9c5b00 }, 0x9c5ac8);
}

/// Comparers of the text writers: `compare_text_writer` (the original into the Text, the copy into a local one).
pub(crate) unsafe extern "C" fn compare_news_not_for_sale_text(news: usize, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize)>(%%I0%%);
    compare_text_writer(text, || o(news, body, text), |t| news_not_for_sale_text(news, body, t), || format!("news_not_for_sale_text(body {})", body));
}

pub(crate) unsafe extern "C" fn compare_news_dressing_room_unrest_text(news: usize, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize)>(%%I1%%);
    compare_text_writer(text, || o(news, body, text), |t| news_dressing_room_unrest_text(news, body, t), || format!("news_dressing_room_unrest_text(body {})", body));
}

pub(crate) unsafe extern "C" fn compare_news_unhappy_with_role_text(news: usize, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize)>(%%I2%%);
    compare_text_writer(text, || o(news, body, text), |t| news_unhappy_with_role_text(news, body, t), || format!("news_unhappy_with_role_text(body {})", body));
}
