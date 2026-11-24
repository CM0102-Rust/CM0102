// ---------- sx18_4: a club match record and a staff name reference into and out of a news item's numbered slots (record_utils.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\record_utils.cpp" (the error boxes).
const RECORD_UTILS_CPP: usize = 0xa7a088;

/// Stores `values` in the news item's slots from `*k` on (`NewsItem::set_slot`), advancing `*k` past them.
pub(crate) unsafe fn news_put_slots(news: usize, k: usize, values: &[u32]) {
    for &v in values {
        let slot = read_u8(k);
        write_u8(k, slot.wrapping_add(1));
        exe::newsitem_set_slot(news, slot as i8, v);
    }
}

/// Reads `N` slots of the news item from `*k` on (`NewsItem::get_slot`), advancing `*k` past them.
pub(crate) unsafe fn news_take_slots<const N: usize>(news: usize, k: usize) -> [u32; N] {
    let mut v = [0u32; N];
    for x in v.iter_mut() {
        let slot = read_u8(k);
        write_u8(k, slot.wrapping_add(1));
        *x = exe::newsitem_get_slot(news, slot as u32);
    }
    v
}

/// Writes the date (day, year) read from two slots into `at` (`Date::Date` then `Date::operator=`).
pub(crate) unsafe fn news_slot_date(at: usize, day: u32, year: u32) {
    let mut d = [0u8; 8];
    let p = exe::new_date(d.as_mut_ptr() as usize, day, year);
    copy_record_date(at, p);
}

/// The two slot values of a date: the day word sign-extended, the year word zero-extended.
pub(crate) unsafe fn date_slot_values(at: usize) -> [u32; 2] {
    [read_i16(at) as i32 as u32, read_u16(at + 2) as u32]
}

/// The eleven slot values of a club match record, in the original's order: the date (day, year), the fixture id, the
/// round prefix and name words (sign-extended), the venue byte (sign-extended), the club and opponent ids, the goals for
/// and against (sign-extended) and the match statistics slot.
pub(crate) unsafe fn match_record_slot_values(record: usize) -> [u32; 11] {
    let [day, year] = date_slot_values(record);
    [day, year, read_u32(record + club_match_record::FIXTURE_ID),
     read_i16(record + club_match_record::ROUND_PREFIX) as i32 as u32, read_i16(record + club_match_record::ROUND_NAME) as i32 as u32,
     read_i8(record + club_match_record::VENUE) as i32 as u32, read_u32(record + club_match_record::CLUB_ID),
     read_u32(record + club_match_record::OPPONENT_ID), read_i8(record + club_match_record::GOALS_FOR) as i32 as u32, // sx18_4: the goals for
     read_i8(record + club_match_record::GOALS_AGAINST) as i32 as u32, read_u32(record + club_match_record::MATCH_STATS_SLOT)]
}

/// The six slot values of a staff name reference: the three name ids, the staff id and the date (day, year).
pub(crate) unsafe fn staff_name_ref_slot_values(r: usize) -> [u32; 6] {
    let [day, year] = date_slot_values(r + record_holder::DATE);
    [read_u32(r), read_u32(r + record_holder::SECOND_NAME_ID), read_u32(r + record_holder::COMMON_NAME_ID),
     read_u32(r + record_holder::STAFF_ID), day, year]
}

/// Stores the eleven fields of a club match record (`match_record_slot_values`) in the news item's slots from `*k` on
/// and advances `*k`; returns 1. A null record or news item shows the error box (lines 0x160, 0x168) and returns 0.
/// Original: `write_match_record_to_news` (0x7e4de0, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn write_match_record_to_news(record: usize, news: usize, k: usize) -> u32 {
    if record == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x160);
        return 0;
    }
    if news == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x168);
        return 0;
    }
    covered!("sx18_4 match record to news");
    news_put_slots(news, k, &match_record_slot_values(record));
    1
}

