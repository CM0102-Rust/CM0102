// ---------- batch %%LEVA%%: the tables of the Norwegian, Swedish and Finnish Premier Divisions (nor_prm.cpp, swe_prm.cpp, fin_prm.cpp) ----------

/// Creates the Norwegian Premier Division table: 14 clubs, those of the division in database order; the prize fund
/// by a share of 50,000 per place; no TV money.
///
/// `NorPrm::create_table()`: the body of `eng_prm_fill_league_table` (checked instruction by instruction):
/// `.set_club_count()` = 14, the table (`exe::crt_malloc` of 14 × 0x3b; out of memory: the box of nor_prm.cpp
/// (0xa6977c) line 0x1a7 and 0); the clubs whose division (`club::DIVISION`) has the id of
/// `KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION`, in database order, each into the next row (`exe::league_init_table_row`:
/// push 0; push 0; push club; push row -> thiscall, ret 0x10) until full; not full: 0. Full: the prize fund
/// (`exe::league_calc_prize_fund`, push 50,000), `.set_prize_mode()` = 1 and 1.
/// Original: `NorPrm::create_table` (0x7925e0, nor_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_prm_create_table(this: usize) -> u32 {
    covered!("mwe_6 tippeligaen table");
    eng_prm_fill_league_table(League(this), 0xe, 0xa6977c, 0x1a7, KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, 0xc350, || ()) // mwe6: the Tippeligaen prize share
}

/// Creates the Swedish Premier Division table: 14 clubs, those of the division in database order; the prize fund by a
/// share of 50,000 per place; no TV money.
///
/// `SwePrm::create_table()`: the same body (`eng_prm_fill_league_table`) with `KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION`,
/// the box of swe_prm.cpp (0xa8b578) line 0x143 and the share 50,000.
/// Original: `SwePrm::create_table` (0x890430, swe_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_prm_create_table(this: usize) -> u32 {
    covered!("mwe_6 allsvenskan table");
    eng_prm_fill_league_table(League(this), 0xe, 0xa8b578, 0x143, KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, 0xc350, || ()) // mwe6: the Allsvenskan prize share
}

/// Creates the Finnish Premier Division table: 12 clubs, those of the division in database order; the prize fund by a
/// share of 35,000 per place; no TV money.
///
/// `FinPrm::create_table()`: the same body (`eng_prm_fill_league_table`) with 12 rows,
/// `KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION`, the box of fin_prm.cpp (0x9c8254) line 0x157 and the share 35,000.
/// Original: `FinPrm::create_table` (0x595b70, fin_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_prm_create_table(this: usize) -> u32 {
    covered!("mwe_6 veikkausliiga table");
    eng_prm_fill_league_table(League(this), 0xc, 0x9c8254, 0x157, KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION, 0x88b8, || ()) // mwe6: the Veikkausliiga prize share
}

/// Comparer for `nor_prm_create_table`: only the original runs (it allocates the table), against the plan of
/// `eng_prm_compare_league_table` (the division's clubs in database order, the count, the return value and the prize
/// fund).
pub(crate) unsafe extern "thiscall" fn compare_nor_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    eng_prm_compare_league_table("nor_prm_create_table", League(this), 0xe, KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, 0xc350, || o(this))
}

/// Comparer for `swe_prm_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    eng_prm_compare_league_table("swe_prm_create_table", League(this), 0xe, KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, 0xc350, || o(this))
}

/// Comparer for `fin_prm_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_fin_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    eng_prm_compare_league_table("fin_prm_create_table", League(this), 0xc, KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION, 0x88b8, || o(this))
}
