// ---------- batch %%LEVA%%: the media story about a player (media.cpp) ----------

std::thread_local! {
    /// The pieces of `write_player_media_story_news_text` its last call ran, in order (the comparer names them).
    static PLAYER_STORY_TRAIL: core::cell::RefCell<Vec<&'static str>> = core::cell::RefCell::new(Vec::new());
}

/// Notes the piece that is running.
fn player_story_piece(name: &'static str) {
    PLAYER_STORY_TRAIL.with(|t| t.borrow_mut().push(name));
}

/// The exe functions the player media story calls that `exe_fns.rs` does not name yet (Rust ports, called through the
/// game's address so their own detours and comparers stay in the path).
const PLAYER_STORY_POSITION_MASK_GET_TEXT: usize = 0x5482f0; // position_mask_get_text, cdecl
const PLAYER_STORY_WRITE_POSITION_NAME: usize = 0x5461b0; // write_position_name, cdecl
const PLAYER_STORY_WRITE_PLAYER_DESCRIPTION: usize = 0x5465a0; // write_player_description, cdecl
const PLAYER_STORY_GET_PUNDIT_NAME: usize = 0x7dad20; // PunditList::get_pundit_name, thiscall, ret 0x10
const PLAYER_STORY_FIXTURE_ENTRY_MATCHES: usize = 0x729500; // Match_Stats::fixture_entry_matches, thiscall, ret 0x14

/// The game's text formatter (`FN_LANGLIB_SPRINTF`, cdecl, variadic: every argument a dword).
macro_rules! story_lang {
    ($buf:expr, $tpl:expr $(, $a:expr)* $(,)?) => {{
        let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        f(($buf) as usize, ($tpl) as usize $(, ($a) as usize)*);
    }};
}

/// The C runtime's sprintf (`FN_CRT_SPRINTF`).
macro_rules! story_crt {
    ($buf:expr, $tpl:expr $(, $a:expr)* $(,)?) => {{
        let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        f(($buf) as usize, ($tpl) as usize $(, ($a) as usize)*);
    }};
}

/// `record_utils` 0x7e4fa0 (cdecl): reads a fixture record (0x1f bytes: date, competition, teams, score, clubs) from the
/// news item's slots, starting at `*counter` and moving it on.
const PLAYER_STORY_READ_FIXTURE: usize = 0x7e4fa0;

/// strcat: appends `src` to the string at `dst`.
unsafe fn story_strcat(dst: usize, src: usize) {
    let mut d = dst;
    while read_u8(d) != 0 {
        d += 1;
    }
    let mut s = src;
    loop {
        let c = read_u8(s);
        write_u8(d, c);
        if c == 0 {
            break;
        }
        d += 1;
        s += 1;
    }
}

/// The locals of the original's body path (the slots of the news item, the names written once and the buffers).
struct PlayerStory {
    news: usize,
    /// slot 0: the player (staff index)
    player: i32,
    /// slots 1..3: the player's name ids
    names: [i32; 3],
    /// slot 4: the player's club (club index; -1 none)
    club: i32,
    /// slot 5: the variant of the story
    subtype: i32,
    /// slot 6: the outlet type (1..6, other: website)
    outlet: i32,
    /// slot 7 (unused by the text)
    slot7: i32,
    /// slot 8 (unused by the text)
    slot8: i32,
    /// slot 9: the story kind
    kind: i32,
    /// slot 10: the reader paragraph's case
    reader_case: i32,
    /// slot 11: the second person's index (-1 none)
    second: i32,
    /// slots 12..14: the second person's name ids
    second_names: [i32; 3],
    /// slot 14 read again (the original asks for 14 twice and never for 15)
    slot14_again: i32,
    slot16: i32,
    slot17: i32,
    slot18: i32,
    /// slot 23: the third person (staff index; the human manager when it is him)
    third: i32,
    /// slots 24..26: the third person's name ids
    third_names: [i32; 3],
    slot27: i32,
    /// slot 28: the story comes with a match (fixture record from slot 29)
    with_fixture: i32,
    /// the fixture record (date, ..., +0x15 the other club, +0x19 and +0x1a the goals)
    fixture: [u8; 0x20],
    /// with a fixture, the slot after it (the result code 1..4); without one, never written
    result: i32,
    /// the player's qualities (for `write_player_description`)
    qualities: i32,
    /// the player's position mask (for `write_position_name` and the description)
    position: i32,
    /// the pundit's id (word; -1 none)
    pundit: i32,
    /// the position mask for `position_mask_get_text`
    position_mask: i32,
    byte_3272: u8,
    byte_3271: u8,
    /// the fourth person (-1 none) and his name ids
    fourth: i32,
    fourth_names: [i32; 3],
    /// the player's offset in the staff table and the third person's
    player_off: usize,
    third_off: usize,
    /// the player's club is in the nation of the human manager on screen (the "local" media)
    local: u32,
    /// the outlet's grammar mark
    mark: u8,
    /// the gender byte of the position name (only the low byte is set; the original passes the dword on)
    position_gender: u8,
    /// the gender byte of the position mask text
    mask_gender: u8,
    name: [u8; 0xc8],
    name5: [u8; 0x78],
    third_name: [u8; 0xf0],
    third_name5: [u8; 0x78],
    second_name: [u8; 0xc8],
    fourth_name: [u8; 0x78],
    position_text: [u8; 0xc8],
    mask_text: [u8; 0x200],
    outlet_name: [u8; 0x200],
    adjective: [u8; 0x80],
    score: [u8; 0x38],
    text: [u8; 0x1388],
    extra: [u8; 0x400],
    long_extra: [u8; 0x1588],
}

impl PlayerStory {
    /// The club record of the story (`G_CLUB_TABLE` + `club::SIZE` × index).
    unsafe fn club_rec(&self) -> usize {
        read_usize(G_CLUB_TABLE).wrapping_add((self.club as usize).wrapping_mul(club::SIZE))
    }

    /// The club's short name and its gender byte (sign extended).
    unsafe fn club_short(&self) -> (usize, i32) {
        let c = self.club_rec();
        (c + club::SHORT_NAME, read_i8(c + club::SHORT_GENDER_NAME) as i32)
    }

    /// The club's nation record (`club::NATION`, a pointer after the load).
    unsafe fn club_nation(&self) -> usize {
        read_usize(self.club_rec() + club::NATION)
    }

    /// The other club of the fixture (+0x15).
    unsafe fn other_club(&self) -> usize {
        let i = u32::from_le_bytes([self.fixture[0x15], self.fixture[0x16], self.fixture[0x17], self.fixture[0x18]]);
        read_usize(G_CLUB_TABLE).wrapping_add((i as usize).wrapping_mul(club::SIZE))
    }

    /// The two goal counts of the fixture (+0x19, +0x1a), signed bytes.
    fn goals(&self) -> (i8, i8) {
        (self.fixture[0x19] as i8, self.fixture[0x1a] as i8)
    }

    /// The nationality adjective of the club's nation into `adjective` (`exe::nation_get_nationality_adjective`, size
    /// 0x78, the form given).
    unsafe fn club_adjective(&mut self, form: u32) {
        exe::nation_get_nationality_adjective(self.club_nation(), self.adjective.as_mut_ptr() as usize, 0x78, form);
    }

}


