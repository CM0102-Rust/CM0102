// ---------- batch %%LEVA%%: building the tactics store (tactics.cpp) ----------

/// The 28 formation files of `tactics_fields::CUSTOM_TACTICS`, in load order: the name in CM3_TACTICS (a string of the
/// exe) and the source line of its error box. (442_wide.pct is loaded twice, as in the original.)
const CUSTOM_TACTIC_FILES: [(usize, u32); 28] = [
    (0xa8b934, 0x71), // 352_v1.pct
    (0xa8b8fc, 0x72), // 442_v1.pct
    (0xa8b8f0, 0x73), // 532_v1.pct
    (0xa8b8e4, 0x74), // 442_v4.pct
    (0xa8b8d8, 0x75), // 532_v2.pct
    (0xa8b8cc, 0x76), // 532_v3.pct
    (0xa8b8c0, 0x77), // 532_v4.pct
    (0xa8b8b4, 0x78), // 352_v2.pct
    (0xa8b8a8, 0x79), // 532_v5.pct
    (0xa8b89c, 0x7a), // 532_v6.pct
    (0xa8b890, 0x7b), // 532_v7.pct
    (0xa8b884, 0x7c), // 532_v9.pct
    (0xa8b878, 0x7d), // 442_v8.pct
    (0xa8b868, 0x7e), // sweeper_v1.pct
    (0xa8b858, 0x7f), // 442_push.pct
    (0xa8b840, 0x80), // defensive_counter.pct
    (0xa8b834, 0x81), // 4312_v1.pct
    (0xa8b828, 0x82), // 424_v1.pct
    (0xa8b81c, 0x83), // 442_v10.pct
    (0xa8b80c, 0x84), // 442_wide.pct
    (0xa8b800, 0x85), // 4132.pct
    (0xa8b7f0, 0x87), // 343_default.pct
    (0xa8b7e0, 0x88), // 451_default.pct
    (0xa8b80c, 0x89), // 442_wide.pct
    (0xa8b7cc, 0x8a), // sweeper_default.pct
    (0xa8b7b8, 0x8b), // 451_defensive.pct
    (0xa8b7a4, 0x8c), // 343_defensive.pct
    (0xa8b794, 0x8d), // 451_norway.pct
];

/// The 17 formation files of `tactics_fields::PRESET_TACTICS`, in load order: the file, the display name copied over
/// the loaded name (strings of the exe) and the source line of its error box.
const PRESET_TACTIC_FILES: [(usize, usize, u32); 17] = [
    (0xa8b784, 0xa782b0, 0x9d), // 442_default.pct, "4-4-2"
    (0xa8b768, 0xa8b758, 0x9e), // 442_defensive_default.pct, "4-4-2 Defensive"
    (0xa8b858, 0xa8b748, 0x9f), // 442_push.pct, "4-4-2 Attacking"
    (0xa8b730, 0xa8b720, 0xa0), // 442_diamond_default.pct, "4-4-2 Diamond"
    (0xa8b7f0, 0xa1594c, 0xa1), // 343_default.pct, "3-4-3"
    (0xa8b710, 0xa78298, 0xa2), // 352_default.pct, "3-5-2"
    (0xa8b6f4, 0xa8b6e4, 0xa3), // 352_defensive_default.pct, "3-5-2 Defensive"
    (0xa8b6c8, 0xa1593c, 0xa4), // 352_attacking_default.pct, "3-5-2 Attacking"
    (0xa8b6b4, 0xa7828c, 0xa5), // 41212_default.pct, "4-1-2-1-2"
    (0xa8b6a4, 0xa782a0, 0xa6), // 424_default.pct, "4-2-4"
    (0xa8b694, 0xa782a8, 0xa7), // 433_default.pct, "4-3-3"
    (0xa8b7e0, 0xa78284, 0xa8), // 451_default.pct, "4-5-1"
    (0xa8b684, 0xa782b8, 0xa9), // 532_default.pct, "5-3-2"
    (0xa8b668, 0xa8b658, 0xaa), // 532_defensive_default.pct, "5-3-2 Defensive"
    (0xa8b63c, 0xa8b62c, 0xab), // 532_attacking_default.pct, "5-3-2 Attacking"
    (0xa8b7cc, 0x9b75dc, 0xac), // sweeper_default.pct, "Sweeper"
    (0xa8b800, 0xa8b624, 0xad), // 4132.pct, "4-1-3-2"
];

/// The 9 dwords the tactics store writes at +0xc..+0x2c (constants of the original, meaning unknown).
const TACTICS_CONSTANTS: [u32; 9] = [0x280e8a89, 0x280e8b81, 0x281b06c1, 0x381506c1, 0x2a2a06c1, 0x281b2381, 0x288a26c1, 0x101f06c1, 0x381b0381];

