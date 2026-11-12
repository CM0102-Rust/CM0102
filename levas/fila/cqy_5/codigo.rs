// ---------- batch %%LEVA%%: the cup news lines: a club wins in or is defeated in a cup round, and a team qualifies or fails to qualify from a group (comp_util.cpp) ----------

/// The round name code of a fixture without its variant: `fixture::ROUND_NAME` less its units digit (0 single match,
/// 3 first leg, 4 second leg, 5 replay) when it is below 1000, as the original's idiv by 10 (a word).
fn round_name_without_variant(code: i16) -> i16 {
    if code < 1000 { code.wrapping_sub(code % 10) } else { code }
}

/// Writes the short news line "<club> win in <round>" or "<club> defeat in <round>" into Text `text`.
///
/// `comp_util_write_cup_win_or_defeat_news(club, fixture, result, text)` (cdecl). The label of the fixture's
/// competition (`fixture::COMPETITION`), its stage (`fixture::ROUND_PREFIX`) and round (`round_name_without_variant` of
/// `fixture::ROUND_NAME`) in form 2 (comp_util_format_comp_stage_round_label, a 500-byte buffer); the gender of the
/// label: of the round (comp_util_get_stage_name_gender, form 0) when it is not 0, else of the stage, else the
/// competition's short-name gender (+0x52), else 1. With `result` (byte) 2, 4 or −1 the "defeat in" line (format
/// 0x9a3dbc), otherwise the "win in" line (0x9a3e1c): the club's short name and gender (`club::SHORT_NAME`,
/// `club::SHORT_GENDER_NAME` of club index `club`) and the label, printed into the global text buffer 0xde1f64
/// (langlib_sprintf `FN_LANGLIB_SPRINTF`). Then Text::set_string `exe::text_set_string` with it, and two links
/// (Text::add_link `exe::text_add_link`): the club name (type 0x7d5, the club) and the label (type 0x7d0, the
/// competition).
/// Original: `comp_util_write_cup_win_or_defeat_news` (0x4b4420, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_write_cup_win_or_defeat_news(club: i32, fixture: usize, result: u32, text: usize) {
    let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let f = Fixture(fixture);
    let comp = read_usize(fixture + fixture::COMPETITION);
    let prefix = f.round_prefix();
    let round = round_name_without_variant(f.round_name());
    let mut label = [0u8; 500];
    exe::comp_util_format_comp_stage_round_label(comp, prefix as u16 as u32, round as u16 as u32, 2, label.as_mut_ptr(), 500);
    let gender = if round != 0 {
        exe::comp_util_get_stage_name_gender(round as u16 as u32, 0)
    } else if prefix != 0 {
        exe::comp_util_get_stage_name_gender(prefix as u16 as u32, 0)
    } else if comp != 0 {
        read_u8(comp + 0x52)
    } else {
        1
    };
    let c = club_at(club as isize).addr();
    let cg = read_u8(c + club::SHORT_GENDER_NAME) as i8 as i32;
    let g = gender as i8 as i32;
    let format = match result as u8 {
        2 | 4 | 0xff => 0x9a3dbc, // cqy5: "defeat in"
        _ => 0x9a3e1c,
    };
    covered!("cqy_5 cup win or defeat news");
    lang_sprintf(0xde1f64, format, cg, cg, g, g, c + club::SHORT_NAME, label.as_ptr() as usize);
    exe::text_set_string(text, 0xde1f64 as *const u8);
    exe::text_add_link(text, (c + club::SHORT_NAME) as *const u8, 0x7d5, c);
    exe::text_add_link(text, label.as_ptr(), 0x7d0, comp);
}

/// Comparer for `comp_util_write_cup_win_or_defeat_news`: the original into the real Text and the copy into a local
/// one (`compare_text_writer`).
pub(crate) unsafe extern "C" fn compare_comp_util_write_cup_win_or_defeat_news(club: i32, fixture: usize, result: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(i32, usize, u32, usize)>(%%I0%%);
    compare_text_writer(text, || o(club, fixture, result, text), |t| comp_util_write_cup_win_or_defeat_news(club, fixture, result, t),
        || format!("comp_util_write_cup_win_or_defeat_news(club {}, fixture {:#x}, result {})", club, fixture, result as u8 as i8));
}

