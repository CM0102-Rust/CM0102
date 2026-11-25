/// Writes the text of a bid rejection reason flag into `out`: none (0xa8eaa4 with the empty string), "The offer is
/// totally unacceptable" (0xa), "This player is indispensable to the club" (0x200), "... an important first team
/// player" (0x400), "Do not want to sell to a rival club" (0x800), "... to a club in the same division" (0x1000) or
/// "This player is a hot prospect for the future" (0x2000), through `langlib_sprintf`; any other reason leaves `out`
/// as it is. With `size` below 50 nothing is written: the transfer_manager.cpp "Error" box (line 0x163d,
/// `show_error_box`). The reason is compared as a signed word; `this` is not read.
/// Original: `transfer_manager_get_rejection_reason_text` (0x8c5690, transfer_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn get_rejection_reason_text(_this: usize, reason: u32, out: usize, size: u32) {
    if (size as i32) < 0x32 {
        show_error_box(0xa8d514, 0x163d);
        return;
    }
    let fmt = news_text_format();
    let template = match reason as i16 {
        0 => {
            fmt(out, 0xa8eaa4, 0xad9c64); // "None" (the empty comment)
            return;
        }
        0xa => 0xa8eae0, // rx05_10: unacceptable
        0x200 => 0xa8275c,
        0x400 => 0xa8272c,
        0x800 => 0xa8ea4c,
        0x1000 => 0xa8ea70,
        0x2000 => 0xa826a0,
        _ => return,
    };
    covered!("rx05_10 rejection reason text");
    fmt(out, template);
}

/// Comparer for `get_rejection_reason_text`: side by side over the text (the copy writes into a local buffer of the
/// same size, up to 0x400 bytes, the original into `out`); with `size` below 50 the original only (the error box).
pub(crate) unsafe extern "thiscall" fn compare_get_rejection_reason_text(this: usize, reason: u32, out: usize, size: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32)>(%%I0%%);
    if (size as i32) < 0x32 || size > 0x400 {
        return o(this, reason, out, size); // simplification: original only
    }
    let mut mine = [0u8; 0x400];
    let n = size as usize;
    mine[..n].copy_from_slice(core::slice::from_raw_parts(out as *const u8, n));
    o(this, reason, out, size);
    get_rejection_reason_text(this, reason, mine.as_mut_ptr() as usize, size);
    let theirs = core::slice::from_raw_parts(out as *const u8, n);
    SCOREBOARD.record(theirs == &mine[..n], || {
        format!("get_rejection_reason_text(reason {:#x}): original {:?} rust {:?}", reason as i16,
            String::from_utf8_lossy(theirs.split(|&b| b == 0).next().unwrap_or(&[])),
            String::from_utf8_lossy(mine[..n].split(|&b| b == 0).next().unwrap_or(&[])))
    });
}
