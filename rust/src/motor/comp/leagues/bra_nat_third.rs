use super::*;

/// Club in the first table row of stage k of the league (k = -1: the league itself).
pub(crate) unsafe fn stage_leader(this: League, k: i32) -> usize {
    let t = League(if k == -1 { this.addr() } else { read_usize(this.stages() + 4 * k as usize) });
    read_usize(t.table())
}

/// Creates the Series C final: the winner of third-round group A against the winner of group B, in a two-club cup stage
/// [inference: home and away]; the winner is the Series C champion.
///
/// SEH frame only for the `new`. `BraNatThird::create_final_stage()`, the Series C final, called at the end of the
/// third round: the champions of the two third-round groups (the first of stage 8's table and that of stage 9, in that
/// order; `stage_leader`) in a local list of 2; the round table through `comp_vtable::CREATE_SCHEDULE`
/// (create_round_table: thiscall(this, 10, &rounds, &extra, 0), ret 0x10); the stage (operator new
/// `exe::crt_operator_new`(0xb2) and the Cup_Stage constructor `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret 0x3c: the
/// league, 2, the list, the rounds, `.competition()`, the table, the year, 10, 1, the extra, 0x14, 0, 0, 0, null seeds;
/// clean words) into `.stages()`[10]. Without a stage: the error box at line 0x392 and it returns (without freeing the
/// table). With one: the table freed (free `exe::crt_free`) and `.set_current_stage(10)`. [inference] The final decides
/// the Series C champion, home and away.
/// Original: `BraNatThird::create_final_stage` (0x42e980, bra_nat_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_third_create_final_stage(this: usize) {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let mut list = [0usize; 2];
    list[0] = stage_leader(this, 8); // cqs4: the group A winner
    list[1] = stage_leader(this, 9);
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 10, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        construct(p, this.addr(), 2, list.as_ptr(), rodadas as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 10, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages() + 0x28, stage);
    if read_usize(this.stages() + 0x28) == 0 {
        return show_error_at(0x98aba4, 0x392);
    }
    covered!("cqs_4 final");
    exe::crt_free(t);
    this.set_current_stage((10) as usize);
}

/// The Series C season turnover: archives the season, clears the table, the schedule and the stages, moves to the next
/// year and rebuilds the rules, the 56-club list and the table, the fixtures and the order; then the board
/// expectations, the referee renewal, the seven first-round groups and the predicted positions; the 56-club list is
/// discarded at the end.
///
/// `BraNatThird::start_new_season()`, the Series C turnover (called from the Series A turnover):
/// `league::SEASON_CLOSED` = 0 and the table freed (League::free_table `exe::league_free_table`, thiscall(this, 0), ret
/// 4); the schedule (`league::ROUND_DATES`) freed (free `exe::crt_free`) and cleared; with statistics
/// (`league::COMP_STATS`), Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`,
/// thiscall(stats, 1), ret 4); with the current stage ≥ 0, the existing stages 0..=`.current_stage()` are destroyed
/// (virtual 0, the destructor with 1); `league::CURRENT_STAGE` = −1; the stage array freed; the year (`league::YEAR`,
/// word) goes up by one; a new array of `.stage_count()` stages (malloc of 4 × `.stage_count()`, not cleared; with
/// `.stage_count()` ≤ 0, null) into `league::STAGES`; then, each with its own error box (lines 0x202, 0x209, 0x20f and
/// 0x216) if it returns 0: the rules (`comp_vtable::SET_UP_RULES`), the table and the list of the 56
/// (`exe::branatthird_create_table`, thiscall, ret 0), the fixtures (League::create_fixtures
/// `exe::league_create_fixtures`) and the table order (League::sort_table `exe::league_sort_table`, thiscall(this, 0),
/// ret 4); finally the boards' expectations (League::set_club_season_expectations
/// `exe::league_set_club_season_expectations`, thiscall, ret 0), the referee renewal (OfficialsManager::renew_officials
/// `exe::officialsmanager_renew_officials`, thiscall(`[G_OFFICIALS_MANAGER]`, `league::COMPETITION`), ret 4), the seven
/// first-round groups (`exe::branatthird_create_group_stage`(g), thiscall, ret 4, g from 0 to 6), the predicted
/// positions (`comp_vtable::SET_PREDICTED_POSITIONS`) and the list of the 56 (`bra_nat_third::CLUB_LIST`) freed (if
/// present) and cleared. With an error box, the copy returns 0 (the original returns the box's eax, which the Brazilian
/// turnover does not read); at the end, the eax of the free or `bra_nat_third::CLUB_LIST` (the copy returns 0: nobody
/// reads it).
/// Original: `BraNatThird::start_new_season` (0x42def0, bra_nat_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_third_start_new_season(this: usize) -> u32 {
    let this = League(this);
    write_u32(this.addr() + league::SEASON_CLOSED, 0);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        write_usize(this.addr() + league::ROUND_DATES, 0);
    }
    let stats = read_usize(this.addr() + league::COMP_STATS);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
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
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    exe::crt_free(this.stages());
    let n = this.stage_count() as i32;
    let a = this.year();
    write_i16(this.addr() + league::YEAR, a.wrapping_add(1));
    covered!("cqs_4 serie c rollover");
    let v = if n > 0 { exe::crt_malloc((n * 4) as u32) } else { 0 };
    write_usize(this.addr() + league::STAGES, v);
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES));
    if rules(this.addr()) == 0 {
        show_error_at(0x98aba4, 0x202);
        return 0;
    }
    if exe::branatthird_create_table(this) == 0 {
        show_error_at(0x98aba4, 0x209);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0x98aba4, 0x20f);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(0x98aba4, 0x216);
        return 0;
    }
    exe::league_set_club_season_expectations(this);
    (exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), read_usize(this.addr() + league::COMPETITION)) as u32); // cqs4: the renewal of the Serie C referees
    for g in 0..7 {
        exe::branatthird_create_group_stage(this, g);
    }
    let predicted: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
    predicted(this.addr());
    let l = read_usize(this.addr() + bra_nat_third::CLUB_LIST);
    if l != 0 {
        exe::crt_free(l);
    }
    write_usize(this.addr() + bra_nat_third::CLUB_LIST, 0);
    0
}

