// ---------- batch %%LEVA%%: a nation's World Cup fate in the news; the award news and the award shortlists (world_cup.cpp, award_manager.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\award_manager.cpp" (the error box of add_shortlist).
pub(crate) const AWARD_MANAGER_CPP: usize = 0x987e24;

/// Fields of the award manager (`AWARD_MANAGER`, the object at [`G_AWARD_MANAGER`]) the shortlist routines use.
pub(crate) mod award_manager_fields {
    /// The nation award objects' table (+8: [+8][nation] is a NationAwards, whose +4 is its award table).
    pub(crate) const NATION_AWARDS: usize = 0x8;
    /// The growable array of the open shortlists (+0x14), its count (+0x18, signed word) and capacity (+0x1a, word,
    /// grown by 5).
    pub(crate) const SHORTLISTS: usize = 0x14;
    pub(crate) const SHORTLIST_COUNT: usize = 0x18;
    pub(crate) const SHORTLIST_CAPACITY: usize = 0x1a;
}

/// Rule: the news text of a nation's World Cup fate: in the group stages (stage below 7) only the round-of-16
/// qualification (fate 6) and the group exit (fate −2) have a text, the shared group sentence; in the knockout (stage 7)
/// the win, the runners-up place, the third and fourth places, the elimination ("knocked out of") and any other fate
/// ("through to next round in") of the competition; past stage 7 there is no text.
///
/// The arguments as `goldcup_build_fate_news_text` (the club index `club_index` in [`G_CLUB_TABLE`], the fate byte
/// `fate`, the stage byte `stage` (signed), four values passed on, `full_sentence` and the Text `text`). Stage below 7:
/// fate 6 or 0xfe (−2) write the shared group sentence 0x4b4590 (club_index, a3, the competition record, the fate as
/// 6 or −2, full_sentence, text: 6 pushes; add esp, 0x18 -> cdecl), with or without `full_sentence`; any other fate,
/// nothing. Stage 7: with `full_sentence`, the shared knockout sentence 0x4b0b80 (club_index, a4, a7, fate, a5, text:
/// cdecl); otherwise the template of `copa_america_fate_template` (the same six texts) with the club's gender twice, the
/// competition's gender twice, the club's and the competition's short names (`langlib_sprintf` into 0xde1f64), set as
/// the text (`exe::text_set_string`) with the club's link (0x7d5) and the competition's (0x7d0).
/// Original: `WorldCup::build_fate_news_text` (0x92cea0, world_cup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn world_cup_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, full_sentence: u32, text: usize) {
    let this = League(this);
    let _ = a6;
    let comp = this.competition().addr();
    let s = stage as u8 as i8;
    // simplification: on the paths without text the original leaves al with the stage or the fate byte; the callers do
    // not read it
    if s < 7 {
        let group_fate: i32 = match fate as u8 {
            6 => 6,
            0xfe => -2, // mwg15: the group exit has its text
            _ => return,
        };
        covered!("mwg_15 world cup group fate");
        let sentence: unsafe extern "C" fn(u32, u32, usize, i32, u32, usize) = core::mem::transmute(FN_COMP_UTIL_WRITE_GROUP_QUALIFY_NEWS);
        sentence(club_index, a3, comp, group_fate, full_sentence, text);
        return;
    }
    if s != 7 {
        return;
    }
    covered!("mwg_15 world cup knockout fate");
    if full_sentence != 0 {
        let sentence: unsafe extern "C" fn(u32, u32, u32, u32, u32, usize) = core::mem::transmute(FN_COMP_UTIL_WRITE_CUP_TIE_RESULT_NEWS);
        sentence(club_index, a4, a7, fate, a5, text);
        return;
    }
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add((club_index as usize).wrapping_mul(club::SIZE)));
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let club_gender = club.short_gender_name() as i8 as i32;
    let comp_gender = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
    sprintf(0xde1f64, copa_america_fate_template(fate as u8), club_gender, club_gender, comp_gender, comp_gender, club.addr() + club::SHORT_NAME, comp + competition::SHORT_NAME);
    exe::text_set_string(text, 0xde1f64 as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
    exe::text_add_link(text, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
}

/// Comparer for `world_cup_build_fate_news_text`: side by side into two Texts (`compare_text_writer`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_world_cup_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, full_sentence: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I0%%);
    compare_text_writer(text, || o(this, club_index, fate, stage, a3, a4, a5, a6, a7, full_sentence, text),
        |t| world_cup_build_fate_news_text(this, club_index, fate, stage, a3, a4, a5, a6, a7, full_sentence, t),
        || format!("world_cup_build_fate_news_text(club {}, fate {}, stage {}, full {})", club_index, fate as u8, stage as u8 as i8, full_sentence));
}

