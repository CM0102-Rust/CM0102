// ---------- batch %%LEVA%%: the Belgian, Danish and German cup entrants (bel_fa_cup.cpp, den_cup.cpp, ger_cup.cpp) ----------

/// Rule: the Belgian Cup takes 112 clubs: 44 Belgian clubs of the four Fourth Division groups (sorted by strength, the
/// best 132 shuffled) with class 3, then the Third Division clubs (the national one and groups A and B) with class 2,
/// the Second Division's with class 1 and the First Division's with class 0, each group in database order until the
/// list is full; returns 1 when it is full.
///
/// `team_count` = 112; the list (`exe::crt_malloc` of 0x2a0 into `cup::TEAM_TABLE`; out of memory: the box of
/// bel_fa_cup.cpp at line 0x161 and −1). Over the clubs (a 16-bit index up to [`G_CLUB_COUNT`]) whose division
/// (`club::DIVISION`) has the id of `KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_A` to `_D` and whose nation (`club::NATION`)
/// is Belgium's record (`KEY_NATION_BELGIUM`): a local list of at most 150. `exe::comp_util_sort_and_shuffle_top_clubs`
/// (list, n, 44) and its first 44 into the entries 0..43 (the original copies 44 whatever the count: with fewer it reads
/// stack garbage; the copy reads zeros). Then three passes over the clubs, each adding at the next entry and stopping
/// when the count reaches `team_count` (checked only after adding): the division id of `KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION`,
/// `_A` or `_B` (class 2), `KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION` (1) and `KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION` (0).
/// Returns count == `team_count`.
/// Original: `BelFaCup::collect_entrants` (0x41d8d0, bel_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_fa_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let mut lower = [0usize; 150];
    let mut n: i16 = 0;
    this.set_team_count(0x70);
    let l = exe::crt_malloc(0x2a0);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0x989ae0, 0x161);
        return -1;
    }
    let fourth = [KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_A, KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_B, KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_C, KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_D];
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && fourth.iter().any(|&g| read_usize(d) == read_usize(g)) && read_usize(club + club::NATION) == oceania_nations_nation_record(KEY_NATION_BELGIUM) && n < 0x96 {
            lower[n as usize] = club;
            n += 1;
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    covered!("mwc_22 belgian cup entrants");
    exe::comp_util_sort_and_shuffle_top_clubs(lower.as_mut_ptr() as usize, n as u32, 0x2c); // mwc22: the best 132 shuffled
    for k in 0..0x2c {
        set_cup_entry(this, k, lower[k], 3);
    }
    let mut k = 0x2ci32;
    let passes: [(&[usize], u8); 3] = [(&[KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION, KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION_A, KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION_B], 2),
        (&[KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION], 1), (&[KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION], 0)];
    for (keys, class) in passes {
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i = 0i16;
        while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
            let d = read_usize(club + club::DIVISION);
            if d != 0 && keys.iter().any(|&g| read_usize(d) == read_usize(g)) {
                set_cup_entry(this, k as usize, club, class);
                k += 1;
                if k == this.team_count() as i32 {
                    break;
                }
            }
            i = i.wrapping_add(1);
            club = club.wrapping_add(club::SIZE);
        }
    }
    (k == this.team_count() as i32) as i32
}

