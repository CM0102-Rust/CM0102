use super::*;

// The comparers of the routines in ../scout_manager.rs.

/// Comparer for `scout_manager_daily_update`.
///
/// Comparer of `scout_manager_daily_update`: the reports draw at random and allocate and the searches change the human players' lists, so
/// only the original routine runs.
///
/// Beforehand, the copy in rehearsal mode prunes a copy of the requests (without the reports) and puts them in the
/// original's order; the requests the original leaves (count and the 0x11 bytes of each) must be those.
pub(crate) unsafe extern "thiscall" fn compare_scout_manager_daily_update(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCOUT_MANAGER_DAILY_UPDATE);
    if read_u8(G_CURRENT_DAY_SLICE) != 2 {
        return o(this);
    }
    let n0 = read_i32(this + scout_manager_fields::SCOUT_REQUEST_COUNT).max(0) as usize;
    let base = read_usize(this + scout_manager_fields::SCOUT_REQUESTS);
    let mut copy: Vec<u8> = (0..n0 * 0x11).map(|k| read_u8(base + k)).collect();
    copy.push(0);
    // the trial run prunes the copy: swaps the vector and the count of `this` for the copy’s and restores them afterwards
    let (b0, c0) = (read_usize(this + scout_manager_fields::SCOUT_REQUESTS), read_i32(this + scout_manager_fields::SCOUT_REQUEST_COUNT));
    write_usize(this + scout_manager_fields::SCOUT_REQUESTS, copy.as_mut_ptr() as usize);
    let state = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let a1080 = read_usize(G_CONTRACT_LOOKUP_RESULT);
    SCOUT_DAILY_DRY_RUN.store(true, Relaxed);
    crate::rng::without_counting(|| scout_manager_daily_update(this));
    SCOUT_DAILY_DRY_RUN.store(false, Relaxed);
    state.write_back();
    write_usize(G_CONTRACT_LOOKUP_RESULT, a1080);
    let n1 = read_i32(this + scout_manager_fields::SCOUT_REQUEST_COUNT).max(0) as usize;
    exe::crt_qsort(copy.as_mut_ptr() as usize, (n1 as i32) as u32, 0x11, 0x7fc270);
    let plan: Vec<u8> = copy[..n1 * 0x11].to_vec();
    write_usize(this + scout_manager_fields::SCOUT_REQUESTS, b0);
    write_i32(this + scout_manager_fields::SCOUT_REQUEST_COUNT, c0);
    let r = o(this);
    let n2 = read_i32(this + scout_manager_fields::SCOUT_REQUEST_COUNT).max(0) as usize;
    let now: Vec<u8> = (0..n2 * 0x11).map(|k| read_u8(read_usize(this + scout_manager_fields::SCOUT_REQUESTS) + k)).collect();
    SCOREBOARD.record(r == 1 && now == plan, || format!("scout_manager_daily_update: pedidos {} -> original {} dry run {}; {:x?} x {:x?}", n0, n2, n1, now, plan));
    r
}

/// Comparer for `send_scout_reports`.
///
/// Comparer of `send_scout_reports`: the original routine delivers news items (allocating in the inboxes), so only it runs, with
/// logging of the calls to `send_scout_report_news` turned on; beforehand, the copy as a dry run (without `send_scout_report_news`) gives the groups
/// (player, club, how many) in order, and the original's calls must be those.
pub(crate) unsafe extern "thiscall" fn compare_send_scout_reports(this: usize, list: usize) {
    let list = Llist(list);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SEND_SCOUT_REPORTS);
    let cursor = list.cursor();
    *core::ptr::addr_of_mut!(SCOUT_REPORT_GROUPS) = Some(Vec::new());
    SCOUT_DAILY_DRY_RUN.store(true, Relaxed);
    send_scout_reports(this, list.addr());
    SCOUT_DAILY_DRY_RUN.store(false, Relaxed);
    let plan = (*core::ptr::addr_of_mut!(SCOUT_REPORT_GROUPS)).replace(Vec::new()).unwrap_or_default();
    list.set_cursor(cursor);
    o(this, list.addr());
    let done = (*core::ptr::addr_of_mut!(SCOUT_REPORT_GROUPS)).take().unwrap_or_default();
    SCOREBOARD.record(done == plan, || format!("send_scout_reports(list {:#x}) original {:x?} dry run {:x?}", list.addr(), done, plan));
}

/// Comparer for `send_scout_report_news`.
pub(crate) unsafe extern "thiscall" fn compare_send_scout_report_news(this: usize, r: usize, how_many: u32) {
    if let Some(g) = &mut *core::ptr::addr_of_mut!(SCOUT_REPORT_GROUPS) {
        g.push((read_usize(r + scout_report::PLAYER_ID) as u32, read_usize(r + scout_report::CLUB_ID) as u32, how_many as u8));
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_SEND_SCOUT_REPORT_NEWS)(this, r, how_many);
}