/// The predicted positions of the Série C, in the order of the list of 56 after sorting.
///
/// 1, 2, 3, 3, 5, 5, 7, 7, eight times 9 (the knockout ranges) and, for the other forty, the range in steps of eight:
/// 16, 24, 32, 40 and 48.
pub(crate) const SERIE_C_PREDICTED_PLACES: [u8; 16] = [1, 2, 3, 3, 5, 5, 7, 7, 9, 9, 9, 9, 9, 9, 9, 9]; // cqs4: the expected knockout positions

/// Place for index k: from SERIE_C_PREDICTED_PLACES below 16, else 16 + 8 x ((k - 16) / 8) [inference].
pub(crate) fn place_for_index(k: usize) -> u8 {
    if k < 16 { SERIE_C_PREDICTED_PLACES[k] } else { (16 + 8 * ((k - 16) / 8)) as u8 }
}

/// Sets the Series C predicted positions: the 56 ordered by reputation; the favourite 1st, the second 2nd, the next two
/// 3rd (the semi-finals), two 5th and two 7th, eight 9th (the second round) and the other forty in bands of eight:
/// 16th, 24th, 32nd, 40th and 48th.
///
/// `BraNatThird::set_predicted_positions()`, called from the constructor and the turnover, after the groups. Without
/// statistics (+8): the error box at line 0x4a2 and it returns. The list of the 56 (`bra_nat_third::CLUB_LIST`) is
/// sorted in place by reputation (qsort `exe::crt_qsort` with 0x4b07d0, highest first, nulls at the end) and each club,
/// in order, gets the predicted position from `place_for_index` (Comp_Stats::set_club_final_position
/// `exe::comp_stats_set_club_final_position`, thiscall(+8, club, position), ret 8; the original pushes the band of the
/// forty in a dword with the band's byte and stack garbage above it; `set_club_final_position` reads only the byte: the
/// copy passes the clean byte).
/// Original: `BraNatThird::set_predicted_positions` (0x42f0a0, bra_nat_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_third_set_predicted_positions(this: usize) {
    if read_usize(this + 8) == 0 {
        return show_error_at(0x98aba4, 0x4a2);
    }
    exe::crt_qsort(read_usize(this + bra_nat_third::CLUB_LIST), 0x38, 4, 0x4b07d0); // cqs4: the list of the 56 by reputation
    for k in 0..0x38usize {
        exe::comp_stats_set_club_final_position(read_usize(this + 8), read_usize(read_usize(this + bra_nat_third::CLUB_LIST) + 4 * k), place_for_index(k) as u32);
    }
    covered!("cqs_4 expected positions");
}

#[allow(clippy::type_complexity)]
pub(crate) type LeagueStageCtorFn = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize;

