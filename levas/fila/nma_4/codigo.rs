// ---------- batch %%LEVA%%: the player praise story (media.cpp) ----------

/// Where each text buffer of `write_player_praise_news_text` lives in its frame (the original's `esp` offsets after the
/// three register pushes; the copy keeps the same layout, so a long text overflows the same way).
#[allow(dead_code)]
mod praise_frame {
    pub(crate) const POSITION: usize = 0xe4; // the position name or player description (0xc8)
    pub(crate) const ADJECTIVE: usize = 0x1b8; // a nationality adjective (0x78)
    pub(crate) const PLAYER_FULL: usize = 0x230; // the player's name, style 1 (0x168)
    pub(crate) const LEGEND: usize = 0x398; // the great player's name (0x78)
    pub(crate) const PLAYER_SHORT: usize = 0x410; // the player's name, style 5 (0x168)
    pub(crate) const EXTRA: usize = 0x578; // the stage label of kind 0x34 (0xc8)
    pub(crate) const OUTLET: usize = 0x640; // the outlet or pundit name (0xc8)
    pub(crate) const REPLY: usize = 0x708; // the second paragraph: the manager's reply (0x3e8)
    pub(crate) const MANAGER_FULL: usize = 0xaf0; // the manager's name, style 1 (0x168)
    pub(crate) const DATE_TEXT: usize = 0xc58; // a date text (0x78)
    pub(crate) const HEAD: usize = 0xcd0; // the headline, or the body's first paragraph
    pub(crate) const SPLIT_EXT: usize = 0x2058; // the source file's extension, for the error box (0x100)
    pub(crate) const SPLIT_NAME: usize = 0x2158; // the source file's name, for the error box (0x100)
    pub(crate) const MANAGER_SHORT: usize = 0x2258; // the manager's name, style 5 (0x168)
    pub(crate) const SIZE: usize = 0x23c0;
    /// the three great players named when the story has none (by (variant + player) % 4)
    pub(crate) const LEGENDS: [usize; 4] = [0xa434f0, 0xa43500, 0xa43510, 0xa43518]; // Charlton, Maradona, Pelé, Cruyff
}

/// The news slots and the frame of `write_player_praise_news_text`. The slots after 17 are read from a moving index
/// (0x12, or after the match record when slot 17 is set); the names below are those of the use, [inference] where only
/// the use says so.
struct Praise {
    /// the frame's buffers (`praise_frame`), on the caller's stack
    base: usize,
    /// slot 0: the player (staff index); 1-3 his name ids
    player: i32,
    names: [u32; 3],
    /// slot 4: the player's club (club index); `club` is its record (club table + index × `club::SIZE`, unchecked)
    club_index: i32,
    club: usize,
    /// slot 5: the story kind (0x29..0x34)
    kind: i32,
    /// slot 7: a nation linked at the end (index)
    nation_a: i32,
    /// slot 8: the variant (picks the paragraph)
    variant: i32,
    /// slot 9: the manager who replies (staff index); 10-12 his name ids
    manager: i32,
    manager_names: [u32; 3],
    /// slot 13: the reply (1, 2, 3 no reply)
    response: i32,
    /// slot 14: the player's strength flags; slot 15: his position flags
    strengths: u32,
    position: u32,
    /// slot 16: a competition linked at the end (club comp index)
    comp: i32,
    /// slot 17: the story carries a match record (read by `read_match_record_from_news`)
    has_match: u32,
    /// the moving slots: a rival club, four values, the great player and his name ids, two nations, a value, the great
    /// player's job and club, the outlet type and pundit, the player's age [inference] and a flag
    rival_club: i32,
    m1: i32,
    m2: i32,
    m3: i32,
    m4: i32,
    legend: i32,
    legend_names: [u32; 3],
    nation_b: i32,
    nation_c: i32,
    m11: i32,
    legend_job: i32,
    legend_club: i32,
    outlet_type: i32,
    pundit: i32,
    age: i32,
    m17: i32,
    /// the human manager's (screen manager's) club or nation is the player's club's nation: "national" press, else
    /// "{nationality} national" press
    home: bool,
    /// the gender byte of the position text (the dword at frame +0x10 is what the original passes on)
    pos_gender: u32,
    /// the outlet's gender byte, the great player's job gender byte
    outlet_gender: u8,
    job_gender: u8,
    /// the match record read from the news (the original's frame +0x6c)
    record: [u8; 0x20],
    /// the name of the piece that wrote the text (for the comparer)
    piece: &'static str,
}

type PraiseFormat = unsafe extern "C" fn(usize, usize, ...) -> i32;

impl Praise {
    unsafe fn format() -> PraiseFormat {
        core::mem::transmute(FN_LANGLIB_SPRINTF)
    }
    fn at(&self, off: usize) -> usize {
        self.base + off
    }
    /// the club's short-name gender byte (signed) and short name
    unsafe fn cg(&self) -> i32 {
        read_i8(self.club + club::SHORT_GENDER_NAME) as i32
    }
    fn cname(&self) -> usize {
        self.club + club::SHORT_NAME
    }
    /// the club's long-name gender byte and long name
    unsafe fn lg(&self) -> i32 {
        read_i8(self.club + club::GENDER_NAME) as i32
    }
    fn lname(&self) -> usize {
        self.club + club::NAME
    }
    unsafe fn nation(&self) -> usize {
        read_usize(self.club + club::NATION)
    }
    /// the position text's gender byte, signed
    fn pg(&self) -> i32 {
        self.pos_gender as u8 as i8 as i32
    }
    /// `write_player_description(player, position, strengths, POSITION, 0xc8, gender, 1)` (0x5465a0, cdecl; the
    /// original pushes a 7th argument, 1, that the callee does not read): the description over the position text,
    /// returning its gender byte (signed)
    unsafe fn describe(&mut self) -> i32 {
        let f: unsafe extern "C" fn(i32, u32, u32, usize, i32, u32, u32) -> u8 = core::mem::transmute(0x5465a0usize);
        let out = self.at(praise_frame::POSITION);
        f(self.player, self.position, self.strengths, out, 0xc8, self.pos_gender, 1) as i8 as i32
    }
    /// the description, keeping its gender byte as the position gender (the original stores al at frame +0x10)
    unsafe fn describe_keep(&mut self) {
        let g = self.describe();
        self.pos_gender = (self.pos_gender & 0xffff_ff00) | (g as u8 as u32);
    }
    /// the nationality adjective of the club's nation in `form` into ADJECTIVE (0x78)
    unsafe fn adjective(&mut self, form: u32) {
        let out = self.at(praise_frame::ADJECTIVE);
        exe::nation_get_nationality_adjective(self.nation(), out, 0x78, form);
    }
    /// `database_get_job_name(job, 0, 0, &gender)` of the great player's job (0x53ad00, cdecl)
    unsafe fn legend_job_name(&mut self) -> usize {
        let f: unsafe extern "C" fn(i32, u32, u32, *mut u8) -> usize = core::mem::transmute(0x53ad00usize);
        f(self.legend_job, 0, 0, &mut self.job_gender)
    }
    fn jg(&self) -> i32 {
        self.job_gender as i8 as i32
    }
    /// the great player's club record and its short-name gender byte
    unsafe fn legend_club_rec(&self) -> usize {
        read_usize(G_CLUB_TABLE).wrapping_add((self.legend_club as usize).wrapping_mul(club::SIZE))
    }
    /// the great player's nation (staff `NATION` of his record)
    unsafe fn legend_nation(&self) -> usize {
        read_usize(read_usize(G_STAFF_TABLE).wrapping_add((self.legend as usize).wrapping_mul(staff::SIZE)) + staff::NATION)
    }
    /// the first paragraph (or the headline) and the reply
    fn head(&self) -> usize {
        self.at(praise_frame::HEAD)
    }
    fn reply(&self) -> usize {
        self.at(praise_frame::REPLY)
    }
    fn pos(&self) -> usize {
        self.at(praise_frame::POSITION)
    }
    fn full(&self) -> usize {
        self.at(praise_frame::PLAYER_FULL)
    }
    fn short(&self) -> usize {
        self.at(praise_frame::PLAYER_SHORT)
    }
    fn adj(&self) -> usize {
        self.at(praise_frame::ADJECTIVE)
    }
    fn lname_buf(&self) -> usize {
        self.at(praise_frame::LEGEND)
    }
}