impl PlayerStory {
    /// Reads the slots, writes the names and the position texts, and works out whether the media is local.
    unsafe fn read(news: usize) -> PlayerStory {
        let s = |k: i8| exe::newsitem_get_slot(news, k as u32) as i32;
        let mut st: PlayerStory = core::mem::zeroed();
        st.news = news;
        exe::date_date_54a110(st.fixture.as_mut_ptr() as usize);
        st.player = s(0);
        st.names = [s(1), s(2), s(3)];
        st.club = s(4);
        st.subtype = s(5);
        st.outlet = s(6);
        st.slot7 = s(7);
        st.slot8 = s(8);
        st.kind = s(9);
        st.reader_case = s(10);
        st.second = s(11);
        st.second_names = [s(12), s(13), s(14)];
        st.slot14_again = s(14); // the original asks for slot 14 twice (and never for 15)
        st.slot16 = s(16);
        st.slot17 = s(17);
        st.slot18 = s(18);
        st.third = s(23);
        st.third_names = [s(24), s(25), s(26)];
        st.slot27 = s(27);
        st.with_fixture = s(28);
        if st.with_fixture != 0 {
            let read_fixture: unsafe extern "C" fn(usize, *mut u8, usize) -> u32 = core::mem::transmute(PLAYER_STORY_READ_FIXTURE);
            let mut counter: u8 = 0x1d;
            read_fixture(news, &mut counter, st.fixture.as_mut_ptr() as usize);
            let mut next = || {
                let k = counter;
                counter = counter.wrapping_add(1);
                s(k as i8)
            };
            st.result = next();
            st.qualities = next();
            st.position = next();
            st.pundit = next();
            st.position_mask = 0;
            st.byte_3272 = 0;
            st.byte_3271 = 0;
            st.fourth = -1;
            st.fourth_names = [-1; 3];
        } else {
            st.qualities = s(0x1d);
            st.position = s(0x1e);
            st.pundit = s(0x1f);
            st.position_mask = s(0x20);
            st.byte_3272 = s(0x21) as u8;
            st.byte_3271 = s(0x22) as u8;
            st.fourth = s(0x23);
            st.fourth_names = [s(0x24), s(0x25), s(0x26)];
        }
        let mask_text: unsafe extern "C" fn(u32, usize, u32) -> u8 = core::mem::transmute(PLAYER_STORY_POSITION_MASK_GET_TEXT);
        st.mask_gender = mask_text(st.position_mask as u32, st.mask_text.as_mut_ptr() as usize, 0);
        st.player_off = (st.player as usize).wrapping_mul(staff::SIZE);
        st.third_off = (st.third as usize).wrapping_mul(staff::SIZE);
        let staff_table = read_usize(G_STAFF_TABLE);
        let [a, b, c] = st.names;
        exe::database_format_name_from_ids(a as u32, b as u32, c as u32, st.name.as_mut_ptr(), 0x78, 1, st.player as u32);
        exe::database_format_name_from_ids(a as u32, b as u32, c as u32, st.name5.as_mut_ptr(), 0x78, 5, st.player as u32);
        let [a, b, c] = st.third_names;
        exe::database_format_name_from_ids(a as u32, b as u32, c as u32, st.third_name.as_mut_ptr(), 0x78, 1, st.third as u32);
        exe::database_format_name_from_ids(a as u32, b as u32, c as u32, st.third_name5.as_mut_ptr(), 0x78, 5, st.third as u32);
        // the original passes a fourth argument (1) the routine does not read
        let position_name: unsafe extern "C" fn(u32, usize, u32, u32) -> u8 = core::mem::transmute(PLAYER_STORY_WRITE_POSITION_NAME);
        st.position_gender = position_name(st.position as u32, st.position_text.as_mut_ptr() as usize, 0xc8, 1);
        if staff_table.wrapping_add(st.third_off) == staff_table.wrapping_add(st.player_off) {
            story_lang!(st.position_text.as_mut_ptr(), 0xa242e0); // "player-manager"
            st.position_gender = 0;
        }
        if st.second != -1 {
            // the player's index goes with the second person's names (the original's choice)
            let [a, b, c] = st.second_names;
            exe::database_format_name_from_ids(a as u32, b as u32, c as u32, st.second_name.as_mut_ptr(), 0x78, 1, st.player as u32);
        } else {
            st.second_name[0] = 0;
        }
        if st.fourth == -1 {
            st.fourth_name[0] = 0;
        } else {
            let [a, b, c] = st.fourth_names;
            exe::database_format_name_from_ids(a as u32, b as u32, c as u32, st.fourth_name.as_mut_ptr(), 0x78, 1, st.fourth as u32);
        }
        // the media is local when the player's club is in the nation of the human manager on screen: his club's nation,
        // or his own nation without a club
        let manager = read_usize(G_SCREEN_RECORDS_MANAGER + (read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300));
        st.local = 0;
        if manager != 0 {
            let job = read_usize(manager + staff::CLUB_JOB);
            let nation = st.club_nation();
            if job != 0 {
                if read_usize(job + club::NATION) == nation {
                    st.local = 1;
                }
            } else if nation == read_usize(manager + staff::NATION) {
                st.local = 1;
            }
        }
        st
    }

    /// The outlet's name into `outlet_name` and its grammar mark: the pundit's name when the media is local and the
    /// story names one; otherwise the outlet type (local: "national newspaper"...; foreign: with the nation's adjective).
    unsafe fn write_outlet(&mut self) {
        player_story_piece("outlet");
        covered!("nma_3 outlet");
        let nation = self.club_nation();
        if self.club != -1 && self.outlet != -1 && self.pundit as i16 != -1 && nation != 0 && self.local != 0 {
            covered!("nma_3 pundit");
            let pundit_name: unsafe extern "thiscall" fn(usize, u32, u32, usize, *mut u8) = core::mem::transmute(PLAYER_STORY_GET_PUNDIT_NAME);
            pundit_name(read_usize(G_PUNDIT_LIST), self.outlet as u32, self.pundit as u32, self.outlet_name.as_mut_ptr() as usize, &mut self.mark);
            return;
        }
        let out = self.outlet_name.as_mut_ptr();
        let (plain, form, foreign) = match self.outlet {
            1 => (0xa242cc, 0x16, 0xa24294), // national newspaper
            2 => (0xa24284, 0x17, 0xa24250), // local newspaper
            3 => (0xa24238, 0x18, 0xa24208), // national radio station
            4 => (0xa241f4, 0x18, 0xa241bc), // local radio station
            5 => (0xa241b0, 0x19, 0xa24184), // magazine
            6 => (0xa24174, 0x1a, 0xa24144), // sports paper
            _ => (0xa2413c, 0x1a, 0xa24110), // website
        };
        if self.local != 0 {
            story_lang!(out, plain);
        } else {
            self.club_adjective(form);
            story_lang!(out, foreign, 0x64, self.adjective.as_ptr());
        }
        self.mark = if self.outlet == 5 { (exe::langlib_get_language() == 8) as u8 } else { 0 };
    }

    /// Kind 0x23: the player is fed up with being left out.
    unsafe fn left_out(&mut self) {
        player_story_piece("left out (0x23)");
        covered!("nma_3 left out");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let mark = self.mark as i8 as i32;
        match self.subtype {
            0 if self.local != 0 => story_lang!(text, 0xa23d38, g, g, 0x64, short, name),
            0 => {
                self.club_adjective(0x1c);
                story_lang!(text, 0xa23c40, g, g, 0x64, 0x64, 0x64, short, name, self.adjective.as_ptr());
            }
            1 => story_lang!(text, 0xa23e0c, g, g, 0x64, short, name),
            2 => story_lang!(text, 0xa23efc, g, g, 0x64, short, name),
            _ => story_lang!(text, 0xa23ffc, mark, mark, g, g, name, self.outlet_name.as_ptr(), short),
        }
    }

    /// Kind 0xe: the media questions the player's ability.
    unsafe fn ability(&mut self) {
        player_story_piece("ability (0xe)");
        covered!("nma_3 ability");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let local = self.local != 0;
        let foreign = |st: &mut PlayerStory, form: u32, tpl: usize| {
            st.club_adjective(form);
            story_lang!(text, tpl, 0x64, 0x64, g, g, st.adjective.as_ptr(), name, short);
        };
        match self.subtype {
            0 if local => story_lang!(text, 0xa23b98, 0x64, g, g, name, short),
            0 => foreign(self, 0x16, 0xa23ad0),
            1 if local => story_lang!(text, 0xa23a20, 0x64, g, g, name, short),
            1 => foreign(self, 0x16, 0xa2395c),
            2 if local => story_lang!(text, 0xa2389c, 0x64, g, g, name, short),
            2 => foreign(self, 0x1e, 0xa237bc),
            3 => story_lang!(text, 0xa236ec, 0x64, g, g, name, short),
            _ => story_lang!(text, 0xa23628, g, g, 0x64, short, name),
        }
    }

    /// Kind 0xd: the player is accused of complacency.
    unsafe fn complacency(&mut self) {
        player_story_piece("complacency (0xd)");
        covered!("nma_3 complacency");
        let (short, g) = self.club_short();
        let (text, name, outlet) = (self.text.as_mut_ptr(), self.name.as_ptr(), self.outlet_name.as_ptr());
        let mark = self.mark as i8 as i32;
        match self.subtype {
            0 => story_lang!(text, 0xa232c0, mark, mark, g, g, 0x64, outlet, short, name),
            1 => story_lang!(text, 0xa233b0, mark, mark, 0x64, g, g, outlet, name, short),
            2 => story_lang!(text, 0xa23474, mark, mark, 0x64, g, g, outlet, name, short),
            _ => story_lang!(text, 0xa23558, g, g, 0x64, short, name),
        }
    }

    /// The score of the fixture, the higher number first ("%d-%d").
    unsafe fn write_score(&mut self) {
        let (a, b) = self.goals();
        story_crt!(self.score.as_mut_ptr(), 0x98c114, a.max(b) as i32, a.min(b) as i32);
    }

