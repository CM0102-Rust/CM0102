// ---------- batch 311: the Polish season turnover (pol_first.cpp, pol_second.cpp) ----------

/// Rule: the Polish turnover, led by the First Division: the season reopened; promotion and relegation along the chain;
/// the four best clubs of the lowest Polish division (the best twelve shuffled) up into the Second Division and its
/// bottom clubs down (`pol_first_promote_lower_division_clubs`); the First Division rebuilt for the next year (rules,
/// table, fixtures, order); its second group stage built again; the Second Division turned over; the predicted
/// positions, the board expectations and the officials.
///
/// `top_division_turnover` with, before, virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1) (push 1 ->
/// thiscall, ret 4) and `pol_first_promote_lower_division_clubs` (direct, ecx = this); the table PolFirst::create_table
/// (0x7c9450) and the boxes of pol_first.cpp (0xa78598) at 0x1e7, 0x1ee, 0x1f4, 0x1fb; after the order,
/// PolFirst::create_second_group_stage (0x7c9900, thiscall, ret 0, not ported) and the Second Division's start_new_season
/// (virtual `comp_vtable::START_NEW_SEASON` on the object of `KEY_CLUB_COMP_POLISH_SECOND_DIVISION`, called without a
/// null check). The stages are not destroyed and the current stage is not reset (as the original). After a box: 0.
/// Original: `PolFirst::start_new_season` (0x7c9740, pol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        promotion_relegation_chain(this, 1);
        call_on_league(0x7ca030, this);
    }, 0x7c9450, 0xa78598, [0x1e7, 0x1ee, 0x1f4, 0x1fb], || {
        covered!("mwd_29 polish first division turnover");
        call_on_league(0x7c9900, this);
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_POLISH_SECOND_DIVISION), comp_vtable::START_NEW_SEASON);
    })
}

/// Rule: at the Polish turnover, the four best Polish clubs of the lowest division (`KEY_CLUB_COMP_POLISH_LOWER_DIVISION`,
/// sorted by strength with the best twelve shuffled) go up to the Second Division, and the Second Division's bottom
/// `league::RELEGATED_COUNT` clubs go down to the lowest division; with fewer than 4 candidates nobody moves.
///
/// A list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; out of memory: the box of pol_first.cpp at line 0x2f8 and
/// return) with the clubs whose nation (`club::NATION`) is Poland (`KEY_NATION_POLAND`) and whose division
/// (`club::DIVISION`) is the lowest one, in database order (a 16-bit index); each of them also gets
/// `club::LAST_DIVISION` = its division. With at least 4: `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 4)
/// (cdecl), the first four moved to the Second Division (League::move_club_to_division(this, club, the Second
/// Division's record, 1)); then the rows from `club_count − relegated_count` (a word and a signed byte of the Second
/// Division) up to the last one go down (League::get_table_row, then League::relegate_club_to_division(this, club, the
/// lowest division's record, 1)). The list is freed.
/// Original: `pol_first_promote_lower_division_clubs` (0x7ca030, pol_first.cpp, thiscall: ecx = the First Division, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_first_promote_lower_division_clubs(this: usize) {
    let second = comp_by_global(KEY_CLUB_COMP_POLISH_SECOND_DIVISION);
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0xa78598, 0x2f8);
        return;
    }
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let nation = read_usize(club + club::NATION);
        if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_POLAND) {
            let d = read_usize(club + club::DIVISION);
            if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_POLISH_LOWER_DIVISION) {
                write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
                n = n.wrapping_add(1);
                write_usize(club + club::LAST_DIVISION, read_usize(club + club::DIVISION));
            }
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if (n as u16 as i16) >= 4 {
        covered!("mwd_29 polish lowest division up");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4); // mwd29: the best twelve shuffled
        for k in 0..4 {
            exe::league_move_club_to_division(this, read_usize(list + 4 * k), comp_record_of(KEY_CLUB_COMP_POLISH_SECOND_DIVISION), 1);
        }
        let last = read_i16(second + league::CLUB_COUNT) as i32;
        let mut i = last - read_i8(second + league::RELEGATED_COUNT) as i32;
        while i < last {
            let record = comp_record_of(KEY_CLUB_COMP_POLISH_LOWER_DIVISION);
            let club = read_usize(exe::league_get_table_row(second, i));
            exe::league_relegate_club_to_division(this, club, record, 1); // mwd29: the bottom clubs down
            i += 1;
        }
    }
    exe::crt_free(list);
}

