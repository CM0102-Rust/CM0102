// ---------- batch %%LEVA%%: the screen manager's last base routines: a widget's rectangle, closing every screen, opening a screen for each record and the sounds (scrman.cpp) ----------

/// The sound player (`SoundPlayer`, simulated_stats.cpp; 0 when the game has no sound) and, in the screen manager, the
/// sound slots a remote manager reported playing (16 channels × 10 dwords).
const SOUND_PLAYER: usize = 0xdd7eec;
const SCRMAN_SOUND_PLAYING: usize = 0x26250a;

/// The playing flag of sound slot `slot` of the current channel (the original's `+0x26250a + (channel × 10 + slot) × 4`,
/// the slot sign-extended from its byte).
unsafe fn scrman_sound_flag(sm: usize, slot: u32) -> usize {
    let k = read_i16(sm + screen_manager::CURRENT_SLOT) as isize * 10 + slot as i8 as isize;
    (sm as isize + SCRMAN_SOUND_PLAYING as isize + k * 4) as usize
}

/// Starts a command packet: the buffer position to 0 and the command byte (box line `network.h 133`).
unsafe fn scrman_command_packet(sm: usize, command: u8) {
    write_u32(sm + screen_history::NET_BUFFER_POS, 0);
    scrman_net_put(sm, command as u32, 1, 0x85);
}

/// Appends a string to the packet: its length with the terminator (a word, box line `network.h 134`), then the string
/// with its terminator copied after it and the position advanced, with no room check (defect of the original, as in
/// `screen_manager_set_widget_text`).
unsafe fn scrman_net_put_text(sm: usize, text: usize) {
    use screen_history as h;
    let len = c_string_len(text) + 1;
    scrman_net_put(sm, len as u32, 2, 0x86);
    let pos = read_u32(sm + h::NET_BUFFER_POS);
    core::ptr::copy_nonoverlapping(text as *const u8, (read_usize(sm + h::NET_BUFFER) + pos as usize) as *mut u8, len);
    write_u32(sm + h::NET_BUFFER_POS, pos.wrapping_add(len as u16 as i16 as u32));
}

/// Sends the packet in the buffer to the current record's network player (`Network::send_packet` 0x77ac30, thiscall);
/// failing, the box `scrman.cpp <line>`.
unsafe fn scrman_send_packet(sm: usize, line: u32) {
    use screen_history as h;
    let send: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x77ac30usize);
    let player = read_u16(screen_record(sm) + h::NETWORK_PLAYER);
    if send(read_usize(sm + h::NETWORK), read_usize(sm + h::NET_BUFFER), read_u32(sm + h::NET_BUFFER_POS), player as u32) == 0 {
        show_error_at(SCRMAN_CPP, line);
    }
}

/// The packet left in the buffer checked against the expected bytes, when there was room for it.
unsafe fn scrman_check_packet(name: &str, sm: usize, room: bool, expected: &[u8]) {
    use screen_history as h;
    if !room {
        return;
    }
    let pos = read_u32(sm + h::NET_BUFFER_POS) as usize;
    let got = core::slice::from_raw_parts(read_usize(sm + h::NET_BUFFER) as *const u8, pos.min(expected.len()));
    SCOREBOARD.record(pos == expected.len() && got == expected, || format!("{}: packet {:x?}, expected {:x?}", name, got, expected));
}

/// Rule: a widget of the current screen gets a new rectangle (sub-command 1, four words); nothing for widget −1.
///
/// The packet as `scrman_widget_packet`, then x1, y1, x2 and y2 as words (box line `network.h 134` each) and its
/// delivery (`scrman_deliver_packet`, send failure `scrman.cpp 2644`). The 1080 exe rewrites the entry (it scales the
/// four coordinates to the big screen first): the entry is for the original exe only.
/// Original: `ScreenManager::set_widget_rect` (0x800890, scrman.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_rect(sm: usize, widget: u32, x1: u32, y1: u32, x2: u32, y2: u32) {
    if widget as u16 == 0xffff {
        return;
    }
    scrman_widget_packet(sm, widget, 1); // tel38_5: the rectangle sub-command
    for v in [x1, y1, x2, y2] {
        scrman_net_put(sm, v, 2, 0x86);
    }
    scrman_deliver_packet(sm, 0xa54);
}

