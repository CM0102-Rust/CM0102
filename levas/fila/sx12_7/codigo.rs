// ---------- batch %%LEVA%%: loading the hall of fame file and the order of its entries (hall_of_fame.cpp; sx12_7) ----------

/// `hall_of_fame_unscramble_and_check` (cdecl), called through the exe so that the comparison sees it.
const FN_HALL_OF_FAME_UNSCRAMBLE_AND_CHECK: usize = 0x5f0680;
/// The 480 dwords of the hall of fame scrambling key.
const HALL_OF_FAME_KEY: usize = 0x9cdc88;

/// Reads `hall_of_fame.bin` into the 28 hall of fame tables; returns 1, or 0 when the file is missing or bad.
///
/// `HallOfFame::load()`: opens the file ("rb"); missing: 0 with no box. Reads the 5-byte header (failing: the "Error"
/// box, hall_of_fame.cpp line 0x1c0), checks the magic dword 0x3abfb2 (0x1c8) and the version byte 1 (0x1d0); these
/// close the file and return 0. Then the tables (`hall_of_fame_consts`, 28 × 12000 = 0x52080 bytes, into `this`; 0x1d8),
/// the checksum dword (0x1e1) and the unscrambling with its check (`hall_of_fame_unscramble_and_check`; 0x1ea): any of
/// these failing shows the box, clears the whole table, closes the file and returns 0. Otherwise the file is closed and
/// it returns 1. Called once by the constructor `HallOfFame::HallOfFame` (0x5ef0b0), which writes a new file when it
/// fails.
/// Original: `HallOfFame::load` (0x5efaa0, hall_of_fame.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn hall_of_fame_load(this: usize) -> u32 {
    use hall_of_fame_consts as hf;
    const BYTES: u32 = (hf::TABLES * hf::TABLE_SIZE) as u32;
    let file = exe::crt_fopen(0x9ce450usize, 0x98f928usize); // "hall_of_fame.bin", "rb"
    if file == 0 {
        return 0;
    }
    let failed = |row: u32, clear: bool| {
        show_error_at(hf::HALL_OF_FAME_CPP, row);
        if clear {
            core::ptr::write_bytes(this as *mut u8, 0, BYTES as usize);
        }
        exe::crt_fclose(file);
        0
    };
    let mut header = [0u8; 5];
    if exe::crt_fread(header.as_mut_ptr() as usize, 5, 1, file) != 1 {
        return failed(0x1c0, false);
    }
    if read_u32(header.as_ptr() as usize) != 0x3abfb2 {
        return failed(0x1c8, false); // not a hall of fame file
    }
    if header[4] != 1 {
        return failed(0x1d0, false); // another version
    }
    if exe::crt_fread(this, BYTES, 1, file) != 1 {
        return failed(0x1d8, true);
    }
    let mut checksum = 0u32;
    if exe::crt_fread(&mut checksum as *mut u32 as usize, 4, 1, file) != 1 {
        return failed(0x1e1, true);
    }
    let check: unsafe extern "C" fn(usize, i32, u32) -> u32 = core::mem::transmute(FN_HALL_OF_FAME_UNSCRAMBLE_AND_CHECK);
    if check(this, BYTES as i32, checksum) == 0 {
        return failed(0x1ea, true); // sx12_7: a bad checksum
    }
    covered!("sx12_7 hall of fame loaded");
    exe::crt_fclose(file);
    1
}

