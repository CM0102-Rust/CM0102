// ---------- batch %%LEVA%%: the screen manager's manager records: a channel's keys, the away and done flags by key, the idle channels, a manager's channel, swapping and switching managers and ending the network session (scrman.cpp) ----------

/// The record `k` of channel `c` (16 records of `screen_history::RECORD_SIZE` per channel; offsets from the manager,
/// as `screen_history`).
fn scrman_channel_record(sm: usize, c: i16, k: i16) -> usize {
    (sm as isize + (c as isize * 16 + k as isize) * screen_history::RECORD_SIZE as isize) as usize
}

/// The record count of channel `c` (`screen_history::CHANNEL_RECORD_COUNTS`, a signed word).
unsafe fn scrman_channel_count(sm: usize, c: i16) -> i16 {
    read_i16((sm as isize + screen_history::CHANNEL_RECORD_COUNTS as isize + c as isize * 2) as usize)
}

/// The first record of any channel whose manager key (`screen_history::KEY`) is `key`: channels 0..15, records in
/// order; none: `None`.
unsafe fn scrman_find_record(sm: usize, key: u32) -> Option<usize> {
    for c in 0..16i16 {
        let n = scrman_channel_count(sm, c);
        for k in 0..n.max(0) {
            let rec = scrman_channel_record(sm, c, k);
            if read_u32(rec + screen_history::KEY) == key {
                return Some(rec);
            }
        }
    }
    None
}

/// Rule: the manager keys of a channel's records, and how many there are.
///
/// The record count of `channel` (its low word, signed) is written to the word at `count_out`; then, while the index is
/// below that word (read again each time), the key (`screen_history::KEY`) of each record goes to the dword array at
/// `keys_out`. Returns how many keys were written (0 with no records).
/// Original: `ScreenManager::get_channel_manager_keys` (0x7ffd80, scrman.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn screen_manager_get_channel_manager_keys(sm: usize, keys_out: usize, count_out: usize, channel: u32) -> u32 {
    let c = channel as i16;
    let n = scrman_channel_count(sm, c);
    write_i16(count_out, n);
    if n <= 0 {
        return 0;
    }
    let mut k: u32 = 0;
    loop {
        write_u32(keys_out + 4 * k as usize, read_u32(scrman_channel_record(sm, c, k as i16) + screen_history::KEY));
        k += 1;
        if k as i16 >= read_i16(count_out) {
            return k;
        }
    }
}

/// Comparer for `screen_manager_get_channel_manager_keys`: side by side over the count and the keys written, and the
/// return.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_get_channel_manager_keys(sm: usize, keys_out: usize, count_out: usize, channel: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(%%I0%%);
    let n = scrman_channel_count(sm, channel as i16).clamp(0, 16) as usize;
    let regions = [(count_out, 2), (keys_out, 4 * n)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side_light(&regions[..if n > 0 { 2 } else { 1 }], || o(sm, keys_out, count_out, channel),
        || screen_manager_get_channel_manager_keys(sm, keys_out, count_out, channel));
    SCOREBOARD.record(equal && ro == rr, || format!("screen_manager_get_channel_manager_keys({}): return {} x {}, differences in {:?}", channel as i16, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Rule: whether the manager with this key is away; a manager with no record counts as away.
///
/// The away flag (`screen_history::AWAY`, the whole dword) of the first record with the key (`scrman_find_record`), or
/// 1 when there is none.
/// Original: `ScreenManager::is_manager_away` (0x7ffdd0, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_is_manager_away(sm: usize, key: u32) -> u32 {
    match scrman_find_record(sm, key) {
        Some(rec) => read_u32(rec + screen_history::AWAY),
        None => 1, // tel38_2: no record counts as away
    }
}

compare_value!(compare_screen_manager_is_manager_away, screen_manager_is_manager_away, %%I1%%, (key: u32) -> u32);

/// Rule: whether the manager with this key is done for the day; a manager with no record counts as done.
///
/// The done flag (`screen_history::DONE`, the whole dword) of the first record with the key, or 1 when there is none.
/// Original: `ScreenManager::is_manager_done` (0x7ffe40, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_is_manager_done(sm: usize, key: u32) -> u32 {
    match scrman_find_record(sm, key) {
        Some(rec) => read_u32(rec + screen_history::DONE),
        None => 1, // tel38_2: no record counts as done
    }
}

compare_value!(compare_screen_manager_is_manager_done, screen_manager_is_manager_done, %%I2%%, (key: u32) -> u32);

/// Rule: marks the manager with this key as done (or not) for the day.
///
/// The done flag (`screen_history::DONE`) of the first record with the key gets `done`; nothing when there is none.
/// Original: `ScreenManager::set_manager_done` (0x7ffeb0, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_manager_done(sm: usize, key: u32, done: u32) {
    if let Some(rec) = scrman_find_record(sm, key) {
        covered!("tel38_2 manager done set");
        write_u32(rec + screen_history::DONE, done);
    }
}

