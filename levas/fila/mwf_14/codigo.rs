// ---------- mwf_14: the Spanish season turnover, promotion and relegation (spa_first.cpp) ----------

/// The Spanish promotion and relegation chain: records the final positions of the First and Second Divisions and
/// swaps the marked clubs between them; then, when Segunda B is loaded, records its own and its three first groups'
/// positions and swaps the Second Division with Segunda B and with each of those groups.
///
/// The final positions (virtual `league_vtable::RECORD_FINAL_POSITIONS`, thiscall, ret 0) of the First
/// ([`KEY_CLUB_COMP_SPANISH_FIRST_DIVISION`]) and the Second ([`KEY_CLUB_COMP_SPANISH_SECOND_DIVISION`]) Division
/// objects (`G_COMP_OBJECT_TABLE`, neither checked for null), and League::promote_and_relegate
/// (`exe::league_promote_and_relegate`, thiscall(this, upper, lower, 1, 1, −1, −1), ret 0x18) between them. When the
/// Segunda B object ([`KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B`]) exists: its final positions and those of its stages
/// 0, 1 and 2 (`league::STAGES`), then promote_and_relegate between the Second and Segunda B and between the Second
/// and each of those three stages; the last one's eax is returned. Without Segunda B it returns the competition
/// object table's address (the eax the original leaves; the caller does not read it).
/// Original: `SpaFirst::process_promotion_relegation_chain` (0x850700, spa_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn spa_first_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    let (first, second) = (KEY_CLUB_COMP_SPANISH_FIRST_DIVISION, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION);
    covered!("mwf_14 spanish chain");
    bra_nat_first_call_virtual_no_args(comp_by_global(first), league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(comp_by_global(second), league_vtable::RECORD_FINAL_POSITIONS);
    exe::league_promote_and_relegate(this, comp_by_global(first), comp_by_global(second), 1, 1, -1, -1); // mwf14: the First and the Second
    let b = comp_by_global(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B);
    if b == 0 {
        return read_usize(G_COMP_OBJECT_TABLE) as u32;
    }
    covered!("mwf_14 segunda b chain");
    bra_nat_first_call_virtual_no_args(b, league_vtable::RECORD_FINAL_POSITIONS);
    for k in 0..3 {
        bra_nat_first_call_virtual_no_args(read_usize(read_usize(b + league::STAGES) + 4 * k), league_vtable::RECORD_FINAL_POSITIONS);
    }
    let mut r = exe::league_promote_and_relegate(this, comp_by_global(second), b, 1, 1, -1, -1);
    for k in 0..3 {
        let group = read_usize(read_usize(comp_by_global(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B) + league::STAGES) + 4 * k);
        r = exe::league_promote_and_relegate(this, comp_by_global(second), group, 1, 1, -1, -1);
    }
    r
}

/// Comparer for `spa_first_process_promotion_relegation_chain`: only the original (it moves the clubs between the
/// divisions, as the Brazilian chain's comparer).
pub(crate) unsafe extern "thiscall" fn compare_spa_first_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I0%%)(this, pairs)
}

/// The Segunda B group B1 record (the 0x6b-byte competition record of [`KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1`]).
unsafe fn spanish_b1_record() -> usize {
    comp_record_of_key(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1)
}

/// Is `club` a B team whose senior club plays in the Second Division? The pair from `exe::club_get_linked_b_team`
/// (cdecl(club, &is_senior, 1)): a linked club, `club` not the senior (`is_senior` = 0) and the linked club's division
/// with the Second Division's id. `is_senior` is the same local in every call, as in the original.
unsafe fn senior_in_second_division(club: usize, is_senior: &mut u32) -> Option<usize> {
    let senior = exe::club_get_linked_b_team(club, is_senior, 1);
    if senior == 0 || *is_senior != 0 {
        return None;
    }
    let d = read_usize(senior + club::DIVISION);
    if d == 0 || read_u32(d) != read_u32(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION) {
        return None;
    }
    Some(senior)
}

