// ---------- batch %%LEVA%%: the Portuguese Super Cup constructor (por_super.cpp) ----------

pub(crate) const POR_SUPER_CUP_PARAMS: NationCup = NationCup {
    vtable: 0x96e90c,
    bench: None,
    discipline: discipline_rule::PORTUGAL, // mwe21: the Portuguese Super Cup discipline rule
    entrants: 0x7d3260,
    zero_fails: false,
    predicted: false,
    source_file: 0xa787d4,
    box_lines: [0x3f, 0x4d, 0x54],
};

/// Rule: the Portuguese Super Cup (Supertaça) is born with discipline rule 23; with no saved state, its two clubs
/// (PorSuper::select_teams), the round table, the tie tree (a failure is a negative return) and the statistics, with no
/// predicted positions (`construct_nation_cup`, the HolSuper body checked instruction by instruction: only the
/// discipline rule, the vtable, the clubs routine and the file change).
/// Original: `PorSuper::PorSuper` (0x7d2e40, por_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_por_super(this: usize, year: u32, record: usize) -> usize {
    construct_nation_cup(Cup(this), year, record, &POR_SUPER_CUP_PARAMS)
}

/// Comparer for `construct_por_super`: only the original runs (`compare_nation_cup_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_por_super(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_POR_SUPER);
    compare_nation_cup_ctor("construct_por_super", Cup(this), year, record, &POR_SUPER_CUP_PARAMS, || o(this, year, record))
}
