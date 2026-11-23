use super::*;

/// Rule: the Northern Irish Charity Shield is a single match between two clubs, drawn on 5/7 and played on 5/8 (moved
/// to weekday 5) at the first club's ground [inference: home side code 0], with tie rule bits 0x200 (no draw news
/// [inference]); category 6; no prize.
///
/// The cup's virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table(stage, &rounds, &extra, &out)): only for stage
/// −1; `out` (when given) = 0, `rounds` = 1, `extra` = 0; the table (`exe::crt_malloc` of 0x68; out of memory, the
/// error box at line 0x99 and 0) with `NORTHERN_IRISH_CHARITY_SHIELD_ROUNDS`
/// (`eng_cc_cup_create_single_stage_round_table`, checked instruction by instruction). Returns the table.
/// Original: `NirCharity::create_round_table` (0x78a0a0, nir_charity.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn nir_charity_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &NORTHERN_IRISH_CHARITY_SHIELD_ROUNDS, 0xa695f0, 0x99, "mwa_5 northern irish charity shield rounds")
}

pub(crate) unsafe extern "thiscall" fn compare_nir_charity_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_NIR_CHARITY_CREATE_ROUND_TABLE);
    compare_cup_round_table("nir_charity_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| nir_charity_create_round_table(this, stage, r, e, s))
}

/// Rule: the Northern Irish Charity Shield is between the Irish Cup winner and the Premier Division champion; if the
/// same club won both, the Irish Cup runner-up takes its place; if one is missing, the error box.
/// Original: `NirCharity::select_teams` (0x78a1e0, nir_charity.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_charity_select_teams(this: usize) -> i32 {
    select_northern_irish_charity_teams(this).0
}

pub(crate) unsafe extern "thiscall" fn compare_nir_charity_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_NIR_CHARITY_SELECT_TEAMS);
    compare_team_list("nir_charity_select_teams", Cup(this), || o(this), || select_northern_irish_charity_teams(this))
}

/// Rule: at the season turn the Northern Irish Charity Shield is rebuilt: last edition's tables are freed, the
/// statistics archived, the year goes up by one and the two clubs, the rules and the tie tree are made again (no
/// predicted positions).
///
/// The knockout cups' turnover without the predicted positions (`usa_mls_all_stars_cup_start_new_season` with
/// `predicted` false, checked instruction by instruction: only the entrant routine, NirCharity::select_teams 0x78a1e0,
/// the file and the error box lines 0x125, 0x12c and 0x133 change).
/// Original: `NirCharity::start_new_season` (0x78a410, nir_charity.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_charity_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x78a1e0, 0xa695f0, [0x125, 0x12c, 0x133], 1, false, "mwa_5 northern irish charity shield turnover")
}

pub(crate) unsafe extern "thiscall" fn compare_nir_charity_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_CHARITY_START_NEW_SEASON);
    compare_cup_new_season("nir_charity_start_new_season", Cup(this), || o(this))
}

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

pub(crate) unsafe extern "thiscall" fn compare_construct_nir_charity(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_NIR_CHARITY);
    compare_irish_cup_constructor("construct_nir_charity", this, year, record, &NORTHERN_IRISH_CHARITY_SHIELD_CONSTRUCTOR, || o(this, year, record))
}

/// The body of `nir_charity_select_teams` (SpaSuper::select_teams's template, checked instruction by instruction: only
/// the two competitions and the file change; the error box lines are the same).
///
/// The two-entry list (`alloc_two_team_list`: `cup::TEAM_COUNT` = 2, `exe::crt_malloc` of 0xc; out of memory, the
/// error box at line 0xc2 and −1). The Irish Cup winner (`latest_placed_club`(`KEY_CLUB_COMP_NORTHERN_IRISH_CUP`, 0);
/// without it, line 0xcb and 0) in entry 0 and the Premier Division champion
/// (`KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION`, 0; without it, line 0xd4 and 0) in entry 1; if both are the
/// same club, entry 0 becomes the Irish Cup runner-up (without it, line 0xe0 and 0). Class and status 0; returns 1.
pub(crate) unsafe fn select_northern_irish_charity_teams(this: usize) -> (i32, i32) {
    const F: usize = 0xa695f0;
    let l = match alloc_two_team_list(Cup(this), F, 0xc2) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = latest_placed_club(KEY_CLUB_COMP_NORTHERN_IRISH_CUP, 0); // mwa5: the Irish Cup winner
    if !set_team_slot(l, 0, a, F, 0xcb) {
        return (0, 0);
    }
    let b = latest_placed_club(KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, 0);
    if !set_team_slot(l, 1, b, F, 0xd4) {
        return (0, 0);
    }
    if a == b {
        covered!("mwa_5 northern irish double");
        if !set_team_slot(l, 0, latest_placed_club(KEY_CLUB_COMP_NORTHERN_IRISH_CUP, 1), F, 0xe0) {
            return (0, 0);
        }
    }
    covered!("mwa_5 northern irish charity shield clubs");
    finish_two_team_list(l)
}

/// The Northern Irish Charity Shield: one match between two clubs, drawn on 5/7 and played on 5/8, without a draw
/// (tie rule bits 0x200: the built order [inference]); category 6; no prize.
pub(crate) const NORTHERN_IRISH_CHARITY_SHIELD_ROUNDS: [EngCcCupCupRound; 1] = [
    cql10r((5, 6, 0, 5), (5, 7, 0, 5, 1, 0), 0, (0, 0x200, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwa5: the Charity Shield match
];

/// The Northern Irish Charity Shield's constructor: the vtable 0x96da24, 3 substitutes on the bench and 3
/// substitutions, discipline rule `discipline_rule::NORTHERN_IRELAND`, the clubs NirCharity::select_teams (0x78a1e0;
/// fails when negative), no prize fund, no final ranking; the lines 0x43, 0x51 and 0x58 of nir_charity.cpp.
pub(crate) const NORTHERN_IRISH_CHARITY_SHIELD_CONSTRUCTOR: IrishCupConstructor = IrishCupConstructor { vtable: 0x96da24, bench: Some((3, 3)), discipline: discipline_rule::NORTHERN_IRELAND,
    entrants: 0x78a1e0, fails_when_negative: true, prize: None, rank: false, file: 0xa695f0, rows: [0x43, 0x51, 0x58] };
