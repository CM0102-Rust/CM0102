use super::*;

/// the screen manager has no next screen to go forward to
/// Original: `ScreenManager::is_forward_unavailable` (0x7febb0, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn is_forward_unavailable(manager: usize) -> u32 {
    let slot = *((manager + screen_manager::CURRENT_SLOT) as *const i16) as isize;
    let block = core::ptr::read_unaligned((manager as isize + (screen_manager::PARAM_BLOCK as isize) + slot * 0x300) as *const usize);
    if block == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IS_FORWARD_UNAVAILABLE)(manager);
    }
    (*((block + screen_node::NEXT_NODE) as *const usize) == 0) as u32
}

/// Copies C string origem (with terminator) to the work text 0xde1f64.
pub(crate) unsafe fn copy_to_work_text(source: usize) {
    let mut i = 0;
    loop {
        let b = read_u8(source + i);
        write_u8(0xde1f64 + i, b);
        if b == 0 {
            break;
        }
        i += 1;
    }
}

/// Writes the competition name to 0xde1f64 for the panel: "Friendlies"/"Reserves" for those with more than one round,
/// else the name (+0x38).
pub(crate) unsafe fn panel_competition_name(c: Competition, rounds: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let more_of_one = read_i32(rounds + 8) > read_i32(rounds + 4);
    let key = read_usize(c.addr());
    if key == read_usize(KEY_CLUB_COMP_FRIENDLY) && more_of_one {
        text(0xde1f64, 0xa1631c);
    } else if key == read_usize(KEY_CLUB_COMP_RESERVE) && more_of_one {
        text(0xde1f64, 0xa16310);
    } else {
        copy_to_work_text(c.addr() + competition::SHORT_NAME);
    }
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

/// Comparer for `screen_manager_start_network_session`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_start_network_session(sm: usize, on_join: usize, on_leave: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_SCREEN_MANAGER_START_NETWORK_SESSION)(sm, on_join, on_leave);
    let net = read_usize(sm + screen_history::NETWORK);
    SCOREBOARD.record(read_usize(sm + SCRMAN_JOIN_HOOK) == on_join && read_usize(sm + SCRMAN_LEAVE_HOOK) == on_leave && (r == 0) == (net == 0),
        || format!("start_network_session: returned {:#x}, session {:#x}, hooks {:#x} {:#x}", r, net, read_usize(sm + SCRMAN_JOIN_HOOK), read_usize(sm + SCRMAN_LEAVE_HOOK)));
    r
}

/// Comparer for `scrman_add_player_record`: side by side over the records; with a join hook (it opens screens) only
/// the original.
pub(crate) unsafe extern "C" fn compare_scrman_add_player_record(player: u32) {
    let o = original_fn::<unsafe extern "C" fn(u32)>(IDX_SCRMAN_ADD_PLAYER_RECORD);
    let sm = G_SCREEN_MANAGER;
    let client = read_usize(G_SCREEN_MANAGER_NETWORK) != 0 && read_u32(G_SCREEN_MANAGER_NETWORK_HOST_FLAG) == 0;
    if !client && read_usize(sm + SCRMAN_JOIN_HOOK) != 0 {
        // simplification: the join hook opens screens: it does not run twice
        return o(player);
    }
    compare_scrman_records("scrman_add_player_record", sm, || o(player), || scrman_add_player_record(player));
}

/// Comparer for `scrman_remove_player_records`.
pub(crate) unsafe extern "C" fn compare_scrman_remove_player_records(player: u32) {
    original_fn::<unsafe extern "C" fn(u32)>(IDX_SCRMAN_REMOVE_PLAYER_RECORDS)(player);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_SCREEN_MANAGER_ADD_MANAGER_RECORD);
    if key == 0 && on_join != 0 {
        // simplification: the join hook opens screens: it does not run twice
        return o(sm, on_join, on_leave, key);
    }
    compare_scrman_records("screen_manager_add_manager_record", sm, || o(sm, on_join, on_leave, key), || screen_manager_add_manager_record(sm, on_join, on_leave, key));
}

/// Opens a screen for the current manager: a new history node after the current one (the nodes ahead of it dropped,
/// at most 100 kept), or, for a single-instance screen already open, a jump back to it.
///
/// A record with no key (`screen_history::KEY`) and no screen yet (`FIRST_NODE`) takes the default screen
/// (`DEFAULT_BUILD` and the next four dwords) when there is one, and its slot values afterwards. Without the build or
/// the input function: the original routine runs (error box `scrman.cpp 1261`, returns 0). With a current screen, the
/// flag set and a node with the flag and the same build function in the history: the current node becomes the last
/// flagged node from that one to the end; if its build function is not this one, the current node goes back to what it
/// was; returns 0 (nothing opened). Otherwise, when the current node is not the last one, the nodes after it are freed
/// (`free_screen_node`, the count goes down) and it becomes the last. With more than 100 nodes, the first is dropped
/// the same way, unless it is flagged (then 0). The current screen's leave function runs (cdecl, no arguments); the new
/// node (operator new 0x300, zeroed): build, leave, input, exit, flag, the picture "\"; linked after the last one (and
/// first when the history is empty), current; the flagged count (with the flag) and the node count go up; `AWAY` = 0,
/// `SCREEN_KEY` = −1, `SCREEN_KEY_DATA` = 0, `DONE` = 0 (with the flag), `ACTIVE_FLAG` = 1. From the default screen, the
/// default slot values go to the parameters 0.. (`exe::screenmanager_set_screen_param`, no copy). Returns 1.
/// Original: `ScreenManager::open_screen` (0x7fe600, scrman.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn screen_manager_open_screen(mgr: usize, build: usize, input: usize, flag: u32, leave: usize, exit: usize) -> u32 {
    use screen_history as h;
    use screen_node_fields as n;
    let rec = screen_record(mgr);
    let args = (build, input, flag, leave, exit);
    let (mut build, mut input, mut flag, mut leave, mut exit) = args;
    let mut from_default = false;
    if read_u32(rec + h::KEY) == 0 && read_usize(rec + h::FIRST_NODE) == 0 && read_usize(mgr + h::DEFAULT_BUILD) != 0 {
        build = read_usize(mgr + h::DEFAULT_BUILD);
        input = read_usize(mgr + h::DEFAULT_BUILD + 4);
        flag = read_u32(mgr + h::DEFAULT_BUILD + 8);
        leave = read_usize(mgr + h::DEFAULT_BUILD + 0xc);
        exit = read_usize(mgr + h::DEFAULT_BUILD + 0x10);
        from_default = true;
    }
    if build == 0 || input == 0 {
        // the error box: through the original, called with the caller's arguments (it reaches the same box)
        let (build, input, flag, leave, exit) = (args.0, args.1, args.2, args.3, args.4);
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, usize) -> u32>(IDX_SCREEN_MANAGER_OPEN_SCREEN)(mgr, build, input, flag, leave, exit);
    }
    let current = read_usize(rec + h::CURRENT_NODE);
    if current != 0 && flag != 0 {
        let mut node = read_usize(rec + h::FIRST_NODE);
        while node != 0 {
            if read_u32(node + n::FLAG) != 0 && read_usize(node + n::BUILD) == build {
                covered!("tel29_1 single screen already open");
                let mut m = node;
                while m != 0 {
                    if read_u32(m + n::FLAG) != 0 {
                        write_usize(rec + h::CURRENT_NODE, m);
                    }
                    m = read_usize(m + n::NEXT);
                }
                if read_usize(read_usize(rec + h::CURRENT_NODE) + n::BUILD) != build {
                    write_usize(rec + h::CURRENT_NODE, current);
                }
                return 0;
            }
            node = read_usize(node + n::NEXT);
        }
    }
    if current != read_usize(rec + h::LAST_NODE) {
        covered!("tel29_1 forward screens dropped");
        write_usize(rec + h::LAST_NODE, current);
        let mut victim = read_usize(read_usize(rec + h::CURRENT_NODE) + n::NEXT);
        write_usize(read_usize(rec + h::LAST_NODE) + n::NEXT, 0);
        while victim != 0 {
            write_usize(rec + h::CURRENT_NODE, read_usize(victim + n::NEXT));
            free_screen_node(victim);
            write_i16(rec + h::NODE_COUNT, read_i16(rec + h::NODE_COUNT).wrapping_sub(1));
            victim = read_usize(rec + h::CURRENT_NODE);
        }
        write_usize(rec + h::CURRENT_NODE, read_usize(rec + h::LAST_NODE));
    }
    if read_i16(rec + h::NODE_COUNT) > 100 { // tel29_1: at most 100 screens in the history
        let first = read_usize(rec + h::FIRST_NODE);
        if read_u32(first + n::FLAG) != 0 {
            return 0;
        }
        covered!("tel29_1 oldest screen dropped");
        write_usize(rec + h::FIRST_NODE, read_usize(first + n::NEXT));
        write_usize(read_usize(rec + h::FIRST_NODE) + n::PREV, 0);
        free_screen_node(first);
        write_i16(rec + h::NODE_COUNT, read_i16(rec + h::NODE_COUNT).wrapping_sub(1));
    }
    let current = read_usize(rec + h::CURRENT_NODE);
    if current != 0 && read_usize(current + n::LEAVE) != 0 {
        let leave_fn: unsafe extern "C" fn() = core::mem::transmute(read_usize(current + n::LEAVE));
        leave_fn();
    }
    covered!("tel29_1 screen opened");
    let node = exe::crt_operator_new(n::SIZE as u32);
    core::ptr::write_bytes(node as *mut u8, 0, n::SIZE);
    write_usize(node + n::BUILD, build);
    write_usize(node + n::LEAVE, leave);
    write_usize(node + n::EXIT, exit);
    write_usize(node + n::INPUT, input);
    write_u32(node + n::FLAG, flag);
    copy_c_string(node + n::PICTURE, STR_BACKSLASH);
    let rec = screen_record(mgr);
    if flag != 0 {
        write_i16(rec + h::FLAGGED_COUNT, read_i16(rec + h::FLAGGED_COUNT).wrapping_add(1));
    }
    let last = read_usize(rec + h::LAST_NODE);
    if last != 0 {
        write_usize(last + n::NEXT, node);
        write_usize(node + n::PREV, read_usize(rec + h::LAST_NODE));
    }
    write_usize(rec + h::LAST_NODE, node);
    if read_usize(rec + h::FIRST_NODE) == 0 {
        write_usize(rec + h::FIRST_NODE, node);
    }
    write_usize(rec + h::CURRENT_NODE, node);
    write_i16(rec + h::NODE_COUNT, read_i16(rec + h::NODE_COUNT).wrapping_add(1));
    write_u32(rec + h::AWAY, 0);
    write_i16(rec + h::SCREEN_KEY, -1);
    write_u32(rec + h::SCREEN_KEY_DATA, 0);
    if flag != 0 {
        write_u32(rec + h::DONE, 0);
    }
    write_u32(rec + h::ACTIVE_FLAG, 1);
    if from_default {
        let mut k: i16 = 0;
        while k < read_i16(mgr + h::DEFAULT_SLOT_COUNT) {
            let v = read_u32(read_usize(mgr + h::DEFAULT_SLOT_VALUES).wrapping_add(k as isize as usize * 4));
            exe::screenmanager_set_screen_param(mgr, k as i32 as u32, v as usize, 0);
            k = k.wrapping_add(1);
        }
    }
    1
}

