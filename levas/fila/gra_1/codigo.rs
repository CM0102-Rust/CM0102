// ---------- batch %%LEVA%%: the news text writer (news.cpp) ----------

/// The size of a competition record of `G_CLUB_COMP_TABLE`.
const NEWS_COMP_SIZE: usize = 0x6b;

/// The piece of the last text written (for the comparer's message).
static mut NEWS_LAST_PIECE: &str = "";

/// The game's text formatter (`langlib_sprintf`: `{}` markers take the gender ints).
unsafe fn news_format() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}
/// The C runtime `sprintf`.
unsafe fn news_sprintf() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_CRT_SPRINTF)
}
/// The club at index `i` of `G_CLUB_TABLE` (32-bit wrapping, no bounds check, like the original).
unsafe fn news_club(i: u32) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add((i as usize).wrapping_mul(club::SIZE))
}
/// The competition at index `i` of `G_CLUB_COMP_TABLE`.
unsafe fn news_comp(i: u32) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add((i as usize).wrapping_mul(NEWS_COMP_SIZE))
}
/// The nation at index `i` of `G_NATION_TABLE`.
unsafe fn news_nation(i: u32) -> usize {
    read_usize(G_NATION_TABLE).wrapping_add((i as usize).wrapping_mul(nation::SIZE))
}
/// The person with id `i` of `G_STAFF_TABLE`.
unsafe fn news_staff(i: u32) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE))
}
/// A gender byte (sign-extended, as the original's `movsx`).
unsafe fn news_g(a: usize) -> i32 {
    read_i8(a) as i32
}
/// `Text::add_link(out, text, kind, target)`.
unsafe fn news_link(out: usize, text: usize, kind: u32, target: usize) {
    exe::text_add_link(out, text as *const u8, kind, target);
}
/// `Text::set_string(out, text)`.
unsafe fn news_set(out: usize, text: usize) {
    exe::text_set_string(out, text as *const u8);
}
/// The C `strcmp(a, b) == 0`.
unsafe fn news_streq(a: usize, b: usize) -> bool {
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

/// The news.cpp "Error" message box for source line `line`: the file name and extension (`exe::crt_strtok`, the
/// C runtime splitpath, of the news.cpp path 0xa693ec), the version, "v%s %s.%s %d" at 0xae24d0, the box
/// (`exe::gui_show_message_box`) and the mouse button released.
unsafe fn news_error_box(line: u32) {
    let (mut name, mut ext) = ([0u8; 0x100], [0u8; 0x100]);
    exe::crt_strtok(0xa693ec, 0, 0, name.as_mut_ptr(), ext.as_mut_ptr());
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), line);
    news_sprintf()(0xae24d0, 0x9870e8, v, name.as_ptr(), ext.as_ptr(), line); // "v%s %s.%s %d"
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0); // "Error"
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// How a news type is written by another routine: (category, type, writer).
#[derive(Clone, Copy)]
enum NewsWriter {
    /// A `thiscall(object, news, body, out)` method of the object held by a global.
    Global(usize, usize),
    /// A `thiscall(object, news, body, out)` method of a fixed object.
    Object(usize, usize),
    /// A `cdecl(news, body, out)` routine.
    Plain(usize),
}

/// The types written by another routine, by category (`NewsItem::get_category`) and type, in the original's order.
const NEWS_WRITERS: &[(u8, u32, NewsWriter)] = {
    use NewsWriter::*;
    &[
        (2, 0x1d, Global(0xb74318, 0x76e3b0)),
        (2, 0x1e, Global(0xb74318, 0x76f370)),
        (2, 0x20, Global(0xb74318, 0x76fa40)),
        (2, 0x21, Global(0xb74318, 0x76ff40)),
        (2, 0x22, Global(0xb74318, 0x7703f0)),
        (2, 0x24, Global(0xb74318, 0x770d20)),
        (2, 0x25, Global(0xb74434, 0x7e7280)),
        (2, 0x2c, Global(0xb74318, 0x777ca0)),
        (2, 0x2f, Global(0xde1f58, 0x906130)),
        (2, 0x2e, Global(0xb6b7c8, 0x75c5a0)),
        (2, 0x2a, Global(0xde2f74, 0x924160)),
        (2, 0x2, Global(0xde2f74, 0x923dd0)),
        (2, 0x29, Global(0xb67a44, 0x6aa910)),
        (2, 0x2b, Global(0xb67a44, 0x6aadb0)),
        (2, 0x18, Global(0xdd7ef0, 0x85af40)),
        (2, 0x19, Global(0xdd7ef0, 0x85b190)),
        (7, 0x3e8, Global(0xad9dac, 0x44c650)),
        (3, 0x7d1, Global(0xb63d28, 0x631580)),
        (3, 0x7d2, Global(0xae28f0, 0x5576e0)),
        (3, 0x7d3, Global(0xae28f0, 0x55b8b0)),
        (6, 0x1779, Global(0xae2a38, 0x59f3e0)),
        (6, 0x177f, Global(0xae2a38, 0x5a0230)),
        (6, 0x1780, Global(0xae2a38, 0x5a04c0)),
        (6, 0x177d, Global(0xae2a38, 0x5a0050)),
        (6, 0x177a, Global(0xae2a38, 0x59f9c0)),
        (6, 0x177b, Global(0xae2a38, 0x59fcf0)),
        (6, 0x1783, Global(0xae2a38, 0x5a0380)),
        (6, 0x1786, Global(0xb67a44, 0x6a9470)),
        (6, 0x1784, Global(0xae2a3c, 0x5a7ed0)),
        (6, 0x177c, Global(0xae2a38, 0x59feb0)),
        (6, 0x1775, Global(0xb67a44, 0x6a0690)),
        (6, 0x1778, Global(0xb67a44, 0x6a0ad0)),
        (6, 0x1774, Global(0xb67a44, 0x6a5120)),
        (6, 0x1777, Global(0xb67a44, 0x6a1190)),
        (6, 0x1776, Global(0xb67a44, 0x69fef0)),
        (6, 0x1770, Global(0xb67a44, 0x6a1d60)),
        (6, 0x1773, Global(0xb67a44, 0x69c6b0)),
        (6, 0x177e, Global(0xb67a44, 0x6a4550)),
        (6, 0x1771, Global(0xb67a44, 0x6a4ae0)),
        (6, 0x1772, Global(0xb67a44, 0x6a4d40)),
        (6, 0x1782, Global(0xb67a44, 0x6a5e30)),
        (6, 0x1781, Global(0xb67a44, 0x6a5b40)),
        (6, 0x1788, Global(0xae2a38, 0x59fb90)),
        (6, 0x1789, Global(0xb67a44, 0x6aab70)),
        (6, 0x178a, Global(0xb67a44, 0x6ac190)),
        (6, 0x178e, Global(0xb6b7c8, 0x758fe0)),
        (6, 0x178d, Global(0xb67a44, 0x6ac880)),
        (6, 0x178f, Global(0xb67a44, 0x6adb70)),
        (6, 0x1790, Global(0xb6b7c8, 0x74af60)),
        (6, 0x178c, Plain(0x58a660)),
        (4, 0xbbb, Global(0xb7434c, 0x7aa4c0)),
        (4, 0xbdb, Global(0xb6b7c8, 0x72bd50)),
        (4, 0xbdd, Global(0xb6b7c8, 0x744f80)),
        (4, 0xbdc, Global(0xb6b7c8, 0x743d30)),
        (4, 0xbd0, Global(0xb6b7c8, 0x72d270)),
        (4, 0xbdf, Global(0xb6b7c8, 0x74bbd0)),
        (4, 0xbda, Global(0xb6b7c8, 0x733000)),
        (4, 0xbde, Plain(0x589480)),
        (4, 0xbcf, Global(0xb6b7c8, 0x7435a0)),
        (4, 0xbd9, Global(0xb6b7c8, 0x733fc0)),
        (4, 0xbce, Global(0xb7434c, 0x7a8eb0)),
        (4, 0xbcd, Plain(0x5886d0)),
        (4, 0xbba, Global(0xb7434c, 0x7a9650)),
        (4, 0xbb8, Global(0xae1050, 0x4d4fb0)),
        (4, 0xbb9, Global(0xae1050, 0x4daa00)),
        (4, 0xbbc, Global(0xae1050, 0x4daec0)),
        (4, 0xbbe, Global(0xae1050, 0x4d6ee0)),
        (4, 0xbbd, Global(0xae1050, 0x4d6a00)),
        (4, 0xbbf, Global(0xae1050, 0x4d8990)),
        (4, 0xbe1, Plain(0x589880)),
        (4, 0xbe4, Global(0xb6b7c8, 0x75d210)),
        (4, 0xbe5, Global(0xb6b7c8, 0x75b9c0)),
        (4, 0xbe2, Plain(0x589d60)),
        (4, 0xbe3, Plain(0x58a210)),
        (4, 0xbe6, Global(0xb6b7c8, 0x759a10)),
        (4, 0xbe9, Global(0xae2a30, 0x56a560)),
        (4, 0xbe8, Global(0xb6b7c8, 0x75a940)),
        (4, 0xbea, Global(0xae1050, 0x4d7770)),
        (4, 0xbc0, Global(0xae1050, 0x4d95e0)),
        (4, 0xbc1, Global(0xae1050, 0x4d9a90)),
        (4, 0xbc2, Global(0xae1050, 0x4da190)),
        (4, 0xbc5, Global(0xad9c6c, 0x419c60)),
        (4, 0xbc6, Global(0xae2a3c, 0x5a76e0)),
        (4, 0xbc7, Plain(0x587e90)),
        (4, 0xbc8, Plain(0x5883f0)),
        (4, 0xbc9, Global(0xad9c6c, 0x41a8e0)),
        (4, 0xbcb, Global(0xb6b7c8, 0x7572a0)),
        (4, 0xbca, Plain(0x5889e0)),
        (4, 0xbc3, Global(0xde1f54, 0x8e3b80)),
        (4, 0xbc4, Global(0xae1050, 0x4d65c0)),
        (4, 0xbcc, Global(0xde1f54, 0x8e4460)),
        (5, 0xfa0, Global(0xde1f54, 0x8d3520)),
        (5, 0xfa1, Global(0xde1f54, 0x8d5420)),
        (5, 0xfbc, Global(0xb6b7c8, 0x755860)),
        (5, 0xfbd, Plain(0x587370)),
        (5, 0xfbe, Plain(0x587800)),
        (5, 0xfa2, Global(0xde1f54, 0x8d6e80)),
        (5, 0xfa3, Global(0xde1f54, 0x8d7870)),
        (5, 0xfac, Global(0xde1f54, 0x8d7fb0)),
        (5, 0xfa4, Global(0xde1f54, 0x8d9480)),
        (5, 0xfa5, Global(0xde1f54, 0x8da8f0)),
        (5, 0xfa7, Global(0xde1f54, 0x8db580)),
        (5, 0xfa6, Global(0xde1f54, 0x8dba80)),
        (5, 0xfa8, Global(0xde1f54, 0x8dc370)),
        (5, 0xfa9, Global(0xde1f54, 0x8dd830)),
        (5, 0xfab, Global(0xde1f54, 0x8ddc70)),
        (5, 0xfad, Global(0xde1f54, 0x8de320)),
        (5, 0xfae, Global(0xde1f54, 0x8d84e0)),
        (5, 0xfaf, Global(0xde1f54, 0x8d8af0)),
        (5, 0xfb0, Global(0xde1f54, 0x8de980)),
        (5, 0xfb1, Global(0xde1f54, 0x8dffe0)),
        (5, 0xfb2, Global(0xde1f54, 0x8e05a0)),
        (5, 0xfb3, Global(0xde1f54, 0x8e08e0)),
        (5, 0xfb4, Global(0xde1f54, 0x8e1370)),
        (5, 0xfb5, Global(0xde1f54, 0x8e19c0)),
        (5, 0xfb6, Global(0xde1f54, 0x8e20a0)),
        (5, 0xfb7, Global(0xde1f54, 0x8e2570)),
        (5, 0xfb8, Global(0xae1050, 0x4db2e0)),
        (5, 0xfb9, Global(0xde1f54, 0x8e2cd0)),
        (5, 0xfba, Global(0xde1f54, 0x8e32c0)),
        (5, 0xfbb, Global(0xde1f54, 0x8e3780)),
        (5, 0xfbf, Global(0xde1f54, 0x8e3f90)),
        (5, 0xfc0, Global(0xde1f54, 0x8e47a0)),
        (5, 0xfc2, Plain(0x58aa60)),
        (5, 0xfc1, Plain(0x58a900)),
        (1, 0x138d, Plain(0x588f00)),
        (1, 0x138f, Global(0xad9c6c, 0x41b030)),
        (1, 0x1390, Global(0xb67a44, 0x69c910)),
        (1, 0x1391, Object(0xadadf4, 0x487f60)),
        (1, 0x1393, Global(0xb7433c, 0x795f50)),
        (1, 0x1394, Global(0xae2a3c, 0x5a8090)),
        (1, 0x1392, Object(0xb74344, 0x7a1cd0)),
        (1, 0x138e, Global(0xb74448, 0x7f9d10)),
        (1, 0x138c, Global(0xb67a44, 0x6a6310)),
        (1, 0x1389, Plain(0x7b7840)),
        (1, 0x138a, Plain(0x7b81f0)),
        (1, 0x138b, Plain(0x7b95c0)),
        (6, 0x1787, Global(0xb67a44, 0x6a3060)),
        (4, 0xbe0, Global(0xb6b7c8, 0x738d50)),
        (5, 0xfc3, Global(0xde1f54, 0x8e51d0)),
    ]
};

