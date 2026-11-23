use super::*;

/// Builds the general.dat summary: the active nations (two groups), the human managers with club and nation, the game
/// date, the version, the play time and time(). `build_save_game_summary(summary)` (cdecl): the summary that
/// `save_game_data` writes into `general.dat` (right after the version):
///
/// - +0: the nations of the league table (`G_LEAGUE_TABLE`, 0x23 of 0x48 bytes) with bit 2 set in `league_selected()`,
/// the first through sprintf with its own name as the format (`FN_CRT_SPRINTF`), the others through the translated ",
/// <%s>" (`FN_LANGLIB_SPRINTF`); +1000 (0x3e8): the same with bit 1;
///
/// - 16 managers, from the last of the person table (`G_STAFF_TABLE`, `staff::SIZE` bytes) backwards: a human one
/// (`classification()` = 1) gives the name at +0x7e4 + 0x78 × i (`database_format_staff_name`, 200) and at +0x820 +
/// 0x78 × i "club / nation" (`club_job()` and `national_job()`, name at `club::SHORT_NAME` / `nation::SHORT_NAME` of
/// each), only one of them, or nothing; a non-human one only zeroes the name (the club stays as it was in the caller's
/// buffer);
///
/// - `save_summary::GAME_DATE` the game date (`exe::date_operator_3d` of `G_CURRENT_DATE`), +0x7d0 the version
/// (`exe::utils_get_version_string`), +0xf6c the play time (`G_GAME_PLAY_TIME_TOTAL` += time() −
/// `G_GAME_PLAY_TIME_LAST_STAMP`; `G_GAME_PLAY_TIME_LAST_STAMP` = time()) and +0xf70 the time().
/// Original: `build_save_game_summary` (0x591f80, file_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn build_save_game_summary(summary: *mut u8) {
    let format: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let translate: unsafe extern "C" fn(*mut u8, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut nation = [0u8; 100];
    *summary = 0;
    *summary.add(0x3e8) = 0;
    for (bit, destino) in [(2u8, summary), (1u8, summary.add(0x3e8))] {
        let mut first = true;
        for i in 0..0x23usize {
            let n = Nation(*((G_LEAGUE_TABLE + i * 0x48) as *const usize));
            if n.is_null() || n.league_selected() & bit == 0 {
                continue;
            }
            if first {
                format(nation.as_mut_ptr(), n.addr() + nation::SHORT_NAME);
                first = false;
            } else {
                translate(nation.as_mut_ptr(), 0x9c8134, n.addr() + nation::SHORT_NAME);
            }
            append_text(destino, nation.as_ptr());
        }
        if bit == 1 && first {
            *summary.add(0x3e8) = 0;
        }
    }
    let how_many = *(G_STAFF_COUNT as *const i32);
    let table = *(G_STAFF_TABLE as *const usize);
    for i in 0..0x10i32 {
        let club_txt = summary.add(0x820 + 0x78 * i as usize);
        let p = Staff(table.wrapping_add((how_many - i - 1).wrapping_mul(staff::SIZE as i32) as usize));
        if p.classification() != 1 {
            *club_txt.sub(0x3c) = 0;
            continue;
        }
        exe::database_format_staff_name(p, (club_txt.sub(0x3c)) as *mut u8 as usize, 200, 1);
        let club = p.club_job();
        let nation_of = p.national_job();
        if !club.is_null() && !nation_of.is_null() {
            translate(club_txt, 0x9c80f4, club.addr() + club::SHORT_NAME, nation_of.addr() + nation::SHORT_NAME);
        } else if !club.is_null() {
            copy_text(club_txt, (club.addr() + club::SHORT_NAME) as *const u8);
        } else if !nation_of.is_null() {
            copy_text(club_txt, (nation_of.addr() + nation::SHORT_NAME) as *const u8);
        } else {
            *club_txt = 0;
        }
    }
    let mut copy = [0u8; 8];
    exe::date_operator_3d(summary as usize + save_summary::GAME_DATE, (copy.as_mut_ptr()) as usize, G_CURRENT_DATE);
    copy_text(summary.add(0x7d0), exe::utils_get_version_string());
    let mut now = 0i32;
    let t = exe::at_946ebb(&mut now);
    let total = (*(G_GAME_PLAY_TIME_TOTAL as *const i32)).wrapping_add(t.wrapping_sub(*(G_GAME_PLAY_TIME_LAST_STAMP as *const i32)));
    *(G_GAME_PLAY_TIME_TOTAL as *mut i32) = total;
    core::ptr::write_unaligned(summary.add(0xf6c) as *mut i32, total);
    *(G_GAME_PLAY_TIME_LAST_STAMP as *mut i32) = t;
    let t2 = exe::at_946ebb(core::ptr::null_mut());
    core::ptr::write_unaligned(summary.add(0xf70) as *mut i32, t2);
}

/// Rule: opens the file box: a list of the files (or directories) of a directory, an editable directory and file name,
/// and Cancel, Info and Ok.
///
/// `open_file_dialog(title, subtitle, mode, directory, extension, name, filter, colour 1, colour 2, command, directories
/// only, fixed directory, no cancel, info)`: without a title, a subtitle or a directory the file_screens.cpp 131 box.
/// `open_screen` with `file_dialog_build` and `file_dialog_input` (flag 1); when it opens: a 0x104-byte copy of the
/// directory (the accepted one, `strncpy` and the last byte 0), a 0x100-byte file name (the given name in save mode 1
/// when there is one, else empty) and an empty 0x104-byte result (operator new each); then the parameters in order:
/// 0 the title (copied), 1 the subtitle (copied), 2 the mode, 3 the directory, 4 the extension (copied when given), 5
/// the filter, 6 the accepted directory, 7 the file name, 8 the result, 9 no question, 0xa and 0xb the colours (their
/// low words, sign-extended), 0xc the command, 0xd directories only, 0xe and 0xf 0, 0x10 fixed directory, 0x11 no
/// cancel, 0x12 info.
/// Original: `open_file_dialog` (0x58f500, file_screens.cpp, cdecl).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn open_file_dialog(title: usize, subtitle: usize, mode: u32, directory: usize, extension: usize, name: usize,
    filter: usize, colour_1: u32, colour_2: u32, command: u32, directories: u32, fixed: u32, no_cancel: u32, info: u32) {
    if title == 0 || subtitle == 0 || directory == 0 {
        file_screens_error_box(0x83);
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x58f7c0usize, 0x58ff80usize, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel27_2 file box opened");
    let accepted = exe::crt_operator_new(0x104);
    exe::at_9466d0(accepted, directory as *const u8, 0x104);
    write_u8(accepted + 0x103, 0);
    let file_name = exe::crt_operator_new(0x100);
    if mode == 1 && name != 0 {
        exe::at_9466d0(file_name, name as *const u8, 0x100);
        write_u8(file_name + 0xff, 0);
    } else {
        write_u8(file_name, 0);
    }
    let result = exe::crt_operator_new(0x104);
    write_u8(result, 0);
    let set = |k: u32, v: usize, copy: u32| exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, k, v, copy);
    set(FILE_DIALOG_TITLE, title, 1);
    set(FILE_DIALOG_SUBTITLE, subtitle, 1);
    set(FILE_DIALOG_MODE, mode as usize, 0);
    set(FILE_DIALOG_DIRECTORY, directory, 0);
    set(FILE_DIALOG_EXTENSION, extension, (extension != 0) as u32);
    set(FILE_DIALOG_FILTER, filter, 0);
    set(FILE_DIALOG_ACCEPTED_DIRECTORY, accepted, 0);
    set(FILE_DIALOG_FILE_NAME, file_name, 0);
    set(FILE_DIALOG_RESULT, result, 0);
    set(FILE_DIALOG_QUESTION, 0, 0);
    set(FILE_DIALOG_COLOUR_1, colour_1 as i16 as isize as usize, 0);
    set(FILE_DIALOG_COLOUR_2, colour_2 as i16 as isize as usize, 0);
    set(FILE_DIALOG_COMMAND, command as usize, 0);
    set(FILE_DIALOG_DIRECTORIES, directories as usize, 0);
    set(FILE_DIALOG_SCROLL_ROW, 0, 0);
    set(FILE_DIALOG_LIST_AREA, 0, 0);
    set(FILE_DIALOG_FIXED_DIRECTORY, fixed as usize, 0);
    set(FILE_DIALOG_NO_CANCEL, no_cancel as usize, 0);
    set(FILE_DIALOG_INFO, info as usize, 0);
}

