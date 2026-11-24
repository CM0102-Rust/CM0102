// ---------- batch %%LEVA%%: the screen loop and the network client loop (scrman.cpp) ----------

/// Screen manager fields the two loops use (offsets from the manager, like `screen_history`); named after the global of
/// the same address (`G_SCREEN_MANAGER` + offset) where that one has a name.
/// The menubar handler (cdecl(u32)), run when the clicked item's id is the menubar item id (an i32); the id the network
/// client puts back on that item after the menubar dialog [inference].
const SCRMAN_MENUBAR_HANDLER: usize = MENUBAR_HANDLER - G_SCREEN_MANAGER;
const SCRMAN_MENUBAR_ITEM_ID: usize = 0x37f2;
const SCRMAN_MENUBAR_RESTORE_ID: usize = 0x37f6;
/// The manager key monitor switches to when a screen has ended (0 none).
const SCRMAN_WAITING_MANAGER: usize = G_MATCH_WAITING_MANAGER - G_SCREEN_MANAGER;
/// u32: 1 while a screen's input runs again with −1 after going back [inference].
const SCRMAN_INPUT_AGAIN: usize = 0x3834;
/// u32: the network client shows "Contacting Server..." while it waits for a click to go through.
const SCRMAN_JOIN_BY_ADDRESS: usize = NETWORK_JOIN_BY_ADDRESS - G_SCREEN_MANAGER;
/// u32: 1 while monitor runs [inference].
const SCRMAN_IN_MONITOR: usize = 0x383c;
/// u16: the player the last received packet came from.
const SCRMAN_RECEIVED_PLAYER: usize = 0x382e;
/// u16: the id an item with id 0 stands for.
const SCRMAN_DEFAULT_COMMAND_ID: usize = G_CURRENT_COMMAND_ID - G_SCREEN_MANAGER;
/// The automatic Continue [inference]: the time of the last click (i32), the seconds without a click (u16, 0 = off) and
/// the seconds since monitor started (u16) after which the screen ends by itself.
const SCRMAN_AUTO_CONTINUE_LAST: usize = 0x262798;
const SCRMAN_AUTO_CONTINUE_IDLE: usize = 0x26279c;
const SCRMAN_AUTO_CONTINUE_MIN: usize = 0x26279e;
/// cdecl(): run after every input handled by monitor (0 none) [inference].
const SCRMAN_LOOP_CALLBACK: usize = 0x262506;
/// u32 of a Display (past `display_life::DWORD_551`) that monitor sets on both of the manager's Displays [inference].
const DISPLAY_FLAG_559: usize = 0x12f559;
/// The dword monitor compares with 2016 (0x7e0) to set `DISPLAY_FLAG_559` (the sixth dword of an 8-dword block at
/// 0xae3158 the original copies whole to the stack) [inference].
const MONITOR_MODE_DWORD: usize = 0xae316c;
/// u32: a packet starting with command 0 (a new screen) arrived; the client then sends keepalives while it waits
/// [inference].
const CLIENT_KEEPALIVE_ARMED: usize = 0xdd6bf8;
/// "local player", "network msg", "new player joined" (the three callers named in the invalid player message), the
/// message format, "Processing, Please Wait ..." and "Contacting\nServer...".
const STR_LOCAL_PLAYER: usize = 0xa7de14;
const STR_NETWORK_MSG: usize = 0xa7de24;
const STR_NEW_PLAYER_JOINED: usize = 0xa7de30;
const STR_INVALID_PLAYER: usize = 0xa7dde4;
const STR_PROCESSING: usize = 0xa7dda0;
const STR_CONTACTING_SERVER: usize = 0xa7de44;
/// Exe routines the loops call by address (most are Rust already, through their detours).
const FN_SOUNDPLAYER_TAKE_FINISHED_SLOT: usize = 0x84cf00; // thiscall(player) -> al: a finished slot, −1 none [inference]
const FN_NETWORK_RECEIVE_PACKET: usize = 0x77af20; // thiscall(network, out, *player) ret 8
const FN_DISPLAY_POLL_INPUT: usize = 0x55f620; // thiscall(display) -> the clicked item, −1 none
const FN_DISPLAY_LOG_SCREEN_SENT: usize = 0x5622a0; // thiscall(display, network, player) ret 8
const FN_DISPLAY_SEND_WIDGET_CLICK: usize = 0x568560; // thiscall(display, network, item) ret 8 -> ok
const FN_DISPLAY_SEND_KEEPALIVE: usize = 0x568470; // thiscall(display, network) ret 4
const FN_CLIENT_RELEASE_SLOT_SOCKET: usize = 0x77ac00; // thiscall(network, player) ret 4
const FN_SCREENMANAGER_SWITCH_TO_MANAGER: usize = 0x7ffcd0; // thiscall(manager, key) ret 4

/// A screen node's build, leave and exit functions, and its input function (the clicked item, −1 none -> a code).
type MonitorScreenFn = unsafe extern "C" fn();
type MonitorInputFn = unsafe extern "C" fn(u32) -> i32;

/// Runs the screen function at `field` of `node` when it is set.
unsafe fn run_node_fn(node: usize, field: usize) {
    let f = read_usize(node + field);
    if f != 0 {
        core::mem::transmute::<usize, MonitorScreenFn>(f)();
    }
}

/// The current screen's input function with `click`.
unsafe fn current_screen_input(sm: usize, click: u32) -> i32 {
    core::mem::transmute::<usize, MonitorInputFn>(read_usize(current_screen_node(sm) + screen_node_fields::INPUT))(click)
}

/// `Network::receive_packet` on the manager's session into the command buffer; the sender's player goes to
/// `SCRMAN_RECEIVED_PLAYER`. Nonzero when a packet came.
unsafe fn scrman_receive_packet(sm: usize) -> u32 {
    use screen_history as h;
    let receive: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NETWORK_RECEIVE_PACKET);
    receive(read_usize(sm + h::NETWORK), read_usize(sm + h::NET_BUFFER), sm + SCRMAN_RECEIVED_PLAYER)
}

/// `Display::poll_input`, with the whole eax the original passes on to the input function.
unsafe fn display_poll_input_raw(d: usize) -> u32 {
    core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize) -> u32>(FN_DISPLAY_POLL_INPUT)(d)
}

