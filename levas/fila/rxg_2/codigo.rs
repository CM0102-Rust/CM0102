// ---------- rxg_2: the tactics store loaded from the saved game (tactics.cpp) ----------

/// Data version from which the saved tactics blocks, rows and extra rows are in today's format.
const TACTICS_CURRENT_VERSION: i32 = 0x3e021;

/// The 0x89-byte tactics row of saves below `TACTICS_CURRENT_VERSION` (no playmaker slots).
const OLD_TACTICS_ROW_SIZE: u32 = 0x89;

/// The extra rows read after the statistics (one per human manager slot).
const TACTICS_EXTRA_ROWS: u32 = 0x10;

/// Reads `size` × `n` bytes; false when fewer came.
unsafe fn tactics_read(data: usize, size: u32, n: u32, z: usize) -> bool {
    exe::zread(data, size, n, z) == n
}

/// A tactic block's team instruction 0x20000 when its flags are below 0x8000 (signed), as each loaded block gets.
unsafe fn tactics_mark_loaded_block(block: usize) {
    if read_i32(block + formation_fields::FLAGS) < 0x8000 {
        exe::formation_set_team_instruction(block, 0x20000);
    }
}

/// Reads a tactic block array: the count (`count`), a new array of that many formations (`new_formation_array`) at
/// `blocks`, then the blocks. Below `TACTICS_CURRENT_VERSION` each block is 0x188b bytes and is converted: its 11 old
/// 4-byte position marks (+0x588) become the 8-byte `tactics_fields::POSITION_MARKS` (high half 0) and every position
/// gets the player instructions 8 << 32 and 2 << 32 (`exe::formation_set_player_instruction`). Each block then gets
/// `tactics_mark_loaded_block`. The error lines: (count, memory, old block, blocks).
unsafe fn tactics_read_blocks(this: usize, z: usize, blocks: usize, count: usize, lines: [u32; 4]) -> Result<(), u32> {
    if !tactics_read(this + count, 4, 1, z) {
        return Err(lines[0]);
    }
    let array = new_formation_array(read_u32(this + count));
    write_usize(this + blocks, array);
    if array == 0 {
        return Err(lines[1]);
    }
    if game_data_version() < TACTICS_CURRENT_VERSION {
        covered!("rxg_2 old tactic blocks converted");
        let mut k = 0;
        while k < read_i32(this + count) {
            let block = read_usize(this + blocks) + k as usize * formation_fields::SIZE;
            if !tactics_read(block, 0x188b, 1, z) {
                return Err(lines[2]);
            }
            for position in 0..11u8 {
                let mark = block + tactics_fields::POSITION_MARKS + position as usize * 8;
                write_u32(mark, read_u32(block + 0x588 + position as usize * 4)); // +0x588: the old 4-byte marks
                write_u32(mark + 4, 0);
                exe::formation_set_player_instruction(block, position as i8, 8 << 32);
                exe::formation_set_player_instruction(block, position as i8, 2 << 32);
            }
            tactics_mark_loaded_block(block);
            k += 1;
        }
        return Ok(());
    }
    let n = read_u32(this + count);
    if !tactics_read(array, formation_fields::SIZE as u32, n, z) {
        return Err(lines[3]);
    }
    let mut k = 0;
    while k < read_i32(this + count) {
        tactics_mark_loaded_block(read_usize(this + blocks) + k as usize * formation_fields::SIZE);
        k += 1;
    }
    Ok(())
}

/// An empty tactics row, as the store builds it: the tactic index 0, the captain 0 and every other slot -1.
unsafe fn tactics_reset_row(row: usize) {
    for b in 0..TACTICS_ROW_SIZE as usize {
        write_u8(row + b, 0);
    }
    for b in (2..tactics_row::CAPTAIN).chain(tactics_row::PLAYMAKER..TACTICS_ROW_SIZE as usize) {
        write_u8(row + b, 0xff);
    }
}

/// Converts an old 0x89-byte row to today's 0x91-byte one: the tactic index, the 20 selection slots and the captain as
/// they are, the two playmaker slots -1, and the set piece and shoot-out takers moved up by 8.
unsafe fn tactics_convert_old_row(old: usize, row: usize) {
    for b in 0..tactics_row::PLAYMAKER {
        write_u8(row + b, read_u8(old + b));
    }
    write_i32(row + tactics_row::PLAYMAKER, -1);
    write_i32(row + tactics_row::SECOND_PLAYMAKER, -1);
    for b in tactics_row::PLAYMAKER..OLD_TACTICS_ROW_SIZE as usize {
        write_u8(row + b + 8, read_u8(old + b));
    }
}

