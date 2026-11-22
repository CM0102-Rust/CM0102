// ---------- batch %%LEVA%%: the screen manager's players and network packets: the player records, the network session, the click and scroll packets and the status message (scrman.cpp) ----------

/// The screen manager's join and leave hooks (cdecl(player), given to `start_network_session` and to
/// `add_manager_record`; the join hook is the global 0xdd6bf0 the player record callback calls), the channel a join
/// hook asks `monitor` to switch to (−1 none) and the dword of a record `monitor` stamps with the time [inference].
const SCRMAN_JOIN_HOOK: usize = 0x2627a0;
const SCRMAN_LEAVE_HOOK: usize = 0x2627a4;
const SCRMAN_NEXT_CHANNEL: usize = 0x382c;
const SCRMAN_RECORD_STAMP: usize = 0x2c;
/// The two player record callbacks by their exe address (through their detours, as the original passes them).
const SCRMAN_ADD_PLAYER_RECORD: usize = 0x7ff630;
const SCRMAN_REMOVE_PLAYER_RECORDS: usize = 0x7ff7b0;

/// The address of record `k` of channel `c` of the screen manager `sm` (16 channels of 16 records of 0x30 bytes from
/// +0x7d4; `scrman_channel_record` gives the base to add a `screen_history` offset to).
fn channel_record_at(sm: usize, c: usize, k: usize) -> usize {
    scrman_channel_record(sm, c as i16, k as i16) + screen_history::NETWORK_PLAYER
}

/// The address of the record count of channel `c` (a signed word).
fn channel_count_at(sm: usize, c: usize) -> usize {
    sm + screen_history::CHANNEL_RECORD_COUNTS + 2 * c
}

/// Rule: the machine's own network addresses, for the network game screen.
///
/// Forwards to `Network::get_local_addresses(out)` (0x77a980) on the session (`screen_history::NETWORK`, not checked
/// for null) and returns what it returns (the count).
/// Original: `ScreenManager::get_local_addresses` (0x7fc7c0, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_get_local_addresses(sm: usize, out: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = tcpip_fn(0x77a980);
    f(read_usize(sm + screen_history::NETWORK), out)
}

/// Rule: starting a network session keeps the join and leave hooks, closes the old session and opens a new one whose
/// connections call the player record callbacks; returns whether it opened.
///
/// The join and leave hooks are kept (`SCRMAN_JOIN_HOOK`, `SCRMAN_LEAVE_HOOK`). With a session
/// (`screen_history::NETWORK`): `ScreenManager::disconnect_network` (0x7fc690), then the session destroyed
/// (`Network::~Network` 0x77a890 and `operator delete`) and cleared. A new 0xc3e54-byte object (`operator new`) is built
/// by `Network::Network(ok, scrman_add_player_record, scrman_remove_player_records)` (0x77a7d0) and kept as the session;
/// when it does not open (`ok` 0) it is destroyed and the session cleared. Returns `ok`. Defect of the original: `ok`
/// lives in the join hook's argument slot, so when `operator new` fails it returns the join hook (nonzero, "opened")
/// with no session.
/// Original: `ScreenManager::start_network_session` (0x7ff500, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_start_network_session(sm: usize, on_join: usize, on_leave: usize) -> u32 {
    use screen_history as h;
    covered!("tel40_1 start a network session");
    write_usize(sm + SCRMAN_JOIN_HOOK, on_join);
    write_usize(sm + SCRMAN_LEAVE_HOOK, on_leave);
    let destroy: unsafe extern "thiscall" fn(usize) = tcpip_fn(FN_NETWORK_DESTRUCTOR_NETWORK);
    if read_usize(sm + h::NETWORK) != 0 {
        exe::screenmanager_disconnect_network(sm);
        let old = read_usize(sm + h::NETWORK);
        if old != 0 {
            destroy(old);
            exe::crt_operator_delete(old);
        }
        write_usize(sm + h::NETWORK, 0);
    }
    let mut ok = on_join as u32; // the original's argument slot (see the defect above)
    let p = exe::crt_operator_new(0xc3e54);
    let net = if p != 0 {
        let build: unsafe extern "thiscall" fn(usize, *mut u32, usize, usize) -> usize = tcpip_fn(0x77a7d0);
        build(p, &mut ok, SCRMAN_ADD_PLAYER_RECORD, SCRMAN_REMOVE_PLAYER_RECORDS)
    } else {
        0
    };
    write_usize(sm + h::NETWORK, net);
    if ok == 0 {
        if net != 0 {
            destroy(net);
            exe::crt_operator_delete(net);
        }
        write_usize(sm + h::NETWORK, 0);
    }
    ok
}