/// Comparer for `screen_manager_set_manager_done`: side by side over the 16 channels of records.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_manager_done(sm: usize, key: u32, done: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I3%%);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(sm + screen_history::NETWORK_PLAYER, 16 * screen_history::CHANNEL_SIZE)],
        || o(sm, key, done), || screen_manager_set_manager_done(sm, key, done));
    SCOREBOARD.record(equal, || format!("screen_manager_set_manager_done({:#x}, {}): differences in {:?}", key, done, first_differences(&mo, &mr)));
}

/// Rule: whether a network player is done: 0 as soon as one channel with records whose first record is his
/// (`screen_history::NETWORK_PLAYER`, the low word of `player`) is not done (`DONE` = 0); otherwise 1.
/// Original: `ScreenManager::is_player_done` (0x7fff20, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_is_player_done(sm: usize, player: u32) -> u32 {
    for c in 0..16i16 {
        if scrman_channel_count(sm, c) > 0 {
            let rec = scrman_channel_record(sm, c, 0);
            if read_u16(rec + screen_history::NETWORK_PLAYER) == player as u16 && read_u32(rec + screen_history::DONE) == 0 {
                return 0;
            }
        }
    }
    1
}

compare_value!(compare_screen_manager_is_player_done, screen_manager_is_player_done, %%I4%%, (player: u32) -> u32);

/// Rule: how many channels are idle: with no records, or whose first record is away (`screen_history::AWAY` ≠ 0).
/// Original: `ScreenManager::count_idle_channels` (0x7fff70, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_count_idle_channels(sm: usize) -> u32 {
    (0..16i16).filter(|&c| scrman_channel_count(sm, c) <= 0 || read_u32(scrman_channel_record(sm, c, 0) + screen_history::AWAY) != 0).count() as u32
}

compare_value!(compare_screen_manager_count_idle_channels, screen_manager_count_idle_channels, %%I5%%, () -> u32);

/// Rule: the channel of the manager with this key, or −1 when there is none or the key is 0.
///
/// Channels 0..15, records in order: the first channel holding a record with the key (`screen_history::KEY`); −1
/// otherwise and for key 0. A short (the original leaves the caller's high word in eax for key 0).
/// Original: `ScreenManager::find_manager_channel` (0x7fffa0, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_find_manager_channel(sm: usize, key: u32) -> i16 {
    if key == 0 {
        return -1;
    }
    for c in 0..16i16 {
        for k in 0..scrman_channel_count(sm, c).max(0) {
            if read_u32(scrman_channel_record(sm, c, k) + screen_history::KEY) == key {
                return c;
            }
        }
    }
    -1
}

compare_value!(compare_screen_manager_find_manager_channel, screen_manager_find_manager_channel, %%I6%%, (key: u32) -> i16);

/// Rule: two managers of the current channel swap their keys.
///
/// In the current channel (`screen_manager::CURRENT_SLOT`), records 0..count−1 (the count read once): the last one
/// with key `a` and the last one with key `b`. When both exist, their keys (`screen_history::KEY`) are swapped and the
/// result is 1; otherwise 0 and nothing changes.
/// Original: `ScreenManager::swap_manager_keys` (0x7ff810, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_swap_manager_keys(sm: usize, a: u32, b: u32) -> u32 {
    let c = read_i16(sm + screen_manager::CURRENT_SLOT);
    let n = scrman_channel_count(sm, c);
    let (mut ia, mut ib) = (-1i16, -1i16);
    for k in 0..n.max(0) {
        let key = read_u32(scrman_channel_record(sm, c, k) + screen_history::KEY);
        if key == a {
            ia = k;
        }
        if key == b {
            ib = k;
        }
    }
    if ia <= -1 || ib <= -1 {
        return 0;
    }
    covered!("tel38_2 manager keys swapped");
    let ra = scrman_channel_record(sm, c, ia) + screen_history::KEY;
    let rb = scrman_channel_record(sm, c, ib) + screen_history::KEY;
    let key_a = read_u32(ra);
    write_u32(ra, read_u32(rb)); // tel38_2: the keys swapped
    write_u32(scrman_channel_record(sm, read_i16(sm + screen_manager::CURRENT_SLOT), ib) + screen_history::KEY, key_a);
    1
}