/// Passes a news text request to one award of a nation: the award `award` of the nation awards `nation` writes the text
/// of `item` into `text` (headline or body by `body`).
///
/// The award object is [[`award_manager_fields::NATION_AWARDS`][nation] + 4][award] (no checks); its virtual +0xc
/// (`write_news`(item, text, body): YEAR_AWARD::write_news, TEAM_AWARD::write_news..., thiscall, ret 0xc), whose eax it
/// returns.
/// Original: `AWARD_MANAGER::write_award_news` (0x4143f0, award_manager.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn write_award_news(this: usize, nation: i32, award: i32, item: usize, text: usize, body: u32) -> u32 {
    let awards = read_usize(read_usize(this + award_manager_fields::NATION_AWARDS).wrapping_add((nation as isize * 4) as usize));
    let object = read_usize(read_usize(awards + 4).wrapping_add((award as isize * 4) as usize));
    covered!("mwg_15 award news");
    let write: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(object) + 0xc));
    write(object, item, text, body) // mwg15: headline or body
}

/// Comparer for `write_award_news`: side by side into two Texts (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_write_award_news(this: usize, nation: i32, award: i32, item: usize, text: usize, body: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, usize, usize, u32) -> u32>(%%I1%%);
    let mut r = 0;
    compare_text_writer(text, || r = o(this, nation, award, item, text, body), |t| {
        write_award_news(this, nation, award, item, t, body);
    }, || format!("write_award_news(nation {}, award {}, body {})", nation, award, body));
    r
}

/// Appends a shortlist to the award manager's growable array and gives its index.
///
/// When the count (`SHORTLIST_COUNT`, signed word) is not below the capacity (`SHORTLIST_CAPACITY`), the capacity goes up
/// by 5 and the array (`SHORTLISTS`) is allocated (`exe::crt_malloc`, first time) or grown (`exe::crt_realloc`) to 4 ×
/// capacity bytes; out of memory, the array becomes null, the error box (line 0x39a) and −1 (in ax; the original's high
/// word is the box's). Otherwise the shortlist goes into slot `count`, the count goes up by one and the result is the new
/// count − 1 in ax (the original computes it from esi, whose high word is still `this`'s: kept).
/// Original: `AWARD_MANAGER::add_shortlist` (0x4147f0, award_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn add_shortlist(this: usize, shortlist: usize) -> u32 {
    use award_manager_fields as am;
    let capacity = read_u16(this + am::SHORTLIST_CAPACITY);
    if read_i16(this + am::SHORTLIST_COUNT) >= capacity as i16 {
        let capacity = capacity.wrapping_add(5);
        write_u16(this + am::SHORTLIST_CAPACITY, capacity);
        let bytes = ((capacity as i16 as i32) << 2) as u32;
        let array = read_usize(this + am::SHORTLISTS);
        let array = if array == 0 { exe::crt_malloc(bytes) } else { exe::crt_realloc(array, bytes) };
        write_usize(this + am::SHORTLISTS, array);
        if array == 0 {
            show_error_at(AWARD_MANAGER_CPP, 0x39a);
            return 0xffff;
        }
        covered!("mwg_15 shortlists grown");
    }
    let count = read_i16(this + am::SHORTLIST_COUNT);
    write_usize(read_usize(this + am::SHORTLISTS).wrapping_add((count as isize * 4) as usize), shortlist);
    let count = count.wrapping_add(1);
    write_i16(this + am::SHORTLIST_COUNT, count);
    ((this as u32 & 0xffff_0000) | count as u16 as u32).wrapping_sub(1)
}

// simplification: the array may be (re)allocated: it does not run twice. Original only; the plan: the count up by one, the
// shortlist in the last slot and the old count in ax.
/// Comparer for `add_shortlist`.
pub(crate) unsafe extern "thiscall" fn compare_add_shortlist(this: usize, shortlist: usize) -> u32 {
    use award_manager_fields as am;
    let before = read_i16(this + am::SHORTLIST_COUNT);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, shortlist);
    let after = read_i16(this + am::SHORTLIST_COUNT);
    let ok = r as u16 == 0xffff || (after == before.wrapping_add(1) && r as u16 as i16 == before
        && read_usize(read_usize(this + am::SHORTLISTS) + before as usize * 4) == shortlist);
    SCOREBOARD.record(ok, || format!("add_shortlist({:#x}) count {} -> {}, returned {:#x}", shortlist, before, after, r));
    r
}
