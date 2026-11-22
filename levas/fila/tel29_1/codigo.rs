// ---------- batch %%LEVA%%: the screen manager's history: opening a screen, the default screen, the current screen's flags, picture and parameters, freeing the history and the redraw packet (scrman.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\si\code\scrman.cpp" and "...\network.h" (the error boxes).
pub(crate) const SCRMAN_CPP: usize = 0xa7ddbc;
pub(crate) const NETWORK_H: usize = 0x9c3734;

/// The screen manager's per-manager records (`ScreenManager`, at [`G_SCREEN_MANAGER`]): 16 channels of 0x300 bytes
/// from +0x7d4, each with up to 16 records of 0x30 bytes (record 0 is the one being worked on; the offsets here are those
/// of record 0 of channel 0, add 0x300 × the current slot `screen_manager::CURRENT_SLOT`).
pub(crate) mod screen_history {
    /// The record's network player (0 = local) and manager key (dword at +2).
    pub(crate) const NETWORK_PLAYER: usize = 0x7d4;
    pub(crate) const KEY: usize = 0x7d6;
    /// The screen history: first, last and current node (0x300-byte `screen_node_fields`).
    pub(crate) const FIRST_NODE: usize = 0x7da;
    pub(crate) const LAST_NODE: usize = 0x7de;
    pub(crate) const CURRENT_NODE: usize = 0x7e2;
    /// How many nodes have the single-instance flag, and how many nodes there are (signed words; at most 100 kept).
    pub(crate) const FLAGGED_COUNT: usize = 0x7e6;
    pub(crate) const NODE_COUNT: usize = 0x7e8;
    /// Set to 1 when a screen opens [community map: active_flag; inference].
    pub(crate) const ACTIVE_FLAG: usize = 0x7ea;
    /// The away and done flags (count_active_records counts the records with both 0).
    pub(crate) const AWAY: usize = 0x7ee;
    pub(crate) const DONE: usize = 0x7f2;
    /// The key of the current screen (−1 none) and a dword cleared with it [community map; inference].
    pub(crate) const SCREEN_KEY: usize = 0x7f6;
    pub(crate) const SCREEN_KEY_DATA: usize = 0x7f8;
    /// Size of a record and of a channel.
    pub(crate) const RECORD_SIZE: usize = 0x30;
    pub(crate) const CHANNEL_SIZE: usize = 0x300;
    /// The default screen (set_default_screen; open_screen uses it for a record with no key and no screen yet): build,
    /// input, flag, leave and exit functions, the slot values pointer and their count (word).
    pub(crate) const DEFAULT_BUILD: usize = 0x37d4;
    pub(crate) const DEFAULT_SLOT_VALUES: usize = 0x37e8;
    pub(crate) const DEFAULT_SLOT_COUNT: usize = 0x37ec;
    /// The network command buffer: data pointer, write position, capacity.
    pub(crate) const NET_BUFFER: usize = 0x37fe;
    pub(crate) const NET_BUFFER_POS: usize = 0x3802;
    pub(crate) const NET_BUFFER_CAPACITY: usize = 0x3806;
    /// Number of records of each channel (16 signed words).
    pub(crate) const CHANNEL_RECORD_COUNTS: usize = 0x380a;
    /// The network session and the local Display that receives the command packets.
    pub(crate) const NETWORK: usize = 0x3840;
    pub(crate) const LOCAL_DISPLAY: usize = 0x3844;
}

/// A screen history node (0x300 bytes, operator new): the screen's functions, its single-instance flag, its 60
/// parameter slots (value, owned-copy flag), the links and the background picture path.
pub(crate) mod screen_node_fields {
    pub(crate) const BUILD: usize = 0x0;
    pub(crate) const LEAVE: usize = 0x4;
    pub(crate) const INPUT: usize = 0x8;
    pub(crate) const EXIT: usize = 0xc;
    pub(crate) const FLAG: usize = 0x10;
    pub(crate) const PARAMS: usize = 0x14;
    pub(crate) const PARAM_COUNT: usize = 60;
    pub(crate) const PREV: usize = 0x1f4;
    pub(crate) const NEXT: usize = 0x1f8;
    pub(crate) const PICTURE: usize = 0x1fc;
    pub(crate) const PICTURE_SIZE: usize = 0x104;
    pub(crate) const SIZE: usize = 0x300;
}