    /// Kind 0x11: the media questions the player's contribution (after a match, with the score; without one, the
    /// supporters speak out).
    unsafe fn contribution(&mut self) {
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        if self.with_fixture == 0 {
            player_story_piece("fan criticism (0x11 without a match)");
            covered!("nma_3 fan criticism");
            story_lang!(text, 0xa21cb4, g, g, 0x64, short, name);
            return;
        }
        player_story_piece("contribution (0x11)");
        covered!("nma_3 contribution");
        self.write_score();
        let description: unsafe extern "C" fn(i32, u32, u32, usize, i32, u32, u32) -> u8 = core::mem::transmute(PLAYER_STORY_WRITE_PLAYER_DESCRIPTION);
        let dg = description(self.player, self.position as u32, self.qualities as u32, self.position_text.as_mut_ptr() as usize, 0xc8, self.position_gender as u32, 1) as i8 as i32;
        let (g1, g2) = self.goals();
        let c2 = self.other_club();
        let c2name = c2 + club::NAME;
        let c2g = read_i8(c2 + club::GENDER_NAME) as i32;
        let (score, pos, outlet, name5) = (self.score.as_ptr(), self.position_text.as_ptr(), self.outlet_name.as_ptr(), self.name5.as_ptr());
        let nation = self.club_nation();
        let (nshort, ng) = (nation + nation::SHORT_NAME, read_i8(nation + nation::SHORT_GENDER_NAME) as i32);
        let local = self.local != 0;
        let by_score = |win: usize, draw: usize, loss: usize| if g1 > g2 { win } else if g1 == g2 { draw } else { loss };
        match self.subtype {
            0 => {
                let mark = self.mark as i8 as i32;
                self.slot27 = g;
                let tpl = by_score(0xa22094, 0xa21f7c, 0xa21e60);
                story_lang!(text, tpl, g, g, c2g, c2g, mark, mark, 0x64, short, score, c2name, outlet, name);
            }
            1 => {
                self.slot27 = g;
                if local {
                    let tpl = by_score(0xa22820, 0xa2258c, 0xa222fc);
                    story_lang!(text, tpl, g, g, c2g, c2g, dg, dg, 0x64, short, score, c2name, pos, name);
                } else {
                    self.position_mask = c2g;
                    self.position = ng;
                    let tpl = by_score(0xa226d4, 0xa2243c, 0xa221a8);
                    story_lang!(text, tpl, ng, ng, g, g, c2g, c2g, dg, dg, 0x64, nshort, short, score, c2name, pos, name);
                }
            }
            _ => {
                self.slot27 = g;
                if local {
                    if g1 > g2 {
                        story_lang!(text, 0xa23154, g, g, c2g, c2g, dg, dg, name, short, c2name, score, pos);
                    } else {
                        let tpl = if g1 == g2 { 0xa22e34 } else { 0xa22b14 };
                        story_lang!(text, tpl, g, g, c2g, c2g, dg, dg, name, short, score, c2name, pos);
                    }
                } else {
                    self.position_mask = c2g;
                    self.position = ng;
                    if g1 > g2 {
                        story_lang!(text, 0xa22fa4, ng, ng, 0x64, g, g, c2g, c2g, dg, dg, name, nshort, name5, short, c2name, score, pos);
                    } else {
                        let tpl = if g1 == g2 { 0xa22c84 } else { 0xa22964 };
                        story_lang!(text, tpl, ng, ng, 0x64, g, g, c2g, c2g, dg, dg, name, nshort, name5, short, score, c2name, pos);
                    }
                }
            }
        }
        self.draw_result();
    }

    /// After a drawn match (kinds 0x11 and 0xa): how the club eventually won or lost (the result code), appended.
    unsafe fn draw_result(&mut self) {
        let (g1, g2) = self.goals();
        if g1 != g2 {
            return;
        }
        let tpl = match self.result {
            1 => 0xa21e0c,
            2 => 0xa21684,
            3 => 0xa21dc0,
            4 => 0xa21d74,
            _ => return,
        };
        player_story_piece("draw result");
        covered!("nma_3 draw result");
        let (short, g) = self.club_short();
        story_lang!(self.extra.as_mut_ptr(), tpl, g, g, short);
        story_strcat(self.text.as_mut_ptr() as usize, self.extra.as_ptr() as usize);
    }

    /// Kind 0xc: the media questions the player's commitment.
    unsafe fn commitment(&mut self) {
        player_story_piece("commitment (0xc)");
        covered!("nma_3 commitment");
        let (short, g) = self.club_short();
        let mark = self.mark as i8 as i32;
        story_lang!(self.text.as_mut_ptr(), 0xa21bdc, mark, mark, g, g, self.outlet_name.as_ptr(), self.name.as_ptr(), short);
    }

    /// Kind 0x13: the player's indiscipline (with bit 1 of the position mask, without the position text).
    unsafe fn indiscipline(&mut self) {
        player_story_piece("indiscipline (0x13)");
        covered!("nma_3 indiscipline");
        let (short, g) = self.club_short();
        let mark = self.mark as i8 as i32;
        let (text, name, outlet) = (self.text.as_mut_ptr(), self.name.as_ptr(), self.outlet_name.as_ptr());
        if self.position_mask as u8 & 1 == 1 {
            story_lang!(text, 0xa21acc, mark, mark, g, g, outlet, short, name);
        } else {
            let mg = self.mask_gender as i8 as i32;
            story_lang!(text, 0xa21998, mark, mark, g, g, mg, outlet, short, name, self.mask_text.as_ptr());
        }
    }

    /// Kind 0xa with a match: the player's contribution in the match, with the score and the other club.
    unsafe fn contribution_after_match(&mut self) {
        player_story_piece("contribution after a match (0xa)");
        covered!("nma_3 contribution after a match");
        self.write_score();
        let (short, g) = self.club_short();
        let (g1, g2) = self.goals();
        let c2 = self.other_club();
        let (c2name, c2g) = (c2 + club::NAME, read_i8(c2 + club::GENDER_NAME) as i32);
        let mark = self.mark as i8 as i32;
        let tpl = if g1 > g2 { 0xa21834 } else if g1 == g2 { 0xa216d0 } else { 0xa2151c };
        story_lang!(self.text.as_mut_ptr(), tpl, mark, mark, g, g, c2g, c2g, self.outlet_name.as_ptr(), short, self.name.as_ptr(),
            self.score.as_ptr(), c2name, self.third_name.as_ptr());
        if g1 == g2 {
            self.draw_result();
        }
    }


    /// The record of a nation by index (`G_NATION_TABLE` + 0x122 × index): its name (+4) and the name's gender byte.
    unsafe fn nation_by_index(i: i32) -> (usize, usize, i32) {
        let n = read_usize(G_NATION_TABLE).wrapping_add((i as usize).wrapping_mul(0x122));
        (n, n + 4, read_i8(n + nation::GENDER_NAME) as i32)
    }

    /// The club's nation: its short name and the short name's gender byte.
    unsafe fn club_nation_short(&self) -> (usize, i32) {
        let n = self.club_nation();
        (n + nation::SHORT_NAME, read_i8(n + nation::SHORT_GENDER_NAME) as i32)
    }

    /// Kind 9: the media questions the purchase of the player (slot 27: the rival manager's club).
    unsafe fn purchase(&mut self) {
        player_story_piece("purchase (9)");
        covered!("nma_3 purchase");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let local = self.local != 0;
        let c3 = read_usize(G_CLUB_TABLE).wrapping_add((self.slot27 as usize).wrapping_mul(club::SIZE));
        let (c3short, c3g) = (c3 + club::SHORT_NAME, read_i8(c3 + club::SHORT_GENDER_NAME) as i32);
        let rival = self.second_name.as_ptr();
        match self.subtype {
            0 if local => story_lang!(text, 0xa213f4, c3g, c3g, g, g, c3short, rival, name, short),
            0 => {
                self.club_adjective(0x1c);
                story_lang!(text, 0xa212ac, c3g, c3g, 0x64, 0x64, g, g, c3short, rival, self.adjective.as_ptr(), name, short);
            }
            1 => story_lang!(text, 0xa211e0, c3g, c3g, g, g, c3short, rival, short, name),
            2 if local => story_lang!(text, 0xa21124, g, g, name, short),
            2 => {
                self.club_adjective(0x16);
                story_lang!(text, 0xa21048, g, g, 0x64, name, short, self.adjective.as_ptr());
            }
            3 => story_lang!(text, 0xa20f8c, g, g, short, name),
            4 if local => story_lang!(text, 0xa20ec4, 0x64, g, g, name, short),
            4 => {
                self.club_adjective(0x1c);
                story_lang!(text, 0xa20ddc, 0x64, g, g, 0x64, 0x64, name, short, self.adjective.as_ptr());
            }
            _ if local => story_lang!(text, 0xa20d20, g, g, name, short),
            _ => {
                let (nshort, ng) = self.club_nation_short();
                story_lang!(text, 0xa20c48, ng, ng, g, g, nshort, name, short);
            }
        }
    }