/// Rule: the Polish Second Division turnover (called by the First Division's): the season reopened, last season's
/// stages destroyed and the league rebuilt for the next year (`rebuild_season` with PolSecond::create_table at 0x7cbdf0
/// and the boxes of pol_second.cpp at lines 0x204, 0x20b, 0x211, 0x218; the predicted positions), then the board
/// expectations and the officials (`set_expectations_and_renew_officials`), whose eax is returned. After a box: 0.
/// Original: `PolSecond::start_new_season` (0x7cc000, pol_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x7cbdf0, 0xa7860c, [0x204, 0x20b, 0x211, 0x218], 1) {
        return 0;
    }
    covered!("mwd_29 polish second division turnover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `pol_first_start_new_season`: only the original runs (it draws and rebuilds); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_pol_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POL_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("pol_first_start_new_season", League(this), || o(this))
}

/// Comparer for `pol_first_promote_lower_division_clubs`: only the original runs (it draws); with at least 4
/// candidates, the Second Division's bottom `relegated_count` clubs must end in the lowest division.
pub(crate) unsafe extern "thiscall" fn compare_pol_first_promote_lower_division_clubs(this: usize) {
    let candidates = (0..read_usize(G_CLUB_COUNT)).filter(|&i| {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let (nation, d) = (read_usize(c + club::NATION), read_usize(c + club::DIVISION));
        nation != 0 && read_usize(nation) == read_usize(KEY_NATION_POLAND) && d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_POLISH_LOWER_DIVISION)
    }).count();
    let l = League(comp_by_global(KEY_CLUB_COMP_POLISH_SECOND_DIVISION));
    let mut planned: Vec<usize> = Vec::new();
    if candidates >= 4 && l.table() != 0 {
        let last = l.club_count() as i32;
        for i in (last - read_i8(l.addr() + league::RELEGATED_COUNT) as i32).max(0)..last {
            planned.push(read_usize(l.table() + i as usize * league_table_row::SIZE));
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_POL_FIRST_PROMOTE_LOWER_DIVISION_CLUBS)(this);
    let lower = comp_record_of(KEY_CLUB_COMP_POLISH_LOWER_DIVISION);
    let wrong: Vec<usize> = planned.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != lower).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("pol_first_promote_lower_division_clubs({:#x}): relegated clubs not in the lowest division {:x?}", this, wrong));
}

/// Comparer for `pol_second_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_pol_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_POL_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("pol_second_start_new_season", League(this), || o(this))
}

// ---------- batch 311: the Polish and Greek cup entrants (pol_cup.cpp, pol_lge_cup.cpp, gre_cup.cpp) ----------

