// ---------- cx01_4: a club still in contention, a competition's first date and a cup round's extra fixtures (comp.cpp, Cup.cpp) ----------

/// Tells whether a club is still in contention in a competition (1/0).
///
/// Never in contention (0) when there is no reserve competition object (`G_RESERVE_COMP`) or the competition is a
/// reserve or minor one (`exe::comp_is_reserve_or_minor_comp`), nor when there is no friendly object (`G_FRIENDLY`) or
/// the competition is a friendly one. A stage (`STAGE_INDEX` not 0xff) answers 1. Otherwise 0 when the club's entry in
/// the statistics has equal best and worst positions other than 0 (its place is decided); then the stages are searched
/// from the current one (`CURRENT_STAGE`, as a signed byte) back to the competition itself (−1): in a cup stage
/// (`COMP_TYPE` 1) a club in the team table (6-byte entries, `TEAM_COUNT`) answers 1; in a league stage (2) a club in
/// the table (`league_table_row::SIZE` rows, `CLUB_COUNT`) answers 0 when its status is decided (0xfe, 7, 6 or 5),
/// otherwise whether its best and worst possible positions still differ. A club found nowhere, or a current stage below
/// −1, answers 1.
/// Original: `Comp::is_club_still_in_contention` (0x48f150, comp.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_is_club_still_in_contention(this: usize, club: usize) -> u32 {
    let comp = League(this);
    let reserve = read_usize(G_RESERVE_COMP);
    if reserve == 0 || exe::comp_is_reserve_or_minor_comp(reserve, comp.competition()) != 0 {
        return 0;
    }
    let friendly = read_usize(G_FRIENDLY);
    if friendly == 0 || exe::friendly_is_friendly_competition(friendly, comp.competition()) != 0 {
        return 0;
    }
    if comp.stage_index() != 0xff {
        return 1;
    }
    if comp.comp_stats() != 0 {
        let entry = CompStatsClub(exe::comp_stats_find_club_entry(comp.comp_stats(), club));
        if !entry.is_null() && entry.best_position() == entry.worst_position() && entry.best_position() != 0 {
            covered!("cx01_4 contention: place decided");
            return 0;
        }
    }
    let mut k = comp.current_stage() as u8 as i8;
    while k >= -1 {
        let stage = League(if k == -1 { this } else { read_usize(comp.stages() + k as isize as usize * 4) });
        let n = stage.club_count(); // the cup's TEAM_COUNT at the same offset
        match stage.comp_type() {
            1 => {
                let teams = Cup(stage.addr()).team_table();
                for i in 0..n.max(0) as usize {
                    if teams != 0 && read_usize(teams + 6 * i) == club {
                        covered!("cx01_4 contention: in a cup");
                        return 1;
                    }
                }
            }
            2 => {
                for i in 0..n.max(0) as usize {
                    let table = stage.table();
                    if table != 0 && read_usize(table + league_table_row::SIZE * i) == club {
                        covered!("cx01_4 contention: in a league");
                        let row = table + league_table_row::SIZE * i;
                        return match read_u8(row + league_table_row::STATUS) {
                            0xfe | 7 | 6 | 5 => 0,
                            _ => (read_u8(row + league_table_row::BEST_POSSIBLE_POSITION) != read_u8(row + league_table_row::WORST_POSSIBLE_POSITION)) as u32,
                        };
                    }
                }
            }
            _ => {}
        }
        k -= 1;
    }
    1
}

/// Writes into `out` (8 bytes) the date of the competition's first scheduled entry and returns `out`.
///
/// Asks the competition for its whole schedule (`comp_vtable::CREATE_SCHEDULE` with stage −1). With one, the first
/// entry's date: in a league-style schedule (the flag set by the call) the day word +0 and the year word +2, otherwise
/// the day word +3 and the signed year byte +5, each year relative to the competition's `YEAR`
/// (`exe::new_date`), copied over the default date (`exe::date_date_54a110`, `exe::date_operator_3d`); the schedule is
/// freed. Without one, the default date.
/// Original: `Comp::get_first_schedule_date` (0x48f340, comp.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_get_first_schedule_date(this: usize, out: usize) -> usize {
    let comp = League(this);
    let mut date = [0u8; 8];
    exe::date_date_54a110(date.as_mut_ptr() as usize);
    let (mut rounds, mut extra, mut league) = (0i16, 0i16, 0i32);
    let schedule: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, *mut i32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let s = schedule(this, -1, &mut rounds, &mut extra, &mut league);
    if s != 0 {
        let (day, year) = if league != 0 {
            covered!("cx01_4 first date of a league schedule");
            (read_u16(s), read_u16(s + 2).wrapping_add(comp.year() as u16))
        } else {
            covered!("cx01_4 first date of a cup schedule");
            (read_u16(s + 3), (read_u8(s + 5) as i8 as i16 as u16).wrapping_add(comp.year() as u16))
        };
        let (mut first, mut scratch) = ([0u8; 8], [0u8; 8]);
        exe::new_date(first.as_mut_ptr() as usize, day as u32, year as u32);
        exe::date_operator_3d(date.as_mut_ptr() as usize, scratch.as_mut_ptr() as usize, first.as_ptr() as usize);
        exe::crt_free(s);
    }
    core::ptr::copy_nonoverlapping(date.as_ptr(), out as *mut u8, 8);
    out
}

