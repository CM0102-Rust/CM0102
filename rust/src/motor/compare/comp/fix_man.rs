use super::*;

// The comparers of the routines in ../comp/fix_man.rs.

/// Comparer for `fix_man_daily_update`.
///
/// The comparer of the calendar day: only the original routine runs (simplification: the news items go to the managers,
/// the friendly and queue nodes are freed and the reserve matches enter the calendar: it does not run twice).
///
/// It checks what the routine guarantees: the news queue disappears ([+0x1128] = 0 if there was one) and the conference
/// list ends up empty.
pub(crate) unsafe extern "thiscall" fn compare_fix_man_daily_update(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FIX_MAN_DAILY_UPDATE);
    let havia = read_usize(this + fixture_manager::MOVED_NEWS_QUEUE);
    let before = read_i16(G_FIXTURE_LEGS_LIST);
    o(this);
    let queue = read_usize(this + fixture_manager::MOVED_NEWS_QUEUE);
    let after = read_i16(G_FIXTURE_LEGS_LIST);
    SCOREBOARD.record((havia == 0 || queue == 0) && (before == 0 || after == 0),
        || format!("fix_man_daily_update: queue das news {:x} -> {:x}, checks {} -> {}", havia, queue, before, after));
}

/// Comparer for `get_fixture_list`.
pub(crate) unsafe extern "thiscall" fn compare_get_fixture_list(this: usize, year: u32, day: u32, slice: u32, kind: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize>(IDX_GET_FIXTURE_LIST);
    let other = read_usize(this + fixture_manager::OTHER_YEAR);
    let direct = read_i16(this + fixture_manager::YEAR) == year as i16 || (other != 0 && read_i16(other + fixture_manager::YEAR) == year as i16);
    if !direct {
        return o(this, year, day, slice, kind);
    }
    let r = o(this, year, day, slice, kind);
    let n = get_fixture_list(this, year, day, slice, kind);
    SCOREBOARD.record(r == n, || format!("get_fixture_list(year {} day {} slice {} kind {}): original={:x} rust={:x}",
        year as u16, day as u16, slice as u8, kind as u8 as i8, r, n));
    r
}

/// Comparer for `load_fixtures`.
pub(crate) unsafe extern "thiscall" fn compare_load_fixtures(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_LOAD_FIXTURES)(this);
    let expected = format!("fixtures_{}.tmp", read_i16(this + fixture_manager::YEAR) as u16);
    let s = core::slice::from_raw_parts((this + fixture_manager::FILE_NAME) as *const u8, expected.len() + 1);
    SCOREBOARD.record(&s[..expected.len()] == expected.as_bytes() && s[expected.len()] == 0,
        || format!("load_fixtures: name {:?} (expected {})", s, expected));
}

/// Comparer for `collect_day_clashes`.
///
/// The copy as a dry run (it stores the matches instead of appending them, with its own severity) and the original from
/// the same state; it checks the severity and the new nodes at the end of the list (the 0x4f bytes of each match, in
/// order). The day lists (`get_fixture_list`) have their cursor reset in both.
pub(crate) unsafe extern "thiscall" fn compare_collect_day_clashes(this: usize, game: usize, grav: *mut i32, list: usize, nations: usize, n: u32, data: usize, desloc: u32) {
    let game = Fixture(game);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i32, usize, usize, u32, usize, u32)>(IDX_COLLECT_DAY_CLASHES);
    if game.is_null() || list == 0 || grav.is_null() || game.home_club().is_null() || game.away_club().is_null() {
        return o(this, game.addr(), grav, list, nations, n, data, desloc);
    }
    let mut g = *grav;
    DAY_CLASHES_APPENDED.clear();
    DAY_CLASHES_DRY_RUN.store(true, Relaxed);
    collect_day_clashes(this, game.addr(), &mut g, list, nations, n, data, desloc);
    DAY_CLASHES_DRY_RUN.store(false, Relaxed);
    let expected = fixtures_bytes(&DAY_CLASHES_APPENDED);
    let ne = DAY_CLASHES_APPENDED.len();
    let before = fix_man_list_items(list);
    o(this, game.addr(), grav, list, nations, n, data, desloc);
    let after = fix_man_list_items(list);
    let same = *grav == g && after.len() == before.len() + ne && after[..before.len()] == before[..] && fixtures_bytes(&after[before.len()..]) == expected;
    SCOREBOARD.record(same, || format!("collect_day_clashes(game {:x}, desloc {}) severity original={} rust={}, games new original={} rust={}", game.addr(), desloc as u8 as i8, *grav, g, after.len().saturating_sub(before.len()), ne));
}