/// Comparer for `open_file_dialog`: only the original runs (it opens a screen); when a new file box opened, the plain
/// parameters against the plan and the strings and buffers by content.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn compare_open_file_dialog(title: usize, subtitle: usize, mode: u32, directory: usize, extension: usize, name: usize,
    filter: usize, colour_1: u32, colour_2: u32, command: u32, directories: u32, fixed: u32, no_cancel: u32, info: u32) {
    let before = screen_nodes_built_by(0x58f7c0);
    let dir_text: Vec<u8> = file_c_string(directory).into_iter().take(0x103).collect();
    let name_text: Vec<u8> = if mode == 1 && name != 0 { file_c_string(name).into_iter().take(0xff).collect() } else { Vec::new() };
    let strings = [file_c_string(title), file_c_string(subtitle), file_c_string(extension)];
    original_fn::<unsafe extern "C" fn(usize, usize, u32, usize, usize, usize, usize, u32, u32, u32, u32, u32, u32, u32)>(IDX_OPEN_FILE_DIALOG)(
        title, subtitle, mode, directory, extension, name, filter, colour_1, colour_2, command, directories, fixed, no_cancel, info);
    if title == 0 || subtitle == 0 || directory == 0 || !screen_newly_opened(&before, 0x58f7c0) {
        return;
    }
    let plain = [(2u32, mode as usize), (3, directory), (5, filter), (9, 0), (0xa, colour_1 as i16 as isize as usize),
        (0xb, colour_2 as i16 as isize as usize), (0xc, command as usize), (0xd, directories as usize), (0xe, 0), (0xf, 0),
        (0x10, fixed as usize), (0x11, no_cancel as usize), (0x12, info as usize)];
    let plain_ok = plain.iter().all(|&(k, v)| get_screen_param(k) == v);
    let got_strings = [file_c_string(get_screen_param(0)), file_c_string(get_screen_param(1)), file_c_string(get_screen_param(4))];
    let buffers_ok = file_c_string(get_screen_param(6)) == dir_text && file_c_string(get_screen_param(7)) == name_text
        && file_c_string(get_screen_param(8)).is_empty() && (extension != 0) == (get_screen_param(4) != 0);
    SCOREBOARD.record(plain_ok && buffers_ok && got_strings == strings, || {
        let got: Vec<usize> = (0..0x13).map(|k| get_screen_param(k)).collect();
        format!("file box (open): params {:x?}, strings and buffers {}", got, if buffers_ok { "equal" } else { "differ" })
    });
}

