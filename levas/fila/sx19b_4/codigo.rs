// ---------- sx19b_4: the scout manager in the saved game: the constructor, loading and saving (scout_manager.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\scout_manager.cpp" (the error box) and "scout_manager.dat" (its save archive item).
const SCOUT_MANAGER_CPP: usize = 0xa7b094;
const SCOUT_MANAGER_DAT: usize = 0xa7b0c4;
/// `PLAYER_SEARCH::PLAYER_SEARCH` (0x7ae390) and `PLAYER_SEARCH::~PLAYER_SEARCH` (0x7ae4f0), the vector iterators'
/// element constructor and destructor (thiscall).
const FN_PLAYER_SEARCH_CONSTRUCT: usize = 0x7ae390;
const FN_PLAYER_SEARCH_DESTRUCT: usize = 0x7ae4f0;
/// `PLAYER_SEARCH::load` (0x7b17e0) and `PLAYER_SEARCH::save` (0x7b2360): thiscall(search, archive item), ret 4.
const FN_PLAYER_SEARCH_LOAD: usize = 0x7b17e0;
const FN_PLAYER_SEARCH_SAVE: usize = 0x7b2360;
/// The 1080 exe's no-CD patch in this constructor: the drive type's `jne` and the found test.
const SCOUT_MANAGER_CD_DRIVE_TYPE_JUMP: usize = 0x7f67d1;
const SCOUT_MANAGER_CD_FOUND_TEST: usize = 0x7f6815;
/// Ten player searches for each of the sixteen human managers (0x2b0 bytes each).
const SCOUT_MANAGER_SEARCHES: u32 = 0xa0;

/// A new array of `count` player searches (`operator new` with the count in the dword before them, each built by
/// `PLAYER_SEARCH::PLAYER_SEARCH`); out of memory: the scout_manager.cpp "Error" box at `line`, the fatal cleanup and
/// `exit(-1)`.
unsafe fn scout_manager_new_searches(count: u32, line: u32) -> usize {
    let block = exe::crt_operator_new(count.wrapping_mul(0x2b0).wrapping_add(4));
    if block != 0 {
        write_u32(block, count);
        exe::crt_vector_constructor_iterator(block + 4, 0x2b0, count, FN_PLAYER_SEARCH_CONSTRUCT, FN_PLAYER_SEARCH_DESTRUCT);
        return block + 4;
    }
    show_error_at(SCOUT_MANAGER_CPP, line);
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exe::crt_exit(-1);
    0
}

/// Builds the scout manager: no compare players, the game CD required, no edit sessions, then the saved state or 160
/// fresh player searches.
///
/// `SCOUT_MANAGER::SCOUT_MANAGER(save)`, from creating the game (0x828a9a, no save) and from loading it (0x82bb1a): the
/// compare players 1 and 2 of the 16 human managers (+0x68 and +0xa8, 16 dwords each) zeroed; the game CD required
/// (`require_game_cd`, honouring the 1080 no-CD patch at 0x7f67d1/0x7f6815); the 16 edit sessions
/// (`scout_manager_fields::EDIT_SESSIONS`) zeroed. With a save of data version 0x3e025 or later (`G_GAME_DATA_VERSION`,
/// absolute value) whose `SCOUT_MANAGER::load` answers, nothing more. Otherwise 160 searches (+8 = 0xa0), no scout
/// requests (`SCOUT_REQUESTS`, capacity +0x60 and `SCOUT_REQUEST_COUNT` = 0), the array at +0 (out of memory: the box
/// at line 0x65 and `exit(-1)`), and for human manager m (the person `G_STAFF_COUNT` − 16 + m) his 10 lists k: owner m
/// (`set_manager`), slot k (`set_slot_index`), type 1, 2, 3 and then 4 (short list, player search, staff search and
/// scouts; `set_list_type`) and the translated list name at +0xb (`scout_manager_write_list_name`, 100 bytes); his
/// selected list (`SELECTED_LISTS` + m) = 1. Returns this.
/// Original: `SCOUT_MANAGER::SCOUT_MANAGER` (0x7f6760, scout_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_scout_manager(this: usize, save: usize) -> usize {
    for m in 0..16 {
        write_u32(this + 0x68 + m * 4, 0); // compare player 1 of human manager m
        write_u32(this + 0xa8 + m * 4, 0); // compare player 2
    }
    require_game_cd(SCOUT_MANAGER_CD_DRIVE_TYPE_JUMP as *const u8, SCOUT_MANAGER_CD_FOUND_TEST as *const u8);
    for m in 0..16 {
        write_u32(this + scout_manager_fields::EDIT_SESSIONS + m * 4, 0);
    }
    if save != 0 && read_i32(G_GAME_DATA_VERSION).wrapping_abs() >= 0x3e025 && scout_manager_load(this, save) != 0 {
        return this;
    }
    covered!("sx19b_4 searches built");
    write_u32(this + 8, SCOUT_MANAGER_SEARCHES);
    write_u32(this + scout_manager_fields::SCOUT_REQUEST_COUNT, 0);
    write_u32(this + 0x60, 0); // the scout request capacity
    write_u32(this + scout_manager_fields::SCOUT_REQUESTS, 0);
    let searches = scout_manager_new_searches(SCOUT_MANAGER_SEARCHES, 0x65);
    write_usize(this, searches);
    for m in 0..16usize {
        let manager = staff_at((m as i32 + read_i32(G_STAFF_COUNT) - 16) as isize).addr();
        for k in 0..10u8 {
            let s = searches + (m * 10 + k as usize) * 0x2b0;
            set_manager(s, manager);
            set_slot_index(s, k as i8);
            set_list_type(s, match k { 0 => 1, 1 => 2, 2 => 3, _ => 4 }); // sx19b_4: the list types
            scout_manager_write_list_name(k as u32, s + 0xb, 100); // the list name (+0xb)
        }
        write_u8(this + scout_manager_fields::SELECTED_LISTS + m, 1);
    }
    this
}