/// Without Segunda B: the Second Division's relegated clubs go down to group B1 and as many clubs come up, first from
/// the Spanish lower division table, then from the clubs of groups B1 to B4; a B team never goes up into the division
/// of its senior club, and a B team whose senior club is now in the Second Division is forced down.
///
/// Over the Second Division object's table (`.club_count()` rows, a signed word; League::get_table_row
/// `exe::league_get_table_row`): a B team whose senior club plays in the Second Division
/// (`senior_in_second_division`) and which is not itself in the First Division (the club's division, not checked for
/// null) gets `league_table_row::STATUS` = 3 and the news (Comp::send_promotion_relegation_override_news 0x48ece0,
/// thiscall(second, `.competition()`, club, senior, 1), ret 0x10); every row with status 3 counts one (a byte) and its
/// club goes down to group B1 (League::relegate_club_to_division 0x6831a0, thiscall(this, club, B1 record, 1), ret
/// 0xc).
///
/// Then, while fewer than that count came up, the clubs of the lower division table (`spa_first_fields::LOWER_DIVISION`,
/// in row order) and then the clubs of the club table (`G_CLUB_TABLE`, `G_CLUB_COUNT`) whose division is B1, B2, B3 or
/// B4, each unless it is a B team whose senior club is in the Second Division, go up (League::move_club_to_division
/// `exe::league_move_club_to_division`, thiscall(this, club, the Second Division record, 1), ret 0xc). The counts are
/// compared as words.
/// Original: `SpaFirst::exchange_second_division_clubs_unplayed` (0x8500c0, spa_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_first_exchange_second_division_clubs_unplayed(this: usize) {
    let this = League(this);
    let second = League(comp_by_global(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION));
    let mut is_senior = 0u32;
    let mut down = 0u8;
    let mut i = 0i32;
    while i < second.club_count() as i32 {
        let club = read_usize(exe::league_get_table_row(second, i));
        if let Some(senior) = senior_in_second_division(club, &mut is_senior) {
            if read_u32(read_usize(club + club::DIVISION)) != read_u32(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) {
                covered!("mwf_14 b team forced down");
                write_u8(exe::league_get_table_row(second, i) + league_table_row::STATUS, 3);
                let news: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(0x48ece0usize);
                news(second.addr(), this.competition().addr(), club, senior, 1);
            }
        }
        if read_u8(exe::league_get_table_row(second, i) + league_table_row::STATUS) == 3 {
            down = down.wrapping_add(1);
            covered!("mwf_14 relegated to b1");
            let relegate: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6831a0usize);
            relegate(this.addr(), read_usize(exe::league_get_table_row(second, i)), spanish_b1_record(), 1); // mwf14: down to group B1
        }
        i += 1;
    }
    let wanted = down as i8 as i16;
    let mut up = 0i16;
    let lower = League(read_usize(this.addr() + spa_first_fields::LOWER_DIVISION));
    let mut j = 0i32;
    if lower.club_count() > 0 {
        while up < wanted {
            let club = read_usize(exe::league_get_table_row(lower, j));
            if senior_in_second_division(club, &mut is_senior).is_none() {
                exe::league_move_club_to_division(this, club, comp_record_of_key(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION), 1);
                up += 1;
            }
            j += 1;
            if j >= lower.club_count() as i32 {
                break;
            }
        }
    }
    if up >= wanted {
        return;
    }
    covered!("mwf_14 up from the b groups");
    let groups = [KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B1, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B2, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B3, KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B4];
    let mut k = 0i32;
    while k < read_i32(G_CLUB_COUNT) {
        if up >= wanted {
            break;
        }
        let club = read_usize(G_CLUB_TABLE) + k as usize * club::SIZE;
        let d = read_usize(club + club::DIVISION);
        if d != 0 && groups.iter().any(|&g| read_u32(d) == read_u32(g)) && senior_in_second_division(club, &mut is_senior).is_none() {
            exe::league_move_club_to_division(this, club, comp_record_of_key(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION), 1);
            up += 1;
        }
        k += 1;
    }
}