    /// Kind 0x22: the player blasts his manager.
    unsafe fn blast_at_manager(&mut self) {
        player_story_piece("blast at manager (0x22)");
        covered!("nma_3 blast at manager");
        let (short, g) = self.club_short();
        let (text, name, manager) = (self.text.as_mut_ptr(), self.name.as_ptr(), self.third_name.as_ptr());
        if self.subtype != 0 {
            story_lang!(text, 0xa20b08, g, g, short, manager, name, self.third_name5.as_ptr());
        } else {
            story_lang!(text, 0xa209d0, g, g, 0x64, short, name, manager, self.name5.as_ptr());
        }
    }

    /// Kind 0x21: the player blames his manager for his spell of poor form.
    unsafe fn blames_manager(&mut self) {
        player_story_piece("blames manager (0x21)");
        covered!("nma_3 blames manager");
        let (short, g) = self.club_short();
        let (text, name, name5, manager) = (self.text.as_mut_ptr(), self.name.as_ptr(), self.name5.as_ptr(), self.third_name.as_ptr());
        match self.subtype {
            0 => story_lang!(text, 0xa203d0, 0x64, name, manager),
            1 => story_lang!(text, 0xa20494, g, g, 0x64, short, name, manager, name5),
            2 => story_lang!(text, 0xa205dc, 0x64, name, manager, name5, self.third_name5.as_ptr()),
            _ if self.local != 0 => story_lang!(text, 0xa2086c, g, g, 0x64, short, name, name5, manager),
            _ => {
                let (nshort, ng) = self.club_nation_short();
                story_lang!(text, 0xa206e4, g, g, ng, ng, 0x64, short, name, nshort, name5, manager);
            }
        }
    }

    /// Kind 8: the media questions the player's attitude.
    unsafe fn attitude(&mut self) {
        player_story_piece("attitude (8)");
        covered!("nma_3 attitude");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let mark = self.mark as i8 as i32;
        if self.subtype != 0 {
            story_lang!(text, 0xa202f8, g, mark, mark, g, 0x64, self.outlet_name.as_ptr(), name, short);
        } else if self.local != 0 {
            story_lang!(text, 0xa20244, g, g, 0x64, name, short);
        } else {
            let (nshort, ng) = self.club_nation_short();
            story_lang!(text, 0xa20174, ng, ng, g, g, 0x64, nshort, name, short);
        }
    }

    /// Kind 1: the player's private life in the papers (slot 7: a nation, slot 8: a city). Subtypes above 6 write
    /// nothing (the original leaves its text buffer as it was).
    unsafe fn private_life(&mut self) {
        player_story_piece("private life (1)");
        covered!("nma_3 private life");
        let (short, g) = self.club_short();
        let (text, name, outlet) = (self.text.as_mut_ptr(), self.name.as_ptr(), self.outlet_name.as_ptr());
        let mark = self.mark as i8 as i32;
        let local = self.local != 0;
        let slot8 = self.slot8;
        let city = || {
            let c = read_usize(G_CITY_TABLE).wrapping_add((slot8 as usize).wrapping_mul(0x38));
            (c + 4, read_i8(c + city::GENDER_NAME) as i32)
        };
        match self.subtype {
            0 => story_lang!(text, 0xa2006c, mark, mark, g, g, 0x64, outlet, name, short),
            1 => {
                let (_, nname, natg) = Self::nation_by_index(self.slot7);
                story_lang!(text, 0xa1ff4c, mark, mark, 0x64, natg, natg, g, g, outlet, name, nname, short);
            }
            2 if local => story_lang!(text, 0xa1fe8c, g, g, name, short),
            2 => {
                let (nshort, ng) = self.club_nation_short();
                story_lang!(text, 0xa1fdb0, ng, ng, g, g, nshort, name, short);
            }
            3 if local => {
                let (cname, cg) = city();
                story_lang!(text, 0xa1fcd0, cg, cg, name, cname);
            }
            3 => {
                self.club_adjective(0x1d);
                let (cname, cg) = city();
                story_lang!(text, 0xa1fbd4, 0x64, cg, cg, self.adjective.as_ptr(), name, cname);
            }
            4 | 6 => {
                let (_, nname, natg) = Self::nation_by_index(self.slot7);
                let tpl = if self.subtype == 4 { 0xa1fb00 } else { 0xa1f8ec };
                story_lang!(text, tpl, mark, mark, g, g, natg, natg, outlet, short, name, nname);
            }
            5 => {
                let (n, _, _) = Self::nation_by_index(self.slot7);
                exe::nation_get_nationality_adjective(n, self.adjective.as_mut_ptr() as usize, 0x78, 2);
                story_lang!(text, 0xa1f9f0, mark, mark, g, g, 0x64, 0x64, outlet, short, name, self.adjective.as_ptr());
            }
            _ => {}
        }
    }

    /// Kind 0x25: the player is concerned by a transfer rumour.
    unsafe fn transfer_rumour(&mut self) {
        player_story_piece("transfer rumour (0x25)");
        covered!("nma_3 transfer rumour");
        let c = self.club_rec();
        let mark = self.mark as i8 as i32;
        let (cname, cg) = (c + club::NAME, read_i8(c + club::GENDER_NAME) as i32);
        let mg = self.mask_gender as i8 as i32;
        story_lang!(self.text.as_mut_ptr(), 0xa1f7b4, mark, mark, 0x64, cg, cg, mg, 0x64, self.outlet_name.as_ptr(), self.name.as_ptr(), cname,
            self.mask_text.as_ptr(), self.fourth_name.as_ptr());
    }

    /// Kind 0x1e: an exclusive article about the player.
    unsafe fn exclusive(&mut self) {
        player_story_piece("exclusive (0x1e)");
        covered!("nma_3 exclusive");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let local = self.local != 0;
        if self.subtype != 0 {
            if local {
                story_lang!(text, 0xa1f6d8, g, g, name, short);
            } else {
                self.club_adjective(0x19);
                story_lang!(text, 0xa1f5d8, 0x64, g, g, self.adjective.as_ptr(), name, short);
            }
        } else if local {
            story_lang!(text, 0xa1f4f0, 0x64, g, g, name, short);
        } else {
            self.club_adjective(0x19);
            story_lang!(text, 0xa1f3e8, 0x64, 0x64, 0x64, g, g, self.adjective.as_ptr(), name, short);
        }
    }

    /// Kind 7: the player is too young?
    unsafe fn too_young(&mut self) {
        player_story_piece("too young (7)");
        covered!("nma_3 too young");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let mark = self.mark as i8 as i32;
        if self.subtype == 0 {
            story_lang!(text, 0xa1f1b0, mark, mark, g, g, self.outlet_name.as_ptr(), name, short);
        } else if self.local != 0 {
            story_lang!(text, 0xa1f334, g, g, short, name);
        } else {
            let (nshort, ng) = self.club_nation_short();
            story_lang!(text, 0xa1f268, ng, ng, g, g, nshort, short, name);
        }
    }

    /// Kind 2: the media raises a question about the player.
    unsafe fn question(&mut self) {
        player_story_piece("question (2)");
        covered!("nma_3 question");
        let (short, g) = self.club_short();
        let mark = self.mark as i8 as i32;
        story_lang!(self.text.as_mut_ptr(), 0xa1f0f8, mark, mark, 0x64, g, g, self.outlet_name.as_ptr(), self.name.as_ptr(), short);
    }

    /// Kind 0x20: rumours about the player (the supporters' club, the press).
    unsafe fn rumours(&mut self) {
        player_story_piece("rumours (0x20)");
        covered!("nma_3 rumours");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        match self.subtype {
            0 => story_lang!(text, 0xa1edf4, g, g, 0x64, short, name),
            1 if self.local != 0 => story_lang!(text, 0xa1efb4, g, g, name, short),
            1 => {
                self.club_adjective(0x1c);
                story_lang!(text, 0xa1eef0, 0x64, 0x64, g, g, self.adjective.as_ptr(), name, short);
            }
            _ => story_lang!(text, 0xa1f058, g, g, name, short),
        }
    }