/// Rule: the file box: the directory (editable unless fixed), the file name (editable), up to six visible rows of the
/// directory's files (or subdirectories), and Cancel, Info and Ok.
///
/// `file_dialog_build()`: the background (`background_set_random_picture`), the title (title colours: parameters 0xb
/// and 0xa), the menu bar (4) and the subtitle. A 2-column area (0x6e, 0x91)-(0x30c, 0xa5) (weights 2/7) with
/// "Directory :" and the directory as an edit field (flags 0x8008, 0x103 characters, id 1) or, fixed, as text; a
/// 2-column area (0xbe, 0xc3)-(0x2bc, 0xe7) with "File :" and the file name as an edit field (flags 0x18, 0xff
/// characters). The list (0xbe, 0xea)-(0x2bc, 0x1fe), one column, 6 rows: `file_find_first` of "<directory>\*.<ext>"
/// (`_makepath`; attribute 0x10 for directories): for directories each subdirectory but "." and ".."; for files each
/// file the filter keeps (bit 0; no filter keeps all) as a button (id 2, 0xaebe04 on its row, text colour 0xae317e
/// with bit 2, 0xaebdec with bit 3, else 0xae3184) with its name without the extension (`_splitpath`); empty rows up to
/// six. The list's scroll row comes from parameter 0xe and the area goes to parameter 0xf. The buttons (0x64,
/// 0x22b)-(0x316, 0x24e): "Cancel" (id 3; grey text with no cancel), "Info" (id 5, only with the Info button) and
/// "Ok" (id 4).
/// Original: `file_dialog_build` (0x58f7c0, file_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn file_dialog_build() {
    let directory_weights: [u8; 4] = [2, 7, 3, 1];
    let button_weights: [u8; 3] = [2, 1, 1];
    let title = get_screen_param(FILE_DIALOG_TITLE);
    let subtitle = get_screen_param(FILE_DIALOG_SUBTITLE);
    get_screen_param(FILE_DIALOG_MODE);
    let directory = get_screen_param(FILE_DIALOG_DIRECTORY);
    let extension = get_screen_param(FILE_DIALOG_EXTENSION);
    let filter = get_screen_param(FILE_DIALOG_FILTER);
    let file_name = get_screen_param(FILE_DIALOG_FILE_NAME);
    let colour_1 = get_screen_param(FILE_DIALOG_COLOUR_1);
    let colour_2 = get_screen_param(FILE_DIALOG_COLOUR_2);
    let directories = get_screen_param(FILE_DIALOG_DIRECTORIES);
    let fixed = get_screen_param(FILE_DIALOG_FIXED_DIRECTORY);
    let no_cancel = get_screen_param(FILE_DIALOG_NO_CANCEL);
    let info = get_screen_param(FILE_DIALOG_INFO);
    exe::background_set_random_picture();
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, colour_2 as u32, colour_1 as u32, title);
    exe::menubar_build(4, 0);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, subtitle, 0);
    let display = current_human_display();
    let area = tel26_add_area(display, [0x6e, 0x91, 0x30c, 0xa5, 2, directory_weights.as_ptr() as u32, 1, 0, 2, 0, u32::MAX]);
    comp_lang_format()(JOB_TEXT, 0x9c7a90);
    tel26_area_text([area, 0, 0, 1, 0, 0xc, 2, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
    if fixed == 0 {
        tel26_add_item(current_human_display(), [0x8008, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 1, job_colour(JOB_COLOUR_BDE8), directory as u32, 0x103, 1, 0, area]);
    } else {
        tel26_area_text([area, 1, 0, 1, 0, 1, 1, job_colour(JOB_COLOUR_BDE8), directory as u32]);
    }
    let area = tel26_add_area(current_human_display(), [0xbe, 0xc3, 0x2bc, 0xe7, 2, directory_weights.as_ptr() as u32, 1, 0, 2, 0, u32::MAX]);
    comp_lang_format()(JOB_TEXT, 0x9c7a88);
    tel26_area_text([area, 0, 0, 1, 0, 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
    tel26_add_item(current_human_display(), [0x18, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 3, job_colour(JOB_COLOUR_BDE8), file_name as u32, 0xff, 0, 0, area]);
    let list = tel26_add_area(current_human_display(), [0xbe, 0xea, 0x2bc, 0x1fe, 1, 0, 6, 0, 1, 0, u32::MAX]);
    let mut pattern = [0u8; 0x104];
    let mut found = [0u8; 0x200];
    let fd = found.as_mut_ptr() as usize;
    exe::crt_makepath(pattern.as_mut_ptr(), 0usize, directory, 0x989ad8usize, extension);
    let mut row: u32 = 0;
    if exe::file_find_first(pattern.as_ptr() as usize, fd, if directories != 0 { 0x10 } else { 0 }) == 0 {
        loop {
            let name = fd + FILE_FIND_NAME;
            let mut colour = None;
            if directories != 0 {
                if read_u32(fd + FILE_FIND_ATTRIBUTES) & 0x10 != 0 && file_c_string(name) != b"." && file_c_string(name) != b".." {
                    exe::crt_strtok(name, 0usize, 0usize, pattern.as_mut_ptr(), core::ptr::null_mut());
                    colour = Some(job_colour(JOB_COLOUR_3184));
                }
            } else {
                exe::crt_makepath(pattern.as_mut_ptr(), 0usize, directory, name, 0usize);
                let flags = if filter != 0 {
                    let f: unsafe extern "C" fn(*const u8) -> u32 = core::mem::transmute(filter);
                    f(pattern.as_ptr())
                } else {
                    1
                };
                if flags & 1 != 0 {
                    exe::crt_strtok(name, 0usize, 0usize, pattern.as_mut_ptr(), core::ptr::null_mut());
                    colour = Some(if flags & 4 != 0 {
                        job_colour(JOB_COLOUR_317E)
                    } else if flags & 8 != 0 {
                        job_colour(JOB_COLOUR_BDEC)
                    } else {
                        job_colour(JOB_COLOUR_3184)
                    });
                }
            }
            if let Some(colour) = colour {
                tel26_area_button([list, 0, row, 0x22, job_colour(JOB_COLOUR_BE04), 0xc, 4, colour, pattern.as_ptr() as u32, 2, 0]);
                row += 1;
            }
            if exe::file_find_next(found.as_mut_ptr()) != 0 {
                break;
            }
        }
    }
    exe::file_find_close(fd);
    while row < 6 {
        tel26_area_text([list, 0, row, 0x22, job_colour(JOB_COLOUR_BE04), 0xc, 4, job_colour(JOB_COLOUR_3184), 0]);
        row += 1;
    }
    let scroll = get_screen_param(FILE_DIALOG_SCROLL_ROW);
    if list as u16 != 0xffff {
        exe::area_set_scroll_row(current_human_display() + 4 + (list as i16 as isize as usize).wrapping_mul(0xbf1), scroll);
    }
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, FILE_DIALOG_LIST_AREA, list as i16 as isize as usize, 0);
    let cancel_column = |buttons: u32| {
        comp_lang_format()(JOB_TEXT, 0x98a1a0);
        if no_cancel == 0 {
            tel26_area_button([buttons, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 3, 0]);
        } else {
            tel26_area_text([buttons, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0x2c, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
        }
    };
    let ok_column;
    let buttons;
    if info != 0 {
        buttons = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 3, button_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
        cancel_column(buttons);
        comp_lang_format()(JOB_TEXT, 0x9c7a78);
        tel26_area_button([buttons, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 5, 0]);
        ok_column = 2;
    } else {
        buttons = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, directory_weights[2..].as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
        cancel_column(buttons);
        ok_column = 1;
    }
    comp_lang_format()(JOB_TEXT, 0x990f7c);
    tel26_area_button([buttons, ok_column, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 4, 0]);
}

/// Comparer for `file_dialog_build`: side by side over the display list (the directory is read twice).
pub(crate) unsafe extern "C" fn compare_file_dialog_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_FILE_DIALOG_BUILD);
    compare_match_screen_build("file box", &[], || o(), || file_dialog_build());
}

/// Rule: the file box's clicks: the directory typed in (accepted when it exists; else, in save mode, the question
/// whether to create it), a file picked from the list, Cancel, Info (the save game's info screen) and Ok (checked by the
/// mode: the name not blank, the file there to load or delete, or the question whether to replace it); the answers of
/// the questions come back on the next build. Enter, Y or y is Ok; Esc, N or n Cancel.
///
/// `file_dialog_input(widget)`: with a box just answered (`SCREEN_MANAGER_BOX_ANSWERED`), the question pending
/// (parameter 9, cleared) and the answer ([`G_PENDING_COMMAND_ARG`], 2 yes, 1 no; cleared): creating the directory,
/// yes: `_mkdir` (failing: "The directory ... could not be created." / "Unable To Create", the directory restored from
/// the accepted one, −4); no: the directory restored; replace or delete, yes: `file_dialog_leave` with the result path
/// (−9). Without one, a click on a widget saves the list's scroll row into parameter 0xe. Then the item's id
/// (`tel26_item_id`): 1, a typed directory (not empty and missing: in mode 1 question 1 with "The directory ... could
/// not be found. Create this directory ?" (yes/no), else "... could not be found." and the directory restored; −4;
/// otherwise it becomes the accepted one, −11); 2, a file of the list: its text into the file name (0x100), −11; 3,
/// Cancel: the three buffers freed, the pending command parameter 0xc with argument 0, −9; 5, Info: a blank name
/// "The file name entered is blank."; else the result path (`file_dialog_result_path`): missing "The file ... does not
/// exist."; else general.dat of it (`zopen`) gives the save version (`zread` 4 bytes): a version (absolute) from
/// 0x3e023 opens the save game info screen (`create_save_file_summary_screen`, `save_game_info_input`, flag 1) with
/// parameter 0 the path (copied), else "Could not find information about save game.", −4; 4, Ok: a blank name ("The
/// directory name entered is blank." with directories only, else the file's); mode 0: the file must exist (unless
/// directories only) to leave; mode 1: a new file (or directories only) leaves, an existing one asks "The file ...
/// already exists. Replace this file ?" (question 3); mode 2: a missing file "does not exist", else "Delete the file
/// ... ?" (question 2); −4 for a question. Id 0 is the keyboard: Enter, Y, y as Ok; Esc, N, n as Cancel (the key
/// cleared). The rest goes to `menubar_handle_command`, then `menubar_handle_object_link`.
/// Defect of the original: Info closes general.dat twice when its version is too old (`zclose` of the cleared handle).
/// Original: `file_dialog_input` (0x58ff80, file_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn file_dialog_input(widget: u32) -> i32 {
    let item = widget as i16;
    if read_u32(SCREEN_MANAGER_BOX_ANSWERED) != 0 {
        let title = get_screen_param(FILE_DIALOG_TITLE);
        let directory = get_screen_param(FILE_DIALOG_DIRECTORY);
        let accepted = get_screen_param(FILE_DIALOG_ACCEPTED_DIRECTORY);
        get_screen_param(FILE_DIALOG_FILE_NAME);
        let result = get_screen_param(FILE_DIALOG_RESULT);
        let question = get_screen_param(FILE_DIALOG_QUESTION);
        let colour_1 = get_screen_param(FILE_DIALOG_COLOUR_1);
        let colour_2 = get_screen_param(FILE_DIALOG_COLOUR_2);
        match question {
            1 => {
                set_screen_param(FILE_DIALOG_QUESTION, 0);
                match read_u32(G_PENDING_COMMAND_ARG) {
                    2 => {
                        write_u32(G_PENDING_COMMAND_ARG, 0);
                        if file_make_directory(directory) != 0 {
                            covered!("tel27_2 directory not created");
                            file_dialog_confirm(title, 0x9c7c98, 0x9c7cac, directory, 0, colour_1, colour_2);
                            exe::at_9466d0(directory, accepted as *const u8, 0x104);
                            write_u8(directory + 0x103, 0);
                            return -4;
                        }
                    }
                    1 => {
                        write_u32(G_PENDING_COMMAND_ARG, 0);
                        exe::at_9466d0(directory, accepted as *const u8, 0x104);
                        write_u8(directory + 0x103, 0);
                    }
                    _ => {}
                }
            }
            2 | 3 => {
                set_screen_param(FILE_DIALOG_QUESTION, 0);
                match read_u32(G_PENDING_COMMAND_ARG) {
                    2 => {
                        covered!("tel27_2 question answered yes");
                        write_u32(G_PENDING_COMMAND_ARG, 0);
                        return file_dialog_leave(result);
                    }
                    1 => write_u32(G_PENDING_COMMAND_ARG, 0),
                    _ => {}
                }
            }
            _ => write_u32(G_PENDING_COMMAND_ARG, 0),
        }
    } else if item != -1 {
        let row = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(FILE_DIALOG_LIST_AREA));
        exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, FILE_DIALOG_SCROLL_ROW, row, 0);
    }
    let mut id = tel26_item_id(item) as i16 as i32;
    if id == 0 {
        match read_i8(DISPLAY_KEY_CHAR) {
            10 | 13 | 89 | 121 => {
                write_u8(DISPLAY_KEY_CHAR, 0);
                id = 4;
            }
            27 | 78 | 110 => {
                write_u8(DISPLAY_KEY_CHAR, 0);
                id = 3;
            }
            _ => {}
        }
    }
    match id {
        1 => file_dialog_directory_typed(),
        2 => {
            let file_name = get_screen_param(FILE_DIALOG_FILE_NAME);
            let text = if item != -1 { display_item_at(current_human_display(), item) + 0x80 } else { 0 };
            exe::at_9466d0(file_name, text as *const u8, 0x100);
            write_u8(file_name + 0xff, 0);
            -11
        }
        3 => {
            let accepted = get_screen_param(FILE_DIALOG_ACCEPTED_DIRECTORY);
            let file_name = get_screen_param(FILE_DIALOG_FILE_NAME);
            let result = get_screen_param(FILE_DIALOG_RESULT);
            exe::crt_operator_delete(accepted);
            exe::crt_operator_delete(file_name);
            exe::crt_operator_delete(result);
            write_usize(G_PENDING_COMMAND_CODE, get_screen_param(FILE_DIALOG_COMMAND));
            write_u32(G_PENDING_COMMAND_ARG, 0);
            -9
        }
        5 => file_dialog_info(),
        4 => file_dialog_ok(widget),
        _ => {
            let r = exe::menubar_handle_command(widget);
            if r != 0 {
                return r;
            }
            exe::menubar_handle_object_link(widget)
        }
    }
}

