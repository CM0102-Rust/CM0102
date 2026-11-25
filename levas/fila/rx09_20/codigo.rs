// ---------- rx09_20: the Asian World Cup qualifying progress text (wc_asia_league.cpp) ----------

/// Writes the stage or progress text of an Asian qualifying nation (virtual +0x38; the ecx is not read): a buffer
/// shorter than 30 (signed) shows the box at line 0x3b6 and writes nothing. By the round code (signed word): 0x3fd to
/// 0x406 (the first round groups), status 6 gives 0xad4824 "Qualified For Second Round" and any other 0x9a7c04 "First
/// Round"; 0x407 and 0x408 (the second round groups), status 7 0xad4658 "Qualified For World Cup", 1 0xad47f8
/// "Qualified For Third Round" and any other 0x9a7b94 "Second Round"; 0x1e 0x9a7b10 "Third Round"; 0xa0 0x9a751c
/// "Playoff"; any other code nothing. The "Qualified" texts take the competition's gender byte
/// (`competition::GENDER_NAME`) and the empty string (`EMPTY_STRING`), through langlib_sprintf.
/// Original: `WcAsiaLeague::write_progress_text` (0x912f80, wc_asia_league.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_write_progress_text(_this: usize, buf: usize, size: i32, round: u32, _sub_round: u32, status: u32, comp: usize) -> i32 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if size < 0x1e {
        // simplification: after the box the original returns whatever the box left in eax; the callers do not read it
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x3b6);
        return 0;
    }
    covered!("rx09_20 asian progress text");
    let qualified = |text: usize| format(buf, text, read_u8(comp + competition::GENDER_NAME) as i8 as i32, EMPTY_STRING);
    let s = status as u8;
    match round as u16 as i16 {
        0x3fd..=0x406 => if s == 6 { qualified(0xad4824) } else { format(buf, 0x9a7c04) }, // rx09_20: the first round texts
        0x407..=0x408 => match s {
            7 => qualified(0xad4658),
            1 => qualified(0xad47f8),
            _ => format(buf, 0x9a7b94),
        },
        0x1e => format(buf, 0x9a7b10),
        0xa0 => format(buf, 0x9a751c),
        // simplification: with any other code the original writes nothing and returns the code left in eax
        _ => 0,
    }
}

/// Comparer for `wc_asia_league_write_progress_text`: the original into the caller's buffer, the copy into a local one
/// of the same size (up to 4 KB) side by side; the texts must match. Below 30 bytes (the box) only the original.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_write_progress_text(this: usize, buf: usize, size: i32, round: u32, sub_round: u32, status: u32, comp: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, usize) -> i32>(%%I0%%);
    let r = o(this, buf, size, round, sub_round, status, comp);
    if size < 0x1e || buf == 0 {
        return r;
    }
    if !matches!(round as u16 as i16, 0x3fd..=0x408 | 0x1e | 0xa0) {
        return r;
    }
    let mut local = vec![0u8; (size as usize).min(0x1000)];
    wc_asia_league_write_progress_text(this, local.as_mut_ptr() as usize, local.len() as i32, round, sub_round, status, comp);
    let c = |p: *const u8| core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes().to_vec();
    let (mo, mr) = (c(buf as *const u8), c(local.as_ptr()));
    SCOREBOARD.record(mo == mr, || format!("wc_asia_league_write_progress_text(round {:#x}, status {}): original {:?} rust {:?}", round as u16, status as u8, String::from_utf8_lossy(&mo),
        String::from_utf8_lossy(&mr)));
    r
}