/// `Media::write_player_praise_news_text(news, body, out)`: writes the headline or the body of a media story that
/// praises a player, naming the outlet or pundit, a great player, the player and the manager's reply, then links every
/// name in the text.
///
/// Reads the news slots (`exe::newsitem_get_slot`): 0 the player (staff index) and 1-3 his name ids, 4 his club, 5 the
/// kind (0x29..0x34), 7 a nation, 8 the variant, 9 the manager and 10-12 his name ids, 13 the reply, 14 the strength
/// flags, 15 the position flags, 16 a competition, 17 "with a match record"; then, from slot 0x12 or after the match
/// record (`read_match_record_from_news` 0x7e4fa0, which moves the index), a rival club, four values, the great player
/// and his name ids, two nations, a value, the great player's job and club (−1 out of range), the outlet type and
/// pundit, the player's age [inference] and a flag. With a human manager on screen (`G_SCREEN_RECORDS_MANAGER` of
/// `G_SCREEN_MANAGER_CURRENT_SLOT`), the rival club is dropped when values 1 and 4 are 0, the player % 4 is not 0 and
/// the rival is of the manager's nation (his club job's nation, or his own); `home` is set when the player's club is
/// of that nation. The position text (`write_position_name` of the position flags, 0xc8) gives the gender; when the
/// manager is the player himself it reads "player-manager" (gender 0).
///
/// The outlet (`praise_outlet`): the pundit's name (`PunditList::get_pundit_name` of `G_PUNDIT_LIST`) at home with a
/// nation, an outlet type and a pundit; otherwise by type 1..6 and other: national or local newspaper, national or
/// local radio station, magazine, sports paper, website ("{nationality} ..." abroad, `nation_get_nationality_adjective`
/// forms 0x16..0x1a; the magazine's gender is 1 with language 8, the others 0). The great player: his name from the ids,
/// or Bobby Charlton, Diego Maradona, Pelé or Johan Cruyff by (variant + player) % 4. The names of the player and the
/// manager in styles 1 and 5 (`database_format_name_from_ids`, 0x168).
///
/// Body: the first paragraph by kind (one piece each, `praise_body_*`), then the reply (`praise_reply`), joined as
/// "%s\n\n%s" in the text buffer 0xde1f64. Headline (`praise_headline`): "Praise for {club} {position}" when variant % 8
/// is player % 7, otherwise by kind. The text goes to `out` (`exe::text_set_string`) and the links follow
/// (`praise_links`): the match (0x7d3) and its club, the player, the manager and the great player when their ids match
/// (`database_staff_matches_names`, 0x7d7), the great player's club, the rival club and the player's club (0x7d5), the
/// club's nation and its national team under ten adjective forms (0x16..0x1f), three nations and the competition
/// (0x7d0).
///
/// Defect of the original: the match link's text is the frame byte +0x4c, zeroed only in the headline; in the body it is
/// whatever the stack held (the copy writes an empty text in both, and the comparer does not compare that text).
/// Original: `Media::write_player_praise_news_text` (0x744f80, media.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn write_player_praise_news_text(_this: usize, news: usize, body: u32, out: usize) {
    let mut frame = [0u8; praise_frame::SIZE];
    let mut p = praise_read_slots(news, frame.as_mut_ptr() as usize);
    praise_setup(&mut p);
    praise_outlet(&mut p);
    praise_names(&mut p);
    let text = if body != 0 {
        covered!("nma_4 praise body");
        praise_body(&mut p);
        praise_reply(&mut p);
        write_u8(PRAISE_TEXT_BUFFER, 0);
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        sprintf(PRAISE_TEXT_BUFFER, 0x991948, p.head(), p.reply()); // "%s\n\n%s"
        PRAISE_TEXT_BUFFER
    } else {
        praise_headline(&mut p);
        p.head()
    };
    exe::text_set_string(out, text as *const u8);
    praise_links(&mut p, out);
    PRAISE_LAST_PIECE = p.piece;
}

/// The text buffer the body is joined in (`g_text_buffer`).
const PRAISE_TEXT_BUFFER: usize = 0xde1f64;

/// The piece of the last text written (for the comparer's message).
static mut PRAISE_LAST_PIECE: &str = "";

/// Reads the news slots in the original's order (the match record moves the index of the slots after 17).
unsafe fn praise_read_slots(news: usize, base: usize) -> Praise {
    let s = |k: u32| exe::newsitem_get_slot(news, k);
    let player = s(0) as i32;
    let names = [s(1), s(2), s(3)];
    let club_index = s(4) as i32;
    let kind = s(5) as i32;
    s(6);
    let nation_a = s(7) as i32;
    let variant = s(8) as i32;
    let manager = s(9) as i32;
    let manager_names = [s(10), s(11), s(12)];
    let response = s(13) as i32;
    let strengths = s(14);
    let position = s(15);
    let comp = s(16) as i32;
    let has_match = s(17);
    let mut record = [0u8; 0x20];
    exe::date_date_54a110(record.as_mut_ptr() as usize);
    let mut k: u8 = 0x12;
    if has_match != 0 {
        let f: unsafe extern "C" fn(usize, *mut u8, *mut u8) -> i32 = core::mem::transmute(0x7e4fa0usize);
        f(news, &mut k, record.as_mut_ptr());
    }
    let mut next = || {
        let v = exe::newsitem_get_slot(news, k as u32);
        k = k.wrapping_add(1);
        v
    };
    let rival_club = next() as i32;
    let (m1, m2, m3, m4) = (next() as i32, next() as i32, next() as i32, next() as i32);
    let legend = next() as i32;
    let legend_names = [next(), next(), next()];
    let (nation_b, nation_c, m11) = (next() as i32, next() as i32, next() as i32);
    let legend_job = next() as i32;
    let mut legend_club = next() as i32;
    let (outlet_type, pundit, age, m17) = (next() as i32, next() as i32, next() as i32, next() as i32);
    if legend_club < 0 || legend_club >= read_i32(G_CLUB_COUNT) {
        legend_club = -1;
    }
    let club = read_usize(G_CLUB_TABLE).wrapping_add((club_index as usize).wrapping_mul(club::SIZE));
    Praise {
        base, player, names, club_index, club, kind, nation_a, variant, manager, manager_names, response, strengths,
        position, comp, has_match, rival_club, m1, m2, m3, m4, legend, legend_names, nation_b, nation_c, m11,
        legend_job, legend_club, outlet_type, pundit, age, m17, home: false, pos_gender: 0, outlet_gender: 0,
        job_gender: 0, record, piece: "",
    }
}

/// The C remainder of `a % b` (signed, the sign of `a`), as the original's `idiv` and `and 0x80000003` fix-ups give.
fn praise_rem(a: i32, b: i32) -> i32 {
    a.wrapping_rem(b)
}

/// The human manager's nation test (drops the rival club, sets `home`) and the position text.
unsafe fn praise_setup(p: &mut Praise) {
    let human = read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300)));
    if human != 0 {
        let club_nation = |c: i32| read_usize(read_usize(G_CLUB_TABLE).wrapping_add((c as usize).wrapping_mul(club::SIZE)) + club::NATION);
        let job = read_usize(human + staff::CLUB_JOB);
        let mine = if job != 0 { read_usize(job + club::NATION) } else { read_usize(human + staff::NATION) };
        if p.m1 == 0 && p.m4 == 0 && p.rival_club > -1 && praise_rem(p.player, 4) != 0 && club_nation(p.rival_club) == mine {
            covered!("nma_4 praise rival dropped");
            p.rival_club = -1;
        }
        if club_nation(p.club_index) == mine {
            p.home = true;
        }
    }
    let f: unsafe extern "C" fn(u32, usize, i32, u32) -> u8 = core::mem::transmute(0x5461b0usize);
    p.pos_gender = f(p.position, p.pos(), 0xc8, 1) as u32;
    let staff_of = |i: i32| read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE));
    if staff_of(p.manager) == staff_of(p.player) {
        Praise::format()(p.pos(), 0xa242e0); // "player-manager"
        p.pos_gender &= 0xffff_ff00;
    }
}

/// The outlet's name and gender byte (see the routine's header).
unsafe fn praise_outlet(p: &mut Praise) {
    let f = Praise::format();
    let out = p.at(praise_frame::OUTLET);
    let nation = p.nation();
    if nation != 0 && p.outlet_type != -1 && p.pundit != -1 && p.home {
        covered!("nma_4 praise pundit");
        let g: unsafe extern "thiscall" fn(usize, u32, u32, usize, *mut u8) = core::mem::transmute(0x7dad20usize);
        g(read_usize(G_PUNDIT_LIST), p.outlet_type as u32, p.pundit as u32, out, &mut p.outlet_gender);
        return;
    }
    covered!("nma_4 praise outlet type");
    // by type: (home text, form, abroad text)
    let (home, form, abroad) = match p.outlet_type.wrapping_sub(1) {
        0 => (0xa242cc, 0x16, 0xa24294), // national newspaper
        1 => (0xa24284, 0x17, 0xa24250), // local newspaper
        2 => (0xa24238, 0x18, 0xa24208), // national radio station
        3 => (0xa241f4, 0x18, 0xa241bc), // local radio station
        4 => (0xa241b0, 0x19, 0xa24184), // magazine
        5 => (0xa24174, 0x1a, 0xa24144), // sports paper
        _ => (0xa2413c, 0x1a, 0xa24110), // website
    };
    if p.home {
        f(out, home);
    } else {
        exe::nation_get_nationality_adjective(nation, p.adj(), 0x78, form);
        f(out, abroad, 0x64, p.adj());
    }
    p.outlet_gender = if p.outlet_type.wrapping_sub(1) == 4 { (exe::langlib_get_language() == 8) as u8 } else { 0 };
}

/// The great player's name and the names of the player and the manager.
unsafe fn praise_names(p: &mut Praise) {
    let ids = |b: usize, n: [u32; 3], size: u32, style: u32, who: i32| {
        exe::database_format_name_from_ids(n[0], n[1], n[2], b as *mut u8, size, style, who as u32);
    };
    if p.legend > -1 {
        ids(p.lname_buf(), p.legend_names, 0x78, 1, p.legend);
    } else {
        let k = praise_rem(p.variant.wrapping_add(p.player), 4);
        let name = praise_frame::LEGENDS[if (0..3).contains(&k) { k as usize } else { 3 }];
        let n = (0..).take_while(|&i| read_u8(name + i) != 0).count() + 1;
        core::ptr::copy_nonoverlapping(name as *const u8, p.lname_buf() as *mut u8, n); // strcpy
    }
    ids(p.full(), p.names, 0x168, 1, p.player);
    ids(p.short(), p.names, 0x168, 5, p.player);
    ids(p.at(praise_frame::MANAGER_FULL), p.manager_names, 0x168, 1, p.manager);
    ids(p.at(praise_frame::MANAGER_SHORT), p.manager_names, 0x168, 5, p.manager);
}