/// Rule: a player who joins gets a fresh record in his channel (the channel whose first record is his, or the first
/// empty channel); on a network client nothing happens.
///
/// On a network client (`G_SCREEN_MANAGER_NETWORK` set, `G_SCREEN_MANAGER_NETWORK_HOST_FLAG` 0) it returns. The channels
/// are walked from 0: the first with no records, or whose record 0 is the player (`screen_history::NETWORK_PLAYER`, the
/// low word of `player`), is taken; none, it returns. A full channel (16 records) returns; otherwise its records move one
/// place up (`utils_swap_bytes` 0x90ba90 of records k and k − 1, from the count down to 1, so the free slot comes to
/// the front with its old bytes), the count goes up and record 0 is set: the player, the key, the three history
/// pointers, the flagged and node counts 0, the active flag 1, the Display (the local one, `LOCAL_DISPLAY`, for player 0,
/// the remote one, `SCRMAN_ECHO_DISPLAY`, otherwise), the away flag 0, the screen key −1 and its dword 0, the stamp −1
/// (the done flag keeps the slot's old value). The current channel is set to it: with a join hook (`SCRMAN_JOIN_HOOK`)
/// the hook runs with `player` and `SCRMAN_NEXT_CHANNEL` = the channel; without one the new record gets the active flag 0
/// and the away flag 1 and is swapped with record 1 (it goes behind the current one). The current channel is restored.
/// Original: `scrman_add_player_record` (0x7ff630, scrman.cpp, cdecl).
pub(crate) unsafe extern "C" fn scrman_add_player_record(player: u32) {
    use screen_history as h;
    let sm = G_SCREEN_MANAGER;
    if read_usize(G_SCREEN_MANAGER_NETWORK) != 0 && read_u32(G_SCREEN_MANAGER_NETWORK_HOST_FLAG) == 0 {
        covered!("tel40_1 a client adds no record");
        return;
    }
    let id = player as u16;
    let mut c = 0usize;
    loop {
        if read_i16(channel_count_at(sm, c)) == 0 {
            break; // tel40_1: an empty channel
        }
        if read_u16(channel_record_at(sm, c, 0)) == id {
            let n = read_i16(channel_count_at(sm, c));
            if n >= 0x10 {
                return;
            }
            let mut k = n;
            while k > 0 {
                exe::utils_swap_bytes(channel_record_at(sm, c, k as usize) as *mut i32, channel_record_at(sm, c, k as usize - 1) as *mut i32, 0x30);
                k -= 1;
            }
            break;
        }
        c += 1;
        if c == 0x10 {
            return;
        }
    }
    let count = channel_count_at(sm, c);
    write_i16(count, read_i16(count).wrapping_add(1));
    let r = channel_record_at(sm, c, 0);
    write_u16(r, id);
    for f in [h::KEY, h::FIRST_NODE, h::LAST_NODE, h::CURRENT_NODE] {
        write_u32(r + f - h::NETWORK_PLAYER, 0);
    }
    write_u16(r + h::FLAGGED_COUNT - h::NETWORK_PLAYER, 0);
    write_u16(r + h::NODE_COUNT - h::NETWORK_PLAYER, 0);
    write_u32(r + h::ACTIVE_FLAG - h::NETWORK_PLAYER, 1);
    let display = if id == 0 { sm + h::LOCAL_DISPLAY } else { sm + SCRMAN_ECHO_DISPLAY };
    write_usize(r + screen_manager::DISPLAY - h::NETWORK_PLAYER, display);
    write_u32(r + h::AWAY - h::NETWORK_PLAYER, 0);
    write_u16(r + h::SCREEN_KEY - h::NETWORK_PLAYER, 0xffff);
    write_u32(r + h::SCREEN_KEY_DATA - h::NETWORK_PLAYER, 0);
    write_u32(r + SCRMAN_RECORD_STAMP, 0xffff_ffff);
    let saved = read_u16(sm + screen_manager::CURRENT_SLOT);
    write_u16(sm + screen_manager::CURRENT_SLOT, c as u16);
    let hook = read_usize(sm + SCRMAN_JOIN_HOOK);
    if hook != 0 {
        covered!("tel40_1 join hook");
        let f: unsafe extern "C" fn(u32) = core::mem::transmute(hook);
        f(player);
        write_u16(sm + SCRMAN_NEXT_CHANNEL, c as u16);
        write_u16(sm + screen_manager::CURRENT_SLOT, saved);
        return;
    }
    write_u32(r + h::ACTIVE_FLAG - h::NETWORK_PLAYER, 0);
    write_u32(r + h::AWAY - h::NETWORK_PLAYER, 1);
    exe::utils_swap_bytes(r as *mut i32, channel_record_at(sm, c, 1) as *mut i32, 0x30); // tel40_1: behind the current record
    write_u16(sm + screen_manager::CURRENT_SLOT, saved);
}

