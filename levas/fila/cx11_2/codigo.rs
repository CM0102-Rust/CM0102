// ---------- cx11_2: the constructors of the Italian Serie C2 groups A and B (ita_ser_c2a.cpp, ita_ser_c2b.cpp) ----------

/// What the Serie C2/A constructor passes to `construct_italian_division`: the vtable, the league and 2 stages (the
/// promotion play-off and the relegation play-out), its rules, table and real 2001/02 calendar, the source file and the
/// box lines (the same as Serie C1/A's).
pub(crate) const SERIE_C2A_DIVISION: ItalianDivision = ItalianDivision {
    vtabela: 0x96c69c,
    series: false,
    stages: 2,
    rules: 0x653b90,
    table: 0x653a90,
    fixed: 0x6545c0, // cx11_2: the real 2001/02 C2/A calendar
    file: 0x9ea654,
    rows: [0x49, 0x50, 0x5a, 0x63, 0x6a],
};

/// The same for Serie C2/B (the box lines are also Serie C1/A's).
pub(crate) const SERIE_C2B_DIVISION: ItalianDivision = ItalianDivision {
    vtabela: 0x96c750,
    series: false,
    stages: 2,
    rules: 0x658050,
    table: 0x657f50,
    fixed: 0x658750, // cx11_2: the real 2001/02 C2/B calendar
    file: 0x9ea75c,
    rows: [0x49, 0x50, 0x5a, 0x63, 0x6a],
};

/// Builds the Italian Serie C2/A (`construct_italian_division`): the league base, the database record, the vtable, the
/// year and discipline rule 0x11; with no saved state, stage 0xff, current stage −1, 2 stages (an 8-byte stage vector),
/// the rules, the table, the real 2001/02 calendar by club names, the fixtures, the order, the statistics and the
/// predicted positions. Each step that returns 0 shows its error box (ita_ser_c2a.cpp, lines 0x49, 0x50, 0x5a, 0x63 and
/// 0x6a) and stops. Returns this.
/// Original: `ItaSerC2a::ItaSerC2a` (0x652e90, ita_ser_c2a.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ita_ser_c2a(this: usize, year: u32, record: usize) -> usize {
    covered!("cx11_2 serie c2 built");
    construct_italian_division(League(this), year, record, &SERIE_C2A_DIVISION)
}

/// Builds the Italian Serie C2/B, as C2/A (ita_ser_c2b.cpp, the same box lines).
/// Original: `ItaSerC2b::ItaSerC2b` (0x657350, ita_ser_c2b.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ita_ser_c2b(this: usize, year: u32, record: usize) -> usize {
    construct_italian_division(League(this), year, record, &SERIE_C2B_DIVISION)
}

/// Comparer for `construct_ita_ser_c2a`: only the original runs (it allocates), checked by `compare_italian_constructor`.
pub(crate) unsafe extern "thiscall" fn compare_construct_ita_ser_c2a(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_italian_constructor("construct_ita_ser_c2a", League(this), year, record, &SERIE_C2A_DIVISION, || o(this, year, record))
}

/// Comparer for `construct_ita_ser_c2b`: the same.
pub(crate) unsafe extern "thiscall" fn compare_construct_ita_ser_c2b(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_italian_constructor("construct_ita_ser_c2b", League(this), year, record, &SERIE_C2B_DIVISION, || o(this, year, record))
}