    /// Kind 3: the player is out of favour (slot 7: a nation).
    unsafe fn out_of_favour(&mut self) {
        player_story_piece("out of favour (3)");
        covered!("nma_3 out of favour");
        let (short, g) = self.club_short();
        let (text, name, outlet) = (self.text.as_mut_ptr(), self.name.as_ptr(), self.outlet_name.as_ptr());
        let mark = self.mark as i8 as i32;
        let local = self.local != 0;
        match self.subtype {
            0 => {
                let (_, nname, natg) = Self::nation_by_index(self.slot7);
                story_lang!(text, 0xa1ecc4, mark, mark, g, g, natg, natg, outlet, name, short, nname);
            }
            1 => story_lang!(text, 0xa1ebd8, mark, mark, g, g, outlet, name, short),
            2 if local => story_lang!(text, 0xa1ead4, g, g, name, short),
            2 => {
                let (nshort, ng) = self.club_nation_short();
                story_lang!(text, 0xa1e9b0, ng, ng, g, g, nshort, name, short);
            }
            3 if local => story_lang!(text, 0xa1e8f0, g, g, name, short),
            3 => {
                self.club_adjective(0x1b);
                story_lang!(text, 0xa1e808, 0x64, 0x64, g, g, self.adjective.as_ptr(), name, short);
            }
            _ => story_lang!(text, 0xa1e72c, mark, mark, g, g, outlet, name, short),
        }
    }

    /// Kind 4: the player is past it?
    unsafe fn past_it(&mut self) {
        player_story_piece("past it (4)");
        covered!("nma_3 past it");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let local = self.local != 0;
        match self.subtype {
            0 => story_lang!(text, 0xa1e3a4, 0x64, g, g, name, short),
            1 if local => story_lang!(text, 0xa1e504, name),
            1 => {
                let (nshort, ng) = self.club_nation_short();
                story_lang!(text, 0xa1e448, ng, ng, nshort, name);
            }
            _ if local => story_lang!(text, 0xa1e678, g, g, name, short),
            _ => {
                self.club_adjective(0x1b);
                story_lang!(text, 0xa1e5a4, 0x64, 0x64, g, g, self.adjective.as_ptr(), name, short);
            }
        }
    }


    /// Kind 5: the pressure gets to the player.
    unsafe fn pressure(&mut self) {
        player_story_piece("pressure (5)");
        covered!("nma_3 pressure");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        let local = self.local != 0;
        if self.subtype == 0 {
            if local {
                story_lang!(text, 0xa1e150, g, g, name, short);
            } else {
                let (nshort, ng) = self.club_nation_short();
                story_lang!(text, 0xa1e084, ng, ng, g, g, nshort, name, short);
            }
        } else if local {
            story_lang!(text, 0xa1e2e0, 0x64, g, g, name, short);
        } else {
            let (nshort, ng) = self.club_nation_short();
            story_lang!(text, 0xa1e1fc, ng, ng, 0x64, g, g, nshort, name, short);
        }
    }

    /// Kind 6: the player's indiscipline costs the club dear.
    unsafe fn indiscipline_costs(&mut self) {
        player_story_piece("indiscipline costs (6)");
        covered!("nma_3 indiscipline costs");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        if self.local != 0 {
            story_lang!(text, 0xa1dfb4, 0x64, g, g, name, short);
        } else {
            self.club_adjective(0x1e);
            story_lang!(text, 0xa1dec0, 0x64, 0x64, 0x64, g, g, self.adjective.as_ptr(), name, short);
        }
    }

    /// Kind 0x10: the player's poor performances and the club's poor run. The foreign text passes the club's gender
    /// byte where the club's name goes (a defect of the original, kept).
    unsafe fn poor_run(&mut self) {
        player_story_piece("poor run (0x10)");
        covered!("nma_3 poor run");
        let (short, g) = self.club_short();
        let (text, name) = (self.text.as_mut_ptr(), self.name.as_ptr());
        if self.local != 0 {
            story_lang!(text, 0xa1de14, 0x64, g, g, name, short);
        } else {
            self.club_adjective(0x1e);
            story_lang!(text, 0xa1dd44, 0x64, 0x64, 0x64, g, g, self.adjective.as_ptr(), name, g);
        }
    }

    /// Kind 0xa without a match and the kinds without a text: the programmers' placeholder (C runtime sprintf).
    unsafe fn placeholder(&mut self) {
        player_story_piece("placeholder (other kinds)");
        covered!("nma_3 placeholder");
        story_crt!(self.text.as_mut_ptr(), 0xa1dcbc, self.name.as_ptr());
    }

    /// Kinds 0 and 0x12: the media criticises the player. Subtypes 0..7 pick the text; above 7, the bits of slot 18
    /// pick it, and without one the subtype becomes player % 8 (written back to the subtype) and is tried again.
    unsafe fn criticism(&mut self) {
        player_story_piece("criticism (0, 0x12)");
        covered!("nma_3 criticism");
        let (short, g) = self.club_short();
        let (text, name, name5, outlet, manager) =
            (self.text.as_mut_ptr(), self.name.as_ptr(), self.name5.as_ptr(), self.outlet_name.as_ptr(), self.third_name.as_ptr());
        let mark = self.mark as i8 as i32;
        let flags = self.slot18 as u32;
        let local = self.local != 0;
        let has = |m: u32| flags & m == m;
        loop {
            match self.subtype as u32 {
                0 if local => story_lang!(text, 0xa1dbe0, g, g, 0x64, short, name),
                0 => {
                    let (nshort, ng) = self.club_nation_short();
                    story_lang!(text, 0xa1db04, ng, ng, g, g, 0x64, nshort, short, name);
                }
                1 => story_lang!(text, 0xa1da44, mark, mark, 0x64, g, g, outlet, name, short),
                2 => story_lang!(text, 0xa1d98c, g, g, 0x64, short, name),
                3 => story_lang!(text, 0xa1d8c8, g, g, 0x64, short, name),
                4 if local => story_lang!(text, 0xa1d838, g, g, 0x64, short, name),
                4 => {
                    self.club_adjective(0x1e);
                    story_lang!(text, 0xa1d78c, 0x64, g, g, 0x64, self.adjective.as_ptr(), short, name);
                }
                5 if local => story_lang!(text, 0xa1d690, g, g, 0x64, 0x64, short, manager, name),
                5 => {
                    let (nshort, ng) = self.club_nation_short();
                    story_lang!(text, 0xa1d578, ng, ng, g, g, 0x64, 0x64, nshort, short, manager, name);
                }
                6 if local => story_lang!(text, 0xa1d470, g, g, 0x64, short, manager, name),
                6 => {
                    let (nshort, ng) = self.club_nation_short();
                    story_lang!(text, 0xa1d34c, ng, ng, g, g, 0x64, nshort, short, manager, name);
                }
                7 if local => story_lang!(text, 0xa1d20c, g, g, 0x64, short, manager, name),
                7 => {
                    self.club_adjective(0x1d);
                    story_lang!(text, 0xa1d0a8, 0x64, 0x64, g, g, 0x64, self.adjective.as_ptr(), short, manager, name);
                }
                _ if has(0x400200) => story_lang!(text, 0xa1cfd4, mark, mark, g, g, 0x64, outlet, name, short),
                _ if has(0x102200) => story_lang!(text, 0xa1ce94, mark, mark, 0x64, g, g, 0x64, outlet, name, short, name5),
                _ if has(0x104200) => story_lang!(text, 0xa1cd30, mark, mark, 0x64, g, g, outlet, name, short, name5),
                _ if has(0x100200) => story_lang!(text, 0xa1cc68, mark, mark, 0x64, g, g, outlet, name, short),
                _ if has(0x480) => story_lang!(text, 0xa1cb84, mark, mark, g, g, outlet, name, short),
                _ if has(0x500) => story_lang!(text, 0xa1c9ec, g, g, mark, mark, short, outlet, name),
                _ if has(0x20820) => story_lang!(text, 0xa1c8e8, 0x64, g, g, mark, mark, name, short, outlet),
                _ if has(0x1020) => story_lang!(text, 0xa1c7b4, 0x64, g, g, mark, mark, 0x64, name, short, outlet, name5),
                _ if has(0x20810) && local => story_lang!(text, 0xa1c6bc, 0x64, g, g, name, short),
                _ if has(0x20810) => {
                    self.club_adjective(0x1d);
                    story_lang!(text, 0xa1c5b0, 0x64, g, g, 0x64, name, short, self.adjective.as_ptr());
                }
                _ if has(0x10001) && local => story_lang!(text, 0xa1c480, g, g, short, name, name5),
                _ if has(0x10001) => {
                    let (nshort, ng) = self.club_nation_short();
                    story_lang!(text, 0xa1c334, g, g, ng, ng, short, name, nshort, name5);
                }
                _ if flags & 1 != 0 && local => story_lang!(text, 0xa1c204, g, g, short, name, name5),
                _ if flags & 1 != 0 => {
                    self.club_adjective(0x1e);
                    story_lang!(text, 0xa1c0b8, g, g, 0x64, short, name, self.adjective.as_ptr(), name5);
                }
                _ if flags as u8 & 0xa == 0xa => story_lang!(text, 0xa1bf7c, g, g, mark, mark, short, name, outlet, name5),
                _ if flags as u8 & 8 == 8 && local => story_lang!(text, 0xa1bed8, g, g, short, name),
                _ if flags as u8 & 8 == 8 => {
                    let (nshort, ng) = self.club_nation_short();
                    story_lang!(text, 0xa1be08, g, g, ng, ng, short, name, nshort);
                }
                _ if flags as u8 & 2 == 2 && local => story_lang!(text, 0xa1bcf8, g, g, 0x64, short, name, name5),
                _ if has(0x4000) => story_lang!(text, 0xa1bc50, g, g, short, name),
                _ if has(0x20000) => story_lang!(text, 0xa1bba0, g, g, short, name),
                _ if has(0x40000) && flags as u8 & 0x30 == 0 => story_lang!(text, 0xa1ba44, g, g, short, name, name5),
                _ => {
                    self.subtype = self.player % 8; // the C remainder (a negative player would loop for ever, as in the original)
                    continue;
                }
            }
            return;
        }
    }