/// Writes the headline (`body` 0) or the body of a news item into `out`, returning 1 (0 for a type-26 news whose
/// manager record is missing, and after an error box).
///
/// `NewsItem::write_text(news, manager, out, body, flag)`: with no manager or no `out`, the news.cpp error box (lines
/// 0x9f and 0xa7) and 0. Otherwise `out` is cleared (`exe::text_clear`) and the news is written by its category
/// (`exe::newsitem_get_category`) and type (the news `TYPE` dword, +0): most types go to the writer of their subject
/// (`NEWS_WRITERS`: the media, the coach report, the transfers, the competitions, the discipline, the records...); the
/// types the routine writes itself are the pieces below (category 2: the match moved, televised or rearranged, the host
/// bids, the finances, the season's end, the job offers...; category 3: the match report; category 7: the club
/// record; category 1: the follow-up news). Types 27, 28, 31 and 35 call their writers with other arguments (see the
/// code). An unknown type gives the error box of line 0x1226 and 0.
/// Original: `NewsItem::write_text` (0x77bc80, news.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn newsitem_write_text(news: usize, manager: usize, out: usize, body: u32, flag: u32) -> u32 {
    NEWS_LAST_PIECE = "";
    if manager == 0 {
        news_error_box(0x9f);
        return 0;
    }
    if out == 0 {
        news_error_box(0xa7);
        return 0;
    }
    let clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4afa40usize); // Text::clear
    clear(out);
    let cat = exe::newsitem_get_category(news);
    let ty = read_u32(news);
    let three = |f: usize, this: usize| {
        let w: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(f);
        w(this, news, body, out);
    };
    if cat == 2 {
        match ty {
            0x1b => {
                // the club record news: (this, f5, f9, news, out, body)
                let w: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize, u32) -> u32 = core::mem::transmute(0x4143f0usize);
                w(read_usize(0xad9c60), read_u32(news + 5), read_u32(news + 9), news, out, body);
                return 1;
            }
            0x1c => {
                let w: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x414420usize);
                w(read_usize(0xad9c60), news, out, body);
                return 1;
            }
            0x1f => {
                three(0x48eb00, read_usize(read_usize(0xadadfc).wrapping_add((read_u32(news + 9) as usize).wrapping_mul(4))));
                return 1;
            }
            0x23 => {
                three(0x48ee40, read_usize(read_usize(0xadadfc).wrapping_add((read_u32(news + 5) as usize).wrapping_mul(4))));
                return 1;
            }
            0x1a => {
                let coach = read_usize(read_usize(0xadadfc).wrapping_add((read_u32(news + 5) as usize).wrapping_mul(4)));
                if coach == 0 {
                    return 0;
                }
                three(0x48e590, coach);
                return 1;
            }
            1 => return news_match_rearranged(news, body, out),
            3 => return news_match_televised(news, body, out),
            4 => return news_host_bids(news, body, out),
            5 => return news_host_chosen(news, body, out),
            6 => return news_friendly_proposal(news, body, out),
            7 => return news_friendly_answered(news, body, out),
            8 => return news_friendly_cancelled(news, body, out),
            9 => return news_tournament_invitation(news, body, out),
            10 => return news_tournament_answered(news, body, out),
            11 => return news_tournament_cancelled(news, body, out),
            12 => return news_comp_report(news, body, out),
            13 => return news_fixture_list(news, body, out),
            16 => return news_comp_match_report(news, body, out),
            17 => return news_playoff_place(news, body, out, false),
            18 => return news_playoff_place(news, body, out, true),
            19 => return news_leagues_reshuffled(news, body, out),
            20 => return news_cup_draw(news, body, out),
            21 => return news_group_draw(news, body, out),
            22 => return news_intertoto_invitation(news, body, out),
            23 => return news_intertoto_chosen(news, body, out),
            0x26 => return news_cup_draw_open(news, body, out),
            0x27 => return news_cup_draw_today(news, body, out),
            0x28 => return news_world_club_bye(news, body, out),
            _ => {}
        }
    } else if cat == 7 && ty == 0x3e9 {
        return news_club_record(news, body, out);
    } else if cat == 3 && ty == 0x7d0 {
        return news_match_report(news, body, out);
    } else if cat == 1 && ty == 0x1388 {
        return news_follow_up(news, manager, out, body, flag);
    }
    if let Some(&(_, _, w)) = NEWS_WRITERS.iter().find(|e| e.0 == cat && e.1 == ty) {
        match w {
            NewsWriter::Global(g, f) => three(f, read_usize(g)),
            NewsWriter::Object(o, f) => three(f, o),
            NewsWriter::Plain(f) => {
                let w: unsafe extern "C" fn(usize, u32, usize) -> u32 = core::mem::transmute(f);
                w(news, body, out);
            }
        }
        return 1;
    }
    news_error_box(0x1226);
    0
}