/// Comparer for `screen_manager_open_screen`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_open_screen(mgr: usize, build: usize, input: usize, flag: u32, leave: usize, exit: usize) -> u32 {
    use screen_history as h;
    use screen_node_fields as n;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, usize) -> u32>(IDX_SCREEN_MANAGER_OPEN_SCREEN);
    let rec = screen_record(mgr);
    let mut plan = [build, input, flag as usize, leave, exit];
    if read_u32(rec + h::KEY) == 0 && read_usize(rec + h::FIRST_NODE) == 0 && read_usize(mgr + h::DEFAULT_BUILD) != 0 {
        for (k, p) in plan.iter_mut().enumerate() {
            *p = read_usize(mgr + h::DEFAULT_BUILD + 4 * k);
        }
    }
    if plan[0] == 0 || plan[1] == 0 {
        return o(mgr, build, input, flag, leave, exit);
    }
    let current = read_usize(rec + h::CURRENT_NODE);
    let mut already = false;
    if current != 0 && plan[2] != 0 {
        let mut node = read_usize(rec + h::FIRST_NODE);
        while node != 0 {
            already |= read_u32(node + n::FLAG) != 0 && read_usize(node + n::BUILD) == plan[0];
            node = read_usize(node + n::NEXT);
        }
    }
    let crowded = read_i16(rec + h::NODE_COUNT) >= 100;
    let r = o(mgr, build, input, flag, leave, exit);
    let rec = screen_record(mgr);
    let node = read_usize(rec + h::CURRENT_NODE);
    let ok = if already {
        r == 0
    } else if r == 1 {
        node != 0 && node == read_usize(rec + h::LAST_NODE)
            && [n::BUILD, n::INPUT, n::FLAG, n::LEAVE, n::EXIT].iter().zip(plan.iter()).all(|(&f, &p)| read_usize(node + f) == p)
            && read_u16(node + n::PICTURE) == 0x5c && read_u32(rec + h::ACTIVE_FLAG) == 1 && read_i16(rec + h::SCREEN_KEY) == -1
    } else {
        crowded
    };
    SCOREBOARD.record(ok, || format!("screen_manager_open_screen(build {:#x}): result {} (single screen already open: {}), node {:#x}", plan[0], r, already, node));
    r
}

/// Stores the default screen: its build, input, flag, leave and exit functions and its slot values (pointer and count).
///
/// They go to `screen_history::DEFAULT_BUILD` and the next four dwords, `DEFAULT_SLOT_VALUES` and `DEFAULT_SLOT_COUNT`
/// (word); open_screen uses them for a record with no key and no screen.
/// Original: `ScreenManager::set_default_screen` (0x7feb20, scrman.cpp, thiscall, ret 0x1c).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_default_screen(mgr: usize, build: usize, input: usize, flag: u32, leave: usize, exit: usize, values: usize, count: u32) {
    use screen_history as h;
    covered!("tel29_1 default screen");
    write_usize(mgr + h::DEFAULT_BUILD, build);
    write_usize(mgr + h::DEFAULT_BUILD + 4, input);
    write_u32(mgr + h::DEFAULT_BUILD + 8, flag);
    write_usize(mgr + h::DEFAULT_BUILD + 0xc, leave);
    write_usize(mgr + h::DEFAULT_BUILD + 0x10, exit);
    write_usize(mgr + h::DEFAULT_SLOT_VALUES, values);
    write_u16(mgr + h::DEFAULT_SLOT_COUNT, count as u16);
}

/// Comparer for `screen_manager_set_default_screen`: side by side over the default screen.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_default_screen(mgr: usize, build: usize, input: usize, flag: u32, leave: usize, exit: usize, values: usize, count: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, usize, usize, u32)>(IDX_SCREEN_MANAGER_SET_DEFAULT_SCREEN);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(mgr + screen_history::DEFAULT_BUILD, 0x1a)],
        || o(mgr, build, input, flag, leave, exit, values, count), || screen_manager_set_default_screen(mgr, build, input, flag, leave, exit, values, count));
    SCOREBOARD.record(equal, || format!("screen_manager_set_default_screen: original {:x?} rust {:x?}", mo, mr));
}

/// Counts the records of the current channel whose away flag (`screen_history::AWAY`) and done flag (`DONE`) are both
/// zero (the channel's record count, `CHANNEL_RECORD_COUNTS`, a signed word).
/// Original: `ScreenManager::count_active_records` (0x7feb70, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_count_active_records(mgr: usize) -> u32 {
    use screen_history as h;
    let slot = read_i16(mgr + screen_manager::CURRENT_SLOT) as isize;
    let records = read_i16((mgr as isize + h::CHANNEL_RECORD_COUNTS as isize + slot * 2) as usize);
    let channel = screen_record(mgr);
    let mut count = 0u32;
    for k in 0..records.max(0) as usize {
        let rec = channel + k * h::RECORD_SIZE;
        if read_u32(rec + h::AWAY) == 0 && read_u32(rec + h::DONE) == 0 { // tel29_1: away and done both clear
            count += 1;
        }
    }
    count
}

/// Whether the Back command has nowhere to go: 1 when the current screen has no previous node, or when it is flagged
/// (single instance) and `ignore_root` is 0; otherwise 0.
///
/// Without a current screen: the original routine runs (fatal error box `scrman.cpp 1505` and exit).
/// Original: `ScreenManager::is_back_unavailable` (0x7fec60, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_is_back_unavailable(mgr: usize, ignore_root: u32) -> u32 {
    use screen_node_fields as n;
    let node = current_screen_node(mgr);
    if node == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_SCREEN_MANAGER_IS_BACK_UNAVAILABLE)(mgr, ignore_root);
    }
    if read_usize(node + n::PREV) == 0 {
        return 1;
    }
    if ignore_root != 0 {
        return 0;
    }
    (read_u32(node + n::FLAG) != 0) as u32 // tel29_1: a flagged screen is a root for Back
}

/// The current screen's single-instance flag (`screen_node_fields::FLAG`).
///
/// Without a current screen: the original routine runs (fatal error box `scrman.cpp 1525` and exit).
/// Original: `ScreenManager::get_current_screen_flag` (0x7fed30, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_get_current_screen_flag(mgr: usize) -> u32 {
    let node = current_screen_node(mgr);
    if node == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCREEN_MANAGER_GET_CURRENT_SCREEN_FLAG)(mgr);
    }
    read_u32(node + screen_node_fields::FLAG)
}

/// Whether the current screen has an exit function (`screen_node_fields::EXIT` ≠ 0).
///
/// Without a current screen: the original routine runs (fatal error box `scrman.cpp 1544` and exit).
/// Original: `ScreenManager::current_screen_has_exit_fn` (0x7fedd0, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_current_screen_has_exit_fn(mgr: usize) -> u32 {
    let node = current_screen_node(mgr);
    if node == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCREEN_MANAGER_CURRENT_SCREEN_HAS_EXIT_FN)(mgr);
    }
    (read_usize(node + screen_node_fields::EXIT) != 0) as u32
}