/// The menubar handler when the clicked item (its id, or `SCRMAN_DEFAULT_COMMAND_ID` for id 0 or no click) is the
/// menubar item (`SCRMAN_MENUBAR_ITEM_ID`); 0 otherwise.
unsafe fn menubar_handler_for(sm: usize, display: usize, click: u32) -> usize {
    let hook = read_usize(sm + SCRMAN_MENUBAR_HANDLER);
    if hook == 0 {
        return 0;
    }
    let mut id = if click as i16 != -1 { read_u16(widget_field(display, click as i16, display::ITEM_ID)) } else { 0 };
    if id == 0 {
        id = read_u16(sm + SCRMAN_DEFAULT_COMMAND_ID);
    }
    if read_i32(sm + SCRMAN_MENUBAR_ITEM_ID) == id as i16 as i32 { hook } else { 0 }
}

/// Stops a sound slot that has finished playing (`SoundPlayer` 0x84cf00, then `stop_slot`); the slot, or `None`.
unsafe fn stop_finished_sound() -> Option<u8> {
    let player = read_usize(SOUND_PLAYER);
    if player == 0 {
        return None;
    }
    let slot = core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize) -> u8>(FN_SOUNDPLAYER_TAKE_FINISHED_SLOT)(player);
    if slot == 0xff {
        return None;
    }
    // simplification: the original pushes the slot with a high part it does not set; stop_slot reads only the byte
    exe::soundplayer_stop_slot(player, slot as i8 as u32);
    Some(slot)
}

/// Rule: the screen loop. It builds and draws the screen of every channel that needs it, reads one input (a joined
/// player's channel, a network message or the next local channel), runs the current screen's input function and acts on
/// the code it returns, until every manager has finished (pressed Continue) or a screen returns −7; then it tells every
/// screen "Processing, Please Wait ..." and closes every screen history. Returns whether the last code was −5 (a screen
/// that ended by itself).
///
/// On entry `SCRMAN_IN_MONITOR` = 1, `DISPLAY_FLAG_559` of the echo and the local Display = whether
/// `MONITOR_MODE_DWORD` is 2016, the local Display's `SCREEN_CLEARED` = 0 and the start time is read (`time`).
/// `monitor_mark_records` and `monitor_drain_network` run once. Each pass: the code is 0, `monitor_build_screens`,
/// `pump_messages`, `monitor_pick_channel`; with a channel, `monitor_run_channel` (which also says whether every manager
/// is done). The loop ends when every manager is done or the code is −7. Then `monitor_closing_packet`,
/// `monitor_deliver_closing_packet` and `monitor_close_all_records`; `CURRENT_SLOT` = 0, the local Display's
/// `SCREEN_CLEARED` = 1, `SCRMAN_IN_MONITOR` = 0.
/// Original: `ScreenManager::monitor` (0x7fc7e0, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_monitor(sm: usize) -> u32 {
    use screen_history as h;
    covered!("tel45_1 the screen loop");
    write_u32(sm + SCRMAN_IN_MONITOR, 1);
    let mode = (read_i32(MONITOR_MODE_DWORD) == 0x7e0) as u32;
    write_u32(sm + SCRMAN_ECHO_DISPLAY + DISPLAY_FLAG_559, mode);
    write_u32(sm + h::LOCAL_DISPLAY + DISPLAY_FLAG_559, mode);
    write_u32(sm + h::LOCAL_DISPLAY + display_list::SCREEN_CLEARED, 0);
    let mut start = 0i32;
    exe::at_946ebb(&mut start);
    monitor_mark_records(sm, start as u32);
    monitor_drain_network(sm);
    let mut scan = 0i32;
    let mut finished = 0u32;
    let mut code;
    loop {
        code = 0;
        monitor_build_screens(sm);
        exe::pump_messages();
        match monitor_pick_channel(sm, &mut scan, &mut finished) {
            MonitorPick::Restart => continue,
            MonitorPick::EndCheck => {}
            MonitorPick::Run(click, who) => code = monitor_run_channel(sm, click, who, start, &mut finished),
        }
        if finished != 0 || code == -7 {
            break;
        }
    }
    monitor_closing_packet(sm);
    monitor_deliver_closing_packet(sm);
    monitor_close_all_records(sm);
    write_u16(sm + screen_manager::CURRENT_SLOT, 0);
    write_u32(sm + h::LOCAL_DISPLAY + display_list::SCREEN_CLEARED, 1);
    write_u32(sm + SCRMAN_IN_MONITOR, 0);
    (code == -5) as u32
}

/// What `monitor_pick_channel` found: nothing this pass (straight to the next pass, or to the end check), or the
/// channel now current with its click and the name the invalid player message uses.
enum MonitorPick {
    Restart,
    EndCheck,
    Run(u32, usize),
}

/// Every record of every channel: an inactive record (`ACTIVE_FLAG` 0) is marked away and done; an active one not away
/// and not done, stamped with the start time (`SCRMAN_RECORD_STAMP`), and swapped to the front when record 0 is away.
unsafe fn monitor_mark_records(sm: usize, start: u32) {
    use screen_history as h;
    for c in 0..0x10i16 {
        let mut k = 0i16;
        while k < scrman_channel_count(sm, c) {
            let r = scrman_channel_record(sm, c, k);
            if read_u32(r + h::ACTIVE_FLAG) == 0 {
                write_u32(r + h::AWAY, 1);
                write_u32(r + h::DONE, 1);
            } else {
                write_u32(r + h::AWAY, 0);
                write_u32(r + h::DONE, 0);
                write_u32(r + h::NETWORK_PLAYER + SCRMAN_RECORD_STAMP, start);
                if read_u32(scrman_channel_record(sm, c, 0) + h::AWAY) == 1 {
                    covered!("tel45_1 an active record goes to the front");
                    exe::utils_swap_bytes(channel_record_at(sm, c as usize, 0) as *mut i32, channel_record_at(sm, c as usize, k as usize) as *mut i32, 0x30);
                }
            }
            k += 1;
        }
    }
}

/// With a network session, the packets waiting are read and dropped.
unsafe fn monitor_drain_network(sm: usize) {
    while read_usize(sm + screen_history::NETWORK) != 0 && scrman_receive_packet(sm) != 0 {}
}

/// `CURRENT_SLOT` walks the 16 channels: a channel whose record 0 has `ACTIVE_FLAG` set gets its screen built
/// (`monitor_build_screen`), or, with no records, the flag cleared.
unsafe fn monitor_build_screens(sm: usize) {
    use screen_history as h;
    write_u16(sm + screen_manager::CURRENT_SLOT, 0);
    loop {
        let slot = read_i16(sm + screen_manager::CURRENT_SLOT);
        if read_u32(screen_record(sm) + h::ACTIVE_FLAG) != 0 {
            if scrman_channel_count(sm, slot) <= 0 {
                write_u32(screen_record(sm) + h::ACTIVE_FLAG, 0);
            } else {
                monitor_build_screen(sm, slot);
            }
        }
        let next = read_i16(sm + screen_manager::CURRENT_SLOT).wrapping_add(1);
        write_i16(sm + screen_manager::CURRENT_SLOT, next);
        if next >= 0x10 {
            break;
        }
    }
}