    /// The kind dispatch of the story text (kind = slot 9).
    unsafe fn write_story(&mut self) {
        match self.kind {
            0x23 => self.left_out(),
            0xe => self.ability(),
            0xd => self.complacency(),
            0x11 => self.contribution(),
            0 | 0x12 => self.criticism(),
            0xc => self.commitment(),
            0x13 => self.indiscipline(),
            0xa if self.with_fixture != 0 => self.contribution_after_match(),
            9 => self.purchase(),
            0x22 => self.blast_at_manager(),
            0x21 => self.blames_manager(),
            8 => self.attitude(),
            1 => self.private_life(),
            0x25 => self.transfer_rumour(),
            0x1e => self.exclusive(),
            7 => self.too_young(),
            2 => self.question(),
            0x20 => self.rumours(),
            3 => self.out_of_favour(),
            4 => self.past_it(),
            5 => self.pressure(),
            6 => self.indiscipline_costs(),
            0x10 => self.poor_run(),
            _ => self.placeholder(),
        }
    }

    /// Kinds 0 and 0x12 with a subtype above 7 (picked by the bits of slot 18): the bits of a "bad" story.
    fn bad_flags(&self) -> bool {
        let f = self.slot18 as u32;
        [0x400200, 0x102200, 0x104200, 0x100200, 0x480, 0x500, 0x20820, 0x1020, 0x20810, 0x10001].iter().any(|&m| f & m == m) || f & 1 != 0
    }

    /// Kinds 0 and 0x12 with a subtype above 7, without the bad bits: the bits of a player coping with the pressure.
    fn coping_flags(&self) -> bool {
        let f = self.slot18 as u32;
        f as u8 & 0xa == 0xa || f as u8 & 8 == 8 || (f as u8 & 2 == 2 && self.local != 0)
    }

    /// Kinds 0 and 0x12 with a subtype above 7.
    fn criticism_by_flags(&self) -> bool {
        (self.kind == 0 || self.kind == 0x12) && self.subtype > 7
    }

    /// The parity of the subtype (C remainder by 2).
    fn subtype_even(&self) -> bool {
        self.subtype % 2 == 0
    }

    /// The reply about the player's role in the squad (slot 16: the role, slot 17: the age), by the human manager
    /// (`extra`, with the player's name in form 5).
    unsafe fn human_squad_role(&mut self) {
        let (extra, name5) = (self.extra.as_mut_ptr(), self.name5.as_ptr());
        match self.slot16 {
            1..=3 => story_lang!(extra, 0xa1af60),
            4 if self.slot17 > 0x1a => {
                if self.slot18 as u32 & 0x800000 != 0 {
                    story_lang!(extra, 0xa1b14c);
                } else if self.subtype_even() {
                    story_lang!(extra, 0xa1b074, name5);
                } else {
                    story_lang!(extra, 0xa1b0d0, 0x64, name5);
                }
            }
            4 if self.slot17 > 0x17 => story_lang!(extra, 0xa1b008, name5),
            4 | 5 => story_lang!(extra, 0xa1aefc, name5),
            6 => story_lang!(extra, 0xa1ae98, name5),
            _ => story_lang!(extra, 0xa1b0d0, 0x64, name5),
        }
    }

    /// The same reply by the player's club manager (the third person, slot 23).
    unsafe fn club_manager_squad_role(&mut self) {
        let (extra, name5, manager) = (self.extra.as_mut_ptr(), self.name5.as_ptr(), self.third_name.as_ptr());
        match self.slot16 {
            1 => story_lang!(extra, 0xa19c1c, manager, name5),
            2 | 3 => story_lang!(extra, 0xa19b94, manager, name5),
            4 if self.slot17 > 0x1a => {
                if self.slot18 as u32 & 0x800000 != 0 {
                    story_lang!(extra, 0xa19b10, manager, name5);
                } else if self.subtype_even() {
                    story_lang!(extra, 0xa1996c, manager, name5);
                } else {
                    story_lang!(extra, 0xa19a70, 0x64, manager, name5);
                }
            }
            4 if self.slot17 > 0x17 => story_lang!(extra, 0xa198d8, manager, name5),
            4 | 5 => story_lang!(extra, 0xa1984c, manager, name5),
            6 => story_lang!(extra, 0xa197c4, manager, name5),
            _ => story_lang!(extra, 0xa19a70, 0x64, manager, name5),
        }
    }

    /// The paragraph after the story (`extra`, slot 10 = the reply): when the third person is the human manager on
    /// screen, his reply (1 support, 2 criticise, 3 ignore); otherwise the club manager's (1 support, 2 criticise).
    unsafe fn write_reply(&mut self) {
        let manager = read_usize(G_SCREEN_RECORDS_MANAGER + (read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300));
        let extra = self.extra.as_mut_ptr();
        if read_usize(G_STAFF_TABLE).wrapping_add(self.third_off) == manager {
            player_story_piece("manager reply");
            covered!("nma_3 manager reply");
            match self.reader_case {
                1 => self.human_supports(),
                2 => self.human_criticises(),
                3 => story_lang!(extra, 0xa1ba14), // "You have decided to ignore this news report."
                _ => story_crt!(extra, 0xad9c64),
            }
        } else {
            player_story_piece("club manager reply");
            covered!("nma_3 club manager reply");
            match self.reader_case {
                1 => self.club_manager_supports(),
                2 => self.club_manager_criticises(),
                _ => story_crt!(extra, 0xad9c64),
            }
        }
    }

    /// The human manager criticises the player (reply 2).
    unsafe fn human_criticises(&mut self) {
        let (extra, name5) = (self.extra.as_mut_ptr(), self.name5.as_ptr());
        if self.criticism_by_flags() {
            if !self.bad_flags() && self.coping_flags() {
                story_lang!(extra, 0xa1b958);
            } else {
                story_lang!(extra, 0xa1b9b4, 0x64, name5);
            }
            return;
        }
        match self.kind {
            0x23 => story_lang!(extra, 0xa1b8e4),
            0x11 => story_lang!(extra, 0xa1b86c, 0x64, name5),
            0 => story_lang!(extra, 0xa1b9b4, 0x64, name5),
            0xe => story_lang!(extra, 0xa1b81c),
            0xc | 0xd => story_lang!(extra, 0xa1b18c, 0x64, name5),
            0x13 => story_lang!(extra, 0xa1b79c, name5),
            0xa | 0x21 | 0x22 => story_lang!(extra, 0xa1b6ec, name5),
            8 => story_lang!(extra, 0xa1b674),
            1 => story_lang!(extra, 0xa1b608, name5),
            2 | 9 => story_lang!(extra, 0xa1b1fc, 0x64, name5),
            0x25 => story_lang!(extra, 0xa1b5c8),
            0x1e | 0x20 => story_lang!(extra, 0xa1b268, 0x64, name5),
            3 => story_lang!(extra, 0xa1b56c, 0x64, name5),
            4 => {
                let (short, g) = self.club_short();
                let tpl = if self.slot14_again != 0 { 0xa1b4c0 } else { 0xa1b414 };
                story_lang!(extra, tpl, g, g, name5, short);
            }
            5 | 7 => story_lang!(extra, 0xa1b2dc, name5),
            6 => story_lang!(extra, 0xa1b378, 0x64, name5),
            _ => story_lang!(extra, 0xad9c64),
        }
    }