/// Type 1, the match rearranged. Slots: +5 and +9 the two clubs, +0xd the competition, +0x11 and +0x15 the stage and
/// the round (words), +0x19/+0x1d the old date and +0x35/+0x39 the new one (`Date::Date` of two words each), +0x21 a
/// club, +0x25 another club and +0x29 a competition of the clash (+0x21 = −1 none).
///
/// Body: the round's label (`exe::comp_util_format_comp_stage_round_label`, form 2; "Friendly" and "Reserve" get a
/// lower-case first letter) and its gender (`exe::comp_util_get_label_gender`), both dates without the year
/// (`exe::date_get_long_date_string_no_year`); with a clash 0xa692ec (the clubs' names and genders, the label, the
/// dates, the clash clubs' and competition's short names and short genders), otherwise 0xa691c8. Links: the two clubs
/// (0x7d5), the competition on the label (0x7d0), and with a clash its two clubs' short names and its competition's.
/// Headline 0xa69184 with the clubs' short names and short genders and their links.
/// Original: part of `NewsItem::write_text` (0x77bfeb..0x77c46a).
unsafe fn news_match_rearranged(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "match_rearranged";
    let f = news_format();
    let (c1, c2) = (news_club(read_u32(news + 5)), news_club(read_u32(news + 9)));
    if body == 0 {
        f(NEWS_TEXT_BUFFER, 0xa69184, news_g(c1 + club::SHORT_GENDER_NAME), news_g(c1 + club::SHORT_GENDER_NAME),
          news_g(c2 + club::SHORT_GENDER_NAME), news_g(c2 + club::SHORT_GENDER_NAME), c1 + club::SHORT_NAME, c2 + club::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
        news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
        return 1;
    }
    covered!("gra_1 match rearranged body");
    let comp = news_comp(read_u32(news + 0xd));
    let (stage, round) = (read_u16(news + 0x11) as u32, read_u16(news + 0x15) as u32);
    let mut label = [0u8; 0xc8];
    let lb = label.as_mut_ptr() as usize;
    exe::comp_util_format_comp_stage_round_label(comp, stage, round, 2, label.as_mut_ptr(), 0xc8);
    if news_streq(lb, 0x9db958) {
        label[0] = b'f'; // "Friendly"
    } else if news_streq(lb, 0x9b5b40) {
        label[0] = b'r'; // "Reserve"
    }
    let lg = exe::comp_util_get_label_gender(comp as u32, stage, round, 2) as i8 as i32;
    let (mut d1, mut d2) = ([0u8; 8], [0u8; 8]);
    let (mut s1, mut s2) = ([0u8; 0xc8], [0u8; 0xc8]);
    news_date(d1.as_mut_ptr() as usize, read_u16(news + 0x19), read_u16(news + 0x1d));
    exe::date_get_long_date_string_no_year(d1.as_ptr() as usize, s1.as_mut_ptr() as usize, 0xc8, 0xff);
    news_date(d2.as_mut_ptr() as usize, read_u16(news + 0x35), read_u16(news + 0x39));
    exe::date_get_long_date_string_no_year(d2.as_ptr() as usize, s2.as_mut_ptr() as usize, 0xc8, 0xff);
    let (s1, s2) = (s1.as_ptr() as usize, s2.as_ptr() as usize);
    let clash = read_i32(news + 0x21);
    let (g1, g2) = (news_g(c1 + club::GENDER_NAME), news_g(c2 + club::GENDER_NAME));
    if clash != -1 {
        let comp2 = news_comp(read_u32(news + 0x29));
        let cc = news_club(read_u32(news + 0x25));
        let cb = news_club(clash as u32);
        let (gk, gc, gb) = (news_g(comp2 + competition::SHORT_GENDER_NAME), news_g(cc + club::SHORT_GENDER_NAME), news_g(cb + club::SHORT_GENDER_NAME));
        f(NEWS_TEXT_BUFFER, 0xa692ec, g1, g1, g2, g2, lg, lg, 0x64, gb, gb, gc, gc, gk, gk, 0x64, c1 + club::NAME, c2 + club::NAME,
          lb, s1, cb + club::SHORT_NAME, cc + club::SHORT_NAME, comp2 + competition::SHORT_NAME, s2);
    } else {
        f(NEWS_TEXT_BUFFER, 0xa691c8, lg, lg, g1, g1, g2, g2, 0x64, 0x64, lb, c1 + club::NAME, c2 + club::NAME, s2, s1);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c1 + club::NAME, 0x7d5, c1);
    news_link(out, c2 + club::NAME, 0x7d5, c2);
    news_link(out, lb, 0x7d0, comp);
    if clash != -1 {
        covered!("gra_1 match rearranged clash");
        let cb = news_club(clash as u32);
        news_link(out, cb + club::SHORT_NAME, 0x7d5, cb);
        let cc = news_club(read_u32(news + 0x25));
        news_link(out, cc + club::SHORT_NAME, 0x7d5, cc);
        let comp2 = news_comp(read_u32(news + 0x29));
        news_link(out, comp2 + competition::SHORT_NAME, 0x7d0, comp2);
    }
    1
}

/// `Date::Date(this, a, b)` (8 bytes: the two words and the leap-year flag).
unsafe fn news_date(d: usize, a: u16, b: u16) {
    let ctor: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    ctor(d, a as u32, b as u32);
}

/// Type 3, the match chosen for live television. Slots: +5 and +9 the clubs, +0xd the competition, +0x11 and +0x15
/// the stage and round (words), +0x19/+0x29 and +0x25 the old and new date words (+0x29 the year of the new date,
/// +0x19 the day of the old one compared with the new +0x25), +0x21 and +0x2d the old and new time of day (bytes).
///
/// Body: the round's label (as in type 1, "Friendly"/"Reserve" lower-cased) and its gender; a new day: 0xa690f0 with
/// the new date without the year; the same day at another time of day: 0xa69044 with the time of day's name and
/// gender (`exe::date_get_time_of_day_name`, `exe::date_get_time_of_day_gender`); the same day and time: 0xa68fc0.
/// Links: the two clubs' names (0x7d5) and the competition on the label (0x7d0). Headline 0xa68f7c with the short names.
/// Original: part of `NewsItem::write_text` (0x77c473..0x77c846).
unsafe fn news_match_televised(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "match_televised";
    let f = news_format();
    let (c1, c2) = (news_club(read_u32(news + 5)), news_club(read_u32(news + 9)));
    if body == 0 {
        f(NEWS_TEXT_BUFFER, 0xa68f7c, news_g(c1 + club::SHORT_GENDER_NAME), news_g(c1 + club::SHORT_GENDER_NAME),
          news_g(c2 + club::SHORT_GENDER_NAME), news_g(c2 + club::SHORT_GENDER_NAME), c1 + club::SHORT_NAME, c2 + club::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
        news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
        return 1;
    }
    covered!("gra_1 match televised body");
    let comp = news_comp(read_u32(news + 0xd));
    let (stage, round) = (read_u16(news + 0x11) as u32, read_u16(news + 0x15) as u32);
    let mut label = [0u8; 0xc8];
    let lb = label.as_mut_ptr() as usize;
    exe::comp_util_format_comp_stage_round_label(comp, stage, round, 2, label.as_mut_ptr(), 0xc8);
    if news_streq(lb, 0x9db958) {
        label[0] = b'f';
    } else if news_streq(lb, 0x9b5b40) {
        label[0] = b'r';
    }
    let lg = exe::comp_util_get_label_gender(comp as u32, stage, round, 2) as i8 as i32;
    let (g1, g2) = (news_g(c1 + club::GENDER_NAME), news_g(c2 + club::GENDER_NAME));
    let (old_time, new_time) = (read_u8(news + 0x21), read_u8(news + 0x2d));
    if read_u16(news + 0x25) != read_u16(news + 0x19) {
        covered!("gra_1 match televised new day");
        let mut d = [0u8; 8];
        let mut s = [0u8; 0xc8];
        news_date(d.as_mut_ptr() as usize, read_u16(news + 0x25), read_u16(news + 0x29));
        exe::date_get_long_date_string_no_year(d.as_ptr() as usize, s.as_mut_ptr() as usize, 0xc8, 0xff);
        f(NEWS_TEXT_BUFFER, 0xa690f0, g1, g1, g2, g2, lg, lg, 0x64, c1 + club::NAME, c2 + club::NAME, lb, s.as_ptr() as usize);
        news_set(out, NEWS_TEXT_BUFFER);
    } else {
        if old_time != new_time {
            covered!("gra_1 match televised time of day");
            let name = exe::date_get_time_of_day_name(new_time as u32, 0);
            let tg = exe::date_get_time_of_day_gender(new_time as u32) as i32;
            f(NEWS_TEXT_BUFFER, 0xa69044, g1, g1, g2, g2, lg, lg, tg, c1 + club::NAME, c2 + club::NAME, lb, name);
        } else {
            f(NEWS_TEXT_BUFFER, 0xa68fc0, g1, g1, g2, g2, lg, lg, c1 + club::NAME, c2 + club::NAME, lb);
        }
        news_set(out, NEWS_TEXT_BUFFER);
    }
    news_link(out, c1 + club::NAME, 0x7d5, c1);
    news_link(out, c2 + club::NAME, 0x7d5, c2);
    news_link(out, lb, 0x7d0, comp);
    1
}

/// Type 4, the bids to host a competition. Slots: +5 the competition, +9 the year (word), +0xd, +0x15 and +0x1d three
/// bidding nations and +0x11 the joint bidder of the first one (−1 none).
///
/// Body: first the nations' links (0x7d5 on the nation's name, to its national team, `exe::database_get_national_team`):
/// the first and its joint bidder, or each of the three that is set; then 0xa68ee0 (three nations) or 0xa68e1c (with
/// the joint bid: four), with the genders, the names, the year and the competition's name; the competition's link
/// (0x7d0). Headline 0xa68de0 with the year and the competition's short name.
/// Original: part of `NewsItem::write_text` (0x77c84f..0x77cb6c).
unsafe fn news_host_bids(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "host_bids";
    let f = news_format();
    let comp = news_comp(read_u32(news + 5));
    let year = read_u16(news + 9) as u32;
    if body == 0 {
        let g = news_g(comp + competition::SHORT_GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa68de0, g, g, year, comp + competition::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 host bids body");
    let n = [read_i32(news + 0xd), read_i32(news + 0x11), read_i32(news + 0x15), read_i32(news + 0x19), read_i32(news + 0x1d)];
    let team_link = |i: i32| {
        let nat = news_nation(i as u32);
        news_link(out, nat + 4, 0x7d5, exe::database_get_national_team(nat));
    };
    for k in 0..3 {
        if k == 0 && n[0] > -1 && n[1] > -1 {
            team_link(n[0]);
            team_link(n[1]);
        } else if n[2 * k] > -1 {
            team_link(n[2 * k]);
        }
    }
    let gc = news_g(comp + competition::GENDER_NAME);
    let nat = |i: i32| news_nation(i as u32);
    if n[1] == -1 {
        let (a, b, c) = (nat(n[0]), nat(n[2]), nat(n[4]));
        f(NEWS_TEXT_BUFFER, 0xa68ee0, news_g(a + 0x37), news_g(a + 0x37), news_g(b + 0x37), news_g(b + 0x37), news_g(c + 0x37),
          news_g(c + 0x37), gc, gc, a + 4, b + 4, c + 4, year, comp + competition::NAME);
    } else {
        covered!("gra_1 host bids joint");
        let (a, j, b, c) = (nat(n[0]), nat(n[1]), nat(n[2]), nat(n[4]));
        f(NEWS_TEXT_BUFFER, 0xa68e1c, news_g(a + 0x37), news_g(a + 0x37), news_g(j + 0x37), news_g(j + 0x37), news_g(b + 0x37),
          news_g(b + 0x37), news_g(c + 0x37), news_g(c + 0x37), gc, gc, a + 4, j + 4, b + 4, c + 4, year, comp + competition::NAME);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, comp + competition::NAME, 0x7d0, comp);
    1
}

/// Type 5, the host of a competition chosen. Slots: +5 the competition, +9 the year (word), +0xd the host nation (−1
/// none) and +0x11 its co-host (−1 none).
///
/// Body: with a host, 0xa68d60 (co-hosts) or 0xa68d00 (one host) with the genders, the names, the year and the
/// competition's name, the text, and each host's link (0x7d5, to its national team); without a host no text; then
/// the competition's link (0x7d0). Headline 0xa68ccc with the year and the competition's short name.
/// Original: part of `NewsItem::write_text` (0x77cb75..0x77cd8e).
unsafe fn news_host_chosen(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "host_chosen";
    let f = news_format();
    let comp = news_comp(read_u32(news + 5));
    let year = read_u16(news + 9) as u32;
    if body == 0 {
        let g = news_g(comp + competition::SHORT_GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa68ccc, g, g, year, comp + competition::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 host chosen body");
    let (h, co) = (read_i32(news + 0xd), read_i32(news + 0x11));
    let gc = news_g(comp + competition::GENDER_NAME);
    let team_link = |nat: usize| news_link(out, nat + 4, 0x7d5, exe::database_get_national_team(nat));
    if h > -1 {
        let a = news_nation(h as u32);
        if co > -1 {
            covered!("gra_1 host chosen co-hosts");
            let b = news_nation(co as u32);
            f(NEWS_TEXT_BUFFER, 0xa68d60, news_g(a + 0x37), news_g(a + 0x37), news_g(b + 0x37), news_g(b + 0x37), gc, gc, a + 4, b + 4,
              year, comp + competition::NAME);
            news_set(out, NEWS_TEXT_BUFFER);
            team_link(a);
            team_link(b);
        } else {
            f(NEWS_TEXT_BUFFER, 0xa68d00, news_g(a + 0x37), news_g(a + 0x37), gc, gc, a + 4, year, comp + competition::NAME);
            news_set(out, NEWS_TEXT_BUFFER);
            team_link(a);
        }
    }
    news_link(out, comp + competition::NAME, 0x7d0, comp);
    1
}

/// The friendly invitation's answer text writer (`friendly_check_match_invitation`, 0x5ca090, thiscall on
/// `G_FRIENDLY` 0xae2c74: the inviting club, the invited, the club asked, the date's two words, the match, the buffer
/// and its size 0x190; nonzero when it wrote the whole answer).
unsafe fn news_friendly_answer(a: usize, b: usize, c: usize, w1: u32, w2: u32, m: usize, buf: usize) -> u32 {
    let w: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, usize, usize, u32) -> u32 = core::mem::transmute(0x5ca090usize);
    w(read_usize(0xae2c74), a, b, c, w1, w2, m, buf, 0x190)
}

/// `strcpy(dst, src)`.
unsafe fn news_strcpy(dst: usize, src: usize) {
    let mut k = 0;
    loop {
        let c = read_u8(src + k);
        write_u8(dst + k, c);
        if c == 0 {
            break;
        }
        k += 1;
    }
}

/// Type 6, the friendly proposal. Slots: +5 and +9 the two clubs, +0xd the host club, +0x11 and +0x15 the date
/// (words), +0x19 the answer (3 accepted, 4 rejected).
///
/// Body: the date (`Date::Date`, assigned with `exe::date_operator_3d`), the host's friendly match on that date
/// (`exe::friendly_find_friendly_match`) and the date's long text. When the first club is the host it "offered" the
/// match (0xa68c38 at the first club's stadium, whose name is copied with `sprintf` used as format, or 0xa68bc0
/// without a stadium: the first club then the second); otherwise the first club "challenged" the second (0xa68aac or
/// 0xa68a2c, the second club then the first; the stadium is still the first club's). The answer paragraph: accepted
/// 0xa68b84 or rejected 0xa68b48 with the answering club (the second club in the offer, the first in the challenge),
/// joined as "%s\n\n%s" in the buffer; any other answer by `friendly_check_match_invitation` (the first paragraph alone
/// when it returns nonzero, otherwise joined with what it left in the second). Links: the two clubs' short names
/// (0x7d5). Headline 0xa689d4 with the short names and their links.
/// Original: part of `NewsItem::write_text` (0x77cd97..0x77d2e7).
unsafe fn news_friendly_proposal(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "friendly_proposal";
    let f = news_format();
    let sprintf = news_sprintf();
    let (mut d, mut d2, mut tmp) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let dp = d.as_mut_ptr() as usize;
    exe::date_date_54a110(dp);
    let host = read_u32(news + 0xd);
    let (w1, w2) = (read_u16(news + 0x11), read_u16(news + 0x15));
    let (i1, i2) = (read_u32(news + 5), read_u32(news + 9));
    let answer = read_u8(news + 0x19);
    news_date(d2.as_mut_ptr() as usize, w1, w2);
    exe::date_operator_3d(dp, tmp.as_mut_ptr() as usize, d2.as_ptr() as usize);
    let (c1, c2) = (news_club(i1), news_club(i2));
    let sg = |c: usize| news_g(c + club::SHORT_GENDER_NAME);
    if body == 0 {
        f(NEWS_TEXT_BUFFER, 0xa689d4, sg(c1), sg(c1), sg(c2), sg(c2), c1 + club::SHORT_NAME, c2 + club::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
        news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
        return 1;
    }
    covered!("gra_1 friendly proposal body");
    let (mut first, mut second, mut stadium_name, mut date_text) = ([0u8; 0x400], [0u8; 0x190], [0u8; 0x12c], [0u8; 0x194]);
    let (p1, p2, ps, pd) = (first.as_mut_ptr() as usize, second.as_mut_ptr() as usize, stadium_name.as_mut_ptr() as usize, date_text.as_mut_ptr() as usize);
    let m = exe::friendly_find_friendly_match(read_usize(0xae2c74), dp, news_club(host));
    exe::date_get_long_date_string(dp, pd, 0x64, 0xff);
    let offer = i1 == host;
    let (lead, other) = if offer { (c1, c2) } else { (c2, c1) };
    let stadium = read_usize(c1 + club::STADIUM);
    let (with_stadium, without) = if offer { (0xa68c38, 0xa68bc0) } else { (0xa68aac, 0xa68a2c) };
    if stadium != 0 {
        sprintf(ps, stadium + 4);
        let gs = news_g(stadium + 0x37);
        f(p1, with_stadium, sg(lead), sg(lead), sg(other), sg(other), gs, gs, 0x64, lead + club::SHORT_NAME, other + club::SHORT_NAME, ps, pd);
    } else {
        f(p1, without, sg(lead), sg(lead), sg(other), sg(other), 0x64, lead + club::SHORT_NAME, other + club::SHORT_NAME, pd);
    }
    let answering = if offer { c2 } else { c1 };
    let joined = match answer {
        3 => {
            f(p2, 0xa68b84, sg(answering), sg(answering), answering + club::SHORT_NAME);
            true
        }
        4 => {
            f(p2, 0xa68b48, sg(answering), sg(answering), answering + club::SHORT_NAME);
            true
        }
        _ => {
            covered!("gra_1 friendly proposal answer");
            news_friendly_answer(c1, c2, answering, w1 as u32, w2 as u32, m, p2) == 0
        }
    };
    if joined {
        sprintf(NEWS_TEXT_BUFFER, 0x991948, p1, p2); // "%s\n\n%s"
    } else {
        news_strcpy(NEWS_TEXT_BUFFER, p1);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
    news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
    1
}

/// The date of two words of a news, as the original builds it: a default `Date` (`exe::date_date_54a110`) assigned
/// (`exe::date_operator_3d`) from `Date::Date(a, b)`.
unsafe fn news_assigned_date(d: usize, a: u16, b: u16) {
    let (mut d2, mut tmp) = ([0u8; 8], [0u8; 8]);
    exe::date_date_54a110(d);
    news_date(d2.as_mut_ptr() as usize, a, b);
    exe::date_operator_3d(d, tmp.as_mut_ptr() as usize, d2.as_ptr() as usize);
}

/// Type 7, the answer to a friendly offer. Slots: +5 and +9 the two clubs, +0xd the club that made the offer,
/// +0x11 and +0x15 the date (words), +0x19 the answer (3 accepted, 4 rejected). The answering club is the second
/// when the first made the offer, otherwise the first.
///
/// Body: the date's long text (`exe::date_get_long_date_string`); accepted 0xa68948 or rejected 0xa688bc with the
/// answering club and the offering club (genders, names) and the date; any other answer leaves the buffer as it was.
/// Links: the first and second clubs' names (0x7d5). Headline: accepted 0xa68880 or rejected 0xa68844 with the
/// answering club's short name, and its link.
/// Original: part of `NewsItem::write_text` (0x77d2f0..0x77d4f3).
unsafe fn news_friendly_answered(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "friendly_answered";
    let f = news_format();
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    let (i1, i2, offering) = (read_u32(news + 5), read_u32(news + 9), read_u32(news + 0xd));
    let answer = read_u8(news + 0x19);
    let answering = news_club(if offering == i1 { i2 } else { i1 });
    news_assigned_date(dp, read_u16(news + 0x11), read_u16(news + 0x15));
    if body == 0 {
        let g = news_g(answering + club::SHORT_GENDER_NAME);
        match answer {
            3 => {
                f(NEWS_TEXT_BUFFER, 0xa68880, g, g, answering + club::SHORT_NAME);
            }
            4 => {
                f(NEWS_TEXT_BUFFER, 0xa68844, g, g, answering + club::SHORT_NAME);
            }
            _ => {}
        }
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, answering + club::SHORT_NAME, 0x7d5, answering);
        return 1;
    }
    covered!("gra_1 friendly answered body");
    let mut s = [0u8; 0x194];
    let sp = s.as_mut_ptr() as usize;
    exe::date_get_long_date_string(dp, sp, 0x64, 0xff);
    let o = news_club(offering);
    let (ga, go) = (news_g(answering + club::GENDER_NAME), news_g(o + club::GENDER_NAME));
    match answer {
        3 => {
            f(NEWS_TEXT_BUFFER, 0xa68948, ga, ga, go, go, 0x64, answering + club::NAME, o + club::NAME, sp);
        }
        4 => {
            f(NEWS_TEXT_BUFFER, 0xa688bc, ga, ga, go, go, 0x64, answering + club::NAME, o + club::NAME, sp);
        }
        _ => {}
    }
    news_set(out, NEWS_TEXT_BUFFER);
    let (c1, c2) = (news_club(i1), news_club(i2));
    news_link(out, c1 + club::NAME, 0x7d5, c1);
    news_link(out, c2 + club::NAME, 0x7d5, c2);
    1
}

/// Type 8, the friendly cancelled. Slots: +5 and +9 the two clubs, +0xd and +0x11 the date (words).
///
/// Body 0xa6878c with the clubs' short genders and short names and the date's long text; headline 0xa68734 with the
/// short names. Links: the two clubs' short names (0x7d5).
/// Original: part of `NewsItem::write_text` (0x77d4fc..0x77d625).
unsafe fn news_friendly_cancelled(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "friendly_cancelled";
    let f = news_format();
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    let (c1, c2) = (news_club(read_u32(news + 5)), news_club(read_u32(news + 9)));
    news_assigned_date(dp, read_u16(news + 0xd), read_u16(news + 0x11));
    let (g1, g2) = (news_g(c1 + club::SHORT_GENDER_NAME), news_g(c2 + club::SHORT_GENDER_NAME));
    if body != 0 {
        covered!("gra_1 friendly cancelled body");
        let mut s = [0u8; 0x194];
        let sp = s.as_mut_ptr() as usize;
        exe::date_get_long_date_string(dp, sp, 0x64, 0xff);
        f(NEWS_TEXT_BUFFER, 0xa6878c, g1, g1, g2, g2, 0x64, c1 + club::SHORT_NAME, c2 + club::SHORT_NAME, sp);
    } else {
        f(NEWS_TEXT_BUFFER, 0xa68734, g1, g1, g2, g2, c1 + club::SHORT_NAME, c2 + club::SHORT_NAME);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
    news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
    1
}

/// Type 9, the invitation to a friendly tournament. Slots: +5 the inviting club, +9 the invited club, +0xd the
/// tournament's index (byte), +0x11 the answer (2 none yet, 3 accepted, 4 declined), +0x19 and +0x1d the date (words).
///
/// Body: the date (assigned as in the friendlies) and the tournament on it (`friendly_find_tournament`, 0x5c89a0,
/// thiscall on `G_FRIENDLY`: the index byte and the date). With the tournament: 0xa68698 with the clubs' short names,
/// the tournament's name (+0x31) and its start date's long text (+4); accepted 0xa68658 or declined 0xa68618 with the
/// invited club's short name, joined as "%s\n\n%s"; any other answer by `friendly_tournament_answer` (0x5ca290: the
/// invited club, the tournament, the buffer, 0x190; the first paragraph alone when it returns nonzero). Links: both
/// clubs' short names (0x7d5) and the tournament's name (0x7d1). Without it: 0xa6858c with the news date's long
/// text; accepted or declined with the invited club's full name (and its short gender); no answer yet (2) the first
/// paragraph alone; links of both clubs. Headline 0xa6854c with the inviting club's short name, and its link.
///
/// Defect of the original: without the tournament, an answer other than 2, 3 or 4 joins the first paragraph with a
/// buffer never written (stack leftovers); the copy joins it with an empty text.
/// Original: part of `NewsItem::write_text` (0x77d62e..0x77da1a).
unsafe fn news_tournament_invitation(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "tournament_invitation";
    let f = news_format();
    let sprintf = news_sprintf();
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    let (c1, c2) = (news_club(read_u32(news + 5)), news_club(read_u32(news + 9)));
    let index = read_u8(news + 0xd);
    let answer = read_u8(news + 0x11);
    let sg = |c: usize| news_g(c + club::SHORT_GENDER_NAME);
    if body == 0 {
        exe::date_date_54a110(dp);
        f(NEWS_TEXT_BUFFER, 0xa6854c, sg(c1), sg(c1), c1 + club::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
        return 1;
    }
    covered!("gra_1 tournament invitation body");
    news_assigned_date(dp, read_u16(news + 0x19), read_u16(news + 0x1d));
    let find: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x5c89a0usize);
    let tour = find(read_usize(0xae2c74), index as u32, dp);
    let (mut first, mut second, mut date_text) = ([0u8; 0x400], [0u8; 0x190], [0u8; 0x194]);
    let (p1, p2, pd) = (first.as_mut_ptr() as usize, second.as_mut_ptr() as usize, date_text.as_mut_ptr() as usize);
    let joined;
    if tour != 0 {
        exe::date_get_long_date_string(tour + 4, pd, 0x64, 0xff);
        let tname = tour + 0x31;
        f(p1, 0xa68698, sg(c1), sg(c1), sg(c2), sg(c2), 0x64, c1 + club::SHORT_NAME, c2 + club::SHORT_NAME, tname, pd);
        joined = match answer {
            3 => {
                f(p2, 0xa68658, sg(c2), sg(c2), c2 + club::SHORT_NAME);
                true
            }
            4 => {
                f(p2, 0xa68618, sg(c2), sg(c2), c2 + club::SHORT_NAME);
                true
            }
            _ => {
                covered!("gra_1 tournament invitation answer");
                let w: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(0x5ca290usize);
                w(read_usize(0xae2c74), c2, tour, p2, 0x190) == 0
            }
        };
    } else {
        covered!("gra_1 tournament invitation no tournament");
        exe::date_get_long_date_string(dp, pd, 0x64, 0xff);
        f(p1, 0xa6858c, sg(c1), sg(c1), sg(c2), sg(c2), 0x64, c1 + club::SHORT_NAME, c2 + club::SHORT_NAME, pd);
        joined = match answer {
            3 => {
                f(p2, 0xa68658, sg(c2), sg(c2), c2 + club::NAME);
                true
            }
            4 => {
                f(p2, 0xa68618, sg(c2), sg(c2), c2 + club::NAME);
                true
            }
            2 => false,
            _ => true,
        };
    }
    if joined {
        sprintf(NEWS_TEXT_BUFFER, 0x991948, p1, p2); // "%s\n\n%s"
    } else {
        news_strcpy(NEWS_TEXT_BUFFER, p1);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c1 + club::SHORT_NAME, 0x7d5, c1);
    news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
    if tour != 0 {
        news_link(out, tour + 0x31, 0x7d1, tour);
    }
    1
}

/// Type 10, the answer to a friendly tournament invitation. Slots: +5 the inviting club, +9 the invited club, +0xd
/// and +0x11 the date (words), +0x15 the answer (3 accepted, 4 declined), +0x19 the tournament's index (byte).
///
/// Body: the tournament on the date (as in type 9) and the date's text (`Date::get_date_string`, 0x54c3b0, 0x64,
/// 0xff). With the tournament: accepted 0xa6848c or declined 0xa683cc (the invited club, then the inviting club, their
/// genders and names, the tournament's name and the date), links of both clubs' names (0x7d5, invited first) and of
/// the tournament (0x7d1); without it 0xa68328 or 0xa68284 and the two clubs' links. Any other answer leaves the buffer
/// as it was. Headline: accepted 0xa6823c or rejected 0xa681f4 with the invited club's short name, and its link.
/// Original: part of `NewsItem::write_text` (0x77da23..0x77dd7d).
unsafe fn news_tournament_answered(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "tournament_answered";
    let f = news_format();
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    let (c1, c2) = (news_club(read_u32(news + 5)), news_club(read_u32(news + 9)));
    let answer = read_u8(news + 0x15);
    let index = read_u8(news + 0x19);
    news_assigned_date(dp, read_u16(news + 0xd), read_u16(news + 0x11));
    if body == 0 {
        let g = news_g(c2 + club::SHORT_GENDER_NAME);
        match answer {
            3 => {
                f(NEWS_TEXT_BUFFER, 0xa6823c, g, g, c2 + club::SHORT_NAME);
            }
            4 => {
                f(NEWS_TEXT_BUFFER, 0xa681f4, g, g, c2 + club::SHORT_NAME);
            }
            _ => {}
        }
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c2 + club::SHORT_NAME, 0x7d5, c2);
        return 1;
    }
    covered!("gra_1 tournament answered body");
    let find: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x5c89a0usize);
    let tour = find(read_usize(0xae2c74), index as u32, dp);
    let mut s = [0u8; 0x194];
    let sp = s.as_mut_ptr() as usize;
    let date_string: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x54c3b0usize);
    date_string(dp, sp, 0x64, 0xff);
    let (g1, g2) = (news_g(c1 + club::GENDER_NAME), news_g(c2 + club::GENDER_NAME));
    if tour != 0 {
        let tname = tour + 0x31;
        match answer {
            3 => {
                f(NEWS_TEXT_BUFFER, 0xa6848c, g2, g2, g1, g1, 0x64, c2 + club::NAME, c1 + club::NAME, tname, sp);
            }
            4 => {
                f(NEWS_TEXT_BUFFER, 0xa683cc, g2, g2, g1, g1, 0x64, c2 + club::NAME, c1 + club::NAME, tname, sp);
            }
            _ => {}
        }
    } else {
        covered!("gra_1 tournament answered no tournament");
        match answer {
            3 => {
                f(NEWS_TEXT_BUFFER, 0xa68328, g2, g2, g1, g1, 0x64, c2 + club::NAME, c1 + club::NAME, sp);
            }
            4 => {
                f(NEWS_TEXT_BUFFER, 0xa68284, g2, g2, g1, g1, 0x64, c2 + club::NAME, c1 + club::NAME, sp);
            }
            _ => {}
        }
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c2 + club::NAME, 0x7d5, c2);
    news_link(out, c1 + club::NAME, 0x7d5, c1);
    if tour != 0 {
        news_link(out, tour + 0x31, 0x7d1, tour);
    }
    1
}

/// Type 11, the friendly tournament cancelled. Slots: +5 the organising club, +9 and +0xd the date (words).
///
/// Body 0xa68174 with the club's short name and the date's long text; headline 0xa68130 with the short name. Link:
/// the club's short name (0x7d5).
/// Original: part of `NewsItem::write_text` (0x77dd86..0x77de79).
unsafe fn news_tournament_cancelled(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "tournament_cancelled";
    let f = news_format();
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    let c = news_club(read_u32(news + 5));
    news_assigned_date(dp, read_u16(news + 9), read_u16(news + 0xd));
    let g = news_g(c + club::SHORT_GENDER_NAME);
    if body != 0 {
        covered!("gra_1 tournament cancelled body");
        let mut s = [0u8; 0x194];
        let sp = s.as_mut_ptr() as usize;
        exe::date_get_long_date_string(dp, sp, 0x64, 0xff);
        f(NEWS_TEXT_BUFFER, 0xa68174, g, g, 0x64, c + club::SHORT_NAME, sp);
    } else {
        f(NEWS_TEXT_BUFFER, 0xa68130, g, g, c + club::SHORT_NAME);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c + club::SHORT_NAME, 0x7d5, c);
    1
}

/// Type 12, the coach's report. Slots: +5 a club, +9 the coach (index of the coach table 0xadadfc), +0xd and +0x11
/// two bytes, +0x15 a word, +0x19 a dword. Without the coach's record the news.cpp error box (line 0x61f) and 0;
/// otherwise the coach's virtual text writer (slot +0x48 of its table: the club, the byte +0x11, the byte +0xd, the
/// word, 0, 0, the dword, 0, `body`, `out`) and 1.
/// Original: part of `NewsItem::write_text` (0x77de82..0x77df0b).
unsafe fn news_comp_report(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "comp_report";
    let coach = read_usize(read_usize(0xadadfc).wrapping_add((read_u32(news + 9) as usize).wrapping_mul(4)));
    if coach == 0 {
        news_error_box(0x61f);
        return 0;
    }
    covered!("gra_1 comp report");
    let w: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> u32 =
        core::mem::transmute(read_usize(read_usize(coach) + 0x48));
    w(coach, read_u32(news + 5), read_u8(news + 0x11) as u32, read_u8(news + 0xd) as u32, read_u16(news + 0x15) as u32, 0, 0,
      read_u32(news + 0x19), 0, body, out);
    1
}

/// Type 13, the league's fixture list decided. Slots: +5 the competition, +9 the club, +0xd the season's year (word);
/// the club's first three games at +0x11 + 16·k (k = 0..2): the date's two words (+0x11, +0x15), the opponent
/// (+0x19) and home (+0x1d, word, nonzero at home).
///
/// The season's name: "<year>/<(year+1) % 100>" (0xa680e0) when the key nation of the competition
/// (`exe::key_nation_index_of_comp`) has a season over two years (the dword at 0xb63d77 + nation·0x48), otherwise
/// "<year>" (0xa680a4), with the competition's short name. Headline 0xa67f20 with the season's name and the
/// competition's link (0x7d0). Body: each game "<opponent> (home)" (0xa68078) or "(away)" (0xa6804c) with the
/// opponent's short name, and its date's long text (`Date::Date` assigned, `exe::date_get_long_date_string`, 0x32);
/// then 0xa67f68 with the club's short name, the three games and the three dates; links: the competition's short name
/// (0x7d0), the club's and the three opponents' short names (0x7d5).
/// Original: part of `NewsItem::write_text` (0x77df14..0x77e2e2).
unsafe fn news_fixture_list(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "fixture_list";
    let f = news_format();
    let comp = news_comp(read_u32(news + 5));
    let year = read_u16(news + 0xd) as i32;
    let nation = exe::key_nation_index_of_comp(comp) as i8 as i32;
    let mut season = [0u8; 0xc8];
    let sp = season.as_mut_ptr() as usize;
    let gs = news_g(comp + competition::SHORT_GENDER_NAME);
    if read_u32((0xb63d77usize).wrapping_add((nation as usize).wrapping_mul(0x48))) != 0 {
        f(sp, 0xa680e0, gs, gs, year, (year + 1) % 100, comp + competition::SHORT_NAME);
    } else {
        f(sp, 0xa680a4, gs, gs, year, comp + competition::SHORT_NAME);
    }
    if body == 0 {
        f(NEWS_TEXT_BUFFER, 0xa67f20, gs, gs, sp);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 fixture list body");
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    exe::date_date_54a110(dp);
    let c = news_club(read_u32(news + 9));
    let (mut games, mut dates) = ([0u8; 0x3d * 3], [0u8; 0x32 * 3]);
    let (gp, dtp) = (games.as_mut_ptr() as usize, dates.as_mut_ptr() as usize);
    let mut opp = [0usize; 3];
    for k in 0..3 {
        let base = news + 0x11 + 16 * k;
        let (w1, w2) = (read_u16(base), read_u16(base + 4));
        let o = news_club(read_u32(base + 8));
        let home = read_u16(base + 0xc) != 0;
        opp[k] = o;
        let g = news_g(o + club::SHORT_GENDER_NAME);
        f(gp + 0x3d * k, if home { 0xa68078 } else { 0xa6804c }, g, g, o + club::SHORT_NAME);
        let (mut d2, mut tmp) = ([0u8; 8], [0u8; 8]);
        news_date(d2.as_mut_ptr() as usize, w1, w2);
        exe::date_operator_3d(dp, tmp.as_mut_ptr() as usize, d2.as_ptr() as usize);
        exe::date_get_long_date_string(dp, dtp + 0x32 * k, 0x32, 0xff);
    }
    let sg = |x: usize| news_g(x + club::SHORT_GENDER_NAME);
    f(NEWS_TEXT_BUFFER, 0xa67f68, sg(c), sg(c), sg(opp[0]), sg(opp[1]), sg(opp[2]), 0x64, 0x64, 0x64, c + club::SHORT_NAME, gp,
      gp + 0x3d, gp + 0x7a, dtp, dtp + 0x32, dtp + 0x64);
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
    news_link(out, c + club::SHORT_NAME, 0x7d5, c);
    for o in opp {
        news_link(out, o + club::SHORT_NAME, 0x7d5, o);
    }
    1
}

/// Type 16, the coach's match report. Slots: +5 a club, +9 the coach (index of the coach table 0xadadfc), +0xd and
/// +0x11 two bytes, +0x15 a dword, and the fixture from slot 5 on (`comp_util_read_fixture_from_news_slots`, 0x4b0010,
/// cdecl: the news, the fixture, the slot cursor). Without the coach's record 0 (no box); otherwise the coach's virtual
/// text writer (slot +0x48: the club, the byte +0xd, the fixture's dwords +0x42 and +0x32, the fixture, the word of
/// the slot after the fixture, the dword +0x15, the byte +0x11, `body`, `out`) and 1.
/// Original: part of `NewsItem::write_text` (0x77e2eb..0x77e37d).
unsafe fn news_comp_match_report(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "comp_match_report";
    let (b1, b2, d, c) = (read_u8(news + 0xd), read_u8(news + 0x11), read_u32(news + 0x15), read_u32(news + 5));
    let mut fixture = [0u8; 0xc8];
    let fp = fixture.as_mut_ptr() as usize;
    let mut k: u8 = 5;
    let read: unsafe extern "C" fn(usize, usize, *mut u8) -> u32 = core::mem::transmute(0x4b0010usize);
    read(news, fp, &mut k);
    let coach = read_usize(read_usize(0xadadfc).wrapping_add((read_u32(news + 9) as usize).wrapping_mul(4)));
    if coach == 0 {
        return 0;
    }
    covered!("gra_1 comp match report");
    let slot = read_u16(news.wrapping_add(5).wrapping_add((k as i8 as isize as usize).wrapping_mul(4))) as u32;
    let w: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize) -> u32 =
        core::mem::transmute(read_usize(read_usize(coach) + 0x48));
    w(coach, c, b1 as u32, read_u32(fp + 0x42), read_u32(fp + 0x32), fp, slot, d, b2 as u32, body, out);
    1
}

/// Types 17 and 18, a play-off place won by the form over a third or a quarter of the season. Slots: +5 the club,
/// +9 the part (1, 2, 3...), +0xd the competition.
///
/// Body: by the part, the third (17: 0xa67e88 first, 0xa67df0 middle, 0xa67d58 last) or the quarter (18: 0xa67c98,
/// 0xa67bfc, 0xa67b64, 0xa67acc), with the club's and the competition's genders and names; links: the club's name
/// (0x7d5) and the competition's (0x7d0). Headline (both) 0xa67d30 with the club's short name, and its link.
/// Original: part of `NewsItem::write_text` (0x77e386..0x77e56f).
unsafe fn news_playoff_place(news: usize, body: u32, out: usize, quarters: bool) -> u32 {
    NEWS_LAST_PIECE = if quarters { "playoff_place_quarter" } else { "playoff_place_third" };
    let f = news_format();
    let c = news_club(read_u32(news + 5));
    let part = read_u8(news + 9);
    let comp = news_comp(read_u32(news + 0xd));
    if body == 0 {
        let g = news_g(c + club::SHORT_GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa67d30, g, g, c + club::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c + club::SHORT_NAME, 0x7d5, c);
        return 1;
    }
    covered!("gra_1 playoff place body");
    let fmt = if quarters {
        match part {
            1 => 0xa67c98,
            2 => 0xa67bfc,
            3 => 0xa67b64,
            _ => 0xa67acc,
        }
    } else {
        match part {
            1 => 0xa67e88,
            2 => 0xa67df0,
            _ => 0xa67d58,
        }
    };
    let (gc, gk) = (news_g(c + club::GENDER_NAME), news_g(comp + competition::GENDER_NAME));
    f(NEWS_TEXT_BUFFER, fmt, gc, gc, gk, gk, c + club::NAME, comp + competition::NAME);
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c + club::NAME, 0x7d5, c);
    news_link(out, comp + competition::NAME, 0x7d0, comp);
    1
}

/// Type 19, the nation's leagues reshuffled. Slots: +5 the club, +9 the competition. Without the club's stadium, the
/// stadium's city (+0x38), the competition or its nation (`competition::NATION`), the text "League reshuffled error
/// message" alone.
///
/// Body 0xa67a2c with the club's name, the competition's short name, the city's name and the nation's name (and their
/// genders, `city::GENDER_NAME` for the city); links: the club (0x7d5), the competition's short name (0x7d0) and the
/// nation's name to its national team (0x7d5, `exe::database_get_national_team`). Headline 0xa67a00 with the nation's
/// short name, and its link.
/// Original: part of `NewsItem::write_text` (0x77e578..0x77e6f4).
unsafe fn news_leagues_reshuffled(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "leagues_reshuffled";
    let f = news_format();
    let c = news_club(read_u32(news + 5));
    let comp = news_comp(read_u32(news + 9));
    let stadium = if c != 0 { read_usize(c + club::STADIUM) } else { 0 };
    let city = if stadium != 0 { read_usize(stadium + 0x38) } else { 0 };
    let nation = if city != 0 && comp != 0 { read_usize(comp + competition::NATION) } else { 0 };
    if nation == 0 {
        news_set(out, 0xa679e0); // "League reshuffled error message"
        return 1;
    }
    if body == 0 {
        let g = news_g(nation + 0x52);
        f(NEWS_TEXT_BUFFER, 0xa67a00, g, g, nation + 0x38);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, nation + 0x38, 0x7d5, exe::database_get_national_team(nation));
        return 1;
    }
    covered!("gra_1 leagues reshuffled body");
    let (gc, gk, gt, gn) = (news_g(c + club::GENDER_NAME), news_g(comp + competition::SHORT_GENDER_NAME), news_g(city + city::GENDER_NAME), news_g(nation + 0x37));
    f(NEWS_TEXT_BUFFER, 0xa67a2c, gc, gc, gk, gk, gt, gt, gn, gn, c + club::NAME, comp + competition::SHORT_NAME, city + 4, nation + 4);
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c + club::NAME, 0x7d5, c);
    news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
    news_link(out, nation + 4, 0x7d5, exe::database_get_national_team(nation));
    1
}

/// The round's label of a competition in `buf` (0xc8 bytes, `exe::comp_util_format_comp_stage_round_label` with the
/// form; "Friendly" and "Reserve" get a lower-case first letter) and its gender (`exe::comp_util_get_label_gender`).
unsafe fn news_round_label(comp: usize, stage: u16, round: u16, form: u32, buf: &mut [u8; 0xc8]) -> i32 {
    let lb = buf.as_mut_ptr() as usize;
    exe::comp_util_format_comp_stage_round_label(comp, stage as u32, round as u32, form, buf.as_mut_ptr(), 0xc8);
    if news_streq(lb, 0x9db958) {
        buf[0] = b'f'; // "Friendly"
    } else if news_streq(lb, 0x9b5b40) {
        buf[0] = b'r'; // "Reserve"
    }
    exe::comp_util_get_label_gender(comp as u32, stage as u32, round as u32, form) as i8 as i32
}

/// Type 20, the cup draw. Slots: +5 the competition, +9 and +0xd the stage and round (words), +0x11 the drawn club,
/// +0x15 its opponent, +0x19 nonzero for two legs, +0x1d the stadium (−1 unknown), +0x21 and +0x25 the date (words).
///
/// Body: 0xa67958 with the drawn club and the opponent (short names and genders) and the round's label (form 2), its
/// text and links (the two clubs 0x7d5, the competition on the label 0x7d0); then a second paragraph appended
/// (`exe::text_append_string`): with the stadium, its name — with the city's name when it differs and is not
/// "Unknown": "%s (%s)" (0x9a3d8c), or "%s, %s" (0x98f6ac) in languages 0 and 1; the city's name alone when they are
/// equal or the stadium is "Unknown"; the stadium's when it has no city — and 0xa678cc (first leg) or 0xa67854 (the
/// game) with the date's long text; without the stadium 0xa677f8 or 0xa677a8 with the date. Headline 0xa6776c with the
/// round's label (form 3), and its link.
/// Original: part of `NewsItem::write_text` (0x77e6fd..0x77ed30).
unsafe fn news_cup_draw(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "cup_draw";
    let f = news_format();
    let sprintf = news_sprintf();
    let comp = news_comp(read_u32(news + 5));
    let (stage, round) = (read_u16(news + 9), read_u16(news + 0xd));
    let mut label = [0u8; 0xc8];
    let lb = label.as_ptr() as usize;
    if body == 0 {
        let lg = news_round_label(comp, stage, round, 3, &mut label);
        f(NEWS_TEXT_BUFFER, 0xa6776c, lg, lg, lb);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, lb, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 cup draw body");
    let (drawn, opponent) = (news_club(read_u32(news + 0x11)), news_club(read_u32(news + 0x15)));
    let two_legs = read_u32(news + 0x19) != 0;
    let stadium = read_i32(news + 0x1d);
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    news_date(dp, read_u16(news + 0x21), read_u16(news + 0x25));
    let lg = news_round_label(comp, stage, round, 2, &mut label);
    let (ga, gb) = (news_g(drawn + club::SHORT_GENDER_NAME), news_g(opponent + club::SHORT_GENDER_NAME));
    f(NEWS_TEXT_BUFFER, 0xa67958, ga, ga, gb, gb, lg, lg, drawn + club::SHORT_NAME, opponent + club::SHORT_NAME, lb);
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, drawn + club::SHORT_NAME, 0x7d5, drawn);
    news_link(out, opponent + club::SHORT_NAME, 0x7d5, opponent);
    news_link(out, lb, 0x7d0, comp);
    let mut s = [0u8; 0xc8];
    let sp = s.as_mut_ptr() as usize;
    exe::date_get_long_date_string(dp, sp, 0x64, 0xff);
    if stadium != -1 {
        covered!("gra_1 cup draw stadium");
        let pair = if exe::langlib_get_language() == 0 || exe::langlib_get_language() == 1 { 0x98f6ac } else { 0x9a3d8c };
        let st = read_usize(G_STADIUM_TABLE).wrapping_add((stadium as usize).wrapping_mul(stadium::SIZE));
        let city = read_usize(st + stadium::CITY);
        let mut place = [0u8; 0x138];
        let pp = place.as_mut_ptr() as usize;
        if city != 0 {
            if !news_streq(st + stadium::NAME, city + 4) && !news_streq(st + stadium::NAME, 0xa67950) {
                sprintf(pp, pair, st + stadium::NAME, city + 4); // "%s (%s)" or "%s, %s"
            } else {
                news_strcpy(pp, city + 4); // the same name, or "Unknown"
            }
        } else {
            news_strcpy(pp, st + stadium::NAME);
        }
        let g = news_g(st + stadium::GENDER_NAME);
        f(NEWS_TEXT_BUFFER, if two_legs { 0xa678cc } else { 0xa67854 }, g, g, 0x64, pp, sp);
    } else {
        f(NEWS_TEXT_BUFFER, if two_legs { 0xa677f8 } else { 0xa677a8 }, 0x64, sp);
    }
    exe::text_append_string(out, NEWS_TEXT_BUFFER);
    1
}

/// Type 21, the group draw. Slots: +5 the competition, +9 the stage (word), +0xd the club, +0x11 and +0x15 the date
/// of the first group game (words), +0x19 the number of the other clubs in the group (byte), +0x1d on their ids (one
/// dword each).
///
/// Body: more than 8 clubs gives the news.cpp error box (line 0x942) and 8. The list of the clubs' short names "A and
/// B", "A, B and C"... up to 8 (0xa673cc, 0xa67360, 0xa672d4, 0xa6721c, 0xa67148, 0xa67054, 0xa66f40; the genders in
/// pairs, then the names); the stage's name (`exe::comp_util_get_stage_name`, 0x64, the stage, 0); 0xa66e6c with the
/// club, the stage, the competition and the list, and its text; with the date's second word set, 0xa66e20 with the
/// date's long text appended. Links: the club (0x7d5), the competition (0x7d0), the stage's name (0x7e5, to ((stage &
/// 0x7ff) | competition << 11) << 11) and the listed clubs (0x7d5). Headline: by the stage's parent
/// (`exe::comp_util_get_parent_stage_id`): 0x41c 0xa66de0, 10 0xa66d94, 0x14 0xa66d48, 0x82 0xa66cfc, 0x449 0xa66cb4,
/// 0x44a 0xa66c6c, others no text; with the competition's short name, and its link.
///
/// Defect of the original: with fewer than 2 other clubs the list buffer is never written (stack leftovers go into
/// the text); the copy uses an empty list.
/// Original: part of `NewsItem::write_text` (0x77f3ed..0x77fd91).
unsafe fn news_group_draw(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "group_draw";
    let f = news_format();
    let ci = read_u32(news + 5);
    let comp = news_comp(ci);
    let stage = read_u16(news + 9);
    if body == 0 {
        let fmt = match exe::comp_util_get_parent_stage_id(stage as i16 as i32) {
            0x41c => 0xa66de0,
            0xa => 0xa66d94,
            0x14 => 0xa66d48,
            0x82 => 0xa66cfc,
            0x449 => 0xa66cb4,
            0x44a => 0xa66c6c,
            _ => 0,
        };
        if fmt != 0 {
            let g = news_g(comp + competition::SHORT_GENDER_NAME);
            f(NEWS_TEXT_BUFFER, fmt, g, g, comp + competition::SHORT_NAME);
        }
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 group draw body");
    let c = news_club(read_u32(news + 0xd));
    let mut d = [0u8; 8];
    let dp = d.as_mut_ptr() as usize;
    news_date(dp, read_u16(news + 0x11), read_u16(news + 0x15));
    let mut n = read_u8(news + 0x19) as i8;
    if n > 8 {
        news_error_box_keep(0x942);
        n = 8;
        write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
    }
    let mut ids = [0u32; 10];
    let mut k = 0i8;
    while k < n && k < 10 {
        ids[k as usize] = read_u32(news + 0x1d + 4 * k as usize);
        k += 1;
    }
    let mut list = [0u8; 0x190];
    let lp = list.as_mut_ptr() as usize;
    let cl: [usize; 8] = core::array::from_fn(|i| news_club(ids[i]));
    let sg = |i: usize| news_g(cl[i] + club::SHORT_GENDER_NAME);
    let sn = |i: usize| cl[i] + club::SHORT_NAME;
    match n {
        2 => {
            f(lp, 0xa673cc, sg(0), sg(0), sg(1), sg(1), sn(0), sn(1));
        }
        3 => {
            f(lp, 0xa67360, sg(0), sg(0), sg(1), sg(1), sg(2), sg(2), sn(0), sn(1), sn(2));
        }
        4 => {
            f(lp, 0xa672d4, sg(0), sg(0), sg(1), sg(1), sg(2), sg(2), sg(3), sg(3), sn(0), sn(1), sn(2), sn(3));
        }
        5 => {
            f(lp, 0xa6721c, sg(0), sg(0), sg(1), sg(1), sg(2), sg(2), sg(3), sg(3), sg(4), sg(4), sn(0), sn(1), sn(2), sn(3), sn(4));
        }
        6 => {
            f(lp, 0xa67148, sg(0), sg(0), sg(1), sg(1), sg(2), sg(2), sg(3), sg(3), sg(4), sg(4), sg(5), sg(5), sn(0), sn(1), sn(2), sn(3),
              sn(4), sn(5));
        }
        7 => {
            f(lp, 0xa67054, sg(0), sg(0), sg(1), sg(1), sg(2), sg(2), sg(3), sg(3), sg(4), sg(4), sg(5), sg(5), sg(6), sg(6), sn(0), sn(1),
              sn(2), sn(3), sn(4), sn(5), sn(6));
        }
        8 => {
            f(lp, 0xa66f40, sg(0), sg(0), sg(1), sg(1), sg(2), sg(2), sg(3), sg(3), sg(4), sg(4), sg(5), sg(5), sg(6), sg(6), sg(7), sg(7),
              sn(0), sn(1), sn(2), sn(3), sn(4), sn(5), sn(6), sn(7));
        }
        _ => {}
    }
    let mut stage_name = [0u8; 0xc8];
    let sp = stage_name.as_mut_ptr() as usize;
    exe::comp_util_get_stage_name(sp as *mut u8, 0x64, stage as u32, 0);
    let (gc, gk) = (news_g(c + club::SHORT_GENDER_NAME), news_g(comp + competition::SHORT_GENDER_NAME));
    f(NEWS_TEXT_BUFFER, 0xa66e6c, gc, gc, 0x64, gk, gk, c + club::SHORT_NAME, sp, comp + competition::SHORT_NAME, lp);
    news_set(out, NEWS_TEXT_BUFFER);
    if read_u16(dp + 2) != 0 {
        covered!("gra_1 group draw start date");
        let mut s = [0u8; 0xc8];
        exe::date_get_long_date_string(dp, s.as_mut_ptr() as usize, 0x64, 0xff);
        f(NEWS_TEXT_BUFFER, 0xa66e20, 0x64, s.as_ptr() as usize);
        exe::text_append_string(out, NEWS_TEXT_BUFFER);
    }
    news_link(out, c + club::SHORT_NAME, 0x7d5, c);
    news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
    news_link(out, sp, 0x7e5, ((((stage as u32) & 0x7ff) | (ci << 11)) << 11) as usize);
    for i in 0..n.max(0) as usize {
        let x = news_club(ids[i]);
        news_link(out, x + club::SHORT_NAME, 0x7d5, x);
    }
    1
}

/// The news.cpp error box without releasing the mouse button (the caller does it after its own fix-up, as the
/// original's inline box of type 21).
unsafe fn news_error_box_keep(line: u32) {
    let (mut name, mut ext) = ([0u8; 0x100], [0u8; 0x100]);
    exe::crt_strtok(0xa693ec, 0, 0, name.as_mut_ptr(), ext.as_mut_ptr());
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), line);
    news_sprintf()(0xae24d0, 0x9870e8, v, name.as_ptr(), ext.as_ptr(), line); // "v%s %s.%s %d"
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0); // "Error"
}