/// The current screen's background picture path (`screen_node_fields::PICTURE`), or the empty string (`STR_EMPTY`)
/// without a current screen.
/// Original: `ScreenManager::get_background_picture` (0x7fee80, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_get_background_picture(mgr: usize) -> usize {
    let node = current_screen_node(mgr);
    if node == 0 { STR_EMPTY } else { node + screen_node_fields::PICTURE }
}

/// Sets the current screen's background picture path: a copy of `title` (with its terminator), or the empty path when
/// `title` is null; nothing without a current screen.
/// Original: `ScreenManager::set_background_picture` (0x7feeb0, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_background_picture(mgr: usize, title: usize) {
    let node = current_screen_node(mgr);
    if node == 0 {
        return;
    }
    covered!("tel29_1 background picture");
    if title != 0 {
        copy_c_string(node + screen_node_fields::PICTURE, title);
    } else {
        write_u8(node + screen_node_fields::PICTURE, 0);
    }
}

/// Comparer for `screen_manager_set_background_picture`: side by side over the node's picture path.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_background_picture(mgr: usize, title: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SCREEN_MANAGER_SET_BACKGROUND_PICTURE);
    let node = current_screen_node(mgr);
    if node == 0 || (title != 0 && c_string_len(title) >= screen_node_fields::PICTURE_SIZE) {
        return o(mgr, title);
    }
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(node + screen_node_fields::PICTURE, screen_node_fields::PICTURE_SIZE)],
        || o(mgr, title), || screen_manager_set_background_picture(mgr, title));
    SCOREBOARD.record(equal, || format!("screen_manager_set_background_picture: differences em {:?}", first_differences(&mo, &mr)));
}

/// Frees the whole screen history of the current record: each node from the first (the current node walks along,
/// `free_screen_node`, the count goes down), then the first, last and current nodes and both counts are cleared.
/// Original: `ScreenManager::free_all_screens` (0x7fef10, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_free_all_screens(mgr: usize) {
    use screen_history as h;
    use screen_node_fields as n;
    covered!("tel29_1 screens freed");
    let mut node = read_usize(screen_record(mgr) + h::FIRST_NODE);
    while node != 0 {
        write_usize(screen_record(mgr) + h::CURRENT_NODE, read_usize(node + n::NEXT));
        free_screen_node(node);
        let rec = screen_record(mgr);
        write_i16(rec + h::NODE_COUNT, read_i16(rec + h::NODE_COUNT).wrapping_sub(1));
        node = read_usize(rec + h::CURRENT_NODE);
    }
    let rec = screen_record(mgr);
    write_usize(rec + h::FIRST_NODE, 0); // tel29_1: the history emptied
    write_usize(rec + h::LAST_NODE, 0);
    write_usize(rec + h::CURRENT_NODE, 0);
    write_u16(rec + h::NODE_COUNT, 0);
    write_u16(rec + h::FLAGGED_COUNT, 0);
}

/// Comparer for `screen_manager_free_all_screens`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_free_all_screens(mgr: usize) {
    use screen_history as h;
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SCREEN_MANAGER_FREE_ALL_SCREENS)(mgr);
    let rec = screen_record(mgr);
    let ok = read_usize(rec + h::FIRST_NODE) == 0 && read_usize(rec + h::LAST_NODE) == 0 && read_usize(rec + h::CURRENT_NODE) == 0
        && read_u16(rec + h::NODE_COUNT) == 0 && read_u16(rec + h::FLAGGED_COUNT) == 0;
    SCOREBOARD.record(ok, || format!("screen_manager_free_all_screens: first {:#x} last {:#x} current {:#x} count {}", read_usize(rec + h::FIRST_NODE), read_usize(rec + h::LAST_NODE), read_usize(rec + h::CURRENT_NODE), read_i16(rec + h::NODE_COUNT)));
}

/// The value in parameter slot `index` (0..59) of the current screen.
///
/// Without a current screen (fatal box `scrman.cpp 1673`) or with an index out of range (box `scrman.cpp 1679`, 0): the
/// original routine runs.
/// Original: `ScreenManager::get_screen_param` (0x7ff010, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_get_screen_param(mgr: usize, index: i32) -> usize {
    let node = current_screen_node(mgr);
    if node == 0 || !(0..screen_node_fields::PARAM_COUNT as i32).contains(&index) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(IDX_SCREEN_MANAGER_GET_SCREEN_PARAM)(mgr, index);
    }
    read_usize(node + screen_node_fields::PARAMS + 8 * index as usize)
}

/// Comparer for `screen_manager_get_screen_param` (the error paths only through the original: the box would show twice).
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_get_screen_param(mgr: usize, index: i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(IDX_SCREEN_MANAGER_GET_SCREEN_PARAM);
    if current_screen_node(mgr) == 0 || !(0..screen_node_fields::PARAM_COUNT as i32).contains(&index) {
        return o(mgr, index);
    }
    let r = o(mgr, index);
    let v = screen_manager_get_screen_param(mgr, index);
    SCOREBOARD.record(r == v, || format!("screen_manager_get_screen_param({}) original={:#x} rust={:#x}", index, r, v));
    r
}

/// Sets one parameter slot on the current screen of every record of every channel whose manager key is `key` (all of
/// them when `key` is 0).
///
/// The current slot is saved and walks the 16 channels; in a channel with records, record 0 gets the parameter
/// (`exe::screenmanager_set_screen_param`) when its key matches, then each other record k in turn is swapped into place
/// 0 (`exe::utils_swap_bytes`, 0x30 bytes) and gets it when its key matches; with more than one record, records 0 and 1
/// are swapped at the end. The slot is restored. Defect of the original: the swaps rotate the records, and the final
/// swap only undoes the rotation for two records: with three or more, the records after the first come back in another
/// order.
/// Original: `ScreenManager::set_screen_param_for_each_record` (0x7ff130, scrman.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_screen_param_for_each_record(mgr: usize, key: u32, index: u32, value: usize, copy: u32) {
    use screen_history as h;
    covered!("tel29_1 parameter for each record");
    let saved = read_u16(mgr + screen_manager::CURRENT_SLOT);
    write_u16(mgr + screen_manager::CURRENT_SLOT, 0);
    let count = |mgr: usize| read_i16((mgr as isize + h::CHANNEL_RECORD_COUNTS as isize + read_i16(mgr + screen_manager::CURRENT_SLOT) as isize * 2) as usize);
    loop {
        if count(mgr) != 0 {
            let first = screen_record(mgr) + h::NETWORK_PLAYER;
            if key == 0 || key == read_u32(screen_record(mgr) + h::KEY) {
                exe::screenmanager_set_screen_param(mgr, index, value, copy);
            }
            let mut k: i16 = 1;
            while count(mgr) > k {
                let other = first + k as usize * h::RECORD_SIZE;
                exe::utils_swap_bytes(first as *mut i32, other as *mut i32, h::RECORD_SIZE as u32);
                if key == 0 || key == read_u32(screen_record(mgr) + h::KEY) {
                    exe::screenmanager_set_screen_param(mgr, index, value, copy);
                }
                k += 1;
            }
            if count(mgr) > 1 {
                let first = screen_record(mgr) + h::NETWORK_PLAYER;
                exe::utils_swap_bytes(first as *mut i32, (first + h::RECORD_SIZE) as *mut i32, h::RECORD_SIZE as u32);
            }
        }
        let slot = read_i16(mgr + screen_manager::CURRENT_SLOT).wrapping_add(1);
        write_i16(mgr + screen_manager::CURRENT_SLOT, slot);
        if slot >= 16 {
            break;
        }
    }
    write_u16(mgr + screen_manager::CURRENT_SLOT, saved);
}

/// Comparer for `screen_manager_set_screen_param_for_each_record`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_screen_param_for_each_record(mgr: usize, key: u32, index: u32, value: usize, copy: u32) {
    let slot = read_u16(mgr + screen_manager::CURRENT_SLOT);
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, u32)>(IDX_SCREEN_MANAGER_SET_SCREEN_PARAM_FOR_EACH_RECORD)(mgr, key, index, value, copy);
    let ok = read_u16(mgr + screen_manager::CURRENT_SLOT) == slot;
    SCOREBOARD.record(ok, || format!("screen_manager_set_screen_param_for_each_record: slot {} came back as {}", slot, read_u16(mgr + screen_manager::CURRENT_SLOT)));
}