/// Builds the current channel's screen: for channel 0 the mouse buttons are let go; `ACTIVE_FLAG` = 0, the Display
/// reset (`Display::reset(1)`), the current screen's build function runs with the Display's redraw blocked
/// (`DISPLAY_REDRAW_BLOCKED`), then `SCREEN_CLEARED` = 0 and the screen drawn (`Display::draw_screen` −1 ×4) for a
/// local record, or logged as sent (`Display::log_screen_sent`) for a remote one.
unsafe fn monitor_build_screen(sm: usize, slot: i16) {
    use screen_history as h;
    if slot == 0 {
        write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
        write_u32(DISPLAY_RIGHT_BUTTON, 0);
    }
    write_u32(screen_record(sm) + h::ACTIVE_FLAG, 0);
    exe::display_reset(screens_current_display(sm), 1);
    write_u32(screens_current_display(sm) + DISPLAY_REDRAW_BLOCKED, 1);
    core::mem::transmute::<usize, MonitorScreenFn>(read_usize(current_screen_node(sm) + screen_node_fields::BUILD))();
    write_u32(screens_current_display(sm) + DISPLAY_REDRAW_BLOCKED, 0);
    write_u32(screens_current_display(sm) + display_list::SCREEN_CLEARED, 0);
    let player = read_u16(screen_record(sm) + h::NETWORK_PLAYER);
    if player == 0 {
        exe::display_draw_screen(screens_current_display(sm), u32::MAX, u32::MAX, u32::MAX, u32::MAX);
    } else {
        covered!("tel45_1 a remote screen sent");
        let log: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_DISPLAY_LOG_SCREEN_SENT);
        log(screens_current_display(sm), read_usize(sm + h::NETWORK), player as u32);
    }
}

/// Which channel's input this pass reads.
///
/// A channel a joined player asked for (`SCRMAN_NEXT_CHANNEL`): with no records the request is dropped (and the "all
/// done" mark cleared) and the pass starts again; otherwise it becomes current, with no click. Else, with a network
/// session and a packet received: the channel whose record 0 is the sender (none: to the end check) becomes current,
/// and its Display applies the packet (`Display::process_command_packet`; the item clicked) unless the record is away.
/// Else the next local channel after `scan` with records and record 0 not away becomes current (`scan` follows it;
/// none: channel 0 and `scan` 0), and only channel 0, when its record 0 is not away, reads its Display's click
/// (`Display::poll_input`).
unsafe fn monitor_pick_channel(sm: usize, scan: &mut i32, finished: &mut u32) -> MonitorPick {
    use screen_history as h;
    let next = read_i16(sm + SCRMAN_NEXT_CHANNEL);
    if next != -1 {
        if scrman_channel_count(sm, next) <= 0 {
            write_i16(sm + SCRMAN_NEXT_CHANNEL, -1);
            *finished = 0;
            return MonitorPick::Restart;
        }
        covered!("tel45_1 a joined player's channel");
        write_i16(sm + screen_manager::CURRENT_SLOT, next);
        write_i16(sm + SCRMAN_NEXT_CHANNEL, -1);
        return MonitorPick::Run(u32::MAX, STR_NEW_PLAYER_JOINED);
    }
    if read_usize(sm + h::NETWORK) != 0 && scrman_receive_packet(sm) != 0 {
        covered!("tel45_1 a network message");
        let from = read_u16(sm + SCRMAN_RECEIVED_PLAYER);
        let Some(c) = (0..0x10i16).find(|&c| scrman_channel_count(sm, c) != 0 && read_u16(scrman_channel_record(sm, c, 0) + h::NETWORK_PLAYER) == from) else {
            return MonitorPick::EndCheck;
        };
        write_i16(sm + screen_manager::CURRENT_SLOT, c);
        let mut click = u32::MAX;
        if read_u32(screen_record(sm) + h::AWAY) == 0 {
            let apply: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(FN_DISPLAY_PROCESS_COMMAND_PACKET);
            click = apply(screens_current_display(sm), read_usize(sm + h::NETWORK), from as u32, sm + h::NET_BUFFER);
        }
        return MonitorPick::Run(click, STR_NETWORK_MSG);
    }
    let mut s = *scan + 1;
    loop {
        write_i16(sm + screen_manager::CURRENT_SLOT, s as i16);
        if s as i16 >= 0x10 {
            *scan = 0;
            write_i16(sm + screen_manager::CURRENT_SLOT, 0);
            break;
        }
        if scrman_channel_count(sm, s as i16) != 0 && read_u32(scrman_channel_record(sm, s as i16, 0) + h::AWAY) == 0 {
            *scan = s;
            break;
        }
        s += 1;
    }
    let mut click = u32::MAX;
    if read_i16(sm + screen_manager::CURRENT_SLOT) == 0 && read_u32(sm + h::AWAY) == 0 {
        click = display_poll_input_raw(read_usize(sm + screen_manager::DISPLAY));
    }
    MonitorPick::Run(click, STR_LOCAL_PLAYER)
}

/// Runs the current channel's input and acts on it; returns the code (and sets `finished`).
///
/// A current channel out of range, with no records or with no screen: the invalid player message (`who`) and the code
/// stays 0. Otherwise the code is −5 when `monitor_auto_continue_due`; else, for a record not away, −4 after the menubar
/// handler runs with 0 when the click is on the menubar item (`menubar_handler_for`), or the current screen's input
/// function with the click; an away record keeps 0. A finished sound slot is stopped, `monitor_handle_code` acts on
/// the code, `finished` = `monitor_all_done` and `SCRMAN_LOOP_CALLBACK` runs when set.
unsafe fn monitor_run_channel(sm: usize, click: u32, who: usize, start: i32, finished: &mut u32) -> i32 {
    use screen_history as h;
    let slot = read_i16(sm + screen_manager::CURRENT_SLOT);
    if slot < 0 || scrman_channel_count(sm, slot) <= 0 || current_screen_node(sm) == 0 {
        monitor_invalid_player(who);
        return 0;
    }
    let mut code = 0;
    if monitor_auto_continue_due(sm, click, start) {
        covered!("tel45_1 the screen ends by itself");
        code = -5;
    } else if read_u32(screen_record(sm) + h::AWAY) == 0 {
        let hook = menubar_handler_for(sm, screens_current_display(sm), click);
        code = if hook != 0 {
            covered!("tel45_1 the menubar item");
            core::mem::transmute::<usize, unsafe extern "C" fn(u32)>(hook)(0);
            -4
        } else {
            current_screen_input(sm, click)
        };
    }
    stop_finished_sound();
    code = monitor_handle_code(sm, code, click);
    *finished = monitor_all_done(sm);
    let callback = read_usize(sm + SCRMAN_LOOP_CALLBACK);
    if callback != 0 {
        core::mem::transmute::<usize, MonitorScreenFn>(callback)();
    }
    code
}

