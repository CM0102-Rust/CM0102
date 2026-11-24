// ---------- batch %%LEVA%%: the human manager's squad numbers (human_manager.cpp; sx12_8) ----------

/// The human manager's squad number table for the team on view: 50 player ids (-1 a free number) at +0x1e05 of the
/// active tactic slot (`human_manager_record::ACTIVE_TACTIC_SLOT`, signed), the reserve slot 1 sharing the first
/// team's (slot 0).
unsafe fn squad_number_table(record: usize) -> usize {
    let mut slot = read_i8(record + human_manager_record::ACTIVE_TACTIC_SLOT);
    if slot == 1 {
        slot = 0; // the reserves use the first team's numbers
    }
    record.wrapping_add((slot as isize * 0x1ecd) as usize) + 0x1e05
}

/// The squad number table of a human manager (null: the current human), or `None` after the "Error" box
/// (`human_record_or_error`, sx12_4).
unsafe fn squad_numbers_or_error(this: usize, manager: usize, missing: u32, out_of_range: u32) -> Option<usize> {
    human_record_or_error(this, manager, missing, out_of_range).map(|r| unsafe { squad_number_table(r) })
}

/// Whether a squad number is one of the 50 (0..49, signed).
fn valid_squad_number(number: i8) -> bool {
    (0..0x32).contains(&number)
}

/// Clears the current human's squad numbers held by players who no longer belong to the team on view.
///
/// `HUMAN_MANAGER::prune_squad_numbers()`: the current human (none: the "Error" box, line 0xb90, and nothing); his
/// index is not checked. For each of the 50 numbers held, by the active tactic slot (read as
/// stored, 1 not folded into 0): slots 0 and 1 (the club) keep a player whose `.club_job()` is the human's; slot 2 (the
/// national team) one whose `.national_job()` is the human's and who is in its squad (`playing_squad::NATIONAL_SQUAD`);
/// slot 3 the same with `playing_squad::NATIONAL_B_SQUAD`; slot 4 (the under-21 side) one whose `.national_job()` is
/// the under-21 team of the nation of the human's national team (`exe::database_get_national_u21_team`) and in the
/// national squad; any other slot keeps everyone. The others' numbers become free (-1). Defect of the original: with no
/// national job the under-21 team is a stack dword never set (only read in slot 4, where the human has one).
/// Original: `HUMAN_MANAGER::prune_squad_numbers` (0x600ee0, human_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn prune_squad_numbers(this: usize) {
    let human = Staff(manager_or_current_human(0));
    if human.is_null() {
        show_error_at(0x9cf11c, 0xb90); // "...\human_manager.cpp"
        return;
    }
    let k = read_i32(human.addr()).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10) as usize;
    let record = human_manager_record_at(this, k);
    let table = squad_number_table(record);
    let under_21 = if human.national_job().is_null() {
        0 // the original leaves this dword unset
    } else {
        exe::database_get_national_u21_team(read_usize(human.national_job().addr() + club::NATION))
    };
    for n in 0..0x32usize {
        let id = read_i32(table + 4 * n);
        if id == -1 {
            continue;
        }
        let player = staff_at(id as isize);
        let keep = match read_i8(record + human_manager_record::ACTIVE_TACTIC_SLOT) {
            0 | 1 => player.club_job() == human.club_job(),
            2 => player.national_job() == human.national_job() && player.playing_squad() & playing_squad::NATIONAL_SQUAD != 0,
            3 => player.national_job() == human.national_job() && player.playing_squad() & playing_squad::NATIONAL_B_SQUAD != 0,
            4 => player.national_job().addr() == under_21 && player.playing_squad() & playing_squad::NATIONAL_SQUAD != 0,
            _ => true,
        };
        if !keep {
            covered!("sx12_8 prune squad number");
            write_i32(table + 4 * n, -1); // sx12_8: the number becomes free
        }
    }
}

/// Returns the player holding a squad number for the team on view, or 0 when it is free.
///
/// `HUMAN_MANAGER::get_squad_number_player(number, manager)`: no manager: the "Error" box (line 0xbf6); out of range:
/// 0xc02; a number outside 0..49: 0xc09; all give 0. Otherwise the person (`staff_at`) of the id in the table
/// (`squad_number_table`), or 0 for -1.
/// Original: `HUMAN_MANAGER::get_squad_number_player` (0x601070, human_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_squad_number_player(this: usize, number: i8, manager: usize) -> usize {
    let Some(table) = squad_numbers_or_error(this, manager, 0xbf6, 0xc02) else {
        return 0;
    };
    if !valid_squad_number(number) {
        show_error_at(0x9cf11c, 0xc09);
        return 0;
    }
    let id = read_i32(table + 4 * number as usize);
    if id == -1 {
        return 0;
    }
    covered!("sx12_8 squad number player");
    staff_at(id as isize).addr() // sx12_8: the holder
}

