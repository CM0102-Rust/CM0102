// ---------- rx13_6: the article form of a region's name (index.cpp) ----------

/// The article form of each region (1..=14) in language 9, read from the original's jump table 0x6177b0.
const REGION_ARTICLE_FORM_LANGUAGE_9: [u8; 14] = [4, 1, 3, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0];

/// rule: the form of a region's name that picks its article in the scouting texts: only language 9 has them (region
/// 1, 4; region 3, 3; regions 7 and 14, 0; any other, 1); every other language, 0
///
/// `get_region_article_form(region)` (the region as a signed byte), by the language (`exe::langlib_get_language`): in
/// language 9, the entry region − 1 of `REGION_ARTICLE_FORM_LANGUAGE_9`, 1 outside 1..=14; otherwise 0 (the original
/// asks for the language a second time on that path and ignores the answer). Only the low byte is the result.
/// Original: `get_region_article_form` (0x617780, index.cpp, cdecl).
pub(crate) unsafe extern "C" fn get_region_article_form(region: u32) -> u32 {
    covered!("rx13_6 region article form");
    if exe::langlib_get_language() != LANGUAGE_9 {
        return 0; // rx13_6: no article form
    }
    let k = (region as u8 as i8 as i32).wrapping_sub(1);
    REGION_ARTICLE_FORM_LANGUAGE_9.get(k as u32 as usize).copied().unwrap_or(1) as u32
}

/// Comparer for `get_region_article_form`: the low byte, side by side.
pub(crate) unsafe extern "C" fn compare_get_region_article_form(region: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u32>(%%I0%%)(region);
    let n = get_region_article_form(region);
    SCOREBOARD.record(r as u8 == n as u8, || format!("get_region_article_form({}) original {} rust {}", region as u8 as i8, r as u8, n as u8));
    r
}