/// Rule: a player who leaves loses every record of the channels that are his; only on the host.
///
/// With `G_SCREEN_MANAGER_NETWORK_HOST_FLAG` set, each of the 16 channels whose record 0 is the player (the low word of
/// `player`) has its records removed (`ScreenManager::remove_manager_record` 0x7ff990 with record 0's key and the
/// channel) while its count is above 0, then the count 0.
/// Original: `scrman_remove_player_records` (0x7ff7b0, scrman.cpp, cdecl).
pub(crate) unsafe extern "C" fn scrman_remove_player_records(player: u32) {
    let sm = G_SCREEN_MANAGER;
    if read_u32(G_SCREEN_MANAGER_NETWORK_HOST_FLAG) == 0 {
        return;
    }
    covered!("tel40_1 a player leaves");
    for c in 0..0x10usize {
        if read_u16(channel_record_at(sm, c, 0)) != player as u16 {
            continue;
        }
        while read_i16(channel_count_at(sm, c)) > 0 {
            exe::screenmanager_remove_manager_record(sm, read_usize(channel_record_at(sm, c, 0) + 2), c as i32);
        }
        write_i16(channel_count_at(sm, c), 0);
    }
}

/// Rule: a manager gets a new record in the current channel, for the current channel's player, keeping the session's
/// hooks; with a manager key the record goes behind the current one and gets that key.
///
/// With a key: the join hook 0 and the leave hook `on_leave`, `scrman_add_player_record` with the low word of record 0's
/// player (with no join hook it puts the new record at place 1), record 1's key = `key`, then the join hook `on_join`.
/// Without a key: the hooks `on_leave` and `on_join`, then `scrman_add_player_record` (which runs the join hook).
/// Original: `ScreenManager::add_manager_record` (0x7ff8f0, scrman.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_manager_record(sm: usize, on_join: usize, on_leave: usize, key: u32) {
    use screen_history as h;
    let player = read_u16(screen_record(sm) + h::NETWORK_PLAYER) as u32;
    if key != 0 {
        covered!("tel40_1 a manager record with a key");
        write_usize(sm + SCRMAN_JOIN_HOOK, 0);
        write_usize(sm + SCRMAN_LEAVE_HOOK, on_leave);
        scrman_add_player_record(player);
        write_u32(screen_record(sm) + h::RECORD_SIZE + h::KEY, key); // tel40_1: the key on record 1
        write_usize(sm + SCRMAN_JOIN_HOOK, on_join);
        return;
    }
    write_usize(sm + SCRMAN_LEAVE_HOOK, on_leave);
    write_usize(sm + SCRMAN_JOIN_HOOK, on_join);
    scrman_add_player_record(player);
}