/// Comparer for `file_dialog_input`: side by side over the screen's parameters, the directory, the accepted directory
/// and the file name and the answer, for the clicks that only copy text (no box answered: a file of the list, or a
/// typed directory that exists or is empty); the rest (boxes, files, freeing, leaving, the menu bar) only the original.
pub(crate) unsafe extern "C" fn compare_file_dialog_input(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_FILE_DIALOG_INPUT);
    let item = widget as i16;
    let id = tel26_item_id(item);
    let directory = get_screen_param(FILE_DIALOG_DIRECTORY);
    let simple = read_u32(SCREEN_MANAGER_BOX_ANSWERED) == 0 && item != -1
        && (id == 2 || (id == 1 && (c_string_len(directory) == 0 || file_is_directory(directory) != 0)));
    if !simple {
        return o(widget);
    }
    let mut regions = screen_param_regions();
    regions.extend_from_slice(&[(directory, 0x104), (get_screen_param(FILE_DIALOG_ACCEPTED_DIRECTORY), 0x104), (get_screen_param(FILE_DIALOG_FILE_NAME), 0x100)]);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(widget), || file_dialog_input(widget));
    SCOREBOARD.record(equal && ro == rr, || format!("file box (click {}): answer {} x {}, differences in {:?}", id, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Rule: the save game info screen: what general.dat's summary of a save holds.
///
/// `create_save_file_summary_screen()`: parameter 0 the save's path; general.dat in it (`zopen`; failing: the load
/// error box line 1171), the version (4 bytes; failing: line 1178, closed) and the summary (0xf74 bytes from version
/// 0x3e027, else 0xf70 with the time of the save 0; failing: line 1189 / 1202, closed); closed. A dark panel over the
/// whole screen (the screen's size as the running exe has it), a green 0x38 area (0x78, 0x64)-(0x301, 0x1f3), the
/// heading "Save Game Info", an "Ok" button (id 1) in (0x82, 0x1cb)-(0x2f2, 0x1e9), a pitch (0x8c, 0xaf)-(0x2e8, 0x1b7),
/// and a 2-column area (0x91, 0xb9)-(0x2e3, 0x1b2) (weights 2/5, 12 rows): "File Name" (the path), "Last Saved"
/// (`localtime` of the time of the save: "<hours>:<minutes>, <weekday> <day> <month> <year>"), "Version", "Date" (the game
/// date, `Date::get_long_date_string`), "Game Time" ("Days: .. Hours: .. Minutes: .. Seconds: .." of the play time); in
/// English with a day or more of play, "Addictedness Rating" and its comment (`file_addictedness`); "Foreground Leagues"
/// and the nations' text (wrapped from x 315 (594 / (7 / 2) + 146 of the weights) to 0x2e3, `Text::draw_wrapped`, font
/// 2), "Background Leagues" and theirs when there are any, and "Managers" with each human ("<name> (<club>)" or the
/// name).
/// Original: `create_save_file_summary_screen` (0x590c00, file_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn create_save_file_summary_screen() {
    if read_u8(0xae2a36) & 1 == 0 {
        write_u8(0xae2a36, read_u8(0xae2a36) | 1);
        write_u16(0xae2a34, exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0) as u16);
    }
    let weights: [u8; 2] = [2, 5];
    let mut summary = vec![0u8; 0xf74];
    let summary = summary.as_mut_ptr() as usize;
    exe::date_date_54a110(summary + save_summary::GAME_DATE);
    let mut text = [0usize; 2];
    exe::new_text(text.as_mut_ptr(), 0usize);
    let path = get_screen_param(0);
    let mut z = exe::zopen(path, 0x9c7b84usize, 0, 0x15, 4);
    if z == 0 {
        file_load_error_box(0x493, 0x9c80a8);
        exe::drop_text(text.as_mut_ptr());
        return;
    }
    let mut version: i32 = 0;
    if exe::zread(&mut version as *mut i32 as usize, 4, 1, z) != 1 {
        file_load_error_box(0x49a, 0x9c8058);
        exe::zclose(&mut z);
        exe::drop_text(text.as_mut_ptr());
        return;
    }
    let long = version.wrapping_abs() >= SAVE_SUMMARY_LONG_VERSION;
    if exe::zread(summary, if long { 0xf74 } else { 0xf70 }, 1, z) != 1 {
        file_load_error_box(if long { 0x4b2 } else { 0x4a5 }, 0x9c800c);
        exe::zclose(&mut z);
        exe::drop_text(text.as_mut_ptr());
        return;
    }
    if !long {
        write_u32(summary + 0xf70, 0);
    }
    covered!("tel27_2 save game info");
    exe::zclose(&mut z);
    exe::background_set_picture(0usize);
    let (width, height) = (exe_immediate(0x590f2b as *const i32) as u32, exe_immediate(0x590f23 as *const i32) as u32);
    tel26_add_area(current_human_display(), [0, 0, width, height, 1, 0, 1, 0, 2, 0, u32::MAX]);
    let green = exe::graphics_rgb_to_pixel(0, 0x38, 0, 0);
    tel26_add_area(current_human_display(), [0x78, 0x64, 0x301, 0x1f3, 1, 0, 1, 0, 0x30, green, u32::MAX]);
    comp_lang_format()(JOB_TEXT, 0x9c7ffc);
    comp_text_item([0x82, 0x6e, 0x2f2, 0x9b, 0x30, job_colour(JOB_COLOUR_BDEA), 0xc, 4, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 1]);
    let ok = tel26_add_area(current_human_display(), [0x82, 0x1cb, 0x2f2, 0x1e9, 1, 0, 1, 0, 1, 0, u32::MAX]);
    comp_lang_format()(JOB_TEXT, 0x990f7c);
    tel26_area_button([ok, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]);
    let (mut left, mut top, mut right, mut bottom) = (0i32, 0i32, 0i32, 0i32);
    let pitch: unsafe extern "C" fn(i32, i32, i32, i32, *mut i32, *mut i32, *mut i32, *mut i32, u32, i32, u32) -> u32 = core::mem::transmute(FN_DRAW_PITCH);
    pitch(0x8c, 0xaf, 0x2e8, 0x1b7, &mut left, &mut top, &mut right, &mut bottom, 1, ok as i32, 0x4080_0000);
    let area = tel26_add_area(current_human_display(), [0x91, 0xb9, 0x2e3, 0x1b2, 2, weights.as_ptr() as u32, 0xc, 0, 1, 0, u32::MAX]);
    let x = (594.0f64 / ((weights[0] as f64 + weights[1] as f64) / weights[0] as f64) + 146.0) as i32;
    let mut row: i32 = 0;
    let label = |row: i32, format: usize| {
        comp_lang_format()(JOB_TEXT, format);
        tel26_area_text([area, 0, row as u32, 1, 0, 1, 2, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
    };
    let value = |row: i32, text: usize| tel26_area_text([area, 1, row as u32, 1, 0, 1, 2, job_colour(JOB_COLOUR_BDEC), text as u32]);
    label(row, 0x9c7ff0);
    value(row, path);
    row += 1;
    label(row, 0x9c7fe4);
    file_last_saved(summary + 0xf70);
    value(row, JOB_TEXT);
    row += 1;
    label(row, 0x9c7f60);
    value(row, summary + 0x7d0);
    row += 1;
    label(row, 0x988078);
    exe::date_get_long_date_string(summary + save_summary::GAME_DATE, JOB_TEXT, 0x7d0, 0xff);
    value(row, JOB_TEXT);
    row += 1;
    comp_lang_format()(JOB_TEXT, 0x9c7f54);
    let played = read_i32(summary + save_summary::PLAY_TIME_TOTAL);
    let (days, hours, minutes, seconds) = (played / 86400, played % 86400 / 3600, played % 86400 % 3600 / 60, played % 86400 % 3600 % 60);
    tel26_area_text([area, 0, row as u32, 1, 0, 1, 2, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
    comp_lang_format()(JOB_TEXT, 0x9c7ed0, days, hours, minutes, seconds);
    value(row, JOB_TEXT);
    row += 1;
    if exe::langlib_get_language() == 0 && days > 0 {
        comp_crt_format()(JOB_TEXT, 0x9c7ebc);
        tel26_area_text([area, 0, row as u32, 1, 0, 1, 2, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
        comp_crt_format()(JOB_TEXT, file_addictedness(days, hours));
        value(row, JOB_TEXT);
        row += 1;
    }
    let wrapped = |row: &mut i32, nations: usize| {
        comp_lang_format()(JOB_TEXT, nations);
        exe::text_set_string(text.as_ptr() as usize, JOB_TEXT as *const u8);
        exe::text_draw_wrapped(text.as_ptr() as usize, area, x, 0x2e3, row, 2, job_colour(JOB_COLOUR_BDEC), 0, 0);
        *row += 1;
    };
    label(row, 0x9c7d2c);
    wrapped(&mut row, summary);
    if read_u8(summary + 0x3e8) != 0 {
        label(row, 0x9c7d18);
        wrapped(&mut row, summary + 0x3e8);
    }
    label(row, 0x9c7d0c);
    for i in 0..0x10usize {
        let manager = summary + 0x7e4 + i * 0x78;
        if read_u8(manager) == 0 {
            continue;
        }
        if read_u8(manager + 0x3c) != 0 {
            comp_lang_format()(JOB_TEXT, 0x9c7ce4, manager, manager + 0x3c);
        } else {
            tel26_copy_text(JOB_TEXT as *mut u8, manager as *const u8);
        }
        value(row, JOB_TEXT);
        row += 1;
    }
    exe::drop_text(text.as_mut_ptr());
}

/// Comparer for `create_save_file_summary_screen`: side by side over the display list and the colour made once (the
/// save is read twice).
pub(crate) unsafe extern "C" fn compare_create_save_file_summary_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CREATE_SAVE_FILE_SUMMARY_SCREEN);
    compare_match_screen_build("save game info", &[(0xae2a34, 4)], || o(), || create_save_file_summary_screen());
}

/// Rule: the save game info screen's only click: "Ok" (id 1) closes it (−9); anything else 0.
///
/// `save_game_info_input(widget)`: the item's id (`tel26_item_id`) 1 gives −9, the rest 0.
/// Original: `save_game_info_input` (0x5917f0, file_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn save_game_info_input(widget: u32) -> i32 {
    if tel26_item_id(widget as i16) == 1 { -9 } else { 0 }
}

/// Comparer for `save_game_info_input`: the answer side by side (it writes nothing).
pub(crate) unsafe extern "C" fn compare_save_game_info_input(widget: u32) -> i32 {
    let ro = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_SAVE_GAME_INFO_INPUT)(widget);
    let rr = save_game_info_input(widget);
    SCOREBOARD.record(ro == rr, || format!("save game info (click): answer {} x {}", ro, rr));
    ro
}

/// Rule: opens a confirmation box: a title, a heading and a message, with "Ok" or "No" and "Yes".
///
/// `confirm_push(title, heading, message, yes/no, colour 1, colour 2, command)`: without a title, a heading or a message
/// the file_screens.cpp 1506 box. `open_screen` with `confirm_build` and `confirm_input` (flag 1); when it opens the
/// parameters 0..2 the three strings (copied), 3 yes/no, 4 and 5 the colours' low words sign-extended, 6 no Text, 7
/// the command.
/// Original: `confirm_push` (0x591840, file_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn confirm_push(title: usize, heading: usize, message: usize, yes_no: u32, colour_1: u32, colour_2: u32, command: u32) {
    if title == 0 || heading == 0 || message == 0 {
        file_screens_error_box(0x5e2);
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x591b70usize, 0x591dc0usize, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel27_2 confirmation box");
    let set = |k: u32, v: usize, copy: u32| exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, k, v, copy);
    set(CONFIRM_TITLE, title, 1);
    set(CONFIRM_HEADING, heading, 1);
    set(CONFIRM_MESSAGE, message, 1);
    set(CONFIRM_YES_NO, yes_no as usize, 0);
    set(CONFIRM_COLOUR_1, colour_1 as i16 as isize as usize, 0);
    set(CONFIRM_COLOUR_2, colour_2 as i16 as isize as usize, 0);
    set(CONFIRM_TEXT, 0, 0);
    set(CONFIRM_COMMAND, command as usize, 0);
}

/// Comparer for `confirm_push`: only the original runs (it opens a screen); when a new box opened, its parameters
/// against the plan.
pub(crate) unsafe extern "C" fn compare_confirm_push(title: usize, heading: usize, message: usize, yes_no: u32, colour_1: u32, colour_2: u32, command: u32) {
    let before = screen_nodes_built_by(0x591b70);
    let strings = [file_c_string(title), file_c_string(heading), file_c_string(message)];
    original_fn::<unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32)>(IDX_CONFIRM_PUSH)(title, heading, message, yes_no, colour_1, colour_2, command);
    if title != 0 && heading != 0 && message != 0 && screen_newly_opened(&before, 0x591b70) {
        compare_confirm_opened("confirmation box (open)", &strings, yes_no, colour_1, colour_2, Some(0), command);
    }
}

