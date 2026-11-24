// ---------- cx09_11: the Belgian Third Division promotion and relegation play-offs (bel_third.cpp) ----------

/// The Cup_Stage constructor (0x522e00, thiscall, ret 0x3c): the parent league, the club count, the club list, the
/// rounds, the competition, the round table, the year, the stage, 1, the extra, 0x14, three words and the seed list.
type BelThirdCupStageCtor = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize;

/// Tops up a group's play-off places to three: counting the rows of `group`'s table with `league_table_row::STATUS` 1,
/// then while fewer than three, each row still undecided (0xff), in table order, gets status 1, the announcement
/// (virtual `comp_vtable::ANNOUNCE_TEAM_STATUS` of this league with (club, 1, `league::STAGE_INDEX`, 0, 0)) and its
/// progress (virtual `comp_vtable::SET_CLUB_PROGRESS` with (club, `league::STAGE_INDEX`, 2, 2, 7)).
unsafe fn bel_third_top_up_playoff_clubs(this: League, group: League) {
    let row_at = |k: i32| LeagueTableRow(group.table() + k as usize * league_table_row::SIZE);
    let mut n = (0..group.club_count() as i32).filter(|&k| row_at(k).status() == 1).count();
    let announce: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::ANNOUNCE_TEAM_STATUS));
    let progress: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_CLUB_PROGRESS));
    let mut k: i16 = 0;
    while k < group.club_count() && n < 3 {
        let row = row_at(k as i32);
        if row.status() == 0xff {
            covered!("cx09_11 belgian third division play-off place");
            row.set_status(1);
            n += 1;
            announce(this.addr(), row.club() as usize, 1, this.stage_index() as u32, 0, 0);
            progress(this.addr(), row.club() as usize, this.stage_index() as u32, 2, 2, 7);
        }
        k += 1;
    }
}

/// Builds the Belgian Third Division promotion play-off: each group topped up to three play-off clubs
/// (`bel_third_top_up_playoff_clubs`, group A then group B); a list of seven: group A's first three clubs with status
/// 1 (table order) in slots 0, 2 and 4, group B's in slots 5, 3 and 1, and in slot 6 the club in 16th place of the
/// division above (the object of `league::UPPER_DIVISION_ID`; its table row 15); the round table through virtual
/// `comp_vtable::CREATE_SCHEDULE` (8, &rounds, &extra, 0) and a Cup_Stage (`exe::crt_operator_new`(0xb2); 0x522e00:
/// this league, 7 clubs, the list, the rounds, the competition, the table, the year, stage 8, 1, the extra, 0x14, 1, 0,
/// 0, no seeds) into stage slot 8. Without a stage: the bel_third.cpp box of line 0x611 (the table is not freed). With
/// one: Cup::set_up_round (`exe::cup_set_up_round`(stage, 0)), the table freed and `league::CURRENT_STAGE` = 8. A
/// group with fewer than three marked clubs leaves its slots as the original's stack had them (here 0).
/// Original: `BelThird::create_promotion_playoff_stage` (0x423880, bel_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_third_create_promotion_playoff_stage(this: usize) {
    let l = League(this);
    let b = League(read_usize(l.stages()));
    bel_third_top_up_playoff_clubs(l, l);
    bel_third_top_up_playoff_clubs(l, b);
    let mut list = [0usize; 7];
    let mut s: i16 = 0;
    let mut k: i16 = 0;
    while k < l.club_count() && s < 6 {
        let row = LeagueTableRow(l.table() + k as usize * league_table_row::SIZE);
        if row.status() == 1 {
            list[s as usize] = row.club() as usize;
            s += 2;
        }
        k += 1;
    }
    let (mut s, mut k): (i16, i16) = (5, 0);
    while k < b.club_count() && s > 0 {
        let row = LeagueTableRow(b.table() + k as usize * league_table_row::SIZE);
        if row.status() == 1 {
            list[s as usize] = row.club() as usize;
            s -= 2;
        }
        k += 1;
    }
    let upper = League(read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * l.upper_division_id() as usize));
    list[6] = LeagueTableRow(upper.table() + 15 * league_table_row::SIZE).club() as usize; // the 16th of the division above
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = schedule(this, 8, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        let construct: BelThirdCupStageCtor = core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, 7, list.as_ptr() as usize, rounds as u16 as u32, l.competition().addr(), table, l.year() as u16 as u32, 8, 1,
                  extra as u16 as u32, 0x14, 1, 0, 0, 0)
    } else {
        0
    };
    write_usize(l.stages() + 4 * 8, stage);
    if stage == 0 {
        return show_error_at(BEL_THIRD_SOURCE, 0x611);
    }
    covered!("cx09_11 belgian third division promotion play-off");
    exe::cup_set_up_round(stage, 0);
    exe::crt_free(table);
    l.set_current_stage(8);
}

/// The Belgian Fourth Division keys (A to D).
const BEL_FOURTH_DIVISIONS: [usize; 4] = [KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_A, KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_B,
    KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_C, KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_D];
