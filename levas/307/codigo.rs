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
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_SPA_FIRST_PROCESS_PROMOTION_RELEGATION_CHAIN)(this, pairs)
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
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SPA_FIRST_EXCHANGE_SECOND_DIVISION_CLUBS_UNPLAYED)(this);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("spa_first_start_new_season", League(this), || o(this))
}

// ---------- mwf_15: the French season turnover and the CFA and National exchange (fra_first.cpp) ----------

/// The CFA and National exchange: the top CFA clubs go up to National (six in the 2001 turnover, four later) and the
/// clubs relegated from National go back to the CFA.
///
/// National is the competition object of [`KEY_CLUB_COMP_FRENCH_NATIONAL`] (`G_COMP_OBJECT_TABLE`). A list of
/// `G_CLUB_COUNT` × 4 bytes (`exe::crt_malloc`; out of memory, the box of fra_first.cpp line 0x28e and it returns)
/// takes, in club table order, every club with a division, a nation and a reserve division (`club::RESERVE_DIVISION`)
/// whose division has the id of [`KEY_CLUB_COMP_A_LOWER_DIVISION`] and whose reserve division has the id of
/// [`KEY_CLUB_COMP_FRENCH_CFA`] (the CFA clubs; the count is a word). With none, the list is freed and it returns.
/// Otherwise the list is sorted by reputation and shuffled in fours (`exe::comp_util_sort_and_shuffle_top_clubs`(list,
/// n, 4), cdecl) and its first clubs (6 when `.year()` is 2001, otherwise 4; not checked against the count, a defect of
/// the original when the CFA has fewer clubs [inference]) go up to National (League::move_club_to_division
/// `exe::league_move_club_to_division`, thiscall(this, club, National's competition record, 1), ret 0xc).
///
/// Then every National table row (`.club_count()`, a signed word; League::get_table_row `exe::league_get_table_row`)
/// with `league_table_row::STATUS` 3 sends its club down to the lower division (League::relegate_club_to_division
/// 0x6831a0, thiscall(this, club, the record of [`KEY_CLUB_COMP_A_LOWER_DIVISION`], 1), ret 0xc) with the CFA record
/// as its reserve division (`club::RESERVE_DIVISION`). The list is freed.
/// Original: `FraFirst::exchange_cfa_national_clubs` (0x5bc780, fra_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_first_exchange_cfa_national_clubs(this: usize) {
    let this = League(this);
    let national = League(comp_by_global(KEY_CLUB_COMP_FRENCH_NATIONAL));
    let list = exe::crt_malloc((read_i32(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9ccb5c, 0x28e);
        return;
    }
    let mut n = 0i16;
    let mut k = 0i32;
    while k < read_i32(G_CLUB_COUNT) {
        let c = read_usize(G_CLUB_TABLE) + k as usize * club::SIZE;
        let (d, r) = (read_usize(c + club::DIVISION), read_usize(c + club::RESERVE_DIVISION));
        if d != 0 && read_usize(c + club::NATION) != 0 && r != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_A_LOWER_DIVISION) && read_u32(r) == read_u32(KEY_CLUB_COMP_FRENCH_CFA) {
            write_usize(list + n as usize * 4, c);
            n = n.wrapping_add(1);
        }
        k += 1;
    }
    if n == 0 {
        exe::crt_free(list);
        return;
    }
    covered!("mwf_15 cfa up to national");
    exe::comp_util_sort_and_shuffle_top_clubs(list, n as u16 as u32, 4); // mwf15: the CFA clubs by reputation, shuffled in fours
    let up: i8 = if this.year() == 0x7d1 { 6 } else { 4 }; // six up in 2001, four later
    for t in 0..up.max(0) as usize {
        exe::league_move_club_to_division(this, read_usize(list + t * 4), comp_record_of_key(KEY_CLUB_COMP_FRENCH_NATIONAL), 1);
    }
    let relegate: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6831a0usize);
    let mut i = 0i32;
    while i < national.club_count() as i32 {
        if read_u8(exe::league_get_table_row(national, i) + league_table_row::STATUS) == 3 {
            relegate(this.addr(), read_usize(exe::league_get_table_row(national, i)), comp_record_of_key(KEY_CLUB_COMP_A_LOWER_DIVISION), 1);
            let cfa = comp_record_of_key(KEY_CLUB_COMP_FRENCH_CFA);
            write_usize(read_usize(exe::league_get_table_row(national, i)) + club::RESERVE_DIVISION, cfa);
        }
        i += 1;
    }
    exe::crt_free(list);
}