/// With the automatic Continue on (`SCRMAN_AUTO_CONTINUE_IDLE` seconds): the last click time is set when unset and on
/// every click; due when the idle seconds have passed since the last click and `SCRMAN_AUTO_CONTINUE_MIN` seconds since
/// monitor started.
unsafe fn monitor_auto_continue_due(sm: usize, click: u32, start: i32) -> bool {
    let idle = read_u16(sm + SCRMAN_AUTO_CONTINUE_IDLE);
    if idle == 0 {
        return false;
    }
    let last = sm + SCRMAN_AUTO_CONTINUE_LAST;
    if read_i32(last) == 0 {
        exe::at_946ebb(last as *mut i32);
    }
    if click as i16 != -1 {
        exe::at_946ebb(last as *mut i32);
    }
    let mut now = 0i32;
    exe::at_946ebb(&mut now);
    now >= (idle as i32).wrapping_add(read_i32(last)) && now >= (read_u16(sm + SCRMAN_AUTO_CONTINUE_MIN) as i32).wrapping_add(start)
}

/// The invalid player message (`who` in "SCREEN_MANAGER::monitor() - invalid player (%s)", into the text buffer; the
/// box does not show it) and the box `scrman.cpp 455`.
unsafe fn monitor_invalid_player(who: usize) {
    covered!("tel45_1 invalid player");
    let format: Tel28FormatFn = core::mem::transmute(FN_CRT_SPRINTF);
    format(GUI_SCRATCH_TEXT, STR_INVALID_PLAYER, who);
    tel26_error_box(SCRMAN_CPP, 0x1c7);
}

/// Acts on a screen's code, as many times as a going back asks the screen's input again.
///
/// −12 first sends the acknowledge (`monitor_acknowledge`) and then acts as any other code. −13, −8, −6, −5: the
/// screen ends (`monitor_finish_screen`). −11, −4: the screen is built again (`ACTIVE_FLAG`). −10: back to before the
/// last single-instance run (`monitor_back_to_flagged`), −9: back one screen (`monitor_back`); both ask the input again
/// and a nonzero answer is acted on in turn. −3 forward, −2 back without dropping (not from a single-instance screen),
/// −1 to the first screen: the leave function runs and, when there is a screen to go to, it becomes current and is
/// built again. Any other code: with a click and no rebuild pending, the screen is redrawn
/// (`ScreenManager::redraw_screen(0, 0)`).
unsafe fn monitor_handle_code(sm: usize, mut code: i32, click: u32) -> i32 {
    use screen_history as h;
    use screen_node_fields as n;
    loop {
        if code == -12 {
            monitor_acknowledge(sm);
        }
        match code {
            -13 | -8 | -6 | -5 => return monitor_finish_screen(sm, code),
            -11 | -4 => {
                write_u32(screen_record(sm) + h::ACTIVE_FLAG, 1);
                return code;
            }
            -10 | -9 => {
                code = if code == -9 { monitor_back(sm) } else { monitor_back_to_flagged(sm) };
                if code == 0 {
                    return 0;
                }
            }
            -3 => {
                run_node_fn(current_screen_node(sm), n::LEAVE);
                let next = read_usize(current_screen_node(sm) + n::NEXT);
                if next != 0 {
                    write_usize(screen_record(sm) + h::CURRENT_NODE, next);
                    write_u32(screen_record(sm) + h::ACTIVE_FLAG, 1);
                }
                return code;
            }
            -2 => {
                run_node_fn(current_screen_node(sm), n::LEAVE);
                let node = current_screen_node(sm);
                let prev = read_usize(node + n::PREV);
                if prev != 0 && read_u32(node + n::FLAG) == 0 {
                    write_usize(screen_record(sm) + h::CURRENT_NODE, prev);
                    write_u32(screen_record(sm) + h::ACTIVE_FLAG, 1);
                }
                return code;
            }
            -1 => {
                run_node_fn(current_screen_node(sm), n::LEAVE);
                let first = read_usize(screen_record(sm) + h::FIRST_NODE);
                if first != 0 {
                    write_usize(screen_record(sm) + h::CURRENT_NODE, first);
                    write_u32(screen_record(sm) + h::ACTIVE_FLAG, 1);
                }
                return code;
            }
            _ => {
                if click as i16 != -1 && read_u32(screen_record(sm) + h::ACTIVE_FLAG) == 0 {
                    exe::screenmanager_redraw_screen(sm, 0, 0);
                }
                return code;
            }
        }
    }
}

/// Code −12: the acknowledge packet (command 4, `scrman_command_packet`) sent to the player of the last received
/// packet (`Network::send_packet`, the result not checked).
unsafe fn monitor_acknowledge(sm: usize) {
    use screen_history as h;
    covered!("tel45_1 network acknowledge");
    scrman_command_packet(sm, 4);
    exe::network_send_packet(read_usize(sm + h::NETWORK), read_usize(sm + h::NET_BUFFER), read_u32(sm + h::NET_BUFFER_POS),
        read_u16(sm + SCRMAN_RECEIVED_PLAYER) as u32);
}

/// The current screen's input asked again with −1 (`SCRMAN_INPUT_AGAIN` = 1 meanwhile): an answer of 0 marks the
/// screen to be built again (`ACTIVE_FLAG`).
unsafe fn monitor_input_again(sm: usize) -> i32 {
    write_u32(sm + SCRMAN_INPUT_AGAIN, 1);
    let code = current_screen_input(sm, u32::MAX);
    write_u32(sm + SCRMAN_INPUT_AGAIN, 0);
    if code == 0 {
        write_u32(screen_record(sm) + screen_history::ACTIVE_FLAG, 1);
    }
    code
}