/// Comparer for `screen_manager_set_widget_rect`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_rect(sm: usize, widget: u32, x1: u32, y1: u32, x2: u32, y2: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32)>(%%I0%%);
    let mut value = Vec::new();
    for v in [x1, y1, x2, y2] {
        value.extend_from_slice(&(v as u16).to_le_bytes());
    }
    compare_scrman_set_widget("screen_manager_set_widget_rect", sm, widget, widget as u16 == 0xffff, scrman_expected_packet(widget, 1, &value),
        || o(sm, widget, x1, y1, x2, y2));
}

/// Rule: closing every screen of the current manager runs each screen's exit function and the current screen's leave
/// function, empties the Display and frees the history; a single-instance screen with no exit function blocks it.
///
/// With flagged screens (`screen_history::FLAGGED_COUNT` > 0), the current node walks back from the last one
/// (`screen_node_fields::PREV`); a node with the flag and no exit function stops the walk: `ACTIVE_FLAG` = 1, returns 0
/// (defect of the original: the current node is left at that node). When the walk ends the current node is 0, so the
/// next part is skipped (defect of the original [inference]: with flagged screens open no exit function runs). With a
/// current node: from the first node each exit function runs (cdecl, no arguments; the current node walks along by
/// `NEXT`), the current node comes back, its leave function runs, the record's Display is reset (`Display::reset`
/// 0x55f7c0, thiscall, releasing its items) and the current node goes to 0. Then `ACTIVE_FLAG` = 0,
/// `ScreenManager::free_all_screens` and 1.
/// Original: `ScreenManager::close_all_screens` (0x803d80, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_close_all_screens(sm: usize) -> u32 {
    use screen_history as h;
    use screen_node_fields as n;
    let current = |sm: usize| screen_record(sm) + h::CURRENT_NODE;
    if read_i16(screen_record(sm) + h::FLAGGED_COUNT) > 0 {
        write_usize(current(sm), read_usize(screen_record(sm) + h::LAST_NODE));
        let mut node = read_usize(current(sm));
        while node != 0 {
            if read_u32(node + n::FLAG) != 0 && read_usize(node + n::EXIT) == 0 {
                covered!("tel38_5 closing blocked");
                write_u32(screen_record(sm) + h::ACTIVE_FLAG, 1); // tel38_5: a flagged screen with no exit blocks
                return 0;
            }
            write_usize(current(sm), read_usize(node + n::PREV));
            node = read_usize(current(sm));
        }
    }
    let saved = read_usize(current(sm));
    if saved != 0 {
        write_usize(current(sm), read_usize(screen_record(sm) + h::FIRST_NODE));
        let mut node = read_usize(current(sm));
        while node != 0 {
            let exit = read_usize(node + n::EXIT);
            if exit != 0 {
                let f: unsafe extern "C" fn() = core::mem::transmute(exit);
                f();
            }
            write_usize(current(sm), read_usize(read_usize(current(sm)) + n::NEXT));
            node = read_usize(current(sm));
        }
        write_usize(current(sm), saved);
        let leave = read_usize(read_usize(current(sm)) + n::LEAVE);
        if leave != 0 {
            let f: unsafe extern "C" fn() = core::mem::transmute(leave);
            f();
        }
        let reset: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x55f7c0usize);
        reset(read_usize(screen_record(sm) + screen_manager::DISPLAY), 1);
        write_usize(current(sm), 0);
    }
    write_u32(screen_record(sm) + h::ACTIVE_FLAG, 0);
    let free_all: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7fef10usize);
    free_all(sm);
    1
}

// simplification: runs the screens' functions and frees the history: it does not run twice. Original only; the plan:
// with 1, the history empty and `ACTIVE_FLAG` 0; with 0, `ACTIVE_FLAG` 1.
/// Comparer for `screen_manager_close_all_screens`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_close_all_screens(sm: usize) -> u32 {
    use screen_history as h;
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(sm);
    let rec = screen_record(sm);
    let ok = match r {
        1 => read_usize(rec + h::FIRST_NODE) == 0 && read_usize(rec + h::CURRENT_NODE) == 0 && read_u32(rec + h::ACTIVE_FLAG) == 0,
        0 => read_u32(rec + h::ACTIVE_FLAG) == 1,
        _ => false,
    };
    SCOREBOARD.record(ok, || format!("screen_manager_close_all_screens: returned {} first {:#x} current {:#x} active {}", r, read_usize(rec + h::FIRST_NODE),
        read_usize(rec + h::CURRENT_NODE), read_u32(rec + h::ACTIVE_FLAG)));
    r
}

