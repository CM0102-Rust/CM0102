// --- sx20_13: the defaults, resets, setters and attribute minimums of the player search filters (search_filters.cpp,
// 0x806450..0x808280; set_option_flag 0x806630 and player_matches_filter 0x807450 stay original) ---

/// The search filter record (`SEARCH_FILTERS_SIZE` bytes): +0 and +4 the two 32-bit halves of the option mask, +8..+0x17
/// four more option dwords, then the single-value filters and the two attribute range tables (31 player attributes:
/// minimums from +0x25, maximums from +0x44, two counters at +0x63/+0x64; 12 non-player attributes: minimums from
/// +0x65, maximums from +0x71, counters at +0x7d/+0x7e).
const SEARCH_FILTERS_SIZE: usize = 0x80;
const SEARCH_NATIONALITY_NATION: usize = 0x18;
const SEARCH_NATIONALITY_REGION: usize = 0x1c;
const SEARCH_LANGUAGE: usize = 0x1d;
const SEARCH_BASED_NATION: usize = 0x1e;
const SEARCH_BASED_REGION: usize = 0x22;
const SEARCH_VALUE_BAND: usize = 0x23;
const SEARCH_JOB: usize = 0x24;
const SEARCH_PLAYER_MINIMUMS: usize = 0x24; // + the attribute (1 to 31)
const SEARCH_PLAYER_MAXIMUMS: usize = 0x43; // + the attribute (1 to 31)
const SEARCH_PLAYER_ATTRIBUTES: usize = 31;
const SEARCH_PLAYER_RANGES_SET: usize = 0x63;
const SEARCH_PLAYER_RANGES_SHOWN: usize = 0x64;
const SEARCH_STAFF_MINIMUMS: usize = 0x64; // + the attribute (1 to 12)
const SEARCH_STAFF_MAXIMUMS: usize = 0x70; // + the attribute (1 to 12)
const SEARCH_STAFF_ATTRIBUTES: usize = 12;
const SEARCH_STAFF_RANGES_SET: usize = 0x7d;
const SEARCH_STAFF_RANGES_SHOWN: usize = 0x7e;
/// The top of an attribute range.
const SEARCH_ATTRIBUTE_TOP: u8 = 20;
/// search_filters.cpp, the source path of the error boxes.
const SEARCH_FILTERS_SOURCE: usize = 0xa7df38;
const FN_SEARCH_FILTERS_SET_OPTION_FLAG: usize = 0x806630;

/// `SearchFilters::set_option_flag(low, high)`: sets one bit of the 64-bit option mask (the low and high dwords),
/// clearing its group siblings; returns whether the mask changed.
unsafe fn search_set_option_flag(this: usize, low: u32, high: u32) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_SEARCH_FILTERS_SET_OPTION_FLAG);
    f(this, low, high)
}

/// The general options' defaults (no nationality or based-in nation or region, every language option, any value
/// band, job 5, option dwords +8 = 0x10 and +0x10 = 0x800000).
unsafe fn search_general_defaults(this: usize) {
    write_u32(this + 8, 0x10);
    write_u32(this + 0x10, 0x800000);
    write_u32(this + 0xc, 0);
    write_u32(this + 0x14, 0);
    write_i32(this + SEARCH_NATIONALITY_NATION, -1);
    write_i8(this + SEARCH_NATIONALITY_REGION, -1);
    write_u8(this + SEARCH_LANGUAGE, 1);
    write_i32(this + SEARCH_BASED_NATION, -1);
    write_i8(this + SEARCH_BASED_REGION, -1);
    write_u8(this + SEARCH_VALUE_BAND, 0);
    write_u8(this + SEARCH_JOB, 5); // sx20_13: the default job filter
}

/// One attribute range table back to 1..20 for every attribute, with its two counters cleared.
unsafe fn search_attribute_defaults(this: usize, minimums: usize, count: usize, set: usize, shown: usize) {
    for k in 1..=count {
        write_u8(this + minimums + k, 1);
        write_u8(this + minimums + count + k, SEARCH_ATTRIBUTE_TOP);
    }
    write_u8(this + set, 0);
    write_u8(this + shown, 0);
}