/// Comparer for `fra_first_exchange_cfa_national_clubs`: only the original runs (it draws, moves clubs and sends
/// news); afterwards every club in a National row marked relegated (status 3) must be in the lower division with the
/// CFA as its reserve division.
pub(crate) unsafe extern "thiscall" fn compare_fra_first_exchange_cfa_national_clubs(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FRA_FIRST_EXCHANGE_CFA_NATIONAL_CLUBS)(this);
    let national = League(comp_by_global(KEY_CLUB_COMP_FRENCH_NATIONAL));
    let (lower, cfa) = (comp_record_of_key(KEY_CLUB_COMP_A_LOWER_DIVISION), comp_record_of_key(KEY_CLUB_COMP_FRENCH_CFA));
    let wrong: Vec<usize> = (0..national.club_count().max(0) as i32)
        .map(|i| exe::league_get_table_row(national, i))
        .filter(|&row| read_u8(row + league_table_row::STATUS) == 3)
        .map(|row| read_usize(row))
        .filter(|&c| read_usize(c + club::DIVISION) != lower || read_usize(c + club::RESERVE_DIVISION) != cfa)
        .collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("fra_first_exchange_cfa_national_clubs({:#x}) relegated clubs not in the CFA: {:x?}", this, wrong));
}

/// The French season turnover, led by the First Division: promotion and relegation, the CFA and National exchange,
/// then the First Division rebuilt (next year, rules, 18 clubs, fixtures, order); the division tier of the National
/// clubs capped; the Second Division and National turned over; then the predicted positions, the board expectations
/// and the referee renewal; finally a CFA and a lower league object are built for the new year and destroyed at once.
///
/// `league::SEASON_CLOSED` = 0; the chain (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`,
/// thiscall(this, 0), ret 4); FraFirst::exchange_cfa_national_clubs (0x5bc780, thiscall, ret 0); the table freed
/// (League::free_table `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed and zeroed; with
/// statistics, Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret
/// 4); the year up by one (the stages are not touched). Then, each with its own box (fra_first.cpp, lines 0x245, 0x24c,
/// 0x252 and 0x259) when it returns 0: the rules (virtual `comp_vtable::SET_UP_RULES`), the table
/// (FraFirst::create_table 0x5bc2b0), the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order
/// (League::sort_table `exe::league_sort_table`, thiscall(this, 0), ret 4).
///
/// Then League::limit_club_division_tier (`exe::league_limit_club_division_tier`, thiscall(National, 1, 3, −3, 1), ret
/// 0x10) on National ([`KEY_CLUB_COMP_FRENCH_NATIONAL`]); the turnovers (virtual `comp_vtable::START_NEW_SEASON`) of the
/// Second Division ([`KEY_CLUB_COMP_FRENCH_SECOND_DIVISION`]) and of National, neither checked for null; the predicted
/// positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`) and `set_expectations_and_renew_officials`. Finally a FraCfa
/// (`exe::crt_operator_new` of 0xee; FraCfa::FraCfa `FN_FRACFA_FRACFA`, thiscall(p, year), ret 4) and a FraLower (0xee;
/// FraLower::FraLower `FN_FRALOWER_FRALOWER`) for the new `.year()`, each destroyed right away (virtual 0 with 1) when
/// it was built. Returns the referee renewal's eax (simplification: the original returns the eax of its last call; the
/// turnover's caller does not read it). After a box the copy returns 0.
/// Original: `FraFirst::start_new_season` (0x5bc4d0, fra_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this.addr(), 0);
    let exchange: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5bc780usize);
    exchange(this.addr()); // the CFA and National exchange
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
    this.set_year(a.wrapping_add(1)); // mwf15: the year of the French First Division
    covered!("mwf_15 french rollover");
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(0x9ccb5c, 0x245);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5bc2b0usize);
    if table(this.addr()) == 0 {
        show_error_at(0x9ccb5c, 0x24c);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0x9ccb5c, 0x252);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(0x9ccb5c, 0x259);
        return 0;
    }
    exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_FRENCH_NATIONAL), 1, 3, -3, 1);
    bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_FRENCH_SECOND_DIVISION), comp_vtable::START_NEW_SEASON);
    bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_FRENCH_NATIONAL), comp_vtable::START_NEW_SEASON);
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    let r = set_expectations_and_renew_officials(this);
    let build = |ctor: usize| -> usize {
        let p = exe::crt_operator_new(0xee);
        if p == 0 {
            return 0;
        }
        let construct: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(ctor);
        construct(p, this.year() as u16 as u32)
    };
    let cfa = build(FN_FRACFA_FRACFA);
    let lower = build(FN_FRALOWER_FRALOWER);
    for o in [cfa, lower] {
        if o != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(o)));
            destroy(o, 1);
        }
    }
    r
}

/// Comparer for `fra_first_start_new_season`: only the original runs (it moves clubs and rebuilds the French leagues);
/// the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_fra_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("fra_first_start_new_season", League(this), || o(this))
}

// ---------- mwf_16: the Italian promotion and relegation, the Serie C2 exchange and the Serie C1 regrouping (ita_ser_a.cpp) ----------

