// ---------- batch %%LEVA%%: the screen manager's life: building and destroying it, closing every channel's screens, leaving the network session and removing a manager's record (network.h) ----------

/// The screen manager's command buffer size (`NetworkBuffer::NetworkBuffer` at `screen_history::NET_BUFFER`).
const SCRMAN_NET_BUFFER_SIZE: u32 = 0xc350;
/// The host flag of the network session, from the manager (`G_SCREEN_MANAGER_NETWORK_HOST_FLAG` − `G_SCREEN_MANAGER`).
const SCRMAN_HOST_FLAG: usize = 0x3830;
/// The quit command of the network packets (a manager or the host leaves).
const PACKET_QUIT: u8 = 4;
/// `Display::Display` (thiscall(d, channel) ret 4) and `Display::~Display` (thiscall), both Rust already.
const FN_DISPLAY_BUILD: usize = 0x55ef50;
const FN_DISPLAY_DESTROY: usize = 0x55efe0;
/// `display_free_guio_snapshot_pool` (cdecl) and `display_clear_guio_snapshot` (stdcall(channel); the original calls it
/// with the record's Display in ecx, unused).
const FN_DISPLAY_FREE_SNAPSHOT_POOL: usize = 0x55f270;
const FN_DISPLAY_CLEAR_SNAPSHOT: usize = 0x55f290;
/// `Network::release_slot_socket` (thiscall(session, player) ret 4).
const FN_NETWORK_RELEASE_SLOT_SOCKET: usize = 0x77ac00;

/// The fields the constructor sets besides its buffer, its Displays, the channel counts and sound flags and record 0:
/// (offset from the manager, size, value), in the original's order. Raw offsets have no name yet.
const SCREEN_MANAGER_INITIAL_FIELDS: [(usize, u32, u32); 17] = [
    (screen_history::NETWORK, 4, 0),
    (0x383c, 4, 0), // the dword after the session's neighbour [no name]
    (SCRMAN_JOIN_HOOK, 4, 0),
    (SCRMAN_LEAVE_HOOK, 4, 0),
    (SCRMAN_HOST_FLAG, 4, 0),
    (0x262506, 4, 0), // the dword before the sound flags [no name]
    (0x26278a, 4, 0),
    (0x37fa, 4, 0), // the dword before the command buffer [no name]
    (0x262798, 4, 0),
    (0x26279c, 2, 0),
    (0x26279e, 2, 0),
    (screen_manager::CURRENT_SLOT, 2, 0),
    (SCRMAN_NEXT_CHANNEL, 2, 0xffff),
    (0x3834, 4, 0),
    (0x262790, 4, 0),
    (0x262794, 4, 0),
    (0x26278e, 2, 0), // the injected action id (club_screens.rs)
];

/// Record 0 of channel 0 and the default screen as the constructor leaves them (after the record's Display pointer):
/// one record in channel 0, local, with no key and no screen, active, no screen key; the default screen's five
/// functions, slot values and count cleared.
const SCREEN_MANAGER_FIRST_RECORD: [(usize, u32, u32); 21] = [
    (screen_history::CHANNEL_RECORD_COUNTS, 2, 1),
    (screen_history::NETWORK_PLAYER, 2, 0),
    (screen_history::KEY, 4, 0),
    (screen_history::FIRST_NODE, 4, 0),
    (screen_history::LAST_NODE, 4, 0),
    (screen_history::CURRENT_NODE, 4, 0),
    (screen_history::FLAGGED_COUNT, 2, 0),
    (screen_history::NODE_COUNT, 2, 0),
    (screen_history::ACTIVE_FLAG, 4, 1),
    (screen_history::SCREEN_KEY, 2, 0xffff),
    (screen_history::SCREEN_KEY_DATA, 4, 0),
    (0x37ee, 4, 0), // three dwords after the default screen [no name]
    (0x37f2, 4, 0),
    (0x37f6, 4, 0),
    (screen_history::DEFAULT_BUILD, 4, 0),
    (screen_history::DEFAULT_BUILD + 4, 4, 0), // input
    (screen_history::DEFAULT_BUILD + 8, 4, 0), // flag
    (screen_history::DEFAULT_BUILD + 0xc, 4, 0), // leave
    (screen_history::DEFAULT_BUILD + 0x10, 4, 0), // exit
    (screen_history::DEFAULT_SLOT_VALUES, 4, 0),
    (screen_history::DEFAULT_SLOT_COUNT, 2, 0),
];