/// Stores `value` in parameter slot `index` (0..59) of the current screen: an owned old copy is freed first; with
/// `copy` and a value, a copy of the string (operator new, strlen + 1) is stored and marked owned, otherwise the value
/// itself, not owned.
///
/// Without a current screen (fatal box `scrman.cpp 1754`) or with an index out of range (box `scrman.cpp 1760`): the
/// original routine runs. Out of memory for the copy: the box `scrman.cpp 1783` and nothing more.
/// Original: `ScreenManager::set_screen_param` (0x7ff270, scrman.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_screen_param(mgr: usize, index: i32, value: usize, copy: u32) {
    use screen_node_fields as n;
    let node = current_screen_node(mgr);
    if node == 0 || !(0..n::PARAM_COUNT as i32).contains(&index) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32, usize, u32)>(IDX_SCREEN_MANAGER_SET_SCREEN_PARAM)(mgr, index, value, copy);
    }
    let slot = n::PARAMS + 8 * index as usize;
    if read_u32(node + slot + 4) != 0 {
        exe::crt_operator_delete(read_usize(node + slot));
        write_u32(current_screen_node(mgr) + slot + 4, 0);
    }
    if value != 0 && copy != 0 {
        covered!("tel29_1 parameter copied");
        let size = c_string_len(value) + 1;
        let p = exe::crt_operator_new(size as u32);
        write_usize(current_screen_node(mgr) + slot, p);
        if read_usize(current_screen_node(mgr) + slot) == 0 {
            show_error_at(SCRMAN_CPP, 0x6f7);
            return;
        }
        core::ptr::copy_nonoverlapping(value as *const u8, p as *mut u8, size);
        write_u32(current_screen_node(mgr) + slot + 4, 1); // tel29_1: the copy is owned
    } else {
        write_usize(current_screen_node(mgr) + slot, value);
        write_u32(current_screen_node(mgr) + slot + 4, 0);
    }
}

/// Comparer for `screen_manager_set_screen_param`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_screen_param(mgr: usize, index: i32, value: usize, copy: u32) {
    use screen_node_fields as n;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, usize, u32)>(IDX_SCREEN_MANAGER_SET_SCREEN_PARAM);
    let node = current_screen_node(mgr);
    if node == 0 || !(0..n::PARAM_COUNT as i32).contains(&index) {
        return o(mgr, index, value, copy);
    }
    let slot = n::PARAMS + 8 * index as usize;
    let owned_before = read_u32(node + slot + 4) != 0 && read_usize(node + slot) == value;
    let mut text = Vec::new();
    if value != 0 && copy != 0 && !owned_before {
        text.extend_from_slice(core::slice::from_raw_parts(value as *const u8, c_string_len(value) + 1));
    }
    o(mgr, index, value, copy);
    let node = current_screen_node(mgr);
    let v = read_usize(node + slot);
    let owned = read_u32(node + slot + 4);
    let ok = if owned_before {
        true // simplification: the value was the slot's own copy, freed before it is read again (the original reads freed memory)
    } else if value != 0 && copy != 0 {
        v == 0 || (owned == 1 && core::slice::from_raw_parts(v as *const u8, text.len()) == &text[..])
    } else {
        v == value && owned == 0
    };
    SCOREBOARD.record(ok, || format!("screen_manager_set_screen_param({}, {:#x}, copy {}): slot {:#x} owned {}", index, value, copy, v, owned));
}

/// Asks the screen to be redrawn: network command 9 into the command buffer, then the packet applied to the local
/// Display or sent to the network player, for the current channel or (`all_channels`) every channel with records.
///
/// Nothing when the current Display blocks it (`DISPLAY_REDRAW_BLOCKED`). The buffer position goes to 0 and the byte 9
/// is written (no room: the box `network.h 133`, nothing written). For each channel (every channel with records, or
/// the current one): a local record (`screen_history::NETWORK_PLAYER` = 0) gets the packet applied to the local Display
/// (Display::process_command_packet 0x564080, thiscall, with the session and slot 0) unless `remote_only`; a remote one
/// gets it sent (Network::send_packet 0x77ac30, thiscall; a failure gives the box `scrman.cpp 3136`, `3156` for the
/// current channel).
/// Original: `ScreenManager::redraw_screen` (0x802d00, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_redraw_screen(mgr: usize, all_channels: u32, remote_only: u32) {
    use screen_history as h;
    let display = read_usize(screen_record(mgr) + screen_manager::DISPLAY);
    if read_u32(display + DISPLAY_REDRAW_BLOCKED) != 0 {
        return;
    }
    covered!("tel29_1 screen redraw");
    let buffer = mgr + h::NET_BUFFER;
    write_u32(mgr + h::NET_BUFFER_POS, 0);
    let pos = read_u32(mgr + h::NET_BUFFER_POS);
    if pos.wrapping_add(1) > read_u32(mgr + h::NET_BUFFER_CAPACITY) {
        show_error_at(NETWORK_H, 0x85);
    } else {
        write_u8(read_usize(buffer) + pos as usize, 9); // tel29_1: the redraw command
        write_u32(mgr + h::NET_BUFFER_POS, pos + 1);
    }
    // simplification: the original passes the player in ax with leftover high bits; here zero-extended
    let deliver = |player: u16, line: u32| {
        if player == 0 {
            if remote_only == 0 {
                exe::display_process_command_packet(mgr + h::LOCAL_DISPLAY, read_usize(mgr + h::NETWORK), 0, buffer);
            }
        } else if exe::network_send_packet(read_usize(mgr + h::NETWORK), read_usize(buffer), read_u32(mgr + h::NET_BUFFER_POS), player as u32) == 0 {
            show_error_at(SCRMAN_CPP, line);
        }
    };
    if all_channels != 0 {
        for c in 0..16 {
            if read_i16(mgr + h::CHANNEL_RECORD_COUNTS + 2 * c) > 0 {
                deliver(read_u16(mgr + h::NETWORK_PLAYER + c * h::CHANNEL_SIZE), 0xc40);
            }
        }
    } else {
        deliver(read_u16(screen_record(mgr) + h::NETWORK_PLAYER), 0xc54);
    }
}

/// Comparer for `screen_manager_redraw_screen`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_redraw_screen(mgr: usize, all_channels: u32, remote_only: u32) {
    use screen_history as h;
    let blocked = read_u32(read_usize(screen_record(mgr) + screen_manager::DISPLAY) + DISPLAY_REDRAW_BLOCKED) != 0;
    let room = read_u32(mgr + h::NET_BUFFER_CAPACITY) >= 1;
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SCREEN_MANAGER_REDRAW_SCREEN)(mgr, all_channels, remote_only);
    if blocked || !room {
        return;
    }
    let ok = read_u8(read_usize(mgr + h::NET_BUFFER)) == 9;
    SCOREBOARD.record(ok, || format!("screen_manager_redraw_screen: command byte {}", read_u8(read_usize(mgr + h::NET_BUFFER))));
}

/// Rule: a screen's title is a text item across the top of the screen, in font 0xc, style 7, centred.
///
/// `Display::add_item` of the current slot's `Display` with type 1, the rectangle (100, 10)–(790, 70), two zeros, the
/// text flags 0x30, `colour` (the background colour, field 8), 0, the font 0xc, the style 7, `text_colour`, `text`,
/// three zeros and no area (−1). The item index comes back.
/// Original: `ScreenManager::add_screen_title` (0x803f30, scrman.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_screen_title(sm: usize, colour: u32, text_colour: u32, text: u32) -> u16 {
    covered!("tel38_1 screen title");
    scrman_add_item(sm, [1, 0x64, 0xa, 0x316, 0x46, 0, 0, 0x30, colour, 0, 0xc, 7, text_colour, text, 0, 0, 0, u32::MAX]) // tel38_1: the title rectangle
}

/// Comparer for `screen_manager_add_screen_title`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_screen_title(sm: usize, colour: u32, text_colour: u32, text: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u16>(IDX_SCREEN_MANAGER_ADD_SCREEN_TITLE);
    compare_scrman_item("screen_manager_add_screen_title", sm, u32::MAX, || o(sm, colour, text_colour, text),
        || screen_manager_add_screen_title(sm, colour, text_colour, text))
}

/// Rule: a screen's subtitle is a text item under the title, in font 0xc, style 6, in the yellow text colour.
///
/// `Display::add_item` of the current slot's `Display` with type 1, the rectangle (100, 80)–(790, 125), two zeros, the
/// text flags 2, two zeros, the font 0xc, the style 6, the yellow colour (the word `G_GUI_COLOUR_YELLOW`), `text`, three
/// zeros and no area (−1). The second argument is not read. The item index comes back.
/// Original: `ScreenManager::add_screen_subtitle` (0x803f80, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_screen_subtitle(sm: usize, text: u32, _unused: u32) -> u16 {
    covered!("tel38_1 screen subtitle");
    // simplification: the original pushes the colour in dx, whose high word is the caller's edx; Guio::init keeps a word
    let yellow = read_u16(G_GUI_COLOUR_YELLOW) as u32;
    scrman_add_item(sm, [1, 0x64, 0x50, 0x316, 0x7d, 0, 0, 2, 0, 0, 0xc, 6, yellow, text, 0, 0, 0, u32::MAX]) // tel38_1: the subtitle style
}

/// Comparer for `screen_manager_add_screen_subtitle`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_screen_subtitle(sm: usize, text: u32, unused: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u16>(IDX_SCREEN_MANAGER_ADD_SCREEN_SUBTITLE);
    compare_scrman_item("screen_manager_add_screen_subtitle", sm, u32::MAX, || o(sm, text, unused), || screen_manager_add_screen_subtitle(sm, text, unused))
}

