// ---------- cx02_13: the English cups' own data in the saved game (eng_cc_cup.cpp, eng_fa_cup.cpp) ----------

/// Reads the League Cup's own data from the saved game: the count of entrants in the European cups
/// (`eng_cc_cup::EUROPEAN_ENTRANT_COUNT`, 1 byte; `exe::crt_fread`). A short read shows the eng_cc_cup.cpp box (line
/// 0x2e4) and returns 0; otherwise 1.
/// Original: `EngCcCup::load_extra_data` (0x56d2e0, eng_cc_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn eng_cc_cup_load_extra_data(this: usize, file: usize) -> u32 {
    if exe::crt_fread(this + eng_cc_cup::EUROPEAN_ENTRANT_COUNT, 1, 1, file) != 1 {
        show_error_at(0x9c4224, 0x2e4); // eng_cc_cup.cpp
        return 0;
    }
    covered!("cx02_13 league cup data loaded");
    1
}

/// Reads the FA Cup's own data from the saved game, one byte each (`exe::crt_fread`, unchecked): the exemption count
/// (`eng_fa_cup::EXEMPTION_COUNT`) and the fifth-round entry choice (`FIFTH_ROUND_ENTRY`); then, when the saved game's
/// data version (`G_GAME_DATA_VERSION`, absolute value) is above 0x3e001, whether the fifth-round question was asked
/// (`FIFTH_ROUND_QUESTION_ASKED`), otherwise that byte becomes 3. Returns 1.
/// Original: `EngFaCup::load_extra_data` (0x570830, eng_fa_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn eng_fa_cup_load_extra_data(this: usize, file: usize) -> u32 {
    exe::crt_fread(this + eng_fa_cup::EXEMPTION_COUNT, 1, 1, file);
    exe::crt_fread(this + eng_fa_cup::FIFTH_ROUND_ENTRY, 1, 1, file);
    if read_i32(G_GAME_DATA_VERSION).wrapping_abs() > 0x3e001 {
        covered!("cx02_13 fa cup question flag loaded");
        exe::crt_fread(this + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED, 1, 1, file);
    } else {
        write_u8(this + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED, 3);
    }
    1
}

/// Writes the FA Cup's own data to the saved game, one byte each (`exe::crt_fwrite`, unchecked), in the order the load
/// reads it: the exemption count, the fifth-round entry choice and whether the fifth-round question was asked. Returns
/// the last write's count.
/// Original: `EngFaCup::save_extra_data` (0x5708a0, eng_fa_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn eng_fa_cup_save_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx02_13 fa cup data saved");
    exe::crt_fwrite(this + eng_fa_cup::EXEMPTION_COUNT, 1, 1, file);
    exe::crt_fwrite(this + eng_fa_cup::FIFTH_ROUND_ENTRY, 1, 1, file);
    exe::crt_fwrite(this + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED, 1, 1, file)
}

/// Comparer of the cups' own data readers: the original reads first; then the file goes back to where it was and the
/// copy reads the same bytes again over the `fields` (the object as the original left it is put back); the returns,
/// the fields and the file positions must match.
pub(crate) unsafe fn compare_cup_extra_data_load(name: &str, this: usize, file: usize, fields: &[(usize, usize)], original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    let at = exe::crt_ftell(file);
    let (mut end_o, mut end_c) = (0, 0);
    let regions: Vec<(usize, usize)> = fields.iter().map(|&(o, n)| (this + o, n)).collect();
    let (r, c, same, mo, mc) = crate::comparison::side_by_side(&regions, || {
        let r = original();
        end_o = exe::crt_ftell(file);
        r
    }, || {
        exe::crt_fseek(file, at, 0);
        let c = copy();
        end_c = exe::crt_ftell(file);
        c
    });
    exe::crt_fseek(file, end_o, 0);
    SCOREBOARD.record(same && r == c && end_o == end_c, || format!("{}({:#x}) returned {} x {}, file at {} x {} (from {}), original {:x?} copy {:x?}", name, this, r, c, end_o, end_c, at, mo, mc));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_eng_cc_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    compare_cup_extra_data_load("eng_cc_cup_load_extra_data", this, file, &[(eng_cc_cup::EUROPEAN_ENTRANT_COUNT, 1)], || o(this, file), || eng_cc_cup_load_extra_data(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_eng_fa_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%);
    compare_cup_extra_data_load("eng_fa_cup_load_extra_data", this, file, &[(eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED, 3)], || o(this, file), || eng_fa_cup_load_extra_data(this, file))
}

/// Comparer for `eng_fa_cup_save_extra_data`: it writes the saved game (it cannot run twice), so only the original
/// runs; the copy is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_eng_fa_cup_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, file)
}