/// Rule: opens a confirmation box whose message is a Text (with links), copied for the box.
///
/// `confirm_push_text_object(title, heading, text, yes/no, colour 1, colour 2, command)`: without a title, a heading or a
/// text the file_screens.cpp 1553 box. A copy of the Text (operator new 8, `Text::Text(copy of)`); `open_screen` as
/// `confirm_push`; when it opens the parameters as there with no message (2 = 0) and the copy in 6.
/// Defect of the original: when the screen does not open, the copy is never freed.
/// Original: `confirm_push_text_object` (0x5919b0, file_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn confirm_push_text_object(title: usize, heading: usize, text: usize, yes_no: u32, colour_1: u32, colour_2: u32, command: u32) {
    if title == 0 || heading == 0 || text == 0 {
        file_screens_error_box(0x611);
        return;
    }
    let memory = exe::crt_operator_new(8);
    let copy = if memory != 0 {
        exe::text_text_4ae5c0(memory, text)
    } else {
        0
    };
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x591b70usize, 0x591dc0usize, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel27_2 confirmation box with a text");
    let set = |k: u32, v: usize, copy: u32| exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, k, v, copy);
    set(CONFIRM_TITLE, title, 1);
    set(CONFIRM_HEADING, heading, 1);
    set(CONFIRM_MESSAGE, 0, 0);
    set(CONFIRM_YES_NO, yes_no as usize, 0);
    set(CONFIRM_COLOUR_1, colour_1 as i16 as isize as usize, 0);
    set(CONFIRM_COLOUR_2, colour_2 as i16 as isize as usize, 0);
    set(CONFIRM_TEXT, copy, 0);
    set(CONFIRM_COMMAND, command as usize, 0);
}

/// Comparer for `confirm_push_text_object`: only the original runs (it opens a screen and allocates the copy); when a
/// new box opened, its parameters against the plan (the copy any non-null one).
pub(crate) unsafe extern "C" fn compare_confirm_push_text_object(title: usize, heading: usize, text: usize, yes_no: u32, colour_1: u32, colour_2: u32, command: u32) {
    let before = screen_nodes_built_by(0x591b70);
    let strings = [file_c_string(title), file_c_string(heading), Vec::new()];
    original_fn::<unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32)>(IDX_CONFIRM_PUSH_TEXT_OBJECT)(title, heading, text, yes_no, colour_1, colour_2, command);
    if title != 0 && heading != 0 && text != 0 && screen_newly_opened(&before, 0x591b70) {
        compare_confirm_opened("confirmation box with a text (open)", &strings, yes_no, colour_1, colour_2, None, command);
    }
}

