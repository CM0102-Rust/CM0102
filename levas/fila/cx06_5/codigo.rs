// ---------- cx06_5: the European Championship qualifiers' runners-up table (euro_champ_qual.cpp) ----------

/// Creates the qualifiers' runners-up stage (`.stages()`[+0x100]): a League_Stage of the 9 group runners-up, with no
/// teams, round table or rounds of its own, the tie-breaks 1, 2 and 5 (a local array), id 0x3fc. While it is built
/// `.promoted_count()` is 0 (the stage copies it) and then 1 again. The stage (`exe::crt_operator_new`(0xee) and
/// League_Stage::League_Stage `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58: the qualifiers, 9, no list, 0,
/// `.competition()`, no table, 0 rounds, the win, draw and category bytes, the local tie-breaks, &promoted, the year, the
/// stage index, 0x3fc, 0, 1, 0, 2, −1, 0, 2; null if out of memory) goes into its slot, `.current_stage()` becomes its
/// index and the table is filled (`euro_champ_qual_update_runners_up_table`).
/// Original: `EuroChampQual::create_runners_up_stage` (0x57f920, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_create_runners_up_stage(this: usize) {
    let l = League(this);
    let tiebreaks: [u8; 4] = [1, 2, 5, 0];
    l.set_promoted_count(0);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let index = read_u8(this + 0x100) as i8; // the runners-up stage index
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        covered!("cx06_5 runners-up stage built");
        construct(p, this, 9, 0, 0, l.competition().addr(), 0, 0, l.win_points() as u32, l.draw_points() as u32, l.match_category() as u32,
            tiebreaks.as_ptr() as usize, this + league::PROMOTED_COUNT, l.year() as u16 as u32, index as u8 as u32, 0x3fc, 0, 1, 0, 2, -1, 0, 2)
    } else {
        0
    };
    write_usize(l.stages() + 4 * index as isize as usize, stage);
    l.set_promoted_count(1);
    l.set_current_stage(index as isize as usize);
    euro_champ_qual_update_runners_up_table(this);
}

/// The record of a group's runner-up against the top four of its group, built as a sub-league of their games.
///
/// SubLeague::SubLeague (`exe::crt_operator_new`(0xee) and `FN_SUBLEAGUE_SUBLEAGUE`, thiscall, ret 0x30): the 4 club
/// ids `ids`, −1, −1, the dates `from` and `to`, the competition id, the win and draw points, &tie-breaks, 9 × n × (n −
/// 1) (n = the group's `.club_count()`) and the stored group results (+0x10). Null if out of memory.
unsafe fn runners_up_sub_league(this: League, group: League, ids: usize, from: usize, to: usize) -> usize {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, u32, usize, i32, i32, usize, usize, i32, u32, u32, usize, u32, usize) -> usize = core::mem::transmute(FN_SUBLEAGUE_SUBLEAGUE);
    let p = exe::crt_operator_new(0xee);
    if p == 0 {
        return 0;
    }
    let n = group.club_count() as i32;
    construct(p, 4, ids, -1, -1, from, to, read_i32(this.competition().addr()), this.win_points() as u32, this.draw_points() as u32,
        this.addr() + league::TIEBREAKS, (n.wrapping_sub(1).wrapping_mul(n).wrapping_mul(9)) as u32, read_usize(this.addr() + 0x10))
}