/// Rule: the Danish Cup takes 92 clubs, all class 0: up to 48 Danish clubs of the lowest division, then the Danish
/// Second Division clubs, the First Division's and the Superliga's, each in database order until the list is full;
/// returns 1 when it is full.
///
/// `team_count` = 92; the list (`exe::crt_malloc` of 0x228; out of memory: the box of den_cup.cpp at line 0x15a and −1).
/// Four passes over the clubs (a 16-bit index up to [`G_CLUB_COUNT`]), each adding at the next entry with class 0: the
/// division id of `KEY_CLUB_COMP_A_LOWER_DIVISION` with Denmark's nation record (`KEY_NATION_DENMARK`) while fewer than
/// 48 are in; `KEY_CLUB_COMP_DANISH_SECOND_DIVISION` with Denmark's nation record while the count is below
/// `team_count` (the pass does not stop); `KEY_CLUB_COMP_DANISH_FIRST_DIVISION` and
/// `KEY_CLUB_COMP_DANISH_PREMIER_DIVISION` (no nation check) while below `team_count`, stopping when it is reached.
/// Returns count == `team_count`.
/// Original: `DenCup::collect_entrants` (0x551240, den_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    this.set_team_count(0x5c);
    let l = exe::crt_malloc(0x228);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0x9b8dfc, 0x15a);
        return -1;
    }
    let mut k = 0i32;
    let denmark = || oceania_nations_nation_record(KEY_NATION_DENMARK);
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION) && read_usize(club + club::NATION) == denmark() && k * 6 < 0x120 { // mwc22: up to 48 from the lowest division
            set_cup_entry(this, k as usize, club, 0);
            k += 1;
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    covered!("mwc_22 danish cup entrants");
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_DANISH_SECOND_DIVISION) && read_usize(club + club::NATION) == denmark() && k < this.team_count() as i32 {
            set_cup_entry(this, k as usize, club, 0);
            k += 1;
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    for g in [KEY_CLUB_COMP_DANISH_FIRST_DIVISION, KEY_CLUB_COMP_DANISH_PREMIER_DIVISION] {
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i = 0i16;
        while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
            let d = read_usize(club + club::DIVISION);
            if d != 0 && read_usize(d) == read_usize(g) && k < this.team_count() as i32 {
                set_cup_entry(this, k as usize, club, 0);
                k += 1;
                if k == this.team_count() as i32 {
                    break;
                }
            }
            i = i.wrapping_add(1);
            club = club.wrapping_add(club::SIZE);
        }
    }
    (k == this.team_count() as i32) as i32
}

