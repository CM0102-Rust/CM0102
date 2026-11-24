// ---------- sbA_8: scanning the fixture calendar from a date (fix_man.cpp) ----------

/// The key nations whose fixture lists a calendar scan reads: the competition's (`comp_key_nation`), or for club A a
/// national team's 0 and 1 or a club's own (`comp_active_key_nation_of_club`), its partner
/// (`fix_man_get_partner_key_nation_index`) and the continental entries of its nation's continent (the entries of
/// `G_LEAGUE_TABLE` without a nation whose +4 is that continent), then the same three for club B; a repeated nation is
/// left out (0xff).
unsafe fn scan_fixtures_nations(comp: i32, club_a: i32, club_b: i32) -> Vec<u8> {
    let mut nations = Vec::new();
    let club_nations = |club: Club, nations: &mut Vec<u8>| {
        nations.push(exe::comp_active_key_nation_of_club(club));
        nations.push(exe::fix_man_get_partner_key_nation_index(club));
        for e in 0..0x23 {
            let entry = G_LEAGUE_TABLE + e * 0x48;
            let nation = club.nation();
            if read_usize(entry) == 0 && !nation.is_null() && read_usize(entry + 4) == nation.continent() { // +4: the entry's continent
                nations.push(read_u8(entry + key_nation_record::ACTIVE_INDEX));
            }
        }
    };
    if comp != -1 {
        nations.push(comp_key_nation(comp as u32));
    } else {
        if club_a != -1 {
            let club = club_at(club_a as isize);
            if exe::database_is_national_team(club) != 0 {
                nations.extend_from_slice(&[0, 1]);
            } else {
                club_nations(club, &mut nations);
            }
        }
        if club_b != -1 {
            club_nations(club_at(club_b as isize), &mut nations);
        }
    }
    for i in 0..nations.len().saturating_sub(1) {
        for j in i + 1..nations.len() {
            if (nations[i] as i8) > -1 && nations[i] == nations[j] {
                nations[j] = 0xff;
            }
        }
    }
    nations
}

/// The filters of a calendar scan: the stage (+0x42; 0xfe any), the competition (+0; -1 any), the round (+0x34, word:
/// from `round_min` to `round_max`, or exactly `round_min` without a maximum; -1 any), clubs A and B (home +0xc or
/// away +0x10; -1 any), the side marks (`fixture::HOME_SIDE_MARK` and `AWAY_SIDE_MARK` both; 0xff any), played (0: not played yet, +0x43 = 0xff; 1: played;
/// other any) and, without `with_odd`, a result byte not under -1.
#[derive(Clone, Copy)]
struct ScanFilter {
    comp: i32,
    round_min: i32,
    round_max: i32,
    club_a: i32,
    club_b: i32,
    played: u8,
    stage: u8,
    side: u8,
    with_odd: u8,
}

unsafe fn scan_filter_matches(f: Fixture, s: &ScanFilter) -> bool {
    let a = f.addr();
    let mut ok = s.stage == 0xfe || s.stage == read_u8(a + fixture::STAGE_INDEX);
    if s.comp != -1 && s.comp != read_i32(a) {
        ok = false;
    }
    if s.round_min != -1 {
        let round = read_i16(a + fixture::ROUND_NUMBER) as i32;
        if s.round_max != -1 {
            if s.round_min > round || s.round_max < round {
                ok = false;
            }
        } else if s.round_min != round {
            ok = false;
        }
    }
    for club in [s.club_a, s.club_b] {
        if club != -1 && club != read_i32(a + fixture::HOME_CLUB_ID) && club != read_i32(a + fixture::AWAY_CLUB_ID) {
            ok = false;
        }
    }
    if s.side != 0xff && (s.side != read_u8(a + fixture::HOME_SIDE_MARK) || s.side != read_u8(a + fixture::AWAY_SIDE_MARK)) {
        ok = false;
    }
    let goals = read_u8(a + fixture::HOME_GOALS);
    if (s.played == 0 && goals != 0xff) || (s.played == 1 && goals == 0xff) {
        ok = false;
    }
    if s.with_odd == 0 && (goals as i8) < -1 {
        return false;
    }
    ok
}