/// Writes `value` in `size` bytes (2 or 4) at `at`.
unsafe fn write_field(at: usize, size: u32, value: u32) {
    if size == 2 {
        write_u16(at, value as u16);
    } else {
        write_u32(at, value);
    }
}

/// Whether the `size` bytes (2 or 4) at `at` hold `value`.
unsafe fn field_is(at: usize, size: u32, value: u32) -> bool {
    if size == 2 { read_u16(at) == value as u16 } else { read_u32(at) == value }
}

/// The sound flags of channel `c` (10 dwords at `SCRMAN_SOUND_PLAYING` + 0x28 × c).
fn channel_sound_flags(sm: usize, c: usize) -> usize {
    sm + SCRMAN_SOUND_PLAYING + 0x28 * c
}

/// Runs the leave function of a screen node (`screen_node_fields::LEAVE`, cdecl), when there is a node and it has one.
unsafe fn leave_screen(node: usize) {
    if node != 0 && read_usize(node + screen_node_fields::LEAVE) != 0 {
        let leave: unsafe extern "C" fn() = core::mem::transmute(read_usize(node + screen_node_fields::LEAVE));
        leave();
    }
}

/// Rule: the screen manager starts with its command buffer, its local and echo Displays, no session and one local
/// record in channel 0 with no screen.
///
/// The command buffer (`NetworkBuffer::NetworkBuffer(screen_history::NET_BUFFER, 50000)`), the local Display
/// (`Display::Display(screen_history::LOCAL_DISPLAY, 0)`) and the echo Display (`SCRMAN_ECHO_DISPLAY`, channel 1); the
/// fields of `SCREEN_MANAGER_INITIAL_FIELDS`; each of the 16 channels' record count 0 and its 10 sound flags 0; record
/// 0's Display (`screen_manager::DISPLAY`) the local Display; the fields of `SCREEN_MANAGER_FIRST_RECORD`. Returns the
/// manager. The original also opens an exception frame to destroy what was built if a constructor throws; the copy
/// does not.
/// Original: `ScreenManager::ScreenManager` (0x7fc380, network.h, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_construct(sm: usize) -> usize {
    use screen_history as h;
    covered!("tel46_2 build the screen manager");
    let build_buffer: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_NETWORKBUFFER_NETWORKBUFFER);
    build_buffer(sm + h::NET_BUFFER, SCRMAN_NET_BUFFER_SIZE);
    let build_display: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_DISPLAY_BUILD);
    build_display(sm + h::LOCAL_DISPLAY, 0);
    build_display(sm + SCRMAN_ECHO_DISPLAY, 1);
    for &(at, size, value) in SCREEN_MANAGER_INITIAL_FIELDS.iter() {
        write_field(sm + at, size, value);
    }
    for c in 0..16 {
        write_u16(channel_count_at(sm, c), 0);
        core::ptr::write_bytes(channel_sound_flags(sm, c) as *mut u8, 0, 0x28);
    }
    write_usize(sm + screen_manager::DISPLAY, sm + h::LOCAL_DISPLAY); // tel46_2: record 0 shows the local Display
    for &(at, size, value) in SCREEN_MANAGER_FIRST_RECORD.iter() {
        write_field(sm + at, size, value);
    }
    sm
}