/// Writes the default of every search filter: the option masks 0x14140081 / 0x44aaa884, the general options
/// (`search_general_defaults`), both attribute range tables (`search_attribute_defaults`) and +0x7f cleared.
/// Original: `SearchFilters::set_defaults` (0x806450, search_filters.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn search_filters_set_defaults(this: usize) {
    search_general_defaults(this);
    write_u32(this, 0x14140081);
    write_u32(this + 4, 0x44aaa884);
    search_attribute_defaults(this, SEARCH_PLAYER_MINIMUMS, SEARCH_PLAYER_ATTRIBUTES, SEARCH_PLAYER_RANGES_SET, SEARCH_PLAYER_RANGES_SHOWN);
    search_attribute_defaults(this, SEARCH_STAFF_MINIMUMS, SEARCH_STAFF_ATTRIBUTES, SEARCH_STAFF_RANGES_SET, SEARCH_STAFF_RANGES_SHOWN);
    write_u8(this + 0x7f, 0);
}

/// Resets the general search options: the default bit of each option group (`set_option_flag` with the fifteen
/// default bits, in the original's order), the general options' defaults and +0x7f cleared; the attribute ranges stay.
/// Original: `SearchFilters::reset_general_filters` (0x8064d0, search_filters.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn search_filters_reset_general_filters(this: usize) {
    const DEFAULT_BITS: [(u32, u32); 15] = [(1, 0), (0x80, 0), (0x40000, 0), (0x100000, 0), (0x4000000, 0), (0x10000000, 0), (0x80000000, 3),
        (0, 4), (0, 0x80), (0, 0x800), (0, 0x2000), (0, 0x8000), (0, 0x200000), (0, 0x800000), (0, 0x4000000)];
    for (low, high) in DEFAULT_BITS {
        search_set_option_flag(this, low, high);
    }
    search_general_defaults(this);
    write_u8(this + 0x7f, 0);
}

/// Resets both attribute range tables: the player ranges' option bit (high 0x20000), the player table, the
/// non-player ranges' bit (high 0x80000) and the non-player table.
/// Original: `SearchFilters::reset_attribute_filters` (0x8065d0, search_filters.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn search_filters_reset_attribute_filters(this: usize) {
    search_set_option_flag(this, 0, 0x20000);
    search_attribute_defaults(this, SEARCH_PLAYER_MINIMUMS, SEARCH_PLAYER_ATTRIBUTES, SEARCH_PLAYER_RANGES_SET, SEARCH_PLAYER_RANGES_SHOWN);
    search_set_option_flag(this, 0, 0x80000);
    search_attribute_defaults(this, SEARCH_STAFF_MINIMUMS, SEARCH_STAFF_ATTRIBUTES, SEARCH_STAFF_RANGES_SET, SEARCH_STAFF_RANGES_SHOWN);
}

/// Stores a nation id filter when it lies from -1 to the nation count less one (returns 1); otherwise the error box at
/// `line` and 0.
unsafe fn search_set_nation(this: usize, field: usize, nation: i32, line: u32) -> u32 {
    if nation < -1 || nation > read_i32(G_NATION_COUNT) - 1 {
        show_error_at(SEARCH_FILTERS_SOURCE, line);
        return 0;
    }
    write_i32(this + field, nation);
    1
}

/// Stores a byte filter when it lies from `low` to `high` (returns 1); otherwise the error box at `line` and 0.
unsafe fn search_set_byte(this: usize, field: usize, value: i8, low: i8, high: i8, line: u32) -> u32 {
    if value < low || value > high {
        show_error_at(SEARCH_FILTERS_SOURCE, line);
        return 0;
    }
    write_i8(this + field, value);
    1
}

/// The nationality nation filter (-1 any), line 0x14a.
/// Original: `SearchFilters::set_nationality_nation` (0x806ff0, search_filters.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn search_filters_set_nationality_nation(this: usize, nation: i32) -> u32 {
    search_set_nation(this, SEARCH_NATIONALITY_NATION, nation, 0x14a)
}