/// Ranks the qualifying groups' runners-up on their results against the top four of their own group.
///
/// The period: from 1 June 1998 (edition 1998, Date::Date `exe::date_date_549f70`(1, 5, 1998, −1)) or the first round
/// date (`.round_dates()`), to one week (`exe::date_add_weeks`) after the last round date (`.round_count()` − 1); the
/// dates go through Date::operator= (`exe::date_operator_3d`). For each group (the qualifiers themselves, then
/// `.stages()`[0..7]): its table sorted (`exe::league_sort_table`), its runner-up (row 1, `exe::league_get_table_row`)
/// and the ids of its top four in a new list (`exe::crt_malloc`(0x10); out of memory: the box at line 0x55b and
/// return, without the final sort); the sub-league of their games (`runners_up_sub_league`); when the runner-up is one
/// of its first 4 rows, that row is copied into row k + 1 of the runners-up table (`.stages()`[+0x100]) with its
/// head-to-head and position-history pointers cleared (the sub-league owns them); the sub-league destroyed (its virtual
/// 0 with 1; a null sub-league is read as one, a defect of the original out of memory) and the list freed. At the end
/// the runners-up table sorted.
/// Original: `EuroChampQual::update_runners_up_table` (0x57fa10, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_update_runners_up_table(this: usize) {
    let l = League(this);
    let (mut from, mut to, mut copy, mut date) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u16; 4]);
    exe::date_date_54a110(from.as_mut_ptr() as usize);
    exe::date_date_54a110(to.as_mut_ptr() as usize);
    if l.year() == 0x7ce {
        exe::date_date_549f70(&mut date, 1, 5, 0x7ce, 0xffff_ffff); // 1 June 1998
    } else {
        let first = l.round_dates();
        exe::new_date(date.as_mut_ptr() as usize, read_u16(first) as u32, read_u16(first + 2).wrapping_add(l.year() as u16) as u32);
    }
    exe::date_operator_3d(from.as_mut_ptr() as usize, copy.as_mut_ptr() as usize, date.as_ptr() as usize);
    let last = l.round_dates().wrapping_add((l.round_count() as i32 as usize).wrapping_mul(league_round_date::SIZE)).wrapping_sub(league_round_date::SIZE);
    exe::new_date(date.as_mut_ptr() as usize, read_u16(last) as u32, read_u16(last + 2).wrapping_add(l.year() as u16) as u32);
    let mut later = [0u8; 8];
    let week_after = exe::date_add_weeks(date.as_ptr() as usize, &mut later, 1);
    exe::date_operator_3d(to.as_mut_ptr() as usize, copy.as_mut_ptr() as usize, week_after);
    let runners_up_stage = || League(read_usize(l.stages() + 4 * read_u8(this + 0x100) as i8 as isize as usize));
    for k in -1i32..8 {
        let group = if k <= -1 { l } else { League(read_usize(l.stages() + 4 * k as usize)) };
        exe::league_sort_table(group, 0);
        let runner = read_usize(exe::league_get_table_row(group, 1));
        let ids = exe::crt_malloc(0x10);
        if ids == 0 {
            show_error_at(0x9c4674, 0x55b); // euro_champ_qual.cpp
            return;
        }
        for i in 0..4 {
            write_u32(ids + 4 * i as usize, read_u32(read_usize(exe::league_get_table_row(group, i))));
        }
        let sub = runners_up_sub_league(l, group, ids, from.as_ptr() as usize, to.as_ptr() as usize);
        for j in 0..4 {
            if read_usize(exe::league_get_table_row(sub, j)) == runner {
                covered!("cx06_5 runner-up row copied");
                let row = runners_up_stage().table() + (k + 1) as usize * league_table_row::SIZE;
                core::ptr::copy_nonoverlapping((League(sub).table() + j as usize * league_table_row::SIZE) as *const u8, row as *mut u8, league_table_row::SIZE);
                write_u32(row + league_table_row::POSITION_HISTORY, 0);
                write_u32(row + league_table_row::H2H_ENTRIES, 0);
                break;
            }
        }
        if sub != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(sub) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
            destroy(sub, 1);
        }
        exe::crt_free(ids);
    }
    exe::league_sort_table(runners_up_stage(), 0);
}

/// Comparer for `euro_champ_qual_create_runners_up_stage`: it allocates the stage (it cannot run twice), so only the
/// original runs; the plan: the stage in its slot with 9 rows, `.promoted_count()` 1 and `.current_stage()` its index.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_create_runners_up_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let l = League(this);
    let index = read_u8(this + 0x100) as i8 as isize as usize;
    let stage = League(read_usize(l.stages() + 4 * index));
    let ok = stage.addr() != 0 && stage.club_count() == 9 && l.promoted_count() == 1 && l.current_stage() == index;
    SCOREBOARD.record(ok, || format!("euro_champ_qual_create_runners_up_stage({:#x}): stage {:#x}, current stage {}", this, stage.addr(), l.current_stage() as i32));
}

/// Comparer for `euro_champ_qual_update_runners_up_table`: side by side on the runners-up table (its 9 rows); the
/// groups' sorts and the sub-leagues are the same on both sides.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_update_runners_up_table(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let l = League(this);
    let stage = League(read_usize(l.stages() + 4 * read_u8(this + 0x100) as i8 as isize as usize));
    if stage.addr() == 0 || stage.table() == 0 {
        return o(this);
    }
    let regions = [(stage.table(), stage.club_count().clamp(0, 9) as usize * league_table_row::SIZE)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || euro_champ_qual_update_runners_up_table(this));
    SCOREBOARD.record(same, || format!("euro_champ_qual_update_runners_up_table({:#x}): {}", this, first_differences(&mo, &mr).join(", ")));
}
