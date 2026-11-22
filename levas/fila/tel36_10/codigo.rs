// ---------- batch %%LEVA%%: the search filters' attribute count and the filter bit index (search_screens.cpp) ----------

/// The SearchFilters fields of the "attributes to match" counters: for outfield players the maximum (+0x63) and the
/// count (+0x64); with the goalkeeper set (`which` ≠ 0) the maximum (+0x7d) and the count (+0x7e). Signed bytes.
const SEARCH_FILTERS_MAX: usize = 0x63;
const SEARCH_FILTERS_COUNT: usize = 0x64;
const SEARCH_FILTERS_GK_MAX: usize = 0x7d;
const SEARCH_FILTERS_GK_COUNT: usize = 0x7e;

/// Rule: one more attribute must match, up to the maximum.
///
/// `SearchFilters::increase_attributes_to_match(which)`: the counter of the set (`which` ≠ 0 the goalkeeper's) goes up
/// by one while below its maximum (signed bytes) and 1 comes back; at the maximum 0.
/// Original: `SearchFilters::increase_attributes_to_match` (0x8084c0, search_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn search_filters_increase_attributes_to_match(filters: usize, which: u32) -> u32 {
    let (count, max) = if which != 0 { (SEARCH_FILTERS_GK_COUNT, SEARCH_FILTERS_GK_MAX) } else { (SEARCH_FILTERS_COUNT, SEARCH_FILTERS_MAX) };
    let n = read_u8(filters + count) as i8;
    if n >= read_u8(filters + max) as i8 {
        return 0;
    }
    covered!("tel36_10 one more attribute to match");
    write_u8(filters + count, n.wrapping_add(1) as u8);
    1
}

/// Rule: one attribute fewer must match, down to one.
///
/// `SearchFilters::decrease_attributes_to_match(which)`: the counter of the set goes down by one while above 1 (signed)
/// and 1 comes back; at 1 or below 0.
/// Original: `SearchFilters::decrease_attributes_to_match` (0x808500, search_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn search_filters_decrease_attributes_to_match(filters: usize, which: u32) -> u32 {
    let count = if which != 0 { SEARCH_FILTERS_GK_COUNT } else { SEARCH_FILTERS_COUNT };
    let n = read_u8(filters + count) as i8;
    if n <= 1 {
        return 0;
    }
    covered!("tel36_10 one attribute fewer to match");
    write_u8(filters + count, n.wrapping_sub(1) as u8);
    1
}

/// The 65th search filter: two bits (bits 31 to 33 of the 64-bit mask: 0x3_8000_0000) for index 0x40.
const SEARCH_FILTER_INDEX_0X40: u64 = 0x3_8000_0000;

/// Rule: a search filter mask with a single bit gives its index (0..63); the double mask 0x3_8000_0000 gives 0x40;
/// anything else -1.
///
/// `search_screens_filter_bit_to_index(low, high)`: the 64-bit mask high:low compared against 1 << k for k = 0..63 in
/// order, then against `SEARCH_FILTER_INDEX_0X40`.
/// Original: `search_screens_filter_bit_to_index` (0x81b0e0, search_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn search_screens_filter_bit_to_index(low: u32, high: u32) -> i8 {
    let mask = (high as u64) << 32 | low as u64;
    if mask.count_ones() == 1 {
        return mask.trailing_zeros() as i8;
    }
    if mask == SEARCH_FILTER_INDEX_0X40 {
        covered!("tel36_10 filter bit index 0x40");
        return 0x40;
    }
    -1
}

/// Rule: a search filter index gives its mask: 1 << k for 0..63, 0x3_8000_0000 for 0x40, 0 otherwise.
///
/// `search_screens_filter_index_to_bit(k)`: the 64-bit mask in edx:eax for the char `k`.
/// Original: `search_screens_filter_index_to_bit` (0x81b4e0, search_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn search_screens_filter_index_to_bit(k: u32) -> u64 {
    match k as u8 {
        b @ 0..=0x3f => 1u64 << b,
        0x40 => SEARCH_FILTER_INDEX_0X40,
        _ => 0,
    }
}

/// Comparer for `search_filters_increase_attributes_to_match`: the two counters and the answer side by side.
pub(crate) unsafe extern "thiscall" fn compare_search_filters_increase_attributes_to_match(filters: usize, which: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I0%%);
    let reg = [(filters + SEARCH_FILTERS_MAX, 2), (filters + SEARCH_FILTERS_GK_MAX, 2)];
    let (r, n, same, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(filters, which), || search_filters_increase_attributes_to_match(filters, which));
    SCOREBOARD.record(same && r == n, || format!("SearchFilters::increase_attributes_to_match({}): original={} rust={} differences {:?}", which, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `search_filters_decrease_attributes_to_match`.
pub(crate) unsafe extern "thiscall" fn compare_search_filters_decrease_attributes_to_match(filters: usize, which: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I1%%);
    let reg = [(filters + SEARCH_FILTERS_MAX, 2), (filters + SEARCH_FILTERS_GK_MAX, 2)];
    let (r, n, same, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(filters, which), || search_filters_decrease_attributes_to_match(filters, which));
    SCOREBOARD.record(same && r == n, || format!("SearchFilters::decrease_attributes_to_match({}): original={} rust={} differences {:?}", which, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `search_screens_filter_bit_to_index`.
pub(crate) unsafe extern "C" fn compare_search_screens_filter_bit_to_index(low: u32, high: u32) -> i8 {
    let r = original_fn::<unsafe extern "C" fn(u32, u32) -> i8>(%%I2%%)(low, high);
    let n = search_screens_filter_bit_to_index(low, high);
    SCOREBOARD.record(r == n, || format!("search_screens_filter_bit_to_index({:#x}, {:#x}): original={} rust={}", low, high, r, n));
    r
}

/// Comparer for `search_screens_filter_index_to_bit`.
pub(crate) unsafe extern "C" fn compare_search_screens_filter_index_to_bit(k: u32) -> u64 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u64>(%%I3%%)(k);
    let n = search_screens_filter_index_to_bit(k);
    SCOREBOARD.record(r == n, || format!("search_screens_filter_index_to_bit({}): original={:#x} rust={:#x}", k as u8, r, n));
    r
}