/// Type 22, the invitation to apply for the Intertoto Cup. Slots: +5 nonzero when answered, +9 the club, +0x11
/// nonzero when it applied.
///
/// Body 0xa66bf4 with the club's short name and the Intertoto Cup's name (`KEY_CLUB_COMP_INTER_TOTO_CUP`); when
/// answered, "The application has been made." (0xa66bd0) or "The invitation has been declined." (0xa66bac) appended;
/// links: the club's short name (0x7d5) and the cup's name (0x7d0). Headline 0xa66b80 with the cup's short name, and
/// its link.
/// Original: part of `NewsItem::write_text` (0x77fd9a..0x77feb5).
unsafe fn news_intertoto_invitation(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "intertoto_invitation";
    let f = news_format();
    let comp = news_comp(read_u32(KEY_CLUB_COMP_INTER_TOTO_CUP));
    let c = news_club(read_u32(news + 9));
    if body == 0 {
        let g = news_g(comp + competition::SHORT_GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa66b80, g, g, comp + competition::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 intertoto invitation body");
    let (gc, gk) = (news_g(c + club::SHORT_GENDER_NAME), news_g(comp + competition::GENDER_NAME));
    f(NEWS_TEXT_BUFFER, 0xa66bf4, gc, gc, gk, gk, c + club::SHORT_NAME, comp + competition::NAME);
    news_set(out, NEWS_TEXT_BUFFER);
    if read_u32(news + 5) != 0 {
        f(NEWS_TEXT_BUFFER, if read_u32(news + 0x11) != 0 { 0xa66bd0 } else { 0xa66bac });
        exe::text_append_string(out, NEWS_TEXT_BUFFER);
    }
    news_link(out, c + club::SHORT_NAME, 0x7d5, c);
    news_link(out, comp + competition::NAME, 0x7d0, comp);
    1
}

/// Type 23, the club chosen for the Intertoto Cup. Slot: +5 the club.
///
/// Body 0xa66af0 with the club's short name and the Intertoto Cup's name; headline 0xa66aa8 with both short names.
/// Links: the club's short name (0x7d5) and the cup's name or short name (0x7d0).
/// Original: part of `NewsItem::write_text` (0x77febe..0x77ffb6).
unsafe fn news_intertoto_chosen(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "intertoto_chosen";
    let f = news_format();
    let comp = news_comp(read_u32(KEY_CLUB_COMP_INTER_TOTO_CUP));
    let c = news_club(read_u32(news + 5));
    let gc = news_g(c + club::SHORT_GENDER_NAME);
    let name = if body != 0 {
        let gk = news_g(comp + competition::GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa66af0, gc, gc, gk, gk, c + club::SHORT_NAME, comp + competition::NAME);
        comp + competition::NAME
    } else {
        let gk = news_g(comp + competition::SHORT_GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa66aa8, gc, gc, gk, gk, c + club::SHORT_NAME, comp + competition::SHORT_NAME);
        comp + competition::SHORT_NAME
    };
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, c + club::SHORT_NAME, 0x7d5, c);
    news_link(out, name, 0x7d0, comp);
    1
}

/// The club asked to skip the FA Cup for the World Club Championship (the key club index at 0x9d0240).
const NEWS_KEY_CLUB_WORLD_CLUB_BYE: usize = 0x9d0240;

/// Type 40, the FA's offer of a bye to the club playing the World Club Championship. Slot: +5 the answer (2
/// accepted, 3 rejected).
///
/// Body 0xa669ac with the club's (0x9d0240), the FA Cup's (`KEY_CLUB_COMP_ENGLISH_FA_CUP`) and the World Club
/// Championship's (`KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP`) short names; accepted 0xa66984 or rejected 0xa66934
/// appended; links: the club (0x7d5) and the two competitions (0x7d0). Headline 0xa668f4 with the club's short name, and
/// its link.
/// Original: part of `NewsItem::write_text` (0x780039..0x7801e9).
unsafe fn news_world_club_bye(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "world_club_bye";
    let f = news_format();
    let c = news_club(read_u32(NEWS_KEY_CLUB_WORLD_CLUB_BYE));
    let gc = news_g(c + club::SHORT_GENDER_NAME);
    if body == 0 {
        f(NEWS_TEXT_BUFFER, 0xa668f4, gc, gc, c + club::SHORT_NAME);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, c + club::SHORT_NAME, 0x7d5, c);
        return 1;
    }
    covered!("gra_1 world club bye body");
    let fa = news_comp(read_u32(KEY_CLUB_COMP_ENGLISH_FA_CUP));
    let wcc = news_comp(read_u32(KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP));
    let (gf, gw) = (news_g(fa + competition::SHORT_GENDER_NAME), news_g(wcc + competition::SHORT_GENDER_NAME));
    f(NEWS_TEXT_BUFFER, 0xa669ac, gc, gc, gf, gf, gw, gw, c + club::SHORT_NAME, fa + competition::SHORT_NAME, wcc + competition::SHORT_NAME);
    news_set(out, NEWS_TEXT_BUFFER);
    if read_u32(news + 5) == 2 {
        f(NEWS_TEXT_BUFFER, 0xa66984);
        exe::text_append_string(out, NEWS_TEXT_BUFFER);
    }
    if read_u32(news + 5) == 3 {
        f(NEWS_TEXT_BUFFER, 0xa66934);
        exe::text_append_string(out, NEWS_TEXT_BUFFER);
    }
    news_link(out, c + club::SHORT_NAME, 0x7d5, c);
    news_link(out, fa + competition::SHORT_NAME, 0x7d0, fa);
    news_link(out, wcc + competition::SHORT_NAME, 0x7d0, wcc);
    1
}

