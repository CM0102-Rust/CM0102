// ---------- batch %%LEVA%%: the constructors of the Belgian and Dutch Super Cups and the German League Cup (bel_super.cpp, hol_super.cpp, ger_lge_cup.cpp) ----------

pub(crate) const BEL_SUPER_CUP_PARAMS: NationCup = NationCup {
    vtable: 0x967aa8,
    bench: Some((7, 7)),
    discipline: discipline_rule::BELGIUM_CUP, // mwc17: the Belgian Super Cup discipline rule
    entrants: 0x4212f0,
    zero_fails: false,
    predicted: true,
    source_file: 0x989ccc,
    box_lines: [0x44, 0x52, 0x59],
};

/// The Belgian Super Cup is born with discipline rule 5, the bench (7 substitutes named, 7 substitutions); with no
/// saved state, the entrants (BelSuper::select_teams), the round table, the tie tree (a failure is a negative return),
/// the statistics and the predicted positions (`construct_nation_cup`).
/// Original: `BelSuper::BelSuper` (0x420ef0, bel_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bel_super(this: usize, year: u32, record: usize) -> usize {
    construct_nation_cup(Cup(this), year, record, &BEL_SUPER_CUP_PARAMS)
}

/// Comparer for `construct_bel_super`: only the original runs (`compare_nation_cup_ctor`).
unsafe extern "thiscall" fn compare_construct_bel_super(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_nation_cup_ctor("construct_bel_super", Cup(this), year, record, &BEL_SUPER_CUP_PARAMS, || o(this, year, record))
}

pub(crate) const HOL_SUPER_CUP_PARAMS: NationCup = NationCup {
    vtable: 0x96b9d4,
    bench: None,
    discipline: discipline_rule::HOLLAND, // mwc17: the Dutch Super Cup discipline rule
    entrants: 0x5f8c10,
    zero_fails: false,
    predicted: false,
    source_file: 0x9cefdc,
    box_lines: [0x3f, 0x4d, 0x54],
};

/// The Dutch Super Cup is born with discipline rule 15; with no saved state, the entrants (HolSuper::select_teams), the
/// round table, the tie tree (a failure is a negative return), the statistics (no predicted positions)
/// (`construct_nation_cup`).
/// Original: `HolSuper::HolSuper` (0x5f8820, hol_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_hol_super(this: usize, year: u32, record: usize) -> usize {
    construct_nation_cup(Cup(this), year, record, &HOL_SUPER_CUP_PARAMS)
}

/// Comparer for `construct_hol_super`: only the original runs (`compare_nation_cup_ctor`).
unsafe extern "thiscall" fn compare_construct_hol_super(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_nation_cup_ctor("construct_hol_super", Cup(this), year, record, &HOL_SUPER_CUP_PARAMS, || o(this, year, record))
}

pub(crate) const GER_LGE_CUP_CUP_PARAMS: NationCup = NationCup {
    vtable: 0x96b1b0,
    bench: None,
    discipline: discipline_rule::GERMANY_LEAGUE, // mwc17: the German League Cup discipline rule
    entrants: 0x5dc700,
    zero_fails: false,
    predicted: true,
    source_file: 0x9cd450,
    box_lines: [0x3f, 0x4d, 0x54],
};

/// The German League Cup is born with discipline rule 13; with no saved state, the entrants
/// (GerLgeCup::collect_entrants), the round table, the tie tree (a failure is a negative return), the statistics and
/// the predicted positions (`construct_nation_cup`).
/// Original: `GerLgeCup::GerLgeCup` (0x5dc0e0, ger_lge_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ger_lge_cup(this: usize, year: u32, record: usize) -> usize {
    construct_nation_cup(Cup(this), year, record, &GER_LGE_CUP_CUP_PARAMS)
}

/// Comparer for `construct_ger_lge_cup`: only the original runs (`compare_nation_cup_ctor`).
unsafe extern "thiscall" fn compare_construct_ger_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    compare_nation_cup_ctor("construct_ger_lge_cup", Cup(this), year, record, &GER_LGE_CUP_CUP_PARAMS, || o(this, year, record))
}
