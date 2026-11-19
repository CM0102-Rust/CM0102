// ---------- batch %%LEVA%%: the German season turnover (ger_first.cpp) ----------

/// Rule: the German turnover, led by the Bundesliga: the season reopened; with the Regionalliga loaded, its own step
/// first; promotion and relegation along the chain; the clubs exchanged between the 2. Bundesliga and the Regionalliga
/// (with it loaded by GerFirst's 0x5db980, without it by `ger_first_exchange_regional_clubs_unplayed`); the
/// Regionalliga clubs split north and south; the Bundesliga rebuilt for the next year (rules, table, fixtures, order);
/// the division tier of the 2. Bundesliga clubs capped and the 2. Bundesliga (and the Regionalliga, when loaded) turned
/// over; the predicted positions, the board expectations and the officials.
///
/// `top_division_turnover` with, before: 0x5dbf50 (thiscall, ret 0, not ported) when the object of
/// `KEY_CLUB_COMP_GERMAN_REGIONAL` exists; virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1)
/// (`ger_first_process_promotion_relegation_chain`); 0x5db980 (not ported) or `ger_first_exchange_regional_clubs_unplayed`
/// by that object again; `ger_first_split_regional_north_south` (0x5dbbd0, not ported; called with ecx = this, it does
/// not read it); the table GerFirst::create_table (0x5db230) and the boxes of ger_first.cpp (0x9cd414) at 0x2ec, 0x2f3,
/// 0x2f9, 0x300; after: League::limit_club_division_tier(1, 3, −3, 1) and the start_new_season (virtual
/// `comp_vtable::START_NEW_SEASON`, no null check) of the 2. Bundesliga, then the Regionalliga's when it exists. After a
/// box: 0.
/// Original: `GerFirst::start_new_season` (0x5db4a0, ger_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        if comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL) != 0 {
            call_on_league(0x5dbf50, this);
        }
        promotion_relegation_chain(this, 1);
        if comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL) != 0 {
            call_on_league(0x5db980, this);
        } else {
            call_on_league(0x5db7b0, this);
        }
        call_on_league(0x5dbbd0, this);
    }, 0x5db230, 0x9cd414, [0x2ec, 0x2f3, 0x2f9, 0x300], || {
        covered!("mwc_21 german bundesliga turnover");
        exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), 1, 3, -3, 1);
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), comp_vtable::START_NEW_SEASON);
        start_new_season_of(KEY_CLUB_COMP_GERMAN_REGIONAL);
    })
}

/// Rule: German promotion and relegation: the final positions of the Bundesliga and the 2. Bundesliga recorded and
/// their marked clubs swapped; with the Regionalliga loaded, its final positions and its first group's recorded and
/// the marked clubs swapped between the 2. Bundesliga and the Regionalliga, and between the 2. Bundesliga and that
/// group.
///
/// Virtual `league_vtable::RECORD_FINAL_POSITIONS` on the objects of `KEY_CLUB_COMP_GERMAN_FIRST_DIVISION` and
/// `KEY_CLUB_COMP_GERMAN_SECOND_DIVISION`; League::promote_and_relegate (`exe::league_promote_and_relegate`(this,
/// above, below, 1, 1, −1, −1): 6 pushes -> thiscall, ret 0x18) Bundesliga / 2. Bundesliga; when the object of
/// `KEY_CLUB_COMP_GERMAN_REGIONAL` exists: RECORD_FINAL_POSITIONS on it and on its stage 0 (`league::STAGES`[0]), then
/// promote_and_relegate 2. Bundesliga / Regionalliga and 2. Bundesliga / stage 0. Returns the last call's eax (without
/// the Regionalliga, the value of `G_COMP_OBJECT_TABLE` the original leaves in eax); the argument is not read.
/// Original: `GerFirst::process_promotion_relegation_chain` (0x5db6c0, ger_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ger_first_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION), league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), league_vtable::RECORD_FINAL_POSITIONS);
    covered!("mwc_21 german promotion and relegation");
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION), comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), 1, 1, -1, -1); // mwc21: the Bundesliga and the 2. Bundesliga
    let regional = comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL);
    if regional == 0 {
        return read_usize(G_COMP_OBJECT_TABLE) as u32;
    }
    bra_nat_first_call_virtual_no_args(regional, league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(read_usize(read_usize(comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL) + league::STAGES)), league_vtable::RECORD_FINAL_POSITIONS);
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL), 1, 1, -1, -1);
    let stage = read_usize(read_usize(comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL) + league::STAGES));
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), stage, 1, 1, -1, -1)
}