/// Rule: the screen manager ends by closing every channel's screens, leaving and destroying the network session and
/// destroying its two Displays and its command buffer.
///
/// `ScreenManager::close_all_screens` (0x7fc5e0); with a session (`screen_history::NETWORK`):
/// `ScreenManager::disconnect_network` (0x7fc690), then the session (read again), when there is one, destroyed
/// (`Network::~Network` and `operator delete`) and cleared. Then `Display::~Display` on the echo and the local
/// Displays and `NetworkBuffer::~NetworkBuffer` on the command buffer. The original also keeps an exception frame for
/// the members; the copy does not.
/// Original: `ScreenManager::~ScreenManager` (0x7fc530, network.h, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_destruct(sm: usize) {
    use screen_history as h;
    covered!("tel46_2 destroy the screen manager");
    exe::screenmanager_close_all_screens(sm);
    if read_usize(sm + h::NETWORK) != 0 {
        exe::screenmanager_disconnect_network(sm);
        let net = read_usize(sm + h::NETWORK);
        if net != 0 {
            let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_NETWORK_DESTRUCTOR_NETWORK);
            destroy(net);
            exe::crt_operator_delete(net);
        }
        write_usize(sm + h::NETWORK, 0); // tel46_2: no session left
    }
    let destroy_display: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_DISPLAY_DESTROY);
    destroy_display(sm + SCRMAN_ECHO_DISPLAY);
    destroy_display(sm + h::LOCAL_DISPLAY);
    let destroy_buffer: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_NETWORKBUFFER_DESTRUCTOR_NETWORKBUFFER);
    destroy_buffer(sm + h::NET_BUFFER);
}

/// Rule: every record of every channel has its screens closed: the current screen's leave function runs and the whole
/// history is freed.
///
/// For each of the 16 channels, for k from 0 while k is below the channel's record count (a signed word, read anew each
/// turn): the current slot (`screen_manager::CURRENT_SLOT`) = the channel; from k = 1 on, record k is swapped into
/// record 0 (`utils_swap_bytes`, 0x30 bytes); the leave function of record 0's current node (`leave_screen`) and
/// `ScreenManager::free_all_screens` (0x7fef10). The current slot is left at the last channel with records.
/// Original: `ScreenManager::close_all_screens` (0x7fc5e0, network.h, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_close_all_channels(sm: usize) {
    use screen_history as h;
    covered!("tel46_2 close every channel's screens");
    for c in 0..16usize {
        let mut k: i16 = 0;
        while k < read_i16(channel_count_at(sm, c)) {
            write_u16(sm + screen_manager::CURRENT_SLOT, c as u16);
            if k != 0 {
                exe::utils_swap_bytes(channel_record_at(sm, c, k as usize) as *mut i32, channel_record_at(sm, c, 0) as *mut i32, h::RECORD_SIZE as u32); // tel46_2: record k to the front
            }
            leave_screen(read_usize(scrman_channel_record(sm, c as i16, 0) + h::CURRENT_NODE));
            exe::screenmanager_free_all_screens(sm);
            k += 1;
        }
    }
}

/// Rule: leaving a hosted network session tells the players (the quit packet) and removes every remote record.
///
/// Without a session (`screen_history::NETWORK`) nothing. Otherwise the widget snapshot pool is freed
/// (`display_free_guio_snapshot_pool`); when hosting (`SCRMAN_HOST_FLAG`) the flag goes down, the quit packet
/// (`PACKET_QUIT` alone, `scrman_command_packet`) is sent to every player (`Network::send_packet` with player 0, the
/// result ignored), and for each channel 1..15, while its record count is above 0: record 0's manager is removed
/// (`ScreenManager::remove_manager_record` 0x7ff990 with its key and channel −1, a search by key) and its player's
/// connection closed (`Network::release_slot_socket` with record 0's player, read after the removal). Defect of the
/// original [inference]: when the key of record 0 is found first in an earlier channel the count never goes down and
/// the loop does not end.
/// Original: `ScreenManager::disconnect_network` (0x7fc690, network.h, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_disconnect_network(sm: usize) {
    use screen_history as h;
    if read_usize(sm + h::NETWORK) == 0 {
        return;
    }
    covered!("tel46_2 leave the network session");
    let free_pool: unsafe extern "C" fn() = core::mem::transmute(FN_DISPLAY_FREE_SNAPSHOT_POOL);
    free_pool();
    if read_u32(sm + SCRMAN_HOST_FLAG) == 0 {
        return;
    }
    covered!("tel46_2 the host leaves");
    write_u32(sm + SCRMAN_HOST_FLAG, 0);
    scrman_command_packet(sm, PACKET_QUIT); // tel46_2: the quit packet
    exe::network_send_packet(read_usize(sm + h::NETWORK), read_usize(sm + h::NET_BUFFER), read_u32(sm + h::NET_BUFFER_POS), 0);
    let release: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_NETWORK_RELEASE_SLOT_SOCKET);
    for c in 1..16usize {
        while read_i16(channel_count_at(sm, c)) > 0 {
            exe::screenmanager_remove_manager_record(sm, read_usize(scrman_channel_record(sm, c as i16, 0) + h::KEY), -1);
            release(read_usize(sm + h::NETWORK), read_u16(channel_record_at(sm, c, 0)) as u32);
        }
    }
}