/// The first paragraph of the body, by kind.
unsafe fn praise_body(p: &mut Praise) {
    match p.kind.wrapping_sub(0x29) {
        0 => praise_body_future_international(p),
        1 => praise_body_youngster(p),
        2 => praise_body_praised(p),
        4 => praise_body_on_the_rise(p),
        5 => praise_body_big_move(p),
        6 => praise_body_attitude(p),
        7 => praise_body_world_class(p),
        8 => praise_body_world_class_b(p),
        9 => praise_body_impresses(p),
        10 => praise_body_world_stage(p),
        11 => praise_body_competition(p),
        _ => praise_body_call_up(p),
    }
}

/// Kind 0x2e, "Big move close for {player}?": by variant, a pundit praising the club's player, the press speculating
/// about a move to a bigger club, a national newspaper calling him a big fish in a small pond, a club touting him, or a
/// television pundit heaping praise ("{nationality} ..." abroad).
/// Original: part of `Media::write_player_praise_news_text` (0x745855..0x745ba9).
unsafe fn praise_body_big_move(p: &mut Praise) {
    covered!("nma_4 praise big move");
    p.piece = "body_big_move";
    let f = Praise::format();
    let (h, pos, full, short, adj, cname, cg) = (p.head(), p.pos(), p.full(), p.short(), p.adj(), p.cname(), p.cg());
    match p.variant {
        0 => {
            f(h, 0xa433f8, cg, cg, p.pg(), p.pg(), cname, pos, full);
        }
        1 => {
            if p.home {
                let g = p.describe();
                f(h, 0xa432e4, 0x64, cg, cg, g, g, full, cname, pos);
            } else {
                p.describe_keep();
                p.adjective(0x1c);
                f(h, 0xa431b0, 0x64, 0x64, 0x64, cg, cg, p.pg(), p.pg(), full, adj, cname, pos);
            }
        }
        2 => {
            if p.home {
                let g = p.describe();
                f(h, 0xa430a4, g, g, cg, cg, pos, full, cname);
            } else {
                p.adjective(0x16);
                let g = p.describe();
                f(h, 0xa42f78, 0x64, g, g, cg, cg, adj, pos, full, cname);
            }
        }
        3 => {
            let g = p.describe();
            f(h, 0xa42e7c, cg, cg, g, g, cname, full, pos);
        }
        _ => {
            if p.home {
                let g = p.describe();
                f(h, 0xa42d4c, cg, cg, 0x64, 0x64, g, g, cname, full, short, pos);
            } else {
                p.adjective(0x1f);
                let g = p.describe();
                f(h, 0xa42c00, 0x64, cg, cg, 0x64, g, g, adj, cname, full, short, pos);
            }
        }
    }
}

/// The great player's lines of kinds 0x30 and 0x31: with his club, (club gender, job gender, his club's gender,
/// his club's name, his job, his name); returns None without a club.
unsafe fn praise_legend_club(p: &mut Praise) -> Option<(i32, usize, usize)> {
    if p.legend_club <= -1 {
        return None;
    }
    let job = p.legend_job_name();
    let lc = p.legend_club_rec();
    Some((read_i8(lc + club::SHORT_GENDER_NAME) as i32, lc + club::SHORT_NAME, job))
}

/// Kind 0x30, "{player} world class talent": by variant, a former great (with his club and job, or his nation) or a
/// world cup winning manager saying the player will be a world star; variant 1 the same about his form this season;
/// the others, a household name throughout the world.
/// Original: part of `Media::write_player_praise_news_text` (0x745ba9..0x745ed9).
unsafe fn praise_body_world_class(p: &mut Praise) {
    covered!("nma_4 praise world class");
    p.piece = "body_world_class";
    let f = Praise::format();
    let (h, pos, full, cname, cg, legend) = (p.head(), p.pos(), p.full(), p.cname(), p.cg(), p.lname_buf());
    let sum = p.legend.wrapping_add(p.club_index).wrapping_add(p.player);
    match p.variant {
        0 => {
            p.describe_keep();
            let pg = p.pg();
            if p.legend > -1 {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    f(h, 0xa426ac, lcg, lcg, jg, jg, cg, cg, pg, pg, lcname, job, legend, cname, pos, full);
                    return;
                }
                if praise_rem(sum, 3) != 0 {
                    let n = p.legend_nation();
                    let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                    f(h, 0xa42598, ng, ng, cg, cg, pg, pg, n + nation::SHORT_NAME, legend, cname, pos, full);
                    return;
                }
            }
            f(h, 0xa424b4, cg, cg, pg, pg, cname, pos, full);
        }
        1 => {
            let pg = p.pg();
            if p.legend > -1 {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    f(h, 0xa429cc, cg, cg, pg, pg, 0x64, lcg, lcg, jg, jg, cname, pos, full, lcname, job, legend);
                    return;
                }
                if praise_rem(sum, 13) > 7 {
                    let n = p.legend_nation();
                    let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                    f(h, 0xa428a8, cg, cg, pg, pg, 0x64, 0x64, ng, ng, 0x64, 0x64, cname, pos, full, n + nation::SHORT_NAME, legend);
                    return;
                }
            }
            f(h, 0xa427c4, cg, cg, pg, pg, 0x64, cname, pos, full);
        }
        _ => {
            let g = p.describe();
            f(h, 0xa42af4, cg, cg, g, g, cname, full, pos);
        }
    }
}

/// Kind 0x31, "{player} world class talent" (the second set): by variant, a former great or a world cup winning
/// manager saying the player is world class (with % 11 < 6 for the great's nation), or the player named among the
/// best in a world poll.
/// Original: part of `Media::write_player_praise_news_text` (0x745ed9..0x7461de).
unsafe fn praise_body_world_class_b(p: &mut Praise) {
    covered!("nma_4 praise world class b");
    p.piece = "body_world_class_b";
    let f = Praise::format();
    let (h, pos, full, short, cname, cg, legend) = (p.head(), p.pos(), p.full(), p.short(), p.cname(), p.cg(), p.lname_buf());
    let sum = p.legend.wrapping_add(p.club_index).wrapping_add(p.player);
    match p.variant {
        0 => {
            p.describe_keep();
            let pg = p.pg();
            if p.legend > -1 {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    f(h, 0xa41f78, lcg, lcg, jg, jg, cg, cg, pg, pg, lcname, job, legend, cname, pos, full);
                    return;
                }
                if praise_rem(sum, 11) < 6 {
                    let n = p.legend_nation();
                    let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                    f(h, 0xa41e78, ng, ng, cg, cg, pg, pg, n + nation::SHORT_NAME, legend, cname, pos, full);
                    return;
                }
            }
            f(h, 0xa41d94, cg, cg, pg, pg, cname, pos, full);
        }
        1 => {
            let pg = p.pg();
            if p.legend > -1 {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    f(h, 0xa42268, cg, cg, pg, pg, 0x64, lcg, lcg, jg, jg, cname, pos, full, lcname, job, legend);
                    return;
                }
                if praise_rem(sum, 11) < 6 {
                    let n = p.legend_nation();
                    let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                    f(h, 0xa42158, cg, cg, pg, pg, 0x64, ng, ng, cname, pos, full, n + nation::SHORT_NAME, legend);
                    return;
                }
            }
            f(h, 0xa42084, cg, cg, pg, pg, 0x64, cname, pos, full);
        }
        _ => {
            f(h, 0xa42384, cg, cg, 0x64, cname, full, short);
        }
    }
}

/// Kind 0x2d, "{player} on the rise": a media pundit's belief (variant 0), a television pundit's (variant 1), or a
/// national paper's report on his strengths (the others), "{nationality} ..." abroad.
/// Original: part of `Media::write_player_praise_news_text` (0x7461de..0x746414).
unsafe fn praise_body_on_the_rise(p: &mut Praise) {
    covered!("nma_4 praise on the rise");
    p.piece = "body_on_the_rise";
    let f = Praise::format();
    let (h, pos, full, short, adj, cname, cg) = (p.head(), p.pos(), p.full(), p.short(), p.adj(), p.cname(), p.cg());
    let (lname, lg) = (p.lname(), p.lg());
    match p.variant {
        0 => {
            let g = p.describe();
            f(h, 0xa417cc, cg, cg, g, g, cname, pos, full);
        }
        1 => {
            if p.home {
                let g = p.describe();
                f(h, 0xa41c6c, cg, cg, g, g, cname, full, short, pos);
            } else {
                p.adjective(0x1c);
                let g = p.describe();
                f(h, 0xa41b28, cg, cg, g, g, 0x64, 0x64, cname, full, short, pos, adj);
            }
        }
        _ => {
            if p.home {
                let g = p.describe();
                f(h, 0xa41a04, g, g, 0x64, lg, lg, pos, full, lname);
            } else {
                p.adjective(0x16);
                let g = p.describe();
                f(h, 0xa418c0, 0x64, g, g, 0x64, lg, lg, adj, pos, full, lname);
            }
        }
    }
}

/// The nation of slot 7 (nation table + index × `nation::SIZE`), its name gender byte and name.
unsafe fn praise_nation_a(p: &Praise) -> (usize, i32) {
    let n = read_usize(G_NATION_TABLE).wrapping_add((p.nation_a as usize).wrapping_mul(nation::SIZE));
    (n + 4, read_i8(n + nation::GENDER_NAME) as i32)
}

