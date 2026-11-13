// ---------- mwf_9: the season turnovers of the Serie C Italian Cup, the Coppa Italia and the Korean League Cup (ita_c_cup.cpp, ita_cup.cpp, kor_league_cup.cpp) ----------

/// The season turnover of a cup played as a league with group stages (the Coppa Italia, the Serie C cup and the Korean
/// League Cup: the same body, checked instruction by instruction for the two Italian ones and by reading for the Korean
/// one, which only uses another register as zero; only the routines, the number of groups, the source file and the box
/// lines change).
///
/// `league::SEASON_CLOSED` = 0; the table freed (League::free_table `exe::league_free_table`, thiscall(this, 0), ret 4);
/// `.round_dates()` freed (`exe::crt_free`) and zeroed; with statistics, Comp_Stats::archive_season_and_reset
/// (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the entrant list at `+0xee`
/// (`ita_cup_fields::ENTRANT_CLUBS`; the ten selected clubs in the Korean League Cup) freed when there is one, but not
/// zeroed (the entrants routine allocates a new one next); with a current stage ≥ 0, the stages 0..=`.current_stage()` destroyed (virtual 0 with 1); the year up by
/// `year` and `league::CURRENT_STAGE` = −1.
///
/// Then the entrants (`entrants`, thiscall, ret 0; its result is not checked) and the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`); then, each with its own box (lines `rows`) when it returns 0: the rules
/// (virtual `comp_vtable::SET_UP_RULES`), the first group's table (`first_group`, thiscall, ret 0: the cup itself is
/// group 1), the fixtures (League::create_fixtures `exe::league_create_fixtures`) and the order (League::sort_table
/// `exe::league_sort_table`, thiscall(this, 0), ret 4). Finally the group stages `group_stage`(g) for g = 0 to
/// `groups` − 1 (thiscall, ret 4), the last one's eax returned. After a box the copy returns 0 (the original returns
/// the box's eax; the turnover's caller does not read it).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn ita_group_cup_start_new_season(this: League, entrants: usize, first_group: usize, group_stage: usize, groups: u32, file: usize, rows: [u32; 4], year: i16, ramo: &'static str) -> u32 {
    this.set_season_closed(0);
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
    let list = read_usize(this.addr() + ita_cup_fields::ENTRANT_CLUBS);
    if list != 0 {
        exe::crt_free(list);
    }
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    let a = this.year();
    this.set_year(a.wrapping_add(year));
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    covered!(ramo);
    let entrants: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(entrants);
    entrants(this.addr());
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    let first_group: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(first_group);
    if first_group(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, rows[3]);
        return 0;
    }
    let group_stage: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(group_stage);
    let mut r = 0;
    for g in 0..groups {
        r = group_stage(this.addr(), g);
    }
    r
}

/// The season turnover of the Serie C Italian Cup: the season archived, last season's stages and entrant list dropped,
/// the year plus one, the 90 Serie C entrants drawn again (ItaCCup::select_entrants), the predicted positions, the
/// rules, the first group of 5 (ItaCCup::init_first_group), its fixtures and order, and the 15 group stages
/// (ItaCCup::create_group_stage 0..14).
///
/// `ita_group_cup_start_new_season` with 0x63dfe0, 0x63d5e0, 0x63d9e0, 15 groups and the ita_c_cup.cpp boxes at lines
/// 0x212, 0x219, 0x21f and 0x226.
/// Original: `ItaCCup::start_new_season` (0x63d780, ita_c_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_c_cup_start_new_season(this: usize) -> u32 {
    ita_group_cup_start_new_season(League(this), 0x63dfe0, 0x63d5e0, 0x63d9e0, 15, 0x9ea204, [0x212, 0x219, 0x21f, 0x226], 1, "mwf_9 serie c cup rollover") // mwf9: the Serie C cup turnover
}

/// Comparer for `ita_c_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go
/// up by one.
pub(crate) unsafe extern "thiscall" fn compare_ita_c_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("ita_c_cup_start_new_season", League(this), || o(this))
}

/// The season turnover of the Coppa Italia: as the Serie C cup's, with the 48 entrants (ItaCup::select_entrants), the
/// first group of 4 (ItaCup::create_first_group_table) and 7 group stages (ItaCup::create_group_stage 0..6).
///
/// `ita_group_cup_start_new_season` with 0x63faf0, 0x63f250, 0x63f640, 7 groups and the ita_cup.cpp boxes at lines
/// 0x210, 0x217, 0x21d and 0x224.
/// Original: `ItaCup::start_new_season` (0x63f3e0, ita_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_cup_start_new_season(this: usize) -> u32 {
    ita_group_cup_start_new_season(League(this), 0x63faf0, 0x63f250, 0x63f640, 7, 0x9ea23c, [0x210, 0x217, 0x21d, 0x224], 1, "mwf_9 coppa italia rollover") // mwf9: the Coppa Italia turnover
}

/// Comparer for `ita_cup_start_new_season` (original only; the year plus one).
pub(crate) unsafe extern "thiscall" fn compare_ita_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("ita_cup_start_new_season", League(this), || o(this))
}

/// The season turnover of the Korean League Cup: the season archived, last season's stages and team list dropped, the
/// year plus one, the ten K League clubs selected again (KorLeagueCup::select_teams, sorted by last season's
/// position), the predicted positions, the rules, group A (KorLeagueCup::create_group_a_table: the even-numbered seeds,
/// the cup itself), its fixtures and order, and the group B stage (KorLeagueCup::create_group_b_stage: the odd-numbered
/// seeds).
///
/// `ita_group_cup_start_new_season` with 0x66cf20, 0x66c770, 0x66cb60 (push 0: the one group stage, ret 4) and the
/// kor_league_cup.cpp boxes at lines 0x1f1, 0x1f8, 0x1fe and 0x205.
/// Original: `KorLeagueCup::start_new_season` (0x66c900, kor_league_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_start_new_season(this: usize) -> u32 {
    ita_group_cup_start_new_season(League(this), 0x66cf20, 0x66c770, 0x66cb60, 1, 0x9eb15c, [0x1f1, 0x1f8, 0x1fe, 0x205], 1, "mwf_9 korean league cup rollover") // mwf9: the Korean League Cup turnover
}

/// Comparer for `kor_league_cup_start_new_season` (original only; the year plus one).
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_state_league_new_season("kor_league_cup_start_new_season", League(this), || o(this))
}