/// Rule: opens a screen on every record of every channel whose manager key is `key` (all of them when `key` is 0).
///
/// The walk of `screen_manager_set_screen_param_for_each_record`: the current slot saved and walking the 16 channels;
/// in a channel with records, record 0 opens the screen (`exe::screenmanager_open_screen` with `build`, `input`, `flag`,
/// `leave`, `exit`) when its key matches, then each other record k is swapped into place 0 (`exe::utils_swap_bytes`,
/// 0x30 bytes) and opens it when its key matches; with more than one record, records 0 and 1 are swapped at the end
/// (the same defect: with three or more records they come back in another order). The slot is restored; returns 1.
/// Original: `ScreenManager::open_screen_for_each_record` (0x7fe4a0, scrman.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn screen_manager_open_screen_for_each_record(sm: usize, key: u32, build: usize, input: usize, flag: u32, leave: usize,
    exit: u32) -> u32 {
    use screen_history as h;
    let saved = read_u16(sm + screen_manager::CURRENT_SLOT);
    write_u16(sm + screen_manager::CURRENT_SLOT, 0);
    let count = |sm: usize| read_i16((sm as isize + h::CHANNEL_RECORD_COUNTS as isize + read_i16(sm + screen_manager::CURRENT_SLOT) as isize * 2) as usize);
    let open = |sm: usize| {
        if key == 0 || key == read_u32(screen_record(sm) + h::KEY) { // tel38_5: only the records with the key
            exe::screenmanager_open_screen(sm, build, input, flag, leave, exit);
        }
    };
    loop {
        if count(sm) != 0 {
            open(sm);
            let mut k: i16 = 1;
            while count(sm) > k {
                let first = screen_record(sm) + h::NETWORK_PLAYER;
                exe::utils_swap_bytes(first as *mut i32, (first + k as usize * h::RECORD_SIZE) as *mut i32, h::RECORD_SIZE as u32);
                open(sm);
                k += 1;
            }
            if count(sm) > 1 {
                covered!("tel38_5 screen opened on several records");
                let first = screen_record(sm) + h::NETWORK_PLAYER;
                exe::utils_swap_bytes(first as *mut i32, (first + h::RECORD_SIZE) as *mut i32, h::RECORD_SIZE as u32);
            }
        }
        let slot = read_i16(sm + screen_manager::CURRENT_SLOT).wrapping_add(1);
        write_i16(sm + screen_manager::CURRENT_SLOT, slot);
        if slot >= 16 {
            break;
        }
    }
    write_u16(sm + screen_manager::CURRENT_SLOT, saved);
    1
}

// simplification: opens screens (allocates history nodes, runs leave functions): it does not run twice. Original only;
// the plan: the current slot restored and 1.
/// Comparer for `screen_manager_open_screen_for_each_record`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_open_screen_for_each_record(sm: usize, key: u32, build: usize, input: usize, flag: u32,
    leave: usize, exit: u32) -> u32 {
    let slot = read_u16(sm + screen_manager::CURRENT_SLOT);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, usize, u32, usize, u32) -> u32>(%%I2%%)(sm, key, build, input, flag, leave, exit);
    let back = read_u16(sm + screen_manager::CURRENT_SLOT);
    SCOREBOARD.record(r == 1 && back == slot, || format!("screen_manager_open_screen_for_each_record: returned {}, slot {} came back as {}", r, slot, back));
    r
}