/// `g.create_group_stage()` (`bra_nat_third_create_group_stage`, thiscall, ret 4, bra_nat_third.cpp; SEH frame only for the new):
/// group g (0 to 6) of the first stage of Série C, from the season rollover and the constructor (g from 0 to 6; the
/// league is the top group, and the seven groups are stages 0 to 6). Round table, the 7 clubs of group g, League_Stage
/// in slot g, +0x30 = g.
///
/// 1) The round table via the virtual +0x3c (create_round_table: thiscall(this, g as int, &rounds, &extra, 0), ret
/// 0x10).
///
/// 2) A vector of 4 × +0x3e (malloc `exe::crt_malloc`; out of memory: the box for line 0x475 and return) with the seven clubs of
/// the group: slots 7(g + 1) to 7(g + 1) + 6 of the list of 56 (+0xee; the first seven belong to the league).
///
/// 3) The stage: operator new `exe::crt_operator_new`(0xee) and the League_Stage constructor 0x68b100 (thiscall, ret 0x58, 22
/// arguments, in the original's order: the league, 7, the vector, 2 (legs: home and away), +4, the round table, the
/// rounds (word), the points +0xc2 (`pv`) and +0xc3, the category +0xc4, &+0xc5, &+0xbe (the league's slots), the year
/// +0x40 (word), g (the whole dword), the extra (word), +0x51, 1, 0, +0xd9 (word), −1, 0, 2; words and bytes clean).
/// Out of memory: 0.
///
/// 4) [+0xc][g] = the stage. No stage: the box for line 0x487 and return (without freeing the vector or the table).
/// With it: the vector and the table freed (free `exe::crt_free`) and +0x30 = g (the signed byte).
pub(crate) unsafe fn bra_nat_third_create_group_stage_cqs3_group(this: League, g: u32, pv: usize) {
    let construct: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), g, &mut rodadas, &mut extra, 0);
    let v = exe::crt_malloc(((this.club_count() as i32) << 2) as u32);
    if v == 0 {
        return show_error_at(0x98aba4, 0x475);
    }
    let k = g as u8 as i8 as i32;
    for j in 0..7usize {
        write_usize(v + 4 * j, read_usize(read_usize(this.addr() + bra_nat_third::CLUB_LIST) + 4 * (7 * (k + 1) as usize + j)));
    }
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(p, this.addr(), 7, v, 2, this.competition().addr(), t, rodadas as u16 as u32, read_u8(this.addr() + pv) as u32, this.draw_points() as u32,
                  this.match_category() as u32, this.addr() + league::TIEBREAKS, this.addr() + league::PROMOTED_COUNT, this.year() as u16 as u32, g, extra as u16 as u32,
                  this.importance() as u32, 1, 0, this.league_flags() as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    write_usize(this.stages() + 4 * k as usize, stage);
    if read_usize(this.stages() + 4 * k as usize) == 0 {
        return show_error_at(0x98aba4, 0x487);
    }
    covered!("cqs_3 group");
    exe::crt_free(v);
    exe::crt_free(t);
    this.set_current_stage((k) as usize);
}

/// A Series C first-round group: seven clubs of the 56-club list (the first seven slots are the league's own group;
/// group g takes slots 7(g+1) to 7(g+1)+6), home and away round robin, with the league points, tie-breakers and places;
/// the top two of each of the eight groups go to the second round
/// Original: `BraNatThird::create_group_stage` (0x42ee80, bra_nat_third.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn bra_nat_third_create_group_stage(this: usize, g: u32) {
    bra_nat_third_create_group_stage_cqs3_group(League(this), g, 0xc2) // cqs3: the points per win in the groups
}

/// A ordem da segunda fase: o k-ésimo melhor (pelos pontos) vai à vaga `SECOND_ROUND_SEEDS[k]` da lista dos dezesseis.
pub(crate) const SECOND_ROUND_SEEDS: [u8; 16] = [0, 2, 4, 6, 8, 10, 12, 14, 15, 13, 11, 9, 7, 5, 3, 1]; // cqs3: the second stage slots

/// Creates the Series C second round: the sixteen (the top two of each group) seeded by first-round points (ties go to
/// the earlier one: the league, then groups 0 to 6, first before second), in a knockout where the best meets the 16th,
/// the 2nd the 15th and so on [inference: pairs follow the list].
///
/// SEH frame only for the `new`. `BraNatThird::create_second_round_stage()`, the Series C second round, called at the
/// end of the groups.
///
/// 1) A list of 16 (malloc `exe::crt_malloc` of 0x40; out of memory: the error box at line 0x2e6 and it returns).
///
/// 2) The top two of each of the eight groups (the league's table `league::TABLE` and those of stages 0 to 6, in that
/// order), in 6-byte records (the club and the points, the row's `league_table_row::POINTS`).
///
/// 3) Sixteen times: the record with most points (the first one, on a tie) among those that still have a club is taken
/// out (the club becomes 0) and the club goes to slot `SECOND_ROUND_SEEDS[k]`: the best against the 16th, the 2nd
/// against the 15th... (with none left: the error box at line 0x30b; the original then reads address 0 and crashes).
///
/// 4) The round table through `comp_vtable::CREATE_SCHEDULE` (thiscall(this, 7, &rounds, &extra, 0), ret 0x10); the
/// stage (operator new `exe::crt_operator_new`(0xb2) and the Cup_Stage constructor `FN_CUP_STAGE_CUP_STAGE`, thiscall,
/// ret 0x3c: the league, 16, the list, the rounds, `.competition()`, the table, the year, 7, 1, the extra, 0x14, 0, 0,
/// 0, null seeds; clean words) into `.stages()`[7]. Without a stage: the error box at line 0x31b and it returns. With
/// one: the table and the list freed (free `exe::crt_free`) and `.set_current_stage(7)`.
/// Original: `BraNatThird::create_second_round_stage` (0x42e430, bra_nat_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn create_second_round_stage(this: usize) {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let list = exe::crt_malloc(0x40);
    if list == 0 {
        return show_error_at(0x98aba4, 0x2e6);
    }
    let mut r = [(0usize, 0i16); 16];
    for (k, g) in (-1i32..7).enumerate() {
        let t = read_usize(if g == -1 { this.addr() } else { read_usize(this.stages() + 4 * g as usize) } + league::TABLE);
        r[2 * k] = (read_usize(t), read_i16(t + league_table_row::POINTS));
        r[2 * k + 1] = (read_usize(t + 0x3b), read_i16(t + 0x54));
    }
    for k in 0..16 {
        let mut best: Option<usize> = None;
        for (i, e) in r.iter().enumerate() {
            if best.map_or(true, |m| e.1 > r[m].1) && e.0 != 0 {
                best = Some(i);
            }
        }
        let c = match best {
            Some(m) => {
                let c = r[m].0;
                r[m].0 = 0;
                c
            }
            None => {
                show_error_at(0x98aba4, 0x30b);
                core::ptr::read_volatile(core::ptr::null::<usize>()) // simplification: the original reads address 0 here and crashes
            }
        };
        write_usize(list + 4 * SECOND_ROUND_SEEDS[k] as usize, c);
    }
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 7, &mut rodadas, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        construct(p, this.addr(), 0x10, list, rodadas as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 7, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages() + 0x1c, stage);
    if read_usize(this.stages() + 0x1c) == 0 {
        return show_error_at(0x98aba4, 0x31b);
    }
    covered!("cqs_3 second stage");
    exe::crt_free(t);
    exe::crt_free(list);
    this.set_current_stage((7) as usize);
}

