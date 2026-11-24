// ---------- batch %%LEVA%%: the key nation menu order and the nation menu split count (key_nation.cpp; sx12_2) ----------

/// Sorts the 35 key nation row indices into menu order and returns the static index array (0xb63d38).
///
/// `key_nation_sorted_order(use_short)`: writes 0..34 into the byte array at 0xb63d38, stores the flag in the dword at
/// 0xb63d5c (read by `key_nation_compare_entries`: compare the nations by their short name) and sorts the array with
/// the C runtime's qsort (`exe::crt_qsort`, 35 one-byte elements) and `key_nation_compare_entries` (0x66a100). Called
/// by the menu bar (0x75db00, every time it is built), the manager, search, setup and transfer screens.
/// Original: `key_nation_sorted_order` (0x66a0c0, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_sorted_order(use_short: u32) -> usize {
    for k in 0..0x23u8 {
        write_u8(0xb63d38 + k as usize, k); // the static index array
    }
    write_u32(0xb63d5c, use_short); // the flag of the comparison
    covered!("sx12_2 key nation order");
    exe::crt_qsort(0xb63d38usize, 0x23, 1, 0x66a100usize); // sx12_2: key_nation_compare_entries
    0xb63d38
}

/// Orders the three world rows first by their index, then those of the continents by the continent's name, then the
/// nations by name.
///
/// `key_nation_compare_entries(a, b)` (qsort, one-byte row indices of `G_LEAGUE_TABLE`, nation at +0 and continent at
/// +4 (`G_LEAGUE_WORLD_CUP_CONTINENT`)): a null a gives 1 and a null b -1. A row with neither nation nor continent (a
/// world row) goes before any other; two of them by the index (0 before 1 before 2; other indices: equal). A row with
/// a continent goes before a row with neither: with both continents, `exe::utils_string_compare` of their names (+4); a
/// with a continent and b without: -1; a without (a nation row) and b with: 1. Two nation rows: by the nations' names
/// (+4), or their short names (`nation::SHORT_NAME`) when the flag at 0xb63d5c is set.
/// Original: `key_nation_compare_entries` (0x66a100, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_compare_entries(a: *const i8, b: *const i8) -> i32 {
    if a.is_null() {
        return 1;
    }
    if b.is_null() {
        return -1;
    }
    let (ia, ib) = (*a as isize, *b as isize);
    let row = |i: isize| (i * 72) as usize;
    let (na, ca) = (read_usize(G_LEAGUE_TABLE.wrapping_add(row(ia))), read_usize(G_LEAGUE_WORLD_CUP_CONTINENT.wrapping_add(row(ia))));
    let (nb, cb) = (read_usize(G_LEAGUE_TABLE.wrapping_add(row(ib))), read_usize(G_LEAGUE_WORLD_CUP_CONTINENT.wrapping_add(row(ib))));
    if na == 0 && ca == 0 {
        if nb != 0 || cb != 0 {
            return -1;
        }
        covered!("sx12_2 world rows");
        // the world rows by their index (only 0, 1 and 2 are ordered)
        return if (0..=2).contains(&ia) && (0..=2).contains(&ib) { (ia - ib).signum() as i32 } else { 0 };
    }
    if nb == 0 && cb == 0 {
        return 1;
    }
    if ca != 0 {
        if cb == 0 {
            return -1;
        }
        covered!("sx12_2 continent rows");
        return exe::utils_string_compare((ca + 4) as *const u8, (cb + 4) as *const u8, 1, 0); // the continents' names
    }
    if cb != 0 {
        return 1;
    }
    if na == 0 || nb == 0 {
        return 0;
    }
    covered!("sx12_2 nation rows");
    if read_u32(0xb63d5c) != 0 {
        exe::utils_string_compare((na + nation::SHORT_NAME) as *const u8, (nb + nation::SHORT_NAME) as *const u8, 1, 0)
    } else {
        exe::utils_string_compare((na + 4) as *const u8, (nb + 4) as *const u8, 1, 0) // sx12_2: the nations' names
    }
}

/// Returns 2 when a nation menu holds more than 16 nations and is shown in two parts, else 1.
///
/// `NATION_MENU_SPLIT::part_count()`: the signed byte count at +9 above 16 gives 2. Only al is the return value (the
/// original increments the whole eax, whose high bytes are left over).
/// Original: `NATION_MENU_SPLIT::part_count` (0x66a350, key_nation.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn nation_menu_split_part_count(this: usize) -> u8 {
    covered!("sx12_2 menu parts");
    (read_i8(this + 9) > 16) as u8 + 1 // sx12_2: more than 16 nations
}

/// Comparer for `key_nation_sorted_order`: the index array and the flag (0xb63d38, 0x28 bytes) and the return, side by
/// side.
pub(crate) unsafe extern "C" fn compare_key_nation_sorted_order(use_short: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32) -> usize>(%%I0%%);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(0xb63d38, 0x28)], || o(use_short), || key_nation_sorted_order(use_short));
    SCOREBOARD.record(equal && r == n, || format!("key_nation_sorted_order({}) original {:#x} rust {:#x}: {:?}", use_short, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `key_nation_compare_entries`: the return, side by side (the routine only reads).
pub(crate) unsafe extern "C" fn compare_key_nation_compare_entries(a: *const i8, b: *const i8) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const i8, *const i8) -> i32>(%%I1%%)(a, b);
    let n = key_nation_compare_entries(a, b);
    SCOREBOARD.record(r == n, || format!("key_nation_compare_entries({:?}, {:?}) original {} rust {}",
        if a.is_null() { None } else { Some(*a) }, if b.is_null() { None } else { Some(*b) }, r, n));
    r
}

/// Comparer for `nation_menu_split_part_count`: al, side by side.
pub(crate) unsafe extern "thiscall" fn compare_nation_menu_split_part_count(this: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(%%I2%%)(this);
    let n = nation_menu_split_part_count(this);
    SCOREBOARD.record(r == n, || format!("NATION_MENU_SPLIT::part_count(count {}) original {} rust {}", read_i8(this + 9), r, n));
    r
}