/// Delivers the screen manager's packet like the click senders: a local current record (player 0) has it applied to
/// the local Display (`Display::process_command_packet` 0x564080 with the session and 0), otherwise it is sent to its
/// player (`scrman_send_packet`, failing: the box `scrman.cpp <line>`). No echo Display here.
unsafe fn scrman_apply_or_send(sm: usize, line: u32) {
    use screen_history as h;
    if read_u16(screen_record(sm) + h::NETWORK_PLAYER) == 0 {
        exe::display_process_command_packet(sm + h::LOCAL_DISPLAY, read_usize(sm + h::NETWORK), 0, sm + h::NET_BUFFER);
        return;
    }
    covered!("tel40_1 click packet to a remote manager");
    scrman_send_packet(sm, line);
}

/// Rule: the current screen's queued click is cleared (command 0x11), on the local Display or the remote player's.
///
/// `scrman_command_packet` with 0x11 (box line `network.h 133`), delivered by `scrman_apply_or_send` (send failure
/// `scrman.cpp 2480`).
/// Original: `ScreenManager::send_clear_click` (0x800000, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_send_clear_click(sm: usize) {
    scrman_command_packet(sm, 0x11); // tel40_1: the clear click command
    scrman_apply_or_send(sm, 0x9b0);
}

/// Rule: the next left click on the current screen returns the given widget (command 0x13 with the widget).
///
/// `scrman_command_packet` with 0x13, the widget as a word (box line `network.h 134`), delivered by
/// `scrman_apply_or_send` (send failure `scrman.cpp 2527`).
/// Original: `ScreenManager::set_pending_click_widget` (0x800160, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_pending_click_widget(sm: usize, widget: u32) {
    scrman_command_packet(sm, 0x13);
    scrman_net_put(sm, widget, 2, 0x86); // tel40_1: the widget of the pending click
    scrman_apply_or_send(sm, 0x9df);
}

/// The scroll packet of an area into the network send buffer: the position to 0, command 0xf (box line
/// `network.h 133`), the area as a word (134) and the first visible row as a dword (136).
unsafe fn area_scroll_row_packet(area: u32, row: u32) {
    write_u32(NET_SEND_BUFFER + network_buffer::POS, 0);
    if network_buffer_room(NET_SEND_BUFFER, 1, 0x85) {
        write_u8(network_buffer_cursor(NET_SEND_BUFFER), 0xf);
        network_buffer_advance(NET_SEND_BUFFER, 1);
    }
    if network_buffer_room(NET_SEND_BUFFER, 2, 0x86) {
        write_u16(network_buffer_cursor(NET_SEND_BUFFER), area as u16);
        network_buffer_advance(NET_SEND_BUFFER, 2);
    }
    if network_buffer_room(NET_SEND_BUFFER, 4, 0x88) {
        write_u32(network_buffer_cursor(NET_SEND_BUFFER), row); // tel40_1: the first visible row
        network_buffer_advance(NET_SEND_BUFFER, 4);
    }
}