/// Comparer for `new_scout_manager`: the CD check, the allocations and the searches' load run once, so only the
/// original runs; checked afterwards on a new game: 160 searches, no requests, each human's selected list 1.
pub(crate) unsafe extern "thiscall" fn compare_new_scout_manager(this: usize, save: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, save);
    let ok = save != 0 || (r == this && read_usize(this) != 0 && read_u32(this + 8) == SCOUT_MANAGER_SEARCHES
        && read_u32(this + scout_manager_fields::SCOUT_REQUEST_COUNT) == 0
        && (0..16).all(|m| read_u8(this + scout_manager_fields::SELECTED_LISTS + m) == 1));
    SCOREBOARD.record(ok, || format!("new_scout_manager({:#x}): searches {} at {:#x}", this, read_u32(this + 8), read_usize(this)));
    r
}

/// Reads scout_manager.dat from a save archive into the search array, the scout request array and the selected lists.
///
/// `SCOUT_MANAGER::load(archive)`, from the constructor with a save: the item "scout_manager.dat" (`exe::zopen`,
/// reading); the search count (+8): none leaves +0 null, otherwise a new array (out of memory: the box at line 0x37b
/// and `exit(-1)`) and each search read (`PLAYER_SEARCH::load`; failing: the box at line 0x383, the item closed and 0);
/// the request count (`SCOUT_REQUEST_COUNT`, also the capacity +0x60) and, with any, a new array of 17-byte requests
/// (`exe::crt_malloc`) read whole; the 16 selected list bytes (`SELECTED_LISTS`). The item is closed and 1 returned.
/// No archive (line 0x361) or no item (0x369): the box and 0. A short read: the box with the field's line (0x370,
/// 0x3b3: the item closed; 0x38f, 0x39f, 0x3a7: the item left open, a defect of the original) and 0.
/// Original: `SCOUT_MANAGER::load` (0x7f7bb0, scout_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn scout_manager_load(this: usize, archive: usize) -> u32 {
    if archive == 0 {
        show_error_at(SCOUT_MANAGER_CPP, 0x361);
        return 0;
    }
    let mut item = exe::zopen(archive, SCOUT_MANAGER_DAT, 0, 0x16, 4);
    if item == 0 {
        show_error_at(SCOUT_MANAGER_CPP, 0x369);
        return 0;
    }
    let fail_and_close = |item: &mut usize, line: u32| {
        show_error_at(SCOUT_MANAGER_CPP, line);
        exe::zclose(item);
        0
    };
    if exe::zread(this + 8, 4, 1, item) != 1 {
        return fail_and_close(&mut item, 0x370);
    }
    let count = read_i32(this + 8);
    if count < 1 {
        write_usize(this, 0);
    } else {
        let searches = scout_manager_new_searches(count as u32, 0x37b);
        write_usize(this, searches);
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_PLAYER_SEARCH_LOAD);
        let mut k = 0i32;
        while k < read_i32(this + 8) {
            if load(searches + k as usize * 0x2b0, item) == 0 {
                return fail_and_close(&mut item, 0x383);
            }
            k += 1;
        }
    }
    if exe::zread(this + scout_manager_fields::SCOUT_REQUEST_COUNT, 4, 1, item) != 1 {
        show_error_at(SCOUT_MANAGER_CPP, 0x38f);
        return 0;
    }
    let requests = read_u32(this + scout_manager_fields::SCOUT_REQUEST_COUNT);
    write_u32(this + 0x60, requests); // the scout request capacity
    if requests == 0 {
        write_usize(this + scout_manager_fields::SCOUT_REQUESTS, 0);
    } else {
        covered!("sx19b_4 scout requests loaded");
        let array = exe::crt_malloc(requests.wrapping_mul(0x11));
        write_usize(this + scout_manager_fields::SCOUT_REQUESTS, array);
        if array == 0 {
            show_error_at(SCOUT_MANAGER_CPP, 0x39f);
            return 0;
        }
        if exe::zread(array, 0x11, requests, item) != read_u32(this + scout_manager_fields::SCOUT_REQUEST_COUNT) {
            show_error_at(SCOUT_MANAGER_CPP, 0x3a7);
            return 0;
        }
    }
    if exe::zread(this + scout_manager_fields::SELECTED_LISTS, 1, 0x10, item) != 0x10 {
        return fail_and_close(&mut item, 0x3b3);
    }
    exe::zclose(&mut item);
    1
}