/// Type 38, the cup draw with ties still to be played. Slots: +5 the competition, +9 and +0xd the stage and round
/// (words), +0x11 and +0x15 the club (or the two clubs of the tie) on one side, +0x19 and +0x1d those of the other
/// side (+0x15 and +0x1d −1 when the side is a single club).
///
/// Body: the round's label (form 2) and its gender; "A or B will play either C or D" 0xa676a0, "A or B will play C"
/// 0xa675f8, or (+0x15 = −1) "A will play either C or D" 0xa6754c, with the clubs' short names and genders and the
/// label; links: the clubs +0x11 and +0x19, +0x15 and +0x1d when set (0x7d5), the competition on the label (0x7d0).
/// Headline 0xa6776c with the round's label (form 3), and its link.
/// Original: part of `NewsItem::write_text` (0x77ed39..0x77f20c).
unsafe fn news_cup_draw_open(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "cup_draw_open";
    let f = news_format();
    let comp = news_comp(read_u32(news + 5));
    let (stage, round) = (read_u16(news + 9), read_u16(news + 0xd));
    let mut label = [0u8; 0xc8];
    let lb = label.as_ptr() as usize;
    if body == 0 {
        let lg = news_round_label(comp, stage, round, 3, &mut label);
        f(NEWS_TEXT_BUFFER, 0xa6776c, lg, lg, lb);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, lb, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 cup draw open body");
    let (a, b, c, d) = (read_u32(news + 0x11), read_i32(news + 0x15), read_u32(news + 0x19), read_i32(news + 0x1d));
    let lg = news_round_label(comp, stage, round, 2, &mut label);
    let sg = |x: usize| news_g(x + club::SHORT_GENDER_NAME);
    let (ca, cc) = (news_club(a), news_club(c));
    if b != -1 {
        let cb = news_club(b as u32);
        if d != -1 {
            let cd = news_club(d as u32);
            f(NEWS_TEXT_BUFFER, 0xa676a0, sg(ca), sg(ca), sg(cb), sg(cb), sg(cc), sg(cc), sg(cd), sg(cd), lg, lg, ca + club::SHORT_NAME,
              cb + club::SHORT_NAME, cc + club::SHORT_NAME, cd + club::SHORT_NAME, lb);
        } else {
            f(NEWS_TEXT_BUFFER, 0xa675f8, sg(ca), sg(ca), sg(cb), sg(cb), sg(cc), sg(cc), lg, lg, ca + club::SHORT_NAME, cb + club::SHORT_NAME,
              cc + club::SHORT_NAME, lb);
        }
    } else {
        let cd = news_club(d as u32);
        f(NEWS_TEXT_BUFFER, 0xa6754c, sg(ca), sg(ca), sg(cc), sg(cc), sg(cd), sg(cd), lg, lg, ca + club::SHORT_NAME, cc + club::SHORT_NAME,
          cd + club::SHORT_NAME, lb);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, ca + club::SHORT_NAME, 0x7d5, ca);
    news_link(out, cc + club::SHORT_NAME, 0x7d5, cc);
    if b != -1 {
        let cb = news_club(b as u32);
        news_link(out, cb + club::SHORT_NAME, 0x7d5, cb);
    }
    if d != -1 {
        let cd = news_club(d as u32);
        news_link(out, cd + club::SHORT_NAME, 0x7d5, cd);
    }
    news_link(out, lb, 0x7d0, comp);
    1
}

/// Type 39, the cup draw made today. Slots: +5 the competition, +9 and +0xd the stage and round (words), +0x11 the
/// club, +0x21 nonzero for the general story.
///
/// Body: the round's label (form 1, without the lower-casing of the other types) and its gender; the general story
/// 0xa674f4 with the label, or the club's 0xa67458 with its short name, and the club's link (0x7d5) in that case; the
/// competition's link on the label (0x7d0). Headline 0xa67414 with the label (form 2), and its link.
/// Original: part of `NewsItem::write_text` (0x77f215..0x77f3e4).
unsafe fn news_cup_draw_today(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "cup_draw_today";
    let f = news_format();
    let comp = news_comp(read_u32(news + 5));
    let (stage, round) = (read_u16(news + 9) as u32, read_u16(news + 0xd) as u32);
    let c = news_club(read_u32(news + 0x11));
    let general = read_u32(news + 0x21) != 0;
    let mut label = [0u8; 0xc8];
    let lb = label.as_ptr() as usize;
    let form = if body != 0 { 1 } else { 2 };
    exe::comp_util_format_comp_stage_round_label(comp, stage, round, form, label.as_mut_ptr(), 0xc8);
    let lg = exe::comp_util_get_label_gender(comp as u32, stage, round, form) as i8 as i32;
    if body == 0 {
        f(NEWS_TEXT_BUFFER, 0xa67414, lg, lg, lb);
        news_set(out, NEWS_TEXT_BUFFER);
        news_link(out, lb, 0x7d0, comp);
        return 1;
    }
    covered!("gra_1 cup draw today body");
    if general {
        f(NEWS_TEXT_BUFFER, 0xa674f4, lg, lg, lb);
    } else {
        let g = news_g(c + club::SHORT_GENDER_NAME);
        f(NEWS_TEXT_BUFFER, 0xa67458, g, g, lg, lg, c + club::SHORT_NAME, lb);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    if !general {
        news_link(out, c + club::SHORT_NAME, 0x7d5, c);
    }
    news_link(out, lb, 0x7d0, comp);
    1
}

/// Category 7, type 0x3e9, a competition record. Slot: +5 the competition.
///
/// Body: the competition's object (`G_COMP_OBJECT_TABLE`) and its records object (+8) write it (`comp_records_write
/// _news_text`, 0x4a7f50, thiscall on the records object: the news and `out`); without them the news.cpp error box
/// (line 0xb51) and 0. Headline 0xa668cc "New {competition} record" with the competition's short name, and its link
/// (0x7d0).
/// Original: part of `NewsItem::write_text` (0x78022b..0x7802cb).
unsafe fn news_club_record(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "comp_record";
    let ci = read_u32(news + 5);
    if body != 0 {
        let obj = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((ci as usize).wrapping_mul(4)));
        let records = if obj != 0 { read_usize(obj + 8) } else { 0 };
        if records == 0 {
            news_error_box(0xb51);
            return 0;
        }
        covered!("gra_1 comp record body");
        let w: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x4a7f50usize);
        w(records, news, out);
        return 1;
    }
    let comp = news_comp(ci);
    let g = news_g(comp + competition::SHORT_GENDER_NAME);
    news_format()(NEWS_TEXT_BUFFER, 0xa668cc, g, g, comp + competition::SHORT_NAME);
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, comp + competition::SHORT_NAME, 0x7d0, comp);
    1
}