/// Creates the Series C third round: the eight second-round winners, drawn (shuffled) into two groups of four, home and
/// away, with 3 points for a win, 1 for a draw, tie-breakers {1, 2, 4} and two top places in each group [inference: the
/// top two of each group go up to Series B and the group winners play the final].
///
/// SEH frame only for the `new`. `BraNatThird::create_third_round_groups()`, the Series C third round, called at the
/// end of the second: the second-round winners (`.stages()`[7], the 6-byte records of `.team_table()` with +5 = 1, in
/// order) in a local list of 8, shuffled (`exe::comp_util_shuffle_pointer_array`, cdecl(v, 8)); the first four form
/// group A (stage 8) and the other four group B (stage 9). Each group: the round table through
/// `comp_vtable::CREATE_SCHEDULE` (thiscall(this, 8 or 9, &rounds, &extra, 0)) and the stage (operator new
/// `exe::crt_operator_new`(0xee) and the League_Stage constructor `FN_LEAGUE_STAGE_LEAGUE_STAGE`: the league, 4, the
/// list (or the list + 4), 2 (home and away), `.competition()`, the table, the rounds (word), 3 points per win, 1 per
/// draw, category 5, &{1,2,4,0} (the tie-break criteria), &{2,0,0,0} (two top places), the year, 8 or 9, the extra
/// (word), 0x14, 1, 0, 2 (the rule bits), −1, 0, 2) into `.stages()`[8] or `[9]`; without a stage: the error box at
/// line 0x353 or 0x363 and it returns; with one, the table freed (free `exe::crt_free`). After the second one,
/// `.set_current_stage(9)`. [inference] The top two of each group go up to Series B; the group winners play the final.
/// Original: `BraNatThird::create_third_round_groups` (0x42e700, bra_nat_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn create_third_round_groups(this: usize) {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, *const u8, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let slots: [u8; 4] = [2, 0, 0, 0];
    let tiebreak: [u8; 4] = [1, 2, 4, 0];
    let mut v = [0usize; 8];
    let second = Cup(read_usize(this.stages() + 0x1c));
    let mut n = 0usize;
    let mut i = 0i32;
    while i < second.team_count() as i32 {
        let e = second.team_table() + 6 * i as usize;
        // simplification: the original writes without bounds into a list of 8 on the stack; the stage has 8 winners (the copy stops at the 8th).
        if read_u8(e + 5) == 1 && n < 8 {
            v[n] = read_usize(e);
            n += 1;
        }
        i += 1;
    }
    exe::comp_util_shuffle_pointer_array((v.as_mut_ptr()) as usize, 8); // cqs3: os oito vencedores embaralhados
    for (k, f) in [(8u32, 0x353u32), (9, 0x363)] {
        let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
        let (mut rodadas, mut extra) = (0i16, 0i16);
        let t = table(this.addr(), k, &mut rodadas, &mut extra, 0);
        let p = exe::crt_operator_new(0xee);
        let stage = if p != 0 {
            construct(p, this.addr(), 4, v[4 * (k as usize - 8)..].as_ptr(), 2, this.competition().addr(), t, rodadas as u16 as u32, 3, 1, 5,
                      tiebreak.as_ptr(), slots.as_ptr(), this.year() as u16 as u32, k, extra as u16 as u32, 0x14, 1, 0, 2, -1, 0, 2)
        } else {
            0
        };
        write_usize(this.stages() + 4 * k as usize, stage);
        if read_usize(this.stages() + 4 * k as usize) == 0 {
            return show_error_at(0x98aba4, f);
        }
        exe::crt_free(t);
    }
    covered!("cqs_3 third stage");
    this.set_current_stage((9) as usize);
}

