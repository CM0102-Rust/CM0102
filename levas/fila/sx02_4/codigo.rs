/// Whether the past transfer record `r` is due within the months asked: (past the end: stop, inside the range).
/// The year of its date (unsigned) against the end year, then the month (`exe::date_get_month`, signed) when equal;
/// then the same against the start.
unsafe fn past_transfer_in_range(r: usize, start: (u32, u32), end: (u32, u32)) -> (bool, bool) {
    let year = read_u16(r + past_transfer::DATE + cm_date::YEAR);
    if year > end.1 as u16 || year == end.1 as u16 && exe::date_get_month(r + past_transfer::DATE) as i8 > end.0 as i8 {
        return (true, false);
    }
    let inside = year > start.1 as u16 || year == start.1 as u16 && exe::date_get_month(r + past_transfer::DATE) as i8 >= start.0 as i8;
    (false, inside)
}

/// Appends record `r` to the caller's array `*list` (`*capacity` entries; it grows by 100 through the game's
/// `exe::crt_realloc`/`exe::crt_malloc`, the "Memory Allocation Error" box at `row` when it fails) at index `n`.
unsafe fn append_past_transfer(list: *mut usize, capacity: *mut i32, n: &mut i32, r: usize, row: u32) {
    if *n == *capacity {
        *capacity += 100;
        let bytes = (*capacity as usize * PAST_TRANSFER_SIZE) as u32;
        *list = if *list != 0 { exe::crt_realloc(*list, bytes) } else { exe::crt_malloc(bytes) };
        if *list == 0 {
            show_out_of_memory_box(row, 0xa8ecb8); // "TRANSFER_MANAGER::get_past_transfers() : cannot malloc transfers"
        }
    }
    let to = *list + *n as usize * PAST_TRANSFER_SIZE;
    *n += 1;
    core::ptr::copy_nonoverlapping(r as *const u8, to as *mut u8, past_transfer::DATE);
    let mut tmp = [0u8; 8];
    exe::date_operator_3d(to + past_transfer::DATE, tmp.as_mut_ptr() as usize, r + past_transfer::DATE);
}

/// Collects the past transfers from the first day of `start_month`/`start_year` to `end_month`/`end_year` into the
/// caller's growable array `*list`, whose `*count` is the capacity on entry and the number collected on exit.
///
/// - The caller's array already covering the range (start after its first record's date and the 28th of the end month
/// before its last one, the dates through `exe::date_date_549f70`): nothing, `*count` unchanged. With no array,
/// `*count` = 0.
/// - The records in memory (`past_transfers()`, `past_transfer_count()`, 37 bytes each, in date order): up to the first
/// one past the end month; those from the start month on are copied (`append_past_transfer`, box line 0x291f).
/// - Then each year from the start year to the end year: the file `transfers_<year>.tmp` in CM3_TEMP
/// (`exe::utils_build_file_path`, the name written into 0xde1f64), read record by record (`exe::crt_fread` of 37
/// bytes) up to the first one past the end month, copying those from the start month on (box line 0x295f); closed.
/// Original: `TRANSFER_MANAGER::get_past_transfers` (0x8cc030, transfer_manager.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn get_past_transfers(this: usize, start_month: u32, start_year: u32, end_month: u32, end_year: u32,
                                                          list: *mut usize, count: *mut i32) {
    let this = TransferManagerFields(this);
    let (start, end) = ((start_month, start_year), (end_month, end_year));
    let mut n = 0i32;
    if *list == 0 {
        *count = 0;
    }
    if *list != 0 && *count > 0 {
        // the dates compared by year (unsigned) and day of the year (signed)
        let after = |d: usize, r: usize| {
            let (y, ry) = (read_u16(d + cm_date::YEAR), read_u16(r + cm_date::YEAR));
            y > ry || y == ry && read_i16(d) > read_i16(r)
        };
        let mut date = [0u16; 4];
        let first = *list + past_transfer::DATE;
        if after(exe::date_date_549f70(&mut date, 1, start_month, start_year, u32::MAX), first) {
            let last = *list + (*count as usize - 1) * PAST_TRANSFER_SIZE + past_transfer::DATE;
            if after(last, exe::date_date_549f70(&mut date, 0x1c, end_month, end_year, u32::MAX)) {
                covered!("sx02_4 already covered");
                return;
            }
        }
    }
    for i in 0..this.past_transfer_count() as i32 {
        let r = this.past_transfers() + i as usize * PAST_TRANSFER_SIZE;
        let (past_end, inside) = past_transfer_in_range(r, start, end);
        if past_end {
            break;
        }
        if inside {
            append_past_transfer(list, count, &mut n, r, 0x291f); // sx02_4: a record in memory
        }
    }
    if start_year as u16 <= end_year as u16 {
        let mut record = [0u8; PAST_TRANSFER_SIZE];
        let r = record.as_mut_ptr() as usize;
        exe::date_date_54a110(r + past_transfer::DATE);
        let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        for year in start_year as u16..=end_year as u16 {
            format(0xde1f64, 0xa8ea38, year as u32); // "transfers_%d.tmp"
            let file = exe::crt_fopen(exe::utils_build_file_path(0x987da0, 0xde1f64), 0x98f928); // "CM3_TEMP", "rb"
            if file == 0 {
                continue;
            }
            covered!("sx02_4 yearly file");
            while exe::crt_fread(r, PAST_TRANSFER_SIZE as u32, 1, file) == 1 {
                let (past_end, inside) = past_transfer_in_range(r, start, end);
                if past_end {
                    break;
                }
                if inside {
                    append_past_transfer(list, count, &mut n, r, 0x295f); // sx02_4: a record from the file
                }
            }
            exe::crt_fclose(file);
        }
    }
    *count = n;
}

/// Comparer for `get_past_transfers`: the copy runs first on a copy of the caller's array (allocated with the game's
/// `exe::crt_malloc`, freed afterwards), then the original on the real one; checks the count and the records.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_get_past_transfers(this: usize, start_month: u32, start_year: u32, end_month: u32, end_year: u32,
                                                                  list: *mut usize, count: *mut i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, *mut usize, *mut i32)>(%%I0%%);
    let (mut copy, mut copy_count) = (0usize, *count);
    if *list != 0 {
        let bytes = (*count).max(0) as usize * PAST_TRANSFER_SIZE;
        copy = exe::crt_malloc(bytes.max(1) as u32);
        core::ptr::copy_nonoverlapping(*list as *const u8, copy as *mut u8, bytes);
    }
    crate::rng::pause_trace(|| get_past_transfers(this, start_month, start_year, end_month, end_year, &mut copy, &mut copy_count));
    o(this, start_month, start_year, end_month, end_year, list, count);
    let n = (*count).max(0) as usize * PAST_TRANSFER_SIZE;
    let same = copy_count == *count && (n == 0 || core::slice::from_raw_parts(*list as *const u8, n) == core::slice::from_raw_parts(copy as *const u8, n));
    SCOREBOARD.record(same, || format!("get_past_transfers({}/{} .. {}/{}): original {} rust {}", start_month, start_year, end_month, end_year, *count, copy_count));
    if copy != 0 {
        exe::crt_free(copy);
    }
}