/// The channel and record of manager key `key`: channels 0..15 in order, each one's records from the last down (the
/// count a signed word); none: None.
unsafe fn find_manager_key(sm: usize, key: u32) -> Option<(u32, i16)> {
    for c in 0..16i16 {
        let mut k = scrman_channel_count(sm, c).wrapping_sub(1);
        while k >= 0 {
            if read_u32(scrman_channel_record(sm, c, k) + screen_history::KEY) == key {
                return Some((c as u32, k));
            }
            k -= 1;
        }
    }
    None
}

/// Record `k` of channel `c` of the global screen manager (its first byte, `screen_history::NETWORK_PLAYER`).
fn global_record(c: i16, k: i16) -> usize {
    scrman_channel_record(G_SCREEN_MANAGER, c, k) + screen_history::NETWORK_PLAYER
}

/// The last record of a channel emptied for good: channel 0 gets back one local record with no screen and opens the
/// default screen; a remote channel sends the quit packet to its player.
///
/// Channel 0 (the low word): record 0 of the global manager reset (no player, no key, no screen, active, no screen key,
/// the local Display, the "Continue Game" time −1; the away flag cleared, the done flag kept), its count 1, then
/// `ScreenManager::open_screen` with the default screen's five values (`screen_history::DEFAULT_BUILD`). Another channel:
/// the quit packet (`PACKET_QUIT`, `scrman_command_packet`) sent to that channel number (`Network::send_packet`, the
/// result ignored).
unsafe fn last_record_removed(sm: usize, channel: u32) {
    use screen_history as h;
    if channel as u16 == 0 {
        covered!("tel46_2 channel 0 starts again");
        let g = G_SCREEN_MANAGER;
        write_u16(g + h::NETWORK_PLAYER, 0);
        write_u16(channel_count_at(g, 0), 1);
        write_u32(g + h::ACTIVE_FLAG, 1);
        write_u32(g + h::KEY, 0);
        write_usize(g + h::FIRST_NODE, 0);
        write_usize(g + h::LAST_NODE, 0);
        write_usize(g + h::CURRENT_NODE, 0);
        write_u16(g + h::FLAGGED_COUNT, 0);
        write_u16(g + h::NODE_COUNT, 0);
        write_usize(g + screen_manager::DISPLAY, g + h::LOCAL_DISPLAY);
        write_u32(g + h::AWAY, 0);
        write_u16(g + h::SCREEN_KEY, 0xffff);
        write_u32(g + h::SCREEN_KEY_DATA, 0);
        write_i32(g + MENUBAR_SLOT_CONTINUE_TIME, -1);
        let d = sm + h::DEFAULT_BUILD;
        exe::screenmanager_open_screen(sm, read_usize(d), read_usize(d + 4), read_u32(d + 8), read_usize(d + 0xc), read_u32(d + 0x10)); // tel46_2: the default screen
        return;
    }
    covered!("tel46_2 a remote channel emptied");
    scrman_command_packet(sm, PACKET_QUIT);
    exe::network_send_packet(read_usize(sm + h::NETWORK), read_usize(sm + h::NET_BUFFER), read_u32(sm + h::NET_BUFFER_POS), channel);
}

