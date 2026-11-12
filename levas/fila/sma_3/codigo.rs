// ---------- batch %%LEVA%%: club records: the season snapshot and the record news fields (club_records.cpp) ----------

/// Exe addresses of the record news routines without an `exe::` wrapper.
pub(crate) mod record_news_fns {
    pub(crate) const WRITE_RUN_RECORD_TO_NEWS: usize = 0x7e5420; // writes a run record (+0x14 bytes) into a news item, cdecl
    pub(crate) const DATE_CONSTRUCT_645800: usize = 0x645800; // a match record's constructor: only its Date (0x54a110)
    pub(crate) const CLUB_RECORDS_CPP: usize = 0x98b6a0; // "E:\dev\CM3\cm3 00-01\cm3\code\club_records.cpp"
}

/// Constructs a club's season history snapshot (0x389 bytes): runs the Date constructors of its records in the
/// original's order: the dates at +0x12 and +0x35 (`exe::date_date_54a110`); the match records at +0x58, +0x7d, +0x9c,
/// +0xbb, +0xda and +0xf9 (0x645800, which only constructs their Date); the run records at +0x118, +0x12c, +0x140
/// and +0x154 (`run_record_construct`); the record holders at +0x169, +0x184 and +0x19f (`record_holder_construct`),
/// the match record at +0x1b7; the holders at +0x1d7, +0x1f5, +0x211, +0x22d, +0x249 and +0x263; two blocks of a Date,
/// a holder at +8 and a match record at +0x20 (+0x27b and +0x2ba); the holder at +0x2fb; the transfer fee records at
/// +0x313 (`transfer_fee_record_construct`) and +0x33c (`transfer_fee_record_construct_2`); the holder at +0x371.
/// Returns `this`.
/// Original: `season_history_entry_construct` (0x448260, club_records.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn season_history_entry_construct(this: usize) -> usize {
    let mtch: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(record_news_fns::DATE_CONSTRUCT_645800);
    exe::date_date_54a110(this + 0x12);
    exe::date_date_54a110(this + 0x35);
    for o in [0x58, 0x7d, 0x9c, 0xbb, 0xda, 0xf9] {
        mtch(this + o);
    }
    for o in [0x118, 0x12c, 0x140, 0x154] {
        run_record_construct(this + o);
    }
    for o in [0x169, 0x184, 0x19f] {
        record_holder_construct(this + o);
    }
    mtch(this + 0x1b7);
    for o in [0x1d7, 0x1f5, 0x211, 0x22d, 0x249, 0x263] {
        record_holder_construct(this + o);
    }
    for b in [0x27b, 0x2ba] {
        covered!("sma_3 season snapshot");
        exe::date_date_54a110(this + b);
        record_holder_construct(this + b + 8); // sma3: the holder of the block
        mtch(this + b + 0x20);
    }
    record_holder_construct(this + 0x2fb);
    transfer_fee_record_construct(this + 0x313);
    transfer_fee_record_construct_2(this + 0x33c);
    record_holder_construct(this + 0x371);
    this
}

/// Comparer for `season_history_entry_construct`: side by side over the 0x389 bytes of the snapshot, and the return.
pub(crate) unsafe extern "thiscall" fn compare_season_history_entry_construct(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x389)], || o(this), || season_history_entry_construct(this));
    SCOREBOARD.record(igual && r == n, || format!("season_history_entry_construct {:x?} x {:x?}", mo, mr));
    r
}

/// One field of a broken record as the news item gets it.
#[derive(Clone, Copy)]
pub(crate) enum RecordNewsField {
    /// a byte, zero-extended
    Byte(usize),
    /// a signed byte
    SByte(usize),
    /// a word, zero-extended
    Word(usize),
    /// a signed word
    SWord(usize),
    /// a dword
    Dword(usize),
    /// a float × 100, truncated (`ftol`)
    Percent(usize),
    /// a match record (`exe::write_match_record_to_news`)
    Match(usize),
    /// a record holder's name reference (`exe::write_staff_name_ref_to_news`)
    Holder(usize),
    /// a run record (0x7e5420)
    Run(usize),
}