/// Rule: plays a sound file for the current manager: locally through the sound player, or by a network command to a
/// remote manager; the slot that plays it goes to `*out` (0xff none).
///
/// `*out` = 0xff first. A local record (`screen_history::NETWORK_PLAYER` = 0) with a sound player: `*out` =
/// `SoundPlayer::play_file(pan, file, looping)` (0x84cbc0, thiscall) and that slot's volume to 100
/// (`SoundPlayer::set_slot_volume` 0x84ccc0); with no sound player nothing more. A remote one: the packet command 0xa,
/// `out` itself (a dword, box line `network.h 137`; the remote side answers with `on_remote_sound_started`), `pan` and
/// `looping` (bytes), the file name (`scrman_net_put_text`), sent (failure `scrman.cpp 2992`).
/// Original: `ScreenManager::play_sound` (0x8025e0, scrman.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn screen_manager_play_sound(sm: usize, out: usize, pan: u32, file: usize, looping: u32) {
    write_u8(out, 0xff);
    if read_u16(screen_record(sm) + screen_history::NETWORK_PLAYER) == 0 {
        let player = read_usize(SOUND_PLAYER);
        if player == 0 {
            return;
        }
        let play: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(0x84cbc0usize);
        let volume: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x84ccc0usize);
        let slot = play(player, pan, file, looping);
        write_u8(out, slot as u8);
        volume(read_usize(SOUND_PLAYER), slot, 100); // tel38_5: full volume
        return;
    }
    covered!("tel38_5 sound for a remote manager");
    scrman_command_packet(sm, 0xa);
    scrman_net_put(sm, out as u32, 4, 0x89);
    scrman_net_put(sm, pan, 1, 0x85);
    scrman_net_put(sm, looping, 1, 0x85);
    scrman_net_put_text(sm, file);
    scrman_send_packet(sm, 0xbb0);
}

// simplification: plays a sound or sends the command: it does not run twice. Original only; the plan: with no sound
// player `*out` = 0xff; for a remote manager the packet.
/// Comparer for `screen_manager_play_sound`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_play_sound(sm: usize, out: usize, pan: u32, file: usize, looping: u32) {
    let remote = read_u16(screen_record(sm) + screen_history::NETWORK_PLAYER) != 0;
    let mut expected = vec![0xa];
    if remote {
        expected.extend_from_slice(&(out as u32).to_le_bytes());
        expected.extend_from_slice(&[pan as u8, looping as u8]);
        let len = c_string_len(file) + 1;
        expected.extend_from_slice(&(len as u16).to_le_bytes());
        expected.extend_from_slice(core::slice::from_raw_parts(file as *const u8, len));
    }
    let room = read_u32(sm + screen_history::NET_BUFFER_CAPACITY) as usize >= expected.len();
    let silent = read_usize(SOUND_PLAYER) == 0;
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32)>(%%I3%%)(sm, out, pan, file, looping);
    if remote {
        scrman_check_packet("screen_manager_play_sound", sm, room, &expected);
    } else if silent {
        SCOREBOARD.record(read_u8(out) == 0xff, || format!("screen_manager_play_sound: no sound player, slot {:#x}", read_u8(out)));
    }
}

/// Rule: whether a sound slot still plays: never for slot 0xff; locally the sound player's answer (0 without one); for a
/// remote manager the flag his side reported.
///
/// Local record: `SoundPlayer::is_slot_playing(slot)` (0x84cc80, thiscall; the whole argument passed on). Remote: the
/// dword `scrman_sound_flag`.
/// Original: `ScreenManager::is_sound_playing` (0x8029f0, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_is_sound_playing(sm: usize, slot: u32) -> u32 {
    if slot as u8 == 0xff {
        return 0;
    }
    if read_u16(screen_record(sm) + screen_history::NETWORK_PLAYER) != 0 {
        return read_u32(scrman_sound_flag(sm, slot)); // tel38_5: the remote flag
    }
    let player = read_usize(SOUND_PLAYER);
    if player == 0 {
        return 0;
    }
    let playing: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x84cc80usize);
    playing(player, slot) // tel38_5: the sound player's answer
}

compare_value!(compare_screen_manager_is_sound_playing, screen_manager_is_sound_playing, %%I4%%, (slot: u32) -> u32);