/// Rule: a text item is a type-1 item outside any area, with no id and no data.
///
/// `ScreenManager::add_text_item(x1, y1, x2, y2, flags, colour, font, style, text_colour, text, _)`: `Display::add_item`
/// of the current slot's `Display` with type 1, the rectangle, two zeros, the flags, the colour, 0, the font, the
/// style, the text colour, the text, three zeros and no area (−1). The eleventh argument is not read. The item index
/// comes back.
/// Original: `ScreenManager::add_text_item` (0x803fd0, scrman.cpp, thiscall, ret 0x2c).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_text_item(sm: usize, x1: u32, y1: u32, x2: u32, y2: u32, flags: u32, colour: u32, font: u32,
    style: u32, text_colour: u32, text: u32, _unused: u32) -> u16 {
    scrman_add_item(sm, [1, x1, y1, x2, y2, 0, 0, flags, colour, 0, font, style, text_colour, text, 0, 0, 0, u32::MAX]) // tel38_1: a text item
}

/// Comparer for `screen_manager_add_text_item`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_text_item(sm: usize, x1: u32, y1: u32, x2: u32, y2: u32, flags: u32, colour: u32,
    font: u32, style: u32, text_colour: u32, text: u32, unused: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u16>(IDX_SCREEN_MANAGER_ADD_TEXT_ITEM);
    compare_scrman_item("screen_manager_add_text_item", sm, u32::MAX,
        || o(sm, x1, y1, x2, y2, flags, colour, font, style, text_colour, text, unused),
        || screen_manager_add_text_item(sm, x1, y1, x2, y2, flags, colour, font, style, text_colour, text, unused))
}

/// Rule: a button item is a type-2 item outside any area whose highlight colours are its own colour, carrying its id
/// and data.
///
/// `ScreenManager::add_button_item(x1, y1, x2, y2, flags, colour, font, style, text_colour, text, id, data)`:
/// `Display::add_item` of the current slot's `Display` with type 2, the rectangle, two zeros, the flags, the colour
/// twice (fields 8 and 9), the font, the style, the text colour, the text, 0, the id, the data and no area (−1). The
/// item index comes back.
/// Original: `ScreenManager::add_button_item` (0x804030, scrman.cpp, thiscall, ret 0x30).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_button_item(sm: usize, x1: u32, y1: u32, x2: u32, y2: u32, flags: u32, colour: u32, font: u32,
    style: u32, text_colour: u32, text: u32, id: u32, data: u32) -> u16 {
    scrman_add_item(sm, [2, x1, y1, x2, y2, 0, 0, flags, colour, colour, font, style, text_colour, text, 0, id, data, u32::MAX]) // tel38_1: a button item
}

/// Comparer for `screen_manager_add_button_item`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_button_item(sm: usize, x1: u32, y1: u32, x2: u32, y2: u32, flags: u32, colour: u32,
    font: u32, style: u32, text_colour: u32, text: u32, id: u32, data: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u16>(IDX_SCREEN_MANAGER_ADD_BUTTON_ITEM);
    compare_scrman_item("screen_manager_add_button_item", sm, u32::MAX,
        || o(sm, x1, y1, x2, y2, flags, colour, font, style, text_colour, text, id, data),
        || screen_manager_add_button_item(sm, x1, y1, x2, y2, flags, colour, font, style, text_colour, text, id, data))
}

/// Rule: an area button is a type-2 item placed by its area (no rectangle), with its own colour as the highlight.
///
/// `ScreenManager::add_area_button(area, w, h, flags, colour, font, style, text_colour, text, id, data)`:
/// `Display::add_item` of the current slot's `Display` with type 2, four zeros (the area lays it out), the width and
/// height (fields 5 and 6), the flags, the colour twice, the font, the style, the text colour, the text, 0, the id, the
/// data and the area. The item index comes back.
/// Original: `ScreenManager::add_area_button` (0x8040a0, scrman.cpp, thiscall, ret 0x2c).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_area_button(sm: usize, area: u32, w: u32, h: u32, flags: u32, colour: u32, font: u32, style: u32,
    text_colour: u32, text: u32, id: u32, data: u32) -> u16 {
    scrman_add_item(sm, [2, 0, 0, 0, 0, w, h, flags, colour, colour, font, style, text_colour, text, 0, id, data, area]) // tel38_1: an area button
}

/// Comparer for `screen_manager_add_area_button`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_area_button(sm: usize, area: u32, w: u32, h: u32, flags: u32, colour: u32, font: u32,
    style: u32, text_colour: u32, text: u32, id: u32, data: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u16>(IDX_SCREEN_MANAGER_ADD_AREA_BUTTON);
    compare_scrman_item("screen_manager_add_area_button", sm, area,
        || o(sm, area, w, h, flags, colour, font, style, text_colour, text, id, data),
        || screen_manager_add_area_button(sm, area, w, h, flags, colour, font, style, text_colour, text, id, data))
}

/// Rule: an area text is a type-1 item placed by its area, with no id and no data.
///
/// `ScreenManager::add_area_text(area, w, h, flags, colour, font, style, text_colour, text)`: `Display::add_item` of
/// the current slot's `Display` with type 1, four zeros, the width and height, the flags, the colour, 0, the font, the
/// style, the text colour, the text, three zeros and the area. The item index comes back.
/// Original: `ScreenManager::add_area_text` (0x804100, scrman.cpp, thiscall, ret 0x24).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_area_text(sm: usize, area: u32, w: u32, h: u32, flags: u32, colour: u32, font: u32, style: u32,
    text_colour: u32, text: u32) -> u16 {
    scrman_add_item(sm, [1, 0, 0, 0, 0, w, h, flags, colour, 0, font, style, text_colour, text, 0, 0, 0, area]) // tel38_1: an area text
}

/// Comparer for `screen_manager_add_area_text`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_area_text(sm: usize, area: u32, w: u32, h: u32, flags: u32, colour: u32, font: u32,
    style: u32, text_colour: u32, text: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u16>(IDX_SCREEN_MANAGER_ADD_AREA_TEXT);
    compare_scrman_item("screen_manager_add_area_text", sm, area, || o(sm, area, w, h, flags, colour, font, style, text_colour, text),
        || screen_manager_add_area_text(sm, area, w, h, flags, colour, font, style, text_colour, text))
}

/// Rule: like `screen_manager_add_area_text`, with the height read through a pointer.
///
/// `ScreenManager::add_area_text_by_ref(area, w, h_ptr, flags, colour, font, style, text_colour, text, _, _)`: the same
/// item as `add_area_text`, field 6 being the dword at `h_ptr`. The last two arguments are not read. The item index
/// comes back.
/// Original: `ScreenManager::add_area_text_by_ref` (0x804160, scrman.cpp, thiscall, ret 0x2c).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_area_text_by_ref(sm: usize, area: u32, w: u32, h_ptr: usize, flags: u32, colour: u32, font: u32,
    style: u32, text_colour: u32, text: u32, _unused1: u32, _unused2: u32) -> u16 {
    scrman_add_item(sm, [1, 0, 0, 0, 0, w, read_u32(h_ptr), flags, colour, 0, font, style, text_colour, text, 0, 0, 0, area])
}

/// Comparer for `screen_manager_add_area_text_by_ref`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_area_text_by_ref(sm: usize, area: u32, w: u32, h_ptr: usize, flags: u32, colour: u32,
    font: u32, style: u32, text_colour: u32, text: u32, unused1: u32, unused2: u32) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, u32, u32, u32, u32, u32, u32, u32, u32) -> u16>(IDX_SCREEN_MANAGER_ADD_AREA_TEXT_BY_REF);
    compare_scrman_item("screen_manager_add_area_text_by_ref", sm, area,
        || o(sm, area, w, h_ptr, flags, colour, font, style, text_colour, text, unused1, unused2),
        || screen_manager_add_area_text_by_ref(sm, area, w, h_ptr, flags, colour, font, style, text_colour, text, unused1, unused2))
}

/// Rule: how many single-instance screens are open in all records of all channels.
///
/// The sum of the flagged screen counts (`screen_history::FLAGGED_COUNT`) of the 16 records of each of the 16 channels,
/// as a 16-bit word (the callers read the short).
/// Original: `ScreenManager::count_flagged_screens` (0x803d50, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_count_flagged_screens(sm: usize) -> u16 {
    (0..256).fold(0u16, |s, k| s.wrapping_add(read_u16(sm + screen_history::FLAGGED_COUNT + k * screen_history::RECORD_SIZE)))
}

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_SCREEN_MANAGER_SWAP_MANAGER_KEYS);
    let channel = screen_record(sm) + screen_history::NETWORK_PLAYER;
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&[(channel, screen_history::CHANNEL_SIZE)], || o(sm, a, b),
        || screen_manager_swap_manager_keys(sm, a, b));
    SCOREBOARD.record(equal && ro == rr, || format!("screen_manager_swap_manager_keys({:#x}, {:#x}): return {} x {}, differences in {:?}", a, b, ro, rr, first_differences(&mo, &mr)));
    ro
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
    exe::screenmanager_disconnect_network(sm);
    let net = read_usize(sm + h::NETWORK);
    if net != 0 {
        exe::drop_network(net);
        exe::crt_operator_delete(net);
    }
    write_usize(sm + h::NETWORK, 0);
}