/// Comparer for `screen_manager_swap_manager_keys`: side by side over the current channel and the return.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_swap_manager_keys(sm: usize, a: u32, b: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I7%%);
    let channel = screen_record(sm) + screen_history::NETWORK_PLAYER;
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&[(channel, screen_history::CHANNEL_SIZE)], || o(sm, a, b),
        || screen_manager_swap_manager_keys(sm, a, b));
    SCOREBOARD.record(equal && ro == rr, || format!("screen_manager_swap_manager_keys({:#x}, {:#x}): return {} x {}, differences in {:?}", a, b, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Rule: the manager with this key becomes the current one of his channel, when he is neither away nor done.
///
/// In the current channel, the first record with the key (`screen_history::KEY`); none, or the record away (`AWAY`) or
/// done (`DONE`): nothing. Otherwise, when it is not record 0, it swaps places with record 0 (`exe::utils_swap_bytes`,
/// 0x30 bytes), and record 0's `ACTIVE_FLAG` = 1.
/// Original: `ScreenManager::switch_to_manager` (0x7ffcd0, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_switch_to_manager(sm: usize, key: u32) {
    use screen_history as h;
    let c = read_i16(sm + screen_manager::CURRENT_SLOT);
    let n = scrman_channel_count(sm, c);
    let mut k = 0i16;
    while k < n && read_u32(scrman_channel_record(sm, c, k) + h::KEY) != key {
        k += 1;
    }
    if k >= n {
        return;
    }
    let rec = scrman_channel_record(sm, c, k);
    if read_u32(rec + h::AWAY) != 0 || read_u32(rec + h::DONE) != 0 { // tel38_2: away or done managers stay put
        return;
    }
    if k != 0 {
        covered!("tel38_2 manager switched in");
        let first = scrman_channel_record(sm, c, 0) + h::NETWORK_PLAYER;
        exe::utils_swap_bytes((rec + h::NETWORK_PLAYER) as *mut i32, first as *mut i32, h::RECORD_SIZE as u32);
    }
    write_u32(screen_record(sm) + h::ACTIVE_FLAG, 1);
}

/// Comparer for `screen_manager_switch_to_manager`: side by side over the current channel.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_switch_to_manager(sm: usize, key: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I8%%);
    let channel = screen_record(sm) + screen_history::NETWORK_PLAYER;
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(channel, screen_history::CHANNEL_SIZE)], || o(sm, key),
        || screen_manager_switch_to_manager(sm, key));
    SCOREBOARD.record(equal, || format!("screen_manager_switch_to_manager({:#x}): differences in {:?}", key, first_differences(&mo, &mr)));
}

/// Rule: ends the network session, when there is one.
///
/// With a session (`screen_history::NETWORK`): 0x7fc690 (thiscall on the manager, the session's farewell [inference]),
/// then, when the session is still there, its destructor (0x77a890, thiscall) and operator delete; the session pointer
/// goes to 0.
/// Original: `ScreenManager::end_network_session` (0x7ff5f0, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_end_network_session(sm: usize) {
    use screen_history as h;
    if read_usize(sm + h::NETWORK) == 0 {
        return;
    }
    covered!("tel38_2 network session ended");
    let farewell: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7fc690usize);
    farewell(sm);
    let net = read_usize(sm + h::NETWORK);
    if net != 0 {
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x77a890usize);
        destroy(net);
        exe::crt_operator_delete(net);
    }
    write_usize(sm + h::NETWORK, 0);
}

// simplification: closes and frees the session: it does not run twice. Original only; the plan: no session left.
/// Comparer for `screen_manager_end_network_session`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_end_network_session(sm: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I9%%)(sm);
    let net = read_usize(sm + screen_history::NETWORK);
    SCOREBOARD.record(net == 0, || format!("screen_manager_end_network_session: session {:#x} left", net));
}