/// Kind 0x29, "{player} rated highly / a 'future international' / set for call-up?": by variant, an outlet praising
/// him for the national team (variant 0), the club's player of the nation (1), a former great of that nation joining
/// the experts who want him called up (2), or a first cap in the offing (the others).
/// Original: part of `Media::write_player_praise_news_text` (0x746414..0x746851).
unsafe fn praise_body_future_international(p: &mut Praise) {
    covered!("nma_4 praise future international");
    p.piece = "body_future_international";
    let f = Praise::format();
    let (h, pos, full, adj, cname, cg, legend) = (p.head(), p.pos(), p.full(), p.adj(), p.cname(), p.cg(), p.lname_buf());
    let outlet = p.at(praise_frame::OUTLET);
    match p.variant {
        0 => {
            let g = p.describe();
            let og = p.outlet_gender as i8 as i32;
            if p.home {
                let (na, nag) = praise_nation_a(p);
                f(h, 0xa40f08, og, og, cg, cg, g, g, 0x64, nag, nag, outlet, cname, pos, full, na);
            } else {
                f(h, 0xa40df0, og, og, cg, cg, g, g, 0x64, outlet, cname, pos, full);
            }
        }
        1 => {
            let pg = p.pg();
            if p.home {
                let (na, nag) = praise_nation_a(p);
                f(h, 0xa4115c, cg, cg, pg, pg, nag, nag, cname, pos, full, na);
            } else {
                p.adjective(0x1c);
                let (na, nag) = praise_nation_a(p);
                f(h, 0xa41044, cg, cg, pg, pg, 0x64, 0x64, nag, nag, cname, pos, full, adj, na);
            }
        }
        2 => {
            let pg = p.pg();
            if p.legend > -1 && read_i32(p.legend_nation()) == p.nation_a {
                if p.legend_club > -1 {
                    let job = p.legend_job_name();
                    let (na, nag) = praise_nation_a(p);
                    let lc = p.legend_club_rec();
                    let lcg = read_i8(lc + club::SHORT_GENDER_NAME) as i32;
                    let jg = p.jg();
                    f(h, 0xa41444, lcg, lcg, jg, jg, cg, cg, pg, pg, nag, nag, lc + club::SHORT_NAME, job, legend, cname, pos, full, na);
                } else {
                    let (na, nag) = praise_nation_a(p);
                    f(h, 0xa41334, cg, cg, pg, pg, nag, nag, legend, cname, pos, full, na);
                }
                return;
            }
            let (na, nag) = praise_nation_a(p);
            f(h, 0xa41254, cg, cg, pg, pg, nag, nag, cname, pos, full, na);
        }
        _ => {
            let g = p.describe();
            if p.home {
                f(h, 0xa416b0, cg, cg, g, g, cname, pos, full);
            } else {
                let n = p.nation();
                let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                f(h, 0xa4157c, cg, cg, g, g, ng, ng, cname, pos, full, n + nation::SHORT_NAME);
            }
        }
    }
}

/// Kind 0x2a, "{player} shows promise / talented {player} given boost / youngster {player} praised": by variant, an
/// outlet praising the youngster (0), the club's young player (1, "{nationality}" abroad), an exciting future (2), or a
/// player type with a future (the others), with the club's nation abroad.
/// Original: part of `Media::write_player_praise_news_text` (0x746851..0x746ac4).
unsafe fn praise_body_youngster(p: &mut Praise) {
    covered!("nma_4 praise youngster");
    p.piece = "body_youngster";
    let f = Praise::format();
    let (h, pos, full, adj, cname, cg) = (p.head(), p.pos(), p.full(), p.adj(), p.cname(), p.cg());
    let outlet = p.at(praise_frame::OUTLET);
    match p.variant {
        0 => {
            let og = p.outlet_gender as i8 as i32;
            f(h, 0xa406d0, og, og, cg, cg, p.pg(), p.pg(), 0x64, outlet, cname, pos, full);
        }
        1 => {
            if p.home {
                f(h, 0xa408c0, cg, cg, p.pg(), p.pg(), cname, pos, full);
            } else {
                p.adjective(0x1c);
                f(h, 0xa407e4, cg, cg, p.pg(), p.pg(), 0x64, 0x64, cname, pos, full, adj);
            }
        }
        2 => {
            let g = p.describe();
            if p.home {
                f(h, 0xa40aa0, cg, cg, g, g, cname, pos, full);
            } else {
                let n = p.nation();
                let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                f(h, 0xa40978, cg, cg, g, g, ng, ng, cname, pos, full, n + nation::SHORT_NAME);
            }
        }
        _ => {
            let g = p.describe();
            if p.home {
                f(h, 0xa40cdc, g, g, cg, cg, pos, full, cname);
            } else {
                let n = p.nation();
                let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                f(h, 0xa40bb0, g, g, ng, ng, cg, cg, pos, full, n + nation::SHORT_NAME, cname);
            }
        }
    }
}

/// Kind 0x32, "{player} impresses at {club}": an outlet praising his impact (variant 0), an outlet on his type
/// (1), or his performances since joining (the others).
/// Original: part of `Media::write_player_praise_news_text` (0x746ac4..0x746bd1).
unsafe fn praise_body_impresses(p: &mut Praise) {
    covered!("nma_4 praise impresses");
    p.piece = "body_impresses";
    let f = Praise::format();
    let (h, pos, full, short, cname, cg) = (p.head(), p.pos(), p.full(), p.short(), p.cname(), p.cg());
    let outlet = p.at(praise_frame::OUTLET);
    let og = p.outlet_gender as i8 as i32;
    match p.variant {
        0 => {
            f(h, 0xa40608, og, og, cg, cg, outlet, cname, full);
        }
        1 => {
            let g = p.describe();
            f(h, 0xa40530, og, og, g, g, cg, cg, outlet, pos, full, cname);
        }
        _ => {
            let g = p.describe();
            f(h, 0xa403ec, cg, cg, g, g, full, cname, pos, short);
        }
    }
}

/// strcat(dst, src).
unsafe fn praise_strcat(dst: usize, src: usize) {
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

/// strcmp(a, b) == 0.
unsafe fn praise_same_text(a: usize, b: usize) -> bool {
    let mut k = 0;
    loop {
        let (x, y) = (read_u8(a + k), read_u8(b + k));
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
        k += 1;
    }
}

/// The match of kind 0x34: the record's club (+0x15), its long-name gender and name, the stage gender (frame +0xe),
/// the score text (frame +0x4c) and the stage label (EXTRA).
struct PraiseMatch {
    rg: i32,
    rname: usize,
    sg: i32,
    score: usize,
    stage: usize,
}

/// The rival club (slot "rival club") of kind 0x34: its short-name gender and name, its nation's gender and name.
struct PraiseRival {
    rgs: i32,
    name: usize,
    rng: i32,
    nation_name: usize,
}

unsafe fn praise_rival(p: &Praise) -> PraiseRival {
    let r = read_usize(G_CLUB_TABLE).wrapping_add((p.rival_club as usize).wrapping_mul(club::SIZE));
    let n = read_usize(r + club::NATION);
    PraiseRival {
        rgs: read_i8(r + club::SHORT_GENDER_NAME) as i32,
        name: r + club::SHORT_NAME,
        rng: read_i8(n + nation::SHORT_GENDER_NAME) as i32,
        nation_name: n + nation::SHORT_NAME,
    }
}

/// The date of values 2 and 3 as a long date without the year (`Date::get_long_date_string_no_year`, 0x78, no time of
/// day) into the frame at +0xc58.
unsafe fn praise_date_text(p: &Praise) -> usize {
    let mut date = [0u16; 4];
    exe::new_date(date.as_mut_ptr() as usize, p.m2 as u32, p.m3 as u32);
    let g: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x54bef0usize);
    let out = p.at(praise_frame::DATE_TEXT);
    g(date.as_mut_ptr() as usize, out, 0x78, 0xff);
    out
}