/// Builds the Series C table: up to 56 clubs (those of the division, in database order), which are drawn (3 swaps per
/// club); the first 7 of the draw make the first table, of 7 clubs, and the drawn list is kept for the other groups
/// [inference: 8 groups of 7 in the first stage].
///
/// `BraNatThird::create_table()`: `.set_club_count(7)`; the list of entrants (`bra_nat_third::CLUB_LIST`) is rebuilt
/// (the old one freed, `exe::crt_free` push; add esp -> cdecl; the new one with 0xe0 bytes = 56 pointers; out of
/// memory: the error box at line 0x18f and it returns 1, as the original); the table (`.set_table()`, 7 × 0x3b; out of
/// memory: the error box at line 0x196 and 0); the list receives, in database order, up to 56 clubs whose division
/// (`club::DIVISION`) is Series C itself (the record in `.competition()`); the list is shuffled
/// (`exe::comp_util_shuffle_pointer_array`: push count (word); push list; add esp, 8 -> cdecl; 3 × count random swaps)
/// and the first 7 of the draw go into rows 0..6 (`exe::league_init_table_row`, the row in a clean dword). Source
/// 0x98aba4. Returns 1.
/// Original: `BraNatThird::create_table` (0x42dc90, bra_nat_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_third_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(7);
    let old = read_usize(this.addr() + bra_nat_third::CLUB_LIST);
    if old != 0 {
        exe::crt_free(old);
    }
    let list = exe::crt_malloc(0xe0);
    write_usize(this.addr() + bra_nat_third::CLUB_LIST, list);
    if list == 0 {
        show_error_at(0x98aba4, 0x18f);
        return 1;
    }
    let t = exe::crt_malloc((this.club_count() as i32 as u32).wrapping_mul(0x3b));
    this.set_table(t);
    if t == 0 {
        show_error_at(0x98aba4, 0x196);
        return 0;
    }
    let mut k: u8 = 0;
    let mut i: i32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    while i < read_usize(G_CLUB_COUNT) as i32 {
        if (k as i8) >= 0x38 {
            break;
        }
        if read_usize(club + club::DIVISION) == this.competition().addr() {
            write_usize(read_usize(this.addr() + bra_nat_third::CLUB_LIST) + (k as i8 as isize as usize) * 4, club);
            k = k.wrapping_add(1);
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    covered!("cql_7 serie c draw");
    exe::comp_util_shuffle_pointer_array(read_usize(this.addr() + bra_nat_third::CLUB_LIST), (k as i8 as i32) as u32); // cql7: the random draw of the Serie C clubs
    for j in 0..7usize {
        exe::league_init_table_row(this, j as u32, read_usize(read_usize(this.addr() + bra_nat_third::CLUB_LIST) + j * 4), 0, 0);
    }
    1
}

/// Series C: double round robin (home-and-away groups [inference: the groups of 7 of the 0x42d620 schedule]); win 3,
/// draw 1; tie-break by goal difference, goals scored and away goals; promotes to Series B, no league below; 2 top
/// places leading to the next stage (rule word 8) and 2 bottom ones [inference: no effect, there is no league below]; 7
/// substitutes, 3 substitutions, fixed numbers. Then the schedule
/// Original: `BraNatThird::set_up_rules` (0x42de50, bra_nat_third.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn bra_nat_third_set_up_rules(this: usize) -> u32 {
    set_serie_c_rules(this);
    covered!("cqi_7 serie c rules");
    create_main_schedule(League(this));
    1
}

/// Series C: in the groups, the qualified get the qualification record; in stage 7, those who go through; in the final
/// group (stages 8 and 9), the two promoted get the promotion (weight 0x32) and qualification records and fate 0
/// (promotion) in the first table that has them; in the final (stage 10), the champion gets the champion record
/// Original: `BraNatThird::update_staff_records_for_status` (0x42eb00, bra_nat_third.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn bra_nat_third_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let _ = (a4, a5, a6);
    serie_c_staff_records(League(this), club, status, stage, true).0
}

/// Series C: in the groups (stages up to 6), the group position becomes the position on the 49-place Series C scale
/// [inference]: 1 and 2 → 1 (the highest → 9), 3 → 17, 4 → 25, 5 → 33, 6 → 41, 7 → 49; in later stages, the codes as
/// they come
/// Original: `BraNatThird::set_club_progress` (0x42e100, bra_nat_third.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn bra_nat_third_set_club_progress(this: usize, club: usize, stage: u32, a: u32, b: u32, c: u32) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, club);
    let (fa, a, b, c) = (stage as u8 as i8, a as u8, b as u8, c as u8);
    if fa < 7 {
        let (a, c) = (serie_c_progress_code(a, 1), serie_c_progress_code(c, 9));
        let b2 = serie_c_progress_code(b, 1);
        if b == 3 {
            covered!("cqk_8 serie c second 3");
        }
        write_progress_codes(e, a, b2, c); // cqk8: the Serie C group codes
    } else {
        write_progress_codes(e, a, b, c);
    }
    e
}