/// Unscrambles a hall of fame buffer in place and tells whether its byte sum matches the checksum.
///
/// `hall_of_fame_unscramble_and_check(buffer, length, checksum)`: a null buffer or a negative length shows the "Error"
/// box (hall_of_fame.cpp line 0x39f) and returns 0. From the last byte down (i = length − 1 .. 0), it swaps the bytes
/// at key[i mod 480] mod length and key[(i + 1) mod 480] mod length (the key dwords at 0x9cdc88, signed remainders);
/// then XORs each byte i with the low byte of key[i mod 480] and adds the result to a sum. Returns 1 when the sum
/// equals the checksum, else 0.
/// Original: `hall_of_fame_unscramble_and_check` (0x5f0680, hall_of_fame.cpp, cdecl).
pub(crate) unsafe extern "C" fn hall_of_fame_unscramble_and_check(buffer: usize, length: i32, checksum: u32) -> u32 {
    if buffer == 0 || length < 0 {
        show_error_at(hall_of_fame_consts::HALL_OF_FAME_CPP, 0x39f);
        return 0;
    }
    let key = |i: u32| read_i32(HALL_OF_FAME_KEY + 4 * (i % 0x1e0) as usize);
    let mut i = length - 1;
    while i >= 0 {
        let a = buffer + (key(i as u32) % length) as usize;
        let b = buffer + (key(i as u32 + 1) % length) as usize;
        let t = read_u8(a);
        write_u8(a, read_u8(b));
        write_u8(b, t);
        i -= 1;
    }
    let mut sum = 0u32;
    for i in 0..length as u32 {
        let v = read_u8(buffer + i as usize) ^ key(i) as u8;
        write_u8(buffer + i as usize, v); // sx12_7: the unscrambled byte
        sum = sum.wrapping_add(v as u32);
    }
    covered!("sx12_7 hall of fame unscrambled");
    (sum == checksum) as u32
}

/// Orders two hall of fame entries for qsort: the higher score (`hall_of_fame_consts::SCORE`, unsigned) first, then
/// the older time stamp (`hall_of_fame_consts::TIME`, signed) first; equal: 0. A null a gives 1 and a null b -1.
/// Original: `hall_of_fame_compare_entries` (0x5f0620, hall_of_fame.cpp, cdecl).
pub(crate) unsafe extern "C" fn hall_of_fame_compare_entries(a: usize, b: usize) -> i32 {
    use hall_of_fame_consts as hf;
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (sa, sb) = (read_u32(a + hf::SCORE), read_u32(b + hf::SCORE));
    if sa < sb {
        return 1;
    }
    if sa > sb {
        return -1;
    }
    covered!("sx12_7 hall of fame equal scores");
    let (ta, tb) = (read_i32(a + hf::TIME), read_i32(b + hf::TIME));
    if tb < ta {
        1
    } else if ta >= tb {
        0
    } else {
        -1 // sx12_7: the older entry first
    }
}

/// Comparer for `hall_of_fame_load`: the original loads into the table; when it succeeds the copy loads the same file
/// into a buffer of its own and the two tables and returns are compared. A failed load (missing file or the "Error"
/// box): only the original.
pub(crate) unsafe extern "thiscall" fn compare_hall_of_fame_load(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    if r == 0 {
        return r;
    }
    let bytes = hall_of_fame_consts::TABLES * hall_of_fame_consts::TABLE_SIZE;
    let mut copy = vec![0u8; bytes];
    let n = hall_of_fame_load(copy.as_mut_ptr() as usize);
    let original = core::slice::from_raw_parts(this as *const u8, bytes);
    let first = original.iter().zip(copy.iter()).position(|(x, y)| x != y);
    SCOREBOARD.record(r == n && first.is_none(), || format!("HallOfFame::load original {} rust {}, first different byte {:?}", r, n, first));
    r
}

/// Comparer for `hall_of_fame_unscramble_and_check`: the buffer and the return, side by side. A null buffer or a
/// negative length (the "Error" box): only the original.
pub(crate) unsafe extern "C" fn compare_hall_of_fame_unscramble_and_check(buffer: usize, length: i32, checksum: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, u32) -> u32>(%%I1%%);
    if buffer == 0 || length < 0 {
        return o(buffer, length, checksum);
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(buffer, length as usize)], || o(buffer, length, checksum),
        || hall_of_fame_unscramble_and_check(buffer, length, checksum));
    SCOREBOARD.record(equal && r == n, || format!("hall_of_fame_unscramble_and_check({}) original {} rust {}: {:?}", length, r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `hall_of_fame_compare_entries`: the return, side by side (the routine only reads).
pub(crate) unsafe extern "C" fn compare_hall_of_fame_compare_entries(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = hall_of_fame_compare_entries(a, b);
    SCOREBOARD.record(r == n, || format!("hall_of_fame_compare_entries({:#x}, {:#x}) original {} rust {}", a, b, r, n));
    r
}