/// Kind 0x34, "{player} lights up / stars in {competition}": the player's part in a match of a competition (the match
/// record of slot 17): the score (higher first, "%d-%d") and the stage label (`comp_util_format_comp_stage_round_label`,
/// "Friendly" and "Reserve" with a small first letter; gender `comp_util_get_label_gender`). By variant and whether the
/// club won (record +0x19 above +0x1a): a paragraph on the match, then, with a rival club, a second one (the player due
/// to sign for it on a date when value 1 is set; attracting interest while playing in a nation when value 4 is set;
/// playing for it otherwise); after a draw or a defeat, value 11 adds how it ended (won or lost on penalties or a
/// shootout). Without the match record: the original's error box (media.cpp line 0x4743) and an empty paragraph.
/// Original: part of `Media::write_player_praise_news_text` (0x746bd1..0x7483d2).
unsafe fn praise_body_competition(p: &mut Praise) {
    covered!("nma_4 praise competition");
    p.piece = "body_competition";
    let f = Praise::format();
    let h = p.head();
    if p.has_match == 0 {
        covered!("nma_4 praise competition without match");
        f(h, 0xad9c64); // ""
        praise_error_box(p, 0x4743);
        return;
    }
    p.describe_keep();
    let rec = p.record.as_ptr() as usize;
    let (a, b) = (read_i8(rec + 0x19), read_i8(rec + 0x1a));
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let score = p.at(0x4c);
    sprintf(score, 0x98c114, a.max(b) as i32, a.min(b) as i32); // "%d-%d"
    let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_u32(rec + 8) as usize).wrapping_mul(0x6b));
    let stage = p.at(praise_frame::EXTRA);
    exe::comp_util_format_comp_stage_round_label(comp, read_u32(rec + 0xc), read_u32(rec + 0xe), 1, stage as *mut u8, 0xc8);
    if praise_same_text(stage, 0x9db958) {
        write_u8(stage, b'f'); // "Friendly"
    } else if praise_same_text(stage, 0x9b5b40) {
        write_u8(stage, b'r'); // "Reserve"
    }
    p.job_gender = exe::comp_util_get_label_gender(comp as u32, read_u32(rec + 0xc), read_u32(rec + 0xe), 1);
    let rc = read_usize(G_CLUB_TABLE).wrapping_add((read_u32(rec + 0x15) as usize).wrapping_mul(club::SIZE));
    let m = PraiseMatch { rg: read_i8(rc + club::GENDER_NAME) as i32, rname: rc + club::NAME, sg: p.jg(), score, stage };
    let won = a > b;
    let ended = match p.variant {
        0 => praise_competition_v0(p, &m, won),
        1 => praise_competition_v1(p, &m, won),
        2 => praise_competition_v2(p, &m, won),
        _ => praise_competition_other(p, &m, won),
    };
    if ended {
        return;
    }
    // after a draw or a defeat: how it ended
    let tail = match p.m11 {
        1 => 0xa21e0c, // won after a penalty shootout
        2 => 0xa21684, // lost on penalties
        3 => 0xa21dc0, // won after a shootout
        4 => 0xa21d74, // lost on a shootout
        _ => return,
    };
    covered!("nma_4 praise competition shootout");
    let (cg, cname, r) = (p.cg(), p.cname(), p.reply());
    f(r, tail, cg, cg, cname);
    praise_strcat(h, r);
}

/// The original's assertion box: the file and line (`crt` splitpath of the media.cpp path), the version, "Error"
/// (`gui_show_message_box`), and the mouse button released.
unsafe fn praise_error_box(p: &Praise, line: u32) {
    let (name, ext) = (p.at(praise_frame::SPLIT_NAME), p.at(praise_frame::SPLIT_EXT));
    exe::crt_strtok(0xa17bec, 0, 0, name as *mut u8, ext as *mut u8);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    sprintf(0xae24d0, 0x9870e8, exe::utils_get_version_string(), name, ext, line); // "v%s %s.%s %d"
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0); // "Error"
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// Variant 0 of kind 0x34 (the player shone in the match). Returns true when the paragraph is complete (a win).
unsafe fn praise_competition_v0(p: &mut Praise, m: &PraiseMatch, won: bool) -> bool {
    let f = Praise::format();
    let (h, r, pos, full, short, cname, cg, pg) = (p.head(), p.reply(), p.pos(), p.full(), p.short(), p.cname(), p.cg(), p.pg());
    if p.rival_club > -1 {
        let rv = praise_rival(p);
        let (first, second) = match (won, p.m1 != 0, p.m4 != 0) {
            (true, true, _) => (0xa3eb20, 0xa3ea64),
            (true, false, true) => (0xa3eb20, 0xa3e980),
            (true, false, false) => (0xa3eb20, 0xa3e8c8),
            (false, true, _) => (0xa3e6b4, 0xa3e5f4),
            (false, false, true) => (0xa3e6b4, 0xa3e54c),
            (false, false, false) => (0xa3e6b4, 0xa3e8c8),
        };
        f(h, first, cg, cg, m.sg, m.sg, m.rg, m.rg, full, cname, m.score, m.stage, m.rname);
        if second == 0xa3e54c {
            f(r, second, pg, pg, rv.rgs, rv.rgs, pos, rv.name);
        } else {
            f(r, second, pg, pg, rv.rgs, rv.rgs, pos, short, rv.name);
        }
        praise_strcat(h, r);
    } else {
        let t = if won { 0xa3e77c } else { 0xa3e400 };
        f(h, t, cg, cg, m.sg, m.sg, m.rg, m.rg, pg, pg, full, cname, m.score, m.stage, m.rname, pos, short);
    }
    won
}

/// Variant 1 of kind 0x34 (the player starred in the match). Returns true after a win.
unsafe fn praise_competition_v1(p: &mut Praise, m: &PraiseMatch, won: bool) -> bool {
    let f = Praise::format();
    let (h, pos, full, short, cname, cg, pg) = (p.head(), p.pos(), p.full(), p.short(), p.cname(), p.cg(), p.pg());
    if p.rival_club > -1 {
        let rv = praise_rival(p);
        if won && p.m1 == 0 && p.m4 != 0 {
            f(h, 0xa3f420, 0x64, cg, cg, m.sg, m.sg, m.rg, m.rg, rv.rgs, rv.rgs, 0x64, full, cname, m.score, m.stage, m.rname, rv.name, short);
        } else if !won && p.m1 == 0 && p.m4 != 0 {
            f(h, 0xa3ee64, 0x64, cg, cg, m.sg, m.sg, m.rg, m.rg, rv.rgs, rv.rgs, pg, pg, full, cname, m.score, m.stage, m.rname, rv.name, pos);
        } else {
            let t = match (won, p.m1 != 0) {
                (true, true) => 0xa3f5f8,
                (true, false) => 0xa3f2d0,
                (false, true) => 0xa3f050,
                (false, false) => 0xa3ed14,
            };
            f(h, t, 0x64, cg, cg, m.sg, m.sg, m.rg, m.rg, rv.rgs, rv.rgs, full, cname, m.score, m.stage, m.rname, rv.name);
        }
    } else {
        let t = if won { 0xa3f1a4 } else { 0xa3ebe4 };
        f(h, t, 0x64, cg, cg, m.sg, m.sg, m.rg, m.rg, full, cname, m.score, m.stage, m.rname);
    }
    won
}

/// Variant 2 of kind 0x34 (after helping the club to the score). Returns true after a win.
unsafe fn praise_competition_v2(p: &mut Praise, m: &PraiseMatch, won: bool) -> bool {
    let f = Praise::format();
    let (h, full, short, cname, cg) = (p.head(), p.full(), p.short(), p.cname(), p.cg());
    if p.rival_club > -1 {
        let rv = praise_rival(p);
        if p.m1 != 0 {
            let t = if won { 0xa3fd7c } else { 0xa3f9a8 };
            f(h, t, cg, cg, m.sg, m.sg, m.rg, m.rg, 0x64, 0x64, rv.rgs, rv.rgs, cname, m.score, m.stage, m.rname, full, short, rv.name);
        } else {
            let t = if won { 0xa3fc3c } else { 0xa3f868 };
            f(h, t, cg, cg, m.sg, m.sg, m.rg, m.rg, 0x64, rv.rgs, rv.rgs, cname, m.score, m.stage, m.rname, rv.name, full);
        }
    } else {
        let t = if won { 0xa3fb20 } else { 0xa3f74c };
        f(h, t, cg, cg, m.sg, m.sg, m.rg, m.rg, 0x64, cname, m.score, m.stage, m.rname, full);
    }
    won
}

/// The other variants of kind 0x34 (after the club saw off the challenge / played out the score). Returns true after
/// a win.
unsafe fn praise_competition_other(p: &mut Praise, m: &PraiseMatch, won: bool) -> bool {
    let f = Praise::format();
    let (h, r, pos, full, short, cname, cg, pg, legend) =
        (p.head(), p.reply(), p.pos(), p.full(), p.short(), p.cname(), p.cg(), p.pg(), p.lname_buf());
    let first = |h: usize| {
        if won {
            f(h, 0xa40290, cg, cg, m.rg, m.rg, m.sg, m.sg, pg, pg, 0x64, 0x64, cname, m.rname, m.score, m.stage, pos, full, legend);
        } else {
            f(h, 0xa3ff94, cg, cg, m.sg, m.sg, m.rg, m.rg, pg, pg, 0x64, 0x64, cname, m.score, m.stage, m.rname, pos, full, legend);
        }
    };
    if p.rival_club <= -1 {
        first(h);
        return won;
    }
    let rv = praise_rival(p);
    if p.m1 != 0 {
        let date = praise_date_text(p);
        first(h);
        f(r, 0xa40214, 0x64, rv.rgs, rv.rgs, 0x64, short, rv.name, date);
    } else {
        first(h);
        if p.m4 != 0 && won {
            f(r, 0xa40168, 0x64, rv.rng, rv.rng, rv.rgs, rv.rgs, short, rv.nation_name, rv.name);
        } else if p.m4 != 0 {
            f(r, 0xa3fef0, rv.rng, rv.rng, rv.rgs, rv.rgs, short, rv.nation_name, rv.name);
        } else {
            f(r, 0xa400e8, 0x64, rv.rng, rv.rng, rv.rgs, rv.rgs, short, rv.nation_name, rv.name);
        }
    }
    praise_strcat(h, r);
    won
}

/// A nation by index (nation table + index × `nation::SIZE`, unchecked as in the original): its short-name gender
/// byte and short name.
unsafe fn praise_nation_by_index(i: i32) -> (i32, usize) {
    let n = read_usize(G_NATION_TABLE).wrapping_add((i as usize).wrapping_mul(nation::SIZE));
    (read_i8(n + nation::SHORT_GENDER_NAME) as i32, n + nation::SHORT_NAME)
}

/// The competition of slot 16 (club comp table + index × 0x6b): its short-name gender byte and short name.
unsafe fn praise_comp(p: &Praise) -> (i32, usize) {
    let c = read_usize(G_CLUB_COMP_TABLE).wrapping_add((p.comp as usize).wrapping_mul(0x6b));
    (read_i8(c + competition::SHORT_GENDER_NAME) as i32, c + competition::SHORT_NAME)
}