/// Comparer for `find_fixture_clashes`.
///
/// The comparer of one match's conflicts: the original routine and the copy, each with its own new list, from the same
/// state (the original does not change the match: it only creates the list and moves the cursors of the day lists,
/// which both redo).
///
/// It checks the return value (list or 0), the output and the matches of the two lists (0x4f bytes each, in order), and
/// destroys and frees the copy's list as the calendar does (`exe::drop_llist`, `exe::crt_operator_delete`).
pub(crate) unsafe extern "thiscall" fn compare_find_fixture_clashes(this: usize, game: usize, out: *mut i32, two_days: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize>(IDX_FIND_FIXTURE_CLASHES);
    if game == 0 || out.is_null() {
        return o(this, game, out, two_days);
    }
    let r = o(this, game, out, two_days);
    let go = *out;
    let mut gr: i32 = 0;
    let nr = find_fixture_clashes(this, game, &mut gr, two_days);
    let (lo, lr) = (fixtures_bytes(&fix_man_list_items(r)), fixtures_bytes(&fix_man_list_items(nr)));
    let same = (r == 0) == (nr == 0) && go == gr && lo == lr;
    SCOREBOARD.record(same, || format!("find_fixture_clashes(game {:x}) original={:x} out={} games={} rust={:x} out={} games={}", game, r, go, lo.len() / 0x4f, nr, gr, lr.len() / 0x4f));
    if nr != 0 {
        exe::drop_llist(nr);
        exe::crt_operator_delete(nr);
    }
    r
}

/// Comparer for `fix_man_load_fixture_leg_checks`.
pub(crate) unsafe extern "C" fn compare_fix_man_load_fixture_leg_checks() {
    original_fn::<unsafe extern "C" fn()>(IDX_FIX_MAN_LOAD_FIXTURE_LEG_CHECKS)()
}

/// Comparer for `find_new_fixture_date`.
pub(crate) unsafe extern "thiscall" fn compare_find_new_fixture_date(this: usize, game: usize) {
    let game = Fixture(game);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FIND_NEW_FIXTURE_DATE);
    // the window box without a date (+0x24, type 2) would appear twice: original only
    if game.is_null() || matches!(exe::comp_util_get_round_variant((game.round_name() as u16) as i16 as i32), 3 | 4) || (game.category() == match_category::LEAGUE && game.next_round_year() == 0) {
        return o(this, game.addr());
    }
    let reg = [(game.addr(), 0x4f)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, game.addr()), || find_new_fixture_date(this, game.addr()));
    SCOREBOARD.record(igual, || format!("find_new_fixture_date(game {:x}) original={:x?} rust={:x?}", game.addr(), mo, mr));
}

/// Comparer for `add_fixture`.
pub(crate) unsafe extern "thiscall" fn compare_add_fixture(this: usize, game: usize, clear: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ADD_FIXTURE)(this, game, clear)
}

/// Comparer for `set_year`.
pub(crate) unsafe extern "thiscall" fn compare_set_year(this: usize, year: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_SET_YEAR)(this, year)
}

/// Comparer for `fix_man_save_club_dates`.
pub(crate) unsafe extern "C" fn compare_fix_man_save_club_dates() {
    // simplification: writes the save file (cannot run twice); the copy is proved by the save proof (the .qst
    // written in Rust mode with and without the batch, item by item: club_dates.tmp)
    original_fn::<unsafe extern "C" fn()>(IDX_FIX_MAN_SAVE_CLUB_DATES)()
}

/// Comparer for `fix_man_load_club_dates`.
pub(crate) unsafe extern "C" fn compare_fix_man_load_club_dates() {
    // simplification: allocates the global table and reads the file (cannot run twice); the copy is proved by loading the
    // holiday save in the save proof (levas/N/CARGA) and, without the file, by the new career
    original_fn::<unsafe extern "C" fn()>(IDX_FIX_MAN_LOAD_CLUB_DATES)()
}

/// Comparer for `fix_man_save_fixture_leg_checks`.
pub(crate) unsafe extern "C" fn compare_fix_man_save_fixture_leg_checks() {
    // simplification: writes the save file and advances the list cursor; the copy is proven by the save proof
    original_fn::<unsafe extern "C" fn()>(IDX_FIX_MAN_SAVE_FIXTURE_LEG_CHECKS)()
}