/// The name of an injury type into `buf` (`injury_get_type_name`, 0x62ef30, stdcall; the original loads `G_INJURY`
/// into ecx).
unsafe fn news_injury_name(kind: u8, buf: usize) {
    let w: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(0x62ef30usize);
    w(read_usize(G_INJURY), kind as u32, buf, 0x32, 1);
}
/// The grammatical gender of an injury type's name (`injury_get_type_name_gender`, 0x6306e0).
unsafe fn news_injury_gender(kind: u8) -> i32 {
    let w: unsafe extern "thiscall" fn(usize, u32) -> i8 = core::mem::transmute(0x6306e0usize);
    w(read_usize(G_INJURY), kind as u32) as i32
}
/// The weekday's name of a date (`Date::get_weekday`, `date_get_weekday_name` 0x54d170).
unsafe fn news_weekday_name(date: usize) -> usize {
    let w: unsafe extern "C" fn(i32) -> usize = core::mem::transmute(0x54d170usize);
    w(exe::date_get_weekday(date))
}
/// The weekday's gender of a date in the current language (`exe::date_get_weekday_gender`).
unsafe fn news_weekday_gender(date: usize) -> i32 {
    let wd = exe::date_get_weekday(date);
    exe::date_get_weekday_gender(exe::langlib_get_language() as u32, wd as u32) as i32
}