/// The CD check of the tactics store: among the drives (GetLogicalDriveStringsA, 200 bytes) the first CD drive
/// (GetDriveTypeA = 5, always asked) holding "<root>cm0102.exe" (`exe::file_find_first`; on the others the search is
/// closed, `exe::file_find_close`); none: the "Insert CD" box (langlib_sprintf of 0x987df4 and 0x987dc4, the box with
/// 1) and again, forever. cm0102-1080 has no CD requirement here: the jne on the drive type (0x893e25) became nops and
/// the found test (0x893e67, 85 c0) became xor; the copy reads those bytes of the running exe.
unsafe fn tactics_check_cd() {
    let drives: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = read_u8(0x893e25) == 0x90;
    let always_found = read_u8(0x893e67) != 0x85;
    loop {
        // simplification: the buffer starts zeroed; the original would only read garbage with more than 200 bytes of drives
        let mut list = [0u8; 0x100];
        if drives(200, list.as_mut_ptr()) != 0 && list[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = list.as_ptr() as usize + i;
                let t = kind(root);
                if any_drive || t == 5 {
                    let (mut name, mut found) = ([0u8; 0x104], [0u8; 0x114]);
                    let nm = name.as_mut_ptr() as usize;
                    format(nm, 0x987e00, root, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    let f = found.as_mut_ptr() as usize;
                    if exe::file_find_first(nm, f, 0) == 0 || always_found {
                        return;
                    }
                    exe::file_find_close(f);
                }
                // simplification: the original walks with an i8 (steps of 4): it would only pass 127 with more than 32 drives
                i += 4;
                if list[i] == 0 {
                    break;
                }
            }
        }
        let (mut title, mut msg) = ([0u8; 0x100], [0u8; 0x100]);
        text(title.as_mut_ptr() as usize, 0x987df4);
        text(msg.as_mut_ptr() as usize, 0x987dc4);
        exe::gui_show_message_box(title.as_ptr() as usize, msg.as_ptr() as usize, 1);
    }
}

/// A new array of `n` formations of `formation_fields::SIZE` bytes, as `new Formation[n]` does: `exe::crt_operator_new`(4
/// + n × size) with the count in the first dword and the constructor 0x5b27c0 run on each
/// (`exe::crt_vector_constructor_iterator`, destructor 0x5a53c0); out of memory, 0.
unsafe fn new_formation_array(n: u32) -> usize {
    let p = exe::crt_operator_new(n.wrapping_mul(formation_fields::SIZE as u32).wrapping_add(4));
    if p == 0 {
        return 0;
    }
    write_u32(p, n);
    exe::crt_vector_constructor_iterator(p + 4, formation_fields::SIZE as u32, n, 0x5b27c0, 0x5a53c0);
    p + 4
}

/// Loads CM3_TACTICS\`file` into formation `slot` (`exe::utils_build_file_path`(0x9ccae4 "CM3_TACTICS", file), cdecl;
/// `Formation::load_from_disk`(path, 0, 1) 0x5b5d10: 3 pushes; ecx = the slot -> thiscall: not quiet, obfuscated).
unsafe fn load_tactic_file(slot: usize, file: usize) -> u32 {
    let load: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x5b5d10usize);
    load(slot, exe::utils_build_file_path(0x9ccae4, file), 0, 1)
}

