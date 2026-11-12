// ---------- cqz_14: the news text of a nation's Copa América fate (goldcup.cpp) ----------

/// The Copa América fate templates of the knockout (stage 2), by fate byte: 1 win, 2 runners-up, 3 third place, 4
/// fourth place, 0xff knocked out; any other, "through to next round" (0x987198).
fn copa_america_fate_template(fate: u8) -> usize {
    match fate {
        1 => 0x98732c, // cqz14: "win"
        2 => 0x9872c8,
        3 => 0x987264,
        4 => 0x987200,
        0xff => 0x98713c,
        _ => 0x987198,
    }
}

/// rule: the news text of a nation's Copa América fate: in the group stages (stage below 2) only the quarter-final
/// qualification (fate 6) has a text, "<team> qualify for quarter finals"; in the knockout (stage 2) the win, the
/// runners-up place, the third and fourth places, the elimination ("knocked out of") and any other fate ("through to
/// next round in") of the competition; from stage 3 on there is no text
///
/// The arguments: the club index `club_index` in [`G_CLUB_TABLE`] (`club::SIZE` each), the fate byte `fate`, the stage
/// byte `stage` (signed), four values passed on (`a3`..`a7`), `full_sentence` and the Text `text`.
///
/// Stage below 2: fate other than 6 writes nothing. With `full_sentence`, the shared qualification sentence 0x4b4590
/// (club_index, a3, the competition record `league::COMPETITION`, 6, full_sentence, text: 6 pushes; add esp, 0x18 ->
/// cdecl). Otherwise `langlib_sprintf` (`FN_LANGLIB_SPRINTF`, cdecl, variadic) into the shared buffer 0xde1f64 of
/// template 0x98737c with the club's gender (`club::SHORT_GENDER_NAME`, signed byte, twice) and short name
/// (`club::SHORT_NAME`), set as the text (`exe::text_set_string`: push; mov ecx, text -> thiscall, ret 4) with the
/// club's link (`exe::text_add_link`(text, short name, 0x7d5, club): 3 pushes -> thiscall, ret 0xc).
///
/// Stage 2: with `full_sentence`, the shared knockout sentence 0x4b0b80 (club_index, a4, a7, fate, a5, text: cdecl).
/// Otherwise the template of `copa_america_fate_template` with the club's gender twice, the competition's gender
/// (`competition::SHORT_GENDER_NAME`) twice, the club's and the competition's short names, set as the text, with the
/// club's link (0x7d5) and the competition's (`competition::SHORT_NAME`, 0x7d0, the record).
/// Original: `GoldCup::build_fate_news_text` (0x5e0f50, goldcup.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn goldcup_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, full_sentence: u32, text: usize) {
    let _ = a6;
    let comp = read_usize(this + league::COMPETITION);
    let club = read_usize(G_CLUB_TABLE).wrapping_add((club_index as usize).wrapping_mul(club::SIZE));
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let club_gender = read_u8(club + club::SHORT_GENDER_NAME) as i8 as i32;
    let s = stage as u8 as i8;
    // simplification: on the paths without text the original leaves al with the stage or the fate byte; the callers do not read it
    if s < 2 {
        if fate as u8 != 6 {
            return;
        }
        covered!("cqz_14 copa america quarter finals");
        if full_sentence != 0 {
            let sentence: unsafe extern "C" fn(u32, u32, usize, u32, u32, usize) = core::mem::transmute(0x4b4590usize);
            sentence(club_index, a3, comp, 6, full_sentence, text);
            return;
        }
        sprintf(0xde1f64, 0x98737c, club_gender, club_gender, club + club::SHORT_NAME);
        exe::text_set_string(text, 0xde1f64 as *const u8);
        exe::text_add_link(text, (club + club::SHORT_NAME) as *const u8, 0x7d5, club);
        return;
    }
    if s != 2 {
        return;
    }
    covered!("cqz_14 copa america knockout fate");
    if full_sentence != 0 {
        let sentence: unsafe extern "C" fn(u32, u32, u32, u32, u32, usize) = core::mem::transmute(0x4b0b80usize);
        sentence(club_index, a4, a7, fate, a5, text);
        return;
    }
    let comp_gender = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
    sprintf(0xde1f64, copa_america_fate_template(fate as u8), club_gender, club_gender, comp_gender, comp_gender, club + club::SHORT_NAME, comp + competition::SHORT_NAME);
    exe::text_set_string(text, 0xde1f64 as *const u8);
    exe::text_add_link(text, (club + club::SHORT_NAME) as *const u8, 0x7d5, club);
    exe::text_add_link(text, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
}

/// Comparer for `goldcup_build_fate_news_text`: side by side into two Texts (`compare_text_writer`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_goldcup_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, full_sentence: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(%%I0%%);
    compare_text_writer(text, || o(this, club_index, fate, stage, a3, a4, a5, a6, a7, full_sentence, text),
        |t| goldcup_build_fate_news_text(this, club_index, fate, stage, a3, a4, a5, a6, a7, full_sentence, t),
        || format!("goldcup_build_fate_news_text(club {}, fate {}, stage {}, full {})", club_index, fate as u8, stage as u8 as i8, full_sentence));
}