/// Comparer for `screen_manager_end_network_session`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_end_network_session(sm: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SCREEN_MANAGER_END_NETWORK_SESSION)(sm);
    let net = read_usize(sm + screen_history::NETWORK);
    SCOREBOARD.record(net == 0, || format!("screen_manager_end_network_session: session {:#x} left", net));
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
        let reset: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_DISPLAY_RESET);
        reset(read_usize(screen_record(sm) + screen_manager::DISPLAY), 1);
        write_usize(current(sm), 0);
    }
    write_u32(screen_record(sm) + h::ACTIVE_FLAG, 0);
    exe::screenmanager_free_all_screens(sm);
    1
}

/// Comparer for `screen_manager_close_all_screens`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_close_all_screens(sm: usize) -> u32 {
    use screen_history as h;
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCREEN_MANAGER_CLOSE_ALL_SCREENS)(sm);
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

/// Comparer for `screen_manager_open_screen_for_each_record`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_open_screen_for_each_record(sm: usize, key: u32, build: usize, input: usize, flag: u32,
    leave: usize, exit: u32) -> u32 {
    let slot = read_u16(sm + screen_manager::CURRENT_SLOT);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, usize, u32, usize, u32) -> u32>(IDX_SCREEN_MANAGER_OPEN_SCREEN_FOR_EACH_RECORD)(sm, key, build, input, flag, leave, exit);
    let back = read_u16(sm + screen_manager::CURRENT_SLOT);
    SCOREBOARD.record(r == 1 && back == slot, || format!("screen_manager_open_screen_for_each_record: returned {}, slot {} came back as {}", r, slot, back));
    r
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize)>(IDX_SCREEN_MANAGER_ON_REMOTE_SOUND_STARTED);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SCREEN_MANAGER_ON_REMOTE_SOUND_FINISHED);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side_light(&[(scrman_sound_flag(sm, slot), 4)], || o(sm, slot),
        || screen_manager_on_remote_sound_finished(sm, slot));
    SCOREBOARD.record(equal, || format!("screen_manager_on_remote_sound_finished({}): differences in {:?}", slot as i8, first_differences(&mo, &mr)));
}

/// The address of record `k` of channel `c` of the screen manager `sm` (16 channels of 16 records of 0x30 bytes from
/// +0x7d4; `scrman_channel_record` gives the base to add a `screen_history` offset to).
pub(crate) fn channel_record_at(sm: usize, c: usize, k: usize) -> usize {
    scrman_channel_record(sm, c as i16, k as i16) + screen_history::NETWORK_PLAYER
}

/// The address of the record count of channel `c` (a signed word).
pub(crate) fn channel_count_at(sm: usize, c: usize) -> usize {
    sm + screen_history::CHANNEL_RECORD_COUNTS + 2 * c
}

/// Side by side over the player records (`scrman_records_regions`).
pub(crate) unsafe fn compare_scrman_records(name: &str, sm: usize, original: impl FnOnce(), rust: impl FnOnce()) {
    let reg = scrman_records_regions(sm);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&reg, original, rust);
    SCOREBOARD.record(same, || format!("{}: player records differ {:?}", name, first_differences(&mo, &mr)));
}

/// Record 0 of the current channel (`screen_manager::CURRENT_SLOT`, read anew each time, as the original does).
pub(crate) unsafe fn screen_record(mgr: usize) -> usize {
    mgr.wrapping_add((read_i16(mgr + screen_manager::CURRENT_SLOT) as isize * screen_history::CHANNEL_SIZE as isize) as usize)
}

/// The current screen node of the current channel (0 when there is none).
pub(crate) unsafe fn current_screen_node(mgr: usize) -> usize {
    read_usize(screen_record(mgr) + screen_history::CURRENT_NODE)
}

/// Frees a screen node: each parameter slot holding an owned copy (value ≠ 0 and the flag ≠ 0), then the node
/// (operator delete).
pub(crate) unsafe fn free_screen_node(node: usize) {
    use screen_node_fields as n;
    for k in 0..n::PARAM_COUNT {
        let v = read_usize(node + n::PARAMS + 8 * k);
        if v != 0 && read_u32(node + n::PARAMS + 8 * k + 4) != 0 {
            exe::crt_operator_delete(v);
        }
    }
    exe::crt_operator_delete(node);
}

/// Comparer of an item helper: side by side over `scrman_item_regions` and the item index (the low word: `Display::add_item`
/// returns a short).
pub(crate) unsafe fn compare_scrman_item(name: &str, sm: usize, area: u32, original: impl FnOnce() -> u16, rust: impl FnOnce() -> u16) -> u16 {
    let regions = scrman_item_regions(sm, area);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, original, rust);
    SCOREBOARD.record(equal && ro == rr, || format!("{}: index {:#x} x {:#x}, differences in {:?}", name, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// `Display::add_item` of the screen manager's current slot with the 18 fields; the item index (a short) comes back.
pub(crate) unsafe fn scrman_add_item(sm: usize, f: [u32; 18]) -> u16 {
    let add: DisplayAddItemFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    add(screens_current_display(sm), f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10], f[11], f[12], f[13], f[14], f[15],
        f[16], f[17]) as u16
}

pub(crate) const SCRMAN_NEXT_CHANNEL: usize = 0x382c;

pub(crate) const SCRMAN_RECORD_STAMP: usize = 0x2c;

/// The two player record callbacks by their exe address (through their detours, as the original passes them).
pub(crate) const SCRMAN_ADD_PLAYER_RECORD: usize = 0x7ff630;

pub(crate) const SCRMAN_REMOVE_PLAYER_RECORDS: usize = 0x7ff7b0;

/// "\" (the path a new screen node starts with), "" (the picture with no screen) and the Display flag that blocks the
/// redraw packet [inference].
pub(crate) const STR_BACKSLASH: usize = 0x9c3730;

/// Rule: the machine's own network addresses, for the network game screen.
///
/// Forwards to `Network::get_local_addresses(out)` (0x77a980) on the session (`screen_history::NETWORK`, not checked
/// for null) and returns what it returns (the count).
/// Original: `ScreenManager::get_local_addresses` (0x7fc7c0, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_get_local_addresses(sm: usize, out: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = tcpip_fn(0x77a980);
    f(read_usize(sm + screen_history::NETWORK), out)
}

/// Comparer for `screen_manager_get_local_addresses`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_get_local_addresses(sm: usize, out: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SCREEN_MANAGER_GET_LOCAL_ADDRESSES)(sm, out)
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

/// Comparer for `screen_manager_send_clear_click`: the packet side by side, delivered once by the original.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_send_clear_click(sm: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SCREEN_MANAGER_SEND_CLEAR_CLICK);
    compare_packet_build("send_clear_click", sm + screen_history::NET_BUFFER, 1, || scrman_command_packet(sm, 0x11), || o(sm));
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

/// Comparer for `screen_manager_set_pending_click_widget`: the packet side by side, delivered once by the original.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_pending_click_widget(sm: usize, widget: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SCREEN_MANAGER_SET_PENDING_CLICK_WIDGET);
    compare_packet_build("set_pending_click_widget", sm + screen_history::NET_BUFFER, 3, || {
        scrman_command_packet(sm, 0x13);
        scrman_net_put(sm, widget, 2, 0x86);
    }, || o(sm, widget));
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

/// Comparer for `screen_manager_send_area_scroll_row`: in a local game (channel 0 not on the remote Display) side by
/// side over the network send buffer (its header and the 7 bytes); otherwise the packet side by side and the send once
/// by the original.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_send_area_scroll_row(sm: usize, area: u32, row: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SCREEN_MANAGER_SEND_AREA_SCROLL_ROW);
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

/// Comparer for `screen_manager_send_network_status_message`: the packet side by side, applied or sent once by the
/// original (the local Display draws the progress).
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_send_network_status_message(sm: usize, text: usize, value: u32, total: u32, channel: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(IDX_SCREEN_MANAGER_SEND_NETWORK_STATUS_MESSAGE);
    let need = 1 + 4 + 4 + 2 + c_string_len(text) + 1;
    compare_packet_build("send_network_status_message", sm + screen_history::NET_BUFFER, need, || status_message_packet(sm, text, value, total),
        || o(sm, text, value, total, channel));
}

/// Rule: the row an area of the current screen shows; for a remote manager, the row he last reported for that area.
///
/// The record of the current slot (`screen_record`, tel29_1): when it is local (`screen_history::NETWORK_PLAYER` = 0),
/// area −1 gives 0 and any other area (its low word, signed) of the slot's `Display` (`screen_manager::DISPLAY`) gives
/// `Area::get_shown_row` (0x4032e0, thiscall). When it is remote, the reported row (`SCREEN_KEY_DATA`) when the area is
/// the reported one (`SCREEN_KEY`), otherwise 0.
/// Original: `ScreenManager::get_area_scroll_row` (0x803320, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_get_area_scroll_row(sm: usize, area: u32) -> u32 {
    let rec = screen_record(sm);
    if read_u16(rec + screen_history::NETWORK_PLAYER) == 0 {
        if area as u16 == 0xffff {
            return 0;
        }
        return exe::area_get_shown_row(display_area_at(read_usize(rec + screen_manager::DISPLAY), area as i16));
    }
    covered!("tel38_1 remote scroll row");
    if area as u16 == read_u16(rec + screen_history::SCREEN_KEY) {
        return read_u32(rec + screen_history::SCREEN_KEY_DATA);
    }
    0
}