/// Kind 0x33, "{player} tipped to shine on world stage": the player in a competition (slot 16) with two nations
/// (values 9 and 10): by variant, an outlet praising his performances (0), a former international star or a former
/// great with his club and job predicting he will shine (1 and 2; variant 2 names the great's nation when he has no
/// club, and a great's tip without one), or "{nationality} player type" (the others, adjective form 0xc). The nations
/// are left out when one of them is the club's nation, and the second one when either is missing. With a rival club,
/// the paragraph is then kept as the first sentence of one about his form for that club (when value 1 is 0 and the
/// great % 3 is not 0) or about signing for it on a date (value 1 set).
/// Original: part of `Media::write_player_praise_news_text` (0x7483d2..0x74948e).
unsafe fn praise_body_world_stage(p: &mut Praise) {
    covered!("nma_4 praise world stage");
    p.piece = "body_world_stage";
    let f = Praise::format();
    let (h, pos, full, adj, cname, cg, legend) = (p.head(), p.pos(), p.full(), p.adj(), p.cname(), p.cg(), p.lname_buf());
    let outlet = p.at(praise_frame::OUTLET);
    let (compg, compname) = praise_comp(p);
    let cid = read_i32(p.nation());
    let own = p.nation_b == cid || p.nation_c == cid;
    let both = p.nation_b > -1 && p.nation_c > -1;
    match p.variant {
        0 => {
            let g = p.describe();
            let og = p.outlet_gender as i8 as i32;
            if own {
                if p.home {
                    f(h, 0xa3c97c, og, og, g, g, compg, compg, outlet, pos, full, compname);
                } else {
                    f(h, 0xa3ca9c, og, og, cg, cg, g, g, 0x64, compg, compg, outlet, cname, pos, full, compname);
                }
            } else if both {
                if p.home {
                    let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                    let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                    f(h, 0xa3ce7c, og, og, g, g, 0x64, compg, compg, nbg, nbg, ncg, ncg, outlet, pos, full, compname, nbname, ncname);
                } else {
                    let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                    let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                    f(h, 0xa3cfdc, og, og, cg, cg, g, g, 0x64, compg, compg, nbg, nbg, ncg, ncg, outlet, cname, pos, full, compname, nbname, ncname);
                }
            } else if p.home {
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3cbe4, og, og, g, g, 0x64, compg, compg, nbg, nbg, outlet, pos, full, compname, nbname);
            } else {
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3cd1c, og, og, cg, cg, g, g, 0x64, compg, compg, nbg, nbg, outlet, cname, pos, full, compname, nbname);
            }
        }
        1 => {
            p.describe();
            if own {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    f(h, 0xa3d25c, lcg, lcg, jg, jg, cg, cg, 0x64, compg, compg, lcname, job, legend, cname, full, compname);
                } else {
                    f(h, 0xa3d164, cg, cg, 0x64, compg, compg, legend, cname, full, compname);
                }
            } else if both {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                    let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                    f(h, 0xa3d6d8, lcg, lcg, jg, jg, cg, cg, compg, compg, nbg, nbg, ncg, ncg, lcname, job, legend, cname, full, compname, nbname, ncname);
                } else {
                    let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                    let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                    f(h, 0xa3d5a4, cg, cg, 0x64, compg, compg, nbg, nbg, ncg, ncg, legend, cname, full, compname, nbname, ncname);
                }
            } else if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                let jg = p.jg();
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3d47c, lcg, lcg, jg, jg, cg, cg, compg, compg, nbg, nbg, lcname, job, legend, cname, full, compname, nbname);
            } else {
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3d370, cg, cg, compg, compg, nbg, nbg, legend, cname, full, compname, nbname);
            }
        }
        2 => {
            p.describe_keep();
            let pg = p.pg();
            if p.legend == -1 {
                f(h, 0xa3e034, cg, cg, pg, pg, compg, compg, legend, cname, pos, full, compname);
            } else if own {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    f(h, 0xa3d970, lcg, lcg, jg, jg, cg, cg, pg, pg, compg, compg, lcname, job, legend, cname, pos, full, compname);
                } else {
                    let ln = p.legend_nation();
                    let lng = read_i8(ln + nation::SHORT_GENDER_NAME) as i32;
                    f(h, 0xa3d828, lng, lng, cg, cg, pg, pg, 0x64, compg, compg, ln + nation::SHORT_NAME, legend, cname, pos, full, compname);
                }
            } else if both {
                if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                    let jg = p.jg();
                    let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                    let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                    f(h, 0xa3dec0, lcg, lcg, jg, jg, cg, cg, pg, pg, compg, compg, nbg, nbg, ncg, ncg, lcname, job, legend, cname, pos, full, compname, nbname, ncname);
                } else {
                    let ln = p.legend_nation();
                    let lng = read_i8(ln + nation::SHORT_GENDER_NAME) as i32;
                    let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                    let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                    f(h, 0xa3dd50, lng, lng, cg, cg, pg, pg, compg, compg, nbg, nbg, ncg, ncg, ln + nation::SHORT_NAME, legend, cname, pos, full, compname, nbname, ncname);
                }
            } else if let Some((lcg, lcname, job)) = praise_legend_club(p) {
                let jg = p.jg();
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3dc04, lcg, lcg, jg, jg, cg, cg, pg, pg, compg, compg, nbg, nbg, lcname, job, legend, cname, pos, full, compname, nbname);
            } else {
                let ln = p.legend_nation();
                let lng = read_i8(ln + nation::SHORT_GENDER_NAME) as i32;
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3dabc, lng, lng, cg, cg, pg, pg, compg, compg, nbg, nbg, ln + nation::SHORT_NAME, legend, cname, pos, full, compname, nbname);
            }
        }
        _ => {
            p.adjective(0xc);
            let pg = p.pg();
            if own {
                f(h, 0xa3e120, 0x64, 0x64, pg, pg, compg, compg, adj, pos, full, compname);
            } else if both {
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                let (ncg, ncname) = praise_nation_by_index(p.nation_c);
                f(h, 0xa3e2e8, 0x64, 0x64, pg, pg, compg, compg, nbg, nbg, ncg, ncg, adj, pos, full, compname, nbname, ncname);
            } else {
                let (nbg, nbname) = praise_nation_by_index(p.nation_b);
                f(h, 0xa3e1f8, 0x64, 0x64, pg, pg, compg, compg, nbg, nbg, adj, pos, full, compname, nbname);
            }
        }
    }
    if p.rival_club <= -1 {
        return;
    }
    let r = p.reply();
    let rc = read_usize(G_CLUB_TABLE).wrapping_add((p.rival_club as usize).wrapping_mul(club::SIZE));
    let rlg = read_i8(rc + club::GENDER_NAME) as i32;
    if p.m1 == 0 {
        if praise_rem(p.legend, 3) == 0 {
            return;
        }
        covered!("nma_4 praise world stage form");
        praise_strcpy(r, h);
        f(h, 0xa3c8f0, rlg, rlg, r, p.short(), rc + club::NAME);
    } else {
        covered!("nma_4 praise world stage signing");
        let date = praise_date_text(p);
        praise_strcpy(r, h);
        f(h, 0xa3c860, rlg, rlg, 0x64, r, p.short(), rc + club::NAME, date);
    }
}

/// strcpy(dst, src).
unsafe fn praise_strcpy(dst: usize, src: usize) {
    write_u8(dst, 0);
    praise_strcat(dst, src);
}

