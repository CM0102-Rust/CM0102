// ---------- cx17_3: the news text of a nation's fate in the South American World Cup qualifying (wc_south_american_league.cpp), and the pot order of the World Cup draw (world_cup.cpp) ----------

/// rule: the news text of a nation's fate in the South American qualifying: a qualified nation (fate 7) "have qualified
/// from the" South American qualifying in the full sentence, "qualify for" the World Cup in the short one; a nation going
/// to the play-off (1) "qualify for Oceania playoff"; an eliminated one (−2) the shared group sentence; any other fate,
/// no text
///
/// The arguments as `world_cup_build_fate_news_text` (the club index `club_index` in [`G_CLUB_TABLE`], the fate byte
/// `fate`, six values not read here, `full_sentence` and the Text `text`). The lines are printed into the global buffer
/// 0xde1f64 (`FN_LANGLIB_SPRINTF`) and set as the text (`exe::text_set_string`); the club's gender and short name
/// (`club::SHORT_GENDER_NAME`, `club::SHORT_NAME`) go into every one.
/// - Fate 7 with `full_sentence`: 0xad4d6c with the club's gender twice, the competition record's short gender twice
///   (`competition::SHORT_GENDER_NAME`), the club's short name and the competition's long name (`competition::NAME`);
///   the links: the club (0x7d5) and the competition's long name (0x7d0).
/// - Fate 7 without it: 0xad44cc with the club's gender twice, the World Cup record's short gender twice
///   (`KEY_CLUB_COMP_FIFA_WORLD_CUP` in [`G_CLUB_COMP_TABLE`], 0x6b bytes each), the club's and the World Cup's short
///   names; the links: the World Cup (0x7d0) first, then the club (0x7d5).
/// - Fate 1: 0xad4d2c with the club's gender twice and its short name; the club's link (0x7d5).
/// - Fate 0xfe: the shared group sentence (`FN_COMP_UTIL_WRITE_GROUP_QUALIFY_NEWS`: club_index, the stage id, the
///   competition record, −2, full_sentence, text: 6 pushes; add esp, 0x18 -> cdecl). The stage id goes as the dword
///   the original builds: the word `league::STAGE_ID` over the high half of `text` (the callee reads the word).
/// Original: `WcSouthAmericanLeague::build_fate_news_text` (0x920b40, wc_south_american_league.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, full_sentence: u32, text: usize) {
    let _ = (stage, a3, a4, a5, a6, a7);
    let this = League(this);
    let club = club_at(club_index as i32 as isize);
    let club_name = club.addr() + club::SHORT_NAME;
    let club_gender = club.short_gender_name() as i8 as i32;
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    // simplification: the original leaves al with the fate byte or a callee's eax; the callers do not read it
    match fate as u8 {
        7 if full_sentence != 0 => {
            covered!("cx17_3 south american qualified, full sentence");
            let comp = this.competition().addr();
            let comp_gender = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
            sprintf(0xde1f64, 0xad4d6c, club_gender, club_gender, comp_gender, comp_gender, club_name, comp + competition::NAME);
            exe::text_set_string(text, 0xde1f64 as *const u8);
            exe::text_add_link(text, club_name as *const u8, 0x7d5, club);
            exe::text_add_link(text, (comp + competition::NAME) as *const u8, 0x7d0, comp);
        }
        7 => {
            covered!("cx17_3 south american qualified");
            let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b));
            let cup_gender = read_u8(world_cup + competition::SHORT_GENDER_NAME) as i8 as i32;
            sprintf(0xde1f64, 0xad44cc, club_gender, club_gender, cup_gender, cup_gender, club_name, world_cup + competition::SHORT_NAME);
            exe::text_set_string(text, 0xde1f64 as *const u8);
            exe::text_add_link(text, (world_cup + competition::SHORT_NAME) as *const u8, 0x7d0, world_cup); // cx17_3: the World Cup's link first
            exe::text_add_link(text, club_name as *const u8, 0x7d5, club);
        }
        1 => {
            covered!("cx17_3 south american to the play-off");
            sprintf(0xde1f64, 0xad4d2c, club_gender, club_gender, club_name);
            exe::text_set_string(text, 0xde1f64 as *const u8);
            exe::text_add_link(text, club_name as *const u8, 0x7d5, club);
        }
        0xfe => {
            covered!("cx17_3 south american out");
            let sentence: unsafe extern "C" fn(u32, u32, usize, i32, u32, usize) = core::mem::transmute(FN_COMP_UTIL_WRITE_GROUP_QUALIFY_NEWS);
            let stage_id = (text as u32 & 0xffff_0000) | this.stage_id() as u16 as u32;
            sentence(club_index, stage_id, this.competition().addr(), -2, full_sentence, text);
        }
        _ => {}
    }
}

/// Comparer for `wc_south_american_league_build_fate_news_text`: side by side into two Texts (`compare_text_writer`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, full_sentence: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I0%%);
    compare_text_writer(text, || o(this, club_index, fate, stage, a3, a4, a5, a6, a7, full_sentence, text),
        |t| wc_south_american_league_build_fate_news_text(this, club_index, fate, stage, a3, a4, a5, a6, a7, full_sentence, t),
        || format!("wc_south_american_league_build_fate_news_text(club {}, fate {}, full {})", club_index, fate as u8, full_sentence));
}

/// rule: the draw entries are sorted by pot, lowest first
///
/// The qsort comparator of the draw and seeding entries: the signed pot byte at +4 of `a` less that of `b`.
/// Original: `compare_entries_by_pot` (0x92f660, world_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_entries_by_pot(a: usize, b: usize) -> i32 {
    covered!("cx17_3 pot order");
    read_i8(a + 4) as i32 - read_i8(b + 4) as i32 // cx17_3: the pot byte
}

/// Comparer for `compare_entries_by_pot`: it only reads; side by side on the result.
pub(crate) unsafe extern "C" fn compare_compare_entries_by_pot(a: usize, b: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let r = compare_entries_by_pot(a, b);
    SCOREBOARD.record(o == r, || format!("compare_entries_by_pot({:#x}, {:#x}) original={} rust={}", a, b, o, r));
    o
}
