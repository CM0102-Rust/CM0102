// ---------- batch %%LEVA%%: the screen manager's print commands: starting and ending a print job, its lines and page breaks, printing it and printing the screen (scrman.cpp) ----------

/// Delivers a print command packet for the current record: a local one (`screen_history::NETWORK_PLAYER` = 0) has it
/// applied to the local Display (`Display::process_command_packet` 0x564080, thiscall, with the session and 0); a
/// remote one has it sent (`scrman_send_packet`, box `scrman.cpp <line>`). Unlike the widget packets there is no echo
/// Display.
unsafe fn scrman_deliver_print(sm: usize, line: u32) {
    use screen_history as h;
    if read_u16(screen_record(sm) + h::NETWORK_PLAYER) == 0 {
        let process: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(0x564080usize);
        process(sm + h::LOCAL_DISPLAY, read_usize(sm + h::NETWORK), 0, sm + h::NET_BUFFER);
        return;
    }
    covered!("tel38_6 print command for a remote manager");
    scrman_send_packet(sm, line);
}

// simplification: the packet is applied to the Display (it prints or changes the print job) or sent: it does not run
// twice. Original only; the plan: the packet in the buffer, when there was room for it.
/// The comparer of a print command.
unsafe fn compare_scrman_print(name: &str, sm: usize, expected: Vec<u8>, original: impl FnOnce()) {
    let room = read_u32(sm + screen_history::NET_BUFFER_CAPACITY) as usize >= expected.len();
    original();
    scrman_check_packet(name, sm, room, &expected);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I5%%);
    compare_scrman_print("screen_manager_print_screen", sm, vec![0x15], || o(sm));
}