/// The schedule of the Brazilian Série C: the first-round groups (stages up to 6) play 14 rounds from 5 Jul to 7 Sep;
/// stage 7 is a one-round knockout on 14 Sep; the third-round groups (stages 8 and 9) play 6 rounds from 28 Sep to
/// 2 Nov; stage 10 is the one-round final on 9 Nov; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures, the group stages and the final (`bra_nat_third_create_final_stage`). For a stage with a
/// table (`serie_c_schedule_stage`) it writes the outputs (is_league when that pointer is not null, the rounds and the
/// extra: stage + 0x420 in the first-round groups, stage + 0x476 in the third-round ones, 0 in the knockouts),
/// allocates the round table (`exe::crt_malloc`, 0x41 bytes per league round, 0x68 per knockout round; out of memory,
/// the error box of bra_nat_third.cpp, lines 0xbb, 0x10d, 0x136 and 0x15d, and 0) and writes the date (day, month,
/// years after the season year `year()`, weekday and code) and the matches of each round, in the original's order
/// (`create_round_table`, the cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table,
/// or 0 without writing anything for a stage above 10.
/// Original: `BraNatThird::create_schedule` (0x42d620, bra_nat_third.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_nat_third_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), serie_c_schedule_stage(stage), rounds, extra, league, 0x98aba4) // cqy1: the Série C schedule
}

/// Comparer for `bra_nat_third_create_schedule`: side by side (`asia_club_champ_compare_round_table`), the outputs and
/// the bytes written to the two tables.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_third_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_NAT_THIRD_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_nat_third_create_schedule", stage, serie_c_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_nat_third_create_schedule(this, stage, r, e, l))
}

/// The Brazilian third division at career creation: the year and the database competition, discipline rule 7
/// (`discipline_rule::BRAZIL_REGIONAL`) and, with no saved state, eleven stages besides the league (the seven
/// first-round groups of seven, the second and third rounds and the final), the rules, the table, the fixtures, the
/// order, the statistics, the seven first-round groups with the 56 clubs, the predicted positions (by reputation) and
/// the list of 56 freed (`bra_nat_third_construct_league` with `BRA_NAT_THIRD_LEAGUE`).
/// Original: `BraNatThird::BraNatThird` (0x42d2c0, bra_nat_third.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bra_nat_third(this: usize, year: u32, record: usize) -> usize {
    bra_nat_third_construct_league(League(this), year, record, &BRA_NAT_THIRD_LEAGUE, discipline_rule::BRAZIL_REGIONAL, "cqu_2 bra_nat_third new") // cqu2: the third division
}

/// Comparer for `construct_bra_nat_third`.
pub(crate) unsafe extern "thiscall" fn compare_construct_bra_nat_third(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_NAT_THIRD);
    bra_nat_third_compare_league_ctor("construct_bra_nat_third", League(this), year, record, &BRA_NAT_THIRD_LEAGUE, discipline_rule::BRAZIL_REGIONAL, || o(this, year, record))
}

/// The stages of the round table of the Brazilian Série C (virtual `comp_vtable::CREATE_SCHEDULE`); the stage is the
/// signed byte of the argument.
///
/// Stages −128 to 6 (the first-round groups, league): 14 rounds from 5 Jul to 7 Sep, extra = stage + 0x420 (the word of
/// the sign-extended byte); stage 7 (knockout): 1 round, draw 8 Sep and match 14 Sep, extra 0; stages 8 and 9 (the
/// third-round groups, league): 6 rounds from 28 Sep to 2 Nov, extra = stage + 0x476; stage 10 (the final): 1 round, draw
/// 3 Nov and match 9 Nov, extra 0; any stage above 10 has no table (None). is_league is written only through a
/// non-null pointer; the rounds and the extra always.
pub(crate) fn serie_c_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    let s = stage as u8 as i8;
    match s {
        -128..=6 => Some(ScheduleStage { league: 1, rounds: 14, extra: (s as i16).wrapping_add(0x420), extra_check: false, size: 0x38e,
            row: 0xbb, ops: &SERIE_C_SCHEDULE_FIRST_ROUND_OPS, name: "cqy_1 serie c first round" }),
        7 => Some(ScheduleStage { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x10d,
            ops: &SERIE_C_SCHEDULE_SECOND_ROUND_OPS, name: "cqy_1 serie c stage 7" }),
        8 | 9 => Some(ScheduleStage { league: 1, rounds: 6, extra: (s as i16).wrapping_add(0x476), extra_check: false, size: 0x186,
            row: 0x136, ops: &SERIE_C_SCHEDULE_THIRD_ROUND_OPS, name: "cqy_1 serie c third round" }),
        10 => Some(ScheduleStage { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x15d,
            ops: &SERIE_C_SCHEDULE_FINAL_OPS, name: "cqy_1 serie c final" }),
        _ => None,
    }
}