/// Rule: the German Cup takes 64 clubs: up to 4 amateur clubs relegated from the 2. Bundesliga last season (class 2),
/// then the other amateur clubs (sorted by strength, the best 72 shuffled) until 28 are in (class 2), then the 2.
/// Bundesliga clubs (class 2 for those promoted from below, 1 for the others) and the Bundesliga's (class 1) until the
/// list is full; returns 1 when it is full. The amateur clubs: those of the Regionalliga division with a reserve
/// division, and the German clubs of the lowest division; B teams left out.
///
/// `team_count` = 64; the list (`exe::crt_malloc` of 0x180; out of memory: the box of ger_cup.cpp at line 0x13c and
/// −1); a local list of 700 pointers (`exe::crt_malloc` of 0xaf0; out of memory: the box at line 0x144 and 0, the team
/// list left allocated). Over the first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`] clubs (a 16-bit index, the limit read
/// again each step) with a division: an amateur club is one with `club::RESERVE_DIVISION` set and the division id of
/// `KEY_CLUB_COMP_GERMAN_REGIONAL`, or else with a nation whose id is Germany's (`KEY_NATION_GERMANY`) and the division
/// id of `KEY_CLUB_COMP_A_LOWER_DIVISION`; B teams (`exe::club_get_linked_b_team`(club, &flag, 1), cdecl: a partner
/// with the flag 0; one flag shared by every call) are left out. An amateur club whose `club::LAST_DIVISION` has the id
/// of `KEY_CLUB_COMP_GERMAN_SECOND_DIVISION` goes straight in (class 2) while fewer than 4 are in; the others go to the
/// local list (at most 700). `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 24); then, while the count is below
/// 28, the list's clubs in order (class 2), each slot zeroed after it is copied; and a second pass over the list from
/// its start while still below 28 (it copies the zeroed slots: with too few amateur clubs the original fills the rest
/// with null clubs; a defect of the original, kept). The local list is freed. Then a pass over the clubs (16-bit index
/// up to `G_CLUB_COUNT`) for the 2. Bundesliga (division id of `KEY_CLUB_COMP_GERMAN_SECOND_DIVISION`) while below
/// `team_count`: class 2 when the last division is set and is neither the 2. Bundesliga nor the Bundesliga, else 1;
/// then the Bundesliga (`KEY_CLUB_COMP_GERMAN_FIRST_DIVISION`, class 1); both stop when the count reaches
/// `team_count`. Returns count == `team_count`.
/// Original: `GerCup::collect_entrants` (0x5d9370, ger_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    this.set_team_count(0x40);
    let l = exe::crt_malloc(0x180);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0x9cd3e0, 0x13c);
        return -1;
    }
    let list = exe::crt_malloc(0xaf0);
    if list == 0 {
        show_error_at(0x9cd3e0, 0x144);
        return 0;
    }
    let mut k = 0i32;
    let mut n: u32 = 0;
    let mut flag: u32 = 0;
    let limit = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < limit() {
        let d = read_usize(club + club::DIVISION);
        if d != 0 {
            let amateur = (read_usize(club + club::RESERVE_DIVISION) != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GERMAN_REGIONAL)) || {
                let nation = read_usize(club + club::NATION);
                nation != 0 && read_usize(nation) == read_usize(KEY_NATION_GERMANY) && read_usize(d) == read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION)
            };
            if amateur && (exe::club_get_linked_b_team(club, &mut flag, 1) == 0 || flag != 0) {
                let last = read_usize(club + club::LAST_DIVISION);
                if last != 0 && read_usize(last) == read_usize(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION) && k * 6 < 0x18 { // mwc22: up to 4 relegated amateur clubs
                    set_cup_entry(this, k as usize, club, 2);
                    k += 1;
                } else if (n as u16 as i16) < 0x2bc {
                    write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
                    n = n.wrapping_add(1);
                }
            }
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    covered!("mwc_22 german cup entrants");
    exe::comp_util_sort_and_shuffle_top_clubs(list, n, 0x18); // mwc22: the best 72 amateur clubs shuffled
    let count = n as u16 as i16;
    if count > 0 {
        let mut j = 0i16;
        while k * 6 < 0xa8 {
            let slot = list + 4 * j as usize;
            set_cup_entry(this, k as usize, read_usize(slot), 2);
            write_usize(slot, 0);
            k += 1;
            j += 1;
            if j >= count {
                break;
            }
        }
    }
    if count > 0 {
        let mut j = 0i16;
        while k * 6 < 0xa8 {
            set_cup_entry(this, k as usize, read_usize(list + 4 * j as usize), 2);
            k += 1;
            j += 1;
            if j >= count {
                break;
            }
        }
    }
    exe::crt_free(list);
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION) && k < this.team_count() as i32 {
            let last = read_usize(club + club::LAST_DIVISION);
            let up = last != 0 && read_usize(last) != read_usize(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION) && read_usize(last) != read_usize(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION);
            set_cup_entry(this, k as usize, club, if up { 2 } else { 1 });
            k += 1;
            if k == this.team_count() as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION) && k < this.team_count() as i32 {
            set_cup_entry(this, k as usize, club, 1);
            k += 1;
            if k == this.team_count() as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    (k == this.team_count() as i32) as i32
}

/// Comparer for `bel_fa_cup_collect_entrants` (side by side with the random generator restored, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_bel_fa_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    compare_entrants_with_seeds("bel_fa_cup_collect_entrants", Cup(this), || o(this), || bel_fa_cup_collect_entrants(this), |_| true)
}

/// Comparer for `den_cup_collect_entrants` (side by side, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_den_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    compare_entrants_with_seeds("den_cup_collect_entrants", Cup(this), || o(this), || den_cup_collect_entrants(this), |_| true)
}

/// Comparer for `ger_cup_collect_entrants` (side by side with the random generator restored, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_ger_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%);
    compare_entrants_with_seeds("ger_cup_collect_entrants", Cup(this), || o(this), || ger_cup_collect_entrants(this), |_| true)
}