/// The Italian promotion and relegation chain: the final positions of Serie A, B, C1 A and C1 B; four up and four down
/// between Serie A and B, the swaps between Serie B and each C1 group; when Serie C2 is loaded, its three groups'
/// positions and the swaps between each C1 group and each C2 group.
///
/// The final positions (virtual `league_vtable::RECORD_FINAL_POSITIONS`, thiscall, ret 0) of the objects of
/// [`KEY_CLUB_COMP_ITALIAN_SERIE_A`], [`KEY_CLUB_COMP_ITALIAN_SERIE_B`], [`KEY_CLUB_COMP_ITALIAN_SERIE_C1_A`] and
/// [`KEY_CLUB_COMP_ITALIAN_SERIE_C1_B`] (`G_COMP_OBJECT_TABLE`, none checked for null); League::promote_and_relegate
/// (`exe::league_promote_and_relegate`, thiscall(this, upper, lower, a, b, c, d), ret 0x18) for A and B with (1, 1, 4,
/// 4), B and C1 A and B and C1 B with (1, 1, −1, −1). When the Serie C2 A object exists: the final positions of C2 A,
/// C2 B and C2 C, then C1 A with C2 A (1, 1, −1, −1), C1 B with C2 A (0, 1, −1, −1), C1 A with C2 B (1, 1), C1 B with
/// C2 B (0, 1), C1 A with C2 C (1, 1) and C1 B with C2 C (0, 1); the last one's eax is returned. Without C2 A it returns
/// the competition object table's address (the eax the original leaves; the caller does not read it).
/// Original: `ItaSerA::process_promotion_relegation_chain` (0x642410, ita_ser_a.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    let (a, b, c1a, c1b) = (KEY_CLUB_COMP_ITALIAN_SERIE_A, KEY_CLUB_COMP_ITALIAN_SERIE_B, KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B);
    covered!("mwf_16 italian chain");
    for k in [a, b, c1a, c1b] {
        bra_nat_first_call_virtual_no_args(comp_by_global(k), league_vtable::RECORD_FINAL_POSITIONS);
    }
    exe::league_promote_and_relegate(this, comp_by_global(a), comp_by_global(b), 1, 1, 4, 4); // mwf16: four up and four down between A and B
    exe::league_promote_and_relegate(this, comp_by_global(b), comp_by_global(c1a), 1, 1, -1, -1);
    exe::league_promote_and_relegate(this, comp_by_global(b), comp_by_global(c1b), 1, 1, -1, -1);
    if comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A) == 0 {
        return read_usize(G_COMP_OBJECT_TABLE) as u32;
    }
    covered!("mwf_16 serie c2 chain");
    let (c2a, c2b, c2c) = (KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C);
    for k in [c2a, c2b, c2c] {
        bra_nat_first_call_virtual_no_args(comp_by_global(k), league_vtable::RECORD_FINAL_POSITIONS);
    }
    let mut r = 0;
    for lower in [c2a, c2b, c2c] {
        exe::league_promote_and_relegate(this, comp_by_global(c1a), comp_by_global(lower), 1, 1, -1, -1);
        r = exe::league_promote_and_relegate(this, comp_by_global(c1b), comp_by_global(lower), 0, 1, -1, -1);
    }
    r
}

/// Comparer for `ita_ser_a_process_promotion_relegation_chain`: only the original (it moves the clubs between the
/// divisions, as the Brazilian chain's comparer).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_ITA_SER_A_PROCESS_PROMOTION_RELEGATION_CHAIN)(this, pairs)
}

/// Without Serie C2: the six best Serie C2 clubs go up to Serie C1 A and the clubs relegated from either C1 group go
/// down to Serie C2 A (the C1 regrouping that follows rebalances the two groups).
///
/// C1 A and C1 B are the objects of [`KEY_CLUB_COMP_ITALIAN_SERIE_C1_A`] and [`KEY_CLUB_COMP_ITALIAN_SERIE_C1_B`]. A
/// list of `G_CLUB_COUNT` × 4 bytes (`exe::crt_malloc`; out of memory, the box of ita_ser_a.cpp line 0x419 and it
/// returns) takes, in club table order, every club whose division has the id of C2 A, C2 B or C2 C (the count is a
/// word). With fewer than six, nothing moves. Otherwise the list is sorted by reputation and shuffled in sixes
/// (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 6), cdecl) and its first six go up to C1 A
/// (League::move_club_to_division `exe::league_move_club_to_division`, thiscall(this, club, the C1 A record, 1), ret
/// 0xc). Then for each row i below C1 A's `.club_count()` (read once; C1 B is assumed the same size): a C1 A row with
/// `league_table_row::STATUS` 3 and then a C1 B row with status 3 send their clubs down to C2 A
/// (League::relegate_club_to_division 0x6831a0, thiscall(this, club, the C2 A record, 1), ret 0xc). The list is freed.
/// Original: `ItaSerA::exchange_serie_c2_clubs_unplayed` (0x641fc0, ita_ser_a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_exchange_serie_c2_clubs_unplayed(this: usize) {
    let this = League(this);
    let (c1a, c1b) = (League(comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A)), League(comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C1_B)));
    let list = exe::crt_malloc((read_i32(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9ea2a8, 0x419);
        return;
    }
    let c2 = [KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C];
    let mut n = 0i16;
    let mut k = 0i32;
    while k < read_i32(G_CLUB_COUNT) {
        let c = read_usize(G_CLUB_TABLE) + k as usize * club::SIZE;
        let d = read_usize(c + club::DIVISION);
        if d != 0 && c2.iter().any(|&g| read_u32(d) == read_u32(g)) {
            write_usize(list + n as usize * 4, c);
            n = n.wrapping_add(1);
        }
        k += 1;
    }
    if n >= 6 {
        covered!("mwf_16 serie c2 up to c1");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n as u16 as u32, 6); // mwf16: the C2 clubs by reputation, shuffled in sixes
        for t in 0..6 {
            exe::league_move_club_to_division(this, read_usize(list + t * 4), comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A), 1);
        }
        let relegate: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6831a0usize);
        let rows = c1a.club_count() as i32;
        for i in 0..rows.max(0) {
            for group in [c1a, c1b] {
                if read_u8(exe::league_get_table_row(group, i) + league_table_row::STATUS) == 3 {
                    relegate(this.addr(), read_usize(exe::league_get_table_row(group, i)), comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A), 1);
                }
            }
        }
    }
    exe::crt_free(list);
}