/// Category 3, type 0x7d0, the injury or illness news. Slots: +5 the player, +9..+0x11 his name ids, +0x15 the injury
/// type, +0x19/+0x1d the day it happened and +0x25/+0x29 the date compared with the name ids (words), +0x21 the days
/// out, +0x2d the club he was injured against (−1 in training), +0x31 the other club of that match, +0x35 the
/// competition, +0x39 the nation where an illness was caught, +0x3d the player's club (−1 none), +0x41 0x420e9 when
/// the match record that follows (from slot 0x10, `exe::read_match_record_from_news`) is the match of the injury.
///
/// Headline (the short name, style 5): by the type's category (`exe::injury_get_type_category`): treatments (12) by
/// the type (0x4d..0x51: faith healing, physiotherapy, surgery, radiotherapy, leave of absence; others surgery);
/// illnesses (0) "struck down with" the illness's name; injuries over 100 days "injury shocker" / "Injury blow" / "out
/// for" the duration (`format_duration_text`) by (type + player) % 3, the others "injured" / "suffers injury" / "out
/// for"; the player's link (0x7d8) when his name ids still match.
///
/// Body (the full name, style 1; the injury's name and the duration first): an illness 0xa6655c (caught at the club in
/// the nation) or 0xa664e4, with the links of the player (when the ids match), the nation (to its national team) and the
/// club. An injury in a match: in Swedish, for types 0x3b and 0x47, a text of its own (`sprintf` 0xa6648c); with the
/// match record, by the score (a win 0xa661e4, a draw 0xa66084, a defeat 0xa65f24: the round's label in form 1 and
/// "%d-%d" of the higher and lower goals); without it 0xa6633c; the competition's gender is 0x64 for the friendly
/// competition (`KEY_CLUB_COMP_FRIENDLY`). Links: the competition (when not a reserve or minor one,
/// `exe::comp_is_reserve_or_minor_comp`), the player (always), the two clubs, and with the record the match on the score
/// (0x7d3, when `exe::match_stats_fixture_entry_matches`) and the record's away club's name. An injury in training: in
/// Swedish, for types 0x3b and 0x47, 0xa65edc or 0xa65e94 by the weekday name's last letter; for another club's player
/// (not the human manager's club, `G_SCREEN_RECORDS_MANAGER`), treatments with the club's short name (a recurring
/// injury, `Injury::get_recurring_injury_type`, names the illness treated: 0xa65908, 0xa659d4, 0xa65aa4, 0xa65b40;
/// otherwise 0xa65d58, 0xa65ca8, 0xa65c10, 0xa65aa4, 0xa65e04) and injuries 0xa65814; for the human manager's club or no
/// club, treatments 0xa656f4, 0xa65664, 0xa65574, 0xa655ec, 0xa65780 and injuries 0xa654a4; links: the player and his
/// club.
/// Original: part of `NewsItem::write_text` (0x7802e7..0x78164b).
unsafe fn news_match_report(news: usize, body: u32, out: usize) -> u32 {
    NEWS_LAST_PIECE = "injury";
    let f = news_format();
    let sprintf = news_sprintf();
    let mut rec = [0u8; 0x20];
    let rp = rec.as_mut_ptr() as usize;
    exe::date_date_54a110(rp);
    let player = read_u32(news + 5);
    let ids = [read_u32(news + 9), read_u32(news + 0xd), read_u32(news + 0x11)];
    let kind = read_u8(news + 0x15);
    let (mut d1, mut d2) = ([0u8; 8], [0u8; 8]);
    let (d1p, d2p) = (d1.as_mut_ptr() as usize, d2.as_mut_ptr() as usize);
    news_date(d1p, read_u16(news + 0x19), read_u16(news + 0x1d));
    let days = read_i32(news + 0x21);
    news_date(d2p, read_u16(news + 0x25), read_u16(news + 0x29));
    let against = read_i32(news + 0x2d);
    let other = read_u32(news + 0x31);
    let comp_i = read_u32(news + 0x35);
    let nation_i = read_i32(news + 0x39);
    let club_i = read_i32(news + 0x3d);
    let in_match = read_u32(news + 0x41) != 0x420e9;
    let mut cursor: u8 = 0x10;
    exe::read_match_record_from_news(news, &mut cursor, rec.as_mut_ptr());
    let mut label = [0u8; 0xc8];
    label[0] = read_u8(0xad9c64);
    let (ga, gb) = (rec[0x19] as i8, rec[0x1a] as i8);
    let mut score = [0u8; 0xc];
    let scp = score.as_mut_ptr() as usize;
    sprintf(scp, 0x98c114, ga.max(gb) as i32, ga.min(gb) as i32); // "%d-%d"
    let human = read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as i32 as usize).wrapping_mul(0x300)));
    let job = if human != 0 { read_usize(human + 0x39) } else { 0 };
    let mine = if job != 0 { club_i == -1 || club_i == read_i32(job) } else { club_i == -1 };
    let mut name = [0u8; 0x12c];
    let np = name.as_mut_ptr() as usize;
    let staff = news_staff(player);
    let (mut ill, mut dur) = ([0u8; 0xc8], [0u8; 0xc8]);
    let (ip, dp) = (ill.as_mut_ptr() as usize, dur.as_mut_ptr() as usize);
    let duration: unsafe extern "C" fn(i32, usize, u32, u32) -> i32 = core::mem::transmute(0x7e5780usize);
    let category = || exe::injury_get_type_category(read_usize(G_INJURY), kind as i8);
    if body == 0 {
        exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x69, 5, player);
        let cat = category();
        if cat == 0xc {
            let fmt = match (kind as i8 as i32).wrapping_sub(0x4d) {
                0 => 0xa6684c, // faith healing
                1 => 0xa66810, // physiotherapy
                3 => 0xa667c8, // radiotherapy
                4 => 0xa66894, // leave of absence
                _ => 0xa66790, // surgery
            };
            f(NEWS_TEXT_BUFFER, fmt, np);
        } else if cat == 0 {
            news_injury_name(kind, ip);
            f(NEWS_TEXT_BUFFER, 0xa66740, np, ip);
        } else {
            let r = (kind as i8 as i32).wrapping_add(player as i32).wrapping_rem(3);
            let fmt = match (days > 100, r) {
                (true, 0) => 0xa6668c,
                (true, 1) => 0xa666c0,
                (false, 0) => 0xa6662c,
                (false, 1) => 0xa66658,
                _ => 0,
            };
            if fmt != 0 {
                f(NEWS_TEXT_BUFFER, fmt, np);
            } else {
                duration(days, dp, 0x32, 1);
                f(NEWS_TEXT_BUFFER, 0xa666f4, np, dp);
            }
        }
        news_set(out, NEWS_TEXT_BUFFER);
        if exe::database_staff_matches_names(staff, ids[0], ids[1], ids[2], d2p) != 0 {
            news_link(out, np, 0x7d8, staff);
        }
        return 1;
    }
    covered!("gra_1 injury body");
    exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x69, 1, player);
    news_injury_name(kind, ip);
    duration(days, dp, 0x32, 1);
    let cat = category();
    if cat == 0 {
        covered!("gra_1 injury illness");
        let a = news_injury_gender(kind);
        let b = news_injury_gender(kind);
        if nation_i != -1 && club_i != -1 {
            let (nat, cl) = (news_nation(nation_i as u32), news_club(club_i as u32));
            let (gc, gn) = (news_g(cl + club::SHORT_GENDER_NAME), news_g(nat + 0x52));
            f(NEWS_TEXT_BUFFER, 0xa6655c, b, a, gc, gc, gn, gn, np, dp, ip, cl + club::SHORT_NAME, nat + 0x38);
        } else {
            f(NEWS_TEXT_BUFFER, 0xa664e4, b, a, np, dp, ip);
        }
        news_set(out, NEWS_TEXT_BUFFER);
        if exe::database_staff_matches_names(staff, ids[0], ids[1], ids[2], d2p) != 0 {
            news_link(out, np, 0x7d8, staff);
        }
        if nation_i != -1 {
            let nat = news_nation(nation_i as u32);
            news_link(out, nat + 0x38, 0x7d5, exe::database_get_national_team(nat));
        }
        if club_i != -1 {
            let cl = news_club(club_i as u32);
            news_link(out, cl + club::SHORT_NAME, 0x7d5, cl);
        }
        return 1;
    }
    if against == -1 {
        return news_injury_in_training(out, kind, np, dp, ip, d1p, staff, club_i, mine);
    }
    covered!("gra_1 injury in a match");
    let (cs, co, comp) = (news_club(against as u32), news_club(other), news_comp(comp_i));
    if exe::langlib_get_language() == 0xa && (kind == 0x3b || kind == 0x47) {
        let wd = news_weekday_name(d1p);
        sprintf(NEWS_TEXT_BUFFER, 0xa6648c, np, dp, ip, comp + competition::NAME, cs + club::SHORT_NAME, co + club::SHORT_NAME, wd);
    } else {
        let gk = if comp_i == read_u32(KEY_CLUB_COMP_FRIENDLY) { 0x64 } else { news_g(comp + competition::GENDER_NAME) as i8 as i32 };
        let (gcs, gco) = (news_g(cs + club::SHORT_GENDER_NAME), news_g(co + club::SHORT_GENDER_NAME));
        if in_match {
            let wd = news_weekday_name(d1p);
            let gwd = news_weekday_gender(d1p);
            let a = news_injury_gender(kind);
            let b = news_injury_gender(kind);
            f(NEWS_TEXT_BUFFER, 0xa6633c, b, a, gk, gk, gcs, gcs, gco, gco, gwd, 0x64, np, dp, ip, comp + competition::NAME,
              cs + club::SHORT_NAME, co + club::SHORT_NAME, wd);
        } else {
            covered!("gra_1 injury with the match record");
            let rc = news_comp(read_u32(rp + 8));
            let lg = news_round_label(rc, read_u16(rp + 0xc), read_u16(rp + 0xe), 1, &mut label);
            let lb = label.as_ptr() as usize;
            let wd = news_weekday_name(d1p);
            let gwd = news_weekday_gender(d1p);
            let a = news_injury_gender(kind);
            let b = news_injury_gender(kind);
            let (csn, con) = (cs + club::SHORT_NAME, co + club::SHORT_NAME);
            if ga > gb {
                f(NEWS_TEXT_BUFFER, 0xa661e4, b, a, gcs, gcs, lg, lg, gco, gco, gwd, 0x64, np, dp, ip, csn, scp, lb, con, wd);
            } else if ga == gb {
                f(NEWS_TEXT_BUFFER, 0xa66084, 0x64, 0x64, b, a, gcs, gcs, lg, lg, gco, gco, gwd, 0x64, np, dp, ip, csn, scp, lb, con, wd);
            } else {
                f(NEWS_TEXT_BUFFER, 0xa65f24, b, a, gcs, gcs, lg, lg, gco, gco, gwd, 0x64, np, dp, ip, csn, scp, lb, con, wd);
            }
        }
    }
    news_set(out, NEWS_TEXT_BUFFER);
    if exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), comp) == 0 {
        news_link(out, comp + competition::NAME, 0x7d0, comp);
    }
    news_link(out, np, 0x7d8, staff);
    news_link(out, cs + club::SHORT_NAME, 0x7d5, cs);
    news_link(out, co + club::SHORT_NAME, 0x7d5, co);
    if rec[0x19] != 0xff && !in_match {
        let fixture = read_u32(rp + 0x1b);
        if exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), fixture as i32, read_i32(rp + 0x11), read_i32(rp + 0x15),
            read_u16(rp) as i32, read_u16(rp + 2) as i32) != 0
        {
            news_link(out, scp, 0x7d3, fixture as usize);
        }
        let away = news_club(read_u32(rp + 0x15));
        news_link(out, away + club::NAME, 0x7d5, away);
    }
    1
}