/// Rule: the tactics store is built once at the start of a game: the 28 formations the AI clubs use and the 17 preset
/// formations come from the CM3_TACTICS files (or from the saved game), and each club and human manager gets an empty
/// selection row.
///
/// `Tactics::Tactics(saved)`: the CD check (`tactics_check_cd`); the 16 undo slots (`tactics_fields::UNDO_SLOTS`, 0x5c
/// bytes each) get their dword +8 zeroed and `tactics_fields::UNDO_COUNT` = 0. With `saved` (the saved game's path) and
/// the tactics loaded from it (0x8985c0: push saved; ecx = this -> thiscall, ret 4; nonzero), that is all. Otherwise:
/// `tactics_fields::CUSTOM_TACTIC_COUNT` = 28 and `tactics_fields::CUSTOM_TACTICS` = a new array of 28 formations
/// (`new_formation_array`); each file of `CUSTOM_TACTIC_FILES` in turn: while fewer than the count are loaded, it is
/// loaded into the next slot (`load_tactic_file`) and counts when the load succeeds; with the count already reached,
/// the file's error box (`show_error_at` with tactics.cpp, its line); at the end, fewer than the count loaded: the box at
/// line 0x92. The same with `tactics_fields::PRESET_TACTIC_COUNT` = 17, `tactics_fields::PRESET_TACTICS` and
/// `PRESET_TACTIC_FILES` (end box at line 0xb4), each loaded preset getting its display name copied over its name (+0).
/// Then `tactics_fields::ROW_COUNT` = `G_CLUB_COUNT` + 32 rows of 0x91 bytes (`tactics_fields::ROWS`,
/// `exe::crt_malloc`; out of memory: the box at line 0xc3 and return), each 0, 0 and then 0x8f bytes of 0xff (no
/// selection, captain, playmakers, set piece or shoot-out takers); the per-tactic statistics (`tactics_fields::
/// TACTIC_STATS`, +4 = 28 + 17 records of 0x28 bytes, `exe::crt_operator_new`; out of memory: the box at line 0xe2 and
/// return), each zeroed with its index in the first word; and the 9 constants `TACTICS_CONSTANTS` at +0xc..+0x2c.
/// Returns this.
/// Original: `Tactics::Tactics` (0x893dc0, tactics.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_tactics(this: usize, saved: usize) -> usize {
    let load_saved: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8985c0usize);
    tactics_check_cd();
    for k in 0..0x10 {
        write_u32(this + tactics_fields::UNDO_SLOTS + 8 + 0x5c * k, 0);
    }
    write_u8(this + tactics_fields::UNDO_COUNT, 0);
    if saved != 0 && load_saved(this, saved) != 0 {
        covered!("gra_23 tactics from the saved game");
        return this;
    }
    covered!("gra_23 tactics from the files");
    write_u32(this + tactics_fields::CUSTOM_TACTIC_COUNT, 0x1c);
    write_usize(this + tactics_fields::CUSTOM_TACTICS, new_formation_array(0x1c));
    let mut loaded: i32 = 0;
    for &(file, line) in CUSTOM_TACTIC_FILES.iter() {
        if loaded < read_i32(this + tactics_fields::CUSTOM_TACTIC_COUNT) {
            let slot = read_usize(this + tactics_fields::CUSTOM_TACTICS) + loaded as usize * formation_fields::SIZE;
            if load_tactic_file(slot, file) != 0 {
                loaded += 1; // gra23: a loaded AI formation
            }
        } else {
            show_error_at(0xa8b908, line);
        }
    }
    if loaded < read_i32(this + tactics_fields::CUSTOM_TACTIC_COUNT) {
        show_error_at(0xa8b908, 0x92);
    }
    write_u32(this + tactics_fields::PRESET_TACTIC_COUNT, 0x11);
    write_usize(this + tactics_fields::PRESET_TACTICS, new_formation_array(0x11));
    let mut loaded: i32 = 0;
    for &(file, name, line) in PRESET_TACTIC_FILES.iter() {
        if loaded < read_i32(this + tactics_fields::PRESET_TACTIC_COUNT) {
            let slot = read_usize(this + tactics_fields::PRESET_TACTICS) + loaded as usize * formation_fields::SIZE;
            if load_tactic_file(slot, file) != 0 {
                let mut b = 0;
                loop {
                    let ch = read_u8(name + b);
                    write_u8(slot + b, ch); // gra23: the preset's display name (inline strcpy, the zero included)
                    if ch == 0 {
                        break;
                    }
                    b += 1;
                }
                loaded += 1;
            }
        } else {
            show_error_at(0xa8b908, line);
        }
    }
    if loaded < read_i32(this + tactics_fields::PRESET_TACTIC_COUNT) {
        show_error_at(0xa8b908, 0xb4);
    }
    let rows = read_i32(G_CLUB_COUNT).wrapping_add(0x20);
    write_i32(this + tactics_fields::ROW_COUNT, rows);
    let r = exe::crt_malloc((rows as u32).wrapping_mul(0x91));
    write_usize(this + tactics_fields::ROWS, r);
    if r == 0 {
        show_error_at(0xa8b908, 0xc3);
        return this;
    }
    for k in 0..rows.max(0) as usize {
        let row = r + k * 0x91;
        write_u16(row, 0);
        for b in 2..0x91 {
            write_u8(row + b, 0xff); // gra23: an empty selection row
        }
    }
    let n = read_i32(this + tactics_fields::CUSTOM_TACTIC_COUNT).wrapping_add(read_i32(this + tactics_fields::PRESET_TACTIC_COUNT));
    write_i32(this + 4, n);
    let s = exe::crt_operator_new((n as u32).wrapping_mul(0x28));
    write_usize(this + tactics_fields::TACTIC_STATS, s);
    if s == 0 {
        show_error_at(0xa8b908, 0xe2);
        return this;
    }
    for k in 0..n.max(0) as usize {
        for b in (0..0x28).step_by(4) {
            write_u32(s + k * 0x28 + b, 0);
        }
        write_u16(s + k * 0x28, k as u16);
    }
    for (k, &c) in TACTICS_CONSTANTS.iter().enumerate() {
        write_u32(this + 0xc + 4 * k, c);
    }
    this
}