/// Comparer for `ita_ser_a_exchange_serie_c2_clubs_unplayed`: only the original runs (it draws and moves clubs);
/// afterwards every club in a C1 A or C1 B row marked relegated (status 3) must be in Serie C2 A.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_exchange_serie_c2_clubs_unplayed(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ITA_SER_A_EXCHANGE_SERIE_C2_CLUBS_UNPLAYED)(this);
    let c2a = comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A);
    let mut wrong = Vec::new();
    for key in [KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B] {
        let g = League(comp_by_global(key));
        for i in 0..g.club_count().max(0) as i32 {
            let row = exe::league_get_table_row(g, i);
            if read_u8(row + league_table_row::STATUS) == 3 && read_usize(read_usize(row) + club::DIVISION) != c2a {
                wrong.push(read_usize(row));
            }
        }
    }
    SCOREBOARD.record(wrong.is_empty(), || format!("ita_ser_a_exchange_serie_c2_clubs_unplayed({:#x}) relegated clubs not in C2 A: {:x?}", this, wrong));
}

/// The Serie C1 regrouping: the 36 clubs of the two C1 groups sorted from north to south (by their stadium's city) and
/// split, the first 18 into C1 B and the other 18 into C1 A; the human manager of a club that changes group gets the
/// news.
///
/// In club table order (the index compared as a word with `G_CLUB_COUNT`, read once), the clubs whose division is the
/// C1 A or the C1 B record (`comp_record_of_key`) enter a local list of 36 (the count is a signed byte; it stops at
/// 36). The list is sorted (`exe::crt_qsort`(list, n, 4, 0x53fd40), the latitude comparer). Then the 36 entries (always
/// 36: with fewer clubs the original reads stack garbage, a defect [inference]): entry k < 18 goes to C1 B, the others
/// to C1 A; a club that was not already there and whose manager is human (`exe::news_club_has_human_manager`, thiscall
/// (`G_NEWS`, club)) gets the news item 0x13 (NewsItem::NewsItem `exe::new_newsitem`(item, 0x13, 0); slot 0 the club's
/// id, slot 1 the group's id; News::send_to_club_manager `exe::news_send_to_club_manager`(`G_NEWS`, item, club)); every
/// club's `club::DIVISION` becomes its group's record.
/// Original: `ita_ser_a_regroup_serie_c1_clubs` (0x642630, ita_ser_a.cpp, called with ecx = the Serie A object and no
/// arguments, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_regroup_serie_c1_clubs(_this: usize) {
    let (a, b) = (comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A), comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C1_B));
    let count = read_i32(G_CLUB_COUNT);
    let mut list = [0usize; 36];
    let mut n = 0i8;
    let mut k = 0i16;
    if count > 0 {
        loop {
            let c = read_usize(G_CLUB_TABLE) + k as u16 as usize * club::SIZE;
            let d = read_usize(c + club::DIVISION);
            if d == a || d == b {
                list[n as usize] = c;
                n += 1;
                if n == 36 {
                    break;
                }
            }
            k = k.wrapping_add(1);
            if k as i32 >= count {
                break;
            }
        }
    }
    covered!("mwf_16 serie c1 regrouping");
    exe::crt_qsort(list.as_mut_ptr() as usize, n as i32 as u32, 4, 0x53fd40usize); // mwf16: north to south
    for (k, &c) in list.iter().enumerate() {
        if c == 0 {
            continue; // simplification: an empty entry (fewer than 36 clubs) is skipped; the original reads stack garbage
        }
        let group = if k < 18 { b } else { a };
        if read_usize(c + club::DIVISION) != group && exe::news_club_has_human_manager(read_usize(G_NEWS), c) != 0 {
            let mut news = [0u32; 0x38];
            let nt = news.as_mut_ptr() as usize;
            exe::new_newsitem(nt, 0x13, 0);
            exe::newsitem_set_slot(nt, 0, read_u32(c));
            exe::newsitem_set_slot(nt, 1, read_u32(group));
            exe::news_send_to_club_manager(read_usize(G_NEWS), nt, c);
        }
        write_usize(c + club::DIVISION, group);
    }
}

