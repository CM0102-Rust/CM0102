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
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("fra_first_start_new_season", League(this), || o(this))
}