/// Rule: when an area scrolls, its first visible row is written as a packet (command 0xf) and sent to the remote
/// screen when channel 0 shows the remote Display; a local game only fills the buffer.
///
/// `area_scroll_row_packet` into `NET_SEND_BUFFER`; when channel 0's record shows the remote Display
/// (`screen_manager::DISPLAY` = `SCRMAN_ECHO_DISPLAY` of the global screen manager), `Network::send_packet` on the
/// session with the buffer and the channel of the screen being applied (`SCREEN_CHANNEL`, a word); failing, the box
/// `scrman.cpp 2571`. The record looked at is channel 0's, whatever the current channel.
/// Original: `ScreenManager::send_area_scroll_row` (0x800330, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_send_area_scroll_row(sm: usize, area: u32, row: u32) {
    area_scroll_row_packet(area, row);
    if read_usize(sm + screen_manager::DISPLAY) != G_SCREEN_MANAGER + SCRMAN_ECHO_DISPLAY {
        return;
    }
    covered!("tel40_1 scroll row to the remote screen");
    if exe::network_send_packet(read_usize(sm + screen_history::NETWORK), read_usize(NET_SEND_BUFFER), read_u32(NET_SEND_BUFFER + network_buffer::POS),
        read_u16(SCREEN_CHANNEL) as u32) == 0 {
        tel26_error_box(SCRMAN_CPP, 0xa0b);
    }
}

/// The status packet into the screen manager's buffer: command 7, the two numbers as dwords (box line `network.h 136`
/// each) and the text (`scrman_net_put_text`: its length with the terminator, then the text with no room check).
unsafe fn status_message_packet(sm: usize, text: usize, value: u32, total: u32) {
    scrman_command_packet(sm, 7); // tel40_1: the status command
    scrman_net_put(sm, value, 4, 0x88);
    scrman_net_put(sm, total, 4, 0x88);
    scrman_net_put_text(sm, text);
}

/// Rule: a progress or status message (a text and two numbers, the progress bar of the long jobs) is shown on the given
/// channel's screen, or on every channel's for channel −1: applied to the local Display for a local record, sent to the
/// player for a remote one.
///
/// `status_message_packet`. Channel −1 (the word 0xffff): for each of the 16 channels with records, record 0's player 0
/// has the packet applied to the local Display (`Display::process_command_packet` 0x564080 with the session and 0),
/// another player has it sent (`Network::send_packet`, failing: the box `scrman.cpp 3212`). One channel: record 0's
/// player 0 applies it locally (even with no records), another sends it when the channel has records (failing:
/// `scrman.cpp 3188`).
/// Original: `ScreenManager::send_network_status_message` (0x802f80, scrman.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn screen_manager_send_network_status_message(sm: usize, text: usize, value: u32, total: u32, channel: u32) {
    use screen_history as h;
    status_message_packet(sm, text, value, total);
    let apply = || exe::display_process_command_packet(sm + h::LOCAL_DISPLAY, read_usize(sm + h::NETWORK), 0, sm + h::NET_BUFFER);
    let send = |player: u16, line: u32| {
        if exe::network_send_packet(read_usize(sm + h::NETWORK), read_usize(sm + h::NET_BUFFER), read_u32(sm + h::NET_BUFFER_POS), player as u32) == 0 {
            tel26_error_box(SCRMAN_CPP, line);
        }
    };
    if channel as u16 != 0xffff {
        let c = channel as i16 as isize;
        let player = read_u16((sm as isize + h::NETWORK_PLAYER as isize + c * h::CHANNEL_SIZE as isize) as usize);
        if player == 0 {
            apply();
            return;
        }
        covered!("tel40_1 status message to a remote channel");
        if read_i16((sm as isize + h::CHANNEL_RECORD_COUNTS as isize + c * 2) as usize) > 0 {
            send(player, 0xc74);
        }
        return;
    }
    covered!("tel40_1 status message to every channel");
    for c in 0..0x10usize {
        if read_i16(channel_count_at(sm, c)) <= 0 {
            continue;
        }
        let player = read_u16(channel_record_at(sm, c, 0));
        if player == 0 {
            apply();
        } else {
            send(player, 0xc8c);
        }
    }
}