/// Rule: a scrollbar key of the current screen is pressed as a click on the open scrollbar.
///
/// `Display::press_scrollbar_key(key, held)` (0x5621f0, thiscall, ret 8) of the current slot's `Display`, with both
/// arguments: with an open scrollbar (`G_ACTIVE_AREA_SCROLLBAR`) it fakes a left click on the arrow or the track the key
/// names (0..3).
/// Original: `ScreenManager::press_scrollbar_key` (0x803390, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_press_scrollbar_key(sm: usize, key: u32, held: u32) {
    covered!("tel38_1 scrollbar key");
    exe::display_press_scrollbar_key(screens_current_display(sm), key, held);
}

/// Comparer for `screen_manager_press_scrollbar_key`: side by side over the mouse globals.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_press_scrollbar_key(sm: usize, key: u32, held: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SCREEN_MANAGER_PRESS_SCROLLBAR_KEY);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side_light(&[(SCROLLBAR_KEY_MOUSE, 0x2c)], || o(sm, key, held),
        || screen_manager_press_scrollbar_key(sm, key, held));
    SCOREBOARD.record(equal, || format!("screen_manager_press_scrollbar_key({}, {}): differences in {:?}", key, held, first_differences(&mo, &mr)));
}

/// Rule: how many manager records there are in all channels.
///
/// The sum of the 16 channel record counts (`screen_history::CHANNEL_RECORD_COUNTS`), as a 16-bit word (the original
/// keeps garbage in the high word; the callers read the short).
/// Original: `ScreenManager::count_all_records` (0x8033c0, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_count_all_records(sm: usize) -> u16 {
    (0..16).fold(0u16, |s, c| s.wrapping_add(read_u16(sm + screen_history::CHANNEL_RECORD_COUNTS + 2 * c))) // tel38_1: all 16 channels
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_SCREEN_MANAGER_GET_CHANNEL_MANAGER_KEYS);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SCREEN_MANAGER_SET_MANAGER_DONE);
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

/// Rule: how many channels are idle: with no records, or whose first record is away (`screen_history::AWAY` ≠ 0).
/// Original: `ScreenManager::count_idle_channels` (0x7fff70, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_count_idle_channels(sm: usize) -> u32 {
    (0..16i16).filter(|&c| scrman_channel_count(sm, c) <= 0 || read_u32(scrman_channel_record(sm, c, 0) + screen_history::AWAY) != 0).count() as u32
}

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SCREEN_MANAGER_SWITCH_TO_MANAGER);
    let channel = screen_record(sm) + screen_history::NETWORK_PLAYER;
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(channel, screen_history::CHANNEL_SIZE)], || o(sm, key),
        || screen_manager_switch_to_manager(sm, key));
    SCOREBOARD.record(equal, || format!("screen_manager_switch_to_manager({:#x}): differences in {:?}", key, first_differences(&mo, &mr)));
}

/// Rule: a widget of the current screen gets its flags (sub-command 0, a dword).
///
/// `scrman_set_widget` with widget −1 doing nothing, the dword written with the box line `network.h 141`, the send
/// failure `scrman.cpp 2606`.
/// Original: `ScreenManager::set_widget_flags` (0x800580, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_flags(sm: usize, widget: u32, flags: u32) {
    scrman_set_widget(sm, widget, 0, flags, 4, 0x8d, 0xa2e, false); // tel38_4: the flags sub-command
}

/// Comparer for `screen_manager_set_widget_flags`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_flags(sm: usize, widget: u32, flags: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SCREEN_MANAGER_SET_WIDGET_FLAGS);
    compare_scrman_set_widget("screen_manager_set_widget_flags", sm, widget, widget as u16 == 0xffff, scrman_expected_packet(widget, 0, &flags.to_le_bytes()),
        || o(sm, widget, flags));
}

/// Rule: a widget of the current screen gets its style (sub-command 2, a dword; box line `network.h 141`, send failure
/// `scrman.cpp 2679`); nothing for widget −1.
/// Original: `ScreenManager::set_widget_style` (0x800d30, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_style(sm: usize, widget: u32, style: u32) {
    scrman_set_widget(sm, widget, 2, style, 4, 0x8d, 0xa77, false);
}

/// Comparer for `screen_manager_set_widget_style`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_style(sm: usize, widget: u32, style: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SCREEN_MANAGER_SET_WIDGET_STYLE);
    compare_scrman_set_widget("screen_manager_set_widget_style", sm, widget, widget as u16 == 0xffff, scrman_expected_packet(widget, 2, &style.to_le_bytes()),
        || o(sm, widget, style));
}

/// Rule: a widget of the current screen gets its colour (sub-command 3, a word; box line `network.h 139`, send failure
/// `scrman.cpp 2714`); nothing for widget −1.
/// Original: `ScreenManager::set_widget_colour` (0x801040, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_colour(sm: usize, widget: u32, colour: u32) {
    scrman_set_widget(sm, widget, 3, colour, 2, 0x8b, 0xa9a, false);
}

/// Comparer for `screen_manager_set_widget_colour`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_colour(sm: usize, widget: u32, colour: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SCREEN_MANAGER_SET_WIDGET_COLOUR);
    compare_scrman_set_widget("screen_manager_set_widget_colour", sm, widget, widget as u16 == 0xffff,
        scrman_expected_packet(widget, 3, &(colour as u16).to_le_bytes()), || o(sm, widget, colour));
}

/// Rule: a widget of the current screen gets its text flags (sub-command 5, a dword; box line `network.h 141`, send
/// failure `scrman.cpp 2784`); nothing for widget −1.
/// Original: `ScreenManager::set_widget_text_flags` (0x801350, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_text_flags(sm: usize, widget: u32, flags: u32) {
    scrman_set_widget(sm, widget, 5, flags, 4, 0x8d, 0xae0, false);
}

/// Comparer for `screen_manager_set_widget_text_flags`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_text_flags(sm: usize, widget: u32, flags: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SCREEN_MANAGER_SET_WIDGET_TEXT_FLAGS);
    compare_scrman_set_widget("screen_manager_set_widget_text_flags", sm, widget, widget as u16 == 0xffff,
        scrman_expected_packet(widget, 5, &flags.to_le_bytes()), || o(sm, widget, flags));
}

/// Rule: a widget of the current screen gets its font (sub-command 6, a byte; box line `network.h 133`, send failure
/// `scrman.cpp 2817`). Unlike the other setters, widget −1 is not skipped: the packet goes out with it.
/// Original: `ScreenManager::set_widget_font` (0x801660, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_font(sm: usize, widget: u32, font: u32) {
    scrman_set_widget(sm, widget, 6, font, 1, 0x85, 0xb01, true); // tel38_4: the font has no −1 check
}

/// Comparer for `screen_manager_set_widget_font`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_font(sm: usize, widget: u32, font: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SCREEN_MANAGER_SET_WIDGET_FONT);
    compare_scrman_set_widget("screen_manager_set_widget_font", sm, widget, false, scrman_expected_packet(widget, 6, &[font as u8]), || o(sm, widget, font));
}

/// Rule: a widget of the current screen gets its text colour (sub-command 7, a word; box line `network.h 139`, send
/// failure `scrman.cpp 2852`); nothing for widget −1.
/// Original: `ScreenManager::set_widget_text_colour` (0x801960, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_text_colour(sm: usize, widget: u32, colour: u32) {
    scrman_set_widget(sm, widget, 7, colour, 2, 0x8b, 0xb24, false);
}

/// Comparer for `screen_manager_set_widget_text_colour`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_text_colour(sm: usize, widget: u32, colour: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SCREEN_MANAGER_SET_WIDGET_TEXT_COLOUR);
    compare_scrman_set_widget("screen_manager_set_widget_text_colour", sm, widget, widget as u16 == 0xffff,
        scrman_expected_packet(widget, 7, &(colour as u16).to_le_bytes()), || o(sm, widget, colour));
}

/// Rule: a widget of the current screen gets a new text (sub-command 8, the string with its terminator, after its
/// length as a word); nothing for widget −1.
///
/// The packet as `scrman_widget_packet`; the length (strlen + 1, a word, box line `network.h 134`); then the string
/// with its terminator is copied after it and the position advances, with no room check (defect of the original: a
/// long text runs past the buffer). Delivery with the send failure `scrman.cpp 2887`.
/// Original: `ScreenManager::set_widget_text` (0x801c70, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_text(sm: usize, widget: u32, text: usize) {
    use screen_history as h;
    if widget as u16 == 0xffff {
        return;
    }
    scrman_widget_packet(sm, widget, 8);
    let len = c_string_len(text) + 1;
    scrman_net_put(sm, len as u32, 2, 0x86);
    let pos = read_u32(sm + h::NET_BUFFER_POS);
    core::ptr::copy_nonoverlapping(text as *const u8, (read_usize(sm + h::NET_BUFFER) + pos as usize) as *mut u8, len); // tel38_4: the text copied unchecked
    write_u32(sm + h::NET_BUFFER_POS, pos.wrapping_add(len as u16 as i16 as u32));
    scrman_deliver_packet(sm, 0xb47);
}

