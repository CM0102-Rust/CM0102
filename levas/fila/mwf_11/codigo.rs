// ---------- mwf_11: the Korean league turnover and the rules of the Korean league and League Cup (kor_league.cpp, kor_league_cup.cpp) ----------

/// The season turnover of the Korean league (the K League): the final positions recorded first, then the season
/// archived, the table and the dates cleared, the year plus one, the rules, the ten clubs, the fixtures and the order
/// rebuilt; then the predicted positions, the board expectations and the referee renewal. Unlike the sisters it does
/// not touch the stages (the K League has none) nor `league::CURRENT_STAGE`.
///
/// `league::SEASON_CLOSED` = 0; the final positions (virtual `league_vtable::RECORD_FINAL_POSITIONS`, thiscall, ret 0);
/// the table freed (League::free_table `exe::league_free_table`, thiscall(this, 0), ret 4); `.round_dates()` freed
/// (`exe::crt_free`) and zeroed; with statistics, Comp_Stats::archive_season_and_reset
/// (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the year up by one. Then, each with its own
/// box (kor_league.cpp, lines 0x206, 0x20d, 0x213 and 0x21a) when it returns 0: the rules (virtual
/// `comp_vtable::SET_UP_RULES`), the table (KorLeague::create_table 0x66bba0, thiscall, ret 0: the ten clubs of the
/// division and the prize fund), the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order
/// (League::sort_table `exe::league_sort_table`, thiscall(this, 0), ret 4); then the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`) and `set_expectations_and_renew_officials`, whose eax (the referee
/// renewal's) it returns. After a box the copy returns 0 (the original returns the box's eax; the turnover's caller
/// does not read it).
/// Original: `KorLeague::start_new_season` (0x66bd60, kor_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    bra_nat_first_call_virtual_no_args(this.addr(), league_vtable::RECORD_FINAL_POSITIONS);
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
    this.set_year(a.wrapping_add(1)); // mwf11: the year of the K League
    covered!("mwf_11 k league rollover");
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(0x9eb120, 0x206);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x66bba0usize);
    if table(this.addr()) == 0 {
        show_error_at(0x9eb120, 0x20d);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0x9eb120, 0x213);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(0x9eb120, 0x21a);
        return 0;
    }
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// Comparer for `kor_league_start_new_season`: only the original runs (it frees and rebuilds the league); the year must
/// go up by one.
pub(crate) unsafe extern "thiscall" fn compare_kor_league_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("kor_league_start_new_season", League(this), || o(this))
}

/// Writes the K League rule fields (what `kor_league_set_up_rules` writes before the schedule).
pub(crate) unsafe fn set_k_league_rules(o: usize) {
    write_u16_at(o, 0xab, 0); // no match rule bits
    write_u16_at(o, 0xdb, 0);
    write_u8_at(o, 0xbe, 0); // nobody goes up or down
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 0);
    write_u16_at(o, 0x3c, 3); // three round robins
    write_u8_at(o, 0xc2, 3); // mwf11: the K League points per win
    write_u8_at(o, 0xc7, 3);
    write_u8_at(o, 0x4a, 3); // substitutions
    write_u8_at(o, 0xc4, 2); // match category: league
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0x52, 2); // fixed squad numbering
    write_u8_at(o, 0xc3, 1); // points per draw
    write_u8_at(o, 0x42, 1); // club sides
    write_u8_at(o, 0xc5, 1); // tiebreakers 1, 2, 3
    write_u32_at(o, 0x1c, 0xffff_ffff); // no league above
    write_u32_at(o, 0x20, 0xffff_ffff); // no league below
    write_u8_at(o, 0x49, 7); // substitutes on the bench
}

/// The K League rules: a triple round robin (3 legs), 3 points a win and 1 a draw, tiebreakers 1, 2 and 3, no
/// promotion or relegation (no league above or below), club sides, league category, 7 substitutes and 3 changes, fixed
/// squad numbers; then the season schedule.
///
/// Writes the bytes and words of `set_k_league_rules` (`+0xab` and `+0xdb` words 0, `+0xbe`..`+0xc1` 0, `+0x3c` word
/// 3, `+0xc2` 3, `+0xc7` 3, `+0x4a` 3, `+0xc4` 2, `+0xc6` 2, `+0x52` 2, `+0xc3` 1, `+0x42` 1, `+0xc5` 1, `+0x1c` and
/// `+0x20` −1, `+0x49` 7), then the round table of stage −1 (virtual +0x3c: push 0; push &`+0x3a`; push &`+0xa9`;
/// push −1 -> thiscall, ret 0x10) into `.round_dates()` (`create_main_schedule`). Returns 1.
/// Original: `KorLeague::set_up_rules` (0x66bcc0, kor_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_set_up_rules(this: usize) -> u32 {
    set_k_league_rules(this);
    covered!("mwf_11 k league rules");
    create_main_schedule(League(this));
    1
}

/// Comparer for `kor_league_set_up_rules`: side by side over the league object without the schedule outputs (+0x3a,
/// +0xa9, +0xba, which the virtual +0x3c allocates); the copy only writes the rule fields.
pub(crate) unsafe extern "thiscall" fn compare_kor_league_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this), || {
        set_k_league_rules(this);
        1
    });
    SCOREBOARD.record(same && r == n, || format!("kor_league_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Writes the Korean League Cup rule fields (what `kor_league_cup_set_up_rules` writes before the schedule).
pub(crate) unsafe fn set_korean_league_cup_rules(o: usize) {
    write_u8_at(o, 0xc7, 0);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 0);
    write_u16_at(o, 0x3c, 2); // a double round robin
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0xbe, 2); // the top two of each group go through
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u8_at(o, 0xc2, 3); // mwf11: the League Cup points per win
    write_u8_at(o, 0xc3, 1); // points per draw
    write_u8_at(o, 0xc4, 4); // match category: cup
    write_u8_at(o, 0xc5, 1);
    write_u8_at(o, 0x42, 1); // club sides
    write_u16_at(o, 0xab, 0x83); // match rule bits
    write_u32_at(o, 0x28, 0x186a0); // prize fund 100,000
}

/// The Korean League Cup group rules: a double round robin, 3 points a win and 1 a draw, tiebreakers 1 and 2, the top
/// two of each group through, no league above or below, club sides, cup category (4), match rule bits 0x83 and a prize
/// fund of 100,000; then the group schedule. The substitutes and changes are not set here.
///
/// Writes the bytes and words of `set_korean_league_cup_rules` (`+0xc7` and `+0xbf`..`+0xc1` 0, `+0x3c` word 2,
/// `+0xc6` 2, `+0xbe` 2, `+0x1c` and `+0x20` −1, `+0xc2` 3, `+0xc3` 1, `+0xc4` 4, `+0xc5` 1, `+0x42` 1, `+0xab` word
/// 0x83, `+0x28` 100,000), then the round table of stage −1 (virtual +0x3c: push 0; push &`+0x3a`; push &`+0xa9`; push
/// −1 -> thiscall, ret 0x10) into `.round_dates()` (`create_main_schedule`). Returns 1.
/// Original: `KorLeagueCup::set_up_rules` (0x66c870, kor_league_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_set_up_rules(this: usize) -> u32 {
    set_korean_league_cup_rules(this);
    covered!("mwf_11 korean league cup rules");
    create_main_schedule(League(this));
    1
}

/// Comparer for `kor_league_cup_set_up_rules`: side by side over the league object without the schedule outputs.
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this), || {
        set_korean_league_cup_rules(this);
        1
    });
    SCOREBOARD.record(same && r == n, || format!("kor_league_cup_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}