/// Comparer for `current_search_has_no_club`.
pub(crate) unsafe extern "thiscall" fn compare_current_search_has_no_club(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CURRENT_SEARCH_HAS_NO_CLUB);
    if matches!(current_human_index(), Ok(None)) {
        return o(this);
    }
    let r = o(this);
    let n = current_search_has_no_club(this);
    SCOREBOARD.record(r == n, || format!("current_search_has_no_club() original={} rust={}", r, n));
    r
}

/// Comparer for `is_club_in_session_searches`.
pub(crate) unsafe extern "thiscall" fn compare_is_club_in_session_searches(this: usize, club: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_IS_CLUB_IN_SESSION_SEARCHES);
    if matches!(current_human_index(), Ok(None)) {
        return o(this, club);
    }
    let r = o(this, club);
    let n = is_club_in_session_searches(this, club);
    SCOREBOARD.record(r == n, || format!("is_club_in_session_searches(club {:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `scout_manager_scout_match`.
///
/// The scouting (`player_search_scout_match`, original) writes what the scouts know and the reports, `exe::player_search_start_scout_assignment` starts searches and `send_scout_reports`
/// delivers the news: it cannot run twice. Only the original runs (simplification: the routine itself only writes the
/// local array and list, which are discarded); it checks the return value (1).
pub(crate) unsafe extern "thiscall" fn compare_scout_manager_scout_match(this: usize, p: usize, result: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SCOUT_MANAGER_SCOUT_MATCH)(this, p, result);
    SCOREBOARD.record(r == 1, || format!("scout_manager_scout_match(match {:x}) original={}", p, r));
    r
}

/// Comparer for `get_edit_session`.
pub(crate) unsafe extern "thiscall" fn compare_get_edit_session(this: usize, manager: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_EDIT_SESSION);
    if scout_manager_human_manager_index(manager).is_none() {
        return o(this, manager);
    }
    let r = o(this, manager);
    let n = get_edit_session(this, manager);
    SCOREBOARD.record(r == n, || format!("get_edit_session(manager {:x}) original={:x} rust={:x}", manager, r, n));
    r
}

/// Comparer for `set_edit_session`.
pub(crate) unsafe extern "thiscall" fn compare_set_edit_session(this: usize, sessao: usize, manager: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SET_EDIT_SESSION);
    if scout_manager_human_manager_index(manager).is_none() {
        return o(this, sessao, manager);
    }
    let reg = [(this + scout_manager_fields::EDIT_SESSIONS, 0x40)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, sessao, manager),
        || set_edit_session(this, sessao, manager));
    SCOREBOARD.record(igual && r == n, || format!("set_edit_session(sessao {:x}, manager {:x}) original={} rust={} {:x?} x {:x?}",
        sessao, manager, r, n, mo, mr));
    r
}

/// Comparer for `get_selected_list`.
pub(crate) unsafe extern "thiscall" fn compare_get_selected_list(this: usize, manager: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_GET_SELECTED_LIST);
    if manager_human_index(manager).is_none() {
        return o(this, manager);
    }
    let r = o(this, manager);
    let n = get_selected_list(this, manager);
    SCOREBOARD.record(r == n, || format!("get_selected_list(manager {:x}) original={} rust={}", manager, r, n));
    r
}

/// Comparer for `set_selected_list`.
pub(crate) unsafe extern "thiscall" fn compare_set_selected_list(this: usize, list: u32, manager: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> u32>(IDX_SET_SELECTED_LIST);
    if manager_human_index(manager).is_none() {
        return o(this, list, manager);
    }
    let reg = [(this + scout_manager_fields::SELECTED_LISTS, 0x10)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, list, manager), || set_selected_list(this, list, manager));
    SCOREBOARD.record(igual && r == n, || format!("set_selected_list(list {}, manager {:x}) original={} rust={} {:x?} x {:x?}", list as u8, manager, r, n, mo, mr));
    r
}

/// Comparer for `fixture_has_scouted_player`.
pub(crate) unsafe extern "thiscall" fn compare_fixture_has_scouted_player(this: usize, fixture: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_FIXTURE_HAS_SCOUTED_PLAYER)(this, fixture);
    let n = fixture_has_scouted_player(this, fixture);
    SCOREBOARD.record(r == n, || format!("fixture_has_scouted_player(match {:x}) original={} rust={}", fixture, r, n));
    r
}

/// Comparer for `is_player_shortlisted`.
pub(crate) unsafe extern "thiscall" fn compare_is_player_shortlisted(this: usize, person: usize, manager: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_IS_PLAYER_SHORTLISTED);
    if person == 0 || (manager == 0 && scout_manager_current_human_manager() == 0) {
        return o(this, person, manager);
    }
    let r = o(this, person, manager);
    let n = is_player_shortlisted(this, person, manager);
    SCOREBOARD.record(r == n, || format!("is_player_shortlisted(person {:x}, manager {:x}) original={} rust={}", person, manager, r, n));
    r
}

/// Comparer for `scout_record`.
pub(crate) unsafe extern "thiscall" fn compare_scout_record(t: usize, which: i8, manager: *const i32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8, *const i32) -> usize>(IDX_SCOUT_RECORD)(t, which, manager);
    let n = scout_record(t, which, manager);
    SCOREBOARD.record(r == n, || format!("scout_record({:#x}, {}, {:p}) original={:#x} rust={:#x}", t, which, manager, r, n));
    r
}