/// "\" (the path a new screen node starts with), "" (the picture with no screen) and the Display flag that blocks the
/// redraw packet [inference].
pub(crate) const STR_BACKSLASH: usize = 0x9c3730;
pub(crate) const STR_EMPTY: usize = 0xad9c64;
pub(crate) const DISPLAY_REDRAW_BLOCKED: usize = 0x12f53d;

/// Record 0 of the current channel (`screen_manager::CURRENT_SLOT`, read anew each time, as the original does).
pub(crate) unsafe fn screen_record(mgr: usize) -> usize {
    mgr.wrapping_add((read_i16(mgr + screen_manager::CURRENT_SLOT) as isize * screen_history::CHANNEL_SIZE as isize) as usize)
}

/// The current screen node of the current channel (0 when there is none).
pub(crate) unsafe fn current_screen_node(mgr: usize) -> usize {
    read_usize(screen_record(mgr) + screen_history::CURRENT_NODE)
}

/// Copies the C string at `src` with its terminator to `dst`.
pub(crate) unsafe fn copy_c_string(dst: usize, src: usize) {
    let mut i = 0;
    loop {
        let b = read_u8(src + i);
        write_u8(dst + i, b);
        if b == 0 {
            break;
        }
        i += 1;
    }
}

/// Length of the C string at `s`.
pub(crate) unsafe fn c_string_len(s: usize) -> usize {
    let mut n = 0;
    while read_u8(s + n) != 0 {
        n += 1;
    }
    n
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
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, usize) -> u32>(%%I0%%)(mgr, build, input, flag, leave, exit);
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

// simplification: allocates and frees nodes and runs the leave function: it does not run twice. Original only; the
// plan: the result predicted from the history (0 for a single-instance screen already open) and, for a new screen, the
// new current node's functions, flag, picture and the record's marks.
/// Comparer for `screen_manager_open_screen`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_open_screen(mgr: usize, build: usize, input: usize, flag: u32, leave: usize, exit: usize) -> u32 {
    use screen_history as h;
    use screen_node_fields as n;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, usize) -> u32>(%%I0%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, usize, usize, u32)>(%%I1%%);
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

compare_value!(compare_screen_manager_count_active_records, screen_manager_count_active_records, %%I2%%, () -> u32);

/// Whether the Back command has nowhere to go: 1 when the current screen has no previous node, or when it is flagged
/// (single instance) and `ignore_root` is 0; otherwise 0.
///
/// Without a current screen: the original routine runs (fatal error box `scrman.cpp 1505` and exit).
/// Original: `ScreenManager::is_back_unavailable` (0x7fec60, scrman.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_is_back_unavailable(mgr: usize, ignore_root: u32) -> u32 {
    use screen_node_fields as n;
    let node = current_screen_node(mgr);
    if node == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I3%%)(mgr, ignore_root);
    }
    if read_usize(node + n::PREV) == 0 {
        return 1;
    }
    if ignore_root != 0 {
        return 0;
    }
    (read_u32(node + n::FLAG) != 0) as u32 // tel29_1: a flagged screen is a root for Back
}

compare_value!(compare_screen_manager_is_back_unavailable, screen_manager_is_back_unavailable, %%I3%%, (ignore_root: u32) -> u32);

/// The current screen's single-instance flag (`screen_node_fields::FLAG`).
///
/// Without a current screen: the original routine runs (fatal error box `scrman.cpp 1525` and exit).
/// Original: `ScreenManager::get_current_screen_flag` (0x7fed30, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_get_current_screen_flag(mgr: usize) -> u32 {
    let node = current_screen_node(mgr);
    if node == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I4%%)(mgr);
    }
    read_u32(node + screen_node_fields::FLAG)
}

compare_value!(compare_screen_manager_get_current_screen_flag, screen_manager_get_current_screen_flag, %%I4%%, () -> u32);

