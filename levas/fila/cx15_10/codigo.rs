// ---------- cx15_10: the African World Cup qualifiers: the entrants and the first round entries (wc_african_cup.cpp) ----------

/// The African qualifiers' fields beyond the Cup body: the exempt count (`wc_cup_turnover_consts::AFRICAN_FORMAT`, the
/// hosts and the holder that are African), the entrant list (`wc_cup_turnover_consts::CUP_ENTRANT_LIST`, 52 national
/// team pointers) and the number of places in it (+0xb6, a dword).
const AFRICAN_PLACES: usize = 0xb6;

/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\worldcup\wc_african_cup.cpp" (the error boxes).
const WC_AFRICAN_CUP_CPP: usize = wc_cup_turnover_consts::WC_AFRICAN_CUP_CPP;

/// Whether a nation record is in Africa (`continent()` with the id of `KEY_CONTINENT_AFRICA`).
unsafe fn is_african(nation: Nation) -> bool {
    let c = nation.continent();
    c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_AFRICA)
}

/// The African World Cup qualifying entrants: 50 places minus the African nations exempt (the hosts of the next World
/// Cup and the last winner when African), filled with the other African national teams in database order and sorted by
/// strength.
///
/// The exempt count (`wc_cup_turnover_consts::AFRICAN_FORMAT`, a byte) = 0. Without [`G_HOST_COUNTRY`]: the error box of
/// line 0x3b7 and no hosts; otherwise the hosts of the World Cup two years later (`exe::hostcountry_find_hosts`: 5
/// pushes; mov ecx -> thiscall, ret 0x14; the year with garbage in the high 16 bits, read as a word); for each (k signed
/// byte below the count) up to the first −1, an African one adds one. The last World Cup winner
/// (`exe::comp_util_get_latest_placed_club`(World Cup record, 0)): its nation, with a continent, African and neither
/// host, adds one. `AFRICAN_PLACES` = 50 − exempt (signed byte) and the list (`exe::crt_malloc(0xd0)`, 52 entries
/// whatever the places; out of memory, the box of line 0x3e6 and 0). Over the nations (i word signed), each African one
/// while the list holds fewer than 52: its national team (`exe::database_get_national_team`), when the nation is neither
/// host and the team is not the winner, goes into the list. Short of the places it returns 0 (unsorted); otherwise the
/// list sorted by strength (`exe::crt_qsort`(list, n, 4, 0x4b07d0)) and 1. The same body as the Asian
/// `wc_asia_league_build_entrant_list`, with a fixed list and no cap.
/// Original: `WcAfricanCup::collect_entrants` (0x9106a0, wc_african_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_collect_entrants(this: usize) -> u32 {
    let cup = Cup(this);
    let exempt = this + wc_cup_turnover_consts::AFRICAN_FORMAT;
    write_u8(exempt, 0);
    let mut hosts = [-1i32; 2];
    if read_usize(G_HOST_COUNTRY) == 0 {
        show_error_at(WC_AFRICAN_CUP_CPP, 0x3b7);
    } else {
        let (a, b) = hosts.split_at_mut(1);
        let n = exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP), (cup.year() as u16).wrapping_add(2) as u32, &mut a[0], &mut b[0], 1) as i8;
        let mut k = 0i8;
        // simplification: the original reads the host slot k of its two locals for any k below the count; find_hosts gives at most two
        while k < n && (k as usize) < hosts.len() {
            let id = hosts[k as usize];
            if id == -1 {
                break;
            }
            if is_african(Nation(read_usize(G_NATION_TABLE).wrapping_add((id as usize).wrapping_mul(nation::SIZE)))) {
                covered!("cx15_10 african host exempt");
                write_u8(exempt, read_u8(exempt).wrapping_add(1));
            }
            k += 1;
        }
    }
    let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b));
    let winner = exe::comp_util_get_latest_placed_club(world_cup, 0);
    if winner != 0 {
        let nation = Club(winner).nation();
        if !nation.is_null() && is_african(nation) {
            let id = read_usize(nation.addr()) as i32;
            if id != hosts[0] && id != hosts[1] {
                write_u8(exempt, read_u8(exempt).wrapping_add(1));
            }
        }
    }
    let places = 0x32 - read_u8(exempt) as i8 as i32;
    write_u32(this + AFRICAN_PLACES, places as u32);
    let list = exe::crt_malloc(0xd0);
    write_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST, list);
    if list == 0 {
        show_error_at(WC_AFRICAN_CUP_CPP, 0x3e6);
        return 0;
    }
    let mut count = 0i32;
    let mut i = 0i16;
    while (i as i32) < read_i32(G_NATION_COUNT) {
        let nation = Nation(read_usize(G_NATION_TABLE).wrapping_add((i as isize as usize).wrapping_mul(nation::SIZE)));
        if is_african(nation) && count < 0x34 {
            let team = exe::database_get_national_team(nation);
            let id = read_usize(nation.addr()) as i32;
            if id != hosts[0] && id != hosts[1] && team != winner {
                write_usize(list + count as usize * 4, team);
                count += 1;
            }
        }
        i = i.wrapping_add(1);
    }
    if count < read_i32(this + AFRICAN_PLACES) {
        return 0;
    }
    covered!("cx15_10 african entrants sorted");
    exe::crt_qsort(list, count as u32, 4, 0x4b07d0);
    1
}