/// Comparer for `resolve_same_day_clash` (comparison mode: see the header of motor.rs), as a dry run.
///
/// The routine only writes the list cursor (+0xa, read by `next_list_item`) and the rest goes through the calendar calls
/// (remove, merge, clashes that allocate the list, the news item), which cannot run twice.
///
/// The copy runs as a dry run (it stops at the decision, without the box) and the original routine afterwards, from the
/// same state: side by side on the list header (+2 start, +0xa cursor). Without the match or the first in the list:
/// only the original (the box).
pub(crate) unsafe extern "thiscall" fn compare_resolve_same_day_clash(this: usize, game: usize, conflitos: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_RESOLVE_SAME_DAY_CLASH);
    let cabeca = read_usize(conflitos + 2);
    if game == 0 || cabeca == 0 || read_usize(cabeca) == 0 {
        return o(this, game, conflitos);
    }
    let reg = [(conflitos + 2, 0xc)];
    let before = crate::comparison::MemorySnapshot::read(&reg);
    SAME_DAY_CLASH_DRY_RUN.store(true, Relaxed);
    resolve_same_day_clash(this, game, conflitos);
    SAME_DAY_CLASH_DRY_RUN.store(false, Relaxed);
    let mr = crate::comparison::MemorySnapshot::read(&reg);
    before.write_back();
    let r = o(this, game, conflitos);
    let mo = crate::comparison::MemorySnapshot::read(&reg);
    SCOREBOARD.record(mo == mr && (r == 0 || r == 1), || format!("resolve_same_day_clash(game {:#x}) returned {}, cursor of the list {:x?} x {:x?}", game, r, mo, mr));
    r
}

/// Comparer for `move_nearby_club_fixtures`.
pub(crate) unsafe extern "thiscall" fn compare_move_nearby_club_fixtures(this: usize, game: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_MOVE_NEARBY_CLUB_FIXTURES)(this, game)
}

/// Comparer for `reschedule_club_fixtures_near_date`.
pub(crate) unsafe extern "thiscall" fn compare_reschedule_club_fixtures_near_date(this: usize, club: usize, data: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RESCHEDULE_CLUB_FIXTURES_NEAR_DATE)(this, club, data)
}

/// Comparer for `queue_fixture_moved_news` (comparison mode: see the header of motor.rs), as a dry run.
///
/// Already in the queue: side by side on the queue header and the found news item (the new date). New: the copy builds
/// the news item without queueing it and the original runs; the queue must grow by one and the last item (the tail, +6,
/// node +0) must be the copy's news item over the 0xde bytes the constructor (`construct_news_item`) writes; the last of the 0xdf
/// copied bytes is stack leftover in the original (the copy puts zero).
pub(crate) unsafe extern "thiscall" fn compare_queue_fixture_moved_news(this: usize, f: usize, causa: usize, old: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_QUEUE_FIXTURE_MOVED_NEWS);
    let l = read_usize(this + fixture_manager::MOVED_NEWS_QUEUE);
    let target = if l != 0 { find_queued_fixture_news(l, f) } else { 0 };
    if target != 0 {
        let regions = [(this + fixture_manager::MOVED_NEWS_QUEUE, 4), (l, 0xe), (target, 0xdf)];
        let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, f, causa, old), || queue_fixture_moved_news(this, f, causa, old));
        SCOREBOARD.record(same, || format!("queue_fixture_moved_news(game {:#x}, na queue) original={:x?} rust={:x?}", f, mo, mr));
        return;
    }
    let mut regions = vec![(this + fixture_manager::MOVED_NEWS_QUEUE, 4)];
    if l != 0 {
        regions.push((l, 0xe));
    }
    let before = crate::comparison::MemorySnapshot::read(&regions);
    let count = if l != 0 { read_i16_as_i32(l) } else { 0 };
    FIXTURE_MOVED_NEWS_DRY_RUN.store(true, Relaxed);
    FIXTURE_MOVED_NEWS_WAS_BUILT.store(false, Relaxed);
    crate::rng::pause_trace(|| queue_fixture_moved_news(this, f, causa, old));
    FIXTURE_MOVED_NEWS_DRY_RUN.store(false, Relaxed);
    before.write_back();
    o(this, f, causa, old);
    let l2 = read_usize(this + fixture_manager::MOVED_NEWS_QUEUE);
    let built = FIXTURE_MOVED_NEWS_WAS_BUILT.load(Relaxed);
    let same = built
        && l2 != 0
        && read_i16_as_i32(l2) == count + 1
        && read_usize(read_usize(l2 + 6)) != 0
        && core::slice::from_raw_parts(read_usize(read_usize(l2 + 6)) as *const u8, 0xde)
            == core::slice::from_raw_parts(core::ptr::addr_of!(FIXTURE_MOVED_NEWS_BUILT) as *const u8, 0xde);
    SCOREBOARD.record(same, || format!("queue_fixture_moved_news(game {:#x}, new) built={} count {} -> {}", f, built, count, if l2 != 0 { read_i16_as_i32(l2) } else { -1 }));
}