/// Whether the current screen has an exit function (`screen_node_fields::EXIT` ≠ 0).
///
/// Without a current screen: the original routine runs (fatal error box `scrman.cpp 1544` and exit).
/// Original: `ScreenManager::current_screen_has_exit_fn` (0x7fedd0, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_current_screen_has_exit_fn(mgr: usize) -> u32 {
    let node = current_screen_node(mgr);
    if node == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I5%%)(mgr);
    }
    (read_usize(node + screen_node_fields::EXIT) != 0) as u32
}

compare_value!(compare_screen_manager_current_screen_has_exit_fn, screen_manager_current_screen_has_exit_fn, %%I5%%, () -> u32);

/// The current screen's background picture path (`screen_node_fields::PICTURE`), or the empty string (`STR_EMPTY`)
/// without a current screen.
/// Original: `ScreenManager::get_background_picture` (0x7fee80, scrman.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn screen_manager_get_background_picture(mgr: usize) -> usize {
    let node = current_screen_node(mgr);
    if node == 0 { STR_EMPTY } else { node + screen_node_fields::PICTURE }
}

compare_value!(compare_screen_manager_get_background_picture, screen_manager_get_background_picture, %%I6%%, () -> usize);

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I7%%);
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

// simplification: frees the nodes: it does not run twice. Original only; the plan: the history empty.
/// Comparer for `screen_manager_free_all_screens`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_free_all_screens(mgr: usize) {
    use screen_history as h;
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I8%%)(mgr);
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
        return original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(%%I9%%)(mgr, index);
    }
    read_usize(node + screen_node_fields::PARAMS + 8 * index as usize)
}

/// Comparer for `screen_manager_get_screen_param` (the error paths only through the original: the box would show twice).
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_get_screen_param(mgr: usize, index: i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(%%I9%%);
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

// simplification: sets parameters (allocates copies, frees the old ones): it does not run twice. Original only; the
// plan: the current slot restored and, on the current record (when its key matches), the parameter set.
/// Comparer for `screen_manager_set_screen_param_for_each_record`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_screen_param_for_each_record(mgr: usize, key: u32, index: u32, value: usize, copy: u32) {
    let slot = read_u16(mgr + screen_manager::CURRENT_SLOT);
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, u32)>(%%I10%%)(mgr, key, index, value, copy);
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
        return original_fn::<unsafe extern "thiscall" fn(usize, i32, usize, u32)>(%%I11%%)(mgr, index, value, copy);
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

// simplification: allocates the copy and frees the old one: it does not run twice. Original only; the plan: the value
// (or an owned copy of the same string) in the slot.
/// Comparer for `screen_manager_set_screen_param`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_set_screen_param(mgr: usize, index: i32, value: usize, copy: u32) {
    use screen_node_fields as n;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, usize, u32)>(%%I11%%);
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
    let process: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(0x564080usize);
    let send: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x77ac30usize);
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
                process(mgr + h::LOCAL_DISPLAY, read_usize(mgr + h::NETWORK), 0, buffer);
            }
        } else if send(read_usize(mgr + h::NETWORK), read_usize(buffer), read_u32(mgr + h::NET_BUFFER_POS), player as u32) == 0 {
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

// simplification: applies or sends the packet (draws the screen, network): it does not run twice. Original only; the
// plan: the command byte 9 at the start of the buffer.
/// Comparer for `screen_manager_redraw_screen`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_redraw_screen(mgr: usize, all_channels: u32, remote_only: u32) {
    use screen_history as h;
    let blocked = read_u32(read_usize(screen_record(mgr) + screen_manager::DISPLAY) + DISPLAY_REDRAW_BLOCKED) != 0;
    let room = read_u32(mgr + h::NET_BUFFER_CAPACITY) >= 1;
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(%%I12%%)(mgr, all_channels, remote_only);
    if blocked || !room {
        return;
    }
    let ok = read_u8(read_usize(mgr + h::NET_BUFFER)) == 9;
    SCOREBOARD.record(ok, || format!("screen_manager_redraw_screen: command byte {}", read_u8(read_usize(mgr + h::NET_BUFFER))));
}