/// Reads the club rows: the count, then `count + 16` rows of 0x91 bytes allocated and `count` read; below
/// `TACTICS_CURRENT_VERSION` they come as 0x89-byte rows into a scratch array and are converted
/// (`tactics_convert_old_row`; the scratch array is freed, but not when the second malloc fails).
unsafe fn tactics_read_rows(this: usize, z: usize) -> Result<(), u32> {
    use tactics_fields as f;
    if !tactics_read(this + f::ROW_COUNT, 4, 1, z) {
        return Err(0x853);
    }
    let n = read_u32(this + f::ROW_COUNT);
    if game_data_version() >= TACTICS_CURRENT_VERSION {
        let rows = exe::crt_malloc(n.wrapping_add(TACTICS_EXTRA_ROWS).wrapping_mul(TACTICS_ROW_SIZE));
        write_usize(this + f::ROWS, rows);
        if rows == 0 {
            return Err(0x88c);
        }
        if !tactics_read(rows, TACTICS_ROW_SIZE, read_u32(this + f::ROW_COUNT), z) {
            return Err(0x894);
        }
        return Ok(());
    }
    let old = exe::crt_malloc(n.wrapping_add(TACTICS_EXTRA_ROWS).wrapping_mul(OLD_TACTICS_ROW_SIZE));
    if old == 0 {
        return Err(0x85f);
    }
    let n = read_u32(this + f::ROW_COUNT);
    if !tactics_read(old, OLD_TACTICS_ROW_SIZE, n, z) {
        return Err(0x867);
    }
    let rows = exe::crt_malloc(n.wrapping_add(TACTICS_EXTRA_ROWS).wrapping_mul(TACTICS_ROW_SIZE));
    write_usize(this + f::ROWS, rows);
    if rows == 0 {
        return Err(0x870);
    }
    covered!("rxg_2 old tactics rows converted");
    let mut k = 0;
    while k < read_i32(this + f::ROW_COUNT) {
        tactics_convert_old_row(old + k as usize * OLD_TACTICS_ROW_SIZE as usize, rows + k as usize * TACTICS_ROW_SIZE as usize);
        k += 1;
    }
    exe::crt_free(old);
    Ok(())
}

/// The 16 extra rows after the ten defaults, read after the `count` rows already there (0x89-byte ones into a scratch
/// array below `TACTICS_CURRENT_VERSION`, freed after). A short read adds 16 to the count and makes those rows empty
/// (`tactics_reset_row`; the original first copies the old rows over them and then resets them whole); a full read
/// leaves the count as it was.
unsafe fn tactics_read_extra_rows(this: usize, z: usize) -> Result<(), u32> {
    use tactics_fields as f;
    let count = read_u32(this + f::ROW_COUNT);
    let full = if game_data_version() >= TACTICS_CURRENT_VERSION {
        tactics_read(read_usize(this + f::ROWS) + count as usize * TACTICS_ROW_SIZE as usize, TACTICS_ROW_SIZE, TACTICS_EXTRA_ROWS, z)
    } else {
        let old = exe::crt_malloc(count.wrapping_add(TACTICS_EXTRA_ROWS).wrapping_mul(OLD_TACTICS_ROW_SIZE));
        if old == 0 {
            return Err(0x8c2);
        }
        let full = tactics_read(old + count as usize * OLD_TACTICS_ROW_SIZE as usize, OLD_TACTICS_ROW_SIZE, TACTICS_EXTRA_ROWS, z);
        exe::crt_free(old);
        full
    };
    if !full {
        covered!("rxg_2 extra tactics rows reset");
        let first = read_i32(this + f::ROW_COUNT);
        write_i32(this + f::ROW_COUNT, first.wrapping_add(TACTICS_EXTRA_ROWS as i32));
        for k in first..read_i32(this + f::ROW_COUNT) {
            tactics_reset_row(read_usize(this + f::ROWS) + k as usize * TACTICS_ROW_SIZE as usize); // rxg_2: an extra row
        }
    }
    Ok(())
}