/// Comparer for `find_stored_fixture`.
pub(crate) unsafe extern "thiscall" fn compare_find_stored_fixture(this: usize, f: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FIND_STORED_FIXTURE)(this, f);
    let n = find_stored_fixture(this, f);
    SCOREBOARD.record(r == n, || format!("find_stored_fixture(game {:x}) original={:x} rust={:x}", f, r, n));
    r
}

/// Comparer for `insert_fixture`.
pub(crate) unsafe extern "thiscall" fn compare_insert_fixture(this: usize, f: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_INSERT_FIXTURE);
    let l = find_fixture_list(this, f);
    if l == 0 {
        return o(this, f); // simplification: no memory for the block, original only (the box)
    }
    let count = read_u16(l);
    o(this, f);
    let tail = read_usize(l + 6);
    let same = read_u16(l) == count.wrapping_add(1) && tail != 0
        && core::slice::from_raw_parts(read_usize(tail) as *const u8, 0x4f) == core::slice::from_raw_parts(f as *const u8, 0x4f);
    SCOREBOARD.record(same, || format!("insert_fixture(game {:x}) count before={} after={}", f, count, read_u16(l)));
}

/// Comparer for `remove_fixture`.
pub(crate) unsafe extern "thiscall" fn compare_remove_fixture(this: usize, f: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_REMOVE_FIXTURE);
    let l = find_fixture_list(this, f);
    if l == 0 {
        return o(this, f); // simplification: no memory for the block, original only (the box)
    }
    let count = read_u16(l);
    let mut in_the = read_usize(l + 2);
    let mut has = false;
    while in_the != 0 && !has {
        let n = read_i16(in_the + 4) as isize as usize;
        has = core::slice::from_raw_parts(read_usize(in_the) as *const u8, n) == core::slice::from_raw_parts(f as *const u8, n);
        in_the = read_usize(in_the + 6);
    }
    let r = o(this, f);
    let expected = has as u32;
    let same = r == expected && read_u16(l) == count.wrapping_sub(expected as u16);
    SCOREBOARD.record(same, || format!("remove_fixture(game {:x}) original={} expected={} count before={} after={}",
        f, r, expected, count, read_u16(l)));
    r
}

/// Comparer for `find_fixtures`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_find_fixtures(this: usize, dia0: u32, ano0: u32, dia1: u32, ano1: u32, club: u32,
                                                               list: usize, comp: u32, kind: u32, side: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32>(IDX_FIND_FIXTURES);
    if list == 0 {
        let r = o(this, dia0, ano0, dia1, ano1, club, 0, comp, kind, side);
        let n = find_fixtures(this, dia0, ano0, dia1, ano1, club, 0, comp, kind, side);
        SCOREBOARD.record(r as u16 == n as u16, || format!("find_fixtures({}/{} a {}/{}, club {}, comp {}, kind {:x}, side {:x}) original={} rust={}",
            dia0 as u16, ano0 as u16, dia1 as u16, ano1 as u16, club as i32, comp as i32, kind as u8, side as u8, r as u16, n as u16));
        return r;
    }
    let (conta, cauda) = (read_u16(list), read_u32(list + 6));
    let r = o(this, dia0, ano0, dia1, ano1, club, list, comp, kind, side);
    let (n, iguais) = run_with_temp_list(list, conta, cauda,
        |p| find_fixtures(this, dia0, ano0, dia1, ano1, club, p, comp, kind, side));
    SCOREBOARD.record(iguais && r as u16 == n as u16, || format!("find_fixtures({}/{} a {}/{}, club {}, comp {}, kind {:x}, side {:x}, with list) original={} rust={} items equal {}",
        dia0 as u16, ano0 as u16, dia1 as u16, ano1 as u16, club as i32, comp as i32, kind as u8, side as u8, r as u16, n as u16, iguais));
    r
}

/// Comparer for `collect_comp_fixtures_on_day`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_collect_comp_fixtures_on_day(this: usize, year: u32, day: u32, slice: u32, comp: u32, round: u32,
                                                                kind: u32, list: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, usize) -> u32>(IDX_COLLECT_COMP_FIXTURES_ON_DAY);
    let (conta, cauda) = (read_u16(list), read_u32(list + 6));
    let r = o(this, year, day, slice, comp, round, kind, list);
    let (n, iguais) = run_with_temp_list(list, conta, cauda,
        |p| collect_comp_fixtures_on_day(this, year, day, slice, comp, round, kind, p));
    SCOREBOARD.record(iguais && r as u16 == n as u16, || format!("collect_comp_fixtures_on_day({}/{} slice {}, comp {}, round {}, kind {:x}) original={} rust={} items equal {}",
        day as u16, year as u16, slice as u8, comp as i32, round as i32, kind as u8, r as u16, n as u16, iguais));
    r
}