/// The nationality region filter, 1 to 14 (line 0x161).
/// Original: `SearchFilters::set_nationality_region` (0x807090, search_filters.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn search_filters_set_nationality_region(this: usize, region: i8) -> u32 {
    search_set_byte(this, SEARCH_NATIONALITY_REGION, region, 1, 14, 0x161)
}

/// The language option, 1 to 3 (line 0x178).
/// Original: `SearchFilters::set_language_option` (0x807130, search_filters.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn search_filters_set_language_option(this: usize, option: i8) -> u32 {
    search_set_byte(this, SEARCH_LANGUAGE, option, 1, 3, 0x178)
}

/// The nation where the player is based (-1 any), line 0x18f.
/// Original: `SearchFilters::set_based_nation` (0x8071d0, search_filters.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn search_filters_set_based_nation(this: usize, nation: i32) -> u32 {
    search_set_nation(this, SEARCH_BASED_NATION, nation, 0x18f)
}

/// The region where the player is based, 1 to 14 (line 0x1a6).
/// Original: `SearchFilters::set_based_region` (0x807270, search_filters.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn search_filters_set_based_region(this: usize, region: i8) -> u32 {
    search_set_byte(this, SEARCH_BASED_REGION, region, 1, 14, 0x1a6)
}

/// The player value band, -1 to 7 (line 0x1bd).
/// Original: `SearchFilters::set_value_band` (0x807310, search_filters.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn search_filters_set_value_band(this: usize, band: i8) -> u32 {
    search_set_byte(this, SEARCH_VALUE_BAND, band, -1, 7, 0x1bd) // sx20_13: the value bands
}

/// The staff job option, 1 to 9 (line 0x1d4).
/// Original: `SearchFilters::set_job_filter` (0x8073b0, search_filters.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn search_filters_set_job_filter(this: usize, job: i8) -> u32 {
    search_set_byte(this, SEARCH_JOB, job, 1, 9, 0x1d4)
}

/// The minimum set for attribute `attribute`: of the non-player table (1 to 12) when `staff` is not 0, else of the
/// player table (1 to 31); an attribute out of range shows the error box (lines 0x3c6 / 0x3d2) and answers 1.
/// Original: `SearchFilters::get_attribute_minimum` (0x808190, search_filters.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn search_filters_get_attribute_minimum(this: usize, attribute: i8, staff: u32) -> u8 {
    let (minimums, count, line) = if staff != 0 {
        (SEARCH_STAFF_MINIMUMS, SEARCH_STAFF_ATTRIBUTES, 0x3c6)
    } else {
        (SEARCH_PLAYER_MINIMUMS, SEARCH_PLAYER_ATTRIBUTES, 0x3d2)
    };
    if attribute < 1 || attribute as usize > count {
        show_error_at(SEARCH_FILTERS_SOURCE, line);
        return 1;
    }
    read_u8(this + minimums + attribute as usize) // sx20_13: the minimum read
}