/// Kind 0x2b, "{player} praised": by variant 0..3, an outlet praising his form (strength 0x1000000) or journalists
/// hailing him ("in {nation}" abroad), a club's player of the nation, an outlet lost for words ("{nationality} pundit"
/// abroad), an outlet lavishing praise. Other variants go by the strength flags when variant % 4 is not 0: 0x2c0040
/// lethal striker, 0x240040 striker of the competition (long name), 0xc0040 hotshot, 0x40040 striker of the
/// competition, 0x102040, 0x200840 and 0x840 strikers; otherwise the description is kept and, with strength
/// 0x1000000, the media praise him ("{nationality} media" abroad), else the variant becomes club % 4 (C remainder) and
/// the choice starts again (a negative club index with none of these flags loops for ever, as in the original).
/// Original: part of `Media::write_player_praise_news_text` (0x74948e..0x749b63).
unsafe fn praise_body_praised(p: &mut Praise) {
    covered!("nma_4 praise praised");
    p.piece = "body_praised";
    let f = Praise::format();
    let (h, pos, full, short, adj, cname, cg) = (p.head(), p.pos(), p.full(), p.short(), p.adj(), p.cname(), p.cg());
    let outlet = p.at(praise_frame::OUTLET);
    let og = p.outlet_gender as i8 as i32;
    let s = p.strengths;
    let nation = |p: &Praise| {
        let n = p.nation();
        (read_i8(n + nation::SHORT_GENDER_NAME) as i32, n + nation::SHORT_NAME)
    };
    let comp_long = |p: &Praise| {
        let c = read_usize(G_CLUB_COMP_TABLE).wrapping_add((p.comp as usize).wrapping_mul(0x6b));
        (read_i8(c + competition::GENDER_NAME) as i32, c + competition::NAME)
    };
    let mut v = p.variant;
    loop {
        match v {
            0 => {
                if s & 0x100_0000 != 0 {
                    f(h, 0xa3c754, og, og, cg, cg, 0x64, 0x64, outlet, cname, full, short);
                } else if p.home {
                    f(h, 0xa3c654, cg, cg, p.pg(), p.pg(), 0x64, cname, pos, full);
                } else {
                    let (ng, nname) = nation(p);
                    f(h, 0xa3c538, ng, ng, cg, cg, p.pg(), p.pg(), 0x64, nname, cname, pos, full);
                }
                return;
            }
            1 => {
                if p.home {
                    f(h, 0xa3c480, cg, cg, cname, full);
                } else {
                    let (ng, nname) = nation(p);
                    f(h, 0xa3c3c8, cg, cg, ng, ng, cname, full, nname);
                }
                return;
            }
            2 => {
                if p.home {
                    f(h, 0xa3c300, og, og, cg, cg, outlet, cname, full);
                } else {
                    p.adjective(0x1f);
                    f(h, 0xa3c240, 0x64, cg, cg, adj, cname, full);
                }
                return;
            }
            3 => {
                f(h, 0xa3c164, og, og, cg, cg, outlet, cname, full);
                return;
            }
            _ => {}
        }
        let odd = praise_rem(v, 4) != 0;
        if s & 0x2c0040 == 0x2c0040 && odd {
            let (ng, nname) = nation(p);
            f(h, 0xa3c030, cg, cg, 0x64, ng, ng, cname, full, short, nname);
            return;
        }
        if s & 0x240040 == 0x240040 && odd {
            let (clg, clname) = comp_long(p);
            f(h, 0xa3bee0, cg, cg, 0x64, clg, clg, cname, full, short, clname);
            return;
        }
        if s & 0xc0040 == 0xc0040 && odd {
            if p.home {
                f(h, 0xa3bdf4, cg, cg, cname, full);
            } else {
                p.adjective(0x16);
                f(h, 0xa3bcfc, cg, cg, 0x64, cname, full, adj);
            }
            return;
        }
        if s & 0x40040 == 0x40040 && odd {
            let (clg, clname) = comp_long(p);
            f(h, 0xa3bc10, cg, cg, clg, clg, cname, full, clname);
            return;
        }
        if s & 0x102040 == 0x102040 && odd {
            if p.home {
                f(h, 0xa3bb60, cg, cg, cname, full);
            } else {
                let (ng, nname) = nation(p);
                f(h, 0xa3ba90, cg, cg, ng, ng, cname, full, nname);
            }
            return;
        }
        if s & 0x200840 == 0x200840 && odd {
            if p.home {
                f(h, 0xa3b9e0, cg, cg, cname, full);
            } else {
                let (ng, nname) = nation(p);
                f(h, 0xa3b910, cg, cg, ng, ng, cname, full, nname);
            }
            return;
        }
        if s & 0x840 == 0x840 && odd {
            let (ng, nname) = nation(p);
            f(h, 0xa3b7d0, cg, cg, ng, ng, cname, full, short, nname);
            return;
        }
        p.describe_keep();
        if s & 0x100_0000 != 0 {
            if p.home {
                f(h, 0xa3b6cc, cg, cg, p.pg(), p.pg(), 0x64, cname, pos, full);
            } else {
                p.adjective(0x1c);
                f(h, 0xa3b5a8, 0x64, 0x64, cg, cg, p.pg(), p.pg(), 0x64, adj, cname, pos, full);
            }
            return;
        }
        v = praise_rem(p.club_index, 4);
    }
}

/// Kind 0x2f, "{player} attitude praised": a media pundit's belief (variant 0) or a national newspaper columnist's.
/// Original: part of `Media::write_player_praise_news_text` (0x749b63..0x749c05).
unsafe fn praise_body_attitude(p: &mut Praise) {
    covered!("nma_4 praise attitude");
    p.piece = "body_attitude";
    let f = Praise::format();
    let (h, pos, full, cname, cg) = (p.head(), p.pos(), p.full(), p.cname(), p.cg());
    let g = p.describe();
    if p.variant != 0 {
        f(h, 0xa3b48c, cg, cg, g, g, full, cname, pos);
    } else {
        f(h, 0xa3b378, g, g, cg, cg, pos, full, cname);
    }
}

/// Kind 0x2c (and any kind out of 0x29..0x34), "{player} set for call-up?": national papers lavishing praise, with
/// the rumour of a call-up to the nation of slot 7 ("in {nation}" abroad).
/// Original: part of `Media::write_player_praise_news_text` (0x749c05..0x749d35).
unsafe fn praise_body_call_up(p: &mut Praise) {
    covered!("nma_4 praise call up");
    p.piece = "body_call_up";
    let f = Praise::format();
    let (h, pos, full, cname, cg) = (p.head(), p.pos(), p.full(), p.cname(), p.cg());
    let g = p.describe();
    let (na, nag) = praise_nation_a(p);
    if p.home {
        f(h, 0xa3b258, cg, cg, g, g, nag, nag, cname, full, pos, na);
    } else {
        let n = p.nation();
        let ng = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
        f(h, 0xa3b118, ng, ng, cg, cg, g, g, nag, nag, n + nation::SHORT_NAME, cname, full, pos, na);
    }
}

/// The second paragraph: the reply of the manager (slot 13: 1, 2, 3 for none, other a blank), in the second person
/// when the manager is the human on screen, by kind (and, for the players' age [inference], by value 16 against 24
/// and 34, or 36 with value 17).
/// Original: part of `Media::write_player_praise_news_text` (0x749d35..0x74a30c).
unsafe fn praise_reply(p: &mut Praise) {
    let f = Praise::format();
    let (r, short) = (p.reply(), p.short());
    let manager = p.at(praise_frame::MANAGER_FULL);
    let k = p.kind.wrapping_sub(0x29);
    let k = if (0..12).contains(&k) { k } else { -1 };
    let staff_of = |i: i32| read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE));
    let human = read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300)));
    let older = if p.m17 != 0 { p.age > 0x24 } else { p.age > 0x22 };
    let blank = || {
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        sprintf(r, 0x9f1308); // " "
    };
    if human == staff_of(p.manager) {
        covered!("nma_4 praise reply human");
        match p.response {
            1 => match k {
                0 => {
                    f(r, 0xa3aaa0);
                }
                1 => {
                    f(r, 0xa3aa38, short);
                }
                3 => {
                    f(r, 0xa3a9d8);
                }
                4 | 7 => {
                    f(r, 0xa3a978, short);
                }
                5 => {
                    f(r, 0xa3aafc);
                }
                6 => {
                    f(r, 0xa3ab4c);
                }
                8 => {
                    f(r, 0xa3a908, short);
                }
                9 => {
                    f(r, 0xa3a8a0, 0x64, short);
                }
                10 => {
                    f(r, 0xa3a7c8, 0x64, short);
                }
                11 => {
                    f(r, 0xa3a83c, 0x64, short);
                }
                _ => {
                    f(r, 0xa3a754, 0x64, short);
                }
            },
            2 => match k {
                0 | 1 | 4 | 5 | 7 => {
                    f(r, 0xa3b008, 0x64, short);
                }
                3 => {
                    f(r, 0xa3af90, short);
                }
                6 => {
                    f(r, 0xa3b05c, 0x64, short);
                }
                8 => {
                    f(r, 0xa3af08, short);
                }
                9 => {
                    f(r, 0xa3ae80, short);
                }
                10 => {
                    f(r, 0xa3ad8c, short);
                }
                11 => {
                    f(r, 0xa3adf8, short);
                }
                _ => {
                    let t = if older {
                        0xa3ac40
                    } else if p.age >= 0x18 {
                        0xa3aba4
                    } else {
                        0xa3ad04
                    };
                    f(r, t, short);
                }
            },
            3 => {
                f(r, 0xa3b0e0); // "You have chosen not to reply to this press statement."
            }
            _ => blank(),
        }
    } else {
        covered!("nma_4 praise reply manager");
        match p.response {
            1 => match k {
                0 => {
                    f(r, 0xa39f14, manager);
                }
                1 => {
                    f(r, 0xa39e84, manager, short);
                }
                3 => {
                    f(r, 0xa39e00, manager);
                }
                4 | 7 => {
                    f(r, 0xa39d80, manager, short);
                }
                5 => {
                    f(r, 0xa39f98, manager);
                }
                6 => {
                    f(r, 0xa3a010, manager);
                }
                8 => {
                    f(r, 0xa39d0c, manager, short);
                }
                9 => {
                    f(r, 0xa39c80, 0x64, manager, short);
                }
                10 => {
                    f(r, 0xa39b5c, 0x64, manager, short);
                }
                11 => {
                    f(r, 0xa39bf8, 0x64, manager, short);
                }
                _ => {
                    f(r, 0xa39ac4, 0x64, manager, short);
                }
            },
            2 => {
                let t = match k {
                    0 | 1 | 4 | 5 | 7 => 0xa3a630,
                    3 => 0xa3a588,
                    6 => 0xa3a6ac,
                    8 => 0xa3a4d8,
                    9 => 0xa3a430,
                    10 => 0xa3a2ec,
                    11 => 0xa3a384,
                    _ => {
                        if older {
                            0xa3a14c
                        } else if p.age >= 0x18 {
                            0xa3a08c
                        } else {
                            0xa3a238
                        }
                    }
                };
                f(r, t, 0x64, 0x64, manager, short);
            }
            3 => {
                f(r, 0x9f1308); // " "
            }
            _ => blank(),
        }
    }
}