/// Rule: the Polish Cup takes 52 clubs: the Second Division clubs placed 13th to 20th last season, then those placed up
/// to 12th (class 2), then lowest-division clubs (the promoted ones first, then the others sorted by strength with the
/// best ones shuffled) until 36 are in (class 3), then the First Division clubs of last season (class 1); returns 1.
///
/// `team_count` = 52; the list (`exe::crt_malloc` of 0x138 into `cup::TEAM_TABLE`; out of memory: the box of
/// pol_cup.cpp at line 0x142 and −1). Over the first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`] clubs (the limit read
/// once for the first two passes): a local list of at most 70, first the clubs whose `club::LAST_DIVISION` has the id of
/// `KEY_CLUB_COMP_POLISH_LOWER_DIVISION` and whose division is set and another one (promoted), then those whose division
/// is that same last division (still there); `exe::comp_util_sort_and_shuffle_top_clubs` on the second part only
/// (list + first, n − first, 36 − first). Then, the limit read again each step: the clubs whose last division has the id of
/// `KEY_CLUB_COMP_POLISH_SECOND_DIVISION` and whose `club::LAST_POSITION` (a signed byte) is 13..20, then ≤ 12, each at
/// the next entry with class 2. While fewer than 36 are in, the local list's clubs (up to its count) at the next entries
/// with class 3. Then the clubs whose last division has the id of `KEY_CLUB_COMP_POLISH_FIRST_DIVISION`, while the
/// entry offset is below 0x138, with class 1 (the count is not kept). Status 0 for all. Returns 1.
/// Original: `PolCup::collect_entrants` (0x7c8550, pol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let mut lower = [0usize; 0x46];
    let mut n: i16 = 0;
    let mut k = 0i32;
    this.set_team_count(0x34);
    let l = exe::crt_malloc(0x138);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0xa78564, 0x142);
        return -1;
    }
    let limit = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let count = limit();
    let mut first = 0i32;
    for promoted in [true, false] {
        if !promoted {
            first = n as i32;
        }
        for i in 0..count.max(0) as usize {
            let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
            let (last, d) = (read_usize(c + club::LAST_DIVISION), read_usize(c + club::DIVISION));
            let fits = if promoted { d != 0 && d != last } else { d == last };
            if last != 0 && n < 0x46 && read_usize(last) == read_usize(KEY_CLUB_COMP_POLISH_LOWER_DIVISION) && fits {
                lower[n as usize] = c;
                n += 1;
            }
        }
    }
    covered!("mwd_30 polish cup entrants");
    exe::comp_util_sort_and_shuffle_top_clubs(lower.as_mut_ptr() as usize + 4 * first as usize, (n as i32 - first) as u32, (0x24 - first) as u32); // mwd30: the lowest division clubs shuffled
    for upper in [false, true] {
        let mut i = 0i32;
        while i < limit() {
            let c = read_usize(G_CLUB_TABLE) + i as usize * club::SIZE;
            let last = read_usize(c + club::LAST_DIVISION);
            if last != 0 && read_usize(last) == read_usize(KEY_CLUB_COMP_POLISH_SECOND_DIVISION) {
                let pos = read_i8(c + club::LAST_POSITION);
                if if upper { pos <= 0xc } else { (0xd..0x15).contains(&pos) } {
                    set_cup_entry(this, k as usize, c, 2);
                    k += 1;
                }
            }
            i += 1;
        }
    }
    if k < 0x24 {
        let mut j = 0i32;
        while j < n as i32 {
            set_cup_entry(this, k as usize, lower[j as usize], 3); // mwd30: the lowest division clubs class 3
            k += 1;
            j += 1;
            if k * 6 >= 0xd8 {
                break;
            }
        }
    }
    let mut e = k * 6;
    let mut i = 0i32;
    while i < limit() {
        let c = read_usize(G_CLUB_TABLE) + i as usize * club::SIZE;
        let last = read_usize(c + club::LAST_DIVISION);
        if last != 0 && e < 0x138 && read_usize(last) == read_usize(KEY_CLUB_COMP_POLISH_FIRST_DIVISION) {
            set_cup_entry(this, (e / 6) as usize, c, 1);
            e += 6;
        }
        i += 1;
    }
    1
}