/// Comparer for `collect_club_scout_targets`.
pub(crate) unsafe extern "thiscall" fn compare_collect_club_scout_targets(this: usize, club: *const u32, list: usize) -> u32 {
    let alvos = if club.is_null() { Vec::new() } else { scouting_targets_of_club(this, *club) };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const u32, usize) -> u32>(IDX_COLLECT_CLUB_SCOUT_TARGETS)(this, club, list);
    let mut same = r == (!alvos.is_empty()) as u32;
    if list != 0 {
        let items = list_items_capped(list);
        same &= items.len() == alvos.len()
            && items.iter().zip(&alvos).all(|(&i, &p)| core::slice::from_raw_parts(i as *const u8, 0x6e) == core::slice::from_raw_parts(p as *const u8, 0x6e));
    }
    SCOREBOARD.record(same, || format!("collect_club_scout_targets original={} alvos {}", r, alvos.len()));
    r
}

/// Comparer for `remove_scout_request`.
pub(crate) unsafe extern "thiscall" fn compare_remove_scout_request(this: usize, person: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_REMOVE_SCOUT_REQUEST);
    let n = read_i32(this + scout_manager_fields::SCOUT_REQUEST_COUNT).max(0) as usize;
    let reg = [(this + scout_manager_fields::SCOUT_REQUESTS, 0xc), (read_u32(this + scout_manager_fields::SCOUT_REQUESTS) as usize, n * 17)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, person, club), || remove_scout_request(this, person, club));
    SCOREBOARD.record(igual, || format!("remove_scout_request({:x}, {:x}) {:x?} x {:x?}", person, club, mo, mr));
}

/// Comparer for `can_club_scout_nation`.
pub(crate) unsafe extern "thiscall" fn compare_can_club_scout_nation(this: usize, nation: usize, club: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_CAN_CLUB_SCOUT_NATION)(this, nation, club);
    if club_or_current_human(club) == 0 {
        return r;
    }
    let n = can_club_scout_nation(this, nation, club);
    SCOREBOARD.record(r == n, || format!("can_club_scout_nation({:x}, {:x}) original={} rust={}", nation, club, r, n));
    r
}

/// Comparer for `sync_scout_lists`.
pub(crate) unsafe extern "thiscall" fn compare_sync_scout_lists(this: usize, manager: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SYNC_SCOUT_LISTS);
    let m = if manager != 0 { manager } else { scout_manager_current_human_manager() };
    if m == 0 {
        return o(this, manager);
    }
    let slot = read_u8(m).wrapping_sub(read_u8(G_STAFF_COUNT)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&slot) || read_usize(this + slot as usize * 4 + scout_manager_fields::EDIT_SESSIONS) != 0 {
        return o(this, manager);
    }
    let mut regions = Vec::new();
    for i in 0..10u8 {
        let l = get_search(this, i, Staff(m));
        if l != 0 {
            regions.push((l, 0x2b0usize));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, manager), || sync_scout_lists(this, manager));
    SCOREBOARD.record(igual && r == n, || format!("sync_scout_lists(manager {:x}) original={} rust={} {:x?} x {:x?}", m, r, n, mo, mr));
    r
}

/// Comparer for `find_club_search_entry`.
pub(crate) unsafe extern "thiscall" fn compare_find_club_search_entry(this: usize, p: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FIND_CLUB_SEARCH_ENTRY);
    let r = o(this, p);
    let n = find_club_search_entry(this, p);
    SCOREBOARD.record(r == n, || format!("find_club_search_entry(person {:x}) original={:x} rust={:x}", p, r, n));
    r
}

/// Comparer for `scout_manager_write_list_name`.
pub(crate) unsafe extern "C" fn compare_scout_manager_write_list_name(index: u32, text: usize, size: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, i32) -> u32>(IDX_SCOUT_MANAGER_WRITE_LIST_NAME);
    let k = index as u8 as i8;
    if !(0..=9).contains(&k) || text == 0 || size < 0x32 {
        return o(index, text, size);
    }
    let reg = [(text, (size as usize).min(0x100))];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(index, text, size), || scout_manager_write_list_name(index, text, size));
    SCOREBOARD.record(igual && r == n, || format!("scout_manager_write_list_name(index {}) original={} rust={} {:x?} x {:x?}", k, r, n, mo, mr));
    r
}

/// Comparer for `has_scout_request`.
pub(crate) unsafe extern "thiscall" fn compare_has_scout_request(this: usize, player: usize, club: usize, so_vencidos: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_HAS_SCOUT_REQUEST);
    if player == 0 {
        return o(this, player, club, so_vencidos);
    }
    let r = o(this, player, club, so_vencidos);
    let n = has_scout_request(this, player, club, so_vencidos);
    SCOREBOARD.record(r == n, || format!("has_scout_request(player {:x}, club {:x}, {}) original={} rust={}", player, club, so_vencidos, r, n));
    r
}