/// Comparer for `screen_manager_set_widget_text`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_text(sm: usize, widget: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize)>(IDX_SCREEN_MANAGER_SET_WIDGET_TEXT);
    let skip = widget as u16 == 0xffff || text == 0;
    let mut value = Vec::new();
    if !skip {
        let len = c_string_len(text) + 1;
        value.extend_from_slice(&(len as u16).to_le_bytes());
        value.extend_from_slice(core::slice::from_raw_parts(text as *const u8, len));
    }
    compare_scrman_set_widget("screen_manager_set_widget_text", sm, widget, skip, scrman_expected_packet(widget, 8, &value), || o(sm, widget, text));
}

/// Rule: a widget of the current screen gets its id (sub-command 9, a dword; box line `network.h 136`, send failure
/// `scrman.cpp 2922`); nothing for widget −1.
/// Original: `ScreenManager::set_widget_id` (0x801fc0, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_id(sm: usize, widget: u32, id: u32) {
    scrman_set_widget(sm, widget, 9, id, 4, 0x88, 0xb6a, false);
}

/// Comparer for `screen_manager_set_widget_id`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_id(sm: usize, widget: u32, id: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SCREEN_MANAGER_SET_WIDGET_ID);
    compare_scrman_set_widget("screen_manager_set_widget_id", sm, widget, widget as u16 == 0xffff, scrman_expected_packet(widget, 9, &id.to_le_bytes()),
        || o(sm, widget, id));
}

/// Rule: a widget of the current screen gets its data (sub-command 0xa, a dword; box line `network.h 137`, send failure
/// `scrman.cpp 2957`); nothing for widget −1.
/// Original: `ScreenManager::set_widget_data` (0x8022d0, scrman.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn screen_manager_set_widget_data(sm: usize, widget: u32, data: u32) {
    scrman_set_widget(sm, widget, 0xa, data, 4, 0x89, 0xb8d, false);
}

/// Comparer for `screen_manager_set_widget_data`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_widget_data(sm: usize, widget: u32, data: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SCREEN_MANAGER_SET_WIDGET_DATA);
    compare_scrman_set_widget("screen_manager_set_widget_data", sm, widget, widget as u16 == 0xffff, scrman_expected_packet(widget, 0xa, &data.to_le_bytes()),
        || o(sm, widget, data));
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32)>(IDX_SCREEN_MANAGER_SET_WIDGET_RECT);
    let mut value = Vec::new();
    for v in [x1, y1, x2, y2] {
        value.extend_from_slice(&(v as u16).to_le_bytes());
    }
    compare_scrman_set_widget("screen_manager_set_widget_rect", sm, widget, widget as u16 == 0xffff, scrman_expected_packet(widget, 1, &value),
        || o(sm, widget, x1, y1, x2, y2));
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
        let slot = exe::soundplayer_play_file(player, pan, file, looping) as u32;
        write_u8(out, slot as u8);
        exe::soundplayer_set_slot_volume(read_usize(SOUND_PLAYER), slot, 100); // tel38_5: full volume
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
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32)>(IDX_SCREEN_MANAGER_PLAY_SOUND)(sm, out, pan, file, looping);
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
    exe::soundplayer_is_slot_playing(player, slot) // tel38_5: the sound player's answer
}

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
                exe::soundplayer_stop_all(player);
            } else {
                exe::soundplayer_stop_slot(player, slot);
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

/// Comparer for `screen_manager_stop_sound`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_stop_sound(sm: usize, slot: u32) {
    let remote = read_u16(screen_record(sm) + screen_history::NETWORK_PLAYER) != 0;
    let room = read_u32(sm + screen_history::NET_BUFFER_CAPACITY) >= 2;
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SCREEN_MANAGER_STOP_SOUND)(sm, slot);
    if remote {
        scrman_check_packet("screen_manager_stop_sound", sm, room, &[0xb, slot as u8]);
    }
    let cleared = if slot as u8 == 0xff { (0..10).all(|k| read_u32(scrman_sound_flag(sm, k)) == 0) } else { read_u32(scrman_sound_flag(sm, slot)) == 0 };
    SCOREBOARD.record(cleared, || format!("screen_manager_stop_sound({}): a flag still set", slot as i8));
}

/// Rule: starts a print job of `columns` columns: the packet command 0x17 and the columns (a word, box line `network.h
/// 134`), delivered (`scrman_deliver_print`, send failure `scrman.cpp 3280`).
/// Original: `ScreenManager::begin_print_job` (0x8033e0, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_begin_print_job(sm: usize, columns: u32) {
    scrman_command_packet(sm, 0x17); // tel38_6: the begin command
    scrman_net_put(sm, columns, 2, 0x86); // tel38_6: the column count
    scrman_deliver_print(sm, 0xcd0);
}

/// Comparer for `screen_manager_begin_print_job`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_begin_print_job(sm: usize, columns: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SCREEN_MANAGER_BEGIN_PRINT_JOB);
    compare_scrman_print("screen_manager_begin_print_job", sm, vec![0x17, columns as u8, (columns >> 8) as u8], || o(sm, columns));
}

/// Rule: ends the print job (the Display frees its lines): the packet command 0x18, delivered (send failure
/// `scrman.cpp 3303`).
/// Original: `ScreenManager::end_print_job` (0x8035b0, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_end_print_job(sm: usize) {
    scrman_command_packet(sm, 0x18);
    scrman_deliver_print(sm, 0xce7);
}

/// Comparer for `screen_manager_end_print_job`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_end_print_job(sm: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SCREEN_MANAGER_END_PRINT_JOB);
    compare_scrman_print("screen_manager_end_print_job", sm, vec![0x18], || o(sm));
}

/// Rule: adds a line to the print job: the packet command 0x19 and the text (an empty one for a null pointer) with its
/// length (`scrman_net_put_text`: copied with no room check, defect of the original), delivered (send failure
/// `scrman.cpp 3330`).
/// Original: `ScreenManager::add_print_line` (0x803710, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_print_line(sm: usize, text: usize) {
    let text = if text == 0 { STR_EMPTY } else { text }; // tel38_6: a null line prints empty
    scrman_command_packet(sm, 0x19);
    scrman_net_put_text(sm, text);
    scrman_deliver_print(sm, 0xd02);
}

/// Comparer for `screen_manager_add_print_line`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_print_line(sm: usize, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SCREEN_MANAGER_ADD_PRINT_LINE);
    let t = if text == 0 { STR_EMPTY } else { text };
    let len = c_string_len(t) + 1;
    let mut expected = vec![0x19];
    expected.extend_from_slice(&(len as u16).to_le_bytes());
    expected.extend_from_slice(core::slice::from_raw_parts(t as *const u8, len));
    compare_scrman_print("screen_manager_add_print_line", sm, expected, || o(sm, text));
}

/// Rule: adds a page break to the print job (the Display adds a null line): the packet command 0x1a, delivered (send
/// failure `scrman.cpp 3353`).
/// Original: `ScreenManager::add_print_page_break` (0x803930, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_print_page_break(sm: usize) {
    scrman_command_packet(sm, 0x1a);
    scrman_deliver_print(sm, 0xd19);
}

/// Comparer for `screen_manager_add_print_page_break`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_print_page_break(sm: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SCREEN_MANAGER_ADD_PRINT_PAGE_BREAK);
    compare_scrman_print("screen_manager_add_print_page_break", sm, vec![0x1a], || o(sm));
}

/// Rule: prints the lines of the print job: the packet command 0x16, delivered (send failure `scrman.cpp 3376`).
/// Original: `ScreenManager::print_job` (0x803a90, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_print_job(sm: usize) {
    scrman_command_packet(sm, 0x16);
    scrman_deliver_print(sm, 0xd30);
}

/// Comparer for `screen_manager_print_job`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_print_job(sm: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SCREEN_MANAGER_PRINT_JOB);
    compare_scrman_print("screen_manager_print_job", sm, vec![0x16], || o(sm));
}

/// Rule: prints the screen itself (the Display calls the print routine with no lines): the packet command 0x15,
/// delivered (send failure `scrman.cpp 3399`).
/// Original: `ScreenManager::print_screen` (0x803bf0, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_print_screen(sm: usize) {
    scrman_command_packet(sm, 0x15); // tel38_6: the print screen command
    scrman_deliver_print(sm, 0xd47);
}

/// Comparer for `screen_manager_print_screen`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_print_screen(sm: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SCREEN_MANAGER_PRINT_SCREEN);
    compare_scrman_print("screen_manager_print_screen", sm, vec![0x15], || o(sm));
}