/// Rule: the Polish League Cup takes 36 clubs in 2001 and 34 afterwards, all class 0: the Second Division clubs that
/// came up from below (or have no last division), then the other Second Division clubs, then the First Division clubs
/// placed below 7th last season, then those placed up to 7th, each in database order until the list is full; returns
/// 1 when it is full.
///
/// `team_count` = 36 when the cup's year (+0x40) is 2001, else 34; the list (`exe::crt_malloc` of 6 × team_count; out
/// of memory: the box of pol_lge_cup.cpp at line 0x168 and −1). Four passes over the clubs (a 16-bit index up to
/// [`G_CLUB_COUNT`]), each adding at the next entry while the count is below `team_count`: division id of
/// `KEY_CLUB_COMP_POLISH_SECOND_DIVISION` with no `club::LAST_DIVISION` or one whose id is neither the First nor the
/// Second Division's; the same division with a last division that is one of them; division id of
/// `KEY_CLUB_COMP_POLISH_FIRST_DIVISION` with `club::LAST_POSITION` (a signed byte) above 7; and at most 7. Class and
/// status 0. Returns count == `team_count`.
/// Original: `PolLgeCup::collect_entrants` (0x7cad30, pol_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_lge_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let mut k = 0i32;
    this.set_team_count(if read_i16(this.addr() + 0x40) == 0x7d1 { 0x24 } else { 0x22 });
    let l = exe::crt_malloc((this.team_count() as i32 * 6) as u32);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0xa785d4, 0x168);
        return -1;
    }
    covered!("mwd_30 polish league cup entrants");
    for pass in 0..4 {
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i = 0i16;
        while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
            let d = read_usize(club + club::DIVISION);
            let fits = if pass < 2 {
                d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_POLISH_SECOND_DIVISION) && k < this.team_count() as i32 && {
                    let last = read_usize(club + club::LAST_DIVISION);
                    let upper = last != 0 && (read_usize(last) == read_usize(KEY_CLUB_COMP_POLISH_FIRST_DIVISION) || read_usize(last) == read_usize(KEY_CLUB_COMP_POLISH_SECOND_DIVISION));
                    if pass == 0 { !upper } else { upper }
                }
            } else {
                d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_POLISH_FIRST_DIVISION) && k < this.team_count() as i32 && {
                    let pos = read_i8(club + club::LAST_POSITION);
                    if pass == 2 { pos > 7 } else { pos < 8 } // mwd30: the First Division, the lower half first
                }
            };
            if fits {
                set_cup_entry(this, k as usize, club, 0);
                k += 1;
            }
            i = i.wrapping_add(1);
            club = club.wrapping_add(club::SIZE);
        }
    }
    (k == this.team_count() as i32) as i32
}

/// Rule: the Greek Cup takes 44 clubs: 16 Third Division clubs (sorted by strength, the best 48 shuffled), then the
/// Second Division clubs and the First Division clubs in database order until the list is full; short of 44, an error
/// box. Returns 1.
///
/// The entrant list (+0xf2 = 44; +0xee = `exe::crt_calloc`(4, 44), cdecl; out of memory: the box of gre_cup.cpp at line
/// 0x2f2 and 0). Over the clubs (a 16-bit index up to [`G_CLUB_COUNT`] read once): the division id of
/// `KEY_CLUB_COMP_GREEK_LOWER_DIVISION` into a local list of at most 100 (a signed byte count);
/// `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 16) and its first 16 into the entrant list (the original copies
/// 16 whatever the count: with fewer it reads stack garbage; the copy reads zeros). Then the clubs of
/// `KEY_CLUB_COMP_GREEK_SECOND_DIVISION` and of `KEY_CLUB_COMP_GREEK_PREMIER_DIVISION`, each while the count is below
/// +0xf2. Count ≠ +0xf2: the box at line 0x322. Returns 1.
/// Original: `GreCup::collect_entrants` (0x5e9a80, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_collect_entrants(this: usize) -> i32 {
    let mut third = [0usize; 100];
    let mut n: i8 = 0;
    write_u32(this + 0xf2, 0x2c);
    let list = exe::crt_calloc(4, 0x2c);
    write_usize(this + 0xee, list);
    if list == 0 {
        show_error_at(0x9cdab0, 0x2f2);
        return 0;
    }
    let count = read_usize(G_CLUB_COUNT) as i32;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < count {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GREEK_LOWER_DIVISION) && n < 0x64 {
            third[n as usize] = club;
            n += 1;
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    covered!("mwd_30 greek cup entrants");
    exe::comp_util_sort_and_shuffle_top_clubs(third.as_mut_ptr() as usize, n as i16 as u16 as u32, 0x10); // mwd30: the best 48 third division clubs shuffled
    for k in 0..0x10usize {
        write_usize(read_usize(this + 0xee) + 4 * k, third[k]);
    }
    let mut k = 0x10i32;
    for g in [KEY_CLUB_COMP_GREEK_SECOND_DIVISION, KEY_CLUB_COMP_GREEK_PREMIER_DIVISION] {
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i = 0i16;
        while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
            let d = read_usize(club + club::DIVISION);
            if d != 0 && read_usize(d) == read_usize(g) && k < read_i32(this + 0xf2) {
                k += 1;
                write_usize(read_usize(this + 0xee) + 4 * (k - 1) as usize, club);
            }
            i = i.wrapping_add(1);
            club = club.wrapping_add(club::SIZE);
        }
    }
    if k != read_i32(this + 0xf2) {
        show_error_at(0x9cdab0, 0x322);
    }
    1
}