/// Comparer for `ita_ser_a_regroup_serie_c1_clubs`: only the original runs (it sends news); the plan, made before from
/// the same sort over a copy of the list, is the group of each of the 36 clubs.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_regroup_serie_c1_clubs(this: usize) {
    let (a, b) = (comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C1_A), comp_record_of_key(KEY_CLUB_COMP_ITALIAN_SERIE_C1_B));
    let mut list: Vec<usize> = (0..read_i32(G_CLUB_COUNT).max(0) as usize)
        .map(|k| read_usize(G_CLUB_TABLE) + k * club::SIZE)
        .filter(|&c| { let d = read_usize(c + club::DIVISION); d == a || d == b })
        .take(36)
        .collect();
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ITA_SER_A_REGROUP_SERIE_C1_CLUBS)(this);
    if list.len() != 36 {
        return; // simplification: with fewer than 36 the original reads stack garbage; nothing to check
    }
    exe::crt_qsort(list.as_mut_ptr() as usize, 36, 4, 0x53fd40usize);
    let wrong: Vec<usize> = list.iter().enumerate().filter(|&(k, &c)| read_usize(c + club::DIVISION) != if k < 18 { b } else { a }).map(|(_, &c)| c).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("ita_ser_a_regroup_serie_c1_clubs: clubs in the wrong group {:x?}", wrong));
}

// ---------- mwf_17: the Italian season turnover (ita_ser_a.cpp) ----------

/// The Italian season turnover, led by Serie A: promotion and relegation, the exchange with the level below (Serie D
/// and the C2 regrouping when Serie C2 is loaded, otherwise the C2 exchange), the C1 regrouping, then Serie A rebuilt
/// (next year, rules, 18 clubs, fixtures, order); the division tier capped in the lowest loaded groups; Serie B, C1 A,
/// C1 B (and C2 A, B and C) turned over; then the predicted positions, the board expectations and the referee renewal.
///
/// `league::SEASON_CLOSED` = 0; the chain (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`, thiscall(this,
/// 1), ret 4); with the Serie C2 A object ([`KEY_CLUB_COMP_ITALIAN_SERIE_C2_A`] in `G_COMP_OBJECT_TABLE`),
/// ItaSerA::exchange_serie_d_clubs (0x6421c0) and ita_ser_a_regroup_serie_c2_clubs (0x642790), otherwise
/// ItaSerA::exchange_serie_c2_clubs_unplayed (0x641fc0); then ita_ser_a_regroup_serie_c1_clubs (0x642630); all called
/// with ecx = this and no arguments. The table freed (League::free_table `exe::league_free_table`, thiscall(this, 0),
/// ret 4); `.round_dates()` freed and zeroed; with statistics, Comp_Stats::archive_season_and_reset
/// (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the stage array freed when there is one
/// (without destroying stages) and zeroed, `league::STAGE_COUNT` = 0; the year up by one; `league::CURRENT_STAGE` = −1;
/// and `league::SEASON_CLOSED` = 1 (as in Serie B; the sisters leave 0).
///
/// Then, each with its own box (ita_ser_a.cpp, lines 0x2da, 0x2e1, 0x2e7 and 0x2ee) when it returns 0: the rules
/// (virtual `comp_vtable::SET_UP_RULES`), the table (ItaSerA::create_table 0x641510), the fixtures
/// (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table `exe::league_sort_table`,
/// thiscall(this, 0), ret 4). Then League::limit_club_division_tier (`exe::league_limit_club_division_tier`, thiscall(
/// group, 1, 3, −3, 1), ret 0x10) on C2 A, C2 B and C2 C when C2 A exists, otherwise on C1 A and C1 B; the turnovers
/// (virtual `comp_vtable::START_NEW_SEASON`) of Serie B, C1 A and C1 B (none checked for null) and, when C2 A exists,
/// of C2 A, C2 B and C2 C; the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`) and
/// `set_expectations_and_renew_officials`, whose eax (the referee renewal's) it returns. After a box the copy returns 0
/// (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `ItaSerA::start_new_season` (0x641700, ita_ser_a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_a_start_new_season(this: usize) -> u32 {
    let this = League(this);
    let call = |f: usize| {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
        f(this.addr());
    };
    this.set_season_closed(0);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this.addr(), 1); // mwf17: the Italian promotion and relegation
    if comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A) != 0 {
        call(0x6421c0);
        call(0x642790);
    } else {
        call(0x641fc0);
    }
    call(0x642630); // the Serie C1 regrouping
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
    if this.stages() != 0 {
        exe::crt_free(this.stages());
    }
    let a = this.year();
    this.set_year(a.wrapping_add(1)); // mwf17: the year of Serie A
    this.set_stages(0);
    write_u32(this.addr() + league::STAGE_COUNT, 0);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    this.set_season_closed(1);
    covered!("mwf_17 serie a rollover");
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(0x9ea2a8, 0x2da);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x641510usize);
    if table(this.addr()) == 0 {
        show_error_at(0x9ea2a8, 0x2e1);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0x9ea2a8, 0x2e7);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(0x9ea2a8, 0x2ee);
        return 0;
    }
    let c2 = comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A) != 0;
    let capped: &[usize] = if c2 {
        &[KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C]
    } else {
        &[KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B]
    };
    for &k in capped {
        exe::league_limit_club_division_tier(comp_by_global(k), 1, 3, -3, 1);
    }
    for k in [KEY_CLUB_COMP_ITALIAN_SERIE_B, KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B] {
        bra_nat_first_call_virtual_no_args(comp_by_global(k), comp_vtable::START_NEW_SEASON);
    }
    if comp_by_global(KEY_CLUB_COMP_ITALIAN_SERIE_C2_A) != 0 {
        for k in [KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C] {
            bra_nat_first_call_virtual_no_args(comp_by_global(k), comp_vtable::START_NEW_SEASON);
        }
    }
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Comparer for `ita_ser_a_start_new_season`: only the original runs (it moves clubs and rebuilds the Italian
/// leagues); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_a_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ITA_SER_A_START_NEW_SEASON);
    compare_state_league_new_season("ita_ser_a_start_new_season", League(this), || o(this))
}