/// Writes the news line of a team qualifying from, or failing to qualify from, a competition group into Text `text`.
///
/// `comp_util_write_group_qualify_news(club, stage, comp, result, with_stage, text)` (cdecl). Only `result` (byte) 6 or
/// 7 (qualified) and −2 (failed) write anything. With `with_stage`: the stage's full name (`exe::comp_util_get_stage_name`
/// form 0, 100 bytes) and gender, and the line (format 0x9a3ed4 qualified, 0x9a3fd0 failed) with the team's short name
/// and gender (club index `club`), the stage name and gender and the competition's long name and gender
/// (`club_comp::NAME`, +4; +0x37); then Text::set_string and three links: the team name (type 0x7d5, the club), the
/// competition name (0x7d0, the competition) and the stage name (0x7e5, with the data (stage & 0x7ff | competition id
/// << 11) << 11 when qualified, (sign-extended stage word | 0xfffff800) << 11 when failed). Without it: the line (format
/// 0x9a3e74 qualified, 0x9a3f68 failed) with the team and the competition's short name and gender (+0x38, +0x52) and two
/// links: the team (0x7d5) and the competition's short name (0x7d0). The line is printed into the global text buffer
/// 0xde1f64 (langlib_sprintf `FN_LANGLIB_SPRINTF`).
/// Original: `comp_util_write_group_qualify_news` (0x4b4590, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_util_write_group_qualify_news(club: i32, stage: u32, comp: usize, result: u32, with_stage: u32, text: usize) {
    let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let qualified = match result as u8 {
        6 | 7 => true, // cqy5: qualified
        0xfe => false,
        _ => return,
    };
    let c = club_at(club as isize).addr();
    let cg = read_u8(c + club::SHORT_GENDER_NAME) as i8 as i32;
    let team = c + club::SHORT_NAME;
    if with_stage != 0 {
        covered!("cqy_5 group news with the stage");
        let mut name = [0u8; 100];
        exe::comp_util_get_stage_name(name.as_mut_ptr(), 100, stage, 0);
        let sg = exe::comp_util_get_stage_name_gender(stage, 0) as i8 as i32;
        let kg = read_u8(comp + 0x37) as i8 as i32;
        lang_sprintf(0xde1f64, if qualified { 0x9a3ed4 } else { 0x9a3fd0 }, cg, cg, sg, sg, kg, kg, team, name.as_ptr() as usize, comp + 4);
        exe::text_set_string(text, 0xde1f64 as *const u8);
        exe::text_add_link(text, team as *const u8, 0x7d5, c);
        exe::text_add_link(text, (comp + 4) as *const u8, 0x7d0, comp);
        let data = if qualified {
            ((stage & 0x7ff) | (read_u32(comp) << 0xb)) << 0xb
        } else {
            ((stage as u16 as i16 as i32 as u32) | 0xffff_f800) << 0xb
        };
        exe::text_add_link(text, name.as_ptr(), 0x7e5, data as usize);
    } else {
        covered!("cqy_5 group news without the stage");
        let kg = read_u8(comp + 0x52) as i8 as i32;
        lang_sprintf(0xde1f64, if qualified { 0x9a3e74 } else { 0x9a3f68 }, cg, cg, kg, kg, team, comp + 0x38);
        exe::text_set_string(text, 0xde1f64 as *const u8);
        exe::text_add_link(text, team as *const u8, 0x7d5, c);
        exe::text_add_link(text, (comp + 0x38) as *const u8, 0x7d0, comp);
    }
}

/// Comparer for `comp_util_write_group_qualify_news`: the original into the real Text and the copy into a local one
/// (`compare_text_writer`).
pub(crate) unsafe extern "C" fn compare_comp_util_write_group_qualify_news(club: i32, stage: u32, comp: usize, result: u32, with_stage: u32, text: usize) {
    let o = original_fn::<unsafe extern "C" fn(i32, u32, usize, u32, u32, usize)>(%%I1%%);
    compare_text_writer(text, || o(club, stage, comp, result, with_stage, text), |t| comp_util_write_group_qualify_news(club, stage, comp, result, with_stage, t),
        || format!("comp_util_write_group_qualify_news(club {}, stage {}, comp {:#x}, result {}, {})", club, stage as u16 as i16, comp, result as u8 as i8, with_stage));
}