/// Rule: a manager's record leaves its channel: its screens are closed and the records behind it move up; the last
/// record of channel 0 gives way to a fresh local record on the default screen, and a remote channel emptied tells its
/// player.
///
/// With `channel` (low word) not −1 the record is record 0 of that channel; with −1, the record with manager key `key`
/// (`find_manager_key`; none: nothing). The manager's current slot (`screen_manager::CURRENT_SLOT`) goes to 0 when it
/// was that channel. The global current slot (`G_SCREEN_MANAGER_CURRENT_SLOT`) is set to the channel for the closing:
/// record 0's leave function (`leave_screen`) runs, or, for record k ≥ 1, record k is swapped into record 0 (no leave
/// function); `ScreenManager::free_all_screens` on the global manager, the channel's widget snapshot cleared
/// (`display_clear_guio_snapshot`), and the leave hook (`G_SCREEN_MANAGER_PLAYER_LEFT_CALLBACK`, cdecl) told record 0's
/// player; then the global current slot comes back. The channel's records 1.. move up one place, the last one is
/// zeroed (record 0 when the count was 1 or less) and the count goes down; with records left, record 0's
/// `screen_history::ACTIVE_FLAG` = 1; with none, `last_record_removed`. The original pushes the player to the hook as
/// a dword whose high word is whatever ecx held; the hooks read the low word, the copy pushes it zero-extended.
/// Original: `ScreenManager::remove_manager_record` (0x7ff990, network.h, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_remove_manager_record(sm: usize, key: u32, channel: u32) {
    use screen_history as h;
    let (channel, k) = if channel as u16 != 0xffff {
        (channel, 0i16)
    } else {
        match find_manager_key(sm, key) {
            Some(found) => found,
            None => return,
        }
    };
    covered!("tel46_2 remove a manager record");
    let c = channel as i16;
    if read_i16(sm + screen_manager::CURRENT_SLOT) == c {
        write_u16(sm + screen_manager::CURRENT_SLOT, 0);
    }
    let saved = read_u16(G_SCREEN_MANAGER_CURRENT_SLOT);
    write_u16(G_SCREEN_MANAGER_CURRENT_SLOT, c as u16);
    let base = scrman_channel_record(sm, c, 0);
    if k == 0 {
        leave_screen(read_usize(base + h::CURRENT_NODE));
    } else {
        exe::utils_swap_bytes((scrman_channel_record(sm, c, k) + h::NETWORK_PLAYER) as *mut i32, (base + h::NETWORK_PLAYER) as *mut i32, h::RECORD_SIZE as u32);
    }
    exe::screenmanager_free_all_screens(G_SCREEN_MANAGER);
    let clear: unsafe extern "stdcall" fn(u32) = core::mem::transmute(FN_DISPLAY_CLEAR_SNAPSHOT);
    clear(c as i32 as u32);
    let hook = read_usize(G_SCREEN_MANAGER_PLAYER_LEFT_CALLBACK);
    if hook != 0 {
        covered!("tel46_2 the leave hook");
        let f: unsafe extern "C" fn(u32) = core::mem::transmute(hook);
        f(read_u16(base + h::NETWORK_PLAYER) as u32);
    }
    write_u16(G_SCREEN_MANAGER_CURRENT_SLOT, saved);
    let count_at = (G_SCREEN_MANAGER_SLOT_RECORD_COUNTS as isize + 2 * c as isize) as usize;
    let count = read_i16(count_at);
    let mut j: i16 = 1;
    while j < count {
        core::ptr::copy_nonoverlapping(global_record(c, j) as *const u8, global_record(c, j - 1) as *mut u8, h::RECORD_SIZE); // tel46_2: the records move up
        j += 1;
    }
    core::ptr::write_bytes(global_record(c, j - 1) as *mut u8, 0, h::RECORD_SIZE);
    write_i16(count_at, count.wrapping_sub(1));
    if count.wrapping_sub(1) != 0 {
        write_u32(scrman_channel_record(G_SCREEN_MANAGER, c, 0) + h::ACTIVE_FLAG, 1);
        return;
    }
    last_record_removed(sm, channel);
}