/// Scans the fixture calendar from a date and slice, forwards or backwards, copying the fixtures that pass the filters
/// (`ScanFilter`) into `out` (`LLIST::add_tail`, 0x4f bytes); returns how many.
///
/// The lists read are those of the scan's key nations (`scan_fixtures_nations`) for each day and slice
/// (`FIXTURE_MANAGER::get_fixture_list`). With a maximum (`max`, word) the scan stops when it is reached, writing the
/// day, year and slice where it stopped; with -2 it stops after the first day with something copied (slice 0 forwards,
/// 2 backwards); with -1 it stops after one more year, otherwise after 5, writing the first (forwards) or last
/// (backwards) day of the year it reached, the year and the slice it had. Forwards the days run to 365 and the year
/// grows; backwards they run down to 0 and the year shrinks. A failed copy shows the error box (0xa54 forwards, 0xaf5
/// backwards) and returns the count without writing the position.
/// Original: `FIXTURE_MANAGER::scan_fixtures` (0x5acee0, fix_man.cpp, thiscall, ret 0x48).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn scan_fixtures(this: usize, year_at: *mut i16, day_at: *mut i16, slice_at: *mut u8, forward: u32,
                                                     comp: i32, round_min: i32, round_max: i32, club_a: i32, club_b: i32,
                                                     _a10: u32, _a11: u32, _a12: u32, max: u32, out: usize, played: u32,
                                                     stage: u32, leg: u32, with_odd: u32) -> u32 {
    let filter = ScanFilter { comp, round_min, round_max, club_a, club_b, played: played as u8, stage: stage as u8, side: leg as u8, with_odd: with_odd as u8 };
    let max = max as u16 as i16;
    let start_year = *year_at as i32;
    let (mut year, mut day, mut slice) = (*year_at, *day_at, *slice_at as i16);
    let nations = scan_fixtures_nations(comp, club_a, club_b);
    let mut count: u16 = 0;
    let stop = |year: i16, day: i16, slice: u8| {
        *day_at = day;
        *year_at = year;
        *slice_at = slice;
    };
    // the lists of one day and slice: Err(()) after the error box, Ok(true) when the maximum is reached
    let mut scan_slice = |year: i16, day: i16, slice: i16, count: &mut u16, row: u32| -> Result<bool, ()> {
        for &nation in &nations {
            if nation == 0xff {
                continue;
            }
            let list = exe::fixture_manager_get_fixture_list(this, year as u32, day as u32, slice as u32, nation as u32);
            if list == 0 {
                continue;
            }
            let mut f = list_first(list);
            while f != 0 {
                if scan_filter_matches(Fixture(f), &filter) {
                    if exe::llist_add_tail(out, f, 0x4f) == 0 {
                        show_error_at(0x9cc208, row);
                        return Err(());
                    }
                    *count = count.wrapping_add(1);
                    if max != -1 && *count as i32 == max as i32 {
                        return Ok(true);
                    }
                }
                f = exe::llist_get_next(list);
            }
        }
        Ok(false)
    };
    if forward != 0 {
        loop {
            while day < 0x16e {
                while slice < 3 {
                    match scan_slice(year, day, slice, &mut count, 0xa54) {
                        Err(()) => return count as u32,
                        Ok(true) => {
                            stop(year, day, slice as u8);
                            return count as u32;
                        }
                        Ok(false) => {}
                    }
                    slice += 1;
                }
                if max == -2 && count != 0 {
                    stop(year, day, 0);
                    return count as u32;
                }
                slice = 0;
                day += 1;
            }
            year += 1;
            day = 0;
            let years = year as i32 - start_year;
            if years >= 5 || (max == -1 && years > 1) {
                stop(year, 0, slice as u8);
                return count as u32;
            }
        }
    }
    loop {
        while day >= 0 {
            while slice >= 0 {
                match scan_slice(year, day, slice, &mut count, 0xaf5) {
                    Err(()) => return count as u32,
                    Ok(true) => {
                        stop(year, day, slice as u8);
                        return count as u32;
                    }
                    Ok(false) => {}
                }
                slice -= 1;
            }
            slice = 2;
            if max == -2 && count != 0 {
                stop(year, day, 2);
                return count as u32;
            }
            day -= 1;
        }
        year -= 1;
        day = 0x16d;
        let years = start_year - year as i32;
        if years >= 5 || (max == -1 && years > 1) {
            stop(year, 0x16d, slice as u8);
            return count as u32;
        }
    }
}

/// Comparer for `scan_fixtures`: the original on the real list and position; the copy on a temporary list and copies
/// of the position: the count, the copied fixtures and the position written must match.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_scan_fixtures(this: usize, year_at: *mut i16, day_at: *mut i16, slice_at: *mut u8, forward: u32,
                                                             comp: i32, round_min: i32, round_max: i32, club_a: i32, club_b: i32,
                                                             a10: u32, a11: u32, a12: u32, max: u32, out: usize, played: u32,
                                                             stage: u32, leg: u32, with_odd: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut i16, *mut i16, *mut u8, u32, i32, i32, i32, i32, i32, u32, u32, u32, u32, usize, u32, u32, u32, u32) -> u32>(%%I0%%);
    let (mut y, mut d, mut s) = (*year_at, *day_at, *slice_at);
    let (count_before, tail_before) = (read_u16(out), read_u32(out + llist::TAIL));
    let r = o(this, year_at, day_at, slice_at, forward, comp, round_min, round_max, club_a, club_b, a10, a11, a12, max, out, played, stage, leg, with_odd);
    let (n, equal) = run_with_temp_list(out, count_before, tail_before, |p| scan_fixtures(this, &mut y, &mut d, &mut s, forward, comp, round_min,
        round_max, club_a, club_b, a10, a11, a12, max, p, played, stage, leg, with_odd));
    let same_place = (y, d, s) == (*year_at, *day_at, *slice_at);
    SCOREBOARD.record(equal && same_place && r as u16 == n as u16,
        || format!("scan_fixtures(forward {}, comp {}, clubs {}/{}, max {}) original={} at {}/{}/{} rust={} at {}/{}/{} items equal {}",
            forward, comp, club_a, club_b, max as u16 as i16, r as u16, *year_at, *day_at, *slice_at, n as u16, y, d, s, equal));
    r
}