/// The two fixtures that `label_round_extra_fixtures` labels (each with the value its `ROUND_NAME` gets), found
/// without writing; with `write`, labelled as they are found.
unsafe fn round_extra_fixtures(this: usize, round: i16, home: u32, away: u32, write: bool) -> [(usize, i16); 2] {
    let cup = Cup(this);
    let record = CupRound(cup.round_table() + round as isize as usize * 0x68);
    let comp_id = read_u32(League(this).competition().addr()) as i32;
    let (mut date, mut scratch, mut moved) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let d = date.as_mut_ptr() as usize;
    exe::new_date(d, record.day() as u16 as u32, (record.year_offset() as i8 as i16).wrapping_add(cup.year()) as u16 as u32);
    let earlier = exe::date_sub_weeks(d, &mut scratch, 2);
    exe::date_operator_3d(d, moved.as_mut_ptr() as usize, earlier);
    let mut found = [(0usize, 0i16); 2];
    for (k, step) in [3i16, 4].into_iter().enumerate() {
        let (mut year, mut day, mut slice) = (read_u16(d + 2), read_u16(d), 0u8);
        let f = find_next_fixture(read_usize(G_FIXTURE_MANAGER), &mut year, &mut day, &mut slice, 1, comp_id, round as i32, home as i32, away as i32, u32::MAX, u32::MAX, u32::MAX);
        if f == 0 {
            break;
        }
        let fixture = Fixture(f);
        let name = record.round_code().wrapping_add(step);
        found[k] = (f, name);
        if write {
            fixture.set_round_name(name);
        }
        if k == 0 {
            // the search goes on from the day after this fixture
            let mut on = [0u8; 8];
            exe::new_date(on.as_mut_ptr() as usize, fixture.day() as u32, fixture.year() as u32);
            exe::date_operator_3d(d, scratch.as_mut_ptr() as usize, on.as_ptr() as usize);
            let next = exe::date_add_days(d, moved.as_mut_ptr() as usize, 1);
            exe::date_operator_3d(d, scratch.as_mut_ptr() as usize, next);
        }
    }
    found
}

/// Labels a cup round's two extra fixtures between the same clubs: from two weeks before the round's date
/// (`CupRound` day and year offset, the cup's `YEAR`; `exe::date_sub_weeks`), the next fixture of this competition, this
/// round number and the two clubs (`find_next_fixture` forward from slice 0, the other filters −1) gets the round's code
/// + 3 as its `ROUND_NAME`; then, from the day after it, the next one gets the code + 4. The search stops at the first
/// one not found.
/// Original: `Cup::label_round_extra_fixtures` (0x522c70, Cup.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn cup_label_round_extra_fixtures(this: usize, round: u32, home: u32, away: u32) {
    covered!("cx01_4 cup round extra fixtures");
    round_extra_fixtures(this, round as i16, home, away, true);
}

/// Comparer for `comp_is_club_still_in_contention` (the value).
pub(crate) unsafe extern "thiscall" fn compare_comp_is_club_still_in_contention(this: usize, club: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, club);
    let n = comp_is_club_still_in_contention(this, club);
    SCOREBOARD.record(r == n, || format!("comp_is_club_still_in_contention({:#x}, club {:#x}): original {} rust {}", this, club, r, n));
    r
}

/// Comparer for `comp_get_first_schedule_date`: the original into `out` and the copy into a local date; the 8 bytes and
/// the return (the copy's own local) must match. A cup must not build its round table twice (the Cup.cpp box): there
/// only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_comp_get_first_schedule_date(this: usize, out: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, out);
    if League(this).comp_type() != 2 {
        return r;
    }
    let mut mine = [0u8; 8];
    let n = comp_get_first_schedule_date(this, mine.as_mut_ptr() as usize);
    let theirs = core::slice::from_raw_parts(out as *const u8, 8);
    SCOREBOARD.record(r == out && n == mine.as_ptr() as usize && theirs == mine, || format!("comp_get_first_schedule_date({:#x}): original {:x?} rust {:x?}", this, theirs, mine));
    r
}

/// Comparer for `cup_label_round_extra_fixtures`: the copy finds the fixtures without writing, then the original runs;
/// the fixtures it labelled must be the copy's, with the same names.
pub(crate) unsafe extern "thiscall" fn compare_cup_label_round_extra_fixtures(this: usize, round: u32, home: u32, away: u32) {
    let plan = round_extra_fixtures(this, round as i16, home, away, false);
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32)>(%%I2%%)(this, round, home, away);
    let ok = plan.iter().all(|&(f, name)| f == 0 || Fixture(f).round_name() == name);
    SCOREBOARD.record(ok, || format!("cup_label_round_extra_fixtures({:#x}, round {}, {}, {}): planned {:x?}, names {:x?}", this, round as i16, home as i32, away as i32,
        plan, plan.iter().map(|&(f, _)| if f != 0 { Fixture(f).round_name() } else { 0 }).collect::<Vec<_>>()));
}