/// Drops the last history node: its flag lowers the flagged count, the node before it becomes the current and the last
/// one, the node is freed (`free_screen_node`) and the count goes down. `None` when it has no node before it (the
/// flagged count is then 0 and nothing is dropped).
unsafe fn monitor_drop_last_node(sm: usize, node: usize) -> Option<()> {
    use screen_history as h;
    use screen_node_fields as n;
    if read_u32(node + n::FLAG) != 0 {
        write_i16(screen_record(sm) + h::FLAGGED_COUNT, read_i16(screen_record(sm) + h::FLAGGED_COUNT).wrapping_sub(1));
    }
    let prev = read_usize(node + n::PREV);
    if prev == 0 {
        write_u16(screen_record(sm) + h::FLAGGED_COUNT, 0);
        return None;
    }
    write_usize(screen_record(sm) + h::CURRENT_NODE, prev);
    write_usize(screen_record(sm) + h::LAST_NODE, prev);
    write_usize(prev + n::NEXT, 0);
    free_screen_node(node);
    write_i16(screen_record(sm) + h::NODE_COUNT, read_i16(screen_record(sm) + h::NODE_COUNT).wrapping_sub(1));
    Some(())
}

/// Code −9, back one screen: the leave function runs; the last nodes are dropped (`monitor_drop_last_node`) up to and
/// including the first single-instance one; a node with nothing before it gives −5 instead. Then the input again
/// (`monitor_input_again`).
unsafe fn monitor_back(sm: usize) -> i32 {
    use screen_history as h;
    use screen_node_fields as n;
    covered!("tel45_1 back one screen");
    run_node_fn(current_screen_node(sm), n::LEAVE);
    let mut node = read_usize(screen_record(sm) + h::LAST_NODE);
    let mut flag = 0;
    while node != 0 && flag == 0 {
        flag = read_u32(node + n::FLAG);
        if monitor_drop_last_node(sm, node).is_none() {
            return -5;
        }
        node = read_usize(screen_record(sm) + h::LAST_NODE);
    }
    monitor_input_again(sm)
}

/// Code −10, back to before the last single-instance run: the leave function runs; from the last node back, the first
/// node of the last run of flagged nodes is found (none: straight to the input again). When it is the first node: the
/// flagged count 0 and −5. Otherwise the history ends before it and it and every node after it are freed (the flagged
/// and node counts go down), the current node is the new last one, and the input runs again.
unsafe fn monitor_back_to_flagged(sm: usize) -> i32 {
    use screen_history as h;
    use screen_node_fields as n;
    covered!("tel45_1 back before the flagged screens");
    run_node_fn(current_screen_node(sm), n::LEAVE);
    let mut node = read_usize(screen_record(sm) + h::LAST_NODE);
    while node != 0 {
        if read_u32(node + n::FLAG) != 0 {
            let prev = read_usize(node + n::PREV);
            if prev == 0 || read_u32(prev + n::FLAG) == 0 {
                break;
            }
        }
        node = read_usize(node + n::PREV);
    }
    if node != 0 {
        if node == read_usize(screen_record(sm) + h::FIRST_NODE) {
            write_u16(screen_record(sm) + h::FLAGGED_COUNT, 0);
            return -5;
        }
        let prev = read_usize(node + n::PREV);
        write_usize(screen_record(sm) + h::LAST_NODE, prev);
        write_usize(prev + n::NEXT, 0);
        while node != 0 {
            if read_u32(node + n::FLAG) != 0 {
                write_i16(screen_record(sm) + h::FLAGGED_COUNT, read_i16(screen_record(sm) + h::FLAGGED_COUNT).wrapping_sub(1));
            }
            write_usize(screen_record(sm) + h::CURRENT_NODE, read_usize(node + n::NEXT));
            free_screen_node(node);
            write_i16(screen_record(sm) + h::NODE_COUNT, read_i16(screen_record(sm) + h::NODE_COUNT).wrapping_sub(1));
            node = current_screen_node(sm);
        }
        write_usize(screen_record(sm) + h::CURRENT_NODE, read_usize(screen_record(sm) + h::LAST_NODE));
    }
    monitor_input_again(sm)
}

/// Codes −13, −8, −6, −5: the screen ends. Returns the code, 0 when a single-instance screen takes over.
///
/// With flagged screens in the history: from the last node back, the first flagged one with no exit function (or any
/// flagged one for −13) becomes current and is built again, the code becomes 0, and only the waiting manager switch
/// follows. None found: the code also becomes 0 (defect of the original: −6 and −8 then lose their effect) and the
/// current node goes back to what it was. Then: −6 marks the record done and away; otherwise a record not away has its
/// done flag toggled (Continue pressed or taken back). The record is built again. Walking the channel's records from
/// 0: for −8 each record not away takes record 0's done flag and is built again; the first record neither away nor done
/// is swapped to the front (`utils_swap_bytes`) and built again. Last, `monitor_switch_to_waiting_manager`.
unsafe fn monitor_finish_screen(sm: usize, mut code: i32) -> i32 {
    use screen_history as h;
    use screen_node_fields as n;
    covered!("tel45_1 the screen ends");
    let saved = current_screen_node(sm);
    if read_i16(screen_record(sm) + h::FLAGGED_COUNT) > 0 {
        write_usize(screen_record(sm) + h::CURRENT_NODE, read_usize(screen_record(sm) + h::LAST_NODE));
        let mut found = false;
        loop {
            let node = current_screen_node(sm);
            if node == 0 {
                break;
            }
            if read_u32(node + n::FLAG) != 0 && (read_usize(node + n::EXIT) == 0 || code == -13) {
                write_u32(screen_record(sm) + h::ACTIVE_FLAG, 1);
                found = true;
                break;
            }
            write_usize(screen_record(sm) + h::CURRENT_NODE, read_usize(node + n::PREV));
        }
        code = 0;
        if found {
            covered!("tel45_1 a single-instance screen takes over");
            monitor_switch_to_waiting_manager(sm);
            return 0;
        }
    }
    write_usize(screen_record(sm) + h::CURRENT_NODE, saved);
    let rec = screen_record(sm);
    if code == -6 {
        write_u32(rec + h::DONE, 1);
        write_u32(rec + h::AWAY, 1);
    } else if read_u32(rec + h::AWAY) == 0 {
        write_u32(rec + h::DONE, (read_u32(rec + h::DONE) == 0) as u32);
    }
    write_u32(rec + h::ACTIVE_FLAG, 1);
    let slot = read_i16(sm + screen_manager::CURRENT_SLOT);
    let mut k = 0i16;
    while k < scrman_channel_count(sm, slot) {
        let r = scrman_channel_record(sm, slot, k);
        if code == -8 && read_u32(r + h::AWAY) == 0 {
            write_u32(r + h::DONE, read_u32(rec + h::DONE));
            write_u32(r + h::ACTIVE_FLAG, 1);
        }
        if read_u32(r + h::AWAY) == 0 && read_u32(r + h::DONE) == 0 {
            exe::utils_swap_bytes(channel_record_at(sm, slot as usize, 0) as *mut i32, channel_record_at(sm, slot as usize, k as usize) as *mut i32, 0x30);
            write_u32(rec + h::ACTIVE_FLAG, 1);
            break;
        }
        k += 1;
    }
    monitor_switch_to_waiting_manager(sm);
    code
}