/// Comparer for `spa_first_exchange_second_division_clubs_unplayed`: only the original runs (it moves clubs and sends
/// news); afterwards every club in a Second Division row marked relegated (status 3) must be in group B1.
pub(crate) unsafe extern "thiscall" fn compare_spa_first_exchange_second_division_clubs_unplayed(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let second = League(comp_by_global(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION));
    let b1 = spanish_b1_record();
    let wrong: Vec<usize> = (0..second.club_count().max(0) as i32)
        .map(|i| exe::league_get_table_row(second, i))
        .filter(|&row| read_u8(row + league_table_row::STATUS) == 3 && read_usize(read_usize(row) + club::DIVISION) != b1)
        .map(|row| read_usize(row))
        .collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("spa_first_exchange_second_division_clubs_unplayed({:#x}) relegated clubs not in B1: {:x?}", this, wrong));
}

/// The Spanish season turnover, led by the First Division: the B team clashes and the promotion and relegation chain,
/// the exchange with Segunda B (or, without it, with the lower division), then the First Division rebuilt (next year,
/// rules, 20 clubs, fixtures, order) and the Second Division and Segunda B (or the lower division) turned over; then
/// the predicted positions, the board expectations and the referee renewal.
///
/// `league::SEASON_CLOSED` = 0; with Segunda B ([`KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B`] in `G_COMP_OBJECT_TABLE`),
/// SpaFirst::resolve_b_team_clashes (0x850970, thiscall, ret 0); the chain (virtual
/// `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`, thiscall(this, 1), ret 4); with Segunda B, the Segunda B
/// exchange and the group assignment (0x850300 and 0x850530, called with ecx = this, no arguments), without it
/// SpaFirst::exchange_second_division_clubs_unplayed (0x8500c0); the table freed (League::free_table
/// `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed and zeroed; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the year
/// up by one (the stages and `league::CURRENT_STAGE` are not touched).
///
/// Then, each with its own box (spa_first.cpp, lines 0x2da, 0x2e1, 0x2e7 and 0x2ee) when it returns 0: the rules
/// (virtual `comp_vtable::SET_UP_RULES`), the table (SpaFirst::create_table 0x84fc70), the fixtures
/// (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table `exe::league_sort_table`,
/// thiscall(this, 0), ret 4). Then the turnover (virtual `comp_vtable::START_NEW_SEASON`) of the Second Division (not
/// checked for null) and of Segunda B, or, without it, of the lower division
/// ([`KEY_CLUB_COMP_SPANISH_LOWER_DIVISION`]); the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`)
/// and `set_expectations_and_renew_officials`, whose eax (the referee renewal's) it returns. After a box the copy
/// returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `SpaFirst::start_new_season` (0x84feb0, spa_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    let call = |f: usize| {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
        f(this.addr());
    };
    this.set_season_closed(0);
    if comp_by_global(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B) != 0 {
        call(0x850970);
    }
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this.addr(), 1);
    if comp_by_global(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B) != 0 {
        call(0x850300);
        call(0x850530);
    } else {
        call(0x8500c0); // mwf14: the exchange without Segunda B
    }
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    let a = this.year();
    this.set_year(a.wrapping_add(1)); // mwf14: the year of the First Division
    covered!("mwf_14 primera rollover");
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(0xa82844, 0x2da);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x84fc70usize);
    if table(this.addr()) == 0 {
        show_error_at(0xa82844, 0x2e1);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0xa82844, 0x2e7);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(0xa82844, 0x2ee);
        return 0;
    }
    bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION), comp_vtable::START_NEW_SEASON);
    let b = comp_by_global(KEY_CLUB_COMP_SPANISH_SECOND_DIVISION_B);
    let lower = if b != 0 { b } else { comp_by_global(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) };
    bra_nat_first_call_virtual_no_args(lower, comp_vtable::START_NEW_SEASON);
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Comparer for `spa_first_start_new_season`: only the original runs (it moves clubs and rebuilds the Spanish
/// leagues); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_spa_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_state_league_new_season("spa_first_start_new_season", League(this), || o(this))
}