/// Sets the minimum of attribute `attribute` (the non-player table when `staff` is not 0, else the player one) to
/// `value`, from 1 to that attribute's maximum; out of range, the error box (lines 0x3ec / 0x420) and 0; the same
/// value, 0. On a full-range attribute (maximum 20) the first raise from 1 counts the range (both counters up) and
/// switches the table's range option on (high 0x100000 for non-players, 0x40000 for players); going back to 1
/// counts it off (the set counter down, the shown one down while above 1) and, with none left, clears the shown
/// counter and switches the "no ranges" option (high 0x80000 / 0x20000). Returns 1.
/// Original: `SearchFilters::set_attribute_minimum` (0x808280, search_filters.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn search_filters_set_attribute_minimum(this: usize, attribute: i8, value: i8, staff: u32) -> u32 {
    let (minimums, maximums, count, set, shown, on, off, line) = if staff != 0 {
        (SEARCH_STAFF_MINIMUMS, SEARCH_STAFF_MAXIMUMS, SEARCH_STAFF_ATTRIBUTES, SEARCH_STAFF_RANGES_SET, SEARCH_STAFF_RANGES_SHOWN, 0x100000, 0x80000, 0x3ec)
    } else {
        (SEARCH_PLAYER_MINIMUMS, SEARCH_PLAYER_MAXIMUMS, SEARCH_PLAYER_ATTRIBUTES, SEARCH_PLAYER_RANGES_SET, SEARCH_PLAYER_RANGES_SHOWN, 0x40000, 0x20000, 0x420)
    };
    if attribute < 1 || attribute as usize > count || value < 1 || value > read_i8(this + maximums + attribute as usize) {
        show_error_at(SEARCH_FILTERS_SOURCE, line);
        return 0;
    }
    let minimum = this + minimums + attribute as usize;
    let top = read_u8(this + maximums + attribute as usize) == SEARCH_ATTRIBUTE_TOP;
    let current = read_i8(minimum);
    if current == value {
        return 0;
    }
    if current == 1 && top {
        write_u8(this + set, read_u8(this + set).wrapping_add(1));
        write_u8(this + shown, read_u8(this + shown).wrapping_add(1));
        search_set_option_flag(this, 0, on); // sx20_13: the range option on
    } else if value == 1 && top {
        let left = read_u8(this + set).wrapping_sub(1);
        write_u8(this + set, left);
        if read_i8(this + shown) > value {
            write_u8(this + shown, read_u8(this + shown).wrapping_sub(1));
        }
        if left == 0 {
            write_u8(this + shown, 0);
            search_set_option_flag(this, 0, off);
        }
    }
    write_i8(minimum, value);
    1
}

/// Runs a search filter routine side by side over the filter record: the record and the answer must match.
unsafe fn search_filters_compare<R: PartialEq + core::fmt::Debug>(name: &str, this: usize, original: impl FnOnce() -> R, copy: impl FnOnce() -> R) -> R {
    let (r, c, same, mo, mr) = crate::comparison::side_by_side(&[(this, SEARCH_FILTERS_SIZE)], original, copy);
    SCOREBOARD.record(same && r == c, || format!("{} (search filters {:#x}): original={:?} rust={:?} {:?}", name, this, r, c, first_differences(&mo, &mr)));
    r
}

/// Comparer for `search_filters_set_defaults`.
pub(crate) unsafe extern "thiscall" fn compare_search_filters_set_defaults(this: usize) {
    let original = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    search_filters_compare("search_filters_set_defaults", this, || original(this), || search_filters_set_defaults(this))
}

/// Comparer for `search_filters_reset_general_filters`.
pub(crate) unsafe extern "thiscall" fn compare_search_filters_reset_general_filters(this: usize) {
    let original = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    search_filters_compare("search_filters_reset_general_filters", this, || original(this), || search_filters_reset_general_filters(this))
}

/// Comparer for `search_filters_reset_attribute_filters`.
pub(crate) unsafe extern "thiscall" fn compare_search_filters_reset_attribute_filters(this: usize) {
    let original = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    search_filters_compare("search_filters_reset_attribute_filters", this, || original(this), || search_filters_reset_attribute_filters(this))
}

/// Runs a setter side by side, or only the original when the value would show the error box.
unsafe fn search_filters_compare_setter<V: Copy>(name: &str, this: usize, value: V, valid: bool, original: unsafe extern "thiscall" fn(usize, V) -> u32,
                                                  copy: unsafe extern "thiscall" fn(usize, V) -> u32) -> u32 {
    if !valid {
        return original(this, value); // the error box: once
    }
    search_filters_compare(name, this, || original(this, value), || copy(this, value))
}

/// Comparer for `search_filters_set_nationality_nation`.
pub(crate) unsafe extern "thiscall" fn compare_search_filters_set_nationality_nation(this: usize, nation: i32) -> u32 {
    search_filters_compare_setter("search_filters_set_nationality_nation", this, nation, nation >= -1 && nation < read_i32(G_NATION_COUNT),
        original_fn::<unsafe extern "thiscall" fn(usize, i32) -> u32>(%%I3%%), search_filters_set_nationality_nation)
}