// ---------- mwf_18: the Conference exchange when the Conference is not played (eng_prm.cpp) ----------

/// Without a played Conference: its strongest club goes up to the Third Division if its ground meets the Third
/// Division's requirements (and the Third Division's relegated clubs go down to the Conference); otherwise the Third
/// Division's bottom club is reprieved and the Conference club gets the news.
///
/// The Third Division is the object of [`KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION`] (`G_COMP_OBJECT_TABLE`). A list of
/// `G_CLUB_COUNT` × 4 bytes (`exe::crt_malloc`; out of memory, the box of eng_prm.cpp line 0x2a9 and it returns) takes,
/// in club table order (the index compared as a word), every club whose division has the id of
/// [`KEY_CLUB_COMP_ENGLISH_CONFERENCE`]. With none: the list is freed and the box of line 0x2be (sic: no Conference
/// clubs is an error). Otherwise the list is sorted by reputation (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n,
/// 1): no shuffle) and the first club's ground is checked (`exe::financetable_get_club_finance`(`G_FINANCE_TABLE`,
/// club), then Finance::upgrade_stadium_for_league `exe::finance_upgrade_stadium_for_league`(finance, the Third
/// Division's `league::REQUIRED_SEATS`, `league::REQUIRED_CAPACITY` (signed words), its competition id, 1)).
///
/// Failing: the Third Division's last row (`.club_count()` − 1) gets `league_table_row::STATUS` 0xfe (stays up) and the
/// news (Comp::send_promotion_relegation_override_news 0x48ece0, thiscall(Third, the Conference record, club, 0, 2), ret
/// 0x10). Passing: the club goes up (League::move_club_to_division `exe::league_move_club_to_division`, thiscall(this,
/// club, the Third Division record, 1), ret 0xc) and every Third Division row with status 3 sends its club down to the
/// Conference (League::relegate_club_to_division 0x6831a0, thiscall(this, club, the Conference record, 1), ret 0xc).
/// The list is freed.
/// Original: `EngPrm::exchange_conference_clubs_unplayed` (0x574e60, eng_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_prm_exchange_conference_clubs_unplayed(this: usize) {
    let this = League(this);
    let third = League(comp_by_global(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION));
    let list = exe::crt_malloc((read_i32(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9c43b0, 0x2a9);
        return;
    }
    let mut n = 0i16;
    let mut k = 0i16;
    if read_i32(G_CLUB_COUNT) > 0 {
        loop {
            let c = read_usize(G_CLUB_TABLE) + k as u16 as usize * club::SIZE;
            let d = read_usize(c + club::DIVISION);
            if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_ENGLISH_CONFERENCE) {
                write_usize(list + n as usize * 4, c);
                n = n.wrapping_add(1);
            }
            k = k.wrapping_add(1);
            if k as i32 >= read_i32(G_CLUB_COUNT) {
                break;
            }
        }
    }
    if n == 0 {
        exe::crt_free(list);
        show_error_at(0x9c43b0, 0x2be);
        return;
    }
    covered!("mwf_18 conference exchange");
    exe::comp_util_sort_and_shuffle_top_clubs(list, n as u16 as u32, 1);
    let best = read_usize(list);
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), best);
    let seats = read_i16(third.addr() + league::REQUIRED_SEATS) as i32;
    let capacity = read_i16(third.addr() + league::REQUIRED_CAPACITY) as i32;
    let ground_ok = exe::finance_upgrade_stadium_for_league(finance, seats, capacity, read_usize(third.competition().addr()), 1) != 0; // mwf18: the Conference club's ground
    if !ground_ok {
        covered!("mwf_18 conference club refused");
        write_u8(exe::league_get_table_row(third, third.club_count() as i32 - 1) + league_table_row::STATUS, 0xfe);
        let news: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(0x48ece0usize);
        news(third.addr(), comp_record_of_key(KEY_CLUB_COMP_ENGLISH_CONFERENCE), best, 0, 2);
        exe::crt_free(list);
        return;
    }
    covered!("mwf_18 conference club up");
    exe::league_move_club_to_division(this, best, comp_record_of_key(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION), 1);
    let relegate: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6831a0usize);
    let mut i = 0i32;
    while i < third.club_count() as i32 {
        if read_u8(exe::league_get_table_row(third, i) + league_table_row::STATUS) == 3 {
            relegate(this.addr(), read_usize(exe::league_get_table_row(third, i)), comp_record_of_key(KEY_CLUB_COMP_ENGLISH_CONFERENCE), 1);
        }
        i += 1;
    }
    exe::crt_free(list);
}