/// A manager waiting for the screens (`SCRMAN_WAITING_MANAGER`, nonzero) gets them (`ScreenManager::switch_to_manager`)
/// and the wait is cleared.
unsafe fn monitor_switch_to_waiting_manager(sm: usize) {
    let key = read_u32(sm + SCRMAN_WAITING_MANAGER);
    if key != 0 {
        covered!("tel45_1 switch to the waiting manager");
        core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32)>(FN_SCREENMANAGER_SWITCH_TO_MANAGER)(sm, key);
        write_u32(sm + SCRMAN_WAITING_MANAGER, 0);
    }
}

/// 1 when every channel with records has record 0 done, else 0.
unsafe fn monitor_all_done(sm: usize) -> u32 {
    (0..0x10i16).all(|c| scrman_channel_count(sm, c) == 0 || read_u32(scrman_channel_record(sm, c, 0) + screen_history::DONE) != 0) as u32
}

/// The closing status packet: "Processing, Please Wait ..." translated into the text buffer (`langlib_sprintf`), then
/// the status message packet with 0 and 0 (`status_message_packet`: command 7, the two dwords, the text).
unsafe fn monitor_closing_packet(sm: usize) {
    let format: Tel28FormatFn = core::mem::transmute(FN_LANGLIB_SPRINTF);
    format(GUI_SCRATCH_TEXT, STR_PROCESSING);
    status_message_packet(sm, GUI_SCRATCH_TEXT, 0, 0); // tel45_1: progress 0 of 0
}

/// Every channel with records gets the closing packet: applied to the local Display (`Display::process_command_packet`
/// with player 0) when record 0 is local, sent to the player otherwise (`Network::send_packet`, failing: the box
/// `scrman.cpp 869`).
unsafe fn monitor_deliver_closing_packet(sm: usize) {
    use screen_history as h;
    for c in 0..0x10i16 {
        if scrman_channel_count(sm, c) <= 0 {
            continue;
        }
        let player = read_u16(scrman_channel_record(sm, c, 0) + h::NETWORK_PLAYER);
        if player == 0 {
            exe::display_process_command_packet(sm + h::LOCAL_DISPLAY, read_usize(sm + h::NETWORK), 0, sm + h::NET_BUFFER);
        } else if exe::network_send_packet(read_usize(sm + h::NETWORK), read_usize(sm + h::NET_BUFFER), read_u32(sm + h::NET_BUFFER_POS), player as u32) == 0 {
            tel26_error_box(SCRMAN_CPP, 0x365);
        }
    }
}

/// `CURRENT_SLOT` walks the 16 channels; each record of a channel with records is closed (`monitor_close_record`),
/// record k > 0 swapped into place 0 first (the records end rotated).
unsafe fn monitor_close_all_records(sm: usize) {
    write_u16(sm + screen_manager::CURRENT_SLOT, 0);
    loop {
        let slot = read_i16(sm + screen_manager::CURRENT_SLOT);
        if scrman_channel_count(sm, slot) != 0 {
            monitor_close_record(sm);
            let mut k = 1i16;
            loop {
                let slot = read_i16(sm + screen_manager::CURRENT_SLOT);
                if k >= scrman_channel_count(sm, slot) {
                    break;
                }
                exe::utils_swap_bytes(channel_record_at(sm, slot as usize, 0) as *mut i32, channel_record_at(sm, slot as usize, k as usize) as *mut i32, 0x30);
                monitor_close_record(sm);
                k += 1;
            }
        }
        let next = read_i16(sm + screen_manager::CURRENT_SLOT).wrapping_add(1);
        write_i16(sm + screen_manager::CURRENT_SLOT, next);
        if next >= 0x10 {
            break;
        }
    }
}

/// Closes the current record: with a current screen, the exit function of every node from the first runs (the current
/// node walking along), the current node comes back, its leave function runs, the Display is reset
/// (`Display::reset(1)`) and there is no current node; `ACTIVE_FLAG` = 0 and the history freed
/// (`ScreenManager::free_all_screens`).
unsafe fn monitor_close_record(sm: usize) {
    use screen_history as h;
    use screen_node_fields as n;
    let saved = current_screen_node(sm);
    if saved != 0 {
        write_usize(screen_record(sm) + h::CURRENT_NODE, read_usize(screen_record(sm) + h::FIRST_NODE));
        loop {
            let node = current_screen_node(sm);
            if node == 0 {
                break;
            }
            run_node_fn(node, n::EXIT);
            write_usize(screen_record(sm) + h::CURRENT_NODE, read_usize(current_screen_node(sm) + n::NEXT));
        }
        write_usize(screen_record(sm) + h::CURRENT_NODE, saved);
        run_node_fn(current_screen_node(sm), n::LEAVE);
        exe::display_reset(screens_current_display(sm), 1);
        write_usize(screen_record(sm) + h::CURRENT_NODE, 0);
    }
    write_u32(screen_record(sm) + h::ACTIVE_FLAG, 0);
    exe::screenmanager_free_all_screens(sm);
}