/// Rule: the confirmation box: the title, the heading, the message (or the Text) and "Ok" (id 3) or "No" (id 1) and
/// "Yes" (id 2).
///
/// `confirm_build()`: the background (`background_set_random_picture`), the menu bar (4), the title (colours: parameters
/// 5 and 4) and the heading as subtitle. The message as a text area (0x6e, 0x91)-(0x30c, 0x217) (16 lines, font 4,
/// `gui_add_text_area`), or the Text (`gui_add_text_area_from_text`). Without yes/no an "Ok" button item (0x64,
/// 0x22b)-(0x316, 0x24e) id 3; with yes/no 1 a 2-column area there (weights 3/1) with "No" (id 1) and "Yes" (id 2);
/// another yes/no value puts the area at (0, 0)-(3, 1) (as the original's code has it).
/// Original: `confirm_build` (0x591b70, file_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn confirm_build() {
    let weights: [u8; 2] = [3, 1];
    let title = get_screen_param(CONFIRM_TITLE);
    let heading = get_screen_param(CONFIRM_HEADING);
    let message = get_screen_param(CONFIRM_MESSAGE);
    let yes_no = get_screen_param(CONFIRM_YES_NO);
    let colour_1 = get_screen_param(CONFIRM_COLOUR_1);
    let colour_2 = get_screen_param(CONFIRM_COLOUR_2);
    let text = get_screen_param(CONFIRM_TEXT);
    exe::background_set_random_picture();
    exe::menubar_build(4, 0);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, colour_2 as u32, colour_1 as u32, title);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, heading, 0);
    if message != 0 {
        tel26_text_area([0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, job_colour(JOB_COLOUR_3184), message as u32, 0]);
    } else {
        exe::gui_add_text_area_from_text(0x6e, 0x91, 0x30c, 0x217, 2, 0, (0x10) as u32 as i32, 4, job_colour(JOB_COLOUR_3184), text);
    }
    if yes_no == 0 {
        comp_lang_format()(JOB_TEXT, 0x990f7c);
        tel26_add_item(current_human_display(), [2, 0x64, 0x22b, 0x316, 0x24e, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), job_colour(JOB_COLOUR_317E),
            0xc, 4, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0, 3, 0, u32::MAX]);
        return;
    }
    covered!("tel27_2 confirmation yes and no");
    let rect: [u32; 4] = if yes_no == 1 { [0x64, 0x22b, 0x316, 0x24e] } else { [0, 0, 3, 1] };
    let area = tel26_add_area(current_human_display(), [rect[0], rect[1], rect[2], rect[3], 2, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    comp_lang_format()(JOB_TEXT, 0x98e8cc);
    tel26_area_button([area, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]);
    comp_lang_format()(JOB_TEXT, 0x98e8d0);
    tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]);
}

/// Comparer for `confirm_build`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_confirm_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_CONFIRM_BUILD);
    compare_match_screen_build("confirmation box", &[], || o(), || confirm_build());
}

/// Rule: the confirmation box's answer: "No" 1, "Yes" 2, "Ok" 3 (and ids 4, 5); Enter, Y or y is Yes (or Ok without
/// yes/no), Esc, N or n No. The answer closes the box: the Text freed, the pending command parameter 7 with the answer
/// as argument, −9.
///
/// `confirm_input(widget)`: the item's id (`tel26_item_id`); 0 is the keyboard ([`DISPLAY_KEY_CHAR`], cleared): Enter,
/// Y, y → 2 with yes/no (parameter 3), else 3; Esc, N, n → 1. An answer 1..5: the Text of parameter 6 (when there is
/// one) destroyed and freed and the parameter cleared; [`G_PENDING_COMMAND_CODE`] = parameter 7,
/// [`G_PENDING_COMMAND_ARG`] = the answer; −9. The rest goes to `menubar_handle_command`, then
/// `menubar_handle_object_link`.
/// Original: `confirm_input` (0x591dc0, file_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn confirm_input(widget: u32) -> i32 {
    let mut answer = tel26_item_id(widget as i16) as i16 as i32;
    if answer == 0 {
        match read_i8(DISPLAY_KEY_CHAR) {
            10 | 13 | 89 | 121 => {
                let yes_no = get_screen_param(CONFIRM_YES_NO);
                write_u8(DISPLAY_KEY_CHAR, 0);
                answer = if yes_no != 0 { 2 } else { 3 };
            }
            27 | 78 | 110 => {
                answer = 1;
                write_u8(DISPLAY_KEY_CHAR, 0);
            }
            _ => {}
        }
    }
    if !(1..=5).contains(&answer) {
        let r = exe::menubar_handle_command(widget);
        if r != 0 {
            return r;
        }
        return exe::menubar_handle_object_link(widget);
    }
    if get_screen_param(CONFIRM_TEXT) != 0 {
        let text = get_screen_param(CONFIRM_TEXT);
        if text != 0 {
            exe::drop_text(text as *mut usize);
            exe::crt_operator_delete(text);
        }
        set_screen_param(CONFIRM_TEXT, 0);
    }
    write_usize(G_PENDING_COMMAND_CODE, get_screen_param(CONFIRM_COMMAND));
    write_u32(G_PENDING_COMMAND_ARG, answer as u32);
    -9
}

/// Comparer for `confirm_input`: only the original runs (it frees the Text); for an answer, the pending command, its
/// argument, the Text parameter and the answer are checked against the plan afterwards.
pub(crate) unsafe extern "C" fn compare_confirm_input(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_CONFIRM_INPUT);
    let mut answer = tel26_item_id(widget as i16) as i16 as i32;
    if answer == 0 {
        answer = match read_i8(DISPLAY_KEY_CHAR) {
            10 | 13 | 89 | 121 => if get_screen_param(CONFIRM_YES_NO) != 0 { 2 } else { 3 },
            27 | 78 | 110 => 1,
            _ => 0,
        };
    }
    if !(1..=5).contains(&answer) {
        return o(widget);
    }
    let command = get_screen_param(CONFIRM_COMMAND);
    let r = o(widget);
    let ok = r == -9 && read_usize(G_PENDING_COMMAND_CODE) == command && read_u32(G_PENDING_COMMAND_ARG) == answer as u32
        && get_screen_param(CONFIRM_TEXT) == 0;
    SCOREBOARD.record(ok, || format!("confirmation box (answer {}): answer {}, command {:#x} x {:#x}, argument {}", answer, r,
        read_usize(G_PENDING_COMMAND_CODE), command, read_u32(G_PENDING_COMMAND_ARG)));
    r
}

/// The screen nodes of the current channel whose build routine is `build` (to tell a new screen from one already
/// in the history).
pub(crate) unsafe fn screen_nodes_built_by(build: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut node = read_usize(screen_record(G_SCREEN_MANAGER) + screen_history::FIRST_NODE);
    while node != 0 {
        if read_usize(node + screen_node_fields::BUILD) == build {
            v.push(node);
        }
        node = read_usize(node + screen_node_fields::NEXT);
    }
    v
}

/// Whether the call just made opened a new screen with the build routine `build` (the current node is one, and it
/// was not in the history before).
pub(crate) unsafe fn screen_newly_opened(before: &[usize], build: usize) -> bool {
    let node = current_screen_node(G_SCREEN_MANAGER);
    node != 0 && read_usize(node + screen_node_fields::BUILD) == build && !before.contains(&node)
}

/// The file_screens.cpp "Error" box at a line.
pub(crate) unsafe fn file_screens_error_box(line: u32) {
    tel26_error_box(FILE_SCREENS_CPP, line);
}

/// The C string at `p` (empty for 0), for the comparers.
pub(crate) unsafe fn file_c_string(p: usize) -> Vec<u8> {
    if p == 0 {
        return Vec::new();
    }
    core::slice::from_raw_parts(p as *const u8, c_string_len(p)).to_vec()
}

/// `_mkdir` (0x945bd2, cdecl): 0 when the directory was made.
pub(crate) unsafe fn file_make_directory(path: usize) -> i32 {
    exe::at_945bd2(path)
}

/// `utils_is_directory` (0x90bf90, cdecl): 1 when the path names an existing directory.
pub(crate) unsafe fn file_is_directory(path: usize) -> i32 {
    exe::utils_is_directory(path)
}