/// Comparer for `scout_manager_load`: it reads the save, so only the original runs; checked afterwards: the capacity
/// equals the request count.
pub(crate) unsafe extern "thiscall" fn compare_scout_manager_load(this: usize, archive: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, archive);
    let (n, c) = (read_u32(this + scout_manager_fields::SCOUT_REQUEST_COUNT), read_u32(this + 0x60));
    SCOREBOARD.record(r == 0 || n == c, || format!("scout_manager_load({:#x}): {} requests, capacity {}", this, n, c));
    r
}

/// Writes the search array, the scout request array and the selected lists to scout_manager.dat in a save archive.
///
/// `SCOUT_MANAGER::save(archive)`, from saving the game (0x82ea57): the item "scout_manager.dat" (`exe::zopen`,
/// writing); the search count (+8) and each search (`PLAYER_SEARCH::save`); the request count
/// (`SCOUT_REQUEST_COUNT`) and the requests (17 bytes each; neither write is checked); the 16 selected list bytes
/// (`SELECTED_LISTS`). The item is closed and 1 returned. No archive (line 0x3ce) or no item (0x3d6): the box and 0;
/// a short write of the count (0x3dd), a search that fails (0x3eb) or a short write of the lists (0x3f9): the box, the
/// item closed and 0.
/// Original: `SCOUT_MANAGER::save` (0x7f8020, scout_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn scout_manager_save(this: usize, archive: usize) -> u32 {
    if archive == 0 {
        show_error_at(SCOUT_MANAGER_CPP, 0x3ce);
        return 0;
    }
    let mut item = exe::zopen(archive, SCOUT_MANAGER_DAT, 1, 0x16, 4);
    if item == 0 {
        show_error_at(SCOUT_MANAGER_CPP, 0x3d6);
        return 0;
    }
    let fail_and_close = |item: &mut usize, line: u32| {
        show_error_at(SCOUT_MANAGER_CPP, line);
        exe::zclose(item);
        0
    };
    if exe::zwrite(this + 8, 4, 1, item) != 1 {
        return fail_and_close(&mut item, 0x3dd);
    }
    let save: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_PLAYER_SEARCH_SAVE);
    let mut k = 0i32;
    while k < read_i32(this + 8) {
        if save(read_usize(this) + k as usize * 0x2b0, item) == 0 {
            return fail_and_close(&mut item, 0x3eb);
        }
        k += 1;
    }
    exe::zwrite(this + scout_manager_fields::SCOUT_REQUEST_COUNT, 4, 1, item);
    exe::zwrite(read_usize(this + scout_manager_fields::SCOUT_REQUESTS), 0x11, read_u32(this + scout_manager_fields::SCOUT_REQUEST_COUNT), item);
    if exe::zwrite(this + scout_manager_fields::SELECTED_LISTS, 1, 0x10, item) != 0x10 { // sx19b_4: the selected lists written
        return fail_and_close(&mut item, 0x3f9);
    }
    exe::zclose(&mut item);
    1
}

/// Comparer for `scout_manager_save`: it writes the save, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_scout_manager_save(this: usize, archive: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, archive)
}