/// The headline: "Praise for {club} {position}" when variant % 8 is player % 7 (C remainders), otherwise by kind:
/// rated highly, a 'future international' or set for call-up (0x29, by (variant + player) % 3), shows promise, given
/// boost or youngster praised (0x2a, the same), praised, set for call-up, on the rise, big move close, attitude
/// praised, world class talent (0x30, 0x31), impresses at {club}, tipped to shine on world stage, lights up or stars in
/// {competition} (0x34, by (variant × player) % 17 above 6), touted (other kinds). The match link's text (frame +0x4c)
/// is emptied.
/// Original: part of `Media::write_player_praise_news_text` (0x74a33f..0x74a5d4).
unsafe fn praise_headline(p: &mut Praise) {
    covered!("nma_4 praise headline");
    p.piece = "headline";
    let f = Praise::format();
    let (h, short) = (p.head(), p.short());
    write_u8(p.at(0x4c), 0);
    if praise_rem(p.variant, 8) == praise_rem(p.player, 7) {
        f(h, 0xa39a70, p.cg(), p.cg(), p.pg(), p.pg(), p.cname(), p.pos());
        return;
    }
    let three = praise_rem(p.variant.wrapping_add(p.player), 3);
    match p.kind.wrapping_sub(0x29) {
        0 => {
            let t = match three {
                0 => 0xa39900,
                1 => 0xa39930,
                _ => 0xa3996c,
            };
            f(h, t, short);
        }
        1 => {
            let t = match three {
                0 => 0xa39864,
                1 => 0xa39894,
                _ => 0xa398cc,
            };
            f(h, t, short);
        }
        2 => {
            f(h, 0xa39708, short);
        }
        3 => {
            f(h, 0xa3996c, short);
        }
        4 => {
            f(h, 0xa399a0, short);
        }
        5 => {
            f(h, 0xa39a04, short);
        }
        6 => {
            f(h, 0xa39a3c, 0x64, short);
        }
        7 | 8 => {
            f(h, 0xa399d0, short);
        }
        9 => {
            f(h, 0xa39810, p.cg(), p.cg(), short, p.cname());
        }
        10 => {
            f(h, 0xa39734, short);
        }
        11 => {
            let (compg, compname) = praise_comp(p);
            let t = if praise_rem(p.variant.wrapping_mul(p.player), 0x11) > 6 { 0xa397c0 } else { 0xa39774 };
            f(h, t, compg, compg, short, compname);
        }
        _ => {
            f(h, 0xa396e0, short);
        }
    }
}

/// The links of the text (see the routine's header).
unsafe fn praise_links(p: &mut Praise, out: usize) {
    covered!("nma_4 praise links");
    let link = |name: usize, kind: u32, data: usize| {
        exe::text_add_link(out, name as *const u8, kind, data);
    };
    let staff_of = |i: i32| read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE));
    let club_of = |i: i32| read_usize(G_CLUB_TABLE).wrapping_add((i as usize).wrapping_mul(club::SIZE));
    let national = |n: usize| exe::database_get_national_team(n);
    let rec = p.record.as_ptr() as usize;
    if p.has_match != 0 {
        // Match_Stats::fixture_entry_matches(index, club a, club b, day, year) (thiscall, ret 0x14; the dwords pushed)
        let g: unsafe extern "thiscall" fn(usize, i32, i32, i32, i32, i32) -> i32 = core::mem::transmute(0x729500usize);
        if g(read_usize(G_MATCH_STATS), read_i32(rec + 0x1b), read_i32(rec + 0x11), read_i32(rec + 0x15), read_i32(rec), read_i32(rec + 2)) != 0 {
            link(p.at(0x4c), 0x7d3, read_usize(rec + 0x1b));
        }
        let c = club_of(read_i32(rec + 0x15));
        link(c + club::NAME, 0x7d5, c);
    }
    if p.player > -1 && p.player < read_i32(G_STAFF_COUNT)
        && exe::database_staff_matches_names(staff_of(p.player), p.names[0], p.names[1], p.names[2], 0usize) != 0
    {
        link(p.full(), 0x7d7, staff_of(p.player));
        link(p.short(), 0x7d7, staff_of(p.player));
    }
    if p.manager > -1
        && exe::database_staff_matches_names(staff_of(p.manager), p.manager_names[0], p.manager_names[1], p.manager_names[2], 0usize) != 0
    {
        link(p.at(praise_frame::MANAGER_FULL), 0x7d7, staff_of(p.manager));
        link(p.at(praise_frame::MANAGER_SHORT), 0x7d7, staff_of(p.manager));
    }
    if p.legend > -1
        && exe::database_staff_matches_names(staff_of(p.legend), p.legend_names[0], p.legend_names[1], p.legend_names[2], 0usize) != 0
    {
        link(p.lname_buf(), 0x7d7, staff_of(p.legend));
    }
    for c in [p.legend_club, p.rival_club] {
        if c != -1 {
            let r = club_of(c);
            link(r + club::SHORT_NAME, 0x7d5, r);
            link(r + club::NAME, 0x7d5, r);
        }
    }
    if p.club_index != -1 {
        link(p.cname(), 0x7d5, p.club);
        link(p.lname(), 0x7d5, p.club);
        let n = p.nation();
        link(n + 4, 0x7d5, national(n));
        link(n + nation::SHORT_NAME, 0x7d5, national(n));
        for form in 0x16..=0x1f {
            exe::nation_get_nationality_adjective(n, p.adj(), 0x78, form);
            link(p.adj(), 0x7d5, national(n));
        }
    }
    for i in [p.nation_a, p.nation_b, p.nation_c] {
        if i != -1 {
            let n = read_usize(G_NATION_TABLE).wrapping_add((i as usize).wrapping_mul(nation::SIZE));
            link(n + nation::SHORT_NAME, 0x7d5, national(n));
            link(n + 4, 0x7d5, national(n));
        }
    }
    if p.comp != -1 {
        let c = read_usize(G_CLUB_COMP_TABLE).wrapping_add((p.comp as usize).wrapping_mul(0x6b));
        link(c + competition::NAME, 0x7d0, c);
        link(c + competition::SHORT_NAME, 0x7d0, c);
    }
}

/// The bytes of a C string (empty for a null pointer).
unsafe fn nma4_c_text(p: usize) -> Vec<u8> {
    let mut v = Vec::new();
    if p == 0 {
        return v;
    }
    let mut k = 0;
    while read_u8(p + k) != 0 {
        v.push(read_u8(p + k));
        k += 1;
    }
    v
}

/// The links of a Text object (`+4` the link list): the list's cursor (+0xa) starts at its head (+2) and each step
/// (as `0x68d1b0` does) takes the node's link (+0) and moves to the next (+6); a link holds its text at +0, its type at
/// +0xc8 and its data at +0xcc. The cursor is put back afterwards.
unsafe fn nma4_links(text: usize) -> Vec<(Vec<u8>, u32, u32)> {
    let mut v = Vec::new();
    let list = read_usize(text + 4);
    if list == 0 {
        return v;
    }
    let saved = read_u32(list + 0xa);
    write_u32(list + 0xa, read_u32(list + 2));
    loop {
        let node = read_usize(list + 0xa);
        if node == 0 {
            break;
        }
        let link = read_usize(node);
        write_u32(list + 0xa, read_u32(node + 6));
        v.push((nma4_c_text(link), read_u32(link + 0xc8), read_u32(link + 0xcc)));
    }
    write_u32(list + 0xa, saved);
    v
}

/// Comparer for `write_player_praise_news_text`: the original writes into `out`; the copy, side by side (the text
/// buffer 0xde1f64 and the generator's state), into a local empty Text; then the strings and the new links (text, type,
/// data) are compared and the local Text is destroyed. A divergence names the piece that wrote the text. The match
/// link's text (0x7d3) is not compared in the body of a kind other than 0x34 (the original's uninitialised frame byte).
pub(crate) unsafe extern "thiscall" fn compare_write_player_praise_news_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I0%%);
    let before = nma4_links(out).len();
    let mut local = [0usize; 2];
    let lp = local.as_mut_ptr() as usize;
    let est = crate::rng::STATE_REGIONS;
    let regions = [(PRAISE_TEXT_BUFFER, 2000usize), est[0], est[1]];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions,
        || o(this, news, body, out),
        || crate::rng::without_counting(|| write_player_praise_news_text(this, news, body, lp)));
    let piece = PRAISE_LAST_PIECE;
    let (so, sr) = (nma4_c_text(read_usize(out)), nma4_c_text(read_usize(lp)));
    let lo: Vec<_> = nma4_links(out).into_iter().skip(before).collect();
    let lr = nma4_links(lp);
    let kind = exe::newsitem_get_slot(news, 5);
    let loose = body != 0 && kind != 0x34;
    let links_same = lo.len() == lr.len()
        && lo.iter().zip(lr.iter()).all(|(a, b)| a.1 == b.1 && a.2 == b.2 && (a.0 == b.0 || (loose && a.1 == 0x7d3)));
    SCOREBOARD.record(same && so == sr && links_same, || format!(
        "write_player_praise_news_text(kind {:#x}, body {}, piece {}) text {} links {} ({} x {}) memory {}\n original: {}\n rust:     {}\n links original {:x?}\n links rust {:x?}\n {:x?} x {:x?}",
        kind, body, piece, so == sr, links_same, lo.len(), lr.len(), same, String::from_utf8_lossy(&so), String::from_utf8_lossy(&sr),
        lo.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>(),
        lr.iter().map(|l| (String::from_utf8_lossy(&l.0).into_owned(), l.1, l.2)).collect::<Vec<_>>(), mo, mr));
    exe::drop_text(local.as_mut_ptr());
}