/// Rule: stops a sound slot (every slot for 0xff) locally or by a network command, and clears the playing flags.
///
/// Local record: with a sound player, `SoundPlayer::stop_all` (0x84cb10) for 0xff, otherwise
/// `SoundPlayer::stop_slot(slot)` (0x84cb70). Remote: the packet command 0xb and the slot (byte), sent (failure
/// `scrman.cpp 3043`). Then the current channel's 10 flags go to 0 for 0xff, otherwise the slot's
/// (`scrman_sound_flag`).
/// Original: `ScreenManager::stop_sound` (0x802a50, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_stop_sound(sm: usize, slot: u32) {
    let all = slot as u8 == 0xff;
    if read_u16(screen_record(sm) + screen_history::NETWORK_PLAYER) == 0 {
        let player = read_usize(SOUND_PLAYER);
        if player != 0 {
            if all {
                let stop_all: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x84cb10usize);
                stop_all(player);
            } else {
                let stop: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x84cb70usize);
                stop(player, slot);
            }
        }
    } else {
        covered!("tel38_5 sound stopped for a remote manager");
        scrman_command_packet(sm, 0xb);
        scrman_net_put(sm, slot, 1, 0x85);
        scrman_send_packet(sm, 0xbe3);
    }
    if all {
        for k in 0..10 {
            write_u32(scrman_sound_flag(sm, k), 0);
        }
    } else {
        write_u32(scrman_sound_flag(sm, slot), 0); // tel38_5: the slot no longer plays
    }
}

// simplification: stops the sound or sends the command: it does not run twice. Original only; the plan: the flags
// cleared and, for a remote manager, the packet.
/// Comparer for `screen_manager_stop_sound`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_stop_sound(sm: usize, slot: u32) {
    let remote = read_u16(screen_record(sm) + screen_history::NETWORK_PLAYER) != 0;
    let room = read_u32(sm + screen_history::NET_BUFFER_CAPACITY) >= 2;
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I5%%)(sm, slot);
    if remote {
        scrman_check_packet("screen_manager_stop_sound", sm, room, &[0xb, slot as u8]);
    }
    let cleared = if slot as u8 == 0xff { (0..10).all(|k| read_u32(scrman_sound_flag(sm, k)) == 0) } else { read_u32(scrman_sound_flag(sm, slot)) == 0 };
    SCOREBOARD.record(cleared, || format!("screen_manager_stop_sound({}): a flag still set", slot as i8));
}

/// Rule: a remote manager's side reports the slot a sound plays in: `*out` = the slot and, unless 0xff, its flag
/// (`scrman_sound_flag`) = 1.
/// Original: `ScreenManager::on_remote_sound_started` (0x802cb0, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_on_remote_sound_started(sm: usize, slot: u32, out: usize) {
    write_u8(out, slot as u8);
    if slot as u8 != 0xff {
        write_u32(scrman_sound_flag(sm, slot), 1); // tel38_5: the remote slot plays
    }
}

/// Comparer for `screen_manager_on_remote_sound_started`: side by side over `*out` and the channel's flags.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_on_remote_sound_started(sm: usize, slot: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize)>(%%I6%%);
    let flags = scrman_sound_flag(sm, 0);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side_light(&[(out, 1), (scrman_sound_flag(sm, slot), 4)], || o(sm, slot, out),
        || screen_manager_on_remote_sound_started(sm, slot, out));
    SCOREBOARD.record(equal, || format!("screen_manager_on_remote_sound_started({}) flags {:#x}: differences in {:?}", slot as i8, flags, first_differences(&mo, &mr)));
}

/// Rule: a remote manager's side reports a sound ended: its flag (`scrman_sound_flag`) = 0.
/// Original: `ScreenManager::on_remote_sound_finished` (0x802ce0, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_on_remote_sound_finished(sm: usize, slot: u32) {
    write_u32(scrman_sound_flag(sm, slot), 0);
}

/// Comparer for `screen_manager_on_remote_sound_finished`: side by side over the flag.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_on_remote_sound_finished(sm: usize, slot: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I7%%);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side_light(&[(scrman_sound_flag(sm, slot), 4)], || o(sm, slot),
        || screen_manager_on_remote_sound_finished(sm, slot));
    SCOREBOARD.record(equal, || format!("screen_manager_on_remote_sound_finished({}): differences in {:?}", slot as i8, first_differences(&mo, &mr)));
}