/// Gives a squad number to a player for the team on view.
///
/// `HUMAN_MANAGER::set_squad_number_player(number, player, manager)`: no manager: the "Error" box (line 0xc36); out of
/// range: 0xc42; a number outside 0..49: 0xc49; a null player: 0xc50. Otherwise the player's id goes into the table
/// (`squad_number_table`); a number he held before stays his.
/// Original: `HUMAN_MANAGER::set_squad_number_player` (0x601200, human_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn set_squad_number_player(this: usize, number: i8, player: usize, manager: usize) {
    let Some(table) = squad_numbers_or_error(this, manager, 0xc36, 0xc42) else {
        return;
    };
    if !valid_squad_number(number) {
        return show_error_at(0x9cf11c, 0xc49);
    }
    if player == 0 {
        return show_error_at(0x9cf11c, 0xc50);
    }
    covered!("sx12_8 set squad number");
    write_i32(table + 4 * number as usize, read_i32(player)); // sx12_8: the player's id
}

/// Frees the squad number a player holds for the team on view (the first one found).
///
/// `HUMAN_MANAGER::clear_player_squad_number(player, manager)`: no manager: the "Error" box (line 0xc75); out of range:
/// 0xc81; a null player: 0xc88. Otherwise the first of the 50 numbers holding his id becomes -1.
/// Original: `HUMAN_MANAGER::clear_player_squad_number` (0x6013c0, human_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn clear_player_squad_number(this: usize, player: usize, manager: usize) {
    let Some(table) = squad_numbers_or_error(this, manager, 0xc75, 0xc81) else {
        return;
    };
    if player == 0 {
        return show_error_at(0x9cf11c, 0xc88);
    }
    let id = read_i32(player);
    if let Some(n) = (0..0x32usize).find(|&n| read_i32(table + 4 * n) == id) {
        covered!("sx12_8 clear player squad number");
        write_i32(table + 4 * n, -1);
    }
}

/// Frees one squad number for the team on view.
///
/// `HUMAN_MANAGER::clear_squad_number(number, manager)`: no manager: the "Error" box (line 0xcb5); out of range: 0xcc1;
/// a number outside 0..49: 0xcc8. Otherwise the number becomes -1.
/// Original: `HUMAN_MANAGER::clear_squad_number` (0x601560, human_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn clear_squad_number(this: usize, number: i8, manager: usize) {
    let Some(table) = squad_numbers_or_error(this, manager, 0xcb5, 0xcc1) else {
        return;
    };
    if !valid_squad_number(number) {
        return show_error_at(0x9cf11c, 0xcc8);
    }
    write_i32(table + 4 * number as usize, -1);
}

/// Swaps the players of two squad numbers for the team on view.
///
/// `HUMAN_MANAGER::swap_squad_numbers(a, b, manager)`: no manager: the "Error" box (line 0xcef); out of range: 0xcfb;
/// either number outside 0..49: 0xd03. Otherwise the two ids change places.
/// Original: `HUMAN_MANAGER::swap_squad_numbers` (0x6016e0, human_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn swap_squad_numbers(this: usize, a: i8, b: i8, manager: usize) {
    let Some(table) = squad_numbers_or_error(this, manager, 0xcef, 0xcfb) else {
        return;
    };
    if !valid_squad_number(a) || !valid_squad_number(b) {
        return show_error_at(0x9cf11c, 0xd03);
    }
    covered!("sx12_8 swap squad numbers");
    let (pa, pb) = (table + 4 * a as usize, table + 4 * b as usize);
    let held = read_i32(pa);
    write_i32(pa, read_i32(pb));
    write_i32(pb, held); // sx12_8: the two holders swapped
}

/// Returns the squad number a player holds for the team on view (the first found, 0..49), or -1.
///
/// `HUMAN_MANAGER::get_player_squad_number(player, manager)`: no manager: the "Error" box (line 0xd2a); out of range:
/// 0xd36; a null player: 0xd3d; all give -1. Only al is the return value.
/// Original: `HUMAN_MANAGER::get_player_squad_number` (0x601880, human_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_player_squad_number(this: usize, player: usize, manager: usize) -> i8 {
    let Some(table) = squad_numbers_or_error(this, manager, 0xd2a, 0xd36) else {
        return -1;
    };
    if player == 0 {
        show_error_at(0x9cf11c, 0xd3d);
        return -1;
    }
    let id = read_i32(player);
    match (0..0x32usize).find(|&n| read_i32(table + 4 * n) == id) {
        Some(n) => {
            covered!("sx12_8 player squad number");
            n as i8 // sx12_8: the player's number
        }
        None => -1,
    }
}