/// The fields of each record type (1..=32) in the season record, in the original's order.
pub(crate) fn record_news_fields(kind: u8) -> &'static [RecordNewsField] {
    use RecordNewsField::*;
    match kind {
        1 => &[Byte(0), Dword(1), Word(5)],
        2 => &[Byte(7), Dword(8), Word(0xc)],
        3 => &[Dword(0xe), Match(0x12)],
        4 => &[Dword(0x31), Match(0x35)],
        5 => &[Dword(0x54), Match(0x58)],
        6 => &[Dword(0x77), Word(0x7b)],
        7 => &[Match(0x7d)],
        8 => &[Match(0x9c)],
        9 => &[Match(0xbb)],
        10 => &[Match(0xda)],
        11 => &[Match(0xf9)],
        12 => &[Run(0x118)],
        13 => &[Run(0x12c)],
        14 => &[Run(0x140)],
        15 => &[Run(0x154)],
        16 => &[SByte(0x168), Holder(0x169), Word(0x181)],
        17 => &[SByte(0x183), Holder(0x184), Word(0x19c)],
        18 => &[SByte(0x19e), Holder(0x19f), Match(0x1b7)],
        19 => &[SByte(0x1d6), Holder(0x1d7), Word(0x1ef)],
        20 => &[Percent(0x1f1), Holder(0x1f5), SByte(0x20d), Word(0x20e)],
        21 => &[SByte(0x210), Holder(0x211), Word(0x229)],
        22 => &[SByte(0x22b), SByte(0x22c), Holder(0x22d), Word(0x245)],
        23 => &[SWord(0x247), Holder(0x249)],
        24 => &[SWord(0x261), Holder(0x263)],
        25 => &[SWord(0x27b), Word(0x27d), Holder(0x283), Match(0x29b)],
        26 => &[SWord(0x2ba), Word(0x2bc), Holder(0x2c2), Match(0x2da)],
        27 => &[SWord(0x2f9), Holder(0x2fb)],
        28 => &[Dword(0x313), Holder(0x317), Dword(0x32f), SByte(0x33b)],
        29 => &[Dword(0x33c), Holder(0x340), Dword(0x358), SByte(0x364)],
        30 => &[Dword(0x365)],
        31 => &[Dword(0x36b)],
        32 => &[Holder(0x371)],
        _ => &[],
    }
}

/// Writes the club id, the record type and the fields of one broken club record into the slots of a news item.
///
/// `club_records_write_record_news_fields(club_id, record, kind, news)`: without the record (error line 0x1829) or
/// the news item (0x1831), the game's error box and 0. Slot 0 the club id, slot 1 the type (`kind`, signed), then the
/// type's fields (`record_news_fields`) from slot 2 on (`exe::newsitem_set_slot`; the match, holder and run records
/// write their own slots and advance the index). A type outside 1..=32: the error box (line 0x1989) after slots 0 and
/// 1, and 0. Otherwise 1.
/// Original: `club_records_write_record_news_fields` (0x453120, club_records.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_records_write_record_news_fields(club_id: u32, record: usize, kind: u32, news: usize) -> u32 {
    use record_news_fns::CLUB_RECORDS_CPP as FILE;
    if record == 0 {
        show_error_at(FILE, 0x1829);
        return 0;
    }
    if news == 0 {
        show_error_at(FILE, 0x1831);
        return 0;
    }
    let mut k: u8 = 1;
    exe::newsitem_set_slot(news, 0, club_id);
    let kind = kind as u8 as i8;
    exe::newsitem_set_slot(news, 1, kind as i32 as u32); // sma3: the record type
    k += 1;
    let fields = record_news_fields(kind as u8);
    if fields.is_empty() {
        show_error_at(FILE, 0x1989);
        return 0;
    }
    covered!("sma_3 record news fields");
    let run: unsafe extern "C" fn(usize, usize, *mut u8) -> u32 = core::mem::transmute(record_news_fns::WRITE_RUN_RECORD_TO_NEWS);
    let slot = |v: u32, k: &mut u8| {
        exe::newsitem_set_slot(news, *k as i8, v);
        *k = k.wrapping_add(1);
    };
    for f in fields {
        let r = record;
        match *f {
            RecordNewsField::Byte(o) => slot(read_u8(r + o) as u32, &mut k),
            RecordNewsField::SByte(o) => slot(read_i8(r + o) as i32 as u32, &mut k), // sma3: a signed byte field
            RecordNewsField::Word(o) => slot(read_u16(r + o) as u32, &mut k),
            RecordNewsField::SWord(o) => slot(read_i16(r + o) as i32 as u32, &mut k),
            RecordNewsField::Dword(o) => slot(read_u32(r + o), &mut k),
            RecordNewsField::Percent(o) => slot(ftol(read_f32(r + o) as f64 * read_f32(rdata::F32_100) as f64) as u32, &mut k), // sma3: the percentage
            RecordNewsField::Match(o) => {
                exe::write_match_record_to_news(r + o, news, &mut k);
            }
            RecordNewsField::Holder(o) => exe::write_staff_name_ref_to_news(r + o, news, &mut k),
            RecordNewsField::Run(o) => {
                run(r + o, news, &mut k);
            }
        }
    }
    1
}

/// Comparer for `club_records_write_record_news_fields`: side by side over the news item (0xe0 bytes) and the return;
/// without the record or the news item, or with a type outside 1..=32, only the original (the error box).
pub(crate) unsafe extern "C" fn compare_club_records_write_record_news_fields(club_id: u32, record: usize, kind: u32, news: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, u32, usize) -> u32>(%%I1%%);
    if record == 0 || news == 0 || record_news_fields(kind as u8).is_empty() {
        return o(club_id, record, kind, news);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(news, 0xe0)], || o(club_id, record, kind, news),
        || club_records_write_record_news_fields(club_id, record, kind, news));
    SCOREBOARD.record(igual && r == n, || format!("club_records_write_record_news_fields(kind {}) {:x?} x {:x?}", kind as u8, mo, mr));
    r
}
