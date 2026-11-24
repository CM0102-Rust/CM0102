// ---------- sx04b_7: the month ratings' save and a nation's awards in the saved game (month_ratings.cpp, nation_awards.cpp) ----------

/// `MONTH_RATINGS::save_to_disk(file)` (0x767d60, thiscall, ret 4), as `NationAwards::save` calls it.
const FN_MONTH_RATINGS_SAVE_TO_DISK: usize = 0x767d60;
/// The size of a `MONTH_RATINGS` object.
const MONTH_RATINGS_SIZE: u32 = 0xf;
/// A nation's virtual `init_awards(file)` (slot 0: builds its award table, each award loading itself from the file)
/// and each award's virtual `save_to_disk(file)` (slot +4).
const NATION_AWARDS_INIT_SLOT: usize = 0x0;
const AWARD_SAVE_SLOT: usize = 0x4;

/// Writes the month ratings to the saved game as `month_ratings_load_from_disk` reads them: the entry count, the
/// entries (when there are any), the month and the year (`exe::zwrite`); a short write shows the "Error" box
/// (month_ratings.cpp 138, 147, 155, 162) and stops there.
/// Original: `MONTH_RATINGS::save_to_disk` (0x767d60, month_ratings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn month_ratings_save_to_disk(this: usize, file: usize) {
    if exe::zwrite(this + MONTH_RATINGS_COUNT, 4, 1, file) != 1 {
        return tel26_error_box(MONTH_RATINGS_CPP, 0x8a);
    }
    let count = read_u32(this + MONTH_RATINGS_COUNT);
    if count != 0 && exe::zwrite(read_usize(this + MONTH_RATINGS_ENTRIES), MONTH_RATING_ENTRY_SIZE, count, file) != read_u32(this + MONTH_RATINGS_COUNT) {
        return tel26_error_box(MONTH_RATINGS_CPP, 0x93);
    }
    if exe::zwrite(this + month_ratings_fields::MONTH, 1, 1, file) != 1 {
        return tel26_error_box(MONTH_RATINGS_CPP, 0x9b);
    }
    if exe::zwrite(this + month_ratings_fields::YEAR, 2, 1, file) != 1 { // sx04b_7: the ratings' year
        return tel26_error_box(MONTH_RATINGS_CPP, 0xa2);
    }
    covered!("sx04b_7 month ratings saved");
}

/// Comparer for `month_ratings_save_to_disk`: it writes the file once; only the original runs (the save proof compares
/// the written game).
pub(crate) unsafe extern "thiscall" fn compare_month_ratings_save_to_disk(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, file)
}

/// Loads a nation's awards from the saved game: the nation's virtual `init_awards(file)` (its awards, each read from
/// the file), then a 4-byte flag; with the flag set, new month ratings (`exe::crt_operator_new` of 0xf bytes,
/// `MONTH_RATINGS::MONTH_RATINGS(file)`, which reads them; 0 when out of memory) go to
/// `nation_awards_fields::MONTH_RATINGS`. Returns 1, or 0 after the "Error" box (nation_awards.cpp 2059) when the
/// flag cannot be read. The original keeps the new object under an SEH frame; nothing here throws.
/// Original: `NationAwards::load` (0x76a280, nation_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nation_awards_load(this: usize, file: usize) -> u32 {
    let init: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_usize(read_usize(this) + NATION_AWARDS_INIT_SLOT));
    init(this, file);
    let mut has_ratings = 0u32;
    if exe::zread(&mut has_ratings as *mut u32 as usize, 4, 1, file) != 1 {
        tel26_error_box(NATION_AWARDS_CPP, 0x80b);
        return 0;
    }
    if has_ratings != 0 { // sx04b_7: the saved month ratings
        covered!("sx04b_7 nation month ratings loaded");
        let p = exe::crt_operator_new(MONTH_RATINGS_SIZE);
        let ratings = if p != 0 {
            let construct: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_MONTH_RATINGS_CONSTRUCT);
            construct(p, file)
        } else {
            0
        };
        write_usize(this + nation_awards_fields::MONTH_RATINGS, ratings);
    }
    1
}

/// Comparer for `nation_awards_load`: the load reads the file once and allocates; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_load(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, file)
}

/// Saves a nation's awards to the saved game as `nation_awards_load` reads them: each award of the table
/// (`nation_awards_fields::TABLE`, `COUNT` read again at each step) through its virtual `save_to_disk(file)`, then the
/// 4-byte flag of whether the nation has month ratings (taken before the awards), and the month ratings themselves
/// (`MONTH_RATINGS::save_to_disk`) when it has them. A short write of the flag: the "Error" box (nation_awards.cpp
/// 2091) and nothing more.
/// Original: `NationAwards::save` (0x76a380, nation_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nation_awards_save(this: usize, file: usize) {
    let has_ratings = (read_usize(this + nation_awards_fields::MONTH_RATINGS) != 0) as u32;
    let mut k = 0i16;
    while k < read_i16(this + nation_awards_fields::COUNT) {
        let award = read_usize(read_usize(this + nation_awards_fields::TABLE) + 4 * k as usize);
        let save: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_usize(read_usize(award) + AWARD_SAVE_SLOT));
        save(award, file);
        k += 1;
    }
    if exe::zwrite(&has_ratings as *const u32 as usize, 4, 1, file) != 1 { // sx04b_7: the month ratings flag
        return tel26_error_box(NATION_AWARDS_CPP, 0x82b);
    }
    let ratings = read_usize(this + nation_awards_fields::MONTH_RATINGS);
    if ratings != 0 {
        covered!("sx04b_7 nation month ratings saved");
        let save: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_MONTH_RATINGS_SAVE_TO_DISK);
        save(ratings, file);
    }
}

/// Comparer for `nation_awards_save`: it writes the file once; only the original runs (the save proof compares the
/// written game).
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_save(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, file)
}