/// A confirmation box of the file box: the message (`langlib_sprintf` of the format with up to one string argument)
/// into [`JOB_TEXT`], the heading (the heading's format, the message as an unused argument) into a local, and
/// `confirm_push(title, heading, message, yes/no, colour 1, colour 2, 0)`.
pub(crate) unsafe fn file_dialog_confirm(title: usize, heading: usize, format: usize, argument: usize, yes_no: u32, colour_1: usize, colour_2: usize) {
    let message = comp_lang_format()(JOB_TEXT, format, argument) as usize;
    let mut buffer = [0u8; 0xc8];
    let head = comp_lang_format()(buffer.as_mut_ptr() as usize, heading, message) as usize;
    confirm_push(title, head, message, yes_no, colour_1 as u32, colour_2 as u32, 0);
}

/// The path of the file box's file into the result buffer: `_makepath(result, 0, directory, name, extension)`, the
/// extension only when given and not already the name's (`_splitpath` and `_stricmp`).
pub(crate) unsafe fn file_dialog_result_path(result: usize, directory: usize, name: usize, extension: usize) {
    let mut ext = [0u8; 0x100];
    exe::crt_strtok(name, 0usize, 0usize, core::ptr::null_mut(), ext.as_mut_ptr());
    let extension = if extension != 0 && exe::crt_stricmp(ext.as_ptr() as usize, extension) != 0 { extension } else { 0 };
    exe::crt_makepath(result as *mut u8, 0usize, directory, name, extension);
}

/// Leaving the file box with an answer: the accepted directory and the file name freed, the pending command = parameter
/// 0xc and its argument the result path (`G_PENDING_COMMAND_CODE`, `G_PENDING_COMMAND_ARG`); −9.
pub(crate) unsafe fn file_dialog_leave(result: usize) -> i32 {
    exe::crt_operator_delete(get_screen_param(FILE_DIALOG_ACCEPTED_DIRECTORY));
    exe::crt_operator_delete(get_screen_param(FILE_DIALOG_FILE_NAME));
    write_usize(G_PENDING_COMMAND_CODE, get_screen_param(FILE_DIALOG_COMMAND));
    write_usize(G_PENDING_COMMAND_ARG, result);
    -9
}

/// The file box's typed directory (id 1; a part of `file_dialog_input`, see there).
pub(crate) unsafe fn file_dialog_directory_typed() -> i32 {
    let title = get_screen_param(FILE_DIALOG_TITLE);
    let mode = get_screen_param(FILE_DIALOG_MODE);
    let directory = get_screen_param(FILE_DIALOG_DIRECTORY);
    let accepted = get_screen_param(FILE_DIALOG_ACCEPTED_DIRECTORY);
    let colour_1 = get_screen_param(FILE_DIALOG_COLOUR_1);
    let colour_2 = get_screen_param(FILE_DIALOG_COLOUR_2);
    if c_string_len(directory) != 0 && file_is_directory(directory) == 0 {
        if mode == 1 {
            covered!("tel27_2 create the directory?");
            set_screen_param(FILE_DIALOG_QUESTION, 1);
            file_dialog_confirm(title, 0x9c7c34, 0x9c7c48, directory, 1, colour_1, colour_2);
            return -4;
        }
        file_dialog_confirm(title, 0x9c7c34, 0x9c7bfc, directory, 0, colour_1, colour_2);
        exe::at_9466d0(directory, accepted as *const u8, 0x104);
        write_u8(directory + 0x103, 0);
        return -4;
    }
    exe::at_9466d0(accepted, directory as *const u8, 0x104);
    write_u8(accepted + 0x103, 0);
    -11
}

/// The file box's Info button (id 5; a part of `file_dialog_input`, see there).
pub(crate) unsafe fn file_dialog_info() -> i32 {
    let title = get_screen_param(FILE_DIALOG_TITLE);
    get_screen_param(FILE_DIALOG_MODE);
    let directory = get_screen_param(FILE_DIALOG_DIRECTORY);
    let extension = get_screen_param(FILE_DIALOG_EXTENSION);
    get_screen_param(FILE_DIALOG_ACCEPTED_DIRECTORY);
    let file_name = get_screen_param(FILE_DIALOG_FILE_NAME);
    let result = get_screen_param(FILE_DIALOG_RESULT);
    let colour_1 = get_screen_param(FILE_DIALOG_COLOUR_1);
    let colour_2 = get_screen_param(FILE_DIALOG_COLOUR_2);
    get_screen_param(FILE_DIALOG_DIRECTORIES);
    if c_string_len(file_name) == 0 {
        file_dialog_confirm(title, 0x9c7bcc, 0x9c7bdc, 0, 0, colour_1, colour_2);
        return -4;
    }
    file_dialog_result_path(result, directory, file_name, extension);
    if exe::utils_file_exists(result) == 0 {
        let mut path = [0u8; 0x104];
        tel26_copy_text(path.as_mut_ptr(), result as *const u8);
        file_dialog_confirm(title, 0x9c7b90, 0x9c7ba4, path.as_ptr() as usize, 0, colour_1, colour_2);
        return -4;
    }
    let mut z = exe::zopen(result, 0x9c7b84usize, 0, 0x15, 4);
    if z != 0 {
        let mut version: i32 = 0;
        let n = exe::zread(&mut version as *mut i32 as usize, 4, 1, z);
        if n == 1 {
            exe::zclose(&mut z);
            if version.wrapping_abs() >= SAVE_OLDEST_LOADABLE_VERSION {
                covered!("tel27_2 save game info opened");
                if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x590c00usize, 0x5917f0usize, 1, 0usize, 0) != 0 {
                    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0, result, 1);
                }
                return -4;
            }
        }
        exe::zclose(&mut z);
    }
    file_dialog_confirm(title, 0x9c7b40, 0x9c7b58, 0, 0, colour_1, colour_2);
    -4
}

/// The file box's Ok (id 4, Enter; a part of `file_dialog_input`, see there).
pub(crate) unsafe fn file_dialog_ok(widget: u32) -> i32 {
    let title = get_screen_param(FILE_DIALOG_TITLE);
    let mode = get_screen_param(FILE_DIALOG_MODE);
    let directory = get_screen_param(FILE_DIALOG_DIRECTORY);
    let extension = get_screen_param(FILE_DIALOG_EXTENSION);
    get_screen_param(FILE_DIALOG_ACCEPTED_DIRECTORY);
    let file_name = get_screen_param(FILE_DIALOG_FILE_NAME);
    let result = get_screen_param(FILE_DIALOG_RESULT);
    let colour_1 = get_screen_param(FILE_DIALOG_COLOUR_1);
    let colour_2 = get_screen_param(FILE_DIALOG_COLOUR_2);
    let directories = get_screen_param(FILE_DIALOG_DIRECTORIES);
    if c_string_len(file_name) == 0 {
        if directories != 0 {
            file_dialog_confirm(title, 0x9c7b04, 0x9c7b18, 0, 0, colour_1, colour_2);
        } else {
            file_dialog_confirm(title, 0x9c7bcc, 0x9c7bdc, 0, 0, colour_1, colour_2);
        }
        return -4;
    }
    if mode > 2 {
        let r = exe::menubar_handle_command(widget);
        if r != 0 {
            return r;
        }
        return exe::menubar_handle_object_link(widget);
    }
    file_dialog_result_path(result, directory, file_name, extension);
    let missing = |result: usize| {
        let mut path = [0u8; 0x104];
        tel26_copy_text(path.as_mut_ptr(), result as *const u8);
        file_dialog_confirm(title, 0x9c7b90, 0x9c7ba4, path.as_ptr() as usize, 0, colour_1, colour_2);
        -4
    };
    match mode {
        0 => {
            if directories == 0 && exe::utils_file_exists(result) == 0 {
                return missing(result);
            }
            file_dialog_leave(result)
        }
        1 => {
            if directories != 0 || exe::utils_file_exists(result) == 0 {
                return file_dialog_leave(result);
            }
            covered!("tel27_2 replace the file?");
            set_screen_param(FILE_DIALOG_QUESTION, 3);
            let mut path = [0u8; 0x104];
            tel26_copy_text(path.as_mut_ptr(), result as *const u8);
            file_dialog_confirm(title, 0x9c7abc, 0x9c7ac8, path.as_ptr() as usize, 1, colour_1, colour_2);
            -4
        }
        _ => {
            if directories != 0 {
                return file_dialog_leave(result);
            }
            if exe::utils_file_exists(result) == 0 {
                return missing(result);
            }
            covered!("tel27_2 delete the file?");
            set_screen_param(FILE_DIALOG_QUESTION, 2);
            let mut path = [0u8; 0x104];
            tel26_copy_text(path.as_mut_ptr(), result as *const u8);
            file_dialog_confirm(title, 0x9909b8, 0x9c7a9c, path.as_ptr() as usize, 1, colour_1, colour_2);
            -4
        }
    }
}