/// Comparer for `pol_cup_collect_entrants` (side by side with the random generator restored, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_pol_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_POL_CUP_COLLECT_ENTRANTS);
    compare_entrants_with_seeds("pol_cup_collect_entrants", Cup(this), || o(this), || pol_cup_collect_entrants(this), |_| true)
}

/// Comparer for `pol_lge_cup_collect_entrants` (side by side, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_pol_lge_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_POL_LGE_CUP_COLLECT_ENTRANTS);
    compare_entrants_with_seeds("pol_lge_cup_collect_entrants", Cup(this), || o(this), || pol_lge_cup_collect_entrants(this), |_| true)
}

/// Comparer for `gre_cup_collect_entrants`: side by side with the random generator restored; +0xf2, the return and the
/// 44 entrant pointers compared; the copy's list is freed and the original's kept.
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_GRE_CUP_COLLECT_ENTRANTS);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this + 0xf2, 4usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4), crate::rng::STATE_REGIONS[0], crate::rng::STATE_REGIONS[1]];
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(read_usize(this + 0xee));
        r
    }, || {
        let r = gre_cup_collect_entrants(this);
        lr.set(read_usize(this + 0xee));
        write_usize(this + 0xee, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let list = if lo != 0 && lr != 0 { (0..0x2c).all(|k| read_usize(lo + 4 * k) == read_usize(lr + 4 * k)) } else { lo == 0 && lr == 0 };
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(same && a == b && list, || format!("gre_cup_collect_entrants({:#x}) returned {} x {} memory {} list {}", this, a, b, same, list));
    a
}

// ---------- batch 311: the Turkish second division category B turnover and stages, and the Polish First Division stages (tur_second_b.cpp, pol_first.cpp) ----------

/// Rule: the Turkish second division category B turnover (called by the First Division's): the season reopened, last
/// season's stages destroyed, the next year; the rules, the table, the order and the fixtures (in this order); the
/// four group stages built again; the predicted positions and the officials (no board expectations).
///
/// `league::SEASON_CLOSED` = 0; League::free_table (`exe::league_free_table`(0)); the round dates freed and cleared;
/// with statistics, `exe::comp_stats_archive_season_and_reset`(1); with a current stage ≥ 0, the stages 0..=current
/// (re-read on each step) destroyed by their virtual 0 with 1; the year + 1 and the current stage −1. Then, each with
/// the box of tur_second_b.cpp (0xaa18b8) at its line and 0 returned when it returns 0: the rules (virtual
/// `comp_vtable::SET_UP_RULES`, 0x233), TurSecondB::create_table (0x9002c0, thiscall, ret 0; 0x23a), League::sort_table
/// (`exe::league_sort_table`(0), 0x241) and League::create_fixtures (0x247). Then TurSecondB::create_group_stage(i)
/// (0x900780, push i -> thiscall, ret 4) for i = 0..3, the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`) and OfficialsManager::renew_officials on `G_OFFICIALS_MANAGER` with the
/// record, whose eax is returned.
/// Original: `TurSecondB::start_new_season` (0x900500, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_start_new_season(this: usize) -> u32 {
    let this = League(this);
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
    this.set_year(this.year().wrapping_add(1));
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    let file = 0xaa18b8;
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, 0x233);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x9002c0usize);
    if table(this.addr()) == 0 {
        show_error_at(file, 0x23a);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, 0x241);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, 0x247);
        return 0;
    }
    covered!("mwd_31 turkish category b turnover");
    let group: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x900780usize);
    for i in 0..4 {
        group(this.addr(), i);
    }
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr()) as u32
}

