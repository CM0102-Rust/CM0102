// ---------- sx18_5: a staff name reference out of a news item's slots, and a dated span record into and out of them (record_utils.cpp) ----------

/// The five slot values of a span record: the value (+0), the start date (day, year) and the end date (day, year).
pub(crate) unsafe fn date_span_slot_values(r: usize) -> [u32; 5] {
    let [d0, y0] = date_slot_values(r + run_record::START_DATE);
    let [d1, y1] = date_slot_values(r + run_record::END_DATE);
    [read_u32(r), d0, y0, d1, y1]
}

/// Reads a staff name reference back from the news item's slots from `*k` on and advances `*k`: the three name ids and
/// the staff id (dwords), then the date from the next two (day, year); returns 1. A null news item or reference shows
/// the error box (lines 0x1e4, 0x1ec) and returns 0.
/// Original: `read_staff_name_ref_from_news` (0x7e52c0, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn read_staff_name_ref_from_news(news: usize, k: usize, r: usize) -> u32 {
    if news == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x1e4);
        return 0;
    }
    if r == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x1ec);
        return 0;
    }
    covered!("sx18_5 name reference from news");
    let v = news_take_slots::<6>(news, k);
    write_u32(r, v[0]); // sx18_5: the first name id
    write_u32(r + record_holder::SECOND_NAME_ID, v[1]);
    write_u32(r + record_holder::COMMON_NAME_ID, v[2]);
    write_u32(r + record_holder::STAFF_ID, v[3]);
    news_slot_date(r + record_holder::DATE, v[4], v[5]);
    1
}

/// Stores a span record (`date_span_slot_values`: the value, the start and end dates) in the news item's slots from
/// `*k` on and advances `*k`; returns 1. A null record or news item shows the error box (lines 0x20c, 0x214) and
/// returns 0.
/// Original: `write_date_span_record_to_news` (0x7e5420, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn write_date_span_record_to_news(r: usize, news: usize, k: usize) -> u32 {
    if r == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x20c);
        return 0;
    }
    if news == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x214);
        return 0;
    }
    covered!("sx18_5 span record to news");
    news_put_slots(news, k, &date_span_slot_values(r));
    1
}

/// Reads a span record back from the news item's slots from `*k` on and advances `*k`: the value (+0), then the start
/// date and the end date, each from two slots (day, year); returns 1. A null news item or record shows the error box
/// (lines 0x234, 0x23c) and returns 0.
/// Original: `read_date_span_record_from_news` (0x7e5570, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn read_date_span_record_from_news(news: usize, k: usize, r: usize) -> u32 {
    if news == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x234);
        return 0;
    }
    if r == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x23c);
        return 0;
    }
    covered!("sx18_5 span record from news");
    let [value, d0, y0] = news_take_slots::<3>(news, k);
    write_u32(r, value);
    news_slot_date(r + run_record::START_DATE, d0, y0);
    let [d1, y1] = news_take_slots::<2>(news, k);
    news_slot_date(r + run_record::END_DATE, d1, y1); // sx18_5: the end date
    1
}

pub(crate) unsafe extern "C" fn compare_read_staff_name_ref_from_news(news: usize, k: usize, r: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(%%I0%%);
    if news == 0 || r == 0 {
        return o(news, k, r);
    }
    compare_news_slot_reader(k, r, 0x18, || o(news, k, r), |kc, rc| read_staff_name_ref_from_news(news, kc, rc), "read_staff_name_ref_from_news")
}

pub(crate) unsafe extern "C" fn compare_write_date_span_record_to_news(r: usize, news: usize, k: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(%%I1%%);
    if r == 0 || news == 0 {
        return o(r, news, k);
    }
    compare_news_slot_writer(news, k, &date_span_slot_values(r), || o(r, news, k), "write_date_span_record_to_news")
}

pub(crate) unsafe extern "C" fn compare_read_date_span_record_from_news(news: usize, k: usize, r: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(%%I2%%);
    if news == 0 || r == 0 {
        return o(news, k, r);
    }
    compare_news_slot_reader(k, r, 0x14, || o(news, k, r), |kc, rc| read_date_span_record_from_news(news, kc, rc), "read_date_span_record_from_news")
}