/// Rule: the remote client loop of a network game. Channel 0 shows the echo Display; the loop applies every packet the
/// server sends to it and sends the local clicks to the server, until the server says the game is over or a click
/// cannot be sent; then the screens are told the game left and channel 0 shows the local Display again.
///
/// Without a session: the box `scrman.cpp 977` and nothing more. Otherwise `CURRENT_SLOT` = 0 and channel 0's Display
/// = the echo Display (`SCRMAN_ECHO_DISPLAY`). Each pass: `client_report_finished_sound`; a packet received sets
/// `CLIENT_KEEPALIVE_ARMED` when its command is 0 and is applied to the current Display (`Display::process_command_packet`):
/// the answer −2 (game over) releases the sender's socket (`Network::release_slot_socket`) and ends the loop; else the
/// last contact time is read. `pump_messages`. While the Display is not ready (`display_life::DWORD_551` 0) and the
/// keepalives are armed, a keepalive (`Display::send_keepalive`) goes when more than 30 s passed since the last contact
/// (or the clock went back), and the time is read again. When ready: `CLIENT_KEEPALIVE_ARMED` = 0 and the click read
/// (`Display::poll_input`); a click (not −1) other than the previous one: the menubar dialog when it is on the menubar
/// item (`client_menubar_dialog`), "Contacting Server..." with `SCRMAN_JOIN_BY_ADDRESS` (`client_show_contacting`),
/// and the click sent (`Display::send_widget_click`; failing, the loop ends without releasing the socket). At the end
/// `client_leave_packet` 0x13 and 0x14 applied to the echo Display, and channel 0's Display = the local Display.
/// Defect of the original: the last contact time is a stack slot read before it is set when `CLIENT_KEEPALIVE_ARMED`
/// is left set by an earlier session (here it starts at 0).
/// Original: `ScreenManager::run_network_client` (0x7fdc00, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_run_network_client(sm: usize) {
    use screen_history as h;
    if read_usize(sm + h::NETWORK) == 0 {
        tel26_error_box(SCRMAN_CPP, 0x3d1);
        return;
    }
    covered!("tel45_1 the network client loop");
    write_u16(sm + screen_manager::CURRENT_SLOT, 0);
    write_usize(sm + screen_manager::DISPLAY, sm + SCRMAN_ECHO_DISPLAY);
    let mut previous = u32::MAX;
    let mut last_contact = 0i32;
    loop {
        client_report_finished_sound(sm);
        if scrman_receive_packet(sm) != 0 {
            if read_u8(read_usize(sm + h::NET_BUFFER)) == 0 {
                write_u32(CLIENT_KEEPALIVE_ARMED, 1);
            }
            let from = read_u16(sm + SCRMAN_RECEIVED_PLAYER);
            if exe::display_process_command_packet(screens_current_display(sm), read_usize(sm + h::NETWORK), from as u32, sm + h::NET_BUFFER) == 0xfffe {
                covered!("tel45_1 the server ends the game");
                let release: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CLIENT_RELEASE_SLOT_SOCKET);
                release(read_usize(sm + h::NETWORK), read_u16(sm + SCRMAN_RECEIVED_PLAYER) as u32);
                break;
            }
            exe::at_946ebb(&mut last_contact);
        }
        exe::pump_messages();
        let d = screens_current_display(sm);
        if read_u32(d + display_life::DWORD_551) == 0 {
            if read_u32(CLIENT_KEEPALIVE_ARMED) != 0 {
                let mut now = 0i32;
                exe::at_946ebb(&mut now);
                if now > last_contact.wrapping_add(30) || now < last_contact {
                    let keepalive: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_DISPLAY_SEND_KEEPALIVE);
                    keepalive(screens_current_display(sm), read_usize(sm + h::NETWORK));
                    exe::at_946ebb(&mut last_contact);
                }
            }
            continue;
        }
        write_u32(CLIENT_KEEPALIVE_ARMED, 0);
        let click = display_poll_input_raw(d);
        if click as i16 != -1 && click as i16 != previous as i16 {
            let hook = menubar_handler_for(sm, d, click);
            if hook != 0 {
                client_menubar_dialog(sm, hook, click);
            }
            if read_u32(sm + SCRMAN_JOIN_BY_ADDRESS) != 0 {
                client_show_contacting();
            }
            let send: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DISPLAY_SEND_WIDGET_CLICK);
            if send(screens_current_display(sm), read_usize(sm + h::NETWORK), click) == 0 {
                covered!("tel45_1 a click could not be sent");
                break;
            }
        }
        previous = click;
    }
    client_leave_packet(sm, 0x13);
    exe::display_process_command_packet(sm + SCRMAN_ECHO_DISPLAY, read_usize(sm + h::NETWORK), 0, sm + h::NET_BUFFER);
    client_leave_packet(sm, 0x14);
    exe::display_process_command_packet(sm + SCRMAN_ECHO_DISPLAY, read_usize(sm + h::NETWORK), 0, sm + h::NET_BUFFER);
    write_usize(sm + screen_manager::DISPLAY, sm + h::LOCAL_DISPLAY);
}

/// A finished sound slot (`stop_finished_sound`) is reported to the server: command 0xe and the slot into the network
/// send buffer (`NET_SEND_BUFFER`, from position 0; no room: the box `network.h 133`), sent on the session to the
/// channel being applied (`SCREEN_CHANNEL`; failing, the box `scrman.cpp 1006`).
unsafe fn client_report_finished_sound(sm: usize) {
    let Some(slot) = stop_finished_sound() else { return };
    covered!("tel45_1 a finished sound reported");
    write_u32(NET_SEND_BUFFER + network_buffer::POS, 0);
    for b in [0xe, slot] {
        if network_buffer_room(NET_SEND_BUFFER, 1, 0x85) {
            write_u8(network_buffer_cursor(NET_SEND_BUFFER), b);
            network_buffer_advance(NET_SEND_BUFFER, 1);
        }
    }
    if exe::network_send_packet(read_usize(sm + screen_history::NETWORK), read_usize(NET_SEND_BUFFER), read_u32(NET_SEND_BUFFER + network_buffer::POS),
        read_u16(SCREEN_CHANNEL) as u32) == 0 {
        tel26_error_box(SCRMAN_CPP, 0x3ee);
    }
}

/// The menubar dialog on the client: the handler runs with 1; then the current screen is built and drawn locally
/// (`Display::reset(1)`, the build function with the redraw blocked, `SCREEN_CLEARED` 0, `Display::draw_screen`) and its
/// input read (`pump_messages`, `Display::poll_input`, the input function) until it answers −9 (−11 builds it again).
/// The dialog's node, the last one, is dropped (the one before it becomes current and last, no check that there is
/// one) and freed, and the clicked item's id becomes `SCRMAN_MENUBAR_RESTORE_ID`.
unsafe fn client_menubar_dialog(sm: usize, hook: usize, click: u32) {
    use screen_history as h;
    use screen_node_fields as n;
    covered!("tel45_1 the menubar dialog on the client");
    core::mem::transmute::<usize, unsafe extern "C" fn(u32)>(hook)(1);
    'build: loop {
        exe::display_reset(screens_current_display(sm), 1);
        write_u32(screens_current_display(sm) + DISPLAY_REDRAW_BLOCKED, 1);
        core::mem::transmute::<usize, MonitorScreenFn>(read_usize(current_screen_node(sm) + n::BUILD))();
        write_u32(screens_current_display(sm) + DISPLAY_REDRAW_BLOCKED, 0);
        write_u32(screens_current_display(sm) + display_list::SCREEN_CLEARED, 0);
        exe::display_draw_screen(screens_current_display(sm), u32::MAX, u32::MAX, u32::MAX, u32::MAX);
        let mut code = 0;
        loop {
            if code == -11 {
                continue 'build;
            }
            exe::pump_messages();
            code = current_screen_input(sm, display_poll_input_raw(screens_current_display(sm)));
            if code == -9 {
                break 'build;
            }
        }
    }
    let node = read_usize(screen_record(sm) + h::LAST_NODE);
    write_usize(screen_record(sm) + h::CURRENT_NODE, read_usize(node + n::PREV));
    write_usize(screen_record(sm) + h::LAST_NODE, current_screen_node(sm));
    write_usize(read_usize(screen_record(sm) + h::LAST_NODE) + n::NEXT, 0);
    free_screen_node(node);
    write_i16(screen_record(sm) + h::NODE_COUNT, read_i16(screen_record(sm) + h::NODE_COUNT).wrapping_sub(1));
    write_u32(widget_field(screens_current_display(sm), click as i16, display::ITEM_ID), read_u32(sm + SCRMAN_MENUBAR_RESTORE_ID));
}