/// The constructor (the 544-byte template of the state leagues, with the variations of `BraNatThirdLeagueVariant`).
///
/// League::League (`exe::new_league`, thiscall, ret 0); `league::COMPETITION` = the record; the vtable; in the third
/// division, `bra_nat_third::CLUB_LIST` = 0; the year (`league::YEAR`, word); the discipline rule
/// (`league::DISCIPLINE_RULE`); in MLS, the importance. If the saved state loads (League::load_from_temp_file
/// `exe::league_load_from_temp_file`, thiscall(this, 1), ret 4; ≠ 0), returns this.
///
/// Otherwise: the importance (if only on a new league), `league::STAGE_INDEX` = `stage_index::MAIN` (0xff),
/// `league::CURRENT_STAGE` = −1, `league::STAGE_COUNT` = the stages and their array `league::STAGES` (malloc
/// `exe::crt_malloc`, unchecked; no stages: 0), the inline rules; then, each with its own error box (returning this) if
/// it returns 0: the rules, the table, the fixtures and the order (League::sort_table `exe::league_sort_table`,
/// thiscall(this, 0), ret 4); the statistics (operator new `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats
/// `exe::new_comp_stats`, thiscall(new, this), ret 4) at `league::COMP_STATS` (without them: the error box). In the
/// third division: the seven first-round groups (BraNatThird::create_group_stage
/// `exe::branatthird_create_group_stage`(g), thiscall, ret 4, g from 0 to 6; the original pushes the dword with g in the
/// low byte and, above it, the high bytes of the new's pointer; the routine only uses g: the copy passes g clean), the
/// predicted positions and the list of 56 (`bra_nat_third::CLUB_LIST`) freed (free `exe::crt_free`, if there is one)
/// and zeroed. In the others, the predicted positions.
pub(crate) unsafe fn bra_nat_third_construct_league(this: League, year: u32, record: usize, e: &BraNatThirdLeagueVariant, discipline: u8, branch: &'static str) -> usize {
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.table);
    let fixtures: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.fixtures);
    let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(e.predicted);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), e.vtable);
    if e.serie_c {
        write_usize(this.addr() + bra_nat_third::CLUB_LIST, 0);
    }
    write_u16(this.addr() + league::YEAR, year as u16);
    this.set_discipline_rule(discipline);
    if let Some((v, true)) = e.importance {
        this.set_importance(v);
    }
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    if let Some((v, false)) = e.importance {
        this.set_importance(v);
    }
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    write_u32(this.addr() + league::STAGE_COUNT, e.stages);
    let v = if e.stages != 0 { exe::crt_malloc(e.stages * 4) } else { 0 };
    this.set_stages(v);
    for &(o, n, x) in e.writes {
        match n {
            1 => write_u8(this.addr() + o, x as u8),
            2 => write_u16(this.addr() + o, x as u16),
            _ => write_u32(this.addr() + o, x),
        }
    }
    let mut rows = e.rows.iter();
    if let Some(r) = e.rules {
        let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(r);
        let row = *rows.next().unwrap();
        if rules(this.addr()) == 0 {
            show_error_at(e.file, row);
            return this.addr();
        }
    }
    let (row_table, row_fixtures, row_order, row_stats) = (*rows.next().unwrap(), *rows.next().unwrap(), *rows.next().unwrap(), *rows.next().unwrap());
    if table(this.addr()) == 0 {
        show_error_at(e.file, row_table);
        return this.addr();
    }
    if fixtures(this.addr()) == 0 {
        show_error_at(e.file, row_fixtures);
        return this.addr();
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(e.file, row_order);
        return this.addr();
    }
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(e.file, row_stats);
        return this.addr();
    }
    if e.serie_c {
        for g in 0..7u32 {
            exe::branatthird_create_group_stage(this, g); // cqu2: the seven groups of the third division
        }
        predicted(this.addr());
        let list = read_usize(this.addr() + bra_nat_third::CLUB_LIST);
        if list != 0 {
            exe::crt_free(list);
        }
        write_usize(this.addr() + bra_nat_third::CLUB_LIST, 0);
        return this.addr();
    }
    predicted(this.addr());
    this.addr()
}