/// The squad number table the comparers compare (`None` when the original would show the "Error" box: only the
/// original runs then).
unsafe fn squad_table_for_comparison(this: usize, manager: usize) -> Option<usize> {
    if !human_record_found(this, manager) {
        return None;
    }
    let m = manager_or_current_human(manager);
    human_manager_human_manager_index(this, m).map(|k| unsafe { squad_number_table(human_manager_record_at(this, k)) })
}

/// Runs a squad number writer side by side over the 50 numbers (200 bytes) of the team on view; the "Error" box (no
/// table, `box_too`): only the original.
unsafe fn compare_squad_writer(name: &str, this: usize, manager: usize, box_too: bool, original: impl FnOnce(), copy: impl FnOnce()) {
    let Some(table) = squad_table_for_comparison(this, manager).filter(|_| !box_too) else {
        return original();
    };
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(table, 0xc8)], original, copy);
    SCOREBOARD.record(equal, || format!("HUMAN_MANAGER::{}: {:?}", name, first_differences(&mo, &mr)));
}

/// Comparer for `prune_squad_numbers`: the current human's table, side by side.
pub(crate) unsafe extern "thiscall" fn compare_prune_squad_numbers(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let human = manager_or_current_human(0);
    if human == 0 {
        return o(this);
    }
    let k = read_i32(human).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(0x10) as usize;
    let table = squad_number_table(human_manager_record_at(this, k));
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(table, 0xc8)], || o(this), || prune_squad_numbers(this));
    SCOREBOARD.record(equal, || format!("HUMAN_MANAGER::prune_squad_numbers: {:?}", first_differences(&mo, &mr)));
}

/// Comparer for `get_squad_number_player`: the return; a bad number (the "Error" box): only the original.
pub(crate) unsafe extern "thiscall" fn compare_get_squad_number_player(this: usize, number: i8, manager: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, usize) -> usize>(%%I1%%);
    if !valid_squad_number(number) {
        return o(this, number, manager);
    }
    compare_human_getter("get_squad_number_player", this, manager, || o(this, number, manager), || get_squad_number_player(this, number, manager))
}

/// Comparer for `set_squad_number_player`.
pub(crate) unsafe extern "thiscall" fn compare_set_squad_number_player(this: usize, number: i8, player: usize, manager: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, usize, usize)>(%%I2%%);
    compare_squad_writer("set_squad_number_player", this, manager, !valid_squad_number(number) || player == 0,
        || o(this, number, player, manager), || set_squad_number_player(this, number, player, manager));
}

/// Comparer for `clear_player_squad_number`.
pub(crate) unsafe extern "thiscall" fn compare_clear_player_squad_number(this: usize, player: usize, manager: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I3%%);
    compare_squad_writer("clear_player_squad_number", this, manager, player == 0,
        || o(this, player, manager), || clear_player_squad_number(this, player, manager));
}

/// Comparer for `clear_squad_number`.
pub(crate) unsafe extern "thiscall" fn compare_clear_squad_number(this: usize, number: i8, manager: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, usize)>(%%I4%%);
    compare_squad_writer("clear_squad_number", this, manager, !valid_squad_number(number),
        || o(this, number, manager), || clear_squad_number(this, number, manager));
}

/// Comparer for `swap_squad_numbers`.
pub(crate) unsafe extern "thiscall" fn compare_swap_squad_numbers(this: usize, a: i8, b: i8, manager: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, usize)>(%%I5%%);
    compare_squad_writer("swap_squad_numbers", this, manager, !valid_squad_number(a) || !valid_squad_number(b),
        || o(this, a, b, manager), || swap_squad_numbers(this, a, b, manager));
}

/// Comparer for `get_player_squad_number`: al; a null player (the "Error" box): only the original.
pub(crate) unsafe extern "thiscall" fn compare_get_player_squad_number(this: usize, player: usize, manager: usize) -> i8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i8>(%%I6%%);
    if player == 0 {
        return o(this, player, manager);
    }
    compare_human_getter("get_player_squad_number", this, manager, || o(this, player, manager) as u8, || get_player_squad_number(this, player, manager) as u8) as i8
}