/// Comparer for `search_filters_set_nationality_region`.
pub(crate) unsafe extern "thiscall" fn compare_search_filters_set_nationality_region(this: usize, region: i8) -> u32 {
    search_filters_compare_setter("search_filters_set_nationality_region", this, region, (1..=14).contains(&region),
        original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(%%I4%%), search_filters_set_nationality_region)
}

/// Comparer for `search_filters_set_language_option`.
pub(crate) unsafe extern "thiscall" fn compare_search_filters_set_language_option(this: usize, option: i8) -> u32 {
    search_filters_compare_setter("search_filters_set_language_option", this, option, (1..=3).contains(&option),
        original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(%%I5%%), search_filters_set_language_option)
}

/// Comparer for `search_filters_set_based_nation`.
pub(crate) unsafe extern "thiscall" fn compare_search_filters_set_based_nation(this: usize, nation: i32) -> u32 {
    search_filters_compare_setter("search_filters_set_based_nation", this, nation, nation >= -1 && nation < read_i32(G_NATION_COUNT),
        original_fn::<unsafe extern "thiscall" fn(usize, i32) -> u32>(%%I6%%), search_filters_set_based_nation)
}

/// Comparer for `search_filters_set_based_region`.
pub(crate) unsafe extern "thiscall" fn compare_search_filters_set_based_region(this: usize, region: i8) -> u32 {
    search_filters_compare_setter("search_filters_set_based_region", this, region, (1..=14).contains(&region),
        original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(%%I7%%), search_filters_set_based_region)
}

/// Comparer for `search_filters_set_value_band`.
pub(crate) unsafe extern "thiscall" fn compare_search_filters_set_value_band(this: usize, band: i8) -> u32 {
    search_filters_compare_setter("search_filters_set_value_band", this, band, (-1..=7).contains(&band),
        original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(%%I8%%), search_filters_set_value_band)
}

/// Comparer for `search_filters_set_job_filter`.
pub(crate) unsafe extern "thiscall" fn compare_search_filters_set_job_filter(this: usize, job: i8) -> u32 {
    search_filters_compare_setter("search_filters_set_job_filter", this, job, (1..=9).contains(&job),
        original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(%%I9%%), search_filters_set_job_filter)
}

/// Comparer for `search_filters_get_attribute_minimum`.
pub(crate) unsafe extern "thiscall" fn compare_search_filters_get_attribute_minimum(this: usize, attribute: i8, staff: u32) -> u8 {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, i8, u32) -> u8>(%%I10%%);
    let count = if staff != 0 { SEARCH_STAFF_ATTRIBUTES } else { SEARCH_PLAYER_ATTRIBUTES };
    if attribute < 1 || attribute as usize > count {
        return original(this, attribute, staff); // the error box: once
    }
    search_filters_compare("search_filters_get_attribute_minimum", this, || original(this, attribute, staff),
        || search_filters_get_attribute_minimum(this, attribute, staff))
}

/// Comparer for `search_filters_set_attribute_minimum`.
pub(crate) unsafe extern "thiscall" fn compare_search_filters_set_attribute_minimum(this: usize, attribute: i8, value: i8, staff: u32) -> u32 {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, u32) -> u32>(%%I11%%);
    let (count, maximums) = if staff != 0 { (SEARCH_STAFF_ATTRIBUTES, SEARCH_STAFF_MAXIMUMS) } else { (SEARCH_PLAYER_ATTRIBUTES, SEARCH_PLAYER_MAXIMUMS) };
    if attribute < 1 || attribute as usize > count || value < 1 || value > read_i8(this + maximums + attribute as usize) {
        return original(this, attribute, value, staff); // the error box: once
    }
    search_filters_compare("search_filters_set_attribute_minimum", this, || original(this, attribute, value, staff),
        || search_filters_set_attribute_minimum(this, attribute, value, staff))
}