/// The regions the player records touch: the 16 channels of records, their counts, the current and the next channel and
/// the two hooks.
fn scrman_records_regions(sm: usize) -> [(usize, usize); 4] {
    [(sm + screen_history::NETWORK_PLAYER, 0x10 * screen_history::CHANNEL_SIZE), (sm + screen_history::CHANNEL_RECORD_COUNTS, 0x20),
        (sm + screen_manager::CURRENT_SLOT, 4), (sm + SCRMAN_JOIN_HOOK, 8)]
}

/// Side by side over the player records (`scrman_records_regions`).
unsafe fn compare_scrman_records(name: &str, sm: usize, original: impl FnOnce(), rust: impl FnOnce()) {
    let reg = scrman_records_regions(sm);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&reg, original, rust);
    SCOREBOARD.record(same, || format!("{}: player records differ {:?}", name, first_differences(&mo, &mr)));
}

/// Side by side over the packet a sender builds before delivering it: the copy's builder runs first into the
/// NetworkBuffer `buf`, its bytes are kept and the buffer (position and the `need` bytes) restored; then the original
/// runs once, building and delivering, and the bytes it left are compared with the copy's. No room for the `need`
/// bytes: only the original (the error box does not run twice).
unsafe fn compare_packet_build(name: &str, buf: usize, need: usize, build: impl FnOnce(), original: impl FnOnce()) {
    let data = read_usize(buf + network_buffer::DATA);
    if data == 0 || need > read_u32(buf + network_buffer::CAPACITY) as usize {
        return original();
    }
    let pos = read_u32(buf + network_buffer::POS);
    let saved = core::slice::from_raw_parts(data as *const u8, need).to_vec();
    build();
    let n = read_u32(buf + network_buffer::POS) as usize;
    let copy = core::slice::from_raw_parts(data as *const u8, n.min(need)).to_vec();
    core::ptr::copy_nonoverlapping(saved.as_ptr(), data as *mut u8, need);
    write_u32(buf + network_buffer::POS, pos);
    original();
    let got = core::slice::from_raw_parts(data as *const u8, copy.len());
    SCOREBOARD.record(n == need && got == &copy[..], || format!("{}: packet {:x?} (length {}), copy {:x?} (length {})", name, got, need, copy, n));
}

// simplification: the session is Winsock and sockets: it does not run twice. Original only.
/// Comparer for `screen_manager_get_local_addresses`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_get_local_addresses(sm: usize, out: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(sm, out)
}

// simplification: it opens Winsock and the broadcast sockets: it does not run twice. Original only; the plan: the hooks
// kept, and the session set exactly when it returns nonzero (allocation failure aside).
/// Comparer for `screen_manager_start_network_session`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_start_network_session(sm: usize, on_join: usize, on_leave: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%)(sm, on_join, on_leave);
    let net = read_usize(sm + screen_history::NETWORK);
    SCOREBOARD.record(read_usize(sm + SCRMAN_JOIN_HOOK) == on_join && read_usize(sm + SCRMAN_LEAVE_HOOK) == on_leave && (r == 0) == (net == 0),
        || format!("start_network_session: returned {:#x}, session {:#x}, hooks {:#x} {:#x}", r, net, read_usize(sm + SCRMAN_JOIN_HOOK), read_usize(sm + SCRMAN_LEAVE_HOOK)));
    r
}

/// Comparer for `scrman_add_player_record`: side by side over the records; with a join hook (it opens screens) only
/// the original.
pub(crate) unsafe extern "C" fn compare_scrman_add_player_record(player: u32) {
    let o = original_fn::<unsafe extern "C" fn(u32)>(%%I2%%);
    let sm = G_SCREEN_MANAGER;
    let client = read_usize(G_SCREEN_MANAGER_NETWORK) != 0 && read_u32(G_SCREEN_MANAGER_NETWORK_HOST_FLAG) == 0;
    if !client && read_usize(sm + SCRMAN_JOIN_HOOK) != 0 {
        // simplification: the join hook opens screens: it does not run twice
        return o(player);
    }
    compare_scrman_records("scrman_add_player_record", sm, || o(player), || scrman_add_player_record(player));
}

