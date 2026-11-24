// ---------- batch %%LEVA%%: the nation menu split: building it, its part headers and the side of a nation (key_nation.cpp; sx12_3) ----------

/// Builds a nation menu split: how many nations the menu holds and the letter where its second part starts.
///
/// `NATION_MENU_SPLIT::NATION_MENU_SPLIT(selected_only, order, use_short)` (0xe bytes: +0 the split letter, +1 the
/// order array, +5 the selected-only flag, +9 the nation count, +0xa the short-name flag): clears the letter and stores
/// the three arguments. The count is the rows of `G_LEAGUE_TABLE` whose nation is not null and chosen to be played
/// (`.league_selected()` not zero; with `selected_only`, its bit 2 set). Then, over the 35 indices of `order` (signed
/// bytes), the same nations in that order: the one whose running number equals (count + 1) / 2 (signed, rounded toward
/// zero) gives the split letter, the first character of its short name (`nation::SHORT_NAME`) with `use_short`, else of
/// its name (+4). Returns `this`.
/// Original: `NATION_MENU_SPLIT::NATION_MENU_SPLIT` (0x66a280, key_nation.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn nation_menu_split_construct(this: usize, selected_only: u32, order: usize, use_short: u32) -> usize {
    write_u32(this + 5, selected_only);
    write_usize(this + 1, order);
    write_u8(this, 0);
    write_u32(this + 0xa, use_short);
    let shown = |n: Nation| {
        let s = if n.is_null() { 0 } else { n.league_selected() };
        s != 0 && (selected_only == 0 || s & 2 != 0)
    };
    let mut count: u8 = 0;
    for k in 0..0x23usize {
        if shown(Nation(read_usize(G_LEAGUE_TABLE + 72 * k))) {
            count = count.wrapping_add(1);
        }
    }
    write_u8(this + 9, count);
    let middle = (count as i8 as i32 + 1) / 2; // sx12_3: the middle nation
    let mut seen: i8 = 0;
    for k in 0..0x23usize {
        let n = Nation(read_usize(G_LEAGUE_TABLE.wrapping_add((read_i8(order + k) as isize * 72) as usize)));
        if !shown(n) {
            continue;
        }
        seen = seen.wrapping_add(1);
        if seen as i32 == middle {
            covered!("sx12_3 split letter");
            write_u8(this, if use_short != 0 { n.short_name() } else { read_u8(n.addr() + 4) }); // the name's first letter
        }
    }
    this
}

/// Writes the header of one part of a split nation menu: "Nations A-{letter}" for the first part, "Nations
/// {next letter}-Z" for the second.
///
/// `NATION_MENU_SPLIT::format_header(text, size, second)`: a size below 20 shows the "Error" box (key_nation.cpp line
/// 0x57f) and writes nothing. Otherwise the game's text formatter (`FN_LANGLIB_SPRINTF`) with "Nations A-<%c -
/// Character>" and the split letter (+0, signed), or, when the low byte of `second` is set, "Nations <%c -
/// Character>-Z" and the letter plus one.
/// Original: `NATION_MENU_SPLIT::format_header` (0x66a360, key_nation.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn nation_menu_split_format_header(this: usize, text: usize, size: i32, second: u32) {
    if size < 0x14 {
        show_error_at(0x9eaff4, 0x57f); // "...\key_nation.cpp"
        return;
    }
    let letter = read_i8(this) as i32;
    if second as u8 == 0 {
        covered!("sx12_3 first part header");
        story_lang!(text, 0x9eb0cc, letter); // "Nations A-<%c - Character>"
    } else {
        covered!("sx12_3 second part header");
        story_lang!(text, 0x9eb0b0, letter + 1); // sx12_3: "Nations <%c - Character>-Z"
    }
}

/// Tells whether a nation goes in the second part of a split menu: the first character of its short name
/// (`nation::SHORT_NAME`, with the short-name flag at +0xa) or of its name (+4) above the split letter (+0), signed.
/// Only al is the return value.
/// Original: `NATION_MENU_SPLIT::in_second_part` (0x66a430, key_nation.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nation_menu_split_in_second_part(this: usize, nation: usize) -> u8 {
    let n = Nation(nation);
    let first = if read_u32(this + 0xa) != 0 { n.short_name() } else { read_u8(n.addr() + 4) };
    covered!("sx12_3 second part");
    (first as i8 > read_i8(this)) as u8 // sx12_3: after the split letter
}

/// Comparer for `nation_menu_split_construct`: the 0xe bytes of the object and the return, side by side.
pub(crate) unsafe extern "thiscall" fn compare_nation_menu_split_construct(this: usize, selected_only: u32, order: usize, use_short: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize>(%%I0%%);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0xe)], || o(this, selected_only, order, use_short),
        || nation_menu_split_construct(this, selected_only, order, use_short));
    SCOREBOARD.record(equal && r == n, || format!("NATION_MENU_SPLIT::NATION_MENU_SPLIT({}, {:#x}, {}): {:?}", selected_only, order, use_short, first_differences(&mo, &mr)));
    r
}

/// Comparer for `nation_menu_split_format_header`: the original writes into the caller's text and the copy into a
/// local one; the two strings side by side. A size below 20 (the "Error" box): only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_nation_menu_split_format_header(this: usize, text: usize, size: i32, second: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32)>(%%I1%%);
    o(this, text, size, second);
    if size < 0x14 {
        return;
    }
    let mut copy = [0u8; 0x100];
    nation_menu_split_format_header(this, copy.as_mut_ptr() as usize, size, second);
    let a = core::ffi::CStr::from_ptr(text as *const core::ffi::c_char).to_bytes();
    let b = core::ffi::CStr::from_ptr(copy.as_ptr() as *const core::ffi::c_char).to_bytes();
    SCOREBOARD.record(a == b, || format!("NATION_MENU_SPLIT::format_header(second {}) original {:?} rust {:?}", second as u8,
        String::from_utf8_lossy(a), String::from_utf8_lossy(b)));
}

/// Comparer for `nation_menu_split_in_second_part`: al, side by side (the routine only reads).
pub(crate) unsafe extern "thiscall" fn compare_nation_menu_split_in_second_part(this: usize, nation: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I2%%)(this, nation);
    let n = nation_menu_split_in_second_part(this, nation);
    SCOREBOARD.record(r == n, || format!("NATION_MENU_SPLIT::in_second_part(nation {:#x}) original {} rust {}", nation, r, n));
    r
}