/// Rule: the next stage of the Turkish second division category B: past the last stage, the error box; otherwise the
/// current stage goes up by one; on reaching stage 4 the promotion group and the classification groups are built, on
/// reaching stage 10 the play-off.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box of tur_second_b.cpp at line
/// 0x265 and nothing else. Otherwise the current stage + 1; at 4, TurSecondB::create_promotion_group (0x900ab0) and
/// TurSecondB::create_classification_groups (0x900d00); at 10, TurSecondB::create_playoff_stage (0x901040) (all
/// thiscall, ret 0, not ported).
/// Original: `TurSecondB::advance_stage` (0x9006c0, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0xaa18b8, 0x265);
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    covered!("mwd_31 turkish category b next stage");
    if stage == 4 { // mwd31: the promotion and classification groups after the four groups
        call_on_league(0x900ab0, l);
        call_on_league(0x900d00, l);
    } else if stage == 10 {
        call_on_league(0x901040, l);
    }
}

/// Rule: the next stage of the Polish First Division: past the last stage, the error box; otherwise the current stage
/// goes up by one; leaving stage 0 (reaching 1) the final stage is built.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box of pol_first.cpp at line
/// 0x24f and nothing else. Otherwise the current stage + 1; when the old one was 0, PolFirst::create_final_stage
/// (0x7c9b60, thiscall, ret 0, not ported).
/// Original: `PolFirst::advance_stage` (0x7c9ad0, pol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_first_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0xa78598, 0x24f);
    }
    l.set_current_stage((stage + 1) as usize);
    covered!("mwd_31 polish first division next stage");
    if stage == 0 {
        call_on_league(0x7c9b60, l);
    }
}

/// Side by side on the current stage of a league's advance_stage when it builds nothing (`builds`(new stage) false and
/// not at the last stage); otherwise only the original (a stage is built once).
pub(crate) unsafe fn compare_advance_stage_building(name: &str, this: League, builds: impl Fn(i32) -> bool, original: impl FnOnce(), copy: impl FnOnce()) {
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) || builds(stage + 1) {
        return original();
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(this.addr() + league::CURRENT_STAGE, 4)], original, copy);
    SCOREBOARD.record(same, || format!("{}({:#x}) original={:x?} rust={:x?}", name, this.addr(), mo, mr));
}

/// Comparer for `tur_second_b_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_TUR_SECOND_B_START_NEW_SEASON);
    compare_state_league_new_season("tur_second_b_start_new_season", League(this), || o(this))
}

/// Comparer for `tur_second_b_advance_stage` (`compare_advance_stage_building`: stages 4 and 10 build).
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TUR_SECOND_B_ADVANCE_STAGE);
    compare_advance_stage_building("tur_second_b_advance_stage", League(this), |s| s == 4 || s == 10, || o(this), || tur_second_b_advance_stage(this))
}

/// Comparer for `pol_first_advance_stage` (`compare_advance_stage_building`: stage 1 builds).
pub(crate) unsafe extern "thiscall" fn compare_pol_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_POL_FIRST_ADVANCE_STAGE);
    compare_advance_stage_building("pol_first_advance_stage", League(this), |s| s == 1, || o(this), || pol_first_advance_stage(this))
}
