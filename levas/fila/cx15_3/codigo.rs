// ---------- cx15_3: the UEFA seeding's sort orders: nations by coefficient, clubs for the seeding and for the qualifiers news ----------

/// qsort order of the seeding nation entries (0x12 bytes) by the five-season coefficient (the float at +8), highest
/// first: -1 when `a`'s is greater, 0 when equal, otherwise 1 (also when either is not a number: the x87 compare is
/// unordered, as in the original).
/// Original: `uefa_compare_nations_by_coefficient` (0x905cb0, uefa_seeding.cpp, cdecl).
pub(crate) unsafe extern "C" fn uefa_compare_nations_by_coefficient(a: usize, b: usize) -> i32 {
    covered!("cx15_3 nations by coefficient");
    let (x, y) = (read_f32(a + 8), read_f32(b + 8)); // the coefficient total written by roll_coefficients
    if x > y {
        -1
    } else if !(x >= y) {
        1
    } else {
        0
    }
}

/// qsort and bsearch order of the 12-byte European club entries (`european_club_entry`): by nation id (+4, ascending,
/// the dword difference); in the same nation, 0 when either reputation band (+8) is 0xff (an entry used as a search key
/// for the nation only); otherwise by division reputation (+9, a signed word, highest first), then by last league
/// position (`european_club_entry::LAST_POSITION`, signed byte, lowest first), then by reputation band (+8, signed
/// byte, highest first).
/// Original: `uefa_compare_seed_clubs` (0x905ce0, uefa_seeding.cpp, cdecl).
pub(crate) unsafe extern "C" fn uefa_compare_seed_clubs(a: usize, b: usize) -> i32 {
    let nation = read_u32(a + 4).wrapping_sub(read_u32(b + 4)) as i32;
    if nation != 0 {
        return nation;
    }
    let (band_a, band_b) = (read_u8(a + 8), read_u8(b + 8)); // the club's reputation / 500
    if band_a == 0xff || band_b == 0xff {
        return 0;
    }
    let division = read_i16(b + 9) as i32 - read_i16(a + 9) as i32; // the reputation of the club's division
    if division != 0 {
        return division;
    }
    let position = read_i8(a + european_club_entry::LAST_POSITION) as i32 - read_i8(b + european_club_entry::LAST_POSITION) as i32;
    if position != 0 {
        return position;
    }
    covered!("cx15_3 seed clubs by reputation");
    band_b as i8 as i32 - band_a as i8 as i32
}

/// The database's order of two clubs by name (qsort comparer over club pointers, cdecl(a, b)).
const FN_DATABASE_COMPARE_CLUBS_BY_NAME: usize = 0x540080;

/// qsort order of the qualified clubs of the European qualifiers news (entries that start with the club pointer): by
/// the nation id of the clubs' nations (`club::NATION`, only when the nation records differ, ascending); then by the
/// reputation of the club's European competition (`club::CONTINENTAL_COMP_ID`, `competition_reputation`, highest first,
/// only when the ids differ); then by the entry round (`club::EURO_SEEDING`, signed byte, lowest first); then by name
/// (`FN_DATABASE_COMPARE_CLUBS_BY_NAME` with the two entries).
/// Original: `uefa_compare_qualified_clubs` (0x906410, uefa_seeding.cpp, cdecl).
pub(crate) unsafe extern "C" fn uefa_compare_qualified_clubs(a: usize, b: usize) -> i32 {
    let (ca, cb) = (Club(read_usize(a)), Club(read_usize(b)));
    let (na, nb) = (ca.nation(), cb.nation());
    if na.addr() != nb.addr() {
        return read_u32(na.addr()).wrapping_sub(read_u32(nb.addr())) as i32;
    }
    let (ka, kb) = (ca.continental_comp_id(), cb.continental_comp_id());
    if ka != kb {
        covered!("cx15_3 qualified clubs by competition");
        return competition_reputation(kb as i32) as i32 - competition_reputation(ka as i32) as i32;
    }
    let (ra, rb) = (ca.euro_seeding(), cb.euro_seeding());
    if ra != rb {
        return ra as i8 as i32 - rb as i8 as i32;
    }
    let by_name: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(FN_DATABASE_COMPARE_CLUBS_BY_NAME);
    by_name(a, b)
}

/// Comparers of the three sort orders: both run on the same pair and the results must be equal.
pub(crate) unsafe extern "C" fn compare_uefa_compare_nations_by_coefficient(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I0%%)(a, b);
    let n = uefa_compare_nations_by_coefficient(a, b);
    SCOREBOARD.record(r == n, || format!("uefa_compare_nations_by_coefficient({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

pub(crate) unsafe extern "C" fn compare_uefa_compare_seed_clubs(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = uefa_compare_seed_clubs(a, b);
    SCOREBOARD.record(r == n, || format!("uefa_compare_seed_clubs({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

pub(crate) unsafe extern "C" fn compare_uefa_compare_qualified_clubs(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = uefa_compare_qualified_clubs(a, b);
    SCOREBOARD.record(r == n, || format!("uefa_compare_qualified_clubs({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}