/// Reads a club match record back from the news item's slots from `*k` on and advances `*k`: the date from the first
/// two (day, year), then the fixture id, the round prefix and name (words), the venue (byte), the club and opponent
/// ids, the goals for and against (bytes) and the match statistics slot; returns 1. A null news item or record shows
/// the error box (lines 0x18e, 0x196) and returns 0.
/// Original: `read_match_record_from_news` (0x7e4fa0, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn read_match_record_from_news(news: usize, k: usize, record: usize) -> u32 {
    if news == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x18e);
        return 0;
    }
    if record == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x196);
        return 0;
    }
    covered!("sx18_4 match record from news");
    let [day, year] = news_take_slots::<2>(news, k);
    news_slot_date(record, day, year);
    let v = news_take_slots::<9>(news, k);
    write_u32(record + club_match_record::FIXTURE_ID, v[0]);
    write_u16(record + club_match_record::ROUND_PREFIX, v[1] as u16);
    write_u16(record + club_match_record::ROUND_NAME, v[2] as u16);
    write_u8(record + club_match_record::VENUE, v[3] as u8);
    write_u32(record + club_match_record::CLUB_ID, v[4]);
    write_u32(record + club_match_record::OPPONENT_ID, v[5]);
    write_u8(record + club_match_record::GOALS_FOR, v[6] as u8);
    write_u8(record + club_match_record::GOALS_AGAINST, v[7] as u8); // sx18_4: the goals against
    write_u32(record + club_match_record::MATCH_STATS_SLOT, v[8]);
    1
}

/// Stores a staff name reference (`staff_name_ref_slot_values`: the three name ids, the staff id, the date) in the news
/// item's slots from `*k` on and advances `*k`; returns 1. A null reference or news item shows the error box (lines
/// 0x1bb, 0x1c3) and returns 0.
/// Original: `write_staff_name_ref_to_news` (0x7e5160, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn write_staff_name_ref_to_news(r: usize, news: usize, k: usize) -> u32 {
    if r == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x1bb);
        return 0;
    }
    if news == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x1c3);
        return 0;
    }
    covered!("sx18_4 name reference to news");
    news_put_slots(news, k, &staff_name_ref_slot_values(r));
    1
}

/// Comparer of the slot writers: the copy's values are taken first, then only the original writes (a news item's slots
/// are written once); the slots it wrote from the old `*k` must hold the copy's values, and `*k` must have moved past
/// them.
pub(crate) unsafe fn compare_news_slot_writer(news: usize, k: usize, values: &[u32], original: impl FnOnce() -> u32, name: &str) -> u32 {
    let k0 = read_u8(k);
    let r = original();
    let got: Vec<u32> = (0..values.len()).map(|i| exe::newsitem_get_slot(news, k0.wrapping_add(i as u8) as u32)).collect();
    let ok = r == 1 && read_u8(k) == k0.wrapping_add(values.len() as u8) && got == values;
    SCOREBOARD.record(ok, || format!("{}(news {:#x}, slot {}): original {:x?}, copy {:x?}", name, news, k0, got, values));
    r
}

/// Comparer of the slot readers: the copy reads into a copy of the record with its own `*k` first, then the original
/// reads into the record; the `size` bytes of both records and the two `*k` must match.
pub(crate) unsafe fn compare_news_slot_reader(k: usize, record: usize, size: usize, original: impl FnOnce() -> u32, copy: impl FnOnce(usize, usize) -> u32, name: &str) -> u32 {
    let mut local = vec![0u8; size];
    core::ptr::copy_nonoverlapping(record as *const u8, local.as_mut_ptr(), size);
    let mut k_copy = read_u8(k);
    let n = copy(&mut k_copy as *mut u8 as usize, local.as_mut_ptr() as usize);
    let r = original();
    let same = core::slice::from_raw_parts(record as *const u8, size) == &local[..];
    SCOREBOARD.record(r == n && same && read_u8(k) == k_copy, || format!("{}: original {:x?} k {}, copy {:x?} k {}", name,
        core::slice::from_raw_parts(record as *const u8, size), read_u8(k), local, k_copy));
    r
}

pub(crate) unsafe extern "C" fn compare_write_match_record_to_news(record: usize, news: usize, k: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(%%I0%%);
    if record == 0 || news == 0 {
        return o(record, news, k);
    }
    compare_news_slot_writer(news, k, &match_record_slot_values(record), || o(record, news, k), "write_match_record_to_news")
}

pub(crate) unsafe extern "C" fn compare_read_match_record_from_news(news: usize, k: usize, record: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(%%I1%%);
    if news == 0 || record == 0 {
        return o(news, k, record);
    }
    compare_news_slot_reader(k, record, 0x1f, || o(news, k, record), |kc, rc| read_match_record_from_news(news, kc, rc), "read_match_record_from_news")
}

pub(crate) unsafe extern "C" fn compare_write_staff_name_ref_to_news(r: usize, news: usize, k: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(%%I2%%);
    if r == 0 || news == 0 {
        return o(r, news, k);
    }
    compare_news_slot_writer(news, k, &staff_name_ref_slot_values(r), || o(r, news, k), "write_staff_name_ref_to_news")
}