/// Rule: at the German turnover without the Regionalliga loaded, the four best clubs of the Regionalliga division
/// (sorted by strength with the best twelve shuffled; B teams left out) go up to the 2. Bundesliga, and its relegated
/// clubs (status 3) go down to the Regionalliga division; with no candidate nobody moves.
///
/// A list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; out of memory: the box of ger_first.cpp at line 0x354 and
/// return) with the clubs in database order whose division (`club::DIVISION`) is `KEY_CLUB_COMP_GERMAN_REGIONAL`'s,
/// with a nation (`club::NATION` not null) and not a B team (`exe::club_get_linked_b_team`(club, &flag, 1), cdecl: a
/// partner with the flag 0 leaves the club out; the flag is one local shared by every call, as the original's stack
/// slot) (a 16-bit index). With at least one: `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 4) and the first four
/// moved to the 2. Bundesliga (League::move_club_to_division(this, club, its record, 1)); with fewer than four the
/// original moves the list's unwritten slots as well (a defect of the original, kept); then each row i of the 2.
/// Bundesliga (its club count read again on each step) with status 3 goes down (League::get_table_row, called twice,
/// and League::relegate_club_to_division(this, club, the Regionalliga's record, 1)). The list is freed.
/// Original: `ger_first_exchange_regional_clubs_unplayed` (0x5db7b0, ger_first.cpp, thiscall: ecx = the Bundesliga, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_first_exchange_regional_clubs_unplayed(this: usize) {
    let second = comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION);
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9cd414, 0x354);
        return;
    }
    let mut flag: u32 = 0;
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(club + club::NATION) != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GERMAN_REGIONAL) {
            if exe::club_get_linked_b_team(club, &mut flag, 1) == 0 || flag != 0 {
                write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
                n = n.wrapping_add(1);
            }
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if n as u16 != 0 {
        covered!("mwc_21 german regional division up");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4); // mwc21: the best twelve shuffled
        for k in 0..4 {
            exe::league_move_club_to_division(this, read_usize(list + 4 * k), comp_record_of(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), 1);
        }
        let mut i = 0i32;
        while i < League(second).club_count() as i32 {
            if read_u8(exe::league_get_table_row(second, i) + league_table_row::STATUS) == 3 {
                let record = comp_record_of(KEY_CLUB_COMP_GERMAN_REGIONAL);
                let club = read_usize(exe::league_get_table_row(second, i));
                exe::league_relegate_club_to_division(this, club, record, 1);
            }
            i += 1;
        }
    }
    exe::crt_free(list);
}

/// Comparer for `ger_first_start_new_season`: only the original runs (it draws and rebuilds); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ger_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("ger_first_start_new_season", League(this), || o(this))
}

/// Comparer for `ger_first_process_promotion_relegation_chain`: only the original runs (it moves clubs between the
/// divisions; as the Turkish chain's, `tur_first_process_promotion_relegation_chain`).
pub(crate) unsafe extern "thiscall" fn compare_ger_first_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I1%%)(this, pairs)
}

/// Comparer for `ger_first_exchange_regional_clubs_unplayed`: only the original runs (it draws); with at least 4
/// candidates, the 2. Bundesliga's clubs of status 3 must end in the Regionalliga division.
pub(crate) unsafe extern "thiscall" fn compare_ger_first_exchange_regional_clubs_unplayed(this: usize) {
    let mut flag: u32 = 0;
    let mut candidates = 0;
    for i in 0..read_usize(G_CLUB_COUNT) {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let d = read_usize(c + club::DIVISION);
        if d != 0 && read_usize(c + club::NATION) != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GERMAN_REGIONAL) && (exe::club_get_linked_b_team(c, &mut flag, 1) == 0 || flag != 0) {
            candidates += 1;
        }
    }
    let l = League(comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION));
    let mut planned: Vec<usize> = Vec::new();
    if candidates >= 4 && l.table() != 0 {
        for i in 0..l.club_count() as usize {
            let row = l.table() + i * league_table_row::SIZE;
            if read_u8(row + league_table_row::STATUS) == 3 {
                planned.push(read_usize(row));
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let lower = comp_record_of(KEY_CLUB_COMP_GERMAN_REGIONAL);
    let wrong: Vec<usize> = planned.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != lower).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("ger_first_exchange_regional_clubs_unplayed({:#x}): relegated clubs not in the Regionalliga division {:x?}", this, wrong));
}