/// The bytes of a loaded formation that the comparer looks at: the name (+0, up to its zero) and the fields
/// `Formation::load_from_disk` writes (password, creator, positions, display order, coordinates, flags, marking types,
/// the position lookup, the mask and flags at +0x187b..+0x188b and the player instructions), not the padding the
/// allocation leaves.
unsafe fn tactic_block_bytes(f: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut k = 0;
    while k < 0x33 && read_u8(f + k) != 0 {
        v.push(read_u8(f + k));
        k += 1;
    }
    for &(o, n) in &[(0x33usize, 0x32usize), (0x65, 200), (0x12d, 0x16), (0x143, 0xb), (0x14e, 0x16), (0x164, 0x420), (0x584, 4), (0x5b4, 0xb), (0x5eb, 0x1290), (0x187b, 0x10), (0x188b, 0x58)] {
        v.extend((0..n).map(|b| read_u8(f + o + b)));
    }
    v
}

/// Comparer for `construct_tactics`. With a saved game only the original runs (the save is read once). In a new game
/// the original builds the real store and the copy builds a second one in a scratch object; the counts, the loaded
/// formations (`tactic_block_bytes`), the rows, the statistics, the undo slots and the constants must be equal; the
/// copy's arrays are then freed (the formations with their destructor, as `delete[]`).
unsafe extern "thiscall" fn compare_construct_tactics(this: usize, saved: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, saved);
    if saved != 0 {
        return r;
    }
    let scratch = vec![0u8; 0x609];
    let c = scratch.as_ptr() as usize;
    let rc = construct_tactics(c, 0);
    let summary = |t: usize| -> Vec<u8> {
        let mut v = Vec::new();
        let custom = read_i32(t + tactics_fields::CUSTOM_TACTIC_COUNT);
        let preset = read_i32(t + tactics_fields::PRESET_TACTIC_COUNT);
        let rows = read_i32(t + tactics_fields::ROW_COUNT);
        let stats = read_i32(t + 4);
        for x in [custom, preset, rows, stats] {
            v.extend_from_slice(&x.to_le_bytes());
        }
        v.extend((0xc..0x30).map(|b| read_u8(t + b)));
        for k in 0..0x10 {
            v.extend_from_slice(&read_u32(t + tactics_fields::UNDO_SLOTS + 8 + 0x5c * k).to_le_bytes());
        }
        v.push(read_u8(t + tactics_fields::UNDO_COUNT));
        for (arr, n) in [(read_usize(t + tactics_fields::CUSTOM_TACTICS), custom), (read_usize(t + tactics_fields::PRESET_TACTICS), preset)] {
            if arr != 0 {
                for k in 0..n.max(0) as usize {
                    v.extend(tactic_block_bytes(arr + k * formation_fields::SIZE));
                }
            }
        }
        let rw = read_usize(t + tactics_fields::ROWS);
        if rw != 0 {
            v.extend((0..rows.max(0) as usize * 0x91).map(|b| read_u8(rw + b)));
        }
        let st = read_usize(t + tactics_fields::TACTIC_STATS);
        if st != 0 {
            v.extend((0..stats.max(0) as usize * 0x28).map(|b| read_u8(st + b)));
        }
        v
    };
    let (mo, mr) = (summary(this), summary(c));
    for (field, count) in [(tactics_fields::CUSTOM_TACTICS, tactics_fields::CUSTOM_TACTIC_COUNT), (tactics_fields::PRESET_TACTICS, tactics_fields::PRESET_TACTIC_COUNT)] {
        let a = read_usize(c + field);
        if a != 0 {
            exe::crt_vector_destructor_iterator(a, formation_fields::SIZE as u32, read_u32(c + count), 0x5a53c0);
            exe::crt_operator_delete(a - 4);
        }
    }
    if read_usize(c + tactics_fields::ROWS) != 0 {
        exe::crt_free(read_usize(c + tactics_fields::ROWS));
    }
    if read_usize(c + tactics_fields::TACTIC_STATS) != 0 {
        exe::crt_operator_delete(read_usize(c + tactics_fields::TACTIC_STATS));
    }
    let first = mo.iter().zip(mr.iter()).position(|(a, b)| a != b);
    SCOREBOARD.record(r == this && rc == c && mo == mr, || format!("construct_tactics({:#x}): {} and {} bytes compared, first different at {:?}", this, mo.len(), mr.len(), first));
    r
}