/// The Belgian Third Division's four stored club ids: the Fourth Division clubs promoted directly.
const BEL_THIRD_PROMOTED_FOURTH: usize = 0xee;

/// Builds the Belgian Third Division relegation play-off: the clubs of the Fourth Divisions A to D (club index 0 to
/// `G_CLUB_COUNT` − 2 × `G_NATION_COUNT`, a signed word; with a nation and a `club::DIVISION` whose id is one of the
/// four keys) into a list of up to 100 (stops at 100); fewer than 16 (or no club to scan): the bel_third.cpp box of line
/// 0x64a and nothing else. The list ordered by `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 16) (cdecl); the
/// ids of the first four go to the four stored clubs (+0xee, +0xf2, +0xf6, +0xfa: promoted directly); the next twelve
/// and the last clubs with status 2 of group A and of group B (both tables scanned over this league's club count) make
/// the fourteen; either group without one: the box of line 0x66a. The round table through virtual
/// `comp_vtable::CREATE_SCHEDULE` (9, &rounds, &extra, 0) and a Cup_Stage (0x522e00: this league, 14 clubs, the list,
/// the rounds, the competition, the table, the year, stage 9, 1, the extra, 0x14, 0, 0, 0, the seeds: 2 for the twelve
/// Fourth Division clubs and 1 for the two Third Division clubs) into stage slot 9. Without a stage: the box of line
/// 0x678 (the table is not freed). With one: Cup::set_up_round(0), the table freed and `league::CURRENT_STAGE` = 9.
/// Original: `BelThird::create_relegation_playoff_stage` (0x423ba0, bel_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_third_create_relegation_playoff_stage(this: usize) {
    let l = League(this);
    let b = League(read_usize(l.stages()));
    let mut pool = [0usize; 100];
    let mut n: i16 = 0;
    let count = read_i32(G_CLUB_COUNT) - 2 * read_i32(G_NATION_COUNT);
    let mut i: i16 = 0;
    while (i as i32) < count {
        let club = club_at(i as isize);
        let division = club.division().addr();
        if read_u32(club.addr() + club::NATION) != 0 && division != 0 && BEL_FOURTH_DIVISIONS.iter().any(|&k| read_u32(division) == read_u32(k)) {
            pool[n as usize] = club.addr();
            n += 1;
            if n == 100 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    if n < 16 {
        return show_error_at(BEL_THIRD_SOURCE, 0x64a);
    }
    exe::comp_util_sort_and_shuffle_top_clubs(pool.as_mut_ptr() as usize, n as u32, 0x10);
    for k in 0..4 {
        write_u32(this + BEL_THIRD_PROMOTED_FOURTH + 4 * k, read_u32(pool[k]));
    }
    let mut list = [0usize; 14];
    list[..12].copy_from_slice(&pool[4..16]);
    let mut k: i32 = 0;
    while k < l.club_count() as i32 {
        let ra = LeagueTableRow(l.table() + k as usize * league_table_row::SIZE);
        if ra.status() == 2 {
            list[12] = ra.club() as usize;
        }
        let rb = LeagueTableRow(b.table() + k as usize * league_table_row::SIZE);
        if rb.status() == 2 {
            list[13] = rb.club() as usize;
        }
        k += 1;
    }
    if list[12] == 0 || list[13] == 0 {
        return show_error_at(BEL_THIRD_SOURCE, 0x66a);
    }
    let seeds: [u8; 14] = [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1];
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = schedule(this, 9, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        let construct: BelThirdCupStageCtor = core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, 0xe, list.as_ptr() as usize, rounds as u16 as u32, l.competition().addr(), table, l.year() as u16 as u32, 9, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, seeds.as_ptr() as usize)
    } else {
        0
    };
    write_usize(l.stages() + 4 * 9, stage);
    if stage == 0 {
        return show_error_at(BEL_THIRD_SOURCE, 0x678);
    }
    covered!("cx09_11 belgian third division relegation play-off");
    exe::cup_set_up_round(stage, 0);
    exe::crt_free(table);
    l.set_current_stage(9);
}

/// Comparer for `bel_third_create_promotion_playoff_stage`: only the original (it builds the stage); afterwards stage 8
/// must exist and be the current one.
pub(crate) unsafe extern "thiscall" fn compare_bel_third_create_promotion_playoff_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let l = League(this);
    let s = read_usize(l.stages() + 4 * 8);
    SCOREBOARD.record(s != 0 && l.current_stage() as i32 == 8, || format!("bel_third_create_promotion_playoff_stage({:#x}): stage {:#x}, current {}", this, s, l.current_stage() as i32));
}

/// Comparer for `bel_third_create_relegation_playoff_stage`: only the original (it shuffles and builds the stage);
/// afterwards stage 9 must exist and be the current one.
pub(crate) unsafe extern "thiscall" fn compare_bel_third_create_relegation_playoff_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let l = League(this);
    let s = read_usize(l.stages() + 4 * 9);
    SCOREBOARD.record(s != 0 && l.current_stage() as i32 == 9, || format!("bel_third_create_relegation_playoff_stage({:#x}): stage {:#x}, current {}", this, s, l.current_stage() as i32));
}
