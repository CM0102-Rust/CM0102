// ---------- batch %%LEVA%%: the constructors of the Irish President's Cup, the Northern Irish Charity Shield and the Northern Irish Cup (ire_pres_cup.cpp, nir_charity.cpp, nir_cup.cpp) ----------

/// The President's Cup's constructor: the vtable 0x96bf74, the bench of the base, discipline rule
/// `discipline_rule::IRELAND`, the entrants IrePresCup::collect_entrants (0x639670; fails when negative), no prize
/// fund, no final ranking; the error box lines 0x3f, 0x4d and 0x54 of ire_pres_cup.cpp.
pub(crate) const IRISH_PRESIDENTS_CUP_CONSTRUCTOR: IrishCupConstructor = IrishCupConstructor { vtable: 0x96bf74, bench: None, discipline: discipline_rule::IRELAND,
    entrants: 0x639670, fails_when_negative: true, prize: None, rank: false, file: 0x9ea000, rows: [0x3f, 0x4d, 0x54] };

/// Rule: the President's Cup is a cup of one nation's clubs of the game's year, with no prize fund of its own and the
/// base's bench, under the Irish discipline rules; with nothing saved, its two clubs, the round, the tie tree and the
/// statistics are made (no final ranking).
///
/// `construct_irish_cup` with `IRISH_PRESIDENTS_CUP_CONSTRUCTOR` (checked instruction by instruction against
/// FraSuper::FraSuper 0x5bf320: only the vtable, the discipline rule, the entrant routine and the file change).
/// Original: `IrePresCup::IrePresCup` (0x639280, ire_pres_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_pres_cup(this: usize, year: u32, record: usize) -> usize {
    construct_irish_cup(Cup(this), year, record, &IRISH_PRESIDENTS_CUP_CONSTRUCTOR, "mwa_8 presidents cup new") // mwa8: the President's Cup
}

unsafe extern "thiscall" fn compare_construct_ire_pres_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_irish_cup_constructor("construct_ire_pres_cup", this, year, record, &IRISH_PRESIDENTS_CUP_CONSTRUCTOR, || o(this, year, record))
}

/// The Northern Irish Charity Shield's constructor: the vtable 0x96da24, 3 substitutes on the bench and 3
/// substitutions, discipline rule `discipline_rule::NORTHERN_IRELAND`, the clubs NirCharity::select_teams (0x78a1e0;
/// fails when negative), no prize fund, no final ranking; the lines 0x43, 0x51 and 0x58 of nir_charity.cpp.
pub(crate) const NORTHERN_IRISH_CHARITY_SHIELD_CONSTRUCTOR: IrishCupConstructor = IrishCupConstructor { vtable: 0x96da24, bench: Some((3, 3)), discipline: discipline_rule::NORTHERN_IRELAND,
    entrants: 0x78a1e0, fails_when_negative: true, prize: None, rank: false, file: 0xa695f0, rows: [0x43, 0x51, 0x58] };

/// Rule: the Northern Irish Charity Shield is a cup of one nation's clubs of the game's year, with no prize fund of its
/// own, 3 substitutes on the bench and 3 substitutions, under the Northern Irish discipline rules; with nothing saved,
/// its two clubs, the round, the tie tree and the statistics are made (no final ranking).
///
/// `construct_irish_cup` with `NORTHERN_IRISH_CHARITY_SHIELD_CONSTRUCTOR` (checked instruction by instruction against
/// SpaSuper::SpaSuper 0x8586b0: the same steps with the year and the record written in another order).
/// Original: `NirCharity::NirCharity` (0x789df0, nir_charity.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_nir_charity(this: usize, year: u32, record: usize) -> usize {
    construct_irish_cup(Cup(this), year, record, &NORTHERN_IRISH_CHARITY_SHIELD_CONSTRUCTOR, "mwa_8 northern irish charity shield new") // mwa8: the Charity Shield
}

unsafe extern "thiscall" fn compare_construct_nir_charity(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_irish_cup_constructor("construct_nir_charity", this, year, record, &NORTHERN_IRISH_CHARITY_SHIELD_CONSTRUCTOR, || o(this, year, record))
}

/// The Northern Irish Cup's constructor: the vtable 0x96dac4, 3 substitutes on the bench and 3 substitutions,
/// discipline rule `discipline_rule::NORTHERN_IRELAND`, the entrants NirCup::collect_entrants (0x78ac90; fails on 0),
/// a prize fund of 0, the final ranking; the lines 0x43, 0x51 and 0x58 of nir_cup.cpp.
pub(crate) const NORTHERN_IRISH_CUP_CONSTRUCTOR: IrishCupConstructor = IrishCupConstructor { vtable: 0x96dac4, bench: Some((3, 3)), discipline: discipline_rule::NORTHERN_IRELAND,
    entrants: 0x78ac90, fails_when_negative: false, prize: Some(0), rank: true, file: 0xa69628, rows: [0x43, 0x51, 0x58] };

/// Rule: the Northern Irish Cup is a cup of one nation's clubs of the game's year, with a prize fund of 0 (the rounds
/// pay fixed prizes), 3 substitutes on the bench and 3 substitutions, under the Northern Irish discipline rules; with
/// nothing saved, its 32 entrants, the rounds, the tie tree, the statistics and the final ranking are made.
///
/// `construct_irish_cup` with `NORTHERN_IRISH_CUP_CONSTRUCTOR` (checked instruction by instruction against
/// FraLgeCup::FraLgeCup 0x5bc980 and JapEmpCup::JapEmpCup).
/// Original: `NirCup::NirCup` (0x78a5b0, nir_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_nir_cup(this: usize, year: u32, record: usize) -> usize {
    construct_irish_cup(Cup(this), year, record, &NORTHERN_IRISH_CUP_CONSTRUCTOR, "mwa_8 irish cup new") // mwa8: the Irish Cup
}

unsafe extern "thiscall" fn compare_construct_nir_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    compare_irish_cup_constructor("construct_nir_cup", this, year, record, &NORTHERN_IRISH_CUP_CONSTRUCTOR, || o(this, year, record))
}