/// Comparer for `screen_manager_construct`: only the original (it builds the buffer and the Displays, once, before the
/// game's WinMain); the plan afterwards: every field of the two tables, the channel counts and sound flags, record 0's
/// Display, the command buffer allocated with its capacity, and the manager returned.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_construct(sm: usize) -> usize {
    use screen_history as h;
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%)(sm);
    let mut wrong: Vec<usize> = SCREEN_MANAGER_INITIAL_FIELDS.iter().chain(SCREEN_MANAGER_FIRST_RECORD.iter())
        .filter(|&&(at, size, value)| !field_is(sm + at, size, value)).map(|f| f.0).collect();
    for c in 1..16 {
        if read_i16(channel_count_at(sm, c)) != 0 {
            wrong.push(channel_count_at(sm, c) - sm);
        }
    }
    if (0..16 * 10).any(|k| read_u32(channel_sound_flags(sm, 0) + 4 * k) != 0) {
        wrong.push(SCRMAN_SOUND_PLAYING);
    }
    if read_usize(sm + screen_manager::DISPLAY) != sm + h::LOCAL_DISPLAY {
        wrong.push(screen_manager::DISPLAY);
    }
    if read_usize(sm + h::NET_BUFFER) == 0 || read_u32(sm + h::NET_BUFFER_CAPACITY) != SCRMAN_NET_BUFFER_SIZE {
        wrong.push(h::NET_BUFFER);
    }
    SCOREBOARD.record(wrong.is_empty() && r == sm, || format!("ScreenManager::ScreenManager: returned {:#x}, fields off the plan at {:x?}", r, wrong));
    r
}

/// Comparer for `screen_manager_destruct`: only the original (it frees and destroys); the plan: no session left.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_destruct(sm: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(sm);
    let net = read_usize(sm + screen_history::NETWORK);
    SCOREBOARD.record(net == 0, || format!("ScreenManager::~ScreenManager: session {:#x} left", net));
}

/// The (player, key) of each record of channel `c`.
unsafe fn channel_owners(sm: usize, c: usize) -> Vec<(u16, u32)> {
    (0..read_i16(channel_count_at(sm, c)).max(0) as usize).map(|k| (read_u16(channel_record_at(sm, c, k)), read_u32(channel_record_at(sm, c, k) + 2))).collect()
}

/// Comparer for `screen_manager_close_all_channels`: only the original (it runs the leave functions and frees the
/// screens); the plan, from the records before: each channel's records in the order the swaps leave them (record k
/// swapped with record 0 for k = 1, 2, ...), every one with no screen history, and the current slot at the last
/// channel with records.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_close_all_channels(sm: usize) {
    use screen_history as h;
    let before: Vec<Vec<(u16, u32)>> = (0..16).map(|c| channel_owners(sm, c)).collect();
    let slot = (0..16).rev().find(|&c| !before[c].is_empty()).map(|c| c as u16).unwrap_or(read_u16(sm + screen_manager::CURRENT_SLOT));
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(sm);
    let mut wrong = Vec::new();
    for (c, owners) in before.into_iter().enumerate() {
        let mut want = owners;
        for k in 1..want.len() {
            want.swap(k, 0);
        }
        let empty = (0..want.len()).all(|k| {
            let rec = scrman_channel_record(sm, c as i16, k as i16);
            read_usize(rec + h::FIRST_NODE) == 0 && read_usize(rec + h::LAST_NODE) == 0 && read_usize(rec + h::CURRENT_NODE) == 0 && read_u16(rec + h::NODE_COUNT) == 0
        });
        if channel_owners(sm, c) != want || !empty {
            wrong.push(c);
        }
    }
    let now = read_u16(sm + screen_manager::CURRENT_SLOT);
    SCOREBOARD.record(wrong.is_empty() && now == slot, || format!("ScreenManager::close_all_screens: channels {:?} off the plan, current slot {} (plan {})", wrong, now, slot));
}