/// The "Error" box of a save game load (file_screens.cpp at `line`, "v%s %s.%s %d (%s)" with the message), as the
/// "Load Game Error" box: "The save game is corrupted and cannot be loaded. Debug Info: ...".
pub(crate) unsafe fn file_load_error_box(line: u32, message: usize) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(FILE_SCREENS_CPP, 0usize, 0usize, name.as_mut_ptr(), ext.as_mut_ptr());
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), line, message);
    comp_crt_format()(0xae24d0, 0x9a8120, v, name.as_ptr(), ext.as_ptr(), line, message);
    comp_lang_format()(0xae28bc, 0x9af3e0);
    comp_lang_format()(0xae26c4, 0x9af334, 0xae24d0);
    exe::gui_show_message_box(0xae28bc, 0xae26c4, 0);
}

/// The save game info screen's row comment for the play time in days (English only; `days` ≥ 1).
pub(crate) fn file_addictedness(days: i32, hours: i32) -> usize {
    match days {
        d if d > 10 => 0x9c7e70,
        9 | 10 => 0x9c7e2c,
        8 => 0x9c7e08,
        7 => 0x9c7de4,
        6 => 0x9c7dcc,
        5 => 0x9c7dbc,
        4 => 0x9c7da4,
        3 => 0x9c7d88,
        2 => 0x9c7d74,
        _ => if hours > 12 { 0x9c7d50 } else { 0x9c7d40 },
    }
}

/// "Last Saved" of the save game info screen into [`JOB_TEXT`]: `localtime` of the time of the save (at `time`), the
/// weekday from Monday (`date_get_weekday_name`), the month name and the year (+1900).
pub(crate) unsafe fn file_last_saved(time: usize) {
    let tm = exe::at_946c40(time);
    let mut weekday = (read_u8(tm + 0x18) as i8).wrapping_sub(1);
    if weekday < 0 {
        weekday = weekday.wrapping_add(7);
    }
    let year = read_i32(tm + 0x14) + 0x76c;
    let month = exe::date_get_month_name(read_u8(tm + 0x10) as u32);
    let day = exe::date_get_weekday_name(weekday as i16 as u16 as u32);
    comp_lang_format()(JOB_TEXT, 0x9c7f68, 0x64, 0x64, read_i32(tm + 8), read_i32(tm + 4), day, read_i32(tm + 0xc), month, year);
}

/// The plan check of a confirmation box opener: the strings by content, the plain parameters by value (`text` the
/// expected Text parameter: 0, or `None` for any non-null one).
pub(crate) unsafe fn compare_confirm_opened(name: &str, strings: &[Vec<u8>], yes_no: u32, colour_1: u32, colour_2: u32, text: Option<usize>, command: u32) {
    let got_strings: Vec<Vec<u8>> = (0..strings.len() as u32).map(|k| file_c_string(get_screen_param(k))).collect();
    let plain = get_screen_param(CONFIRM_YES_NO) == yes_no as usize && get_screen_param(CONFIRM_COLOUR_1) == colour_1 as i16 as isize as usize
        && get_screen_param(CONFIRM_COLOUR_2) == colour_2 as i16 as isize as usize && get_screen_param(CONFIRM_COMMAND) == command as usize;
    let text_ok = match text {
        Some(v) => get_screen_param(CONFIRM_TEXT) == v,
        None => get_screen_param(CONFIRM_TEXT) != 0,
    };
    SCOREBOARD.record(plain && text_ok && got_strings == strings, || {
        let got: Vec<usize> = (0..8).map(|k| get_screen_param(k)).collect();
        format!("{}: params {:x?}", name, got)
    });
}

/// "E:\dev\CM3\cm3 00-01\cm3\code\file_screens.cpp", the path the file_screens.cpp error boxes print.
pub(crate) const FILE_SCREENS_CPP: usize = 0x9c7a48;

/// The file box's screen parameters (`open_file_dialog` writes them): the title and the subtitle (copied strings), the
/// mode (0 load, 1 save, 2 delete [inference: from the questions each asks]), the directory buffer (edited in place),
/// the extension (copied, or 0), the filter callback (cdecl(path) → flags: bit 0 listed, bit 2 grey, bit 3
/// highlighted), the directory as last accepted (0x104), the file name (0x100), the result path (0x104), the question
/// pending (1 create the directory, 2 delete, 3 replace), the title colours, the command code on leaving, whether only
/// directories are listed, the list's scroll row and area, whether the directory is fixed, whether Cancel is off and
/// whether there is an Info button.
pub(crate) const FILE_DIALOG_TITLE: u32 = 0;

pub(crate) const FILE_DIALOG_SUBTITLE: u32 = 1;

pub(crate) const FILE_DIALOG_MODE: u32 = 2;

pub(crate) const FILE_DIALOG_DIRECTORY: u32 = 3;

pub(crate) const FILE_DIALOG_EXTENSION: u32 = 4;

pub(crate) const FILE_DIALOG_FILTER: u32 = 5;

pub(crate) const FILE_DIALOG_ACCEPTED_DIRECTORY: u32 = 6;

pub(crate) const FILE_DIALOG_FILE_NAME: u32 = 7;

pub(crate) const FILE_DIALOG_RESULT: u32 = 8;

pub(crate) const FILE_DIALOG_QUESTION: u32 = 9;

pub(crate) const FILE_DIALOG_COLOUR_1: u32 = 0xa;

pub(crate) const FILE_DIALOG_COLOUR_2: u32 = 0xb;

pub(crate) const FILE_DIALOG_COMMAND: u32 = 0xc;

pub(crate) const FILE_DIALOG_DIRECTORIES: u32 = 0xd;

pub(crate) const FILE_DIALOG_SCROLL_ROW: u32 = 0xe;

pub(crate) const FILE_DIALOG_LIST_AREA: u32 = 0xf;

pub(crate) const FILE_DIALOG_FIXED_DIRECTORY: u32 = 0x10;

pub(crate) const FILE_DIALOG_NO_CANCEL: u32 = 0x11;

pub(crate) const FILE_DIALOG_INFO: u32 = 0x12;

/// The screen manager's flag raised when the screen is rebuilt after a box it opened was answered (+0x3834)
/// [inference: the file box reads the answer only then].
pub(crate) const SCREEN_MANAGER_BOX_ANSWERED: usize = 0xb77c84;

/// The save game version from which general.dat's summary is 0xf74 bytes (with the time of the save), and the oldest
/// a load accepts.
pub(crate) const SAVE_SUMMARY_LONG_VERSION: i32 = 0x3e027;

pub(crate) const SAVE_OLDEST_LOADABLE_VERSION: i32 = 0x3e023;

/// The file list's find data (`file_find_first`): +8 the attributes, +0x10 the name.
pub(crate) const FILE_FIND_ATTRIBUTES: usize = 8;

pub(crate) const FILE_FIND_NAME: usize = 0x10;

/// The parameters a confirmation box keeps (`confirm_push` and `confirm_push_text_object` write them): the title, the
/// heading and the message (copied strings; the message 0 with a Text), yes/no (0: "Ok" only), the title colours
/// (sign-extended words), the Text (owned by the box) and the command on answering.
pub(crate) const CONFIRM_TITLE: u32 = 0;

pub(crate) const CONFIRM_HEADING: u32 = 1;

pub(crate) const CONFIRM_MESSAGE: u32 = 2;

pub(crate) const CONFIRM_YES_NO: u32 = 3;

pub(crate) const CONFIRM_COLOUR_1: u32 = 4;

pub(crate) const CONFIRM_COLOUR_2: u32 = 5;

pub(crate) const CONFIRM_TEXT: u32 = 6;

pub(crate) const CONFIRM_COMMAND: u32 = 7;