/// Comparer for `eng_prm_exchange_conference_clubs_unplayed`: only the original runs (it may upgrade the stadium, move
/// clubs and send news); afterwards every club in a Third Division row marked relegated (status 3) must be in the
/// Conference, unless the Conference club was refused (the last row then has status 0xfe).
pub(crate) unsafe extern "thiscall" fn compare_eng_prm_exchange_conference_clubs_unplayed(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ENG_PRM_EXCHANGE_CONFERENCE_CLUBS_UNPLAYED)(this);
    let third = League(comp_by_global(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION));
    let conference = comp_record_of_key(KEY_CLUB_COMP_ENGLISH_CONFERENCE);
    let last = third.club_count() as i32 - 1;
    if last >= 0 && read_u8(exe::league_get_table_row(third, last) + league_table_row::STATUS) == 0xfe {
        return; // the refused case: nothing moved
    }
    let wrong: Vec<usize> = (0..third.club_count().max(0) as i32)
        .map(|i| exe::league_get_table_row(third, i))
        .filter(|&row| read_u8(row + league_table_row::STATUS) == 3 && read_usize(read_usize(row) + club::DIVISION) != conference)
        .map(|row| read_usize(row))
        .collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("eng_prm_exchange_conference_clubs_unplayed({:#x}) relegated clubs not in the Conference: {:x?}", this, wrong));
}

// ---------- mwf_19: the Korean League Cup clubs, group A and next stage (kor_league_cup.cpp) ----------

/// The ten Korean League Cup clubs: every K League club, sorted by last season's position.
///
/// `+0xf2` = 10 and the list at `+0xee` (`exe::crt_calloc`(4, 10): push 10; push 4 -> cdecl; out of memory, the box of
/// kor_league_cup.cpp line 0x2ab and 0). In club table order (the index compared as a word with `G_CLUB_COUNT`), every
/// club whose division has the id of [`KEY_CLUB_COMP_KOREAN_LEAGUE`] enters the list (not checked against the ten
/// places: an eleventh K League club writes past the list, a defect of the original [inference]). The list is sorted
/// (`exe::crt_qsort`(list, n, 4, 0x4b0780), the comparer of last season's position); with fewer than `+0xf2` clubs, the
/// box of line 0x2bf (and it goes on). Returns 1.
/// Original: `KorLeagueCup::select_teams` (0x66cf20, kor_league_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_select_teams(this: usize) -> u32 {
    write_u32(this + 0xf2, 10);
    let list = exe::crt_calloc(4, 10);
    write_usize(this + 0xee, list);
    if list == 0 {
        show_error_at(0x9eb15c, 0x2ab);
        return 0;
    }
    let mut n = 0u32;
    let mut k = 0i16;
    if read_i32(G_CLUB_COUNT) > 0 {
        loop {
            let c = read_usize(G_CLUB_TABLE) + k as u16 as usize * club::SIZE;
            let d = read_usize(c + club::DIVISION);
            if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_KOREAN_LEAGUE) {
                n += 1;
                write_usize(read_usize(this + 0xee) + (n as usize - 1) * 4, c);
            }
            k = k.wrapping_add(1);
            if k as i32 >= read_i32(G_CLUB_COUNT) {
                break;
            }
        }
    }
    covered!("mwf_19 korean league cup clubs");
    exe::crt_qsort(read_usize(this + 0xee), n, 4, 0x4b0780usize); // mwf19: by last season's position
    if (n as i32) < read_i32(this + 0xf2) {
        show_error_at(0x9eb15c, 0x2bf);
    }
    1
}