// simplification: removing a record frees its screen history: it does not run twice. Original only; the plan: on the
// host no channel left whose record 0 is the player and has records.
/// Comparer for `scrman_remove_player_records`.
pub(crate) unsafe extern "C" fn compare_scrman_remove_player_records(player: u32) {
    original_fn::<unsafe extern "C" fn(u32)>(%%I3%%)(player);
    let sm = G_SCREEN_MANAGER;
    if read_u32(G_SCREEN_MANAGER_NETWORK_HOST_FLAG) == 0 {
        return;
    }
    let left = (0..0x10usize).find(|&c| read_u16(channel_record_at(sm, c, 0)) == player as u16 && read_i16(channel_count_at(sm, c)) != 0);
    SCOREBOARD.record(left.is_none(), || format!("scrman_remove_player_records(player {}): channel {:?} still has records", player as u16, left));
}

/// Comparer for `screen_manager_add_manager_record`: side by side over the records; without a key and with a join hook
/// (it opens screens) only the original.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_manager_record(sm: usize, on_join: usize, on_leave: usize, key: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I4%%);
    if key == 0 && on_join != 0 {
        // simplification: the join hook opens screens: it does not run twice
        return o(sm, on_join, on_leave, key);
    }
    compare_scrman_records("screen_manager_add_manager_record", sm, || o(sm, on_join, on_leave, key), || screen_manager_add_manager_record(sm, on_join, on_leave, key));
}

/// Comparer for `screen_manager_send_clear_click`: the packet side by side, delivered once by the original.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_send_clear_click(sm: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I5%%);
    compare_packet_build("send_clear_click", sm + screen_history::NET_BUFFER, 1, || scrman_command_packet(sm, 0x11), || o(sm));
}

/// Comparer for `screen_manager_set_pending_click_widget`: the packet side by side, delivered once by the original.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_pending_click_widget(sm: usize, widget: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I6%%);
    compare_packet_build("set_pending_click_widget", sm + screen_history::NET_BUFFER, 3, || {
        scrman_command_packet(sm, 0x13);
        scrman_net_put(sm, widget, 2, 0x86);
    }, || o(sm, widget));
}

/// Comparer for `screen_manager_send_area_scroll_row`: in a local game (channel 0 not on the remote Display) side by
/// side over the network send buffer (its header and the 7 bytes); otherwise the packet side by side and the send once
/// by the original.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_send_area_scroll_row(sm: usize, area: u32, row: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I7%%);
    let data = read_usize(NET_SEND_BUFFER + network_buffer::DATA);
    if data == 0 || read_u32(NET_SEND_BUFFER + network_buffer::CAPACITY) < 7 {
        // simplification: the error boxes do not run twice
        return o(sm, area, row);
    }
    if read_usize(sm + screen_manager::DISPLAY) == G_SCREEN_MANAGER + SCRMAN_ECHO_DISPLAY {
        return compare_packet_build("send_area_scroll_row", NET_SEND_BUFFER, 7, || area_scroll_row_packet(area, row), || o(sm, area, row));
    }
    let reg = [(NET_SEND_BUFFER, 12), (data, 7)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(sm, area, row), || screen_manager_send_area_scroll_row(sm, area, row));
    SCOREBOARD.record(same, || format!("send_area_scroll_row(area {}, row {}): differences {:?}", area as u16, row as i32, first_differences(&mo, &mr)));
}

/// Comparer for `screen_manager_send_network_status_message`: the packet side by side, applied or sent once by the
/// original (the local Display draws the progress).
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_send_network_status_message(sm: usize, text: usize, value: u32, total: u32, channel: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(%%I8%%);
    let need = 1 + 4 + 4 + 2 + c_string_len(text) + 1;
    compare_packet_build("send_network_status_message", sm + screen_history::NET_BUFFER, need, || status_message_packet(sm, text, value, total),
        || o(sm, text, value, total, channel));
}