/// "Contacting\nServer..." in a box drawn straight on the screen: the rectangle (10, 550)-(80, 590) in the 0xaebdea
/// colour, the translated text in it in the 0xaebdec colour, and the rectangle flipped.
unsafe fn client_show_contacting() {
    // simplification: the original pushes the two colour words with high halves it does not set; both read the word
    exe::graphics_draw_rect(10, 0x226, 0x50, 0x24e, 0x10, read_u16(JOB_COLOUR_BDEA) as u32);
    let format: Tel28FormatFn = core::mem::transmute(FN_LANGLIB_SPRINTF);
    format(GUI_SCRATCH_TEXT, STR_CONTACTING_SERVER);
    exe::gui_draw_text_box(10, 0x226, 0x50, 0x24e, 0xc, 1, read_u16(JOB_COLOUR_BDEC) as u32, GUI_SCRATCH_TEXT, -1);
    exe::graphics_flip_rect(10, 0x226, 0x50, 0x24e);
}

/// A leaving packet into the manager's command buffer: `command` (`scrman_command_packet`) and the word 0xffff (box
/// line `network.h 134`).
unsafe fn client_leave_packet(sm: usize, command: u8) {
    scrman_command_packet(sm, command);
    scrman_net_put(sm, 0xffff, 2, 0x86); // tel45_1: all players
}

/// After the original ran: the copy rebuilds a packet the original left in the manager's command buffer (`build`, `need`
/// bytes, first byte `command`) and the bytes are compared; the buffer and the text buffer are restored. Skipped when
/// the buffer no longer starts with `command` (a later packet replaced it) or has no room (the box would show).
unsafe fn compare_rebuilt_packet(name: &str, sm: usize, need: usize, command: u8, build: impl FnOnce()) {
    use screen_history as h;
    let data = read_usize(sm + h::NET_BUFFER);
    if data == 0 || need > read_u32(sm + h::NET_BUFFER_CAPACITY) as usize || read_u8(data) != command {
        return;
    }
    let regions = [(sm + h::NET_BUFFER, 12), (data, need), (GUI_SCRATCH_TEXT, 0x100)];
    let saved: Vec<Vec<u8>> = regions.iter().map(|&(a, n)| core::slice::from_raw_parts(a as *const u8, n).to_vec()).collect();
    build();
    let len = read_u32(sm + h::NET_BUFFER_POS) as usize;
    let copy = core::slice::from_raw_parts(data as *const u8, len.min(need)).to_vec();
    for (&(a, _), s) in regions.iter().zip(&saved) {
        core::ptr::copy_nonoverlapping(s.as_ptr(), a as *mut u8, s.len());
    }
    SCOREBOARD.record(len == need && copy[..] == saved[1][..copy.len()], || format!("{}: original {:x?}, copy {:x?} (length {})", name, saved[1], copy, len));
}

/// Comparer for `screen_manager_monitor`: the loop builds, draws, reads the input and runs every screen, so only the
/// original runs. The plan checked afterwards: the result 0 or 1, `CURRENT_SLOT` 0, `SCRMAN_IN_MONITOR` 0, the local
/// Display's `SCREEN_CLEARED` 1, every record of every channel with no history and `ACTIVE_FLAG` 0; and the closing
/// packet rebuilt by the copy (`monitor_closing_packet`) against the one the original left in the buffer.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_monitor(sm: usize) -> u32 {
    use screen_history as h;
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(sm);
    let open = (0..0x10i16).flat_map(|c| (0..scrman_channel_count(sm, c).max(0)).map(move |k| (c, k))).find(|&(c, k)| {
        let b = scrman_channel_record(sm, c, k);
        read_usize(b + h::FIRST_NODE) != 0 || read_usize(b + h::LAST_NODE) != 0 || read_usize(b + h::CURRENT_NODE) != 0
            || read_i16(b + h::NODE_COUNT) != 0 || read_i16(b + h::FLAGGED_COUNT) != 0 || read_u32(b + h::ACTIVE_FLAG) != 0
    });
    let slot = read_i16(sm + screen_manager::CURRENT_SLOT);
    let ok = r <= 1 && slot == 0 && read_u32(sm + SCRMAN_IN_MONITOR) == 0 && read_u32(sm + h::LOCAL_DISPLAY + display_list::SCREEN_CLEARED) == 1
        && open.is_none();
    SCOREBOARD.record(ok, || format!("monitor: returned {:#x}, slot {}, in monitor {}, record still open (channel, record) {:?}", r, slot,
        read_u32(sm + SCRMAN_IN_MONITOR), open));
    compare_rebuilt_packet("monitor closing packet", sm, 12 + c_string_len(GUI_SCRATCH_TEXT), 7, || monitor_closing_packet(sm));
    r
}

/// Comparer for `screen_manager_run_network_client`: it reads and sends packets and draws, so only the original runs.
/// The plan checked afterwards (with a session): channel 0 shows the local Display again, and the last leaving packet
/// (0x14) rebuilt by the copy (`client_leave_packet`) against the one the original left.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_run_network_client(sm: usize) {
    use screen_history as h;
    let session = read_usize(sm + h::NETWORK);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(sm);
    if session == 0 {
        return;
    }
    let display = read_usize(sm + screen_manager::DISPLAY);
    SCOREBOARD.record(display == sm + h::LOCAL_DISPLAY, || format!("run_network_client: channel 0 shows {:#x}, not the local Display", display));
    compare_rebuilt_packet("run_network_client leaving packet", sm, 3, 0x14, || client_leave_packet(sm, 0x14));
}