/// Comparer for `kor_league_cup_select_teams`: only the original runs (it allocates the list); the plan, made before,
/// is the K League clubs in club table order sorted by the same comparer, and the original's list must hold them.
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_select_teams(this: usize) -> u32 {
    let mut plan: Vec<usize> = (0..read_i32(G_CLUB_COUNT).max(0) as usize)
        .map(|k| read_usize(G_CLUB_TABLE) + k * club::SIZE)
        .filter(|&c| { let d = read_usize(c + club::DIVISION); d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_KOREAN_LEAGUE) })
        .collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_KOR_LEAGUE_CUP_SELECT_TEAMS)(this);
    if r == 0 || plan.len() > 10 {
        return r; // simplification: out of memory, or more clubs than places (the original writes past the list)
    }
    exe::crt_qsort(plan.as_mut_ptr() as usize, plan.len() as u32, 4, 0x4b0780usize);
    let list = read_usize(this + 0xee);
    let got: Vec<usize> = (0..plan.len()).map(|k| read_usize(list + k * 4)).collect();
    SCOREBOARD.record(got == plan && read_u32(this + 0xf2) == 10, || format!("kor_league_cup_select_teams({:#x}) plan {:x?} list {:x?}", this, plan, got));
    r
}

/// Korean League Cup group A: the cup itself is group A, with the five even-numbered seeds (list places 0, 2, 4, 6 and
/// 8), each taken out of the list.
///
/// `.club_count()` = 5; the table (`exe::crt_malloc` of 0x127 = 5 rows of 0x3b; out of memory, the box of line 0x17f
/// and 0) into `.table()`. For i = 0..4 (a signed byte compared with the word `.club_count()`): row i gets the club at
/// list place 2i (League::init_table_row `exe::league_init_table_row`, thiscall(this, i, club, 0, 0), ret 0x10; the
/// original pushes the dword of the local byte, with stack garbage above it, and the callee reads the byte) and that
/// place is zeroed. Then the group draw news (League::send_group_draw_news `exe::league_send_group_draw_news`). Returns
/// 1.
/// Original: `KorLeagueCup::create_group_a_table` (0x66c770, kor_league_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_create_group_a_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(5);
    let t = exe::crt_malloc(0x127);
    this.set_table(t);
    if t == 0 {
        show_error_at(0x9eb15c, 0x17f);
        return 0;
    }
    covered!("mwf_19 korean league cup group a");
    let mut i: i8 = 0;
    while (i as i16) < this.club_count() {
        let slot = read_usize(this.addr() + 0xee) + i as isize as usize * 8; // the even-numbered seeds
        exe::league_init_table_row(this, i as u8 as u32, read_usize(slot), 0, 0); // mwf19: row i
        write_usize(slot, 0);
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

/// Comparer for `kor_league_cup_create_group_a_table`: only the original runs (init_table_row allocates and the news
/// goes to the human managers); the plan is the clubs at list places 0, 2, 4, 6 and 8, which must be in the table rows
/// in order, with those places zeroed.
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_create_group_a_table(this: usize) -> u32 {
    let list = read_usize(this + 0xee);
    let plan: Vec<usize> = (0..5).map(|k| read_usize(list + k * 8)).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_KOR_LEAGUE_CUP_CREATE_GROUP_A_TABLE)(this);
    let l = League(this);
    let t = l.table();
    let ok = r == 1 && l.club_count() == 5 && t != 0 && plan.iter().enumerate().all(|(k, &c)| read_usize(t + k * 0x3b) == c && read_usize(list + k * 8) == 0);
    SCOREBOARD.record(ok, || format!("kor_league_cup_create_group_a_table({:#x}) returned {}; plan {:x?}", this, r, plan));
    r
}

/// The Korean League Cup next stage: the group stage moves on and, when it leaves stage 0 for stage 1, the knockout
/// stage is built; past the last stage it is an error (the box).
///
/// At the last stage (`.current_stage()` ≥ `.stage_count()` − 1, signed) the box of line 0x21e and nothing else;
/// otherwise `league::CURRENT_STAGE` goes up by one and, when it was 0, KorLeagueCup::create_knockout_stage (0x66cd50,
/// thiscall, ret 0).
/// Original: `KorLeagueCup::advance_stage` (0x66cad0, kor_league_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_advance_stage(this: usize) {
    let this = League(this);
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x9eb15c, 0x21e);
    }
    this.set_current_stage((stage + 1) as usize);
    if stage == 0 {
        covered!("mwf_19 korean league cup knockout");
        let knockout: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x66cd50usize);
        knockout(this.addr());
    }
}

/// Comparer for `kor_league_cup_advance_stage`: only the original (it may build the knockout stage); the stage must
/// stay at the last one or go up by one.
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_KOR_LEAGUE_CUP_ADVANCE_STAGE);
    compare_state_league_advance_stage("kor_league_cup_advance_stage", League(this), || o(this))
}