    /// The human manager supports the player (reply 1).
    unsafe fn human_supports(&mut self) {
        let (extra, name5) = (self.extra.as_mut_ptr(), self.name5.as_ptr());
        if self.criticism_by_flags() {
            if !self.bad_flags() && self.coping_flags() {
                story_lang!(extra, 0xa1afa4);
            } else {
                self.human_squad_role();
            }
            return;
        }
        match self.kind {
            0x23 => story_lang!(extra, 0xa1ae44),
            0x13 if self.byte_3272 as i8 > 3 => story_lang!(extra, 0xa1ad78, name5),
            0x13 => story_lang!(extra, 0xa1acdc, name5),
            0xa => story_lang!(extra, 0xa1ac78, name5),
            0xc | 0xd => story_lang!(extra, 0xa1a6ac),
            0xe => story_lang!(extra, 0xa1ac38),
            0x11 => story_lang!(extra, 0xa1abcc, 0x64, name5),
            0 if self.subtype == 3 => story_lang!(extra, 0xa1ab6c),
            0 => self.human_squad_role(),
            0x21 | 0x22 => story_lang!(extra, 0xa1a714, name5),
            8 => story_lang!(extra, 0xa1ab14),
            1 => story_lang!(extra, 0xa1aabc, name5),
            2 | 9 => story_lang!(extra, 0xa1a77c, name5),
            0x25 => {
                let mg = self.mask_gender as i8 as i32;
                story_lang!(extra, 0xa1aa18, mg, 0x64, self.mask_text.as_ptr(), name5);
            }
            0x1e | 0x20 => story_lang!(extra, 0xa1a7e0),
            3 => story_lang!(extra, 0xa1a9b8, name5),
            4 => {
                let (short, g) = self.club_short();
                story_lang!(extra, 0xa1a920, g, g, name5, short);
            }
            5 | 7 => story_lang!(extra, 0xa1a81c, name5),
            6 => story_lang!(extra, 0xa1a8a0, 0x64, name5),
            _ => story_lang!(extra, 0xad9c64),
        }
    }

    /// The club manager criticises the player (reply 2).
    unsafe fn club_manager_criticises(&mut self) {
        let (extra, name, name5, manager, manager5) =
            (self.extra.as_mut_ptr(), self.name.as_ptr(), self.name5.as_ptr(), self.third_name.as_ptr(), self.third_name5.as_ptr());
        if self.criticism_by_flags() {
            if !self.bad_flags() && self.coping_flags() {
                story_lang!(extra, 0xa1a5ec, manager);
            } else {
                story_lang!(extra, 0xa1a65c, manager);
            }
            return;
        }
        let (short, g) = self.club_short();
        match self.kind {
            0xe => story_lang!(extra, 0xa1a580, manager),
            0x11 => story_lang!(extra, 0xa1a4e8, 0x64, manager, name5),
            0 | 2 | 9 => story_lang!(extra, 0xa1a65c, manager),
            0xc | 0xd => story_lang!(extra, 0xa19ca0, 0x64, manager, name5),
            8 => story_lang!(extra, 0xa1a440, manager, name),
            0x13 if self.byte_3271 != 0 => story_lang!(extra, 0xa1a3a8, manager5, name5),
            0x13 => story_lang!(extra, 0xa1a310, manager5, name5),
            0x21 | 0xa | 0x22 => story_lang!(extra, 0xa19d2c, manager5, name5),
            1 => story_lang!(extra, 0xa1a254, g, g, short, manager, name5),
            0x25 => story_lang!(extra, 0xa1a164, manager),
            0x1e | 0x20 => story_lang!(extra, 0xa19e14, manager),
            3 => story_lang!(extra, 0xa1a0dc, manager, name5),
            4 => story_lang!(extra, 0xa1a014, g, g, manager, name5, short),
            6 => story_lang!(extra, 0xa19f64, 0x64, manager, name5),
            5 | 7 => story_lang!(extra, 0xa19e80, g, g, short, manager, name5),
            _ => story_crt!(extra, 0xad9c64),
        }
    }

    /// The club manager supports the player (reply 1).
    unsafe fn club_manager_supports(&mut self) {
        let (extra, name5, manager, manager5) =
            (self.extra.as_mut_ptr(), self.name5.as_ptr(), self.third_name.as_ptr(), self.third_name5.as_ptr());
        if self.criticism_by_flags() {
            if !self.bad_flags() && self.coping_flags() {
                story_lang!(extra, 0xa199ec, manager);
            } else {
                self.club_manager_squad_role();
            }
            return;
        }
        let (short, g) = self.club_short();
        match self.kind {
            0xe => story_lang!(extra, 0xa1975c, manager),
            0x11 => story_lang!(extra, 0xa196d0, 0x64, manager, name5),
            0 if self.subtype == 3 => story_lang!(extra, 0xa19644, manager),
            0 => self.club_manager_squad_role(),
            0xc | 0xd => story_lang!(extra, 0xa18e44, manager, name5),
            8 => story_lang!(extra, 0xa195e0, manager),
            1 => story_lang!(extra, 0xa19560, manager, name5),
            0x13 if self.byte_3272 as i8 > 3 => story_lang!(extra, 0xa1947c, name5),
            0x13 => story_lang!(extra, 0xa193c8, name5),
            0x21 | 0xa | 0x22 => story_lang!(extra, 0xa18eec, manager5, name5),
            2 | 9 => story_lang!(extra, 0xa18f70, manager, name5),
            0x25 => {
                let mg = self.mask_gender as i8 as i32;
                story_lang!(extra, 0xa19324, mg, 0x64, manager, self.mask_text.as_ptr());
            }
            0x1e | 0x20 => story_lang!(extra, 0xa18ffc, manager),
            3 => story_lang!(extra, 0xa192a0, manager, name5),
            4 => story_lang!(extra, 0xa191e4, g, g, manager, name5, short),
            5 | 7 => story_lang!(extra, 0xa19060, g, g, short, manager, name5),
            6 => story_lang!(extra, 0xa19128, 0x64, manager, name5),
            _ => story_crt!(extra, 0xad9c64),
        }
    }

    /// The links of a person when his record matches the names (`exe::database_staff_matches_names`): one per text
    /// given, type 0x7d7, the person's record.
    unsafe fn link_person(out: usize, index: i32, names: [i32; 3], texts: &[usize]) {
        let rec = read_usize(G_STAFF_TABLE).wrapping_add((index as usize).wrapping_mul(staff::SIZE));
        if exe::database_staff_matches_names(rec, names[0] as u32, names[1] as u32, names[2] as u32, 0usize) != 0 {
            for &t in texts {
                exe::text_add_link(out, t as *const u8, 0x7d7, rec);
            }
        }
    }

    /// The body's text ("%s\n\n%s": the story and the reply) and its links: the persons, the club's nation (name,
    /// short name and the adjectives of forms 0x16..0x1f, to the national team), the club, and after a match the score
    /// (to the fixture, when `Match_Stats::fixture_entry_matches` finds it) and the other club.
    unsafe fn finish(&mut self, out: usize) {
        player_story_piece("links");
        covered!("nma_3 links");
        story_crt!(self.long_extra.as_mut_ptr(), 0x991948, self.text.as_ptr(), self.extra.as_ptr());
        exe::text_set_string(out, self.long_extra.as_ptr());
        if self.second > -1 {
            Self::link_person(out, self.second, self.second_names, &[self.second_name.as_ptr() as usize]);
        }
        if self.player != -1 {
            Self::link_person(out, self.player, self.names, &[self.name.as_ptr() as usize, self.name5.as_ptr() as usize]);
        }
        if self.fourth != -1 {
            Self::link_person(out, self.fourth, self.fourth_names, &[self.fourth_name.as_ptr() as usize]);
        }
        if self.third != -1 {
            Self::link_person(out, self.third, self.third_names, &[self.third_name5.as_ptr() as usize, self.third_name.as_ptr() as usize]);
        }
        if self.club != -1 {
            let team = |st: &PlayerStory| exe::database_get_national_team(st.club_nation());
            let t = team(self);
            exe::text_add_link(out, (self.club_nation() + 4) as *const u8, 0x7d5, t);
            let t = team(self);
            exe::text_add_link(out, (self.club_nation() + nation::SHORT_NAME) as *const u8, 0x7d5, t);
            for form in 0x16..=0x1f {
                self.club_adjective(form);
                let t = team(self);
                exe::text_add_link(out, self.adjective.as_ptr(), 0x7d5, t);
            }
            let c = self.club_rec();
            exe::text_add_link(out, (c + club::SHORT_NAME) as *const u8, 0x7d5, c);
            exe::text_add_link(out, (c + club::NAME) as *const u8, 0x7d5, c);
        }
        if self.with_fixture == 0 {
            return;
        }
        let d = |k: usize| u32::from_le_bytes([self.fixture[k], self.fixture[k + 1], self.fixture[k + 2], self.fixture[k + 3]]);
        let matches: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(PLAYER_STORY_FIXTURE_ENTRY_MATCHES);
        if matches(read_usize(G_MATCH_STATS), d(0x1b), d(0x11), d(0x15), d(0), d(2)) != 0 {
            exe::text_add_link(out, self.score.as_ptr(), 0x7d3, d(0x1b) as usize);
        }
        let c2 = self.other_club();
        exe::text_add_link(out, (c2 + club::NAME) as *const u8, 0x7d5, c2);
    }
}

