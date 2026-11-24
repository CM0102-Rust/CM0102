// ---------- batch %%LEVA%%: writing the hall of fame file (hall_of_fame.cpp; sx12b_5) ----------

/// `hall_of_fame_unscramble_and_check` (cdecl(buffer, length, checksum) -> 1/0), called through the exe so that its
/// own detour runs.
const FN_HALL_OF_FAME_UNSCRAMBLE: usize = 0x5f0680;

/// Writes the hall of fame (28 tables, 0x52080 bytes) to `hall_of_fame.bin`; returns 1, or 0 after an error.
///
/// `HallOfFame::save()`: `hall_of_fame.bin` opened "wb"; failing: the box with the translated "Error saving to disk" and
/// its message (0x9ce464) and 0. The header (the magic dword 0x3abfb2 and the version byte 1, 5 bytes; failing: the
/// "Error" box, hall_of_fame.cpp line 0x294, the file closed, 0). A null `this`: the boxes of lines 0x375 and 0x29c,
/// closed, 0. The table is scrambled in place: each byte i is added (unscrambled) to the checksum and XORed with the
/// low byte of the key dword i mod 480 (0x9cdc88); then for i from 0 up, the bytes at key[i mod 480] mod 0x52080 and
/// key[(i + 1) mod 480] mod 0x52080 (signed remainders) are swapped. The scrambled table is written (failing: line
/// 0x2a4), then the checksum (0x2ac), each failing with the file closed and 0; the table is unscrambled and checked
/// again (`hall_of_fame_unscramble_and_check`; failing: 0x2b4, closed, 0). The file closed, 1. Called by the game save
/// (0x82e430), the hall of fame screen (0x825cc0) and the constructor when the load fails. Defect of the original: a
/// failed write of the table or the checksum leaves the table in memory scrambled.
/// Original: `HallOfFame::save` (0x5f00c0, hall_of_fame.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn hall_of_fame_save(this: usize) -> u32 {
    use hall_of_fame_consts as hf;
    const BYTES: usize = hf::TABLES * hf::TABLE_SIZE;
    let file = exe::crt_fopen(0x9ce450usize, 0x9a7f28usize); // "hall_of_fame.bin", "wb"
    if file == 0 {
        let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let (mut title, mut message) = ([0u8; 0x200], [0u8; 0x200]);
        text(title.as_mut_ptr() as usize, 0x9ce4e4); // "Error saving to disk"
        text(message.as_mut_ptr() as usize, 0x9ce464);
        exe::gui_show_message_box(title.as_ptr() as usize, message.as_ptr() as usize, 0);
        return 0;
    }
    let failed = |row: u32| {
        show_error_at(hf::HALL_OF_FAME_CPP, row);
        exe::crt_fclose(file);
        0
    };
    let mut header = [0xb2u8, 0xbf, 0x3a, 0x00, 1]; // the magic 0x3abfb2 and the version 1
    if exe::crt_fwrite(header.as_mut_ptr() as usize, 5, 1, file) != 1 {
        return failed(0x294);
    }
    if this == 0 {
        show_error_at(hf::HALL_OF_FAME_CPP, 0x375);
        return failed(0x29c);
    }
    let key = |i: usize| read_i32(0x9cdc88 + 4 * (i % 0x1e0)); // the 480 key dwords
    let mut checksum = 0u32;
    for i in 0..BYTES {
        let b = read_u8(this + i);
        checksum = checksum.wrapping_add(b as u32); // sx12b_5: the checksum of the plain bytes
        write_u8(this + i, b ^ key(i) as u8);
    }
    for i in 0..BYTES {
        let a = this.wrapping_add((key(i) % BYTES as i32) as usize);
        let b = this.wrapping_add((key(i + 1) % BYTES as i32) as usize);
        let t = read_u8(a);
        write_u8(a, read_u8(b));
        write_u8(b, t);
    }
    if exe::crt_fwrite(this, BYTES as u32, 1, file) != 1 {
        return failed(0x2a4);
    }
    if exe::crt_fwrite(&mut checksum as *mut u32 as usize, 4, 1, file) != 1 {
        return failed(0x2ac);
    }
    let check: unsafe extern "C" fn(usize, i32, u32) -> u32 = core::mem::transmute(FN_HALL_OF_FAME_UNSCRAMBLE);
    if check(this, BYTES as i32, checksum) == 0 {
        return failed(0x2b4);
    }
    covered!("sx12b_5 hall of fame saved");
    exe::crt_fclose(file);
    1
}

/// Comparer for `hall_of_fame_save`: the original writes the file; the copy then writes it again (the same file name,
/// the same table): the file bytes, the table and the return are compared. When the original fails (a box), only the
/// original.
pub(crate) unsafe extern "thiscall" fn compare_hall_of_fame_save(this: usize) -> u32 {
    let bytes = hall_of_fame_consts::TABLES * hall_of_fame_consts::TABLE_SIZE;
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    if r == 0 {
        return r;
    }
    let written = std::fs::read("hall_of_fame.bin").unwrap_or_default();
    let table = core::slice::from_raw_parts(this as *const u8, bytes).to_vec();
    let n = hall_of_fame_save(this);
    let again = std::fs::read("hall_of_fame.bin").unwrap_or_default();
    let now = core::slice::from_raw_parts(this as *const u8, bytes);
    let first = written.iter().zip(again.iter()).position(|(x, y)| x != y);
    SCOREBOARD.record(r == n && written == again && table == now, || format!("HallOfFame::save original {} rust {}: file sizes {} {}, first different byte {:?}, table equal {}",
        r, n, written.len(), again.len(), first, table == now));
    r
}