/// Every staff id kept in the rows (the 20 selection slots, the two playmakers and the 10 set piece takers) that is
/// not below the staff count (signed) becomes -1.
unsafe fn tactics_drop_unknown_staff(this: usize) {
    use tactics_fields as f;
    let staff = read_i32(G_STAFF_COUNT);
    let mut k = 0;
    while k < read_i32(this + f::ROW_COUNT) {
        let row = read_usize(this + f::ROWS) + k as usize * TACTICS_ROW_SIZE as usize;
        let slots = (0..20).map(|s| tactics_row::SELECTION + 4 * s)
            .chain([tactics_row::PLAYMAKER, tactics_row::SECOND_PLAYMAKER])
            .chain((0..10).map(|s| tactics_row::SET_PIECE_TAKERS + 4 * s));
        for slot in slots {
            if read_i32(row + slot) >= staff {
                write_i32(row + slot, -1);
            }
        }
        k += 1;
    }
}

/// The contents of tactics.dat in the order `Tactics::write_to_save` writes them; the first failed step gives its line.
unsafe fn tactics_read_file(this: usize, z: usize) -> Result<(), u32> {
    use tactics_fields as f;
    covered!("rxg_2 tactics loaded");
    tactics_read_blocks(this, z, f::CUSTOM_TACTICS, f::CUSTOM_TACTIC_COUNT, [0x7d7, 0x7e0, 0x7ed, 0x803])?;
    tactics_read_blocks(this, z, f::PRESET_TACTICS, f::PRESET_TACTIC_COUNT, [0x815, 0x81e, 0x82b, 0x841])?;
    tactics_read_rows(this, z)?;
    if !tactics_read(this + 4, 4, 1, z) {
        return Err(0x89d); // the statistics count
    }
    let stats = exe::crt_operator_new(read_u32(this + 4).wrapping_mul(TACTIC_STATS_SIZE));
    write_usize(this + f::TACTIC_STATS, stats);
    if stats == 0 {
        return Err(0x8a6);
    }
    if !tactics_read(stats, TACTIC_STATS_SIZE, read_u32(this + 4), z) {
        return Err(0x8ae);
    }
    if !tactics_read(this + 8, 4, 10, z) {
        return Err(0x8b6); // the ten default values
    }
    tactics_read_extra_rows(this, z)?;
    tactics_drop_unknown_staff(this);
    Ok(())
}

/// Loads the tactics store from tactics.dat in the saved game.
///
/// `Tactics::read_from_save(save)`: no save, the "Error" box of line 0x7c8 and 0; opens tactics.dat (`exe::zopen` with
/// 0, 0x16, 4; failing, the box of line 0x7d0 and 0). Then `tactics_read_file`: the clubs' own tactic blocks and the
/// presets (`tactics_read_blocks`: count, a new formation array and the blocks; old saves' blocks converted), the club
/// rows (`tactics_read_rows`: count + 16 allocated, count read; old 0x89-byte rows converted), the statistics count
/// (+4) and records (0x28 bytes, `exe::crt_operator_new`), the ten default values (+8), the 16 extra rows
/// (`tactics_read_extra_rows`) and, last, every staff id in the rows at or above the staff count cleared to -1. Closes
/// the file and returns 1. A failed step shows the "Error" box with its line (`tel26_error_box`, which also clears the
/// left mouse button flag), closes the file and returns 0.
///
/// Defects of the original: the extra rows are read after the ten defaults, where today's save has nothing, so every
/// load adds 16 empty rows to the count, and the next save writes them all (the row count grows by 16 at each save and
/// load) [inference: from `Tactics::write_to_save`, sx20_9]; a full read of them leaves the count as it was; in old
/// saves the extra rows are converted and then reset whole; the old rows' scratch array leaks when the second malloc
/// fails; a failure leaves the arrays allocated so far. The copy does the same, without the dead conversion.
/// Original: `Tactics::read_from_save` (0x8985c0, tactics.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn tactics_read_from_save(this: usize, save: usize) -> u32 {
    if save == 0 {
        tel26_error_box(TACTICS_SOURCE, 0x7c8);
        return 0;
    }
    let mut z = exe::zopen(save, 0xa8b940usize, 0, 0x16, 4); // "tactics.dat"
    if z == 0 {
        tel26_error_box(TACTICS_SOURCE, 0x7d0);
        return 0;
    }
    let loaded = match tactics_read_file(this, z) {
        Ok(()) => 1,
        Err(line) => {
            tel26_error_box(TACTICS_SOURCE, line);
            0
        }
    };
    exe::zclose(&mut z);
    loaded
}

/// Comparer for `tactics_read_from_save`: the original only (it allocates and reads the stream). The proof is the load
/// of the saved game with and without the routine.
pub(crate) unsafe extern "thiscall" fn compare_tactics_read_from_save(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, save)
}