/// The headline (`body` 0): by the story kind (slot 9), with the player's name in form 5 (slots 0..3) and the second
/// person's (slots 11..14); links: the player, the second person, the club (short name and name) and its nation (to
/// the national team).
unsafe fn player_story_headline(news: usize, out: usize) {
    player_story_piece("headline");
    covered!("nma_3 headline");
    let s = |k: i8| exe::newsitem_get_slot(news, k as u32) as i32;
    let player = s(0);
    let names = [s(1), s(2), s(3)];
    let club = s(4);
    let subtype = s(5);
    s(6);
    s(6); // the original asks for slot 6 twice and never for 7 and 8
    let kind = s(9);
    s(10);
    let second = s(11);
    let second_names = [s(12), s(13), s(14)];
    s(18);
    s(23);
    s(24);
    s(25);
    s(26);
    let mut mask_text = [0u8; 0x200];
    let mask = s(0x20);
    let mask_text_fn: unsafe extern "C" fn(u32, usize, u32) -> u8 = core::mem::transmute(PLAYER_STORY_POSITION_MASK_GET_TEXT);
    mask_text_fn(mask as u32, mask_text.as_mut_ptr() as usize, 1);
    let c = read_usize(G_CLUB_TABLE).wrapping_add((club as usize).wrapping_mul(club::SIZE));
    let mut name = [0u8; 0x200];
    let mut second_name = [0u8; 0x200];
    let mut text = [0u8; 0x1388];
    // the size given (0x190) is larger than the original's buffers (0xc8 and 0x78)
    exe::database_format_name_from_ids(names[0] as u32, names[1] as u32, names[2] as u32, name.as_mut_ptr(), 0x190, 5, player as u32);
    if second != -1 {
        exe::database_format_name_from_ids(second_names[0] as u32, second_names[1] as u32, second_names[2] as u32, second_name.as_mut_ptr(), 0x190, 5, player as u32);
    } else {
        second_name[0] = 0;
    }
    let (t, n) = (text.as_mut_ptr(), name.as_ptr());
    let (short, g) = (c + club::SHORT_NAME, read_i8(c + club::SHORT_GENDER_NAME) as i32);
    match kind {
        0x23 => story_lang!(t, 0xa18e14, n),
        0xe => story_lang!(t, 0xa18dd4, 0x64, n),
        4 => story_lang!(t, 0xa18da4, n),
        0xd => story_lang!(t, 0xa18d64, n),
        0x11 => match (club / (subtype + 1) + player) % 3 {
            0 => story_lang!(t, 0xa18c94, 0x64, n),
            1 => story_lang!(t, 0xa18cdc, n),
            _ => story_lang!(t, 0xa18d18, 0x64, n),
        },
        0 => match subtype % 3 {
            0 => story_lang!(t, 0xa18c60, n),
            1 => story_lang!(t, 0xa18c2c, n),
            _ => story_lang!(t, 0xa18bf8, n),
        },
        9 if subtype == 0 || subtype == 1 => story_lang!(t, 0xa18b50, 0x64, second_name.as_ptr(), n),
        9 => story_lang!(t, 0xa18bb4, 0x64, n),
        0x13 | 0x21 | 0xa => story_lang!(t, 0xa188d8, g, g, short),
        0x22 => story_lang!(t, 0xa18b18, n),
        8 => story_lang!(t, 0xa18ad4, 0x64, n),
        3 => story_lang!(t, 0xa18aa0, n),
        5 => story_lang!(t, 0xa18a68, n),
        7 => story_lang!(t, 0xa18a34, n),
        6 => story_lang!(t, 0xa189d0, 0x64, g, g, n, short),
        0xc => story_lang!(t, 0xa18990, 0x64, n),
        0x25 => story_lang!(t, 0xa18950, n),
        _ => story_lang!(t, 0xa18910, 0x64, n),
    }
    exe::text_set_string(out, text.as_ptr());
    let rec = read_usize(G_STAFF_TABLE).wrapping_add((player as usize).wrapping_mul(staff::SIZE));
    if exe::database_staff_matches_names(rec, names[0] as u32, names[1] as u32, names[2] as u32, 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7d7, rec);
    }
    if second > -1 {
        let rec = read_usize(G_STAFF_TABLE).wrapping_add((second as usize).wrapping_mul(staff::SIZE));
        if exe::database_staff_matches_names(rec, second_names[0] as u32, second_names[1] as u32, second_names[2] as u32, 0usize) != 0 {
            exe::text_add_link(out, second_name.as_ptr(), 0x7d7, rec);
        }
    }
    if club == -1 {
        return;
    }
    exe::text_add_link(out, (c + club::SHORT_NAME) as *const u8, 0x7d5, c);
    exe::text_add_link(out, (c + club::NAME) as *const u8, 0x7d5, c);
    let n = read_usize(c + club::NATION);
    let t = exe::database_get_national_team(n);
    exe::text_add_link(out, (read_usize(c + club::NATION) + 4) as *const u8, 0x7d5, t);
}

/// `Media::write_player_media_story_news_text(news, body, out)`: writes the headline or the body of a media story about
/// a player into the text `out`.
///
/// The headline (`body` 0) is one line by the story kind (slot 9). The body reads the slots (0: the player, 1..3 his
/// names, 4: his club, 5: the subtype, 6: the outlet type, 9: the kind, 10: the reply, 11..14: a second person, 16..18:
/// the squad role, the age and the story bits, 23..26: the third person, the club manager or the human manager, 28:
/// with a match, then the fixture record and the result, or the qualities, the position, the pundit and a fourth
/// person) and writes, in pieces: the outlet's name (the pundit's when the media is local, else the outlet type, plain
/// for the local media or with the adjective of the club's nation), the story by kind (left out, ability, complacency,
/// contribution with the score, criticism by the bits of slot 18, commitment, indiscipline, purchase, the manager rows,
/// attitude, private life, transfer rumour, exclusive, too young, rumours, out of favour, past it, pressure, poor run;
/// the other kinds the programmers' placeholder), how a drawn match was eventually won or lost, then the reply
/// paragraph: the human manager's when he is the third person (support, criticise, ignore), otherwise the club
/// manager's; the text is "%s\n\n%s" of the two, with the links of the persons whose names match, the club's nation
/// (name, short name, ten adjectives), the club, the score and the other club.
///
/// Defects of the original kept: slot 14 is read twice and 15 never; the kind 0x10 foreign text passes the club's
/// gender byte where the club's name goes; the kind 1 story above subtype 6 writes nothing (the original then prints its
/// uninitialised buffer; the copy an empty text); the headline's names get a size (0x190) larger than their buffers;
/// the headline of kind 0x11 divides by subtype + 1 (a subtype of -1 divides by zero, in both).
/// Original: `Media::write_player_media_story_news_text` (0x72d270, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_player_media_story_news_text(_this: usize, news: usize, body: u32, out: usize) {
    PLAYER_STORY_TRAIL.with(|t| t.borrow_mut().clear());
    if body == 0 {
        return player_story_headline(news, out);
    }
    let mut st = PlayerStory::read(news);
    st.write_outlet();
    st.write_story();
    st.write_reply();
    st.finish(out);
}

/// The string and the links (text, type, data) of a Text object (`+0` the string, `+4` the link list; each link holds
/// its text at +0, the type at +0xc8 and the data at +0xcc).
unsafe fn nma3_text_contents(t: usize) -> (Vec<u8>, Vec<(Vec<u8>, u32, u32)>) {
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

/// Comparer for `write_player_media_story_news_text`: the original writes into `out`, the copy into a fresh local Text
/// side by side (the random generator's state, without counting); the strings and the links (text, type and data) must
/// be equal. The message names the last piece the copy ran (headline, outlet, the story kind, the draw result, the
/// reply, the links).
pub(crate) unsafe extern "thiscall" fn compare_write_player_media_story_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    let mut local = [0usize; 2];
    let lp = local.as_mut_ptr() as usize;
    let mut pieces = Vec::new();
    let (_, _, same_rng, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || o(this, news, body, out), || {
        crate::rng::without_counting(|| {
            write_player_media_story_news_text(this, news, body, lp);
            pieces = PLAYER_STORY_TRAIL.with(|t| t.borrow().clone());
        })
    });
    let (mo, mr) = (nma3_text_contents(out), nma3_text_contents(lp));
    exe::drop_text(local.as_mut_ptr());
    SCOREBOARD.record(same_rng && mo == mr, || {
        format!("write_player_media_story_news_text(body {}) pieces {:?}; original {:?} {:x?} rust {:?} {:x?}", body, pieces,
            String::from_utf8_lossy(&mo.0), mo.1.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>(),
            String::from_utf8_lossy(&mr.0), mr.1.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>())
    });
}