/// The body of an injury in training (see `news_match_report`): the text, the player's link (0x7d8, always) and his
/// club's (0x7d5, when he has one).
/// Original: part of `NewsItem::write_text` (0x780fe1..0x78164b).
unsafe fn news_injury_in_training(out: usize, kind: u8, np: usize, dp: usize, ip: usize, d1p: usize, staff: usize, club_i: i32, mine: bool) -> u32 {
    covered!("gra_1 injury in training");
    let f = news_format();
    let sprintf = news_sprintf();
    let category = || exe::injury_get_type_category(read_usize(G_INJURY), kind as i8);
    if exe::langlib_get_language() == 0xa && (kind == 0x3b || kind == 0x47) {
        let mut day = [0u8; 0x40];
        news_strcpy(day.as_mut_ptr() as usize, news_weekday_name(d1p));
        if day[0] != 0 {
            let len = day.iter().position(|&c| c == 0).unwrap_or(0);
            let k = (day[len - 1] as i8 as i32).wrapping_sub(0x43);
            // the Swedish genitive: no extra "s" after C, S, X or Z
            let plain = (0..=0x37).contains(&k) && read_u8(0x782b04 + k as usize) == 0;
            sprintf(NEWS_TEXT_BUFFER, if plain { 0xa65edc } else { 0xa65e94 }, np, dp, ip, day.as_ptr() as usize);
        }
    } else if !mine && club_i != -1 {
        let cl = news_club(club_i as u32);
        let gcl = news_g(cl + club::SHORT_GENDER_NAME);
        let csn = cl + club::SHORT_NAME;
        if category() == 0xc {
            let recur: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62ed30usize);
            let r = recur(read_usize(G_INJURY), staff);
            if r == 0xff {
                match (kind as i8 as i32).wrapping_sub(0x4d) {
                    0 => { f(NEWS_TEXT_BUFFER, 0xa65d58, gcl, gcl, csn, np, dp); }
                    1 => { f(NEWS_TEXT_BUFFER, 0xa65ca8, gcl, gcl, csn, np, dp); }
                    3 => { f(NEWS_TEXT_BUFFER, 0xa65aa4, gcl, gcl, csn, np); }
                    4 => { f(NEWS_TEXT_BUFFER, 0xa65e04, np); }
                    _ => { f(NEWS_TEXT_BUFFER, 0xa65c10, gcl, gcl, csn, np, dp); }
                }
            } else {
                covered!("gra_1 injury recurring");
                news_injury_name(r, ip);
                match (kind as i8 as i32).wrapping_sub(0x4d) {
                    0 => {
                        let a = news_injury_gender(r);
                        let b = news_injury_gender(r);
                        f(NEWS_TEXT_BUFFER, 0xa65908, gcl, gcl, b, a, csn, np, dp, ip);
                    }
                    1 => {
                        let a = news_injury_gender(r);
                        let b = news_injury_gender(r);
                        f(NEWS_TEXT_BUFFER, 0xa659d4, gcl, gcl, b, a, csn, np, dp, ip);
                    }
                    3 => { f(NEWS_TEXT_BUFFER, 0xa65aa4, gcl, gcl, csn, np); }
                    _ => {
                        let a = news_injury_gender(r);
                        let b = news_injury_gender(r);
                        f(NEWS_TEXT_BUFFER, 0xa65b40, gcl, gcl, 0x64, b, a, csn, np, dp, ip);
                    }
                }
            }
        } else {
            let wd = news_weekday_name(d1p);
            let gwd = news_weekday_gender(d1p);
            let a = news_injury_gender(kind);
            let b = news_injury_gender(kind);
            f(NEWS_TEXT_BUFFER, 0xa65814, gcl, gcl, 0x64, 0x64, 0x64, b, a, gwd, 0x64, csn, np, dp, ip, wd);
        }
    } else if category() == 0xc {
        match (kind as i8 as i32).wrapping_sub(0x4d) {
            0 => { f(NEWS_TEXT_BUFFER, 0xa656f4, np, dp); }
            1 => { f(NEWS_TEXT_BUFFER, 0xa65664, np, dp); }
            3 => { f(NEWS_TEXT_BUFFER, 0xa655ec, np); }
            4 => { f(NEWS_TEXT_BUFFER, 0xa65780, np); }
            _ => { f(NEWS_TEXT_BUFFER, 0xa65574, np, dp); }
        }
    } else {
        let wd = news_weekday_name(d1p);
        let gwd = news_weekday_gender(d1p);
        let a = news_injury_gender(kind);
        let b = news_injury_gender(kind);
        f(NEWS_TEXT_BUFFER, 0xa654a4, b, a, gwd, 0x64, np, dp, ip, wd);
    }
    news_set(out, NEWS_TEXT_BUFFER);
    news_link(out, np, 0x7d8, staff);
    if club_i > -1 {
        let cl = news_club(club_i as u32);
        news_link(out, cl + club::SHORT_NAME, 0x7d5, cl);
    }
    1
}

/// Category 1, type 0x1388, a manager's message. Slots: +5 the sending manager, +9 nonzero for a private message,
/// +0xd the id of the news it answers (0 none), slot 3 its text.
///
/// Headline: "Private message from %s" (0xa65470) or "Public message from %s" (0xa65448) with the manager's name
/// (`exe::database_format_staff_name`, 0xc8, 1). Body: when it answers a news and `flag` is 0, the quoted news is looked
/// up (`News::find_item_by_id` 0x786b70 on the news object 0xb7431c, into a blank news item dated with the game's
/// date 0xae2c90 and day slice 0xae2c88) and, when found, its body is written (this routine, flag 1) into a new text
/// and appended as "> " text "\n>\n"; then the message's text (slot 3, `NewsItem::get_slot_string` 0x785a20, 0x7d0) is
/// appended. The body only appends to `out`.
/// Original: part of `NewsItem::write_text` (0x78284a..0x782a02).
unsafe fn news_follow_up(news: usize, manager: usize, out: usize, body: u32, flag: u32) -> u32 {
    NEWS_LAST_PIECE = "manager_message";
    let sender = read_u32(news + 5);
    let private = read_u32(news + 9) != 0;
    let quoted = read_u32(news + 0xd);
    if body == 0 {
        let mut name = [0u8; 0xc8];
        exe::database_format_staff_name(news_staff(sender), name.as_mut_ptr() as usize, 0xc8, 1);
        news_format()(NEWS_TEXT_BUFFER, if private { 0xa65470 } else { 0xa65448 }, name.as_ptr() as usize);
        news_set(out, NEWS_TEXT_BUFFER);
        return 1;
    }
    covered!("gra_1 manager message body");
    if quoted != 0 && flag == 0 {
        covered!("gra_1 manager message quote");
        let mut item = [0u8; 0xe0];
        let ip = item.as_mut_ptr() as usize;
        exe::date_date_54a110(ip + news_item::DATE);
        let mut tmp = [0u8; 8];
        exe::date_operator_3d(ip + news_item::DATE, tmp.as_mut_ptr() as usize, 0xae2c90usize);
        write_u8(ip + news_item::DAY_SLICE, read_u8(0xae2c88));
        let mut text = [0usize; 2];
        let tp = text.as_mut_ptr() as usize;
        let ctor: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x4ae590usize);
        ctor(tp, 0);
        let find: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(0x786b70usize);
        if find(read_usize(0xb7431c), 0, quoted, ip) != 0 {
            let write: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x77bc80usize);
            write(ip, manager, tp, 1, 1);
            exe::text_append_string(out, 0xa654a0usize); // "> "
            exe::text_append_string(out, text[0]);
            exe::text_append_string(out, 0xa6549cusize); // "\n>\n"
        }
        exe::drop_text(text.as_mut_ptr());
    }
    let get: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(0x785a20usize);
    get(news, 3, NEWS_TEXT_BUFFER, 0x7d0);
    exe::text_append_string(out, NEWS_TEXT_BUFFER);
    1
}

/// Compares `NewsItem::write_text`: the original writes into `out` and the copy into a local text, side by side
/// (`compare_text_writer`: the text, the links and the random generator), and the return values. Without a manager or
/// `out` (the error box) only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_newsitem_write_text(news: usize, manager: usize, out: usize, body: u32, flag: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(%%I0%%);
    if manager == 0 || out == 0 {
        return o(news, manager, out, body, flag);
    }
    let (ro, rr) = (core::cell::Cell::new(0u32), core::cell::Cell::new(0u32));
    let (cat, ty) = (exe::newsitem_get_category(news), read_u32(news));
    compare_text_writer(out, || ro.set(o(news, manager, out, body, flag)), |t| rr.set(newsitem_write_text(news, manager, t, body, flag)),
        || format!("newsitem_write_text(category {}, type {:#x}, body {}, flag {}, piece {})", cat, ty, body, flag, NEWS_LAST_PIECE));
    SCOREBOARD.record(ro.get() == rr.get(), || format!("newsitem_write_text(type {:#x}) returned {} x {}", ty, ro.get(), rr.get()));
    ro.get()
}