/// Comparer of the three constructors: they allocate and draw the fixtures, so only the original runs, and what it
/// writes is checked: the return value, the vtable, the year, the record, the discipline rule and the importance when
/// it is always written (MLS); on a new league (`stage_index::MAIN`, with the statistics), the stages, their array (null
/// without stages), the importance, the inline rules and, in the third division, the seven groups in slots 0 to 6 and
/// the list of 56 zeroed.
pub(crate) unsafe fn bra_nat_third_compare_league_ctor(name: &str, this: League, year: u32, record: usize, e: &BraNatThirdLeagueVariant, discipline: u8, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == e.vtable && built_year_ok(this.year() as u16, year as u16)
        && this.competition().addr() == record && this.discipline_rule() == discipline;
    if let Some((v, true)) = e.importance {
        ok = ok && this.importance() == v;
    }
    if this.stage_index() == stage_index::MAIN && this.comp_stats() != 0 {
        ok = ok && this.stage_count() == e.stages as usize && (this.stages() != 0) == (e.stages != 0);
        if let Some((v, _)) = e.importance {
            ok = ok && this.importance() == v;
        }
        // ponytail: only the points and tie-break rules (PROMOTED_COUNT..=TIEBREAKS + 3, +0xbe..+0xc8); the order
        // (sort_table) rewrites GAMES_PER_CLUB and the others may change with the fixtures
        for &(o, n, x) in e.writes.iter().filter(|w| (league::PROMOTED_COUNT..=league::TIEBREAKS + 3).contains(&w.0)) {
            let value = match n {
                1 => read_u8(this.addr() + o) as u32,
                2 => read_u16(this.addr() + o) as u32,
                _ => read_usize(this.addr() + o) as u32,
            };
            ok = ok && value == x;
        }
        if e.serie_c {
            ok = ok && read_usize(this.addr() + bra_nat_third::CLUB_LIST) == 0 && (0..7).all(|g| read_usize(this.stages() + 4 * g) != 0);
        }
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): returned {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}, +0x51 = {}", name, this.addr(), r, read_usize(this.addr()), this.stage_count(), this.stages(), this.comp_stats(), this.importance()));
    r
}

/// The Brazilian Série C, the first-round groups (stages −128 to 6, league): 14 rounds (5 Jul to 7 Sep), 910 bytes, 28 operations.
pub(crate) const SERIE_C_SCHEDULE_FIRST_ROUND_OPS: [AsiaClubChampScheduleOp; 28] = [
    Cqp11D(0, 5, 7, 0, 6, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0xc, 7, 0, 6, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0xf, 7, 0, 2, 2, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0x13, 7, 0, 6, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 0x1a, 7, 0, 6, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 0x1d, 7, 0, 2, 2, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 2, 8, 0, 6, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 5, 8, 0, 2, 2, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 9, 8, 0, 6, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 0x10, 8, 0, 6, 1, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 0x13, 8, 0, 2, 2, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0x17, 8, 0, 6, 1, 0), Cqp11L(0xb, 0, -1, -1, -1, 0),
    Cqp11D(0xc, 0x1e, 8, 0, 6, 1, 0), Cqp11L(0xc, 0, -1, -1, -1, 0), Cqp11D(0xd, 7, 9, 0, 6, 1, 0),
    Cqp11L(0xd, 0, -1, -1, -1, 0),
];

/// The Brazilian Série C, stage 7 (knockout): 1 round (draw 8 Sep, match 14 Sep), 104 bytes, 19 operations.
pub(crate) const SERIE_C_SCHEDULE_SECOND_ROUND_OPS: [AsiaClubChampScheduleOp; 19] = [
    Cqp11S(0, 8, 9, 0, 0), Cqp11J(0, 0xe, 9, 0, 6, 1, 0), Cqp11W(0x7, 0x14), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 5), Cqp11W(0x18, 0x10), Cqp11W(0x1a, 8), Cqp11W(0x1c, 0x10), Cqp11W(0xf, 1),
    Cqp11W(0x11, 0), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0),
    Cqp11X(0x60, 0), Cqp11X(0x64, 0),
];

/// The Brazilian Série C, the third-round groups (stages 8 and 9, league): 6 rounds (28 Sep to 2 Nov), 390 bytes, 12 operations.
pub(crate) const SERIE_C_SCHEDULE_THIRD_ROUND_OPS: [AsiaClubChampScheduleOp; 12] = [
    Cqp11D(0, 0x1c, 9, 0, 6, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 4, 0xa, 0, 6, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0xb, 0xa, 0, 6, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0x12, 0xa, 0, 6, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 0x19, 0xa, 0, 6, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 2, 0xb, 0, 6, 1, 0), Cqp11L(5, 0, -1, -1, -1, 0),
];

/// The Brazilian Série C, stage 10, the final: 1 round (draw 3 Nov, match 9 Nov), 104 bytes, 19 operations.
pub(crate) const SERIE_C_SCHEDULE_FINAL_OPS: [AsiaClubChampScheduleOp; 19] = [
    Cqp11S(0, 3, 0xb, 0, 0), Cqp11J(0, 9, 0xb, 0, 6, 1, 0), Cqp11W(0x7, 0x96), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0), Cqp11B(0x17, 5), Cqp11W(0x18, 2), Cqp11W(0x1a, 1), Cqp11W(0x1c, 2), Cqp11W(0xf, 1),
    Cqp11W(0x11, 1), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0),
    Cqp11X(0x60, 0), Cqp11X(0x64, 0),
];

pub(crate) const BRA_NAT_THIRD_LEAGUE: BraNatThirdLeagueVariant = BraNatThirdLeagueVariant {
    vtable: 0x967f00,
    importance: None,
    stages: 0xb,
    writes: &[],
    rules: Some(0x42de50),
    table: FN_BRANATTHIRD_CREATE_TABLE,
    fixtures: FN_LEAGUE_CREATE_FIXTURES,
    predicted: 0x42f0a0,
    serie_c: true,
    file: 0x98aba4,
    rows: &[0x4a, 0x51, 0x58, 0x61, 0x68],
};