/// Comparer for `screen_manager_disconnect_network`: only the original (network); the plan: with a session the snapshot
/// pool freed, and when hosting the host flag down, the quit packet left in the buffer and channels 1..15 empty.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_disconnect_network(sm: usize) {
    use screen_history as h;
    let (net, host) = (read_usize(sm + h::NETWORK), read_u32(sm + SCRMAN_HOST_FLAG));
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%)(sm);
    if net == 0 {
        return;
    }
    let mut ok = read_usize(DISPLAY_SNAPSHOT_POOL) == 0;
    if host != 0 {
        let room = read_u32(sm + h::NET_BUFFER_CAPACITY) >= 1;
        ok &= read_u32(sm + SCRMAN_HOST_FLAG) == 0 && (1..16).all(|c| read_i16(channel_count_at(sm, c)) <= 0)
            && (!room || (read_u32(sm + h::NET_BUFFER_POS) == 1 && read_u8(read_usize(sm + h::NET_BUFFER)) == PACKET_QUIT));
    }
    SCOREBOARD.record(ok, || format!("ScreenManager::disconnect_network (host {}): pool {:#x}, host flag {}, counts {:?}", host, read_usize(DISPLAY_SNAPSHOT_POOL),
        read_u32(sm + SCRMAN_HOST_FLAG), (1..16).map(|c| read_i16(channel_count_at(sm, c))).collect::<Vec<_>>()));
}

/// Comparer for `screen_manager_remove_manager_record`: only the original (it runs the leave function, frees the screens
/// and may open the default screen or send a packet); the plan, from the channel's records before: the record found as
/// the copy finds it, the others moved up (record 0 active), the freed place zeroed and the count one less; channel 0
/// emptied: one local record with no key on the local Display; a remote channel emptied: the quit packet in the buffer.
/// With a leave hook (a network session: it removes records itself) only the original.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_remove_manager_record(sm: usize, key: u32, channel: u32) {
    use screen_history as h;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I4%%);
    let found = if channel as u16 != 0xffff { Some((channel, 0i16)) } else { find_manager_key(sm, key) };
    let Some((channel, k)) = found else {
        return o(sm, key, channel);
    };
    if read_usize(G_SCREEN_MANAGER_PLAYER_LEFT_CALLBACK) != 0 {
        // simplification: the leave hook removes the player's records too
        return o(sm, key, channel);
    }
    let c = channel as i16;
    let count = read_i16((G_SCREEN_MANAGER_SLOT_RECORD_COUNTS as isize + 2 * c as isize) as usize);
    let size = h::RECORD_SIZE;
    let mut records = core::slice::from_raw_parts(global_record(c, 0) as *const u8, size * 16).to_vec();
    if k != 0 {
        let (k, z) = (k as usize * size, 0);
        for b in 0..size {
            records.swap(z + b, k + b);
        }
    }
    let last = count.clamp(1, 16) as usize;
    records.copy_within(size..last * size, 0);
    records[(last - 1) * size..last * size].fill(0);
    let left = count.wrapping_sub(1);
    o(sm, key, channel);
    let after = read_i16((G_SCREEN_MANAGER_SLOT_RECORD_COUNTS as isize + 2 * c as isize) as usize);
    let ok = if left != 0 {
        records[h::ACTIVE_FLAG - h::NETWORK_PLAYER..h::ACTIVE_FLAG - h::NETWORK_PLAYER + 4].copy_from_slice(&1u32.to_le_bytes());
        let now = core::slice::from_raw_parts(global_record(c, 0) as *const u8, size * 16);
        after == left && now == &records[..]
    } else if channel as u16 == 0 {
        after == 1 && read_u16(global_record(0, 0)) == 0 && read_u32(global_record(0, 0) + 2) == 0
            && read_usize(G_SCREEN_MANAGER + screen_manager::DISPLAY) == G_SCREEN_MANAGER + h::LOCAL_DISPLAY
    } else {
        let room = read_u32(sm + h::NET_BUFFER_CAPACITY) >= 1;
        after == 0 && (!room || read_u8(read_usize(sm + h::NET_BUFFER)) == PACKET_QUIT)
    };
    SCOREBOARD.record(ok, || format!("ScreenManager::remove_manager_record(key {:#x}, channel {}): record {} of channel {}, count {} -> {} (plan {})",
        key, channel as i32, k, c, count, after, if left != 0 { left } else { (channel as u16 == 0) as i16 }));
}