/// Comparer for `fix_man_comp_has_fixture_on_date`.
pub(crate) unsafe extern "stdcall" fn compare_fix_man_comp_has_fixture_on_date(data: usize, comp: u32) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, u32) -> u8>(IDX_FIX_MAN_COMP_HAS_FIXTURE_ON_DATE)(data, comp);
    let n = fix_man_comp_has_fixture_on_date(data, comp);
    SCOREBOARD.record(r == n, || format!("fix_man_comp_has_fixture_on_date(date {:x}, comp {}) original={} rust={}", data, comp as i32, r, n));
    r
}

/// Comparer for `find_next_fixture`.
pub(crate) unsafe extern "thiscall" fn compare_find_next_fixture(this: usize, pano: *mut u16, pdia: *mut u16, pfatia: *mut u8, adiante: u32,
    comp: i32, stage: i32, club: i32, club_b: i32, a9: u32, a10: u32, a11: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut u16, *mut u16, *mut u8, u32, i32, i32, i32, i32, u32, u32, u32) -> usize>(IDX_FIND_NEXT_FIXTURE);
    let before = (*pano, *pdia, *pfatia);
    let reg = [(pano as usize, 2), (pdia as usize, 2), (pfatia as usize, 1)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg,
        || o(this, pano, pdia, pfatia, adiante, comp, stage, club, club_b, a9, a10, a11),
        || find_next_fixture(this, pano, pdia, pfatia, adiante, comp, stage, club, club_b, a9, a10, a11));
    SCOREBOARD.record(igual && r == n, || format!("find_next_fixture(cursor {:?}, adiante {}, comp {}, stage {}, clubs {:x} {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        before, adiante, comp, stage, club, club_b, r, n, mo, mr));
    r
}

/// Comparer for `get_time_slice`.
pub(crate) unsafe extern "thiscall" fn compare_get_time_slice(this: usize, comp: usize, day: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(IDX_GET_TIME_SLICE)(this, comp, day);
    let n = get_time_slice(this, comp, day);
    SCOREBOARD.record(r == n, || format!("get_time_slice(day {}) original={} rust={}", day as i16, r, n));
    r
}

/// Comparer for `add_away_trip_illness_risk`.
pub(crate) unsafe extern "thiscall" fn compare_add_away_trip_illness_risk(this: usize, f: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_AWAY_TRIP_ILLNESS_RISK)(this, f)
}

/// Comparer for `fix_man_get_partner_key_nation_index`.
pub(crate) unsafe extern "C" fn compare_fix_man_get_partner_key_nation_index(club: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_FIX_MAN_GET_PARTNER_KEY_NATION_INDEX)(club);
    let n = fix_man_get_partner_key_nation_index(club);
    SCOREBOARD.record(r == n, || format!("fix_man_get_partner_key_nation_index({:x}) original={:x} rust={:x}", club, r, n));
    r
}

/// Comparer for `get_fixture_list_for_date`.
pub(crate) unsafe extern "stdcall" fn compare_get_fixture_list_for_date(data: usize, slice: u32, group: u32) -> usize {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, u32, u32) -> usize>(IDX_GET_FIXTURE_LIST_FOR_DATE)(data, slice, group);
    let n = get_fixture_list_for_date(data, slice, group);
    SCOREBOARD.record(r == n, || format!("get_fixture_list_for_date({:x}, slice {}, group {}) original={:x} rust={:x}", data, slice, group, r, n));
    r
}

/// Comparer for `mark_calendar_year`.
pub(crate) unsafe extern "C" fn compare_mark_calendar_year(game: usize) {
    let o = original_fn::<unsafe extern "C" fn(usize)>(IDX_MARK_CALENDAR_YEAR);
    let table = read_usize(G_CLUB_FIXTURE_YEAR_TABLE);
    let reg: Vec<(usize, usize)> = if table == 0 {
        vec![]
    } else {
        [0xc, 0x10].iter().map(|&o| (table.wrapping_add((read_i32(game + o) as usize).wrapping_mul(2)), 2)).collect()
    };
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(game), || mark_calendar_year(game));
    SCOREBOARD.record(igual, || format!("mark_calendar_year({:#x}) {:x?} x {:x?}", game, mo, mr));
}
