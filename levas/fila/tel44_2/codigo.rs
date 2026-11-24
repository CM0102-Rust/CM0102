// ---------- batch %%LEVA%%: the sort orders of an award shortlist (award_shortlist.cpp) ----------

/// The qsort order of a shortlist by the players' names: each entry's player (the staff id at +0; none for −1) named
/// by `exe::database_format_staff_name` (2000 bytes, style 4; an empty name without a player), the two names compared
/// by `exe::utils_string_compare` (mode 1, 0).
/// Original: `award_shortlist_compare_by_name` (0x418340, award_shortlist.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_shortlist_compare_by_name(a: *const usize, b: *const usize) -> i32 {
    let named = |entry: usize, buf: &mut [u8; 0x7d0]| {
        let id = read_i32(entry);
        if id != -1 {
            exe::database_format_staff_name(staff_at(id as isize), buf.as_mut_ptr() as usize, 0x7d0, 4);
        }
    };
    let (mut name_a, mut name_b) = ([0u8; 0x7d0], [0u8; 0x7d0]);
    named(a as usize, &mut name_a);
    named(b as usize, &mut name_b);
    exe::utils_string_compare(name_a.as_ptr(), name_b.as_ptr(), 1, 0) // tel44_2: a's name against b's
}

/// The qsort order of a shortlist by score, the highest first: −1 when a's score (float, `ENTRY_SCORE`) is above b's,
/// 1 when below, otherwise 0 (also unordered).
/// Original: `award_shortlist_compare_by_score` (0x4183f0, award_shortlist.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_shortlist_compare_by_score(a: *const usize, b: *const usize) -> i32 {
    use award_shortlist_fields as asl;
    let (sa, sb) = (read_f32(a as usize + asl::ENTRY_SCORE), read_f32(b as usize + asl::ENTRY_SCORE));
    if sa > sb {
        -1 // tel44_2: the higher score first
    } else if sb > sa {
        1
    } else {
        0
    }
}

/// The qsort order of a shortlist by votes, the most voted first: b's votes minus a's (signed words, `ENTRY_VOTES`).
/// Original: `award_shortlist_compare_by_votes` (0x418420, award_shortlist.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_shortlist_compare_by_votes(a: *const usize, b: *const usize) -> i32 {
    use award_shortlist_fields as asl;
    read_i16(b as usize + asl::ENTRY_VOTES) as i32 - read_i16(a as usize + asl::ENTRY_VOTES) as i32 // tel44_2: the most votes first
}

compare_order!(compare_award_shortlist_compare_by_name, award_shortlist_compare_by_name, IDX_AWARD_SHORTLIST_COMPARE_BY_NAME);
compare_order!(compare_award_shortlist_compare_by_score, award_shortlist_compare_by_score, IDX_AWARD_SHORTLIST_COMPARE_BY_SCORE);
compare_order!(compare_award_shortlist_compare_by_votes, award_shortlist_compare_by_votes, IDX_AWARD_SHORTLIST_COMPARE_BY_VOTES);