/// The African first round entries: the entry count (`cup::TEAM_COUNT`) by the exempt count
/// (`wc_cup_turnover_consts::AFRICAN_FORMAT`, signed byte): 0 gives 50, below 3 gives 48, otherwise 46. The team table
/// (`cup.team_table()`, 6-byte entries) is allocated for them (`exe::crt_malloc`; out of memory, the error box of line
/// 0x2c5 and −1). Entry k takes the entrant from the end of the sorted list (index `AFRICAN_PLACES` − 1 − k, the
/// weakest first), which is cleared in the list; its +4 byte is 1 for the first half (k below the count / 2, truncated)
/// and 2 for the rest, its +5 byte 0. Returns 1.
/// Original: `WcAfricanCup::build_first_round_entries` (0x910120, wc_african_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_build_first_round_entries(this: usize) -> i32 {
    let cup = Cup(this);
    let exempt = read_i8(this + wc_cup_turnover_consts::AFRICAN_FORMAT);
    let entries: i16 = if exempt == 0 { 0x32 } else if exempt < 3 { 0x30 } else { 0x2e };
    cup.set_team_count(entries);
    let table = exe::crt_malloc((entries as i32 * 6) as u32);
    cup.set_team_table(table);
    if table == 0 {
        show_error_at(WC_AFRICAN_CUP_CPP, 0x2c5);
        return -1;
    }
    covered!("cx15_10 african first round entries");
    let list = this + wc_cup_turnover_consts::CUP_ENTRANT_LIST;
    let mut from = read_i32(this + AFRICAN_PLACES) - 1;
    let mut k = 0i32;
    while k < cup.team_count() as i32 {
        let e = cup.team_table() + 6 * k as usize;
        write_u8(e + 4, if k < cup.team_count() as i32 / 2 { 1 } else { 2 }); // the round the entrant starts in
        let slot = read_usize(list) + 4 * from as usize;
        write_usize(e, read_usize(slot));
        write_u8(e + 5, 0);
        write_usize(slot, 0);
        from -= 1;
        k += 1;
    }
    1
}

/// Comparer for `wc_african_cup_collect_entrants`, side by side by hand: the original first (its list stays), then,
/// with a host object (without one the copy would show the box again), the copy with the same fields, which allocates
/// its own list; the result, the exempt count, the places and the filled part of the lists must be equal; the copy's
/// list is freed and the original's put back.
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_collect_entrants(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    let list_at = this + wc_cup_turnover_consts::CUP_ENTRANT_LIST;
    let original = (read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT), read_u32(this + AFRICAN_PLACES), read_usize(list_at));
    if read_usize(G_HOST_COUNTRY) == 0 || original.2 == 0 {
        return r;
    }
    let c = wc_african_cup_collect_entrants(this);
    let copy = (read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT), read_u32(this + AFRICAN_PLACES), read_usize(list_at));
    let n = if r == 1 { original.1 as usize } else { 0 };
    let same_list = copy.2 != 0 && (0..n.min(0x34)).all(|k| read_usize(original.2 + 4 * k) == read_usize(copy.2 + 4 * k));
    if copy.2 != 0 {
        exe::crt_free(copy.2);
    }
    write_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT, original.0);
    write_u32(this + AFRICAN_PLACES, original.1);
    write_usize(list_at, original.2);
    SCOREBOARD.record(r == c && original.0 == copy.0 && original.1 == copy.1 && same_list, || format!("wc_african_cup_collect_entrants({:#x}): original {} exempt {} places {}, rust {} exempt {} places {}, list {}",
        this, r, original.0, original.1, c, copy.0, copy.1, same_list));
    r
}

/// Comparer for `wc_african_cup_build_first_round_entries`, side by side by hand: the entrant list (which the routine
/// clears) and the entry count saved; the original first (its table stays); the list and the count put back; the copy,
/// which allocates its own table; the result, the count, the tables and the lists after must be equal; the copy's table
/// is freed and the original's state put back.
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_build_first_round_entries(this: usize) -> i32 {
    let cup = Cup(this);
    let list = read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    let places = read_i32(this + AFRICAN_PLACES).clamp(0, 0x34) as usize;
    let snapshot = |p: usize, n: usize| -> Vec<u8> { if p == 0 { Vec::new() } else { core::slice::from_raw_parts(p as *const u8, n).to_vec() } };
    let list_before = snapshot(list, 4 * places);
    let count_before = cup.team_count();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%)(this);
    let (table_o, count_o) = (cup.team_table(), cup.team_count());
    if r != 1 || list == 0 || table_o == 0 {
        return r;
    }
    let (rows_o, list_o) = (snapshot(table_o, 6 * count_o.max(0) as usize), snapshot(list, 4 * places));
    core::ptr::copy_nonoverlapping(list_before.as_ptr(), list as *mut u8, list_before.len());
    cup.set_team_count(count_before);
    let c = wc_african_cup_build_first_round_entries(this);
    let (table_c, count_c) = (cup.team_table(), cup.team_count());
    let same = c == r && count_c == count_o && table_c != 0 && snapshot(table_c, 6 * count_c.max(0) as usize) == rows_o && snapshot(list, 4 * places) == list_o;
    if table_c != 0 {
        exe::crt_free(table_c);
    }
    core::ptr::copy_nonoverlapping(list_o.as_ptr(), list as *mut u8, list_o.len());
    cup.set_team_table(table_o);
    cup.set_team_count(count_o);
    SCOREBOARD.record(same, || format!("wc_african_cup_build_first_round_entries({:#x}): original {} with {} entries, rust {} with {}", this, r, count_o, c, count_c));
    r
}
